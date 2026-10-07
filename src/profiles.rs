//! User-editable unit presets, range sets and per-person profiles.
//!
//! Three kinds of small TOML files live next to `config.toml` (all optional):
//!
//! * `units/<name>.toml` - a *unit preset*: which unit each marker is shown
//!   in. Built-ins: `canonical`, `us`, `si`, `uk`. A file with a built-in's
//!   name replaces it. `extends = "<preset>"` inherits; `[units]` maps a
//!   marker slug (or alias, or `@category`) to a unit; `[[conversion]]`
//!   adds conversions the preset needs (e.g. mg/dL -> mg/mL).
//! * `ranges/<name>.toml` - a named *range set* (`[[range]]` entries, with
//!   optional sex / age band / lab), selectable per person or globally.
//! * `people/<slug>.toml` - one person's *profile*: `unit_preset`,
//!   `range_set`, per-marker `[units]` and personal `[[range]]` entries.
//!
//! `config.toml` may also carry a `[units]` table of per-marker overrides
//! on top of the global `unit_system`.
//!
//! Range precedence per measurement: the person's own ranges (this file,
//! then `range set --person` from the database) > the person's (or the
//! global) range set > the catalog ranges. When any range entry names the
//! measurement's `lab`, only those lab-specific entries are considered.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::error::{AppError, Result};
use crate::ranges::{Range, RangeKind};
use crate::store::{Catalog, Marker};
use crate::units::{same_unit, Conversion};

