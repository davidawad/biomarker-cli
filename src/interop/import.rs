//! `biomarker import` for FHIR R4 (JSON, Bundles, NDJSON, directories) and
//! HL7 v2 ORU^R01 messages: format detection, input reading, person check,
//! the transaction, and the summary.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::cli::{ImportArgs, InputFormat};
use crate::commands::import::load_mapping;
use crate::context::Ctx;
use crate::db::Db;
use crate::error::{AppError, Result};
use crate::interop::candidate::{Parsed, PatientHint};
use crate::interop::pipeline::{self, Options, Summary, Unmapped};
use crate::interop::{fhir, hl7v2};
use crate::output::{to_record, Report};
use crate::store::{self, Dedupe, Person};
use crate::util::{new_batch_id, now_iso};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Fhir,
    FhirNdjson,
    Hl7v2,
}

impl Source {
    pub fn format(self) -> &'static str {
        match self {
            Self::Fhir => "fhir",
            Self::FhirNdjson => "fhir-ndjson",
            Self::Hl7v2 => "hl7v2",
        }
    }
}

const HL7_EXTENSIONS: &[&str] = &["hl7", "oru", "hl7v2"];
const FHIR_EXTENSIONS: &[&str] = &["json", "ndjson", "jsonl", "fhir"];

fn ext_of(p: &Path) -> String {
    p.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default()
}

/// Does this import read FHIR or HL7 v2 rather than a table? Decided by
/// `--input-format`, else by extension, else by looking at the file's start.
pub fn detect(a: &ImportArgs) -> Result<Option<Source>> {
    match a.input_format {
        Some(InputFormat::Fhir) => return Ok(Some(Source::Fhir)),
        Some(InputFormat::FhirNdjson) => return Ok(Some(Source::FhirNdjson)),
        Some(InputFormat::Hl7v2) => return Ok(Some(Source::Hl7v2)),
        Some(_) => return Ok(None),
        None => {}
    }
    if a.file.as_os_str() == "-" {
        return Ok(None);
    }
    if a.file.is_dir() {
        let any_hl7 = std::fs::read_dir(&a.file)
            .map(|d| d.flatten().any(|e| HL7_EXTENSIONS.contains(&ext_of(&e.path()).as_str())))
            .unwrap_or(false);
        return Ok(Some(if any_hl7 { Source::Hl7v2 } else { Source::Fhir }));
    }
    let ext = ext_of(&a.file);
    if HL7_EXTENSIONS.contains(&ext.as_str()) {
        return Ok(Some(Source::Hl7v2));
    }
    if ext == "fhir" {
        return Ok(Some(Source::Fhir));
    }
    if !(FHIR_EXTENSIONS.contains(&ext.as_str()) || ext.is_empty() || ext == "txt") {
        return Ok(None);
    }
    Ok(sniff(&a.file, &ext))
}

/// Look at the start of a file: `MSH|` is HL7 v2; a `resourceType` is FHIR.
fn sniff(path: &Path, ext: &str) -> Option<Source> {
    let mut head = vec![0u8; 64 * 1024];
    let n = std::fs::File::open(path).and_then(|mut f| f.read(&mut head)).ok()?;
    let text = String::from_utf8_lossy(&head[..n]);
    let t = text.trim_start_matches('\u{feff}').trim_start();
    if t.starts_with("MSH") {
        Some(Source::Hl7v2)
    } else if t.contains("\"resourceType\"") {
        Some(if matches!(ext, "ndjson" | "jsonl") { Source::FhirNdjson } else { Source::Fhir })
    } else {
        None
    }
}

/// The files to read: the file itself, or a directory's matching files in name order.
fn input_files(path: &Path, src: Source) -> Result<Vec<PathBuf>> {
    if !path.is_dir() {
        return Ok(vec![path.to_path_buf()]);
    }
    let wanted: &[&str] = if src == Source::Hl7v2 { &["hl7", "oru", "hl7v2", "txt"] } else { FHIR_EXTENSIONS };
    let mut files: Vec<PathBuf> = std::fs::read_dir(path)
        .map_err(|e| AppError::io(format!("reading {}: {e}", path.display())))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && wanted.contains(&ext_of(p).as_str()))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(AppError::invalid(format!("no {} files in {}", src.format(), path.display())));
    }
    Ok(files)
}

