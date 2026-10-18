//! FHIR R4 reader: Observations (with DiagnosticReport / Specimen context)
//! into [`ResultCandidate`]s.
//!
//! Parsing is deliberately tolerant. It works on `serde_json::Value`, never
//! rejects a resource for unknown fields or minor profile violations, and
//! copes with the shapes older servers emit (a single object where R4 has an
//! array). Anything it cannot use is counted and reported, not fatal.

use std::collections::HashMap;

use serde_json::Value;

use super::candidate::{
    resource_hash, Coding, LabRange, Parsed, PatientHint, ResultCandidate, ResultValue, SourceKind, Status, TimeBasis,
};
use super::ucum::resolve_unit;
use crate::error::{AppError, Result};

/// A resource with the `fullUrl` it carried in its Bundle, if any.
#[derive(Debug, Clone)]
pub struct Res {
    pub value: Value,
    pub full_url: Option<String>,
}

fn str_of<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty())
}

/// An R4 array, or the single object older versions use in its place.
fn list(v: Option<&Value>) -> Vec<&Value> {
    match v {
        Some(Value::Array(a)) => a.iter().collect(),
        Some(o @ Value::Object(_)) => vec![o],
        _ => vec![],
    }
}

fn number(v: Option<&Value>) -> Option<f64> {
    match v? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
    .filter(|f| f.is_finite())
}

// ---------------------------------------------------------------------------
// Reading files into resources
// ---------------------------------------------------------------------------

/// Resources in one file: a single resource, a Bundle (any type, nested ones
/// included), a JSON array of resources, or NDJSON with one resource per line.
pub fn read_resources(text: &str, out: &mut Vec<Res>) -> Result<()> {
    let text = text.trim_start_matches('\u{feff}');
    match serde_json::from_str::<Value>(text) {
        Ok(v) => {
            flatten(v, None, out);
            Ok(())
        }
        Err(whole) => {
            let lines: Vec<(usize, &str)> =
                text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()).map(|(i, l)| (i + 1, l)).collect();
            if lines.len() < 2 {
                return Err(AppError::invalid(format!("not valid FHIR JSON: {whole}")));
            }
            lines.into_iter().try_for_each(|(n, l)| {
                serde_json::from_str::<Value>(l)
                    .map(|v| flatten(v, None, out))
                    .map_err(|e| AppError::invalid(format!("NDJSON line {n}: {e}")))
            })
        }
    }
}

fn flatten(v: Value, full_url: Option<String>, out: &mut Vec<Res>) {
    match v {
        Value::Array(a) => a.into_iter().for_each(|x| flatten(x, None, out)),
        Value::Object(_) if v.get("resourceType").and_then(Value::as_str) == Some("Bundle") => {
            list(v.get("entry")).into_iter().for_each(|e| {
                if let Some(r) = e.get("resource") {
                    flatten(r.clone(), str_of(e, "fullUrl").map(str::to_string), out);
                }
            });
        }
        Value::Object(_) => out.push(Res { value: v, full_url }),
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Resource index
// ---------------------------------------------------------------------------

struct Index<'a> {
    by_ref: HashMap<String, &'a Value>,
}

/// `http://host/fhir/Observation/123/_history/2` -> `Observation/123`.
fn type_and_id(reference: &str) -> Option<String> {
    let parts: Vec<&str> = reference.split('/').filter(|p| !p.is_empty()).collect();
    let parts: Vec<&str> = match parts.iter().position(|p| *p == "_history") {
        Some(i) => parts[..i].to_vec(),
        None => parts,
    };
    match parts.as_slice() {
        [.., t, id] if t.chars().next().is_some_and(char::is_uppercase) => Some(format!("{t}/{id}")),
        _ => None,
    }
}

impl<'a> Index<'a> {
    fn build(resources: &'a [Res]) -> Self {
        let mut by_ref = HashMap::new();
        for r in resources {
            if let (Some(t), Some(id)) = (str_of(&r.value, "resourceType"), str_of(&r.value, "id")) {
                by_ref.insert(format!("{t}/{id}"), &r.value);
            }
            if let Some(u) = &r.full_url {
                by_ref.insert(u.clone(), &r.value);
                if let Some(ti) = type_and_id(u) {
                    by_ref.entry(ti).or_insert(&r.value);
                }
            }
        }
        Self { by_ref }
    }

    fn get(&self, reference: &str) -> Option<&'a Value> {
        self.by_ref.get(reference).or_else(|| type_and_id(reference).and_then(|k| self.by_ref.get(&k))).copied()
    }

    fn resolve(&self, r: &Value) -> Option<&'a Value> {
        str_of(r, "reference").and_then(|s| self.get(s))
    }
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

