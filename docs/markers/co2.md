# Carbon Dioxide (Bicarbonate) (`co2`)

Serum bicarbonate (total CO2) reflects the metabolic component of acid-base balance. In CKD, low bicarbonate is common and associated with worse outcomes, and KDIGO recommends considering treatment of acidosis when bicarbonate falls below 18 mmol/L.

| | |
|---|---|
| Category | [cmp](index.md#cmp) |
| Canonical unit | mmol/L |
| LOINC | [2028-9](https://loinc.org/2028-9/) |
| Specimen | - |
| Aliases | `bicarbonate`, `hco3`, `carbon dioxide, total` |
| Panels | [Kidney function and CKD](../panels/kidney.md) |

## Ranges

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 20 | 29 | - | - | uncited (legacy) | - |
| clinical | warn | ckd | 18 | - | guideline | Adults with chronic kidney disease | [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Practice Point 3.10.1 | KDIGO 2024: in people with CKD, consider pharmacological treatment (with or without diet) to prevent acidosis when bicarbonate is below 18 mmol/L. Monitor so that bicarbonate does not exceed the upper limit of normal. Source band <= 18; stored as the warn low bound 18. |
| optimal | optimal | ckd | 26 | 29 | cohort | Adults with non-dialysis-dependent CKD (moderate and advanced; male cohort) | [Association of serum bicarbonate levels with mortality in patients with non-dialysis-dependent CKD.](https://pubmed.ncbi.nlm.nih.gov/19015169/), Results | Lowest observed mortality in this cohort at 26-29 mmol/L (the source reports this as the lowest-risk band; it is an observational finding, not a proven treatment target). |
| alt-1 | reference | all | 22 | 30 | reference-interval-study | Healthy adults aged about 18-75 years (n = 182, both sexes), Pakistan; venous blood gas bicarbonate, 2.5-97.5 percentile (specimen: venous-blood-gas) | [Reference Intervals for Venous Blood Gas in Healthy Adults: A One-Year Prospective Study](https://pubmed.ncbi.nlm.nih.gov/39712818/), Table 2 | Venous blood gas bicarbonate, not serum total CO2 (a different specimen and method). Use for orientation only; no serum total CO2 reference interval from an open-access adult study was found in the sources checked. The source's 90 % confidence limits are 22-23 and 28-30 mmol/L. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. Alternative to the default population-ri reference range for the same population. |
| alt-1 | warn | ckd | 22 | - | cohort | Adults with non-dialysis-dependent CKD (moderate and advanced; male cohort) | [Association of serum bicarbonate levels with mortality in patients with non-dialysis-dependent CKD.](https://pubmed.ncbi.nlm.nih.gov/19015169/), Results; [Serum bicarbonate and mortality in adults in NHANES III.](https://pubmed.ncbi.nlm.nih.gov/23348878/), Results | Strictly below 22 mmol/L: highest mortality in this non-dialysis CKD cohort (multivariable HR 1.33 vs 22 or higher, per the source). Source band <= 22; stored as the warn low bound 22. Alternative to the default clinical warn range for the same population. |
| population-ri | reference | all | 18 | 37 | reference-interval-study | Healthy elderly adults (Asmara, Eritrea; CLSI C28-A3 reference interval study) | [Biochemistry reference intervals for healthy elderly population in Asmara, Eritrea.](https://pubmed.ncbi.nlm.nih.gov/29258577/), Table 1 | Reference interval for healthy elderly Eritrean adults (combined sexes, 2.5th-97.5th percentile). Broader than typical adult intervals; use with care. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 0-2 | 11.15 | 22.51 | reference-interval-study | Healthy Chinese children aged 28 days to under 2 years (Shenzhen, residual serum, CLSI EP28-A3c nonparametric method) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Serum CO2 (total carbon dioxide) in mmol/L as stated in the source (not converted). Wide interval; the source reports age-dependent values. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 2-18 | 15.83 | 23.2 | reference-interval-study | Healthy Chinese children aged 2 to under 18 years (Shenzhen, residual serum, CLSI EP28-A3c nonparametric method) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Serum CO2 (total carbon dioxide) in mmol/L as stated in the source (not converted). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
