use serde_json::{json, Value};

use crate::cli::{DiffArgs, FilterArgs, QueryArgs, SortKey, TrendArgs};
use crate::commands::measure::select;
use crate::commands::person_slug;
use crate::context::Ctx;
use crate::error::Result;
use crate::output::{to_record, Record, Report};
use crate::ranges::Flag;
use crate::stats::{self, Point};
use crate::store::Catalog;
use crate::util::{date_before, date_of, day_number, parse_date, parse_span};
use crate::view::Evaluated;

/// (window name, days spanned, (change, percent change) if the series reaches back far enough).
type WindowResult = (String, i64, Option<(f64, f64)>);

/// Group evaluated rows (sorted by date) into per (person, marker) series.
fn series(rows: Vec<Evaluated>) -> Vec<Vec<Evaluated>> {
    let mut groups: Vec<Vec<Evaluated>> = Vec::new();
    rows.into_iter().for_each(|e| {
        match groups.iter_mut().find(|g| g[0].row.person_id == e.row.person_id && g[0].row.marker_id == e.row.marker_id)
        {
            Some(g) => g.push(e),
            None => groups.push(vec![e]),
        }
    });
    groups.sort_by(|a, b| {
        (&a[0].row.person, &a[0].marker.category, &a[0].marker.slug).cmp(&(
            &b[0].row.person,
            &b[0].marker.category,
            &b[0].marker.slug,
        ))
    });
    groups
}

fn query_all(filter: FilterArgs) -> QueryArgs {
    QueryArgs { filter, flagged: false, latest: false, sort: SortKey::Date, reverse: false, limit: None }
}

fn flag_str(f: Option<Flag>) -> Option<&'static str> {
    f.map(Flag::as_str)
}

pub fn trend(ctx: &Ctx, a: TrendArgs) -> Result<()> {
    let windows = a
        .windows
        .clone()
        .unwrap_or_else(|| ctx.trend_windows())
        .iter()
        .filter(|w| !w.trim().is_empty())
        .map(|w| parse_span(w).map(|s| (w.trim().to_string(), s)))
        .collect::<Result<Vec<_>>>()?;
    let min_points = a.min_points.unwrap_or_else(|| ctx.trend_min_points());
    let (cat, rows) = select(ctx, &query_all(a.filter))?;
    let flavor = ctx.range_flavor();
    let groups: Vec<Vec<Evaluated>> = series(rows).into_iter().filter(|g| g.len() >= min_points).collect();

    let build = |g: &[Evaluated]| -> Option<(Record, Value)> {
        let pts: Vec<Point> =
            g.iter().filter_map(|e| day_number(&e.row.taken_at).map(|d| (d, e.display_value))).collect();
        let s = stats::summarize(&pts)?;
        let (first, last) = (&g[0], &g[g.len() - 1]);
        let rec = last.record(&cat, flavor);
        let last_date = date_of(&last.row.taken_at).ok()?;
        // (name, days spanned, change)
        let win: Vec<WindowResult> = windows
            .iter()
            .map(|(name, span)| {
                let cutoff = date_before(last_date, *span);
                let cutoff_day = day_number(&cutoff.to_string()).unwrap_or(f64::NEG_INFINITY);
                // datetimes on the cutoff day count as "at or before" it
                (name.clone(), (last_date - cutoff).num_days(), stats::window_change(&pts, cutoff_day + 0.999_99))
            })
            .collect();
        let mut flat = to_record(&json!({
            "person": last.row.person,
            "marker": last.marker.slug,
            "marker_name": last.marker.name,
            "category": last.marker.category,
            "unit": last.display_unit,
            "n": s.n,
            "first_date": first.row.taken_at,
            "last_date": last.row.taken_at,
            "min": s.min,
            "max": s.max,
            "mean": s.mean,
            "median": s.median,
            "stddev": s.stddev,
            "first": s.first,
            "last": s.last,
            "change": s.change,
            "change_pct": s.change_pct,
            "slope_per_year": s.slope_per_year,
            "last_flag": flag_str(last.flag(flavor)),
        }));
        win.iter().for_each(|(name, _, w)| {
            flat.insert(format!("change_pct_{name}"), json!(w.map(|w| w.1)));
        });
        let mut nested = flat.clone();
        win.iter().for_each(|(name, _, _)| {
            nested.shift_remove(&format!("change_pct_{name}"));
        });
        nested.insert(
            "windows".into(),
            Value::Object(
                win.iter()
                    .map(|(name, days, w)| {
                        (name.clone(), json!({"days": days, "change": w.map(|w| w.0), "change_pct": w.map(|w| w.1)}))
                    })
                    .collect(),
            ),
        );
        ["ref_low", "ref_high", "opt_low", "opt_high"].iter().for_each(|k| {
            nested.insert((*k).into(), rec.get(*k).cloned().unwrap_or(Value::Null));
        });
        if !a.no_points {
            nested.insert(
                "points".into(),
                Value::Array(
                    g.iter()
                        .map(|e| {
                            json!({
                                "id": e.row.id,
                                "taken_at": e.row.taken_at,
                                "value": e.display_value,
                                "qualifier": e.row.qualifier,
                                "flag": flag_str(e.flag(flavor)),
                            })
                        })
                        .collect(),
                ),
            );
        }
        Some((flat, Value::Object(nested)))
    };
    let built: Vec<(Record, Value)> = groups.iter().filter_map(|g| build(g)).collect();
    let table_cols: Vec<String> = [
        "person",
        "marker",
        "unit",
        "n",
        "first_date",
        "last_date",
        "min",
        "max",
        "mean",
        "median",
        "last",
        "change_pct",
        "slope_per_year",
    ]
    .iter()
    .map(|c| c.to_string())
    .chain(windows.iter().map(|(n, _)| format!("change_pct_{n}")))
    .collect();
    ctx.emit(
        &Report::list("trend", built.iter().map(|b| b.0.clone()).collect())
            .json_data(Value::Array(built.into_iter().map(|b| b.1).collect()))
            .table_columns(&table_cols)
            .dates(&["first_date", "last_date"])
            .meta("unit_system", json!(ctx.unit_system()))
            .meta("range_flavor", json!(flavor))
            .meta("windows", json!(windows.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>())),
    )
}

