//! Structured range conditions (`when`) beyond sex and age, the person facts
//! they are matched against, and the overlap test used by the ambiguity lint.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

pub const MENOPAUSE: &[&str] = &["pre", "peri", "post"];
pub const CYCLE_PHASES: &[&str] = &["menstrual", "follicular", "ovulatory", "luteal"];
pub const OPS: &[&str] = &["<", "<=", ">", ">=", "="];

/// `{ marker = "glucose", op = ">", value = 100.0 }`: another marker's
/// latest value (canonical unit, on or before the draw) must satisfy `op value`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkerCond {
    pub marker: String,
    pub op: String,
    pub value: f64,
}

impl MarkerCond {
    pub fn holds(&self, v: f64) -> bool {
        match self.op.as_str() {
            "<" => v < self.value,
            "<=" => v <= self.value,
            ">" => v > self.value,
            ">=" => v >= self.value,
            "=" => v == self.value,
            _ => false,
        }
    }

    /// The values satisfying the condition as `(low, low inclusive, high, high inclusive)`.
    fn interval(&self) -> (f64, bool, f64, bool) {
        let (inf, v) = (f64::INFINITY, self.value);
        match self.op.as_str() {
            "<" => (-inf, false, v, false),
            "<=" => (-inf, false, v, true),
            ">" => (v, false, inf, false),
            ">=" => (v, true, inf, false),
            _ => (v, true, v, true),
        }
    }
}

/// The conditions of a range other than sex and age (those live in
/// [`crate::ranges::Range`] itself). Every given field must hold.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Conditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pregnant: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trimester: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fasting: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menopause: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cycle_phase: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub markers: Vec<MarkerCond>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
}

/// What is known about a person at a draw, for matching [`Conditions`].
pub struct Facts<'a> {
    pub sex: Option<&'a str>,
    pub age: Option<f64>,
    /// The measurement's own `fasting` flag.
    pub fasting: Option<bool>,
    pub attrs: Option<&'a PersonAttrs>,
    /// Latest canonical value of another marker (by slug) on or before the draw.
    pub value_of: &'a dyn Fn(&str) -> Option<f64>,
}

fn no_value(_: &str) -> Option<f64> {
    None
}

impl<'a> Facts<'a> {
    /// Only sex and age are known.
    pub fn basic(sex: Option<&'a str>, age: Option<f64>) -> Self {
        Self { sex, age, fasting: None, attrs: None, value_of: &no_value }
    }
}

impl Conditions {
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// How many conditions are given (each marker condition and each named
    /// condition counts once): the specificity beyond sex and age.
    pub fn count(&self) -> usize {
        usize::from(self.pregnant.is_some())
            + usize::from(self.trimester.is_some())
            + usize::from(self.fasting.is_some())
            + usize::from(self.menopause.is_some())
            + usize::from(self.cycle_phase.is_some())
            + self.markers.len()
            + self.conditions.len()
    }

    /// Names of the dimensions used, for coverage reports.
    pub fn dimensions(&self) -> Vec<&'static str> {
        [
            ("pregnant", self.pregnant.is_some()),
            ("trimester", self.trimester.is_some()),
            ("fasting", self.fasting.is_some()),
            ("menopause", self.menopause.is_some()),
            ("cycle_phase", self.cycle_phase.is_some()),
            ("markers", !self.markers.is_empty()),
            ("conditions", !self.conditions.is_empty()),
        ]
        .into_iter()
        .filter_map(|(n, used)| used.then_some(n))
        .collect()
    }

    /// Every condition holds for `f`. An unknown fact never satisfies a condition.
    pub fn matches(&self, f: &Facts) -> bool {
        let attrs = f.attrs.cloned().unwrap_or_default();
        let eq = |want: Option<&String>, have: Option<&String>| want.is_none_or(|w| have == Some(w));
        self.pregnant.is_none_or(|p| attrs.pregnant == Some(p))
            && self.trimester.is_none_or(|t| attrs.pregnant == Some(true) && attrs.trimester == Some(t))
            && self.fasting.is_none_or(|x| f.fasting == Some(x))
            && eq(self.menopause.as_ref(), attrs.menopause.as_ref())
            && eq(self.cycle_phase.as_ref(), attrs.cycle_phase.as_ref())
            && self.markers.iter().all(|c| (f.value_of)(&c.marker).is_some_and(|v| c.holds(v)))
            && self.conditions.iter().all(|c| attrs.conditions.contains(c))
    }

    /// Could one person satisfy both `self` and `other` at once?
    pub fn overlaps(&self, other: &Self) -> bool {
        let same = |a: &Option<String>, b: &Option<String>| a.is_none() || b.is_none() || a == b;
        let same_v = |a: Option<u8>, b: Option<u8>| a.is_none() || b.is_none() || a == b;
        let same_b = |a: Option<bool>, b: Option<bool>| a.is_none() || b.is_none() || a == b;
        let pregnancy_ok = !(self.trimester.is_some() && other.pregnant == Some(false))
            && !(other.trimester.is_some() && self.pregnant == Some(false));
        same_b(self.pregnant, other.pregnant)
            && same_v(self.trimester, other.trimester)
            && pregnancy_ok
            && same_b(self.fasting, other.fasting)
            && same(&self.menopause, &other.menopause)
            && same(&self.cycle_phase, &other.cycle_phase)
            && markers_overlap(&self.markers, &other.markers)
    }
}

