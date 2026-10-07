//! Reference / optimal range selection and flagging.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RangeKind {
    Reference,
    Optimal,
    /// Near-limit ("approaching") thresholds: `low`/`high` are the warn
    /// bounds that, together with the reference limits, delimit the near-low
    /// and near-high zones (see [`status`]).
    Warn,
}

impl RangeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Optimal => "optimal",
            Self::Warn => "warn",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "reference" | "ref" => Some(Self::Reference),
            "optimal" | "opt" => Some(Self::Optimal),
            "warn" | "near" => Some(Self::Warn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Range {
    pub id: i64,
    pub marker_id: i64,
    pub kind: RangeKind,
    /// `any`, `male`, `female`.
    pub sex: String,
    pub age_min: f64,
    pub age_max: f64,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub note: Option<String>,
    /// `Some` for a person-specific range (overrides the catalog ranges for that person).
    pub person_id: Option<i64>,
    /// Beyond these the result is critical (profile files only; the database has none).
    pub critical_low: Option<f64>,
    pub critical_high: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Flag {
    Low,
    Normal,
    High,
}

impl Flag {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
        }
    }
    pub fn is_out(self) -> bool {
        self != Self::Normal
    }
}

/// Finer-grained than [`Flag`]: where a value sits relative to a range.
/// Ordered from most to least severe by [`Level::severity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Level {
    #[serde(rename = "critical-low")]
    CriticalLow,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "borderline-low")]
    BorderlineLow,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "borderline-high")]
    BorderlineHigh,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical-high")]
    CriticalHigh,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CriticalLow => "critical-low",
            Self::Low => "low",
            Self::BorderlineLow => "borderline-low",
            Self::Normal => "normal",
            Self::BorderlineHigh => "borderline-high",
            Self::High => "high",
            Self::CriticalHigh => "critical-high",
        }
    }

    /// 0 normal, 1 borderline, 2 out of range, 3 critical.
    pub fn severity(self) -> u8 {
        match self {
            Self::Normal => 0,
            Self::BorderlineLow | Self::BorderlineHigh => 1,
            Self::Low | Self::High => 2,
            Self::CriticalLow | Self::CriticalHigh => 3,
        }
    }

    pub fn is_critical(self) -> bool {
        self.severity() == 3
    }
}

/// Classify a value against a range, adding critical bounds and a borderline
/// band: inside the range but within `margin_pct` percent of a bound (of the
/// range width, or of the bound itself for a one-sided range). A margin of 0
/// disables the borderline band.
pub fn level(value: f64, qualifier: Option<&str>, range: &Range, margin_pct: f64) -> Level {
    let censored_above = matches!(qualifier, Some(">") | Some(">="));
    let censored_below = matches!(qualifier, Some("<") | Some("<="));
    if !censored_above && range.critical_low.is_some_and(|c| value < c) {
        return Level::CriticalLow;
    }
    if !censored_below && range.critical_high.is_some_and(|c| value > c) {
        return Level::CriticalHigh;
    }
    match flag(value, qualifier, range) {
        Flag::Low => Level::Low,
        Flag::High => Level::High,
        Flag::Normal => {
            let span = match (range.low, range.high) {
                (Some(l), Some(h)) => (h - l).abs(),
                (Some(b), None) | (None, Some(b)) => b.abs(),
                (None, None) => 0.0,
            };
            let band = span * margin_pct / 100.0;
            if band > 0.0 && range.low.is_some_and(|l| value >= l && value - l < band) && !censored_below {
                Level::BorderlineLow
            } else if band > 0.0 && range.high.is_some_and(|h| value <= h && h - value < band) && !censored_above {
                Level::BorderlineHigh
            } else {
                Level::Normal
            }
        }
    }
}

/// Where a value sits relative to the reference range and the near-limit
/// (warn) bounds: the per-row `status` of the JSON interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Low,
    NearLow,
    InRange,
    NearHigh,
    High,
    Unknown,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::NearLow => "near-low",
            Self::InRange => "in-range",
            Self::NearHigh => "near-high",
            Self::High => "high",
            Self::Unknown => "unknown",
        }
    }
}

/// Classify a value by the reference range and the optional warn bounds.
///
/// On each side the reference limit and the warn bound are two cut points:
/// beyond the outer one is `low`/`high`, between them is `near-low`/
/// `near-high`. A warn bound inside the reference range marks a margin
/// before the limit (eGFR 60-89); one outside it marks an approaching zone
/// before the clinical cut-off (HbA1c 5.7-6.4 above a 5.6 reference limit),
/// so `status` can be `near-high` where `ref_flag` is `high`. Without a
/// reference range the status is `unknown`. Censored values follow
/// [`flag`]: `<x` is never (near-)high and `>x` never (near-)low.
pub fn status(value: f64, qualifier: Option<&str>, reference: Option<&Range>, warn: Option<&Range>) -> Status {
    let Some(reference) = reference else { return Status::Unknown };
    let warn_low = warn.and_then(|w| w.low);
    let warn_high = warn.and_then(|w| w.high);
    let can_be_low = !matches!(qualifier, Some(">") | Some(">="));
    let can_be_high = !matches!(qualifier, Some("<") | Some("<="));
    // (outer, inner) cut points; `None` when that side has no bound.
    let low = match (reference.low, warn_low) {
        (Some(r), Some(w)) => (Some(r.min(w)), Some(r.max(w))),
        (r, w) => (r, w),
    };
    let high = match (reference.high, warn_high) {
        (Some(r), Some(w)) => (Some(r.max(w)), Some(r.min(w))),
        (r, w) => (r, w),
    };
    if can_be_low && low.0.is_some_and(|lo| value < lo) {
        Status::Low
    } else if can_be_high && high.0.is_some_and(|hi| value > hi) {
        Status::High
    } else if can_be_low && low.1.is_some_and(|lo| value < lo) {
        Status::NearLow
    } else if can_be_high && high.1.is_some_and(|hi| value > hi) {
        Status::NearHigh
    } else {
        Status::InRange
    }
}

