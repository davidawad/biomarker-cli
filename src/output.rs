//! Rendering reports as table, JSON (versioned envelope), JSONL, CSV or TSV.

use std::io::Write;
use std::path::PathBuf;

use serde_json::{json, Map, Value};

use crate::error::{AppError, Result};

pub const SCHEMA: &str = "biomarker/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Table,
    Json,
    Jsonl,
    Csv,
    Tsv,
}

impl Format {
    pub fn parse(s: &str) -> Result<Self> {
        <Self as clap::ValueEnum>::from_str(s, true).map_err(|_| AppError::usage(format!("unknown format '{s}'")))
    }
}

#[derive(Debug, Clone)]
pub struct OutputOpts {
    pub format: Format,
    pub output: Option<PathBuf>,
    pub precision: usize,
    pub delimiter: u8,
    pub quote: u8,
    pub header: bool,
    pub null: String,
    pub color: bool,
    pub date_format: String,
    pub columns: Option<Vec<String>>,
}

pub type Record = Map<String, Value>;

#[derive(Debug, Clone)]
pub enum Body {
    /// Tabular data. `table_columns` (if non-empty) selects the compact table view.
    List { rows: Vec<Record>, table_columns: Vec<String> },
    /// A single object, rendered as key/value pairs in table mode.
    Object(Record),
}

#[derive(Debug, Clone)]
pub struct Report {
    pub kind: &'static str,
    pub body: Body,
    /// Replaces `data` in JSON output when present (e.g. nested trend points).
    pub json_data: Option<Value>,
    /// Extra top-level envelope fields.
    pub meta: Record,
    /// Machine-exact rendering (no rounding / date reformatting), used by export.
    pub exact: bool,
    /// Fields holding stored dates, reformatted with `date_format` for display.
    pub date_fields: Vec<&'static str>,
}

impl Report {
    pub fn list(kind: &'static str, rows: Vec<Record>) -> Self {
        Self {
            kind,
            body: Body::List { rows, table_columns: Vec::new() },
            json_data: None,
            meta: Map::new(),
            exact: false,
            date_fields: Vec::new(),
        }
    }
    pub fn object(kind: &'static str, rec: Record) -> Self {
        Self { body: Body::Object(rec), ..Self::list(kind, Vec::new()) }
    }
    pub fn table_columns<S: AsRef<str>>(mut self, cols: &[S]) -> Self {
        if let Body::List { table_columns, .. } = &mut self.body {
            *table_columns = cols.iter().map(|c| c.as_ref().to_string()).collect();
        }
        self
    }
    pub fn json_data(mut self, v: Value) -> Self {
        self.json_data = Some(v);
        self
    }
    pub fn meta(mut self, k: &str, v: Value) -> Self {
        self.meta.insert(k.to_string(), v);
        self
    }
    pub fn exact(mut self) -> Self {
        self.exact = true;
        self
    }
    pub fn dates(mut self, fields: &[&'static str]) -> Self {
        self.date_fields = fields.to_vec();
        self
    }
    pub fn rows(&self) -> &[Record] {
        match &self.body {
            Body::List { rows, .. } => rows,
            Body::Object(_) => &[],
        }
    }
}

/// Serialize any value into a JSON object record.
pub fn to_record<T: serde::Serialize>(v: &T) -> Record {
    match serde_json::to_value(v) {
        Ok(Value::Object(m)) => m,
        _ => Map::new(),
    }
}

pub fn envelope(r: &Report) -> Value {
    let data = r.json_data.clone().unwrap_or_else(|| match &r.body {
        Body::List { rows, .. } => Value::Array(rows.iter().cloned().map(Value::Object).collect()),
        Body::Object(o) => Value::Object(o.clone()),
    });
    let mut env = Map::new();
    env.insert("schema".into(), json!(SCHEMA));
    env.insert("kind".into(), json!(r.kind));
    env.insert("generated_at".into(), json!(crate::util::now_iso()));
    if let Value::Array(a) = &data {
        env.insert("count".into(), json!(a.len()));
    }
    r.meta.iter().for_each(|(k, v)| {
        env.insert(k.clone(), v.clone());
    });
    env.insert("data".into(), data);
    Value::Object(env)
}

/// Render a cell for text formats.
fn cell(v: &Value, key: &str, r: &Report, o: &OutputOpts) -> String {
    match v {
        Value::Null => o.null.clone(),
        Value::String(s) if !r.exact && r.date_fields.contains(&key) => crate::util::display_when(s, &o.date_format),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) if n.is_f64() && !r.exact => {
            let f = n.as_f64().unwrap_or_default();
            format!("{:.*}", o.precision, f)
        }
        Value::Number(n) => n.to_string(),
        Value::Array(a) => a.iter().map(|x| cell(x, "", r, o)).collect::<Vec<_>>().join(","),
        Value::Object(_) => v.to_string(),
    }
}

