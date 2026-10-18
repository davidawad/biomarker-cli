# Urine White Blood Cells (per microlitre) (`urine-wbc-count`)

Urine leukocyte concentration per microlitre from automated urinalysis or flow cytometry, as opposed to the per high-power-field count from manual microscopy.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | /µL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in /µL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| pyuria-cutoffs-older-women-2023 | warn | all | - | 10 | consensus | Adults: commonly used pyuria cut-off (urine leukocytes by automated microscopy or flow cytometry) | [Current Pyuria Cutoffs Promote Inappropriate Urinary Tract Infection Diagnosis in Older Women.](https://pubmed.ncbi.nlm.nih.gov/36806580/), Abstract, Results | Commonly used pyuria cut-off of 10 leukocytes/µL. In women aged 65 and over with lower urinary tract symptoms, this cut-off had sensitivity 100% but specificity only 36% for UTI, which the authors use to argue for higher thresholds. Source kind: decision. Source band >= 10; stored as the warn high bound 10. |
| pyuria-cutoffs-older-women-2023 | warn | female, age 65+ | - | 264 | cohort | Women aged 65 and over with new lower urinary tract symptoms, UTI versus asymptomatic bacteriuria and controls (164 participants) | [Current Pyuria Cutoffs Promote Inappropriate Urinary Tract Infection Diagnosis in Older Women.](https://pubmed.ncbi.nlm.nih.gov/36806580/), Abstract, Results; [Current Pyuria Cutoffs Promote Inappropriate Urinary Tract Infection Diagnosis in Older Women.](https://pubmed.ncbi.nlm.nih.gov/36806580/), Abstract, Methods | Microscopy cut-off derived for UTI in older women (sensitivity 88%, positive likelihood ratio 7.2). It is a single-study result in a selected population, not a general adult pyuria threshold. Source kind: decision. Source band >= 264; stored as the warn high bound 264. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
