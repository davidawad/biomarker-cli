# Research shard r4: report

Bead: biomarker-cli-8h7.3.4 (parent biomarker-cli-8h7.3).
Scope: inflammation and immune, cardiac, tumour markers, autoimmune antibodies, infectious serology,
urinalysis, stool markers, body composition, vitals and wearable-derived measures, derived indices,
aging clocks and composite scores.

## Summary

```
markers: 99; ranges: 449
citation entries: 479; verified: 479; failed: 0
distinct PMIDs: 187; distinct DOIs: 183; url-only sources: 11
kinds: decision=246, reference=134, warn=34, optimal=27, critical=8
evidence: reference-interval-study=114, consensus=102, guideline=94, cohort-study=58, validation-study=47, meta-analysis=23, textbook=6, outcome-study=5
when dimensions (ranges using each): sex=109, age_min=87, age_max=71, setting=55, method=44, conditions=23, ethnicity=16, pregnancy=16, trimester=10, fasting=10, markers=6, menopause=6
```

Every one of the 479 citation entries in the committed files passed the deterministic verifier
(0 failures in the final tree). Citations that failed while files were being drafted were fixed or
removed before commit. See "Verification" below.

## Format used

The epic (biomarker-cli-8h7) was not in this box's bead graph: only biomarker-cli-8h7.3 and .3.4
were imported, so `br show biomarker-cli-8h7` was unavailable. The format below follows the field
list in the bead text (`set, kind, low, high, when, population, evidence, citations=[{pmid, doi, url,
locator, quote}], note`), and the condition keys mirror the existing `ranges.rs` and `profiles.rs`
semantics. The merge step should map it onto the epic's exact schema if the two differ.

Top level: `slug, name, loinc (only when known), category, unit (canonical), description, aliases (optional)`.

`[[range]]`:
- `set`: kebab id of the source the range comes from (e.g. `kdigo-2024-ckd`, `eular-acr-2019-sle`).
- `kind`: one of the following.
  - `reference`: population reference interval.
  - `optimal`: target justified by outcomes or guidelines.
  - `warn`: borderline band.
  - `critical`: urgent-action band.
  - `decision`: guideline or diagnostic cut-off or risk category.

  For `decision`, `warn` and `critical`, `[low, high]` is the band that the label in `note`
  applies to. Open sides are omitted.
- `low`, `high`: numbers. Whether a bound is strict (`<` vs `<=`) is recorded in `note`.
- `unit`: optional per-range override, used when the source states numbers in another unit
  (e.g. `mg/mmol` vs `mg/g`, `METs`, `x ULN`). This matches the `unit` field the app's `RangeSpec` already accepts.
- `when`: inline table of conditions. Keys:
  - `sex`: `male` or `female`.
  - `age_min`: inclusive. `age_max`: exclusive, as in `ranges.rs`.
  - `pregnancy`, `trimester`.
  - `menopause`: `pre`, `peri` or `post`.
  - `cycle_phase`, `fasting`, `ethnicity`.
  - `conditions`: list.
  - `markers`: other-marker thresholds, as a table of slug to comparator string.
  - `setting`: e.g. `suspected-acute-mi`, `acute-dyspnoea`, `ambulatory`.
  - `method`: assay or device.
- `population`, `evidence`. Evidence is one of `guideline`, `consensus`, `reference-interval-study`,
  `outcome-study`, `meta-analysis`, `cohort-study`, `validation-study`, `regulatory`, `textbook`.
- `note`.
- `citations = [{ pmid, doi, title, url, locator, quote }]`. `title` is an addition: it is the
  PubMed title, so the title-match check can be re-run deterministically.

## Verification

Run by the lead, deterministically, over every file before each commit. The tool is
`/work/artifacts/r4tools/verify.py`; it is not committed, because only `catalog/research/` was in scope.

1. Each PMID resolves via E-utilities esummary, and `title` matches the PubMed title after normalisation.
2. If both are given, the DOI equals PubMed's DOI for that PMID. Every DOI resolves via
   `https://doi.org/api/handles/<doi>` (responseCode 1).
