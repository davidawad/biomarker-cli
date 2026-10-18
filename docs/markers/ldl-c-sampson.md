# LDL Cholesterol, Sampson/NIH Equation Calculated (`ldl-c-sampson`)

LDL cholesterol calculated from the standard lipid profile by the Sampson/NIH equation (Sampson et al., JAMA Cardiology 2020). The equation estimates very-LDL cholesterol from triglycerides and non-HDL-C, then subtracts it to give LDL-C. It was derived in NIH Clinical Center patients and is intended for normolipidemia and hypertriglyceridemia, including low LDL-C. Calculated, not measured. Used for the same ATP III LDL-C categories and goals as measured LDL-C. Expected result for a healthy adult: below 100 mg/dL (optimal).

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | mg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 20+ | 100 | 130 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2; [A New Equation for Calculation of Low-Density Lipoprotein Cholesterol in Patients With Normolipidemia and/or Hypertriglyceridemia.](https://pubmed.ncbi.nlm.nih.gov/32101259/), Abstract | Near optimal/above optimal, 100 to 129 mg/dL (ATP III), applied to calculated LDL-C. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| optimal | optimal | age 20+ | - | 100 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2; [A New Equation for Calculation of Low-Density Lipoprotein Cholesterol in Patients With Normolipidemia and/or Hypertriglyceridemia.](https://pubmed.ncbi.nlm.nih.gov/32101259/), Abstract | Optimal: strictly below 100 mg/dL (ATP III), applied to calculated LDL-C. |
| alt-1 | optimal | all | - | 130 | guideline | Adults with two or more risk factors and 10-year CHD risk of 20% or less | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 4 | ATP III LDL-C goal (strictly below 130 mg/dL) for two or more risk factors, applied to calculated LDL-C. Applies only to: risk=two-or-more-risk-factors. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-2 | optimal | all | - | 160 | guideline | Adults with zero to one risk factor | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 4 | ATP III LDL-C goal (strictly below 160 mg/dL) for zero to one risk factor, applied to calculated LDL-C. Applies only to: risk=zero-to-one-risk-factor. Source kind: target. Alternative to the default target optimal range for the same population. |
| diagnostic | warn | age 20+ | - | 190 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2; [A New Equation for Calculation of Low-Density Lipoprotein Cholesterol in Patients With Normolipidemia and/or Hypertriglyceridemia.](https://pubmed.ncbi.nlm.nih.gov/32101259/), Abstract | Very high, 190 mg/dL or above (ATP III), applied to calculated LDL-C. Source kind: diagnostic. Source band >= 190; stored as the warn high bound 190. |
| target | optimal | all | - | 100 | guideline | Adults with coronary heart disease or CHD risk equivalents | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 4 | ATP III LDL-C goal (strictly below 100 mg/dL) for CHD and CHD risk equivalents, applied to calculated LDL-C. Applies only to: risk=chd-or-risk-equivalent. Source kind: target. |
| ldl-c-sampson-bands-130-160 | reference | age 20+ | 130 | 160 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2; [A New Equation for Calculation of Low-Density Lipoprotein Cholesterol in Patients With Normolipidemia and/or Hypertriglyceridemia.](https://pubmed.ncbi.nlm.nih.gov/32101259/), Abstract | Borderline high, 130 to 159 mg/dL (ATP III), applied to calculated LDL-C. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| ldl-c-sampson-bands-160-190 | reference | age 20+ | 160 | 190 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2; [A New Equation for Calculation of Low-Density Lipoprotein Cholesterol in Patients With Normolipidemia and/or Hypertriglyceridemia.](https://pubmed.ncbi.nlm.nih.gov/32101259/), Abstract | High, 160 to 189 mg/dL (ATP III), applied to calculated LDL-C. Source kind: diagnostic. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
