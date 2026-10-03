//! Reference / optimal range selection and flagging.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RangeKind {
    Reference,
    Optimal,
}

impl RangeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Optimal => "optimal",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "reference" | "ref" => Some(Self::Reference),
            "optimal" | "opt" => Some(Self::Optimal),
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
            sa.cmp(&sb).then(wa.total_cmp(&wb))
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
}
