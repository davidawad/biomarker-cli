//! `biomarker catalog lint`: schema and cross-file checks over the catalog,
//! the range coverage matrix, and coverage against catalog/universe.toml.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::cond::{CYCLE_PHASES, MENOPAUSE, OPS};
use super::{
    parse_conditions, parse_marker, parse_sets, parse_universe, Citation, MarkerDef, RangeDef, Sources, EVIDENCE,
    KINDS, PRIORITIES,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub severity: Severity,
    /// Stable machine code, e.g. `uncited`, `ambiguous`, `universe-missing`.
    pub code: &'static str,
    pub marker: Option<String>,
    pub message: String,
}

/// One row of the coverage matrix: which range kinds a marker has in each
/// set, and which `when` dimensions its ranges use.
#[derive(Debug, Clone, Serialize)]
pub struct Coverage {
    pub marker: String,
    pub category: String,
    /// set -> kind -> number of ranges.
    pub sets: BTreeMap<String, BTreeMap<String, usize>>,
    pub dimensions: BTreeSet<&'static str>,
    pub ranges: usize,
    pub cited: usize,
    pub legacy: usize,
    pub no_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UniverseGap {
    pub slug: String,
    pub name: String,
    pub category: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UniverseReport {
    pub entries: usize,
    pub covered: usize,
    /// priority -> (covered, total).
    pub by_priority: BTreeMap<String, (usize, usize)>,
    pub missing: Vec<UniverseGap>,
    /// Catalog markers the universe does not list.
    pub not_in_universe: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LintReport {
    pub markers: usize,
    pub ranges: usize,
    pub errors: usize,
    pub warnings: usize,
    pub findings: Vec<Finding>,
    pub coverage: Vec<Coverage>,
    pub universe: Option<UniverseReport>,
}

impl LintReport {
    pub fn ok(&self) -> bool {
        self.errors == 0
    }
}

struct Ctx {
    findings: Vec<Finding>,
}

impl Ctx {
    fn push(&mut self, severity: Severity, code: &'static str, marker: Option<&str>, message: String) {
        self.findings.push(Finding { severity, code, marker: marker.map(str::to_string), message });
    }
    fn err(&mut self, code: &'static str, marker: &str, message: String) {
        self.push(Severity::Error, code, Some(marker), message);
    }
    fn warn(&mut self, code: &'static str, marker: &str, message: String) {
        self.push(Severity::Warning, code, Some(marker), message);
    }
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.split('-').all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
}

fn is_loinc(s: &str) -> bool {
    s.split_once('-').is_some_and(|(a, b)| {
        (1..=7).contains(&a.len())
            && a.chars().all(|c| c.is_ascii_digit())
            && b.len() == 1
            && b.chars().all(|c| c.is_ascii_digit())
    })
}

/// Lint the catalog. `known_units` are the unit symbols the database knows;
/// `universe` adds coverage against universe.toml; `strict` turns every
/// warning (uncited legacy ranges, coverage gaps) into an error.
pub fn lint(src: &Sources, known_units: &[&str], universe: bool, strict: bool) -> LintReport {
    let mut cx = Ctx { findings: Vec::new() };
    let sets = parse_sets(&src.sets).unwrap_or_else(|e| {
        cx.push(Severity::Error, "schema", None, e.message);
        Vec::new()
    });
    let set_names: BTreeSet<&str> = sets.iter().map(|s| s.name.as_str()).collect();
    let conditions = parse_conditions(&src.conditions).unwrap_or_else(|e| {
        cx.push(Severity::Error, "schema", None, e.message);
        Vec::new()
    });
    let condition_slugs: BTreeSet<&str> = conditions.iter().map(|c| c.slug.as_str()).collect();
    let mut markers: Vec<MarkerDef> = Vec::new();
    for (file, text) in &src.markers {
        match parse_marker(file, text) {
            Ok(m) => {
                if *file != format!("{}.toml", m.slug) {
                    cx.err("file-name", &m.slug, format!("{file} must be named {}.toml", m.slug));
                }
                markers.push(m);
            }
            Err(e) => cx.push(Severity::Error, "schema", None, e.message),
        }
    }
    let slugs: BTreeSet<&str> = markers.iter().map(|m| m.slug.as_str()).collect();
    let mut names: BTreeMap<String, &str> = BTreeMap::new();
    for m in &markers {
        for n in std::iter::once(&m.slug).chain(&m.aliases) {
            let key = n.trim().to_lowercase();
            match names.get(&key) {
                Some(other) if *other != m.slug => {
                    let msg = format!("name or alias '{key}' is also used by {other}");
                    cx.err("alias", &m.slug, msg);
                }
                _ => {
                    names.insert(key, &m.slug);
                }
            }
        }
    }
    let world = World { slugs: &slugs, sets: &set_names, conditions: &condition_slugs, units: known_units };
    markers.iter().for_each(|m| lint_marker(&mut cx, m, &world));
    let coverage = markers.iter().map(coverage_of).collect();
    let universe = universe.then(|| universe_report(&mut cx, &src.universe, &markers));
    if strict {
        cx.findings.iter_mut().for_each(|f| f.severity = Severity::Error);
    }
    cx.findings.sort_by(|a, b| (a.severity, &a.marker, a.code).cmp(&(b.severity, &b.marker, b.code)));
    let count = |s| cx.findings.iter().filter(|f| f.severity == s).count();
    LintReport {
        markers: markers.len(),
        ranges: markers.iter().map(|m| m.ranges.len()).sum(),
        errors: count(Severity::Error),
        warnings: count(Severity::Warning),
        coverage,
        universe,
        findings: cx.findings,
    }
}

struct World<'a> {
    slugs: &'a BTreeSet<&'a str>,
    sets: &'a BTreeSet<&'a str>,
    conditions: &'a BTreeSet<&'a str>,
    units: &'a [&'a str],
}

impl World<'_> {
    fn knows_unit(&self, u: &str) -> bool {
        self.units.iter().any(|k| crate::units::same_unit(k, u))
    }
}

