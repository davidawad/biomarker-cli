//! Every `--format` against golden files, plus round-trips proving the
//! structured formats (YAML, TOON) carry exactly the JSON envelope.
//!
//! Regenerate the goldens with `BIOMARKER_BLESS=1 cargo test --test formats`.

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

/// Raw 256-bit test KEK (never use a fixed key outside tests).
const TEST_KEY: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

/// Canonical name and golden-file extension of every output format.
const FORMATS: &[(&str, &str)] = &[
    ("table", "txt"),
    ("json", "json"),
    ("jsonl", "jsonl"),
    ("csv", "csv"),
    ("tsv", "tsv"),
    ("yaml", "yaml"),
    ("toon", "toon"),
    ("markdown", "md"),
    ("html", "html"),
    ("org", "org"),
];

struct Env {
    dir: TempDir,
}

impl Env {
    /// A database with two people and a few measurements, all with fixed dates.
    fn seeded() -> Self {
        let e = Self { dir: TempDir::new().unwrap() };
        e.run(&["person", "add", "alex", "--sex", "male", "--dob", "1984-06-01", "--name", "Alex Example"]);
        e.run(&["add", "ldl-c", "96", "--person", "alex", "--date", "2024-03-05", "--note", "fasting | am"]);
        e.run(&["add", "hdl-c", "55", "--person", "alex", "--date", "2024-03-05", "--lab", "Quest <East>"]);
        e.run(&["add", "apob", "112", "--person", "alex", "--date", "2024-03-05"]);
        e
    }

    fn cmd(&self) -> Command {
        let p = |n: &str| self.dir.path().join(n);
        let mut c = Command::cargo_bin("biomarker").unwrap();
        c.env_clear()
            // Windows system DLLs expect SYSTEMROOT even in a cleared environment.
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
}

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("golden").join("formats")
}

/// Compare against (or, with BIOMARKER_BLESS set, rewrite) a golden file.
fn check_golden(name: &str, actual: &str) {
    let path = golden_dir().join(name);
    if std::env::var_os("BIOMARKER_BLESS").is_some() {
        std::fs::create_dir_all(golden_dir()).unwrap();
        std::fs::write(&path, actual).unwrap();
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(actual == expected, "{name} changed:\n--- expected\n{expected}\n--- actual\n{actual}");
}

#[test]
fn every_format_matches_its_golden() {
    let e = Env::seeded();
    for (format, ext) in FORMATS {
        check_golden(&format!("latest.{ext}"), &e.run(&["latest", "--person", "alex", "-f", format]));
    }
    // An object body (key/value pairs) in the text-table formats.
    for (format, ext) in [("markdown", "md"), ("html", "html"), ("org", "org")] {
        check_golden(&format!("person.{ext}"), &e.run(&["person", "show", "alex", "-f", format]));
    }
}

#[test]
fn columns_and_no_header_are_honoured() {
    let e = Env::seeded();
    let sel = ["latest", "--person", "alex", "--columns", "marker,value", "--no-header"];
    let org = e.run(&[&sel[..], &["-f", "org"]].concat());
    assert_eq!(org, "| apob | 112.00 |\n| hdl-c | 55.00 |\n| ldl-c | 96.00 |\n");
    let html = e.run(&[&sel[..], &["-f", "html"]].concat());
    assert!(!html.contains("<thead>") && html.contains("<td>apob</td><td>112.00</td>"), "{html}");
    // Markdown tables cannot exist without a header row, so it stays.
    let md = e.run(&[&sel[..], &["-f", "md"]].concat());
    assert!(md.starts_with("| marker | value |\n|---|---|\n"), "{md}");
}

/// Numbers as f64, so `112.0` (JSON) equals TOON's canonical `112`.
fn numbers_as_f64(v: Value) -> Value {
    match v {
        Value::Number(n) => serde_json::json!(n.as_f64()),
        Value::Array(a) => Value::Array(a.into_iter().map(numbers_as_f64).collect()),
        Value::Object(o) => Value::Object(o.into_iter().map(|(k, v)| (k, numbers_as_f64(v))).collect()),
        other => other,
    }
}

#[test]
fn yaml_and_toon_carry_the_json_envelope() {
    let e = Env::seeded();
    let json: Value = serde_json::from_str(&e.run(&["latest", "--person", "alex", "-f", "json"])).unwrap();
    let yaml: Value = yaml_serde::from_str(&e.run(&["latest", "--person", "alex", "-f", "yaml"])).unwrap();
    assert_eq!(yaml, json);
    let toon: Value = toon_format::decode_no_coerce(&e.run(&["latest", "--person", "alex", "-f", "toon"])).unwrap();
    assert_eq!(numbers_as_f64(toon), numbers_as_f64(json));
}

#[test]
fn aliases_and_config_default() {
    let e = Env::seeded();
    let md = e.run(&["latest", "--person", "alex", "-f", "markdown"]);
    for alias in ["md", "MD"] {
        assert_eq!(e.run(&["latest", "--person", "alex", "-f", alias]), md);
    }
    let yaml = e.run(&["latest", "--person", "alex", "-f", "yaml"]);
    assert_eq!(e.run(&["latest", "--person", "alex", "-f", "yml"]), yaml);
    let jsonl = e.run(&["latest", "--person", "alex", "-f", "jsonl"]);
    assert_eq!(e.run(&["latest", "--person", "alex", "-f", "ndjson"]), jsonl);
    // The default format comes from config/env like every other setting.
    let out = e.cmd().args(["latest", "--person", "alex"]).env("BIOMARKER_FORMAT", "toon").output().unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), e.run(&["latest", "--person", "alex", "-f", "toon"]));
    e.run(&["config", "set", "format", "org"]);
    assert!(e.run(&["latest", "--person", "alex"]).starts_with("| id | person |"));
}

#[test]
fn export_in_structured_formats_keeps_observations() {
    let e = Env::seeded();
    for format in ["yaml", "toon"] {
        let out = e.run(&["export", "-f", format]);
        assert!(out.contains("observations"), "{format} export lost the observations envelope field:\n{out}");
    }
}