3. `quote` appears verbatim (after whitespace, case and symbol normalisation) in text that was actually
   retrieved: the PubMed abstract (efetch), the PMC open-access full text (NCBI efetch db=pmc, falling back
   to Europe PMC fullTextXML), or the cited `url` (HTML or PDF via pdftotext) for url-only guideline citations.
4. Every `low`/`high` number appears literally in one of that range's quotes, compared by absolute value.
   The same applies to `when.age_min`, `when.age_max` (quote value or value + 1, because `age_max` is
   exclusive) and the numbers in `when.markers`.
5. Schema checks: allowed keys, kinds, evidence values and `when` keys; `low <= high`; slug equals filename.

NCBI traffic was throttled globally to at most 2.5 req/s through a file lock shared by all subagents,
and all responses were cached.

Final run: 99/99 files PASS, 479/479 citation entries verified, 0 failed.

Failures found during drafting, all fixed or dropped before commit:
- Unknown `when` keys (`method` at the range level).
- Negative cut-offs: BMD T-score and NAFLD fibrosis score failed until the number check compared absolute values.
- Citations whose quote text was not in any retrievable text. The subagents dropped these rather than
  paraphrase; examples are listed under "Gaps".
- One wrong PMID (29073398, cited for the NSF 2017 sleep-quality report) resolved to a different paper. It was caught and not used.

Lead review after verification:
- Removed `biological-age`. Its only candidate range was a ±1 SD band of a standardised PhenoAgeAccel
  distribution with no stated unit, which would mislead if read as years.
- Relabelled the HRV SDNN > 100 ms post-MI comparator from `optimal` to `decision`.
- Normalised the bmi unit to the seed's `kg/m²`.

## Markers covered (99)

