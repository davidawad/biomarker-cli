//! Synthetic spreadsheets for the import tests and `examples/dashboard.xlsx`
//! (regenerate with `cargo run --example make-sheets`). Every value is made up.

use std::path::Path;

use chrono::NaiveDate;
use rust_xlsxwriter::{DocProperties, ExcelDateTime, Format, Workbook, Worksheet, XlsxError};

/// Excel (1900 system) serial number of a date.
pub fn serial(y: i32, m: u32, d: u32) -> f64 {
    let base = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();
    (NaiveDate::from_ymd_opt(y, m, d).unwrap() - base).num_days() as f64
}

/// A cell in a synthetic row.
#[derive(Clone, Copy)]
pub enum V {
    N(f64),
    S(&'static str),
    E,
}

fn put(ws: &mut Worksheet, row: u32, col: u16, v: V) -> Result<(), XlsxError> {
    match v {
        V::N(n) => ws.write_number(row, col, n).map(|_| ()),
        V::S(s) => ws.write_string(row, col, s).map(|_| ()),
        V::E => Ok(()),
    }
}

use V::{E, N, S};

/// (test, notes, units, reference interval, [ref. low, ref. high], values per date)
pub type LabRow = (&'static str, &'static str, &'static str, &'static str, [V; 2], [V; 4]);
/// ((year, month, day), values)
pub type BodyRow = ((i32, u32, u32), [V; 5]);

/// Dashboard sheet "Labs": one row per test, one column per draw date.
/// Header dates are a date-formatted serial, a bare serial, ISO text and US text.
#[rustfmt::skip]
pub const LABS: &[LabRow] = &[
    ("Complete Blood Count (CBC)", "", "", "", [E, E], [E, E, E, E]),
    ("White Blood Cell Count (WBC)", "", "x10E3/uL", "3.4-10.8", [N(3.4), N(10.8)], [N(5.6), N(6.1), N(4.9), N(5.2)]),
    ("Red Blood Cell Count (RBC)", "", "x10E6/uL", "4.14-5.80", [N(4.14), N(5.8)], [N(4.9), N(5.0), N(4.8), N(5.1)]),
    ("Hemoglobin (Hgb)", "", "g/dL", "13.0-17.7", [N(13.0), N(17.7)], [N(15.1), N(15.3), N(14.9), N(15.4)]),
    ("Mean Corpuscular Volume (MCV)", "", "μm³", "79-97", [N(79.0), N(97.0)], [N(90.0), N(91.0), N(89.0), N(90.0)]),
    ("Platelets", "", "μm³ (x10E3/uL)", "150-450", [N(150.0), N(450.0)], [N(250.0), N(240.0), N(262.0), N(255.0)]),
    ("Lipid Panel + ApoB", "", "", "", [E, E], [E, E, E, E]),
    ("Low-Density Lipoprotein (LDL-C)", "calculated", "mg/dL", "0-99", [N(0.0), N(99.0)], [N(131.0), N(118.0), N(104.0), N(96.0)]),
    ("High-Density Lipoprotein (HDL-C)", "", "mg/dL", ">39", [N(39.0), E], [N(52.0), N(55.0), N(58.0), N(60.0)]),
    ("Apolipoprotein B (ApoB)", "", "mg/dL", "<90", [E, N(90.0)], [N(104.0), N(96.0), N(88.0), N(84.0)]),
    ("Lipoprotein Particle Score", "research", "score", "", [E, E], [N(12.0), E, N(9.0), E]),
    ("Metabolic", "", "", "", [E, E], [E, E, E, E]),
    ("Hemoglobin A1c", "", "%Hb", "4.8-5.6", [N(4.8), N(5.6)], [N(5.6), N(5.5), N(5.4), S("5.3")]),
    ("ALT (SGPT)", "", "IU/L", "0-44", [N(0.0), N(44.0)], [N(31.0), N(28.0), N(25.0), N(22.0)]),
    ("Thyroid Stimulating Hormone (TSH)", "", "uIU/ml", "0.450-4.500", [N(0.45), N(4.5)], [N(2.1), N(1.9), E, N(2.3)]),
    ("Urinalysis", "", "", "", [E, E], [E, E, E, E]),
    ("Urine Protein", "", "", "Negative", [E, E], [S("Negative"), S("Negative"), S("1+ Abnormal"), S("Negative")]),
    ("Urine Appearance", "", "", "Clear", [E, E], [S("Clear"), S("Clear"), S("Clear"), E]),
    ("WBC, Urine", "", "/hpf", "0-5", [N(0.0), N(5.0)], [S("None seen"), S("0-5"), S("6-10 Abnormal"), S("None seen")]),
    ("Control Sample", "lab QC", "", "", [E, E], [N(1.0), N(1.0), N(1.0), N(1.0)]),
];

/// Draw dates of the four value columns (2024-12-02 is the column the
/// mapping example corrects to the lab report's 2024-12-04).
pub const LAB_DATES: [(i32, u32, u32); 4] = [(2024, 3, 1), (2024, 8, 15), (2024, 12, 2), (2025, 3, 11)];

/// Long sheet "Body": one row per date, several value columns.
#[rustfmt::skip]
pub const BODY: &[BodyRow] = &[
    ((2024, 1, 6), [N(184.2), N(25.7), N(22.4), N(41.5), N(61.0)]),
    ((2024, 4, 6), [N(181.0), N(25.2), N(21.1), N(40.8), N(59.0)]),
    ((2024, 7, 6), [N(178.6), N(24.9), N(20.3), E, N(57.0)]),
    ((2024, 10, 5), [N(176.4), N(24.6), N(19.6), N(39.9), N(56.0)]),
];

pub const BODY_HEADERS: [&str; 6] =
    ["Date", "Weight (lbs.)", "BMI kg/m²", "Body Fat %", "Biological Age", "Resting HR"];

fn labs(ws: &mut Worksheet) -> Result<(), XlsxError> {
    ws.set_name("Labs")?;
    let date_fmt = Format::new().set_num_format("m/d/yyyy");
    ws.write_string(0, 0, "Health dashboard (synthetic example data)")?;
    for (c, h) in
        ["Test", "Notes", "Control", "Units", "Reference Interval", "ref. low", "ref. high"].iter().enumerate()
    {
        ws.write_string(2, c as u16, *h)?;
    }
    let [(y0, m0, d0), (y1, m1, d1), (y2, m2, d2), (y3, m3, d3)] = LAB_DATES;
    ws.write_number_with_format(2, 7, serial(y0, m0, d0), &date_fmt)?;
    ws.write_number(2, 8, serial(y1, m1, d1))?;
    ws.write_string(2, 9, format!("{y2:04}-{m2:02}-{d2:02}"))?;
    ws.write_string(2, 10, format!("{m3}/{d3}/{y3}"))?;
    for (i, (test, notes, units, interval, refs, values)) in LABS.iter().enumerate() {
        let r = 3 + i as u32;
        ws.write_string(r, 0, *test)?;
        put(ws, r, 1, if notes.is_empty() { E } else { S(notes) })?;
        if *test == "Control Sample" {
            ws.write_string(r, 2, "x")?;
        }
        put(ws, r, 3, if units.is_empty() { E } else { S(units) })?;
        put(ws, r, 4, if interval.is_empty() { E } else { S(interval) })?;
        put(ws, r, 5, refs[0])?;
        put(ws, r, 6, refs[1])?;
        for (j, v) in values.iter().enumerate() {
            put(ws, r, 7 + j as u16, *v)?;
        }
    }
    Ok(())
}

fn body(ws: &mut Worksheet) -> Result<(), XlsxError> {
    ws.set_name("Body")?;
    let date_fmt = Format::new().set_num_format("yyyy-mm-dd");
    for (c, h) in BODY_HEADERS.iter().enumerate() {
        ws.write_string(0, c as u16, *h)?;
    }
    for (i, ((y, m, d), values)) in BODY.iter().enumerate() {
        let r = 1 + i as u32;
        ws.write_number_with_format(r, 0, serial(*y, *m, *d), &date_fmt)?;
        for (j, v) in values.iter().enumerate() {
            put(ws, r, 1 + j as u16, *v)?;
        }
    }
    Ok(())
}

/// Workbook with the "Labs" dashboard and the "Body" log.
pub fn dashboard(path: &Path) -> Result<(), XlsxError> {
    let mut wb = Workbook::new();
    let when = ExcelDateTime::from_ymd(2026, 1, 1)?;
    wb.set_properties(&DocProperties::new().set_creation_datetime(&when).set_author("biomarker-cli"));
    labs(wb.add_worksheet())?;
    body(wb.add_worksheet())?;
    wb.save(path)
}
