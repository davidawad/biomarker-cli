//! Evaluating stored measurements for display: display units, applicable
//! ranges and flags, and the stable `biomarker/v1` measurement record.

use serde_json::json;

use crate::output::Record;
use crate::ranges::{self, Flag, Level, Range, RangeKind};
use crate::store::{Catalog, Marker, MeasurementRow};
use crate::util::{age_years, date_of};

#[derive(Debug, Clone)]
pub struct Evaluated {
    pub row: MeasurementRow,
    pub marker: Marker,
    pub display_value: f64,
    pub display_unit: String,
    pub reference: Option<Range>,
    pub optimal: Option<Range>,
    pub ref_flag: Option<Flag>,
    pub opt_flag: Option<Flag>,
    pub ref_level: Option<Level>,
    pub opt_level: Option<Level>,
    /// The person's own `range_flavor`, when their profile sets one.
    pub flavor: Option<String>,
}

fn age_at(row: &MeasurementRow) -> Option<f64> {
    let dob = row.dob.as_deref().and_then(|d| date_of(d).ok())?;
    date_of(&row.taken_at).ok().map(|on| age_years(dob, on))
}

/// The range that applies, and where it came from: the person's own ranges
/// (profile file, then `range set --person`), their range set, the catalog.
#[allow(clippy::too_many_arguments)]
pub fn pick_range(
    cat: &Catalog,
    person: &str,
    person_id: i64,
    marker: &Marker,
    kind: RangeKind,
    sex: Option<&str>,
    age: Option<f64>,
    lab: Option<&str>,
) -> Option<(Range, String)> {
    let mut own = cat.profiles.personal_ranges(person, marker.id, kind, lab);
    own.extend(
        cat.person_ranges
            .iter()
            .filter(|r| r.person_id == Some(person_id) && r.marker_id == marker.id && r.kind == kind)
            .cloned(),
    );
    ranges::select(&own, marker.id, kind, sex, age)
        .map(|r| (r.clone(), "person".to_string()))
        .or_else(|| {
            let set = cat.profiles.range_set_for(person)?;
            let in_set = cat.profiles.set_ranges(set, marker.id, kind, lab);
            ranges::select(&in_set, marker.id, kind, sex, age).map(|r| (r.clone(), format!("set:{set}")))
        })
        .or_else(|| ranges::select(&cat.ranges, marker.id, kind, sex, age).map(|r| (r.clone(), "catalog".to_string())))
}

pub fn evaluate(row: &MeasurementRow, cat: &Catalog, unit_system: &str) -> Option<Evaluated> {
    let marker = cat.by_id(row.marker_id)?.clone();
    let wanted = cat.display_unit(&marker, unit_system, Some(&row.person));
    let (display_value, display_unit) = cat
        .conversions
        .convert(marker.id, row.value, &marker.unit, &wanted)
        .map_or_else(|_| (row.value, marker.unit.clone()), |v| (v, wanted));
    let age = age_at(row);
    let pick = |kind| {
        pick_range(cat, &row.person, row.person_id, &marker, kind, row.sex.as_deref(), age, row.lab.as_deref())
            .map(|(r, _)| r)
    };
    let reference = pick(RangeKind::Reference);
    let optimal = pick(RangeKind::Optimal);
    let q = row.qualifier.as_deref();
    let margin = cat.profiles.borderline_margin;
    Some(Evaluated {
        ref_flag: reference.as_ref().map(|r| ranges::flag(row.value, q, r)),
        opt_flag: optimal.as_ref().map(|r| ranges::flag(row.value, q, r)),
        ref_level: reference.as_ref().map(|r| ranges::level(row.value, q, r, margin)),
        opt_level: optimal.as_ref().map(|r| ranges::level(row.value, q, r, margin)),
        flavor: cat.profiles.flavor_for(&row.person).map(str::to_string),
        row: row.clone(),
        marker,
        display_value,
        display_unit,
        reference,
        optimal,
    })
}

