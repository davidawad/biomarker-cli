//! Unit normalisation and conversion.
//!
//! Conversions are affine edges `to = from * factor + offset`, either
//! marker-specific or generic (apply to every marker). They are usable in both
//! directions; [`ConversionSet::convert`] finds the shortest path (preferring
//! marker-specific edges) with a breadth-first search.

use std::collections::{HashMap, VecDeque};

use crate::error::{AppError, Result};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Conversion {
    pub id: i64,
    /// 0 = generic (applies to all markers).
    pub marker_id: i64,
    pub from_unit: String,
    pub to_unit: String,
    pub factor: f64,
    pub offset: f64,
}

/// User-defined spellings (`[unit_aliases]` in the config file): built-in key
/// of the alias -> built-in key of the unit it means.
static ALIASES: std::sync::RwLock<Option<HashMap<String, String>>> = std::sync::RwLock::new(None);

/// Install extra unit spellings: `alias -> unit` pairs such as
/// `"mcg/dl" -> "µg/dL"`. Replaces any previously installed set.
pub fn set_aliases<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) {
    let map: HashMap<String, String> = pairs.into_iter().map(|(a, t)| (builtin_key(a), builtin_key(t))).collect();
    if let Ok(mut w) = ALIASES.write() {
        *w = Some(map).filter(|m| !m.is_empty());
    }
}

/// Comparison key for unit spellings: case-insensitive, micro-sign agnostic,
/// common lab shorthands folded together (`mcg/dl` == `µg/dL`, `K/uL` == `10^3/µL`),
/// plus the user's `[unit_aliases]`.
pub fn unit_key(u: &str) -> String {
    let k = builtin_key(u);
    ALIASES.read().ok().and_then(|a| a.as_ref().and_then(|m| m.get(&k).cloned())).unwrap_or(k)
}

fn builtin_key(u: &str) -> String {
    let k = u
        .trim()
        .trim_end_matches('.')
        .to_lowercase()
        .replace(['µ', 'μ'], "u")
        .replace("mcg", "ug")
        .replace("mcmol", "umol")
        .replace("mciu", "uiu")
        .replace(' ', "")
        .replace("x10", "10")
        .replace('*', "")
        .replace("e3/", "^3/")
        .replace("e6/", "^6/")
        .replace("e9/", "^9/")
        .replace("e12/", "^12/")
        .replace("m2", "m²");
    match k.as_str() {
        "k/ul" | "thou/ul" | "10^3/mm3" | "103/ul" => "10^3/ul".into(),
        "m/ul" | "mil/ul" | "10^6/mm3" | "106/ul" => "10^6/ul".into(),
        "109/l" => "10^9/l".into(),
        "1012/l" => "10^12/l".into(),
        "percent" | "pct" => "%".into(),
        "lbs" | "pound" | "pounds" => "lb".into(),
        "year" | "yrs" | "yr" => "years".into(),
        "beats/min" | "/min" => "bpm".into(),
        "iu/l" | "u/l" => k,
        "mlmin1.73m²" | "ml/min/1.73" | "ml/min/1.73m^2" => "ml/min/1.73m²".into(),
        _ => k,
    }
}

/// Clean up a unit as written in a spreadsheet before spelling lookup:
/// `μm³ (x10E3/uL)` -> `x10E3/uL` (a parenthesised unit wins), `%Hb` -> `%`,
/// `x10E3/mm3` -> `x10E3/µL`, `μm³` -> `fL`.
pub fn clean_unit(raw: &str) -> String {
    let t = raw.trim();
    let paren = t.find('(').and_then(|o| t[o..].find(')').map(|c| (o, o + c)));
    let t = match paren {
        Some((o, c)) => {
            let inner = t[o + 1..c].trim();
            let outer = format!("{}{}", &t[..o], &t[c + 1..]).trim().to_string();
            if inner.contains(['/', '%', '^']) || outer.is_empty() {
                inner.to_string()
            } else {
                outer
            }
        }
        None => t.to_string(),
    };
    let k = t.to_lowercase().replace(['µ', 'μ'], "u").replace(' ', "");
    if k.starts_with('%') {
        return "%".into();
    }
    if matches!(k.as_str(), "um3" | "um³" | "um^3" | "cubicmicrons") {
        return "fL".into();
    }
    match k.strip_suffix("/mm3").or_else(|| k.strip_suffix("/mm³")) {
        Some(_) => format!("{}/µL", &t[..t.rfind('/').unwrap_or(t.len())]),
        None => t,
    }
}