fn read_text(path: &Path) -> Result<String> {
    if path.as_os_str() == "-" {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        Ok(s)
    } else {
        std::fs::read_to_string(path).map_err(|e| AppError::io(format!("reading {}: {e}", path.display())))
    }
}

fn read_parsed(a: &ImportArgs, src: Source) -> Result<Parsed> {
    let files = input_files(&a.file, src)?;
    let origin = a.file.file_name().map_or_else(|| "input".to_string(), |n| n.to_string_lossy().into_owned());
    if src == Source::Hl7v2 {
        let mut all = Parsed::default();
        for f in &files {
            all.merge(hl7v2::parse(&read_text(f)?).map_err(|e| e.context(f.display().to_string()))?);
        }
        return Ok(all);
    }
    let mut resources = vec![];
    for f in &files {
        fhir::read_resources(&read_text(f)?, &mut resources).map_err(|e| e.context(f.display().to_string()))?;
    }
    Ok(fhir::parse(&resources, &origin))
}

// ---------------------------------------------------------------------------
// Person
// ---------------------------------------------------------------------------

fn same_day(a: &str, b: &str) -> bool {
    let n = a.len().min(b.len()).min(10);
    n >= 4 && a[..n] == b[..n]
}

/// Where the file's Patient disagrees with the person results are going to.
fn patient_mismatches(p: &Person, hints: &[PatientHint]) -> Vec<String> {
    let known_sex = |s: &str| matches!(s, "male" | "female");
    hints
        .iter()
        .flat_map(|h| {
            let sex = match (&p.sex, &h.gender) {
                (Some(ps), Some(hs)) if known_sex(&ps.to_lowercase()) && known_sex(hs) && ps.to_lowercase() != *hs => {
                    Some(format!("sex {hs} in the file, {ps} on {}", p.slug))
                }
                _ => None,
            };
            let dob = match (&p.dob, &h.birth_date) {
                (Some(pd), Some(hd)) if !same_day(pd, hd) => {
                    Some(format!("birth date {hd} in the file, {pd} on {}", p.slug))
                }
                _ => None,
            };
            [sex, dob].into_iter().flatten()
        })
        .collect()
}

fn resolve_person(ctx: &Ctx, db: &Db, a: &ImportArgs, parsed: &Parsed) -> Result<Person> {
    let sole = || -> Result<Option<String>> {
        let people = store::list_people(db)?;
        Ok(match people.as_slice() {
            [only] => Some(only.slug.clone()),
            _ => None,
        })
    };
    let slug = match a.person.clone().or_else(|| ctx.default_person()) {
        Some(s) => s,
        None => sole()?.ok_or_else(|| {
            AppError::usage("which person? pass --person (or set default_person); results from a file are never assigned by guessing")
        })?,
    };
    let slug = slug.trim().to_lowercase();
    match store::find_person(db, &slug)? {
        Some(p) => Ok(p),
        None if a.create_people => {
            let hint = (parsed.patients.len() == 1).then(|| &parsed.patients[0]);
            store::insert_person(
                db,
                &Person {
                    id: 0,
                    slug: crate::util::validate_slug(&slug)?,
                    name: None,
                    sex: hint.and_then(|h| h.gender.clone()).filter(|g| matches!(g.as_str(), "male" | "female")),
                    dob: hint.and_then(|h| h.birth_date.clone()),
                    notes: None,
                    tags: vec![],
                    created_at: now_iso(),
                    attributes: Default::default(),
                },
            )
        }
        None => Err(AppError::not_found(format!("unknown person '{slug}' (use --create-people)"))),
    }
}

// ---------------------------------------------------------------------------

