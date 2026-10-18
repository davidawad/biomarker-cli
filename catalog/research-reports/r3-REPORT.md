# Research shard r3: endocrine, bone, pregnancy and pediatric ranges

Bead: `biomarker-cli-8h7.3.3` (parent `biomarker-cli-8h7.3`).
Shard: thyroid, adrenal, sex and reproductive hormones (incl. AMH, SHBG, free and bioavailable fractions), pituitary, growth (IGF-1, IGFBP-3), bone turnover, pregnancy-specific and pediatric-specific markers.

## Summary

- **65 marker files**, **430 ranges** and **487 citation entries**. That covers 152 unique PMIDs and 147 unique DOIs.
- **0 verification failures.** Every PMID resolves with a matching title, every DOI resolves, every quote was found verbatim, and every `low`/`high` number appears in its range's quotes.
- Existing seed slugs were reused: `tsh`, `free-t4`, `free-t3`, `tpo-ab`, `testosterone-total`, `testosterone-free`, `estradiol`, `igf-1`, `shbg`, `dhea-s`, `cortisol`, `lh`, `fsh` and `prolactin`. Unit and LOINC match `src/seed.rs` for all of them. Three markers use a more specific category than the seed (`cortisol`, `dhea-s` → `adrenal`; `igf-1` → `growth`; the seed says `hormone`). The merge step can map these back if it needs to.

## Format, and the assumptions behind it

The epic body (`br show biomarker-cli-8h7`) was not shipped in this box: `.beads/issues.jsonl` only holds the shard and parent stubs. I used the field list from the bead text and pinned down the values as follows, so the merge step can remap them mechanically if the epic differs:

- Top level: `slug`, `name`, `loinc` (only where known), `category`, `unit` (canonical), `description`, then `[[range]]` entries.
- `set` is a kebab id of the source framework or study, e.g. `prognosis-2016`, `nhanes-surks-2007`, `endocrine-society-2025`.
- `kind` is one of `reference`, `optimal`, `warn`, `critical`, `diagnostic` (decision cut-off) or `target` (treatment target). There are no `optimal` ranges: no outcome study justifying one was found for these markers.
- `low`/`high` are written exactly as printed in the source. A one-sided `low = X` means the zone is values ≥ X; `high = X` means values ≤ X. The `note` says what the bound means.
- `unit` on a range appears only when the source unit differs from the canonical unit. The numbers are then left unconverted so that they can be checked against the quote.
- `when` is an inline table with these keys: `sex`, `age_min`, `age_max`, `age_unit` (default years), `pregnancy`, `trimester`, `gestational_week_min`/`gestational_week_max`, `postpartum`, `menopause`, `cycle_phase`, `tanner_min`/`tanner_max`, `fasting`, `time_of_day`, `specimen`, `method`, `ethnicity`, `conditions`, `on_treatment`, `markers` (other-marker thresholds, `{slug, op, value, unit}`) and `test` (stimulation or suppression test context).
- `evidence` is one of `guideline`, `consensus`, `reference-interval`, `outcome`, `meta-analysis`, `review` or `textbook`. When a paywalled guideline value is quoted from an open-access or abstract source that restates it, the range is marked `review` and its note names the guideline.
- Citations are `{pmid, doi, title, url, locator, quote}`. `title` is an extra field, needed for the PMID title-match check.

## Method

The lead (Opus) fanned out 18 Haiku subagent batches in three waves:

1. Thirteen category batches.
2. Five gap-filling and extra-marker batches.
3. Two batches for adult IGF-1/IGFBP-3 and pediatric sex-hormone age bands.

Each file was owned by exactly one agent at a time. Literature access went through a shared client that queries NCBI E-utilities and Europe PMC REST (the research CLI is not installed). A file lock holds the whole box to about 2.5 requests/s.

Text sources were PubMed abstracts and Europe PMC open-access full text. In the full text, tables are rendered as `label | cell | cell`.

### Deterministic verification

The lead ran this on every file before each commit:

1. The TOML parses and matches the schema above: allowed keys and enums, filename equal to slug, `low ≤ high`.
2. Every PMID resolves through `esummary`, and the citation `title` matches the PubMed title (similarity ≥ 0.95 after normalisation). Any DOI given must equal PubMed's DOI.
3. Every DOI resolves via `https://doi.org/api/handles/<doi>` (`responseCode == 1`).
4. Every `quote` is a verbatim substring of the PubMed abstract or the Europe PMC open-access full text. Case, whitespace and punctuation are normalised, but a boundary between two digit groups is preserved, so a quote cannot re-split run-together table cells.
5. Every `low`/`high` value appears as a number in that range's verified quotes. Numbers written as words ("three times the upper limit") also count.

