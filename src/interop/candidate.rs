//! The intermediate every external result format is parsed into.
//!
//! FHIR Observations and HL7 v2 OBX segments both become a [`ResultCandidate`],
//! so marker mapping, unit conversion, dedupe and storage are written once
//! (`pipeline.rs`).

use std::collections::BTreeMap;

use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Fhir,
    Hl7v2,
}

impl SourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fhir => "fhir",
            Self::Hl7v2 => "hl7v2",
        }
    }
}

/// Which input a result's `taken_at` came from (recorded in `result_source.time_basis`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeBasis {
    Effective,
    Specimen,
    ReportEffective,
    Issued,
    /// HL7 v2: OBX-14, else OBR-7, else the specimen collection time.
    Obx,
    Obr,
}

impl TimeBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Effective => "effective",
            Self::Specimen => "specimen",
            Self::ReportEffective => "report-effective",
            Self::Issued => "issued",
            Self::Obx => "obx-14",
            Self::Obr => "obr-7",
        }
    }
}

/// Result status, collapsed from FHIR ObservationStatus / HL7 v2 OBX-11.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Final,
    Amended,
    Corrected,
    Preliminary,
    Registered,
    /// entered-in-error, cancelled, deleted, wrong patient: the result must not stand.
    Retracted,
    Unknown,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Final => "final",
            Self::Amended => "amended",
            Self::Corrected => "corrected",
            Self::Preliminary => "preliminary",
            Self::Registered => "registered",
            Self::Retracted => "retracted",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_fhir(s: &str) -> Self {
        match s {
            "final" => Self::Final,
            "amended" => Self::Amended,
            "corrected" => Self::Corrected,
            "preliminary" => Self::Preliminary,
            "registered" => Self::Registered,
            "entered-in-error" | "cancelled" => Self::Retracted,
            _ => Self::Unknown,
        }
    }

    /// Imported without any flag.
    pub fn is_final(self) -> bool {
        matches!(self, Self::Final | Self::Amended | Self::Corrected)
    }

    pub fn is_provisional(self) -> bool {
        matches!(self, Self::Preliminary | Self::Registered)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coding {
    pub system: String,
    pub code: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResultValue {
    /// A number; `unit` is the unit spelling resolved from UCUM / the unit text.
    Numeric { value: f64, comparator: Option<String>, unit: Option<String> },
    /// Anything qualitative: a string, a coded result, a boolean.
    Text(String),
}

/// A reference range exactly as the lab reported it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LabRange {
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub unit: Option<String>,
    pub text: Option<String>,
    pub kind: Option<String>,
    pub applies_to: Vec<String>,
    pub age_low: Option<f64>,
    pub age_high: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ResultCandidate {
    pub kind: SourceKind,
    /// Stable identity of the source record (`system|value` or a path), for idempotent re-import.
    pub key: String,
    pub version: Option<String>,
    pub sha256: String,
    /// Human description of where this came from, for messages (`Observation/abc`).
    pub origin: String,
    pub category: Option<String>,
    pub status: Status,
    pub codes: Vec<Coding>,
    /// Free-text names of the test (code text, codings' display), best first.
    pub names: Vec<String>,
    /// Raw date/time text; parsed by the pipeline with the configured zone.
    pub taken_at: Option<String>,
    pub time_basis: Option<TimeBasis>,
    pub lab: Option<String>,
    pub fasting: Option<bool>,
    pub specimen_type: Option<String>,
    pub value: ResultValue,
    pub ranges: Vec<LabRange>,
    /// Interpretation codes (N, L, H, LL, HH, A, AA, <, >).
    pub interpretation: Vec<String>,
    pub notes: Vec<String>,
}

impl ResultCandidate {
    pub fn loinc_codes(&self) -> impl Iterator<Item = &str> {
        self.codes
            .iter()
            .filter(|c| {
                let s = c.system.trim_end_matches('/');
                s.eq_ignore_ascii_case("http://loinc.org")
                    || s.eq_ignore_ascii_case("LN")
                    || s.eq_ignore_ascii_case("LOINC")
            })
            .map(|c| c.code.as_str())
    }

    /// A short label for messages: LOINC code, else the first name.
    pub fn label(&self) -> String {
        self.loinc_codes()
            .next()
            .map(str::to_string)
            .or_else(|| self.names.first().cloned())
            .unwrap_or_else(|| self.origin.clone())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PatientHint {
    /// FHIR administrative gender: male | female | other | unknown.
    pub gender: Option<String>,
    pub birth_date: Option<String>,
}

/// Everything read from the input before any database work.
#[derive(Debug, Default)]
pub struct Parsed {
    pub candidates: Vec<ResultCandidate>,
    pub patients: Vec<PatientHint>,
    /// Resource type (or segment) -> how many were set aside.
    pub ignored: BTreeMap<String, usize>,
    /// Observations that only group others (`hasMember`, no value).
    pub panel_headers: usize,
    /// Observations with neither a value nor members (`dataAbsentReason`).
    pub no_value: usize,
    /// Values of a type this importer does not read (ratio, range, sampled data, attachments).
    pub unsupported_value: usize,
    pub warnings: Vec<String>,
}

impl Parsed {
    pub fn ignore(&mut self, what: &str) {
        *self.ignored.entry(what.to_string()).or_default() += 1;
    }

    /// Fold the results of another file into this one.
    pub fn merge(&mut self, other: Parsed) {
        self.candidates.extend(other.candidates);
        self.patients.extend(other.patients);
        other.ignored.into_iter().for_each(|(k, n)| *self.ignored.entry(k).or_default() += n);
        self.panel_headers += other.panel_headers;
        self.no_value += other.no_value;
        self.unsupported_value += other.unsupported_value;
        self.warnings.extend(other.warnings);
    }
}

// ---------------------------------------------------------------------------
// Interpretation codes
// ---------------------------------------------------------------------------

/// Display bucket for a set of interpretation codes:
/// normal; out-of-range (L/H); critical (LL/HH/AA); abnormal (A); off-scale (< >).
pub fn bucket(codes: &[String]) -> Option<&'static str> {
    let has = |set: &[&str]| codes.iter().any(|c| set.contains(&c.trim().to_ascii_uppercase().as_str()));
    if has(&["LL", "HH", "AA"]) {
        Some("critical")
    } else if has(&["<", ">"]) {
        Some("off-scale")
    } else if has(&["L", "H"]) {
        Some("out-of-range")
    } else if has(&["A"]) {
        Some("abnormal")
    } else if has(&["N"]) {
        Some("normal")
    } else {
        None
    }
}

/// True when the lab marked the result abnormal in any way.
pub fn is_abnormal(codes: &[String]) -> bool {
    matches!(bucket(codes), Some("critical" | "off-scale" | "out-of-range" | "abnormal"))
}

/// Does the lab's flag agree with the catalog's (`low` | `normal` | `high`)?
/// `None` when either side has no opinion. A direction-less abnormal flag (A, AA)
/// agrees with any out-of-range catalog flag.
pub fn flag_agrees(codes: &[String], catalog_flag: Option<&str>) -> Option<bool> {
    let flag = catalog_flag?;
    let up = |c: &String| c.trim().to_ascii_uppercase();
    let low = codes.iter().map(up).any(|c| matches!(c.as_str(), "L" | "LL" | "<"));
    let high = codes.iter().map(up).any(|c| matches!(c.as_str(), "H" | "HH" | ">"));
    match bucket(codes)? {
        "normal" => Some(flag == "normal"),
        _ if low && !high => Some(flag == "low"),
        _ if high && !low => Some(flag == "high"),
        _ => Some(flag != "normal"),
    }
}

// ---------------------------------------------------------------------------
// Hashing
// ---------------------------------------------------------------------------

/// The same JSON with object keys sorted at every level, so two renderings of
/// one resource hash alike.
pub fn canonical(v: &Value) -> Value {
    match v {
        Value::Object(o) => {
            let mut keys: Vec<&String> = o.keys().collect();
            keys.sort();
            Value::Object(keys.into_iter().map(|k| (k.clone(), canonical(&o[k]))).collect())
        }
        Value::Array(a) => Value::Array(a.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// Content hash of a resource: canonical JSON without the server-assigned
/// `meta` (versionId, lastUpdated), which changes when a portal re-issues the same data.
pub fn resource_hash(v: &Value) -> String {
    let mut c = canonical(v);
    if let Value::Object(o) = &mut c {
        o.remove("meta");
    }
    sha256_hex(c.to_string().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn s(codes: &[&str]) -> Vec<String> {
        codes.iter().map(|c| (*c).to_string()).collect()
    }

    #[test]
    fn buckets_interpretation_codes() {
        assert_eq!(bucket(&s(&["N"])), Some("normal"));
        assert_eq!(bucket(&s(&["H"])), Some("out-of-range"));
        assert_eq!(bucket(&s(&["l"])), Some("out-of-range"));
        assert_eq!(bucket(&s(&["HH"])), Some("critical"));
        assert_eq!(bucket(&s(&["A"])), Some("abnormal"));
        assert_eq!(bucket(&s(&[">"])), Some("off-scale"));
        assert_eq!(bucket(&s(&["H", "AA"])), Some("critical"));
        assert_eq!(bucket(&s(&["POS"])), None);
        assert!(is_abnormal(&s(&["A"])));
        assert!(!is_abnormal(&s(&["N"])));
    }

    #[test]
    fn lab_flag_vs_catalog_flag() {
        assert_eq!(flag_agrees(&s(&["H"]), Some("high")), Some(true));
        assert_eq!(flag_agrees(&s(&["H"]), Some("low")), Some(false));
        assert_eq!(flag_agrees(&s(&["N"]), Some("high")), Some(false));
        assert_eq!(flag_agrees(&s(&["A"]), Some("low")), Some(true));
        assert_eq!(flag_agrees(&s(&["A"]), Some("normal")), Some(false));
        assert_eq!(flag_agrees(&s(&["H"]), None), None);
        assert_eq!(flag_agrees(&[], Some("normal")), None);
    }

    #[test]
    fn hash_ignores_key_order_and_meta() {
        let a = json!({"a": 1, "b": {"x": [1, 2], "y": 2}, "meta": {"versionId": "1"}});
        let b = json!({"b": {"y": 2, "x": [1, 2]}, "meta": {"versionId": "7"}, "a": 1});
        assert_eq!(resource_hash(&a), resource_hash(&b));
        assert_ne!(resource_hash(&a), resource_hash(&json!({"a": 2})));
        assert_eq!(resource_hash(&a).len(), 64);
    }

    #[test]
    fn status_mapping() {
        assert!(Status::from_fhir("final").is_final());
        assert!(Status::from_fhir("corrected").is_final());
        assert!(Status::from_fhir("preliminary").is_provisional());
        assert_eq!(Status::from_fhir("entered-in-error"), Status::Retracted);
        assert_eq!(Status::from_fhir("nonsense"), Status::Unknown);
    }
}
