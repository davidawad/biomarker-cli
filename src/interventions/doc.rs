//! Parsing `supplement-interactions/1` documents (health-cli
//! `research/supplements/<slug>/interactions.json`). Pure: no database access.

use serde_json::Value;

use super::interaction::{parse_kind, parse_severity, parse_target, parse_target_kind};
use super::{parse_rx_status, Ingredient};
use crate::error::{AppError, Result};
use crate::util::slugify;

pub const SCHEMA: &str = "supplement-interactions/1";

/// The intervention a document is about.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Subject {
    pub slug: String,
    pub name: Option<String>,
    pub aliases: Vec<String>,
    pub category: Option<String>,
    pub rx_status: Option<String>,
    pub classes: Vec<String>,
    pub ingredients: Vec<Ingredient>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeSpec {
    pub target_kind: String,
    pub target: String,
    pub kind: String,
    pub severity: String,
    pub mechanism: Option<String>,
    pub citations: Vec<Value>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Doc {
    pub subject: Subject,
    /// (index in `interactions`, parsed edge or error)
    pub edges: Vec<(usize, std::result::Result<EdgeSpec, String>)>,
    pub warnings: Vec<String>,
}

fn str_of<'a>(v: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|k| v.get(*k).and_then(Value::as_str)).map(str::trim).filter(|s| !s.is_empty())
}

fn strings(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::String(s)) => vec![s.trim().to_string()],
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).map(|s| s.trim().to_string()).collect(),
        _ => Vec::new(),
    }
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect()
}

fn ingredient(v: &Value) -> Result<Ingredient> {
    match v {
        Value::String(s) => Ingredient::parse(s),
        Value::Object(_) => {
            let name =
                str_of(v, &["name", "ingredient"]).ok_or_else(|| AppError::invalid("ingredient without a name"))?;
            Ok(Ingredient {
                name: name.to_lowercase(),
                amount: ["amount", "dose", "quantity"].iter().find_map(|k| v.get(*k).and_then(Value::as_f64)),
                unit: str_of(v, &["unit", "units"]).map(str::to_string),
            })
        }
        _ => Err(AppError::invalid("ingredient must be a string or an object")),
    }
}

fn subject(doc: &Value, fallback_slug: Option<&str>) -> Result<Subject> {
    let s = ["supplement", "subject", "intervention"].iter().find_map(|k| doc.get(*k));
    let obj = s.filter(|v| v.is_object());
    let slug = s
        .and_then(|_| obj.and_then(|o| str_of(o, &["slug", "id", "name"])))
        .or_else(|| doc.get("slug").and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty()))
        .or_else(|| s.and_then(Value::as_str))
        .or(fallback_slug)
        .map(slugify)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            AppError::invalid("document names no supplement (expected a `supplement` object with a `slug`)")
        })?;
    let Some(o) = obj else { return Ok(Subject { slug, ..Default::default() }) };
    Ok(Subject {
        slug,
        name: str_of(o, &["name", "display_name"])
            .or_else(|| doc.get("name").and_then(Value::as_str))
            .or_else(|| s.and_then(Value::as_str))
            .map(str::to_string),
        aliases: strings(o.get("aliases")),
        category: str_of(o, &["category", "type"]).map(str::to_lowercase),
        rx_status: str_of(o, &["rx_status", "rx", "status"]).map(parse_rx_status).transpose()?,
        classes: strings(o.get("classes").or_else(|| o.get("drug_classes"))).iter().map(|c| slugify(c)).collect(),
        ingredients: o
            .get("ingredients")
            .and_then(Value::as_array)
            .map(|a| a.iter().map(ingredient).collect::<Result<Vec<_>>>())
            .transpose()?
            .unwrap_or_default(),
        description: str_of(o, &["description", "summary"]).map(str::to_string),
    })
}

fn target(v: &Value) -> Result<(String, String)> {
    match v {
        Value::String(s) => parse_target(s),
        Value::Object(_) => {
            let kind = parse_target_kind(str_of(v, &["type", "kind", "target_kind"]).unwrap_or(""))?;
            let slug = str_of(v, &["slug", "id", "name"]).map(slugify).filter(|s| !s.is_empty());
            Ok((kind, slug.ok_or_else(|| AppError::invalid("interaction target has no slug, id or name"))?))
        }
        _ => Err(AppError::invalid("interaction target must be a string or an object")),
    }
}

fn citations(v: Option<&Value>) -> Vec<Value> {
    match v {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(a)) => a.clone(),
        Some(other) => vec![other.clone()],
    }
}

