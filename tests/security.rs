//! Encryption-at-rest tests: nothing personal reaches disk in plaintext,
//! wrong keys and tampering fail cleanly, rekey / migration / audit work.

use std::path::{Path, PathBuf};
use std::time::Instant;

use assert_cmd::Command;
use serde_json::Value;
use tempfile::TempDir;

const KEY_A: &str = "raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
const KEY_B: &str = "raw:ffeeddccbbaa99887766554433221100ffeeddccbbaa99887766554433221100";
/// Distinctive strings that must never appear in any file we write.
const MARK_NAME: &str = "Zyxwquartz Plaintextson";
const MARK_NOTE: &str = "SECRETNOTE-7f3a9c";
const MARK_LAB: &str = "LabCorpMarkerXYZ";

struct Env {
    dir: TempDir,
    key: String,
}

impl Env {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join("data")).unwrap();
        std::fs::create_dir_all(dir.path().join("in")).unwrap();
        Self { dir, key: KEY_A.into() }
    }
    fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }
    fn db(&self) -> PathBuf {
        self.path("data/labs.db")
    }
    fn cmd(&self) -> Command {
        let mut c = Command::cargo_bin("biomarker").unwrap();
        c.env_clear()
            .env("HOME", self.dir.path())
            .env("XDG_CONFIG_HOME", self.path("data/config"))
            .env("XDG_DATA_HOME", self.path("data/xdg"))
            .env("BIOMARKER_DB", self.db())
            .env("BIOMARKER_TZ", "UTC")
            .env("BIOMARKER_KEY_SOURCE", "env")
            .env("BIOMARKER_NO_KEYCHAIN", "1")
            .env("NO_COLOR", "1");
        if !self.key.is_empty() {
            c.env("BIOMARKER_KEY", &self.key);
        }
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
        serde_json::from_str(&self.run(&a)).unwrap()
    }
    /// Populate with personal data containing the marker strings.
    fn populate(&self, extra: &[&str]) {
        fn with<'a>(a: &[&'a str], extra: &[&'a str]) -> Vec<&'a str> {
            [a, extra].concat()
        }
        self.run(&with(
            &["person", "add", "alex", "--name", MARK_NAME, "--dob", "1984-06-01", "--notes", MARK_NOTE],
            extra,
        ));
        self.run(&with(
            &["add", "ldl", "131", "mg/dL", "-p", "alex", "-d", "2024-01-05", "--note", MARK_NOTE, "--lab", MARK_LAB],
            extra,
        ));
        let csv = self.path("in/import.csv");
        std::fs::write(&csv, format!("person,marker,value,unit,date,lab\nalex,hdl,48,mg/dL,2024-02-01,{MARK_LAB}\n"))
            .unwrap();
        self.run(&with(&["import", csv.to_str().unwrap()], extra));
    }
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(files_under(&p));
        } else {
            out.push(p);
        }
    }
    out
}

fn contains(hay: &[u8], needle: &str) -> bool {
    hay.windows(needle.len()).any(|w| w == needle.as_bytes())
}

/// Assert that no file under `dir` contains any marker string.
fn assert_no_plaintext(dir: &Path) {
    let files = files_under(dir);
    assert!(!files.is_empty());
    for f in files {
        let bytes = std::fs::read(&f).unwrap();
        for m in [MARK_NAME, MARK_NOTE, MARK_LAB, "alex", "SQLite format 3"] {
            assert!(!contains(&bytes, m), "{} contains plaintext {m:?}", f.display());
        }
    }
}

// ---------------------------------------------------------------------------
// Step 1: does fsqlite's documented `PRAGMA fsqlite.key` encrypt? (No.)
// ---------------------------------------------------------------------------

fn fsqlite_with_pragma(dir: &Path, pragmas: &[&str]) -> Vec<(PathBuf, bool)> {
    let path = dir.join("pragma.db");
    let p = path.to_string_lossy().into_owned();
    let pragmas: Vec<String> = pragmas.iter().map(|s| (*s).to_string()).collect();
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let rt = asupersync::runtime::RuntimeBuilder::current_thread().build().unwrap();
            rt.block_on(async {
                let conn = fsqlite::Connection::open(p).await.unwrap();
                for pr in &pragmas {
                    // Unknown pragmas are accepted silently.
                    conn.execute(pr).await.unwrap();
                }
                conn.execute("CREATE TABLE t (v TEXT)").await.unwrap();
                conn.execute(&format!("INSERT INTO t VALUES ('{MARK_NOTE}')")).await.unwrap();
                conn.close().await.unwrap();
            });
        })
        .unwrap()
        .join()
        .unwrap();
    std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with("pragma.db"))
        .map(|p| {
            let found = contains(&std::fs::read(&p).unwrap(), MARK_NOTE);
            (p, found)
        })
        .collect()
}

