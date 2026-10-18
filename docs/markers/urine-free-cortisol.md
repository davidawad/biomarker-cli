# Cortisol, Free, 24-Hour Urine (`urine-free-cortisol`)

Unconjugated (free) cortisol excreted in a 24-hour urine collection. Reflects the biologically active cortisol fraction integrated over the day; used as a screening test for Cushing's syndrome. Results depend on a complete collection and on the assay (LC-MS/MS vs immunoassay).

| | |
|---|---|
| Category | [adrenal](index.md#adrenal) |
| Canonical unit | µg/24 h |
| LOINC | - |
| Specimen | - |
| Aliases | `cortisol-urine-free-24h` |

## Ranges

Bounds in µg/24 h. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 11 | 70 | reference-interval-study | Literature reference range (11-70 µg/day, LC-MS/MS), confirmed in a healthy study group (specimen: urine-24h; method: lc-ms/ms) | [Measurement of urinary free cortisol by LC-MS-MS: adoption of a literature reference range and comparison with our current immunometric method.](https://pubmed.ncbi.nlm.nih.gov/31012054/), Abstract, Results | Literature-derived reference range for 24-hour urinary free cortisol by LC-MS/MS, confirmed in a healthy group in one laboratory. Immunoassays overestimated results compared with LC-MS/MS. Converted from the source's µg/day (11-70) to µg/24 h using factor 1. |
| neoplastic-hypercortisolism-lcms-2026 | warn | neoplastic-hypercortisolism | - | 43.5 | cohort | Adults screened for neoplastic hypercortisolism at one centre (2015-2022), with 24-hour urine collected on two consecutive days; 27 cases and 417 non-cases for urinary cortisol (specimen: urine-24h; method: lc-ms/ms) | [Cut-off values of urinary free cortisol and cortisone for diagnosing neoplastic hypercortisolism.](https://pubmed.ncbi.nlm.nih.gov/42173303/), Abstract, Results | ROC-derived optimal cut-off for 24-hour urinary free cortisol by LC-MS/MS (sensitivity 89%, specificity 86%, AUC 0.937); values at or above it are suggestive of neoplastic hypercortisolism. Single-centre retrospective cut-off that the authors say needs validation in another cohort; not a guideline upper limit. Source unit nmol/24 h, not converted. Converted from the source's nmol/24 h (120) to µg/24 h using factor 0.3625. Source kind: diagnostic. Source band >= 43.5; stored as the warn high bound 43.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
