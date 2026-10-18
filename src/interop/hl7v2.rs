//! HL7 v2 ORU^R01 reader.
//!
//! Follows the field mapping of the HL7 v2-to-FHIR implementation guide, so a
//! result read from a v2 message and the same result read from FHIR produce
//! the same [`ResultCandidate`]. Segment roles: MSH header, PID patient
//! (identity check only), OBR order/report, SPM specimen, OBX observation,
//! NTE note on the preceding OBX.

use super::candidate::{
    sha256_hex, Coding, LabRange, Parsed, PatientHint, ResultCandidate, ResultValue, SourceKind, Status, TimeBasis,
};
use super::ucum::resolve_unit;
use crate::error::{AppError, Result};

struct Delims {
    field: char,
    comp: char,
    rep: char,
    esc: char,
    sub: char,
}

impl Delims {
    /// From MSH-1 (the character after `MSH`) and MSH-2 (`^~\&`).
    fn from_msh(line: &str) -> Option<Self> {
        let rest = line.strip_prefix("MSH")?;
        let mut cs = rest.chars();
        let field = cs.next()?;
        let enc: Vec<char> = cs.take_while(|c| *c != field).collect();
        Some(Self {
            field,
            comp: *enc.first()?,
            rep: enc.get(1).copied().unwrap_or('~'),
            esc: enc.get(2).copied().unwrap_or('\\'),
            sub: enc.get(3).copied().unwrap_or('&'),
        })
    }
}

/// Segments of the input: `\r`, `\n` and `\r\n` all terminate one; MLLP framing bytes are dropped.
fn segments(text: &str) -> Vec<String> {
    text.trim_start_matches('\u{feff}')
        .replace(['\u{b}', '\u{1c}'], "")
        .split(['\r', '\n'])
        .map(str::trim_end)
        .filter(|l| !l.trim().is_empty())
        .map(str::to_string)
        .collect()
}

/// Resolve the escape sequences of one value: `\F\ \S\ \T\ \R\ \E\` give the
/// delimiter characters, `\.br\` a line break; formatting, character-set and
/// hex escapes are dropped. A dangling escape character is kept as text.
fn unescape(s: &str, d: &Delims) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(start) = rest.find(d.esc) {
        out.push_str(&rest[..start]);
        let after = &rest[start + d.esc.len_utf8()..];
        let Some(end) = after.find(d.esc) else {
            out.push(d.esc);
            out.push_str(after);
            return out;
        };
        match &after[..end] {
            "F" => out.push(d.field),
            "S" => out.push(d.comp),
            "T" => out.push(d.sub),
            "R" => out.push(d.rep),
            "E" => out.push(d.esc),
            ".br" => out.push('\n'),
            _ => {}
        }
        rest = &after[end + d.esc.len_utf8()..];
    }
    out.push_str(rest);
    out
}

struct Seg<'a> {
    d: &'a Delims,
    fields: Vec<&'a str>,
}

impl<'a> Seg<'a> {
    fn new(line: &'a str, d: &'a Delims) -> Self {
        Self { d, fields: line.split(d.field).collect() }
    }

    fn name(&self) -> &str {
        self.fields.first().copied().unwrap_or("")
    }

    /// Field `n` (1-based, as the standard numbers it). MSH-1 is the separator itself.
    fn raw(&self, n: usize) -> &'a str {
        let idx = if self.name() == "MSH" { n.saturating_sub(1) } else { n };
        self.fields.get(idx).copied().unwrap_or("")
    }

    /// Component `c` (1-based) of the first repetition of field `n`, unescaped.
    fn comp(&self, n: usize, c: usize) -> String {
        let first = self.raw(n).split(self.d.rep).next().unwrap_or("");
        first.split(self.d.comp).nth(c - 1).map(|s| unescape(s, self.d)).unwrap_or_default().trim().to_string()
    }

    fn opt(&self, n: usize, c: usize) -> Option<String> {
        Some(self.comp(n, c)).filter(|s| !s.is_empty())
    }

    fn repetitions(&self, n: usize) -> Vec<String> {
        self.raw(n)
            .split(self.d.rep)
            .map(|r| unescape(r, self.d).trim().to_string())
            .filter(|r| !r.is_empty())
            .collect()
    }
}

