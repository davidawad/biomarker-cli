//! The catalog as data: upgrading a v0.7.2 database, range sets and config
//! overrides, person attributes, `catalog lint` and `catalog docs` (including
//! the check that the committed docs/markers are up to date). Synthetic
//! people and values only.

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

/// Raw 256-bit test KEK (never use a fixed key outside tests).
const TEST_KEY: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

struct Env {
    dir: TempDir,
    db: PathBuf,
    plaintext: bool,
}

impl Env {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        let db = dir.path().join("test.db");
        Self { dir, db, plaintext: false }
    }

    /// A database as v0.7.2 left it, rebuilt from the frozen v0.7.2 schema and
    /// seed data (migrations 1-6, pruned to the 71 markers v0.7.2 shipped):
    /// people alex (male, 1985-06-01) and kim (female); glucose 104 and HDL 42
    /// for alex on 2024-01-01, LDL 95 for kim on 2024-02-01; `range set hdl
    /// --sex male --age-min 50 --low 45`, `range set ldl --kind optimal --high 60`
    /// and `range set glucose --person kim --high 90`.
    fn v072() -> Self {
        let mut e = Self::new();
        e.db = e.dir.path().join("v0.7.2.db");
        let p = e.db.clone();
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || write_v072(&p))
            .unwrap()
            .join()
            .unwrap();
        e.plaintext = true;
        e
    }

    fn write(&self, rel: &str, text: &str) {
        let p = self.dir.path().join("config").join("biomarker-cli").join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }

    fn out(&self, args: &[&str]) -> std::process::Output {
        let mut cmd = Command::cargo_bin("biomarker").unwrap();
        cmd.env_clear()
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.dir.path().join("config"))
            .env("XDG_DATA_HOME", self.dir.path().join("data"))
            .env("BIOMARKER_DB", &self.db)
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("NO_COLOR", "1");
        if self.plaintext {
            cmd.args(["--insecure-plaintext", "-q"]);
        }
        cmd.args(args).output().unwrap()
    }

    fn run(&self, args: &[&str]) -> String {
        let out = self.out(args);
        assert!(out.status.success(), "biomarker {args:?} failed:\n{}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    }

    fn code(&self, args: &[&str]) -> i32 {
        self.out(args).status.code().unwrap_or(-1)
    }

    fn json(&self, args: &[&str]) -> Value {
        let mut a = args.to_vec();
        a.extend(["--format", "json"]);
        let v: Value = serde_json::from_str(&self.run(&a)).unwrap();
        assert_eq!(v["schema"], "biomarker/v1");
        v
    }

    fn data(&self, args: &[&str]) -> Vec<Value> {
        self.json(args)["data"].as_array().unwrap().clone()
    }
}

/// Write what a v0.7.2 binary would have: schema version 6 and its rows.
fn write_v072(path: &Path) {
    use biomarker_cli::{migrations, seed};
    let db = biomarker_cli::db::Db::open_plain(path).unwrap();
    migrations::applied(&db).unwrap();
    for m in migrations::MIGRATIONS.iter().filter(|m| m.version <= 6) {
        (m.apply)(&db).unwrap();
        db.execute(
            "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, '2024-01-01T00:00:00Z')",
            &[biomarker_cli::db::int(m.version), biomarker_cli::db::text(m.name)],
        )
        .unwrap();
    }
    let legacy = seed::LEGACY_SLUGS.iter().map(|s| format!("'{s}'")).collect::<Vec<_>>().join(", ");
    let newer = format!("SELECT id FROM markers WHERE slug NOT IN ({legacy})");
    db.execute_batch(&format!(
        "DELETE FROM marker_aliases WHERE marker_id IN ({newer});
         DELETE FROM unit_conversions WHERE marker_id IN ({newer});
         DELETE FROM ranges WHERE marker_id IN ({newer});
         DELETE FROM markers WHERE slug NOT IN ({legacy});
         INSERT INTO people (slug, sex, dob, created_at) VALUES ('alex', 'male', '1985-06-01', '2024-01-01T00:00:00Z');
         INSERT INTO people (slug, sex, created_at) VALUES ('kim', 'female', '2024-01-01T00:00:00Z');
         INSERT INTO measurements (person_id, marker_id, taken_at, value_raw, unit_raw, value, created_at)
           SELECT p.id, m.id, '2024-01-01', 104, 'mg/dL', 104, '2024-01-01T00:00:00Z' FROM people p, markers m
           WHERE p.slug = 'alex' AND m.slug = 'glucose';
         INSERT INTO measurements (person_id, marker_id, taken_at, value_raw, unit_raw, value, created_at)
           SELECT p.id, m.id, '2024-01-01', 42, 'mg/dL', 42, '2024-01-01T00:00:00Z' FROM people p, markers m
           WHERE p.slug = 'alex' AND m.slug = 'hdl-c';
         INSERT INTO measurements (person_id, marker_id, taken_at, value_raw, unit_raw, value, created_at)
           SELECT p.id, m.id, '2024-02-01', 95, 'mg/dL', 95, '2024-02-01T00:00:00Z' FROM people p, markers m
           WHERE p.slug = 'kim' AND m.slug = 'ldl-c';
         INSERT INTO ranges (marker_id, kind, sex, age_min, age_max, low, note)
           SELECT id, 'reference', 'male', 50, 200, 45, 'synthetic 50+' FROM markers WHERE slug = 'hdl-c';
         UPDATE ranges SET low = NULL, high = 60, note = NULL
           WHERE marker_id = (SELECT id FROM markers WHERE slug = 'ldl-c') AND kind = 'optimal' AND sex = 'any';
         INSERT INTO person_ranges (person_id, marker_id, kind, high)
           SELECT p.id, m.id, 'reference', 90 FROM people p, markers m WHERE p.slug = 'kim' AND m.slug = 'glucose';
         PRAGMA user_version = 6;"
    ))
    .unwrap();
}

fn find<'a>(rows: &'a [Value], marker: &str) -> &'a Value {
    rows.iter().find(|r| r["marker"] == marker).unwrap_or_else(|| panic!("no {marker} in {rows:?}"))
}

