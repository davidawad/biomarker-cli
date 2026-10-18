//! `biomarker export --format fhir|fhir-ndjson`: measurements as FHIR R4
//! Laboratory Result Observations (US Core shaped), nothing transmitted.
//!
//! The lab's own reference range is what a result carries by default; the
//! catalog's cited population range is only added on request and is labelled
//! as ours, never presented as the lab's.

use std::collections::{BTreeMap, HashMap};

use chrono::{NaiveDateTime, Offset, TimeZone};
use serde_json::{json, Value};

use super::candidate::sha256_hex;
use super::{loinc, schema, ucum};
use crate::cli::{ExportArgs, FhirRanges, InterpretArg};
use crate::context::Ctx;
use crate::db::{Db, RowExt};
use crate::error::{AppError, Result};
use crate::output::Report;
use crate::store::{self, Catalog, ObservationRow, Person};
use crate::util::{now_iso, Tz};
use crate::view::Evaluated;

const OBSERVATION_CATEGORY: &str = "http://terminology.hl7.org/CodeSystem/observation-category";
const INTERPRETATION: &str = "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation";
const REFERENCE_MEANING: &str = "http://terminology.hl7.org/CodeSystem/referencerange-meaning";
const FASTING: &str = "http://terminology.hl7.org/CodeSystem/v2-0916";
const DIAGNOSTIC_SERVICE: &str = "http://terminology.hl7.org/CodeSystem/v2-0074";
const UCUM: &str = "http://unitsofmeasure.org";
const US_CORE_LAB: &str = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-lab";

pub struct Opts {
    pub ranges: FhirRanges,
    pub interpret_catalog: bool,
    pub include_name: bool,
    pub group_reports: bool,
    /// An explicit `--units` (or `unit_system` setting): values in that system, not as reported.
    pub explicit_units: bool,
    pub flavor: String,
}

/// Resources of one export, each with the `fullUrl` its Bundle entry carries.
pub struct Exported {
    pub resources: Vec<(String, Value)>,
    pub warnings: Vec<String>,
}

/// A deterministic UUID for a resource of this database: stable across exports.
fn urn(db_uuid: &str, kind: &str, id: &str) -> String {
    let h = sha256_hex(format!("{db_uuid}/{kind}/{id}").as_bytes());
    format!("urn:uuid:{}-{}-5{}-8{}-{}", &h[..8], &h[8..12], &h[13..16], &h[17..20], &h[20..32])
}

fn fmt_offset(secs: i32) -> String {
    if secs == 0 {
        return "Z".into();
    }
    let sign = if secs < 0 { '-' } else { '+' };
    let a = secs.abs();
    format!("{sign}{:02}:{:02}", a / 3600, (a % 3600) / 60)
}

/// `taken_at` as a FHIR dateTime: a date stays a date, a time gets the zone's offset.
pub fn date_time(taken_at: &str, tz: &Tz) -> String {
    let Ok(naive) = NaiveDateTime::parse_from_str(taken_at, "%Y-%m-%dT%H:%M:%S") else { return taken_at.to_string() };
    let secs = match tz {
        Tz::Fixed(o) => o.local_minus_utc(),
        Tz::Named(z) => z.offset_from_local_datetime(&naive).earliest().map_or(0, |o| o.fix().local_minus_utc()),
        Tz::Local => {
            chrono::Local.offset_from_local_datetime(&naive).earliest().map_or(0, |o| o.fix().local_minus_utc())
        }
    };
    format!("{taken_at}{}", fmt_offset(secs))
}

fn quantity(value: f64, unit: &str) -> Value {
    let mut q = json!({"value": value, "unit": unit});
    if let Some(code) = ucum::to_ucum(unit) {
        q["system"] = json!(UCUM);
        q["code"] = json!(code);
    }
    q
}

fn interpretation_concept(code: &str) -> Value {
    json!({"coding": [{"system": INTERPRETATION, "code": code}]})
}

fn code_concept(marker_name: &str, loinc: Option<&String>) -> Value {
    match loinc {
        Some(l) => {
            json!({"coding": [{"system": "http://loinc.org", "code": l, "display": marker_name}], "text": marker_name})
        }
        None => json!({"text": marker_name}),
    }
}

