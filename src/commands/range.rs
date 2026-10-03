use serde_json::json;

use crate::cli::{KindArg, RangeCmd, RangeSetArgs, RangeSex};
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::output::{to_record, Record, Report};
use crate::ranges::{Range, RangeKind};
use crate::store::{self, Catalog};

pub fn run(ctx: &Ctx, cmd: RangeCmd) -> Result<()> {
    match cmd {
        RangeCmd::Set(a) => set(ctx, a),
        RangeCmd::List { marker, kind } => list(ctx, marker.as_deref(), kind),
        RangeCmd::Rm { id } => rm(ctx, id),
    }
}

fn kind_of(k: KindArg) -> RangeKind {
    match k {
        KindArg::Reference => RangeKind::Reference,
        KindArg::Optimal => RangeKind::Optimal,
    }
}

fn range_record(cat: &Catalog, r: &Range) -> Record {
    let m = cat.by_id(r.marker_id);
    to_record(&json!({
        "id": r.id,
        "marker": m.map(|m| m.slug.as_str()),
        "kind": r.kind.as_str(),
        "sex": r.sex,
        "age_min": r.age_min,
        "age_max": r.age_max,
        "low": r.low,
        "high": r.high,
        "unit": m.map(|m| m.unit.as_str()),
        "note": r.note,
    }))
}

fn set(ctx: &Ctx, a: RangeSetArgs) -> Result<()> {
    if a.low.is_none() && a.high.is_none() {
        return Err(AppError::usage("give at least one of --low / --high"));
    }
    if a.age_min >= a.age_max {
        return Err(AppError::usage("--age-min must be below --age-max"));
    }
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let m = cat.get(&a.marker)?;
    let unit = a.unit.clone().unwrap_or_else(|| m.unit.clone());
    let conv = |v: Option<f64>| v.map(|x| cat.to_canonical(m, x, &unit)).transpose();
    let (low, high) = (conv(a.low)?, conv(a.high)?);
    if let (Some(l), Some(h)) = (low, high) {
        if l > h {
            return Err(AppError::invalid(format!("low ({l}) is above high ({h})")));
        }
    }
    let sex = match a.sex {
        RangeSex::Any => "any",
        RangeSex::Male => "male",
        RangeSex::Female => "female",
    };
    let r = Range {
        id: 0,
        marker_id: m.id,
        kind: kind_of(a.kind),
        sex: sex.into(),
        age_min: a.age_min,
        age_max: a.age_max,
        low,
        high,
        note: a.note,
    };
    store::upsert_range(&db, &r)?;
    let cat = Catalog::load(&db)?;
    let saved = cat
        .ranges
        .iter()
        .find(|x| {
            x.marker_id == r.marker_id
                && x.kind == r.kind
                && x.sex == r.sex
                && x.age_min == r.age_min
                && x.age_max == r.age_max
        })
        .ok_or_else(|| AppError::db("range not saved"))?;
    ctx.info(&format!("set {} range for {}", r.kind.as_str(), m.slug));
    ctx.emit_mutation(&Report::object("range", range_record(&cat, saved)))
}

fn list(ctx: &Ctx, marker: Option<&str>, kind: Option<KindArg>) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let marker_id = marker.map(|m| cat.get(m).map(|m| m.id)).transpose()?;
    let rows = cat
        .ranges
        .iter()
        .filter(|r| marker_id.is_none_or(|id| r.marker_id == id))
        .filter(|r| kind.is_none_or(|k| r.kind == kind_of(k)))
        .map(|r| range_record(&cat, r))
        .collect();
    ctx.emit(&Report::list("ranges", rows))
}

fn rm(ctx: &Ctx, id: i64) -> Result<()> {
    let db = ctx.db()?;
    match store::delete_range(&db, id)? {
        0 => Err(AppError::not_found(format!("no range with id {id}"))),
        _ => {
            ctx.info(&format!("removed range {id}"));
            ctx.emit_mutation(&Report::object("removed", to_record(&json!({"range": id}))))
        }
    }
}
