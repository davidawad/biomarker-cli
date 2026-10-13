use serde_json::{json, Value};

use crate::cli::InteractionCmd;
use crate::cli_interventions::{InteractionAddArgs, InteractionLsArgs};
use crate::commands::person_slug;
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::interventions::interaction::{self, Edge, NewEdge};
use crate::interventions::{regimen, Intervention, IvCatalog};
use crate::output::{to_record, Format, Record, Report};
use crate::store::{self, Catalog};
use crate::util::parse_date;

pub fn run(ctx: &Ctx, cmd: InteractionCmd) -> Result<()> {
    match cmd {
        InteractionCmd::Add(a) => add(ctx, a),
        InteractionCmd::Ls(a) => list(ctx, a),
        InteractionCmd::Grid { person, as_of, catalog } => grid(ctx, person.as_deref(), as_of.as_deref(), catalog),
        InteractionCmd::Rm { ids } => rm(ctx, &ids),
    }
}

const TABLE: &[&str] = &["id", "a", "target_kind", "target", "kind", "severity", "mechanism"];

/// The JSON form of an edge, with its target resolved against the catalogs.
pub fn edge_record(e: &Edge, ivs: &IvCatalog, markers: &Catalog) -> Record {
    let mut rec = to_record(e);
    let name = interaction::target_name(e, ivs, markers);
    let resolvable = matches!(e.target_kind.as_str(), "intervention" | "lab-test");
    rec.insert("a_name".into(), json!(ivs.find(&e.a).map(|i| i.name.clone())));
    rec.insert("target_name".into(), json!(name));
    rec.insert("resolved".into(), if resolvable { json!(name.is_some()) } else { Value::Null });
    rec
}

fn add(ctx: &Ctx, a: InteractionAddArgs) -> Result<()> {
    let db = ctx.db()?;
    let ivs = IvCatalog::load(&db)?;
    let subject = ivs.get(&a.intervention)?;
    let (target_kind, target) = interaction::parse_target(&a.target)?;
    let edge = NewEdge {
        a_id: subject.id,
        target_kind,
        target,
        kind: interaction::parse_kind(&a.kind)?,
        severity: interaction::parse_severity(&a.severity)?,
        mechanism: a.mechanism,
        citations: a.citations.into_iter().map(Value::String).collect(),
        note: a.note,
        source: None,
    };
    let (outcome, id) = interaction::upsert(&db, &edge)?;
    let saved =
        interaction::list(&db)?.into_iter().find(|e| e.id == id).ok_or_else(|| AppError::db("edge vanished"))?;
    ctx.info(&format!(
        "{} interaction {}: {} -> {}:{}",
        format!("{outcome:?}").to_lowercase(),
        saved.kind,
        saved.a,
        saved.target_kind,
        saved.target
    ));
    let mut rec = edge_record(&saved, &ivs, &Catalog::load(&db)?);
    rec.insert("outcome".into(), json!(outcome));
    ctx.emit_mutation(&Report::object("interaction", rec))
}

fn list(ctx: &Ctx, a: InteractionLsArgs) -> Result<()> {
    let db = ctx.db()?;
    let ivs = IvCatalog::load(&db)?;
    let markers = Catalog::load(&db)?;
    let only = a.intervention.as_deref().map(|i| ivs.get(i)).transpose()?;
    let kinds = a.kinds.iter().map(|k| interaction::parse_kind(k)).collect::<Result<Vec<_>>>()?;
    let min = a.min_severity.as_deref().map(interaction::parse_severity).transpose()?;
    let target_kind = a.target_kind.as_deref().map(interaction::parse_target_kind).transpose()?;
    let mut edges: Vec<Edge> = interaction::list(&db)?
        .into_iter()
        .filter(|e| only.is_none_or(|i| interaction::touches(e, i, &ivs)))
        .filter(|e| kinds.is_empty() || kinds.contains(&e.kind))
        .filter(|e| {
            min.as_deref().is_none_or(|m| interaction::severity_rank(&e.severity) >= interaction::severity_rank(m))
        })
        .filter(|e| target_kind.as_ref().is_none_or(|t| &e.target_kind == t))
        .collect();
    edges.sort_by(|x, y| {
        x.a.cmp(&y.a).then(interaction::kind_rank(&y.kind).cmp(&interaction::kind_rank(&x.kind))).then(x.id.cmp(&y.id))
    });
    let rows = edges.iter().map(|e| edge_record(e, &ivs, &markers)).collect();
    ctx.emit(&Report::list("interactions", rows).table_columns(TABLE))
}

