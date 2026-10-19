# Urine Albumin/Creatinine Ratio (`uacr`)

Urinary albumin-to-creatinine ratio on a spot urine sample. Staging marker for albuminuria (categories A1-A3) in chronic kidney disease, and a continuous marker of kidney and cardiovascular risk.

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | mg/g |
| LOINC | - |
| Specimen | - |
| Aliases | - |
| Panels | [Diabetes and glycemic control](../panels/diabetes.md), [Kidney function and CKD](../panels/kidney.md) |

## Ranges

Bounds in mg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 30 | guideline | Adults and children with or at risk of CKD (persistent albuminuria categories) (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease | KDIGO albuminuria category A1 (normal to mildly increased), strictly below 30 mg/g. |
| clinical | warn | all | - | 300 | guideline | Adults and children with or at risk of CKD (persistent albuminuria categories) (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease | KDIGO albuminuria category A2 (moderately increased), 30 to 300 mg/g inclusive. A3 starts strictly above 300 mg/g. Source band 30-300; stored as the warn high bound 300. |
| optimal | optimal | all | - | 10 | cohort | Adults in a general population cohort (China Cardiometabolic Disease and Cancer Cohort, 82,509 participants, median follow-up 3.0 years) (specimen: spot-urine) | [Lifestyle and Metabolic Health, Low-Grade to Clinically Elevated Albuminuria, and Cardiovascular-Renal Diseases.](https://pubmed.ncbi.nlm.nih.gov/42584389/), Abstract, Results | Below 10 mg/g, the lowest-risk region of a J-shaped association with cardiorenal outcomes; risk already rises from 10 mg/g in this cohort. Not a guideline threshold: a single cohort, and the guideline cut-offs remain 30 and 300 mg/g. |
| alt-1 | reference | all | - | 26.52 | guideline | Adults and children with or at risk of CKD (persistent albuminuria categories) (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease | KDIGO albuminuria category A1 (normal to mildly increased), strictly below 3 mg/mmol (the same category as below 30 mg/g). Converted from the source's mg/mmol (3) to mg/g using factor 8.84. Alternative to the default clinical reference range for the same population. |
| alt-1 | warn | all | - | 265.2 | guideline | Adults and children with or at risk of CKD (persistent albuminuria categories) (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease | KDIGO albuminuria category A2 (moderately increased), 3 to 30 mg/mmol inclusive. A3 starts strictly above 30 mg/mmol. Converted from the source's mg/mmol (3-30) to mg/g using factor 8.84. Source band 26.52-265.2; stored as the warn high bound 265.2. Alternative to the default clinical warn range for the same population. |
| alt-1 | warn | all | - | 265.2 | guideline | Adults and children with or at risk of CKD (persistent albuminuria categories) (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease | KDIGO albuminuria category A3 (severely increased), strictly above 30 mg/mmol. Converted from the source's mg/mmol (30) to mg/g using factor 8.84. Source kind: diagnostic. Source band >= 265.2; stored as the warn high bound 265.2. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 300 | guideline | Adults and children with or at risk of CKD (persistent albuminuria categories) (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease | KDIGO albuminuria category A3 (severely increased), strictly above 300 mg/g. Source kind: diagnostic. Source band >= 300; stored as the warn high bound 300. |
| diagnostic | warn | female | - | 30.9 | guideline | Women; historical sex-specific ACR criterion (superseded by a unified 30 mg/g threshold in KDIGO 2012) (specimen: spot-urine) | [Developments in albuminuria testing: A key biomarker for detection, prognosis and surveillance of kidney and cardiovascular disease-A practical update for clinicians.](https://pubmed.ncbi.nlm.nih.gov/40143452/), Sex-specific ACR cut-offs (text) | Historical sex-specific abnormal ACR cut-off for women (about 3.5 mg/mmol). Superseded; kept only to document the sex-specific partition. Do not use as a current diagnostic threshold. Source kind: diagnostic. Source band >= 30.9; stored as the warn high bound 30.9. |
| diagnostic | warn | male | - | 22.1 | guideline | Men; historical sex-specific ACR criterion (superseded by a unified 30 mg/g threshold in KDIGO 2012) (specimen: spot-urine) | [Developments in albuminuria testing: A key biomarker for detection, prognosis and surveillance of kidney and cardiovascular disease-A practical update for clinicians.](https://pubmed.ncbi.nlm.nih.gov/40143452/), Sex-specific ACR cut-offs (text) | Historical sex-specific abnormal ACR cut-off for men (about 2.5 mg/mmol). Superseded; kept only to document the sex-specific partition. Do not use as a current diagnostic threshold. Source kind: diagnostic. Source band >= 22.1; stored as the warn high bound 22.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
