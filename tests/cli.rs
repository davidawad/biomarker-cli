//! End-to-end tests driving the `biomarker` binary.

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::TempDir;

/// Raw 256-bit test KEK (never use a fixed key outside tests).
const TEST_KEY: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

struct Env {
    dir: TempDir,
}

impl Env {
    fn new() -> Self {
        Self { dir: TempDir::new().unwrap() }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    fn db(&self) -> PathBuf {
        self.path("test.db")
    }

    /// A command isolated from the user's environment and config.
    fn cmd(&self) -> Command {
        let mut c = Command::cargo_bin("biomarker").unwrap();
        c.env_clear()
            // Windows system DLLs expect SYSTEMROOT even in a cleared environment.
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("BIOMARKER_DB", self.db())
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("BIOMARKER_NO_KEYCHAIN", "1")
            .env("NO_COLOR", "1");
        c
    }

    fn run(&self, args: &[&str]) -> String {
        let out = self.cmd().args(args).output().unwrap();
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
        a.extend(["--format", "json"]);
        let v: Value = serde_json::from_str(&self.run(&a)).unwrap();
        assert_eq!(v["schema"], "biomarker/v1");
        v
    }

    fn people(&self) {
        self.run(&["person", "add", "alex", "--sex", "male", "--dob", "1984-06-01", "--name", "Alex Example"]);
        self.run(&["person", "add", "sam", "--sex", "female", "--dob", "1991-09-23"]);
    }
}

fn examples() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn example(name: &str) -> String {
    examples().join(name).to_string_lossy().into_owned()
}

fn approx(v: &Value, expected: f64) {
    let got = v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"));
    assert!((got - expected).abs() < 1e-6, "expected {expected}, got {got}");
}

#[test]
fn init_and_catalog_seeded() {
    let e = Env::new();
    let v = e.json(&["db", "init"]);
    assert_eq!(v["data"]["created"], true);
    let markers = e.json(&["marker", "list"]);
    let slugs: Vec<&str> = markers["data"].as_array().unwrap().iter().map(|m| m["slug"].as_str().unwrap()).collect();
    for s in
        ["ldl-c", "hdl-c", "apob", "lpa", "hba1c", "hscrp", "vitamin-d", "ferritin", "tsh", "testosterone-total", "wbc"]
    {
        assert!(slugs.contains(&s), "missing {s}");
    }
    let show = e.json(&["marker", "show", "LDL"]);
    assert_eq!(show["data"]["slug"], "ldl-c");
    assert!(show["data"]["convertible_units"].as_array().unwrap().iter().any(|u| u == "mmol/L"));
    let check = e.json(&["db", "check"]);
    assert_eq!(check["data"]["ok"], true);
}

#[test]
fn source_date_epoch_pins_generated_at() {
    let e = Env::new();
    let out = e.cmd().args(["person", "list", "-f", "json"]).env("SOURCE_DATE_EPOCH", "1767225600").output().unwrap();
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["generated_at"], "2026-01-01T00:00:00Z");
}

#[test]
fn person_crud() {
    let e = Env::new();
    e.people();
    let list = e.json(&["person", "list"]);
    assert_eq!(list["count"], 2);
    e.run(&["person", "edit", "sam", "--rename", "samantha", "--add-tag", "family,example", "--notes", "hello"]);
    let show = e.json(&["person", "show", "samantha"]);
    assert_eq!(show["data"]["notes"], "hello");
    assert_eq!(show["data"]["tags"], serde_json::json!(["family", "example"]));
    e.run(&["add", "ferritin", "40", "-p", "samantha", "-d", "2024-01-01"]);
    e.cmd().args(["person", "rm", "samantha"]).assert().code(4);
    e.run(&["person", "rm", "samantha", "--force"]);
    e.cmd().args(["person", "show", "samantha"]).assert().code(3);
    e.cmd().args(["person", "add", "alex"]).assert().code(4).stderr(predicate::str::contains("already exists"));
}

