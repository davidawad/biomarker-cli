//! The built-in marker catalog as data: one TOML file per marker under
//! `catalog/markers/<slug>.toml` (embedded at build time by build.rs), plus
//! `catalog/sets.toml` (range sets), `catalog/conditions.toml` (named
//! conditions for `when.conditions`) and `catalog/universe.toml` (every
//! biomarker the catalog should eventually cover).
//!
//! Markers, aliases and conversions are seeded into the database
//! ([`crate::seed`]); catalog ranges are read from here at run time, so a new
//! binary brings its ranges with it. The file format is documented in
//! docs/json-schema.md (JSON Schema in docs/marker-file.schema.json).

pub mod cond;
pub mod docs;
pub mod lint;

use std::path::Path;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ranges::{Range, RangeKind};
pub use cond::{Conditions, Facts, MarkerCond, PersonAttrs};

mod files {
    include!(concat!(env!("OUT_DIR"), "/catalog_files.rs"));
}

pub const SETS_TOML: &str = include_str!("../../catalog/sets.toml");
pub const CONDITIONS_TOML: &str = include_str!("../../catalog/conditions.toml");
pub const UNIVERSE_TOML: &str = include_str!("../../catalog/universe.toml");
pub const SCHEMA_JSON: &str = include_str!("../../docs/marker-file.schema.json");

pub const KINDS: &[&str] = &["reference", "optimal", "warn", "critical"];
pub const EVIDENCE: &[&str] =
    &["guideline", "reference-interval-study", "cohort", "rct", "consensus", "lab-convention"];
pub const PRIORITIES: &[&str] = &["core", "common", "specialist"];
/// The set of a range that names none.
pub const DEFAULT_SET: &str = "clinical";
/// `range_sets` when not configured.
pub const DEFAULT_SETS: &[&str] = &["clinical", "optimal"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Panel {
    pub slug: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default, rename = "member")]
    pub members: Vec<PanelMember>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PanelMember {
    pub marker: String,
    pub role: PanelRole,
    pub reason: String,
    pub term: Option<String>,
    #[serde(default)]
    pub citations: Vec<Citation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelRole {
    Core,
    Extended,
}

fn default_set() -> String {
    DEFAULT_SET.into()
}

fn default_kind() -> String {
    "reference".into()
}

/// One `catalog/markers/<slug>.toml`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkerDef {
    pub slug: String,
    pub name: String,
    pub category: String,
    /// Canonical unit: values and range bounds are stored in it.
    pub unit: String,
    pub loinc: Option<String>,
    pub specimen: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Slugs of related markers.
    #[serde(default)]
    pub related: Vec<String>,
    /// Why the marker has no `clinical` reference range (e.g. none is established).
    pub no_reference: Option<String>,
    #[serde(default, rename = "conversion")]
    pub conversions: Vec<ConversionDef>,
    #[serde(default, rename = "range")]
    pub ranges: Vec<RangeDef>,
}

/// `[[conversion]]`: `canonical = from * factor + offset`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversionDef {
    pub from: String,
    pub factor: f64,
    #[serde(default)]
    pub offset: f64,
}

/// `[[range]]` of a marker file. Bounds are in the marker's canonical unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RangeDef {
    #[serde(default = "default_set")]
    pub set: String,
    #[serde(default = "default_kind")]
    pub kind: String,
    pub low: Option<f64>,
    pub high: Option<f64>,
    #[serde(default)]
    pub when: When,
    pub population: Option<String>,
    pub evidence: Option<String>,
    #[serde(default)]
    pub citations: Vec<Citation>,
    pub note: Option<String>,
    /// Carried over from the hard-coded catalog without a citation: reported
    /// as an uncited warning instead of an error.
    #[serde(default)]
    pub legacy: bool,
}

impl Default for RangeDef {
    fn default() -> Self {
        Self {
            set: default_set(),
            kind: default_kind(),
            low: None,
            high: None,
            when: When::default(),
            population: None,
            evidence: None,
            citations: Vec::new(),
            note: None,
            legacy: false,
        }
    }
}

/// `when = { ... }`: who the range applies to. Every given field must hold;
/// the most specific matching range wins.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct When {
    /// `male` or `female`.
    pub sex: Option<String>,
    /// `[min, max)` in years.
    pub age: Option<[f64; 2]>,
    pub pregnant: Option<bool>,
    pub trimester: Option<u8>,
    pub fasting: Option<bool>,
    pub menopause: Option<String>,
    pub cycle_phase: Option<String>,
    #[serde(default)]
    pub markers: Vec<MarkerCond>,
    #[serde(default)]
    pub conditions: Vec<String>,
}