A final cache-free pass re-queried every PMID (batch `esummary`) and every DOI: 152/152 and 147/147 passed.

There are 93 non-blocking warnings. In each, a `when` age or gestational-week bound comes from a table row label or the study's methods, not from the quoted sentence itself: 58 `age_max`, 26 `age_min`, 9 gestational week.

### Lead review: corrections made

The verifier proves that each quote is real and contains the number, not that the range means what the quote says. The lead read every range against its quote and the source tables. Corrections made:

- **Inverted directions on one-sided cut-offs**, fixed:
  - `papp-a`: ≤0.4 and <0.3 MoM become `high`.
  - `afp-maternal`: ≥2.5 MoM becomes `low`.
  - `inhibin-a`: >2.0 MoM becomes `low`.
  - `estriol-unconjugated`: <0.5 MoM becomes `high`.
  - `17-oh-progesterone`: newborn recall above 12 ng/mL becomes `low`.
- **Guideline values that reached us only through secondary papers**, relabelled `evidence = "review"`: German calcitonin thresholds; Endocrine Society 2014, AACE and national pediatric GH criteria.
- **Table quotes checked against the table headers** for column mapping: Advia LH/FSH/progesterone; LC-MS/MS adrenal steroids (ALL | Males | Females); Wuhan pediatric E2/FSH/LH/testosterone (Male | Female); VARIETE, Korean and INDIIGo IGF-1; female testosterone quantiles.
- **Verifier bugs found and fixed during the run:**
  - A space-thousands heuristic merged `167 150` into one number.
  - Europe PMC HTTP 500 responses crashed the verifier; that is what caused the Travison 2017 range to be dropped in wave 1, and it was later restored.
  - Run-together table cells could be re-split with spaces inside a quote.
- **Other fixes:**
  - Dropped a free β-hCG range that was an AUC statement, not a threshold.
  - Added `pregnancy = true` to the free β-hCG ranges.
  - Tagged the newborn-screening ranges.
  - Gave the metanephrine upper reference cut-off `kind = reference`.
  - Added the missing upper bound to the luteal progesterone range.
  - Added the Korean ethnicity tag on the Korean TSH range.
  - Added the PROGNOSIS (Zeisler 2016, NEJM) sFlt-1/PlGF ≤38 rule-out range.
  - Added the ATA >3× ULN TRAb pregnancy threshold.
  - Added adult 17-OHP and DHEA-S LC-MS/MS ranges by sex.

## Markers covered

