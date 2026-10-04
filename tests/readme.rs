//! Golden check for the README sample session: `docs/readme-session.txt` (and
//! the copy embedded in README.md) must match what the binary prints now.
//! Regenerate both with `just readme` (scripts/readme-samples.sh).

#![cfg(unix)]

use std::path::Path;
use std::process::Command;

const BEGIN: &str = "<!-- readme-session:begin -->\n```console\n";
const END: &str = "```\n<!-- readme-session:end -->";

fn repo_file(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn readme_session_matches_binary() {
    let bin = Path::new(env!("CARGO_BIN_EXE_biomarker"));
    let path = format!("{}:{}", bin.parent().unwrap().display(), std::env::var("PATH").unwrap_or_default());
    let out = Command::new("sh")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/readme-session.sh"))
        .arg("full")
        .env("PATH", path)
        .env("BIOMARKER_COLOR", "never")
        .output()
        .unwrap();
    assert!(out.status.success(), "readme-session.sh failed:\n{}", String::from_utf8_lossy(&out.stderr));
    let actual = String::from_utf8(out.stdout).unwrap();
    let expected = repo_file("docs/readme-session.txt");
    assert!(
        actual == expected,
        "docs/readme-session.txt is stale; run `just readme`.\n--- expected\n{expected}\n--- actual\n{actual}"
    );
}

#[test]
fn readme_embeds_session() {
    let readme = repo_file("README.md");
    let start = readme.find(BEGIN).expect("README.md lacks the readme-session:begin marker") + BEGIN.len();
    let len = readme[start..].find(END).expect("README.md lacks the readme-session:end marker");
    assert!(
        readme[start..start + len] == repo_file("docs/readme-session.txt"),
        "README.md session block differs from docs/readme-session.txt; run `just readme`"
    );
}
