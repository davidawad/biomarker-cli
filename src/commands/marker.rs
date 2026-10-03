use serde_json::json;

use crate::cli::{MarkerAddArgs, MarkerCmd, MarkerEditArgs};
use crate::context::Ctx;
use crate::db::{int, opt_real, real, Db, RowExt};
use crate::error::{AppError, Result};
use crate::output::{to_record, Record, Report};
use crate::store::{self, Catalog, Marker};
use crate::util::validate_slug;

pub fn run(ctx: &Ctx, cmd: MarkerCmd) -> Result<()> {
    match cmd {
        MarkerCmd::Add(a) => add(ctx, a),
        MarkerCmd::List { category, search } => list(ctx, category.as_deref(), search.as_deref()),
        MarkerCmd::Show { marker } => show(ctx, &marker),
        MarkerCmd::Edit(a) => edit(ctx, a),
        MarkerCmd::Rm { marker, force } => rm(ctx, &marker, force),
        MarkerCmd::Alias { marker, aliases, remove } => alias(ctx, &marker, &aliases, remove),
        MarkerCmd::Categories => categories(ctx),
    }
}

pub fn marker_record(m: &Marker) -> Record {
    to_record(m)
}

fn add(ctx: &Ctx, a: MarkerAddArgs) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let slug = validate_slug(&a.slug)?;
    if let Some(existing) = cat.find(&slug) {
        return Err(AppError::invalid(format!("marker '{slug}' already exists (as {})", existing.slug)));
    }
    let m = Marker {
        id: 0,
        name: a.name.unwrap_or_else(|| slug.clone()),
        slug,
        category: a.category.to_lowercase(),
        unit: cat.spell_unit(&a.unit),
        loinc: a.loinc,
        description: a.description,
        builtin: false,
        aliases: a.aliases,
    };
    store::insert_marker(&db, &m)?;
    store::ensure_unit(&db, &m.unit, "both")?;
    let cat = Catalog::load(&db)?;
    ctx.info(&format!("added marker {}", m.slug));
    ctx.emit_mutation(&Report::object("marker", marker_record(cat.get(&m.slug)?)))
}

fn list(ctx: &Ctx, category: Option<&str>, search: Option<&str>) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let needle = search.map(str::to_lowercase);
    let rows = cat
        .markers
        .iter()
        .filter(|m| category.is_none_or(|c| m.category.eq_ignore_ascii_case(c)))
        .filter(|m| {
            needle.as_deref().is_none_or(|n| {
                m.slug.contains(n) || m.name.to_lowercase().contains(n) || m.aliases.iter().any(|a| a.contains(n))
            })
        })
        .map(marker_record)
        .collect();
    ctx.emit(&Report::list("markers", rows).table_columns(&["slug", "name", "category", "unit", "loinc"]))
}

fn show(ctx: &Ctx, name: &str) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let m = cat.get(name)?;
    let mut rec = marker_record(m);
    let ranges: Vec<_> = cat.ranges_for(m.id).map(to_record).collect();
    let conversions: Vec<_> = cat
        .conversions
        .conversions
        .iter()
        .filter(|c| c.marker_id == m.id)
        .map(|c| json!({"from": c.from_unit, "to": c.to_unit, "factor": c.factor, "offset": c.offset}))
        .collect();
    let units = cat.conversions.reachable(m.id, &m.unit).iter().map(|u| cat.spell_unit(u)).collect::<Vec<_>>();
    rec.insert("measurements".into(), json!(store::count_measurements_for_marker(&db, m.id)?));
    rec.insert("convertible_units".into(), json!(units));
    rec.insert("conversions".into(), json!(conversions));
    rec.insert("ranges".into(), json!(ranges));
    ctx.emit(&Report::object("marker", rec))
}

