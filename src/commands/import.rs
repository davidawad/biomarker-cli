//! `biomarker import`: CSV / TSV / JSON / JSONL and spreadsheets (xlsx, xls,
//! ods); long, wide or transposed layout, with configurable column mapping,
//! dedupe policy and dry-run.
//!
//! The whole import runs in one transaction; `--dry-run` simply rolls it back,
//! so the reported counts are exactly what a real run would do.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Read;
use std::path::Path;

use serde_json::{json, Value};

use crate::cli::{ImportArgs, InputFormat, Layout, QualitativeArg, RangesArg};
use crate::commands::import_sheet::{self, SheetRange};
use crate::commands::measure::{build, Input};
use crate::context::Ctx;
use crate::db::Db;
use crate::error::{AppError, ErrorKind, Result};
use crate::matching::{self, name_key};
use crate::output::{to_record, Report};
use crate::ranges::{Range, RangeKind};
use crate::sheet;
use crate::store::{self, Catalog, Dedupe, Marker, NewObservation, Outcome, Person};
use crate::units::{clean_unit, same_unit};
use crate::util::{new_batch_id, now_iso, parse_bool, parse_tags, parse_value, parse_when, slugify};

/// One input row as ordered (column, text) pairs.
pub type RawRow = Vec<(String, String)>;

pub const FIELDS: &[&str] =
    &["person", "marker", "value", "unit", "date", "time", "lab", "fasting", "note", "tags", "qualifier"];

fn synonyms(field: &str) -> &'static [&'static str] {
    match field {
        "person" => &["person", "person_slug", "patient", "who", "subject"],
        "marker" => &["marker", "marker_slug", "biomarker", "test", "test_name", "analyte", "component"],
        "value" => &["value", "result", "value_raw", "result_value"],
        "unit" => &["unit", "units", "unit_raw"],
        "date" => &[
            "date",
            "taken_at",
            "datetime",
            "timestamp",
            "collected",
            "collected_at",
            "collection_date",
            "date_collected",
            "drawn",
        ],
        "time" => &["time", "collection_time"],
        "lab" => &["lab", "source", "laboratory", "provider"],
        "fasting" => &["fasting", "fasted"],
        "note" => &["note", "notes", "comment", "comments"],
        "tags" => &["tags", "tag", "labels"],
        "qualifier" => &["qualifier", "operator", "modifier"],
        _ => &[],
    }
}

/// Parsed mapping file.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MappingFile {
    #[serde(default)]
    pub columns: BTreeMap<String, String>,
    #[serde(default)]
    pub defaults: BTreeMap<String, toml::Value>,
    /// Source marker name -> catalog marker.
    #[serde(default)]
    pub markers: BTreeMap<String, String>,
    /// Same as `[markers]`: source name -> marker slug.
    #[serde(default)]
    pub rename: BTreeMap<String, String>,
    /// Source unit -> unit spelling to use.
    #[serde(default)]
    pub units: BTreeMap<String, String>,
    /// Source date -> corrected date (e.g. a sheet column dated off from the lab report).
    #[serde(default)]
    pub dates: BTreeMap<String, String>,
    /// Source names to ignore: an array, or a table of `"name" = true`.
    pub skip: Option<toml::Value>,
    /// Section title or source name -> category for created markers.
    #[serde(default)]
    pub category: BTreeMap<String, String>,
    /// Value columns for long sheets: slug -> `HEADER` or `HEADER:UNIT`.
    #[serde(default)]
    pub value_columns: BTreeMap<String, String>,
    /// `sheet` sets person-specific reference ranges from the sheet's ref columns.
    pub ranges: Option<String>,
    pub layout: Option<String>,
    pub sheet: Option<String>,
    pub header_row: Option<usize>,
    pub marker_col: Option<String>,
    /// Spreadsheet imports stop at the first row with this marker name (it
    /// and every row below are ignored), e.g. a section whose cells are misaligned.
    pub stop_at: Option<String>,
    pub sections_as_category: Option<bool>,
    pub wide: Option<bool>,
    pub date_format: Option<String>,
    pub delimiter: Option<String>,
}

/// One `--value-column SLUG=HEADER[:UNIT]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueColumn {
    pub marker: String,
    pub header: String,
    pub unit: Option<String>,
}

pub fn parse_value_column(spec: &str) -> Result<ValueColumn> {
    let (slug, rest) =
        spec.split_once('=').filter(|(s, h)| !s.trim().is_empty() && !h.trim().is_empty()).ok_or_else(|| {
            AppError::usage(format!("bad --value-column '{spec}' (expected SLUG=HEADER or SLUG=HEADER:UNIT)"))
        })?;
    let (header, unit) = match rest.rsplit_once(':') {
        Some((h, u)) if !h.trim().is_empty() && !u.trim().is_empty() => (h, Some(u.trim().to_string())),
        _ => (rest, None),
    };
    Ok(ValueColumn { marker: slug.trim().to_string(), header: header.trim().to_string(), unit })
}

/// Effective mapping after merging the file and flags.
#[derive(Debug, Default, Clone)]
pub struct Mapping {
    pub columns: BTreeMap<String, String>,
    pub defaults: BTreeMap<String, String>,
    /// [`name_key`] of a source name -> marker.
    pub markers: HashMap<String, String>,
    pub wide: bool,
    pub date_format: Option<String>,
    pub value_columns: Vec<ValueColumn>,
    /// Lowercased source unit -> spelling.
    pub units: HashMap<String, String>,
    pub dates: BTreeMap<String, String>,
    /// [`name_key`]s of names to ignore.
    pub skip: HashSet<String>,
    /// [`name_key`] of a section title or source name -> category.
    pub category: HashMap<String, String>,
    pub ranges_from_sheet: bool,
    pub layout: Option<Layout>,
    pub sheet: Option<String>,
    pub header_row: Option<usize>,
    pub marker_col: Option<String>,
    /// `name_key` of the row where spreadsheet imports stop (`stop_at`).
    pub stop_at: Option<String>,
    pub sections_as_category: Option<bool>,
}

impl Mapping {
    /// Apply `[markers]` / `[rename]` to a source name.
    pub fn rename(&self, name: &str) -> String {
        self.markers.get(&name_key(name)).cloned().unwrap_or_else(|| name.trim().to_string())
    }

    pub fn skips(&self, name: &str) -> bool {
        self.skip.contains(&name_key(name))
    }

    /// Unit as written in the source, after `[units]` (and, for spreadsheets, clean-up).
    pub fn unit(&self, raw: &str, clean: bool) -> String {
        let t = raw.trim();
        match self.units.get(&t.to_lowercase()) {
            Some(u) => u.clone(),
            None if clean => clean_unit(t),
            None => t.to_string(),
        }
    }
}