#[test]
fn unit_conversion_on_add_and_display() {
    let e = Env::new();
    e.people();
    let v = e.json(&["add", "glucose", "5.5", "mmol/L", "-p", "alex", "-d", "2024-03-01"]);
    approx(&v["data"]["value_canonical"], 5.5 * 18.016);
    assert_eq!(v["data"]["unit_raw"], "mmol/L");
    assert_eq!(v["data"]["unit_canonical"], "mg/dL");
    // HbA1c IFCC -> NGSP is affine
    let v = e.json(&["add", "a1c", "48", "mmol/mol", "-p", "alex", "-d", "2024-03-01"]);
    approx(&v["data"]["value_canonical"], 48.0 * 0.09148 + 2.152);
    // testosterone nmol/L -> ng/dL
    let v = e.json(&["add", "testosterone", "20", "nmol/l", "-p", "alex", "-d", "2024-03-01"]);
    approx(&v["data"]["value_canonical"], 576.8);
    // SI display converts back
    let q = e.json(&["query", "-m", "glucose", "--units", "si"]);
    approx(&q["data"][0]["value"], 5.5);
    assert_eq!(q["data"][0]["unit"], "mmol/L");
    assert_eq!(q["unit_system"], "si");
    // unknown unit is rejected with exit code 4
    e.cmd().args(["add", "glucose", "5", "furlongs", "-p", "alex"]).assert().code(4);
    // unit convert helper
    let c = e.json(&["unit", "convert", "200", "mg/dL", "mmol/L", "--marker", "total-cholesterol"]);
    approx(&c["data"]["result"], 200.0 / 38.67);
}

#[test]
fn custom_conversion_and_marker() {
    let e = Env::new();
    e.people();
    e.run(&["marker", "add", "omega3-index", "--name", "Omega-3 Index", "--unit", "%", "--category", "fatty-acid"]);
    e.run(&["marker", "alias", "omega3-index", "o3i", "omega-3"]);
    e.run(&["range", "set", "o3i", "--kind", "optimal", "--low", "8"]);
    e.run(&["unit", "add-conversion", "--marker", "o3i", "--from", "fraction", "--to", "%", "--factor", "100"]);
    let v = e.json(&["add", "omega-3", "0.065", "fraction", "-p", "sam", "-d", "2024-05-05"]);
    approx(&v["data"]["value_canonical"], 6.5);
    assert_eq!(v["data"]["opt_flag"], "low");
    e.cmd().args(["marker", "rm", "o3i"]).assert().code(4);
    e.run(&["marker", "rm", "o3i", "--force"]);
    e.cmd().args(["marker", "show", "omega3-index"]).assert().code(3);
}

