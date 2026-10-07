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
        RangeCmd::List { marker, kind, person } => list(ctx, marker.as_deref(), kind, person.as_deref()),
        RangeCmd::Rm { id, personal } => rm(ctx, id, personal),
    }
}

fn kind_of(k: KindArg) -> RangeKind {
    match k {
        KindArg::Reference => RangeKind::Reference,
        KindArg::Optimal => RangeKind::Optimal,
    }
}

fn range_record(cat: &Catalog, r: &Range, person: Option<&str>) -> Record {
    let m = cat.by_id(r.marker_id);
    to_record(&json!({
        "id": r.id,
        "person": person,
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
    if let Some(p) = &a.person {
        let person = store::get_person(&db, p)?;
        let r = Range {
            id: 0,
            marker_id: m.id,
            kind: kind_of(a.kind),
            sex: "any".into(),
            age_min: 0.0,
            age_max: 200.0,
            low,
            high,
            note: a.note,
            person_id: Some(person.id),
            critical_low: None,
            critical_high: None,
        };
        store::upsert_person_range(&db, person.id, &r)?;
        let cat = Catalog::load(&db)?;
        let saved = cat
            .person_ranges
            .iter()
            .find(|x| x.person_id == r.person_id && x.marker_id == r.marker_id && x.kind == r.kind)
            .ok_or_else(|| AppError::db("range not saved"))?;
        ctx.info(&format!("set {} range for {} ({})", r.kind.as_str(), m.slug, person.slug));
        return ctx.emit_mutation(&Report::object("range", range_record(&cat, saved, Some(&person.slug))));
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
        person_id: None,
        critical_low: None,
        critical_high: None,
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
    ctx.emit_mutation(&Report::object("range", range_record(&cat, saved, None)))
}

fn list(ctx: &Ctx, marker: Option<&str>, kind: Option<KindArg>, person: Option<&str>) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let marker_id = marker.map(|m| cat.get(m).map(|m| m.id)).transpose()?;
    let person = person.map(|p| store::get_person(&db, p)).transpose()?;
    let people = store::list_people(&db)?;
    let slug = |id: Option<i64>| id.and_then(|id| people.iter().find(|p| p.id == id)).map(|p| p.slug.as_str());
    let rows = cat
        .person_ranges
        .iter()
        .filter(|r| person.as_ref().is_none_or(|p| r.person_id == Some(p.id)))
        .chain(cat.ranges.iter())
        .filter(|r| marker_id.is_none_or(|id| r.marker_id == id))
        .filter(|r| kind.is_none_or(|k| r.kind == kind_of(k)))
        .map(|r| range_record(&cat, r, slug(r.person_id)))
        .collect();
    ctx.emit(&Report::list("ranges", rows))
}

fn rm(ctx: &Ctx, id: i64, personal: bool) -> Result<()> {
    let db = ctx.db()?;
    let deleted = if personal { store::delete_person_range(&db, id)? } else { store::delete_range(&db, id)? };
    match deleted {
        0 => Err(AppError::not_found(format!("no range with id {id}"))),
        _ => {
            ctx.info(&format!("removed range {id}"));
            ctx.emit_mutation(&Report::object("removed", to_record(&json!({"range": id}))))
        }
    }
}