#[test]
fn fsqlite_pragma_key_does_not_encrypt() {
    for pragmas in [&["PRAGMA fsqlite.key = 'correct horse battery staple'"][..], &["PRAGMA key = 'pw'"][..]] {
        let dir = TempDir::new().unwrap();
        let files = fsqlite_with_pragma(dir.path(), pragmas);
        eprintln!("{pragmas:?}: {files:?}");
        // Documented in docs/security.md: the marker is readable in the main
        // file and/or its -wal sidecar, i.e. the pragma is a silent no-op in
        // fsqlite 0.4.7. If this ever fails, fsqlite gained real page
        // encryption and docs/security.md must be revisited.
        assert!(files.iter().any(|(_, found)| *found), "marker not found in {files:?}: fsqlite now encrypts?");
    }
}

// ---------------------------------------------------------------------------
// The sealed container
// ---------------------------------------------------------------------------

#[test]
fn no_plaintext_in_any_written_file() {
    let e = Env::new();
    e.populate(&[]);
    e.run(&["query", "--all-people"]);
    e.run(&["db", "backup", e.path("data/backup.db").to_str().unwrap()]);
    e.run(&["db", "vacuum"]);
    let enc = e.path("data/export.age");
    e.run(&["export", "--encrypt-output", "--recipient", &age_identity().1, "-o", enc.to_str().unwrap()]);
    e.run(&["audit", "log"]);
    let names: Vec<String> =
        files_under(&e.path("data")).iter().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()).collect();
    for expected in ["labs.db", "labs.db.audit", "labs.db.lock", "backup.db", "export.age"] {
        assert!(names.iter().any(|n| n == expected), "{expected} missing from {names:?}");
    }
    for stray in ["labs.db-wal", "labs.db-shm", "labs.db-journal"] {
        assert!(!names.iter().any(|n| n == stray), "unexpected sidecar {stray}");
    }
    assert_no_plaintext(&e.path("data"));
    // ... and the data is really there.
    let v = e.json(&["query", "--all-people"]);
    assert_eq!(v["count"], 2);
    assert_eq!(e.json(&["person", "show", "alex"])["data"]["name"], MARK_NAME);
}

#[cfg(unix)]
#[test]
fn files_are_private() {
    use std::os::unix::fs::PermissionsExt;
    let e = Env::new();
    e.populate(&[]);
    for f in ["data/labs.db", "data/labs.db.audit"] {
        let m = std::fs::metadata(e.path(f)).unwrap().permissions().mode() & 0o777;
        assert_eq!(m, 0o600, "{f} mode {m:o}");
    }
}

#[test]
fn new_database_directory_is_owner_only() {
    use std::os::unix::fs::PermissionsExt;
    let e = Env::new();
    let fresh = e.path("data/fresh/labs.db");
    let out = e.cmd().env("BIOMARKER_DB", &fresh).args(["db", "init"]).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let m = std::fs::metadata(e.path("data/fresh")).unwrap().permissions().mode() & 0o777;
    assert_eq!(m, 0o700, "new db directory mode {m:o}");
}

#[test]
fn read_only_commands_do_not_rewrite_the_database() {
    let e = Env::new();
    e.populate(&[]);
    let before = std::fs::read(e.db()).unwrap();
    e.run(&["query", "--all-people"]);
    e.run(&["trend", "--all-people"]);
    e.run(&["db", "info"]);
    assert!(std::fs::read(e.db()).unwrap() == before, "a read-only command re-sealed the database");
}

#[test]
fn missing_and_wrong_key_fail_cleanly() {
    let mut e = Env::new();
    e.populate(&[]);
    e.key = KEY_B.into();
    let err = e.fail(&["query", "--all-people"], 8);
    assert!(err.contains("wrong key"), "{err}");
    e.key = "a passphrase".into();
    assert!(e.fail(&["query"], 8).contains("wrong key"));
    e.key = String::new();
    let err = e.fail(&["query"], 8);
    assert!(err.contains("no key is available"), "{err}");
    let v: Value = serde_json::from_slice(&e.out(&["query", "--format", "json"]).stderr).unwrap();
    assert_eq!(v["error"]["kind"], "key");
}

