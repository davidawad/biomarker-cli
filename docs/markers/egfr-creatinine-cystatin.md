# eGFR (Creatinine + Cystatin C, CKD-EPI 2021) (`egfr-creatinine-cystatin`)

Estimated glomerular filtration rate from the race-free CKD-EPI 2021 creatinine-cystatin C equation (Inker et al., NEJM 2021), reported in mL/min/1.73 m2. Combining creatinine and cystatin C is more accurate than creatinine alone and is used to confirm GFR when creatinine-based estimates are unreliable. Used to classify GFR categories G1-G5 and, with albuminuria, to define chronic kidney disease. Expected result for a healthy adult: 90 or higher (G1).

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | mL/min/1.73m² |
| LOINC | [98980-6](https://loinc.org/98980-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mL/min/1.73m². The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 90 | - | guideline | Adults; GFR estimated by the creatinine-cystatin C equation | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (GFR categories in CKD) | GFR category G1 (normal or high). CKD requires markers of kidney damage in addition to GFR category. |
| clinical | warn | all | 60 | - | guideline | Adults; GFR estimated by the creatinine-cystatin C equation | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (GFR categories in CKD) | GFR category G2 (mildly decreased), 60 to strictly below 90. CKD is diagnosed only with markers of kidney damage or GFR below 60 for more than 3 months. Source band 60-90; stored as the warn low bound 60. |
| alt-1 | warn | all | 30 | - | guideline | Adults; GFR estimated by the creatinine-cystatin C equation | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (GFR categories in CKD) | GFR category G3b (moderately to severely decreased), 30 to strictly below 45. Source kind: diagnostic. Source band 30-45; stored as the warn low bound 30. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | warn | all | 15 | - | guideline | Adults; GFR estimated by the creatinine-cystatin C equation | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (GFR categories in CKD) | GFR category G4 (severely decreased), 15 to strictly below 30. Source kind: diagnostic. Source band 15-30; stored as the warn low bound 15. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | warn | all | 15 | - | guideline | Adults; GFR estimated by the creatinine-cystatin C equation | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (GFR categories in CKD) | GFR category G5 (kidney failure), strictly below 15. Source kind: diagnostic. Source band <= 15; stored as the warn low bound 15. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | 45 | - | guideline | Adults; GFR estimated by the creatinine-cystatin C equation | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (GFR categories in CKD) | GFR category G3a (mildly to moderately decreased), 45 to strictly below 60. CKD when GFR is below 60 for more than 3 months. Source kind: diagnostic. Source band 45-60; stored as the warn low bound 45. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
