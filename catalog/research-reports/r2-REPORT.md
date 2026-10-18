# Research shard r2: report

Bead `biomarker-cli-8h7.3.2`. Scope: hematology/CBC (differential and indices), coagulation, iron studies,
vitamins, trace elements, toxic metals, omega-3 index and fatty acids, amino and organic acids in common use,
and homocysteine/methylation. Output is one `catalog/research/<slug>.toml` per marker. Nothing outside
`catalog/research/` was changed.

## Summary

- **92 markers** covered, **370 ranges**, **381 citation entries** (128 distinct PMIDs, 119 distinct DOIs,
  plus 3 URL-only guideline citations: the IOM 2011 vitamin D report on nap.edu, CDC lead clinical guidance, and the Labcorp beta-carotene page).
- Deterministic verification on the final tree: **92/92 files OK; 381/381 citations passed, 0 failed.**
- Every range value is present in the quoted source text, either verbatim or after the declared `source_factor` unit conversion.

## Format notes (read before merging)

The epic `biomarker-cli-8h7` was **not in the bead graph shipped to this box** (`br show` only knew
`8h7.3` and `8h7.3.2`), so its "Ranges with structured conditions and mandatory citations" section could not be
read. The files follow the field list given in the bead itself:
`slug, name, loinc, category, unit, description` plus `[[range]]` with
`set, kind, low, high, when, population, evidence, citations = [{pmid, doi, url, locator, quote}], note`.
The merge step should map the following choices onto the epic's exact format:

- **Extra citation field `title`**: the PubMed title. It is used to check that the PMID's esummary title matches.
- **Extra range fields `source_unit` / `source_factor`**: present only when the quote's unit differs from the
  canonical `unit`. The rule is canonical value = quoted value × `source_factor`. Bounds are always in the canonical unit.
- **`set`**: a short kebab id of the authority or study the range comes from (`norip`, `who`, `iom-2011`, `easl-2022`, ...).
- **`kind`** is one of `reference optimal therapeutic warn deficient insufficient excess toxic critical diagnostic`.
  For `reference`, `optimal` and `therapeutic`, `low..high` is the normal or target interval. For every other kind,
  `low..high` is the **zone where that label applies**. For example, "anaemia if Hb < 13 g/dL" is `diagnostic, high = 13`,
  and "toxic above 150" is `toxic, low = 150`. Note that `warn` here means a borderline or elevated-risk zone. It is
  not the app's current near-limit margin semantics.
- **`evidence`** is one of `guideline consensus reference-interval outcome review textbook`.
- **`when` keys**: `sex` (male/female), `age_min`/`age_max` (years, min inclusive, max exclusive), `pregnant`,
  `trimester`, `gestation_week_min`/`gestation_week_max`, `postpartum`, `lactating`, `menopause` (pre/post),
  `cycle_phase`, `fasting`, `ethnicity`, `condition` (array of kebab strings, e.g. `ckd`, `on-warfarin`,
  `hfe-c282y-homozygous`), `specimen`, `smoking`, `altitude_m_min`/`altitude_m_max`, `method`, `region`, and
  `markers`. `markers` holds other-marker thresholds: an array of `{marker, low, high}`, e.g. TSAT < 20 is
  `{ marker = "transferrin-saturation", high = 20 }`.
- Existing slugs, units, categories and LOINCs from `src/seed.rs` were kept for the 19 seeded markers in this shard.
  New slugs are lowercase-kebab. Where the LOINC was uncertain, it was omitted rather than guessed.

## Verification method

Verification used `lit.py verify` (kept outside the repo at `/work/tools/lit.py`; the full output is in
`/work/artifacts/verify.txt` and `verify.json`). For every file it checks the following:

1. The TOML parses, the slug matches the file name, required fields are present, and `kind`, `evidence` and `when` keys are valid.
2. **PMID**: NCBI E-utilities `esummary` resolves it, and its title matches the citation `title` (similarity ≥ 0.9).
   If PubMed records a DOI, it must equal the cited DOI.