#[test]
fn tampering_is_detected() {
    let e = Env::new();
    e.populate(&[]);
    let good = std::fs::read(e.db()).unwrap();
    // A flipped byte in the encrypted body.
    let mut bad = good.clone();
    let mid = 184 + (bad.len() - 184) / 2;
    bad[mid] ^= 0x01;
    std::fs::write(e.db(), &bad).unwrap();
    let err = e.fail(&["query"], 5);
    assert!(err.contains("tampered"), "{err}");
    // A flipped byte in the authenticated header (database id).
    let mut bad = good.clone();
    bad[44] ^= 0x01;
    std::fs::write(e.db(), &bad).unwrap();
    e.fail(&["query"], 8);
    // The trailing tag.
    let mut bad = good.clone();
    let n = bad.len();
    bad[n - 1] ^= 0x80;
    std::fs::write(e.db(), &bad).unwrap();
    assert!(e.fail(&["query"], 5).contains("tampered"));
    std::fs::write(e.db(), &good).unwrap();
    e.run(&["query"]);
    // Audit log tampering.
    let audit = e.path("data/labs.db.audit");
    let mut a = std::fs::read(&audit).unwrap();
    let n = a.len();
    a[n - 30] ^= 0x01;
    std::fs::write(&audit, &a).unwrap();
    let err = e.fail(&["audit", "log"], 5);
    assert!(err.contains("failed verification"), "{err}");
}

#[test]
fn rekey_to_passphrase_and_back_with_dek_rotation() {
    let mut e = Env::new();
    e.populate(&[]);
    let id_before = std::fs::read(e.db()).unwrap()[40..56].to_vec();
    // raw key -> env passphrase (Argon2id)
    let out = e.cmd().args(["db", "rekey"]).env("BIOMARKER_NEW_KEY", "correct horse battery staple").output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    e.fail(&["query"], 8);
    e.key = "correct horse battery staple".into();
    assert_eq!(e.json(&["query", "--all-people"])["count"], 2);
    let doctor = e.run(&["doctor"]);
    assert!(doctor.contains("passphrase-argon2id"), "{doctor}");
    // passphrase -> raw key, rotating the data key
    let out = e.cmd().args(["db", "rekey", "--rotate-dek"]).env("BIOMARKER_NEW_KEY", KEY_B).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    e.fail(&["query"], 8);
    e.key = KEY_B.into();
    assert_eq!(e.json(&["query", "--all-people"])["count"], 2);
    assert_eq!(std::fs::read(e.db()).unwrap()[40..56], id_before[..], "database id survives rekey");
    // The audit log written under the old keys is still readable.
    let log = e.json(&["audit", "log"]);
    let cmds: Vec<&str> = log["data"].as_array().unwrap().iter().map(|r| r["command"].as_str().unwrap()).collect();
    assert!(cmds.contains(&"person add") && cmds.contains(&"db rekey"), "{cmds:?}");
    assert_no_plaintext(&e.path("data"));
}

#[test]
fn migrate_plaintext_database_in_place() {
    let e = Env::new();
    let out = e.out(&["--insecure-plaintext", "db", "init"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("UNENCRYPTED"));
    e.populate(&["--insecure-plaintext"]);
    assert!(contains(&std::fs::read(e.db()).unwrap(), "SQLite format 3"));
    // Without the flag the plaintext database is refused.
    let err = e.fail(&["query"], 8);
    assert!(err.contains("db encrypt"), "{err}");
    let v = e.json(&["db", "encrypt"]);
    assert_eq!(v["data"]["verified_rows"]["measurements"], 2);
    assert_no_plaintext(&e.path("data"));
    assert!(!e.path("data/labs.db.plaintext-wipe").exists());
    assert_eq!(e.json(&["query", "--all-people"])["count"], 2);
    // Already encrypted.
    e.fail(&["db", "encrypt"], 4);
}

#[test]
fn plaintext_output_warns_and_encrypt_output_is_age() {
    let e = Env::new();
    e.populate(&[]);
    let plain = e.path("in/export.csv");
    let out = e.out(&["export", "--all-people", "-o", plain.to_str().unwrap()]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("plaintext health data"));
    let (identity, recipient) = age_identity();
    let enc = e.path("in/export.age");
    let out = e.out(&["export", "--all-people", "--recipient", &recipient, "-o", enc.to_str().unwrap()]);
    assert!(out.status.success());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("plaintext"));
    let armored = std::fs::read(&enc).unwrap();
    assert!(armored.starts_with(b"-----BEGIN AGE ENCRYPTED FILE-----"));
    assert!(!contains(&armored, MARK_LAB));
    let decrypted = age_decrypt(&armored, &identity);
    assert!(contains(&decrypted, MARK_LAB));
    assert_eq!(decrypted, std::fs::read(&plain).unwrap());
    // Passphrase mode (no recipient) without a passphrase source is a usage error.
    e.fail(&["export", "--encrypt-output"], 2);
}