fn category_of(marker_category: &str) -> (&'static str, &'static str) {
    match marker_category {
        "vitals" | "body" => ("vital-signs", "Vital Signs"),
        "wearable" => ("activity", "Activity"),
        _ => ("laboratory", "Laboratory"),
    }
}

struct Meta {
    ranges: Vec<Value>,
    interpretation: Vec<String>,
    specimen_type: Option<String>,
    status: Option<String>,
    revision: i64,
}

fn load_meta(db: &Db) -> Result<HashMap<i64, Meta>> {
    let mut out: HashMap<i64, Meta> = HashMap::new();
    for r in
        db.query("SELECT measurement_id, ranges_json, interpretation, specimen_type, status FROM lab_result_meta", &[])?
    {
        let Some(id) = r.i(0) else { continue };
        out.insert(
            id,
            Meta {
                ranges: r.s(1).and_then(|j| serde_json::from_str::<Vec<Value>>(&j).ok()).unwrap_or_default(),
                interpretation: r.s(2).map(|i| i.split(',').map(str::to_string).collect()).unwrap_or_default(),
                specimen_type: r.s(3),
                status: r.s(4),
                revision: 1,
            },
        );
    }
    for r in db.query("SELECT target_id, revision FROM result_source WHERE kind = 'measurement'", &[])? {
        if let (Some(id), Some(rev)) = (r.i(0), r.i(1)) {
            if let Some(m) = out.get_mut(&id) {
                m.revision = rev;
            }
        }
    }
    Ok(out)
}

fn lab_range(r: &Value) -> Value {
    let unit = r["unit"].as_str();
    let bound = |k: &str| r[k].as_f64().map(|v| quantity(v, unit.unwrap_or("")));
    let strip = |q: Value| if unit.is_none() { json!({"value": q["value"]}) } else { q };
    let mut out = json!({});
    if let Some(q) = bound("low") {
        out["low"] = strip(q);
    }
    if let Some(q) = bound("high") {
        out["high"] = strip(q);
    }
    if let Some(t) = r["text"].as_str() {
        out["text"] = json!(t);
    }
    if let Some(k) = r["type"].as_str() {
        out["type"] = match k.to_ascii_lowercase().as_str() {
            c @ ("normal" | "recommended" | "treatment" | "therapeutic" | "pre" | "post" | "endpoint" | "absolute") => {
                json!({"coding": [{"system": REFERENCE_MEANING, "code": c}]})
            }
            _ => json!({"text": k}),
        };
    }
    if let Some(a) = r["applies_to"].as_array().filter(|a| !a.is_empty()) {
        out["appliesTo"] = Value::Array(a.iter().filter_map(Value::as_str).map(|t| json!({"text": t})).collect());
    }
    if r["age_low"].is_number() || r["age_high"].is_number() {
        let year = |v: &Value| json!({"value": v, "unit": "year", "system": UCUM, "code": "a"});
        let mut age = json!({});
        if r["age_low"].is_number() {
            age["low"] = year(&r["age_low"]);
        }
        if r["age_high"].is_number() {
            age["high"] = year(&r["age_high"]);
        }
        out["age"] = age;
    }
    out
}

/// "Positive" and friends as a coded value; anything else stays a string.
fn qualitative_value(t: &str) -> Value {
    let code = match t.trim().to_ascii_lowercase().as_str() {
        "positive" => Some("POS"),
        "negative" => Some("NEG"),
        "detected" => Some("DET"),
        "not detected" => Some("ND"),
        "normal" => Some("N"),
        "abnormal" => Some("A"),
        _ => None,
    };
    match code {
        Some(c) => json!({"valueCodeableConcept": {"coding": [{"system": INTERPRETATION, "code": c}], "text": t}}),
        None => json!({"valueString": t}),
    }
}

struct Ctxt<'a> {
    cat: &'a Catalog,
    tz: &'a Tz,
    o: &'a Opts,
    db_uuid: String,
    patient_ref: String,
    loincs: HashMap<i64, String>,
    meta: HashMap<i64, Meta>,
}

