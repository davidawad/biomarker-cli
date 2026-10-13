//! End-to-end tests for interventions, regimens and interactions
//! (docs/interventions.md). All data is synthetic.

use std::path::{Path, PathBuf};

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
        Self { dir: TempDir::new().unwrap() }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
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

    /// alex, a warfarin entry and the three synthetic interactions.json documents.
    fn catalog(&self) {
        self.run(&["person", "add", "alex", "--sex", "male", "--dob", "1984-06-01"]);
        self.run(&[
            "intervention",
            "add",
            "warfarin",
            "--category",
            "drug",
            "--rx-status",
            "rx",
            "--class",
            "anticoagulant",
            "--ingredient",
            "warfarin=5mg",
        ]);
        self.run(&["intervention", "import", &fixtures()]);
    }

    /// fish-oil from 2025-01-10, warfarin from 2025-02-01, biotin during 2024-01-01..2024-06-01.
    fn regimens(&self) {
        self.run(&[
            "regimen",
            "add",
            "fish-oil",
            "-p",
            "alex",
            "--dose",
            "2",
            "--unit",
            "capsule",
            "--start",
            "2025-01-10",
        ]);
        self.run(&["regimen", "add", "warfarin", "-p", "alex", "--dose", "5", "--unit", "mg", "--start", "2025-02-01"]);
        self.run(&["regimen", "add", "biotin", "-p", "alex", "--start", "2024-01-01", "--stop", "2024-06-01"]);
    }
}

fn fixtures() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/interactions").to_string_lossy().into_owned()
}

fn slugs(v: &Value, key: &str) -> Vec<String> {
    v["data"].as_array().unwrap().iter().map(|r| r[key].as_str().unwrap().to_string()).collect()
}

#[test]
fn catalog_add_list_show_and_rm() {
    let e = Env::new();
    let v = e.json(&[
        "intervention",
        "add",
        "Magnesium-Glycinate",
        "--name",
        "Magnesium glycinate",
        "--rx-status",
        "otc",
        "--ingredient",
        "magnesium=120mg",
        "--alias",
        "mag",
    ]);
    assert_eq!(v["kind"], "intervention");
    assert_eq!(v["data"]["slug"], "magnesium-glycinate");
    assert_eq!(v["data"]["ingredients"][0]["amount"], 120.0);
    assert_eq!(v["data"]["aliases"][0], "mag");
    e.fail(&["intervention", "add", "magnesium-glycinate"], 4);
    e.fail(&["intervention", "add", "x", "--ingredient", "zinc=lots"], 4);
    assert_eq!(e.json(&["intervention", "ls", "--search", "magnesium"])["count"], 1);
    assert_eq!(e.json(&["iv", "ls", "--category", "drug"])["count"], 0);
    assert_eq!(e.json(&["intervention", "show", "mag"])["data"]["regimens"], 0);
    e.fail(&["intervention", "show", "nope"], 3);
    e.run(&["person", "add", "alex"]);
    e.run(&["regimen", "add", "mag", "-p", "alex", "--start", "2025-01-01"]);
    assert!(e.fail(&["intervention", "rm", "mag"], 4).contains("--force"));
    let v = e.json(&["intervention", "rm", "mag", "--force"]);
    assert_eq!(v["data"]["regimens_deleted"], 1);
    assert_eq!(e.json(&["regimen", "ls", "-p", "alex"])["count"], 0);
}