fn columns_of(rows: &[Record]) -> Vec<String> {
    rows.iter().flat_map(|r| r.keys().cloned()).fold(Vec::new(), |mut acc, k| {
        if !acc.contains(&k) {
            acc.push(k);
        }
        acc
    })
}

fn selected_columns(r: &Report, o: &OutputOpts, table: bool) -> Vec<String> {
    match (&o.columns, &r.body) {
        (_, Body::Object(_)) => vec!["field".into(), "value".into()],
        (Some(c), _) => c.clone(),
        (None, Body::List { rows, table_columns }) if table && !table_columns.is_empty() => {
            let all = columns_of(rows);
            table_columns.iter().filter(|c| rows.is_empty() || all.contains(c)).cloned().collect()
        }
        (None, Body::List { rows, .. }) => columns_of(rows),
    }
}

/// Rows as text cells, with object bodies pivoted into field/value pairs.
fn text_rows(r: &Report, o: &OutputOpts, cols: &[String]) -> Vec<Vec<String>> {
    match &r.body {
        Body::List { rows, .. } => rows
            .iter()
            .map(|row| cols.iter().map(|c| row.get(c).map_or_else(|| o.null.clone(), |v| cell(v, c, r, o))).collect())
            .collect(),
        Body::Object(obj) => obj
            .iter()
            .filter(|(k, _)| o.columns.as_ref().is_none_or(|c| c.contains(k)))
            .map(|(k, v)| vec![k.clone(), cell(v, k, r, o)])
            .collect(),
    }
}

fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.parse::<f64>().is_ok()
}

fn colorize(col: &str, s: &str) -> String {
    let code = match (col, s) {
        (c, "high") if c.ends_with("flag") => "31",
        (c, "low") if c.ends_with("flag") => "33",
        (c, "normal") if c.ends_with("flag") => "32",
        _ => return s.to_string(),
    };
    format!("\x1b[{code}m{s}\x1b[0m")
}

pub fn render_table(r: &Report, o: &OutputOpts) -> String {
    let cols = selected_columns(r, o, true);
    let rows = text_rows(r, o, &cols);
    if rows.is_empty() {
        return String::new();
    }
    let width = |s: &str| s.chars().count();
    let widths: Vec<usize> = cols
        .iter()
        .enumerate()
        .map(|(i, c)| {
            rows.iter()
                .map(|row| width(&row[i]))
                .chain(matches!(r.body, Body::List { .. }).then(|| width(c)))
                .max()
                .unwrap_or(0)
        })
        .collect();
    let numeric: Vec<bool> = (0..cols.len())
        .map(|i| {
            rows.iter().any(|row| is_numeric(&row[i])) && rows.iter().all(|row| is_numeric(&row[i]) || row[i] == o.null)
        })
        .collect();
    let fmt_row = |row: &[String], header: bool| {
        row.iter()
            .enumerate()
            .map(|(i, s)| {
                let pad = " ".repeat(widths[i].saturating_sub(width(s)));
                let shown = match (o.color, header) {
                    (true, true) => format!("\x1b[1m{s}\x1b[0m"),
                    (true, false) => colorize(&cols[i], s),
                    _ => s.clone(),
                };
                if numeric[i] && !header {
                    format!("{pad}{shown}")
                } else {
                    format!("{shown}{pad}")
                }
            })
            .collect::<Vec<_>>()
            .join("  ")
            .trim_end()
            .to_string()
    };
    let header = cols.iter().map(|c| c.to_uppercase()).collect::<Vec<_>>();
    let rule = widths.iter().map(|w| "─".repeat(*w)).collect::<Vec<_>>().join("  ");
    let body = rows.iter().map(|row| fmt_row(row, false));
    let lines: Vec<String> = match r.body {
        Body::Object(_) => body.collect(),
        Body::List { .. } => [fmt_row(&header, true), rule].into_iter().chain(body).collect(),
    };
    lines.join("\n") + "\n"
}

