//! Data access: people, marker catalog, ranges, conversions, measurements.

use serde::Serialize;

use crate::db::{int, opt_bool, opt_real, opt_text, real, text, Db, Row, RowExt, Value};
use crate::error::{AppError, Result};
use crate::ranges::{Range, RangeKind};
use crate::units::{canonical_spelling, same_unit, unit_key, Conversion, ConversionSet};
use crate::util::{tags_from_db, tags_to_json};

// ---------------------------------------------------------------------------
// People
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Person {
    pub id: i64,
    pub slug: String,
    pub name: Option<String>,
    pub sex: Option<String>,
    pub dob: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
}

const PERSON_COLS: &str = "id, slug, name, sex, dob, notes, tags, created_at";

fn person_from(r: &Row) -> Person {
    Person {
        id: r.i(0).unwrap_or_default(),
        slug: r.s(1).unwrap_or_default(),
        name: r.s(2),
        sex: r.s(3),
        dob: r.s(4),
        notes: r.s(5),
        tags: tags_from_db(r.s(6)),
        created_at: r.s(7).unwrap_or_default(),
    }
}

pub fn list_people(db: &Db) -> Result<Vec<Person>> {
    Ok(db.query(&format!("SELECT {PERSON_COLS} FROM people ORDER BY slug"), &[])?.iter().map(person_from).collect())
}

pub fn find_person(db: &Db, slug: &str) -> Result<Option<Person>> {
    Ok(db
        .query_opt(&format!("SELECT {PERSON_COLS} FROM people WHERE slug = ?1"), &[text(slug.trim().to_lowercase())])?
        .map(|r| person_from(&r)))
}

pub fn get_person(db: &Db, slug: &str) -> Result<Person> {
    find_person(db, slug)?
        .ok_or_else(|| AppError::not_found(format!("no such person '{slug}' (add with: biomarker person add {slug})")))
}

pub fn insert_person(db: &Db, p: &Person) -> Result<Person> {
    db.execute(
        "INSERT INTO people (slug, name, sex, dob, notes, tags, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[
            text(&p.slug),
            opt_text(p.name.as_ref()),
            opt_text(p.sex.as_ref()),
            opt_text(p.dob.as_ref()),
            opt_text(p.notes.as_ref()),
            text(tags_to_json(&p.tags)),
            text(&p.created_at),
        ],
    )
    .map_err(|e| {
        if e.message.contains("UNIQUE") {
            AppError::invalid(format!("person '{}' already exists", p.slug))
        } else {
            e
        }
    })?;
    get_person(db, &p.slug)
}

pub fn update_person(db: &Db, p: &Person) -> Result<Person> {
    db.execute(
        "UPDATE people SET slug = ?1, name = ?2, sex = ?3, dob = ?4, notes = ?5, tags = ?6 WHERE id = ?7",
        &[
            text(&p.slug),
            opt_text(p.name.as_ref()),
            opt_text(p.sex.as_ref()),
            opt_text(p.dob.as_ref()),
            opt_text(p.notes.as_ref()),
            text(tags_to_json(&p.tags)),
            int(p.id),
        ],
    )
    .map_err(|e| {
        if e.message.contains("UNIQUE") {
            AppError::invalid(format!("person '{}' already exists", p.slug))
        } else {
            e
        }
    })?;
    get_person(db, &p.slug)
}

pub fn delete_person(db: &Db, id: i64) -> Result<usize> {
    db.transaction(|db| {
        let n = db.execute("DELETE FROM measurements WHERE person_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM observations WHERE person_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM person_ranges WHERE person_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM people WHERE id = ?1", &[int(id)])?;
        Ok(n)
    })
}

// ---------------------------------------------------------------------------
// Markers & catalog
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Marker {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub category: String,
    pub unit: String,
    pub loinc: Option<String>,
    pub description: Option<String>,
    pub builtin: bool,
    pub aliases: Vec<String>,
}

/// Everything needed to interpret measurements, loaded once per command.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub markers: Vec<Marker>,
    pub ranges: Vec<Range>,
    /// Person-specific ranges (`person_id` is set); they win over `ranges`.
    pub person_ranges: Vec<Range>,
    pub conversions: ConversionSet,
    /// (symbol, system)
    pub units: Vec<(String, String)>,
    /// Unit presets, range sets and person profiles (see [`crate::profiles`]).
    pub profiles: crate::profiles::Profiles,
}