pub fn same_unit(a: &str, b: &str) -> bool {
    unit_key(a) == unit_key(b)
}

/// Pick the canonical spelling for `u` from the known unit symbols, if any.
pub fn canonical_spelling(u: &str, known: &[String]) -> String {
    let k = unit_key(u);
    known.iter().find(|s| unit_key(s) == k).cloned().unwrap_or_else(|| u.trim().to_string())
}

#[derive(Debug, Clone, Default)]
pub struct ConversionSet {
    pub conversions: Vec<Conversion>,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    factor: f64,
    offset: f64,
    inverse: bool,
}

impl Edge {
    fn apply(self, v: f64) -> f64 {
        if self.inverse {
            (v - self.offset) / self.factor
        } else {
            v * self.factor + self.offset
        }
    }
}

impl ConversionSet {
    pub fn new(conversions: Vec<Conversion>) -> Self {
        Self { conversions }
    }

    /// Edges usable for `marker_id`, marker-specific first.
    fn edges(&self, marker_id: i64) -> HashMap<String, Vec<(String, Edge)>> {
        let mut relevant: Vec<&Conversion> = self
            .conversions
            .iter()
            .filter(|c| c.marker_id == marker_id || c.marker_id == 0)
            .filter(|c| c.factor != 0.0)
            .collect();
        relevant.sort_by_key(|c| i64::from(c.marker_id == 0));
        relevant.iter().fold(HashMap::new(), |mut g, c| {
            let (f, t) = (unit_key(&c.from_unit), unit_key(&c.to_unit));
            let fwd = Edge { factor: c.factor, offset: c.offset, inverse: false };
            let inv = Edge { inverse: true, ..fwd };
            g.entry(f.clone()).or_insert_with(Vec::new).push((t.clone(), fwd));
            g.entry(t).or_insert_with(Vec::new).push((f, inv));
            g
        })
    }

    /// Convert `value` from unit `from` to unit `to` for the given marker.
    pub fn convert(&self, marker_id: i64, value: f64, from: &str, to: &str) -> Result<f64> {
        let (src, dst) = (unit_key(from), unit_key(to));
        if src == dst {
            return Ok(value);
        }
        let graph = self.edges(marker_id);
        let mut prev: HashMap<String, (String, Edge)> = HashMap::new();
        let mut queue = VecDeque::from([src.clone()]);
        while let Some(u) = queue.pop_front() {
            if u == dst {
                break;
            }
            for (v, e) in graph.get(&u).into_iter().flatten() {
                if *v != src && !prev.contains_key(v) {
                    prev.insert(v.clone(), (u.clone(), *e));
                    queue.push_back(v.clone());
                }
            }
        }
        // Reconstruct the path back from dst, then apply edges forward.
        let path = std::iter::successors(Some(dst.clone()), |n| prev.get(n).map(|(p, _)| p.clone()))
            .take_while(|n| *n != src)
            .map(|n| prev.get(&n).map(|(_, e)| *e))
            .collect::<Option<Vec<Edge>>>()
            .filter(|p| !p.is_empty())
            .ok_or_else(|| AppError::invalid(format!("no unit conversion from '{from}' to '{to}'")))?;
        Ok(path.iter().rev().fold(value, |v, e| e.apply(v)))
    }

