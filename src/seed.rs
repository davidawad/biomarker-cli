//! Seeding the database from the built-in catalog ([`crate::catalog`]):
//! units, generic conversions, and each catalog marker with its aliases and
//! conversions. Catalog ranges are not stored: they are read from the
//! embedded catalog at run time. Ranges are informational only; labs differ.
//! Override them with `biomarker range set`, a range set or a profile.

use crate::catalog::MarkerDef;
use crate::db::{int, opt_real, opt_text, real, text, Db, RowExt};
use crate::error::Result;

/// (symbol, system, description)
pub const UNITS: &[(&str, &str, &str)] = &[
    ("mg/dL", "us", "milligrams per decilitre"),
    ("g/dL", "us", "grams per decilitre"),
    ("µg/dL", "us", "micrograms per decilitre"),
    ("ng/dL", "us", "nanograms per decilitre"),
    ("ng/mL", "us", "nanograms per millilitre"),
    ("pg/mL", "us", "picograms per millilitre"),
    ("µIU/mL", "us", "micro international units per millilitre"),
    ("mEq/L", "us", "milliequivalents per litre"),
    ("10^3/µL", "us", "thousands per microlitre"),
    ("10^6/µL", "us", "millions per microlitre"),
    ("mmol/L", "si", "millimoles per litre"),
    ("µmol/L", "si", "micromoles per litre"),
    ("nmol/L", "si", "nanomoles per litre"),
    ("pmol/L", "si", "picomoles per litre"),
    ("g/L", "si", "grams per litre"),
    ("µg/L", "si", "micrograms per litre"),
    ("mmol/mol", "si", "millimoles per mole (IFCC HbA1c)"),
    ("10^9/L", "si", "billions per litre"),
    ("10^12/L", "si", "trillions per litre"),
    ("L/L", "si", "litre per litre (fraction)"),
    ("µkat/L", "si", "microkatals per litre"),
    ("mg/L", "both", "milligrams per litre"),
    ("%", "both", "percent"),
    ("U/L", "both", "units per litre"),
    ("IU/L", "both", "international units per litre"),
    ("mIU/L", "both", "milli international units per litre"),
    ("mIU/mL", "both", "milli international units per millilitre"),
    ("IU/mL", "both", "international units per millilitre"),
    ("fL", "both", "femtolitres"),
    ("pg", "both", "picograms"),
    ("mL/min/1.73m²", "both", "estimated GFR"),
    ("ratio", "both", "dimensionless ratio"),
];

/// Conversions that apply to every marker: (from, to, factor). `to = from * factor`.
pub const GENERIC_CONVERSIONS: &[(&str, &str, f64)] = &[
    ("g/L", "g/dL", 0.1),
    ("g/L", "mg/dL", 100.0),
    ("mg/dL", "mg/L", 10.0),
    ("µg/L", "ng/mL", 1.0),
    ("10^9/L", "10^3/µL", 1.0),
    ("10^12/L", "10^6/µL", 1.0),
    ("mEq/L", "mmol/L", 1.0),
    ("µkat/L", "U/L", 60.0),
    ("IU/L", "U/L", 1.0),
    ("µIU/mL", "mIU/L", 1.0),
    ("mIU/mL", "IU/L", 1.0),
    ("L/L", "%", 100.0),
];

/// Units added with the body / vitals markers (schema v4).
pub const VITALS_UNITS: &[(&str, &str, &str)] = &[
    ("kg", "si", "kilograms"),
    ("lb", "us", "pounds"),
    ("kg/m²", "both", "kilograms per square metre"),
    ("years", "both", "years"),
    ("bpm", "both", "beats per minute"),
    ("ms", "both", "milliseconds"),
    ("mL/kg/min", "both", "millilitres of oxygen per kilogram per minute"),
    ("mmHg", "both", "millimetres of mercury"),
];

/// Every built-in unit symbol.
pub fn unit_symbols() -> Vec<&'static str> {
    UNITS.iter().chain(VITALS_UNITS).map(|u| u.0).collect()
}

/// Schema v4: body composition and vital-sign units (the markers come from the catalog).
pub fn seed_vitals(db: &Db) -> Result<()> {
    insert_units(db, VITALS_UNITS)?;
    seed_markers(db, &crate::catalog::builtin()?.markers)
}

/// Schema v5: the near-limit (warn) rows v0.7.2 seeded (hidden by v7: the
/// catalog's warn ranges apply instead).
pub fn seed_warn_zones(db: &Db) -> Result<()> {
    insert_legacy_rows(db, |kind| kind == "warn")
}

