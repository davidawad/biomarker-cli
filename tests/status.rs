//! Per-draw range resolution (sex, age at `taken_at`), near-limit (warn)
//! bounds and the per-row `status` of the `biomarker/v1` JSON interface.
//! Synthetic people and values only.

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

/// Raw 256-bit test KEK (never use a fixed key outside tests).
const TEST_KEY: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

struct Env {
    dir: TempDir,
}

impl Env {
    fn new() -> Self {
        let e = Self { dir: TempDir::new().unwrap() };
        e.run(&["person", "add", "pat", "--sex", "male", "--dob", "1970-03-15"]);
        e.run(&["person", "add", "kim", "--sex", "female"]);
        e
    }

    /// `<config dir>/biomarker-cli/<rel>`, creating parents.
    fn write(&self, rel: &str, text: &str) {
        let p = self.dir.path().join("config").join("biomarker-cli").join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }

    fn run(&self, args: &[&str]) -> String {
        let out = Command::cargo_bin("biomarker")
            .unwrap()
            .env_clear()
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.dir.path().join("config"))
            .env("XDG_DATA_HOME", self.dir.path().join("data"))
            .env("BIOMARKER_DB", self.dir.path().join("test.db"))
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("NO_COLOR", "1")
            .args(args)
            .output()
            .unwrap();
        assert!(out.status.success(), "biomarker {args:?} failed:\n{}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    }

    fn data(&self, args: &[&str]) -> Vec<Value> {
        let mut a = args.to_vec();
        a.extend(["--format", "json"]);
        let v: Value = serde_json::from_str(&self.run(&a)).unwrap();
        assert_eq!(v["schema"], "biomarker/v1");
        v["data"].as_array().unwrap().clone()
    }
}

fn row<'a>(rows: &'a [Value], marker: &str, date: &str) -> &'a Value {
    rows.iter().find(|r| r["marker"] == marker && r["taken_at"] == date).unwrap()
}

#[test]
fn age_at_draw_selects_the_range_and_rows_report_it() {
    let e = Env::new();
    e.run(&["range", "set", "hdl", "--sex", "male", "--age-min", "50", "--low", "45", "--note", "synthetic 50+"]);
    e.run(&["add", "hdl", "42", "-p", "pat", "-d", "2019-01-01"]); // age 48
    e.run(&["add", "hdl", "42", "-p", "pat", "-d", "2021-01-01"]); // age 50
    e.run(&["add", "hdl", "42", "-p", "kim", "-d", "2021-01-01"]); // no birth date
    let q = e.data(&["query", "--all-people"]);
    let young = q.iter().find(|r| r["person"] == "pat" && r["taken_at"] == "2019-01-01").unwrap();
    let older = q.iter().find(|r| r["person"] == "pat" && r["taken_at"] == "2021-01-01").unwrap();
    let kim = q.iter().find(|r| r["person"] == "kim").unwrap();

    assert_eq!(young["sex"], "male");
    assert_eq!(young["age"].as_f64().unwrap().floor(), 48.0);
    assert_eq!(young["range_set"]["reference"]["sex"], "male");
    assert_eq!(young["range_set"]["reference"]["age_min"], 0.0);
    assert_eq!(young["range_set"]["reference"]["source"], "catalog");
    assert_eq!(young["ref_low"], 40.0);
    assert_eq!(young["status"], "in-range");

    assert_eq!(older["age"].as_f64().unwrap().floor(), 50.0);
    assert_eq!(older["range_set"]["reference"]["age_min"], 50.0);
    assert_eq!(older["range_set"]["reference"]["note"], "synthetic 50+");
    assert_eq!(older["ref_low"], 45.0);
    assert_eq!(older["ref_flag"], "low");
    assert_eq!(older["status"], "low");

    assert_eq!(kim["age"], Value::Null);
    assert_eq!(kim["sex"], "female");
    assert_eq!(kim["ref_low"], 50.0);
    assert_eq!(kim["range_set"]["warn"], Value::Null);
}

#[test]
fn warn_zones_drive_status_and_keep_flags() {
    let e = Env::new();
    for (marker, value, date) in [
        ("hba1c", "5.2", "2023-01-01"),
        ("hba1c", "5.9", "2024-01-01"),
        ("hba1c", "6.8", "2025-01-01"),
        ("egfr", "75", "2025-01-01"),
        ("glucose", "<60", "2025-01-01"),
        ("ferritin", "150", "2025-01-01"),
    ] {
        e.run(&["add", marker, value, "-p", "pat", "-d", date]);
    }
    let q = e.data(&["query", "-p", "pat"]);
    let a1c = row(&q, "hba1c", "2024-01-01");
    assert_eq!(a1c["warn_high"], 6.4);
    assert_eq!(a1c["warn_low"], Value::Null);
    assert_eq!(a1c["status"], "near-high");
    assert_eq!(a1c["ref_flag"], "high");
    assert_eq!(a1c["flag"], "high");
    assert_eq!(a1c["range_set"]["warn"]["source"], "catalog");
    assert_eq!(row(&q, "hba1c", "2023-01-01")["status"], "in-range");
    assert_eq!(row(&q, "hba1c", "2025-01-01")["status"], "high");
    let egfr = row(&q, "egfr", "2025-01-01");
    assert_eq!((egfr["status"].as_str(), egfr["ref_flag"].as_str()), (Some("near-low"), Some("normal")));
    // censored below the reference low is still low; no warn zone falls back to the reference
    assert_eq!(row(&q, "glucose", "2025-01-01")["status"], "low");
    let ferritin = row(&q, "ferritin", "2025-01-01");
    assert_eq!((ferritin["status"].as_str(), ferritin["warn_high"].is_null()), (Some("in-range"), true));

    // latest and flag carry the same fields
    let latest = e.data(&["latest", "-p", "pat", "-m", "hba1c"]);
    assert_eq!(latest[0]["status"], "high");
    let flagged = e.data(&["flag", "-p", "pat"]);
    assert!(flagged.iter().all(|r| r["status"].is_string()));

    // trend: per-point status, last status and the last draw's warn bounds
    let t = e.data(&["trend", "-p", "pat", "-m", "hba1c"]);
    let statuses: Vec<&str> =
        t[0]["points"].as_array().unwrap().iter().map(|p| p["status"].as_str().unwrap()).collect();
    assert_eq!(statuses, ["in-range", "near-high", "high"]);
    assert_eq!(t[0]["last_status"], "high");
    assert_eq!(t[0]["warn_high"], 6.4);
}