const OBSERVATION_CATEGORY: &str = "http://terminology.hl7.org/CodeSystem/observation-category";

pub fn parse(resources: &[Res], origin: &str) -> Parsed {
    let mut out = Parsed::default();
    let ix = Index::build(resources);

    // Observation -> the DiagnosticReport that groups it.
    let mut report_of: HashMap<*const Value, &Value> = HashMap::new();
    for r in resources.iter().filter(|r| str_of(&r.value, "resourceType") == Some("DiagnosticReport")) {
        for member in list(r.value.get("result")) {
            if let Some(o) = ix.resolve(member) {
                report_of.entry(std::ptr::from_ref(o)).or_insert(&r.value);
            }
        }
    }

    for r in resources {
        match str_of(&r.value, "resourceType") {
            Some("Observation") => {
                observation(r, &ix, report_of.get(&std::ptr::from_ref(&r.value)).copied(), origin, &mut out)
            }
            Some("Patient") => out.patients.push(PatientHint {
                gender: str_of(&r.value, "gender").map(str::to_ascii_lowercase),
                birth_date: str_of(&r.value, "birthDate").map(str::to_string),
            }),
            // grouping context, read through references
            Some("DiagnosticReport" | "Specimen") => {}
            Some(other) => out.ignore(other),
            None => out.warnings.push("a JSON object without resourceType was ignored".into()),
        }
    }
    out
}

fn coding_list(cc: &Value) -> Vec<Coding> {
    list(cc.get("coding"))
        .into_iter()
        .filter_map(|c| {
            Some(Coding {
                system: str_of(c, "system").unwrap_or_default().to_string(),
                code: str_of(c, "code")?.to_string(),
                display: str_of(c, "display").map(str::to_string),
            })
        })
        .collect()
}

/// Display text of a CodeableConcept: its `text`, else the first coding's display, else its code.
fn concept_text(cc: &Value) -> Option<String> {
    str_of(cc, "text").map(str::to_string).or_else(|| {
        coding_list(cc).into_iter().find_map(|c| c.display.or_else(|| Some(c.code).filter(|s| !s.is_empty())))
    })
}

fn names_of(code: &Value) -> Vec<String> {
    let mut names: Vec<String> = str_of(code, "text").map(str::to_string).into_iter().collect();
    names.extend(coding_list(code).into_iter().filter_map(|c| c.display));
    names.dedup();
    names
}

/// `2024-03` -> `2024-03-01`; a bare year cannot be placed on a day.
fn fhir_date(s: &str) -> Option<String> {
    let t = s.trim();
    match t.len() {
        0..=4 => None,
        7 if t.as_bytes().get(4) == Some(&b'-') => Some(format!("{t}-01")),
        _ => Some(t.to_string()),
    }
}

fn effective(o: &Value, prefix: &str) -> Option<String> {
    str_of(o, &format!("{prefix}DateTime"))
        .or_else(|| str_of(o, &format!("{prefix}Instant")))
        .or_else(|| o.get(format!("{prefix}Period")).and_then(|p| str_of(p, "start")))
        .and_then(fhir_date)
}

fn quantity_unit(q: &Value) -> Option<String> {
    resolve_unit(str_of(q, "system"), str_of(q, "code"), str_of(q, "unit"))
}

