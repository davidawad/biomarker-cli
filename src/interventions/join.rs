//! Joining regimens onto measurements (`query`/`trend --with-regimens`).

use serde_json::{json, Value};

use super::interaction;
use super::regimen::{self, Regimen};
use crate::db::Db;
use crate::error::Result;
use crate::store::Catalog;

pub struct RegimenIndex {
    regs: Vec<Regimen>,
    /// (intervention slug, marker id) pairs with a lab-interference edge.
    interferes: Vec<(String, i64)>,
}

impl RegimenIndex {
    pub fn load(db: &Db, cat: &Catalog) -> Result<Self> {
        let regs = regimen::list(db, &[], None)?;
        let interferes = interaction::list(db)?
            .into_iter()
            .filter(|e| e.kind == "lab-interference" && e.target_kind == "lab-test")
            .filter_map(|e| cat.find(&e.target).map(|m| (e.a, m.id)))
            .collect();
        Ok(Self { regs, interferes })
    }

    /// Interventions taken on the day of `taken_at`.
    pub fn taking(&self, person_id: i64, taken_at: &str) -> Vec<String> {
        regimen::taking_on(&self.regs, person_id, taken_at)
    }

    /// Of `taking`, those with a lab-interference edge to the marker.
    pub fn interferences(&self, taking: &[String], marker_id: i64) -> Vec<String> {
        taking.iter().filter(|t| self.interferes.iter().any(|(s, m)| s == *t && *m == marker_id)).cloned().collect()
    }

    /// Start/stop events of a person's regimens between two `taken_at`s (inclusive, by day).
    pub fn events(&self, person_id: i64, from: &str, to: &str) -> Vec<Value> {
        let (from, to) = (from.get(..10).unwrap_or(from), to.get(..10).unwrap_or(to));
        let mut ev: Vec<(String, Value)> = self
            .regs
            .iter()
            .filter(|r| r.person_id == person_id)
            .flat_map(|r| {
                std::iter::once(("start", r.started_on.clone()))
                    .chain(r.stopped_on.clone().map(|s| ("stop", s)))
                    .filter(|(_, d)| d.as_str() >= from && d.as_str() <= to)
                    .map(|(event, date)| {
                        let v =
                            json!({"date": date, "event": event, "intervention": r.intervention, "regimen_id": r.id});
                        (date, v)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        ev.sort_by(|a, b| a.0.cmp(&b.0));
        ev.into_iter().map(|e| e.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reg(id: i64, slug: &str, start: &str, stop: Option<&str>) -> Regimen {
        Regimen {
            id,
            person_id: 1,
            person: "alex".into(),
            intervention_id: id,
            intervention: slug.into(),
            intervention_name: slug.into(),
            dose: None,
            unit: None,
            frequency: None,
            time_slot: None,
            started_on: start.into(),
            stopped_on: stop.map(Into::into),
            note: None,
        }
    }

    fn index(regs: Vec<Regimen>) -> RegimenIndex {
        RegimenIndex { regs, interferes: vec![("biotin".into(), 7)] }
    }

    const DRAW: &str = "2025-03-10T08:30:00Z";

    #[test]
    fn regimen_starting_on_the_draw_day_counts() {
        let ix = index(vec![reg(1, "fish-oil", "2025-03-10", None)]);
        assert_eq!(ix.taking(1, DRAW), ["fish-oil"]);
        assert_eq!(ix.taking(1, "2025-03-10"), ["fish-oil"]);
        assert!(ix.taking(1, "2025-03-09T23:59:59Z").is_empty());
        assert!(ix.taking(2, DRAW).is_empty());
    }

    #[test]
    fn regimen_stopped_before_the_draw_day_does_not_count() {
        // `regimen stop --on 2025-03-09`: the 9th is the first day not taken.
        let ix = index(vec![reg(1, "fish-oil", "2025-01-01", Some("2025-03-09"))]);
        assert!(ix.taking(1, DRAW).is_empty());
        assert!(ix.taking(1, "2025-03-09").is_empty());
        assert_eq!(ix.taking(1, "2025-03-08T23:00:00Z"), ["fish-oil"]);
        // Stopped on the draw day itself: not taken that day either.
        let ix = index(vec![reg(1, "fish-oil", "2025-01-01", Some("2025-03-10"))]);
        assert!(ix.taking(1, DRAW).is_empty());
        // Stopped the day after: the draw day was the last day taken.
        let ix = index(vec![reg(1, "fish-oil", "2025-01-01", Some("2025-03-11"))]);
        assert_eq!(ix.taking(1, DRAW), ["fish-oil"]);
    }

    #[test]
    fn adjacent_regimens_hand_over_on_the_boundary_day() {
        // A dose change: the old regimen stops on the day the new one starts.
        let ix = index(vec![reg(1, "biotin", "2025-01-01", Some("2025-03-10")), reg(2, "biotin", "2025-03-10", None)]);
        assert_eq!(ix.taking(1, "2025-03-09"), ["biotin"]);
        assert_eq!(ix.taking(1, DRAW), ["biotin"]);
        assert_eq!(ix.interferences(&ix.taking(1, DRAW), 7), ["biotin"]);
        assert!(ix.interferences(&ix.taking(1, DRAW), 8).is_empty());
    }

    #[test]
    fn events_include_both_boundary_days() {
        let ix =
            index(vec![reg(1, "fish-oil", "2025-03-10", Some("2025-04-10")), reg(2, "biotin", "2025-03-09", None)]);
        let dates = |from, to| -> Vec<(String, String)> {
            ix.events(1, from, to)
                .iter()
                .map(|e| (e["date"].as_str().unwrap().into(), e["event"].as_str().unwrap().into()))
                .collect()
        };
        assert_eq!(
            dates(DRAW, "2025-04-10T23:59:59Z"),
            [("2025-03-10".into(), "start".into()), ("2025-04-10".into(), "stop".into())]
        );
        assert_eq!(dates("2025-03-09", "2025-03-09"), [("2025-03-09".to_string(), "start".to_string())]);
        assert!(dates("2025-03-11", "2025-04-09").is_empty());
        assert!(ix.events(2, "2025-01-01", "2026-01-01").is_empty());
    }
}