fn edge(e: &Value) -> Result<EdgeSpec> {
    let t = ["with", "target", "b"]
        .iter()
        .find_map(|k| e.get(*k))
        .ok_or_else(|| AppError::invalid("interaction has no target (`with`)"))?;
    let (target_kind, target) = match t {
        Value::String(s) => {
            let (explicit_kind, _) = super::interaction::parse_target(s)?;
            if s.split_once(':').is_some_and(|(prefix, _)| super::interaction::is_target_type_spelling(prefix)) {
                if let Some(sibling_kind) = str_of(e, &["with_kind", "target_kind"]) {
                    let sibling_kind = parse_target_kind(sibling_kind)?;
                    if sibling_kind != explicit_kind {
                        return Err(AppError::invalid(format!(
                            "interaction target '{s}' explicitly names type '{explicit_kind}', conflicting with sibling target type '{sibling_kind}'"
                        )));
                    }
                }
                super::interaction::parse_target(s)?
            } else {
                let kind = str_of(e, &["with_kind", "target_kind"]);
                let prefixed = kind.map(|k| format!("{k}:{s}"));
                super::interaction::parse_target(prefixed.as_deref().unwrap_or(s))?
            }
        }
        _ => target(t)?,
    };
    let kind = parse_kind(str_of(e, &["kind", "type", "relation"]).unwrap_or(""))?;
    Ok(EdgeSpec {
        target_kind,
        target,
        kind,
        severity: parse_severity(str_of(e, &["severity", "level"]).unwrap_or(""))?,
        mechanism: str_of(e, &["mechanism"]).map(str::to_string),
        citations: citations(e.get("citations").or_else(|| e.get("references"))),
        note: str_of(e, &["note", "notes", "management"]).map(str::to_string),
    })
}

/// Parse one document; `fallback_slug` (the parent directory name) names the
/// subject when the document does not.
pub fn parse_doc(doc: &Value, fallback_slug: Option<&str>) -> Result<Doc> {
    let mut warnings = Vec::new();
    match doc.get("schema").and_then(Value::as_str) {
        Some(s) if s.starts_with(SCHEMA) => {}
        Some(s) => return Err(AppError::invalid(format!("unsupported schema '{s}' (expected {SCHEMA})"))),
        None => warnings.push(format!("no `schema` field; assuming {SCHEMA}")),
    }
    let subject = subject(doc, fallback_slug)?;
    let edge_arrays: Vec<(&str, &Vec<Value>)> = ["interactions", "edges"]
        .iter()
        .filter_map(|key| doc.get(*key).and_then(Value::as_array).map(|a| (*key, a)))
        .collect();
    let mut edges = Vec::new();
    for (key, array) in edge_arrays {
        let start = edges.len();
        edges.extend(array.iter().enumerate().map(|(i, e)| (i, edge(e).map_err(|e| e.message))));
        if !array.is_empty() && !edges[start..].iter().any(|(_, e)| e.is_ok()) {
            warnings.push(format!("non-empty `{key}` interaction array yielded zero parsed edges"));
        }
    }
    Ok(Doc { subject, edges, warnings })
}