const BUILTIN_PRESETS: &[(&str, &str)] = &[
    ("canonical", include_str!("../presets/units/canonical.toml")),
    ("us", include_str!("../presets/units/us.toml")),
    ("si", include_str!("../presets/units/si.toml")),
    ("uk", include_str!("../presets/units/uk.toml")),
];
const BUILTIN_CONVERSIONS: &str = include_str!("../presets/conversions.toml");

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversionSpec {
    pub marker: Option<String>,
    pub from: String,
    pub to: String,
    pub factor: f64,
    #[serde(default)]
    pub offset: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitPreset {
    #[serde(default)]
    pub description: String,
    pub extends: Option<String>,
    /// Fallback for markers not listed in `units`: `canonical`, `us` or `si`
    /// (pick the first reachable unit of that system).
    pub system: Option<String>,
    #[serde(default)]
    pub units: BTreeMap<String, String>,
    #[serde(default, rename = "conversion")]
    pub conversions: Vec<ConversionSpec>,
    #[serde(skip)]
    pub origin: Option<PathBuf>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RangeSpec {
    /// Required, except inside a `[[marker]]` table of markers.toml.
    #[serde(default)]
    pub marker: String,
    /// `reference` (default) or `optimal`.
    pub kind: Option<String>,
    /// `any` (default), `male` or `female`.
    pub sex: Option<String>,
    pub age_min: Option<f64>,
    pub age_max: Option<f64>,
    pub low: Option<f64>,
    pub high: Option<f64>,
    /// Beyond these the result is flagged critical (same unit as `low` / `high`).
    pub critical_low: Option<f64>,
    pub critical_high: Option<f64>,
    /// Unit of the bounds (default: the marker's canonical unit).
    pub unit: Option<String>,
    /// Only applies to measurements from this lab.
    pub lab: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RangeSet {
    #[serde(default)]
    pub description: String,
    pub extends: Option<String>,
    #[serde(default, rename = "range")]
    pub ranges: Vec<RangeSpec>,
    #[serde(skip)]
    pub origin: Option<PathBuf>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonProfile {
    #[serde(default)]
    pub description: String,
    pub unit_preset: Option<String>,
    pub range_set: Option<String>,
    #[serde(default)]
    pub units: BTreeMap<String, String>,
    #[serde(default, rename = "range")]
    pub ranges: Vec<RangeSpec>,
    /// Display decimals per marker slug or `@category`.
    #[serde(default)]
    pub marker_precision: BTreeMap<String, usize>,
    /// `reference`, `optimal` or `both` for this person's flags.
    pub range_flavor: Option<String>,
    #[serde(skip)]
    pub origin: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct Compiled {
    marker_id: i64,
    lab: Option<String>,
    range: Range,
}

/// A conversion into the marker's own unit (`to` defaults to the marker's unit).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkerConversion {
    pub from: String,
    pub to: Option<String>,
    pub factor: f64,
    #[serde(default)]
    pub offset: f64,
}

/// One `[[marker]]` of markers.toml: a catalog entry to create or update.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkerSpec {
    pub slug: String,
    pub name: Option<String>,
    pub category: Option<String>,
    /// Required for a new marker. An existing marker's unit cannot change.
    pub unit: Option<String>,
    pub loinc: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default, rename = "conversion")]
    pub conversions: Vec<MarkerConversion>,
    /// Catalog ranges (sex / age bands); `marker` is implied.
    #[serde(default, rename = "range")]
    pub ranges: Vec<RangeSpec>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkerFile {
    #[serde(default, rename = "marker")]
    pub markers: Vec<MarkerSpec>,
}

/// Parse a markers.toml (errors name the file).
pub fn load_marker_file(path: &Path) -> Result<MarkerFile> {
    let text = std::fs::read_to_string(path).map_err(|e| AppError::io(format!("reading {}: {e}", path.display())))?;
    parse(&text, &path.display().to_string())
}

/// `<config dir>/markers.toml`.
pub fn default_marker_file(config_path: &Path) -> PathBuf {
    config_path.parent().map(Path::to_path_buf).unwrap_or_default().join("markers.toml")
}

/// Turn a range entry into a catalog range (bounds converted to the marker's unit).
pub fn range_from_spec(cat: &Catalog, spec: &RangeSpec) -> Result<Range> {
    compile_one(cat, spec).map(|c| c.range)
}

/// Settings that shape how profiles are applied (from the resolved config).
#[derive(Debug, Clone, Default)]
pub struct LoadOpts {
    pub range_set: String,
    /// `unit_system` came from a flag or the environment.
    pub force_units: bool,
    /// `range_flavor` came from a flag or the environment.
    pub force_flavor: bool,
    pub borderline_margin: f64,
}

/// One row of `profile list`.
#[derive(Debug, Clone)]
pub struct Entry {
    pub kind: &'static str,
    pub name: String,
    pub extends: Option<String>,
    pub description: String,
    /// `None` for built-ins.
    pub origin: Option<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub struct Profiles {
    presets: BTreeMap<String, UnitPreset>,
    range_sets: BTreeMap<String, RangeSet>,
    people: BTreeMap<String, PersonProfile>,
    /// `[units]` in config.toml: overrides on top of the global preset.
    overrides: BTreeMap<String, String>,
    /// Global `range_set` setting.
    default_range_set: Option<String>,
    /// `--units` / `BIOMARKER_UNITS` was given: it beats per-person unit settings.
    force_units: bool,
    /// Same for `range_flavor`.
    force_flavor: bool,
    /// The `borderline_margin` setting (percent).
    pub borderline_margin: f64,
    extra_conversions: Vec<ConversionSpec>,
    compiled_people: BTreeMap<String, Vec<Compiled>>,
    compiled_sets: BTreeMap<String, Vec<Compiled>>,
}

fn parse<T: DeserializeOwned>(text: &str, origin: &str) -> Result<T> {
    toml::from_str(text).map_err(|e| AppError::config(format!("{origin}: {e}")))
}

/// `(lowercase file stem, path, parsed)` for every `*.toml` in `dir` (missing dir: none).
fn read_dir<T: DeserializeOwned>(dir: &Path) -> Result<Vec<(String, PathBuf, T)>> {
    let Ok(rd) = std::fs::read_dir(dir) else { return Ok(Vec::new()) };
    let mut files: Vec<PathBuf> =
        rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "toml")).collect();
    files.sort();
    files
        .into_iter()
        .map(|p| {
            let stem = p.file_stem().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
            let text =
                std::fs::read_to_string(&p).map_err(|e| AppError::io(format!("reading {}: {e}", p.display())))?;
            parse(&text, &p.display().to_string()).map(|v| (stem, p, v))
        })
        .collect()
}

/// `name`, then what it extends, and so on (child first). Errors on an
/// unknown name or a cycle.
fn chain<'a, T>(
    map: &'a BTreeMap<String, T>,
    name: &str,
    parent: impl Fn(&T) -> Option<&str>,
    what: &str,
) -> Result<Vec<&'a T>> {
    let mut out = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut cur = Some(name.to_lowercase());
    while let Some(n) = cur {
        if seen.contains(&n) {
            return Err(AppError::config(format!("{what} '{n}' extends itself")));
        }
        let t = map.get(&n).ok_or_else(|| {
            AppError::config(format!(
                "unknown {what} '{n}' (known: {})",
                map.keys().cloned().collect::<Vec<_>>().join(", ")
            ))
        })?;
        cur = parent(t).map(str::to_lowercase);
        seen.push(n);
        out.push(t);
    }
    Ok(out)
}

