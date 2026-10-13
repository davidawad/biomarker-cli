//! `intervention import`: supplement-interactions/1 documents into the catalog
//! and interaction edges (docs/interventions.md section 5).

use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::{json, Value};

use crate::cli_interventions::InterventionImportArgs;
use crate::context::Ctx;
use crate::db::Db;
use crate::error::{AppError, Result};
use crate::interventions::doc::{self, Doc, Subject};
use crate::interventions::interaction::{self, NewEdge};
use crate::interventions::{self as iv, Intervention, IvCatalog};
use crate::output::{to_record, Report};
use crate::store::Outcome;

#[derive(Debug, Default, Serialize)]
struct Summary {
    dry_run: bool,
    files: usize,
    interventions_created: Vec<String>,
    interventions_updated: Vec<String>,
    interactions_inserted: usize,
    interactions_updated: usize,
    interactions_unchanged: usize,
    invalid: usize,
    errors: Vec<Value>,
    /// Intervention targets of the imported edges that are not in the catalog
    /// after the import. Their edges are stored and resolve once the target is added.
    unresolved_targets: Vec<String>,
    warnings: Vec<String>,
    #[serde(skip)]
    iv_targets: Vec<String>,
}

/// Files to read: plain files as given; for a directory, `interactions.json`
/// in it and in each subdirectory (sorted).
fn expand(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for p in paths {
        if !p.is_dir() {
            out.push(p.clone());
            continue;
        }
        let read = std::fs::read_dir(p).map_err(|e| AppError::io(format!("reading {}: {e}", p.display())))?;
        let mut found: Vec<PathBuf> = read
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|d| d.is_dir())
            .map(|d| d.join("interactions.json"))
            .chain(std::iter::once(p.join("interactions.json")))
            .filter(|f| f.is_file())
            .collect();
        found.sort();
        if found.is_empty() {
            return Err(AppError::not_found(format!("no */interactions.json under {}", p.display())));
        }
        out.extend(found);
    }
    Ok(out)
}

/// `<slug>/interactions.json` names its subject by the directory.
fn dir_slug(p: &Path) -> Option<String> {
    (p.file_name()? == "interactions.json").then(|| p.parent()?.file_name()?.to_str().map(str::to_string)).flatten()
}

fn read(p: &Path) -> Result<Vec<Doc>> {
    let text = std::fs::read_to_string(p).map_err(|e| AppError::io(format!("reading {}: {e}", p.display())))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| AppError::invalid(format!("{}: {e}", p.display())))?;
    doc::parse(&v, dir_slug(p).as_deref()).map_err(|e| e.context(p.display().to_string()))
}

/// Create the subject, or fill the empty fields of an existing one. Returns its id.
fn upsert_subject(db: &Db, s: &Subject, source: &str, sum: &mut Summary) -> Result<i64> {
    let ivs = IvCatalog::load(db)?;
    let id = match ivs.items.iter().find(|i| i.slug == s.slug) {
        None => {
            let item = Intervention {
                name: s.name.clone().unwrap_or_else(|| s.slug.clone()),
                category: s.category.clone().unwrap_or_else(|| "supplement".into()),
                rx_status: s.rx_status.clone().unwrap_or_else(|| "unknown".into()),
                ingredients: s.ingredients.clone(),
                classes: s.classes.clone(),
                description: s.description.clone(),
                source: Some(source.to_string()),
                ..Intervention::new(&s.slug)
            };
            let id = iv::insert(db, &item)?;
            sum.interventions_created.push(s.slug.clone());
            id
        }
        Some(cur) => {
            let mut next = cur.clone();
            if next.name == next.slug {
                next.name = s.name.clone().unwrap_or(next.name);
            }
            if next.rx_status == "unknown" {
                next.rx_status = s.rx_status.clone().unwrap_or(next.rx_status);
            }
            if next.ingredients.is_empty() {
                next.ingredients = s.ingredients.clone();
            }
            if next.classes.is_empty() {
                next.classes = s.classes.clone();
            }
            next.description = next.description.or_else(|| s.description.clone());
            if &next != cur {
                iv::update(db, &next)?;
                sum.interventions_updated.push(s.slug.clone());
            }
            cur.id
        }
    };
    for a in &s.aliases {
        if let Err(e) = iv::add_alias(db, id, a) {
            sum.warnings.push(format!("{}: {}", s.slug, e.message));
        }
    }
    Ok(id)
}

