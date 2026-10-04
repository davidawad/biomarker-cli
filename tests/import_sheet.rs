//! Spreadsheet import: synthetic workbooks generated with rust_xlsxwriter.

mod support;

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

use support::sheets;

/// Raw 256-bit test KEK (never use a fixed key outside tests).
const TEST_KEY: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

struct Env {
    dir: TempDir,
}

impl Env {
    fn new() -> Self {
        let e = Self { dir: TempDir::new().unwrap() };
        e.run(&["person", "add", "alex", "--sex", "male", "--dob", "1984-06-01"]);
        e
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    /// The synthetic dashboard workbook (sheets "Labs" and "Body").
    fn workbook(&self) -> String {
        let p = self.path("dashboard.xlsx");
        if !p.exists() {
            sheets::dashboard(&p).unwrap();
        }
        p.to_string_lossy().into_owned()
    }

    fn cmd(&self) -> Command {
        let mut c = Command::cargo_bin("biomarker").unwrap();
        c.env_clear()
            // Windows system DLLs expect SYSTEMROOT even in a cleared environment.
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("BIOMARKER_DB", self.path("test.db"))
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("BIOMARKER_NO_KEYCHAIN", "1")
            .env("NO_COLOR", "1");
        c
    }

    /// Run successfully; (stdout, stderr).
    fn run_io(&self, args: &[&str]) -> (String, String) {
        let out = self.cmd().args(args).output().unwrap();
        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(out.status.success(), "biomarker {args:?} failed ({:?}):\n{stderr}", out.status.code());
        (String::from_utf8(out.stdout).unwrap(), stderr)
    }

    fn run(&self, args: &[&str]) -> String {
        self.run_io(args).0
    }

    fn json(&self, args: &[&str]) -> Value {
        let mut a = args.to_vec();
        a.extend(["--format", "json"]);
        serde_json::from_str(&self.run(&a)).unwrap()
    }

    fn query(&self, marker: &str) -> Vec<Value> {
        self.json(&["query", "-p", "alex", "-m", marker])["data"].as_array().unwrap().clone()
    }
}

fn repo(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(name).to_string_lossy().into_owned()
}

fn approx(v: &Value, expected: f64) {
    let got = v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"));
    assert!((got - expected).abs() < 1e-6 * expected.abs().max(1.0), "got {got}, expected {expected}");
}

fn sources(v: &Value, key: &str) -> Vec<String> {
    v["data"][key].as_array().unwrap().iter().map(|x| x["source"].as_str().unwrap().to_string()).collect()
}

const BODY_COLUMNS: &[&str] = &[
    "--value-column",
    "weight=Weight (lbs.):lb",
    "--value-column",
    "bmi=BMI kg/m²",
    "--value-column",
    "body-fat=Body Fat %",
    "--value-column",
    "biological-age=Biological Age",
    "--value-column",
    "resting-hr=Resting HR",
];

#[test]
fn lists_sheets_with_dimensions() {
    let e = Env::new();
    let v = e.json(&["import", &e.workbook(), "--list-sheets"]);
    let names: Vec<&str> = v["data"].as_array().unwrap().iter().map(|s| s["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["Labs", "Body"]);
    assert_eq!(v["data"][0]["dimensions"], "A1:K23");
    assert_eq!(v["data"][1]["rows"], 5);
    assert_eq!(v["data"][1]["columns"], 6);
    e.cmd().args(["import", &repo("examples/wide.csv"), "--list-sheets"]).assert().code(2);
}

fn matched_pairs(v: &Value) -> Vec<(String, String)> {
    v["data"]["matched"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| (m["source"].as_str().unwrap().into(), m["marker"].as_str().unwrap().into()))
        .collect()
}

#[test]
fn transposed_dashboard_report() {
    let e = Env::new();
    let v = e.json(&["import", &e.workbook(), "-p", "alex"]);
    let d = &v["data"];
    assert_eq!(d["layout"], "transposed", "{v}");
    assert_eq!(d["sheet"], "Labs");
    assert_eq!(d["header_row"], 3);
    assert_eq!(d["inserted"], 43);
    assert_eq!(d["invalid"], 0);
    let matched = matched_pairs(&v);
    for (src, slug) in [
        ("Low-Density Lipoprotein (LDL-C)", "ldl-c"),
        ("ALT (SGPT)", "alt"),
        ("White Blood Cell Count (WBC)", "wbc"),
        ("Apolipoprotein B (ApoB)", "apob"),
        ("Thyroid Stimulating Hormone (TSH)", "tsh"),
    ] {
        assert!(matched.contains(&(src.into(), slug.into())), "{src} -> {slug} missing from {matched:?}");
    }
    // unknown tests are reported and skipped, not invalid
    assert_eq!(
        sources(&v, "unmatched"),
        ["Lipoprotein Particle Score", "Urine Protein", "Urine Appearance", "WBC, Urine", "Control Sample"]
    );
    // header dates: formatted serial, bare serial, ISO text, US text
    assert_eq!(
        d["cells_per_date"],
        serde_json::json!({"2024-03-01": 11, "2024-08-15": 11, "2024-12-02": 10, "2025-03-11": 11})
    );
}

#[test]
fn transposed_dashboard_units_values_and_ranges() {
    let e = Env::new();
    e.run(&["import", &e.workbook(), "-p", "alex"]);
    // units normalised to catalog spellings; 1:1 units collapse to the canonical one
    let unit = |m: &str| e.query(m)[0]["unit_raw"].as_str().unwrap().to_string();
    assert_eq!(unit("wbc"), "10^3/µL");
    assert_eq!(unit("rbc"), "10^6/µL");
    assert_eq!(unit("platelets"), "10^3/µL");
    assert_eq!(unit("mcv"), "fL");
    assert_eq!(unit("hba1c"), "%");
    assert_eq!(unit("alt"), "U/L");
    assert_eq!(unit("tsh"), "mIU/L");
    let tsh = e.query("tsh");
    assert_eq!(tsh.len(), 3);
    approx(&tsh[2]["value"], 2.3);
    assert_eq!(tsh[2]["taken_at"], "2025-03-11");
    // catalog ranges apply (no ranges = "sheet")
    approx(&e.query("ldl-c")[0]["ref_high"], 100.0);
    assert_eq!(e.json(&["observations"])["count"], 0);
}

#[test]
fn stop_at_ignores_the_named_row_and_everything_below() {
    let e = Env::new();
    let mapping = e.path("stop.toml");
    std::fs::write(&mapping, "stop_at = \"Urinalysis\"\n").unwrap();
    let (_, stderr) =
        e.run_io(&["import", &e.workbook(), "-p", "alex", "--create-markers", "--mapping", mapping.to_str().unwrap()]);
    assert!(stderr.contains("stop_at"), "{stderr}");
    let created: Vec<String> = e.json(&["marker", "list"])["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["slug"].as_str().unwrap().to_string())
        .collect();
    assert!(!created.iter().any(|s| s.starts_with("urine") || s.contains("urine")), "{created:?}");
    assert!(created.iter().any(|s| s == "lipoprotein-particle-score"), "rows above the stop are imported");
}

#[test]
fn created_markers_take_their_section_category() {
    let e = Env::new();
    let v = e.json(&["import", &e.workbook(), "-p", "alex", "--create-markers", "--qualitative", "store"]);
    let created = v["data"]["markers_created"].as_array().unwrap();
    assert_eq!(created.len(), 5, "{v}");
    let show = |m: &str| e.json(&["marker", "show", m])["data"].clone();
    assert_eq!(show("lipoprotein-particle-score")["category"], "lipid");
    assert_eq!(show("lipoprotein-particle-score")["unit"], "score");
    assert_eq!(show("urine-protein")["category"], "urinalysis");
    assert_eq!(show("wbc-urine")["name"], "WBC, Urine");
    assert_eq!(v["data"]["qualitative"], 11);
    assert_eq!(v["data"]["observations"]["inserted"], 11);
    // without sections the category falls back to "other"
    let e2 = Env::new();
    e2.run(&["import", &e2.workbook(), "-p", "alex", "--create-markers", "--sections-as-category", "false"]);
    assert_eq!(e2.json(&["marker", "show", "urine-protein"])["data"]["category"], "other");
}

#[test]
fn qualitative_values_are_counted_or_stored() {
    let e = Env::new();
    let book = e.workbook();
    let map = e.path("q.toml");
    std::fs::write(&map, "[rename]\n\"Urine Protein\" = \"urine-protein\"\n").unwrap();
    e.run(&["marker", "add", "urine-protein", "--name", "Urine Protein", "--unit", "ratio", "--category", "urine"]);
    let v = e.json(&["import", &book, "-p", "alex", "--mapping", map.to_str().unwrap(), "--dry-run"]);
    assert_eq!(v["data"]["qualitative"], 4);
    assert_eq!(v["data"]["qualitative_values"][2]["text"], "1+ Abnormal");
    assert_eq!(v["data"]["observations"], serde_json::json!({}));

    let v = e.json(&["import", &book, "-p", "alex", "--qualitative", "store"]);
    assert_eq!(v["data"]["observations"]["inserted"], 4);
    let obs = e.json(&["observations", "-m", "urine-protein"]);
    assert_eq!(obs["count"], 4);
    let abnormal = e.json(&["observations", "--flagged"]);
    assert_eq!(abnormal["count"], 1);
    assert_eq!(abnormal["data"][0]["text"], "1+ Abnormal");
    assert_eq!(abnormal["data"][0]["flag"], "abnormal");
    // JSON exports carry observations beside the measurements
    let x = e.json(&["export", "-p", "alex"]);
    assert_eq!(x["observations"].as_array().unwrap().len(), 4);
    assert_eq!(x["count"], 43);
}

#[test]
fn long_sheet_with_value_columns() {
    let e = Env::new();
    let mut args = vec!["import", "--sheet", "body", "-p", "alex"];
    let book = e.workbook();
    args.insert(1, &book);
    args.extend(BODY_COLUMNS);
    let v = e.json(&args);
    assert_eq!(v["data"]["layout"], "long", "{v}");
    assert_eq!(v["data"]["rows"], 4);
    assert_eq!(v["data"]["inserted"], 19);
    let w = e.query("weight");
    assert_eq!(w.len(), 4);
    assert_eq!(w[0]["taken_at"], "2024-01-06");
    assert_eq!(w[0]["unit_raw"], "lb");
    approx(&w[0]["value_raw"], 184.2);
    approx(&w[0]["value_canonical"], 184.2 * 0.453_592_37);
    assert_eq!(w[0]["unit_canonical"], "kg");
    assert_eq!(e.query("biological-age").len(), 3);
    assert_eq!(e.query("resting-hr")[0]["unit"], "bpm");
    approx(&e.query("bmi")[3]["value"], 24.6);

    // the same columns from a mapping file; the unit comes from "(lbs.)"
    let e2 = Env::new();
    let map = e2.path("body.toml");
    std::fs::write(
        &map,
        "sheet = \"Body\"\nlayout = \"long\"\n[value_columns]\nweight = \"Weight (lbs.)\"\nhrv = \"HRV\"\n",
    )
    .unwrap();
    let out =
        e2.cmd().args(["import", &e2.workbook(), "-p", "alex", "--mapping", map.to_str().unwrap()]).output().unwrap();
    assert_eq!(out.status.code(), Some(2), "a missing value column is a usage error");
    assert!(String::from_utf8_lossy(&out.stderr).contains("value column 'HRV'"));
    std::fs::write(&map, "sheet = \"Body\"\n[value_columns]\nweight = \"Weight (lbs.)\"\n").unwrap();
    let v = e2.json(&["import", &e2.workbook(), "-p", "alex", "--mapping", map.to_str().unwrap()]);
    assert_eq!(v["data"]["inserted"], 4);
    approx(&e2.query("weight")[3]["value_canonical"], 176.4 * 0.453_592_37);
}

/// Import the synthetic dashboard with examples/dashboard-mapping.toml; returns the report.
fn import_with_mapping(e: &Env) -> Value {
    let (book, map) = (e.workbook(), repo("examples/dashboard-mapping.toml"));
    e.json(&["import", &book, "-p", "alex", "--mapping", &map, "--create-markers", "--qualitative", "store"])
}

#[test]
fn mapping_renames_dates_and_skip() {
    let e = Env::new();
    let v = import_with_mapping(&e);
    let d = &v["data"];
    assert_eq!(d["inserted"], 43, "{v}");
    assert_eq!(d["skipped_names"], serde_json::json!(["Lipoprotein Particle Score", "Control Sample"]));
    assert_eq!(d["markers_created"], serde_json::json!(["urine-protein", "urine-appearance", "urine-wbc"]));
    assert_eq!(d["date_corrections"], serde_json::json!([{"from": "2024-12-02", "to": "2024-12-04", "cells": 13}]));
    assert!(d["cells_per_date"].get("2024-12-02").is_none());
    assert_eq!(e.json(&["marker", "show", "urine-wbc"])["data"]["category"], "urine");
    assert_eq!(e.query("ldl-c")[2]["taken_at"], "2024-12-04");
    // "6-10 Abnormal" keeps its range on the observation
    let obs = e.json(&["observations", "-m", "urine-wbc", "--flagged"]);
    assert_eq!(obs["data"][0]["range_low"], 6.0);
    assert_eq!(obs["data"][0]["range_high"], 10.0);
    assert_eq!(obs["data"][0]["note"], "range 6-10");
}

#[test]
fn mapping_sheet_ranges_are_person_specific() {
    let e = Env::new();
    let v = import_with_mapping(&e);
    // ranges = "sheet": person ranges replace the catalog's for alex only
    assert_eq!(v["data"]["ranges_set"].as_array().unwrap().len(), 12);
    let ldl = e.query("ldl-c");
    approx(&ldl[0]["ref_high"], 99.0);
    assert_eq!(ldl[0]["ref_flag"], "high");
    approx(&e.query("tsh")[0]["ref_high"], 4.5);
    approx(&e.query("platelets")[0]["ref_low"], 150.0);
    let ranges = e.json(&["range", "list", "ldl-c", "-p", "alex"]);
    assert_eq!(ranges["data"][0]["person"], "alex");
    e.run(&["person", "add", "sam"]);
    e.run(&["add", "ldl-c", "99.5", "-p", "sam", "--date", "2024-01-01"]);
    let sam = e.json(&["query", "-p", "sam"]);
    approx(&sam["data"][0]["ref_high"], 100.0);
}

fn person_range_count(e: &Env, person: &str) -> usize {
    e.json(&["range", "list", "-p", person])["data"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["person"] == person)
        .count()
}

#[test]
fn mapping_import_is_idempotent() {
    let e = Env::new();
    import_with_mapping(&e);
    let again = import_with_mapping(&e);
    assert_eq!(again["data"]["inserted"], 0);
    assert_eq!(again["data"]["skipped"], 43);
    assert_eq!(again["data"]["observations"]["skipped"], 11);
    assert_eq!(again["data"]["markers_created"], serde_json::json!([]));
    assert_eq!(e.json(&["query", "-p", "alex"])["count"], 43);
    assert_eq!(e.json(&["observations"])["count"], 11);
    assert_eq!(person_range_count(&e, "alex"), 12);
    assert_eq!(e.json(&["db", "check"])["data"]["ok"], true);
}

#[test]
fn header_row_and_columns_by_letter() {
    let e = Env::new();
    let book = e.workbook();
    let v = e.json(&[
        "import",
        &book,
        "-p",
        "alex",
        "--layout",
        "transposed",
        "--header-row",
        "3",
        "--marker-col",
        "A",
        "--unit-col",
        "D",
        "--ref-low-col",
        "F",
        "--ref-high-col",
        "g",
        "--ranges",
        "sheet",
        "--dry-run",
    ]);
    assert_eq!(v["data"]["inserted"], 43);
    assert_eq!(v["data"]["ranges_set"].as_array().unwrap().len(), 11);
    assert_eq!(e.json(&["query"])["count"], 0, "dry run writes nothing");
    assert_eq!(e.json(&["range", "list", "-p", "alex"])["data"][0]["person"], Value::Null);
    // a header row without dates is not a transposed header
    e.cmd().args(["import", &book, "-p", "alex", "--layout", "transposed", "--header-row", "4"]).assert().code(4);
    e.cmd().args(["import", &book, "--sheet", "Nope"]).assert().code(3);
    e.cmd().args(["import", &repo("examples/wide.csv"), "--layout", "transposed"]).assert().code(2);
}

/// The dry-run report is stable: golden file, and the committed example
/// workbook still matches the generator.
#[test]
fn dry_run_report_golden() {
    let e = Env::new();
    let args = |book: &str| -> Vec<String> {
        ["import", book, "-p", "alex", "--mapping", "examples/dashboard-mapping.toml", "--create-markers", "--dry-run"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    };
    let (stdout, report) = e.run_io(&args("examples/dashboard.xlsx").iter().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(stdout, "", "table mode prints the report on stderr only");
    let golden = std::fs::read_to_string(repo("tests/golden/import-dry-run.txt")).unwrap();
    assert!(report == golden, "dry-run report changed:\n--- expected\n{golden}\n--- actual\n{report}");
    assert_eq!(e.json(&["query"])["count"], 0);

    let fresh = e.workbook();
    let (_, regenerated) = e.run_io(&args(&fresh).iter().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(
        regenerated.replace(&fresh, "examples/dashboard.xlsx"),
        report,
        "examples/dashboard.xlsx is stale; run `cargo run --example make-sheets`"
    );
}

#[test]
fn spreadsheet_data_stays_encrypted() {
    let e = Env::new();
    let book = e.workbook();
    e.run(&["import", &book, "-p", "alex", "--create-markers", "--qualitative", "store", "--ranges", "sheet"]);
    let bytes = std::fs::read(e.path("test.db")).unwrap();
    for needle in ["1+ Abnormal", "None seen", "urine-protein", "from spreadsheet", "Lipoprotein Particle Score"] {
        assert!(!bytes.windows(needle.len()).any(|w| w == needle.as_bytes()), "database contains plaintext {needle:?}");
    }
    assert_eq!(e.json(&["observations"])["count"], 11);
}