impl When {
    pub fn conditions(&self) -> Conditions {
        Conditions {
            pregnant: self.pregnant,
            trimester: self.trimester,
            fasting: self.fasting,
            menopause: self.menopause.clone(),
            cycle_phase: self.cycle_phase.clone(),
            markers: self.markers.clone(),
            conditions: self.conditions.clone(),
        }
    }

    /// Human-readable summary, e.g. `female, age 18-50, fasting`.
    pub fn describe(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.extend(self.sex.clone());
        if let Some([a, b]) = self.age {
            parts.push(if b >= 200.0 { format!("age {a}+") } else { format!("age {a}-{b}") });
        }
        if let Some(p) = self.pregnant {
            parts.push(if p { "pregnant".into() } else { "not pregnant".into() });
        }
        parts.extend(self.trimester.map(|t| format!("trimester {t}")));
        if let Some(f) = self.fasting {
            parts.push(if f { "fasting".into() } else { "non-fasting".into() });
        }
        parts.extend(self.menopause.as_ref().map(|m| format!("{m}-menopausal")));
        parts.extend(self.cycle_phase.as_ref().map(|c| format!("{c} phase")));
        parts.extend(self.markers.iter().map(|m| format!("{} {} {}", m.marker, m.op, m.value)));
        parts.extend(self.conditions.iter().cloned());
        if parts.is_empty() {
            "all".into()
        } else {
            parts.join(", ")
        }
    }
}

/// A source for a range: at least one of `pmid`, `doi`, `url`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Where in the source, e.g. `Table 2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
}

impl Citation {
    /// The best link for the citation.
    pub fn link(&self) -> Option<String> {
        self.pmid
            .as_ref()
            .map(|p| format!("https://pubmed.ncbi.nlm.nih.gov/{p}/"))
            .or_else(|| self.doi.as_ref().map(|d| format!("https://doi.org/{d}")))
            .or_else(|| self.url.clone())
    }

    /// Short label, e.g. `PMID 12345` or the title.
    pub fn label(&self) -> String {
        self.title
            .clone()
            .or_else(|| self.pmid.as_ref().map(|p| format!("PMID {p}")))
            .or_else(|| self.doi.as_ref().map(|d| format!("doi:{d}")))
            .or_else(|| self.url.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionDef {
    pub slug: String,
    pub name: String,
}

/// One `[[marker]]` of catalog/universe.toml.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UniverseEntry {
    pub slug: String,
    pub name: String,
    pub loinc: Option<String>,
    pub category: String,
    /// `core`, `common` or `specialist`.
    pub priority: String,
    /// True when computed from other measurements (ratios, formulas, scores).
    #[serde(default)]
    pub derived: bool,
    /// Why `loinc` is empty, when no code fits.
    #[serde(default)]
    pub loinc_note: Option<String>,
}

/// The raw catalog text: embedded, or read from a directory laid out like `catalog/`.
pub struct Sources {
    /// `(file name, contents)` of every marker file, sorted by name.
    pub markers: Vec<(String, String)>,
    /// `(file name, contents)` of panel files, sorted by name.
    pub panels: Vec<(String, String)>,
    pub sets: String,
    pub conditions: String,
    pub universe: String,
}

impl Sources {
    pub fn embedded() -> Self {
        Self {
            markers: files::MARKER_FILES.iter().map(|(n, t)| ((*n).to_string(), (*t).to_string())).collect(),
            panels: files::PANEL_FILES.iter().map(|(n, t)| ((*n).to_string(), (*t).to_string())).collect(),
            sets: SETS_TOML.into(),
            conditions: CONDITIONS_TOML.into(),
            universe: UNIVERSE_TOML.into(),
        }
    }

    /// Read `<dir>/markers/*.toml`, optional `<dir>/panels/*.toml`, and the
    /// catalog metadata files (missing metadata falls back to the embedded ones).
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let read =
            |p: &Path| std::fs::read_to_string(p).map_err(|e| AppError::io(format!("reading {}: {e}", p.display())));
        let mdir = dir.join("markers");
        let rd = std::fs::read_dir(&mdir).map_err(|e| AppError::io(format!("reading {}: {e}", mdir.display())))?;
        let mut paths: Vec<_> = rd
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        paths.sort();
        let markers = paths
            .iter()
            .map(|p| Ok((p.file_name().unwrap_or_default().to_string_lossy().into_owned(), read(p)?)))
            .collect::<Result<Vec<_>>>()?;
        let pdir = dir.join("panels");
        let panels = if pdir.exists() {
            let mut paths: Vec<_> = std::fs::read_dir(&pdir)
                .map_err(|e| AppError::io(format!("reading {}: {e}", pdir.display())))?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "toml"))
                .collect();
            paths.sort();
            paths
                .iter()
                .map(|p| Ok((p.file_name().unwrap_or_default().to_string_lossy().into_owned(), read(p)?)))
                .collect::<Result<Vec<_>>>()?
        } else {
            Vec::new()
        };
        let or_embedded = |name: &str, fallback: &str| {
            let p = dir.join(name);
            if p.exists() {
                read(&p)
            } else {
                Ok(fallback.to_string())
            }
        };
        Ok(Self {
            markers,
            panels,
            sets: or_embedded("sets.toml", SETS_TOML)?,
            conditions: or_embedded("conditions.toml", CONDITIONS_TOML)?,
            universe: or_embedded("universe.toml", UNIVERSE_TOML)?,
        })
    }
}

