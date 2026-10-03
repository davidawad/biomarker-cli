//! `biomarker import`: CSV / TSV / JSON / JSONL, long or wide layout, with
//! configurable column mapping, dedupe policy and dry-run.
//!
//! The whole import runs in one transaction; `--dry-run` simply rolls it back,
//! so the reported counts are exactly what a real run would do.

use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use std::path::Path;

use serde_json::{json, Value};

use crate::cli::{ImportArgs, InputFormat};
use crate::commands::measure::{build, Input};
use crate::context::Ctx;
use crate::db::Db;
use crate::error::{AppError, ErrorKind, Result};
use crate::output::{to_record, Report};
use crate::store::{self, Catalog, Dedupe, Marker, Outcome, Person};
use crate::util::{new_batch_id, now_iso, parse_bool, parse_tags, slugify};

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
    pub wide: Option<bool>,
    pub date_format: Option<String>,
    pub delimiter: Option<String>,
}

/// Effective mapping after merging the file and flags.
#[derive(Debug, Default, Clone)]
pub struct Mapping {
    pub columns: BTreeMap<String, String>,
    pub defaults: BTreeMap<String, String>,
    pub markers: HashMap<String, String>,
    pub wide: bool,
    pub date_format: Option<String>,
}

fn toml_text(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
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
    Ok((
        Mapping {
            columns,
            defaults,
            markers: file.markers.into_iter().map(|(k, v)| (k.trim().to_lowercase(), v)).collect(),
            wide: a.wide || file.wide.unwrap_or(false),
            date_format: a.input_date_format.clone().or(file.date_format),
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
    u.contains(['/', '%', '^']) || ["fl", "pg", "ratio", "index"].contains(&u.to_ascii_lowercase().as_str())
}

/// A row turned into one or more measurement inputs, each with its person.
#[derive(Debug, Clone)]
pub struct Planned {
    pub line: usize,
    pub person: Option<String>,
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
    let rename =
        |name: &str| m.markers.get(&name.trim().to_lowercase()).cloned().unwrap_or_else(|| name.trim().to_string());

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
    pub errors: Vec<Value>,
}

struct Resolver<'a> {
    db: &'a Db,
    cat: Catalog,
    people: HashMap<String, Person>,
    create_people: bool,
    create_markers: bool,
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

    fn marker(&mut self, name: &str, unit: Option<&str>) -> Result<Marker> {
        if let Some(m) = self.cat.find(name) {
            return Ok(m.clone());
        }
        if !self.create_markers {
            return Err(AppError::not_found(format!(
                "unknown marker '{name}' (use --create-markers or add a [markers] mapping)"
            )));
        }
        let unit = unit.ok_or_else(|| AppError::invalid(format!("cannot create marker '{name}' without a unit")))?;
        let m = Marker {
            id: 0,
            slug: slugify(name),
            name: name.trim().to_string(),
            category: "other".into(),
            unit: self.cat.spell_unit(unit),
            loinc: None,
            description: None,
            builtin: false,
            aliases: vec![],
        };
        let id = store::insert_marker(self.db, &m)?;
        store::ensure_unit(self.db, &m.unit, "both")?;
        let m = Marker { id, ..m };
        self.cat.markers.push(m.clone());
        self.markers_created.push(m.slug.clone());
        Ok(m)
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

pub fn run(ctx: &Ctx, a: ImportArgs) -> Result<()> {
    let (mapping, file_delim) = load_mapping(&a)?;
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
    let planned = plan_rows(&rows, &mapping, &a.tags)?;
    let policy_name = a.dedupe.map_or_else(|| ctx.resolved.get("dedupe").to_string(), |d| d.as_str().to_string());
    let policy = Dedupe::parse(&policy_name)?;
    let default_person = ctx.default_person();
    let date_formats: Vec<String> = mapping.date_format.iter().cloned().chain(ctx.input_date_formats()).collect();
    let batch = new_batch_id();

    let db = ctx.db()?;
    let mut res = Resolver {
        db: &db,
        cat: Catalog::load(&db)?,
        people: HashMap::new(),
        create_people: a.create_people,
        create_markers: a.create_markers,
        people_created: vec![],
        markers_created: vec![],
    };
    let mut summary = Summary {
        source: a.file.display().to_string(),
        format: format!("{fmt:?}").to_lowercase(),
        dry_run: a.dry_run,
        dedupe: policy_name.clone(),
        rows: rows.len(),
        measurements: planned.len(),
        ..Summary::default()
    };

    db.begin()?;
    let outcome = (|| -> Result<()> {
        for p in &planned {
            let result = p.clone().map_err(|(line, e)| (line, AppError::invalid(e))).and_then(|p| {
                let r = (|| {
                    let slug =
                        p.person.clone().or_else(|| default_person.clone()).ok_or_else(|| {
                            AppError::usage("row has no person; pass --person or map a person column")
                        })?;
                    let pid = res.person(&slug)?;
                    let marker = res.marker(&p.input.marker, p.input.unit.as_deref())?;
                    let mut m = build(&res.cat, &marker, pid, &p.input, &date_formats, &ctx.tz)?;
                    m.batch_id = Some(batch.clone());
                    store::insert_measurement(&db, &m, policy).map(|(o, _)| o).map_err(|e| e.context(&marker.slug))
                })();
                r.map_err(|e| (p.line, e))
            });
            match result {
                Ok(Outcome::Inserted) => summary.inserted += 1,
                Ok(Outcome::Replaced) => summary.replaced += 1,
                Ok(Outcome::Skipped) => summary.skipped += 1,
                Err((line, e)) if e.kind == ErrorKind::Database => return Err(e.context(format!("line {line}"))),
                Err((line, e)) => {
                    summary.invalid += 1;
                    summary.errors.push(json!({"line": line, "error": e.message}));
                }
            }
        }
        if summary.inserted + summary.replaced > 0 {
            store::record_batch(&db, &batch, &summary.source, &summary.format, summary.inserted + summary.replaced)?;
            summary.batch = Some(batch.clone());
        }
        Ok(())
    })();
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
    summary.errors.iter().take(20).for_each(|e| {
        ctx.info(&format!("skipped line {}: {}", e["line"], e["error"].as_str().unwrap_or("")));
    });
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