#[test]
fn upgrades_a_v0_7_2_database_and_keeps_its_flags() {
    let e = Env::v072();
    let status = e.json(&["db", "migrate", "--status"]);
    assert_eq!(status["current_version"], 6);
    e.run(&["db", "migrate"]);
    let info = e.json(&["db", "info"]);
    assert_eq!(info["data"]["schema_version"], 8);
    // migration 8 is additive: the catalog's LOINC codes are mapped, and nothing existing grows new fields
    let loinc = e.data(&["marker", "loinc", "list", "glucose"]);
    assert_eq!((loinc[0]["loinc"].as_str(), loinc[0]["source"].as_str()), (Some("2345-7"), Some("catalog")));
    assert!(e.data(&["query", "--all-people"]).iter().all(|r| r.get("lab_ref_low").is_none()));
    assert_eq!((info["data"]["people"].as_i64(), info["data"]["measurements"].as_i64()), (Some(2), Some(3)));
    // v0.7.2 had 71 markers, all in the catalog; markers added to the catalog since are seeded once
    let catalog: Value = serde_json::from_str(&e.run(&["catalog", "lint", "--json"])).unwrap();
    let markers = catalog["data"]["markers"].as_i64().unwrap();
    assert_eq!(info["data"]["markers"].as_i64(), Some(markers.max(71)), "no marker is seeded twice");

    let q = e.data(&["query", "--all-people"]);
    let glucose = find(&q, "glucose");
    assert_eq!((glucose["ref_high"].as_f64(), glucose["ref_flag"].as_str()), (Some(99.0), Some("high")));
    assert_eq!((glucose["warn_high"].as_f64(), glucose["status"].as_str()), (Some(125.0), Some("near-high")));
    assert_eq!(glucose["range_set"]["reference"]["set"], "clinical");
    // alex is 38: the database's own male 50+ HDL row does not apply, the catalog's male row does
    let hdl = find(&q, "hdl-c");
    assert_eq!((hdl["ref_low"].as_f64(), hdl["ref_flag"].as_str()), (Some(40.0), Some("normal")));
    // the optimal LDL bound v0.7.2's `range set` replaced still wins over the catalog's
    let ldl = find(&q, "ldl-c");
    assert_eq!((ldl["opt_high"].as_f64(), ldl["opt_flag"].as_str()), (Some(60.0), Some("high")));
    assert_eq!(ldl["range_set"]["optimal"]["source"], "catalog");
    assert!(ldl["range_set"]["optimal"]["id"].is_i64(), "a database row: {}", ldl["range_set"]);
    assert_eq!(ldl["range_set"]["optimal"]["set"], Value::Null);

    // the seeded rows are hidden; the edited ones are listed once, next to the catalog's cited ranges
    let opt = e.data(&["range", "list", "ldl", "--kind", "optimal"]);
    let db_rows: Vec<&Value> = opt.iter().filter(|r| r["id"].as_i64().is_some_and(|id| id > 0)).collect();
    assert_eq!(db_rows.len(), 1, "{opt:?}");
    assert_eq!(db_rows[0]["high"], 60.0);
    assert!(opt.iter().all(|r| r["legacy"] != true), "the seeded legacy optimal row is hidden: {opt:?}");
    let hdl_ranges = e.data(&["range", "list", "hdl", "--kind", "reference"]);
    assert!(hdl_ranges.iter().any(|r| r["note"] == "synthetic 50+" && r["id"].is_i64()));
    assert!(hdl_ranges.iter().any(|r| r["set"] == "clinical" && r["sex"] == "female" && r["legacy"] == true));
    assert_eq!(e.data(&["range", "list", "-p", "kim", "glucose", "--kind", "reference"])[0]["high"], 90.0);

    // the new person attributes column exists and is usable
    e.run(&["person", "edit", "kim", "--attr", "pregnant=true"]);
    assert_eq!(e.json(&["person", "show", "kim"])["data"]["attributes"]["pregnant"], true);
    // a second run is a no-op
    e.run(&["db", "migrate"]);
    assert_eq!(e.json(&["db", "info"])["data"]["markers"].as_i64(), Some(markers.max(71)));
}

