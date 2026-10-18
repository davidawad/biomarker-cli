//! Candidates -> stored results.
//!
//! One pipeline for every external format: filter by category and status,
//! map to a marker (LOINC first, then names), convert the unit, dedupe against
//! what is stored (by source key and content hash first, by the natural key
//! second), and write the measurement with the lab's own reference range and
//! flag beside it.

use std::collections::{BTreeMap, HashMap};

use serde::Serialize;
use serde_json::{json, Value};

use super::candidate::{is_abnormal, Coding, LabRange, Parsed, ResultCandidate, ResultValue, Status};
use super::loinc;
use crate::commands::import::{collapse_unit, Mapping};
use crate::db::{int, opt_real, opt_text, text, Db, RowExt};
use crate::error::{AppError, Result};
use crate::matching;
use crate::store::{self, Catalog, Dedupe, Marker, NewMeasurement, NewObservation, Person};
use crate::util::{age_years, date_of, merge_tags, now_iso, parse_qualifier, parse_when, slugify, Tz};

/// What to do with a result no marker matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Unmapped {
    /// List them in the summary and keep them in `fhir_unmapped`
    Report,
    /// Count them and drop them
    Skip,
    /// Create a custom marker (unit from the first result, LOINC recorded)
    Create,
}

pub struct Options<'a> {
    pub person: &'a Person,
    /// Lowercase FHIR observation category codes to import.
    pub categories: Vec<String>,
    pub include_preliminary: bool,
    pub apply_retractions: bool,
    pub unmapped: Unmapped,
    pub dedupe: Dedupe,
    pub link_existing: bool,
    pub lab: Option<String>,
    pub tags: Vec<String>,
    pub mapping: &'a Mapping,
    pub date_formats: Vec<String>,
    pub tz: &'a Tz,
    /// `fhir`, `fhir-ndjson` or `hl7v2`, as recorded in `result_source.source_format`.
    pub format: String,
    pub batch: String,
}

#[derive(Debug, Default, Serialize)]
pub struct Summary {
    pub batch: Option<String>,
    pub source: String,
    pub format: String,
    pub dry_run: bool,
    pub person: String,
    pub dedupe: String,
    /// Results read (after splitting panels into components).
    pub results: usize,
    pub inserted: usize,
    pub replaced: usize,
    pub skipped: usize,
    pub retracted: usize,
    pub retractions_applied: usize,
    pub conflicts: usize,
    pub unmapped: usize,
    pub unconvertible: usize,
    pub invalid: usize,
    /// Qualitative results: inserted / replaced / skipped.
    pub observations: BTreeMap<&'static str, usize>,
    /// Resources set aside because the importer does not read them, by type.
    pub ignored: BTreeMap<String, usize>,
    pub ignored_resources: usize,
    pub skipped_category: BTreeMap<String, usize>,
    pub skipped_status: BTreeMap<String, usize>,
    pub panel_headers: usize,
    pub no_value: usize,
    pub unsupported_value: usize,
    pub markers_created: Vec<String>,
    pub unmapped_results: Vec<Value>,
    pub unconvertible_results: Vec<Value>,
    pub conflict_results: Vec<Value>,
    pub skipped_names: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<Value>,
    #[serde(skip)]
    unmapped_by: BTreeMap<String, (String, usize)>,
    #[serde(skip)]
    unconvertible_by: BTreeMap<String, (String, usize)>,
}

impl Summary {
    pub fn new(source: String, format: &str, dry_run: bool, dedupe: String) -> Self {
        Self { source, format: format.into(), dry_run, dedupe, ..Self::default() }
    }

    fn seed(&mut self, p: &Parsed) {
        self.ignored = p.ignored.clone();
        self.ignored_resources = p.ignored.values().sum();
        self.panel_headers = p.panel_headers;
        self.no_value = p.no_value;
        self.unsupported_value = p.unsupported_value;
        self.warnings.extend(p.warnings.iter().cloned());
    }

    fn finish(&mut self) {
        let list = |m: &BTreeMap<String, (String, usize)>, key: &str| -> Vec<Value> {
            m.iter().map(|(label, (reason, n))| json!({key: label, "reason": reason, "results": n})).collect()
        };
        self.unmapped_results = list(&self.unmapped_by, "result");
        self.unconvertible_results = list(&self.unconvertible_by, "result");
    }

