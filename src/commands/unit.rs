use serde_json::json;

use crate::cli::{AddConversionArgs, UnitCmd};
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::output::{to_record, Report};
use crate::store::{self, Catalog};
use crate::units::Conversion;

pub fn run(ctx: &Ctx, cmd: UnitCmd) -> Result<()> {
    match cmd {
        UnitCmd::List { marker, symbols } => list(ctx, marker.as_deref(), symbols),
        UnitCmd::AddConversion(a) => add_conversion(ctx, a),
        UnitCmd::Convert { value, from, to, marker } => convert(ctx, value, &from, &to, marker.as_deref()),
    }
}

fn list(ctx: &Ctx, marker: Option<&str>, symbols: bool) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    if symbols {
        let rows = cat.units.iter().map(|(s, sys)| to_record(&json!({"unit": s, "system": sys}))).collect();
        return ctx.emit(&Report::list("units", rows));
    }
    let marker_id = marker.map(|m| cat.get(m).map(|m| m.id)).transpose()?;
    let rows = cat
        .conversions
        .conversions
        .iter()
        .filter(|c| marker_id.is_none_or(|id| c.marker_id == id || c.marker_id == 0))
        .map(|c| {
            to_record(&json!({
                "id": c.id,
                "marker": cat.by_id(c.marker_id).map(|m| m.slug.as_str()),
                "from": c.from_unit,
                "to": c.to_unit,
                "factor": c.factor,
                "offset": c.offset,
            }))
        })
        .collect();
    ctx.emit(&Report::list("conversions", rows))
}

fn add_conversion(ctx: &Ctx, a: AddConversionArgs) -> Result<()> {
    if a.factor == 0.0 || !a.factor.is_finite() {
        return Err(AppError::usage("--factor must be a non-zero number"));
    }
    if !matches!(a.system.as_str(), "us" | "si" | "both") {
        return Err(AppError::usage("--system must be us, si or both"));
    }
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let marker = a.marker.as_deref().map(|m| cat.get(m)).transpose()?;
    let c = Conversion {
        id: 0,
        marker_id: marker.map_or(0, |m| m.id),
        from_unit: cat.spell_unit(&a.from),
        to_unit: cat.spell_unit(&a.to),
        factor: a.factor,
        offset: a.offset,
    };
    db.transaction(|db| {
        store::ensure_unit(db, &c.from_unit, &a.system)?;
        store::ensure_unit(db, &c.to_unit, &a.system)?;
        store::upsert_conversion(db, &c)
    })?;
    ctx.info(&format!(
        "added conversion {} -> {} (x{}{}){}",
        c.from_unit,
        c.to_unit,
        c.factor,
        if c.offset == 0.0 { String::new() } else { format!(" + {}", c.offset) },
        marker.map(|m| format!(" for {}", m.slug)).unwrap_or_default()
    ));
    ctx.emit_mutation(&Report::object(
        "conversion",
        to_record(&json!({
            "marker": marker.map(|m| m.slug.as_str()),
            "from": c.from_unit, "to": c.to_unit, "factor": c.factor, "offset": c.offset,
        })),
    ))
}

fn convert(ctx: &Ctx, value: f64, from: &str, to: &str, marker: Option<&str>) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let m = marker.map(|m| cat.get(m)).transpose()?;
    let out = cat.conversions.convert(m.map_or(0, |m| m.id), value, from, to)?;
    ctx.emit(&Report::object(
        "conversion_result",
        to_record(&json!({
            "marker": m.map(|m| m.slug.as_str()),
            "value": value, "from": cat.spell_unit(from), "result": out, "to": cat.spell_unit(to),
        })),
    ))
}