| slug | category | unit | ranges | kinds | condition dimensions | verify |
|---|---|---|---|---|---|---|
| afp | tumour | ng/mL | 13 | decision, reference, warn | age_max, age_min, setting, sex | PASS |
| ana-titer | autoimmune | titer (reciprocal of dilution) | 2 | decision | - | PASS |
| ankle-brachial-index | vitals | ratio | 4 | decision, reference, warn | - | PASS |
| anti-beta2-glycoprotein-igg | autoimmune | U/mL | 2 | decision | - | PASS |
| anti-ccp | autoimmune | U/mL | 2 | decision | method | PASS |
| anti-dsdna | autoimmune | IU/mL | 2 | decision, warn | - | PASS |
| anti-hbs | infectious | mIU/mL | 1 | decision | - | PASS |
| anticardiolipin-igg | autoimmune | GPL/mL | 3 | decision | - | PASS |
| apnea-hypopnea-index | wearable | events/h | 4 | decision, reference | - | PASS |
| appendicular-lean-mass-index | body | kg/m2 | 6 | decision | method, sex | PASS |
| apri | derived | index | 2 | decision | - | PASS |
| ascvd-10y-risk | composite | % | 3 | decision, warn | age_max, age_min | PASS |
| atherogenic-index-of-plasma | derived | index | 3 | decision | - | PASS |
| beta-2-microglobulin | tumour | mg/L | 4 | decision, reference | age_max, age_min, markers | PASS |
| bmd-t-score | body | SD | 3 | decision | age_min, menopause, sex | PASS |
| bmi | body | kg/m² | 15 | decision, optimal, reference, warn | ethnicity | PASS |
| bnp | cardiac | pg/mL | 6 | decision | conditions, setting | PASS |
| body-fat | body | % | 2 | decision | method, sex | PASS |
| body-temperature | vitals | °C | 4 | critical, decision, reference | age_max | PASS |
| bp-diastolic | vitals | mmHg | 4 | decision, optimal | - | PASS |
| bp-systolic | vitals | mmHg | 5 | decision, optimal | - | PASS |
| bun-creatinine-ratio | derived | ratio | 1 | decision | - | PASS |
| c3 | immune | g/L | 4 | reference | age_max, age_min, sex | PASS |
| c4 | immune | g/L | 4 | reference | age_max, age_min, sex | PASS |
| ca-125 | tumour | U/mL | 12 | decision, reference | age_max, age_min, menopause, setting, sex | PASS |
| ca-15-3 | tumour | U/mL | 2 | decision, reference | age_max, age_min, sex | PASS |
| ca-19-9 | tumour | U/mL | 5 | decision, reference | age_max, age_min, sex | PASS |
| cd4-count | infectious | cells/µL | 5 | decision, reference | conditions | PASS |
| cea | tumour | ng/mL | 7 | decision, reference, warn | age_max, age_min, sex | PASS |
| cgm-coefficient-of-variation | wearable | % | 3 | optimal, warn | - | PASS |
| cgm-time-in-range | wearable | % | 7 | optimal | conditions, pregnancy | PASS |
| ch50 | immune | U/mL | 1 | reference | method | PASS |
| ck | cardiac | U/L | 11 | critical, decision, reference, warn | age_max, age_min, conditions, ethnicity, sex | PASS |
| coronary-artery-calcium | composite | Agatston units | 4 | decision, warn | age_min | PASS |
| crp-standard | inflammation | mg/L | 6 | decision, reference | age_min, pregnancy, setting | PASS |
| daily-steps | wearable | steps/day | 3 | optimal | age_max, age_min | PASS |
| dunedinpace | aging | years/year | 1 | reference | - | PASS |
| egfr | derived | mL/min/1.73m² | 8 | critical, decision, warn | conditions | PASS |
| esr | inflammation | mm/h | 10 | reference | age_max, age_min, method, sex | PASS |
| fecal-calprotectin | stool | µg/g | 5 | decision, reference | age_min | PASS |
| fecal-elastase | stool | µg/g | 3 | critical, decision, warn | - | PASS |
| fib-4 | derived | index | 7 | decision, warn | age_min | PASS |
| fibrinogen | inflammation | mg/dL | 5 | reference, warn | pregnancy, trimester | PASS |
| fit-hemoglobin | stool | µg Hb/g | 4 | decision, warn | method | PASS |
| free-light-chain-ratio | immune | ratio | 4 | decision, reference | conditions, ethnicity | PASS |
| gait-speed | body | m/s | 2 | decision | - | PASS |
| glucose-management-indicator | wearable | % | 1 | optimal | - | PASS |
| grimage-acceleration | aging | years | 1 | warn | - | PASS |
| grip-strength | body | kg | 6 | decision, warn | sex | PASS |
| hbv-dna | infectious | IU/mL | 3 | decision | pregnancy, trimester | PASS |
| hcv-rna | infectious | IU/mL | 1 | decision | - | PASS |
| hiv-viral-load | infectious | copies/mL | 3 | decision, warn | conditions | PASS |
| homa-ir | derived | index | 9 | decision, reference | fasting, menopause, method, sex | PASS |
| homocysteine | inflammation | µmol/L | 7 | decision, reference | pregnancy, sex, trimester | PASS |
| hrv | vitals | ms | 5 | decision, reference, warn | method, setting | PASS |
| hs-troponin-i | cardiac | ng/L | 8 | decision, reference | method, setting, sex | PASS |
| hs-troponin-t | cardiac | ng/L | 7 | decision, reference | method, setting, sex | PASS |
| hscrp | inflammation | mg/L | 8 | decision, reference, warn | - | PASS |
| iga | immune | g/L | 5 | decision, reference | age_max, age_min | PASS |
| ige-total | immune | kU/L | 3 | decision, reference | - | PASS |
| igg | immune | g/L | 6 | decision, reference, warn | age_max, age_min, conditions | PASS |
| igm | immune | g/L | 8 | reference | age_max, age_min, sex | PASS |
| il-6 | inflammation | pg/mL | 6 | reference, warn | pregnancy, setting, trimester | PASS |
| kappa-free-light-chain | immune | mg/L | 2 | reference | ethnicity | PASS |
| lambda-free-light-chain | immune | mg/L | 2 | reference | ethnicity | PASS |
| ldh | tumour | U/L | 3 | reference | age_max, age_min, method | PASS |
| lifes-essential-8 | composite | score | 3 | decision, warn | - | PASS |
| lp-pla2 | cardiac | nmol/min/mL | 3 | decision, reference | method, sex | PASS |
| measles-igg | infectious | mIU/mL | 4 | decision, warn | method | PASS |
| nafld-fibrosis-score | derived | score | 4 | decision, warn | age_min | PASS |
| nlr | derived | ratio | 1 | reference | - | PASS |
| nt-probnp | cardiac | pg/mL | 10 | decision, reference | age_max, age_min, conditions, markers, setting, sex | PASS |
| prevent-10y-risk | composite | % | 4 | decision, warn | - | PASS |
| procalcitonin | inflammation | ng/mL | 8 | decision | setting | PASS |
| psa-free-percent | tumour | % | 1 | decision | markers | PASS |
| psa | tumour | ng/mL | 12 | decision, reference, warn | age_max, age_min, setting, sex | PASS |
| respiratory-rate | vitals | breaths/min | 6 | critical, decision, reference, warn | - | PASS |
| resting-hr | vitals | bpm | 1 | warn | - | PASS |
| rheumatoid-factor | autoimmune | IU/mL | 2 | decision | method | PASS |
| rubella-igg | infectious | IU/mL | 1 | decision | - | PASS |
| sleep-duration | wearable | h | 2 | optimal | - | PASS |
| sleep-efficiency | wearable | % | 3 | decision, warn | conditions, method | PASS |
| spo2 | vitals | % | 2 | optimal | conditions | PASS |
| tc-hdl-ratio | derived | ratio | 8 | decision, optimal | setting, sex | PASS |
| tetanus-antitoxin | infectious | IU/mL | 3 | decision | method | PASS |
| tg-hdl-ratio | derived | ratio | 3 | decision | ethnicity | PASS |
| ttg-iga | autoimmune | U/mL | 2 | decision | - | PASS |
| tyg-index | derived | index | 1 | decision | fasting | PASS |
| urine-acr | urinalysis | mg/g | 8 | decision | - | PASS |
| urine-osmolality | urinalysis | mOsm/kg | 3 | decision | setting | PASS |
| urine-pcr | urinalysis | mg/g | 7 | decision, reference | pregnancy, setting, sex | PASS |
| urine-ph | urinalysis | pH | 2 | optimal | - | PASS |
| urine-rbc | urinalysis | /HPF | 1 | decision | - | PASS |
| urine-specific-gravity | urinalysis | ratio | 2 | decision | - | PASS |
| urine-wbc | urinalysis | /HPF | 6 | decision | age_min, markers, sex | PASS |
| vo2max | vitals | mL/kg/min | 15 | decision, optimal, reference | age_max, age_min, sex | PASS |
| waist-circumference | body | cm | 8 | decision | ethnicity, sex | PASS |
| waist-to-height-ratio | body | ratio | 3 | decision, warn | - | PASS |
| waist-to-hip-ratio | body | ratio | 2 | decision | sex | PASS |