    fn note(&mut self, unconvertible: bool, c: &ResultCandidate, reason: &str) {
        let (count, by) = if unconvertible {
            (&mut self.unconvertible, &mut self.unconvertible_by)
        } else {
            (&mut self.unmapped, &mut self.unmapped_by)
        };
        *count += 1;
        let label = match c.names.first() {
            Some(n) if c.loinc_codes().next().is_some() => {
                format!("{} ({})", c.loinc_codes().next().unwrap_or_default(), n)
            }
            _ => c.label(),
        };
        by.entry(label).or_insert_with(|| (reason.to_string(), 0)).1 += 1;
    }

    fn invalid(&mut self, c: &ResultCandidate, why: impl Into<String>) {
        self.invalid += 1;
        self.errors.push(json!({"result": c.origin, "error": why.into()}));
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

pub fn apply(db: &Db, parsed: &Parsed, o: &Options, s: &mut Summary) -> Result<()> {
    s.seed(parsed);
    let mut cat = Catalog::load(db)?;
    let mut loincs = loinc::load(db)?;
    for c in &parsed.candidates {
        s.results += 1;
        candidate(db, &mut cat, &mut loincs, o, c, s)?;
    }
    s.finish();
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Measurement,
    Observation,
}

impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::Observation => "observation",
        }
    }
    fn table(self) -> &'static str {
        match self {
            Self::Measurement => "measurements",
            Self::Observation => "observations",
        }
    }
}

struct Link {
    id: i64,
    target: i64,
    sha: String,
    revision: i64,
}

fn row_exists(db: &Db, kind: Kind, id: i64) -> Result<bool> {
    Ok(db.query_opt(&format!("SELECT 1 FROM {} WHERE id = ?1", kind.table()), &[int(id)])?.is_some())
}

fn delete_target(db: &Db, kind: Kind, id: i64) -> Result<()> {
    db.execute(&format!("DELETE FROM {} WHERE id = ?1", kind.table()), &[int(id)]).map(|_| ())
}

/// The stored result this source key was imported as. A link whose row has
/// since been deleted, or that was a different kind of result, is dropped.
fn find_link(db: &Db, person_id: i64, key: &str, want: Kind) -> Result<Option<Link>> {
    let Some(r) = db.query_opt(
        "SELECT id, kind, target_id, content_sha256, revision FROM result_source WHERE person_id = ?1 AND source_key = ?2",
        &[int(person_id), text(key)],
    )?
    else {
        return Ok(None);
    };
    let kind = if r.s(1).as_deref() == Some("observation") { Kind::Observation } else { Kind::Measurement };
    let link = Link {
        id: r.i(0).unwrap_or_default(),
        target: r.i(2).unwrap_or_default(),
        sha: r.s(3).unwrap_or_default(),
        revision: r.i(4).unwrap_or(1),
    };
    let exists = row_exists(db, kind, link.target)?;
    if exists && kind == want {
        return Ok(Some(link));
    }
    if exists {
        delete_target(db, kind, link.target)?;
    }
    db.execute("DELETE FROM result_source WHERE id = ?1", &[int(link.id)])?;
    Ok(None)
}

fn linked_elsewhere(db: &Db, kind: Kind, target: i64, key: &str) -> Result<bool> {
    Ok(db
        .query_opt(
            "SELECT 1 FROM result_source WHERE kind = ?1 AND target_id = ?2 AND source_key <> ?3 LIMIT 1",
            &[text(kind.as_str()), int(target), text(key)],
        )?
        .is_some())
}