#[test]
fn audit_log_records_commands_without_values() {
    let e = Env::new();
    e.populate(&[]);
    e.run(&["query", "--all-people"]);
    e.fail(&["person", "show", "nobody"], 3);
    let v = e.json(&["audit", "log"]);
    let rows = v["data"].as_array().unwrap();
    let cmds: Vec<&str> = rows.iter().map(|r| r["command"].as_str().unwrap()).collect();
    assert_eq!(&cmds[..], &["person add", "add", "import", "query", "person show"][..]);
    let q = &rows[3];
    assert_eq!(q["rows_out"], 2);
    assert_eq!(rows[4]["ok"], false);
    assert_eq!(rows[2]["rows_changed"].as_u64().map(|n| n >= 1), Some(true));
    let text = serde_json::to_string(&v).unwrap();
    for m in [MARK_NAME, MARK_NOTE, MARK_LAB, "alex", "nobody"] {
        assert!(!text.contains(m), "audit log leaks {m}");
    }
    assert_eq!(e.json(&["audit", "log", "-n", "1"])["data"][0]["command"], "audit log");
}

#[test]
fn backup_is_encrypted_with_same_key() {
    let e = Env::new();
    e.populate(&[]);
    let b = e.path("data/backup.db");
    e.run(&["db", "backup", b.to_str().unwrap()]);
    let v = e.json(&["--db", b.to_str().unwrap(), "query", "--all-people"]);
    assert_eq!(v["count"], 2);
}

#[test]
fn doctor_reports_encryption_state() {
    let e = Env::new();
    e.populate(&[]);
    let v = e.json(&["doctor"]);
    let rows = v["data"].as_array().unwrap();
    let status = |c: &str| rows.iter().find(|r| r["check"] == c).map(|r| r["status"].as_str().unwrap().to_string());
    assert_eq!(status("database").as_deref(), Some("ok"));
    assert_eq!(status("unlock").as_deref(), Some("ok"));
    assert_eq!(status("audit_log").as_deref(), Some("ok"));
    assert_eq!(status("sidecars").as_deref(), Some("ok"));
    assert_eq!(status("keychain").as_deref(), Some("warn"));
}

#[test]
fn db_lock_unlock_need_keychain() {
    let e = Env::new();
    e.populate(&[]);
    let err = e.fail(&["db", "unlock"], 8);
    assert!(err.contains("keychain"), "{err}");
    e.run(&["db", "lock"]);
}

/// Performance smoke test: open + query and commit latency on a few thousand
/// measurements (debug build; release numbers are in docs/security.md).
#[test]
fn performance_smoke() {
    let e = Env::new();
    e.run(&["person", "add", "p"]);
    let mut csv = String::from("person,marker,value,unit,date\n");
    let markers = ["ldl", "hdl", "glucose", "triglycerides", "hba1c"];
    for d in 0..1000 {
        let date = chrono_like(d);
        for (i, m) in markers.iter().enumerate() {
            csv.push_str(&format!(
                "p,{m},{},{},{date}\n",
                50 + (d + i) % 50,
                if *m == "hba1c" { "%" } else { "mg/dL" }
            ));
        }
    }
    let f = e.path("in/big.csv");
    std::fs::write(&f, csv).unwrap();
    let t = Instant::now();
    e.run(&["import", f.to_str().unwrap()]);
    let import = t.elapsed();
    let t = Instant::now();
    let v = e.json(&["latest", "-p", "p"]);
    let open_query = t.elapsed();
    let t = Instant::now();
    e.run(&["add", "ldl", "99", "-p", "p", "-d", "2030-01-01"]);
    let commit = t.elapsed();
    let size = std::fs::metadata(e.db()).unwrap().len();
    eprintln!(
        "perf: 5000 rows, {size} bytes sealed; import {import:?}, open+latest {open_query:?}, add (commit) {commit:?}"
    );
    assert_eq!(v["count"], 5);
    assert!(open_query.as_secs() < 30 && commit.as_secs() < 30);
}

fn chrono_like(d: usize) -> String {
    let (y, rest) = (2000 + d / 336, d % 336);
    format!("{y}-{:02}-{:02}", rest / 28 + 1, rest % 28 + 1)
}

fn age_identity() -> (age::x25519::Identity, String) {
    use age::secrecy::ExposeSecret;
    let id = age::x25519::Identity::generate();
    let r = id.to_public().to_string();
    let _ = id.to_string().expose_secret().len();
    (id, r)
}

fn age_decrypt(armored: &[u8], id: &age::x25519::Identity) -> Vec<u8> {
    use std::io::Read;
    let d = age::Decryptor::new(age::armor::ArmoredReader::new(armored)).unwrap();
    let mut r = d.decrypt(std::iter::once(id as &dyn age::Identity)).unwrap();
    let mut out = Vec::new();
    r.read_to_end(&mut out).unwrap();
    out
}
