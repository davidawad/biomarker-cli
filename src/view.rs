//! Evaluating stored measurements for display: display units, applicable
//! ranges and flags, and the stable `biomarker/v1` measurement record.

use serde_json::json;

use crate::catalog::Facts;
use crate::output::Record;
use crate::ranges::{self, Flag, Level, Range, RangeKind, Status};
use crate::store::{Catalog, Marker, MeasurementRow};
use crate::util::{age_years, date_of};

#[derive(Debug, Clone)]
pub struct Evaluated {
    pub row: MeasurementRow,
    pub marker: Marker,
    pub display_value: f64,
    pub display_unit: String,
    pub reference: Option<Range>,
    pub optimal: Option<Range>,
    /// Near-limit bounds (see [`ranges::status`]).
    pub warn: Option<Range>,
    /// Where each of `reference`, `optimal`, `warn` came from, as
    /// [`pick_range`] reports it (`person`, `set:<name>` or `catalog`).
    pub sources: [Option<String>; 3],
    /// Age in years at `taken_at`, when the person has a birth date.
    pub age: Option<f64>,
    /// Classification against the reference range and the warn bounds.
    pub status: Status,
    pub ref_flag: Option<Flag>,
    pub opt_flag: Option<Flag>,
    pub ref_level: Option<Level>,
    pub opt_level: Option<Level>,
    /// The person's own `range_flavor`, when their profile sets one.
    pub flavor: Option<String>,
}

fn age_at(row: &MeasurementRow) -> Option<f64> {
    let dob = row.dob.as_deref().and_then(|d| date_of(d).ok())?;
    date_of(&row.taken_at).ok().map(|on| age_years(dob, on))
}

/// Who a range is picked for, and the draw it is picked for.
#[derive(Debug, Clone, Copy)]
pub struct Subject<'a> {
    pub person: &'a str,
    pub person_id: i64,
    pub sex: Option<&'a str>,
    pub age: Option<f64>,
    pub lab: Option<&'a str>,
    pub fasting: Option<bool>,
    /// Date of the draw, for `when.markers` conditions (`None`: latest values).
    pub taken_at: Option<&'a str>,
}

/// The ranges of one set for a marker and kind: a range-set file's entries
/// (lab-specific ones first) or the catalog's.
fn set_pool(cat: &Catalog, set: &str, marker_id: i64, kind: RangeKind, lab: Option<&str>) -> Vec<Range> {
    if cat.profiles.is_file_set(set) {
        return cat.profiles.set_ranges(set, marker_id, kind, lab);
    }
    cat.catalog_ranges
        .iter()
        .filter(|r| r.marker_id == marker_id && r.kind == kind && r.set.as_deref() == Some(set))
        .cloned()
        .collect()
}

/// The range that applies, and where it came from: the person's own ranges
/// (profile file, then `range set --person`) as `person`; config.toml
/// `[[range]]` as `config`; their (or the global) `range_set` as
/// `set:<name>`; then the first of the `range_sets` with an applicable range
/// as `catalog` (merged with the database's own ranges, which replace
/// catalog ranges of the same sex and age band; a range-set file in that
/// list reports `set:<name>`). In each tier the most specific range wins
/// (see [`ranges::select_for`]).
pub fn pick_range(cat: &Catalog, s: &Subject, marker: &Marker, kind: RangeKind) -> Option<(Range, String)> {
    let value_of = |slug: &str| cat.condition_value(s.person_id, slug, s.taken_at);
    let facts = Facts {
        sex: s.sex,
        age: s.age,
        fasting: s.fasting,
        attrs: cat.person_attrs.get(&s.person_id),
        value_of: &value_of,
    };
    let af = cat.profiles.age_first;
    let sel = |pool: &[Range]| ranges::select_for(pool, marker.id, kind, &facts, af).cloned();
    let mut own = cat.profiles.personal_ranges(s.person, marker.id, kind, s.lab);
    own.extend(
        cat.person_ranges
            .iter()
            .filter(|r| r.person_id == Some(s.person_id) && r.marker_id == marker.id && r.kind == kind)
            .cloned(),
    );
    if let Some(r) = sel(&own) {
        return Some((r, "person".into()));
    }
    if let Some(r) = sel(&cat.profiles.config_ranges(marker.id, kind, s.lab)) {
        return Some((r, "config".into()));
    }
    if let Some(set) = cat.profiles.range_set_for(s.person) {
        if let Some(r) = sel(&set_pool(cat, set, marker.id, kind, s.lab)) {
            return Some((r, format!("set:{set}")));
        }
    }
    let local: Vec<Range> = cat.ranges.iter().filter(|r| r.marker_id == marker.id && r.kind == kind).cloned().collect();
    for set in cat.profiles.sets_for(Some(s.person), marker.id) {
        let pool = set_pool(cat, &set, marker.id, kind, s.lab);
        if sel(&pool).is_none() {
            continue;
        }
        if cat.profiles.is_file_set(&set) {
            return sel(&pool).map(|r| (r, format!("set:{set}")));
        }
        let shadowed = |c: &Range| {
            c.cond.is_empty()
                && local.iter().any(|l| l.sex == c.sex && l.age_min == c.age_min && l.age_max == c.age_max)
        };
        let merged: Vec<Range> = local.iter().cloned().chain(pool.into_iter().filter(|c| !shadowed(c))).collect();
        return sel(&merged).map(|r| (r, "catalog".into()));
    }
    sel(&local).map(|r| (r, "catalog".into()))
}