fn upsert_link(db: &Db, o: &Options, c: &ResultCandidate, kind: Kind, target: i64, prev: Option<&Link>) -> Result<()> {
    let basis = c.time_basis.map(|b| b.as_str().to_string());
    match prev {
        Some(l) => db
            .execute(
                "UPDATE result_source SET kind = ?1, target_id = ?2, source_format = ?3, resource_version = ?4, content_sha256 = ?5,
                 previous_sha256 = ?6, revision = ?7, time_basis = ?8, batch_id = ?9, imported_at = ?10 WHERE id = ?11",
                &[
                    text(kind.as_str()),
                    int(target),
                    text(&o.format),
                    opt_text(c.version.as_ref()),
                    text(&c.sha256),
                    text(&l.sha),
                    int(l.revision + 1),
                    opt_text(basis.as_ref()),
                    text(&o.batch),
                    text(now_iso()),
                    int(l.id),
                ],
            )
            .map(|_| ()),
        None => db
            .execute(
                "INSERT INTO result_source (person_id, kind, target_id, source_format, source_key, resource_version, content_sha256,
                 time_basis, batch_id, imported_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                &[
                    int(o.person.id),
                    text(kind.as_str()),
                    int(target),
                    text(&o.format),
                    text(&c.key),
                    opt_text(c.version.as_ref()),
                    text(&c.sha256),
                    opt_text(basis.as_ref()),
                    text(&o.batch),
                    text(now_iso()),
                ],
            )
            .map(|_| ()),
    }
}

// ---------------------------------------------------------------------------
// One candidate
// ---------------------------------------------------------------------------

fn candidate(
    db: &Db,
    cat: &mut Catalog,
    loincs: &mut HashMap<String, i64>,
    o: &Options,
    c: &ResultCandidate,
    s: &mut Summary,
) -> Result<()> {
    if let Some(code) = c.category.as_deref().map(str::to_ascii_lowercase) {
        if !o.categories.contains(&code) {
            *s.skipped_category.entry(code).or_default() += 1;
            return Ok(());
        }
    }
    let mut tags = o.tags.clone();
    match c.status {
        Status::Retracted => return retract(db, o, c, s),
        st if st.is_final() => {}
        st if st.is_provisional() && o.include_preliminary => tags.push("preliminary".into()),
        st => {
            *s.skipped_status.entry(st.as_str().to_string()).or_default() += 1;
            return Ok(());
        }
    }
    if let Some(n) = c.names.iter().find(|n| o.mapping.skips(n)) {
        if !s.skipped_names.contains(n) {
            s.skipped_names.push(n.clone());
        }
        return Ok(());
    }
    let marker = match resolve_marker(cat, loincs, o, c, s) {
        Some(m) => m,
        None => match o.unmapped {
            Unmapped::Create => match create_marker(db, cat, loincs, c, s)? {
                Some(m) => m,
                None => return unmapped(db, o, c, s, "no marker matches and one could not be created"),
            },
            _ => return unmapped(db, o, c, s, "no marker matches this code or name"),
        },
    };
    let Some(raw) = c.taken_at.as_deref() else {
        s.invalid(c, "no date on the result, its specimen, report or issue time");
        return Ok(());
    };
    let fmts: Vec<&str> = o.date_formats.iter().map(String::as_str).collect();
    let taken_at = match parse_when(raw, &fmts, o.tz) {
        Ok(t) => t,
        Err(e) => {
            s.invalid(c, e.message);
            return Ok(());
        }
    };
    match &c.value {
        ResultValue::Numeric { .. } => measurement(db, cat, o, c, &marker, &taken_at, tags, s),
        ResultValue::Text(t) => observation(db, o, c, &marker, &taken_at, t, s),
    }
}