| Slug | Name | Category | Unit | Ranges | Condition dimensions | Citations verified |
|---|---|---|---|---|---|---|
| `17-oh-progesterone` | 17-Hydroxyprogesterone | adrenal | ng/dL | 6 | conditions, fasting, method, sex, specimen, test, time_of_day | 6/6 PASS |
| `acth` | Adrenocorticotropic Hormone | pituitary | pg/mL | 3 | age_max, age_min, conditions, specimen, time_of_day | 3/3 PASS |
| `afp-maternal` | Alpha-Fetoprotein, Maternal Serum | pregnancy | MoM | 1 | pregnancy, trimester | 2/2 PASS |
| `aldosterone-renin-ratio` | Aldosterone-to-Renin Ratio | adrenal | ng/dL per ng/mL/h | 6 | age_max, age_min, markers, method, test | 6/6 PASS |
| `aldosterone` | Aldosterone | adrenal | ng/dL | 8 | conditions, ethnicity, markers, method, sex, test | 8/8 PASS |
| `amh` | Anti-Müllerian Hormone | hormone | ng/mL | 15 | age_max, age_min, conditions, menopause, method, sex, specimen | 15/15 PASS |
| `androstenedione` | Androstenedione | adrenal | ng/dL | 2 | age_max, age_min, method, sex | 2/2 PASS |
| `bile-acids-total` | Bile Acids, Total (serum) | pregnancy | µmol/L | 4 | conditions, fasting, pregnancy, specimen, trimester | 5/5 PASS |
| `blood-lead` | Lead, Blood | pediatric | µg/dL | 1 | age_max, age_min, specimen | 2/2 PASS |
| `bone-alp` | Bone-Specific Alkaline Phosphatase | bone | µg/L | 9 | age_max, age_min, fasting, menopause, method, sex, specimen, time_of_day | 15/15 PASS |
| `calcitonin` | Calcitonin | thyroid | pg/mL | 9 | method, sex | 9/9 PASS |
| `copeptin` | Copeptin | pituitary | pmol/L | 3 | age_max, specimen, test | 6/6 PASS |
| `cortisol-salivary-late-night` | Cortisol, Late-Night Salivary | adrenal | ng/dL | 7 | age_max, age_min, conditions, ethnicity, method, specimen, time_of_day | 7/7 PASS |
| `cortisol-urine-free-24h` | Cortisol, Free, 24-Hour Urine | adrenal | µg/24 h | 2 | conditions, method, specimen | 2/2 PASS |
| `cortisol` | Cortisol (AM) | adrenal | µg/dL | 15 | age_max, age_min, method, specimen, test, time_of_day | 15/15 PASS |
| `ctx` | C-Terminal Telopeptide (β-CrossLaps, serum CTX) | bone | ng/mL | 10 | age_max, age_min, ethnicity, fasting, menopause, method, sex, specimen, time_of_day | 16/16 PASS |
| `dhea-s` | DHEA-Sulfate | adrenal | µg/dL | 6 | age_max, age_min, method, sex, specimen | 6/6 PASS |
| `dhea` | DHEA (Dehydroepiandrosterone, unconjugated) | adrenal | ng/dL | 7 | age_max, age_min, method, sex, specimen | 7/7 PASS |
| `dht` | Dihydrotestosterone | hormone | ng/dL | 2 | menopause, method, sex | 2/2 PASS |
| `estradiol` | Estradiol | hormone | pg/mL | 19 | age_max, age_min, cycle_phase, menopause, method, sex, tanner_max | 19/19 PASS |
| `estriol-unconjugated` | Unconjugated Estriol, Maternal Serum | pregnancy | MoM | 1 | pregnancy, trimester | 2/2 PASS |
| `estrone` | Estrone | hormone | pg/mL | 2 | age_max, age_min, method, sex, tanner_max | 2/2 PASS |
| `fetal-fibronectin` | Fetal Fibronectin, Cervicovaginal | pregnancy | ng/mL | 6 | conditions, gestational_week_max, gestational_week_min, pregnancy, specimen | 6/6 PASS |
| `free-androgen-index` | Free Androgen Index | hormone | % | 1 | conditions, method, sex, test | 1/1 PASS |
| `free-beta-hcg` | Free β-hCG, Maternal Serum | pregnancy | MoM | 3 | pregnancy, specimen, trimester | 3/3 PASS |
| `free-t3` | Free T3 | thyroid | pg/mL | 10 | age_max, age_min, method, pregnancy, sex, trimester | 10/10 PASS |
| `free-t4` | Free T4 | thyroid | ng/dL | 12 | age_max, age_min, method, pregnancy, sex, trimester | 12/12 PASS |
| `fsh` | Follicle Stimulating Hormone | hormone | mIU/mL | 13 | age_max, age_min, conditions, cycle_phase, menopause, method, sex | 13/13 PASS |
| `growth-hormone` | Growth Hormone | pituitary | ng/mL | 10 | age_max, conditions, test | 10/10 PASS |
| `hcg` | Human Chorionic Gonadotropin, Quantitative (beta-hCG) | pregnancy | mIU/mL | 9 | markers, menopause, method, pregnancy, trimester | 9/9 PASS |
| `igf-1` | Insulin-like Growth Factor 1 | growth | ng/mL | 28 | age_max, age_min, ethnicity, fasting, method, on_treatment, sex, specimen, time_of_day | 40/40 PASS |
| `igfbp-3` | Insulin-like Growth Factor Binding Protein 3 | growth | mg/L | 10 | age_max, age_min, ethnicity, fasting, method, sex, specimen, time_of_day | 19/19 PASS |
| `immunoreactive-trypsinogen` | Immunoreactive Trypsinogen (newborn screen) | pediatric | ng/mL | 2 | specimen | 2/2 PASS |
| `inhibin-a` | Inhibin A, Maternal Serum | pregnancy | MoM | 1 | pregnancy, trimester | 2/2 PASS |
| `inhibin-b` | Inhibin B | hormone | pg/mL | 9 | age_max, age_min, conditions, sex, tanner_max, tanner_min | 9/9 PASS |
| `lh` | Luteinizing Hormone | hormone | mIU/mL | 23 | age_max, age_min, conditions, cycle_phase, menopause, method, sex, specimen, tanner_max, tanner_min, test | 23/23 PASS |
| `metanephrine-plasma-free` | Metanephrine, Plasma Free | adrenal | pg/mL | 3 | method, specimen | 3/3 PASS |
| `metanephrines-urine-24h` | Metanephrines, Fractionated, 24-Hour Urine | adrenal | µg/24 h | 2 | age_max, age_min, sex, specimen | 2/2 PASS |
| `normetanephrine-plasma-free` | Normetanephrine, Plasma Free | adrenal | pg/mL | 4 | age_max, age_min, method, specimen | 4/4 PASS |
| `ntx-urine` | N-Telopeptide, Urine | bone | nmol BCE/mmol creatinine | 2 | age_max, age_min, menopause, method, sex, specimen | 4/4 PASS |
| `osteocalcin` | Osteocalcin | bone | ng/mL | 3 | age_max, age_min, fasting, menopause, method, sex, specimen, time_of_day | 7/7 PASS |
| `p1np` | Procollagen Type 1 N-Terminal Propeptide | bone | ng/mL | 10 | age_max, age_min, ethnicity, fasting, menopause, method, sex, specimen, time_of_day | 16/16 PASS |
| `papp-a` | Pregnancy-Associated Plasma Protein A | pregnancy | MoM | 3 | pregnancy, trimester | 3/3 PASS |
| `plgf` | Placental Growth Factor | pregnancy | pg/mL | 3 | conditions, gestational_week_max, gestational_week_min, pregnancy | 3/3 PASS |
| `pregnenolone` | Pregnenolone | adrenal | ng/dL | 7 | age_max, age_min, method, sex, specimen | 7/7 PASS |
| `progesterone` | Progesterone | hormone | ng/mL | 7 | conditions, cycle_phase, menopause, method, pregnancy, sex, trimester | 7/7 PASS |
| `prolactin` | Prolactin | hormone | ng/mL | 11 | age_max, age_min, conditions, ethnicity, postpartum, pregnancy, sex, trimester | 11/11 PASS |
| `pth` | Parathyroid Hormone, Intact | bone | pg/mL | 3 | conditions | 3/3 PASS |
| `pthrp` | Parathyroid Hormone-Related Protein | bone | pmol/L | 1 | method | 1/1 PASS |
| `renin-activity` | Plasma Renin Activity | adrenal | ng/mL/h | 2 | markers, method, on_treatment | 2/2 PASS |
| `renin-direct` | Direct Renin Concentration | adrenal | mIU/L | 3 | ethnicity, method, sex, test | 3/3 PASS |
| `sflt1-plgf-ratio` | sFlt-1/PlGF Ratio | pregnancy | ratio | 5 | conditions, gestational_week_max, gestational_week_min, method, pregnancy | 7/7 PASS |
| `shbg` | Sex Hormone Binding Globulin | hormone | nmol/L | 2 | age_max, age_min, ethnicity, menopause, method, sex | 2/2 PASS |
| `sweat-chloride` | Sweat Chloride | pediatric | mmol/L | 3 | specimen | 5/5 PASS |
| `tbg` | Thyroxine-Binding Globulin | thyroid | mg/L | 1 | conditions, specimen | 1/1 PASS |
| `testosterone-bioavailable` | Testosterone, Bioavailable | hormone | ng/dL | 1 | age_max, age_min, sex | 1/1 PASS |
| `testosterone-free` | Testosterone, Free | hormone | pg/mL | 4 | age_max, age_min, method, sex | 4/4 PASS |
| `testosterone-total` | Testosterone, Total | hormone | ng/dL | 13 | age_max, age_min, conditions, method, sex, test | 13/13 PASS |
| `thyroglobulin-ab` | Thyroglobulin Antibodies | thyroid | IU/mL | 3 | conditions | 3/3 PASS |
| `thyroglobulin` | Thyroglobulin | thyroid | ng/mL | 8 | conditions, sex, test | 8/8 PASS |
| `total-t3` | Total T3 (Triiodothyronine) | thyroid | ng/dL | 3 | age_max, age_min, pregnancy, sex, trimester | 3/3 PASS |
| `total-t4` | Total T4 (Thyroxine) | thyroid | µg/dL | 3 | age_max, age_min, pregnancy, sex, trimester | 3/3 PASS |
| `tpo-ab` | Thyroid Peroxidase Antibodies | thyroid | IU/mL | 4 | method, pregnancy | 4/4 PASS |
| `trab` | TSH Receptor Antibodies (TRAb/TSI) | thyroid | IU/L | 13 | conditions, ethnicity, method, pregnancy, trimester | 13/13 PASS |
| `tsh` | Thyroid Stimulating Hormone | thyroid | mIU/L | 28 | age_max, age_min, ethnicity, method, pregnancy, sex, specimen, trimester | 28/28 PASS |