fn lint_marker(cx: &mut Ctx, m: &MarkerDef, w: &World) {
    let s = m.slug.as_str();
    if !is_slug(s) {
        cx.err("schema", s, "slug must be lowercase letters, digits and single hyphens".into());
    }
    if !is_slug(&m.category) {
        cx.err("schema", s, format!("category '{}' must be lowercase letters, digits and hyphens", m.category));
    }
    if m.name.trim().is_empty() {
        cx.err("schema", s, "name is empty".into());
    }
    if let Some(l) = m.loinc.as_deref().filter(|l| !is_loinc(l)) {
        cx.err("loinc", s, format!("'{l}' is not a LOINC code (NNNNN-N)"));
    }
    if m.description.as_deref().is_none_or(|d| d.trim().is_empty()) {
        cx.warn("description", s, "no description".into());
    }
    if !w.knows_unit(&m.unit) {
        cx.warn("unit", s, format!("unit '{}' is not a built-in unit (it is added on seeding)", m.unit));
    }
    for c in &m.conversions {
        if c.factor == 0.0 || !c.factor.is_finite() || !c.offset.is_finite() {
            cx.err("conversion", s, format!("conversion from {}: factor must be a non-zero number", c.from));
        }
        if crate::units::same_unit(&c.from, &m.unit) {
            cx.err("conversion", s, format!("conversion from {} into itself", c.from));
        } else if !w.knows_unit(&c.from) {
            cx.warn("unit", s, format!("conversion unit '{}' is not a built-in unit", c.from));
        }
    }
    let dup_conv = m.conversions.iter().enumerate().any(|(i, a)| m.conversions[..i].iter().any(|b| b.from == a.from));
    if dup_conv {
        cx.err("conversion", s, "duplicate conversion".into());
    }
    for r in m.related.iter().filter(|r| !w.slugs.contains(r.as_str()) || *r == s) {
        cx.err("related", s, format!("related marker '{r}' is not in the catalog"));
    }
    let has_reference = m.ranges.iter().any(|r| r.set == super::DEFAULT_SET && r.kind == "reference");
    match (&m.no_reference, has_reference) {
        (None, false) => {
            cx.err("no-reference", s, "no clinical reference range (add one or no_reference = \"reason\")".into())
        }
        (Some(_), true) => cx.warn("no-reference", s, "has a clinical reference range and no_reference".into()),
        (Some(_), false) => cx.warn("coverage", s, "no clinical reference range (no_reference)".into()),
        (None, true) => {}
    }
    m.ranges.iter().enumerate().for_each(|(i, r)| lint_range(cx, m, i + 1, r, w));
    ambiguity(cx, m);
}