impl Catalog {
    pub fn load(db: &Db) -> Result<Self> {
        let aliases = db.query("SELECT alias, marker_id FROM marker_aliases ORDER BY alias", &[])?;
        let markers = db
            .query(
                "SELECT id, slug, name, category, unit, loinc, description, builtin FROM markers ORDER BY category, slug",
                &[],
            )?
            .iter()
            .map(|r| {
                let id = r.i(0).unwrap_or_default();
                Marker {
                    id,
                    slug: r.s(1).unwrap_or_default(),
                    name: r.s(2).unwrap_or_default(),
                    category: r.s(3).unwrap_or_default(),
                    unit: r.s(4).unwrap_or_default(),
                    loinc: r.s(5),
                    description: r.s(6),
                    builtin: r.b(7).unwrap_or(false),
                    aliases: aliases
                        .iter()
                        .filter(|a| a.i(1) == Some(id))
                        .filter_map(|a| a.s(0))
                        .collect(),
                }
            })
            .collect();
        Ok(Self {
            markers,
            ranges: load_ranges(db)?,
            person_ranges: load_person_ranges(db)?,
            conversions: ConversionSet::new(load_conversions(db)?),
            units: db
                .query("SELECT symbol, system FROM units ORDER BY symbol", &[])?
                .iter()
                .map(|r| (r.s(0).unwrap_or_default(), r.s(1).unwrap_or_default()))
                .collect(),
            profiles: Default::default(),
        })
    }

    /// Resolve a marker by slug, alias, display name, or slugified name.
    pub fn find(&self, name: &str) -> Option<&Marker> {
        let n = name.trim().to_lowercase();
        let slug = crate::util::slugify(&n);
        self.markers
            .iter()
            .find(|m| m.slug == n)
            .or_else(|| self.markers.iter().find(|m| m.aliases.contains(&n)))
            .or_else(|| self.markers.iter().find(|m| m.name.to_lowercase() == n))
            .or_else(|| {
                self.markers
                    .iter()
                    .find(|m| m.slug == slug || m.aliases.iter().any(|a| crate::util::slugify(a) == slug))
            })
    }

    pub fn get(&self, name: &str) -> Result<&Marker> {
        self.find(name).ok_or_else(|| {
            AppError::not_found(format!(
                "unknown marker '{name}' (see: biomarker marker list; add with: biomarker marker add)"
            ))
        })
    }

    pub fn by_id(&self, id: i64) -> Option<&Marker> {
        self.markers.iter().find(|m| m.id == id)
    }

    pub fn unit_symbols(&self) -> Vec<String> {
        self.units.iter().map(|u| u.0.clone()).collect()
    }

    /// Canonical spelling of a unit string (e.g. `mg/dl` -> `mg/dL`).
    pub fn spell_unit(&self, u: &str) -> String {
        canonical_spelling(u, &self.unit_symbols())
    }

    pub fn unit_system(&self, u: &str) -> Option<&str> {
        let k = unit_key(u);
        self.units.iter().find(|(s, _)| unit_key(s) == k).map(|(_, sys)| sys.as_str())
    }

    /// Convert a raw value into the marker's canonical unit.
    pub fn to_canonical(&self, m: &Marker, value: f64, unit: &str) -> Result<f64> {
        self.conversions.convert(m.id, value, unit, &m.unit).map_err(|e| e.context(format!("marker {}", m.slug)))
    }

    /// Display unit for a marker under unit preset `system`, honouring the
    /// person's own profile (`person` is a slug).
    pub fn display_unit(&self, m: &Marker, system: &str, person: Option<&str>) -> String {
        let reachable = self.conversions.reachable(m.id, &m.unit);
        let chosen = self.profiles.unit_for(person, system, m).filter(|u| reachable.iter().any(|k| same_unit(k, u)));
        if let Some(u) = chosen {
            return self.spell_unit(&u);
        }
        let tag = self.profiles.system_tag(self.profiles.preset_name(person, system)).unwrap_or(system);
        if tag == "canonical" || matches!(self.unit_system(&m.unit), Some(s) if s == tag || s == "both") {
            return m.unit.clone();
        }
        reachable
            .into_iter()
            .skip(1)
            .find(|k| matches!(self.unit_system(k), Some(s) if s == tag))
            .map_or_else(|| m.unit.clone(), |k| self.spell_unit(&k))
    }

