use serde_json::json;

use crate::cli::{MarkerAddArgs, MarkerCmd, MarkerEditArgs};
use crate::context::Ctx;
use crate::db::{int, opt_real, real, Db, RowExt};
use crate::error::{AppError, Result};
use crate::output::{to_record, Record, Report};
use crate::profiles;
use crate::store::{self, Catalog, Marker};
use crate::units::{canonical_spelling, same_unit, Conversion};
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
        MarkerCmd::Sync { file, dry_run } => sync(ctx, file.as_deref(), dry_run),
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

/// Create or update catalog markers from a markers.toml (default: next to the
/// config file). Idempotent; an existing marker keeps its unit.
fn sync(ctx: &Ctx, file: Option<&std::path::Path>, dry_run: bool) -> Result<()> {
    let path = file.map_or_else(|| profiles::default_marker_file(&ctx.resolved.config_path), Into::into);
    let spec = profiles::load_marker_file(&path)?;
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let known_units = cat.unit_symbols();
    let mut actions: Vec<(String, &'static str)> = Vec::new();
    for s in &spec.markers {
        let slug = validate_slug(&s.slug)?;
        let at = |e: AppError| e.context(format!("{} ({slug})", path.display()));
        if s.ranges.iter().any(|r| r.critical_low.is_some() || r.critical_high.is_some()) {
            return Err(at(AppError::config(
                "critical bounds are not stored in the catalog; put them in a range set or person profile",
            )));
        }
        let action = match cat.markers.iter().find(|m| m.slug == slug) {
            Some(old) => {
                let unit = s.unit.as_deref().map(|u| canonical_spelling(u, &known_units));
                if unit.as_ref().is_some_and(|u| !same_unit(u, &old.unit)) {
                    return Err(at(AppError::invalid(format!(
                        "unit is {} in the database; an existing marker's unit cannot change",
                        old.unit
                    ))));
                }
                let new = Marker {
                    name: s.name.clone().unwrap_or_else(|| old.name.clone()),
                    category: s.category.as_ref().map_or_else(|| old.category.clone(), |c| c.to_lowercase()),
                    loinc: s.loinc.clone().or_else(|| old.loinc.clone()),
                    description: s.description.clone().or_else(|| old.description.clone()),
                    ..old.clone()
                };
                let new_aliases = s.aliases.iter().any(|a| !old.aliases.contains(&a.trim().to_lowercase()));
                let changed = new != *old || new_aliases;
                if changed && !dry_run {
                    store::update_marker(&db, &new)?;
                    s.aliases.iter().try_for_each(|a| store::add_alias(&db, old.id, a)).map_err(at)?;
                }
                if changed {
                    "updated"
                } else {
                    "unchanged"
                }
            }
            None => {
                let unit = s.unit.as_deref().ok_or_else(|| at(AppError::invalid("a new marker needs a unit")))?;
                let m = Marker {
                    id: 0,
                    name: s.name.clone().unwrap_or_else(|| slug.clone()),
                    slug: slug.clone(),
                    category: s.category.as_deref().unwrap_or("other").to_lowercase(),
                    unit: canonical_spelling(unit, &known_units),
                    loinc: s.loinc.clone(),
                    description: s.description.clone(),
                    builtin: false,
                    aliases: s.aliases.clone(),
                };
                if !dry_run {
                    store::insert_marker(&db, &m).map_err(at)?;
                    store::ensure_unit(&db, &m.unit, "both")?;
                }
                "created"
            }
        };
        actions.push((slug, action));
    }
    if !dry_run {
        let cat = Catalog::load(&db)?;
        for s in &spec.markers {
            let m = cat.get(&s.slug)?.clone();
            let at = |e: AppError| e.context(format!("{} ({})", path.display(), m.slug));
            for c in &s.conversions {
                if c.factor == 0.0 || !c.factor.is_finite() {
                    return Err(at(AppError::invalid("conversion factor must be a non-zero number")));
                }
                let conv = Conversion {
                    id: 0,
                    marker_id: m.id,
                    from_unit: cat.spell_unit(&c.from),
                    to_unit: c.to.as_deref().map_or_else(|| m.unit.clone(), |t| cat.spell_unit(t)),
                    factor: c.factor,
                    offset: c.offset,
                };
                store::ensure_unit(&db, &conv.from_unit, "both")?;
                store::upsert_conversion(&db, &conv)?;
            }
            let cat = Catalog::load(&db)?;
            for r in &s.ranges {
                let one = profiles::RangeSpec { marker: m.slug.clone(), ..r.clone() };
                store::upsert_range(&db, &profiles::range_from_spec(&cat, &one).map_err(at)?)?;
            }
        }
    }
    let rows =
        actions.iter().map(|(slug, a)| to_record(&json!({"marker": slug, "action": a, "dry_run": dry_run}))).collect();
    ctx.info(&format!(
        "{} {} marker(s) from {}",
        if dry_run { "checked" } else { "synced" },
        actions.len(),
        path.display()
    ));
    ctx.emit(&Report::list("marker_sync", rows).table_columns(&["marker", "action"]))
}
