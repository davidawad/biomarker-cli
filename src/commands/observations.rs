//! `biomarker observations`: qualitative results stored beside the numeric model.

use serde_json::json;

use crate::cli::ObservationsArgs;
use crate::commands::build_filter;
use crate::context::Ctx;
use crate::db::Db;
use crate::error::Result;
use crate::output::{to_record, Record, Report};
use crate::store::{self, Catalog, Filter};

pub const OBSERVATION_TABLE: &[&str] = &["id", "person", "taken_at", "marker", "text", "flag", "note"];

/// Observation records for a filter (also used by `export --format json`).
pub fn records(db: &Db, filter: &Filter, flagged: bool) -> Result<Vec<Record>> {
    Ok(store::query_observations(db, filter)?.iter().filter(|o| !flagged || o.flag.is_some()).map(to_record).collect())
}

pub fn list(ctx: &Ctx, a: ObservationsArgs) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let filter = build_filter(ctx, &db, &cat, &a.filter)?;
    let rows = records(&db, &filter, a.flagged)?;
    if rows.is_empty() {
        ctx.info("no observations");
    }
    ctx.emit(
        &Report::list("observations", rows)
            .table_columns(OBSERVATION_TABLE)
            .dates(&["taken_at"])
            .meta("flagged_only", json!(a.flagged)),
    )
}
