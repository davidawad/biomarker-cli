//! `biomarker catalog lint | docs | schema | sets | conditions`. None of them
//! open the database.

use serde_json::json;

use crate::catalog::lint::{lint, LintReport};
use crate::catalog::{builtin, docs, Sources, SCHEMA_JSON};
use crate::cli::CatalogCmd;
use crate::context::Ctx;
use crate::error::{ErrorKind, Result};
use crate::output::{to_record, Format, Report};

use super::Status;

pub fn run(ctx: &Ctx, cmd: CatalogCmd) -> Result<Status> {
    let sources = |dir: Option<&std::path::Path>| dir.map_or_else(|| Ok(Sources::embedded()), Sources::from_dir);
    match cmd {
        CatalogCmd::Lint { json, strict, universe, dir } => {
            let report = lint(&sources(dir.as_deref())?, &crate::seed::unit_symbols(), universe, strict);
            emit_lint(ctx, &report, json)?;
            Ok(if report.ok() { 0 } else { ErrorKind::Invalid.exit_code() })
        }
        CatalogCmd::Docs { out, dir, check } => {
            let src = sources(dir.as_deref())?;
            if check {
                let stale = stale_pages(&src, &out)?;
                stale.iter().for_each(|p| ctx.warn(&format!("stale: {}", out.join(p).display())));
                let rec = to_record(&json!({"out": out, "stale": stale}));
                ctx.emit_mutation(&Report::object("catalog_docs", rec))?;
                return Ok(if stale.is_empty() { 0 } else { ErrorKind::Invalid.exit_code() });
            }
            let (written, removed) = docs::write(&src, &out)?;
            ctx.info(&format!("{}: {} written, {} removed", out.display(), written.len(), removed.len()));
            ctx.emit_mutation(&Report::object(
                "catalog_docs",
                to_record(&json!({"out": out, "written": written, "removed": removed})),
            ))?;
            Ok(0)
        }
        CatalogCmd::Schema => {
            crate::output::write_stdout(SCHEMA_JSON.as_bytes())?;
            Ok(0)
        }
        CatalogCmd::Sets => {
            let rows = builtin()?
                .sets
                .iter()
                .map(|s| to_record(&json!({"name": s.name, "description": s.description})))
                .collect();
            ctx.emit(&Report::list("range_sets", rows)).map(|()| 0)
        }
        CatalogCmd::Conditions => {
            let rows =
                builtin()?.conditions.iter().map(|c| to_record(&json!({"slug": c.slug, "name": c.name}))).collect();
            ctx.emit(&Report::list("conditions", rows)).map(|()| 0)
        }
    }
}

/// Page names that differ from (or are missing in, or are extra in) `out`.
pub fn stale_pages(src: &Sources, out: &std::path::Path) -> Result<Vec<String>> {
    let pages = docs::render(src)?;
    let mut stale: Vec<String> = pages
        .iter()
        .filter(|(name, text)| std::fs::read_to_string(out.join(name)).ok().as_deref() != Some(text.as_str()))
        .map(|(name, _)| name.clone())
        .collect();
    if let Ok(rd) = std::fs::read_dir(out) {
        stale.extend(
            rd.flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.ends_with(".md") && !pages.contains_key(n)),
        );
    }
    let panels = out.parent().unwrap_or(out).join("panels");
    if let Ok(rd) = std::fs::read_dir(&panels) {
        stale.extend(
            rd.flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.ends_with(".md") && !pages.contains_key(&format!("../panels/{n}")))
                .map(|n| format!("../panels/{n}")),
        );
    }
    stale.sort();
    Ok(stale)
}

fn emit_lint(ctx: &Ctx, r: &LintReport, json_flag: bool) -> Result<()> {
    if json_flag || matches!(ctx.out.format, Format::Json | Format::Jsonl | Format::Yaml | Format::Toon) {
        let mut out = ctx.out.clone();
        if json_flag {
            out.format = Format::Json;
        }
        return ctx.emit_with(&Report::object("catalog_lint", to_record(r)), &out);
    }
    let findings = r
        .findings
        .iter()
        .map(|f| to_record(&json!({"severity": f.severity, "code": f.code, "marker": f.marker, "message": f.message})))
        .collect();
    ctx.emit(&Report::list("catalog_lint", findings).table_columns(&["severity", "code", "marker", "message"]))?;
    let sets: Vec<String> = {
        let mut s: Vec<String> = r.coverage.iter().flat_map(|c| c.sets.keys().cloned()).collect();
        s.sort();
        s.dedup();
        s
    };
    let matrix = r
        .coverage
        .iter()
        .map(|c| {
            let mut rec = to_record(&json!({"marker": c.marker, "category": c.category}));
            for set in &sets {
                let kinds =
                    c.sets.get(set).map(|k| k.keys().cloned().collect::<Vec<_>>().join(" ")).unwrap_or_default();
                rec.insert(set.clone(), json!(kinds));
            }
            let dims = c.dimensions.iter().copied().collect::<Vec<_>>().join(",");
            rec.insert("conditions".into(), json!(dims));
            rec.insert("cited".into(), json!(format!("{}/{}", c.cited, c.ranges)));
            rec
        })
        .collect();
    let mut cols = vec!["marker".to_string(), "category".to_string()];
    cols.extend(sets.iter().cloned());
    cols.extend(["conditions".to_string(), "cited".to_string()]);
    ctx.emit(&Report::list("catalog_coverage", matrix).table_columns(&cols))?;
    if let Some(u) = &r.universe {
        let rows = u
            .by_priority
            .iter()
            .map(|(p, (c, t))| to_record(&json!({"priority": p, "covered": c, "total": t})))
            .collect();
        ctx.emit(&Report::list("catalog_universe", rows))?;
    }
    ctx.info(&format!(
        "{} markers, {} panels, {} ranges ({} cited), {} pending: {} error(s), {} warning(s)",
        r.markers, r.panels, r.ranges, r.cited, r.pending, r.errors, r.warnings
    ));
    Ok(())
}
