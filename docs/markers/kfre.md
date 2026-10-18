# Kidney Failure Risk Equation (KFRE) (`kfre`)

Calculated absolute risk of kidney failure (requiring kidney replacement therapy) over 2 years and 5 years, from an externally validated kidney failure risk equation (KFRE). Expressed as a percentage. KDIGO 2024 recommends using such an equation in people with CKD G3-G5 and uses risk thresholds to time nephrology referral and kidney replacement planning. Not applicable outside CKD G3-G5; no normal range applies.

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | % |
| LOINC | [85626-0](https://loinc.org/85626-0/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | ckd | - | 10 | guideline | People with CKD G3-G5; 2-year kidney failure risk | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Practice Point 2.2.2; Figure 15 | 2-year kidney failure risk strictly above 10% (KDIGO 2024 Practice Point 2.2.2): a threshold for timing multidisciplinary care. Figure 15 labels it KF risk >=10% at 2 years. Source band >= 10; stored as the warn high bound 10. |
| clinical | reference | ckd | 3 | 5 | guideline | People with CKD G3-G5; 5-year kidney failure risk | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Practice Point 2.2.1; Figure 15 | 5-year kidney failure risk of 3%-5% (KDIGO 2024 Practice Point 2.2.1): a threshold for nephrology referral in addition to eGFR and ACR criteria. Figure 15 labels the threshold as KF risk >=3%-5% over 5 years. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | ckd | - | 40 | guideline | People with CKD G3-G5; 2-year kidney failure risk | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Practice Point 2.2.3; Figure 15 | 2-year kidney failure risk strictly above 40% (KDIGO 2024 Practice Point 2.2.3): a threshold for kidney replacement therapy modality education, preparation and transplant referral. Figure 15 labels it KF risk >=40% at 2 years. Source band >= 40; stored as the warn high bound 40. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
