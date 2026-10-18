//! UCUM <-> the repo's unit spellings.
//!
//! FHIR (US Core) and HL7 v2 carry machine units as UCUM codes. UCUM facts the
//! normaliser relies on: symbols are case-sensitive, micro is `u`, powers of
//! ten are `10*n`, and curly-brace annotations (`{1.73_m2}`, `{beats}`) carry
//! no meaning. The table below is data: one row per unit, UCUM code on the
//! left, the repo's spelling on the right.

use crate::units::unit_key;

/// (UCUM code, repo spelling). The first row for a spelling is the code written on export.
pub const UCUM_UNITS: &[(&str, &str)] = &[
    ("mg/dL", "mg/dL"),
    ("g/dL", "g/dL"),
    ("ug/dL", "µg/dL"),
    ("ng/dL", "ng/dL"),
    ("ng/mL", "ng/mL"),
    ("pg/mL", "pg/mL"),
    ("10*3/uL", "10^3/µL"),
    ("10*6/uL", "10^6/µL"),
    ("10*9/L", "10^9/L"),
    ("10*12/L", "10^12/L"),
    ("mmol/L", "mmol/L"),
    ("umol/L", "µmol/L"),
    ("nmol/L", "nmol/L"),
    ("pmol/L", "pmol/L"),
    ("g/L", "g/L"),
    ("mg/L", "mg/L"),
    ("ug/L", "µg/L"),
    ("meq/L", "mEq/L"),
    ("%", "%"),
    ("fL", "fL"),
    ("pg", "pg"),
    ("U/L", "U/L"),
    ("[IU]/L", "IU/L"),
    ("m[IU]/L", "mIU/L"),
    ("m[IU]/mL", "mIU/mL"),
    ("[IU]/mL", "IU/mL"),
    ("u[IU]/mL", "µIU/mL"),
    ("mL/min/{1.73_m2}", "mL/min/1.73m²"),
    ("mmol/mol", "mmol/mol"),
    ("L/L", "L/L"),
    ("ukat/L", "µkat/L"),
    ("1", "ratio"),
    ("kg", "kg"),
    ("lb_av", "lb"),
    ("kg/m2", "kg/m²"),
    ("a", "years"),
    ("/min", "bpm"),
    ("ms", "ms"),
    ("mL/kg/min", "mL/kg/min"),
    ("mm[Hg]", "mmHg"),
];

/// Remove curly-brace annotations: `10*3/uL{cells}` -> `10*3/uL`.
/// A code that is only an annotation (`{ratio}`, `{beats}`) becomes `1` for
/// `{ratio}` and is otherwise kept as written between braces stripped.
pub fn strip_annotations(code: &str) -> String {
    let mut out = String::new();
    let mut depth = 0usize;
    for c in code.chars() {
        match c {
            '{' => depth += 1,
            '}' if depth > 0 => depth -= 1,
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out.trim().to_string()
}

/// The repo spelling of a UCUM code, if the table knows it.
///
/// `mL/min/{1.73_m2}` is matched before annotations are stripped, because its
/// annotation is the only thing telling the GFR unit apart from `mL/min`.
pub fn from_ucum(code: &str) -> Option<String> {
    let c = code.trim();
    if c.is_empty() {
        return None;
    }
    let lookup = |k: &str| UCUM_UNITS.iter().find(|(u, _)| *u == k).map(|(_, s)| (*s).to_string());
    if let Some(s) = lookup(c) {
        return Some(s);
    }
    if matches!(c, "{ratio}" | "{index}" | "{score}") {
        return Some("ratio".into());
    }
    if matches!(c, "{beats}/min" | "{Beats}/min" | "{beat}/min") {
        return Some("bpm".into());
    }
    let stripped = strip_annotations(c);
    if stripped.is_empty() {
        return None;
    }
    lookup(&stripped)
}

/// The UCUM code for a repo unit spelling, if there is one.
pub fn to_ucum(unit: &str) -> Option<&'static str> {
    let key = unit_key(unit);
    UCUM_UNITS.iter().find(|(_, s)| unit_key(s) == key).map(|(u, _)| *u)
}

/// A usable unit spelling from a FHIR Quantity / HL7 unit: the UCUM code when
/// the system says UCUM (or is absent and the code looks like UCUM), else the
/// human `unit` text through the spreadsheet clean-up. `None` when neither
/// yields anything.
pub fn resolve_unit(system: Option<&str>, code: Option<&str>, text: Option<&str>) -> Option<String> {
    let is_ucum = system.is_none_or(|s| {
        s.trim_end_matches('/').eq_ignore_ascii_case("http://unitsofmeasure.org") || s.eq_ignore_ascii_case("UCUM")
    });
    let from_code = code.filter(|_| is_ucum).and_then(from_ucum);
    from_code
        .or_else(|| {
            // an unrecognised UCUM code still reads fine as a spelling when it is plain (`mg/dL`)
            code.filter(|_| is_ucum).map(strip_annotations).filter(|c| !c.is_empty() && !c.contains(['[', '*']))
        })
        .or_else(|| text.map(crate::units::clean_unit).filter(|t| !t.trim().is_empty()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_ucum_to_repo_spellings() {
        assert_eq!(from_ucum("mg/dL").as_deref(), Some("mg/dL"));
        assert_eq!(from_ucum("ug/dL").as_deref(), Some("µg/dL"));
        assert_eq!(from_ucum("10*3/uL").as_deref(), Some("10^3/µL"));
        assert_eq!(from_ucum("10*3/uL{cells}").as_deref(), Some("10^3/µL"));
        assert_eq!(from_ucum("[IU]/L").as_deref(), Some("IU/L"));
        assert_eq!(from_ucum("m[IU]/L").as_deref(), Some("mIU/L"));
        assert_eq!(from_ucum("u[IU]/mL").as_deref(), Some("µIU/mL"));
        assert_eq!(from_ucum("mL/min/{1.73_m2}").as_deref(), Some("mL/min/1.73m²"));
        assert_eq!(from_ucum("1").as_deref(), Some("ratio"));
        assert_eq!(from_ucum("{ratio}").as_deref(), Some("ratio"));
        assert_eq!(from_ucum("mm[Hg]").as_deref(), Some("mmHg"));
        assert_eq!(from_ucum("{beats}/min").as_deref(), Some("bpm"));
        assert_eq!(from_ucum("parsec"), None);
        assert_eq!(from_ucum(""), None);
    }

    #[test]
    fn reverse_table_round_trips() {
        for (code, spelling) in UCUM_UNITS {
            let back = to_ucum(spelling).unwrap_or_else(|| panic!("{spelling} has no UCUM code"));
            assert_eq!(from_ucum(back).as_deref(), Some(*spelling), "{code}");
        }
        assert_eq!(to_ucum("mcg/dL"), Some("ug/dL"));
        assert_eq!(to_ucum("furlongs"), None);
    }

    #[test]
    fn resolves_quantity_units() {
        let u = "http://unitsofmeasure.org";
        assert_eq!(resolve_unit(Some(u), Some("mg/dL"), Some("mg/dL")).as_deref(), Some("mg/dL"));
        assert_eq!(resolve_unit(Some(u), Some("ug/dL"), None).as_deref(), Some("µg/dL"));
        assert_eq!(resolve_unit(None, None, Some("x10E3/uL")).as_deref(), Some("x10E3/uL"));
        assert_eq!(resolve_unit(Some("http://example.org/units"), Some("zz"), Some("mg/dL")).as_deref(), Some("mg/dL"));
        assert_eq!(resolve_unit(None, None, None), None);
    }
}
