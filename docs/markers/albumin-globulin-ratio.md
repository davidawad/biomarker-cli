# Albumin/Globulin Ratio (`albumin-globulin-ratio`)

Ratio of serum albumin to calculated globulin. Low values are associated with higher mortality in dialysis and critically ill patients.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | ratio |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male, age 18+ | 1.1 | 1.8 | reference-interval-study | Healthy adult males (Gandaki Province, Nepal; dry-chemistry method) | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Results (sex-specific 95% RIs) | - |
| clinical | reference | female, age 18+ | 1 | 1.7 | reference-interval-study | Healthy adult females (Gandaki Province, Nepal; dry-chemistry method) | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Results (sex-specific 95% RIs) | - |
| clinical | reference | age 18+ | 1 | 1.8 | reference-interval-study | Healthy adults (Gandaki Province, Nepal; dry-chemistry method), sexes combined | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Results (combined 95% RIs) | - |
| clinical | warn | all | 0.8 | - | cohort | Adults admitted to a respiratory intensive care unit (critically ill) | [Albumin-Globulin Ratio Is an Independent Determinant of 28-Day Mortality in Patients with Critical Illness.](https://pubmed.ncbi.nlm.nih.gov/34484470/), Abstract (Methods) | AGR below 0.8 (low AGR group) was associated with higher 28-day mortality (HR 0.484 for medium and 0.332 for high AGR versus low). Single-centre cohort; association only. Source band <= 0.8; stored as the warn low bound 0.8. |
| clinical | warn | peritoneal-dialysis | 1.25 | - | cohort | Incident patients on peritoneal dialysis (retrospective cohort, 2008-2017) | [Albumin-globulin ratio and mortality in patients on peritoneal dialysis: a retrospective study.](https://pubmed.ncbi.nlm.nih.gov/32059708/), Results | AGR of 1.25 or less (lower AGR group) was associated with higher all-cause (HR 1.57) and cardiovascular mortality (HR 2.01). Association only; cut-off chosen by the authors' survival comparison. Source band <= 1.25; stored as the warn low bound 1.25. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