#[test]
fn warn_bounds_from_range_sets_and_person_profiles() {
    let e = Env::new();
    // a person's own warn bound beats the catalog; display units apply
    e.run(&["range", "set", "glucose", "--kind", "warn", "--person", "pat", "--high", "110"]);
    e.run(&["range", "set", "ferritin", "--kind", "warn", "--sex", "female", "--high", "120"]);
    e.run(&["add", "glucose", "115", "-p", "pat", "-d", "2025-01-01"]);
    e.run(&["add", "glucose", "115", "-p", "kim", "-d", "2025-01-01"]);
    e.run(&["add", "ferritin", "130", "-p", "kim", "-d", "2025-01-01"]);
    e.run(&["add", "ferritin", "130", "-p", "pat", "-d", "2025-01-01"]);
    let q = e.data(&["query", "--all-people"]);
    let get = |p: &str, m: &str| q.iter().find(|r| r["person"] == p && r["marker"] == m).unwrap();
    assert_eq!(get("pat", "glucose")["status"], "high");
    assert_eq!(get("pat", "glucose")["range_set"]["warn"]["source"], "person");
    assert_eq!(get("kim", "glucose")["status"], "near-high");
    assert_eq!(get("kim", "glucose")["warn_high"], 125.0);
    // ferritin female reference tops out above 120: 130 sits in the inner margin
    assert_eq!(get("kim", "ferritin")["warn_high"], 120.0);
    assert_eq!(get("kim", "ferritin")["status"], "near-high");
    assert_eq!(get("pat", "ferritin")["warn_high"], Value::Null);

    let si = e.data(&["query", "-p", "kim", "-m", "glucose", "--units", "si"]);
    assert!((si[0]["warn_high"].as_f64().unwrap() - 125.0 / 18.016).abs() < 1e-9);

    let warn = e.data(&["range", "list", "hba1c", "--kind", "warn"]);
    assert_eq!((warn.len(), warn[0]["high"].as_f64()), (1, Some(6.4)));
}

#[test]
fn warn_ranges_follow_the_range_set_and_profile_precedence() {
    let e = Env::new();
    e.write(
        "ranges/strict.toml",
        "[[range]]\nmarker = \"glucose\"\nkind = \"warn\"\nhigh = 120\nnote = \"synthetic set\"\n",
    );
    e.write(
        "people/pat.toml",
        "[[range]]\nmarker = \"glucose\"\nkind = \"warn\"\nage_min = 50\nhigh = 6.0\nunit = \"mmol/L\"\n",
    );
    e.write("config.toml", "range_set = \"strict\"\n");
    e.run(&["add", "glucose", "112", "-p", "pat", "-d", "2019-01-01"]); // age 48: the range set applies
    e.run(&["add", "glucose", "112", "-p", "pat", "-d", "2021-01-01"]); // age 50: pat's own band applies
    e.run(&["add", "glucose", "112", "-p", "kim", "-d", "2021-01-01"]);
    let q = e.data(&["query", "--all-people"]);
    let get = |p: &str, d: &str| q.iter().find(|r| r["person"] == p && r["taken_at"] == d).unwrap();

    let young = get("pat", "2019-01-01");
    assert_eq!(young["warn_high"], 120.0);
    assert_eq!(young["status"], "near-high");
    assert_eq!(young["ref_flag"], "high");
    assert_eq!(young["range_set"]["warn"]["source"], "set:strict");
    assert_eq!(young["range_set"]["warn"]["id"], Value::Null, "file ranges have no database id");
    assert_eq!(young["range_set"]["warn"]["note"], "synthetic set");
    assert_eq!(young["range_set"]["reference"]["source"], "catalog");

    let older = get("pat", "2021-01-01");
    assert!((older["warn_high"].as_f64().unwrap() - 6.0 * 18.016).abs() < 1e-9);
    assert_eq!(older["status"], "high");
    assert_eq!(older["range_set"]["warn"]["source"], "person");
    assert_eq!(older["range_set"]["warn"]["age_min"], 50.0);

    assert_eq!(get("kim", "2021-01-01")["range_set"]["warn"]["source"], "set:strict");

    // profile show lists the warn range with its source too
    let shown = e.data(&["profile", "show", "kim", "--marker", "glucose"]);
    let warn = shown.iter().find(|r| r["kind"] == "warn").unwrap();
    assert_eq!((warn["high"].as_f64(), warn["source"].as_str()), (Some(120.0), Some("set:strict")));
}
