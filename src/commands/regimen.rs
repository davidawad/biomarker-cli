use chrono::NaiveDate;
use serde_json::json;

use crate::cli::{FilterArgs, QueryArgs, RegimenCmd, SortKey};
use crate::cli_interventions::{RegimenAddArgs, RegimenImpactArgs, RegimenLsArgs, StatusArg};
use crate::commands::{measure, person_slug};
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::interventions::impact;
use crate::interventions::regimen::{self, NewRegimen, Regimen, Status};
use crate::interventions::IvCatalog;
use crate::output::{to_record, Record, Report};
use crate::store;
use crate::util::{parse_date, parse_duration_days};

pub fn run(ctx: &Ctx, cmd: RegimenCmd) -> Result<()> {
    match cmd {
        RegimenCmd::Add(a) => add(ctx, a),
        RegimenCmd::Stop { target, person, on } => stop(ctx, &target, person.as_deref(), on.as_deref()),
        RegimenCmd::Ls(a) => list(ctx, a),
        RegimenCmd::Rm { ids } => rm(ctx, &ids),
        RegimenCmd::Impact(a) => impact(ctx, a),
    }
}

const TABLE: &[&str] =
    &["id", "person", "intervention", "dose", "unit", "frequency", "time_slot", "started_on", "stopped_on", "status"];
const DATES: &[&str] = &["started_on", "stopped_on", "as_of"];

/// A regimen with its status and days taken as of `as_of`.
pub fn record(r: &Regimen, as_of: NaiveDate) -> Record {
    let day = as_of.to_string();
    let mut rec = to_record(r);
    rec.insert("status".into(), json!(r.status_on(&day).as_str()));
    rec.insert("as_of".into(), json!(day));
    rec.insert("days".into(), json!(r.days_taken(as_of)));
    rec
}

fn day(ctx: &Ctx, d: Option<&str>) -> Result<NaiveDate> {
    d.map_or_else(|| Ok(ctx.tz.today()), |d| parse_date(d, &ctx.tz))
}

