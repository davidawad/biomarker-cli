use serde_json::json;

use crate::cli::ProfileCmd;
use crate::context::Ctx;
use crate::error::Result;
use crate::output::{to_record, Report};
use crate::profiles;
use crate::ranges::RangeKind;
use crate::store;
use crate::util::{age_years, date_of};
use crate::view::pick_range;

pub fn run(ctx: &Ctx, cmd: ProfileCmd) -> Result<()> {
    match cmd {
        ProfileCmd::List => list(ctx),
        ProfileCmd::Init { person, unit_preset, range_set } => {
            init(ctx, &person, unit_preset.as_deref(), range_set.as_deref())
        }
        ProfileCmd::Show { person, marker } => show(ctx, &person, marker.as_deref()),
    }
}

fn list(ctx: &Ctx) -> Result<()> {
    let p = profiles::Profiles::load(&ctx.resolved.config_path, &ctx.profile_opts())?;
    let rows = p
        .entries()
        .into_iter()
        .map(|e| {
            to_record(&json!({
                "kind": e.kind,
                "name": e.name,
                "extends": e.extends,
                "source": e.origin.as_ref().map_or_else(|| "built-in".to_string(), |o| o.display().to_string()),
                "description": e.description,
            }))
        })
        .collect();
    ctx.emit(&Report::list("profiles", rows).table_columns(&["kind", "name", "extends", "source", "description"]))
}

fn init(ctx: &Ctx, person: &str, unit_preset: Option<&str>, range_set: Option<&str>) -> Result<()> {
    let path = profiles::init_person(&ctx.resolved.config_path, person, unit_preset, range_set)?;
    ctx.info(&format!("wrote {}", path.display()));
    ctx.emit_mutation(&Report::object("profile_init", to_record(&json!({"person": person, "path": path}))))
}

/// The units and ranges in effect for one person, with where each range came from.
fn show(ctx: &Ctx, person: &str, marker: Option<&str>) -> Result<()> {
    let db = ctx.db()?;
    let cat = ctx.catalog(&db)?;
    let who = store::get_person(&db, person)?;
    let age = who.dob.as_deref().and_then(|d| date_of(d).ok()).map(|dob| age_years(dob, ctx.tz.today()));
    let wanted = marker.map(|m| cat.get(m)).transpose()?.map(|m| m.id);
    let rows = cat
        .markers
        .iter()
        .filter(|m| wanted.is_none_or(|id| id == m.id))
        .flat_map(|m| {
            let unit = cat.display_unit(m, ctx.unit_system(), Some(&who.slug));
            let (cat, who) = (&cat, &who);
            [RangeKind::Reference, RangeKind::Optimal, RangeKind::Warn].into_iter().filter_map(move |kind| {
                let (r, source) = pick_range(cat, &who.slug, who.id, m, kind, who.sex.as_deref(), age, None)?;
                let shown = |v: Option<f64>| v.map(|x| cat.conversions.convert(m.id, x, &m.unit, &unit).unwrap_or(x));
                Some(to_record(&json!({
                    "marker": m.slug,
                    "unit": unit,
                    "kind": kind.as_str(),
                    "low": shown(r.low),
                    "high": shown(r.high),
                    "source": source,
                })))
            })
        })
        .collect();
    ctx.emit(
        &Report::list("profile", rows)
            .table_columns(&["marker", "unit", "kind", "low", "high", "source"])
            .meta("person", json!(who.slug))
            .meta("unit_preset", json!(cat.profiles.preset_name(Some(&who.slug), ctx.unit_system())))
            .meta("range_set", json!(cat.profiles.range_set_for(&who.slug))),
    )
}