pub fn build(
    ctx: &Ctx,
    db: &Db,
    cat: &Catalog,
    person: &Person,
    rows: &[Evaluated],
    obs: &[ObservationRow],
    o: &Opts,
) -> Result<Exported> {
    let db_uuid = schema::db_uuid(db)?;
    let patient_url = urn(&db_uuid, "Patient", &person.id.to_string());
    let x = Ctxt {
        cat,
        tz: &ctx.tz,
        o,
        db_uuid,
        patient_ref: patient_url.clone(),
        loincs: loinc::primary_by_marker(db)?,
        meta: load_meta(db)?,
    };
    let mut out: Vec<(String, Value)> = vec![(patient_url, patient(person, o.include_name))];
    let mut no_loinc: BTreeMap<String, ()> = BTreeMap::new();
    let mut no_ucum: BTreeMap<String, ()> = BTreeMap::new();
    let mut groups: BTreeMap<(Option<String>, String, Option<String>), Vec<String>> = BTreeMap::new();

    for e in rows {
        let (specimen, observation) = measurement(&x, e);
        if !x.loincs.contains_key(&e.marker.id) {
            no_loinc.insert(e.marker.slug.clone(), ());
        }
        let used_unit = observation["valueQuantity"]["unit"].as_str().unwrap_or("");
        if observation["valueQuantity"]["code"].is_null() {
            no_ucum.insert(used_unit.to_string(), ());
        }
        let obs_url = urn(&x.db_uuid, "Observation", &format!("m{}", e.row.id));
        if let Some(sp) = specimen {
            out.push((urn(&x.db_uuid, "Specimen", &format!("m{}", e.row.id)), sp));
        }
        groups
            .entry((e.row.lab.clone(), e.row.taken_at.clone(), e.row.batch_id.clone()))
            .or_default()
            .push(obs_url.clone());
        out.push((obs_url, observation));
    }
    for ob in obs {
        let marker = cat.find(&ob.marker);
        let mid = marker.map(|m| m.id);
        if mid.is_none_or(|m| !x.loincs.contains_key(&m)) {
            no_loinc.insert(ob.marker.clone(), ());
        }
        out.push((
            urn(&x.db_uuid, "Observation", &format!("o{}", ob.id)),
            qualitative(&x, ob, mid.and_then(|m| x.loincs.get(&m))),
        ));
    }
    if o.group_reports {
        for (n, ((lab, taken_at, _), refs)) in groups.into_iter().enumerate() {
            let mut report = json!({
                "resourceType": "DiagnosticReport",
                "id": format!("dr{}", n + 1),
                "status": "final",
                "category": [{"coding": [{"system": DIAGNOSTIC_SERVICE, "code": "LAB"}]}],
                "code": {"text": "Laboratory results"},
                "subject": {"reference": x.patient_ref},
                "effectiveDateTime": date_time(&taken_at, x.tz),
                "result": refs.iter().map(|r| json!({"reference": r})).collect::<Vec<_>>(),
            });
            if let Some(l) = lab {
                report["performer"] = json!([{"display": l}]);
            }
            out.push((urn(&x.db_uuid, "DiagnosticReport", &(n + 1).to_string()), report));
        }
    }
    let mut warnings = vec![];
    if !no_loinc.is_empty() {
        warnings.push(format!(
            "{} marker(s) have no LOINC code and export as text only: {}",
            no_loinc.len(),
            no_loinc.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
    }
    if !no_ucum.is_empty() {
        warnings.push(format!(
            "{} unit(s) have no UCUM code and export as unit text only: {}",
            no_ucum.len(),
            no_ucum.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
    }
    Ok(Exported { resources: out, warnings })
}

fn patient(p: &Person, include_name: bool) -> Value {
    let mut v = json!({"resourceType": "Patient", "id": format!("p{}", p.id)});
    if let Some(g) =
        p.sex.as_deref().map(str::to_ascii_lowercase).filter(|g| matches!(g.as_str(), "male" | "female" | "other"))
    {
        v["gender"] = json!(g);
    }
    if let Some(d) = &p.dob {
        v["birthDate"] = json!(d);
    }
    if include_name {
        if let Some(n) = &p.name {
            v["name"] = json!([{"text": n}]);
        }
    }
    v
}

/// (Specimen, Observation) for one measurement.
fn measurement(x: &Ctxt, e: &Evaluated) -> (Option<Value>, Value) {
    let (r, m) = (&e.row, &e.marker);
    let meta = x.meta.get(&r.id);
    let (cat_code, cat_display) = category_of(&m.category);
    let loinc = x.loincs.get(&m.id);
    let id = format!("m{}", r.id);
    let effective = date_time(&r.taken_at, x.tz);

    let status = match meta {
        Some(mt) if mt.status.as_deref() == Some("corrected") => "corrected",
        Some(mt) if mt.status.as_deref() == Some("amended") || mt.revision > 1 => "amended",
        _ => "final",
    };
    let mut obs = json!({
        "resourceType": "Observation",
        "id": id,
        "identifier": [{"system": format!("urn:biomarker-cli:{}", x.db_uuid), "value": r.id.to_string()}],
        "status": status,
        "category": [{"coding": [{"system": OBSERVATION_CATEGORY, "code": cat_code, "display": cat_display}]}],
        "code": code_concept(&m.name, loinc),
        "subject": {"reference": x.patient_ref},
        "effectiveDateTime": effective,
    });
    if cat_code == "laboratory" {
        obs["meta"] = json!({"profile": [US_CORE_LAB]});
    }
    let mut tags: Vec<Value> = r.tags.iter().map(|t| json!({"system": "urn:biomarker-cli:tag", "code": t})).collect();
    if loinc.is_none() {
        tags.push(json!({"system": "urn:biomarker-cli:warning", "code": "no-loinc"}));
    }
    if !tags.is_empty() {
        obs["meta"]["tag"] = Value::Array(tags);
    }

    // value, in the unit as reported unless a unit system was asked for
    let (value, unit) =
        if x.o.explicit_units { (e.display_value, e.display_unit.clone()) } else { (r.value_raw, r.unit_raw.clone()) };
    let mut q = quantity(value, &unit);
    if let Some(c) = &r.qualifier {
        q["comparator"] = json!(c);
    }
    obs["valueQuantity"] = q;

    // reference ranges
    let mut ranges: Vec<Value> = vec![];
    if x.o.ranges != FhirRanges::Omit {
        ranges.extend(meta.into_iter().flat_map(|mt| mt.ranges.iter().map(lab_range)));
    }
    if x.o.ranges == FhirRanges::Catalog {
        ranges.extend(catalog_range(x, e, &unit));
    }
    if !ranges.is_empty() {
        obs["referenceRange"] = Value::Array(ranges);
    }

    // interpretation
    let mut interp: Vec<Value> =
        meta.into_iter().flat_map(|mt| mt.interpretation.iter().map(|c| interpretation_concept(c))).collect();
    if x.o.interpret_catalog {
        if let Some(code) = e.flag(&x.o.flavor).map(|f| match f.as_str() {
            "low" => "L",
            "high" => "H",
            _ => "N",
        }) {
            interp.push(json!({"coding": [{"system": INTERPRETATION, "code": code}], "text": "biomarker-cli catalog"}));
        }
    }
    if !interp.is_empty() {
        obs["interpretation"] = Value::Array(interp);
    }
    if let Some(n) = r.note.as_ref().filter(|n| !n.is_empty()) {
        obs["note"] = json!([{"text": n}]);
    }
    if let Some(l) = r.lab.as_ref().filter(|l| !l.is_empty()) {
        obs["performer"] = json!([{"display": l}]);
    }

    // specimen
    let spec_type = meta.and_then(|mt| mt.specimen_type.clone());
    let specimen = (r.fasting.is_some() || spec_type.is_some() || r.lab.as_deref().is_some_and(|l| !l.is_empty()))
        .then(|| {
            let mut collection = json!({"collectedDateTime": effective});
            if let Some(f) = r.fasting {
                collection["fastingStatusCodeableConcept"] =
                    json!({"coding": [{"system": FASTING, "code": if f { "F" } else { "NF" }}]});
            }
            let mut sp = json!({"resourceType": "Specimen", "id": format!("sp-{id}"), "status": "available",
                            "subject": {"reference": x.patient_ref}, "collection": collection});
            if let Some(t) = &spec_type {
                sp["type"] = json!({"text": t});
            }
            obs["specimen"] = json!({"reference": urn(&x.db_uuid, "Specimen", &id)});
            sp
        });
    (specimen, obs)
}

/// The catalog's reference range, in the unit the value is written in, labelled as ours.
fn catalog_range(x: &Ctxt, e: &Evaluated, unit: &str) -> Option<Value> {
    let range = e.reference.as_ref()?;
    let conv = |v: Option<f64>| -> Option<Option<f64>> {
        match v {
            None => Some(None),
            Some(v) => x.cat.conversions.convert(e.marker.id, v, &e.marker.unit, unit).ok().map(Some),
        }
    };
    let (low, high) = (conv(range.low)?, conv(range.high)?);
    let mut out = json!({"type": {"text": "biomarker-cli catalog"}});
    if let Some(l) = low {
        out["low"] = quantity(l, unit);
    }
    if let Some(h) = high {
        out["high"] = quantity(h, unit);
    }
    let cite = range.citations.first().and_then(|c| {
        c.title
            .clone()
            .or_else(|| c.doi.as_ref().map(|d| format!("doi:{d}")))
            .or_else(|| c.pmid.as_ref().map(|p| format!("PMID {p}")))
            .or_else(|| c.url.clone())
    });
    let set = range.set.as_deref().unwrap_or("catalog");
    out["text"] = json!(match cite {
        Some(c) => format!("biomarker-cli {set} reference range (not the lab's); source: {c}"),
        None => format!("biomarker-cli {set} reference range (not the lab's)"),
    });
    Some(out)
}

fn qualitative(x: &Ctxt, ob: &ObservationRow, loinc: Option<&String>) -> Value {
    let mut o = json!({
        "resourceType": "Observation",
        "id": format!("o{}", ob.id),
        "identifier": [{"system": format!("urn:biomarker-cli:{}", x.db_uuid), "value": format!("o{}", ob.id)}],
        "status": "final",
        "category": [{"coding": [{"system": OBSERVATION_CATEGORY, "code": "laboratory", "display": "Laboratory"}]}],
        "code": code_concept(&ob.marker_name, loinc),
        "subject": {"reference": x.patient_ref},
        "effectiveDateTime": date_time(&ob.taken_at, x.tz),
        "meta": {"profile": [US_CORE_LAB]},
    });
    if let Value::Object(v) = qualitative_value(&ob.text) {
        v.into_iter().for_each(|(k, val)| o[k.as_str()] = val);
    }
    if ob.flag.as_deref() == Some("abnormal") {
        o["interpretation"] = json!([interpretation_concept("A")]);
    }
    if let Some(n) = ob.note.as_ref().filter(|n| !n.is_empty()) {
        o["note"] = json!([{"text": n}]);
    }
    if let Some(l) = ob.lab.as_ref().filter(|l| !l.is_empty()) {
        o["performer"] = json!([{"display": l}]);
    }
    if loinc.is_none() {
        o["meta"]["tag"] = json!([{"system": "urn:biomarker-cli:warning", "code": "no-loinc"}]);
    }
    o
}

// ---------------------------------------------------------------------------

/// The Bundle (`type: collection`) for `fhir`, one resource per line for `fhir-ndjson`.
pub fn render(ex: &Exported, ndjson: bool, now: &str) -> Value {
    if ndjson {
        return Value::Array(ex.resources.iter().map(|(_, r)| r.clone()).collect());
    }
    let mut bundle = json!({
        "resourceType": "Bundle",
        "type": "collection",
        "timestamp": now,
        "entry": ex.resources.iter().map(|(u, r)| json!({"fullUrl": u, "resource": r})).collect::<Vec<_>>(),
    });
    let missing = ex
        .resources
        .iter()
        .any(|(_, r)| r["meta"]["tag"].as_array().is_some_and(|t| t.iter().any(|t| t["code"] == "no-loinc")));
    if missing {
        bundle["meta"] = json!({"tag": [{"system": "urn:biomarker-cli:warning", "code": "no-loinc"}]});
    }
    bundle
}

pub fn run(ctx: &Ctx, a: &ExportArgs, rows: &[Evaluated]) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let slug = a
        .filter
        .persons
        .first()
        .cloned()
        .or_else(|| ctx.default_person())
        .ok_or_else(|| AppError::usage("FHIR export needs --person (one Patient per bundle)"))?;
    let person = store::get_person(&db, &slug)?;
    let filter = crate::commands::build_filter(ctx, &db, &cat, &a.filter)?;
    let obs = store::query_observations(&db, &filter)?;
    let opts = Opts {
        ranges: a.ranges,
        interpret_catalog: a.interpret == Some(InterpretArg::Catalog),
        include_name: a.include_name,
        group_reports: a.group_reports,
        explicit_units: !matches!(ctx.resolved.source("unit_system"), crate::config::Source::Default),
        flavor: ctx.range_flavor().to_string(),
    };
    let ex = build(ctx, &db, &cat, &person, rows, &obs, &opts)?;
    ex.warnings.iter().for_each(|w| ctx.warn(&format!("warning: {w}")));
    let ndjson = ctx.out.format == crate::output::Format::FhirNdjson;
    let doc = render(&ex, ndjson, &now_iso());
    if !ctx.quiet() && ctx.out.output.is_some() {
        eprintln!("exported {} measurement(s) and {} qualitative result(s)", rows.len(), obs.len());
    }
    ctx.emit(&Report::list("fhir", vec![]).json_data(doc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_and_date_times() {
        let utc = Tz::parse("UTC").unwrap();
        assert_eq!(date_time("2024-03-05", &utc), "2024-03-05");
        assert_eq!(date_time("2024-03-05T08:30:00", &utc), "2024-03-05T08:30:00Z");
        let est = Tz::parse("-05:00").unwrap();
        assert_eq!(date_time("2024-03-05T08:30:00", &est), "2024-03-05T08:30:00-05:00");
        let ny = Tz::parse("America/New_York").unwrap();
        assert_eq!(date_time("2024-03-05T08:30:00", &ny), "2024-03-05T08:30:00-05:00");
        assert_eq!(date_time("2024-07-05T08:30:00", &ny), "2024-07-05T08:30:00-04:00");
    }

    #[test]
    fn urns_are_stable_and_uuid_shaped() {
        let a = urn("db-1", "Observation", "m7");
        assert_eq!(a, urn("db-1", "Observation", "m7"));
        assert_ne!(a, urn("db-2", "Observation", "m7"));
        assert_eq!(a.len(), "urn:uuid:".len() + 36);
        let id = a.trim_start_matches("urn:uuid:");
        assert_eq!(id.matches('-').count(), 4);
        assert_eq!(id.as_bytes()[14], b'5');
    }

    #[test]
    fn qualitative_known_words_are_coded() {
        assert_eq!(qualitative_value("Negative")["valueCodeableConcept"]["coding"][0]["code"], "NEG");
        assert_eq!(qualitative_value("trace")["valueString"], "trace");
    }

    #[test]
    fn lab_ranges_keep_their_units_and_bounds() {
        let r = lab_range(
            &json!({"low": 70.0, "high": 99.0, "unit": "mg/dL", "text": null, "type": "normal", "applies_to": ["male"], "age_low": 18.0, "age_high": null}),
        );
        assert_eq!(r["low"]["code"], "mg/dL");
        assert_eq!(r["high"]["value"], 99.0);
        assert_eq!(r["type"]["coding"][0]["code"], "normal");
        assert_eq!(r["appliesTo"][0]["text"], "male");
        assert_eq!(r["age"]["low"]["code"], "a");
        assert!(r["age"]["high"].is_null());
        let text_only = lab_range(
            &json!({"low": null, "high": null, "unit": null, "text": "negative", "type": null, "applies_to": []}),
        );
        assert_eq!(text_only["text"], "negative");
        assert!(text_only["low"].is_null());
    }
}