3. **DOI**: `https://doi.org/api/handles/<doi>` returns `responseCode` 1.
4. **Quote**: after normalisation (case, whitespace, dashes, interpunct decimals), the quote is a verbatim substring of
   the retrieved source text: the PubMed abstract (efetch), the Europe PMC open-access full text, or the fetched URL text.
5. **Numbers**: every `low`/`high` appears in a quote of that range's citations, after `source_factor`
   (1% tolerance for converted values, exact otherwise).
6. Warnings only: `age_min`/`age_max` not found in the quotes (129). These are mostly exclusive upper bounds
   (a quoted band "20–29 y" is stored as `age_max = 30`) and default adult bounds.

NCBI was rate-limited to ≤2.5 req/s through a shared lock across all agents, and every response was cached.

Corrections the lead made after the agents' self-verification:
- Fixed inverted zone directions: the WHO anaemia cutoffs, the PV hematocrit criteria, the MMA > 350 nmol/L criterion,
  and the MSUD leucine dialysis threshold.
- Moved the aPTT "× control" ratios from `aptt` (seconds) to a new `aptt-ratio` marker.
- Reclassified some ranges: D-dimer exclusion cutoffs became reference upper limits; G6PD drug-eligibility thresholds and neutropenia
  became diagnostic; Sepsis-3 lactate became diagnostic; aluminium "< 20 µg/L unlikely toxic" became a reference limit rather
  than a toxic zone.
- Removed a misread vitamin B6 "20–30 nmol/L marginal" band.
- Replaced the fragmentary Henan coagulation table quotes with full table rows, and added gestational-week conditions.
- Regenerated all age/sex bands for the reticulocyte indices from the source table (the agent had taken only 2 of 6 bands).
- Added RDW (Korea CBC table), the Endocrine Society 2011 vitamin D bands (via an OA review by the guideline chair),
  and the WHO-commissioned 2024 fifth-centile haemoglobin thresholds (including the first trimester).
- Spot-checked table-derived values against the source tables: Korea CBC, Henan coagulation, China NHS reticulocytes,
  NZ fatty acids, Russian iron panel, and the Time/race-specific WBC pool.

## Markers covered

