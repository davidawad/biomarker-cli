//! Versioned schema migrations.
//!
//! Each migration runs once, inside a transaction, and is recorded in
//! `schema_migrations`. `PRAGMA user_version` mirrors the latest version so
//! external tools can inspect it cheaply.

use crate::db::{int, text, Db, RowExt};
use crate::error::Result;

pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub apply: fn(&Db) -> Result<()>,
}

const SCHEMA_V1: &str = r"
CREATE TABLE IF NOT EXISTS people (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT NOT NULL UNIQUE,
    name        TEXT,
    sex         TEXT,
    dob         TEXT,
    notes       TEXT,
    tags        TEXT NOT NULL DEFAULT '[]',
    created_at  TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS markers (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    category    TEXT NOT NULL DEFAULT 'other',
    unit        TEXT NOT NULL,
    loinc       TEXT,
    description TEXT,
    builtin     INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS marker_aliases (
    alias       TEXT PRIMARY KEY,
    marker_id   INTEGER NOT NULL REFERENCES markers(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS units (
    symbol      TEXT PRIMARY KEY,
    system      TEXT NOT NULL DEFAULT 'both',
    description TEXT
);
CREATE TABLE IF NOT EXISTS unit_conversions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    marker_id   INTEGER NOT NULL DEFAULT 0,
    from_unit   TEXT NOT NULL,
    to_unit     TEXT NOT NULL,
    factor      REAL NOT NULL,
    offset      REAL NOT NULL DEFAULT 0,
    builtin     INTEGER NOT NULL DEFAULT 0,
    UNIQUE (marker_id, from_unit, to_unit)
);
CREATE TABLE IF NOT EXISTS ranges (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    marker_id   INTEGER NOT NULL REFERENCES markers(id) ON DELETE CASCADE,
    kind        TEXT NOT NULL,
    sex         TEXT NOT NULL DEFAULT 'any',
    age_min     REAL NOT NULL DEFAULT 0,
    age_max     REAL NOT NULL DEFAULT 200,
    low         REAL,
    high        REAL,
    note        TEXT,
    UNIQUE (marker_id, kind, sex, age_min, age_max)
);
CREATE TABLE IF NOT EXISTS import_batches (
    id          TEXT PRIMARY KEY,
    source      TEXT,
    format      TEXT,
    created_at  TEXT NOT NULL,
    row_count   INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS measurements (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id   INTEGER NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    marker_id   INTEGER NOT NULL REFERENCES markers(id),
    taken_at    TEXT NOT NULL,
    value_raw   REAL NOT NULL,
    unit_raw    TEXT NOT NULL,
    value       REAL NOT NULL,
    qualifier   TEXT,
    lab         TEXT,
    fasting     INTEGER,
    note        TEXT,
    tags        TEXT NOT NULL DEFAULT '[]',
    batch_id    TEXT,
    created_at  TEXT NOT NULL,
    UNIQUE (person_id, marker_id, taken_at)
);
CREATE INDEX IF NOT EXISTS idx_measurements_taken ON measurements(taken_at);
CREATE INDEX IF NOT EXISTS idx_measurements_marker ON measurements(marker_id);
CREATE INDEX IF NOT EXISTS idx_measurements_batch ON measurements(batch_id);
CREATE INDEX IF NOT EXISTS idx_ranges_marker ON ranges(marker_id);
";

/// Qualitative results (text, not numbers) and per-person reference ranges.
const SCHEMA_V3: &str = r"
CREATE TABLE IF NOT EXISTS observations (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id   INTEGER NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    marker_id   INTEGER NOT NULL REFERENCES markers(id),
    taken_at    TEXT NOT NULL,
    text        TEXT NOT NULL,
    flag        TEXT,
    range_low   REAL,
    range_high  REAL,
    note        TEXT,
    lab         TEXT,
    batch_id    TEXT,
    created_at  TEXT NOT NULL,
    UNIQUE (person_id, marker_id, taken_at)
);
CREATE INDEX IF NOT EXISTS idx_observations_taken ON observations(taken_at);
CREATE TABLE IF NOT EXISTS person_ranges (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id   INTEGER NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    marker_id   INTEGER NOT NULL REFERENCES markers(id) ON DELETE CASCADE,
    kind        TEXT NOT NULL,
    low         REAL,
    high        REAL,
    note        TEXT,
    UNIQUE (person_id, marker_id, kind)
);
";

pub const MIGRATIONS: &[Migration] = &[
    Migration { version: 1, name: "initial schema", apply: |db| db.execute_batch(SCHEMA_V1) },
    Migration { version: 2, name: "seed built-in marker catalog", apply: crate::seed::seed },
    Migration { version: 3, name: "observations and person ranges", apply: |db| db.execute_batch(SCHEMA_V3) },
    Migration { version: 4, name: "seed body and vitals markers", apply: crate::seed::seed_vitals },
    Migration { version: 5, name: "seed near-limit (warn) zones", apply: crate::seed::seed_warn_zones },
];

pub fn latest_version() -> i64 {
    MIGRATIONS.iter().map(|m| m.version).max().unwrap_or(0)
}

fn ensure_table(db: &Db) -> Result<()> {
    // Check first: a no-op CREATE would still count as a write and re-seal
    // an encrypted database on every read-only command.
    if db.query_opt("SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'schema_migrations'", &[])?.is_some() {
        return Ok(());
    }
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL
        );",
    )
}

/// Applied migrations as (version, name, applied_at).
pub fn applied(db: &Db) -> Result<Vec<(i64, String, String)>> {
    ensure_table(db)?;
    Ok(db
        .query("SELECT version, name, applied_at FROM schema_migrations ORDER BY version", &[])?
        .iter()
        .map(|r| (r.i(0).unwrap_or_default(), r.s(1).unwrap_or_default(), r.s(2).unwrap_or_default()))
        .collect())
}

pub fn current_version(db: &Db) -> Result<i64> {
    Ok(applied(db)?.iter().map(|a| a.0).max().unwrap_or(0))
}

/// Apply all pending migrations; returns the versions that were applied.
pub fn migrate(db: &Db) -> Result<Vec<i64>> {
    let current = current_version(db)?;
    if current > latest_version() {
        return Err(crate::error::AppError::db(format!(
            "database schema version {current} is newer than this binary supports ({})",
            latest_version()
        )));
    }
    MIGRATIONS
        .iter()
        .filter(|m| m.version > current)
        .map(|m| {
            db.transaction(|db| {
                (m.apply)(db)?;
                db.execute(
                    "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
                    &[int(m.version), text(m.name), text(crate::util::now_iso())],
                )?;
                Ok(m.version)
            })
        })
        .collect::<Result<Vec<_>>>()
        .and_then(|v| {
            if !v.is_empty() {
                db.execute(&format!("PRAGMA user_version = {}", latest_version()), &[])?;
            }
            Ok(v)
        })
}