pub fn render_delimited(r: &Report, o: &OutputOpts, delimiter: u8) -> Result<String> {
    // A single object becomes a one-row table with its keys as the header.
    let as_list;
    let r = match &r.body {
        Body::Object(obj) => {
            as_list = Report { body: Body::List { rows: vec![obj.clone()], table_columns: Vec::new() }, ..r.clone() };
            &as_list
        }
        Body::List { .. } => r,
    };
    let cols = selected_columns(r, o, false);
    let mut w = csv::WriterBuilder::new().delimiter(delimiter).quote(o.quote).from_writer(Vec::new());
    if o.header {
        w.write_record(&cols)?;
    }
    text_rows(r, o, &cols).iter().try_for_each(|row| w.write_record(row))?;
    let bytes = w.into_inner().map_err(|e| AppError::io(e.to_string()))?;
    String::from_utf8(bytes).map_err(|e| AppError::io(e.to_string()))
}

pub fn render_jsonl(r: &Report) -> String {
    let items: Vec<Value> = match (&r.json_data, &r.body) {
        (Some(Value::Array(a)), _) => a.clone(),
        (Some(v), _) => vec![v.clone()],
        (None, Body::List { rows, .. }) => rows.iter().cloned().map(Value::Object).collect(),
        (None, Body::Object(o)) => vec![Value::Object(o.clone())],
    };
    items.iter().map(|v| v.to_string() + "\n").collect()
}

pub fn render(r: &Report, o: &OutputOpts) -> Result<String> {
    match o.format {
        Format::Table => Ok(render_table(r, o)),
        Format::Json => Ok(serde_json::to_string_pretty(&envelope(r))? + "\n"),
        Format::Jsonl => Ok(render_jsonl(r)),
        Format::Csv => render_delimited(r, o, o.delimiter),
        Format::Tsv => render_delimited(r, o, b'\t'),
    }
}

/// Render and write to `--output` or stdout.
pub fn emit(r: &Report, o: &OutputOpts) -> Result<()> {
    let text = render(r, o)?;
    match &o.output {
        Some(p) if p.as_os_str() != "-" => {
            std::fs::write(p, text).map_err(|e| AppError::io(format!("writing {}: {e}", p.display())))
        }
        _ => {
            let mut out = std::io::stdout().lock();
            match out.write_all(text.as_bytes()).and_then(|()| out.flush()) {
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
                other => other.map_err(AppError::from),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(format: Format) -> OutputOpts {
        OutputOpts {
            format,
            output: None,
            precision: 1,
            delimiter: b',',
            quote: b'"',
            header: true,
            null: "-".into(),
            color: false,
            date_format: "%d/%m/%Y".into(),
            columns: None,
        }
    }

    fn report() -> Report {
        let rows = vec![
            to_record(&json!({"marker": "ldl-c", "date": "2024-01-02", "value": 101.26, "n": 3, "note": null})),
            to_record(&json!({"marker": "hdl, c", "date": "2024-02-03", "value": 55.0, "n": 1, "note": "x"})),
        ];
        Report::list("measurements", rows).dates(&["date"])
    }

    #[test]
    fn json_envelope_is_versioned() {
        let v: Value = serde_json::from_str(&render(&report(), &opts(Format::Json)).unwrap()).unwrap();
        assert_eq!(v["schema"], "biomarker/v1");
        assert_eq!(v["kind"], "measurements");
        assert_eq!(v["count"], 2);
        assert_eq!(v["data"][0]["value"], 101.26);
    }

    #[test]
    fn csv_quotes_and_formats() {
        let s = render(&report(), &opts(Format::Csv)).unwrap();
        assert_eq!(s, "marker,date,value,n,note\nldl-c,02/01/2024,101.3,3,-\n\"hdl, c\",03/02/2024,55.0,1,x\n");
        let exact = render(&report().exact(), &opts(Format::Csv)).unwrap();
        assert!(exact.contains("ldl-c,2024-01-02,101.26,3,-"));
    }

    #[test]
    fn table_and_jsonl() {
        let t = render(&report(), &opts(Format::Table)).unwrap();
        assert!(t.starts_with("MARKER"));
        assert_eq!(t.lines().count(), 4);
        let l = render(&report(), &opts(Format::Jsonl)).unwrap();
        assert_eq!(l.lines().count(), 2);
    }
}