impl Evaluated {
    /// Overall flag for the configured range flavor.
    pub fn flag(&self, flavor: &str) -> Option<Flag> {
        match self.flavor.as_deref().unwrap_or(flavor) {
            "optimal" => self.opt_flag,
            "both" => match (self.ref_flag, self.opt_flag) {
                (Some(r), _) if r.is_out() => Some(r),
                (r, Some(o)) if o.is_out() => Some(o).or(r),
                (r, o) => r.or(o),
            },
            _ => self.ref_flag,
        }
    }

    /// Severity-aware classification for the configured range flavor
    /// (with `both`, the more severe of the two).
    pub fn level(&self, flavor: &str) -> Option<Level> {
        match self.flavor.as_deref().unwrap_or(flavor) {
            "optimal" => self.opt_level,
            "both" => match (self.ref_level, self.opt_level) {
                (Some(r), Some(o)) => Some(if o.severity() > r.severity() { o } else { r }),
                (r, o) => r.or(o),
            },
            _ => self.ref_level,
        }
    }

    pub fn is_flagged(&self, flavor: &str) -> bool {
        self.flag(flavor).is_some_and(Flag::is_out)
    }

    /// Convert a canonical-unit bound into the display unit.
    fn bound(&self, cat: &Catalog, v: Option<f64>) -> Option<f64> {
        v.map(|x| cat.conversions.convert(self.marker.id, x, &self.marker.unit, &self.display_unit).unwrap_or(x))
    }

    /// The stable JSON measurement record (see docs/json-schema.md).
    pub fn record(&self, cat: &Catalog, flavor: &str) -> Record {
        let r = &self.row;
        let flag_str = |f: Option<Flag>| f.map(Flag::as_str);
        let level_str = |l: Option<Level>| l.map(Level::as_str);
        let rec = json!({
            "id": r.id,
            "person": r.person,
            "marker": self.marker.slug,
            "marker_name": self.marker.name,
            "category": self.marker.category,
            "taken_at": r.taken_at,
            "qualifier": r.qualifier,
            "value": self.display_value,
            "unit": self.display_unit,
            "value_raw": r.value_raw,
            "unit_raw": r.unit_raw,
            "value_canonical": r.value,
            "unit_canonical": self.marker.unit,
            "ref_low": self.bound(cat, self.reference.as_ref().and_then(|x| x.low)),
            "ref_high": self.bound(cat, self.reference.as_ref().and_then(|x| x.high)),
            "ref_flag": flag_str(self.ref_flag),
            "opt_low": self.bound(cat, self.optimal.as_ref().and_then(|x| x.low)),
            "opt_high": self.bound(cat, self.optimal.as_ref().and_then(|x| x.high)),
            "opt_flag": flag_str(self.opt_flag),
            "flag": flag_str(self.flag(flavor)),
            "ref_level": level_str(self.ref_level),
            "opt_level": level_str(self.opt_level),
            "level": level_str(self.level(flavor)),
            "lab": r.lab,
            "fasting": r.fasting,
            "note": r.note,
            "tags": r.tags,
            "batch": r.batch_id,
        });
        crate::output::to_record(&rec)
    }
}

pub const MEASUREMENT_TABLE: &[&str] =
    &["id", "person", "taken_at", "marker", "qualifier", "value", "unit", "ref_low", "ref_high", "flag", "lab"];

/// Evaluate a batch of rows.
pub fn evaluate_all(rows: &[MeasurementRow], cat: &Catalog, unit_system: &str) -> Vec<Evaluated> {
    rows.iter().filter_map(|r| evaluate(r, cat, unit_system)).collect()
}

/// Keep only the latest row per (person, marker). Input must be sorted by date ascending.
pub fn latest_only(rows: Vec<Evaluated>) -> Vec<Evaluated> {
    let mut seen = std::collections::HashSet::new();
    let mut out: Vec<Evaluated> =
        rows.into_iter().rev().filter(|e| seen.insert((e.row.person_id, e.row.marker_id))).collect();
    out.reverse();
    out
}
