//! Built-in marker catalog, units, conversions and default ranges.
//!
//! Ranges are typical adult values drawn from common lab references and are
//! informational only. Labs differ; override them with `biomarker range set`.
//! "Optimal" ranges are commonly cited preventive-medicine targets, kept
//! separate from the lab reference ranges.

use crate::db::{int, opt_real, opt_text, real, text, Db};
use crate::error::Result;

/// (symbol, system, description)
pub const UNITS: &[(&str, &str, &str)] = &[
    ("mg/dL", "us", "milligrams per decilitre"),
    ("g/dL", "us", "grams per decilitre"),
    ("µg/dL", "us", "micrograms per decilitre"),
    ("ng/dL", "us", "nanograms per decilitre"),
    ("ng/mL", "us", "nanograms per millilitre"),
    ("pg/mL", "us", "picograms per millilitre"),
    ("µIU/mL", "us", "micro international units per millilitre"),
    ("mEq/L", "us", "milliequivalents per litre"),
    ("10^3/µL", "us", "thousands per microlitre"),
    ("10^6/µL", "us", "millions per microlitre"),
    ("mmol/L", "si", "millimoles per litre"),
    ("µmol/L", "si", "micromoles per litre"),
    ("nmol/L", "si", "nanomoles per litre"),
    ("pmol/L", "si", "picomoles per litre"),
    ("g/L", "si", "grams per litre"),
    ("µg/L", "si", "micrograms per litre"),
    ("mmol/mol", "si", "millimoles per mole (IFCC HbA1c)"),
    ("10^9/L", "si", "billions per litre"),
    ("10^12/L", "si", "trillions per litre"),
    ("L/L", "si", "litre per litre (fraction)"),
    ("µkat/L", "si", "microkatals per litre"),
    ("mg/L", "both", "milligrams per litre"),
    ("%", "both", "percent"),
    ("U/L", "both", "units per litre"),
    ("IU/L", "both", "international units per litre"),
    ("mIU/L", "both", "milli international units per litre"),
    ("mIU/mL", "both", "milli international units per millilitre"),
    ("IU/mL", "both", "international units per millilitre"),
    ("fL", "both", "femtolitres"),
    ("pg", "both", "picograms"),
    ("mL/min/1.73m²", "both", "estimated GFR"),
    ("ratio", "both", "dimensionless ratio"),
];

/// Conversions that apply to every marker: (from, to, factor). `to = from * factor`.
pub const GENERIC_CONVERSIONS: &[(&str, &str, f64)] = &[
    ("g/L", "g/dL", 0.1),
    ("g/L", "mg/dL", 100.0),
    ("mg/dL", "mg/L", 10.0),
    ("µg/L", "ng/mL", 1.0),
    ("10^9/L", "10^3/µL", 1.0),
    ("10^12/L", "10^6/µL", 1.0),
    ("mEq/L", "mmol/L", 1.0),
    ("µkat/L", "U/L", 60.0),
    ("IU/L", "U/L", 1.0),
    ("µIU/mL", "mIU/L", 1.0),
    ("mIU/mL", "IU/L", 1.0),
    ("L/L", "%", 100.0),
];

pub struct SeedMarker {
    pub slug: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub unit: &'static str,
    pub loinc: Option<&'static str>,
    pub aliases: &'static [&'static str],
    /// Marker-specific conversions into the canonical unit: (from_unit, factor, offset).
    pub conversions: &'static [(&'static str, f64, f64)],
    /// Reference ranges: (sex, low, high).
    pub reference: &'static [(&'static str, Option<f64>, Option<f64>)],
    /// Optimal range: (low, high).
    pub optimal: Option<(Option<f64>, Option<f64>)>,
}

const N: Option<f64> = None;
const fn s(v: f64) -> Option<f64> {
    Some(v)
}

macro_rules! m {
    ($slug:expr, $name:expr, $cat:expr, $unit:expr, $loinc:expr, [$($a:expr),*], [$($c:expr),*], [$($r:expr),*], $opt:expr) => {
        SeedMarker {
            slug: $slug, name: $name, category: $cat, unit: $unit, loinc: $loinc,
            aliases: &[$($a),*], conversions: &[$($c),*], reference: &[$($r),*], optimal: $opt,
        }
    };
}