/// The reference range with its critical bounds filled in from a `critical`
/// range when it has none of its own.
fn with_critical(
    cat: &Catalog,
    s: &Subject,
    marker: &Marker,
    picked: Option<(Range, String)>,
) -> Option<(Range, String)> {
    let (mut r, src) = picked?;
    if r.critical_low.is_none() && r.critical_high.is_none() {
        if let Some((c, _)) = pick_range(cat, s, marker, RangeKind::Critical) {
            r.critical_low = c.low;
            r.critical_high = c.high;
        }
    }
    Some((r, src))
}

pub fn evaluate(row: &MeasurementRow, cat: &Catalog, unit_system: &str) -> Option<Evaluated> {
    let marker = cat.by_id(row.marker_id)?.clone();
    let wanted = cat.display_unit(&marker, unit_system, Some(&row.person));
    let (display_value, display_unit) = cat
        .conversions
        .convert(marker.id, row.value, &marker.unit, &wanted)
        .map_or_else(|_| (row.value, marker.unit.clone()), |v| (v, wanted));
    let age = age_at(row);
    let subject = Subject {
        person: &row.person,
        person_id: row.person_id,
        sex: row.sex.as_deref(),
        age,
        lab: row.lab.as_deref(),
        fasting: row.fasting,
        taken_at: Some(&row.taken_at),
    };
    let pick = |kind| pick_range(cat, &subject, &marker, kind).unzip();
    let (reference, ref_src) =
        with_critical(cat, &subject, &marker, pick_range(cat, &subject, &marker, RangeKind::Reference)).unzip();
    let (optimal, opt_src) = pick(RangeKind::Optimal);
    let (warn, warn_src) = pick(RangeKind::Warn);
    let q = row.qualifier.as_deref();
    let margin = cat.profiles.borderline_margin;
    Some(Evaluated {
        status: ranges::status(row.value, q, reference.as_ref(), warn.as_ref()),
        sources: [ref_src, opt_src, warn_src],
        ref_flag: reference.as_ref().map(|r| ranges::flag(row.value, q, r)),
        opt_flag: optimal.as_ref().map(|r| ranges::flag(row.value, q, r)),
        ref_level: reference.as_ref().map(|r| ranges::level(row.value, q, r, margin)),
        opt_level: optimal.as_ref().map(|r| ranges::level(row.value, q, r, margin)),
        flavor: cat.profiles.flavor_for(&row.person).map(str::to_string),
        row: row.clone(),
        marker,
        display_value,
        display_unit,
        reference,
        optimal,
        warn,
        age,
    })
}

/// Which range applied: its source (person, config, range set or catalog),
/// catalog set, sex and age band and other conditions. `id` is the database
/// id (`range list`, with `--person` for a person's range); ranges from the
/// built-in catalog, profile or range-set files have none.
fn range_set(r: Option<&Range>, source: Option<&str>) -> serde_json::Value {
    r.map_or(serde_json::Value::Null, |r| {
        json!({
            "id": (r.id != 0).then_some(r.id),
            "source": source,
            "set": r.set,
            "sex": r.sex,
            "age_min": r.age_min,
            "age_max": r.age_max,
            "note": r.note,
            "conditions": r.cond,
        })
    })
}

impl Evaluated {
    /// Overall flag for the configured range flavor.
    pub fn flag(&self, flavor: &str) -> Option<Flag> {
        match self.flavor.as_deref().unwrap_or(flavor) {
            "optimal" => self.opt_flag,
            "both" => match (self.ref_flag, self.opt_flag) {
                (Some(r), _) if r.is_out() => Some(r),
                (r, Some(o)) if o.is_out() => Some(o).or(r),
                (r, o) => r.or(o),
            },
            _ => self.ref_flag,
        }
    }

