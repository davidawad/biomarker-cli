//! Regimens: one person taking one intervention from a start day until an
//! optional stop day (the first day not taken).

use chrono::NaiveDate;
use serde::Serialize;

use crate::db::{int, opt_real, opt_text, text, Db, Row, RowExt};
use crate::error::{AppError, Result};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Regimen {
    pub id: i64,
    #[serde(skip)]
    pub person_id: i64,
    pub person: String,
    #[serde(skip)]
    pub intervention_id: i64,
    pub intervention: String,
    pub intervention_name: String,
    pub dose: Option<f64>,
    pub unit: Option<String>,
    pub frequency: Option<String>,
    pub time_slot: Option<String>,
    pub started_on: String,
    pub stopped_on: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Planned,
    Taking,
    Stopped,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Taking => "taking",
            Self::Stopped => "stopped",
        }
    }
}

impl Regimen {
    /// Status on `day` (`YYYY-MM-DD`; a longer `taken_at` is cut to its date).
    pub fn status_on(&self, day: &str) -> Status {
        let d = day.get(..10).unwrap_or(day);
        if d < self.started_on.as_str() {
            Status::Planned
        } else if self.stopped_on.as_deref().is_some_and(|s| d >= s) {
            Status::Stopped
        } else {
            Status::Taking
        }
    }

    pub fn active_on(&self, day: &str) -> bool {
        self.status_on(day) == Status::Taking
    }

    /// Days taken up to and including `as_of`.
    pub fn days_taken(&self, as_of: NaiveDate) -> i64 {
        let parse = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok();
        let Some(start) = parse(&self.started_on) else { return 0 };
        let end = as_of.succ_opt().unwrap_or(as_of);
        let end = self.stopped_on.as_deref().and_then(parse).map_or(end, |s| s.min(end));
        (end - start).num_days().max(0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct NewRegimen {
    pub person_id: i64,
    pub intervention_id: i64,
    pub dose: Option<f64>,
    pub unit: Option<String>,
    pub frequency: Option<String>,
    pub time_slot: Option<String>,
    pub started_on: String,
    pub stopped_on: Option<String>,
    pub note: Option<String>,
}

pub fn insert(db: &Db, r: &NewRegimen) -> Result<i64> {
    if r.stopped_on.as_deref().is_some_and(|s| s <= r.started_on.as_str()) {
        return Err(AppError::invalid("the stop date must be after the start date"));
    }
    db.execute(
        "INSERT INTO regimens (person_id, intervention_id, dose, unit, frequency, time_slot, started_on, stopped_on, note, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        &[
            int(r.person_id),
            int(r.intervention_id),
            opt_real(r.dose),
            opt_text(r.unit.as_ref()),
            opt_text(r.frequency.as_ref()),
            opt_text(r.time_slot.as_ref()),
            text(&r.started_on),
            opt_text(r.stopped_on.as_ref()),
            opt_text(r.note.as_ref()),
            text(crate::util::now_iso()),
        ],
    )?;
    Ok(db.last_insert_rowid())
}

const SELECT: &str = "SELECT r.id, r.person_id, p.slug, r.intervention_id, i.slug, i.name, r.dose, r.unit, r.frequency,
        r.time_slot, r.started_on, r.stopped_on, r.note
     FROM regimens r JOIN people p ON p.id = r.person_id JOIN interventions i ON i.id = r.intervention_id";

fn from_row(r: &Row) -> Regimen {
    Regimen {
        id: r.i(0).unwrap_or_default(),
        person_id: r.i(1).unwrap_or_default(),
        person: r.s(2).unwrap_or_default(),
        intervention_id: r.i(3).unwrap_or_default(),
        intervention: r.s(4).unwrap_or_default(),
        intervention_name: r.s(5).unwrap_or_default(),
        dose: r.f(6),
        unit: r.s(7),
        frequency: r.s(8),
        time_slot: r.s(9),
        started_on: r.s(10).unwrap_or_default(),
        stopped_on: r.s(11),
        note: r.s(12),
    }
}

/// Regimens of the given people (everyone when empty), optionally of one intervention.
pub fn list(db: &Db, person_ids: &[i64], intervention_id: Option<i64>) -> Result<Vec<Regimen>> {
    let rows = db.query(&format!("{SELECT} ORDER BY p.slug, r.started_on, i.slug, r.id"), &[])?;
    Ok(rows
        .iter()
        .map(from_row)
        .filter(|r| person_ids.is_empty() || person_ids.contains(&r.person_id))
        .filter(|r| intervention_id.is_none_or(|i| r.intervention_id == i))
        .collect())
}

pub fn get(db: &Db, id: i64) -> Result<Regimen> {
    db.query_opt(&format!("{SELECT} WHERE r.id = ?1"), &[int(id)])?
        .map(|r| from_row(&r))
        .ok_or_else(|| AppError::not_found(format!("no regimen with id {id}")))
}

pub fn stop(db: &Db, r: &Regimen, on: &str) -> Result<()> {
    if on <= r.started_on.as_str() {
        return Err(AppError::invalid(format!(
            "regimen {} started on {}; the stop date must be after it",
            r.id, r.started_on
        )));
    }
    db.execute("UPDATE regimens SET stopped_on = ?1 WHERE id = ?2", &[text(on), int(r.id)]).map(|_| ())
}

pub fn delete(db: &Db, id: i64) -> Result<usize> {
    db.execute("DELETE FROM regimens WHERE id = ?1", &[int(id)])
}

/// Slugs of the interventions `person_id` is taking on `day` (sorted, unique).
pub fn taking_on(regs: &[Regimen], person_id: i64, day: &str) -> Vec<String> {
    let mut v: Vec<String> =
        regs.iter().filter(|r| r.person_id == person_id && r.active_on(day)).map(|r| r.intervention.clone()).collect();
    v.sort();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reg(start: &str, stop: Option<&str>) -> Regimen {
        Regimen {
            id: 1,
            person_id: 1,
            person: "alex".into(),
            intervention_id: 1,
            intervention: "fish-oil".into(),
            intervention_name: "Fish oil".into(),
            dose: None,
            unit: None,
            frequency: None,
            time_slot: None,
            started_on: start.into(),
            stopped_on: stop.map(Into::into),
            note: None,
        }
    }

    #[test]
    fn status_boundaries() {
        let r = reg("2025-01-10", Some("2025-02-01"));
        assert_eq!(r.status_on("2025-01-09"), Status::Planned);
        assert_eq!(r.status_on("2025-01-10"), Status::Taking);
        assert_eq!(r.status_on("2025-01-31T23:00:00"), Status::Taking);
        assert_eq!(r.status_on("2025-02-01"), Status::Stopped);
        assert_eq!(reg("2025-01-10", None).status_on("2030-01-01"), Status::Taking);
    }

    #[test]
    fn days_taken() {
        let d = |s| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        let r = reg("2025-01-10", Some("2025-02-01"));
        assert_eq!(r.days_taken(d("2025-01-09")), 0);
        assert_eq!(r.days_taken(d("2025-01-10")), 1);
        assert_eq!(r.days_taken(d("2025-06-01")), 22);
    }

    #[test]
    fn taking_on_dedupes() {
        let regs = vec![reg("2025-01-01", None), reg("2025-01-05", None), reg("2024-01-01", Some("2024-06-01"))];
        assert_eq!(taking_on(&regs, 1, "2025-01-06"), ["fish-oil"]);
        assert!(taking_on(&regs, 1, "2024-12-01").is_empty());
        assert!(taking_on(&regs, 2, "2025-01-06").is_empty());
    }
}
