//! Spreadsheet layouts for `biomarker import`: header detection, long/wide
//! sheets as rows, and the transposed dashboard layout (one row per test, one
//! column per draw date, section-header rows between groups of tests).

use chrono::NaiveDate;

use crate::cli::ImportArgs;
use crate::commands::import::{Mapping, Planned, RawRow};
use crate::commands::measure::Input;
use crate::error::{AppError, Result};
use crate::sheet::{col_index, col_name, header_date, Cell, Grid};
use crate::matching::name_key;
use crate::util::{parse_value, slugify};

/// How many leading rows are searched for a header.
const HEADER_SCAN: usize = 50;

fn date_cells(g: &Grid, row: usize) -> usize {
    g.rows.get(row).map_or(0, |r| r.iter().filter(|c| header_date(c).is_some()).count())
}

fn text_cells(g: &Grid, row: usize) -> usize {
    g.rows.get(row).map_or(0, |r| r.iter().filter(|c| matches!(c, Cell::Text(_)) && header_date(c).is_none()).count())
}

/// A sheet "looks transposed" when some header row carries several dates.
pub fn looks_transposed(g: &Grid) -> bool {
    (0..g.rows.len().min(HEADER_SCAN)).any(|r| date_cells(g, r) >= 2 && text_cells(g, r) >= 1)
}

/// 0-based header row: `--header-row` (1-based) or auto-detected.
pub fn header_row(g: &Grid, explicit: Option<usize>, transposed: bool) -> Result<usize> {
    if let Some(n) = explicit {
        return match n {
            0 => Err(AppError::usage("--header-row is 1-based")),
            n if n > g.rows.len() => {
                Err(AppError::usage(format!("--header-row {n} is past the end of the sheet ({} rows)", g.rows.len())))
            }
            n => Ok(n - 1),
        };
    }
    let scan = 0..g.rows.len().min(HEADER_SCAN);
    let found = if transposed {
        scan.clone()
            .find(|&r| date_cells(g, r) >= 2 && text_cells(g, r) >= 1)
            .or_else(|| scan.clone().find(|&r| date_cells(g, r) >= 1 && text_cells(g, r) >= 1))
    } else {
        scan.clone().find(|&r| {
            let row = &g.rows[r];
            let filled = row.iter().filter(|c| !c.is_empty()).count();
            filled >= 2 && row.iter().filter(|c| !c.is_empty()).all(|c| matches!(c, Cell::Text(_)))
        })
    };
    found.ok_or_else(|| {
        AppError::invalid(format!(
            "could not find a header row in sheet '{}'; pass --header-row N{}",
            g.name,
            if transposed { " (the row whose cells after the test columns are dates)" } else { "" }
        ))
    })
}

fn header_texts(g: &Grid, header: usize) -> Vec<String> {
    (0..g.width())
        .map(|c| match g.cell(header, c) {
            c if c.is_empty() => String::new(),
            c => c.text(),
        })
        .collect()
}

/// Long / wide sheets become ordinary input rows keyed by the header text
/// (columns without a header are named by their letter).
pub fn grid_rows(g: &Grid, header: usize) -> Vec<(usize, RawRow)> {
    let headers: Vec<String> = header_texts(g, header)
        .into_iter()
        .enumerate()
        .map(|(i, h)| if h.is_empty() { col_name(i) } else { h })
        .collect();
    g.rows
        .iter()
        .enumerate()
        .skip(header + 1)
        .filter(|(_, r)| r.iter().any(|c| !c.is_empty()))
        .map(|(i, r)| {
            let row =
                headers.iter().enumerate().map(|(c, h)| (h.clone(), r.get(c).map(Cell::text).unwrap_or_default()));
            (i + 1, row.collect())
        })
        .collect()
}

/// Find a column by header text (case/punctuation-insensitive) or by letter.
fn find_col(headers: &[String], spec: &str) -> Option<usize> {
    let want = slugify(spec);
    headers
        .iter()
        .position(|h| h.trim().eq_ignore_ascii_case(spec.trim()))
        .or_else(|| headers.iter().position(|h| !want.is_empty() && slugify(h) == want))
        .or_else(|| col_index(spec))
}