#[rustfmt::skip]
pub const MARKERS: &[SeedMarker] = &[
    // Lipid panel
    m!("total-cholesterol", "Total Cholesterol", "lipid", "mg/dL", Some("2093-3"), ["tc", "cholesterol", "chol", "cholesterol, total"], [("mmol/L", 38.67, 0.0)], [("any", N, s(200.0))], Some((N, s(180.0)))),
    m!("ldl-c", "LDL Cholesterol", "lipid", "mg/dL", Some("13457-7"), ["ldl", "ldl cholesterol", "ldl-cholesterol", "ldl chol calc (nih)"], [("mmol/L", 38.67, 0.0)], [("any", N, s(100.0))], Some((N, s(70.0)))),
    m!("hdl-c", "HDL Cholesterol", "lipid", "mg/dL", Some("2085-9"), ["hdl", "hdl cholesterol", "hdl-cholesterol"], [("mmol/L", 38.67, 0.0)], [("male", s(40.0), N), ("female", s(50.0), N), ("any", s(40.0), N)], Some((s(60.0), N))),
    m!("triglycerides", "Triglycerides", "lipid", "mg/dL", Some("2571-8"), ["tg", "trig", "triglyceride"], [("mmol/L", 88.57, 0.0)], [("any", N, s(150.0))], Some((N, s(100.0)))),
    m!("non-hdl-c", "Non-HDL Cholesterol", "lipid", "mg/dL", Some("43396-1"), ["non-hdl", "non hdl cholesterol"], [("mmol/L", 38.67, 0.0)], [("any", N, s(130.0))], Some((N, s(100.0)))),
    m!("apob", "Apolipoprotein B", "lipid", "mg/dL", Some("1884-6"), ["apo b", "apolipoprotein b", "apo-b"], [], [("any", N, s(90.0))], Some((N, s(80.0)))),
    m!("lpa", "Lipoprotein(a)", "lipid", "nmol/L", Some("43583-4"), ["lp(a)", "lipoprotein a", "lipoprotein(a)", "lpa"], [("mg/dL", 2.15, 0.0)], [("any", N, s(75.0))], Some((N, s(30.0)))),
    // Metabolic
    m!("glucose", "Glucose", "metabolic", "mg/dL", Some("2345-7"), ["glu", "fasting glucose", "blood glucose", "glucose, serum"], [("mmol/L", 18.016, 0.0)], [("any", s(70.0), s(99.0))], Some((s(72.0), s(90.0)))),
    m!("hba1c", "Hemoglobin A1c", "metabolic", "%", Some("4548-4"), ["a1c", "hemoglobin a1c", "glycated hemoglobin", "hb a1c"], [("mmol/mol", 0.09148, 2.152)], [("any", s(4.0), s(5.6))], Some((N, s(5.3)))),
    m!("insulin", "Insulin (fasting)", "metabolic", "µIU/mL", Some("20448-7"), ["fasting insulin"], [("pmol/L", 0.144, 0.0)], [("any", s(2.6), s(24.9))], Some((s(2.0), s(6.0)))),
    m!("uric-acid", "Uric Acid", "metabolic", "mg/dL", Some("3084-1"), ["urate"], [("µmol/L", 0.016_81, 0.0)], [("male", s(3.7), s(8.0)), ("female", s(2.5), s(7.1)), ("any", s(2.5), s(8.0))], Some((N, s(6.0)))),
    // CMP
    m!("sodium", "Sodium", "cmp", "mmol/L", Some("2951-2"), ["na"], [], [("any", s(135.0), s(145.0))], None),
    m!("potassium", "Potassium", "cmp", "mmol/L", Some("2823-3"), ["k"], [], [("any", s(3.5), s(5.2))], None),
    m!("chloride", "Chloride", "cmp", "mmol/L", Some("2075-0"), ["cl"], [], [("any", s(96.0), s(106.0))], None),
    m!("co2", "Carbon Dioxide (Bicarbonate)", "cmp", "mmol/L", Some("2028-9"), ["bicarbonate", "hco3", "carbon dioxide, total"], [], [("any", s(20.0), s(29.0))], None),
    m!("bun", "Blood Urea Nitrogen", "cmp", "mg/dL", Some("3094-0"), ["urea nitrogen", "urea"], [("mmol/L", 2.801, 0.0)], [("any", s(6.0), s(24.0))], None),
    m!("creatinine", "Creatinine", "cmp", "mg/dL", Some("2160-0"), ["creat", "creatinine, serum"], [("µmol/L", 0.011_31, 0.0)], [("male", s(0.76), s(1.27)), ("female", s(0.57), s(1.0)), ("any", s(0.57), s(1.27))], None),
    m!("egfr", "eGFR", "cmp", "mL/min/1.73m²", Some("98979-8"), ["gfr", "estimated gfr"], [], [("any", s(60.0), N)], Some((s(90.0), N))),
    m!("calcium", "Calcium", "cmp", "mg/dL", Some("17861-6"), ["ca"], [("mmol/L", 4.008, 0.0)], [("any", s(8.6), s(10.2))], None),
    m!("total-protein", "Total Protein", "cmp", "g/dL", Some("2885-2"), ["protein, total"], [], [("any", s(6.0), s(8.5))], None),
    m!("albumin", "Albumin", "cmp", "g/dL", Some("1751-7"), ["alb"], [], [("any", s(3.5), s(5.5))], Some((s(4.2), s(5.0)))),
    m!("bilirubin-total", "Bilirubin, Total", "cmp", "mg/dL", Some("1975-2"), ["bilirubin", "tbil", "total bilirubin"], [("µmol/L", 0.058_48, 0.0)], [("any", s(0.1), s(1.2))], None),
    m!("alp", "Alkaline Phosphatase", "cmp", "U/L", Some("6768-6"), ["alkaline phosphatase"], [], [("any", s(44.0), s(121.0))], None),
    m!("ast", "AST (SGOT)", "cmp", "U/L", Some("1920-8"), ["sgot", "aspartate aminotransferase"], [], [("any", s(0.0), s(40.0))], Some((N, s(25.0)))),
    m!("alt", "ALT (SGPT)", "cmp", "U/L", Some("1742-6"), ["sgpt", "alanine aminotransferase"], [], [("any", s(0.0), s(44.0))], Some((N, s(25.0)))),
    m!("ggt", "Gamma-Glutamyl Transferase", "liver", "U/L", Some("2324-2"), ["gamma gt", "ggtp"], [], [("male", s(0.0), s(65.0)), ("female", s(0.0), s(45.0)), ("any", s(0.0), s(65.0))], Some((N, s(25.0)))),
    m!("magnesium", "Magnesium", "electrolyte", "mg/dL", Some("19123-9"), ["mg"], [("mmol/L", 2.431, 0.0)], [("any", s(1.6), s(2.3))], None),
    // CBC
    m!("wbc", "White Blood Cells", "cbc", "10^3/µL", Some("6690-2"), ["white blood cell count", "leukocytes"], [], [("any", s(3.4), s(10.8))], Some((s(4.0), s(7.5)))),
    m!("rbc", "Red Blood Cells", "cbc", "10^6/µL", Some("789-8"), ["red blood cell count", "erythrocytes"], [], [("male", s(4.14), s(5.8)), ("female", s(3.77), s(5.28)), ("any", s(3.77), s(5.8))], None),
    m!("hemoglobin", "Hemoglobin", "cbc", "g/dL", Some("718-7"), ["hgb", "hb", "haemoglobin"], [("mmol/L", 1.611, 0.0)], [("male", s(13.0), s(17.7)), ("female", s(11.1), s(15.9)), ("any", s(11.1), s(17.7))], None),
    m!("hematocrit", "Hematocrit", "cbc", "%", Some("4544-3"), ["hct", "haematocrit"], [], [("male", s(37.5), s(51.0)), ("female", s(34.0), s(46.6)), ("any", s(34.0), s(51.0))], None),
    m!("mcv", "Mean Corpuscular Volume", "cbc", "fL", Some("787-2"), [], [], [("any", s(79.0), s(97.0))], None),
    m!("mch", "Mean Corpuscular Hemoglobin", "cbc", "pg", Some("785-6"), [], [], [("any", s(26.6), s(33.0))], None),
    m!("mchc", "Mean Corpuscular Hemoglobin Concentration", "cbc", "g/dL", Some("786-4"), [], [], [("any", s(31.5), s(35.7))], None),
    m!("rdw", "Red Cell Distribution Width", "cbc", "%", Some("788-0"), ["rdw-cv"], [], [("any", s(11.6), s(15.4))], None),
    m!("platelets", "Platelets", "cbc", "10^3/µL", Some("777-3"), ["plt", "platelet count"], [], [("any", s(150.0), s(450.0))], None),
    m!("neutrophils-abs", "Neutrophils (Absolute)", "cbc", "10^3/µL", Some("751-8"), ["neutrophils", "anc"], [], [("any", s(1.4), s(7.0))], None),
    m!("lymphocytes-abs", "Lymphocytes (Absolute)", "cbc", "10^3/µL", Some("731-0"), ["lymphocytes", "lymphs (absolute)"], [], [("any", s(0.7), s(3.1))], None),
    // Thyroid
    m!("tsh", "Thyroid Stimulating Hormone", "thyroid", "mIU/L", Some("3016-3"), ["thyrotropin"], [], [("any", s(0.45), s(4.5))], Some((s(0.5), s(2.5)))),
    m!("free-t4", "Free T4", "thyroid", "ng/dL", Some("3024-7"), ["ft4", "t4, free", "free thyroxine"], [("pmol/L", 0.0777, 0.0)], [("any", s(0.82), s(1.77))], None),
    m!("free-t3", "Free T3", "thyroid", "pg/mL", Some("3051-0"), ["ft3", "t3, free", "free triiodothyronine"], [("pmol/L", 0.651, 0.0)], [("any", s(2.0), s(4.4))], None),
    m!("tpo-ab", "Thyroid Peroxidase Antibodies", "thyroid", "IU/mL", Some("8099-4"), ["tpo", "anti-tpo"], [], [("any", N, s(34.0))], None),
    // Hormones
    m!("testosterone-total", "Testosterone, Total", "hormone", "ng/dL", Some("2986-8"), ["testosterone", "total testosterone", "tt"], [("nmol/L", 28.84, 0.0)], [("male", s(264.0), s(916.0)), ("female", s(8.0), s(48.0))], None),
    m!("testosterone-free", "Testosterone, Free", "hormone", "pg/mL", Some("2991-8"), ["free testosterone", "ft"], [("pmol/L", 0.2884, 0.0)], [("male", s(46.0), s(224.0)), ("female", s(0.0), s(4.2))], None),
    m!("estradiol", "Estradiol", "hormone", "pg/mL", Some("2243-4"), ["e2", "oestradiol"], [("pmol/L", 0.2724, 0.0)], [("male", s(7.6), s(42.6))], None),
    m!("shbg", "Sex Hormone Binding Globulin", "hormone", "nmol/L", Some("13967-5"), ["sex hormone binding globulin"], [], [("male", s(16.5), s(55.9)), ("female", s(24.6), s(122.0))], None),
    m!("dhea-s", "DHEA-Sulfate", "hormone", "µg/dL", Some("2191-5"), ["dheas", "dhea sulfate"], [("µmol/L", 36.81, 0.0)], [("any", s(35.0), s(430.0))], None),
    m!("cortisol", "Cortisol (AM)", "hormone", "µg/dL", Some("2143-6"), ["am cortisol"], [("nmol/L", 0.036_25, 0.0)], [("any", s(6.2), s(19.4))], None),
    m!("lh", "Luteinizing Hormone", "hormone", "mIU/mL", Some("10501-5"), [], [], [("male", s(1.7), s(8.6))], None),
    m!("fsh", "Follicle Stimulating Hormone", "hormone", "mIU/mL", Some("15067-2"), [], [], [("male", s(1.5), s(12.4))], None),
    m!("prolactin", "Prolactin", "hormone", "ng/mL", Some("2842-3"), ["prl"], [], [("male", s(4.0), s(15.2)), ("female", s(4.8), s(23.3))], None),
    m!("psa", "Prostate Specific Antigen", "hormone", "ng/mL", Some("2857-1"), ["psa, total"], [], [("male", s(0.0), s(4.0))], None),
    // Inflammation
    m!("hscrp", "hs-CRP", "inflammation", "mg/L", Some("30522-7"), ["crp", "c-reactive protein", "hs crp", "high sensitivity crp", "c-reactive protein, cardiac"], [], [("any", N, s(3.0))], Some((N, s(1.0)))),
    m!("homocysteine", "Homocysteine", "inflammation", "µmol/L", Some("13965-9"), ["hcy"], [], [("any", s(0.0), s(15.0))], Some((N, s(10.0)))),
    // Vitamins / iron
    m!("vitamin-d", "Vitamin D, 25-Hydroxy", "vitamin", "ng/mL", Some("1989-3"), ["25-oh vitamin d", "vit d", "25(oh)d", "vitamin d, 25-hydroxy", "25-hydroxyvitamin d"], [("nmol/L", 0.4006, 0.0)], [("any", s(30.0), s(100.0))], Some((s(40.0), s(60.0)))),
    m!("vitamin-b12", "Vitamin B12", "vitamin", "pg/mL", Some("2132-9"), ["b12", "cobalamin"], [("pmol/L", 1.355, 0.0)], [("any", s(232.0), s(1245.0))], Some((s(500.0), N))),
    m!("folate", "Folate", "vitamin", "ng/mL", Some("2284-8"), ["folic acid"], [("nmol/L", 0.4413, 0.0)], [("any", s(3.0), N)], None),
    m!("ferritin", "Ferritin", "iron", "ng/mL", Some("2276-4"), ["ferr"], [], [("male", s(30.0), s(400.0)), ("female", s(15.0), s(150.0)), ("any", s(15.0), s(400.0))], Some((s(50.0), s(150.0)))),
    m!("iron", "Iron, Serum", "iron", "µg/dL", Some("2498-4"), ["serum iron", "fe"], [("µmol/L", 5.585, 0.0)], [("male", s(38.0), s(169.0)), ("female", s(27.0), s(159.0)), ("any", s(27.0), s(169.0))], None),
    m!("tibc", "Total Iron Binding Capacity", "iron", "µg/dL", Some("2500-7"), [], [("µmol/L", 5.585, 0.0)], [("any", s(250.0), s(450.0))], None),
    m!("transferrin-saturation", "Transferrin Saturation", "iron", "%", Some("2502-3"), ["tsat", "iron saturation"], [], [("any", s(15.0), s(55.0))], Some((s(25.0), s(45.0)))),
];

