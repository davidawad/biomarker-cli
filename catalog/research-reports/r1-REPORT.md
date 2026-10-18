# Research shard r1: report

Shard: chemistry and metabolic panel, glycemic, liver, kidney, electrolytes and minerals, lipids and lipoproteins.
Bead: biomarker-cli-8h7.3.1. One TOML file per marker in this directory.

## Format used (read this before merging)

The epic bead (biomarker-cli-8h7) was **not** shipped to this box, so its "Ranges with structured
conditions and mandatory citations" section could not be read. The files follow the field list in the
shard brief exactly: top level `slug, name, loinc, category, unit, description`; each `[[range]]` has
`set, kind, low, high, when, population, evidence, citations = [{pmid, doi, url, locator, quote}], note`.
Choices that the brief did not pin down, so the merge step can map them mechanically:

- `set`: `reference` (population reference-interval studies), `clinical` (guideline/consensus cut-offs,
  categories and treatment goals), `optimal` (outcome-study "lowest risk" bands).
- `kind`: `reference`, `optimal`, `warn` (match src/ranges.rs RangeKind) plus `diagnostic` (disease or
  category cut-offs), `target` (treatment goals, always with `when.conditions`/`when.risk`) and `critical`.
- `when` keys: `sex` (male/female), `age_min`/`age_max` (years, half-open, fractional for infants),
  `pregnant`, `trimester`, `postpartum`, `menopause`, `fasting`, `ethnicity`, `conditions` (list),
  `risk`, `markers` (other-marker thresholds; unused so far), `specimen`, `method`.
- `evidence`: guideline, consensus, reference-interval-study, outcome-study, meta-analysis, textbook.
- Two additions: a range-level `unit` when the source states numbers in another unit than the file's
  canonical unit. Numbers are kept exactly as the source states them and never converted, so every
  bound stays literally checkable. Each citation with a `pmid` also carries a `title` (the PubMed
  title, used by the title-match check).
- Canonical units follow src/seed.rs for existing slugs. New slugs: lowercase-kebab.

## Verification

Deterministic checker: `/work/artifacts/tools/rt.py verify` (box artifact, not in the repo). Final
run over every file:

    73/73 files OK; citations checked 560, ok 560, failed 0

What it checks for each citation:
- Every PMID resolves via E-utilities esummary.
- Its `title` matches the PubMed title.
- Any `doi` equals PubMed's DOI for that PMID.
- Every DOI resolves via https://doi.org/api/handles/ (111 distinct DOIs).
- Every `quote` is found verbatim, after whitespace and case normalisation, in text retrieved
  during the run: the PubMed abstract, the PMC/Europe PMC open-access full text, or the guideline
  page or PDF.

For each range, `low`, `high` and any `when.markers` value must appear as a number in one of that
range's quotes.

The checker also enforces the schema and the allowed values. NCBI calls went through a cross-process
throttle (at most 2.5 requests/s).

Failures during authoring were fixed or the range was dropped by the authoring agent before
the file was written; no failing citation remains. The lead then audited every range by hand for
semantic correctness (citations can verify while the coding is wrong) and fixed:
- ALT healthy limits that had been coded as `warn` (now `reference`).
- An AMORIS band bound borrowed from the neighbouring band.
- The FIB-4 age >=65 cut-off: 2.0 is the low-risk rule-out threshold, not a rule-in.
- Infant bands that had collapsed onto `age_max = 1`.
- Osmolal gap split out of `osmolality` into `osmolal-gap`.
- Single means and IQRs that had been encoded as reference intervals (removed).

Dropped as too weak:
- Pregnancy trace-element "optimal" levels pooled across specimen types.
- A serum osmolality range quoted from a case report.

URL-only guideline sources (no PMID text):
- KDIGO 2024 CKD guideline PDF.
- NCEP ATP III executive summary PDF.
- NHLBI 2011 pediatric lipid guideline PDF.
- NICE NG238 and NG3 recommendation pages.

## Caveats a reviewer should know

