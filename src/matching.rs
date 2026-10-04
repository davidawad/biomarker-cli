//! Resolving free-form source names (spreadsheet rows, column headers) to
//! catalog markers and categories.

use crate::store::{Catalog, Marker};
use crate::util::slugify;

/// Comparison key: lowercase letters and digits only (`ALT (SGPT)` -> `altsgpt`).
pub fn name_key(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

/// Split `Low-Density Lipoprotein (LDL-C)` into (`Low-Density Lipoprotein`, [`LDL-C`]).
pub fn split_parens(s: &str) -> (String, Vec<String>) {
    let mut outer = String::new();
    let mut inner = Vec::new();
    let mut depth = 0usize;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' | '[' => {
                if depth > 0 {
                    cur.push(c);
                }
                depth += 1;
            }
            ')' | ']' if depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    inner.push(std::mem::take(&mut cur).trim().to_string());
                } else {
                    cur.push(c);
                }
            }
            _ if depth > 0 => cur.push(c),
            _ => outer.push(c),
        }
    }
    let outer = outer.split_whitespace().collect::<Vec<_>>().join(" ");
    (outer, inner.into_iter().filter(|i| !i.is_empty()).collect())
}

fn by_key<'a>(cat: &'a Catalog, key: &str) -> Option<&'a Marker> {
    (!key.is_empty())
        .then(|| {
            cat.markers.iter().find(|m| {
                name_key(&m.slug) == key || name_key(&m.name) == key || m.aliases.iter().any(|a| name_key(a) == key)
            })
        })
        .flatten()
}

/// Resolve a source name through slugs, aliases and names: first as written,
/// then ignoring case and punctuation, then without its parenthesised part,
/// then by the parenthesised abbreviation (`ALT (SGPT)` -> alt,
/// `Low-Density Lipoprotein (LDL-C)` -> ldl-c).
pub fn resolve<'a>(cat: &'a Catalog, name: &str) -> Option<&'a Marker> {
    let (outer, inner) = split_parens(name);
    std::iter::once(name.to_string())
        .chain(std::iter::once(outer))
        .chain(inner)
        .filter(|c| !c.trim().is_empty())
        .find_map(|c| cat.find(&c).or_else(|| by_key(cat, &name_key(&c))))
}

/// Category for a section title such as `Complete Blood Count (CBC)` (-> `cbc`)
/// or `Lipid Panel + ApoB` (-> `lipid`): an existing category named by the
/// abbreviation or by one of the words, else the slugified title.
pub fn section_category(section: &str, known: &[String]) -> String {
    let (outer, inner) = split_parens(section);
    let is_known = |s: &str| known.iter().any(|k| k == s);
    inner
        .iter()
        .map(|i| slugify(i))
        .chain(slugify(&outer).split('-').map(str::to_string))
        .find(|s| is_known(s))
        .unwrap_or_else(|| slugify(section))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_and_keys() {
        assert_eq!(
            split_parens("Low-Density Lipoprotein (LDL-C)"),
            ("Low-Density Lipoprotein".into(), vec!["LDL-C".into()])
        );
        assert_eq!(split_parens("Lp(a)"), ("Lp".into(), vec!["a".into()]));
        assert_eq!(name_key("ALT (SGPT)"), "altsgpt");
        let known = vec!["cbc".to_string(), "lipid".to_string()];
        assert_eq!(section_category("Complete Blood Count (CBC)", &known), "cbc");
        assert_eq!(section_category("Lipid Panel + ApoB", &known), "lipid");
        assert_eq!(section_category("Urinalysis", &known), "urinalysis");
    }
}