#[test]
fn a_marker_removed_before_the_upgrade_stays_removed() {
    let e = Env::v072();
    let p = e.db.clone();
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let db = biomarker_cli::db::Db::open_plain(&p).unwrap();
            db.execute_batch(
                "DELETE FROM marker_aliases WHERE marker_id = (SELECT id FROM markers WHERE slug = 'vo2max');
                 DELETE FROM markers WHERE slug = 'vo2max';",
            )
            .unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
    e.run(&["db", "migrate"]);
    let markers = e.data(&["marker", "list"]);
    assert!(markers.iter().all(|m| m["slug"] != "vo2max"), "vo2max was seeded again");
    assert!(markers.iter().any(|m| m["slug"] == "hrv"));
}

#[test]
fn range_sets_marker_overrides_and_config_ranges() {
    let e = Env::new();
    e.run(&["person", "add", "pat", "--sex", "male", "--dob", "1970-03-15"]);
    e.run(&["add", "glucose", "93", "-p", "pat", "-d", "2024-01-01"]);
    e.run(&["add", "ldl", "75", "-p", "pat", "-d", "2024-01-01"]);
    let opt = |m: &str| {
        let q = e.data(&["latest", "-p", "pat", "-m", m]);
        (q[0]["opt_high"].as_f64(), q[0]["range_set"]["optimal"]["set"].as_str().map(str::to_string))
    };
    assert_eq!(opt("glucose"), (Some(90.0), Some("optimal".into())), "default sets: clinical, optimal");
    e.write("config.toml", "range_sets = [\"blueprint\", \"clinical\", \"optimal\"]\n");
    assert_eq!(opt("glucose"), (Some(95.0), Some("blueprint".into())));
    assert_eq!(opt("ldl"), (Some(100.0), Some("blueprint".into())));
    // per-marker set order replaces range_sets for that marker
    e.write(
        "config.toml",
        "range_sets = [\"blueprint\", \"clinical\", \"optimal\"]\n[marker_range_sets]\nldl = [\"optimal\"]\n",
    );
    // the cited ATP III adult (20+) optimal bound is more specific than the uncited any-age one
    assert_eq!(opt("ldl"), (Some(100.0), Some("optimal".into())));
    // config [[range]] entries beat every set
    e.write("config.toml", "[[range]]\nmarker = \"glucose\"\nkind = \"optimal\"\nhigh = 5.0\nunit = \"mmol/L\"\n");
    let q = e.data(&["latest", "-p", "pat", "-m", "glucose"]);
    assert_eq!(q[0]["range_set"]["optimal"]["source"], "config");
    assert!((q[0]["opt_high"].as_f64().unwrap() - 5.0 * 18.016).abs() < 1e-9);
    // the environment and `config set` take lists too
    e.write("config.toml", "");
    e.run(&["config", "set", "range_sets", "blueprint,clinical"]);
    assert_eq!(opt("glucose"), (Some(95.0), Some("blueprint".into())));
    let shown = e.data(&["profile", "show", "pat", "--marker", "glucose"]);
    assert!(shown.iter().any(|r| r["kind"] == "optimal" && r["set"] == "blueprint"));
    e.write("config.toml", "range_sets = [\"nope\"]\n");
    assert_eq!(e.code(&["latest", "-p", "pat"]), 7, "unknown range set is a config error");
}

