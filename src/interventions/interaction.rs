//! Interaction edges between an intervention and another intervention, a drug
//! class, a condition, a lab test or a genotype; and the per-person grid.

use serde::Serialize;
use serde_json::Value;

use super::{Intervention, IvCatalog};
use crate::db::{int, opt_text, text, Db, Row, RowExt};
use crate::error::{AppError, Result};
use crate::store::{Catalog, Outcome};
use crate::util::slugify;

/// Relation kinds, most important first.
pub const KINDS: &[&str] = &["contraindicated", "avoid", "caution", "monitor", "lab-interference", "synergy"];
pub const SEVERITIES: &[&str] = &["major", "moderate", "minor", "unknown"];
pub const TARGET_KINDS: &[&str] = &["intervention", "drug-class", "condition", "food", "lab-test", "genotype"];

/// Normalise a relation kind (accepting common synonyms).
pub fn parse_kind(s: &str) -> Result<String> {
    let v = match slugify(s).as_str() {
        "contraindicated" | "contraindication" | "contra-indicated" => "contraindicated",
        "avoid" | "do-not-combine" => "avoid",
        "caution" | "warning" | "precaution" | "use-caution" => "caution",
        "monitor" | "monitoring" => "monitor",
        "lab-interference" | "interference" | "lab" | "assay-interference" => "lab-interference",
        "synergy" | "synergistic" | "beneficial" | "complementary" => "synergy",
        _ => return Err(AppError::invalid(format!("unknown interaction kind '{s}' ({})", KINDS.join("|")))),
    };
    Ok(v.to_string())
}

pub fn parse_severity(s: &str) -> Result<String> {
    let v = match slugify(s).as_str() {
        "" | "unknown" | "unspecified" => "unknown",
        "major" | "severe" | "high" | "serious" => "major",
        "moderate" | "medium" | "significant" => "moderate",
        "minor" | "low" | "mild" => "minor",
        _ => return Err(AppError::invalid(format!("unknown severity '{s}' ({})", SEVERITIES.join("|")))),
    };
    Ok(v.to_string())
}

pub fn parse_target_kind(s: &str) -> Result<String> {
    let v = match slugify(s).as_str() {
        "" | "intervention" | "supplement" | "drug" | "medication" => "intervention",
        "drug-class" | "class" | "drug-classes" => "drug-class",
        "condition" | "disease" => "condition",
        "food" => "food",
        "lab-test" | "lab" | "marker" | "test" | "biomarker" => "lab-test",
        "genotype" | "gene" | "snp" | "variant" => "genotype",
        _ => return Err(AppError::invalid(format!("unknown target type '{s}' ({})", TARGET_KINDS.join("|")))),
    };
    Ok(v.to_string())
}

/// Parse `slug` or `TYPE:slug` into (target kind, normalised slug).
pub fn parse_target(s: &str) -> Result<(String, String)> {
    let (kind, slug) = match s.split_once(':') {
        Some((k, v)) if is_target_type_spelling(k) => (parse_target_kind(k)?, v),
        Some(_) => ("intervention".to_string(), s),
        None => ("intervention".to_string(), s),
    };
    let slug = slugify(slug);
    if slug.is_empty() {
        return Err(AppError::invalid(format!("empty interaction target '{s}'")));
    }
    Ok((kind, slug))
}

pub fn is_target_type_spelling(s: &str) -> bool {
    matches!(
        slugify(s).as_str(),
        "intervention"
            | "drug"
            | "supplement"
            | "drug-class"
            | "class"
            | "condition"
            | "food"
            | "lab-test"
            | "lab"
            | "marker"
            | "genotype"
            | "gene"
            | "snp"
    )
}

/// Rank of a kind (higher is more important); unknown kinds rank 0.
pub fn kind_rank(k: &str) -> usize {
    KINDS.iter().position(|x| *x == k).map_or(0, |p| KINDS.len() - p)
}