/// Re-express stored canonical values and ranges in a new canonical unit.
fn rescale(db: &Db, cat: &Catalog, m: &Marker, new_unit: &str) -> Result<()> {
    let conv = |v: f64| cat.conversions.convert(m.id, v, &m.unit, new_unit);
    conv(1.0).map_err(|e| e.context("cannot change canonical unit"))?;
    db.query("SELECT id, value FROM measurements WHERE marker_id = ?1", &[int(m.id)])?.iter().try_for_each(|r| {
        let v = conv(r.f(1).unwrap_or_default())?;
        db.execute("UPDATE measurements SET value = ?1 WHERE id = ?2", &[real(v), int(r.i(0).unwrap_or_default())])
            .map(|_| ())
    })?;
    cat.ranges_for(m.id).try_for_each(|r| {
        let lo = r.low.map(conv).transpose()?;
        let hi = r.high.map(conv).transpose()?;
        db.execute("UPDATE ranges SET low = ?1, high = ?2 WHERE id = ?3", &[opt_real(lo), opt_real(hi), int(r.id)])
            .map(|_| ())
    })
}

fn edit(ctx: &Ctx, a: MarkerEditArgs) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let m = cat.get(&a.marker)?.clone();
    let new_unit = a.unit.as_deref().map(|u| cat.spell_unit(u));
    let updated = Marker {
        slug: a.rename.as_deref().map(validate_slug).transpose()?.unwrap_or(m.slug.clone()),
        name: a.name.unwrap_or(m.name.clone()),
        category: a.category.map(|c| c.to_lowercase()).unwrap_or(m.category.clone()),
        unit: new_unit.clone().unwrap_or(m.unit.clone()),
        loinc: a.loinc.or(m.loinc.clone()),
        description: a.description.or(m.description.clone()),
        ..m.clone()
    };
    db.transaction(|db| {
        if let Some(u) = new_unit.as_deref().filter(|u| !crate::units::same_unit(u, &m.unit)) {
            rescale(db, &cat, &m, u)?;
        }
        store::update_marker(db, &updated)
    })?;
    let cat = Catalog::load(&db)?;
    ctx.info(&format!("updated marker {}", updated.slug));
    ctx.emit_mutation(&Report::object("marker", marker_record(cat.get(&updated.slug)?)))
}

fn rm(ctx: &Ctx, name: &str, force: bool) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let m = cat.get(name)?;
    let n = store::count_measurements_for_marker(&db, m.id)?;
    if n > 0 && !force {
        return Err(AppError::invalid(format!(
            "marker '{}' has {n} measurements; use --force to delete them too",
            m.slug
        )));
    }
    let deleted = store::delete_marker(&db, m.id)?;
    ctx.info(&format!("removed marker {} ({deleted} measurements)", m.slug));
    ctx.emit_mutation(&Report::object(
        "removed",
        to_record(&json!({"marker": m.slug, "measurements_deleted": deleted})),
    ))
}

fn alias(ctx: &Ctx, name: &str, aliases: &[String], remove: bool) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let m = cat.get(name)?;
    db.transaction(|db| {
        aliases.iter().try_for_each(|a| {
            if remove {
                store::remove_alias(db, m.id, a).map(|_| ())
            } else if cat.find(a).is_some_and(|other| other.id != m.id) {
                Err(AppError::invalid(format!("'{a}' already refers to another marker")))
            } else {
                store::add_alias(db, m.id, a)
            }
        })
    })?;
    let cat = Catalog::load(&db)?;
    let m = cat.get(&m.slug)?;
    ctx.emit_mutation(&Report::object("aliases", to_record(&json!({"marker": m.slug, "aliases": m.aliases}))))
}

fn categories(ctx: &Ctx) -> Result<()> {
    let db = ctx.db()?;
    let rows = db
        .query("SELECT category, count(*) FROM markers GROUP BY category ORDER BY category", &[])?
        .iter()
        .map(|r| to_record(&json!({"category": r.s(0), "markers": r.i(1)})))
        .collect();
    ctx.emit(&Report::list("categories", rows))
}