/// Insert the frozen v0.7.2 range rows of the kinds `want` accepts, for the
/// markers present, exactly as the old migrations did.
fn insert_legacy_rows(db: &Db, want: impl Fn(&str) -> bool) -> Result<()> {
    legacy_rows().filter(|r| want(r.1)).try_for_each(|(slug, kind, sex, low, high, note)| {
        db.execute(
            "INSERT OR IGNORE INTO ranges (marker_id, kind, sex, low, high, note)
             SELECT id, ?2, ?3, ?4, ?5, ?6 FROM markers WHERE slug = ?1",
            &[text(slug), text(kind), text(sex), opt_real(low), opt_real(high), opt_text(note)],
        )
        .map(|_| ())
    })
}

fn insert_units(db: &Db, units: &[(&str, &str, &str)]) -> Result<()> {
    units.iter().try_for_each(|(sym, sys, desc)| {
        db.execute(
            "INSERT OR IGNORE INTO units (symbol, system, description) VALUES (?1, ?2, ?3)",
            &[text(sym), text(sys), text(desc)],
        )
        .map(|_| ())
    })
}

/// Schema v2: units, generic conversions, the catalog markers and the
/// reference / optimal rows v0.7.2 seeded (hidden by v7).
pub fn seed(db: &Db) -> Result<()> {
    insert_units(db, UNITS)?;
    GENERIC_CONVERSIONS.iter().try_for_each(|(from, to, f)| {
        db.execute(
            "INSERT OR IGNORE INTO unit_conversions (marker_id, from_unit, to_unit, factor, offset, builtin)
             VALUES (0, ?1, ?2, ?3, 0, 1)",
            &[text(from), text(to), real(*f)],
        )
        .map(|_| ())
    })?;
    seed_markers(db, &crate::catalog::builtin()?.markers)?;
    insert_legacy_rows(db, |kind| kind != "warn")
}

fn seed_markers(db: &Db, markers: &[MarkerDef]) -> Result<()> {
    markers.iter().try_for_each(|m| seed_marker(db, m).map(|_| ()))
}

/// Insert a catalog marker (if its slug is free) with aliases and
/// conversions; returns its id.
fn seed_marker(db: &Db, m: &MarkerDef) -> Result<i64> {
    db.execute(
        "INSERT OR IGNORE INTO markers (slug, name, category, unit, loinc, description, builtin) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)",
        &[text(&m.slug), text(&m.name), text(&m.category), text(&m.unit), opt_text(m.loinc.as_ref()), opt_text(m.description.as_ref())],
    )?;
    db.execute("INSERT OR IGNORE INTO units (symbol, system) VALUES (?1, 'both')", &[text(&m.unit)])?;
    let id = db.query_scalar_i64("SELECT id FROM markers WHERE slug = ?1", &[text(&m.slug)])?;
    m.aliases.iter().try_for_each(|a| {
        db.execute(
            "INSERT OR IGNORE INTO marker_aliases (alias, marker_id) VALUES (?1, ?2)",
            &[text(a.to_lowercase()), int(id)],
        )
        .map(|_| ())
    })?;
    m.conversions.iter().try_for_each(|c| {
        db.execute("INSERT OR IGNORE INTO units (symbol, system) VALUES (?1, 'both')", &[text(&c.from)])?;
        db.execute(
            "INSERT OR IGNORE INTO unit_conversions (marker_id, from_unit, to_unit, factor, offset, builtin)
             VALUES (?1, ?2, ?3, ?4, ?5, 1)",
            &[int(id), text(&c.from), text(&m.unit), real(c.factor), real(c.offset)],
        )
        .map(|_| ())
    })?;
    Ok(id)
}

/// The markers v0.7.2 shipped. Migration 7 records them as seen even when
/// you removed one, so it is not seeded again.
const LEGACY_SLUGS: &[&str] = &[
    "albumin",
    "alp",
    "alt",
    "apob",
    "ast",
    "bilirubin-total",
    "biological-age",
    "bmi",
    "body-fat",
    "bp-diastolic",
    "bp-systolic",
    "bun",
    "calcium",
    "chloride",
    "co2",
    "cortisol",
    "creatinine",
    "dhea-s",
    "egfr",
    "estradiol",
    "ferritin",
    "folate",
    "free-t3",
    "free-t4",
    "fsh",
    "ggt",
    "glucose",
    "hba1c",
    "hdl-c",
    "hematocrit",
    "hemoglobin",
    "homocysteine",
    "hrv",
    "hscrp",
    "igf-1",
    "insulin",
    "iron",
    "ldl-c",
    "lh",
    "lpa",
    "lymphocytes-abs",
    "magnesium",
    "mch",
    "mchc",
    "mcv",
    "neutrophils-abs",
    "non-hdl-c",
    "platelets",
    "potassium",
    "prolactin",
    "psa",
    "rbc",
    "rdw",
    "resting-hr",
    "shbg",
    "sodium",
    "testosterone-free",
    "testosterone-total",
    "tibc",
    "total-cholesterol",
    "total-protein",
    "tpo-ab",
    "transferrin-saturation",
    "triglycerides",
    "tsh",
    "uric-acid",
    "vitamin-b12",
    "vitamin-d",
    "vo2max",
    "wbc",
    "weight",
];