#[test]
fn import_is_idempotent_and_dry_run_writes_nothing() {
    let e = Env::new();
    let dry = e.json(&["intervention", "import", &fixtures(), "--dry-run"]);
    assert_eq!(dry["kind"], "intervention_import");
    assert_eq!(dry["data"]["dry_run"], true);
    assert_eq!(dry["data"]["interactions_inserted"], 7);
    assert_eq!(e.json(&["intervention", "ls"])["count"], 0);

    let v = e.json(&["intervention", "import", &fixtures()]);
    assert_eq!(v["data"]["files"], 3);
    assert_eq!(v["data"]["interventions_created"], serde_json::json!(["biotin", "fish-oil", "vitamin-k2"]));
    assert_eq!(v["data"]["interactions_inserted"], 7);
    let again = e.json(&["intervention", "import", &fixtures()]);
    assert_eq!(again["data"]["interactions_inserted"], 0);
    assert_eq!(again["data"]["interactions_unchanged"], 7);

    let fish = e.json(&["intervention", "show", "omega-3"])["data"].clone();
    assert_eq!(fish["slug"], "fish-oil");
    assert_eq!(fish["rx_status"], "otc");
    assert_eq!(fish["classes"][0], "omega-3-fatty-acid");
    assert_eq!(e.json(&["intervention", "show", "biotin"])["data"]["rx_status"], "otc");

    // Synonyms were normalised; citations are kept as given.
    let edges = e.json(&["interaction", "ls", "--intervention", "biotin"]);
    assert_eq!(edges["data"][0]["kind"], "lab-interference");
    assert_eq!(edges["data"][0]["target_kind"], "lab-test");
    assert_eq!(edges["data"][0]["target_name"], "Thyroid Stimulating Hormone");
    let cau = e.json(&["interaction", "ls", "--kind", "caution"]);
    assert_eq!(cau["count"], 1);
    assert_eq!(cau["data"][0]["citations"][1]["url"], "https://example.org/fish-oil");
    assert_eq!(cau["data"][0]["resolved"], Value::Null);
    // warfarin is not in the catalog yet: the edge exists but is unresolved.
    let k2 = e.json(&["interaction", "ls", "--intervention", "vitamin-k2", "--target-kind", "intervention"]);
    // Both directions count: fish-oil -> vitamin-k2 and vitamin-k2 -> warfarin.
    let resolved: Vec<(&str, &Value)> =
        k2["data"].as_array().unwrap().iter().map(|r| (r["target"].as_str().unwrap(), &r["resolved"])).collect();
    assert_eq!(resolved, [("vitamin-k2", &Value::Bool(true)), ("warfarin", &Value::Bool(false))]);
    assert_eq!(e.json(&["interaction", "ls", "--min-severity", "major"])["count"], 2);

    // An updated edge is reported as updated.
    let dir = e.path("research/fish-oil");
    std::fs::create_dir_all(&dir).unwrap();
    let doc = std::fs::read_to_string(Path::new(&fixtures()).join("fish-oil/interactions.json")).unwrap();
    std::fs::write(dir.join("interactions.json"), doc.replace("\"moderate\"", "\"major\"")).unwrap();
    let v = e.json(&["intervention", "import", &e.path("research").to_string_lossy()]);
    assert_eq!(
        (v["data"]["interactions_updated"].as_u64(), v["data"]["interactions_unchanged"].as_u64()),
        (Some(1), Some(3))
    );
}

#[test]
fn invalid_documents_abort_unless_skipped() {
    let e = Env::new();
    let bad = e.path("zinc/interactions.json");
    std::fs::create_dir_all(bad.parent().unwrap()).unwrap();
    std::fs::write(
        &bad,
        r#"{"schema": "supplement-interactions/1",
            "interactions": [{"with": "copper", "kind": "caution"}, {"with": "iron", "kind": "frobnicate"}]}"#,
    )
    .unwrap();
    let f = bad.to_string_lossy().into_owned();
    assert!(e.fail(&["intervention", "import", &f], 4).contains("frobnicate"));
    assert_eq!(e.json(&["intervention", "ls"])["count"], 0);
    let v = e.json(&["intervention", "import", &f, "--skip-invalid"]);
    assert_eq!(v["data"]["interventions_created"][0], "zinc");
    assert_eq!((v["data"]["interactions_inserted"].as_u64(), v["data"]["invalid"].as_u64()), (Some(1), Some(1)));
    std::fs::write(&bad, r#"{"schema": "other/9", "supplement": "zinc"}"#).unwrap();
    assert!(e.fail(&["intervention", "import", &f], 4).contains("unsupported schema"));
    e.fail(&["intervention", "import", &e.path("missing").to_string_lossy()], 6);
}

#[test]
fn imports_real_shape_edges_with_sibling_target_kind() {
    let e = Env::new();
    let f = e.path("thyroid-support/interactions.json");
    std::fs::create_dir_all(f.parent().unwrap()).unwrap();
    std::fs::write(
        &f,
        r#"{
        "schema":"supplement-interactions/1", "supplement":"Thyroid Support",
        "slug":"thyroid-support", "aliases":[], "category":"supplement",
        "rx_status":"otc", "evidence_certainty_overall":"moderate", "citations":[],
        "edges":[{"with":"thyroid hormone", "with_kind":"drug-class", "kind":"monitor",
          "severity":"moderate", "mechanism":"synthetic monitoring note", "evidence":"synthetic",
          "citations":["PMID:00000001"]}]
    }"#,
    )
    .unwrap();
    let v = e.json(&["intervention", "import", &f.to_string_lossy()]);
    assert_eq!(v["data"]["interactions_inserted"], 1);
    let edges = e.json(&["interaction", "ls", "--intervention", "thyroid-support"]);
    assert_eq!(edges["data"][0]["target_kind"], "drug-class");
    assert_eq!(edges["data"][0]["target"], "thyroid-hormone");
}