| slug | name | category | unit | ranges | kinds | condition keys | verify |
|---|---|---|---|---|---|---|---|
| `aluminum-serum` | Aluminum (serum) | toxic-metal | µg/L | 2 | reference, toxic | condition | OK |
| `anti-xa` | Anti-Xa activity (heparin) | coagulation | IU/mL | 2 | therapeutic | condition, pregnant | OK |
| `antithrombin` | Antithrombin activity | coagulation | % | 6 | reference | age_max, age_min, gestation_week_max, gestation_week_min, pregnant, sex, trimester | OK |
| `aptt-ratio` | aPTT Ratio | coagulation | ratio | 2 | therapeutic | condition | OK |
| `aptt` | Activated partial thromboplastin time | coagulation | s | 5 | reference | age_max, age_min, pregnant, region, sex | OK |
| `arachidonic-acid` | Arachidonic acid (AA) | fatty-acid | % | 1 | reference | specimen | OK |
| `arginine` | Arginine | amino-acid | µmol/L | 1 | reference | method, specimen | OK |
| `basophils-abs` | Absolute basophil count | cbc | 10^3/µL | 2 | reference | - | OK |
| `basophils-pct` | Basophils (%) | cbc | % | 1 | reference | age_max, age_min | OK |
| `beta-carotene` | Beta-carotene | vitamin | µg/dL | 1 | reference | - | OK |
| `cadmium-blood` | Cadmium, whole blood | toxic-metal | µg/L | 1 | reference | - | OK |
| `cadmium-urine` | Cadmium, urine | toxic-metal | µg/g creatinine | 2 | reference, warn | age_min | OK |
| `ceruloplasmin` | Ceruloplasmin | trace-element | mg/dL | 3 | diagnostic, reference | - | OK |
| `chromium-blood` | Chromium, whole blood | toxic-metal | µg/L | 1 | warn | condition | OK |
| `cobalt-blood` | Cobalt, whole blood | toxic-metal | µg/L | 3 | reference, warn | condition | OK |
| `copper` | Copper | trace-element | µg/dL | 6 | diagnostic, reference | age_max, age_min | OK |
| `d-dimer` | D-dimer | coagulation | µg/mL FEU | 7 | reference | age_max, pregnant, trimester | OK |
| `dha` | Docosahexaenoic acid (DHA) | fatty-acid | % | 1 | reference | specimen | OK |
| `egrac` | Erythrocyte glutathione reductase activation coefficient (EGRAC) | vitamin | ratio | 3 | deficient, insufficient, optimal | - | OK |
| `eosinophils-abs` | Absolute eosinophil count | cbc | 10^3/µL | 5 | diagnostic, reference | ethnicity | OK |
| `eosinophils-pct` | Eosinophils (%) | cbc | % | 1 | reference | age_max, age_min | OK |
| `epa-aa-ratio` | EPA/arachidonic acid ratio | fatty-acid | ratio | 1 | warn | condition, specimen | OK |
| `epa` | Eicosapentaenoic acid (EPA) | fatty-acid | % | 1 | reference | specimen | OK |
| `esr` | Erythrocyte sedimentation rate (ESR) | inflammation | mm/h | 5 | reference | age_max, age_min, gestation_week_max, gestation_week_min, pregnant, sex | OK |
| `factor-viii` | Factor VIII activity | coagulation | IU/dL | 10 | diagnostic, excess, reference | age_max, age_min, gestation_week_max, gestation_week_min, pregnant, sex, trimester | OK |
| `ferritin` | Ferritin | iron | ng/mL | 26 | deficient, diagnostic, insufficient, reference, therapeutic | age_max, age_min, condition, markers, menopause, method, pregnant, sex, trimester | OK |
| `fibrinogen` | Fibrinogen | coagulation | g/L | 6 | critical, reference | condition, pregnant, sex, trimester | OK |
| `folate` | Folate (serum) | vitamin | ng/mL | 4 | deficient, insufficient, reference, warn | age_min, markers | OK |
| `g6pd` | G6PD activity | cbc | U/g Hb | 5 | deficient, diagnostic, reference, warn | - | OK |
| `glutamine` | Glutamine | amino-acid | µmol/L | 1 | therapeutic | condition | OK |
| `haptoglobin` | Haptoglobin | cbc | g/L | 3 | reference, warn | - | OK |
| `hematocrit` | Hematocrit | cbc | % | 4 | diagnostic, reference | age_max, age_min, sex | OK |
| `hemoglobin` | Hemoglobin | cbc | g/dL | 13 | diagnostic, reference | age_max, age_min, pregnant, sex, trimester | OK |
| `holotranscobalamin` | Holotranscobalamin | vitamin | pmol/L | 2 | deficient, reference | specimen | OK |
| `homocysteine` | Homocysteine (total) | inflammation | µmol/L | 7 | critical, reference, warn | age_max, age_min, sex, specimen | OK |
| `immature-granulocytes-pct` | Immature granulocytes (%) | cbc | % | 1 | reference | age_min | OK |
| `immature-reticulocyte-fraction` | Immature reticulocyte fraction (IRF) | cbc | % | 13 | reference | age_max, age_min, sex | OK |
| `inr` | International normalized ratio | coagulation | ratio | 5 | reference, therapeutic | age_max, age_min, condition, sex | OK |
| `iodine-urine` | Urinary iodine concentration | trace-element | µg/L | 6 | excess, insufficient, optimal | age_max, age_min, lactating, pregnant | OK |
| `iron` | Serum iron | iron | µg/dL | 4 | reference | age_max, age_min, sex | OK |
| `lactate` | Lactate | organic-acid | mmol/L | 4 | critical, diagnostic, warn | condition | OK |
| `lead-blood` | Blood lead | toxic-metal | µg/dL | 3 | critical, reference, warn | age_max, age_min | OK |
| `leucine` | Leucine | amino-acid | µmol/L | 3 | critical, therapeutic | age_max, age_min, condition | OK |
| `linoleic-acid` | Linoleic acid (LA) | fatty-acid | % | 1 | reference | specimen | OK |
| `lymphocytes-abs` | Absolute lymphocyte count | cbc | 10^3/µL | 4 | reference | ethnicity | OK |
| `lymphocytes-pct` | Lymphocytes (%) | cbc | % | 1 | reference | age_max, age_min | OK |
| `mch` | Mean corpuscular hemoglobin | cbc | pg | 1 | reference | age_max, age_min | OK |
| `mchc` | Mean corpuscular hemoglobin concentration | cbc | g/dL | 1 | reference | age_max, age_min | OK |
| `mcv` | Mean corpuscular volume | cbc | fL | 2 | reference, warn | age_max, age_min | OK |
| `mercury-blood` | Mercury, whole blood | toxic-metal | µg/L | 2 | reference, warn | age_max, age_min, sex | OK |
| `methylmalonic-acid` | Methylmalonic acid (MMA) | organic-acid | nmol/L | 3 | reference, warn | age_max, age_min, specimen | OK |
| `monocytes-abs` | Absolute monocyte count | cbc | 10^3/µL | 4 | reference | ethnicity | OK |
| `monocytes-pct` | Monocytes (%) | cbc | % | 1 | reference | age_max, age_min | OK |
| `mpv` | Mean platelet volume | cbc | fL | 1 | reference | - | OK |
| `neutrophils-abs` | Absolute neutrophil count | cbc | 10^3/µL | 10 | diagnostic, reference | condition, ethnicity | OK |
| `neutrophils-pct` | Neutrophils (%) | cbc | % | 1 | reference | age_max, age_min | OK |
| `nlr` | Neutrophil-to-lymphocyte ratio (NLR) | cbc | ratio | 2 | reference | age_max, age_min, ethnicity | OK |
| `nrbc` | Nucleated red blood cells (NRBC) | cbc | /100 WBC | 1 | reference | age_min | OK |
| `omega-3-index` | Omega-3 Index | fatty-acid | % | 3 | deficient, optimal, warn | specimen | OK |
| `phe-tyr-ratio` | Phenylalanine/Tyrosine ratio | amino-acid | ratio | 1 | warn | condition, method, specimen | OK |
| `phenylalanine` | Phenylalanine | amino-acid | µmol/L | 4 | therapeutic | age_max, age_min, condition, pregnant | OK |
| `platelets` | Platelet count | cbc | 10^3/µL | 2 | reference | age_max, age_min, sex | OK |
| `protein-c` | Protein C activity | coagulation | % | 6 | reference | age_max, age_min, gestation_week_max, gestation_week_min, pregnant, sex, trimester | OK |
| `protein-s-free` | Free protein S antigen | coagulation | % | 6 | reference | age_max, age_min, gestation_week_max, gestation_week_min, pregnant, sex, trimester | OK |
| `pt` | Prothrombin time | coagulation | s | 5 | reference | age_max, age_min, pregnant, region, sex | OK |
| `rbc-folate` | Red blood cell folate | vitamin | ng/mL | 2 | deficient, optimal | sex | OK |
| `rbc` | Red blood cell count | cbc | 10^6/µL | 2 | reference | age_max, age_min, sex | OK |
| `rdw-sd` | Red cell distribution width, standard deviation | cbc | fL | 3 | reference | age_max, age_min | OK |
| `rdw` | Red Cell Distribution Width | cbc | % | 2 | reference | age_max, age_min | OK |
| `reticulocyte-hemoglobin` | Reticulocyte hemoglobin (Ret-He / CHr) | iron | pg | 15 | deficient, reference, therapeutic | age_max, age_min, condition, sex | OK |
| `reticulocytes-abs` | Reticulocytes (absolute) | cbc | 10^9/L | 12 | reference | age_max, age_min, sex | OK |
| `reticulocytes-pct` | Reticulocytes (%) | cbc | % | 12 | reference | age_max, age_min, sex | OK |
| `selenium` | Selenium | trace-element | µg/L | 3 | deficient, optimal, toxic | specimen | OK |
| `soluble-transferrin-receptor` | Soluble transferrin receptor | iron | mg/L | 3 | reference | menopause, sex | OK |
| `thrombin-time` | Thrombin time | coagulation | s | 3 | reference | pregnant, region | OK |
| `tibc` | Total iron-binding capacity | iron | µg/dL | 4 | reference | age_max, age_min, sex | OK |
| `transferrin-saturation` | Transferrin saturation | iron | % | 7 | deficient, diagnostic, reference | condition, markers, menopause, sex | OK |
| `transferrin` | Transferrin | iron | mg/dL | 4 | reference | age_max, age_min, sex | OK |
| `tyrosine` | Tyrosine | amino-acid | µmol/L | 1 | warn | - | OK |
| `uibc` | Unsaturated iron-binding capacity | iron | µg/dL | 2 | reference | sex | OK |
| `vitamin-a` | Retinol (vitamin A) | vitamin | µg/dL | 4 | critical, deficient | lactating, pregnant | OK |
| `vitamin-b1` | Vitamin B1 (thiamine diphosphate, whole blood) | vitamin | nmol/L | 2 | reference | - | OK |
| `vitamin-b12` | Vitamin B12 | vitamin | pg/mL | 5 | deficient, insufficient, reference, warn | age_min, markers, pregnant | OK |
| `vitamin-b6` | Vitamin B6 (pyridoxal 5'-phosphate) | vitamin | nmol/L | 1 | deficient | - | OK |
| `vitamin-c` | Vitamin C (ascorbic acid) | vitamin | mg/dL | 3 | deficient, insufficient, optimal | - | OK |
| `vitamin-d-1-25` | 1,25-dihydroxyvitamin D | vitamin | pg/mL | 7 | reference, warn | age_max, age_min, condition, method | OK |
| `vitamin-d` | 25-hydroxyvitamin D | vitamin | ng/mL | 10 | deficient, excess, insufficient, optimal, reference, toxic | - | OK |
| `vitamin-e` | Alpha-tocopherol (vitamin E) | vitamin | mg/L | 2 | deficient, optimal | - | OK |
| `vitamin-k1` | Phylloquinone (vitamin K1) | vitamin | ng/mL | 1 | reference | - | OK |
| `vwf-antigen` | von Willebrand factor antigen (VWF:Ag) | coagulation | IU/dL | 3 | diagnostic, reference, warn | condition | OK |
| `wbc` | White blood cell count | cbc | 10^3/µL | 6 | reference | age_max, age_min, condition, ethnicity | OK |
| `zinc` | Zinc | trace-element | µg/dL | 6 | deficient, reference | age_max, age_min, fasting, pregnant, sex | OK |

Totals: 92 markers, 370 ranges, 128 distinct PMIDs, 119 distinct DOIs.

Range kinds: reference 230, diagnostic 36, deficient 33, warn 22, therapeutic 17, optimal 11, insufficient 9, critical 6, toxic 3, excess 3

Evidence types: reference-interval 206, guideline 67, review 44, outcome 29, consensus 21, textbook 3

Condition dimensions (`when` keys, ranges using each): `age_min` 159, `sex` 148, `age_max` 140, `pregnant` 54, `condition` 48, `trimester` 30, `gestation_week_min` 21, `gestation_week_max` 21, `specimen` 18, `method` 15, `markers` 13, `ethnicity` 11, `menopause` 6, `fasting` 4, `region` 3, `lactating` 2

Citation entries: 381 checked, 381 passed, 0 failed.

## Markers deliberately skipped, and why

No range was written unless the number could be retrieved and quoted. These markers stay uncovered:

| marker | reason |
|---|---|
| vwf-activity (VWF:RCo / GPIbM) | The ASH/ISTH/NHF/WFH 2021 guideline and BSH 2014 have no retrievable full text, and no OA source quotes activity cutoffs. VWF antigen is covered. |
| eosinophilia 0.5 ×10^9/L threshold | Not stated in any retrievable text (the Valent 2012 consensus is not OA). The ≥1.5 ×10^9/L HE threshold is covered. |
| zinc-protoporphyrin | No WHO/CDC cutoff found; only a 1994 single-study "normal ≤40 µmol/mol heme". |
| stfr-ferritin-index | Only cutoff found comes from one paediatric severe-anaemia study (research use). |
| hepcidin | Research/limited clinical use; no harmonised clinical cutoffs. |
| arsenic-urine | No NHANES 95th percentile or ACGIH BEI in retrievable text; only medians/IQRs. |
| manganese (whole blood) | NHANES percentiles are only on CDC web pages (not retrievable); papers give means and medians only. |
| chromium (serum), molybdenum, nickel, thallium | Specialist or research use; no clinically used quoted reference ranges. Whole-blood chromium is covered for metal-on-metal implants. |
| vitamin-b2 (plasma riboflavin) | No quoted plasma reference interval. Riboflavin status is covered by `egrac`. |
| biotin, niacin, pantothenate | No clinically used blood test with quoted ranges (niacin: research-level NAD metabolites only). |
| citrulline | Only mean ± SD from healthy controls; no reference interval. |
| omega-6/omega-3 ratio, RBC trans-fat index | Only dietary ratios or quartile risk ratios; no blood-level cutoffs. |
| SAM/SAH, other methylation metabolites, MTHFR | Research-only assays or genotypes, with no ranges. |
| beta-hydroxybutyrate | Ketone body, treated as part of the metabolic shard, not organic acids in common use here. |
| magnesium (seeded, category electrolyte) | Belongs to the electrolyte shard. |

## Known gaps and caveats within covered markers

- **Not retrievable, so not encoded**:
  - KDIGO 2012 ferritin/TSAT CKD thresholds: the PDF renders "≤" as "r". The BSG 2021 CKD definitions are used instead.
  - ESC 2021 HF iron-deficiency text: cited through a 2023 review instead, under set `hf-trial-definition`.
  - INR 2.5–3.5 for mechanical mitral valves: CHEST 2012 point targets 2.5 and 3.0 are given instead.
  - LMWH once-daily anti-Xa target.
  - IZiNCG men's and pregnancy zinc cutoffs.
  - BSH 2014 cobalamin/folate guideline numbers.
  - Refsum 2004 homocysteine grades: grades are from a 2024 systematic review (16–30 / 31–100 / >100).
  - Microcytosis < 80 fL.
- **Secondary citations**: some guideline numbers are quoted through reviews or papers that cite the guideline. The `set` and `note`
  say "as cited" where that is the case.
- **Analyser- and population-specific intervals**: coagulation times, reticulocyte indices, RDW-SD, IG/NRBC and the
  indirect ferritin intervals are tied to an analyser or local population (noted per range). Labs should prefer their own limits.
- **Single-study or weaker sources** are flagged in `note`. Examples: the IG/NRBC Sysmex study (n=280); the EPA/AA < 0.3 enrolment
  threshold; the 1989 vitamin K1 HPLC study; the Labcorp beta-carotene range.
- **Not encoded (no `when` key)**: time-of-day partitions (evening WBC differentials) and haemoglobin altitude/smoking adjustments
  (effect sizes only, no adjusted cutoffs in OA text).