fn rm(ctx: &Ctx, ids: &[i64]) -> Result<()> {
    let db = ctx.db()?;
    let removed = db.transaction(|db| {
        ids.iter()
            .map(|id| match interaction::delete(db, *id)? {
                0 => Err(AppError::not_found(format!("no interaction with id {id}"))),
                _ => Ok(*id),
            })
            .collect::<Result<Vec<_>>>()
    })?;
    ctx.info(&format!("removed {} interaction(s)", removed.len()));
    ctx.emit_mutation(&Report::object("removed", to_record(&json!({"interactions": removed}))))
}

fn grid(ctx: &Ctx, person: Option<&str>, as_of: Option<&str>, with_catalog: bool) -> Result<()> {
    let db = ctx.db()?;
    let person = store::get_person(&db, &person_slug(ctx, person)?)?;
    let day = as_of.map_or_else(|| Ok(ctx.tz.today()), |d| parse_date(d, &ctx.tz))?.to_string();
    let ivs = IvCatalog::load(&db)?;
    let edges = interaction::list(&db)?;
    let taking = regimen::taking_on(&regimen::list(&db, &[person.id], None)?, person.id, &day);
    let rows: Vec<&Intervention> = taking.iter().filter_map(|s| ivs.items.iter().find(|i| &i.slug == s)).collect();
    let columns: Vec<&Intervention> = if with_catalog {
        let mut c: Vec<&Intervention> = ivs
            .items
            .iter()
            .filter(|c| rows.iter().any(|r| r.id == c.id || interaction::cell(&edges, r, c, &ivs).is_some()))
            .collect();
        c.sort_by_key(|c| (!rows.iter().any(|r| r.id == c.id), c.slug.clone()));
        c
    } else {
        rows.clone()
    };
    let cells: Vec<interaction::Cell> =
        rows.iter().flat_map(|r| columns.iter().filter_map(|c| interaction::cell(&edges, r, c, &ivs))).collect();
    let other: Vec<Value> = rows
        .iter()
        .flat_map(|r| interaction::others(&edges, r, &columns))
        .map(|e| {
            json!({"intervention": e.a, "target_kind": e.target_kind, "target": e.target, "kind": e.kind,
                   "severity": e.severity, "mechanism": e.mechanism, "id": e.id})
        })
        .collect();
    let table: Vec<Record> = rows
        .iter()
        .map(|r| {
            let mut rec = Record::new();
            rec.insert("intervention".into(), json!(r.slug));
            columns.iter().for_each(|c| {
                let k = cells.iter().find(|x| x.row == r.slug && x.column == c.slug).map(|x| x.kind.clone());
                rec.insert(c.slug.clone(), json!(k));
            });
            rec
        })
        .collect();
    let data = json!({
        "person": person.slug,
        "as_of": day,
        "rows": rows.iter().map(|r| &r.slug).collect::<Vec<_>>(),
        "columns": columns.iter().map(|c| &c.slug).collect::<Vec<_>>(),
        "cells": cells,
        "other": other,
    });
    if rows.is_empty() {
        ctx.info(&format!("{} is taking nothing on {day}", person.slug));
    }
    ctx.emit(
        &Report::list("interaction_grid", table)
            .json_data(data)
            .meta("person", json!(person.slug))
            .meta("as_of", json!(day)),
    )?;
    if ctx.out.format == Format::Table && ctx.out.output.is_none() && !other.is_empty() {
        crate::output::write_stdout(b"\n")?;
        let recs = other.iter().map(to_record).collect();
        let cols = ["intervention", "target_kind", "target", "kind", "severity", "mechanism"];
        ctx.emit(&Report::list("interaction_other", recs).table_columns(&cols))?;
    }
    Ok(())
}