/// Parse a file's JSON: one document or an array of documents.
pub fn parse(v: &Value, fallback_slug: Option<&str>) -> Result<Vec<Doc>> {
    match v {
        Value::Array(a) => a.iter().map(|d| parse_doc(d, None)).collect(),
        Value::Object(_) => Ok(vec![parse_doc(v, fallback_slug)?]),
        _ => Err(AppError::invalid("expected a JSON object or array")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn full_document() {
        let d = parse_doc(
            &json!({
                "schema": "supplement-interactions/1",
                "supplement": {"slug": "Fish Oil", "name": "Fish oil", "aliases": ["omega-3"], "rx_status": "OTC",
                    "classes": ["Omega 3"], "ingredients": [{"name": "EPA", "amount": 500, "unit": "mg"}, "dha=250mg"]},
                "interactions": [
                    {"with": {"type": "class", "slug": "anticoagulant"}, "kind": "caution", "severity": "moderate",
                     "mechanism": "additive", "citations": "PMID:1"},
                    {"with": "vitamin-e", "kind": "synergistic"},
                    {"with": "planet:mars", "kind": "avoid"},
                    {"kind": "avoid"}
                ]
            }),
            None,
        )
        .unwrap();
        assert!(d.warnings.is_empty());
        assert_eq!(d.subject.slug, "fish-oil");
        assert_eq!(d.subject.rx_status.as_deref(), Some("otc"));
        assert_eq!(d.subject.classes, ["omega-3"]);
        assert_eq!(d.subject.ingredients[1].amount, Some(250.0));
        let ok: Vec<&EdgeSpec> = d.edges.iter().filter_map(|(_, e)| e.as_ref().ok()).collect();
        assert_eq!(ok[0].target_kind, "drug-class");
        assert_eq!(ok[0].citations, [json!("PMID:1")]);
        assert_eq!(
            (ok[1].target.as_str(), ok[1].kind.as_str(), ok[1].severity.as_str()),
            ("vitamin-e", "synergy", "unknown")
        );
        assert_eq!(d.edges.iter().filter(|(_, e)| e.is_err()).map(|(i, _)| *i).collect::<Vec<_>>(), [3]);
    }

    #[test]
    fn leniency_and_rejection() {
        let d = parse_doc(&json!({"interactions": []}), Some("Zinc")).unwrap();
        assert_eq!(d.subject.slug, "zinc");
        assert_eq!(d.warnings.len(), 1);
        assert_eq!(parse_doc(&json!({"subject": "Magnesium"}), None).unwrap().subject.slug, "magnesium");
        assert!(parse_doc(&json!({"schema": "other/2", "supplement": "x"}), None).is_err());
        assert!(parse_doc(&json!({"schema": SCHEMA}), None).is_err());
        assert_eq!(parse(&json!([{"supplement": "a"}, {"supplement": "b"}]), None).unwrap().len(), 2);
    }

    #[test]
    fn imports_real_edges_shape_and_slugifies_display_names() {
        let d = parse_doc(
            &json!({
                "schema": SCHEMA,
                "supplement": {"slug": "synthetic-supplement"},
                "edges": [
                    {"with": "thyroid hormone", "with_kind": "drug-class", "kind": "caution", "severity": "moderate", "mechanism": "synthetic", "evidence": "synthetic evidence", "citations": ["PMID:00000000"]},
                    {"with": "Vitamin D-3", "target_kind": "supplement", "kind": "synergy"},
                    {"with": "hypothyroidism", "with_kind": "condition", "kind": "monitor"},
                    {"with": "grapefruit", "with_kind": "food", "kind": "avoid"}
                ]
            }),
            None,
        )
        .unwrap();
        assert_eq!(d.edges.len(), 4);
        let parsed: Vec<&EdgeSpec> = d.edges.iter().map(|(_, e)| e.as_ref().unwrap()).collect();
        assert_eq!((parsed[0].target_kind.as_str(), parsed[0].target.as_str()), ("drug-class", "thyroid-hormone"));
        assert_eq!((parsed[1].target_kind.as_str(), parsed[1].target.as_str()), ("intervention", "vitamin-d-3"));
        assert_eq!((parsed[2].target_kind.as_str(), parsed[2].target.as_str()), ("condition", "hypothyroidism"));
        assert_eq!((parsed[3].target_kind.as_str(), parsed[3].target.as_str()), ("food", "grapefruit"));
    }

    #[test]
    fn explicit_target_prefix_wins_and_conflicts_are_invalid() {
        let d = parse_doc(
            &json!({"supplement": "synthetic", "edges": [
                {"with": "drug:warfarin", "with_kind": "drug", "kind": "caution"},
                {"with": "drug:warfarin", "with_kind": "food", "kind": "caution"}
            ]}),
            None,
        )
        .unwrap();
        assert_eq!(d.edges[0].1.as_ref().unwrap().target_kind, "intervention");
        let err = d.edges[1].1.as_ref().unwrap_err();
        assert!(err.contains("explicitly names type 'intervention'"));
        assert!(err.contains("conflicting with sibling target type 'food'"));
    }

    #[test]
    fn colon_in_target_names_is_preserved_with_sibling_kind() {
        let d = parse_doc(
            &json!({"supplement": "synthetic", "edges": [
                {"with": "HLA-B*35:01", "with_kind": "gene", "kind": "caution"},
                {"with": "omega-3 index / EPA", "with_kind": "lab-test", "kind": "caution"},
                {"with": "plasma amino acid profile (tyrosine, Phe", "with_kind": "lab", "kind": "caution"}
            ]}),
            None,
        )
        .unwrap();
        let targets: Vec<_> = d.edges.iter().map(|edge| edge.1.as_ref().unwrap()).collect();
        assert_eq!(targets[0].target_kind, "genotype");
        assert_eq!(targets[0].target, "hla-b-35-01");
        assert_eq!(targets[1].target_kind, "lab-test");
        assert_eq!(targets[1].target, "omega-3-index-epa");
        assert_eq!(targets[2].target_kind, "lab-test");
        assert_eq!(targets[2].target, "plasma-amino-acid-profile-tyrosine-phe");
    }

    #[test]
    fn parses_both_interaction_arrays_and_reports_each_invalid_entry() {
        let d = parse_doc(
            &json!({"supplement": "synthetic", "interactions": [
                {"with": "", "kind": "caution"}, {"with": "valid-one", "kind": "caution"}
            ], "edges": [
                {"with": "valid-two", "kind": "monitor"}, {"kind": "avoid"}
            ]}),
            None,
        )
        .unwrap();
        assert_eq!(d.edges.len(), 4);
        assert!(d.edges[0].1.is_err());
        assert!(d.edges[1].1.is_ok());
        assert!(d.edges[2].1.is_ok());
        assert!(d.edges[3].1.is_err());
    }

    #[test]
    fn blank_top_level_slug_does_not_mask_scalar_subject() {
        let d = parse_doc(&json!({"slug": "  ", "subject": "Magnesium"}), None).unwrap();
        assert_eq!(d.subject.slug, "magnesium");
    }
}