/// Marker slug, then its aliases, then `@category`.
fn lookup<'a>(map: &'a BTreeMap<String, String>, m: &Marker) -> Option<&'a String> {
    map.get(&m.slug)
        .or_else(|| m.aliases.iter().find_map(|a| map.get(&a.to_lowercase())))
        .or_else(|| map.get(&format!("@{}", m.category)))
}

fn lowercase_keys(m: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    m.iter().map(|(k, v)| (k.trim().to_lowercase(), v.trim().to_string())).collect()
}

impl Profiles {
    /// Built-ins plus the files under the config file's directory.
    pub fn load(config_path: &Path, opts: &LoadOpts) -> Result<Self> {
        let dir = config_path.parent().map(Path::to_path_buf).unwrap_or_default();
        let mut p = Self {
            default_range_set: Some(opts.range_set.trim().to_lowercase()).filter(|s| !s.is_empty()),
            force_units: opts.force_units,
            force_flavor: opts.force_flavor,
            borderline_margin: opts.borderline_margin,
            ..Self::default()
        };
        for (name, text) in BUILTIN_PRESETS {
            p.presets.insert((*name).to_string(), parse(text, &format!("built-in preset {name}"))?);
        }
        #[derive(Deserialize)]
        struct Extra {
            #[serde(default, rename = "conversion")]
            conversions: Vec<ConversionSpec>,
        }
        p.extra_conversions = parse::<Extra>(BUILTIN_CONVERSIONS, "built-in conversions")?.conversions;
        for (name, path, mut v) in read_dir::<UnitPreset>(&dir.join("units"))? {
            v.units = lowercase_keys(&v.units);
            v.origin = Some(path);
            p.presets.insert(name, v);
        }
        for (name, path, mut v) in read_dir::<RangeSet>(&dir.join("ranges"))? {
            v.origin = Some(path);
            p.range_sets.insert(name, v);
        }
        for (name, path, mut v) in read_dir::<PersonProfile>(&dir.join("people"))? {
            v.units = lowercase_keys(&v.units);
            v.origin = Some(path);
            p.people.insert(name, v);
        }
        p.overrides = overrides_from_config(config_path)?;
        Ok(p)
    }

    pub fn entries(&self) -> Vec<Entry> {
        let presets = self.presets.iter().map(|(n, v)| Entry {
            kind: "unit-preset",
            name: n.clone(),
            extends: v.extends.clone(),
            description: v.description.clone(),
            origin: v.origin.clone(),
        });
        let sets = self.range_sets.iter().map(|(n, v)| Entry {
            kind: "range-set",
            name: n.clone(),
            extends: v.extends.clone(),
            description: v.description.clone(),
            origin: v.origin.clone(),
        });
        let people = self.people.iter().map(|(n, v)| Entry {
            kind: "person",
            name: n.clone(),
            extends: None,
            description: v.description.clone(),
            origin: v.origin.clone(),
        });
        presets.chain(sets).chain(people).collect()
    }

    /// The person's own `range_flavor`, unless a flag or variable forced one.
    pub fn flavor_for(&self, person: &str) -> Option<&str> {
        (!self.force_flavor).then(|| self.people.get(person).and_then(|p| p.range_flavor.as_deref())).flatten()
    }