fn parse<T: serde::de::DeserializeOwned>(text: &str, origin: &str) -> Result<T> {
    toml::from_str(text).map_err(|e| AppError::config(format!("{origin}: {e}")))
}

pub fn parse_marker(file: &str, text: &str) -> Result<MarkerDef> {
    parse(text, file)
}

pub fn parse_panel(file: &str, text: &str) -> Result<Panel> {
    parse(text, file)
}

pub fn parse_sets(text: &str) -> Result<Vec<SetDef>> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct F {
        #[serde(default)]
        set: Vec<SetDef>,
    }
    parse::<F>(text, "sets.toml").map(|f| f.set)
}

pub fn parse_conditions(text: &str) -> Result<Vec<ConditionDef>> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct F {
        #[serde(default)]
        condition: Vec<ConditionDef>,
    }
    parse::<F>(text, "conditions.toml").map(|f| f.condition)
}

pub fn parse_universe(text: &str) -> Result<Vec<UniverseEntry>> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct F {
        #[serde(default)]
        marker: Vec<UniverseEntry>,
    }
    // `loinc = ""` (with a `loinc_note`) means no code fits, the same as leaving it out.
    let mut entries = parse::<F>(text, "universe.toml")?.marker;
    entries.iter_mut().for_each(|e| e.loinc = e.loinc.take().filter(|l| !l.is_empty()));
    Ok(entries)
}

/// The parsed catalog.
#[derive(Debug, Clone)]
pub struct Defs {
    pub markers: Vec<MarkerDef>,
    pub sets: Vec<SetDef>,
    pub conditions: Vec<ConditionDef>,
    pub panels: Vec<Panel>,
}

impl Defs {
    /// Parse every file; the first error wins (see [`lint`] for a full report).
    pub fn parse(src: &Sources) -> Result<Self> {
        Ok(Self {
            markers: src.markers.iter().map(|(n, t)| parse_marker(n, t)).collect::<Result<_>>()?,
            sets: parse_sets(&src.sets)?,
            conditions: parse_conditions(&src.conditions)?,
            panels: src.panels.iter().map(|(n, t)| parse_panel(n, t)).collect::<Result<_>>()?,
        })
    }

    pub fn marker(&self, slug: &str) -> Option<&MarkerDef> {
        self.markers.iter().find(|m| m.slug == slug)
    }

    pub fn has_set(&self, name: &str) -> bool {
        self.sets.iter().any(|s| s.name == name)
    }
}

/// The embedded catalog, parsed once.
pub fn builtin() -> Result<&'static Defs> {
    static CELL: OnceLock<std::result::Result<Defs, String>> = OnceLock::new();
    CELL.get_or_init(|| Defs::parse(&Sources::embedded()).map_err(|e| e.message))
        .as_ref()
        .map_err(|e| AppError::config(format!("built-in catalog: {e}")))
}