fn clean(s: Option<String>) -> Option<String> {
    s.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

fn add(ctx: &Ctx, a: RegimenAddArgs) -> Result<()> {
    let db = ctx.db()?;
    let person = store::get_person(&db, &person_slug(ctx, a.person.as_deref())?)?;
    let ivs = IvCatalog::load(&db)?;
    let item = ivs.get(&a.intervention)?;
    let start = day(ctx, a.start.as_deref())?;
    let new = NewRegimen {
        person_id: person.id,
        intervention_id: item.id,
        dose: a.dose,
        unit: clean(a.unit),
        frequency: clean(a.frequency).map(|s| s.to_lowercase()),
        time_slot: clean(a.slot).map(|s| s.to_lowercase()),
        started_on: start.to_string(),
        stopped_on: a.stop.as_deref().map(|d| parse_date(d, &ctx.tz)).transpose()?.map(|d| d.to_string()),
        note: clean(a.note),
    };
    let existing = regimen::list(&db, &[person.id], Some(item.id))?;
    let overlap = existing.iter().find(|r| {
        r.stopped_on.as_deref().is_none_or(|s| s > new.started_on.as_str())
            && new.stopped_on.as_deref().is_none_or(|s| s > r.started_on.as_str())
    });
    if let Some(o) = overlap {
        ctx.warn(&format!(
            "warning: overlaps regimen {} ({} from {}); adding anyway",
            o.id, o.intervention, o.started_on
        ));
    }
    let id = regimen::insert(&db, &new)?;
    let saved = regimen::get(&db, id)?;
    ctx.info(&format!("{} takes {} from {}", person.slug, item.slug, saved.started_on));
    ctx.emit_mutation(&Report::object("regimen", record(&saved, ctx.tz.today())).dates(DATES))
}

fn stop(ctx: &Ctx, target: &str, person: Option<&str>, on: Option<&str>) -> Result<()> {
    let db = ctx.db()?;
    let on = day(ctx, on)?;
    let on_s = on.to_string();
    let targets: Vec<Regimen> = match target.parse::<i64>() {
        Ok(id) => vec![regimen::get(&db, id)?],
        Err(_) => {
            let person = store::get_person(&db, &person_slug(ctx, person)?)?;
            let item = IvCatalog::load(&db)?.get(target)?.clone();
            let active: Vec<Regimen> = regimen::list(&db, &[person.id], Some(item.id))?
                .into_iter()
                .filter(|r| r.started_on <= on_s && r.stopped_on.as_deref().is_none_or(|s| s > on_s.as_str()))
                .collect();
            if active.is_empty() {
                return Err(AppError::not_found(format!(
                    "{} has no active regimen of {} on {on_s}",
                    person.slug, item.slug
                )));
            }
            active
        }
    };
    let stopped = db.transaction(|db| {
        targets
            .iter()
            .map(|r| regimen::stop(db, r, &on_s).and_then(|()| regimen::get(db, r.id)))
            .collect::<Result<Vec<_>>>()
    })?;
    ctx.info(&format!("stopped {} regimen(s) on {on_s}", stopped.len()));
    let rows = stopped.iter().map(|r| record(r, on)).collect();
    ctx.emit_mutation(&Report::list("regimens", rows).table_columns(TABLE).dates(DATES))
}

fn list(ctx: &Ctx, a: RegimenLsArgs) -> Result<()> {
    let db = ctx.db()?;
    let persons: Vec<String> = match (a.persons.is_empty(), a.all_people, ctx.default_person()) {
        (false, _, _) => a.persons.clone(),
        (true, false, Some(d)) => vec![d],
        _ => Vec::new(),
    };
    let ids = persons.iter().map(|p| store::get_person(&db, p).map(|p| p.id)).collect::<Result<Vec<_>>>()?;
    let only = a.intervention.as_deref().map(|i| IvCatalog::load(&db)?.get(i).map(|i| i.id)).transpose()?;
    let as_of = day(ctx, a.as_of.as_deref())?;
    let want = match a.status {
        StatusArg::Taking => Some(Status::Taking),
        StatusArg::Stopped => Some(Status::Stopped),
        StatusArg::Planned => Some(Status::Planned),
        StatusArg::All => None,
    };
    let rows = regimen::list(&db, &ids, only)?
        .iter()
        .filter(|r| want.is_none_or(|w| r.status_on(&as_of.to_string()) == w))
        .map(|r| record(r, as_of))
        .collect();
    ctx.emit(&Report::list("regimens", rows).table_columns(TABLE).dates(DATES).meta("as_of", json!(as_of.to_string())))
}

fn rm(ctx: &Ctx, ids: &[i64]) -> Result<()> {
    let db = ctx.db()?;
    let removed = db.transaction(|db| {
        ids.iter()
            .map(|id| match regimen::delete(db, *id)? {
                0 => Err(AppError::not_found(format!("no regimen with id {id}"))),
                _ => Ok(*id),
            })
            .collect::<Result<Vec<_>>>()
    })?;
    ctx.info(&format!("removed {} regimen(s)", removed.len()));
    ctx.emit_mutation(&Report::object("removed", to_record(&json!({"regimens": removed}))))
}

fn impact(ctx: &Ctx, a: RegimenImpactArgs) -> Result<()> {
    let window = parse_duration_days(&a.window)?.round() as i64;
    if window <= 0 {
        return Err(AppError::usage("--window must be positive"));
    }
    let person = person_slug(ctx, a.person.as_deref())?;
    let (regs, item) = {
        let db = ctx.db()?;
        let p = store::get_person(&db, &person)?;
        let item = IvCatalog::load(&db)?.get(&a.intervention)?.clone();
        (regimen::list(&db, &[p.id], Some(item.id))?, item)
    };
    let filter = FilterArgs {
        persons: vec![person.clone()],
        markers: a.markers.clone(),
        categories: a.categories.clone(),
        ..FilterArgs::default()
    };
    let q = QueryArgs {
        filter,
        flagged: false,
        latest: false,
        sort: SortKey::Date,
        reverse: false,
        limit: None,
        with_regimens: false,
    };
    let (_, rows) = measure::select(ctx, &q)?;
    let mut series: Vec<(i64, Vec<&crate::view::Evaluated>)> = Vec::new();
    rows.iter().for_each(|e| match series.iter_mut().find(|s| s.0 == e.row.marker_id) {
        Some(s) => s.1.push(e),
        None => series.push((e.row.marker_id, vec![e])),
    });
    series.sort_by(|x, y| {
        (&x.1[0].marker.category, &x.1[0].marker.slug).cmp(&(&y.1[0].marker.category, &y.1[0].marker.slug))
    });
    let events: Vec<(&Regimen, &str, &str)> = regs
        .iter()
        .flat_map(|r| {
            std::iter::once((r, "start", r.started_on.as_str())).chain(r.stopped_on.as_deref().map(|s| (r, "stop", s)))
        })
        .collect();
    let mut out: Vec<Record> = Vec::new();
    for (r, event, date) in &events {
        let Ok(ev) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else { continue };
        for (_, g) in &series {
            let pts: Vec<(String, f64)> = g.iter().map(|e| (e.row.taken_at.clone(), e.display_value)).collect();
            let s = impact::sides(&pts, ev, window);
            if s.is_empty() {
                continue;
            }
            let change = s.change();
            out.push(to_record(&json!({
                "person": person,
                "intervention": item.slug,
                "regimen_id": r.id,
                "event": event,
                "event_date": date,
                "window_days": window,
                "marker": g[0].marker.slug,
                "unit": g[0].display_unit,
                "n_before": s.n_before,
                "before": s.before.as_ref().map(|b| b.1),
                "before_date": s.before.as_ref().map(|b| &b.0),
                "before_mean": s.before_mean,
                "n_after": s.n_after,
                "after": s.after.as_ref().map(|b| b.1),
                "after_date": s.after.as_ref().map(|b| &b.0),
                "after_mean": s.after_mean,
                "change": change,
                "change_pct": s.before.as_ref().zip(s.after.as_ref()).and_then(|(b, a)| crate::stats::pct_change(b.1, a.1)),
            })));
        }
    }
    if regs.is_empty() {
        ctx.info(&format!("{person} has no regimen of {}", item.slug));
    }
    ctx.emit(
        &Report::list("regimen_impact", out)
            .table_columns(&[
                "event",
                "event_date",
                "marker",
                "unit",
                "n_before",
                "before",
                "before_date",
                "n_after",
                "after",
                "after_date",
                "change",
                "change_pct",
            ])
            .dates(&["event_date", "before_date", "after_date"])
            .meta("person", json!(person))
            .meta("intervention", json!(item.slug))
            .meta("window_days", json!(window))
            .meta("unit_system", json!(ctx.unit_system())),
    )
}