## Condition dimensions used

Number of markers using each `when` key: method 41, sex 36, age_max 36, age_min 34, specimen 29, conditions 23, pregnancy 19, trimester 15, menopause 13, test 11, ethnicity 11, time_of_day 10, fasting 8, markers 4, tanner_max 4, cycle_phase 4, gestational_week_min/max 3, tanner_min 2, on_treatment 2, postpartum 1.

Range kinds: reference 286, diagnostic 130, warn 7, critical 4, target 3. Evidence: reference-interval 281, outcome 98, review 34, guideline 9, meta-analysis 5, consensus 3.

## Citations: verified and failed

- 487 of 487 citation entries verified, covering 152 unique PMIDs and 147 DOIs. Every citation has a PMID; there are no URL-only citations.
- Failed or never written: any citation an agent could not verify was dropped or fixed before staging. Known cases:
  - One DOI recalled from memory was rejected and replaced with the PubMed DOI (IRT, Georgia).
  - A guessed PMID for ATA 2017 turned out to be an unrelated paper; it was caught before use.
  - The Travison 2017 range was dropped by the Europe PMC crash, then restored after the fix.
  - Several candidate ranges were dropped because their numbers appear only in figures or in tables that couldn't be retrieved.

## Deliberately skipped markers

| Marker | Reason |
|---|---|
| reverse-t3 | The only numeric source (PMID 32523672) has unlabelled AM/PM table columns and an implausible 195 ng/dL bound. Not shipped without a check against the PDF. |
| trap-5b | No numeric reference interval in any retrievable text, and the 2025 ESCEO/IOF/IFCC consensus gives none. |
| psa, free-psa | Prostate and tumour markers, not hormones. Left to the oncology/urology shard; the seed lists `psa` under `hormone`. |
| semen analysis (WHO 2021) | Not a hormone; out of shard scope. |
| vitamin-d, 1,25-dihydroxyvitamin D, calcium, phosphate, ALP, urine iodine | Owned by the vitamin, CMP and nutrition shards. |
| neonatal bilirubin, pregnancy OGTT/glucose, pregnancy urine protein:creatinine, pregnancy hemoglobin/ferritin | Pediatric/pregnancy thresholds for markers whose slugs (`bilirubin-total`, `glucose`, `hemoglobin`, `ferritin` ...) belong to other shards. Adding files here would collide at merge; those shards should carry the pregnancy and neonatal `when` conditions. |
| plasma/urine catecholamines | Superseded by metanephrines for pheochromocytoma work-up. |
| 11-deoxycortisol, alpha-subunit, macroprolactin, oxytocin, kisspeptin, sclerostin | Niche, specialist or research-only; no consumer or routine-tracking use. |

