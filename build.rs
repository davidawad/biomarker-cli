//! Embed catalog/markers/*.toml: writes `$OUT_DIR/catalog_files.rs` with one
//! `(file name, contents)` entry per marker file, sorted by name.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=catalog");
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let dir = Path::new(&root).join("catalog").join("markers");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("catalog/markers")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    let mut out = String::from("pub const MARKER_FILES: &[(&str, &str)] = &[\n");
    for f in &files {
        let name = f.file_name().unwrap_or_default().to_string_lossy();
        out.push_str(&format!("    ({name:?}, include_str!({:?})),\n", f.display().to_string()));
    }
    out.push_str("];\n");
    let dest = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("catalog_files.rs");
    std::fs::write(dest, out).expect("writing catalog_files.rs");
}