fn toml_text(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn skip_names(v: Option<&toml::Value>) -> Result<Vec<String>> {
    let bad = || AppError::invalid("mapping [skip]: expected an array of names or a table of \"name\" = true");
    match v {
        None => Ok(vec![]),
        Some(toml::Value::Array(a)) => a.iter().map(|x| x.as_str().map(str::to_string).ok_or_else(bad)).collect(),
        Some(toml::Value::Table(t)) => t
            .iter()
            .flat_map(|(k, v)| match (k.as_str(), v) {
                ("names", toml::Value::Array(a)) => {
                    a.iter().map(|x| x.as_str().map(str::to_string).ok_or_else(bad)).collect()
                }
                (_, toml::Value::Boolean(true)) => vec![Ok(k.clone())],
                (_, toml::Value::Boolean(false)) => vec![],
                _ => vec![Err(bad())],
            })
            .collect(),
        Some(_) => Err(bad()),
    }
}

fn parse_layout(s: &str) -> Result<Layout> {
    <Layout as clap::ValueEnum>::from_str(s, true)
        .map_err(|_| AppError::invalid(format!("mapping: unknown layout '{s}' (long, wide, transposed)")))
}

pub fn load_mapping(a: &ImportArgs) -> Result<(Mapping, Option<String>)> {
    let file = a
        .mapping
        .as_deref()
        .map(|p| {
            std::fs::read_to_string(p)
                .map_err(|e| AppError::io(format!("reading mapping {}: {e}", p.display())))
                .and_then(|t| {
                    toml::from_str::<MappingFile>(&t)
                        .map_err(|e| AppError::invalid(format!("mapping {}: {e}", p.display())))
                })
        })
        .transpose()?
        .unwrap_or_default();
    let flag_maps = a
        .maps
        .iter()
        .map(|m| {
            m.split_once('=')
                .map(|(f, c)| (f.trim().to_lowercase(), c.trim().to_string()))
                .ok_or_else(|| AppError::usage(format!("bad --map '{m}' (expected FIELD=COLUMN)")))
        })
        .collect::<Result<Vec<_>>>()?;
    let columns: BTreeMap<String, String> =
        file.columns.into_iter().map(|(k, v)| (k.to_lowercase(), v)).chain(flag_maps).collect();
    if let Some(bad) = columns.keys().find(|k| !FIELDS.contains(&k.as_str())) {
        return Err(AppError::usage(format!("unknown import field '{bad}' (fields: {})", FIELDS.join(", "))));
    }
    let defaults = file
        .defaults
        .iter()
        .map(|(k, v)| (k.to_lowercase(), toml_text(v)))
        .chain(a.person.clone().map(|p| ("person".into(), p)))
        .chain(a.unit.clone().map(|u| ("unit".into(), u)))
        .chain(a.lab.clone().map(|l| ("lab".into(), l)))
        .collect();
    let value_columns = file
        .value_columns
        .iter()
        .map(|(k, v)| parse_value_column(&format!("{k}={v}")))
        .chain(a.value_columns.iter().map(|v| parse_value_column(v)))
        .collect::<Result<Vec<_>>>()?;
    let ranges_from_sheet = match (a.ranges, file.ranges.as_deref().map(str::to_ascii_lowercase).as_deref()) {
        (Some(r), _) => r == RangesArg::Sheet,
        (None, None | Some("catalog")) => false,
        (None, Some("sheet")) => true,
        (None, Some(other)) => {
            return Err(AppError::invalid(format!("mapping: ranges = '{other}' (expected \"sheet\" or \"catalog\")")))
        }
    };
    let skip = skip_names(file.skip.as_ref())?;
    Ok((
        Mapping {
            columns,
            defaults,
            markers: file.markers.into_iter().chain(file.rename).map(|(k, v)| (name_key(&k), v)).collect(),
            wide: a.wide || file.wide.unwrap_or(false),
            date_format: a.input_date_format.clone().or(file.date_format),
            value_columns,
            units: file.units.into_iter().map(|(k, v)| (k.trim().to_lowercase(), v)).collect(),
            dates: file.dates,
            skip: skip.iter().map(|n| name_key(n)).collect(),
            category: file.category.into_iter().map(|(k, v)| (name_key(&k), v)).collect(),
            ranges_from_sheet,
            layout: file.layout.as_deref().map(parse_layout).transpose()?,
            sheet: file.sheet,
            header_row: file.header_row,
            marker_col: file.marker_col,
            stop_at: file.stop_at.as_deref().map(name_key),
            sections_as_category: file.sections_as_category,
        },
        file.delimiter,
    ))
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

pub fn detect_format(path: &Path, explicit: Option<InputFormat>, text: &str) -> InputFormat {
    explicit.unwrap_or_else(|| {
        match path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
            Some("json") => InputFormat::Json,
            Some("jsonl" | "ndjson") => InputFormat::Jsonl,
            Some("tsv" | "tab") => InputFormat::Tsv,
            Some("csv") => InputFormat::Csv,
            Some(e) if sheet::EXTENSIONS.contains(&e) => InputFormat::Spreadsheet,
            _ => match text.trim_start().chars().next() {
                Some('[') => InputFormat::Json,
                Some('{') if text.trim_start().starts_with("{\"schema\"") => InputFormat::Json,
                Some('{') => InputFormat::Jsonl,
                _ => InputFormat::Csv,
            },
        }
    })
}

fn json_text(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Array(a) => a.iter().map(json_text).collect::<Vec<_>>().join(","),
        other => other.to_string(),
    }
}

fn object_row(v: &Value) -> Result<RawRow> {
    v.as_object()
        .map(|o| o.iter().map(|(k, v)| (k.clone(), json_text(v))).collect())
        .ok_or_else(|| AppError::invalid("expected a JSON object per record"))
}