/// The range rows v0.7.2 seeded: (slug, kind, sex, low, high, note).
type LegacyRow<'a> = (&'a str, &'a str, &'a str, Option<f64>, Option<f64>, Option<&'a str>);

fn legacy_rows() -> impl Iterator<Item = LegacyRow<'static>> {
    include_str!("legacy_ranges.tsv").lines().filter(|l| !l.starts_with('#') && !l.is_empty()).filter_map(|l| {
        let f: Vec<&str> = l.split('\t').collect();
        let num = |s: &str| s.parse::<f64>().ok();
        (f.len() == 6).then(|| (f[0], f[1], f[2], num(f[3]), num(f[4]), Some(f[5]).filter(|n| !n.is_empty())))
    })
}

/// Schema v7: the catalog as data. Adds person attributes, marks the
/// `ranges` rows that still hold v0.7.2's seeded values as built-in (the
/// catalog files replace them; rows you added or changed keep overriding),
/// and records which database marker each catalog slug was seeded as.
pub fn catalog_as_data(db: &Db) -> Result<()> {
    db.execute_batch(
        "ALTER TABLE people ADD COLUMN attributes TEXT NOT NULL DEFAULT '{}';
         ALTER TABLE ranges ADD COLUMN builtin INTEGER NOT NULL DEFAULT 0;
         CREATE TABLE IF NOT EXISTS catalog_seeded (
             slug       TEXT PRIMARY KEY,
             marker_id  INTEGER NOT NULL
         );",
    )?;
    legacy_rows().try_for_each(|(slug, kind, sex, low, high, note)| {
        db.execute(
            "UPDATE ranges SET builtin = 1
             WHERE marker_id = (SELECT id FROM markers WHERE slug = ?1) AND kind = ?2 AND sex = ?3
               AND age_min = 0 AND age_max = 200 AND low IS ?4 AND high IS ?5 AND note IS ?6",
            &[text(slug), text(kind), text(sex), opt_real(low), opt_real(high), opt_text(note)],
        )
        .map(|_| ())
    })?;
    for m in &crate::catalog::builtin()?.markers {
        let by_slug = db.query_opt("SELECT id FROM markers WHERE slug = ?1 AND builtin = 1", &[text(&m.slug)])?;
        // a renamed built-in marker is found by one of its catalog aliases
        let found = match by_slug {
            Some(r) => r.i(0),
            None => m.aliases.iter().find_map(|a| {
                db.query_opt(
                    "SELECT m.id FROM marker_aliases a JOIN markers m ON m.id = a.marker_id WHERE a.alias = ?1 AND m.builtin = 1",
                    &[text(a.to_lowercase())],
                )
                .ok()
                .flatten()
                .and_then(|r| r.i(0))
            }),
        };
        // 0: a v0.7.2 marker that was removed; anything else unseen is seeded by sync_catalog
        if let Some(id) = found.or_else(|| LEGACY_SLUGS.contains(&m.slug.as_str()).then_some(0)) {
            db.execute(
                "INSERT OR IGNORE INTO catalog_seeded (slug, marker_id) VALUES (?1, ?2)",
                &[text(&m.slug), int(id)],
            )?;
        }
    }
    Ok(())
}

/// Seed catalog markers this database has never seen (a newer binary's
/// additions). Writes nothing when there are none. A marker you removed
/// stays removed.
pub fn sync_catalog(db: &Db) -> Result<()> {
    let seen: std::collections::BTreeSet<String> =
        db.query("SELECT slug FROM catalog_seeded", &[])?.iter().filter_map(|r| r.s(0)).collect();
    let new: Vec<&MarkerDef> = crate::catalog::builtin()?.markers.iter().filter(|m| !seen.contains(&m.slug)).collect();
    if new.is_empty() {
        return Ok(());
    }
    db.transaction(|db| {
        new.iter().try_for_each(|m| {
            let id = seed_marker(db, m)?;
            db.execute(
                "INSERT OR IGNORE INTO catalog_seeded (slug, marker_id) VALUES (?1, ?2)",
                &[text(&m.slug), int(id)],
            )
            .map(|_| ())
        })
    })
}
