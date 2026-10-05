//! Command implementations.

pub mod config_cmd;
pub mod db_cmd;
pub mod import;
pub mod import_sheet;
pub mod key_cmd;
pub mod marker;
pub mod measure;
pub mod misc;
pub mod observations;
pub mod person;
pub mod range;
pub mod security;
pub mod trend;
pub mod unit;

use crate::cli::{Command, FilterArgs};
use crate::context::Ctx;
use crate::db::Db;
use crate::error::Result;
use crate::store::{self, Catalog, Filter};
use crate::util::{date_before, parse_date, parse_span};

/// Process exit status requested by a successful command (e.g. `flag --exit-code`).
pub type Status = i32;

pub fn dispatch(ctx: &Ctx, cmd: Command) -> Result<Status> {
    match cmd {
        Command::Person(c) => person::run(ctx, c).map(|()| 0),
        Command::Marker(c) => marker::run(ctx, c).map(|()| 0),
        Command::Range(c) => range::run(ctx, c).map(|()| 0),
        Command::Unit(c) => unit::run(ctx, c).map(|()| 0),
        Command::Add(a) => measure::add(ctx, a).map(|()| 0),
        Command::Rm(a) => measure::rm(ctx, a).map(|()| 0),
        Command::Import(a) => import::run(ctx, a).map(|()| 0),
        Command::Export(a) => measure::export(ctx, a).map(|()| 0),
        Command::Query(a) => measure::query(ctx, a).map(|()| 0),
        Command::Latest(mut a) => {
            a.latest = true;
            measure::query(ctx, a).map(|()| 0)
        }
        Command::Trend(a) => trend::trend(ctx, a).map(|()| 0),
        Command::Flag(a) => measure::flag(ctx, a),
        Command::Observations(a) => observations::list(ctx, a).map(|()| 0),
        Command::Diff(a) => trend::diff(ctx, a).map(|()| 0),
        Command::Db(c) => db_cmd::run(ctx, c).map(|()| 0),
        Command::Config(c) => config_cmd::run(ctx, c).map(|()| 0),
        Command::Audit(c) => security::audit(ctx, c).map(|()| 0),
        Command::Key(c) => key_cmd::run(ctx, c).map(|()| 0),
        Command::Doctor => security::doctor(ctx).map(|()| 0),
        Command::Completions(a) => misc::completions(a).map(|()| 0),
        Command::Man(a) => misc::man(a).map(|()| 0),
    }
}

/// Resolve the single person a command acts on (`--person` or default_person).
pub fn person_slug(ctx: &Ctx, explicit: Option<&str>) -> Result<String> {
    explicit.map(str::to_string).or_else(|| ctx.default_person()).ok_or_else(|| {
        crate::error::AppError::usage(
            "no person given: pass --person or set one with `biomarker config set default_person <slug>`",
        )
    })
}

/// Turn CLI filter flags into a store filter, resolving people and markers.
pub fn build_filter(ctx: &Ctx, db: &Db, cat: &Catalog, f: &FilterArgs) -> Result<Filter> {
    let persons: Vec<String> = match (f.persons.is_empty(), f.all_people, ctx.default_person()) {
        (false, _, _) => f.persons.clone(),
        (true, false, Some(d)) => vec![d],
        _ => Vec::new(),
    };
    let person_ids = persons.iter().map(|p| store::get_person(db, p).map(|p| p.id)).collect::<Result<Vec<_>>>()?;
    let marker_ids = f.markers.iter().map(|m| cat.get(m).map(|m| m.id)).collect::<Result<Vec<_>>>()?;
    let since_last = f.last.as_deref().map(parse_span).transpose()?.map(|s| date_before(ctx.tz.today(), s).to_string());
    let from = f
        .from
        .as_deref()
        .map(|d| parse_date(d, &ctx.tz).map(|d| d.to_string()))
        .transpose()?
        .into_iter()
        .chain(since_last)
        .max();
    let to = f.to.as_deref().map(|d| parse_date(d, &ctx.tz).map(|d| d.to_string())).transpose()?;
    Ok(Filter {
        person_ids,
        marker_ids,
        categories: f.categories.clone(),
        from,
        to,
        lab: f.lab.clone(),
        batch: f.batch.clone(),
        tag: f.tag.clone(),
    })
}