#[test]
fn nonempty_unparseable_edges_fail_and_warn_when_skipped() {
    let e = Env::new();
    let f = e.path("zinc/interactions.json");
    std::fs::create_dir_all(f.parent().unwrap()).unwrap();
    std::fs::write(
        &f,
        r#"{"schema":"supplement-interactions/1", "supplement":"zinc",
        "interactions":[], "edges":[{"unexpected":"shape"}]}"#,
    )
    .unwrap();
    let file = f.to_string_lossy().into_owned();
    assert!(e.fail(&["intervention", "import", &file], 4).contains("interaction has no target"));
    let skipped = e.json(&["intervention", "import", &file, "--skip-invalid"]);
    assert_eq!(skipped["data"]["invalid"], 1);
    assert!(skipped["data"]["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|w| w.as_str().unwrap().contains("zero parsed edges")));
}

#[test]
fn regimen_status_as_of_and_stop() {
    let e = Env::new();
    e.catalog();
    e.regimens();
    let at = |d: &str, status: &str| {
        slugs(&e.json(&["regimen", "ls", "-p", "alex", "--as-of", d, "--status", status]), "intervention")
    };
    assert_eq!(at("2025-03-01", "taking"), ["fish-oil", "warfarin"]);
    assert_eq!(at("2025-01-20", "planned"), ["warfarin"]);
    assert_eq!(at("2025-03-01", "stopped"), ["biotin"]);
    assert_eq!(at("2024-05-31", "taking"), ["biotin"]);
    assert!(at("2024-06-01", "taking").is_empty());
    let ls = e.json(&["regimen", "ls", "-p", "alex", "--as-of", "2025-01-12", "--intervention", "omega-3"]);
    assert_eq!(ls["data"][0]["days"], 3);
    assert_eq!(ls["data"][0]["dose"], 2.0);
    assert_eq!(ls["as_of"], "2025-01-12");

    let stopped = e.json(&["regimen", "stop", "fish-oil", "-p", "alex", "--on", "2025-04-01"]);
    assert_eq!(stopped["data"][0]["stopped_on"], "2025-04-01");
    assert_eq!(at("2025-04-01", "taking"), ["warfarin"]);
    assert!(e
        .fail(&["regimen", "stop", "fish-oil", "-p", "alex", "--on", "2025-05-01"], 3)
        .contains("no active regimen"));
    e.fail(&["regimen", "stop", "2", "--on", "2025-01-01"], 4);
    e.fail(&["regimen", "add", "biotin", "-p", "alex", "--start", "2025-01-01", "--stop", "2024-01-01"], 4);
    let out = e.out(&["regimen", "add", "warfarin", "-p", "alex", "--start", "2025-03-01", "--dose", "7.5"]);
    assert!(String::from_utf8_lossy(&out.stderr).contains("overlaps regimen 2"));
    let rm = e.json(&["regimen", "rm", "4"]);
    assert_eq!(rm["data"]["regimens"][0], 4);
    e.fail(&["regimen", "rm", "4"], 3);
    // person rm takes the regimens along.
    e.run(&["person", "rm", "alex"]);
    assert_eq!(e.json(&["db", "info"])["data"]["regimens"], 0);
}

#[test]
fn grid_is_a_contraindication_matrix() {
    let e = Env::new();
    e.catalog();
    e.regimens();
    let g = e.json(&["interaction", "grid", "-p", "alex", "--as-of", "2025-03-01"]);
    assert_eq!(g["kind"], "interaction_grid");
    let d = &g["data"];
    assert_eq!(d["rows"], serde_json::json!(["fish-oil", "warfarin"]));
    assert_eq!(d["columns"], d["rows"]);
    let cells = d["cells"].as_array().unwrap();
    assert_eq!(cells.len(), 2);
    assert_eq!(cells[0]["kind"], "caution");
    assert_eq!(cells[0]["edges"][0]["via"], "drug-class:anticoagulant");
    let other: Vec<&str> = d["other"].as_array().unwrap().iter().map(|o| o["target"].as_str().unwrap()).collect();
    assert_eq!(other, ["bleeding-disorder", "triglycerides"]);

    let c = e.json(&["interaction", "grid", "-p", "alex", "--as-of", "2025-03-01", "--catalog"]);
    assert_eq!(c["data"]["columns"], serde_json::json!(["fish-oil", "warfarin", "vitamin-k2"]));
    let k2 = c["data"]["cells"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["row"] == "warfarin" && c["column"] == "vitamin-k2")
        .cloned();
    assert_eq!(k2.unwrap()["kind"], "contraindicated");

    let csv = e.run(&["interaction", "grid", "-p", "alex", "--as-of", "2025-03-01", "-f", "csv", "--null", ""]);
    assert_eq!(csv, "intervention,fish-oil,warfarin\nfish-oil,,caution\nwarfarin,caution,\n");
    let table = e.run(&["interaction", "grid", "-p", "alex", "--as-of", "2025-03-01"]);
    assert!(table.contains("bleeding-disorder"), "{table}");

    let empty = e.json(&["interaction", "grid", "-p", "alex", "--as-of", "2020-01-01"]);
    assert_eq!(empty["data"]["rows"], serde_json::json!([]));
}

#[test]
fn interaction_add_and_rm() {
    let e = Env::new();
    e.catalog();
    let v = e.json(&[
        "interaction",
        "add",
        "warfarin",
        "condition:Liver Disease",
        "--kind",
        "Caution",
        "--severity",
        "high",
        "--cite",
        "PMID:00000003",
    ]);
    assert_eq!(v["data"]["outcome"], "inserted");
    assert_eq!((v["data"]["target"].as_str(), v["data"]["severity"].as_str()), (Some("liver-disease"), Some("major")));
    let again = e.json(&[
        "interaction",
        "add",
        "warfarin",
        "condition:liver-disease",
        "--kind",
        "caution",
        "--severity",
        "minor",
    ]);
    assert_eq!(again["data"]["outcome"], "replaced");
    e.fail(&["interaction", "add", "warfarin", "planet:mars", "--kind", "caution"], 4);
    e.fail(&["interaction", "add", "warfarin", "x", "--kind", "meh"], 4);
    let id = v["data"]["id"].as_i64().unwrap().to_string();
    e.run(&["interaction", "rm", &id]);
    e.fail(&["interaction", "rm", &id], 3);
    // Removing an intervention removes the edges from and to it.
    e.run(&["intervention", "rm", "warfarin"]);
    let left = e.json(&["interaction", "ls", "--target-kind", "intervention"]);
    assert_eq!(slugs(&left, "target"), ["vitamin-k2"]);
}

#[test]
fn query_trend_and_impact_join_regimens() {
    let e = Env::new();
    e.catalog();
    e.regimens();
    for (marker, value, date) in [
        ("tsh", "2.0", "2024-03-01"),
        ("triglycerides", "160", "2024-12-01"),
        ("triglycerides", "150", "2025-01-10"),
        ("triglycerides", "120", "2025-02-15"),
        ("triglycerides", "100", "2025-03-20"),
    ] {
        e.run(&["add", marker, value, "-p", "alex", "-d", date]);
    }
    let q = e.json(&["query", "-p", "alex", "--with-regimens"]);
    let rows = q["data"].as_array().unwrap();
    assert_eq!(rows[0]["regimens"], serde_json::json!(["biotin"]));
    assert_eq!(rows[0]["interferences"], serde_json::json!(["biotin"]));
    assert_eq!(rows[1]["regimens"], serde_json::json!([]));
    assert_eq!(rows[2]["regimens"], serde_json::json!(["fish-oil"]));
    assert_eq!(rows[4]["regimens"], serde_json::json!(["fish-oil", "warfarin"]));
    assert_eq!(rows[4]["interferences"], serde_json::json!(["fish-oil"]));
    assert!(e.json(&["query", "-p", "alex"])["data"][0].get("regimens").is_none());
    assert_eq!(e.json(&["latest", "-p", "alex", "--with-regimens"])["data"][1]["regimens"][1], "warfarin");

    let t = e.json(&["trend", "-p", "alex", "-m", "triglycerides", "--with-regimens"]);
    let s = &t["data"][0];
    let events: Vec<(&str, &str)> = s["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (v["event"].as_str().unwrap(), v["intervention"].as_str().unwrap()))
        .collect();
    assert_eq!(events, [("start", "fish-oil"), ("start", "warfarin")]);
    assert_eq!(s["points"][3]["regimens"], serde_json::json!(["fish-oil", "warfarin"]));
    assert!(e.json(&["trend", "-p", "alex", "-m", "triglycerides"])["data"][0].get("events").is_none());

    let i = e.json(&["regimen", "impact", "fish-oil", "-p", "alex", "--window", "90d"]);
    assert_eq!(i["kind"], "regimen_impact");
    assert_eq!(i["count"], 1);
    let r = &i["data"][0];
    assert_eq!((r["event"].as_str(), r["marker"].as_str()), (Some("start"), Some("triglycerides")));
    assert_eq!(
        (r["n_before"].as_u64(), r["before"].as_f64(), r["before_date"].as_str()),
        (Some(2), Some(150.0), Some("2025-01-10"))
    );
    assert_eq!((r["n_after"].as_u64(), r["after"].as_f64()), (Some(2), Some(100.0)));
    assert_eq!(r["change"], -50.0);
    assert_eq!(r["before_mean"], 155.0);
    // The stop event is reported once the regimen stops.
    e.run(&["regimen", "stop", "fish-oil", "-p", "alex", "--on", "2025-03-01"]);
    let i = e.json(&["regimen", "impact", "fish-oil", "-p", "alex", "-m", "triglycerides"]);
    assert_eq!(slugs(&i, "event"), ["start", "stop"]);
    e.fail(&["regimen", "impact", "fish-oil", "-p", "alex", "--window", "0d"], 2);
}

#[test]
fn encrypted_at_rest_and_audited() {
    let e = Env::new();
    e.catalog();
    e.run(&["regimen", "add", "fish-oil", "-p", "alex", "--note", "SECRET-REGIMEN-NOTE", "--start", "2025-01-01"]);
    e.run(&["interaction", "grid", "-p", "alex"]);
    for entry in std::fs::read_dir(e.dir.path()).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            let bytes = std::fs::read(&p).unwrap();
            for needle in ["SECRET-REGIMEN-NOTE", "Fish oil (EPA/DHA)", "anticoagulant"] {
                assert!(!bytes.windows(needle.len()).any(|w| w == needle.as_bytes()), "{} leaks {needle}", p.display());
            }
        }
    }
    let log = e.json(&["audit", "log"]);
    let cmds: Vec<&str> = log["data"].as_array().unwrap().iter().map(|r| r["command"].as_str().unwrap()).collect();
    for c in ["intervention import", "regimen add", "interaction grid"] {
        assert!(cmds.contains(&c), "{cmds:?}");
    }
    let changed = log["data"].as_array().unwrap().iter().find(|r| r["command"] == "regimen add").unwrap();
    assert_eq!(changed["rows_changed"], 1);
}

