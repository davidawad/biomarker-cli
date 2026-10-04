use serde_json::json;

use crate::cli::{AddArgs, ExportArgs, FlagArgs, QueryArgs, RmMeasurementArgs, SortKey};
use crate::commands::{build_filter, person_slug, Status};
use crate::context::Ctx;
use crate::error::{AppError, Result};
use crate::output::{to_record, Record, Report};
use crate::store::{self, Catalog, Dedupe, Marker, NewMeasurement};
use crate::util::{merge_tags, parse_value, parse_when, Tz};
use crate::view::{self, Evaluated, MEASUREMENT_TABLE};

/// Raw, textual description of a measurement (from flags or an import row).
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub marker: String,
    pub value: String,
    pub qualifier: Option<String>,
    pub unit: Option<String>,
    pub date: String,
    pub lab: Option<String>,
    pub fasting: Option<bool>,
    pub note: Option<String>,
    pub tags: Vec<String>,
}

/// Validate an input and compute the canonical value (pure apart from catalog lookup).
pub fn build(
    cat: &Catalog,
    marker: &Marker,
    person_id: i64,
    i: &Input,
    date_formats: &[String],
    tz: &Tz,
) -> Result<NewMeasurement> {
    let (prefix_q, value_raw) = parse_value(&i.value)?;
    let qualifier = match (&i.qualifier, prefix_q) {
        (Some(q), _) => crate::util::parse_qualifier(q)?,
        (None, q) => q,
    };
    let unit_raw =
        i.unit.as_deref().filter(|u| !u.trim().is_empty()).map_or_else(|| marker.unit.clone(), |u| cat.spell_unit(u));
    let value = cat.to_canonical(marker, value_raw, &unit_raw)?;
    let fmts: Vec<&str> = date_formats.iter().map(String::as_str).collect();
    Ok(NewMeasurement {
        person_id,
        marker_id: marker.id,
        taken_at: parse_when(&i.date, &fmts, tz)?,
        value_raw,
        unit_raw,
        value,
        qualifier,
        lab: i.lab.clone().filter(|s| !s.is_empty()),
        fasting: i.fasting,
        note: i.note.clone().filter(|s| !s.is_empty()),
        tags: merge_tags(&[], &i.tags, &[]),
        batch_id: None,
    })
}

pub fn add(ctx: &Ctx, a: AddArgs) -> Result<()> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let person = store::get_person(&db, &person_slug(ctx, a.person.as_deref())?)?;
    let marker = cat.get(&a.marker)?;
    let input = Input {
        marker: a.marker.clone(),
        value: a.value.clone(),
        qualifier: None,
        unit: a.unit.clone(),
        date: a.date.clone().unwrap_or_else(|| "today".into()),
        lab: a.lab.clone(),
        fasting: match (a.fasting, a.no_fasting) {
            (true, _) => Some(true),
            (_, true) => Some(false),
            _ => None,
        },
        note: a.note.clone(),
        tags: a.tags.clone(),
    };
    let m = build(&cat, marker, person.id, &input, &ctx.input_date_formats(), &ctx.tz)?;
    let (outcome, id) =
        store::insert_measurement(&db, &m, Dedupe::parse(a.dedupe.as_str())?).map_err(|e| e.context(&marker.slug))?;
    ctx.info(&format!(
        "{} {} = {} {} for {} at {}",
        format!("{outcome:?}").to_lowercase(),
        marker.slug,
        a.value,
        m.unit_raw,
        person.slug,
        m.taken_at
    ));
    let row = store::query_measurements(
        &db,
        &store::Filter { person_ids: vec![person.id], marker_ids: vec![marker.id], ..Default::default() },
    )?
    .into_iter()
    .find(|r| r.id == id)
    .ok_or_else(|| AppError::db("measurement not found after insert"))?;
    let ev = view::evaluate(&row, &cat, ctx.unit_system()).ok_or_else(|| AppError::db("marker vanished"))?;
    let mut rec = ev.record(&cat, ctx.range_flavor());
    rec.insert("outcome".into(), json!(outcome));
    ctx.emit_mutation(&Report::object("measurement", rec).dates(&["taken_at"]))
}

pub fn rm(ctx: &Ctx, a: RmMeasurementArgs) -> Result<()> {
    let db = ctx.db()?;
    let removed = db.transaction(|db| {
        a.ids
            .iter()
            .map(|id| match store::delete_measurement(db, *id)? {
                0 => Err(AppError::not_found(format!("no measurement with id {id}"))),
                _ => Ok(*id),
            })
            .collect::<Result<Vec<_>>>()
    })?;
    ctx.info(&format!("removed {} measurement(s)", removed.len()));
    ctx.emit_mutation(&Report::object("removed", to_record(&json!({"measurements": removed}))))
}

