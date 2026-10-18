# Urine Albumin (`urine-albumin`)

Albumin excreted in urine, measured as the spot urine albumin-to-creatinine ratio (ACR) in mg/g for CKD staging and diabetic kidney disease screening. Albuminuria is the marker of kidney damage used with GFR to define CKD (KDIGO albuminuria categories A1-A3). Expected result for a healthy adult: below 30 mg/g (A1). Ranges below apply to the ACR, not to the raw urine albumin concentration (mg/L).

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | mg/g |
| LOINC | [1754-1](https://loinc.org/1754-1/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 30 | guideline | Adults; spot urine albumin-to-creatinine ratio | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (albuminuria categories in CKD) | Albuminuria category A1 (normal to mildly increased), strictly below 30 mg/g. Category A1 does not by itself define CKD. |
| clinical | warn | all | - | 300 | guideline | Adults; spot urine albumin-to-creatinine ratio | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (albuminuria categories in CKD) | Albuminuria category A2 (moderately increased), 30 to 300 mg/g. Relative to the young adult level, per KDIGO footnote. Source band 30-300; stored as the warn high bound 300. |
| diagnostic | warn | all | - | 300 | guideline | Adults; spot urine albumin-to-creatinine ratio | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 2 (albuminuria categories in CKD) | Albuminuria category A3 (severely increased), strictly above 300 mg/g. Source kind: diagnostic. Source band >= 300; stored as the warn high bound 300. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