/// Pick the most specific range of `kind` for a marker given sex and age.
/// Sex-specific beats `any`; narrower age bands beat wider ones. When the age
/// is unknown, the most general (widest) band is preferred instead.
pub fn select<'a>(
    ranges: &'a [Range],
    marker_id: i64,
    kind: RangeKind,
    sex: Option<&str>,
    age: Option<f64>,
) -> Option<&'a Range> {
    select_by(ranges, marker_id, kind, sex, age, false)
}

/// [`select`] with a choice of what matters first: with `age_first`, the
/// narrowest age band wins and sex only breaks ties.
pub fn select_by<'a>(
    ranges: &'a [Range],
    marker_id: i64,
    kind: RangeKind,
    sex: Option<&str>,
    age: Option<f64>,
    age_first: bool,
) -> Option<&'a Range> {
    let sex = sex.map(str::to_ascii_lowercase);
    ranges
        .iter()
        .filter(|r| r.marker_id == marker_id && r.kind == kind)
        .filter(|r| r.sex == "any" || sex.as_deref() == Some(r.sex.as_str()))
        .filter(|r| age.is_none_or(|a| a >= r.age_min && a < r.age_max))
        .min_by(|a, b| {
            let width = |r: &Range| if age.is_some() { r.age_max - r.age_min } else { r.age_min - r.age_max };
            let spec = |r: &Range| (i32::from(r.sex == "any"), width(r));
            let (sa, wa) = spec(a);
            let (sb, wb) = spec(b);
            if age_first {
                wa.total_cmp(&wb).then(sa.cmp(&sb))
            } else {
                sa.cmp(&sb).then(wa.total_cmp(&wb))
            }
        })
}