impl RangeDef {
    /// A runtime range for `marker_id`; `scale` maps catalog bounds into the
    /// database marker's unit (the identity unless the unit was changed).
    pub fn to_range(&self, marker_id: i64, scale: &dyn Fn(f64) -> f64) -> Range {
        let [age_min, age_max] = self.when.age.unwrap_or([0.0, 200.0]);
        Range {
            marker_id,
            kind: RangeKind::parse(&self.kind).unwrap_or_default(),
            sex: self.when.sex.clone().unwrap_or_else(|| "any".into()),
            age_min,
            age_max,
            low: self.low.map(scale),
            high: self.high.map(scale),
            note: self.note.clone(),
            set: Some(self.set.clone()),
            cond: self.when.conditions(),
            evidence: self.evidence.clone(),
            citations: self.citations.clone(),
            legacy: self.legacy,
            ..Range::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_parses_and_names_files_by_slug() {
        let defs = builtin().unwrap();
        assert_eq!(defs.markers.len(), 708);
        assert_eq!(defs.panels.len(), 11);
        for (file, text) in files::MARKER_FILES {
            let m = parse_marker(file, text).unwrap();
            assert_eq!(format!("{}.toml", m.slug), *file);
        }
        assert!(defs.has_set("clinical") && defs.has_set("optimal") && defs.has_set("blueprint"));
        let universe = parse_universe(UNIVERSE_TOML).unwrap();
        assert!(universe.len() >= defs.markers.len());
        assert!(universe.iter().all(|e| e.loinc.as_deref() != Some("")), "empty loinc reads as none");
    }

    /// The published JSON Schema names exactly the fields the parser accepts.
    #[test]
    fn json_schema_matches_the_file_format() {
        let schema: serde_json::Value = serde_json::from_str(SCHEMA_JSON).unwrap();
        let props = |v: &serde_json::Value| {
            let mut k: Vec<String> = v["properties"].as_object().unwrap().keys().cloned().collect();
            k.sort();
            k
        };
        fn keys<T: Serialize>(v: &T) -> Vec<String> {
            let mut k: Vec<String> = serde_json::to_value(v).unwrap().as_object().unwrap().keys().cloned().collect();
            k.sort();
            k
        }
        let full_when = When {
            sex: Some("female".into()),
            age: Some([18.0, 50.0]),
            pregnant: Some(true),
            trimester: Some(2),
            fasting: Some(true),
            menopause: Some("post".into()),
            cycle_phase: Some("luteal".into()),
            ..When::default()
        };
        let cite = Citation {
            pmid: Some("1".into()),
            doi: Some("10.1/x".into()),
            url: Some("https://x".into()),
            title: Some("t".into()),
            locator: Some("Table 2".into()),
            quote: None,
        };
        let range = RangeDef {
            low: Some(1.0),
            high: Some(2.0),
            when: full_when.clone(),
            population: Some("p".into()),
            evidence: Some("rct".into()),
            citations: vec![cite.clone()],
            note: Some("n".into()),
            ..RangeDef::default()
        };
        let marker = MarkerDef {
            loinc: Some("1-1".into()),
            specimen: Some("serum".into()),
            description: Some("d".into()),
            no_reference: Some("r".into()),
            ..MarkerDef::default()
        };
        assert_eq!(props(&schema), keys(&marker));
        assert_eq!(props(&schema["$defs"]["range"]), keys(&range));
        assert_eq!(props(&schema["$defs"]["when"]), keys(&full_when));
        assert_eq!(props(&schema["$defs"]["citation"]), keys(&cite));
        assert_eq!(schema["$defs"]["range"]["properties"]["kind"]["enum"], serde_json::json!(KINDS));
        assert_eq!(schema["$defs"]["range"]["properties"]["evidence"]["enum"], serde_json::json!(EVIDENCE));
    }

    #[test]
    fn range_defaults_and_conversion() {
        let r: RangeDef =
            toml::from_str("low = 1.0\nwhen = { sex = \"female\", age = [18, 50], fasting = true }").unwrap();
        assert_eq!((r.set.as_str(), r.kind.as_str()), ("clinical", "reference"));
        let range = r.to_range(7, &|v| v * 2.0);
        assert_eq!((range.sex.as_str(), range.age_min, range.age_max, range.low), ("female", 18.0, 50.0, Some(2.0)));
        assert_eq!(range.cond.fasting, Some(true));
        assert_eq!(r.when.describe(), "female, age 18-50, fasting");
        assert!(toml::from_str::<RangeDef>("lo = 1").is_err(), "unknown fields are rejected");
    }
}