    pub fn ranges_for(&self, marker_id: i64) -> impl Iterator<Item = &Range> {
        self.ranges.iter().filter(move |r| r.marker_id == marker_id)
    }
}

pub fn insert_marker(db: &Db, m: &Marker) -> Result<i64> {
    db.transaction(|db| {
        db.execute(
            "INSERT INTO markers (slug, name, category, unit, loinc, description, builtin) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            &[text(&m.slug), text(&m.name), text(&m.category), text(&m.unit), opt_text(m.loinc.as_ref()), opt_text(m.description.as_ref())],
        )
        .map_err(|e| if e.message.contains("UNIQUE") { AppError::invalid(format!("marker '{}' already exists", m.slug)) } else { e })?;
        let id = db.query_scalar_i64("SELECT id FROM markers WHERE slug = ?1", &[text(&m.slug)])?;
        m.aliases.iter().try_for_each(|a| add_alias(db, id, a))?;
        Ok(id)
    })
}

pub fn update_marker(db: &Db, m: &Marker) -> Result<()> {
    db.execute(
        "UPDATE markers SET slug = ?1, name = ?2, category = ?3, unit = ?4, loinc = ?5, description = ?6 WHERE id = ?7",
        &[
            text(&m.slug),
            text(&m.name),
            text(&m.category),
            text(&m.unit),
            opt_text(m.loinc.as_ref()),
            opt_text(m.description.as_ref()),
            int(m.id),
        ],
    )
    .map(|_| ())
}

pub fn add_alias(db: &Db, marker_id: i64, alias: &str) -> Result<()> {
    let a = alias.trim().to_lowercase();
    if let Some(r) = db.query_opt("SELECT marker_id FROM marker_aliases WHERE alias = ?1", &[text(&a)])? {
        return if r.i(0) == Some(marker_id) {
            Ok(())
        } else {
            Err(AppError::invalid(format!("alias '{a}' already belongs to another marker")))
        };
    }
    db.execute("INSERT INTO marker_aliases (alias, marker_id) VALUES (?1, ?2)", &[text(a), int(marker_id)]).map(|_| ())
}

pub fn remove_alias(db: &Db, marker_id: i64, alias: &str) -> Result<usize> {
    db.execute(
        "DELETE FROM marker_aliases WHERE alias = ?1 AND marker_id = ?2",
        &[text(alias.trim().to_lowercase()), int(marker_id)],
    )
}

pub fn delete_marker(db: &Db, id: i64) -> Result<usize> {
    db.transaction(|db| {
        let n = db.execute("DELETE FROM measurements WHERE marker_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM marker_aliases WHERE marker_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM ranges WHERE marker_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM observations WHERE marker_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM person_ranges WHERE marker_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM unit_conversions WHERE marker_id = ?1", &[int(id)])?;
        db.execute("DELETE FROM markers WHERE id = ?1", &[int(id)])?;
        Ok(n)
    })
}

pub fn count_measurements_for_marker(db: &Db, id: i64) -> Result<i64> {
    db.query_scalar_i64("SELECT count(*) FROM measurements WHERE marker_id = ?1", &[int(id)])
}

// ---------------------------------------------------------------------------
// Ranges
// ---------------------------------------------------------------------------

pub fn load_ranges(db: &Db) -> Result<Vec<Range>> {
    Ok(db
        .query(
            "SELECT id, marker_id, kind, sex, age_min, age_max, low, high, note FROM ranges ORDER BY marker_id, kind, sex, age_min",
            &[],
        )?
        .iter()
        .map(|r| Range {
            id: r.i(0).unwrap_or_default(),
            marker_id: r.i(1).unwrap_or_default(),
            kind: r.s(2).and_then(|k| RangeKind::parse(&k)).unwrap_or(RangeKind::Reference),
            sex: r.s(3).unwrap_or_else(|| "any".into()),
            age_min: r.f(4).unwrap_or(0.0),
            age_max: r.f(5).unwrap_or(200.0),
            low: r.f(6),
            high: r.f(7),
            note: r.s(8),
            person_id: None,
            critical_low: None,
            critical_high: None,
        })
        .collect())
}

