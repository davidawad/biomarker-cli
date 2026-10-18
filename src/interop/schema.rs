//! Schema v8: tables for interchange (additive only).

use crate::db::{text, Db, RowExt};
use crate::error::{AppError, Result};

const SCHEMA_V8: &str = r"
CREATE TABLE IF NOT EXISTS interop_meta (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS marker_loinc (
    loinc       TEXT PRIMARY KEY,
    marker_id   INTEGER NOT NULL REFERENCES markers(id) ON DELETE CASCADE,
    relation    TEXT NOT NULL DEFAULT 'exact',
    builtin     INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS result_source (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id       INTEGER NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    kind            TEXT NOT NULL,
    target_id       INTEGER NOT NULL,
    source_format   TEXT NOT NULL,
    source_key      TEXT NOT NULL,
    resource_version TEXT,
    content_sha256  TEXT NOT NULL,
    previous_sha256 TEXT,
    revision        INTEGER NOT NULL DEFAULT 1,
    time_basis      TEXT,
    batch_id        TEXT,
    imported_at     TEXT NOT NULL,
    UNIQUE (person_id, source_key)
);
CREATE INDEX IF NOT EXISTS idx_result_source_target ON result_source(kind, target_id);
CREATE TABLE IF NOT EXISTS lab_result_meta (
    measurement_id  INTEGER PRIMARY KEY REFERENCES measurements(id) ON DELETE CASCADE,
    ranges_json     TEXT,
    ref_low         REAL,
    ref_high        REAL,
    ref_text        TEXT,
    interpretation  TEXT,
    lab_name        TEXT,
    specimen_type   TEXT,
    loinc           TEXT,
    status          TEXT
);
CREATE TABLE IF NOT EXISTS fhir_unmapped (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    batch_id    TEXT,
    person_id   INTEGER,
    loinc       TEXT,
    display     TEXT,
    value_text  TEXT,
    unit        TEXT,
    taken_at    TEXT,
    reason      TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_fhir_unmapped_batch ON fhir_unmapped(batch_id);
INSERT OR IGNORE INTO marker_loinc (loinc, marker_id, relation, builtin)
    SELECT loinc, id, 'exact', builtin FROM markers WHERE loinc IS NOT NULL AND loinc <> '';
";

/// Migration 8: the tables above, the LOINC backfill, and this database's identity.
pub fn migrate_v8(db: &Db) -> Result<()> {
    db.execute_batch(SCHEMA_V8)?;
    db_uuid(db).map(|_| ())
}

fn random_uuid() -> Result<String> {
    let mut b = [0u8; 16];
    getrandom::fill(&mut b).map_err(|e| AppError::db(format!("OS RNG: {e}")))?;
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h: String = b.iter().map(|x| format!("{x:02x}")).collect();
    Ok(format!("{}-{}-{}-{}-{}", &h[..8], &h[8..12], &h[12..16], &h[16..20], &h[20..]))
}

/// This database's random identity, created on first use. Exported
/// identifiers carry it so re-importing an export is idempotent.
pub fn db_uuid(db: &Db) -> Result<String> {
    if let Some(v) = db.query_opt("SELECT value FROM interop_meta WHERE key = 'db_uuid'", &[])?.and_then(|r| r.s(0)) {
        return Ok(v);
    }
    let id = random_uuid()?;
    db.execute("INSERT OR IGNORE INTO interop_meta (key, value) VALUES ('db_uuid', ?1)", &[text(&id)])?;
    Ok(id)
}