pub fn severity_rank(s: &str) -> usize {
    SEVERITIES.iter().position(|x| *x == s).map_or(0, |p| SEVERITIES.len() - p)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Edge {
    pub id: i64,
    #[serde(skip)]
    pub a_id: i64,
    pub a: String,
    pub target_kind: String,
    pub target: String,
    pub kind: String,
    pub severity: String,
    pub mechanism: Option<String>,
    pub citations: Vec<Value>,
    pub note: Option<String>,
    pub source: Option<String>,
}

/// An edge to write (from the CLI or an import).
#[derive(Debug, Clone, PartialEq)]
pub struct NewEdge {
    pub a_id: i64,
    pub target_kind: String,
    pub target: String,
    pub kind: String,
    pub severity: String,
    pub mechanism: Option<String>,
    pub citations: Vec<Value>,
    pub note: Option<String>,
    pub source: Option<String>,
}

fn from_row(r: &Row) -> Edge {
    Edge {
        id: r.i(0).unwrap_or_default(),
        a_id: r.i(1).unwrap_or_default(),
        a: r.s(2).unwrap_or_default(),
        target_kind: r.s(3).unwrap_or_default(),
        target: r.s(4).unwrap_or_default(),
        kind: r.s(5).unwrap_or_default(),
        severity: r.s(6).unwrap_or_default(),
        mechanism: r.s(7),
        citations: r.s(8).and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        note: r.s(9),
        source: r.s(10),
    }
}

pub fn list(db: &Db) -> Result<Vec<Edge>> {
    Ok(db
        .query(
            "SELECT e.id, e.a_id, i.slug, e.b_kind, e.b_ref, e.kind, e.severity, e.mechanism, e.citations, e.note, e.source
             FROM interactions e JOIN interventions i ON i.id = e.a_id ORDER BY i.slug, e.b_kind, e.b_ref, e.kind",
            &[],
        )?
        .iter()
        .map(from_row)
        .collect())
}

/// Insert or update the edge identified by (a, target kind, target, kind).
pub fn upsert(db: &Db, e: &NewEdge) -> Result<(Outcome, i64)> {
    let cites = serde_json::to_string(&e.citations)?;
    let existing = db.query_opt(
        "SELECT id, severity, mechanism, citations, note FROM interactions WHERE a_id = ?1 AND b_kind = ?2 AND b_ref = ?3 AND kind = ?4",
        &[int(e.a_id), text(&e.target_kind), text(&e.target), text(&e.kind)],
    )?;
    match existing {
        Some(r) => {
            let id = r.i(0).unwrap_or_default();
            let same = r.s(1).as_deref() == Some(e.severity.as_str())
                && r.s(2) == e.mechanism
                && r.s(3).as_deref() == Some(cites.as_str())
                && r.s(4) == e.note;
            if same {
                return Ok((Outcome::Skipped, id));
            }
            db.execute(
                "UPDATE interactions SET severity = ?1, mechanism = ?2, citations = ?3, note = ?4, source = ?5 WHERE id = ?6",
                &[
                    text(&e.severity),
                    opt_text(e.mechanism.as_ref()),
                    text(&cites),
                    opt_text(e.note.as_ref()),
                    opt_text(e.source.as_ref()),
                    int(id),
                ],
            )?;
            Ok((Outcome::Replaced, id))
        }
        None => {
            db.execute(
                "INSERT INTO interactions (a_id, b_kind, b_ref, kind, severity, mechanism, citations, note, source, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                &[
                    int(e.a_id),
                    text(&e.target_kind),
                    text(&e.target),
                    text(&e.kind),
                    text(&e.severity),
                    opt_text(e.mechanism.as_ref()),
                    text(&cites),
                    opt_text(e.note.as_ref()),
                    opt_text(e.source.as_ref()),
                    text(crate::util::now_iso()),
                ],
            )?;
            Ok((Outcome::Inserted, db.last_insert_rowid()))
        }
    }
}

pub fn delete(db: &Db, id: i64) -> Result<usize> {
    db.execute("DELETE FROM interactions WHERE id = ?1", &[int(id)])
}

/// Display name of an edge's target when it resolves to a catalog entry.
pub fn target_name(e: &Edge, ivs: &IvCatalog, markers: &Catalog) -> Option<String> {
    match e.target_kind.as_str() {
        "intervention" => ivs.find(&e.target).map(|i| i.name.clone()),
        "lab-test" => markers.find(&e.target).map(|m| m.name.clone()),
        _ => None,
    }
}

/// Does `e` touch intervention `iv` (as subject, as a resolved target, or through one of its classes)?
pub fn touches(e: &Edge, iv: &Intervention, ivs: &IvCatalog) -> bool {
    e.a_id == iv.id
        || (e.target_kind == "intervention" && ivs.find(&e.target).is_some_and(|t| t.id == iv.id))
        || (e.target_kind == "drug-class" && iv.classes.iter().any(|c| slugify(c) == e.target))
}

/// Edges between `x` and `y`, each with the drug class it matched through (if any).
pub fn between<'a>(
    edges: &'a [Edge],
    x: &Intervention,
    y: &Intervention,
    ivs: &IvCatalog,
) -> Vec<(&'a Edge, Option<String>)> {
    let direct = |from: &Intervention, to: &Intervention, e: &Edge| -> Option<Option<String>> {
        if e.a_id != from.id {
            return None;
        }
        match e.target_kind.as_str() {
            "intervention" if ivs.find(&e.target).is_some_and(|t| t.id == to.id) => Some(None),
            "drug-class" if to.classes.iter().any(|c| slugify(c) == e.target) => {
                Some(Some(format!("drug-class:{}", e.target)))
            }
            _ => None,
        }
    };
    if x.id == y.id {
        return Vec::new();
    }
    edges.iter().filter_map(|e| direct(x, y, e).or_else(|| direct(y, x, e)).map(|via| (e, via))).collect()
}

