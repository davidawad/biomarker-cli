# LDL Cholesterol (direct) (`ldl-c-direct`)

Cholesterol carried in low-density lipoprotein particles, measured directly by laboratory assay rather than calculated from the lipid profile. The primary lipid target of cholesterol-lowering therapy. Used to classify ATP III LDL-C categories and to set LDL-lowering goals by cardiovascular risk. Expected result for a healthy adult: below 100 mg/dL (optimal).

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | mg/dL |
| LOINC | [18262-6](https://loinc.org/18262-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 20+ | 100 | 130 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | Near optimal/above optimal, 100 to 129 mg/dL (ATP III). Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| optimal | optimal | age 20+ | - | 100 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | Optimal: strictly below 100 mg/dL (ATP III). |
| alt-1 | optimal | all | - | 130 | guideline | Adults with two or more risk factors and 10-year CHD risk of 20% or less | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 4 | ATP III LDL-C goal (strictly below 130 mg/dL) for two or more risk factors. Applies only to: risk=two-or-more-risk-factors. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-2 | optimal | all | - | 160 | guideline | Adults with zero to one risk factor | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 4 | ATP III LDL-C goal (strictly below 160 mg/dL) for zero to one risk factor. Applies only to: risk=zero-to-one-risk-factor. Source kind: target. Alternative to the default target optimal range for the same population. |
| diagnostic | warn | age 20+ | - | 190 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | Very high, 190 mg/dL or above (ATP III). Source kind: diagnostic. Source band >= 190; stored as the warn high bound 190. |
| target | optimal | all | - | 100 | guideline | Adults with coronary heart disease or CHD risk equivalents | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 4 | ATP III LDL-C goal (strictly below 100 mg/dL) for CHD and CHD risk equivalents. Applies only to: risk=chd-or-risk-equivalent. Source kind: target. |
| target | optimal | established-cvd | - | 77.34 | guideline | Adults with established cardiovascular disease (secondary prevention) | [NICE NG238 recommendations](https://www.nice.org.uk/guidance/ng238/chapter/recommendations), Recommendation 1.7.1 | NICE NG238 secondary-prevention LDL-C target, stated in mmol/L; numbers kept as the source states them, not converted. Converted from the source's mmol/L (2.0) to mg/dL using factor 38.67. Source kind: target. |
| ldl-c-direct-bands-130-160 | reference | age 20+ | 130 | 160 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | Borderline high, 130 to 159 mg/dL (ATP III). Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| ldl-c-direct-bands-160-190 | reference | age 20+ | 160 | 190 | guideline | Adults aged 20 years or older | [ATP III Executive Summary (NHLBI)](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | High, 160 to 189 mg/dL (ATP III). Source kind: diagnostic. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