#[test]
fn migrates_a_version_4_database() {
    use biomarker_cli::db::{text, Db};
    use biomarker_cli::migrations::{self, MIGRATIONS};
    let e = Env::new();
    let path = e.path("old.db");
    let p = path.clone();
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let db = Db::open_plain(&p).unwrap();
            migrations::applied(&db).unwrap();
            for m in MIGRATIONS.iter().filter(|m| m.version <= 4) {
                (m.apply)(&db).unwrap();
                db.execute(
                    "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, '2025-01-01T00:00:00Z')",
                    &[biomarker_cli::db::int(m.version), text(m.name)],
                )
                .unwrap();
            }
            db.execute("INSERT INTO people (slug, created_at) VALUES ('alex', '2025-01-01T00:00:00Z')", &[]).unwrap();
            assert_eq!(migrations::current_version(&db).unwrap(), 4);
        })
        .unwrap()
        .join()
        .unwrap();
    let db = path.to_string_lossy().into_owned();
    let plain = |args: &[&str]| {
        let mut a = vec!["--db", &db, "--insecure-plaintext", "-q"];
        a.extend_from_slice(args);
        a.extend(["-f", "json"]);
        serde_json::from_str::<Value>(&e.run(&a)).unwrap()
    };
    let status = plain(&["db", "migrate", "--status"]);
    assert_eq!((status["current_version"].as_i64(), status["data"][4]["applied"].as_bool()), (Some(4), Some(false)));
    plain(&["intervention", "add", "zinc"]);
    plain(&["regimen", "add", "zinc", "-p", "alex", "--start", "2025-01-01"]);
    let info = plain(&["db", "info"]);
    assert_eq!((info["data"]["schema_version"].as_i64(), info["data"]["people"].as_i64()), (Some(6), Some(1)));
    assert_eq!(info["data"]["regimens"], 1);
}