    /// Severity-aware classification for the configured range flavor
    /// (with `both`, the more severe of the two).
    pub fn level(&self, flavor: &str) -> Option<Level> {
        match self.flavor.as_deref().unwrap_or(flavor) {
            "optimal" => self.opt_level,
            "both" => match (self.ref_level, self.opt_level) {
                (Some(r), Some(o)) => Some(if o.severity() > r.severity() { o } else { r }),
                (r, o) => r.or(o),
            },
            _ => self.ref_level,
        }
    }

    pub fn is_flagged(&self, flavor: &str) -> bool {
        self.flag(flavor).is_some_and(Flag::is_out)
    }

    /// Convert a canonical-unit bound into the display unit.
    fn bound(&self, cat: &Catalog, v: Option<f64>) -> Option<f64> {
        v.map(|x| cat.conversions.convert(self.marker.id, x, &self.marker.unit, &self.display_unit).unwrap_or(x))
    }

    /// The stable JSON measurement record (see docs/json-schema.md).
    pub fn record(&self, cat: &Catalog, flavor: &str) -> Record {
        let r = &self.row;
        let flag_str = |f: Option<Flag>| f.map(Flag::as_str);
        let level_str = |l: Option<Level>| l.map(Level::as_str);
        let rec = json!({
            "id": r.id,
            "person": r.person,
            "marker": self.marker.slug,
            "marker_name": self.marker.name,
            "category": self.marker.category,
            "taken_at": r.taken_at,
            "qualifier": r.qualifier,
            "value": self.display_value,
            "unit": self.display_unit,
            "value_raw": r.value_raw,
            "unit_raw": r.unit_raw,
            "value_canonical": r.value,
            "unit_canonical": self.marker.unit,
            "ref_low": self.bound(cat, self.reference.as_ref().and_then(|x| x.low)),
            "ref_high": self.bound(cat, self.reference.as_ref().and_then(|x| x.high)),
            "ref_flag": flag_str(self.ref_flag),
            "opt_low": self.bound(cat, self.optimal.as_ref().and_then(|x| x.low)),
            "opt_high": self.bound(cat, self.optimal.as_ref().and_then(|x| x.high)),
            "opt_flag": flag_str(self.opt_flag),
            "flag": flag_str(self.flag(flavor)),
            "ref_level": level_str(self.ref_level),
            "opt_level": level_str(self.opt_level),
            "level": level_str(self.level(flavor)),
            "warn_low": self.bound(cat, self.warn.as_ref().and_then(|x| x.low)),
            "warn_high": self.bound(cat, self.warn.as_ref().and_then(|x| x.high)),
            "status": self.status.as_str(),
            "sex": r.sex,
            "age": self.age.map(|a| (a * 100.0).floor() / 100.0),
            "range_set": {
                "reference": range_set(self.reference.as_ref(), self.sources[0].as_deref()),
                "optimal": range_set(self.optimal.as_ref(), self.sources[1].as_deref()),
                "warn": range_set(self.warn.as_ref(), self.sources[2].as_deref()),
            },
            "lab": r.lab,
            "fasting": r.fasting,
            "note": r.note,
            "tags": r.tags,
            "batch": r.batch_id,
        });
        crate::output::to_record(&rec)
    }
}

pub const MEASUREMENT_TABLE: &[&str] =
    &["id", "person", "taken_at", "marker", "qualifier", "value", "unit", "ref_low", "ref_high", "flag", "lab"];

/// Evaluate a batch of rows.
pub fn evaluate_all(rows: &[MeasurementRow], cat: &Catalog, unit_system: &str) -> Vec<Evaluated> {
    rows.iter().filter_map(|r| evaluate(r, cat, unit_system)).collect()
}