fn retract(db: &Db, o: &Options, c: &ResultCandidate, s: &mut Summary) -> Result<()> {
    let want = if matches!(c.value, ResultValue::Text(_)) { Kind::Observation } else { Kind::Measurement };
    let Some(l) = find_link(db, o.person.id, &c.key, want)? else {
        *s.skipped_status.entry("retracted (never imported)".into()).or_default() += 1;
        return Ok(());
    };
    s.retracted += 1;
    if o.apply_retractions {
        delete_target(db, want, l.target)?;
        db.execute("DELETE FROM result_source WHERE id = ?1", &[int(l.id)])?;
        s.retractions_applied += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Marker mapping
// ---------------------------------------------------------------------------

fn resolve_marker(
    cat: &Catalog,
    loincs: &HashMap<String, i64>,
    o: &Options,
    c: &ResultCandidate,
    s: &mut Summary,
) -> Option<Marker> {
    for code in c.loinc_codes() {
        if let Some(slug) = o.mapping.loinc.get(code) {
            match cat.find(slug) {
                Some(m) => return Some(m.clone()),
                None => s.warnings.push(format!("mapping [loinc] {code} names unknown marker '{slug}'")),
            }
        }
    }
    c.loinc_codes()
        .find_map(|code| loincs.get(code).and_then(|id| cat.by_id(*id)))
        .or_else(|| c.names.iter().find_map(|n| matching::resolve(cat, &o.mapping.rename(n))))
        .or_else(|| {
            // a local code is only ever an alias candidate
            c.codes.iter().filter(|k| !is_loinc(k)).find_map(|k| matching::resolve(cat, &o.mapping.rename(&k.code)))
        })
        .cloned()
}

fn is_loinc(c: &Coding) -> bool {
    c.system.trim_end_matches('/').eq_ignore_ascii_case("http://loinc.org")
}

fn unmapped(db: &Db, o: &Options, c: &ResultCandidate, s: &mut Summary, reason: &str) -> Result<()> {
    s.note(false, c, reason);
    keep_unmapped(db, o, c, None, reason)
}

fn keep_unmapped(db: &Db, o: &Options, c: &ResultCandidate, unit: Option<&str>, reason: &str) -> Result<()> {
    if o.unmapped == Unmapped::Skip {
        return Ok(());
    }
    let value = match &c.value {
        ResultValue::Numeric { value, .. } => value.to_string(),
        ResultValue::Text(t) => t.clone(),
    };
    db.execute(
        "INSERT INTO fhir_unmapped (batch_id, person_id, loinc, display, value_text, unit, taken_at, reason)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[
            text(&o.batch),
            int(o.person.id),
            opt_text(c.loinc_codes().next()),
            opt_text(c.names.first()),
            text(value),
            opt_text(unit),
            opt_text(c.taken_at.as_ref()),
            text(reason),
        ],
    )
    .map(|_| ())
}

/// A custom marker for a result nothing maps to. Never touches catalog markers.
fn create_marker(
    db: &Db,
    cat: &mut Catalog,
    loincs: &mut HashMap<String, i64>,
    c: &ResultCandidate,
    s: &mut Summary,
) -> Result<Option<Marker>> {
    let code = c.loinc_codes().next().map(str::to_string);
    let name = c.names.first().cloned().or_else(|| code.clone());
    let Some(name) = name else { return Ok(None) };
    let unit = match &c.value {
        ResultValue::Numeric { unit: Some(u), .. } => cat.spell_unit(u),
        ResultValue::Numeric { unit: None, .. } => return Ok(None),
        ResultValue::Text(_) => String::new(),
    };
    let mut slug = slugify(&name);
    if slug.is_empty() || cat.find(&slug).is_some() {
        slug = slugify(&format!("{name}-{}", code.clone().unwrap_or_default()));
    }
    if slug.is_empty() || cat.find(&slug).is_some() {
        return Ok(None);
    }
    let category = match c.category.as_deref() {
        Some("vital-signs") => "vitals",
        _ => "other",
    };
    let m = Marker {
        id: 0,
        slug,
        name,
        category: category.into(),
        unit,
        loinc: code.clone(),
        description: Some("created by import from external results".into()),
        builtin: false,
        aliases: vec![],
    };
    let id = store::insert_marker(db, &m)?;
    if !m.unit.is_empty() {
        store::ensure_unit(db, &m.unit, "both")?;
    }
    if let Some(code) = &code {
        db.execute(
            "INSERT OR IGNORE INTO marker_loinc (loinc, marker_id, relation, builtin) VALUES (?1, ?2, 'exact', 0)",
            &[text(code), int(id)],
        )?;
        loincs.insert(code.clone(), id);
    }
    let m = Marker { id, ..m };
    cat.markers.push(m.clone());
    s.markers_created.push(m.slug.clone());
    Ok(Some(m))
}

// ---------------------------------------------------------------------------
// Measurements
// ---------------------------------------------------------------------------

/// The lab range that fits the person: sex- or age-specific first, then an
/// unconstrained one, then any with both bounds.
pub fn pick_range<'a>(ranges: &'a [LabRange], sex: Option<&str>, age: Option<f64>) -> Option<&'a LabRange> {
    let usable: Vec<&LabRange> = ranges.iter().filter(|r| r.low.is_some() || r.high.is_some()).collect();
    let normal = |r: &&LabRange| {
        r.kind
            .as_deref()
            .is_none_or(|k| matches!(k.to_ascii_lowercase().as_str(), "normal" | "recommended" | "reference"))
    };
    let pool: Vec<&LabRange> =
        if usable.iter().any(normal) { usable.iter().copied().filter(normal).collect() } else { usable };
    let range_sex = |r: &LabRange| {
        r.applies_to.iter().find_map(|a| match a.to_ascii_lowercase().as_str() {
            "male" | "m" => Some("male"),
            "female" | "f" => Some("female"),
            _ => None,
        })
    };
    let has_age = |r: &LabRange| r.age_low.is_some() || r.age_high.is_some();
    let constrained = |r: &LabRange| range_sex(r).is_some() || has_age(r);
    let fits = |r: &LabRange| {
        let sex_ok = range_sex(r).is_none_or(|rs| Some(rs) == sex);
        let age_ok = match (has_age(r), age) {
            (false, _) => true,
            (true, None) => false,
            (true, Some(a)) => r.age_low.is_none_or(|l| a >= l) && r.age_high.is_none_or(|h| a <= h),
        };
        sex_ok && age_ok
    };
    pool.iter()
        .find(|r| constrained(r) && fits(r))
        .or_else(|| pool.iter().find(|r| !constrained(r)))
        .or_else(|| pool.iter().find(|r| r.low.is_some() && r.high.is_some()))
        .or_else(|| pool.first())
        .copied()
}