- **Paywalled guidelines.** ADA Standards of Care, the 2018 and 2026 ACC/AHA guidelines, ESC/EAS 2019,
  AASLD/EASL and the ESPEN guidelines are paywalled or bot-blocked (OUP, AHA, diabetesjournals and
  Elsevier return 403, and the PMC HTML is behind a captcha). Where their numbers appear, they are
  cited through open-access papers that restate them, and the note says so. Examples: the ESC/EAS
  LDL-C and apoB goals, AHA/ACC risk enhancers, and the ADA prediabetes and diabetes cut-offs.
- **Adult NORIP and CALIPER tables were not retrievable** (abstracts have no numbers). Adult reference
  intervals therefore rely mainly on:
  - the Egyptian IFCC C-RIDL multicentre study (PMID 33740794, harmonised protocol);
  - the Finnish Health 2000 liver study (PMID 41255223);
  - FINRISK 2007 (apolipoproteins);
  - a Nepalese dry-chemistry study;
  - indirect (reflimR/refineR) studies.

  Pediatric intervals come mainly from a 2026 Chinese study of 41 analytes (PMID 42179679). Every
  range names its population; the merge step may prefer to show sex/age-matched ranges, not all.
- Several outcome-based "optimal" bands come from single cohorts. Examples: sodium >=139, potassium
  4.0-5.0 in CKD, albumin >=4.6 in transplant recipients, and UACR <10 mg/g. Their notes say so.
- Some files carry ranges from several sources for the same population; they are kept side by side
  with their populations rather than merged. Examples: ALT (ACG vs Prati vs Finnish limits) and Lp(a)
  (EAS vs NLA vs 2026 ACC/AHA).

## Markers covered

Markers: 73; ranges: 550; distinct sources: 113 PMIDs, 0 DOI-only, 5 URL-only