fn apply(db: &Db, file: &Path, d: &Doc, sum: &mut Summary) -> Result<()> {
    let source = file.display().to_string();
    sum.warnings.extend(d.warnings.iter().map(|w| format!("{source}: {w}")));
    let a_id = upsert_subject(db, &d.subject, &source, sum)?;
    for (index, e) in &d.edges {
        let spec = match e {
            Ok(s) => s,
            Err(msg) => {
                sum.invalid += 1;
                sum.errors.push(json!({"file": source, "supplement": d.subject.slug, "index": index, "error": msg}));
                continue;
            }
        };
        if spec.target_kind == "intervention" {
            sum.iv_targets.push(spec.target.clone());
        }
        let edge = NewEdge {
            a_id,
            target_kind: spec.target_kind.clone(),
            target: spec.target.clone(),
            kind: spec.kind.clone(),
            severity: spec.severity.clone(),
            mechanism: spec.mechanism.clone(),
            citations: spec.citations.clone(),
            note: spec.note.clone(),
            source: Some(source.clone()),
        };
        match interaction::upsert(db, &edge)?.0 {
            Outcome::Inserted => sum.interactions_inserted += 1,
            Outcome::Replaced => sum.interactions_updated += 1,
            Outcome::Skipped => sum.interactions_unchanged += 1,
        }
    }
    Ok(())
}

/// Edge targets naming an intervention that is still not in the catalog: the
/// edge is kept (no stub is created) and a warning names the missing target.
fn unresolved(db: &Db, sum: &mut Summary) -> Result<()> {
    let ivs = IvCatalog::load(db)?;
    let mut missing: Vec<String> = sum.iv_targets.iter().filter(|t| ivs.find(t).is_none()).cloned().collect();
    missing.sort();
    missing.dedup();
    sum.warnings.extend(
        missing.iter().map(|t| {
            format!("intervention '{t}' is not in the catalog; its edges are kept unresolved until it is added")
        }),
    );
    sum.unresolved_targets = missing;
    Ok(())
}

pub fn run(ctx: &Ctx, a: InterventionImportArgs) -> Result<()> {
    let files = expand(&a.paths)?;
    let docs: Vec<(PathBuf, Vec<Doc>)> =
        files.iter().map(|f| read(f).map(|d| (f.clone(), d))).collect::<Result<_>>()?;
    let db = ctx.db()?;
    let mut sum = Summary { dry_run: a.dry_run, files: files.len(), ..Summary::default() };
    db.begin()?;
    let outcome = docs
        .iter()
        .try_for_each(|(f, ds)| ds.iter().try_for_each(|d| apply(&db, f, d, &mut sum)))
        .and_then(|()| unresolved(&db, &mut sum));
    let failed = sum.invalid > 0 && !a.skip_invalid;
    match (&outcome, a.dry_run || failed) {
        (Ok(()), false) => db.commit()?,
        _ => db.rollback()?,
    }
    outcome?;
    sum.warnings.iter().for_each(|w| ctx.warn(&format!("warning: {w}")));
    if failed {
        ctx.emit_mutation(&Report::object("intervention_import", to_record(&sum)))?;
        let first: Vec<String> = sum.errors.iter().take(5).map(Value::to_string).collect();
        return Err(AppError::invalid(format!(
            "import aborted: {} invalid interaction(s); nothing written (use --skip-invalid to import the rest)\n  {}",
            sum.invalid,
            first.join("\n  ")
        )));
    }
    ctx.info(&format!(
        "{}{} file(s): {} intervention(s) created, {} updated; interactions {} inserted, {} updated, {} unchanged, {} invalid",
        if a.dry_run { "dry run: " } else { "" },
        sum.files,
        sum.interventions_created.len(),
        sum.interventions_updated.len(),
        sum.interactions_inserted,
        sum.interactions_updated,
        sum.interactions_unchanged,
        sum.invalid
    ));
    ctx.emit_mutation(&Report::object("intervention_import", to_record(&sum)))
}