/// Every marker named on both sides has a value range satisfying all its conditions.
fn markers_overlap(a: &[MarkerCond], b: &[MarkerCond]) -> bool {
    let mut by_marker: BTreeMap<&str, Vec<&MarkerCond>> = BTreeMap::new();
    a.iter().chain(b).for_each(|c| by_marker.entry(c.marker.as_str()).or_default().push(c));
    by_marker.values().all(|conds| {
        let (mut lo, mut lo_in, mut hi, mut hi_in) = (f64::NEG_INFINITY, false, f64::INFINITY, false);
        for c in conds {
            let (l, li, h, hi_inc) = c.interval();
            if l > lo || (l == lo && !li) {
                (lo, lo_in) = (l, li);
            }
            if h < hi || (h == hi && !hi_inc) {
                (hi, hi_in) = (h, hi_inc);
            }
        }
        lo < hi || (lo == hi && lo_in && hi_in)
    })
}

/// Person facts that are not columns of their own: pregnancy, menopause,
/// cycle phase and diagnosed conditions (`person add/edit --attr`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonAttrs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pregnant: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trimester: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menopause: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cycle_phase: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
}

pub const ATTR_KEYS: &[&str] = &["pregnant", "trimester", "menopause", "cycle_phase", "conditions"];

fn one_of(key: &str, v: &str, allowed: &[&str]) -> Result<String> {
    let l = v.trim().to_ascii_lowercase();
    if allowed.contains(&l.as_str()) {
        Ok(l)
    } else {
        Err(AppError::usage(format!("{key} must be one of {}", allowed.join(", "))))
    }
}

impl PersonAttrs {
    /// Parse the `attributes` column (lenient: unreadable JSON is empty).
    pub fn from_json(s: Option<&str>) -> Self {
        s.and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }

    /// Apply `key=value` (an empty value clears the attribute).
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let v = value.trim();
        let clear = v.is_empty();
        match key.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "pregnant" => {
                self.pregnant = if clear {
                    None
                } else {
                    Some(match v.to_ascii_lowercase().as_str() {
                        "true" | "yes" | "1" => true,
                        "false" | "no" | "0" => false,
                        _ => return Err(AppError::usage("pregnant must be true or false")),
                    })
                };
                if self.pregnant != Some(true) {
                    self.trimester = None;
                }
            }
            "trimester" => {
                self.trimester = if clear {
                    None
                } else {
                    let t = v.parse::<u8>().ok().filter(|t| (1..=3).contains(t));
                    Some(t.ok_or_else(|| AppError::usage("trimester must be 1, 2 or 3"))?)
                };
                if self.trimester.is_some() {
                    self.pregnant = Some(true);
                }
            }
            "menopause" => self.menopause = (!clear).then(|| one_of("menopause", v, MENOPAUSE)).transpose()?,
            "cycle_phase" => {
                self.cycle_phase = (!clear).then(|| one_of("cycle_phase", v, CYCLE_PHASES)).transpose()?;
            }
            "conditions" => {
                self.conditions =
                    v.split(',').map(|c| c.trim().to_ascii_lowercase()).filter(|c| !c.is_empty()).collect();
                self.conditions.sort();
                self.conditions.dedup();
            }
            other => {
                return Err(AppError::usage(format!(
                    "unknown person attribute '{other}' (known: {})",
                    ATTR_KEYS.join(", ")
                )))
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mc(op: &str, value: f64) -> MarkerCond {
        MarkerCond { marker: "glucose".into(), op: op.into(), value }
    }

    #[test]
    fn conditions_need_known_facts() {
        let c = Conditions { pregnant: Some(true), markers: vec![mc(">", 100.0)], ..Default::default() };
        let attrs = PersonAttrs { pregnant: Some(true), ..Default::default() };
        fn high(_: &str) -> Option<f64> {
            Some(110.0)
        }
        fn low(_: &str) -> Option<f64> {
            Some(90.0)
        }
        let f = |a, v: &'static dyn Fn(&str) -> Option<f64>| Facts {
            sex: None,
            age: None,
            fasting: None,
            attrs: a,
            value_of: v,
        };
        assert!(c.matches(&f(Some(&attrs), &high)));
        assert!(!c.matches(&f(Some(&attrs), &low)));
        assert!(!c.matches(&f(None, &high)), "unknown pregnancy never matches");
        assert!(!c.matches(&Facts::basic(None, None)));
        assert_eq!(c.count(), 2);
        assert!(Conditions::default().matches(&Facts::basic(None, None)));
    }

    #[test]
    fn overlap_of_marker_intervals() {
        let a = Conditions { markers: vec![mc(">", 100.0)], ..Default::default() };
        let b = Conditions { markers: vec![mc("<=", 100.0)], ..Default::default() };
        let c = Conditions { markers: vec![mc(">=", 120.0)], ..Default::default() };
        assert!(!a.overlaps(&b));
        assert!(a.overlaps(&c));
        let d = Conditions { markers: vec![mc(">=", 100.0)], ..Default::default() };
        assert!(d.overlaps(&b), "100 satisfies both");
        let p = Conditions { menopause: Some("post".into()), ..Default::default() };
        let q = Conditions { menopause: Some("pre".into()), ..Default::default() };
        assert!(!p.overlaps(&q));
        assert!(p.overlaps(&Conditions::default()));
    }

    #[test]
    fn attrs_parse_and_validate() {
        let mut a = PersonAttrs::default();
        a.set("trimester", "2").unwrap();
        assert_eq!((a.pregnant, a.trimester), (Some(true), Some(2)));
        a.set("pregnant", "no").unwrap();
        assert_eq!((a.pregnant, a.trimester), (Some(false), None));
        a.set("conditions", "Type-2-Diabetes, ckd,ckd").unwrap();
        assert_eq!(a.conditions, ["ckd", "type-2-diabetes"]);
        assert!(a.set("menopause", "sideways").is_err());
        assert!(a.set("blood_type", "o").is_err());
        a.set("cycle-phase", "luteal").unwrap();
        assert_eq!(PersonAttrs::from_json(Some(&a.to_json())), a);
    }
}