    pub fn has_preset(&self, name: &str) -> bool {
        self.presets.contains_key(&name.to_lowercase())
    }

    pub fn has_person(&self, slug: &str) -> bool {
        self.people.contains_key(&slug.to_lowercase())
    }

    // -- units -------------------------------------------------------------

    /// The preset in effect for `person`: their own `unit_preset`, unless a
    /// flag / environment variable forced `system`.
    pub fn preset_name<'a>(&'a self, person: Option<&str>, system: &'a str) -> &'a str {
        (!self.force_units)
            .then(|| person.and_then(|p| self.people.get(p)).and_then(|p| p.unit_preset.as_deref()))
            .flatten()
            .unwrap_or(system)
    }

    /// The fallback unit system (`canonical` / `us` / `si`) a preset resolves to.
    pub fn system_tag(&self, preset: &str) -> Option<&str> {
        chain(&self.presets, preset, |p| p.extends.as_deref(), "unit preset")
            .ok()?
            .into_iter()
            .find_map(|p| p.system.as_deref())
    }

    /// The unit explicitly chosen for `m`, if any: person `[units]`, then
    /// config `[units]` (global preset only), then the preset chain.
    pub fn unit_for(&self, person: Option<&str>, system: &str, m: &Marker) -> Option<String> {
        let prof = person.and_then(|p| self.people.get(p));
        let preset = self.preset_name(person, system);
        (!self.force_units)
            .then(|| prof.and_then(|p| lookup(&p.units, m)))
            .flatten()
            .or_else(|| (preset == system).then(|| lookup(&self.overrides, m)).flatten())
            .or_else(|| {
                chain(&self.presets, preset, |p| p.extends.as_deref(), "unit preset")
                    .ok()?
                    .into_iter()
                    .find_map(|p| lookup(&p.units, m))
            })
            .cloned()
    }

    // -- ranges ------------------------------------------------------------

    /// The range set that applies to `person`: theirs, else the global one.
    pub fn range_set_for(&self, person: &str) -> Option<&str> {
        self.people.get(person).and_then(|p| p.range_set.as_deref()).or(self.default_range_set.as_deref())
    }

    pub fn personal_ranges(&self, person: &str, marker_id: i64, kind: RangeKind, lab: Option<&str>) -> Vec<Range> {
        self.compiled_people.get(person).map_or_else(Vec::new, |c| select_compiled(c, marker_id, kind, lab))
    }

    pub fn set_ranges(&self, set: &str, marker_id: i64, kind: RangeKind, lab: Option<&str>) -> Vec<Range> {
        self.compiled_sets.get(set).map_or_else(Vec::new, |c| select_compiled(c, marker_id, kind, lab))
    }

    // -- compile -----------------------------------------------------------