/// Keep only the latest row per (person, marker). Input must be sorted by date ascending.
pub fn latest_only(rows: Vec<Evaluated>) -> Vec<Evaluated> {
    let mut seen = std::collections::HashSet::new();
    let mut out: Vec<Evaluated> =
        rows.into_iter().rev().filter(|e| seen.insert((e.row.person_id, e.row.marker_id))).collect();
    out.reverse();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{Conditions, MarkerCond, PersonAttrs};
    use crate::profiles::{LoadOpts, Profiles};

    fn marker(id: i64, slug: &str) -> Marker {
        Marker {
            id,
            slug: slug.into(),
            name: slug.into(),
            category: "metabolic".into(),
            unit: "mg/dL".into(),
            loinc: None,
            description: None,
            builtin: true,
            aliases: vec![],
        }
    }

    fn r(set: &str, kind: RangeKind, sex: &str, lo: f64, hi: f64, cond: Conditions) -> Range {
        Range {
            marker_id: 1,
            kind,
            sex: sex.into(),
            age_min: 0.0,
            age_max: 200.0,
            low: Some(lo),
            high: Some(hi),
            set: Some(set.into()),
            cond,
            ..Range::default()
        }
    }

    fn catalog(sets: &str) -> Catalog {
        let pregnant = Conditions { pregnant: Some(true), ..Default::default() };
        let diabetic_fasting = Conditions {
            fasting: Some(true),
            markers: vec![MarkerCond { marker: "hba1c".into(), op: ">".into(), value: 6.0 }],
            ..Default::default()
        };
        let mut cat = Catalog {
            markers: vec![marker(1, "glucose"), marker(2, "hba1c")],
            catalog_ranges: vec![
                r("clinical", RangeKind::Reference, "any", 70.0, 99.0, Conditions::default()),
                r("clinical", RangeKind::Reference, "female", 60.0, 95.0, pregnant),
                r("clinical", RangeKind::Reference, "any", 80.0, 130.0, diabetic_fasting),
                r("clinical", RangeKind::Critical, "any", 40.0, 400.0, Conditions::default()),
                r("optimal", RangeKind::Optimal, "any", 72.0, 90.0, Conditions::default()),
                r("blueprint", RangeKind::Optimal, "any", 0.0, 95.0, Conditions::default()),
            ],
            ..Catalog::default()
        };
        cat.person_attrs.insert(1, PersonAttrs { pregnant: Some(true), ..Default::default() });
        cat.condition_values.insert((1, "hba1c".into()), vec![("2024-01-01".into(), 6.5)]);
        let opts = LoadOpts { range_sets: sets.into(), ..LoadOpts::default() };
        Profiles::load(std::path::Path::new("/nonexistent/config.toml"), &opts).unwrap().apply(cat).unwrap()
    }

    fn subject<'a>(id: i64, sex: &'a str, fasting: bool, on: &'a str) -> Subject<'a> {
        Subject {
            person: "p",
            person_id: id,
            sex: Some(sex),
            age: Some(40.0),
            lab: None,
            fasting: Some(fasting),
            taken_at: Some(on),
        }
    }

    fn bounds(cat: &Catalog, s: &Subject, kind: RangeKind) -> Option<(f64, f64, String)> {
        let m = cat.markers[0].clone();
        pick_range(cat, s, &m, kind)
            .map(|(r, src)| (r.low.unwrap(), r.high.unwrap(), format!("{src}:{}", r.set.unwrap_or_default())))
    }

    #[test]
    fn the_most_specific_matching_catalog_range_wins() {
        let cat = catalog("");
        let b = |s: &Subject| bounds(&cat, s, RangeKind::Reference).map(|(l, h, _)| (l, h));
        assert_eq!(b(&subject(2, "male", true, "2024-02-01")), Some((70.0, 99.0)));
        assert_eq!(b(&subject(1, "female", false, "2024-02-01")), Some((60.0, 95.0)), "pregnant");
        // fasting with HbA1c > 6 carries two conditions: beats pregnancy (one)
        assert_eq!(b(&subject(1, "female", true, "2024-02-01")), Some((80.0, 130.0)));
        // no HbA1c on or before the draw yet
        assert_eq!(b(&subject(1, "female", true, "2023-12-31")), Some((60.0, 95.0)));
        let m = cat.markers[0].clone();
        let s = subject(2, "male", true, "2024-02-01");
        let picked = with_critical(&cat, &s, &m, pick_range(&cat, &s, &m, RangeKind::Reference)).unwrap().0;
        assert_eq!((picked.critical_low, picked.critical_high), (Some(40.0), Some(400.0)));
    }

    #[test]
    fn range_sets_choose_the_set_and_database_rows_override() {
        let s = subject(2, "male", false, "2024-02-01");
        let cat = catalog("");
        assert_eq!(bounds(&cat, &s, RangeKind::Optimal), Some((72.0, 90.0, "catalog:optimal".into())));
        let cat = catalog("blueprint,clinical,optimal");
        assert_eq!(bounds(&cat, &s, RangeKind::Optimal), Some((0.0, 95.0, "catalog:blueprint".into())));
        // a database row of the same sex / age band replaces the catalog's; conditional ones stay
        let mut cat = catalog("");
        cat.ranges.push(Range {
            id: 9,
            set: None,
            ..r("", RangeKind::Reference, "any", 65.0, 100.0, Conditions::default())
        });
        assert_eq!(bounds(&cat, &s, RangeKind::Reference), Some((65.0, 100.0, "catalog:".into())));
        let pregnant = subject(1, "female", false, "2024-02-01");
        assert_eq!(bounds(&cat, &pregnant, RangeKind::Reference).map(|b| (b.0, b.1)), Some((60.0, 95.0)));
    }
}
