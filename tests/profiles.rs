//! Unit presets, range sets and per-person profile files.

use std::path::PathBuf;

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

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

    /// `<config dir>/biomarker-cli/<rel>`, creating parents.
    fn write(&self, rel: &str, text: &str) {
        let p = self.path("config").join("biomarker-cli").join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }

    fn cmd(&self) -> Command {
        let mut c = Command::cargo_bin("biomarker").unwrap();
        c.env_clear()
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("BIOMARKER_DB", self.path("test.db"))
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("NO_COLOR", "1");
        c
    }

    fn try_run(&self, args: &[&str]) -> std::process::Output {
        self.cmd().args(args).output().unwrap()
    }

    fn run(&self, args: &[&str]) -> String {
        let out = self.try_run(args);
        assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    }

    fn json(&self, args: &[&str]) -> Value {
        let mut a = args.to_vec();
        a.extend(["--format", "json"]);
        serde_json::from_str(&self.run(&a)).unwrap()
    }

    fn seeded() -> Self {
        let e = Self::new();
        e.run(&["person", "add", "alex", "--sex", "male", "--dob", "1984-06-01"]);
        e.run(&["person", "add", "sam", "--sex", "female", "--dob", "1991-09-23"]);
        for p in ["alex", "sam"] {
            e.run(&["add", "glucose", "99", "mg/dL", "--person", p, "--date", "2024-01-01", "--lab", "quest"]);
            e.run(&["add", "hba1c", "5.4", "--person", p, "--date", "2024-01-01"]);
        }
        e
    }

    /// The single row for (person, marker) under extra args.
    fn row(&self, person: &str, marker: &str, extra: &[&str]) -> Value {
        let mut a = vec!["latest", "--person", person, "--marker", marker];
        a.extend(extra);
        self.json(&a)["data"][0].clone()
    }
}

#[test]
fn builtin_presets_and_per_marker_overrides() {
    let e = Env::seeded();
    assert_eq!(e.row("alex", "glucose", &["--units", "si"])["unit"], "mmol/L");
    assert_eq!(e.row("alex", "glucose", &["--units", "us"])["unit"], "mg/dL");
    assert_eq!(e.row("alex", "hba1c", &["--units", "uk"])["unit"], "mmol/mol");
    // mg/mL is not a seeded conversion: the config [units] override relies on the built-in extras.
    e.write("config.toml", "unit_system = \"us\"\n\n[units]\nglucose = \"mg/mL\"\n");
    let g = e.row("alex", "glucose", &[]);
    assert_eq!(g["unit"], "mg/mL");
    assert!((g["value"].as_f64().unwrap() - 0.99).abs() < 1e-9);
}

#[test]
fn user_preset_extends_and_is_per_person() {
    let e = Env::seeded();
    e.write("units/euro-lipids.toml", "extends = \"si\"\n[units]\nglucose = \"mg/dL\"\n\"@metabolic\" = \"mg/dL\"\n");
    e.write("people/sam.toml", "unit_preset = \"euro-lipids\"\n");
    e.write("config.toml", "unit_system = \"us\"\n");
    assert_eq!(e.row("alex", "glucose", &[])["unit"], "mg/dL");
    assert_eq!(e.row("sam", "glucose", &[])["unit"], "mg/dL");
    e.write("people/sam.toml", "unit_preset = \"si\"\n");
    assert_eq!(e.row("sam", "glucose", &[])["unit"], "mmol/L");
    assert_eq!(e.row("alex", "glucose", &[])["unit"], "mg/dL", "alex keeps the global preset");
    // an explicit flag beats the person's own preset
    assert_eq!(e.row("sam", "glucose", &["--units", "us"])["unit"], "mg/dL");
}