pub fn load_person_ranges(db: &Db) -> Result<Vec<Range>> {
    Ok(db
        .query("SELECT id, person_id, marker_id, kind, low, high, note FROM person_ranges ORDER BY person_id, marker_id, kind", &[])?
        .iter()
        .map(|r| Range {
            id: r.i(0).unwrap_or_default(),
            person_id: r.i(1),
            critical_low: None,
            critical_high: None,
            marker_id: r.i(2).unwrap_or_default(),
            kind: r.s(3).and_then(|k| RangeKind::parse(&k)).unwrap_or(RangeKind::Reference),
            sex: "any".into(),
            age_min: 0.0,
            age_max: 200.0,
            low: r.f(4),
            high: r.f(5),
            note: r.s(6),
        })
        .collect())
}

/// Insert or replace the person-specific range identified by (person, marker, kind).
pub fn upsert_person_range(db: &Db, person_id: i64, r: &Range) -> Result<()> {
    db.execute(
        "INSERT INTO person_ranges (person_id, marker_id, kind, low, high, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT (person_id, marker_id, kind) DO UPDATE SET low = excluded.low, high = excluded.high, note = excluded.note",
        &[int(person_id), int(r.marker_id), text(r.kind.as_str()), opt_real(r.low), opt_real(r.high), opt_text(r.note.as_ref())],
    )
    .map(|_| ())
}

pub fn delete_person_range(db: &Db, id: i64) -> Result<usize> {
    db.execute("DELETE FROM person_ranges WHERE id = ?1", &[int(id)])
}

/// Insert or replace the range identified by (marker, kind, sex, age band).
pub fn upsert_range(db: &Db, r: &Range) -> Result<()> {
    db.execute(
        "INSERT INTO ranges (marker_id, kind, sex, age_min, age_max, low, high, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT (marker_id, kind, sex, age_min, age_max) DO UPDATE SET low = excluded.low, high = excluded.high, note = excluded.note",
        &[
            int(r.marker_id),
            text(r.kind.as_str()),
            text(&r.sex),
            real(r.age_min),
            real(r.age_max),
            opt_real(r.low),
            opt_real(r.high),
            opt_text(r.note.as_ref()),
        ],
    )
    .map(|_| ())
}

pub fn delete_range(db: &Db, id: i64) -> Result<usize> {
    db.execute("DELETE FROM ranges WHERE id = ?1", &[int(id)])
}

// ---------------------------------------------------------------------------
// Conversions
// ---------------------------------------------------------------------------

pub fn load_conversions(db: &Db) -> Result<Vec<Conversion>> {
    Ok(db
        .query(
            "SELECT id, marker_id, from_unit, to_unit, factor, offset FROM unit_conversions ORDER BY marker_id, from_unit, to_unit",
            &[],
        )?
        .iter()
        .map(|r| Conversion {
            id: r.i(0).unwrap_or_default(),
            marker_id: r.i(1).unwrap_or_default(),
            from_unit: r.s(2).unwrap_or_default(),
            to_unit: r.s(3).unwrap_or_default(),
            factor: r.f(4).unwrap_or(1.0),
            offset: r.f(5).unwrap_or(0.0),
        })
        .collect())
}

pub fn upsert_conversion(db: &Db, c: &Conversion) -> Result<()> {
    db.execute(
        "INSERT INTO unit_conversions (marker_id, from_unit, to_unit, factor, offset) VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT (marker_id, from_unit, to_unit) DO UPDATE SET factor = excluded.factor, offset = excluded.offset",
        &[int(c.marker_id), text(&c.from_unit), text(&c.to_unit), real(c.factor), real(c.offset)],
    )
    .map(|_| ())
}

pub fn ensure_unit(db: &Db, symbol: &str, system: &str) -> Result<()> {
    db.execute("INSERT OR IGNORE INTO units (symbol, system) VALUES (?1, ?2)", &[text(symbol), text(system)])
        .map(|_| ())
}

