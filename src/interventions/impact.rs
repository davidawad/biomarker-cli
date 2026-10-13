//! Marker values around a regimen start or stop (pure).

use chrono::{Duration, NaiveDate};

/// Values on each side of an event. The event day counts as "before".
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Sides {
    pub n_before: usize,
    /// Value nearest the event in `(event - window, event]`, with its `taken_at`.
    pub before: Option<(String, f64)>,
    pub before_mean: Option<f64>,
    pub n_after: usize,
    /// Last value in `(event, event + window]`, with its `taken_at`.
    pub after: Option<(String, f64)>,
    pub after_mean: Option<f64>,
}

impl Sides {
    pub fn change(&self) -> Option<f64> {
        self.before.as_ref().zip(self.after.as_ref()).map(|(b, a)| a.1 - b.1)
    }

    pub fn is_empty(&self) -> bool {
        self.n_before == 0 && self.n_after == 0
    }
}

fn mean(v: &[f64]) -> Option<f64> {
    (!v.is_empty()).then(|| v.iter().sum::<f64>() / v.len() as f64)
}

/// `points` are `(taken_at, value)` sorted by `taken_at`.
pub fn sides(points: &[(String, f64)], event: NaiveDate, window_days: i64) -> Sides {
    let day = |t: &str| crate::util::date_of(t).ok();
    let lo = event - Duration::days(window_days);
    let hi = event + Duration::days(window_days);
    let before: Vec<&(String, f64)> =
        points.iter().filter(|p| day(&p.0).is_some_and(|d| d > lo && d <= event)).collect();
    let after: Vec<&(String, f64)> =
        points.iter().filter(|p| day(&p.0).is_some_and(|d| d > event && d <= hi)).collect();
    let vals = |v: &[&(String, f64)]| v.iter().map(|p| p.1).collect::<Vec<_>>();
    Sides {
        n_before: before.len(),
        before: before.last().map(|p| (*p).clone()),
        before_mean: mean(&vals(&before)),
        n_after: after.len(),
        after: after.last().map(|p| (*p).clone()),
        after_mean: mean(&vals(&after)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_split_on_event_day() {
        let pts: Vec<(String, f64)> = [
            ("2024-10-01", 1.0),
            ("2025-01-01", 150.0),
            ("2025-01-10", 140.0),
            ("2025-02-15", 110.0),
            ("2025-03-20", 100.0),
            ("2025-09-01", 5.0),
        ]
        .iter()
        .map(|(d, v)| (d.to_string(), *v))
        .collect();
        let event = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();
        let s = sides(&pts, event, 90);
        assert_eq!((s.n_before, s.n_after), (2, 2));
        assert_eq!(s.before, Some(("2025-01-10".into(), 140.0)));
        assert_eq!(s.before_mean, Some(145.0));
        assert_eq!(s.after, Some(("2025-03-20".into(), 100.0)));
        assert_eq!(s.change(), Some(-40.0));
        let none = sides(&pts, NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(), 30);
        assert!(none.is_empty() && none.change().is_none());
    }
}