#[test]
fn range_flagging_by_sex_and_flavor() {
    let e = Env::new();
    e.people();
    // HDL 45: male ref >= 40 normal, female ref >= 50 low
    e.run(&["add", "hdl", "45", "-p", "alex", "-d", "2024-01-01"]);
    e.run(&["add", "hdl", "45", "-p", "sam", "-d", "2024-01-01"]);
    e.run(&["add", "ldl", "130", "-p", "alex", "-d", "2024-01-01"]);
    e.run(&["add", "hscrp", "<0.3", "-p", "alex", "-d", "2024-01-01"]);
    let q = e.json(&["query", "--all-people"]);
    let flag = |person: &str, marker: &str, key: &str| {
        q["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["person"] == person && r["marker"] == marker)
            .map(|r| r[key].clone())
            .unwrap()
    };
    assert_eq!(flag("alex", "hdl-c", "ref_flag"), "normal");
    assert_eq!(flag("sam", "hdl-c", "ref_flag"), "low");
    assert_eq!(flag("alex", "hdl-c", "opt_flag"), "low");
    assert_eq!(flag("alex", "ldl-c", "flag"), "high");
    assert_eq!(flag("alex", "hscrp", "qualifier"), "<");
    assert_eq!(flag("alex", "hscrp", "flag"), "normal");

    let f = e.json(&["flag"]);
    assert_eq!(f["count"], 2); // sam hdl low, alex ldl high
    let f = e.json(&["flag", "--range-flavor", "optimal"]);
    assert_eq!(f["count"], 3); // both HDLs below 60, LDL above 70
    e.cmd().args(["flag", "--exit-code", "-q"]).assert().code(10);
    e.cmd().args(["flag", "--exit-code", "-q", "-m", "hscrp"]).assert().code(0);

    // age-banded custom range overrides the generic one
    e.run(&["range", "set", "hdl", "--sex", "male", "--age-min", "35", "--age-max", "50", "--low", "50"]);
    let q = e.json(&["query", "-p", "alex", "-m", "hdl"]);
    assert_eq!(q["data"][0]["ref_flag"], "low");
    approx(&q["data"][0]["ref_low"], 50.0);
    // ranges given in another unit are stored canonically
    e.run(&["range", "set", "ldl", "--kind", "optimal", "--high", "1.8", "--unit", "mmol/L"]);
    let r = e.json(&["range", "list", "ldl", "--kind", "optimal"]);
    approx(&r["data"][0]["high"], 1.8 * 38.67);
}

#[test]
fn csv_import_export_round_trip() {
    let e = Env::new();
    e.people();
    let out = e.json(&["import", &example("measurements.csv")]);
    assert_eq!(out["data"]["inserted"], 29);
    let csv1 = e.run(&["export"]);
    assert_eq!(csv1.lines().count(), 30);
    assert!(csv1.starts_with("person,marker,date,qualifier,value,unit,lab,fasting,note,tags\n"));
    let file = e.path("export.csv");
    e.run(&["export", "-o", file.to_str().unwrap()]);

    // import into a fresh database and export again: identical
    let e2 = Env::new();
    let v = e2.json(&["import", file.to_str().unwrap(), "--create-people"]);
    assert_eq!(v["data"]["inserted"], 29);
    assert_eq!(v["data"]["people_created"], serde_json::json!(["alex", "sam"]));
    assert_eq!(e2.run(&["export"]), csv1);

    // re-import is deduplicated
    let again = e.json(&["import", file.to_str().unwrap()]);
    assert_eq!(again["data"]["skipped"], 29);
    assert_eq!(again["data"]["inserted"], 0);
}

#[test]
fn json_and_jsonl_round_trip() {
    let e = Env::new();
    e.people();
    e.run(&["import", &example("measurements.json")]);
    e.run(&["import", &example("measurements.jsonl")]);
    let j1 = e.json(&["export"]);
    assert_eq!(j1["kind"], "export");
    assert_eq!(j1["count"], 8);
    let file = e.path("export.json");
    std::fs::write(&file, serde_json::to_string(&j1).unwrap()).unwrap();
    let jl = e.path("export.jsonl");
    e.run(&["export", "-f", "jsonl", "-o", jl.to_str().unwrap()]);

    for f in [&file, &jl] {
        let e2 = Env::new();
        e2.run(&["import", f.to_str().unwrap(), "--create-people"]);
        let j2 = e2.json(&["export"]);
        assert_eq!(j1["data"], j2["data"], "round trip through {}", f.display());
    }
    // censored value and tags survive
    let rows = j1["data"].as_array().unwrap();
    let crp = rows.iter().find(|r| r["marker"] == "hscrp").unwrap();
    assert_eq!(crp["qualifier"], "<");
    approx(&crp["value"], 0.3);
    let ldl = rows.iter().find(|r| r["marker"] == "ldl-c").unwrap();
    assert_eq!(ldl["tags"], serde_json::json!(["follow-up"]));
    assert_eq!(ldl["fasting"], true);
}

#[test]
fn import_dry_run_and_dedupe_policies() {
    let e = Env::new();
    e.people();
    let csv = e.path("in.csv");
    std::fs::write(
        &csv,
        "person,marker,date,value,unit\nalex,ldl,2024-01-01,100,mg/dL\nalex,ldl,2024-02-01,90,mg/dL\n",
    )
    .unwrap();
    let dry = e.json(&["import", csv.to_str().unwrap(), "--dry-run"]);
    assert_eq!(dry["data"]["dry_run"], true);
    assert_eq!(dry["data"]["inserted"], 2);
    assert_eq!(e.json(&["query"])["count"], 0);

    e.run(&["import", csv.to_str().unwrap()]);
    std::fs::write(&csv, "person,marker,date,value,unit\nalex,ldl,2024-01-01,2.0,mmol/L\n").unwrap();
    e.cmd().args(["import", csv.to_str().unwrap(), "--dedupe", "error"]).assert().code(4);
    let r = e.json(&["import", csv.to_str().unwrap(), "--dedupe", "replace"]);
    assert_eq!(r["data"]["replaced"], 1);
    let q = e.json(&["query", "-m", "ldl", "--sort", "date"]);
    approx(&q["data"][0]["value_canonical"], 2.0 * 38.67);
    assert_eq!(q["count"], 2);

    // invalid rows abort the whole import unless --skip-invalid
    std::fs::write(&csv, "person,marker,date,value\nalex,ldl,2024-03-01,abc\nalex,ldl,2024-04-01,80\n").unwrap();
    e.cmd().args(["import", csv.to_str().unwrap()]).assert().code(4);
    assert_eq!(e.json(&["query"])["count"], 2);
    let s = e.json(&["import", csv.to_str().unwrap(), "--skip-invalid"]);
    assert_eq!(s["data"]["invalid"], 1);
    assert_eq!(s["data"]["inserted"], 1);
    // filter by import batch
    let batch = s["data"]["batch"].as_str().unwrap().to_string();
    assert_eq!(e.json(&["query", "--batch", &batch])["count"], 1);
}

#[test]
fn import_with_mapping_file_and_flags() {
    let e = Env::new();
    e.people();
    let v = e.json(&["import", &example("lab-report.csv"), "--mapping", &example("lab-report-mapping.toml")]);
    assert_eq!(v["data"]["inserted"], 4, "{v}");
    let q = e.json(&["query", "-p", "alex", "-m", "vitamin-d"]);
    assert_eq!(q["data"][0]["taken_at"], "2025-04-15");
    assert_eq!(q["data"][0]["lab"], "Example Reference Lab");
    assert_eq!(q["data"][0]["fasting"], true);

    // the same file using --map flags plus --input-date-format, with a default person
    let e2 = Env::new();
    e2.people();
    let csv = e2.path("x.csv");
    std::fs::write(&csv, "Analyte;Result;Drawn\nLDL;101;15.04.2025\nTSH;2,1;15.04.2025\n").unwrap();
    let v = e2.json(&[
        "import",
        csv.to_str().unwrap(),
        "--input-delimiter",
        ";",
        "--map",
        "marker=Analyte,date=Drawn",
        "--input-date-format",
        "%d.%m.%Y",
        "--person",
        "sam",
    ]);
    assert_eq!(v["data"]["inserted"], 2);
    assert_eq!(v["data"]["invalid"], 0);
    // "2,1" is read as a decimal comma
    let q = e2.json(&["query", "-p", "sam", "-m", "tsh"]);
    approx(&q["data"][0]["value"], 2.1);
}

#[test]
fn wide_import() {
    let e = Env::new();
    e.people();
    let v = e.json(&["import", &example("wide.csv"), "--wide"]);
    assert_eq!(v["data"]["inserted"], 5);
    let q = e.json(&["query", "-p", "alex", "-m", "glucose"]);
    approx(&q["data"][0]["value_canonical"], 5.0 * 18.016);
}

#[test]
fn query_filters_multi_person() {
    let e = Env::new();
    e.people();
    e.run(&["import", &example("measurements.csv")]);
    assert_eq!(e.json(&["query"])["count"], 29);
    assert_eq!(e.json(&["query", "-p", "sam"])["count"], 11);
    assert_eq!(e.json(&["query", "-p", "alex", "-m", "ldl,hdl"])["count"], 5);
    assert_eq!(e.json(&["query", "-c", "lipid"])["count"], 11);
    assert_eq!(e.json(&["query", "--from", "2024-01-01", "--to", "2024-03-05"])["count"], 7);
    assert_eq!(e.json(&["query", "--tag", "annual"])["count"], 13);
    let latest = e.json(&["latest", "-p", "alex", "-m", "ldl"]);
    assert_eq!(latest["count"], 1);
    assert_eq!(latest["data"][0]["taken_at"], "2024-03-05");
    let top = e.json(&["query", "-m", "ferritin", "--sort", "value", "-r", "-n", "1"]);
    approx(&top["data"][0]["value"], 61.0);
    // default person from config
    e.run(&["config", "set", "default_person", "sam"]);
    assert_eq!(e.json(&["query"])["count"], 11);
    assert_eq!(e.json(&["query", "--all-people"])["count"], 29);
    e.cmd().args(["query", "-p", "nobody"]).assert().code(3);
}

#[test]
fn trend_stats_json() {
    let e = Env::new();
    e.people();
    for (d, v) in [("2023-01-01", "140"), ("2023-07-01", "120"), ("2024-01-01", "100"), ("2025-01-01", "90")] {
        e.run(&["add", "ldl", v, "-p", "alex", "-d", d]);
    }
    let t = e.json(&["trend", "-p", "alex", "-m", "ldl", "--windows", "6m,1y"]);
    assert_eq!(t["kind"], "trend");
    let s = &t["data"][0];
    assert_eq!(s["n"], 4);
    approx(&s["min"], 90.0);
    approx(&s["max"], 140.0);
    approx(&s["mean"], 112.5);
    approx(&s["median"], 110.0);
    approx(&s["change"], -50.0);
    assert!(s["slope_per_year"].as_f64().unwrap() < 0.0);
    approx(&s["windows"]["1y"]["change_pct"], -10.0);
    assert_eq!(s["points"].as_array().unwrap().len(), 4);
    assert_eq!(s["points"][0]["flag"], "high");
    // stats alias, csv flattens windows into columns
    let csv = e.run(&["stats", "-p", "alex", "-f", "csv", "--windows", "1y"]);
    assert!(csv.lines().next().unwrap().ends_with("change_pct_1y"));
}

#[test]
fn diff_between_dates() {
    let e = Env::new();
    e.people();
    e.run(&["import", &example("measurements.csv")]);
    let d = e.json(&["diff", "2023-02-14", "2024-03-05", "-p", "alex", "--changed"]);
    let ldl = d["data"].as_array().unwrap().iter().find(|r| r["marker"] == "ldl-c").unwrap();
    approx(&ldl["from_value"], 138.0);
    approx(&ldl["to_value"], 96.0);
    approx(&ldl["change"], -42.0);
    assert_eq!(ldl["from_flag"], "high");
    assert_eq!(ldl["to_flag"], "normal");
    let exact = e.json(&["diff", "2023-02-14", "2023-08-30", "-p", "alex", "--exact"]);
    let vd = exact["data"].as_array().unwrap().iter().find(|r| r["marker"] == "vitamin-d").unwrap();
    approx(&vd["change"], 12.0);
    let hdl = exact["data"].as_array().unwrap().iter().find(|r| r["marker"] == "hdl-c").unwrap();
    assert!(hdl["to_value"].is_null());
}

#[test]
fn output_formats() {
    let e = Env::new();
    e.people();
    e.run(&["add", "ldl", "101.256", "-p", "alex", "-d", "2024-01-02", "--note", "a, b"]);
    let table = e.run(&["query"]);
    assert!(table.starts_with("ID"));
    assert!(table.contains("101.26"));
    let csv = e.run(&["query", "-f", "csv", "--columns", "person,marker,value,note", "--precision", "1"]);
    assert_eq!(csv, "person,marker,value,note\nalex,ldl-c,101.3,\"a, b\"\n");
    let tsv = e.run(&["query", "-f", "tsv", "--columns", "person,value", "--no-header"]);
    assert_eq!(tsv, "alex\t101.26\n");
    let semi = e.run(&["query", "-f", "csv", "--columns", "person,lab", "--delimiter", ";", "--null", "NA"]);
    assert_eq!(semi, "person;lab\nalex;NA\n");
    let jl = e.run(&["query", "-f", "jsonl"]);
    let v: Value = serde_json::from_str(jl.trim()).unwrap();
    assert_eq!(v["marker"], "ldl-c");
    let dated = e.run(&["query", "-f", "csv", "--columns", "taken_at", "--date-format", "%d/%m/%Y"]);
    assert_eq!(dated, "taken_at\n02/01/2024\n");
    let out = e.path("q.json");
    e.run(&["query", "-f", "json", "-o", out.to_str().unwrap()]);
    let v: Value = serde_json::from_str(&std::fs::read_to_string(out).unwrap()).unwrap();
    assert_eq!(v["count"], 1);
}

#[test]
fn layered_config_with_sources() {
    let e = Env::new();
    let cfg = e.path("custom.toml");
    std::fs::write(&cfg, "format = \"csv\"\nprecision = 4\nunit_system = \"si\"\n[csv]\ndelimiter = \";\"\n").unwrap();
    let out = e
        .cmd()
        .env("BIOMARKER_CONFIG", &cfg)
        .env("BIOMARKER_PRECISION", "3")
        .args(["config", "show", "--effective", "--format", "json", "--units", "us"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    let get = |k: &str| v["data"].as_array().unwrap().iter().find(|r| r["key"] == k).unwrap().clone();
    assert_eq!(get("format")["source"], "flag");
    assert_eq!(get("precision")["value"], "3");
    assert_eq!(get("precision")["source"], "env");
    assert_eq!(get("unit_system")["value"], "us");
    assert_eq!(get("unit_system")["source"], "flag");
    assert_eq!(get("csv_delimiter")["value"], ";");
    assert_eq!(get("csv_delimiter")["source"], "file");
    assert_eq!(get("color")["source"], "env"); // NO_COLOR
    assert_eq!(get("range_flavor")["source"], "default");
    assert_eq!(v["config_path"], cfg.to_str().unwrap());

    // config set writes the XDG file and is picked up afterwards
    e.run(&["config", "set", "format", "json"]);
    let path = e.run(&["config", "path", "-f", "tsv", "--no-header", "--columns", "path"]);
    let want = e.path("config").join("biomarker-cli").join("config.toml");
    assert_eq!(path.trim(), want.to_str().unwrap());
    let v: Value = serde_json::from_str(&e.run(&["person", "list"])).unwrap();
    assert_eq!(v["kind"], "people");
    e.cmd().args(["config", "set", "format", "xml"]).assert().code(7);
    e.cmd().args(["config", "set", "nope", "1"]).assert().code(7);
    e.run(&["config", "unset", "format"]);
    assert!(e.run(&["person", "list"]).is_empty());
    // a broken explicit config file is a config error
    e.cmd().args(["--config", e.path("missing.toml").to_str().unwrap(), "person", "list"]).assert().code(7);
}

#[test]
fn db_maintenance() {
    let e = Env::new();
    e.people();
    e.run(&["add", "tsh", "2.0", "-p", "sam", "-d", "2024-01-01"]);
    let p = e.json(&["db", "path"]);
    assert_eq!(p["data"]["path"], e.db().to_str().unwrap());
    let m = e.json(&["db", "migrate", "--status"]);
    assert!(m["data"].as_array().unwrap().iter().all(|r| r["applied"] == true));
    let backup = e.path("backup.db");
    e.run(&["db", "backup", backup.to_str().unwrap()]);
    e.run(&["db", "vacuum"]);
    assert_eq!(e.json(&["db", "check"])["data"]["ok"], true);
    // the backup is a working database
    let out = e.cmd().env("BIOMARKER_DB", &backup).args(["query", "-f", "json"]).output().unwrap();
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["count"], 1);
    let info = e.json(&["db", "info"]);
    assert_eq!(info["data"]["people"], 2);
    assert_eq!(info["data"]["measurements"], 1);
}

#[test]
fn measurement_rm_and_errors() {
    let e = Env::new();
    e.people();
    let v = e.json(&["add", "tsh", "2.0", "-p", "sam", "-d", "2024-01-01"]);
    let id = v["data"]["id"].to_string();
    e.cmd().args(["add", "tsh", "2.0", "-p", "sam", "-d", "2024-01-01"]).assert().code(4);
    e.run(&["add", "tsh", "2.5", "-p", "sam", "-d", "2024-01-01", "--dedupe", "replace"]);
    e.run(&["rm", &id]);
    e.cmd().args(["rm", &id]).assert().code(3);
    e.cmd().args(["add", "nonsense-marker", "1", "-p", "sam"]).assert().code(3);
    e.cmd().args(["add", "tsh", "1"]).assert().code(2).stderr(predicate::str::contains("no person"));
    e.cmd().args(["query", "--bogus"]).assert().code(2);
    // JSON-mode errors are machine readable
    let out = e.cmd().args(["person", "show", "ghost", "-f", "json"]).output().unwrap();
    assert_eq!(out.status.code(), Some(3));
    let err: Value = serde_json::from_slice(&out.stderr).unwrap();
    assert_eq!(err["kind"], "error");
    assert_eq!(err["error"]["kind"], "not_found");
}

#[test]
fn completions_and_man() {
    let e = Env::new();
    for shell in ["bash", "zsh", "fish", "nushell"] {
        let out = e.run(&["completions", shell]);
        assert!(out.contains("biomarker"), "{shell}");
    }
    let man = e.run(&["man"]);
    assert!(man.contains(".TH biomarker"));
    let dir = e.path("man");
    e.run(&["man", "--dir", dir.to_str().unwrap()]);
    assert!(dir.join("biomarker-import.1").exists());
    assert!(dir.join("biomarker-person-add.1").exists());
}

#[test]
fn timezone_and_marker_unit_change() {
    let e = Env::new();
    e.people();
    // RFC 3339 input is converted to the configured zone's wall-clock time
    let v = e.json(&["add", "glucose", "90", "-p", "alex", "-d", "2024-03-05T23:30:00Z", "--tz", "Asia/Tokyo"]);
    assert_eq!(v["data"]["taken_at"], "2024-03-06T08:30:00");
    let v = e.json(&["add", "glucose", "95", "-p", "alex", "-d", "2024-03-07 07:15"]);
    assert_eq!(v["data"]["taken_at"], "2024-03-07T07:15:00");
    // --to is inclusive of times on that day
    assert_eq!(e.json(&["query", "--to", "2024-03-06"])["count"], 1);
    e.cmd().args(["query", "--tz", "Mars/Olympus"]).assert().code(7);

    // changing the canonical unit rescales stored values and ranges
    e.run(&["marker", "edit", "glucose", "--unit", "mmol/L"]);
    let q = e.json(&["query", "-m", "glucose"]);
    approx(&q["data"][0]["value_canonical"], 90.0 / 18.016);
    approx(&q["data"][0]["ref_high"], 99.0 / 18.016);
    assert_eq!(q["data"][0]["unit_raw"], "mg/dL");
    assert_eq!(e.json(&["db", "check"])["data"]["ok"], true);
    e.cmd().args(["marker", "edit", "glucose", "--unit", "furlongs"]).assert().code(4);
}

#[test]
fn color_and_json_errors_from_config() {
    let e = Env::new();
    e.people();
    e.run(&["add", "ldl", "130", "-p", "alex", "-d", "2024-01-01"]);
    let colored = e.run(&["query", "--color", "always"]);
    assert!(colored.contains("\x1b[31mhigh\x1b[0m"));
    assert!(!e.run(&["query"]).contains('\x1b'));
    // format from env still yields machine-readable errors
    let out = e.cmd().env("BIOMARKER_FORMAT", "json").args(["person", "show", "ghost"]).output().unwrap();
    let err: Value = serde_json::from_slice(&out.stderr).unwrap();
    assert_eq!(err["error"]["code"], 3);
}

#[test]
fn review_regressions() {
    let e = Env::new();
    e.people();
    // --create-markers inside the import transaction
    let csv = e.path("new.csv");
    std::fs::write(&csv, "person,marker,value,unit,date\nalex,Foo X,3,mg/L,2024-01-01\nalex,foo-x,4,mg/L,2024-02-01\n")
        .unwrap();
    let v = e.json(&["import", csv.to_str().unwrap(), "--create-markers"]);
    assert_eq!(v["data"]["inserted"], 2, "{v}");
    assert_eq!(v["data"]["markers_created"], serde_json::json!(["foo-x"]));
    // dry runs report no batch
    std::fs::write(&csv, "person,marker,value,date\nalex,ldl,100,2024-05-01\n").unwrap();
    let dry = e.json(&["import", csv.to_str().unwrap(), "--dry-run"]);
    assert!(dry["data"]["batch"].is_null());
    // explicit mapping to a missing column
    e.cmd().args(["import", csv.to_str().unwrap(), "--map", "value=Wert"]).assert().code(2);
    // decimal comma with three decimals
    let v = e.json(&["add", "tsh", "0,125", "-p", "alex", "-d", "2024-01-01"]);
    approx(&v["data"]["value_raw"], 0.125);
    // --columns on a single-object table report
    let t = e.run(&["person", "show", "alex", "--columns", "slug,sex"]);
    assert_eq!(t, "slug  alex\nsex   male\n");
    // exact tag match
    e.run(&["add", "ldl", "90", "-p", "alex", "-d", "2024-03-01", "--tag", "a_b"]);
    e.run(&["add", "ldl", "91", "-p", "alex", "-d", "2024-04-01", "--tag", "axb"]);
    assert_eq!(e.json(&["query", "--tag", "a_b"])["count"], 1);
    assert_eq!(e.json(&["query", "--tag", "A_B"])["count"], 0);
    // calendar-aligned windows
    e.run(&["add", "hdl", "50", "-p", "sam", "-d", "2023-01-01"]);
    e.run(&["add", "hdl", "55", "-p", "sam", "-d", "2023-07-01"]);
    e.run(&["add", "hdl", "60", "-p", "sam", "-d", "2024-01-01"]);
    let t = e.json(&["trend", "-p", "sam", "-m", "hdl", "--windows", "6m,1y"]);
    approx(&t["data"][0]["windows"]["1y"]["change"], 10.0);
    approx(&t["data"][0]["windows"]["6m"]["change"], 5.0);
    assert_eq!(t["data"][0]["windows"]["1y"]["days"], 365);
}