| category | slug | name | unit | ranges | set/kind | conditions used |
|---|---|---|---|---|---|---|
| chemistry | amylase | Amylase | U/L | 2 | reference/referencex2 | age_max, age_min, sex |
| chemistry | beta-hydroxybutyrate | Beta-Hydroxybutyrate (Ketones, Blood) | mmol/L | 4 | clinical/diagnosticx1, clinical/targetx1, clinical/warnx2 | conditions |
| chemistry | ck | Creatine Kinase | U/L | 12 | reference/referencex12 | age_max, age_min, ethnicity, sex |
| chemistry | ldh | Lactate Dehydrogenase | U/L | 5 | reference/referencex5 | age_max, age_min |
| chemistry | lipase | Lipase | U/L | 1 | reference/referencex1 | - |
| chemistry | uric-acid | Uric Acid | mg/dL | 11 | clinical/diagnosticx1, clinical/targetx3, reference/referencex7 | age_max, age_min, conditions, sex, unit:µmol/L |
| electrolyte | anion-gap | Anion Gap | mmol/L | 3 | reference/referencex3 | method, specimen |
| electrolyte | chloride | Chloride | mmol/L | 7 | clinical/warnx1, optimal/optimalx1, reference/referencex5 | age_max, age_min, sex |
| electrolyte | co2 | Carbon Dioxide (Bicarbonate) | mmol/L | 7 | clinical/warnx2, optimal/optimalx1, reference/referencex4 | age_max, age_min, conditions, specimen |
| electrolyte | osmolal-gap | Osmolal Gap | mOsm/kg | 2 | clinical/diagnosticx1, reference/referencex1 | - |
| electrolyte | osmolality | Osmolality, Serum | mOsm/kg | 2 | clinical/diagnosticx1, clinical/warnx1 | age_min |
| electrolyte | potassium | Potassium | mmol/L | 16 | clinical/criticalx1, clinical/diagnosticx3, clinical/warnx2, optimal/optimalx2, reference/referencex8 | age_max, age_min, conditions, sex, specimen |
| electrolyte | sodium | Sodium | mmol/L | 13 | clinical/diagnosticx5, optimal/optimalx1, reference/referencex7 | age_max, age_min, sex, specimen |
| glycemic | 1-5-anhydroglucitol | 1,5-Anhydroglucitol (GlycoMark) | µg/mL | 4 | clinical/warnx1, reference/referencex3 | sex |
| glycemic | c-peptide | C-Peptide | ng/mL | 8 | clinical/diagnosticx8 | conditions, fasting, unit:nmol/L |
| glycemic | cgm-glucose-cv | CGM glucose coefficient of variation | % | 2 | clinical/targetx1, optimal/optimalx1 | conditions |
| glycemic | cgm-mean-glucose | CGM Mean Glucose (24-hour) | mg/dL | 1 | reference/referencex1 | - |
| glycemic | cgm-time-above-180 | CGM time above range (>180 mg/dL) | % | 2 | clinical/targetx2 | conditions, pregnant |
| glycemic | cgm-time-above-250 | CGM time well above range (>250 mg/dL) | % | 2 | clinical/targetx2 | conditions, risk |
| glycemic | cgm-time-below-54 | CGM time below range (<54 mg/dL) | % | 2 | clinical/targetx2 | conditions, pregnant |
| glycemic | cgm-time-below-70 | CGM time below range (<70 mg/dL) | % | 3 | clinical/targetx3 | conditions, pregnant, risk |
| glycemic | cgm-time-in-range | CGM time in range (70-180 mg/dL) | % | 3 | clinical/targetx3 | conditions, pregnant, risk |
| glycemic | cgm-time-in-tight-range | CGM Time in Tight Range (70-140 mg/dL) | % | 1 | clinical/targetx1 | conditions |
| glycemic | fructosamine | Fructosamine | µmol/L | 3 | clinical/diagnosticx2, reference/referencex1 | - |
| glycemic | glucose | Glucose | mg/dL | 19 | clinical/criticalx1, clinical/diagnosticx4, clinical/referencex1, clinical/targetx3, clinical/warnx3, optimal/optimalx1, reference/referencex6 | age_max, age_min, conditions, fasting, method, pregnant, sex, unit:mmol/L |
| glycemic | glucose-ogtt-1h | Glucose, 1-hour OGTT (75 g) | mg/dL | 3 | clinical/diagnosticx2, clinical/warnx1 | pregnant |
| glycemic | glucose-ogtt-2h | Glucose, 2-hour OGTT (75 g) | mg/dL | 4 | clinical/diagnosticx2, clinical/referencex1, clinical/warnx1 | pregnant |
| glycemic | glycated-albumin | Glycated Albumin | % | 9 | clinical/diagnosticx4, clinical/warnx2, reference/referencex3 | pregnant, sex |
| glycemic | hba1c | Hemoglobin A1c | % | 12 | clinical/diagnosticx2, clinical/referencex1, clinical/targetx4, clinical/warnx2, optimal/optimalx1, reference/referencex2 | age_max, age_min, conditions, pregnant, unit:mmol/mol |
| glycemic | homa-ir | HOMA-IR | ratio | 13 | clinical/warnx10, reference/referencex3 | age_max, age_min, conditions, fasting, menopause, method, sex |
| glycemic | insulin | Insulin (fasting) | µIU/mL | 3 | reference/referencex3 | age_max, age_min, fasting, sex |
| kidney | bun | Blood Urea Nitrogen | mg/dL | 20 | reference/referencex20 | age_max, age_min, sex, unit:mmol/L |
| kidney | bun-creatinine-ratio | BUN/Creatinine Ratio | ratio | 1 | clinical/diagnosticx1 | - |
| kidney | creatinine | Creatinine | mg/dL | 16 | reference/referencex16 | age_max, age_min, pregnant, sex, unit:µmol/L |
| kidney | cystatin-c | Cystatin C | mg/L | 18 | reference/referencex18 | age_max, age_min, sex |
| kidney | egfr | eGFR | mL/min/1.73m² | 11 | clinical/diagnosticx4, clinical/referencex1, clinical/warnx2, reference/referencex4 | age_max, age_min, method, sex |
| kidney | uacr | Urine Albumin/Creatinine Ratio | mg/g | 9 | clinical/diagnosticx4, clinical/referencex2, clinical/warnx2, optimal/optimalx1 | sex, specimen, unit:mg/mmol |
| kidney | upcr | Urine Protein/Creatinine Ratio | mg/g | 7 | clinical/diagnosticx3, clinical/referencex4 | age_max, age_min, pregnant, specimen, unit:mg/mg, unit:mg/mmol |
| lipid | apoa1 | Apolipoprotein A-I | mg/dL | 3 | clinical/warnx1, reference/referencex2 | sex, unit:g/L |
| lipid | apob | Apolipoprotein B | mg/dL | 9 | clinical/targetx4, clinical/warnx3, reference/referencex2 | risk, sex, unit:g/L |
| lipid | apob-apoa1-ratio | ApoB/ApoA-I Ratio | ratio | 4 | clinical/warnx2, reference/referencex2 | sex |
| lipid | hdl-c | HDL Cholesterol | mg/dL | 20 | clinical/diagnosticx5, clinical/optimalx1, clinical/referencex1, clinical/warnx5, reference/referencex8 | age_max, age_min, fasting, pregnant, sex, trimester, unit:mmol/L |
| lipid | ldl-c | LDL Cholesterol | mg/dL | 25 | clinical/diagnosticx3, clinical/optimalx1, clinical/referencex1, clinical/targetx8, clinical/warnx4, reference/referencex8 | age_max, age_min, conditions, fasting, pregnant, risk, trimester, unit:mmol/L |
| lipid | ldl-p | LDL Particle Number (LDL-P) | nmol/L | 1 | clinical/warnx1 | method |
| lipid | lpa | Lipoprotein(a) | nmol/L | 7 | clinical/diagnosticx1, clinical/referencex1, clinical/warnx5 | unit:mg/dL |
| lipid | non-hdl-c | Non-HDL Cholesterol | mg/dL | 12 | clinical/diagnosticx1, clinical/referencex1, clinical/targetx4, clinical/warnx3, reference/referencex3 | age_max, age_min, conditions, fasting, risk, sex, unit:mmol/L |
| lipid | remnant-cholesterol | Remnant Cholesterol | mg/dL | 3 | clinical/warnx2, optimal/optimalx1 | fasting, unit:mmol/L |
| lipid | sdldl-c | Small Dense LDL Cholesterol | mg/dL | 1 | optimal/optimalx1 | - |
| lipid | tc-hdl-ratio | Total Cholesterol/HDL Ratio | ratio | 4 | clinical/warnx4 | sex |
| lipid | tg-hdl-ratio | Triglyceride/HDL-C Ratio | ratio | 2 | clinical/warnx2 | age_max, age_min, conditions, method |
| lipid | total-cholesterol | Total Cholesterol | mg/dL | 14 | clinical/diagnosticx2, clinical/referencex2, clinical/warnx3, reference/referencex7 | age_max, age_min, fasting, pregnant, trimester, unit:mmol/L |
| lipid | triglycerides | Triglycerides | mg/dL | 23 | clinical/criticalx1, clinical/diagnosticx5, clinical/referencex3, clinical/warnx6, reference/referencex8 | age_max, age_min, fasting, pregnant, sex, trimester, unit:mmol/L |
| lipid | vldl-c | VLDL Cholesterol | mg/dL | 1 | clinical/referencex1 | - |
| liver | albumin | Albumin | g/dL | 6 | optimal/optimalx1, reference/referencex5 | age_max, age_min, conditions, sex, unit:g/L |
| liver | albumin-globulin-ratio | Albumin/Globulin Ratio | ratio | 5 | clinical/warnx2, reference/referencex3 | age_min, conditions, sex |
| liver | alp | Alkaline Phosphatase | U/L | 12 | reference/referencex12 | age_max, age_min, sex |
| liver | alt | ALT (Alanine Aminotransferase) | U/L | 15 | clinical/referencex2, reference/referencex13 | age_max, age_min, sex, unit:IU/L |
| liver | ast | AST (Aspartate Aminotransferase) | U/L | 10 | reference/referencex10 | age_max, age_min, sex |
| liver | bilirubin-direct | Bilirubin, Direct (Conjugated) | mg/dL | 13 | clinical/diagnosticx2, reference/referencex11 | age_max, age_min, sex, unit:µmol/L |
| liver | bilirubin-total | Bilirubin, Total | mg/dL | 14 | clinical/diagnosticx1, reference/referencex13 | age_max, age_min, conditions, sex, unit:µmol/L |
| liver | fib-4 | FIB-4 Index | ratio | 4 | clinical/diagnosticx1, clinical/referencex2, clinical/warnx1 | age_min |
| liver | ggt | Gamma-Glutamyl Transferase | U/L | 7 | reference/referencex7 | age_max, age_min, sex |
| liver | globulin | Globulin (calculated) | g/dL | 5 | clinical/warnx1, reference/referencex4 | age_max, age_min, conditions, sex |
| liver | total-protein | Total Protein | g/dL | 8 | reference/referencex8 | age_max, age_min, sex, unit:g/L |
| mineral | calcium | Calcium | mg/dL | 10 | clinical/diagnosticx3, reference/referencex7 | age_max, age_min, method, unit:mmol/L |
| mineral | ceruloplasmin | Ceruloplasmin | mg/dL | 5 | clinical/diagnosticx3, clinical/warnx1, reference/referencex1 | conditions, unit:g/L |
| mineral | copper | Copper, Serum | µg/dL | 1 | reference/referencex1 | - |
| mineral | ionized-calcium | Calcium, Ionized | mmol/L | 4 | reference/referencex4 | age_max, age_min, sex, specimen |
| mineral | magnesium | Magnesium | mg/dL | 16 | clinical/warnx1, optimal/optimalx1, reference/referencex14 | age_max, age_min, conditions, method, sex, unit:mmol/L |
| mineral | phosphorus | Phosphorus (Phosphate), Inorganic | mg/dL | 6 | reference/referencex6 | age_max, age_min, sex, unit:mmol/L |
| mineral | selenium | Selenium, Serum/Plasma | µg/L | 2 | clinical/warnx1, reference/referencex1 | - |
| mineral | urine-iodine | Iodine, Urine (median, population) | µg/L | 12 | clinical/diagnosticx6, clinical/warnx2, reference/referencex4 | age_max, postpartum, pregnant |
| mineral | zinc | Zinc, Serum | µg/dL | 5 | clinical/diagnosticx5 | age_max, age_min, fasting, sex |