pub fn run(ctx: &Ctx, a: ImportArgs, src: Source) -> Result<()> {
    let parsed = read_parsed(&a, src)?;
    let (mapping, _) = load_mapping(&a)?;
    let policy_name = a.dedupe.map_or_else(|| ctx.resolved.get("dedupe").to_string(), |d| d.as_str().to_string());
    let policy = Dedupe::parse(&policy_name)?;
    let date_formats: Vec<String> = mapping.date_format.iter().cloned().chain(ctx.input_date_formats()).collect();
    let categories: Vec<String> = if a.categories.is_empty() {
        vec!["laboratory".into(), "vital-signs".into()]
    } else {
        a.categories.iter().map(|c| c.trim().to_ascii_lowercase()).collect()
    };
    let batch = new_batch_id();
    let db = ctx.db()?;
    let mut summary = Summary::new(a.file.display().to_string(), src.format(), a.dry_run, policy_name);

    db.begin()?;
    let outcome = (|| -> Result<()> {
        let person = resolve_person(ctx, &db, &a, &parsed)?;
        summary.person = person.slug.clone();
        let mismatches = patient_mismatches(&person, &parsed.patients);
        if !mismatches.is_empty() {
            if !a.force_person {
                return Err(AppError::invalid(format!(
                    "the file's Patient does not match person '{}': {} (use --force-person to import anyway)",
                    person.slug,
                    mismatches.join("; ")
                )));
            }
            mismatches
                .iter()
                .for_each(|m| summary.warnings.push(format!("Patient mismatch ignored (--force-person): {m}")));
        }
        if parsed.patients.len() > 1 {
            summary.warnings.push(format!(
                "the input has {} Patient resources; all results go to {}",
                parsed.patients.len(),
                person.slug
            ));
        }
        let opts = Options {
            person: &person,
            categories,
            include_preliminary: a.include_preliminary,
            apply_retractions: a.apply_retractions,
            unmapped: a.unmapped.unwrap_or(Unmapped::Report),
            dedupe: policy,
            link_existing: a.link_existing,
            lab: a.lab.clone(),
            tags: a.tags.clone(),
            mapping: &mapping,
            date_formats,
            tz: &ctx.tz,
            format: src.format().into(),
            batch: batch.clone(),
        };
        pipeline::apply(&db, &parsed, &opts, &mut summary)?;
        let written = summary.inserted
            + summary.replaced
            + ["inserted", "replaced"].iter().map(|k| summary.observations.get(k).copied().unwrap_or(0)).sum::<usize>();
        if written > 0 {
            store::record_batch(&db, &batch, &summary.source, src.format(), written)?;
            summary.batch = Some(batch.clone());
        }
        Ok(())
    })();
    match (&outcome, a.dry_run) {
        (Ok(()), false) => db.commit()?,
        _ => {
            db.rollback()?;
            summary.batch = None;
        }
    }
    outcome?;

    print_summary(ctx, &summary, a.dry_run);
    ctx.emit_mutation(&Report::object("import", to_record(&summary)))
}

fn print_summary(ctx: &Ctx, s: &Summary, dry_run: bool) {
    let obs = |k: &str| s.observations.get(k).copied().unwrap_or(0);
    let list = |title: &str, items: &[serde_json::Value]| {
        items.iter().take(10).for_each(|i| {
            ctx.info(&format!(
                "  {title}: {} ({} result(s)): {}",
                i["result"].as_str().unwrap_or(""),
                i["results"],
                i["reason"].as_str().unwrap_or("")
            ));
        });
    };
    list("unmapped", &s.unmapped_results);
    list("unconvertible", &s.unconvertible_results);
    s.conflict_results.iter().take(10).for_each(|c| {
        ctx.info(&format!(
            "  conflict: {} {} at {} (first result wins)",
            c["result"].as_str().unwrap_or(""),
            c["marker"].as_str().unwrap_or(""),
            c["taken_at"].as_str().unwrap_or("")
        ));
    });
    s.errors.iter().take(10).for_each(|e| {
        ctx.info(&format!("  invalid: {}: {}", e["result"].as_str().unwrap_or(""), e["error"].as_str().unwrap_or("")))
    });
    s.warnings.iter().take(10).for_each(|w| ctx.info(&format!("  warning: {w}")));
    if s.ignored_resources > 0 {
        ctx.info(&format!(
            "  ignored resources: {}",
            s.ignored.iter().map(|(k, n)| format!("{k} x{n}")).collect::<Vec<_>>().join(", ")
        ));
    }
    ctx.info(&format!(
        "{}{} inserted, {} replaced, {} skipped, {} retracted, {} unmapped, {} unconvertible, {} conflict(s), {} invalid{} ({} results)",
        if dry_run { "dry run: " } else { "" },
        s.inserted,
        s.replaced,
        s.skipped,
        s.retracted,
        s.unmapped,
        s.unconvertible,
        s.conflicts,
        s.invalid,
        if s.observations.is_empty() {
            String::new()
        } else {
            format!("; qualitative: {} inserted, {} replaced, {} skipped", obs("inserted"), obs("replaced"), obs("skipped"))
        },
        s.results
    ));
}
