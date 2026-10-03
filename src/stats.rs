//! Descriptive statistics and trend calculations over a time series.

use serde::Serialize;

/// A single observation: (day number, value).
pub type Point = (f64, f64);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Summary {
    pub n: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub stddev: Option<f64>,
    pub first: f64,
    pub last: f64,
    pub change: f64,
    pub change_pct: Option<f64>,
    /// Least-squares slope in value units per year.
    pub slope_per_year: Option<f64>,
}

pub fn mean(xs: &[f64]) -> Option<f64> {
    (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
}

pub fn median(xs: &[f64]) -> Option<f64> {
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    match v.len() {
        0 => None,
        n if n % 2 == 1 => Some(v[n / 2]),
        n => Some((v[n / 2 - 1] + v[n / 2]) / 2.0),
    }
}

/// Sample standard deviation.
pub fn stddev(xs: &[f64]) -> Option<f64> {
    let m = mean(xs)?;
    (xs.len() > 1).then(|| (xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (xs.len() - 1) as f64).sqrt())
}

/// Least-squares slope of value over day number, scaled to per-year.
pub fn slope_per_year(points: &[Point]) -> Option<f64> {
    let xs: Vec<f64> = points.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = points.iter().map(|p| p.1).collect();
    let (mx, my) = (mean(&xs)?, mean(&ys)?);
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = points.iter().map(|(x, y)| (x - mx) * (y - my)).sum();
    (points.len() > 1 && sxx > 0.0).then(|| sxy / sxx * 365.2425)
}

pub fn pct_change(from: f64, to: f64) -> Option<f64> {
    (from != 0.0).then(|| (to - from) / from.abs() * 100.0)
}

/// Summarise a series sorted by time.
pub fn summarize(points: &[Point]) -> Option<Summary> {
    let ys: Vec<f64> = points.iter().map(|p| p.1).collect();
    let first = *ys.first()?;
    let last = *ys.last()?;
    Some(Summary {
        n: ys.len(),
        min: ys.iter().copied().fold(f64::INFINITY, f64::min),
        max: ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        mean: mean(&ys)?,
        median: median(&ys)?,
        stddev: stddev(&ys),
        first,
        last,
        change: last - first,
        change_pct: pct_change(first, last),
        slope_per_year: slope_per_year(points),
    })
}

/// (change, percent change) from the most recent value at or before day
/// `cutoff` to the last value. `None` when the series does not reach back that far.
pub fn window_change(points: &[Point], cutoff: f64) -> Option<(f64, f64)> {
    let (_, last) = *points.last()?;
    let base = points.iter().rev().find(|p| p.0 <= cutoff + 1e-9)?;
    Some((last - base.1, pct_change(base.1, last)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_stats() {
        let xs = [3.0, 1.0, 4.0, 1.0, 5.0];
        assert_eq!(mean(&xs), Some(2.8));
        assert_eq!(median(&xs), Some(3.0));
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(median(&[]), None);
        assert!((stddev(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).unwrap() - 2.138).abs() < 1e-3);
    }

    #[test]
    fn slope_is_per_year() {
        let pts = [(0.0, 100.0), (365.2425, 110.0), (730.485, 120.0)];
        assert!((slope_per_year(&pts).unwrap() - 10.0).abs() < 1e-9);
        assert_eq!(slope_per_year(&[(0.0, 1.0)]), None);
    }

    #[test]
    fn summary_and_windows() {
        let pts = [(0.0, 200.0), (100.0, 180.0), (200.0, 150.0)];
        let s = summarize(&pts).unwrap();
        assert_eq!(s.n, 3);
        assert_eq!(s.min, 150.0);
        assert_eq!(s.max, 200.0);
        assert_eq!(s.change, -50.0);
        assert_eq!(s.change_pct, Some(-25.0));
        let (d, p) = window_change(&pts, 100.0).unwrap();
        assert_eq!(d, -30.0);
        assert!((p - (-30.0 / 180.0 * 100.0)).abs() < 1e-9);
        assert_eq!(window_change(&pts, -165.0), None);
    }
}