fn lint_range(cx: &mut Ctx, m: &MarkerDef, n: usize, r: &RangeDef, w: &World) {
    let s = m.slug.as_str();
    let at = |msg: String| format!("range {n} ({} {}): {msg}", r.set, r.kind);
    if !w.sets.contains(r.set.as_str()) {
        cx.err("set", s, at(format!("unknown set (known: {})", w.sets.iter().copied().collect::<Vec<_>>().join(", "))));
    }
    if !KINDS.contains(&r.kind.as_str()) {
        cx.err("kind", s, at(format!("kind must be one of {}", KINDS.join(", "))));
    }
    match (r.low, r.high) {
        (None, None) => cx.err("bounds", s, at("give at least one of low / high".into())),
        (Some(l), Some(h)) if l > h => cx.err("bounds", s, at(format!("low ({l}) is above high ({h})"))),
        (l, h) if l.into_iter().chain(h).any(|v| !v.is_finite()) => {
            cx.err("bounds", s, at("bounds must be finite".into()))
        }
        _ => {}
    }
    let wh = &r.when;
    if let Some(x) = wh.sex.as_deref().filter(|x| !matches!(*x, "male" | "female")) {
        cx.err("when", s, at(format!("sex '{x}' must be male or female")));
    }
    if let Some([a, b]) = wh.age.filter(|[a, b]| !(*a >= 0.0 && a < b)) {
        cx.err("when", s, at(format!("age [{a}, {b}] must satisfy 0 <= min < max")));
    }
    if let Some(t) = wh.trimester.filter(|t| !(1..=3).contains(t)) {
        cx.err("when", s, at(format!("trimester {t} must be 1, 2 or 3")));
    }
    if wh.trimester.is_some() && wh.pregnant == Some(false) {
        cx.err("when", s, at("trimester with pregnant = false".into()));
    }
    if (wh.pregnant == Some(true) || wh.trimester.is_some() || wh.menopause.is_some() || wh.cycle_phase.is_some())
        && wh.sex.as_deref() == Some("male")
    {
        cx.err("when", s, at("female-only condition with sex = male".into()));
    }
    if let Some(x) = wh.menopause.as_deref().filter(|x| !MENOPAUSE.contains(x)) {
        cx.err("when", s, at(format!("menopause '{x}' must be one of {}", MENOPAUSE.join(", "))));
    }
    if let Some(x) = wh.cycle_phase.as_deref().filter(|x| !CYCLE_PHASES.contains(x)) {
        cx.err("when", s, at(format!("cycle_phase '{x}' must be one of {}", CYCLE_PHASES.join(", "))));
    }
    for c in &wh.markers {
        if !w.slugs.contains(c.marker.as_str()) {
            cx.err("when-marker", s, at(format!("when.markers names unknown marker '{}'", c.marker)));
        } else if c.marker == m.slug {
            cx.err("when-marker", s, at("when.markers names the marker itself".into()));
        }
        if !OPS.contains(&c.op.as_str()) {
            cx.err("when-marker", s, at(format!("op '{}' must be one of {}", c.op, OPS.join(" "))));
        }
    }
    for c in wh.conditions.iter().filter(|c| !w.conditions.contains(c.as_str())) {
        cx.err("when-condition", s, at(format!("unknown condition '{c}' (see catalog/conditions.toml)")));
    }
    if let Some(e) = r.evidence.as_deref().filter(|e| !EVIDENCE.contains(e)) {
        cx.err("evidence", s, at(format!("evidence '{e}' must be one of {}", EVIDENCE.join(", "))));
    }
    match (r.citations.is_empty(), r.legacy) {
        (true, true) => cx.warn("uncited", s, at("legacy range without a citation".into())),
        (true, false) => {
            cx.err("uncited", s, at("no citation (add citations, or legacy = true for carried-over values)".into()))
        }
        (false, _) => r.citations.iter().for_each(|c| lint_citation(cx, s, &at, c)),
    }
}

fn lint_citation(cx: &mut Ctx, s: &str, at: &dyn Fn(String) -> String, c: &Citation) {
    if c.pmid.is_none() && c.doi.is_none() && c.url.is_none() {
        cx.err("citation", s, at("a citation needs a pmid, doi or url".into()));
    }
    if let Some(p) = c.pmid.as_deref().filter(|p| p.is_empty() || !p.chars().all(|c| c.is_ascii_digit())) {
        cx.err("citation", s, at(format!("pmid '{p}' must be digits")));
    }
    if let Some(d) = c.doi.as_deref().filter(|d| !d.starts_with("10.") || !d.contains('/')) {
        cx.err("citation", s, at(format!("doi '{d}' must look like 10.NNNN/suffix")));
    }
    if let Some(u) = c.url.as_deref().filter(|u| !u.starts_with("http://") && !u.starts_with("https://")) {
        cx.err("citation", s, at(format!("url '{u}' must start with http(s)://")));
    }
}

