//! LOINC code -> marker mappings (`marker_loinc`, plus each marker's own `loinc`).

use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::db::{int, text, Db, RowExt};
use crate::error::{AppError, Result};
use crate::store::Marker;

/// LOINC code -> marker id. Rows of `marker_loinc` win; a marker's own `loinc`
/// column covers catalog markers added since the table was last written.
pub fn load(db: &Db) -> Result<HashMap<String, i64>> {
    let mut map: HashMap<String, i64> = HashMap::new();
    for r in db.query("SELECT loinc, marker_id FROM marker_loinc ORDER BY relation DESC, builtin DESC", &[])? {
        if let (Some(c), Some(m)) = (r.s(0), r.i(1)) {
            map.entry(c.trim().to_string()).or_insert(m);
        }
    }
    for r in db.query("SELECT loinc, id FROM markers WHERE loinc IS NOT NULL AND loinc <> '' ORDER BY id", &[])? {
        if let (Some(c), Some(m)) = (r.s(0), r.i(1)) {
            map.entry(c.trim().to_string()).or_insert(m);
        }
    }
    Ok(map)
}

/// The code written on export for each marker: its own `loinc`, else its first exact mapping.
pub fn primary_by_marker(db: &Db) -> Result<HashMap<i64, String>> {
    let mut map: HashMap<i64, String> = HashMap::new();
    for r in db.query("SELECT id, loinc FROM markers WHERE loinc IS NOT NULL AND loinc <> ''", &[])? {
        if let (Some(m), Some(c)) = (r.i(0), r.s(1)) {
            map.insert(m, c);
        }
    }
    for r in db
        .query("SELECT marker_id, loinc FROM marker_loinc WHERE relation = 'exact' ORDER BY builtin DESC, loinc", &[])?
    {
        if let (Some(m), Some(c)) = (r.i(0), r.s(1)) {
            map.entry(m).or_insert(c);
        }
    }
    Ok(map)
}

#[derive(Debug, Clone, Serialize)]
pub struct LoincRow {
    pub loinc: String,
    pub marker: String,
    pub relation: String,
    /// `catalog` (the marker's own code) or `custom` (added with `marker loinc add`).
    pub source: String,
}

pub fn list(db: &Db, marker_id: Option<i64>) -> Result<Vec<LoincRow>> {
    let mut rows: Vec<LoincRow> = vec![];
    let mut seen: HashSet<String> = HashSet::new();
    for r in db.query(
        "SELECT l.loinc, m.slug, l.relation, l.builtin, m.id FROM marker_loinc l JOIN markers m ON m.id = l.marker_id ORDER BY m.slug, l.loinc",
        &[],
    )? {
        let code = r.s(0).unwrap_or_default();
        seen.insert(code.clone());
        if marker_id.is_some_and(|m| r.i(4) != Some(m)) {
            continue;
        }
        rows.push(LoincRow {
            loinc: code,
            marker: r.s(1).unwrap_or_default(),
            relation: r.s(2).unwrap_or_default(),
            source: if r.b(3).unwrap_or(false) { "catalog" } else { "custom" }.into(),
        });
    }
    for r in
        db.query("SELECT loinc, slug, id FROM markers WHERE loinc IS NOT NULL AND loinc <> '' ORDER BY slug", &[])?
    {
        let code = r.s(0).unwrap_or_default();
        if seen.contains(&code) || marker_id.is_some_and(|m| r.i(2) != Some(m)) {
            continue;
        }
        rows.push(LoincRow {
            loinc: code,
            marker: r.s(1).unwrap_or_default(),
            relation: "exact".into(),
            source: "catalog".into(),
        });
    }
    rows.sort_by(|a, b| a.marker.cmp(&b.marker).then(a.loinc.cmp(&b.loinc)));
    Ok(rows)
}

/// A LOINC code is digits, a hyphen and a check digit: `2345-7`, `85354-9`.
pub fn valid_code(code: &str) -> bool {
    let c = code.trim();
    c.split_once('-').is_some_and(|(a, b)| {
        (1..=7).contains(&a.len())
            && a.chars().all(|x| x.is_ascii_digit())
            && b.len() == 1
            && b.chars().all(|x| x.is_ascii_digit())
    })
}

pub fn add(db: &Db, marker: &Marker, code: &str, relation: &str) -> Result<()> {
    let code = code.trim();
    if !valid_code(code) {
        return Err(AppError::invalid(format!(
            "'{code}' is not a LOINC code (expected digits, a hyphen and a check digit, e.g. 2345-7)"
        )));
    }
    if let Some(other) = load(db)?.get(code).filter(|id| **id != marker.id) {
        return Err(AppError::invalid(format!(
            "LOINC {code} already maps to another marker (id {other}); remove that mapping first"
        )));
    }
    db.execute(
        "INSERT OR REPLACE INTO marker_loinc (loinc, marker_id, relation, builtin) VALUES (?1, ?2, ?3, 0)",
        &[text(code), int(marker.id), text(relation)],
    )
    .map(|_| ())
}

/// Remove a custom mapping; catalog mappings belong to the catalog.
pub fn remove(db: &Db, marker: &Marker, code: &str) -> Result<()> {
    let code = code.trim();
    let row = db.query_opt(
        "SELECT builtin FROM marker_loinc WHERE loinc = ?1 AND marker_id = ?2",
        &[text(code), int(marker.id)],
    )?;
    match row {
        Some(r) if r.b(0).unwrap_or(false) => Err(AppError::invalid(format!(
            "LOINC {code} is the catalog's mapping for {} and cannot be removed",
            marker.slug
        ))),
        Some(_) => db.execute("DELETE FROM marker_loinc WHERE loinc = ?1", &[text(code)]).map(|_| ()),
        None => Err(AppError::not_found(format!("no custom LOINC mapping {code} for {}", marker.slug))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_loinc_codes() {
        assert!(valid_code("2345-7"));
        assert!(valid_code("85354-9"));
        assert!(valid_code(" 1751-7 "));
        assert!(!valid_code("2345"));
        assert!(!valid_code("abc-1"));
        assert!(!valid_code("2345-77"));
        assert!(!valid_code("12345678-1"));
    }
}
