# Prostate Health Index (PHI) (`phi`)

Prostate Health Index: a calculated score combining [-2]proPSA, free PSA and total PSA, used with total PSA to estimate prostate cancer risk and biopsy need. Expected result: a numeric score where higher values carry higher risk; no single normal value is established in the verified sources.

| | |
|---|---|
| Category | [tumor-marker](index.md#tumor-marker) |
| Canonical unit | index |
| LOINC | [97150-7](https://loinc.org/97150-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | suspected-prostate-cancer | - | 40 | cohort | Men at initial prostate biopsy (cohort named in the cited title) | [Accuracy of the prostate health index versus the urinary prostate cancer antigen 3 score to predict overall and significant prostate cancer at initial biopsy.](https://pubmed.ncbi.nlm.nih.gov/25327361/), Abstract | Usual positivity cut-off as stated by the source: PHI above 40 (source wording 'PHI >40'). Strictly above 40. Source band >= 40; stored as the warn high bound 40. |
| clinical | reference | suspected-prostate-cancer | 35 | 55 | cohort | Chinese men with PSA 10-20 ng/mL and normal digital rectal examination | [Extended use of Prostate Health Index and percentage of [-2]pro-prostate-specific antigen in Chinese men with prostate specific antigen 10-20 ng/mL and normal digital rectal examination.](https://pubmed.ncbi.nlm.nih.gov/27617315/), Abstract | PHI category 'PHI 35-55' as labelled by the source, which also labels PHI<35 and PHI>55. The source does not state whether 35 or 55 is inclusive. Percentages in the quote are Gleason 7 or above prevalence by category in this cohort. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
