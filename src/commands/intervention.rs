use serde_json::json;

use crate::cli::InterventionCmd;
use crate::cli_interventions::InterventionAddArgs;
use crate::commands::interaction::edge_record;
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::interventions::{self as iv, interaction, regimen, Ingredient, Intervention, IvCatalog};
use crate::output::{to_record, Record, Report};
use crate::store::Catalog;
use crate::util::{slugify, validate_slug};

pub mod import;

pub fn run(ctx: &Ctx, cmd: InterventionCmd) -> Result<()> {
    match cmd {
        InterventionCmd::Add(a) => add(ctx, a),
        InterventionCmd::Ls { category, search } => list(ctx, category.as_deref(), search.as_deref()),
        InterventionCmd::Show { intervention } => show(ctx, &intervention),
        InterventionCmd::Rm { intervention, force } => rm(ctx, &intervention, force),
        InterventionCmd::Import(a) => import::run(ctx, a),
    }
}

pub const TABLE: &[&str] = &["slug", "name", "category", "rx_status", "classes", "aliases"];

fn add(ctx: &Ctx, a: InterventionAddArgs) -> Result<()> {
    let db = ctx.db()?;
    let slug = validate_slug(&a.slug)?;
    let item = Intervention {
        name: a.name.unwrap_or_else(|| slug.clone()),
        category: a.category.trim().to_lowercase(),
        rx_status: a.rx_status.as_str().to_string(),
        ingredients: a.ingredients.iter().map(|i| Ingredient::parse(i)).collect::<Result<_>>()?,
        classes: a.classes.iter().map(|c| slugify(c)).filter(|c| !c.is_empty()).collect(),
        aliases: a.aliases.iter().map(|a| a.trim().to_lowercase()).filter(|a| !a.is_empty()).collect(),
        description: a.description,
        ..Intervention::new(&slug)
    };
    let id = iv::insert(&db, &item)?;
    let saved = IvCatalog::load(&db)?.by_id(id).cloned().ok_or_else(|| AppError::db("intervention vanished"))?;
    ctx.info(&format!("added intervention {}", saved.slug));
    ctx.emit_mutation(&Report::object("intervention", to_record(&saved)))
}

fn list(ctx: &Ctx, category: Option<&str>, search: Option<&str>) -> Result<()> {
    let db = ctx.db()?;
    let ivs = IvCatalog::load(&db)?;
    let needle = search.map(str::to_lowercase);
    let rows: Vec<Record> = ivs
        .items
        .iter()
        .filter(|i| category.is_none_or(|c| i.category.eq_ignore_ascii_case(c)))
        .filter(|i| {
            needle.as_deref().is_none_or(|n| {
                i.slug.contains(n)
                    || i.name.to_lowercase().contains(n)
                    || i.aliases.iter().any(|a| a.contains(n))
                    || i.ingredients.iter().any(|g| g.name.contains(n))
            })
        })
        .map(to_record)
        .collect();
    ctx.emit(&Report::list("interventions", rows).table_columns(TABLE))
}

fn show(ctx: &Ctx, name: &str) -> Result<()> {
    let db = ctx.db()?;
    let ivs = IvCatalog::load(&db)?;
    let markers = Catalog::load(&db)?;
    let item = ivs.get(name)?;
    let edges: Vec<_> = interaction::list(&db)?
        .iter()
        .filter(|e| interaction::touches(e, item, &ivs))
        .map(|e| edge_record(e, &ivs, &markers))
        .collect();
    let today = ctx.tz.today().to_string();
    let regs = regimen::list(&db, &[], Some(item.id))?;
    let mut rec = to_record(item);
    rec.insert("regimens".into(), json!(regs.len()));
    rec.insert("active_regimens".into(), json!(regs.iter().filter(|r| r.active_on(&today)).count()));
    rec.insert("interactions".into(), json!(edges));
    ctx.emit(&Report::object("intervention", rec))
}

fn rm(ctx: &Ctx, name: &str, force: bool) -> Result<()> {
    let db = ctx.db()?;
    let ivs = IvCatalog::load(&db)?;
    let item = ivs.get(name)?;
    let n = iv::count_regimens(&db, item.id)?;
    if n > 0 && !force {
        return Err(AppError::invalid(format!(
            "intervention '{}' has {n} regimen(s); use --force to delete them too",
            item.slug
        )));
    }
    let deleted = iv::delete(&db, item)?;
    ctx.info(&format!("removed intervention {} ({deleted} regimens)", item.slug));
    ctx.emit_mutation(&Report::object(
        "removed",
        to_record(&json!({"intervention": item.slug, "regimens_deleted": deleted})),
    ))
}