    /// Units reachable from `unit` (including itself) for this marker.
    pub fn reachable(&self, marker_id: i64, unit: &str) -> Vec<String> {
        let graph = self.edges(marker_id);
        let start = unit_key(unit);
        let mut seen = vec![start.clone()];
        let mut queue = VecDeque::from([start]);
        while let Some(u) = queue.pop_front() {
            for (v, _) in graph.get(&u).into_iter().flatten() {
                if !seen.contains(v) {
                    seen.push(v.clone());
                    queue.push_back(v.clone());
                }
            }
        }
        seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conv(marker_id: i64, from: &str, to: &str, factor: f64, offset: f64) -> Conversion {
        Conversion { id: 0, marker_id, from_unit: from.into(), to_unit: to.into(), factor, offset }
    }

    fn set() -> ConversionSet {
        ConversionSet::new(vec![
            conv(1, "mmol/L", "mg/dL", 18.016, 0.0),
            conv(2, "mmol/L", "mg/dL", 38.67, 0.0),
            conv(3, "mmol/mol", "%", 0.09148, 2.152),
            conv(0, "g/L", "mg/dL", 100.0, 0.0),
            conv(0, "mg/dL", "mg/L", 10.0, 0.0),
        ])
    }

    #[test]
    fn aliases_fold_extra_spellings() {
        assert_ne!(builtin_key("gms/dl"), builtin_key("g/dL"));
        set_aliases([("gms/dl", "g/dL")]);
        assert_eq!(unit_key("GMS/dL"), unit_key("g/dL"));
        set_aliases([]);
        assert_ne!(unit_key("gms/dl"), unit_key("g/dL"));
    }

    #[test]
    fn keys_fold_spellings() {
        assert_eq!(unit_key("mcg/dL"), unit_key("µg/dL"));
        assert_eq!(unit_key("μg/dl"), unit_key("ug/dL"));
        assert_eq!(unit_key("K/uL"), unit_key("10^3/µL"));
        assert_eq!(unit_key("x10E9/L"), unit_key("10^9/L"));
        assert!(same_unit("MG/DL", "mg/dL"));
        assert!(same_unit("lbs", "lb"));
    }

    #[test]
    fn cleans_spreadsheet_units() {
        assert_eq!(unit_key(&clean_unit("μm³ (x10E3/uL)")), unit_key("10^3/µL"));
        assert_eq!(unit_key(&clean_unit("x10E3/uL")), unit_key("10^3/µL"));
        assert_eq!(unit_key(&clean_unit("x10E3/mm3")), unit_key("10^3/µL"));
        assert_eq!(clean_unit("%Hb"), "%");
        assert_eq!(clean_unit("% of total"), "%");
        assert_eq!(clean_unit("μm³"), "fL");
        assert_eq!(clean_unit("mg/dL (fasting)"), "mg/dL");
        assert_eq!(unit_key(&clean_unit("uIU/ml")), unit_key("µIU/mL"));
    }

    #[test]
    fn marker_specific_factors() {
        let s = set();
        let g = s.convert(1, 5.5, "mmol/L", "mg/dL").unwrap();
        assert!((g - 99.088).abs() < 1e-6);
        let c = s.convert(2, 5.0, "mmol/L", "mg/dL").unwrap();
        assert!((c - 193.35).abs() < 1e-6);
        // inverse direction
        let back = s.convert(1, 99.088, "mg/dL", "mmol/L").unwrap();
        assert!((back - 5.5).abs() < 1e-9);
        // marker 3 has no mmol/L edge
        assert!(s.convert(3, 1.0, "mmol/L", "mg/dL").is_err());
    }

    #[test]
    fn affine_and_multi_hop() {
        let s = set();
        let pct = s.convert(3, 48.0, "mmol/mol", "%").unwrap();
        assert!((pct - 6.54304).abs() < 1e-6);
        let ifcc = s.convert(3, pct, "%", "mmol/mol").unwrap();
        assert!((ifcc - 48.0).abs() < 1e-9);
        // g/L -> mg/dL -> mg/L via generic edges
        let v = s.convert(9, 1.2, "g/L", "mg/L").unwrap();
        assert!((v - 1200.0).abs() < 1e-9);
        assert_eq!(s.convert(9, 3.0, "mg/dl", "MG/DL").unwrap(), 3.0);
    }
}
