//! Truth-table tests for interventions (docs/interventions.md section 8):
//! migration from schema 5, regimen date validation and overlap, the empty
//! grid and imports naming interventions not in the catalog. All data is
//! synthetic.

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::{json, Value};
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

    fn out(&self, args: &[&str]) -> std::process::Output {
        Command::cargo_bin("biomarker")
            .unwrap()
            .env_clear()
            .envs(std::env::var_os("SYSTEMROOT").map(|v| ("SYSTEMROOT", v)))
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("BIOMARKER_DB", self.path("test.db"))
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY", TEST_KEY)
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("NO_COLOR", "1")
            .args(args)
            .output()
            .unwrap()
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

    fn fail(&self, args: &[&str], code: i32) -> String {
        let out = self.out(args);
        assert_eq!(out.status.code(), Some(code), "biomarker {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stderr).into_owned()
    }

    fn json(&self, args: &[&str]) -> Value {
        let mut a = args.to_vec();
        a.extend(["--format", "json"]);
        let v: Value = serde_json::from_str(&self.run(&a)).unwrap();
        assert_eq!(v["schema"], "biomarker/v1");
        v
    }
}

fn fixtures() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/interactions").to_string_lossy().into_owned()
}

/// Run `f` on a thread with a large stack (the SQL engine recurses deeply in debug builds).
fn big_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new().stack_size(256 * 1024 * 1024).spawn(f).unwrap().join().unwrap()
}

/// Every row of the pre-existing tables (their schema-5 columns; migration 7
/// adds columns), rendered for comparison.
fn snapshot(path: &Path) -> Vec<(String, Vec<String>)> {
    let p = path.to_path_buf();
    big_stack(move || {
        let db = biomarker_cli::db::Db::open_plain(&p).unwrap();
        let cols = |t: &str| match t {
            "people" => "id, slug, name, sex, dob, notes, tags, created_at",
            "ranges" => "id, marker_id, kind, sex, age_min, age_max, low, high, note",
            _ => "*",
        };
        ["people", "markers", "marker_aliases", "ranges", "person_ranges", "measurements", "observations"]
            .iter()
            .map(|t| {
                let rows = db.query(&format!("SELECT {} FROM {t} ORDER BY 1", cols(t)), &[]).unwrap();
                ((*t).to_string(), rows.iter().map(|r| format!("{r:?}")).collect())
            })
            .collect()
    })
}