/// One filled cell of the grid.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Cell {
    pub row: String,
    pub column: String,
    pub kind: String,
    pub severity: String,
    pub edges: Vec<Value>,
}

pub fn cell(edges: &[Edge], row: &Intervention, col: &Intervention, ivs: &IvCatalog) -> Option<Cell> {
    let hits = between(edges, row, col, ivs);
    let top = hits.iter().max_by_key(|(e, _)| (kind_rank(&e.kind), severity_rank(&e.severity)))?;
    Some(Cell {
        row: row.slug.clone(),
        column: col.slug.clone(),
        kind: top.0.kind.clone(),
        severity: top.0.severity.clone(),
        edges: hits
            .iter()
            .map(|(e, via)| serde_json::json!({"id": e.id, "kind": e.kind, "severity": e.severity, "via": via}))
            .collect(),
    })
}

/// Edges from `row` to targets that are not interventions among `columns`
/// (conditions, genotypes, lab tests, drug classes no column belongs to).
pub fn others<'a>(edges: &'a [Edge], row: &Intervention, columns: &[&Intervention]) -> Vec<&'a Edge> {
    edges
        .iter()
        .filter(|e| e.a_id == row.id)
        .filter(|e| match e.target_kind.as_str() {
            "intervention" => false,
            "drug-class" => !columns.iter().any(|c| c.id != row.id && c.classes.iter().any(|k| slugify(k) == e.target)),
            _ => true,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iv(id: i64, slug: &str, classes: &[&str]) -> Intervention {
        Intervention { id, classes: classes.iter().map(|c| c.to_string()).collect(), ..Intervention::new(slug) }
    }

    fn edge(id: i64, a_id: i64, target: &str, kind: &str, severity: &str) -> Edge {
        let (target_kind, target) = parse_target(target).unwrap();
        Edge {
            id,
            a_id,
            a: String::new(),
            target_kind,
            target,
            kind: kind.into(),
            severity: severity.into(),
            mechanism: None,
            citations: Vec::new(),
            note: None,
            source: None,
        }
    }

    #[test]
    fn parsing_and_synonyms() {
        assert_eq!(parse_kind("Contraindication").unwrap(), "contraindicated");
        assert_eq!(parse_kind("interference").unwrap(), "lab-interference");
        assert!(parse_kind("bad").is_err());
        assert_eq!(parse_severity("Severe").unwrap(), "major");
        assert_eq!(parse_severity("mild").unwrap(), "minor");
        assert_eq!(parse_target("Warfarin").unwrap(), ("intervention".into(), "warfarin".into()));
        assert_eq!(parse_target("class:Anti Coagulant").unwrap(), ("drug-class".into(), "anti-coagulant".into()));
        assert_eq!(parse_target("snp:MTHFR C677T").unwrap(), ("genotype".into(), "mthfr-c677t".into()));
        assert_eq!(parse_target("planet:mars").unwrap(), ("intervention".into(), "planet-mars".into()));
        assert!(parse_target("lab:").is_err());
        assert!(kind_rank("contraindicated") > kind_rank("caution"));
        assert!(kind_rank("lab-interference") > kind_rank("synergy"));
        assert!(severity_rank("major") > severity_rank("unknown"));
    }

    #[test]
    fn grid_cells_rank_and_match_classes() {
        let fish = iv(1, "fish-oil", &[]);
        let warf = iv(2, "warfarin", &["anticoagulant"]);
        let k2 = iv(3, "vitamin-k2", &[]);
        let ivs = IvCatalog { items: vec![fish.clone(), warf.clone(), k2.clone()] };
        let edges = vec![
            edge(1, 1, "drug-class:anticoagulant", "caution", "moderate"),
            edge(2, 3, "warfarin", "avoid", "major"),
            edge(3, 1, "vitamin-k2", "synergy", "unknown"),
            edge(4, 1, "lab-test:triglycerides", "lab-interference", "minor"),
        ];
        let c = cell(&edges, &fish, &warf, &ivs).unwrap();
        assert_eq!((c.kind.as_str(), c.edges[0]["via"].as_str()), ("caution", Some("drug-class:anticoagulant")));
        // Symmetric: the edge stored on vitamin-k2 shows from warfarin's row.
        assert_eq!(cell(&edges, &warf, &k2, &ivs).unwrap().kind, "avoid");
        assert_eq!(cell(&edges, &k2, &fish, &ivs).unwrap().kind, "synergy");
        assert!(cell(&edges, &fish, &fish, &ivs).is_none());
        // The anticoagulant edge is "other" only while no column is an anticoagulant.
        assert_eq!(others(&edges, &fish, &[&fish, &warf]).iter().map(|e| e.id).collect::<Vec<_>>(), [4]);
        assert_eq!(others(&edges, &fish, &[&fish]).iter().map(|e| e.id).collect::<Vec<_>>(), [1, 4]);
        assert!(touches(&edges[0], &warf, &ivs) && !touches(&edges[0], &k2, &ivs));
    }
}