fn sort_rows(mut rows: Vec<Evaluated>, key: SortKey, reverse: bool) -> Vec<Evaluated> {
    rows.sort_by(|a, b| {
        let by_date = a.row.taken_at.cmp(&b.row.taken_at);
        match key {
            SortKey::Date => by_date,
            SortKey::Person => a.row.person.cmp(&b.row.person).then(by_date),
            SortKey::Marker => a.marker.slug.cmp(&b.marker.slug).then(by_date),
            SortKey::Category => {
                a.marker.category.cmp(&b.marker.category).then(a.marker.slug.cmp(&b.marker.slug)).then(by_date)
            }
            SortKey::Value => a.display_value.total_cmp(&b.display_value).then(by_date),
        }
    });
    if reverse {
        rows.reverse();
    }
    rows
}

/// Shared pipeline for query/latest/flag: fetch, evaluate, filter, sort, limit.
pub fn select(ctx: &Ctx, q: &QueryArgs) -> Result<(Catalog, Vec<Evaluated>)> {
    let db = ctx.db()?;
    let cat = Catalog::load(&db)?;
    let filter = build_filter(ctx, &db, &cat, &q.filter)?;
    ctx.verbose(&format!("filter: {filter:?}"));
    let rows = store::query_measurements(&db, &filter)?;
    let evaluated = view::evaluate_all(&rows, &cat, ctx.unit_system());
    let evaluated = if q.latest { view::latest_only(evaluated) } else { evaluated };
    let flavor = ctx.range_flavor();
    let evaluated = evaluated.into_iter().filter(|e| !q.flagged || e.is_flagged(flavor)).collect();
    let sorted = sort_rows(evaluated, q.sort, q.reverse);
    let limited = match q.limit {
        Some(n) => sorted.into_iter().take(n).collect(),
        None => sorted,
    };
    Ok((cat, limited))
}

fn records(ctx: &Ctx, cat: &Catalog, rows: &[Evaluated]) -> Vec<Record> {
    rows.iter().map(|e| e.record(cat, ctx.range_flavor())).collect()
}

pub fn query(ctx: &Ctx, q: QueryArgs) -> Result<()> {
    let (cat, rows) = select(ctx, &q)?;
    let kind = if q.latest { "latest" } else { "measurements" };
    ctx.emit(
        &Report::list(kind, records(ctx, &cat, &rows))
            .table_columns(MEASUREMENT_TABLE)
            .dates(&["taken_at"])
            .meta("unit_system", json!(ctx.unit_system()))
            .meta("range_flavor", json!(ctx.range_flavor())),
    )
}

pub fn flag(ctx: &Ctx, a: FlagArgs) -> Result<Status> {
    let q = QueryArgs {
        filter: a.filter,
        flagged: true,
        latest: a.latest,
        sort: SortKey::Date,
        reverse: false,
        limit: None,
    };
    let (cat, rows) = select(ctx, &q)?;
    let report = Report::list("flags", records(ctx, &cat, &rows))
        .table_columns(&[
            "id",
            "person",
            "taken_at",
            "marker",
            "qualifier",
            "value",
            "unit",
            "ref_low",
            "ref_high",
            "ref_flag",
            "opt_low",
            "opt_high",
            "opt_flag",
        ])
        .dates(&["taken_at"])
        .meta("range_flavor", json!(ctx.range_flavor()));
    ctx.emit(&report)?;
    if rows.is_empty() {
        ctx.info("no flagged values");
    }
    Ok(if a.exit_code && !rows.is_empty() { 10 } else { 0 })
}

/// Export record: exactly the fields `import` understands, raw values, ISO dates.
fn export_record(e: &Evaluated, with_ids: bool) -> Record {
    let r = &e.row;
    let mut rec = Record::new();
    if with_ids {
        rec.insert("id".into(), json!(r.id));
    }
    rec.insert("person".into(), json!(r.person));
    rec.insert("marker".into(), json!(e.marker.slug));
    rec.insert("date".into(), json!(r.taken_at));
    rec.insert("qualifier".into(), json!(r.qualifier));
    rec.insert("value".into(), json!(r.value_raw));
    rec.insert("unit".into(), json!(r.unit_raw));
    rec.insert("lab".into(), json!(r.lab));
    rec.insert("fasting".into(), json!(r.fasting));
    rec.insert("note".into(), json!(r.note));
    rec.insert("tags".into(), json!(r.tags));
    if with_ids {
        rec.insert("batch".into(), json!(r.batch_id));
    }
    rec
}

pub fn export(ctx: &Ctx, a: ExportArgs) -> Result<()> {
    let q =
        QueryArgs { filter: a.filter, flagged: false, latest: false, sort: SortKey::Date, reverse: false, limit: None };
    let (_, rows) = select(ctx, &q)?;
    let mut out = ctx.out.clone();
    if out.format == crate::output::Format::Table {
        out.format = crate::output::Format::Csv;
    }
    // Missing values are always empty cells so exports re-import cleanly.
    out.null = String::new();
    let report = Report::list("export", rows.iter().map(|e| export_record(e, a.with_ids)).collect()).exact();
    if !ctx.quiet() && out.output.is_some() {
        eprintln!("exported {} measurement(s)", rows.len());
    }
    ctx.emit_with(&report, &out)
}