fn ranges_json(ranges: &[LabRange]) -> Value {
    Value::Array(
        ranges
            .iter()
            .map(|r| {
                json!({
                    "low": r.low, "high": r.high, "unit": r.unit, "text": r.text, "type": r.kind,
                    "applies_to": r.applies_to, "age_low": r.age_low, "age_high": r.age_high,
                })
            })
            .collect(),
    )
}

#[allow(clippy::too_many_arguments)]
fn measurement(
    db: &Db,
    cat: &Catalog,
    o: &Options,
    c: &ResultCandidate,
    marker: &Marker,
    taken_at: &str,
    tags: Vec<String>,
    s: &mut Summary,
) -> Result<()> {
    let ResultValue::Numeric { value, comparator, unit } = &c.value else { return Ok(()) };
    let Some(unit) = unit.as_deref().map(|u| o.mapping.unit(u, false)).filter(|u| !u.trim().is_empty()) else {
        return unconvertible(db, o, c, s, None, "the result carries no unit");
    };
    let unit = collapse_unit(cat, marker, cat.spell_unit(&unit));
    let canonical = match cat.to_canonical(marker, *value, &unit) {
        Ok(v) => v,
        Err(e) => return unconvertible(db, o, c, s, Some(&unit), &e.message),
    };
    let nm = NewMeasurement {
        person_id: o.person.id,
        marker_id: marker.id,
        taken_at: taken_at.to_string(),
        value_raw: *value,
        unit_raw: unit.clone(),
        value: canonical,
        qualifier: comparator.as_deref().and_then(|q| parse_qualifier(q).ok().flatten()),
        lab: c.lab.clone().or_else(|| o.lab.clone()).filter(|l| !l.is_empty()),
        fasting: c.fasting,
        note: Some(c.notes.join("; ")).filter(|n| !n.is_empty()),
        tags: merge_tags(&[], &tags, &[]),
        batch_id: Some(o.batch.clone()),
    };
    let pid = o.person.id;
    let link = find_link(db, pid, &c.key, Kind::Measurement)?;
    let id = match link {
        Some(l) if l.sha == c.sha256 => {
            s.skipped += 1;
            return Ok(());
        }
        Some(l) => {
            // amended at the source: update the row this key was imported as
            if store::existing_measurement(db, pid, marker.id, taken_at)?.is_some_and(|other| other != l.target) {
                return conflict(s, c, marker, taken_at);
            }
            store::update_measurement(db, l.target, &nm)?;
            upsert_link(db, o, c, Kind::Measurement, l.target, Some(&l))?;
            s.replaced += 1;
            l.target
        }
        None => match store::existing_measurement(db, pid, marker.id, taken_at)? {
            Some(other) => {
                if linked_elsewhere(db, Kind::Measurement, other, &c.key)? {
                    return conflict(s, c, marker, taken_at);
                }
                match o.dedupe {
                    Dedupe::Skip => {
                        s.skipped += 1;
                        if o.link_existing {
                            upsert_link(db, o, c, Kind::Measurement, other, None)?;
                            let has_meta = db
                                .query_opt("SELECT 1 FROM lab_result_meta WHERE measurement_id = ?1", &[int(other)])?
                                .is_some();
                            if !has_meta {
                                write_meta(db, cat, o, c, marker, &unit, taken_at, other)?;
                            }
                        }
                        return Ok(());
                    }
                    Dedupe::Replace => {
                        store::update_measurement(db, other, &nm)?;
                        upsert_link(db, o, c, Kind::Measurement, other, None)?;
                        s.replaced += 1;
                        other
                    }
                    Dedupe::Error => {
                        return Err(AppError::invalid(format!(
                            "duplicate measurement {} at {taken_at} (use --dedupe skip|replace)",
                            marker.slug
                        )))
                    }
                }
            }
            None => {
                let (_, id) = store::insert_measurement(db, &nm, Dedupe::Skip)?;
                upsert_link(db, o, c, Kind::Measurement, id, None)?;
                s.inserted += 1;
                id
            }
        },
    };
    write_meta(db, cat, o, c, marker, &unit, taken_at, id)
}