/// Classify a value against a range. Censored values (`<x`, `>x`) are judged
/// by the side of the range they can lie on: `<0.5` against a high limit of 3
/// is normal; `>1000` against a high limit of 200 is high.
pub fn flag(value: f64, qualifier: Option<&str>, range: &Range) -> Flag {
    let below = range.low.is_some_and(|lo| match qualifier {
        Some(">") | Some(">=") => false,
        _ => value < lo,
    });
    let above = range.high.is_some_and(|hi| match qualifier {
        Some("<") | Some("<=") => false,
        _ => value > hi,
    });
    if below {
        Flag::Low
    } else if above {
        Flag::High
    } else {
        Flag::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(id: i64, kind: RangeKind, sex: &str, ages: (f64, f64), lo: Option<f64>, hi: Option<f64>) -> Range {
        Range {
            id,
            marker_id: 1,
            kind,
            sex: sex.into(),
            age_min: ages.0,
            age_max: ages.1,
            low: lo,
            high: hi,
            note: None,
            person_id: None,
            critical_low: None,
            critical_high: None,
        }
    }

    #[test]
    fn selects_most_specific() {
        let rs = vec![
            r(1, RangeKind::Reference, "any", (0.0, 200.0), Some(10.0), Some(20.0)),
            r(2, RangeKind::Reference, "male", (0.0, 200.0), Some(12.0), Some(22.0)),
            r(3, RangeKind::Reference, "male", (50.0, 200.0), Some(8.0), Some(18.0)),
            r(4, RangeKind::Optimal, "any", (0.0, 200.0), Some(14.0), Some(16.0)),
        ];
        let pick = |sex: Option<&str>, age| select(&rs, 1, RangeKind::Reference, sex, age).map(|r| r.id);
        assert_eq!(pick(None, None), Some(1));
        assert_eq!(pick(Some("male"), None), Some(2));
        assert_eq!(pick(Some("female"), Some(30.0)), Some(1));
        assert_eq!(pick(Some("male"), Some(30.0)), Some(2));
        assert_eq!(pick(Some("Male"), Some(60.0)), Some(3));
        assert_eq!(select(&rs, 1, RangeKind::Optimal, None, None).map(|r| r.id), Some(4));
        assert_eq!(select(&rs, 2, RangeKind::Optimal, None, None), None);
    }

    #[test]
    fn levels_add_critical_and_borderline() {
        let mut range = r(1, RangeKind::Reference, "any", (0.0, 200.0), Some(10.0), Some(20.0));
        range.critical_low = Some(5.0);
        range.critical_high = Some(30.0);
        let lv = |v, q, m| level(v, q, &range, m);
        assert_eq!(lv(15.0, None, 0.0), Level::Normal);
        assert_eq!(lv(10.5, None, 0.0), Level::Normal, "no margin, no borderline");
        assert_eq!(lv(10.5, None, 10.0), Level::BorderlineLow);
        assert_eq!(lv(19.5, None, 10.0), Level::BorderlineHigh);
        assert_eq!(lv(15.0, None, 10.0), Level::Normal);
        assert_eq!(lv(8.0, None, 10.0), Level::Low);
        assert_eq!(lv(22.0, None, 10.0), Level::High);
        assert_eq!(lv(4.0, None, 10.0), Level::CriticalLow);
        assert_eq!(lv(31.0, None, 10.0), Level::CriticalHigh);
        assert_eq!(lv(31.0, Some("<"), 10.0), Level::Normal, "a '<31' result may well be normal");
        assert!(Level::CriticalHigh.severity() > Level::High.severity());
        assert!(Level::High.severity() > Level::BorderlineHigh.severity());
    }

    #[test]
    fn age_first_prefers_the_narrow_band_over_a_sex_match() {
        let rs = vec![
            r(1, RangeKind::Reference, "male", (0.0, 200.0), Some(12.0), Some(22.0)),
            r(2, RangeKind::Reference, "any", (50.0, 70.0), Some(8.0), Some(18.0)),
        ];
        let by = |first| select_by(&rs, 1, RangeKind::Reference, Some("male"), Some(60.0), first).map(|r| r.id);
        assert_eq!(by(false), Some(1), "default: sex first");
        assert_eq!(by(true), Some(2), "age first");
    }

    #[test]
    fn flags_values() {
        let range = r(1, RangeKind::Reference, "any", (0.0, 200.0), Some(10.0), Some(20.0));
        assert_eq!(flag(5.0, None, &range), Flag::Low);
        assert_eq!(flag(10.0, None, &range), Flag::Normal);
        assert_eq!(flag(20.0, None, &range), Flag::Normal);
        assert_eq!(flag(25.0, None, &range), Flag::High);
        assert_eq!(flag(5.0, Some(">"), &range), Flag::Normal);
        assert_eq!(flag(25.0, Some("<"), &range), Flag::Normal);
        let open = r(2, RangeKind::Reference, "any", (0.0, 200.0), None, Some(3.0));
        assert_eq!(flag(0.1, Some("<"), &open), Flag::Normal);
        assert_eq!(flag(4.0, None, &open), Flag::High);
    }

    #[test]
    fn status_with_warn_outside_reference() {
        // HbA1c: reference 4.0-5.6, prediabetes 5.7-6.4 approaches the 6.5 cut-off
        let reference = r(1, RangeKind::Reference, "any", (0.0, 200.0), Some(4.0), Some(5.6));
        let warn = r(2, RangeKind::Warn, "any", (0.0, 200.0), None, Some(6.4));
        let st = |v, q| status(v, q, Some(&reference), Some(&warn));
        assert_eq!(st(3.9, None), Status::Low);
        assert_eq!(st(5.2, None), Status::InRange);
        assert_eq!(st(5.6, None), Status::InRange);
        assert_eq!(st(5.7, None), Status::NearHigh);
        assert_eq!(st(6.4, None), Status::NearHigh);
        assert_eq!(st(6.5, None), Status::High);
        assert_eq!(st(7.0, Some("<")), Status::InRange);
        assert_eq!(flag(5.9, None, &reference), Flag::High);
    }

    #[test]
    fn status_with_warn_inside_reference() {
        // eGFR: reference >= 60, 60-89 is mildly decreased
        let reference = r(1, RangeKind::Reference, "any", (0.0, 200.0), Some(60.0), None);
        let warn = r(2, RangeKind::Warn, "any", (0.0, 200.0), Some(90.0), None);
        let st = |v, q| status(v, q, Some(&reference), Some(&warn));
        assert_eq!(st(55.0, None), Status::Low);
        assert_eq!(st(60.0, None), Status::NearLow);
        assert_eq!(st(89.0, None), Status::NearLow);
        assert_eq!(st(90.0, None), Status::InRange);
        assert_eq!(st(150.0, None), Status::InRange);
        assert_eq!(st(30.0, Some(">")), Status::InRange);
    }

    #[test]
    fn status_without_warn_or_reference() {
        let reference = r(1, RangeKind::Reference, "any", (0.0, 200.0), Some(10.0), Some(20.0));
        assert_eq!(status(5.0, None, Some(&reference), None), Status::Low);
        assert_eq!(status(15.0, None, Some(&reference), None), Status::InRange);
        assert_eq!(status(25.0, None, Some(&reference), None), Status::High);
        assert_eq!(status(15.0, None, None, None), Status::Unknown);
        assert_eq!(Status::NearHigh.as_str(), "near-high");
        assert_eq!(RangeKind::parse("warn"), Some(RangeKind::Warn));
    }
}