## Known gaps and limitations

- **Paywalled guidelines.** These have no open-access text and no numbers in their abstracts: Endocrine Society (hypogonadism 2018, CAH 2018, primary aldosteronism 2016/2025, pheochromocytoma 2014, hyperprolactinemia 2011), ATA 2015 and 2017/2026, CF Foundation 2017 (Farrell), the AACE 2019 GH guideline, and NICE DG49. The CAH 2018 and adrenal-insufficiency 2016 PMC copies have PMCIDs, but Europe PMC returns HTTP 500 for them. Their cut-offs appear only where an open-access paper restates them verbatim, marked `review`. Under the "never write a number not in retrieved text" rule, several canonical values are therefore missing:
  - ATA trimester TSH limits beyond 4.0 mU/L
  - Basal 17-OHP of 200 ng/dL (the nmol/L equivalents are present)
  - The 1 mg DST 1.8 µg/dL threshold (the 50 nmol/L form is present)
  - The EAU 8 nmol/L testosterone threshold
  - STRAW+10 FSH
- **Single-centre and regional sources.** Many ranges come from them, notably Chinese, Korean, Turkish and Dutch cohorts, especially for metanephrines, ACTH, TRAb, calcitonin and inhibin B. The `population` and `note` fields name the cohort and assay; treat these as assay- and population-specific.
- **Thin or partial coverage:**
  - Estrone: pediatric only. No adult numeric interval was retrievable.
  - TBG: newborn dried blood spot only.
  - PTHrP: one review-level upper limit.
  - IGFBP-3: no adult women.
  - SHBG: no pediatric or pregnancy ranges.
  - AMH: no Western/US age nomogram (the source found gave medians only).
- **No optimal ranges.** No outcome study justifying one was found for markers in this shard.

## Reproduction

The verifier, literature client and helpers were kept outside the repo: per the brief, only marker files and this report are added under `catalog/research/`. They are in `/work/artifacts/tools/` on the research box (`lit.py`, `verify.py`, `fresh_check.py`), along with the per-file verifier JSON (`/work/artifacts/verify/`) and the full log (`/work/artifacts/verify-all.txt`).