fn pick_col(headers: &[String], explicit: Option<&str>, names: &[&str], what: &str) -> Result<Option<usize>> {
    match explicit {
        Some(spec) => find_col(headers, spec)
            .map(Some)
            .ok_or_else(|| AppError::usage(format!("{what} column '{spec}' not found in the header row"))),
        None => Ok(names.iter().find_map(|n| headers.iter().position(|h| slugify(h) == *n))),
    }
}

/// A reference interval stated by the sheet for one test.
#[derive(Debug, Clone)]
pub struct SheetRange {
    pub line: usize,
    pub source: String,
    pub marker: String,
    pub unit: Option<String>,
    pub low: Option<f64>,
    pub high: Option<f64>,
}

#[derive(Debug, Default)]
pub struct Transposed {
    /// 1-based header row.
    pub header_row: usize,
    pub dates: Vec<NaiveDate>,
    /// Test rows (rows with a name, excluding section headers).
    pub rows: usize,
    pub sections: Vec<String>,
    pub items: Vec<std::result::Result<Planned, (usize, String)>>,
    pub ranges: Vec<SheetRange>,
    /// (line, name) of the `stop_at` row, when the import stopped there.
    pub stopped_at: Option<(usize, String)>,
}

/// `3.4-10.8`, `<200`, `>= 40`, `0 - 40` -> (low, high).
pub fn parse_interval(s: &str) -> Option<(Option<f64>, Option<f64>)> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    if let Ok((q, v)) = parse_value(t) {
        return match q.as_deref() {
            Some("<" | "<=") => Some((None, Some(v))),
            Some(">" | ">=") => Some((Some(v), None)),
            _ => None,
        };
    }
    let (lo, hi) = t
        .char_indices()
        .skip(1)
        .find(|(_, c)| matches!(c, '-' | '–'))
        .map(|(i, c)| (&t[..i], &t[i + c.len_utf8()..]))?;
    match (parse_value(lo), parse_value(hi)) {
        (Ok((None, l)), Ok((None, h))) if l <= h => Some((Some(l), Some(h))),
        _ => None,
    }
}

fn number(c: &Cell) -> Option<f64> {
    match c {
        Cell::Number(n) => Some(*n),
        Cell::Text(t) => parse_value(t).ok().map(|(_, v)| v),
        _ => None,
    }
}

