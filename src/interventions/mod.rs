//! Interventions (supplements, drugs, ...), per-person regimens and
//! interaction edges. See docs/interventions.md.

pub mod doc;
pub mod impact;
pub mod interaction;
pub mod join;
pub mod regimen;

use serde::{Deserialize, Serialize};

use crate::db::{int, opt_text, text, Db, Row, RowExt};
use crate::error::{AppError, Result};
use crate::util::slugify;

/// Prescription status of an intervention.
pub const RX_STATUSES: &[&str] = &["otc", "rx", "controlled", "unknown"];

/// Normalise a prescription status (accepting common spellings).
pub fn parse_rx_status(s: &str) -> Result<String> {
    let t = slugify(s);
    let v = match t.as_str() {
        "" | "unknown" => "unknown",
        "otc" | "over-the-counter" | "supplement" | "nonprescription" | "non-prescription" => "otc",
        "rx" | "prescription" | "rx-only" => "rx",
        "controlled" | "controlled-substance" | "scheduled" => "controlled",
        _ => return Err(AppError::invalid(format!("bad rx status '{s}' ({})", RX_STATUSES.join("|")))),
    };
    Ok(v.to_string())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: Option<f64>,
    pub unit: Option<String>,
}

impl Ingredient {
    /// Parse `NAME[=AMOUNT[UNIT]]`, e.g. `magnesium=120mg` or `epa=500 mg`.
    pub fn parse(s: &str) -> Result<Self> {
        let (name, rest) = s.split_once('=').map_or((s, None), |(n, r)| (n, Some(r.trim())));
        let name = name.trim().to_lowercase();
        if name.is_empty() {
            return Err(AppError::invalid(format!("bad ingredient '{s}' (expected NAME[=AMOUNT[UNIT]])")));
        }
        let Some(rest) = rest.filter(|r| !r.is_empty()) else { return Ok(Self { name, amount: None, unit: None }) };
        let split = rest.find(|c: char| !(c.is_ascii_digit() || c == '.')).unwrap_or(rest.len());
        let amount = rest[..split]
            .parse::<f64>()
            .map_err(|_| AppError::invalid(format!("bad ingredient amount in '{s}' (e.g. magnesium=120mg)")))?;
        let unit = Some(rest[split..].trim().to_string()).filter(|u| !u.is_empty());
        Ok(Self { name, amount: Some(amount), unit })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Intervention {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub category: String,
    pub rx_status: String,
    pub ingredients: Vec<Ingredient>,
    pub classes: Vec<String>,
    pub aliases: Vec<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub created_at: String,
}

impl Intervention {
    pub fn new(slug: &str) -> Self {
        Self {
            id: 0,
            slug: slug.to_string(),
            name: slug.to_string(),
            category: "supplement".into(),
            rx_status: "unknown".into(),
            ingredients: Vec::new(),
            classes: Vec::new(),
            aliases: Vec::new(),
            description: None,
            source: None,
            created_at: crate::util::now_iso(),
        }
    }
}

const COLS: &str = "id, slug, name, category, rx_status, ingredients, classes, description, source, created_at";

fn from_row(r: &Row, aliases: &[Row]) -> Intervention {
    let id = r.i(0).unwrap_or_default();
    Intervention {
        id,
        slug: r.s(1).unwrap_or_default(),
        name: r.s(2).unwrap_or_default(),
        category: r.s(3).unwrap_or_default(),
        rx_status: r.s(4).unwrap_or_default(),
        ingredients: r.s(5).and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        classes: r.s(6).and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        description: r.s(7),
        source: r.s(8),
        created_at: r.s(9).unwrap_or_default(),
        aliases: aliases.iter().filter(|a| a.i(1) == Some(id)).filter_map(|a| a.s(0)).collect(),
    }
}

/// The intervention catalog, loaded once per command.
#[derive(Debug, Clone, Default)]
pub struct IvCatalog {
    pub items: Vec<Intervention>,
}

impl IvCatalog {
    pub fn load(db: &Db) -> Result<Self> {
        let aliases = db.query("SELECT alias, intervention_id FROM intervention_aliases ORDER BY alias", &[])?;
        let items = db
            .query(&format!("SELECT {COLS} FROM interventions ORDER BY category, slug"), &[])?
            .iter()
            .map(|r| from_row(r, &aliases))
            .collect();
        Ok(Self { items })
    }

    /// Resolve by slug, alias, display name, or slugified name.
    pub fn find(&self, name: &str) -> Option<&Intervention> {
        let n = name.trim().to_lowercase();
        let slug = slugify(&n);
        self.items
            .iter()
            .find(|i| i.slug == n)
            .or_else(|| self.items.iter().find(|i| i.aliases.contains(&n)))
            .or_else(|| self.items.iter().find(|i| i.name.to_lowercase() == n))
            .or_else(|| self.items.iter().find(|i| i.slug == slug || i.aliases.iter().any(|a| slugify(a) == slug)))
    }

    pub fn get(&self, name: &str) -> Result<&Intervention> {
        self.find(name).ok_or_else(|| {
            AppError::not_found(format!(
                "unknown intervention '{name}' (see: biomarker intervention ls; add with: biomarker intervention add)"
            ))
        })
    }

    pub fn by_id(&self, id: i64) -> Option<&Intervention> {
        self.items.iter().find(|i| i.id == id)
    }
}

fn json_list<T: Serialize>(v: &[T]) -> String {
    serde_json::to_string(v).unwrap_or_else(|_| "[]".into())
}

pub fn insert(db: &Db, iv: &Intervention) -> Result<i64> {
    db.transaction(|db| {
        db.execute(
            "INSERT INTO interventions (slug, name, category, rx_status, ingredients, classes, description, source, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[
                text(&iv.slug),
                text(&iv.name),
                text(&iv.category),
                text(&iv.rx_status),
                text(json_list(&iv.ingredients)),
                text(json_list(&iv.classes)),
                opt_text(iv.description.as_ref()),
                opt_text(iv.source.as_ref()),
                text(&iv.created_at),
            ],
        )
        .map_err(|e| {
            if e.message.contains("UNIQUE") {
                AppError::invalid(format!("intervention '{}' already exists", iv.slug))
            } else {
                e
            }
        })?;
        let id = db.query_scalar_i64("SELECT id FROM interventions WHERE slug = ?1", &[text(&iv.slug)])?;
        iv.aliases.iter().try_for_each(|a| add_alias(db, id, a))?;
        Ok(id)
    })
}

/// Update the stored fields (not aliases) of an existing intervention.
pub fn update(db: &Db, iv: &Intervention) -> Result<()> {
    db.execute(
        "UPDATE interventions SET name = ?1, category = ?2, rx_status = ?3, ingredients = ?4, classes = ?5,
         description = ?6, source = ?7 WHERE id = ?8",
        &[
            text(&iv.name),
            text(&iv.category),
            text(&iv.rx_status),
            text(json_list(&iv.ingredients)),
            text(json_list(&iv.classes)),
            opt_text(iv.description.as_ref()),
            opt_text(iv.source.as_ref()),
            int(iv.id),
        ],
    )
    .map(|_| ())
}

/// Add an alias; an alias owned by another intervention is an error.
pub fn add_alias(db: &Db, id: i64, alias: &str) -> Result<()> {
    let a = alias.trim().to_lowercase();
    if a.is_empty() {
        return Ok(());
    }
    if let Some(r) = db.query_opt("SELECT intervention_id FROM intervention_aliases WHERE alias = ?1", &[text(&a)])? {
        return if r.i(0) == Some(id) {
            Ok(())
        } else {
            Err(AppError::invalid(format!("alias '{a}' already belongs to another intervention")))
        };
    }
    db.execute("INSERT INTO intervention_aliases (alias, intervention_id) VALUES (?1, ?2)", &[text(a), int(id)])
        .map(|_| ())
}

pub fn count_regimens(db: &Db, id: i64) -> Result<i64> {
    db.query_scalar_i64("SELECT count(*) FROM regimens WHERE intervention_id = ?1", &[int(id)])
}

/// Delete an intervention with its aliases, regimens and every edge from or to it.
/// Returns the number of regimens deleted.
pub fn delete(db: &Db, iv: &Intervention) -> Result<usize> {
    db.transaction(|db| {
        let n = db.execute("DELETE FROM regimens WHERE intervention_id = ?1", &[int(iv.id)])?;
        db.execute("DELETE FROM interactions WHERE a_id = ?1", &[int(iv.id)])?;
        let refs: Vec<String> = std::iter::once(iv.slug.clone()).chain(iv.aliases.iter().map(|a| slugify(a))).collect();
        refs.iter().try_for_each(|r| {
            db.execute("DELETE FROM interactions WHERE b_kind = 'intervention' AND b_ref = ?1", &[text(r)]).map(|_| ())
        })?;
        db.execute("DELETE FROM intervention_aliases WHERE intervention_id = ?1", &[int(iv.id)])?;
        db.execute("DELETE FROM interventions WHERE id = ?1", &[int(iv.id)])?;
        Ok(n)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingredient_parsing() {
        assert_eq!(
            Ingredient::parse("Magnesium=120mg").unwrap(),
            Ingredient { name: "magnesium".into(), amount: Some(120.0), unit: Some("mg".into()) }
        );
        assert_eq!(Ingredient::parse("epa = 500 mg").unwrap().unit.as_deref(), Some("mg"));
        assert_eq!(Ingredient::parse("zinc").unwrap(), Ingredient { name: "zinc".into(), amount: None, unit: None });
        assert!(Ingredient::parse("=5mg").is_err());
        assert!(Ingredient::parse("zinc=lots").is_err());
    }

    #[test]
    fn rx_status_synonyms() {
        assert_eq!(parse_rx_status("Prescription").unwrap(), "rx");
        assert_eq!(parse_rx_status("over the counter").unwrap(), "otc");
        assert_eq!(parse_rx_status("").unwrap(), "unknown");
        assert!(parse_rx_status("maybe").is_err());
    }
}