#[test]
fn person_attributes_are_validated_and_stored() {
    let e = Env::new();
    e.run(&[
        "person",
        "add",
        "kim",
        "--sex",
        "female",
        "--attr",
        "trimester=2",
        "--attr",
        "conditions=type-2-diabetes",
    ]);
    let p = e.json(&["person", "show", "kim"]);
    assert_eq!(p["data"]["attributes"]["pregnant"], true);
    assert_eq!(p["data"]["attributes"]["trimester"], 2);
    assert_eq!(p["data"]["attributes"]["conditions"][0], "type-2-diabetes");
    e.run(&["person", "edit", "kim", "--attr", "pregnant=false", "--attr", "menopause=post"]);
    let p = e.json(&["person", "show", "kim"]);
    assert_eq!(p["data"]["attributes"]["trimester"], Value::Null, "not pregnant: no trimester");
    assert_eq!(p["data"]["attributes"]["menopause"], "post");
    assert_eq!(e.code(&["person", "edit", "kim", "--attr", "cycle_phase=spring"]), 2);
    assert_eq!(e.code(&["person", "edit", "kim", "--attr", "conditions=made-up"]), 2);
    assert_eq!(e.code(&["person", "edit", "kim", "--attr", "nokey"]), 2);
    let conditions = e.data(&["catalog", "conditions"]);
    assert!(conditions.iter().any(|c| c["slug"] == "type-2-diabetes"));
}

/// A minimal catalog directory with one marker file.
fn catalog_dir(root: &Path, marker: &str) -> PathBuf {
    let dir = root.join("cat");
    std::fs::create_dir_all(dir.join("markers")).unwrap();
    std::fs::write(dir.join("markers").join("x.toml"), marker).unwrap();
    std::fs::write(dir.join("universe.toml"), "[[marker]]\nslug = \"x\"\nname = \"X\"\ncategory = \"lipid\"\npriority = \"core\"\n\n[[marker]]\nslug = \"y\"\nname = \"Y\"\ncategory = \"lipid\"\npriority = \"specialist\"\n").unwrap();
    dir
}