/// `YYYY[MM[DD[HH[MM[SS[.S+]]]]]][+/-ZZZZ]` -> ISO 8601 text the date parser reads.
pub fn timestamp_to_iso(ts: &str) -> Option<String> {
    let t = ts.trim();
    let (body, zone) = match t.find(['+', '-']) {
        Some(i) if i >= 4 => (&t[..i], Some(&t[i..])),
        _ => (t, None),
    };
    let (digits, frac) = body.split_once('.').map_or((body, None), |(a, b)| (a, Some(b)));
    if digits.len() < 8 || digits.len() % 2 != 0 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let part = |a: usize, b: usize| digits.get(a..b);
    let date = format!("{}-{}-{}", part(0, 4)?, part(4, 6)?, part(6, 8)?);
    if digits.len() == 8 {
        return Some(date);
    }
    let time = format!("{}:{}:{}", part(8, 10)?, part(10, 12).unwrap_or("00"), part(12, 14).unwrap_or("00"));
    let frac = frac
        .filter(|f| f.chars().all(|c| c.is_ascii_digit()) && !f.is_empty())
        .map(|f| format!(".{f}"))
        .unwrap_or_default();
    let zone = zone
        .filter(|z| z.len() == 5 && z[1..].chars().all(|c| c.is_ascii_digit()))
        .map(|z| format!("{}{}:{}", &z[..1], &z[1..3], &z[3..5]))
        .unwrap_or_default();
    Some(format!("{date}T{time}{frac}{zone}"))
}

fn num(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok().filter(|f| f.is_finite())
}

/// OBX-7 free text into bounds: `70-99`, `<100`, `>=5`, `3.5 - 5.1`; anything else stays text.
pub fn parse_reference_range(raw: &str, unit: Option<&str>) -> Option<LabRange> {
    let t = raw.trim();
    if t.is_empty() {
        return None;
    }
    let make = |low, high| LabRange {
        low,
        high,
        unit: unit.map(str::to_string),
        text: Some(t.to_string()),
        ..LabRange::default()
    };
    for (prefix, upper) in [("<=", true), ("≤", true), (">=", false), ("≥", false), ("<", true), (">", false)] {
        if let Some(v) = t.strip_prefix(prefix).and_then(num) {
            return Some(if upper { make(None, Some(v)) } else { make(Some(v), None) });
        }
    }
    let bounds = t.char_indices().filter(|(i, c)| *c == '-' && *i > 0).find_map(|(i, _)| {
        let (lo, hi) = (num(&t[..i])?, num(&t[i + 1..])?);
        Some((lo, hi))
    });
    Some(match bounds {
        Some((lo, hi)) => make(Some(lo), Some(hi)),
        None => LabRange { unit: None, ..make(None, None) },
    })
}

/// OBX-11 (HL7 table 0085). Conservative: F and C import, the preliminary
/// family only with `--include-preliminary`, D and W retract, everything else is skipped.
fn result_status(code: &str) -> Status {
    match code.trim().to_ascii_uppercase().as_str() {
        "F" => Status::Final,
        "C" => Status::Corrected,
        "P" | "I" | "S" | "O" => Status::Preliminary,
        "R" => Status::Registered,
        "D" | "W" => Status::Retracted,
        _ => Status::Unknown,
    }
}

fn gender(code: &str) -> Option<String> {
    match code.trim().to_ascii_uppercase().as_str() {
        "M" => Some("male".into()),
        "F" => Some("female".into()),
        "O" | "A" | "N" => Some("other".into()),
        "U" => Some("unknown".into()),
        _ => None,
    }
}

fn coding(seg: &Seg, field: usize, base: usize) -> Option<Coding> {
    let code = seg.comp(field, base);
    if code.is_empty() {
        return None;
    }
    let system = seg.comp(field, base + 2);
    let system = if system.eq_ignore_ascii_case("LN") || system.eq_ignore_ascii_case("LOINC") {
        "http://loinc.org".into()
    } else {
        system
    };
    Some(Coding { system, code, display: seg.opt(field, base + 1) })
}

/// Specimen context from SPM, applied to the OBXs that follow.
#[derive(Default, Clone)]
struct Context {
    sending_facility: String,
    control_id: String,
    filler: String,
    obr_time: Option<String>,
    spm_time: Option<String>,
    specimen_type: Option<String>,
}

pub fn parse(text: &str) -> Result<Parsed> {
    let lines = segments(text);
    if !lines.iter().any(|l| l.starts_with("MSH")) {
        return Err(AppError::invalid("not an HL7 v2 message: no MSH segment"));
    }
    let mut out = Parsed::default();
    let mut i = 0;
    while i < lines.len() {
        if !lines[i].starts_with("MSH") {
            i += 1;
            continue;
        }
        let end = lines[i + 1..].iter().position(|l| l.starts_with("MSH")).map_or(lines.len(), |p| i + 1 + p);
        message(&lines[i..end], &mut out)?;
        i = end;
    }
    Ok(out)
}

