use serde_json::json;

use crate::catalog::{builtin, PanelRole};
use crate::cli_panel::PanelCmd;
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::output::{to_record, Report};
use crate::store::{self, Catalog, Filter};
use crate::view;

pub fn run(ctx: &Ctx, cmd: PanelCmd) -> Result<()> {
    let defs = builtin()?;
    match cmd {
        PanelCmd::List => {
            let rows = defs
                .panels
                .iter()
                .map(|p| {
                    to_record(&json!({
                        "slug": p.slug, "name": p.name,
                        "core": p.members.iter().filter(|m| m.role == PanelRole::Core).count(),
                        "extended": p.members.iter().filter(|m| m.role == PanelRole::Extended).count()
                    }))
                })
                .collect();
            ctx.emit(&Report::list("panels", rows).table_columns(&["slug", "name", "core", "extended"]))
        }
        PanelCmd::Show { panel } => {
            let p = defs
                .panels
                .iter()
                .find(|p| {
                    p.slug.eq_ignore_ascii_case(&panel) || p.aliases.iter().any(|a| a.eq_ignore_ascii_case(&panel))
                })
                .ok_or_else(|| AppError::not_found(format!("unknown panel '{panel}'")))?;
            let mut members: Vec<serde_json::Value> = Vec::new();
            let db_ready = ctx.db_path.exists();
            let (cat, latest) = if db_ready {
                let db = ctx.db()?;
                let cat = Catalog::load(&db)?;
                let person = if let Some(s) = ctx.default_person() { store::find_person(&db, &s)? } else { None };
                let rows = if let Some(person) = person {
                    let ids: Vec<i64> = p.members.iter().filter_map(|m| cat.find(&m.marker).map(|x| x.id)).collect();
                    store::query_measurements(
                        &db,
                        &Filter { person_ids: vec![person.id], marker_ids: ids, ..Filter::default() },
                    )?
                } else {
                    Vec::new()
                };
                (Some(cat), rows)
            } else {
                (None, Vec::new())
            };
            for m in &p.members {
                let marker = cat.as_ref().and_then(|c| c.find(&m.marker));
                let latest_row = marker.and_then(|marker| latest.iter().rev().find(|r| r.marker_id == marker.id));
                let evaluated =
                    latest_row.and_then(|r| view::evaluate(r, cat.as_ref()?, ctx.resolved.get("unit_system")));
                let mut row = json!({
                    "marker": m.marker, "name": marker.map(|x| x.name.as_str()).unwrap_or(&m.marker),
                    "unit": marker.map(|x| x.unit.as_str()), "role": if m.role == PanelRole::Core {"core"} else {"extended"},
                    "reason": m.reason, "term": m.term,
                    "latest": evaluated.as_ref().map(|e| e.display_value),
                    "latest_unit": evaluated.as_ref().map(|e| e.display_unit.as_str()),
                    "taken_at": evaluated.as_ref().map(|e| e.row.taken_at.as_str()),
                    "person": evaluated.as_ref().map(|e| e.row.person.as_str()),
                    "range_flag": evaluated.as_ref().and_then(|e| e.ref_flag).map(|f| format!("{f:?}").to_lowercase())
                });
                if let Some(obj) = row.as_object_mut() {
                    obj.insert("citations".into(), json!(m.citations));
                }
                members.push(row);
            }
            let record =
                to_record(&json!({"slug": p.slug, "name": p.name, "description": p.description, "members": members}));
            ctx.emit(&Report::object("panel", record))
        }
    }
}