pub fn seed(db: &Db) -> Result<()> {
    UNITS.iter().try_for_each(|(sym, sys, desc)| {
        db.execute(
            "INSERT OR IGNORE INTO units (symbol, system, description) VALUES (?1, ?2, ?3)",
            &[text(sym), text(sys), text(desc)],
        )
        .map(|_| ())
    })?;
    GENERIC_CONVERSIONS.iter().try_for_each(|(from, to, f)| {
        db.execute(
            "INSERT OR IGNORE INTO unit_conversions (marker_id, from_unit, to_unit, factor, offset, builtin)
             VALUES (0, ?1, ?2, ?3, 0, 1)",
            &[text(from), text(to), real(*f)],
        )
        .map(|_| ())
    })?;
    MARKERS.iter().try_for_each(|m| seed_marker(db, m))
}

fn seed_marker(db: &Db, m: &SeedMarker) -> Result<()> {
    db.execute(
        "INSERT OR IGNORE INTO markers (slug, name, category, unit, loinc, builtin) VALUES (?1, ?2, ?3, ?4, ?5, 1)",
        &[text(m.slug), text(m.name), text(m.category), text(m.unit), opt_text(m.loinc)],
    )?;
    let id = db.query_scalar_i64("SELECT id FROM markers WHERE slug = ?1", &[text(m.slug)])?;
    m.aliases.iter().try_for_each(|a| {
        db.execute(
            "INSERT OR IGNORE INTO marker_aliases (alias, marker_id) VALUES (?1, ?2)",
            &[text(a.to_lowercase()), int(id)],
        )
        .map(|_| ())
    })?;
    m.conversions.iter().try_for_each(|(from, factor, offset)| {
        db.execute(
            "INSERT OR IGNORE INTO unit_conversions (marker_id, from_unit, to_unit, factor, offset, builtin)
             VALUES (?1, ?2, ?3, ?4, ?5, 1)",
            &[int(id), text(from), text(m.unit), real(*factor), real(*offset)],
        )
        .map(|_| ())
    })?;
    let refs = m.reference.iter().map(|(sex, lo, hi)| ("reference", *sex, *lo, *hi));
    let opt = m.optimal.iter().map(|(lo, hi)| ("optimal", "any", *lo, *hi));
    refs.chain(opt).try_for_each(|(kind, sex, lo, hi)| {
        db.execute(
            "INSERT OR IGNORE INTO ranges (marker_id, kind, sex, low, high) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[int(id), text(kind), text(sex), opt_real(lo), opt_real(hi)],
        )
        .map(|_| ())
    })
}