Existing seed slugs reused, from `src/seed.rs` MARKERS and VITALS_MARKERS: hscrp, homocysteine, psa, egfr,
bmi, body-fat, resting-hr, hrv, vo2max, bp-systolic and bp-diastolic. Their units match the seed. Two
categories differ from the seed: egfr is under `derived` (seed: `cmp`) and psa under `tumour` (seed: `hormone`).

## Caveats the merge step should know

- **Secondary citations.** Where a guideline's own text was not retrievable (paywalled, or no abstract), the
  range cites an open-access paper that states the guideline's numbers, and `note` says so. Affected: the
  CDC/AHA 2003 hs-CRP bands, the ESC NT-proBNP/BNP criteria (via a 2026 systematic review), the ACC/AHA 2017
  BP categories, the Sydney APS criteria, the ACR/EULAR 2010 RA serology thresholds, the WHO waist and WHR
  cut-offs, AFP 400/20 ng/mL, the PCRMP PSA threshold, and the tetanus and measles protective levels.
- **Table-row quotes.** For tc-hdl-ratio (Millán 2009 Table 1), respiratory-rate (RCP NEWS2 chart) and some
  manufacturer cut-offs, the quote is a verbatim table row. The sex, setting or band mapping comes from
  column headers that are given in `locator` and `note`.