pub fn parse_rows(text: &str, fmt: InputFormat, delimiter: u8) -> Result<Vec<(usize, RawRow)>> {
    match fmt {
        InputFormat::Csv | InputFormat::Tsv => {
            let delim = if fmt == InputFormat::Tsv { b'\t' } else { delimiter };
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(delim)
                .flexible(true)
                .trim(csv::Trim::All)
                .from_reader(text.trim_start_matches('\u{feff}').as_bytes());
            let headers: Vec<String> = rdr.headers()?.iter().map(str::to_string).collect();
            rdr.records()
                .map(|r| {
                    let r = r?;
                    let line = r.position().map_or(0, |p| p.line() as usize);
                    Ok((line, headers.iter().cloned().zip(r.iter().map(str::to_string)).collect()))
                })
                .collect()
        }
        InputFormat::Json => {
            let v: Value = serde_json::from_str(text)?;
            let items = match &v {
                Value::Array(a) => a.clone(),
                Value::Object(o) => match o.get("data") {
                    Some(Value::Array(a)) => a.clone(),
                    _ => vec![v.clone()],
                },
                _ => {
                    return Err(AppError::invalid("JSON input must be an array, an object, or a biomarker/v1 envelope"))
                }
            };
            items.iter().enumerate().map(|(i, v)| object_row(v).map(|r| (i + 1, r))).collect()
        }
        InputFormat::Jsonl => text
            .lines()
            .enumerate()
            .filter(|(_, l)| !l.trim().is_empty())
            .map(|(i, l)| {
                serde_json::from_str::<Value>(l)
                    .map_err(|e| AppError::invalid(format!("line {}: {e}", i + 1)))
                    .and_then(|v| object_row(&v))
                    .map(|r| (i + 1, r))
            })
            .collect(),
        InputFormat::Spreadsheet => Err(AppError::invalid("spreadsheets are read with the sheet reader, not as text")),
    }
}

// ---------------------------------------------------------------------------
// Mapping rows to inputs
// ---------------------------------------------------------------------------

/// Locate the column used for `field` (explicit mapping, else known synonyms).
fn column_for<'a>(m: &'a Mapping, headers: &'a [String], field: &str) -> Option<&'a str> {
    m.columns
        .get(field)
        .and_then(|c| headers.iter().find(|h| h.eq_ignore_ascii_case(c)))
        .or_else(|| {
            synonyms(field).iter().find_map(|s| {
                headers.iter().find(|h| h.trim().eq_ignore_ascii_case(s) || slugify(h).replace('-', "_") == *s)
            })
        })
        .map(String::as_str)
}

fn get<'a>(row: &'a RawRow, col: Option<&str>) -> Option<&'a str> {
    col.and_then(|c| row.iter().find(|(h, _)| h == c)).map(|(_, v)| v.as_str()).filter(|v| !v.trim().is_empty())
}

/// Split a wide-format header like `Glucose (mmol/L)` or `ldl-c [mg/dL]` into name and unit.
pub fn split_header_unit(h: &str) -> (String, Option<String>) {
    let t = h.trim();
    [('(', ')'), ('[', ']')]
        .iter()
        .find_map(|(o, c)| {
            t.strip_suffix(*c)
                .and_then(|s| s.rfind(*o).map(|i| (s[..i].trim().to_string(), s[i + 1..].trim().to_string())))
        })
        .filter(|(n, u)| !n.is_empty() && looks_like_unit(u))
        .map_or_else(|| (t.to_string(), None), |(n, u)| (n, Some(u)))
}

fn looks_like_unit(u: &str) -> bool {
    u.contains(['/', '%', '^'])
        || ["fl", "pg", "ratio", "index", "kg", "lb", "lbs", "years", "yrs", "bpm", "ms", "mmhg"]
            .contains(&u.trim_end_matches('.').to_ascii_lowercase().as_str())
}

/// A row turned into one or more measurement inputs, each with its person.
#[derive(Debug, Clone)]
pub struct Planned {
    pub line: usize,
    pub person: Option<String>,
    /// The marker name as written in the source (before `[rename]`).
    pub source: String,
    /// Section header above the row (transposed sheets).
    pub section: Option<String>,
    pub input: Input,
}

