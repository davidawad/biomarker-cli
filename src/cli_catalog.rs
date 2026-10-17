//! `biomarker catalog`: the built-in marker catalog (catalog/ in the source tree).

use std::path::PathBuf;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum CatalogCmd {
    /// Check the catalog: schema, citations, units, sets, conditions, marker
    /// references and ambiguous ranges; print the coverage matrix. Exits 4 on errors.
    Lint {
        /// JSON output (same as --format json)
        #[arg(long)]
        json: bool,
        /// Treat warnings (uncited legacy ranges, coverage gaps) as errors
        #[arg(long)]
        strict: bool,
        /// Also report coverage against catalog/universe.toml
        #[arg(long)]
        universe: bool,
        /// Lint a catalog directory (markers/*.toml, sets.toml, ...) instead of the built-in one
        #[arg(long, value_name = "DIR")]
        dir: Option<PathBuf>,
    },
    /// Write Markdown docs: one page per marker, index.md and coverage.md
    Docs {
        /// Output directory (stale *.md files in it are removed)
        #[arg(long, value_name = "DIR", default_value = "docs/markers")]
        out: PathBuf,
        /// Generate from a catalog directory instead of the built-in catalog
        #[arg(long, value_name = "DIR")]
        dir: Option<PathBuf>,
        /// Write nothing; exit 4 if the docs in --out are stale
        #[arg(long)]
        check: bool,
    },
    /// Print the JSON Schema of a catalog marker file
    Schema,
    /// List the catalog's range sets
    Sets,
    /// List the named conditions usable in `when.conditions` and person attributes
    Conditions,
}