- **Non-catalog units.** Some ranges use `unit = "x ULN"` (rheumatoid-factor, anti-ccp, ttg-iga, ck),
  `METs` (vo2max), `ng/mL/cm3` (PSA density) or `mg/mg` (urine-pcr). The engine needs conversions or
  multiple-of-ULN support for these. They were kept because the source states the threshold that way.
- **Assay-specific cut-offs** carry `when.method`: troponins, anti-dsDNA, RF and anti-CCP (Thermo Fisher),
  CH50, measles IgG and FIT.
- **LOINC codes** are as supplied in the task briefs for existing or common codes, and were not independently
  verified against the LOINC database. They are omitted where unsure (urine-pcr, measles-igg).
- Population-specific reference intervals (Iranian, Spanish, Korean, Chinese, Zimbabwean, Brazilian
  cohorts) are tagged in `population`. `ethnicity` is set only where a source compares ethnic groups.

## Markers deliberately skipped, and why

- **biological-age** (existing seed slug). Phenotypic Age has no published normal or abnormal threshold:
  sources give hazard ratios per year of acceleration, not ranges. The candidate SD band was unitless (see above).
- **pulse-wave-velocity**. The ESC/ESH > 10 m/s threshold was not in any retrievable text, and the Reference
  Values Collaboration age-band table could not be quoted with its row labels.
- **Qualitative serology and dipstick results**: HBsAg, anti-HCV, HIV Ag/Ab, H. pylori stool antigen or
  serology, syphilis screens, varicella, CMV and EBV serostatus, and urine glucose, ketones, nitrite and
  leukocyte esterase. Results are reactive or non-reactive with assay-specific S/CO indices, so there is no numeric range to cite.
- **ANCA (MPO/PR3)**, IgG subclasses and other cytokines (TNF-alpha, IL-1β, IL-10). Cut-offs are assay-specific
  or the assays are research-grade with no clinical reference standard.
- **Body weight** (existing seed slug). There is no population range; BMI covers it.
- **tpo-ab** (seed, thyroid category) is left to the thyroid shard. Ferritin, the CBC-derived WBC and the
  lymphocyte markers are left to their own shards.
- Separate slugs for standard CRP aliases: the seed maps the `crp` alias to hscrp, so standard-assay CRP got
  its own slug, `crp-standard`, rather than `crp`.

## Gaps inside covered markers (no retrievable text stated the number)

- resting-hr: conventional 60–100 and Spodick 50–90 bpm; ACC/AHA/HRS bradycardia thresholds.
- sleep-duration: AASM/SRS adult ≥ 7 h and AASM paediatric bands. The JCSM full texts were not retrievable.
- hs-troponin-i: Architect overall 99th percentile of 26 ng/L.
- nt-probnp and bnp: primary ESC 2021 HF and Universal Definition of HF texts.
- esr: the Miller age formula, which is not expressible as banded numeric ranges.
- crp-standard: NICE CG191/NG250 thresholds of < 20, 20–100 and > 100 mg/L.
- homa-ir: NHANES 2.5/2.73.
- urine-ph and urine-specific-gravity: general reference intervals.
- waist-circumference: Alberti 2009 harmonised per-ethnicity table.
- bmi: NICE ethnicity thresholds.
- fibrinogen: a cardiovascular cut-off.
- c3 and c4: numeric SLE low-complement thresholds.
- vo2max: FRIEND 2022.
- cd4-count: percentage thresholds.

## Method

The lead built the tooling and the format spec, then ran 23 Haiku subagents over three rounds:
- Round 1: 13 agents, one batch per category group.
- Round 2: 7 agents doing gap-fill and new markers, after adding PDF support so that KDIGO and NICE guideline PDFs could be quoted.
- Round 3: 3 agents covering the remaining vitals, cardiac and infectious gaps.

Each subagent used only the cached helper (`pm.py`: PubMed esearch/esummary/efetch, Europe PMC search and
full text, NCBI PMC efetch, and URL/PDF fetch). Each had to get a PASS from the verifier before reporting.
The lead re-verified everything, spot-reviewed every range's quote against its numbers and conditions, and
committed in batches. No personal health data was used, and all compute ran on this box.