#[test]
fn row1_schema_5_with_warn_zones_migrates_to_6_losslessly() {
    use biomarker_cli::db::{int, text, Db};
    use biomarker_cli::migrations::{self, MIGRATIONS};
    let e = Env::new();
    let path = e.path("v5.db");
    let p = path.clone();
    big_stack(move || {
        let db = Db::open_plain(&p).unwrap();
        migrations::applied(&db).unwrap();
        for m in MIGRATIONS.iter().filter(|m| m.version <= 5) {
            (m.apply)(&db).unwrap();
            db.execute(
                "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, '2025-01-01T00:00:00Z')",
                &[int(m.version), text(m.name)],
            )
            .unwrap();
        }
        db.execute("PRAGMA user_version = 5", &[]).unwrap();
        db.execute_batch(
            "INSERT INTO people (slug, sex, dob, created_at) VALUES ('alex', 'male', '1984-06-01', '2025-01-01T00:00:00Z');
             INSERT INTO measurements (person_id, marker_id, taken_at, value_raw, unit_raw, value, created_at)
               SELECT 1, id, '2025-03-10T08:00:00Z', 110, 'mg/dL', 110, '2025-03-10T09:00:00Z' FROM markers WHERE slug = 'glucose';
             INSERT INTO measurements (person_id, marker_id, taken_at, value_raw, unit_raw, value, created_at)
               SELECT 1, id, '2025-03-10T08:00:00Z', 6.2, '%', 6.2, '2025-03-10T09:00:00Z' FROM markers WHERE slug = 'hba1c';
             INSERT INTO ranges (marker_id, kind, sex, age_min, age_max, low, high, note)
               SELECT id, 'optimal', 'male', 18, 65, 70, 85, 'custom' FROM markers WHERE slug = 'glucose';
             INSERT INTO person_ranges (person_id, marker_id, kind, low, high, note)
               SELECT 1, id, 'warn', NULL, 6.0, 'personal warn zone' FROM markers WHERE slug = 'hba1c';
             INSERT INTO observations (person_id, marker_id, taken_at, text, created_at)
               SELECT 1, id, '2025-03-10T08:00:00Z', 'negative', '2025-03-10T09:00:00Z' FROM markers WHERE slug = 'glucose';",
        )
        .unwrap();
        let warn = db.query_scalar_i64("SELECT count(*) FROM ranges WHERE kind = 'warn'", &[]).unwrap();
        assert!(warn >= 7, "migration 5 seeded {warn} warn zones");
        assert_eq!(migrations::current_version(&db).unwrap(), 5);
    });
    let before = snapshot(&path);
    let db = path.to_string_lossy().into_owned();
    let plain = |args: &[&str]| {
        let mut a = vec!["--db", db.as_str(), "--insecure-plaintext", "-q"];
        a.extend_from_slice(args);
        e.json(&a)
    };
    let status = plain(&["db", "migrate", "--status"]);
    assert_eq!(status["current_version"], 5);
    plain(&["db", "migrate"]);
    let info = plain(&["db", "info"]);
    assert_eq!(info["data"]["schema_version"], migrations::latest_version());
    assert_eq!((info["data"]["interventions"].as_i64(), info["data"]["regimens"].as_i64()), (Some(0), Some(0)));
    assert_eq!(snapshot(&path), before, "migrations 6 and 7 changed pre-existing rows");
    let p = path.clone();
    let user_version =
        big_stack(move || Db::open_plain(&p).unwrap().query_scalar_i64("PRAGMA user_version", &[]).unwrap());
    assert_eq!(user_version, migrations::latest_version());

    // The warn zones still drive the status: glucose 110 is near-high (99 < 110 <= 125
    // from migration 5), hba1c 6.2 is past alex's personal warn bound of 6.0
    // (it would be near-high under the built-in 6.4).
    let q = plain(&["query", "-p", "alex"]);
    let status: Vec<(&str, &str)> = q["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["marker"].as_str().unwrap(), r["status"].as_str().unwrap()))
        .collect();
    assert!(status.contains(&("glucose", "near-high")), "{status:?}");
    assert!(status.contains(&("hba1c", "high")), "{status:?}");
    assert_eq!(status.len(), 2, "{status:?}");
    // A second migrate is a no-op and the new tables are usable.
    plain(&["db", "migrate"]);
    plain(&["intervention", "add", "zinc"]);
    plain(&["regimen", "add", "zinc", "-p", "alex", "--start", "2025-01-01"]);
    assert_eq!(snapshot(&path), before);
}

#[test]
fn row2_stop_not_after_start_is_rejected() {
    let e = Env::new();
    e.run(&["person", "add", "alex"]);
    e.run(&["intervention", "add", "zinc"]);
    let err = e.fail(&["regimen", "add", "zinc", "-p", "alex", "--start", "2025-03-10", "--stop", "2025-03-01"], 4);
    assert!(err.contains("stop date 2025-03-01 must be after start date 2025-03-10"), "{err}");
    // Equal dates are a zero-day regimen: rejected too.
    let err = e.fail(&["regimen", "add", "zinc", "-p", "alex", "--start", "2025-03-10", "--stop", "2025-03-10"], 4);
    assert!(err.contains("must be after start date"), "{err}");
    assert_eq!(e.json(&["regimen", "ls", "-p", "alex"])["count"], 0);
    // `regimen stop` applies the same rule.
    e.run(&["regimen", "add", "zinc", "-p", "alex", "--start", "2025-03-10"]);
    let err = e.fail(&["regimen", "stop", "1", "--on", "2025-03-10"], 4);
    assert!(err.contains("the stop date must be after it"), "{err}");
    let ok = e.json(&["regimen", "add", "zinc", "-p", "alex", "--start", "2024-01-01", "--stop", "2024-01-02"]);
    assert_eq!(ok["data"]["stopped_on"], "2024-01-02");
}

#[test]
fn row3_overlapping_regimens_are_kept_with_a_warning() {
    let e = Env::new();
    e.run(&["person", "add", "alex"]);
    e.run(&["person", "add", "sam"]);
    e.run(&["intervention", "add", "zinc"]);
    let add = |args: &[&str]| {
        let mut a = vec!["regimen", "add", "zinc"];
        a.extend_from_slice(args);
        String::from_utf8_lossy(&e.out(&a).stderr).into_owned()
    };
    assert!(!add(&["-p", "alex", "--start", "2025-01-01", "--stop", "2025-03-01", "--dose", "15"]).contains("overlaps"));
    // Adjacent (stop == next start) is a hand-over, not an overlap.
    assert!(!add(&["-p", "alex", "--start", "2025-03-01", "--dose", "30"]).contains("overlaps"));
    // Another person never overlaps.
    assert!(!add(&["-p", "sam", "--start", "2025-02-01"]).contains("overlaps"));
    // A second open regimen overlapping regimen 2: warned, still added.
    let err = add(&["-p", "alex", "--start", "2025-04-01", "--dose", "10", "--slot", "evening"]);
    assert!(err.contains("overlaps regimen 2"), "{err}");
    assert_eq!(e.json(&["regimen", "ls", "-p", "alex"])["count"], 3);
    // The intervention is listed once per day in joins and the grid.
    let g = e.json(&["interaction", "grid", "-p", "alex", "--as-of", "2025-05-01"]);
    assert_eq!(g["data"]["rows"], json!(["zinc"]));
    let taking = e.json(&["regimen", "ls", "-p", "alex", "--as-of", "2025-05-01", "--status", "taking"]);
    assert_eq!(taking["count"], 2);
    // Stopping by intervention stops every overlapping active regimen.
    let stopped = e.json(&["regimen", "stop", "zinc", "-p", "alex", "--on", "2025-06-01"]);
    assert_eq!(stopped["count"], 2);
    let ids: Vec<i64> = stopped["data"].as_array().unwrap().iter().map(|r| r["id"].as_i64().unwrap()).collect();
    assert_eq!(ids, [2, 4]);
    assert_eq!(e.json(&["regimen", "ls", "-p", "sam", "--status", "taking", "--as-of", "2025-06-01"])["count"], 1);
}

#[test]
fn row4_grid_without_active_regimens_is_empty() {
    let e = Env::new();
    e.run(&["person", "add", "alex"]);
    e.run(&["intervention", "import", &fixtures()]);
    for catalog in [false, true] {
        let mut args = vec!["interaction", "grid", "-p", "alex", "--as-of", "2025-03-01"];
        if catalog {
            args.push("--catalog");
        }
        let g = e.json(&args);
        assert_eq!(g["kind"], "interaction_grid");
        let d = &g["data"];
        assert_eq!((&d["rows"], &d["columns"]), (&json!([]), &json!([])), "{g}");
        assert_eq!((&d["cells"], &d["other"]), (&json!([]), &json!([])), "{g}");
        let out = e.out(&args);
        assert_eq!(out.status.code(), Some(0));
        assert!(String::from_utf8_lossy(&out.stderr).contains("alex is taking nothing on 2025-03-01"));
        args.extend(["-f", "table"]);
        e.run(&args);
    }
    // Only a stopped and a planned regimen: still empty on the day.
    e.run(&["regimen", "add", "biotin", "-p", "alex", "--start", "2024-01-01", "--stop", "2025-03-01"]);
    e.run(&["regimen", "add", "fish-oil", "-p", "alex", "--start", "2025-03-02"]);
    let g = e.json(&["interaction", "grid", "-p", "alex", "--as-of", "2025-03-01"]);
    assert_eq!((&g["data"]["rows"], &g["data"]["cells"]), (&json!([]), &json!([])));
}

#[test]
fn row5_edges_to_unknown_interventions_are_kept_unresolved_with_a_warning() {
    let e = Env::new();
    // The fixtures have vitamin-k2 -> intervention:warfarin; warfarin is not in the catalog.
    let out = e.out(&["intervention", "import", &fixtures(), "--format", "json"]);
    assert_eq!(out.status.code(), Some(0));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("intervention 'warfarin' is not in the catalog"), "{err}");
    // fish-oil -> vitamin-k2 resolves within the same import: no warning for it.
    assert!(!err.contains("'vitamin-k2'"), "{err}");
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["data"]["unresolved_targets"], json!(["warfarin"]));
    assert_eq!(v["data"]["interactions_inserted"], 7);
    // No stub was created; the edge exists, unresolved.
    let ls = e.json(&["intervention", "ls"]);
    assert!(!ls["data"].as_array().unwrap().iter().any(|i| i["slug"] == "warfarin"));
    let edge = |e: &Env| {
        e.json(&["interaction", "ls", "--intervention", "vitamin-k2", "--kind", "contraindicated"])["data"][0].clone()
    };
    assert_eq!((edge(&e)["target"].as_str(), edge(&e)["resolved"].as_bool()), (Some("warfarin"), Some(false)));
    // Once warfarin is added, the stored edge resolves and re-import no longer warns.
    e.run(&["intervention", "add", "warfarin", "--category", "drug"]);
    assert_eq!(edge(&e)["resolved"], true);
    let again = e.json(&["intervention", "import", &fixtures()]);
    assert_eq!(again["data"]["unresolved_targets"], json!([]));
    assert_eq!(again["data"]["interactions_unchanged"], 7);
    // A dry run reports the same unresolved targets.
    let dir = e.path("zinc");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("interactions.json"),
        r#"{"schema": "supplement-interactions/1", "interactions": [{"with": "Copper Gluconate", "kind": "caution"}]}"#,
    )
    .unwrap();
    let dry = e.json(&["intervention", "import", &dir.to_string_lossy(), "--dry-run"]);
    assert_eq!(dry["data"]["unresolved_targets"], json!(["copper-gluconate"]));
    assert_eq!(e.json(&["intervention", "ls"])["count"], 4);
}