pub fn diff(ctx: &Ctx, a: DiffArgs) -> Result<()> {
    let from = parse_date(&a.from, &ctx.tz)?;
    let to = parse_date(&a.to, &ctx.tz)?;
    let (from, to) = if from <= to { (from, to) } else { (to, from) };
    let person = person_slug(ctx, a.person.as_deref())?;
    let filter = FilterArgs {
        persons: vec![person.clone()],
        markers: a.markers.clone(),
        categories: a.categories.clone(),
        to: Some(to.to_string()),
        ..FilterArgs::default()
    };
    let (_, rows): (Catalog, Vec<Evaluated>) = select(ctx, &query_all(filter))?;
    let flavor = ctx.range_flavor();
    let pick = |g: &[Evaluated], on: chrono::NaiveDate| -> Option<Evaluated> {
        g.iter()
            .rev()
            .find(|e| date_of(&e.row.taken_at).is_ok_and(|d| if a.exact { d == on } else { d <= on }))
            .cloned()
    };
    let out: Vec<Record> = series(rows)
        .iter()
        .filter_map(|g| {
            let (f, t) = (pick(g, from), pick(g, to));
            if f.is_none() && t.is_none() {
                return None;
            }
            let change = f.as_ref().zip(t.as_ref()).map(|(f, t)| t.display_value - f.display_value);
            if a.changed && change.is_none_or(|c| c.abs() < 1e-12) {
                return None;
            }
            let any = t.as_ref().or(f.as_ref())?;
            Some(to_record(&json!({
                "person": person,
                "marker": any.marker.slug,
                "category": any.marker.category,
                "unit": any.display_unit,
                "from_date": f.as_ref().map(|e| e.row.taken_at.clone()),
                "from_value": f.as_ref().map(|e| e.display_value),
                "to_date": t.as_ref().map(|e| e.row.taken_at.clone()),
                "to_value": t.as_ref().map(|e| e.display_value),
                "change": change,
                "change_pct": f.as_ref().zip(t.as_ref()).and_then(|(f, t)| stats::pct_change(f.display_value, t.display_value)),
                "from_flag": f.as_ref().and_then(|e| flag_str(e.flag(flavor))),
                "to_flag": t.as_ref().and_then(|e| flag_str(e.flag(flavor))),
            })))
        })
        .collect();
    ctx.emit(
        &Report::list("diff", out)
            .dates(&["from_date", "to_date"])
            .meta("person", json!(person))
            .meta("from", json!(from.to_string()))
            .meta("to", json!(to.to_string()))
            .meta("unit_system", json!(ctx.unit_system())),
    )
}
