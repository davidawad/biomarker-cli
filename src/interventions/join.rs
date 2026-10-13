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
