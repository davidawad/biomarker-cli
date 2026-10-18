//! FHIR R4 and HL7 v2 import and FHIR export, end to end through the binary.
//! Synthetic people and values only (fixtures in tests/fixtures/interop).

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

/// Raw 256-bit test KEK (never use a fixed key outside tests).
const TEST_KEY: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

struct Env {
    dir: TempDir,
}

fn fx(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/interop").join(name).display().to_string()
}

impl Env {
    /// A database with one person, alex (male, born 1984-06-01).
    fn new() -> Self {
        let e = Self { dir: TempDir::new().unwrap() };
        e.run(&["person", "add", "alex", "--sex", "male", "--dob", "1984-06-01"]);
        e
    }

    fn cmd(&self) -> Command {
        let p = |n: &str| -> PathBuf { self.dir.path().join(n) };
        let mut c = Command::cargo_bin("biomarker").unwrap();
        c.env_clear()
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", p("config"))
            .env("XDG_DATA_HOME", p("data"))
            .env("APPDATA", p("appdata"))
            .env("BIOMARKER_DB", p("test.db"))
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("SOURCE_DATE_EPOCH", "1767225600")
            .env("NO_COLOR", "1");
        c
    }

    fn out(&self, args: &[&str]) -> std::process::Output {
        self.cmd().args(args).output().unwrap()
    }