fn lab_ranges(parent: &Value) -> Vec<LabRange> {
    list(parent.get("referenceRange"))
        .into_iter()
        .map(|rr| {
            let (low, high) = (rr.get("low"), rr.get("high"));
            let age = rr.get("age");
            LabRange {
                low: number(low.and_then(|q| q.get("value"))),
                high: number(high.and_then(|q| q.get("value"))),
                unit: low.and_then(quantity_unit).or_else(|| high.and_then(quantity_unit)),
                text: str_of(rr, "text").map(str::to_string),
                kind: rr
                    .get("type")
                    .and_then(|t| coding_list(t).into_iter().next().map(|c| c.code).or_else(|| concept_text(t))),
                applies_to: list(rr.get("appliesTo")).into_iter().filter_map(concept_text).collect(),
                age_low: age.and_then(|a| number(a.get("low").and_then(|q| q.get("value")))),
                age_high: age.and_then(|a| number(a.get("high").and_then(|q| q.get("value")))),
            }
        })
        .filter(|r| r.low.is_some() || r.high.is_some() || r.text.is_some())
        .collect()
}

fn interpretation(parent: &Value) -> Vec<String> {
    list(parent.get("interpretation")).into_iter().flat_map(|i| coding_list(i).into_iter().map(|c| c.code)).collect()
}