    /// Validate everything against the catalog, add the preset conversions
    /// to it and precompile the ranges. Errors name the offending file.
    pub fn apply(mut self, mut cat: Catalog) -> Result<Catalog> {
        self.validate_names()?;
        self.add_conversions(&mut cat)?;
        self.validate_units(&cat)?;
        let people = self
            .people
            .iter()
            .map(|(n, p)| Ok((n.clone(), compile(&cat, &p.ranges, p.origin.as_deref(), n)?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        let sets = self
            .range_sets
            .keys()
            .map(|n| Ok((n.clone(), self.compile_set(&cat, n)?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        self.compiled_people = people;
        self.compiled_sets = sets;
        cat.profiles = self;
        Ok(cat)
    }

    fn validate_names(&self) -> Result<()> {
        self.people.iter().try_for_each(|(n, p)| match p.range_flavor.as_deref() {
            None | Some("reference" | "optimal" | "both") => Ok(()),
            Some(f) => Err(AppError::config(format!(
                "person profile '{n}': range_flavor '{f}' must be reference, optimal or both"
            ))),
        })?;
        self.presets
            .keys()
            .try_for_each(|n| chain(&self.presets, n, |p| p.extends.as_deref(), "unit preset").map(|_| ()))?;
        self.range_sets
            .keys()
            .try_for_each(|n| chain(&self.range_sets, n, |s| s.extends.as_deref(), "range set").map(|_| ()))?;
        let ctx = |p: &PersonProfile| p.origin.as_ref().map_or_else(String::new, |o| o.display().to_string());
        self.people.values().try_for_each(|p| {
            p.unit_preset
                .as_deref()
                .map(|u| chain(&self.presets, u, |p| p.extends.as_deref(), "unit preset").map(|_| ()))
                .transpose()
                .map_err(|e| e.context(ctx(p)))?;
            p.range_set
                .as_deref()
                .map(|s| chain(&self.range_sets, s, |s| s.extends.as_deref(), "range set").map(|_| ()))
                .transpose()
                .map_err(|e| e.context(ctx(p)))
                .map(|_| ())
        })?;
        self.default_range_set
            .as_deref()
            .map(|s| chain(&self.range_sets, s, |s| s.extends.as_deref(), "range set").map(|_| ()))
            .transpose()
            .map_err(|e| e.context("range_set setting"))
            .map(|_| ())
    }

    fn add_conversions(&self, cat: &mut Catalog) -> Result<()> {
        let specs = self.extra_conversions.iter().map(|c| (c, "built-in conversions".to_string())).chain(
            self.presets.iter().flat_map(|(n, p)| p.conversions.iter().map(move |c| (c, format!("unit preset '{n}'")))),
        );
        let added = specs
            .map(|(c, origin)| {
                if c.factor == 0.0 || !c.factor.is_finite() {
                    return Err(AppError::config(format!("{origin}: conversion factor must be a non-zero number")));
                }
                let marker_id = match &c.marker {
                    Some(m) => cat.get(m).map_err(|e| e.context(&origin))?.id,
                    None => 0,
                };
                Ok(Conversion {
                    id: 0,
                    marker_id,
                    from_unit: cat.spell_unit(&c.from),
                    to_unit: cat.spell_unit(&c.to),
                    factor: c.factor,
                    offset: c.offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        cat.conversions.conversions.extend(added);
        Ok(())
    }

    /// Every unit named in a preset, person profile or the config `[units]`
    /// table must be reachable from the marker's canonical unit.
    fn validate_units(&self, cat: &Catalog) -> Result<()> {
        let check = |origin: String, map: &BTreeMap<String, String>| {
            map.iter().try_for_each(|(key, unit)| {
                if key.starts_with('@') {
                    return Ok(());
                }
                let m = cat.get(key).map_err(|e| e.context(&origin))?;
                cat.conversions.reachable(m.id, &m.unit).iter().any(|k| same_unit(k, unit)).then_some(()).ok_or_else(
                    || {
                        AppError::config(format!(
                            "{origin}: {} cannot be shown in '{unit}' (no conversion from {}; add a [[conversion]])",
                            m.slug, m.unit
                        ))
                    },
                )
            })
        };
        self.presets
            .iter()
            .filter(|(_, p)| p.origin.is_some())
            .try_for_each(|(n, p)| check(format!("unit preset '{n}'"), &p.units))?;
        self.people.iter().try_for_each(|(n, p)| check(format!("person profile '{n}'"), &p.units))?;
        check("config.toml [units]".to_string(), &self.overrides)
    }

    fn compile_set(&self, cat: &Catalog, name: &str) -> Result<Vec<Compiled>> {
        chain(&self.range_sets, name, |s| s.extends.as_deref(), "range set")?.into_iter().try_fold(
            Vec::new(),
            |mut acc, s| {
                acc.extend(compile(cat, &s.ranges, s.origin.as_deref(), name)?);
                Ok(acc)
            },
        )
    }
}

fn compile(cat: &Catalog, specs: &[RangeSpec], origin: Option<&Path>, owner: &str) -> Result<Vec<Compiled>> {
    let where_ = origin.map_or_else(|| owner.to_string(), |o| o.display().to_string());
    specs.iter().map(|s| compile_one(cat, s).map_err(|e| e.context(&where_))).collect()
}

fn compile_one(cat: &Catalog, s: &RangeSpec) -> Result<Compiled> {
    let m = cat.get(&s.marker)?;
    let kind = RangeKind::parse(s.kind.as_deref().unwrap_or("reference"))
        .ok_or_else(|| AppError::config(format!("{}: kind must be reference or optimal", m.slug)))?;
    let sex = s.sex.as_deref().unwrap_or("any").to_ascii_lowercase();
    if !matches!(sex.as_str(), "any" | "male" | "female") {
        return Err(AppError::config(format!("{}: sex must be any, male or female", m.slug)));
    }
    let (age_min, age_max) = (s.age_min.unwrap_or(0.0), s.age_max.unwrap_or(200.0));
    if age_min >= age_max {
        return Err(AppError::config(format!("{}: age_min must be below age_max", m.slug)));
    }
    if s.low.is_none() && s.high.is_none() {
        return Err(AppError::config(format!("{}: give at least one of low / high", m.slug)));
    }
    let unit = s.unit.clone().unwrap_or_else(|| m.unit.clone());
    let conv = |v: Option<f64>| v.map(|x| cat.to_canonical(m, x, &unit)).transpose();
    let (low, high) = (conv(s.low)?, conv(s.high)?);
    let (critical_low, critical_high) = (conv(s.critical_low)?, conv(s.critical_high)?);
    if critical_low.zip(low).is_some_and(|(c, l)| c > l) || critical_high.zip(high).is_some_and(|(c, h)| c < h) {
        return Err(AppError::config(format!("{}: critical bounds must lie outside low / high", m.slug)));
    }
    if let (Some(l), Some(h)) = (low, high) {
        if l > h {
            return Err(AppError::config(format!(
                "{}: low ({}) is above high ({})",
                m.slug,
                s.low.unwrap_or(l),
                s.high.unwrap_or(h)
            )));
        }
    }
    Ok(Compiled {
        marker_id: m.id,
        lab: s.lab.as_ref().map(|l| l.trim().to_lowercase()).filter(|l| !l.is_empty()),
        range: Range {
            id: 0,
            marker_id: m.id,
            kind,
            sex,
            age_min,
            age_max,
            low,
            high,
            note: s.note.clone(),
            person_id: None,
            critical_low,
            critical_high,
        },
    })
}

/// Entries for this marker and kind; lab-specific ones replace the general
/// ones when any matches the measurement's lab.
fn select_compiled(all: &[Compiled], marker_id: i64, kind: RangeKind, lab: Option<&str>) -> Vec<Range> {
    let lab = lab.map(str::to_lowercase);
    let mine: Vec<&Compiled> = all.iter().filter(|c| c.marker_id == marker_id && c.range.kind == kind).collect();
    let specific: Vec<&Compiled> = mine.iter().copied().filter(|c| c.lab.is_some() && c.lab == lab).collect();
    let chosen = if specific.is_empty() { mine.into_iter().filter(|c| c.lab.is_none()).collect() } else { specific };
    chosen.into_iter().map(|c| c.range.clone()).collect()
}

/// A `[name]` table of config.toml (empty when absent).
fn config_table(path: &Path, name: &str) -> Result<BTreeMap<String, toml::Value>> {
    let Ok(text) = std::fs::read_to_string(path) else { return Ok(BTreeMap::new()) };
    let (head, _) = crate::enc_config::split(&text);
    let table: toml::Table = head.parse().map_err(|e| AppError::config(format!("{}: {e}", path.display())))?;
    match table.get(name) {
        None => Ok(BTreeMap::new()),
        Some(toml::Value::Table(t)) => Ok(t.iter().map(|(k, v)| (k.trim().to_lowercase(), v.clone())).collect()),
        Some(_) => Err(AppError::config(format!("{}: '{name}' must be a table", path.display()))),
    }
}

fn string_table(path: &Path, name: &str) -> Result<BTreeMap<String, String>> {
    config_table(path, name)?
        .into_iter()
        .map(|(k, v)| match v {
            toml::Value::String(s) => Ok((k, s.trim().to_string())),
            _ => Err(AppError::config(format!("{}: [{name}] {k} must be a string", path.display()))),
        })
        .collect()
}

/// `[units]` table of config.toml (marker -> unit).
fn overrides_from_config(path: &Path) -> Result<BTreeMap<String, String>> {
    string_table(path, "units")
}

/// `[unit_aliases]` of config.toml: extra unit spellings (alias -> unit).
pub fn unit_aliases(config_path: &Path) -> Result<BTreeMap<String, String>> {
    string_table(config_path, "unit_aliases")
}

/// Per-marker display precision: the `[marker_precision]` table of
/// config.toml (marker slug or `@category` -> decimals) and each person
/// profile's own. Lenient: a broken file is ignored here and reported by the
/// commands that load profiles.
#[derive(Debug, Clone, Default)]
pub struct PrecisionRules {
    global: BTreeMap<String, usize>,
    people: BTreeMap<String, BTreeMap<String, usize>>,
}

impl PrecisionRules {
    pub fn load(config_path: &Path) -> Self {
        let digits = |m: BTreeMap<String, toml::Value>| {
            m.into_iter()
                .filter_map(|(k, v)| v.as_integer().and_then(|n| usize::try_from(n).ok()).map(|n| (k, n.min(12))))
                .collect::<BTreeMap<_, _>>()
        };
        let dir = config_path.parent().map(Path::to_path_buf).unwrap_or_default();
        Self {
            global: config_table(config_path, "marker_precision").map(digits).unwrap_or_default(),
            people: read_dir::<PersonProfile>(&dir.join("people"))
                .unwrap_or_default()
                .into_iter()
                .map(|(n, _, p)| {
                    (n, p.marker_precision.into_iter().map(|(k, v)| (k.to_lowercase(), v.min(12))).collect())
                })
                .collect(),
        }
    }

    /// Decimals for a row of `person` / `marker` / `category`, if configured:
    /// the person's setting wins, then the global one; slug before `@category`.
    pub fn for_row(&self, person: Option<&str>, marker: &str, category: Option<&str>) -> Option<usize> {
        let find = |m: &BTreeMap<String, usize>| {
            m.get(marker).or_else(|| category.and_then(|c| m.get(&format!("@{c}")))).copied()
        };
        person.and_then(|p| self.people.get(p)).and_then(find).or_else(|| find(&self.global))
    }
}

/// Write a starter profile for `slug` (refuses to overwrite). Returns its path.
pub fn init_person(
    config_path: &Path,
    slug: &str,
    unit_preset: Option<&str>,
    range_set: Option<&str>,
) -> Result<PathBuf> {
    let dir = config_path.parent().map(Path::to_path_buf).unwrap_or_default().join("people");
    let path = dir.join(format!("{}.toml", slug.to_lowercase()));
    if path.exists() {
        return Err(AppError::invalid(format!("{} already exists", path.display())));
    }
    let line = |k: &str, v: Option<&str>, example: &str| match v {
        Some(v) => format!("{k} = \"{v}\"\n"),
        None => format!("# {k} = \"{example}\"\n"),
    };
    let text = format!(
        "# Profile for {slug}. Everything here is optional.\n\
         {}{}\n\
         # Per-marker display units for this person (marker slug, alias or @category):\n\
         # [units]\n# glucose = \"mmol/L\"\n\n\
         # Personal ranges. They beat the range set and the catalog; add sex / age_min /\n\
         # age_max for bands and lab = \"quest\" to apply only to that lab's results.\n\
         # [[range]]\n# marker = \"ldl-c\"\n# kind = \"reference\"\n# high = 90\n# unit = \"mg/dL\"\n",
        line("unit_preset", unit_preset, "si"),
        line("range_set", range_set, "labcorp"),
    );
    std::fs::create_dir_all(&dir).map_err(|e| AppError::io(format!("creating {}: {e}", dir.display())))?;
    std::fs::write(&path, text).map_err(|e| AppError::io(format!("writing {}: {e}", path.display())))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn marker(slug: &str, category: &str, aliases: &[&str]) -> Marker {
        Marker {
            id: 1,
            slug: slug.into(),
            name: slug.into(),
            category: category.into(),
            unit: "mg/dL".into(),
            loinc: None,
            description: None,
            builtin: false,
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn builtin() -> Profiles {
        Profiles::load(Path::new("/nonexistent/config.toml"), &LoadOpts::default()).unwrap()
    }

    #[test]
    fn lookup_prefers_slug_then_alias_then_category() {
        let m = marker("glucose", "metabolic", &["glu"]);
        assert_eq!(lookup(&map(&[("glucose", "a"), ("@metabolic", "c")]), &m).map(String::as_str), Some("a"));
        assert_eq!(lookup(&map(&[("glu", "b"), ("@metabolic", "c")]), &m).map(String::as_str), Some("b"));
        assert_eq!(lookup(&map(&[("@metabolic", "c")]), &m).map(String::as_str), Some("c"));
        assert_eq!(lookup(&map(&[("sodium", "x")]), &m), None);
    }

    #[test]
    fn builtin_presets_resolve_their_system() {
        let p = builtin();
        assert_eq!(p.system_tag("us"), Some("us"));
        assert_eq!(p.system_tag("uk"), Some("si"), "uk extends si");
        assert_eq!(p.system_tag("nope"), None);
        let a1c = marker("hba1c", "metabolic", &[]);
        assert_eq!(p.unit_for(None, "uk", &a1c).as_deref(), Some("mmol/mol"));
        assert_eq!(p.unit_for(None, "si", &a1c), None);
    }

    #[test]
    fn chains_detect_cycles_and_unknowns() {
        let mut m: BTreeMap<String, UnitPreset> = BTreeMap::new();
        m.insert("a".into(), UnitPreset { extends: Some("b".into()), ..Default::default() });
        m.insert("b".into(), UnitPreset { extends: Some("a".into()), ..Default::default() });
        assert!(chain(&m, "a", |p| p.extends.as_deref(), "unit preset")
            .unwrap_err()
            .message
            .contains("extends itself"));
        assert!(chain(&m, "zzz", |p| p.extends.as_deref(), "unit preset").unwrap_err().message.contains("unknown"));
    }

    #[test]
    fn person_preset_beats_global_unless_forced() {
        let mut p = builtin();
        p.people.insert("sam".into(), PersonProfile { unit_preset: Some("si".into()), ..Default::default() });
        assert_eq!(p.preset_name(Some("sam"), "us"), "si");
        assert_eq!(p.preset_name(Some("other"), "us"), "us");
        p.force_units = true;
        assert_eq!(p.preset_name(Some("sam"), "us"), "us");
    }

    #[test]
    fn config_overrides_apply_to_the_global_preset_only() {
        let mut p = builtin();
        p.overrides = map(&[("glucose", "mg/mL")]);
        p.people.insert("sam".into(), PersonProfile { unit_preset: Some("si".into()), ..Default::default() });
        let g = marker("glucose", "metabolic", &[]);
        assert_eq!(p.unit_for(None, "us", &g).as_deref(), Some("mg/mL"));
        assert_eq!(p.unit_for(Some("sam"), "us", &g), None, "sam uses si, not the global overrides");
    }

    fn rs(lab: Option<&str>, low: f64) -> Compiled {
        Compiled {
            marker_id: 1,
            lab: lab.map(str::to_string),
            range: Range {
                id: 0,
                marker_id: 1,
                kind: RangeKind::Reference,
                sex: "any".into(),
                age_min: 0.0,
                age_max: 200.0,
                low: Some(low),
                high: None,
                note: None,
                person_id: None,
                critical_low: None,
                critical_high: None,
            },
        }
    }

    #[test]
    fn lab_specific_ranges_replace_general_ones() {
        let all = vec![rs(None, 1.0), rs(Some("quest"), 2.0)];
        let low =
            |lab| select_compiled(&all, 1, RangeKind::Reference, lab).into_iter().map(|r| r.low).collect::<Vec<_>>();
        assert_eq!(low(Some("Quest")), vec![Some(2.0)]);
        assert_eq!(low(Some("labcorp")), vec![Some(1.0)]);
        assert_eq!(low(None), vec![Some(1.0)]);
    }
}