Condition dimensions (ranges using each `when` key): `age_min` 276, `age_max` 218, `sex` 171, `conditions` 59, `fasting` 49, `specimen` 27, `pregnant` 27, `method` 17, `risk` 17, `trimester` 8, `ethnicity` 6, `menopause` 2, `postpartum` 1

Sets: reference 299, clinical 238, optimal 13; kinds: reference 323, diagnostic 86, warn 82, target 41, optimal 15, critical 3; evidence: reference-interval-study 310, guideline 107, consensus 87, outcome-study 35, textbook 9, meta-analysis 2

## Markers deliberately skipped or left thin

| marker | reason |
|---|---|
| rbc-magnesium | No peer-reviewed erythrocyte-magnesium reference interval with numbers in retrievable text; mostly a consumer test. |
| gmi (Glucose Management Indicator) | A formula on CGM mean glucose (Bergenstal 2018); no consensus target range exists to cite. |
| HDL-P, LDL particle size, apoC-III, Lp-PLA2 | Research-grade or limited clinical use; no guideline or reference values. LDL-P has only the MESA-cited 1300 nmol/L NMR cut-point. |
| ammonia, lactate, 5'-nucleotidase | Acute-care or niche tests that people do not track longitudinally. |
| estimated average glucose (eAG) | Derived from HbA1c by formula, not measured. |
| serum osmolality reference interval (275-295) | No authoritative retrievable source; the osmolality file holds only dehydration thresholds for older adults. |
| adult PCR categories (150/500 mg/g), ISSHP 30 mg/mmol primary text, ADA HbA1c pregnancy (<6%) and pediatric (<7%) targets, ESC and ADA primary text | Not in any retrievable open-access text. Restated versions are used where available. |
| iron/ferritin/TIBC, hs-CRP, homocysteine, vitamins, hormones, CBC | Outside this shard (other boxes). |

## Tools and artifacts (outside the repo, /work/artifacts)

- `tools/rt.py`: search, abstract, full-text, URL fetch, grep and the verifier.
- `tools/FORMAT.md`: the format spec given to agents.
- `prompts/`: the agent instructions.
- `cache/`: retrieved abstracts, full texts and guideline PDFs, which make the quotes reproducible.