    fn run(&self, args: &[&str]) -> String {
        let out = self.out(args);
        assert!(
            out.status.success(),
            "biomarker {args:?} failed ({:?}):\n{}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }

    fn json(&self, args: &[&str]) -> Value {
        let mut a = args.to_vec();
        a.extend(["-f", "json"]);
        serde_json::from_str(&self.run(&a)).unwrap_or_else(|e| panic!("{args:?} did not print JSON: {e}"))
    }

    fn import(&self, args: &[&str]) -> Value {
        let mut a = vec!["import"];
        a.extend(args);
        self.json(&a)["data"].clone()
    }

    fn data(&self, args: &[&str]) -> Vec<Value> {
        self.json(args)["data"].as_array().unwrap().clone()
    }

    fn query(&self, marker: &str) -> Vec<Value> {
        self.data(&["query", "--person", "alex", "--marker", marker])
    }
}

fn n(v: &Value, k: &str) -> i64 {
    v[k].as_i64().unwrap_or_else(|| panic!("{k} in {v}"))
}

fn near(v: &Value, want: f64) {
    let got = v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"));
    assert!((got - want).abs() < 0.01, "{got} != {want}");
}

#[test]
fn imports_a_fhir_bundle_and_reports_what_it_set_aside() {
    let e = Env::new();
    let catalog_before = e.data(&["range", "list", "glucose"]);
    let dry = e.import(&[&fx("lab-bundle.json"), "--person", "alex", "--dry-run"]);
    assert_eq!(dry["dry_run"], true);
    assert_eq!(n(&dry, "inserted"), 6, "{dry}");
    assert!(e.query("glucose").is_empty(), "a dry run writes nothing");

    let s = e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    assert_eq!(s["format"], "fhir");
    assert_eq!((n(&s, "results"), n(&s, "inserted"), n(&s, "replaced"), n(&s, "skipped")), (11, 6, 0, 0), "{s}");
    assert_eq!((n(&s, "unmapped"), n(&s, "unconvertible"), n(&s, "conflicts")), (1, 1, 0), "{s}");
    assert_eq!(s["skipped_category"]["social-history"], 1);
    assert_eq!(s["skipped_status"]["preliminary"], 1);
    assert_eq!(s["skipped_status"]["retracted (never imported)"], 1);
    assert_eq!(s["ignored"]["Practitioner"], 1);
    assert_eq!(s["ignored"]["Organization"], 1);
    assert_eq!(s["unmapped_results"][0]["result"], "99999-9 (Synthetic mystery analyte)");
    assert!(s["unconvertible_results"][0]["reason"].as_str().unwrap().starts_with("unit_unconvertible"));
    assert!(s["batch"].is_string());

    // a unit converted to the canonical one; the value as reported kept beside it
    let glucose = e.query("glucose");
    assert_eq!(glucose.len(), 2);
    let may = glucose.iter().find(|r| r["taken_at"] == "2024-09-10").unwrap();
    near(&may["value_raw"], 5.5);
    assert_eq!(may["unit_raw"], "mmol/L");
    near(&may["value_canonical"], 99.088);
    // lab, fasting and the lab's own range come from the Specimen, performer and referenceRange
    let first = glucose.iter().find(|r| r["taken_at"] == "2024-03-05T08:30:00").unwrap();
    assert_eq!((first["lab"].as_str(), first["fasting"].as_bool()), (Some("Example Reference Lab"), Some(true)));
    near(&first["lab_ref_low"], 70.0);
    near(&first["lab_ref_high"], 99.0);
    assert_eq!(first["lab_interpretation"], "N");
    assert!(first["lab_flag_agrees"].is_boolean(), "{first}");
    // the lab's range converted into the display unit; its flag kept
    near(&may["lab_ref_low"], 70.26);
    assert_eq!(may["lab_interpretation"], "H");
    // the lab's range never reaches the catalog's cited ranges
    assert_eq!(e.data(&["range", "list", "glucose"]), catalog_before);
    assert!(first["ref_high"].is_number(), "the catalog still flags against its own range: {first}");

    // a censored value, a panel split into its components, a category-appropriate marker
    let tsh = &e.query("tsh")[0];
    assert_eq!(tsh["qualifier"], "<");
    near(&tsh["value"], 0.01);
    assert_eq!(e.query("bp-systolic")[0]["value"], 118.0);
    assert_eq!(e.query("bp-diastolic")[0]["value"], 76.0);

    // a database with no imported result gains no new JSON fields
    let plain = Env::new();
    plain.run(&["add", "glucose", "90", "--person", "alex", "--date", "2024-01-01"]);
    assert!(plain.query("glucose")[0].get("lab_ref_low").is_none());
}

#[test]
fn reimport_is_idempotent_and_amendments_update_in_place() {
    let e = Env::new();
    e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    let again = e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    assert_eq!((n(&again, "inserted"), n(&again, "replaced"), n(&again, "skipped")), (0, 0, 6), "{again}");
    let before = e.query("glucose").iter().find(|r| r["taken_at"] == "2024-03-05T08:30:00").unwrap()["id"].clone();

    // the portal re-issues the result under a new server id, corrected
    let amended = e.import(&[&fx("amended-glucose.json"), "--person", "alex"]);
    assert_eq!((n(&amended, "inserted"), n(&amended, "replaced")), (0, 1), "{amended}");
    let rows = e.query("glucose");
    assert_eq!(rows.len(), 2, "no duplicate row");
    let row = rows.iter().find(|r| r["taken_at"] == "2024-03-05T08:30:00").unwrap();
    assert_eq!((row["value_raw"].as_f64(), &row["id"]), (Some(95.0), &before), "the same row, new value");
    let same = e.import(&[&fx("amended-glucose.json"), "--person", "alex"]);
    assert_eq!((n(&same, "replaced"), n(&same, "skipped")), (0, 1));
}

#[test]
fn retractions_are_reported_and_applied_only_on_request() {
    let e = Env::new();
    e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    assert_eq!(e.query("ldl-c").len(), 1);
    let report = e.import(&[&fx("retracted-ldl.json"), "--person", "alex"]);
    assert_eq!((n(&report, "retracted"), n(&report, "retractions_applied")), (1, 0));
    assert_eq!(e.query("ldl-c").len(), 1, "reported, not deleted");
    let applied = e.import(&[&fx("retracted-ldl.json"), "--person", "alex", "--apply-retractions"]);
    assert_eq!(n(&applied, "retractions_applied"), 1);
    assert!(e.query("ldl-c").is_empty());
}

#[test]
fn unmapped_results_are_reported_skipped_or_become_custom_markers() {
    let e = Env::new();
    let s = e.import(&[&fx("lab-bundle.json"), "--person", "alex", "--unmapped", "skip"]);
    assert_eq!(n(&s, "unmapped"), 1);
    let kept = e.json(&["marker", "list", "--search", "mystery"])["data"].as_array().unwrap().len();
    assert_eq!(kept, 0);

    let e = Env::new();
    let s = e.import(&[&fx("lab-bundle.json"), "--person", "alex", "--unmapped", "create"]);
    assert_eq!((n(&s, "unmapped"), n(&s, "inserted")), (0, 7), "{s}");
    assert_eq!(s["markers_created"][0], "synthetic-mystery-analyte");
    let loinc = e.data(&["marker", "loinc", "list", "synthetic-mystery-analyte"]);
    assert_eq!(loinc[0]["loinc"], "99999-9");
    assert_eq!(loinc[0]["source"], "custom");
    assert_eq!(e.query("synthetic-mystery-analyte")[0]["unit"], "U/L");
}

#[test]
fn loinc_mappings_are_managed_with_marker_loinc() {
    let e = Env::new();
    let glucose = e.data(&["marker", "loinc", "list", "glucose"]);
    assert_eq!((glucose[0]["loinc"].as_str(), glucose[0]["source"].as_str()), (Some("2345-7"), Some("catalog")));
    e.run(&["marker", "loinc", "add", "alt", "99999-9", "--relation", "variant"]);
    let s = e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    assert_eq!((n(&s, "unmapped"), n(&s, "inserted")), (0, 7), "the mystery code now maps to ALT: {s}");
    assert_eq!(e.query("alt")[0]["value"], 3.2);
    // a code already owned by another marker is refused; catalog codes are not removable
    assert!(!e.out(&["marker", "loinc", "add", "ast", "99999-9"]).status.success());
    assert!(!e.out(&["marker", "loinc", "rm", "glucose", "2345-7"]).status.success());
    assert!(!e.out(&["marker", "loinc", "add", "ast", "not-a-code"]).status.success());
    e.run(&["marker", "loinc", "rm", "alt", "99999-9"]);
    assert!(e.data(&["marker", "loinc", "list", "alt"]).iter().all(|r| r["loinc"] != "99999-9"));
}

#[test]
fn ndjson_and_directories_import_with_unit_conversion() {
    let e = Env::new();
    let s = e.import(&[&fx("observations.ndjson"), "--person", "alex"]);
    assert_eq!((s["format"].as_str(), n(&s, "inserted")), (Some("fhir-ndjson"), 2), "{s}");
    near(&e.query("hdl-c")[0]["value_canonical"], 50.27);

    let e = Env::new();
    let dir = TempDir::new().unwrap();
    std::fs::copy(fx("observations.ndjson"), dir.path().join("a.ndjson")).unwrap();
    std::fs::copy(fx("amended-glucose.json"), dir.path().join("b.json")).unwrap();
    let s = e.import(&[dir.path().to_str().unwrap(), "--person", "alex"]);
    assert_eq!(n(&s, "inserted"), 3, "{s}");
}

#[test]
fn imports_hl7_v2_oru_messages() {
    let e = Env::new();
    let s = e.import(&[&fx("oru.hl7"), "--person", "alex"]);
    assert_eq!(s["format"], "hl7v2");
    // the ST urine colour maps to the qualitative urine-color marker (LOINC 5778-6)
    assert_eq!((n(&s, "results"), n(&s, "inserted"), n(&s, "unmapped")), (5, 3, 0), "{s}");
    assert_eq!(s["observations"]["inserted"], 1, "{s}");
    assert_eq!(s["skipped_status"]["preliminary"], 1);
    let chol = &e.query("total-cholesterol")[0];
    assert_eq!(chol["value"], 245.0);
    assert_eq!(chol["note"], "Synthetic fasting sample");
    assert_eq!(chol["lab_interpretation"], "H");
    near(&chol["lab_ref_high"], 200.0);
    assert!(chol["lab_ref_low"].is_null());
    assert_eq!(chol["taken_at"], "2024-03-05T08:00:00", "OBR-7 when the OBX has no time");
    let tg = &e.query("triglycerides")[0];
    assert_eq!((tg["qualifier"].as_str(), tg["value"].as_f64()), (Some("<"), Some(45.0)));
    assert_eq!(e.import(&[&fx("oru.hl7"), "--person", "alex"])["inserted"], 0);
    // preliminary results only on request, tagged
    let s = e.import(&[&fx("oru.hl7"), "--person", "alex", "--include-preliminary"]);
    assert_eq!(n(&s, "inserted"), 1);
    assert_eq!(e.query("hdl-c")[0]["tags"][0], "preliminary");
}

#[test]
fn a_patient_that_does_not_match_the_person_is_refused() {
    let e = Env::new();
    e.run(&["person", "add", "sam", "--sex", "female", "--dob", "1990-01-01"]);
    let out = e.out(&["import", &fx("lab-bundle.json"), "--person", "sam"]);
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("does not match") && err.contains("--force-person"), "{err}");
    assert!(e.data(&["query", "--person", "sam"]).is_empty());
    let s = e.import(&[&fx("lab-bundle.json"), "--person", "sam", "--force-person"]);
    assert_eq!(n(&s, "inserted"), 6);
    assert!(s["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("--force-person")));
    // no person at all is never guessed when several exist
    assert!(!e.out(&["import", &fx("lab-bundle.json")]).status.success());
}

#[test]
fn exports_fhir_and_the_export_reimports_idempotently() {
    let e = Env::new();
    e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    let text = e.run(&["export", "--person", "alex", "-f", "fhir"]);
    let bundle: Value = serde_json::from_str(&text).unwrap();
    assert_eq!((bundle["resourceType"].as_str(), bundle["type"].as_str()), (Some("Bundle"), Some("collection")));
    let entries = bundle["entry"].as_array().unwrap();
    let of = |t: &str| -> Vec<&Value> {
        entries.iter().map(|x| &x["resource"]).filter(|r| r["resourceType"] == t).collect()
    };
    assert_eq!(of("Patient").len(), 1);
    assert_eq!(of("Patient")[0]["gender"], "male");
    assert!(of("Patient")[0].get("name").is_none(), "no name unless asked");
    let obs = of("Observation");
    assert_eq!(obs.len(), 6);
    let glucose = obs
        .iter()
        .find(|o| o["code"]["coding"][0]["code"] == "2345-7" && o["effectiveDateTime"] == "2024-03-05T08:30:00Z")
        .unwrap();
    assert_eq!(glucose["status"], "final");
    assert_eq!(glucose["category"][0]["coding"][0]["code"], "laboratory");
    assert_eq!(glucose["valueQuantity"]["system"], "http://unitsofmeasure.org");
    assert_eq!(glucose["valueQuantity"]["code"], "mg/dL");
    assert_eq!(glucose["referenceRange"][0]["low"]["value"], 70.0);
    assert_eq!(glucose["referenceRange"][0]["high"]["code"], "mg/dL");
    assert_eq!(glucose["interpretation"][0]["coding"][0]["code"], "N");
    assert!(glucose["identifier"][0]["system"].as_str().unwrap().starts_with("urn:biomarker-cli:"));
    assert_eq!(glucose["performer"][0]["display"], "Example Reference Lab");
    let spec = of("Specimen");
    assert!(spec
        .iter()
        .any(|s| s["collection"]["fastingStatusCodeableConcept"]["coding"][0]["code"] == "F"
            && s["type"]["text"] == "Serum"));
    // a value in mmol/L stays in mmol/L, with its UCUM code
    let mmol = obs.iter().find(|o| o["effectiveDateTime"] == "2024-09-10").unwrap();
    assert_eq!(
        (mmol["valueQuantity"]["value"].as_f64(), mmol["valueQuantity"]["code"].as_str()),
        (Some(5.5), Some("mmol/L"))
    );
    let tsh = obs.iter().find(|o| o["code"]["coding"][0]["code"] == "3016-3").unwrap();
    assert_eq!(
        (tsh["valueQuantity"]["comparator"].as_str(), tsh["valueQuantity"]["code"].as_str()),
        (Some("<"), Some("m[IU]/L"))
    );

    // the same database: nothing to add; a fresh one: everything, then nothing
    let file = e.dir.path().join("export.json");
    std::fs::write(&file, &text).unwrap();
    let s = e.import(&[file.to_str().unwrap(), "--person", "alex"]);
    assert_eq!((n(&s, "inserted"), n(&s, "replaced")), (0, 0), "{s}");
    let fresh = Env::new();
    let s = fresh.import(&[file.to_str().unwrap(), "--person", "alex"]);
    assert_eq!((n(&s, "inserted"), n(&s, "unmapped")), (6, 0), "{s}");
    let s = fresh.import(&[file.to_str().unwrap(), "--person", "alex"]);
    assert_eq!((n(&s, "inserted"), n(&s, "replaced"), n(&s, "skipped")), (0, 0, 6));
    let rows = fresh.query("glucose");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows.iter().find(|r| r["taken_at"] == "2024-03-05T08:30:00").unwrap()["fasting"], true);
    near(&rows.iter().find(|r| r["taken_at"] == "2024-09-10").unwrap()["value_canonical"], 99.088);
}

#[test]
fn export_options_ndjson_catalog_ranges_and_custom_markers() {
    let e = Env::new();
    e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    e.run(&["marker", "add", "my-custom", "--unit", "widgets", "--category", "other"]);
    e.run(&["add", "my-custom", "3", "--person", "alex", "--date", "2024-04-01", "--lab", "Home"]);

    let out = e.out(&[
        "export",
        "--person",
        "alex",
        "-f",
        "fhir",
        "--ranges",
        "catalog",
        "--interpret",
        "catalog",
        "--group-reports",
        "--include-name",
    ]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("no LOINC"), "{}", String::from_utf8_lossy(&out.stderr));
    let b: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(b["meta"]["tag"][0]["code"] == "no-loinc");
    let res: Vec<&Value> = b["entry"].as_array().unwrap().iter().map(|x| &x["resource"]).collect();
    let custom = res.iter().find(|r| r["code"]["text"] == "my-custom").unwrap();
    assert!(custom["code"].get("coding").is_none());
    assert_eq!(custom["valueQuantity"]["unit"], "widgets");
    assert!(custom["valueQuantity"].get("code").is_none(), "no UCUM code to claim");
    let glucose = res
        .iter()
        .find(|r| r["code"]["coding"][0]["code"] == "2345-7" && r["effectiveDateTime"] == "2024-03-05T08:30:00Z")
        .unwrap();
    let ranges = glucose["referenceRange"].as_array().unwrap();
    assert_eq!(ranges.len(), 2, "the lab's, then ours");
    assert_eq!(ranges[1]["type"]["text"], "biomarker-cli catalog");
    assert!(ranges[1]["text"].as_str().unwrap().contains("not the lab's"));
    assert!(glucose["interpretation"].as_array().unwrap().iter().any(|i| i["text"] == "biomarker-cli catalog"));
    assert!(res.iter().any(|r| r["resourceType"] == "DiagnosticReport" && r["result"].as_array().unwrap().len() >= 2));

    let nd = e.run(&["export", "--person", "alex", "-f", "fhir-ndjson", "--ranges", "none"]);
    let lines: Vec<Value> = nd.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert_eq!(lines[0]["resourceType"], "Patient");
    assert!(lines.iter().filter(|r| r["resourceType"] == "Observation").all(|r| r.get("referenceRange").is_none()));

    // one Patient per bundle: a person is required, and fhir is for export only
    let e2 = Env::new();
    e2.run(&["person", "add", "sam"]);
    assert!(!e2.out(&["export", "-f", "fhir"]).status.success());
    assert!(!e2.out(&["query", "--all-people", "-f", "fhir"]).status.success());
}

#[test]
fn an_explicit_unit_system_exports_canonical_values() {
    let e = Env::new();
    e.import(&[&fx("lab-bundle.json"), "--person", "alex"]);
    let b: Value =
        serde_json::from_str(&e.run(&["export", "--person", "alex", "-f", "fhir", "--units", "canonical"])).unwrap();
    let mmol = b["entry"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| &x["resource"])
        .find(|r| r["effectiveDateTime"] == "2024-09-10")
        .unwrap();
    assert_eq!(mmol["valueQuantity"]["code"], "mg/dL");
    near(&mmol["valueQuantity"]["value"], 99.088);
}