pub fn plan_rows(
    rows: &[(usize, RawRow)],
    m: &Mapping,
    extra_tags: &[String],
) -> Result<Vec<std::result::Result<Planned, (usize, String)>>> {
    let headers: Vec<String> =
        rows.iter().flat_map(|(_, r)| r.iter().map(|(h, _)| h.clone())).fold(Vec::new(), |mut acc, h| {
            if !acc.contains(&h) {
                acc.push(h);
            }
            acc
        });
    if let Some((f, c)) = m.columns.iter().find(|(_, c)| !headers.iter().any(|h| h.eq_ignore_ascii_case(c))) {
        return Err(AppError::usage(format!(
            "mapped column '{c}' for field '{f}' not found (columns: {})",
            headers.join(", ")
        )));
    }
    let col = |f: &str| column_for(m, &headers, f).map(str::to_string);
    let cols: HashMap<&str, Option<String>> = FIELDS.iter().map(|f| (*f, col(f))).collect();
    let def = |f: &str| m.defaults.get(f).cloned();
    let field = |row: &RawRow, f: &str| get(row, cols[f].as_deref()).map(str::to_string).or_else(|| def(f));
    let common = |row: &RawRow, line: usize| -> std::result::Result<Input, (usize, String)> {
        let date = match (field(row, "date"), field(row, "time")) {
            (Some(d), Some(t)) if d.len() <= 10 => format!("{d} {t}"),
            (Some(d), _) => d,
            (None, _) => return Err((line, "missing date".into())),
        };
        let fasting =
            field(row, "fasting").map(|f| parse_bool(&f)).transpose().map_err(|e| (line, e.message))?.flatten();
        Ok(Input {
            date,
            lab: field(row, "lab"),
            fasting,
            note: field(row, "note"),
            tags: field(row, "tags")
                .map(|t| parse_tags(&t))
                .unwrap_or_default()
                .into_iter()
                .chain(extra_tags.iter().cloned())
                .collect(),
            ..Input::default()
        })
    };
    let rename = |name: &str| m.rename(name);

    if !m.value_columns.is_empty() {
        let vcols: Vec<(&ValueColumn, &String)> = m
            .value_columns
            .iter()
            .map(|v| {
                headers.iter().find(|h| h.trim().eq_ignore_ascii_case(&v.header)).map(|h| (v, h)).ok_or_else(|| {
                    AppError::usage(format!(
                        "value column '{}' for {} not found (columns: {})",
                        v.header,
                        v.marker,
                        headers.join(", ")
                    ))
                })
            })
            .collect::<Result<_>>()?;
        return Ok(rows
            .iter()
            .flat_map(|(line, row)| {
                let filled: Vec<_> = vcols.iter().filter_map(|(v, h)| get(row, Some(h)).map(|x| (v, h, x))).collect();
                if filled.is_empty() {
                    return vec![];
                }
                let base = match common(row, *line) {
                    Ok(b) => b,
                    Err(e) => return vec![Err(e)],
                };
                let person = field(row, "person");
                filled
                    .into_iter()
                    .map(|(v, h, x)| {
                        Ok(Planned {
                            line: *line,
                            person: person.clone(),
                            source: (*h).clone(),
                            section: None,
                            input: Input {
                                marker: v.marker.clone(),
                                value: x.to_string(),
                                unit: v.unit.clone().or_else(|| split_header_unit(h).1),
                                ..base.clone()
                            },
                        })
                    })
                    .collect()
            })
            .collect());
    }

    if m.wide {
        let reserved: Vec<&str> =
            FIELDS.iter().filter(|f| **f != "marker" && **f != "value").filter_map(|f| cols[f].as_deref()).collect();
        let marker_cols: Vec<&String> = headers.iter().filter(|h| !reserved.contains(&h.as_str())).collect();
        if marker_cols.is_empty() {
            return Err(AppError::invalid("wide import: no marker columns found"));
        }
        return Ok(rows
            .iter()
            .flat_map(|(line, row)| {
                let base = match common(row, *line) {
                    Ok(b) => b,
                    Err(e) => return vec![Err(e)],
                };
                let person = field(row, "person");
                let row_unit = field(row, "unit");
                marker_cols
                    .iter()
                    .filter_map(|h| get(row, Some(h)).map(|v| (h, v.to_string())))
                    .map(|(h, v)| {
                        let (name, unit) = split_header_unit(h);
                        Ok(Planned {
                            line: *line,
                            person: person.clone(),
                            source: name.clone(),
                            section: None,
                            input: Input {
                                marker: rename(&name),
                                value: v,
                                unit: unit.or_else(|| row_unit.clone()),
                                ..base.clone()
                            },
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect());
    }

    if cols["marker"].is_none() && def("marker").is_none() {
        return Err(AppError::invalid(format!(
            "no marker column found (columns: {}); use --map marker=COLUMN or --wide",
            headers.join(", ")
        )));
    }
    if cols["value"].is_none() {
        return Err(AppError::invalid(format!(
            "no value column found (columns: {}); use --map value=COLUMN",
            headers.join(", ")
        )));
    }
    Ok(rows
        .iter()
        .map(|(line, row)| {
            let base = common(row, *line)?;
            let marker = field(row, "marker").ok_or((*line, "missing marker".to_string()))?;
            let value = field(row, "value").ok_or((*line, "missing value".to_string()))?;
            Ok(Planned {
                line: *line,
                person: field(row, "person"),
                input: Input {
                    marker: rename(&marker),
                    value,
                    unit: field(row, "unit"),
                    qualifier: field(row, "qualifier"),
                    ..base
                },
                source: marker,
                section: None,
            })
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Execution
// ---------------------------------------------------------------------------

#[derive(Debug, Default, serde::Serialize)]
pub struct Summary {
    pub batch: Option<String>,
    pub source: String,
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    pub layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<usize>,
    pub dry_run: bool,
    pub dedupe: String,
    pub rows: usize,
    pub measurements: usize,
    pub inserted: usize,
    pub replaced: usize,
    pub skipped: usize,
    pub invalid: usize,
    pub people_created: Vec<String>,
    pub markers_created: Vec<String>,
    /// Source name -> marker, in order of first appearance.
    pub matched: Vec<Value>,
    /// Source names with no marker (skipped), with their number of values.
    pub unmatched: Vec<Value>,
    /// Source names ignored by the mapping's `[skip]`.
    pub skipped_names: Vec<String>,
    /// Non-numeric values: how many, and the values themselves.
    pub qualitative: usize,
    pub qualitative_values: Vec<Value>,
    /// Observations written (`--qualitative store`): inserted / replaced / skipped.
    pub observations: BTreeMap<&'static str, usize>,
    pub date_corrections: Vec<Value>,
    pub cells_per_date: BTreeMap<String, usize>,
    /// Person-specific reference ranges set from the sheet (`ranges = "sheet"`).
    pub ranges_set: Vec<Value>,
    pub warnings: Vec<String>,
    pub errors: Vec<Value>,
}

struct Resolver<'a> {
    db: &'a Db,
    cat: Catalog,
    people: HashMap<String, Person>,
    create_people: bool,
    create_markers: bool,
    /// Unknown names are reported and skipped rather than invalid (spreadsheets).
    lenient: bool,
    people_created: Vec<String>,
    markers_created: Vec<String>,
}

impl Resolver<'_> {
    fn person(&mut self, slug: &str) -> Result<i64> {
        let key = slug.trim().to_lowercase();
        if let Some(p) = self.people.get(&key) {
            return Ok(p.id);
        }
        let p = match (store::find_person(self.db, &key)?, self.create_people) {
            (Some(p), _) => p,
            (None, true) => {
                let p = store::insert_person(
                    self.db,
                    &Person {
                        id: 0,
                        slug: crate::util::validate_slug(&key)?,
                        name: None,
                        sex: None,
                        dob: None,
                        notes: None,
                        tags: vec![],
                        created_at: now_iso(),
                    },
                )?;
                self.people_created.push(p.slug.clone());
                p
            }
            (None, false) => return Err(AppError::not_found(format!("unknown person '{slug}' (use --create-people)"))),
        };
        self.people.insert(key, p.clone());
        Ok(p.id)
    }

    fn find(&self, name: &str) -> Option<Marker> {
        if self.lenient {
            matching::resolve(&self.cat, name).cloned()
        } else {
            self.cat.find(name).cloned()
        }
    }

    /// The marker for `name`; `None` when unknown in lenient mode without --create-markers.
    /// `display` names a created marker (the source spelling, before `[rename]`).
    fn marker(&mut self, name: &str, display: &str, unit: Option<&str>, category: &str) -> Result<Option<Marker>> {
        if let Some(m) = self.find(name) {
            return Ok(Some(m));
        }
        if !self.create_markers {
            return if self.lenient {
                Ok(None)
            } else {
                Err(AppError::not_found(format!(
                    "unknown marker '{name}' (use --create-markers or add a [markers] mapping)"
                )))
            };
        }
        let unit = match unit {
            Some(u) => self.cat.spell_unit(u),
            None if self.lenient => String::new(),
            None => return Err(AppError::invalid(format!("cannot create marker '{name}' without a unit"))),
        };
        let m = Marker {
            id: 0,
            slug: slugify(name),
            name: display.trim().to_string(),
            category: category.to_string(),
            unit,
            loinc: None,
            description: None,
            builtin: false,
            aliases: vec![],
        };
        let id = store::insert_marker(self.db, &m)?;
        if !m.unit.is_empty() {
            store::ensure_unit(self.db, &m.unit, "both")?;
        }
        let m = Marker { id, ..m };
        self.cat.markers.push(m.clone());
        self.markers_created.push(m.slug.clone());
        Ok(Some(m))
    }
}

fn read_input(path: &Path) -> Result<String> {
    if path.as_os_str() == "-" {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        Ok(s)
    } else {
        std::fs::read_to_string(path).map_err(|e| AppError::io(format!("reading {}: {e}", path.display())))
    }
}

/// The numeric part of a spreadsheet value: `5.2`, `<0.5`, and `105 H` /
/// `5.9 High` (a trailing flag is dropped). `None` for qualitative text.
fn numeric_value(v: &str) -> Option<String> {
    let t = v.trim();
    if parse_value(t).is_ok() {
        return Some(t.to_string());
    }
    let (num, flag) = t.rsplit_once(' ')?;
    let flag = flag.trim_matches(|c: char| !c.is_alphanumeric()).to_ascii_lowercase();
    (["h", "l", "high", "low", "a", "abnormal"].contains(&flag.as_str()) && parse_value(num).is_ok())
        .then(|| num.trim().to_string())
}

/// A qualitative result as an observation: `6-10 Abnormal` -> range 6..10, flag abnormal.
fn observation_parts(text: &str) -> (Option<String>, Option<f64>, Option<f64>, Option<String>) {
    let lower = text.to_lowercase();
    let flag = lower.contains("abnormal").then(|| "abnormal".to_string());
    let first = text.split_whitespace().next().unwrap_or("");
    match import_sheet::parse_interval(first) {
        Some((Some(lo), Some(hi))) => (flag, Some(lo), Some(hi), Some(format!("range {first}"))),
        _ => (flag, None, None, None),
    }
}

/// A unit that converts 1:1 into the marker's canonical unit is stored as that
/// unit (`IU/L` -> `U/L`, `uIU/mL` -> `mIU/L` for TSH).
fn collapse_unit(cat: &Catalog, m: &Marker, unit: String) -> String {
    let identity =
        |u: &str| [1.0, 1000.0].iter().all(|v| cat.to_canonical(m, *v, u).is_ok_and(|x| (x - v).abs() < 1e-9));
    if !m.unit.is_empty() && !same_unit(&unit, &m.unit) && identity(&unit) {
        m.unit.clone()
    } else {
        unit
    }
}

/// Category for a marker created from a source row.
fn category_for(m: &Mapping, p: &Planned, sections: bool, known: &[String]) -> String {
    m.category
        .get(&name_key(&p.source))
        .or_else(|| p.section.as_ref().and_then(|s| m.category.get(&name_key(s))))
        .cloned()
        .or_else(|| p.section.as_ref().filter(|_| sections).map(|s| matching::section_category(s, known)))
        .unwrap_or_else(|| "other".into())
}

/// Replace the date part of a stored `taken_at` per the mapping's `[dates]`.
fn correct_date(taken_at: &str, corrections: &HashMap<String, String>) -> Option<String> {
    let (date, rest) = taken_at.split_at(taken_at.len().min(10));
    corrections.get(date).map(|to| format!("{to}{rest}"))
}

/// Excel serial numbers in a date column of an unformatted sheet.
fn fix_serial(date: &str) -> String {
    date.trim()
        .parse::<f64>()
        .ok()
        .and_then(sheet::serial_date)
        .map_or_else(|| date.to_string(), |d| d.format("%Y-%m-%d").to_string())
}

/// Everything planned from the input, plus spreadsheet context.
struct Plan {
    format: String,
    layout: Layout,
    sheet: Option<String>,
    header_row: Option<usize>,
    rows: usize,
    items: Vec<std::result::Result<Planned, (usize, String)>>,
    ranges: Vec<SheetRange>,
    spreadsheet: bool,
}

fn plan(ctx: &Ctx, a: &ImportArgs, mapping: &Mapping, file_delim: Option<String>) -> Result<Plan> {
    let explicit_layout = a.layout.or(mapping.layout).or(mapping.wide.then_some(Layout::Wide));
    let spreadsheet = a.input_format == Some(InputFormat::Spreadsheet)
        || (a.input_format.is_none() && sheet::is_spreadsheet(&a.file));
    if spreadsheet {
        let g = sheet::read(&a.file, a.sheet.as_deref().or(mapping.sheet.as_deref()))?;
        let format = a.file.extension().and_then(|e| e.to_str()).unwrap_or("xlsx").to_ascii_lowercase();
        let layout = explicit_layout.unwrap_or(if import_sheet::looks_transposed(&g) {
            Layout::Transposed
        } else {
            Layout::Long
        });
        if layout == Layout::Transposed {
            let t = import_sheet::plan_transposed(&g, a, mapping, &a.tags)?;
            if let Some((line, name)) = &t.stopped_at {
                eprintln!("biomarker: stopped at line {line} ({name}) per the mapping's stop_at; it and the rows below were not read");
            }
            return Ok(Plan {
                format,
                layout,
                sheet: Some(g.name),
                header_row: Some(t.header_row),
                rows: t.rows,
                items: t.items,
                ranges: t.ranges,
                spreadsheet,
            });
        }
        let header = import_sheet::header_row(&g, a.header_row.or(mapping.header_row), false)?;
        let rows = import_sheet::grid_rows(&g, header);
        let m = Mapping { wide: layout == Layout::Wide, ..mapping.clone() };
        return Ok(Plan {
            format,
            layout,
            sheet: Some(g.name),
            header_row: Some(header + 1),
            rows: rows.len(),
            items: plan_rows(&rows, &m, &a.tags)?,
            ranges: vec![],
            spreadsheet,
        });
    }
    let layout = explicit_layout.unwrap_or(Layout::Long);
    if layout == Layout::Transposed {
        return Err(AppError::usage("--layout transposed needs a spreadsheet (xlsx, xls, ods)"));
    }
    let text = read_input(&a.file)?;
    let fmt = detect_format(&a.file, a.input_format, &text);
    let delim = a
        .input_delimiter
        .clone()
        .or(file_delim)
        .map(|d| crate::config::normalize("csv_delimiter", &d))
        .transpose()?
        .map_or(ctx.out.delimiter, |d| d.as_bytes()[0]);
    let rows = parse_rows(&text, fmt, delim)?;
    let m = Mapping { wide: layout == Layout::Wide, ..mapping.clone() };
    Ok(Plan {
        format: format!("{fmt:?}").to_lowercase(),
        layout,
        sheet: None,
        header_row: None,
        rows: rows.len(),
        items: plan_rows(&rows, &m, &a.tags)?,
        ranges: vec![],
        spreadsheet: false,
    })
}

fn list_sheets(ctx: &Ctx, path: &Path) -> Result<()> {
    if !sheet::is_spreadsheet(path) {
        return Err(AppError::usage(format!("--list-sheets needs a spreadsheet ({})", sheet::EXTENSIONS.join(", "))));
    }
    let rows = sheet::list_sheets(path)?.iter().map(to_record).collect();
    ctx.emit(&Report::list("sheets", rows))
}

pub fn run(ctx: &Ctx, a: ImportArgs) -> Result<()> {
    if a.list_sheets {
        return list_sheets(ctx, &a.file);
    }
    let (mapping, file_delim) = load_mapping(&a)?;
    let plan = plan(ctx, &a, &mapping, file_delim)?;
    let lenient = plan.spreadsheet;
    let qualitative = a.qualitative.or(lenient.then_some(QualitativeArg::Skip));
    let sections = a.sections_as_category.unwrap_or(mapping.sections_as_category.unwrap_or(true));
    let policy_name = a.dedupe.map_or_else(|| ctx.resolved.get("dedupe").to_string(), |d| d.as_str().to_string());
    let policy = Dedupe::parse(&policy_name)?;
    let default_person = ctx.default_person();
    let date_formats: Vec<String> = mapping.date_format.iter().cloned().chain(ctx.input_date_formats()).collect();
    let fmts: Vec<&str> = date_formats.iter().map(String::as_str).collect();
    let corrections: HashMap<String, String> = mapping
        .dates
        .iter()
        .map(|(from, to)| {
            let d = |s: &str| parse_when(s, &fmts, &ctx.tz).map(|w| w.chars().take(10).collect::<String>());
            Ok((d(from)?, d(to)?))
        })
        .collect::<Result<_>>()
        .map_err(|e: AppError| e.context("mapping [dates]"))?;
    let batch = new_batch_id();

    let db = ctx.db()?;
    let mut res = Resolver {
        db: &db,
        cat: Catalog::load(&db)?,
        people: HashMap::new(),
        create_people: a.create_people,
        create_markers: a.create_markers,
        lenient,
        people_created: vec![],
        markers_created: vec![],
    };
    let known_categories: Vec<String> = res.cat.markers.iter().map(|m| m.category.clone()).collect();
    let mut summary = Summary {
        source: a.file.display().to_string(),
        format: plan.format.clone(),
        sheet: plan.sheet.clone(),
        layout: format!("{:?}", plan.layout).to_lowercase(),
        header_row: plan.header_row,
        dry_run: a.dry_run,
        dedupe: policy_name.clone(),
        rows: plan.rows,
        measurements: plan.items.len(),
        ..Summary::default()
    };
    let mut matched: Vec<(String, String)> = vec![];
    let mut unmatched: Vec<(String, usize)> = vec![];
    let mut skipped_names: Vec<String> = vec![];
    let mut applied: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut resolved: HashMap<String, Marker> = HashMap::new();

    db.begin()?;
    let outcome = (|| -> Result<()> {
        for item in &plan.items {
            let p = match item {
                Ok(p) => p,
                Err((line, e)) => {
                    summary.invalid += 1;
                    summary.errors.push(json!({"line": line, "error": e}));
                    continue;
                }
            };
            if mapping.skips(&p.source) || mapping.skips(&p.input.marker) {
                if !skipped_names.contains(&p.source) {
                    skipped_names.push(p.source.clone());
                }
                continue;
            }
            let r = (|| -> Result<Option<Outcome>> {
                let slug = p
                    .person
                    .clone()
                    .or_else(|| default_person.clone())
                    .ok_or_else(|| AppError::usage("row has no person; pass --person or map a person column"))?;
                let pid = res.person(&slug)?;
                let unit = p.input.unit.as_deref().filter(|u| !u.trim().is_empty()).map(|u| mapping.unit(u, lenient));
                let category = category_for(&mapping, p, sections, &known_categories);
                let created_before = res.markers_created.len();
                let Some(marker) = res.marker(&p.input.marker, &p.source, unit.as_deref(), &category)? else {
                    match unmatched.iter_mut().find(|(s, _)| *s == p.source) {
                        Some((_, n)) => *n += 1,
                        None => unmatched.push((p.source.clone(), 1)),
                    }
                    return Ok(None);
                };
                resolved.insert(p.source.clone(), marker.clone());
                if res.markers_created.len() == created_before
                    && !res.markers_created.contains(&marker.slug)
                    && !matched.iter().any(|(s, _)| *s == p.source)
                {
                    matched.push((p.source.clone(), marker.slug.clone()));
                }
                let unit = unit.map(|u| if lenient { collapse_unit(&res.cat, &marker, u) } else { u });
                let date = if lenient { fix_serial(&p.input.date) } else { p.input.date.clone() };
                let value = if lenient { numeric_value(&p.input.value) } else { Some(p.input.value.clone()) };
                let qual = qualitative.filter(|_| value.as_deref().is_none_or(|v| parse_value(v).is_err()));
                let mut taken_at = parse_when(&date, &fmts, &ctx.tz)?;
                if let Some(fixed) = correct_date(&taken_at, &corrections) {
                    *applied.entry((taken_at[..10].to_string(), fixed[..10].to_string())).or_default() += 1;
                    taken_at = fixed;
                }
                let day = taken_at.chars().take(10).collect::<String>();
                if let Some(mode) = qual {
                    let text = p.input.value.trim().to_string();
                    summary.qualitative += 1;
                    summary
                        .qualitative_values
                        .push(json!({"line": p.line, "marker": marker.slug, "date": day, "text": text}));
                    if mode == QualitativeArg::Skip {
                        return Ok(None);
                    }
                    let (flag, range_low, range_high, note) = observation_parts(&text);
                    let o = NewObservation {
                        person_id: pid,
                        marker_id: marker.id,
                        taken_at,
                        text,
                        flag,
                        range_low,
                        range_high,
                        note,
                        lab: p.input.lab.clone().filter(|s| !s.is_empty()),
                        batch_id: Some(batch.clone()),
                    };
                    let out = store::insert_observation(&db, &o, policy).map_err(|e| e.context(&marker.slug))?;
                    *summary.observations.entry(outcome_name(out)).or_default() += 1;
                    *summary.cells_per_date.entry(day).or_default() += 1;
                    return Ok(None);
                }
                let input = Input { value: value.unwrap_or_default(), unit, date: taken_at, ..p.input.clone() };
                let mut nm = build(&res.cat, &marker, pid, &input, &date_formats, &ctx.tz)?;
                nm.batch_id = Some(batch.clone());
                let (out, _) = store::insert_measurement(&db, &nm, policy).map_err(|e| e.context(&marker.slug))?;
                *summary.cells_per_date.entry(day).or_default() += 1;
                Ok(Some(out))
            })();
            match r {
                Ok(None) => {}
                Ok(Some(Outcome::Inserted)) => summary.inserted += 1,
                Ok(Some(Outcome::Replaced)) => summary.replaced += 1,
                Ok(Some(Outcome::Skipped)) => summary.skipped += 1,
                Err(e) if e.kind == ErrorKind::Database => return Err(e.context(format!("line {}", p.line))),
                Err(e) => {
                    summary.invalid += 1;
                    summary.errors.push(json!({"line": p.line, "error": e.message}));
                }
            }
        }
        if mapping.ranges_from_sheet {
            let person = mapping.defaults.get("person").cloned().or_else(|| default_person.clone());
            set_sheet_ranges(&db, &mut res, &plan.ranges, &resolved, person.as_deref(), &mapping, &mut summary)?;
        }
        let written = summary.inserted + summary.replaced + summary.observations.values().sum::<usize>();
        if written > 0 {
            store::record_batch(&db, &batch, &summary.source, &summary.format, written)?;
            summary.batch = Some(batch.clone());
        }
        Ok(())
    })();
    summary.matched = matched.iter().map(|(s, m)| json!({"source": s, "marker": m})).collect();
    summary.unmatched = unmatched.iter().map(|(s, n)| json!({"source": s, "cells": n})).collect();
    summary.skipped_names = skipped_names;
    summary.date_corrections = applied.iter().map(|((f, t), n)| json!({"from": f, "to": t, "cells": n})).collect();
    summary.people_created = res.people_created.clone();
    summary.markers_created = res.markers_created.clone();
    let failed = summary.invalid > 0 && !a.skip_invalid;
    match (&outcome, a.dry_run || failed) {
        (Ok(()), false) => db.commit()?,
        _ => {
            db.rollback()?;
            // nothing was written: no batch exists
            summary.batch = None;
            if failed {
                summary.people_created.clear();
                summary.markers_created.clear();
            }
        }
    }
    outcome?;

    if failed {
        ctx.emit_mutation(&Report::object("import", to_record(&summary)))?;
        let first: Vec<String> = summary
            .errors
            .iter()
            .take(5)
            .map(|e| format!("line {}: {}", e["line"], e["error"].as_str().unwrap_or("")))
            .collect();
        return Err(AppError::invalid(format!(
            "import aborted: {} invalid row(s); nothing written (use --skip-invalid to import the rest)\n  {}",
            summary.invalid,
            first.join("\n  ")
        )));
    }
    if lenient || a.dry_run {
        ctx.info(render_report(&summary, qualitative).trim_end());
    } else {
        summary.errors.iter().take(20).for_each(|e| {
            ctx.info(&format!("skipped line {}: {}", e["line"], e["error"].as_str().unwrap_or("")));
        });
    }
    ctx.info(&format!(
        "{}{} inserted, {} replaced, {} skipped, {} invalid ({} rows)",
        if a.dry_run { "dry run: " } else { "" },
        summary.inserted,
        summary.replaced,
        summary.skipped,
        summary.invalid,
        summary.rows
    ));
    ctx.emit_mutation(&Report::object("import", to_record(&summary)))
}

fn outcome_name(o: Outcome) -> &'static str {
    match o {
        Outcome::Inserted => "inserted",
        Outcome::Replaced => "replaced",
        Outcome::Skipped => "skipped",
    }
}

/// `ranges = "sheet"`: the sheet's ref low/high become the import person's
/// reference ranges (as `range set --person`), converted to canonical units.
fn set_sheet_ranges(
    db: &Db,
    res: &mut Resolver,
    ranges: &[SheetRange],
    resolved: &HashMap<String, Marker>,
    person: Option<&str>,
    mapping: &Mapping,
    summary: &mut Summary,
) -> Result<()> {
    if ranges.is_empty() {
        return Ok(());
    }
    let slug = person.ok_or_else(|| AppError::usage("ranges = \"sheet\" needs the import's person (--person)"))?;
    let pid = res.person(slug)?;
    for r in ranges.iter().filter(|r| !mapping.skips(&r.source)) {
        let Some(m) = resolved.get(&r.source).cloned().or_else(|| res.find(&r.marker)) else { continue };
        let unit = r.unit.as_deref().map_or_else(|| m.unit.clone(), |u| mapping.unit(u, res.lenient));
        let conv = |v: Option<f64>| v.map(|x| res.cat.to_canonical(&m, x, &unit)).transpose();
        let (low, high) = match (conv(r.low), conv(r.high)) {
            (Ok(l), Ok(h)) => (l, h),
            (Err(e), _) | (_, Err(e)) => {
                summary.warnings.push(format!("line {}: range for {} not set: {}", r.line, m.slug, e.message));
                continue;
            }
        };
        let range = Range {
            id: 0,
            marker_id: m.id,
            kind: RangeKind::Reference,
            sex: "any".into(),
            age_min: 0.0,
            age_max: 200.0,
            low,
            high,
            note: Some("from spreadsheet".into()),
            person_id: Some(pid),
            critical_low: None,
            critical_high: None,
        };
        store::upsert_person_range(db, pid, &range)?;
        summary.ranges_set.push(json!({"marker": m.slug, "person": slug, "low": low, "high": high, "unit": m.unit}));
    }
    Ok(())
}

fn str_of(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn report_header(o: &mut String, s: &Summary) {
    use std::fmt::Write;
    let _ = write!(o, "import {} ({}", s.source, s.format);
    if let Some(sh) = &s.sheet {
        let _ = write!(o, ", sheet \"{sh}\"");
    }
    let _ = write!(o, ", {} layout", s.layout);
    if let Some(h) = s.header_row {
        let _ = write!(o, ", header row {h}");
    }
    let _ = writeln!(o, ")");
}

fn report_matching(o: &mut String, s: &Summary) {
    use std::fmt::Write;
    let w = s.matched.iter().map(|m| str_of(&m["source"]).chars().count()).max().unwrap_or(0);
    let _ = writeln!(o, "matched ({}):", s.matched.len());
    for m in &s.matched {
        let src = str_of(&m["source"]);
        let _ = writeln!(o, "  {src}{} -> {}", " ".repeat(w - src.chars().count()), str_of(&m["marker"]));
    }
    if !s.markers_created.is_empty() {
        let _ = writeln!(o, "created ({}): {}", s.markers_created.len(), s.markers_created.join(", "));
    }
    if !s.unmatched.is_empty() {
        let _ = writeln!(o, "unmatched ({}, skipped; use --create-markers or a [rename] entry):", s.unmatched.len());
        for u in &s.unmatched {
            let n = u["cells"].as_u64().unwrap_or(0);
            let _ = writeln!(o, "  {} ({n} value{})", str_of(&u["source"]), if n == 1 { "" } else { "s" });
        }
    }
    if !s.skipped_names.is_empty() {
        let _ = writeln!(o, "skipped by mapping ({}): {}", s.skipped_names.len(), s.skipped_names.join(", "));
    }
}

fn report_qualitative(o: &mut String, s: &Summary, qualitative: Option<QualitativeArg>) {
    use std::fmt::Write;
    if s.qualitative == 0 {
        return;
    }
    let how = match qualitative {
        Some(QualitativeArg::Store) => "stored as observations",
        _ => "skipped; use --qualitative store to keep them",
    };
    let _ = writeln!(o, "qualitative ({}, {how}):", s.qualitative);
    for q in &s.qualitative_values {
        let _ = writeln!(o, "  {}  {}: {}", str_of(&q["date"]), str_of(&q["marker"]), str_of(&q["text"]));
    }
}

fn report_corrections(o: &mut String, s: &Summary) {
    use std::fmt::Write;
    if !s.date_corrections.is_empty() {
        let _ = writeln!(o, "date corrections ({}):", s.date_corrections.len());
        for d in &s.date_corrections {
            let _ = writeln!(o, "  {} -> {} ({} values)", str_of(&d["from"]), str_of(&d["to"]), d["cells"]);
        }
    }
    if !s.ranges_set.is_empty() {
        let b =
            |v: &Value| v.as_f64().map_or_else(String::new, |x| crate::sheet::fmt_number(crate::util::round_to(x, 4)));
        let _ = writeln!(o, "reference ranges from the sheet ({}):", s.ranges_set.len());
        for r in &s.ranges_set {
            let _ =
                writeln!(o, "  {}: {}..{} {}", str_of(&r["marker"]), b(&r["low"]), b(&r["high"]), str_of(&r["unit"]));
        }
    }
}

fn report_problems_and_dates(o: &mut String, s: &Summary) {
    use std::fmt::Write;
    for w in &s.warnings {
        let _ = writeln!(o, "warning: {w}");
    }
    for e in s.errors.iter().take(20) {
        let _ = writeln!(o, "invalid: line {}: {}", e["line"], str_of(&e["error"]));
    }
    if !s.cells_per_date.is_empty() {
        let _ = writeln!(o, "values per date:");
        for (d, n) in &s.cells_per_date {
            let _ = writeln!(o, "  {d}  {n}");
        }
    }
}

/// Human-readable import report (stderr): what matched, what did not, and why.
pub fn render_report(s: &Summary, qualitative: Option<QualitativeArg>) -> String {
    let mut o = String::new();
    report_header(&mut o, s);
    report_matching(&mut o, s);
    report_qualitative(&mut o, s, qualitative);
    report_corrections(&mut o, s);
    report_problems_and_dates(&mut o, s);
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_units() {
        assert_eq!(split_header_unit("Glucose (mmol/L)"), ("Glucose".into(), Some("mmol/L".into())));
        assert_eq!(split_header_unit("ldl-c [mg/dL]"), ("ldl-c".into(), Some("mg/dL".into())));
        assert_eq!(split_header_unit("hba1c (%)"), ("hba1c".into(), Some("%".into())));
        assert_eq!(split_header_unit("Lp(a)"), ("Lp(a)".into(), None));
        assert_eq!(split_header_unit("Lp(a) (nmol/L)"), ("Lp(a)".into(), Some("nmol/L".into())));
    }

    #[test]
    fn parses_csv_and_json() {
        let csv = "person,marker,value,unit,date\nalice,glucose,90,mg/dL,2024-01-01\n";
        let rows = parse_rows(csv, InputFormat::Csv, b',').unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].1[2], ("value".into(), "90".into()));
        let json = r#"{"schema":"biomarker/v1","data":[{"marker":"ldl-c","value":101.5,"tags":["a","b"]}]}"#;
        let rows = parse_rows(json, InputFormat::Json, b',').unwrap();
        assert_eq!(rows[0].1[1], ("value".into(), "101.5".into()));
        assert_eq!(rows[0].1[2], ("tags".into(), "a,b".into()));
        let jsonl = "{\"marker\":\"tsh\",\"value\":2}\n\n{\"marker\":\"tsh\",\"value\":3}\n";
        assert_eq!(parse_rows(jsonl, InputFormat::Jsonl, b',').unwrap().len(), 2);
    }

    #[test]
    fn plans_long_and_wide() {
        let rows = parse_rows("Test,Result,Collected\nLDL,120,2024-02-02\n", InputFormat::Csv, b',').unwrap();
        let m = Mapping { columns: [("marker".to_string(), "Test".to_string())].into(), ..Mapping::default() };
        let p = plan_rows(&rows, &m, &[]).unwrap();
        let p = p[0].as_ref().unwrap();
        assert_eq!(
            (p.input.marker.as_str(), p.input.value.as_str(), p.input.date.as_str()),
            ("LDL", "120", "2024-02-02")
        );

        let rows = parse_rows("date,glucose (mmol/L),hba1c\n2024-01-01,5.1,\n", InputFormat::Csv, b',').unwrap();
        let m = Mapping { wide: true, ..Mapping::default() };
        let p = plan_rows(&rows, &m, &["x".into()]).unwrap();
        assert_eq!(p.len(), 1);
        let p = p[0].as_ref().unwrap();
        assert_eq!(p.input.unit.as_deref(), Some("mmol/L"));
        assert_eq!(p.input.tags, vec!["x"]);

        // a unit column applies to every marker column; qualifier/unit are not markers
        let rows = parse_rows("date,unit,glucose,qualifier\n2024-01-01,mmol/L,5.1,\n", InputFormat::Csv, b',').unwrap();
        let p = plan_rows(&rows, &Mapping { wide: true, ..Mapping::default() }, &[]).unwrap();
        assert_eq!(p.len(), 1);
        let p = p[0].as_ref().unwrap();
        assert_eq!((p.input.marker.as_str(), p.input.unit.as_deref()), ("glucose", Some("mmol/L")));

        // a row without a date yields one error, not one per marker
        let rows = parse_rows("date,glucose,ldl\n,5,100\n", InputFormat::Csv, b',').unwrap();
        assert_eq!(plan_rows(&rows, &Mapping { wide: true, ..Mapping::default() }, &[]).unwrap().len(), 1);

        // an explicit mapping to a missing column is an error
        let rows = parse_rows("marker,value,date\nldl,1,2024-01-01\n", InputFormat::Csv, b',').unwrap();
        let m = Mapping { columns: [("value".to_string(), "Wert".to_string())].into(), ..Mapping::default() };
        assert!(plan_rows(&rows, &m, &[]).is_err());
    }
}