// ---------------------------------------------------------------------------
// Measurements
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct NewMeasurement {
    pub person_id: i64,
    pub marker_id: i64,
    pub taken_at: String,
    pub value_raw: f64,
    pub unit_raw: String,
    pub value: f64,
    pub qualifier: Option<String>,
    pub lab: Option<String>,
    pub fasting: Option<bool>,
    pub note: Option<String>,
    pub tags: Vec<String>,
    pub batch_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dedupe {
    Skip,
    Replace,
    Error,
}

impl Dedupe {
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "skip" => Ok(Self::Skip),
            "replace" => Ok(Self::Replace),
            "error" => Ok(Self::Error),
            _ => Err(AppError::usage(format!("bad dedupe policy '{s}' (skip|replace|error)"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Inserted,
    Replaced,
    Skipped,
}

pub fn existing_measurement(db: &Db, person_id: i64, marker_id: i64, taken_at: &str) -> Result<Option<i64>> {
    Ok(db
        .query_opt(
            "SELECT id FROM measurements WHERE person_id = ?1 AND marker_id = ?2 AND taken_at = ?3",
            &[int(person_id), int(marker_id), text(taken_at)],
        )?
        .and_then(|r| r.i(0)))
}

fn measurement_params(m: &NewMeasurement) -> Vec<Value> {
    vec![
        int(m.person_id),
        int(m.marker_id),
        text(&m.taken_at),
        real(m.value_raw),
        text(&m.unit_raw),
        real(m.value),
        opt_text(m.qualifier.as_ref()),
        opt_text(m.lab.as_ref()),
        opt_bool(m.fasting),
        opt_text(m.note.as_ref()),
        text(tags_to_json(&m.tags)),
        opt_text(m.batch_id.as_ref()),
        text(crate::util::now_iso()),
    ]
}

/// Insert a measurement honouring the duplicate policy. Returns the outcome and row id.
pub fn insert_measurement(db: &Db, m: &NewMeasurement, policy: Dedupe) -> Result<(Outcome, i64)> {
    match (existing_measurement(db, m.person_id, m.marker_id, &m.taken_at)?, policy) {
        (Some(id), Dedupe::Skip) => Ok((Outcome::Skipped, id)),
        (Some(_), Dedupe::Error) => {
            Err(AppError::invalid(format!("duplicate measurement at {} (use --dedupe skip|replace)", m.taken_at)))
        }
        (Some(id), Dedupe::Replace) => {
            let mut p = measurement_params(m);
            p.push(int(id));
            db.execute(
                "UPDATE measurements SET person_id = ?1, marker_id = ?2, taken_at = ?3, value_raw = ?4, unit_raw = ?5, value = ?6,
                 qualifier = ?7, lab = ?8, fasting = ?9, note = ?10, tags = ?11, batch_id = ?12, created_at = ?13 WHERE id = ?14",
                &p,
            )?;
            Ok((Outcome::Replaced, id))
        }
        (None, _) => {
            db.execute(
                "INSERT INTO measurements (person_id, marker_id, taken_at, value_raw, unit_raw, value, qualifier, lab, fasting, note, tags, batch_id, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                &measurement_params(m),
            )?;
            Ok((Outcome::Inserted, db.last_insert_rowid()))
        }
    }
}

pub fn delete_measurement(db: &Db, id: i64) -> Result<usize> {
    db.execute("DELETE FROM measurements WHERE id = ?1", &[int(id)])
}

/// A measurement joined with its person and marker.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementRow {
    pub id: i64,
    pub person_id: i64,
    pub person: String,
    pub sex: Option<String>,
    pub dob: Option<String>,
    pub marker_id: i64,
    pub taken_at: String,
    pub value_raw: f64,
    pub unit_raw: String,
    pub value: f64,
    pub qualifier: Option<String>,
    pub lab: Option<String>,
    pub fasting: Option<bool>,
    pub note: Option<String>,
    pub tags: Vec<String>,
    pub batch_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub person_ids: Vec<i64>,
    pub marker_ids: Vec<i64>,
    pub categories: Vec<String>,
    /// Inclusive lower bound, `YYYY-MM-DD`.
    pub from: Option<String>,
    /// Inclusive upper bound, `YYYY-MM-DD`.
    pub to: Option<String>,
    pub lab: Option<String>,
    pub batch: Option<String>,
    pub tag: Option<String>,
}

fn placeholders(start: usize, n: usize) -> String {
    (start..start + n).map(|i| format!("?{i}")).collect::<Vec<_>>().join(", ")
}

/// Build a WHERE clause + params for the filter (pure).
pub fn filter_sql(f: &Filter) -> (String, Vec<Value>) {
    let mut clauses: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();
    let add_in = |col: &str, vals: Vec<Value>, clauses: &mut Vec<String>, params: &mut Vec<Value>| {
        if !vals.is_empty() {
            clauses.push(format!("{col} IN ({})", placeholders(params.len() + 1, vals.len())));
            params.extend(vals);
        }
    };
    add_in("m.person_id", f.person_ids.iter().map(|i| int(*i)).collect(), &mut clauses, &mut params);
    add_in("m.marker_id", f.marker_ids.iter().map(|i| int(*i)).collect(), &mut clauses, &mut params);
    add_in("k.category", f.categories.iter().map(|c| text(c.to_lowercase())).collect(), &mut clauses, &mut params);
    let mut cmp = |sql: &str, v: Option<Value>| {
        if let Some(v) = v {
            params.push(v);
            clauses.push(sql.replace('#', &format!("?{}", params.len())));
        }
    };
    cmp("m.taken_at >= #", f.from.as_ref().map(text));
    // `to` is a date; include any time on that day.
    cmp("m.taken_at < #", f.to.as_ref().map(|t| text(format!("{t}~"))));
    cmp("m.lab = #", f.lab.as_ref().map(text));
    cmp("m.batch_id = #", f.batch.as_ref().map(text));
    // tags are stored as a JSON array; match the exact JSON-quoted tag
    cmp("instr(m.tags, #) > 0", f.tag.as_ref().map(|t| text(serde_json::to_string(t).unwrap_or_default())));
    let where_ = if clauses.is_empty() { String::new() } else { format!("WHERE {}", clauses.join(" AND ")) };
    (where_, params)
}

pub fn query_measurements(db: &Db, f: &Filter) -> Result<Vec<MeasurementRow>> {
    let (where_, params) = filter_sql(f);
    let sql = format!(
        "SELECT m.id, m.person_id, p.slug, p.sex, p.dob, m.marker_id, m.taken_at, m.value_raw, m.unit_raw, m.value,
                m.qualifier, m.lab, m.fasting, m.note, m.tags, m.batch_id
         FROM measurements m
         JOIN people p ON p.id = m.person_id
         JOIN markers k ON k.id = m.marker_id
         {where_}
         ORDER BY m.taken_at, p.slug, k.category, k.slug"
    );
    Ok(db
        .query(&sql, &params)?
        .iter()
        .map(|r| MeasurementRow {
            id: r.i(0).unwrap_or_default(),
            person_id: r.i(1).unwrap_or_default(),
            person: r.s(2).unwrap_or_default(),
            sex: r.s(3),
            dob: r.s(4),
            marker_id: r.i(5).unwrap_or_default(),
            taken_at: r.s(6).unwrap_or_default(),
            value_raw: r.f(7).unwrap_or_default(),
            unit_raw: r.s(8).unwrap_or_default(),
            value: r.f(9).unwrap_or_default(),
            qualifier: r.s(10),
            lab: r.s(11),
            fasting: r.b(12),
            note: r.s(13),
            tags: tags_from_db(r.s(14)),
            batch_id: r.s(15),
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Observations (qualitative results)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct NewObservation {
    pub person_id: i64,
    pub marker_id: i64,
    pub taken_at: String,
    pub text: String,
    /// `abnormal` when the source says so; otherwise unset.
    pub flag: Option<String>,
    pub range_low: Option<f64>,
    pub range_high: Option<f64>,
    pub note: Option<String>,
    pub lab: Option<String>,
    pub batch_id: Option<String>,
}

/// Insert a qualitative observation honouring the duplicate policy.
pub fn insert_observation(db: &Db, o: &NewObservation, policy: Dedupe) -> Result<Outcome> {
    let existing = db
        .query_opt(
            "SELECT id FROM observations WHERE person_id = ?1 AND marker_id = ?2 AND taken_at = ?3",
            &[int(o.person_id), int(o.marker_id), text(&o.taken_at)],
        )?
        .and_then(|r| r.i(0));
    let params = vec![
        int(o.person_id),
        int(o.marker_id),
        text(&o.taken_at),
        text(&o.text),
        opt_text(o.flag.as_ref()),
        opt_real(o.range_low),
        opt_real(o.range_high),
        opt_text(o.note.as_ref()),
        opt_text(o.lab.as_ref()),
        opt_text(o.batch_id.as_ref()),
        text(crate::util::now_iso()),
    ];
    match (existing, policy) {
        (Some(_), Dedupe::Skip) => Ok(Outcome::Skipped),
        (Some(_), Dedupe::Error) => {
            Err(AppError::invalid(format!("duplicate observation at {} (use --dedupe skip|replace)", o.taken_at)))
        }
        (Some(id), Dedupe::Replace) => {
            let mut p = params;
            p.push(int(id));
            db.execute(
                "UPDATE observations SET person_id = ?1, marker_id = ?2, taken_at = ?3, text = ?4, flag = ?5, range_low = ?6,
                 range_high = ?7, note = ?8, lab = ?9, batch_id = ?10, created_at = ?11 WHERE id = ?12",
                &p,
            )?;
            Ok(Outcome::Replaced)
        }
        (None, _) => {
            db.execute(
                "INSERT INTO observations (person_id, marker_id, taken_at, text, flag, range_low, range_high, note, lab, batch_id, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                &params,
            )?;
            Ok(Outcome::Inserted)
        }
    }
}

/// An observation joined with its person and marker.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObservationRow {
    pub id: i64,
    pub person: String,
    pub marker: String,
    pub marker_name: String,
    pub category: String,
    pub taken_at: String,
    pub text: String,
    pub flag: Option<String>,
    pub range_low: Option<f64>,
    pub range_high: Option<f64>,
    pub note: Option<String>,
    pub lab: Option<String>,
    pub batch: Option<String>,
}

/// Observations matching a measurement filter (the tag filter does not apply).
pub fn query_observations(db: &Db, f: &Filter) -> Result<Vec<ObservationRow>> {
    let (where_, params) = filter_sql(&Filter { tag: None, ..f.clone() });
    let sql = format!(
        "SELECT m.id, p.slug, k.slug, k.name, k.category, m.taken_at, m.text, m.flag, m.range_low, m.range_high,
                m.note, m.lab, m.batch_id
         FROM observations m
         JOIN people p ON p.id = m.person_id
         JOIN markers k ON k.id = m.marker_id
         {where_}
         ORDER BY m.taken_at, p.slug, k.category, k.slug"
    );
    Ok(db
        .query(&sql, &params)?
        .iter()
        .map(|r| ObservationRow {
            id: r.i(0).unwrap_or_default(),
            person: r.s(1).unwrap_or_default(),
            marker: r.s(2).unwrap_or_default(),
            marker_name: r.s(3).unwrap_or_default(),
            category: r.s(4).unwrap_or_default(),
            taken_at: r.s(5).unwrap_or_default(),
            text: r.s(6).unwrap_or_default(),
            flag: r.s(7),
            range_low: r.f(8),
            range_high: r.f(9),
            note: r.s(10),
            lab: r.s(11),
            batch: r.s(12),
        })
        .collect())
}

pub fn record_batch(db: &Db, id: &str, source: &str, format: &str, rows: usize) -> Result<()> {
    db.execute(
        "INSERT INTO import_batches (id, source, format, created_at, row_count) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[text(id), text(source), text(format), text(crate::util::now_iso()), int(rows as i64)],
    )
    .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_sql_numbers_params() {
        let f = Filter {
            person_ids: vec![1, 2],
            marker_ids: vec![7],
            from: Some("2024-01-01".into()),
            to: Some("2024-12-31".into()),
            ..Default::default()
        };
        let (w, p) = filter_sql(&f);
        assert_eq!(w, "WHERE m.person_id IN (?1, ?2) AND m.marker_id IN (?3) AND m.taken_at >= ?4 AND m.taken_at < ?5");
        assert_eq!(p.len(), 5);
        assert_eq!(filter_sql(&Filter::default()).0, "");
    }
}