/// Plan a transposed sheet: every non-empty cell under a date header is one
/// value of the row's test on that date.
pub fn plan_transposed(g: &Grid, a: &ImportArgs, m: &Mapping, extra_tags: &[String]) -> Result<Transposed> {
    let header = header_row(g, a.header_row.or(m.header_row), true)?;
    let headers = header_texts(g, header);
    let marker_col = pick_col(
        &headers,
        a.marker_col.as_deref().or(m.marker_col.as_deref()),
        &["test", "tests", "marker", "biomarker", "analyte", "name", "test-name", "component"],
        "marker",
    )?;
    let unit_col = pick_col(&headers, a.unit_col.as_deref(), &["units", "unit", "uom"], "unit")?;
    let low_col = pick_col(
        &headers,
        a.ref_low_col.as_deref(),
        &["ref-low", "reference-low", "ref-lo", "low", "lower", "lower-limit", "min"],
        "ref low",
    )?;
    let high_col = pick_col(
        &headers,
        a.ref_high_col.as_deref(),
        &["ref-high", "reference-high", "ref-hi", "high", "upper", "upper-limit", "max"],
        "ref high",
    )?;
    let interval_col = pick_col(
        &headers,
        None,
        &["reference-interval", "reference-range", "ref-range", "ref-interval", "reference", "range"],
        "",
    )?;
    let meta: Vec<usize> = [marker_col, unit_col, low_col, high_col, interval_col].into_iter().flatten().collect();
    let date_cols: Vec<(usize, NaiveDate)> = (0..headers.len())
        .filter(|c| !meta.contains(c))
        .filter_map(|c| header_date(g.cell(header, c)).map(|d| (c, d)))
        .collect();
    if date_cols.is_empty() {
        return Err(AppError::invalid(format!(
            "transposed layout: no date columns in header row {} of sheet '{}' (dates, Excel serials or ISO/US date text)",
            header + 1,
            g.name
        )));
    }
    // Default marker column: the first column that is neither metadata nor a date.
    let marker_col = marker_col
        .or_else(|| (0..headers.len()).find(|c| !meta.contains(c) && !date_cols.iter().any(|(d, _)| d == c)))
        .ok_or_else(|| AppError::usage("transposed layout: no test-name column; pass --marker-col"))?;
    let sections = a.sections_as_category.unwrap_or(m.sections_as_category.unwrap_or(true));

    let mut out = Transposed {
        header_row: header + 1,
        dates: date_cols.iter().map(|(_, d)| *d).collect(),
        ..Transposed::default()
    };
    let mut section: Option<String> = None;
    for r in header + 1..g.rows.len() {
        let line = r + 1;
        let name = g.cell(r, marker_col).text();
        if name.is_empty() {
            continue;
        }
        if m.stop_at.as_deref() == Some(name_key(&name).as_str()) {
            out.stopped_at = Some((line, name));
            break;
        }
        let values: Vec<(NaiveDate, String)> = date_cols
            .iter()
            .filter(|(c, _)| !g.cell(r, *c).is_empty())
            .map(|(c, d)| (*d, g.cell(r, *c).text()))
            .collect();
        let unit = unit_col.map(|c| g.cell(r, c).text()).filter(|u| !u.is_empty());
        let low = low_col.and_then(|c| number(g.cell(r, c)));
        let high = high_col.and_then(|c| number(g.cell(r, c)));
        let (low, high) = match (low, high) {
            (None, None) => interval_col.and_then(|c| parse_interval(&g.cell(r, c).text())).unwrap_or((None, None)),
            lh => lh,
        };
        if values.is_empty() {
            // A name with no values, unit or range is a section header; a test
            // that simply has no results yet is ignored.
            if sections && unit.is_none() && low.is_none() && high.is_none() {
                out.sections.push(name.clone());
                section = Some(name);
            }
            continue;
        }
        out.rows += 1;
        let renamed = m.rename(&name);
        if low.is_some() || high.is_some() {
            out.ranges.push(SheetRange {
                line,
                source: name.clone(),
                marker: renamed.clone(),
                unit: unit.clone(),
                low,
                high,
            });
        }
        out.items.extend(values.into_iter().map(|(date, value)| {
            Ok(Planned {
                line,
                person: m.defaults.get("person").cloned(),
                source: name.clone(),
                section: section.clone(),
                input: Input {
                    marker: renamed.clone(),
                    value,
                    unit: unit.clone(),
                    date: date.format("%Y-%m-%d").to_string(),
                    lab: m.defaults.get("lab").cloned(),
                    tags: extra_tags.to_vec(),
                    ..Input::default()
                },
            })
        }));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intervals() {
        assert_eq!(parse_interval("3.4-10.8"), Some((Some(3.4), Some(10.8))));
        assert_eq!(parse_interval("0 - 40"), Some((Some(0.0), Some(40.0))));
        assert_eq!(parse_interval("<200"), Some((None, Some(200.0))));
        assert_eq!(parse_interval(">= 40"), Some((Some(40.0), None)));
        assert_eq!(parse_interval("Negative"), None);
        assert_eq!(parse_interval("-5"), None);
    }

    fn grid(rows: Vec<Vec<Cell>>) -> Grid {
        Grid { name: "t".into(), rows }
    }

    #[test]
    fn detects_headers() {
        let t = |s: &str| Cell::Text(s.into());
        let g = grid(vec![
            vec![t("My dashboard")],
            vec![],
            vec![t("Test"), t("Units"), Cell::Number(45000.0), Cell::Number(45100.0)],
            vec![t("WBC"), t("x10E3/uL"), Cell::Number(5.1), Cell::Empty],
        ]);
        assert!(looks_transposed(&g));
        assert_eq!(header_row(&g, None, true).unwrap(), 2);
        assert_eq!(header_row(&g, Some(3), true).unwrap(), 2);
        assert!(header_row(&g, Some(0), true).is_err());
        let long = grid(vec![
            vec![t("Weight log")],
            vec![t("Date"), t("Weight (lbs.)")],
            vec![Cell::Date(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap()), Cell::Number(180.5)],
        ]);
        assert!(!looks_transposed(&long));
        assert_eq!(header_row(&long, None, false).unwrap(), 1);
        let rows = grid_rows(&long, 1);
        assert_eq!(
            rows,
            vec![(3, vec![("Date".into(), "2024-01-02".into()), ("Weight (lbs.)".into(), "180.5".into())])]
        );
    }
}