fn message(lines: &[String], out: &mut Parsed) -> Result<()> {
    let d =
        Delims::from_msh(&lines[0]).ok_or_else(|| AppError::invalid("HL7 v2: unreadable MSH-1/MSH-2 delimiters"))?;
    let msh = Seg::new(&lines[0], &d);
    let (kind, trigger) = (msh.comp(9, 1), msh.comp(9, 2));
    if kind != "ORU" {
        out.ignore(&format!("message {kind}^{trigger}"));
        return Ok(());
    }
    let mut ctx = Context { sending_facility: msh.comp(4, 1), control_id: msh.comp(10, 1), ..Context::default() };
    let facility_name = Some(ctx.sending_facility.clone()).filter(|s| !s.is_empty());
    let mut idx = 0usize;
    while idx < lines.len() {
        let seg = Seg::new(&lines[idx], &d);
        match seg.name() {
            "PID" => out
                .patients
                .push(PatientHint { gender: gender(&seg.comp(8, 1)), birth_date: timestamp_to_iso(&seg.comp(7, 1)) }),
            "OBR" => {
                // a new report: its context replaces the previous one's
                ctx.filler = seg.opt(3, 1).or_else(|| seg.opt(2, 1)).unwrap_or_else(|| ctx.control_id.clone());
                ctx.obr_time = timestamp_to_iso(&seg.comp(7, 1));
                ctx.spm_time = None;
                ctx.specimen_type = None;
            }
            "SPM" => {
                ctx.specimen_type = seg.opt(4, 2).or_else(|| seg.opt(4, 1));
                // SPM-17 is a date range: start is its first component
                ctx.spm_time = timestamp_to_iso(&seg.comp(17, 1));
            }
            "OBX" => {
                // NTE segments right after this OBX belong to it
                let notes: Vec<&String> = lines[idx + 1..].iter().take_while(|l| l.starts_with("NTE")).collect();
                obx(&seg, &lines[idx], &notes, &ctx, facility_name.as_deref(), &d, out);
                idx += notes.len();
            }
            _ => {}
        }
        idx += 1;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn obx(
    seg: &Seg,
    raw_line: &str,
    notes: &[&String],
    ctx: &Context,
    facility: Option<&str>,
    d: &Delims,
    out: &mut Parsed,
) {
    let id_code = seg.comp(3, 1);
    let here = format!("OBX|{}|{}", seg.comp(1, 1), id_code);
    let codes: Vec<Coding> = [coding(seg, 3, 1), coding(seg, 3, 4)].into_iter().flatten().collect();
    let names: Vec<String> = [seg.opt(3, 2), seg.opt(3, 5)].into_iter().flatten().collect();

    // value
    let vtype = seg.comp(2, 1).to_ascii_uppercase();
    let unit = {
        let (c1, c2, sys) = (seg.comp(6, 1), seg.comp(6, 2), seg.comp(6, 3));
        let text = Some(c2.clone()).filter(|t| !t.is_empty()).or_else(|| Some(c1.clone()).filter(|t| !t.is_empty()));
        resolve_unit(
            Some(sys.as_str()).filter(|s| !s.is_empty()),
            Some(c1.as_str()).filter(|s| !s.is_empty()),
            text.as_deref(),
        )
    };
    let raw_value = seg.raw(5);
    let first_rep = raw_value.split(d.rep).next().unwrap_or("");
    let value_text = unescape(first_rep, d).trim().to_string();
    let value = match vtype.as_str() {
        "NM" => num(&value_text).map(|value| ResultValue::Numeric { value, comparator: None, unit: unit.clone() }),
        "SN" => {
            let sn: Vec<&str> = first_rep.split(d.comp).collect();
            let (cmp, v1, sep, v2) = (
                sn.first().copied().unwrap_or(""),
                sn.get(1).copied().unwrap_or(""),
                sn.get(2).copied().unwrap_or(""),
                sn.get(3).copied().unwrap_or(""),
            );
            match (matches!(cmp, "" | "=" | "<" | "<=" | ">" | ">="), num(v1), sep.is_empty() && v2.is_empty()) {
                (true, Some(value), true) => Some(ResultValue::Numeric {
                    value,
                    comparator: Some(cmp.to_string()).filter(|c| !c.is_empty() && c != "="),
                    unit: unit.clone(),
                }),
                _ => None,
            }
        }
        "ST" | "TX" | "FT" => {
            // labs often send censored numbers (`<0.5`) as strings: with a unit, read them as numbers
            let numeric = crate::util::parse_value(&value_text).ok().filter(|_| unit.is_some());
            Some(match numeric {
                Some((comparator, value)) => ResultValue::Numeric { value, comparator, unit: unit.clone() },
                None => ResultValue::Text(value_text.clone()),
            })
        }
        "CE" | "CWE" | "CNE" | "CF" | "IS" | "ID" => {
            let comps: Vec<&str> = first_rep.split(d.comp).collect();
            let pick = |i: usize| comps.get(i).map(|s| unescape(s, d).trim().to_string()).filter(|s| !s.is_empty());
            pick(1).or_else(|| pick(0)).map(ResultValue::Text)
        }
        "ED" | "RP" => {
            out.ignore("OBX attachment");
            return;
        }
        _ => {
            out.unsupported_value += 1;
            out.warnings.push(format!("{here}: value type {vtype} is not imported"));
            return;
        }
    };
    let Some(value) = value else {
        if value_text.is_empty() {
            out.no_value += 1;
        } else {
            out.unsupported_value += 1;
            out.warnings.push(format!("{here}: value '{value_text}' is not a {vtype}"));
        }
        return;
    };

    // status, time, lab
    let status = result_status(&seg.comp(11, 1));
    let obx_time = timestamp_to_iso(&seg.comp(14, 1));
    let (taken_at, time_basis) = [
        (obx_time, TimeBasis::Obx),
        (ctx.obr_time.clone(), TimeBasis::Obr),
        (ctx.spm_time.clone(), TimeBasis::Specimen),
    ]
    .into_iter()
    .find_map(|(t, b)| t.map(|t| (Some(t), Some(b))))
    .unwrap_or((None, None));
    let lab = seg.opt(23, 1).or_else(|| facility.map(str::to_string));

    let mut ranges: Vec<LabRange> = parse_reference_range(&seg.comp(7, 1), unit.as_deref()).into_iter().collect();
    ranges.retain(|r| r.low.is_some() || r.high.is_some() || r.text.is_some());

    let key = format!("{}|{}|{}|{}", ctx.sending_facility, ctx.filler, seg.comp(1, 1), id_code);
    let mut hashed = raw_line.to_string();
    notes.iter().for_each(|n| {
        hashed.push('\n');
        hashed.push_str(n);
    });
    let note_texts: Vec<String> = notes.iter().map(|n| Seg::new(n, d)).flat_map(|n| n.repetitions(3)).collect();

    out.candidates.push(ResultCandidate {
        kind: SourceKind::Hl7v2,
        key,
        version: None,
        sha256: sha256_hex(hashed.as_bytes()),
        origin: here,
        category: Some("laboratory".into()),
        status,
        codes,
        names,
        taken_at,
        time_basis,
        lab,
        fasting: None,
        specimen_type: ctx.specimen_type.clone(),
        value,
        ranges,
        interpretation: seg.repetitions(8).into_iter().map(|f| f.to_ascii_uppercase()).collect(),
        notes: note_texts,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oru() -> String {
        [
            "MSH|^~\\&|LABSYS|EXAMPLELAB|EHR|CLINIC|20240305093000||ORU^R01|MSG0001|P|2.5".to_string(),
            "PID|1||SYN-0001||Example^Pat||19840601|M".to_string(),
            "OBR|1|PL-1|FL-9001^EXAMPLELAB|24323-8^Comprehensive metabolic panel^LN|||20240305080000".to_string(),
            format!("SPM|1|||SER^Serum^HL70487{}20240305080000", "|".repeat(13)),
            "OBX|1|NM|2345-7^Glucose^LN||92|mg/dL|70-99|N|||F|||20240305080000".to_string(),
            "OBX|2|NM|2093-3^Cholesterol, total^LN||245|mg/dL|<200|H|||F".to_string(),
            "NTE|1||Fasting sample".to_string(),
            "OBX|3|SN|2571-8^Triglycerides^LN||<^45|mg/dL|<150|N|||F".to_string(),
            "OBX|4|ST|5778-6^Color of Urine^LN||Yellow\\F\\amber||||||F".to_string(),
            "OBX|5|NM|9999-9^Prelim thing^L||5|mmol/L|||||P".to_string(),
        ]
        .join("\r")
            + "\r"
    }

    #[test]
    fn reads_an_oru_message() {
        let p = parse(&oru()).unwrap();
        assert_eq!(p.candidates.len(), 5);
        let g = &p.candidates[0];
        assert_eq!(g.kind, SourceKind::Hl7v2);
        assert_eq!(g.loinc_codes().collect::<Vec<_>>(), vec!["2345-7"]);
        assert_eq!(g.value, ResultValue::Numeric { value: 92.0, comparator: None, unit: Some("mg/dL".into()) });
        assert_eq!(g.taken_at.as_deref(), Some("2024-03-05T08:00:00"));
        assert_eq!(g.time_basis, Some(TimeBasis::Obx));
        assert_eq!(g.key, "EXAMPLELAB|FL-9001|1|2345-7");
        assert_eq!(g.ranges[0].low, Some(70.0));
        assert_eq!(g.ranges[0].high, Some(99.0));
        assert_eq!(g.status, Status::Final);
        assert_eq!(g.specimen_type.as_deref(), Some("Serum"));
        assert_eq!(g.lab.as_deref(), Some("EXAMPLELAB"));
        assert_eq!(g.interpretation, vec!["N".to_string()]);
        assert_eq!(p.patients[0].gender.as_deref(), Some("male"));
        assert_eq!(p.patients[0].birth_date.as_deref(), Some("1984-06-01"));
    }

    #[test]
    fn obr_time_and_notes_apply_when_obx_has_none() {
        let p = parse(&oru()).unwrap();
        let chol = &p.candidates[1];
        assert_eq!(chol.time_basis, Some(TimeBasis::Obr));
        assert_eq!(chol.taken_at.as_deref(), Some("2024-03-05T08:00:00"));
        assert_eq!(chol.notes, vec!["Fasting sample".to_string()]);
        assert_eq!(chol.ranges[0].high, Some(200.0));
        assert_eq!(chol.ranges[0].low, None);
        assert_eq!(chol.interpretation, vec!["H".to_string()]);
    }

    #[test]
    fn structured_numeric_strings_escapes_and_status() {
        let p = parse(&oru()).unwrap();
        assert!(
            matches!(&p.candidates[2].value, ResultValue::Numeric { value, comparator: Some(c), .. } if *value == 45.0 && c == "<")
        );
        assert_eq!(p.candidates[3].value, ResultValue::Text("Yellow|amber".into()));
        assert_eq!(p.candidates[4].status, Status::Preliminary);
        // a local code: system kept, not LOINC
        assert_eq!(p.candidates[4].loinc_codes().count(), 0);
    }

    #[test]
    fn accepts_lf_crlf_batches_and_other_messages() {
        let text = oru().replace('\r', "\r\n");
        let two = format!("{text}{}", "MSH|^~\\&|A|B|||20240306||ADT^A01|X|P|2.5\r\nPID|1||Q\r\n");
        let p = parse(&two).unwrap();
        assert_eq!(p.candidates.len(), 5);
        assert_eq!(p.ignored.get("message ADT^A01"), Some(&1));
        assert!(parse("PID|1||x").is_err());
    }

    #[test]
    fn timestamps() {
        assert_eq!(timestamp_to_iso("20240305").as_deref(), Some("2024-03-05"));
        assert_eq!(timestamp_to_iso("202403050830").as_deref(), Some("2024-03-05T08:30:00"));
        assert_eq!(timestamp_to_iso("20240305083015.25-0500").as_deref(), Some("2024-03-05T08:30:15.25-05:00"));
        assert_eq!(timestamp_to_iso("2024").as_deref(), None);
        assert_eq!(timestamp_to_iso("").as_deref(), None);
    }

    #[test]
    fn reference_range_forms() {
        let r = |s: &str| parse_reference_range(s, Some("mg/dL")).map(|r| (r.low, r.high));
        assert_eq!(r("70-99"), Some((Some(70.0), Some(99.0))));
        assert_eq!(r("3.5 - 5.1"), Some((Some(3.5), Some(5.1))));
        assert_eq!(r("<100"), Some((None, Some(100.0))));
        assert_eq!(r(">=5"), Some((Some(5.0), None)));
        assert_eq!(r("-5 - 5"), Some((Some(-5.0), Some(5.0))));
        assert_eq!(r("NEGATIVE"), Some((None, None)));
        assert_eq!(r(""), None);
    }
}
