//! `biomarker catalog docs`: one Markdown page per marker, an index grouped
//! by category and a coverage page. Output is deterministic (no dates), so
//! the committed docs/markers can be checked for staleness by a test.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::lint::{lint, LintReport};
use super::{Defs, MarkerDef, RangeDef, Sources};
use crate::error::Result;

const FOOTER: &str = "\n---\n\n_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._\n";

/// Escape a table cell.
fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn num(v: Option<f64>) -> String {
    v.map_or_else(|| "-".into(), |v| v.to_string())
}

fn citations(r: &RangeDef) -> String {
    if r.citations.is_empty() {
        return if r.legacy { "uncited (legacy)".into() } else { "uncited".into() };
    }
    r.citations
        .iter()
        .map(|c| {
            let label = cell(&c.label());
            let link = c.link().map_or(label.clone(), |l| format!("[{label}]({l})"));
            match &c.locator {
                Some(loc) => format!("{link}, {}", cell(loc)),
                None => link,
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn marker_page(m: &MarkerDef, defs: &Defs) -> String {
    let mut s = format!("# {} (`{}`)\n\n", m.name, m.slug);
    match &m.description {
        Some(d) => {
            let _ = writeln!(s, "{}\n", d.trim());
        }
        None => s.push_str("_No description yet._\n\n"),
    }
    s.push_str("| | |\n|---|---|\n");
    let _ = writeln!(s, "| Category | [{0}](index.md#{0}) |", m.category);
    let _ = writeln!(s, "| Canonical unit | {} |", cell(&m.unit));
    let loinc = m.loinc.as_ref().map_or_else(|| "-".into(), |l| format!("[{l}](https://loinc.org/{l}/)"));
    let _ = writeln!(s, "| LOINC | {loinc} |");
    let _ = writeln!(s, "| Specimen | {} |", m.specimen.as_deref().map_or_else(|| "-".into(), cell));
    let aliases = if m.aliases.is_empty() {
        "-".into()
    } else {
        m.aliases.iter().map(|a| format!("`{}`", cell(a))).collect::<Vec<_>>().join(", ")
    };
    let _ = writeln!(s, "| Aliases | {aliases} |");
    if !m.related.is_empty() {
        let rel = m.related.iter().map(|r| format!("[{r}]({r}.md)")).collect::<Vec<_>>().join(", ");
        let _ = writeln!(s, "| Related | {rel} |");
    }
    if !m.conversions.is_empty() {
        let _ = write!(s, "\n## Unit conversions\n\nInto {}: `value * factor + offset`.\n\n", cell(&m.unit));
        s.push_str("| From | Factor | Offset |\n|---|---|---|\n");
        for c in &m.conversions {
            let _ = writeln!(s, "| {} | {} | {} |", cell(&c.from), c.factor, c.offset);
        }
    }
    s.push_str("\n## Ranges\n\n");
    if let Some(why) = &m.no_reference {
        let _ = writeln!(s, "No clinical reference range: {}\n", why.trim());
    }
    if m.ranges.is_empty() {
        s.push_str("No ranges yet.\n");
    } else {
        let _ = writeln!(s, "Bounds in {}. The most specific matching range of each set applies.\n", cell(&m.unit));
        s.push_str("| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |\n");
        s.push_str("|---|---|---|---|---|---|---|---|---|\n");
        let order = |r: &RangeDef| defs.sets.iter().position(|x| x.name == r.set).unwrap_or(usize::MAX);
        let mut ranges: Vec<&RangeDef> = m.ranges.iter().collect();
        ranges.sort_by_key(|r| order(r));
        for r in ranges {
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                cell(&r.set),
                cell(&r.kind),
                cell(&r.when.describe()),
                num(r.low),
                num(r.high),
                r.evidence.as_deref().map_or_else(|| "-".into(), cell),
                r.population.as_deref().map_or_else(|| "-".into(), cell),
                citations(r),
                r.note.as_deref().map_or_else(|| "-".into(), cell),
            );
        }
    }
    s.push_str(FOOTER);
    s
}

fn index_page(defs: &Defs, report: &LintReport) -> String {
    let mut by_cat: BTreeMap<&str, Vec<&MarkerDef>> = BTreeMap::new();
    defs.markers.iter().for_each(|m| by_cat.entry(m.category.as_str()).or_default().push(m));
    let mut s = String::from("# Biomarker catalog\n\n");
    let _ = writeln!(
        s,
        "{} markers, {} ranges ({} cited). See [coverage](coverage.md) for the range coverage matrix.\n",
        report.markers,
        report.ranges,
        report.coverage.iter().map(|c| c.cited).sum::<usize>()
    );
    s.push_str("Range sets:\n\n");
    for set in &defs.sets {
        let _ = writeln!(s, "* `{}`: {}", set.name, set.description);
    }
    for (cat, markers) in by_cat {
        let _ =
            write!(s, "\n## {cat}\n\n| Marker | Slug | Unit | LOINC | Ranges | Cited |\n|---|---|---|---|---|---|\n");
        for m in markers {
            let cited = m.ranges.iter().filter(|r| !r.citations.is_empty()).count();
            let _ = writeln!(
                s,
                "| [{}]({}.md) | `{}` | {} | {} | {} | {} |",
                cell(&m.name),
                m.slug,
                m.slug,
                cell(&m.unit),
                m.loinc.as_deref().unwrap_or("-"),
                m.ranges.len(),
                cited
            );
        }
    }
    s.push_str(FOOTER);
    s
}

fn coverage_page(defs: &Defs, report: &LintReport) -> String {
    let mut s = String::from("# Catalog coverage\n\n");
    s.push_str("Range kinds per set (`ref` reference, `opt` optimal, `warn`, `crit` critical; counts above 1 in\n");
    s.push_str("parentheses), the `when` dimensions used, and how many ranges are cited.\n\n");
    let sets: Vec<&str> = defs.sets.iter().map(|x| x.name.as_str()).collect();
    let _ = writeln!(s, "| Marker | Category | {} | Conditions | Cited |", sets.join(" | "));
    let _ = writeln!(s, "|---|---|{}---|---|", "---|".repeat(sets.len()));
    for c in &report.coverage {
        let per_set = sets
            .iter()
            .map(|set| {
                c.sets.get(*set).map_or_else(
                    || "-".to_string(),
                    |kinds| {
                        kinds
                            .iter()
                            .map(|(k, n)| {
                                let short = match k.as_str() {
                                    "reference" => "ref",
                                    "optimal" => "opt",
                                    "critical" => "crit",
                                    other => other,
                                };
                                if *n > 1 {
                                    format!("{short}({n})")
                                } else {
                                    short.to_string()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    },
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        let dims = if c.dimensions.is_empty() {
            "-".into()
        } else {
            c.dimensions.iter().copied().collect::<Vec<_>>().join(", ")
        };
        let _ = writeln!(
            s,
            "| [{0}]({0}.md) | {1} | {per_set} | {dims} | {2}/{3} |",
            c.marker, c.category, c.cited, c.ranges
        );
    }
    if let Some(u) = &report.universe {
        let _ = write!(
            s,
            "\n## Universe\n\n{} of {} biomarkers in catalog/universe.toml have a catalog file.\n\n",
            u.covered, u.entries
        );
        s.push_str("| Priority | Covered | Total |\n|---|---|---|\n");
        for (p, (covered, total)) in &u.by_priority {
            let _ = writeln!(s, "| {p} | {covered} | {total} |");
        }
        if !u.missing.is_empty() {
            s.push_str("\nMissing:\n\n| Slug | Name | Category | Priority |\n|---|---|---|---|\n");
            for g in &u.missing {
                let _ = writeln!(s, "| `{}` | {} | {} | {} |", g.slug, cell(&g.name), g.category, g.priority);
            }
        }
    }
    s.push_str(FOOTER);
    s
}

/// Every generated page as `(file name, contents)`.
pub fn render(src: &Sources) -> Result<BTreeMap<String, String>> {
    let defs = Defs::parse(src)?;
    let report = lint(src, &crate::seed::unit_symbols(), true, false);
    let mut pages: BTreeMap<String, String> =
        defs.markers.iter().map(|m| (format!("{}.md", m.slug), marker_page(m, &defs))).collect();
    pages.insert("index.md".into(), index_page(&defs, &report));
    pages.insert("coverage.md".into(), coverage_page(&defs, &report));
    Ok(pages)
}

/// Write the pages into `out`, removing stale `*.md` files there. Returns
/// `(written, removed)` file names.
pub fn write(src: &Sources, out: &std::path::Path) -> Result<(Vec<String>, Vec<String>)> {
    use crate::error::AppError;
    let pages = render(src)?;
    std::fs::create_dir_all(out).map_err(|e| AppError::io(format!("creating {}: {e}", out.display())))?;
    let mut removed = Vec::new();
    for entry in std::fs::read_dir(out).map_err(|e| AppError::io(format!("reading {}: {e}", out.display())))?.flatten()
    {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".md") && !pages.contains_key(&name) {
            std::fs::remove_file(entry.path()).map_err(|e| AppError::io(format!("removing {name}: {e}")))?;
            removed.push(name);
        }
    }
    let mut written = Vec::new();
    for (name, text) in &pages {
        let path = out.join(name);
        if std::fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
            std::fs::write(&path, text).map_err(|e| AppError::io(format!("writing {}: {e}", path.display())))?;
            written.push(name.clone());
        }
    }
    removed.sort();
    Ok((written, removed))
}