/// Two ranges of one set and kind that a single person can match, with the
/// same specificity but different bounds: selection could not choose.
fn ambiguity(cx: &mut Ctx, m: &MarkerDef) {
    let rs = &m.ranges;
    for (i, a) in rs.iter().enumerate() {
        for (j, b) in rs.iter().enumerate().skip(i + 1) {
            if a.set != b.set || a.kind != b.kind {
                continue;
            }
            let ra = a.to_range(0, &|v| v);
            let rb = b.to_range(0, &|v| v);
            let key = |r: &crate::ranges::Range| (r.cond.count(), r.sex == "any", r.age_max - r.age_min);
            let same_key = key(&ra) == key(&rb);
            let ages_meet = ra.age_min < rb.age_max && rb.age_min < ra.age_max;
            let sexes_meet = ra.sex == "any" || rb.sex == "any" || ra.sex == rb.sex;
            if !(same_key && ages_meet && sexes_meet && ra.cond.overlaps(&rb.cond)) {
                continue;
            }
            let what = format!("ranges {} and {} ({} {})", i + 1, j + 1, a.set, a.kind);
            if (a.low, a.high) == (b.low, b.high) {
                cx.warn("duplicate-range", &m.slug, format!("{what} are duplicates"));
            } else {
                let when = a.when.describe();
                cx.err("ambiguous", &m.slug, format!("{what} are equally specific for '{when}' but differ"));
            }
        }
    }
}

fn coverage_of(m: &MarkerDef) -> Coverage {
    let mut sets: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut dimensions = BTreeSet::new();
    for r in &m.ranges {
        *sets.entry(r.set.clone()).or_default().entry(r.kind.clone()).or_default() += 1;
        if r.when.sex.is_some() {
            dimensions.insert("sex");
        }
        if r.when.age.is_some() {
            dimensions.insert("age");
        }
        dimensions.extend(r.when.conditions().dimensions());
    }
    Coverage {
        marker: m.slug.clone(),
        category: m.category.clone(),
        sets,
        dimensions,
        ranges: m.ranges.len(),
        cited: m.ranges.iter().filter(|r| !r.citations.is_empty()).count(),
        legacy: m.ranges.iter().filter(|r| r.legacy).count(),
        no_reference: m.no_reference.clone(),
    }
}