enum Val {
    Value(ResultValue),
    Absent,
    Unsupported(&'static str),
}

fn read_value(o: &Value) -> Val {
    if let Some(q) = o.get("valueQuantity") {
        return match number(q.get("value")) {
            Some(value) => Val::Value(ResultValue::Numeric {
                value,
                comparator: str_of(q, "comparator").map(str::to_string),
                unit: quantity_unit(q),
            }),
            None => Val::Absent,
        };
    }
    if let Some(n) = o.get("valueInteger").and_then(Value::as_f64) {
        return Val::Value(ResultValue::Numeric { value: n, comparator: None, unit: None });
    }
    if let Some(s) = str_of(o, "valueString") {
        return Val::Value(ResultValue::Text(s.to_string()));
    }
    if let Some(cc) = o.get("valueCodeableConcept") {
        return concept_text(cc).map_or(Val::Absent, |t| Val::Value(ResultValue::Text(t)));
    }
    if let Some(b) = o.get("valueBoolean").and_then(Value::as_bool) {
        return Val::Value(ResultValue::Text(b.to_string()));
    }
    for (k, what) in [
        ("valueRatio", "Ratio"),
        ("valueRange", "Range"),
        ("valueSampledData", "SampledData"),
        ("valueAttachment", "Attachment"),
        ("valueTime", "time"),
        ("valueDateTime", "dateTime"),
        ("valuePeriod", "Period"),
    ] {
        if o.get(k).is_some() {
            return Val::Unsupported(what);
        }
    }
    Val::Absent
}

fn org_name(ix: &Index, r: &Value) -> Option<String> {
    str_of(r, "display")
        .map(str::to_string)
        .or_else(|| ix.resolve(r).and_then(|o| str_of(o, "name")).map(str::to_string))
}

fn source_key(o: &Value, full_url: Option<&String>, origin: &str, hash: &str) -> String {
    list(o.get("identifier"))
        .into_iter()
        .find_map(|i| {
            let value = str_of(i, "value")?;
            Some(match str_of(i, "system") {
                Some(system) => format!("{system}|{value}"),
                None => format!("{origin}|{value}"),
            })
        })
        .or_else(|| {
            let absolute = full_url.filter(|u| u.starts_with("http://") || u.starts_with("https://"));
            absolute.cloned().or_else(|| str_of(o, "id").map(|id| format!("{origin}/Observation/{id}")))
        })
        .unwrap_or_else(|| format!("sha256:{hash}"))
}

fn observation(r: &Res, ix: &Index, report: Option<&Value>, origin: &str, out: &mut Parsed) {
    let o = &r.value;
    let hash = resource_hash(o);
    let key = source_key(o, r.full_url.as_ref(), origin, &hash);
    let id = str_of(o, "id").unwrap_or("?");
    let here = format!("Observation/{id}");
    let status = Status::from_fhir(str_of(o, "status").unwrap_or("unknown"));

    let category = {
        let cats: Vec<Coding> = list(o.get("category")).into_iter().flat_map(coding_list).collect();
        cats.iter()
            .find(|c| c.system.is_empty() || c.system == OBSERVATION_CATEGORY)
            .or_else(|| cats.first())
            .map(|c| c.code.clone())
    };

    let specimen = o
        .get("specimen")
        .and_then(|s| ix.resolve(s))
        .or_else(|| report.and_then(|rp| list(rp.get("specimen")).into_iter().find_map(|s| ix.resolve(s))));
    let collected = specimen.and_then(|s| s.get("collection")).and_then(|c| effective(c, "collected"));

    let (taken_at, time_basis) = [
        (effective(o, "effective"), TimeBasis::Effective),
        (collected, TimeBasis::Specimen),
        (report.and_then(|rp| effective(rp, "effective")), TimeBasis::ReportEffective),
        (str_of(o, "issued").and_then(fhir_date), TimeBasis::Issued),
        (report.and_then(|rp| str_of(rp, "issued")).and_then(fhir_date), TimeBasis::Issued),
    ]
    .into_iter()
    .find_map(|(t, b)| t.map(|t| (Some(t), Some(b))))
    .unwrap_or((None, None));

    let lab = list(o.get("performer"))
        .into_iter()
        .chain(report.into_iter().flat_map(|rp| list(rp.get("performer"))))
        .find_map(|p| org_name(ix, p));

    let fasting =
        specimen.and_then(|s| s.get("collection")).and_then(|c| c.get("fastingStatusCodeableConcept")).and_then(|f| {
            coding_list(f).into_iter().find_map(|c| match c.code.as_str() {
                "F" => Some(true),
                "NF" | "N" => Some(false),
                _ => None,
            })
        });
    let specimen_type = specimen.and_then(|s| s.get("type")).and_then(concept_text);

    let notes: Vec<String> =
        list(o.get("note")).into_iter().filter_map(|n| str_of(n, "text")).map(str::to_string).collect();
    let version = o.get("meta").and_then(|m| str_of(m, "versionId")).map(str::to_string);

    let base = ResultCandidate {
        kind: SourceKind::Fhir,
        key: key.clone(),
        version,
        sha256: hash,
        origin: here.clone(),
        category,
        status,
        codes: vec![],
        names: vec![],
        taken_at,
        time_basis,
        lab,
        fasting,
        specimen_type,
        value: ResultValue::Text(String::new()),
        ranges: vec![],
        interpretation: vec![],
        notes,
    };

    let mut emitted = 0usize;
    let unsupported_before = out.unsupported_value;
    let mut push = |code: &Value, holder: &Value, suffix: Option<String>, out: &mut Parsed| match read_value(holder) {
        Val::Value(value) => {
            out.candidates.push(ResultCandidate {
                key: suffix.map_or_else(|| key.clone(), |s| format!("{key}#{s}")),
                codes: coding_list(code),
                names: names_of(code),
                value,
                ranges: lab_ranges(holder),
                interpretation: interpretation(holder),
                ..base.clone()
            });
            emitted += 1;
        }
        Val::Unsupported(what) => {
            out.unsupported_value += 1;
            out.warnings.push(format!("{here}: value[x] of type {what} is not imported"));
        }
        Val::Absent => {}
    };

    let code = o.get("code").cloned().unwrap_or(Value::Null);
    push(&code, o, None, out);
    for (i, c) in list(o.get("component")).into_iter().enumerate() {
        let ccode = c.get("code").cloned().unwrap_or(Value::Null);
        let suffix = coding_list(&ccode)
            .into_iter()
            .next()
            .map_or_else(|| i.to_string(), |c| format!("{}|{}", c.system, c.code));
        push(&ccode, c, Some(suffix), out);
    }

    if emitted == 0 && out.unsupported_value == unsupported_before {
        if list(o.get("hasMember")).is_empty() {
            out.no_value += 1;
        } else {
            out.panel_headers += 1;
        }
    }
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse_bundle(v: Value) -> Parsed {
        let mut res = vec![];
        flatten(v, None, &mut res);
        parse(&res, "test.json")
    }

    fn glucose(id: &str) -> Value {
        json!({
            "resourceType": "Observation", "id": id, "status": "final",
            "category": [{"coding": [{"system": OBSERVATION_CATEGORY, "code": "laboratory"}]}],
            "code": {"coding": [{"system": "http://loinc.org", "code": "2345-7", "display": "Glucose [Mass/volume] in Serum or Plasma"}], "text": "Glucose"},
            "effectiveDateTime": "2024-03-05T08:30:00+00:00",
            "valueQuantity": {"value": 92, "unit": "mg/dL", "system": "http://unitsofmeasure.org", "code": "mg/dL"},
            "interpretation": [{"coding": [{"system": "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation", "code": "N"}]}],
            "referenceRange": [{"low": {"value": 70, "unit": "mg/dL", "system": "http://unitsofmeasure.org", "code": "mg/dL"}, "high": {"value": 99, "unit": "mg/dL", "system": "http://unitsofmeasure.org", "code": "mg/dL"}}]
        })
    }

    #[test]
    fn reads_a_lab_observation() {
        let p = parse_bundle(
            json!({"resourceType": "Bundle", "type": "collection", "entry": [{"resource": glucose("g1")}]}),
        );
        assert_eq!(p.candidates.len(), 1);
        let c = &p.candidates[0];
        assert_eq!(c.category.as_deref(), Some("laboratory"));
        assert_eq!(c.loinc_codes().collect::<Vec<_>>(), vec!["2345-7"]);
        assert_eq!(c.value, ResultValue::Numeric { value: 92.0, comparator: None, unit: Some("mg/dL".into()) });
        assert_eq!(c.interpretation, vec!["N".to_string()]);
        assert_eq!(c.ranges.len(), 1);
        assert_eq!((c.ranges[0].low, c.ranges[0].high), (Some(70.0), Some(99.0)));
        assert_eq!(c.time_basis, Some(TimeBasis::Effective));
        assert_eq!(c.key, "test.json/Observation/g1");
    }

    #[test]
    fn identifier_is_the_source_key() {
        let mut o = glucose("g1");
        o["identifier"] = json!([{"system": "urn:lab:acc", "value": "A-77"}]);
        let p = parse_bundle(o);
        assert_eq!(p.candidates[0].key, "urn:lab:acc|A-77");
    }

    #[test]
    fn time_falls_back_to_specimen_report_then_issued() {
        let mut o = glucose("g1");
        o.as_object_mut().unwrap().remove("effectiveDateTime");
        o["specimen"] = json!({"reference": "Specimen/s1"});
        let spec = json!({"resourceType": "Specimen", "id": "s1", "type": {"text": "Serum"},
            "collection": {"collectedDateTime": "2024-03-04T07:00:00Z",
                           "fastingStatusCodeableConcept": {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/v2-0916", "code": "F"}]}}});
        let p = parse_bundle(json!([o.clone(), spec]));
        assert_eq!(p.candidates[0].time_basis, Some(TimeBasis::Specimen));
        assert_eq!(p.candidates[0].taken_at.as_deref(), Some("2024-03-04T07:00:00Z"));
        assert_eq!(p.candidates[0].fasting, Some(true));
        assert_eq!(p.candidates[0].specimen_type.as_deref(), Some("Serum"));

        o.as_object_mut().unwrap().remove("specimen");
        let report = json!({"resourceType": "DiagnosticReport", "id": "r1", "effectiveDateTime": "2024-03-03",
            "performer": [{"display": "Example Lab"}], "result": [{"reference": "Observation/g1"}]});
        let p = parse_bundle(json!([o.clone(), report]));
        assert_eq!(p.candidates[0].time_basis, Some(TimeBasis::ReportEffective));
        assert_eq!(p.candidates[0].taken_at.as_deref(), Some("2024-03-03"));
        assert_eq!(p.candidates[0].lab.as_deref(), Some("Example Lab"));

        o["issued"] = json!("2024-03-06T10:00:00Z");
        let p = parse_bundle(o);
        assert_eq!(p.candidates[0].time_basis, Some(TimeBasis::Issued));
    }

    #[test]
    fn components_become_one_candidate_each_and_panels_are_skipped() {
        let bp = json!({"resourceType": "Observation", "id": "bp", "status": "final",
        "category": [{"coding": [{"system": OBSERVATION_CATEGORY, "code": "vital-signs"}]}],
        "code": {"coding": [{"system": "http://loinc.org", "code": "85354-9"}]},
        "effectiveDateTime": "2024-03-05",
        "component": [
            {"code": {"coding": [{"system": "http://loinc.org", "code": "8480-6"}]}, "valueQuantity": {"value": 118, "unit": "mmHg", "system": "http://unitsofmeasure.org", "code": "mm[Hg]"}},
            {"code": {"coding": [{"system": "http://loinc.org", "code": "8462-4"}]}, "valueQuantity": {"value": 76, "unit": "mmHg", "system": "http://unitsofmeasure.org", "code": "mm[Hg]"}}
        ]});
        let panel = json!({"resourceType": "Observation", "id": "panel", "status": "final", "code": {"text": "Lipid panel"},
            "hasMember": [{"reference": "Observation/g1"}]});
        let p = parse_bundle(json!([bp, panel]));
        assert_eq!(p.candidates.len(), 2);
        assert_eq!(p.panel_headers, 1);
        assert_eq!(p.candidates[0].loinc_codes().collect::<Vec<_>>(), vec!["8480-6"]);
        assert_ne!(p.candidates[0].key, p.candidates[1].key);
        assert_eq!(
            p.candidates[0].value,
            ResultValue::Numeric { value: 118.0, comparator: None, unit: Some("mmHg".into()) }
        );
    }

    #[test]
    fn counts_what_it_ignores_and_tolerates_old_shapes() {
        let mut o = glucose("g1");
        o["category"] = json!({"coding": [{"code": "laboratory"}]});
        o["interpretation"] = json!({"coding": [{"code": "H"}]});
        let p = parse_bundle(
            json!([o, {"resourceType": "Practitioner", "id": "x"}, {"resourceType": "Practitioner", "id": "y"}, {"resourceType": "Patient", "gender": "Male", "birthDate": "1984-06-01"}]),
        );
        assert_eq!(p.candidates[0].category.as_deref(), Some("laboratory"));
        assert_eq!(p.candidates[0].interpretation, vec!["H".to_string()]);
        assert_eq!(p.ignored.get("Practitioner"), Some(&2));
        assert_eq!(p.patients[0].gender.as_deref(), Some("male"));
    }

    #[test]
    fn qualitative_and_unsupported_values() {
        let text = json!({"resourceType": "Observation", "id": "u", "status": "final", "code": {"text": "Urine glucose"},
            "effectiveDateTime": "2024-03-05", "valueCodeableConcept": {"text": "Negative"}});
        let ratio = json!({"resourceType": "Observation", "id": "t", "status": "final", "code": {"text": "Titer"},
            "effectiveDateTime": "2024-03-05", "valueRatio": {"numerator": {"value": 1}, "denominator": {"value": 80}}});
        let none = json!({"resourceType": "Observation", "id": "n", "status": "final", "code": {"text": "x"},
            "dataAbsentReason": {"text": "not performed"}});
        let p = parse_bundle(json!([text, ratio, none]));
        assert_eq!(p.candidates.len(), 1);
        assert_eq!(p.candidates[0].value, ResultValue::Text("Negative".into()));
        assert_eq!(p.unsupported_value, 1);
        assert_eq!(p.no_value, 1);
    }

    #[test]
    fn comparator_and_partial_dates() {
        let mut o = glucose("g1");
        o["valueQuantity"] = json!({"value": 0.5, "comparator": "<", "unit": "mg/dL", "system": "http://unitsofmeasure.org", "code": "mg/dL"});
        o["effectiveDateTime"] = json!("2024-03");
        let p = parse_bundle(o);
        assert_eq!(p.candidates[0].taken_at.as_deref(), Some("2024-03-01"));
        assert!(matches!(&p.candidates[0].value, ResultValue::Numeric { comparator: Some(c), .. } if c == "<"));
    }

    #[test]
    fn reads_bundle_json_array_and_ndjson() {
        let b = json!({"resourceType": "Bundle", "entry": [{"fullUrl": "urn:uuid:1", "resource": glucose("a")}, {"resource": {"resourceType": "Bundle", "entry": [{"resource": glucose("b")}]}}]});
        let mut v = vec![];
        read_resources(&b.to_string(), &mut v).unwrap();
        assert_eq!(v.len(), 2);
        let mut v = vec![];
        read_resources(&format!("{}\n{}\n", glucose("a"), glucose("b")), &mut v).unwrap();
        assert_eq!(v.len(), 2);
        assert!(read_resources("not json", &mut vec![]).is_err());
        assert!(read_resources("{\"a\":1}\nnot json\n", &mut vec![]).is_err());
    }
}