#[test]
fn catalog_lint_reports_errors_warnings_and_coverage() {
    let e = Env::new();
    let built_in: Value = serde_json::from_str(&e.run(&["catalog", "lint", "--universe", "--json"])).unwrap();
    assert_eq!(built_in["data"]["errors"], 0, "{}", built_in["data"]["findings"]);
    assert!(built_in["data"]["warnings"].as_u64().unwrap() > 0, "legacy ranges are uncited");
    // universe entries without a catalog file are warnings, not errors
    let universe = &built_in["data"]["universe"];
    let missing = universe["missing"].as_array().map_or(0, Vec::len) as u64;
    assert_eq!(universe["covered"].as_u64().unwrap() + missing, universe["entries"].as_u64().unwrap());
    assert_eq!(e.code(&["catalog", "lint", "--strict"]), 4);

    let head = "slug = \"x\"\nname = \"X\"\ncategory = \"lipid\"\nunit = \"mg/dL\"\ndescription = \"Synthetic.\"\n";
    let cited = format!(
        "{head}[[range]]\nlow = 1.0\nhigh = 2.0\nevidence = \"guideline\"\ncitations = [{{ pmid = \"1\", locator = \"Table 2\" }}]\n\
         [[range]]\nlow = 1.5\nhigh = 2.5\nwhen = {{ sex = \"female\", markers = [{{ marker = \"x2\", op = \">\", value = 100.0 }}] }}\n\
         citations = [{{ doi = \"10.1000/xyz\" }}]\n"
    );
    let dir = catalog_dir(e.dir.path(), &cited);
    let d = dir.to_string_lossy().into_owned();
    let out = e.out(&["catalog", "lint", "--dir", &d, "--json"]);
    assert_eq!(out.status.code(), Some(4));
    let r: Value = serde_json::from_slice(&out.stdout).unwrap();
    let codes: Vec<&str> =
        r["data"]["findings"].as_array().unwrap().iter().map(|f| f["code"].as_str().unwrap()).collect();
    assert_eq!(codes, ["when-marker"], "x2 is not in this catalog");
    std::fs::write(dir.join("markers").join("x.toml"), cited.replace("x2", "x")).unwrap();
    assert_eq!(e.code(&["catalog", "lint", "--dir", &d]), 4, "a marker cannot condition on itself");

    let fine = format!(
        "{head}[[range]]\nlow = 1.0\nevidence = \"cohort\"\ncitations = [{{ url = \"https://example.org/x\" }}]\n"
    );
    std::fs::write(dir.join("markers").join("x.toml"), &fine).unwrap();
    assert_eq!(e.code(&["catalog", "lint", "--dir", &d]), 0);
    let u = e.json(&["catalog", "lint", "--dir", &d, "--universe"]);
    assert_eq!(u["data"]["universe"]["missing"][0]["slug"], "y");
    assert_eq!(e.code(&["catalog", "lint", "--dir", &d, "--universe", "--strict"]), 4, "coverage gap");
    assert_eq!(u["data"]["coverage"][0]["sets"]["clinical"]["reference"], 1);
    let text = e.run(&["catalog", "lint", "--dir", &d, "--universe"]);
    assert!(text.contains("universe-missing") && text.contains("PRIORITY"), "{text}");
}

#[test]
fn catalog_docs_are_generated_and_checked() {
    let e = Env::new();
    let out = e.dir.path().join("docs");
    let o = out.to_string_lossy().into_owned();
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("gone.md"), "stale").unwrap();
    assert_eq!(e.code(&["catalog", "docs", "--out", &o, "--check"]), 4);
    e.run(&["catalog", "docs", "--out", &o]);
    assert!(!out.join("gone.md").exists(), "stale pages are removed");
    let glucose = std::fs::read_to_string(out.join("glucose.md")).unwrap();
    assert!(glucose.contains("| clinical | warn | all | - | 125 |"), "{glucose}");
    assert!(glucose.contains("uncited (legacy)"), "{glucose}");
    assert!(glucose.contains("[2345-7](https://loinc.org/2345-7/)"), "{glucose}");
    let index = std::fs::read_to_string(out.join("index.md")).unwrap();
    assert!(index.contains("## lipid") && index.contains("[Glucose](glucose.md)"), "{index}");
    assert!(std::fs::read_to_string(out.join("coverage.md")).unwrap().contains("| [hdl-c](hdl-c.md) |"));
    assert_eq!(e.code(&["catalog", "docs", "--out", &o, "--check"]), 0);
}

/// The committed docs/markers match the catalog. Regenerate with
/// `cargo run -- catalog docs` (or `BIOMARKER_BLESS=1 cargo test --test catalog`).
#[test]
fn committed_marker_docs_are_up_to_date() {
    let e = Env::new();
    let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs").join("markers");
    let o = out.to_string_lossy().into_owned();
    if std::env::var_os("BIOMARKER_BLESS").is_some() {
        e.run(&["catalog", "docs", "--out", &o]);
    }
    let check = e.out(&["catalog", "docs", "--out", &o, "--check"]);
    assert!(
        check.status.success(),
        "docs/markers is stale; run `cargo run -- catalog docs`:\n{}",
        String::from_utf8_lossy(&check.stderr)
    );
}

#[test]
fn schema_and_sets_are_published() {
    let e = Env::new();
    let schema: Value = serde_json::from_str(&e.run(&["catalog", "schema"])).unwrap();
    assert_eq!(schema["required"], serde_json::json!(["slug", "name", "category", "unit"]));
    let sets = e.data(&["catalog", "sets"]);
    let names: Vec<&str> = sets.iter().map(|s| s["name"].as_str().unwrap()).collect();
    assert_eq!(names[..3], ["clinical", "optimal", "blueprint"]);
    assert!(names.contains(&"population-ri") && names.iter().any(|n| n.starts_with("alt-")));
}