fn universe_report(cx: &mut Ctx, text: &str, markers: &[MarkerDef]) -> UniverseReport {
    let entries = parse_universe(text).unwrap_or_else(|e| {
        cx.push(Severity::Error, "universe", None, e.message);
        Vec::new()
    });
    let mut seen = BTreeSet::new();
    for e in &entries {
        if !seen.insert(e.slug.as_str()) {
            cx.push(Severity::Error, "universe", Some(&e.slug), "listed twice in universe.toml".into());
        }
        if !is_slug(&e.slug) {
            cx.push(Severity::Error, "universe", Some(&e.slug), "universe slug must be a lowercase slug".into());
        }
        if !PRIORITIES.contains(&e.priority.as_str()) {
            let msg = format!("priority '{}' must be one of {}", e.priority, PRIORITIES.join(", "));
            cx.push(Severity::Error, "universe", Some(&e.slug), msg);
        }
        if let Some(l) = e.loinc.as_deref().filter(|l| !is_loinc(l)) {
            cx.push(Severity::Error, "universe", Some(&e.slug), format!("universe loinc '{l}' is not a LOINC code"));
        }
    }
    let by_slug: BTreeMap<&str, &MarkerDef> = markers.iter().map(|m| (m.slug.as_str(), m)).collect();
    let mut by_priority: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut missing = Vec::new();
    for e in &entries {
        let slot = by_priority.entry(e.priority.clone()).or_default();
        slot.1 += 1;
        match by_slug.get(e.slug.as_str()) {
            Some(m) => {
                slot.0 += 1;
                if let (Some(a), Some(b)) = (&e.loinc, &m.loinc) {
                    if a != b {
                        cx.warn(
                            "universe-loinc",
                            &e.slug,
                            format!("universe loinc {a} differs from the catalog's {b}"),
                        );
                    }
                }
            }
            None => {
                cx.warn("universe-missing", &e.slug, format!("{} ({}) has no catalog file", e.name, e.priority));
                missing.push(UniverseGap {
                    slug: e.slug.clone(),
                    name: e.name.clone(),
                    category: e.category.clone(),
                    priority: e.priority.clone(),
                });
            }
        }
    }
    let not_in_universe: Vec<String> =
        markers.iter().filter(|m| !seen.contains(m.slug.as_str())).map(|m| m.slug.clone()).collect();
    not_in_universe
        .iter()
        .for_each(|s| cx.warn("not-in-universe", s, "catalog marker missing from universe.toml".into()));
    UniverseReport {
        entries: entries.len(),
        covered: entries.len() - missing.len(),
        by_priority,
        missing,
        not_in_universe,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn src(markers: &[(&str, &str)]) -> Sources {
        Sources {
            markers: markers.iter().map(|(n, t)| ((*n).to_string(), (*t).to_string())).collect(),
            ..Sources::embedded()
        }
    }

    fn codes(r: &LintReport, sev: Severity) -> Vec<&'static str> {
        r.findings.iter().filter(|f| f.severity == sev).map(|f| f.code).collect()
    }

    const HEAD: &str = "slug = \"x\"\nname = \"X\"\ncategory = \"lipid\"\nunit = \"mg/dL\"\ndescription = \"d\"\n";

    #[test]
    fn embedded_catalog_has_no_errors() {
        let r = lint(&Sources::embedded(), &crate::seed::unit_symbols(), true, false);
        assert!(r.ok(), "{:#?}", r.findings.iter().filter(|f| f.severity == Severity::Error).collect::<Vec<_>>());
        assert!(r.warnings > 0, "legacy ranges are uncited");
        let u = r.universe.as_ref().unwrap();
        assert!(u.covered > 0 && u.covered + u.missing.len() == u.entries, "universe gaps are warnings");
    }

    #[test]
    fn uncited_ambiguous_and_unknown_references_are_errors() {
        let text = format!(
            "{HEAD}[[range]]\nlow = 1.0\nwhen = {{ markers = [{{ marker = \"nope\", op = \">\", value = 1.0 }}] }}\n\
             citations = [{{ pmid = \"1\" }}]\n\
             [[range]]\nlow = 2.0\nwhen = {{ conditions = [\"made-up\"] }}\ncitations = [{{ pmid = \"2\" }}]\n\
             [[range]]\nlow = 3.0\n"
        );
        let r = lint(&src(&[("x.toml", &text)]), &["mg/dL"], false, false);
        let errors = codes(&r, Severity::Error);
        assert!(errors.contains(&"when-marker"), "{errors:?}");
        assert!(errors.contains(&"when-condition"), "{errors:?}");
        assert!(errors.contains(&"uncited"), "{errors:?}");
        assert!(errors.contains(&"ambiguous"), "markers and conditions ranges are equally specific: {errors:?}");
    }

    #[test]
    fn disjoint_conditions_are_not_ambiguous_and_strict_promotes_warnings() {
        let text = format!(
            "{HEAD}[[range]]\nlow = 1.0\nwhen = {{ sex = \"male\" }}\nlegacy = true\n\
             [[range]]\nlow = 2.0\nwhen = {{ sex = \"female\" }}\nlegacy = true\n\
             [[range]]\nlow = 3.0\nwhen = {{ sex = \"female\", menopause = \"post\" }}\nlegacy = true\n"
        );
        let r = lint(&src(&[("x.toml", &text)]), &["mg/dL"], false, false);
        assert!(r.ok(), "{:?}", r.findings);
        assert_eq!(codes(&r, Severity::Warning), ["uncited"; 3]);
        assert_eq!(r.coverage[0].dimensions.iter().copied().collect::<Vec<_>>(), ["menopause", "sex"]);
        assert!(!lint(&src(&[("x.toml", &text)]), &["mg/dL"], false, true).ok());
    }

    #[test]
    fn missing_reference_and_bad_file_name() {
        let r = lint(&src(&[("y.toml", HEAD)]), &["mg/dL"], false, false);
        assert_eq!(codes(&r, Severity::Error), ["file-name", "no-reference"]);
        let r = lint(&src(&[("x.toml", "slug = 1")]), &[], false, false);
        assert_eq!(codes(&r, Severity::Error), ["schema"]);
    }
}
