//! Embed catalog marker and panel TOML files at build time.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=catalog");
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let mut out = String::new();
    for (folder, constant) in [("markers", "MARKER_FILES"), ("panels", "PANEL_FILES")] {
        let dir = Path::new(&root).join("catalog").join(folder);
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        files.sort();
        out.push_str(&format!("pub const {constant}: &[(&str, &str)] = &[\n"));
        for f in &files {
            let name = f.file_name().unwrap_or_default().to_string_lossy();
            out.push_str(&format!("    ({name:?}, include_str!({:?})),\n", f.display()));
        }
        out.push_str("];\n");
    }
    let dest = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("catalog_files.rs");
    std::fs::write(dest, out).expect("writing catalog_files.rs");
}