#[test]
fn unknown_preset_and_unreachable_unit_are_errors() {
    let e = Env::seeded();
    let out = e.try_run(&["latest", "--person", "alex", "--units", "klingon"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown unit preset"));
    e.write("units/bad.toml", "[units]\nglucose = \"furlongs\"\n");
    let out = e.try_run(&["latest", "--person", "alex", "--units", "bad"]);
    assert!(String::from_utf8_lossy(&out.stderr).contains("cannot be shown in 'furlongs'"));
}

#[test]
fn preset_can_define_its_own_conversion() {
    let e = Env::seeded();
    e.write(
        "units/odd.toml",
        "[units]\nglucose = \"mg/L\"\n\n[[conversion]]\nmarker = \"glucose\"\nfrom = \"mg/dL\"\nto = \"mg/L\"\nfactor = 10.0\n",
    );
    assert_eq!(e.row("alex", "glucose", &["--units", "odd"])["value"], 990.0);
}

#[test]
fn personal_ranges_support_bands_units_and_labs() {
    let e = Env::seeded();
    e.write(
        "people/alex.toml",
        "[[range]]\nmarker = \"glucose\"\nhigh = 5.0\nunit = \"mmol/L\"\n\n\
         [[range]]\nmarker = \"glucose\"\nhigh = 120\nunit = \"mg/dL\"\nlab = \"quest\"\n",
    );
    // quest-specific entry replaces the general one for a quest measurement
    let r = e.row("alex", "glucose", &[]);
    assert_eq!(r["ref_high"], 120.0);
    assert_eq!(r["ref_flag"], "normal");
    // sam has no file: catalog range (99 is the top of the normal range)
    assert_eq!(e.row("sam", "glucose", &[])["ref_high"], 99.0);
    e.write("people/alex.toml", "[[range]]\nmarker = \"glucose\"\nhigh = 5.0\nunit = \"mmol/L\"\n");
    let r = e.row("alex", "glucose", &[]);
    assert!((r["ref_high"].as_f64().unwrap() - 90.08).abs() < 0.01);
    assert_eq!(r["ref_flag"], "high");
}

#[test]
fn range_sets_apply_before_the_catalog_and_after_personal() {
    let e = Env::seeded();
    e.write("ranges/strict.toml", "[[range]]\nmarker = \"glucose\"\nhigh = 90\n");
    e.write("ranges/stricter.toml", "extends = \"strict\"\n[[range]]\nmarker = \"hba1c\"\nhigh = 5.0\n");
    e.write("people/sam.toml", "range_set = \"stricter\"\n");
    assert_eq!(e.row("sam", "glucose", &[])["ref_high"], 90.0);
    assert_eq!(e.row("sam", "hba1c", &[])["ref_flag"], "high");
    assert_eq!(e.row("alex", "glucose", &[])["ref_high"], 99.0, "alex has no set");
    e.write("config.toml", "range_set = \"strict\"\n");
    assert_eq!(e.row("alex", "glucose", &[])["ref_high"], 90.0, "global set");
    e.write("people/sam.toml", "range_set = \"strict\"\n[[range]]\nmarker = \"glucose\"\nhigh = 110\n");
    assert_eq!(e.row("sam", "glucose", &[])["ref_high"], 110.0, "personal beats the set");
}

#[test]
fn sex_bands_in_personal_ranges() {
    let e = Env::seeded();
    e.write(
        "ranges/banded.toml",
        "[[range]]\nmarker = \"glucose\"\nsex = \"male\"\nhigh = 95\n\n[[range]]\nmarker = \"glucose\"\nsex = \"female\"\nhigh = 85\n",
    );
    e.write("config.toml", "range_set = \"banded\"\n");
    assert_eq!(e.row("alex", "glucose", &[])["ref_high"], 95.0);
    assert_eq!(e.row("sam", "glucose", &[])["ref_high"], 85.0);
}

#[test]
fn invalid_profile_files_name_the_file() {
    let e = Env::seeded();
    e.write("people/alex.toml", "[[range]]\nmarker = \"not-a-marker\"\nhigh = 1\n");
    let out = e.try_run(&["latest", "--person", "alex"]);
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(!out.status.success());
    assert!(err.contains("alex.toml") && err.contains("not-a-marker"), "{err}");
    e.write("people/alex.toml", "range_set = \"nope\"\n");
    assert!(String::from_utf8_lossy(&e.try_run(&["latest", "--person", "alex"]).stderr).contains("unknown range set"));
    e.write("people/alex.toml", "bogus_key = 1\n");
    assert!(!e.try_run(&["latest", "--person", "alex"]).status.success());
}

#[test]
fn profile_commands() {
    let e = Env::seeded();
    let list = e.json(&["profile", "list"]);
    let names: Vec<String> =
        list["data"].as_array().unwrap().iter().map(|r| r["name"].as_str().unwrap().to_string()).collect();
    assert!(["canonical", "us", "si", "uk"].iter().all(|n| names.contains(&n.to_string())));
    e.run(&["profile", "init", "alex", "--unit-preset", "si"]);
    assert!(!e.try_run(&["profile", "init", "alex"]).status.success(), "refuses to overwrite");
    assert_eq!(e.row("alex", "glucose", &[])["unit"], "mmol/L", "init'd profile is live");
    let shown = e.json(&["profile", "show", "alex", "--marker", "glucose"]);
    assert_eq!(shown["data"][0]["unit"], "mmol/L");
    assert_eq!(shown["data"][0]["source"], "catalog");
}

#[test]
fn critical_and_borderline_levels() {
    let e = Env::seeded();
    e.run(&["add", "glucose", "200", "mg/dL", "--person", "alex", "--date", "2024-02-01"]);
    e.write(
        "ranges/tight.toml",
        "[[range]]\nmarker = \"glucose\"\nlow = 70\nhigh = 100\ncritical_low = 54\ncritical_high = 126\n",
    );
    e.write("config.toml", "range_set = \"tight\"\nborderline_margin = 10\n");
    let rows = e.json(&["query", "--person", "alex", "--marker", "glucose"]);
    let levels: Vec<&str> = rows["data"].as_array().unwrap().iter().map(|r| r["level"].as_str().unwrap()).collect();
    assert_eq!(levels, ["borderline-high", "critical-high"]);
    // 99 is within 10% of the 30-wide range of the high bound, but still "normal" as a flag
    assert_eq!(rows["data"][0]["flag"], "normal");
    let crit = e.json(&["flag", "--person", "alex", "--critical"]);
    assert_eq!(crit["data"].as_array().unwrap().len(), 1);
    let border = e.json(&["flag", "--person", "alex", "--borderline"]);
    assert_eq!(border["data"].as_array().unwrap().len(), 2);
    assert_eq!(e.json(&["flag", "--person", "alex"])["data"].as_array().unwrap().len(), 1, "plain flag is unchanged");
}

#[test]
fn per_person_range_flavor_and_critical_validation() {
    let e = Env::seeded();
    e.write("people/alex.toml", "range_flavor = \"optimal\"\n");
    assert_eq!(e.row("alex", "glucose", &[])["flag"], "high", "optimal glucose tops out at 90");
    assert_eq!(e.row("sam", "glucose", &[])["flag"], "normal");
    assert_eq!(
        e.row("alex", "glucose", &["--range-flavor", "reference"])["flag"],
        "normal",
        "a flag beats the profile"
    );
    e.write("people/alex.toml", "range_flavor = \"sometimes\"\n");
    assert!(!e.try_run(&["latest", "--person", "alex"]).status.success());
    e.write("people/alex.toml", "[[range]]\nmarker = \"glucose\"\nlow = 70\nhigh = 100\ncritical_high = 90\n");
    assert!(String::from_utf8_lossy(&e.try_run(&["latest", "--person", "alex"]).stderr).contains("critical bounds"));
}

#[test]
fn per_marker_and_per_person_precision() {
    let e = Env::seeded();
    let csv = |person: &str| {
        e.run(&["latest", "--person", person, "--marker", "glucose", "--format", "csv", "--columns", "marker,value"])
    };
    assert_eq!(csv("alex").trim(), "marker,value\nglucose,99.00");
    e.write("config.toml", "[marker_precision]\nglucose = 0\n");
    assert_eq!(csv("alex").trim(), "marker,value\nglucose,99");
    e.write("people/sam.toml", "[marker_precision]\n\"@metabolic\" = 3\n");
    assert_eq!(csv("sam").trim(), "marker,value\nglucose,99.000", "person beats global; category works");
    assert_eq!(csv("alex").trim(), "marker,value\nglucose,99");
    // JSON stays exact
    assert_eq!(e.row("alex", "glucose", &[])["value"], 99.0);
}

#[test]
fn trend_defaults_come_from_settings() {
    let e = Env::seeded();
    let windows = |e: &Env| -> Vec<String> {
        e.json(&["trend", "--person", "alex", "--marker", "glucose"])["data"][0]["windows"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    };
    assert_eq!(windows(&e), ["3m", "6m", "1y"]);
    e.run(&["config", "set", "trend_windows", "30d,2y"]);
    assert_eq!(windows(&e), ["30d", "2y"]);
    let out = e.json(&["trend", "--person", "alex", "--windows", "1y"]);
    assert!(out["data"][0]["windows"].get("1y").is_some(), "the flag still wins");
    e.run(&["config", "set", "trend_min_points", "2"]);
    assert_eq!(e.json(&["trend", "--person", "alex"])["data"].as_array().unwrap().len(), 0);
    assert_eq!(e.json(&["trend", "--person", "alex", "--min-points", "1"])["data"].as_array().unwrap().len(), 2);
    assert!(!e.try_run(&["config", "set", "borderline_margin", "150"]).status.success());
}

#[test]
fn unit_aliases_teach_new_spellings() {
    let e = Env::seeded();
    let add = |e: &Env| e.try_run(&["add", "glucose", "88", "mg per dl", "--person", "alex", "--date", "2024-03-01"]);
    assert!(!add(&e).status.success());
    e.write("config.toml", "[unit_aliases]\n\"mg per dl\" = \"mg/dL\"\n");
    assert!(add(&e).status.success());
}

#[test]
fn marker_sync_creates_updates_and_is_idempotent() {
    let e = Env::seeded();
    e.write(
        "markers.toml",
        "[[marker]]\nslug = \"ferritin-x\"\nname = \"Ferritin X\"\ncategory = \"iron\"\nunit = \"ng/mL\"\naliases = [\"ferr-x\"]\n\
         [[marker.conversion]]\nfrom = \"pmol/L\"\nfactor = 0.445\n\
         [[marker.range]]\nsex = \"male\"\nlow = 30\nhigh = 400\n\
         [[marker.range]]\nsex = \"female\"\nlow = 13\nhigh = 150\n\n\
         [[marker]]\nslug = \"glucose\"\naliases = [\"gluc-x\"]\ncategory = \"metabolic\"\n",
    );
    let first = e.json(&["marker", "sync"]);
    let acts: Vec<&str> = first["data"].as_array().unwrap().iter().map(|r| r["action"].as_str().unwrap()).collect();
    assert_eq!(acts, ["created", "updated"]);
    let again = e.json(&["marker", "sync"]);
    assert!(again["data"].as_array().unwrap().iter().all(|r| r["action"] == "unchanged"));
    e.run(&["add", "ferr-x", "20", "--person", "alex", "--date", "2024-01-01"]);
    assert_eq!(e.row("alex", "ferritin-x", &[])["ref_flag"], "low", "male band 30-400");
    e.run(&["add", "gluc-x", "95", "--person", "sam", "--date", "2024-04-01"]);
    // dry run writes nothing
    e.write("markers.toml", "[[marker]]\nslug = \"brand-new\"\nunit = \"mg/dL\"\n");
    assert_eq!(e.json(&["marker", "sync", "--dry-run"])["data"][0]["action"], "created");
    assert!(!e.try_run(&["marker", "show", "brand-new"]).status.success());
    // an existing unit cannot change; critical bounds are refused
    e.write("markers.toml", "[[marker]]\nslug = \"glucose\"\nunit = \"mmol/L\"\n");
    assert!(String::from_utf8_lossy(&e.try_run(&["marker", "sync"]).stderr).contains("cannot change"));
    e.write("markers.toml", "[[marker]]\nslug = \"glucose\"\n[[marker.range]]\nhigh = 99\ncritical_high = 300\n");
    assert!(String::from_utf8_lossy(&e.try_run(&["marker", "sync"]).stderr).contains("critical bounds"));
}

#[test]
fn config_file_with_units_table_still_round_trips_settings() {
    let e = Env::seeded();
    e.write("config.toml", "[units]\nglucose = \"mg/mL\"\n");
    e.run(&["config", "set", "precision", "3"]);
    let text = std::fs::read_to_string(e.path("config").join("biomarker-cli/config.toml")).unwrap();
    assert!(text.contains("mg/mL") && text.contains("precision"), "{text}");
}