fn conflict(s: &mut Summary, c: &ResultCandidate, marker: &Marker, taken_at: &str) -> Result<()> {
    s.conflicts += 1;
    s.conflict_results
        .push(json!({"result": c.origin, "marker": marker.slug, "taken_at": taken_at, "source_key": c.key}));
    Ok(())
}

fn unconvertible(
    db: &Db,
    o: &Options,
    c: &ResultCandidate,
    s: &mut Summary,
    unit: Option<&str>,
    why: &str,
) -> Result<()> {
    let reason = format!("unit_unconvertible: {why}");
    s.note(true, c, &reason);
    keep_unmapped(db, o, c, unit, &reason)
}

#[allow(clippy::too_many_arguments)]
fn write_meta(
    db: &Db,
    cat: &Catalog,
    o: &Options,
    c: &ResultCandidate,
    marker: &Marker,
    unit: &str,
    taken_at: &str,
    id: i64,
) -> Result<()> {
    let age = o
        .person
        .dob
        .as_deref()
        .and_then(|d| date_of(d).ok())
        .and_then(|dob| date_of(taken_at).ok().map(|on| age_years(dob, on)));
    let chosen = pick_range(&c.ranges, o.person.sex.as_deref(), age);
    let to_canonical = |v: Option<f64>, u: Option<&String>| -> Option<f64> {
        let u = u.map_or_else(
            || unit.to_string(),
            |u| collapse_unit(cat, marker, cat.spell_unit(&o.mapping.unit(u, false))),
        );
        cat.to_canonical(marker, v?, &u).ok()
    };
    let (low, high) =
        chosen.map_or((None, None), |r| (to_canonical(r.low, r.unit.as_ref()), to_canonical(r.high, r.unit.as_ref())));
    let ref_text = chosen.and_then(|r| r.text.clone());
    let interp = Some(c.interpretation.join(",")).filter(|i| !i.is_empty());
    db.execute(
        "INSERT OR REPLACE INTO lab_result_meta (measurement_id, ranges_json, ref_low, ref_high, ref_text, interpretation,
         lab_name, specimen_type, loinc, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        &[
            int(id),
            opt_text((!c.ranges.is_empty()).then(|| ranges_json(&c.ranges).to_string())),
            opt_real(low),
            opt_real(high),
            opt_text(ref_text.as_ref()),
            opt_text(interp.as_ref()),
            opt_text(c.lab.as_ref().or(o.lab.as_ref())),
            opt_text(c.specimen_type.as_ref()),
            opt_text(c.loinc_codes().next()),
            text(c.status.as_str()),
        ],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Qualitative results
// ---------------------------------------------------------------------------

fn observation(
    db: &Db,
    o: &Options,
    c: &ResultCandidate,
    marker: &Marker,
    taken_at: &str,
    value: &str,
    s: &mut Summary,
) -> Result<()> {
    let no = NewObservation {
        person_id: o.person.id,
        marker_id: marker.id,
        taken_at: taken_at.to_string(),
        text: value.to_string(),
        flag: is_abnormal(&c.interpretation).then(|| "abnormal".to_string()),
        range_low: None,
        range_high: None,
        note: Some(c.notes.join("; ")).filter(|n| !n.is_empty()),
        lab: c.lab.clone().or_else(|| o.lab.clone()).filter(|l| !l.is_empty()),
        batch_id: Some(o.batch.clone()),
    };
    let pid = o.person.id;
    let bump = |s: &mut Summary, what: &'static str| *s.observations.entry(what).or_default() += 1;
    match find_link(db, pid, &c.key, Kind::Observation)? {
        Some(l) if l.sha == c.sha256 => bump(s, "skipped"),
        Some(l) => {
            if store::existing_observation(db, pid, marker.id, taken_at)?.is_some_and(|other| other != l.target) {
                return conflict(s, c, marker, taken_at);
            }
            store::update_observation(db, l.target, &no)?;
            upsert_link(db, o, c, Kind::Observation, l.target, Some(&l))?;
            bump(s, "replaced");
        }
        None => match store::existing_observation(db, pid, marker.id, taken_at)? {
            Some(other) => {
                if linked_elsewhere(db, Kind::Observation, other, &c.key)? {
                    return conflict(s, c, marker, taken_at);
                }
                match o.dedupe {
                    Dedupe::Skip => {
                        bump(s, "skipped");
                        if o.link_existing {
                            upsert_link(db, o, c, Kind::Observation, other, None)?;
                        }
                    }
                    Dedupe::Replace => {
                        store::update_observation(db, other, &no)?;
                        upsert_link(db, o, c, Kind::Observation, other, None)?;
                        bump(s, "replaced");
                    }
                    Dedupe::Error => {
                        return Err(AppError::invalid(format!(
                            "duplicate observation {} at {taken_at} (use --dedupe skip|replace)",
                            marker.slug
                        )))
                    }
                }
            }
            None => {
                let id = store::insert_observation_row(db, &no)?;
                upsert_link(db, o, c, Kind::Observation, id, None)?;
                bump(s, "inserted");
            }
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(low: Option<f64>, high: Option<f64>) -> LabRange {
        LabRange { low, high, ..LabRange::default() }
    }

    #[test]
    fn picks_the_range_that_fits_the_person() {
        let mut male = range(Some(13.5), Some(17.5));
        male.applies_to = vec!["male".into()];
        let mut female = range(Some(12.0), Some(15.5));
        female.applies_to = vec!["Female".into()];
        let any = range(Some(12.0), Some(17.5));
        let all = [any.clone(), male.clone(), female.clone()];
        assert_eq!(pick_range(&all, Some("female"), None).map(|r| r.low), Some(Some(12.0)));
        assert_eq!(pick_range(&all, Some("male"), Some(40.0)).map(|r| r.high), Some(Some(17.5)));
        assert_eq!(pick_range(&all, Some("male"), None).map(|r| r.low), Some(Some(13.5)));
        // unknown sex: the unconstrained range
        assert_eq!(pick_range(&all, None, None).map(|r| (r.low, r.high)), Some((Some(12.0), Some(17.5))));
        assert!(
            pick_range(&[LabRange { text: Some("see report".into()), ..LabRange::default() }], None, None).is_none()
        );
    }

    #[test]
    fn age_bands_need_a_known_age() {
        let mut kid = range(Some(0.0), Some(5.0));
        kid.age_low = Some(0.0);
        kid.age_high = Some(12.0);
        let adult = range(Some(1.0), Some(3.0));
        let all = [kid, adult];
        assert_eq!(pick_range(&all, None, Some(8.0)).map(|r| r.high), Some(Some(5.0)));
        assert_eq!(pick_range(&all, None, Some(40.0)).map(|r| r.high), Some(Some(3.0)));
        assert_eq!(pick_range(&all, None, None).map(|r| r.high), Some(Some(3.0)));
    }

    #[test]
    fn prefers_normal_ranges_over_treatment_targets() {
        let mut target = range(Some(0.0), Some(7.0));
        target.kind = Some("treatment".into());
        let mut normal = range(Some(4.0), Some(5.6));
        normal.kind = Some("normal".into());
        assert_eq!(pick_range(&[target, normal], None, None).map(|r| r.high), Some(Some(5.6)));
    }
}
