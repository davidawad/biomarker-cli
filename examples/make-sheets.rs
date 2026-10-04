//! Regenerate `examples/dashboard.xlsx` (synthetic data):
//! `cargo run --example make-sheets`.

#[path = "../tests/support/sheets.rs"]
mod sheets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/dashboard.xlsx");
    sheets::dashboard(&out)?;
    println!("wrote {}", out.display());
    Ok(())
}
