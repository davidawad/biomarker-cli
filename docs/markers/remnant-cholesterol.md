# Remnant Cholesterol (`remnant-cholesterol`)

Cholesterol carried in triglyceride-rich remnant lipoproteins (IDL and VLDL remnants). Calculated as non-HDL cholesterol minus LDL cholesterol, or measured directly. Associated with ischemic heart disease and mortality.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | mg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | - |
| Panels | [Cardiovascular risk (lipids and risk enhancers)](../panels/cardiovascular-risk.md) |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | non-fasting | - | 34.8 | consensus | Adults, non-fasting samples | [Fasting is not routinely required for determination of a lipid profile: clinical and laboratory implications including flagging at desirable concentration cut-points-a joint consensus statement from the European Atherosclerosis Society and European Federation of Clinical Chemistry and Laboratory Medicine.](https://pubmed.ncbi.nlm.nih.gov/27122601/), Abstract (laboratory flagging recommendation) | EAS/EFLM 2016 laboratory flag for calculated remnant cholesterol (35 mg/dL). The same consensus gives no remnant-cholesterol flag for fasting samples. Converted from the source's mmol/L (0.9) to mg/dL using factor 38.67. Source band >= 34.8; stored as the warn high bound 34.8. |
| optimal | optimal | non-fasting | - | 19.34 | cohort | Adults from the Copenhagen General Population Study (non-fasting measurement) | [Extreme nonfasting remnant cholesterol vs extreme LDL cholesterol as contributors to cardiovascular disease and all-cause mortality in 90000 individuals from the general population.](https://pubmed.ncbi.nlm.nih.gov/25605681/), Results | Lowest-risk category in the Copenhagen cohort: non-fasting remnant cholesterol below 0.5 mmol/L (19.3 mg/dL). Used as the reference group for hazard ratios. Converted from the source's mmol/L (0.5) to mg/dL using factor 38.67. |
| alt-1 | warn | non-fasting | - | 58.01 | cohort | Adults from the Copenhagen General Population Study (non-fasting measurement) | [Extreme nonfasting remnant cholesterol vs extreme LDL cholesterol as contributors to cardiovascular disease and all-cause mortality in 90000 individuals from the general population.](https://pubmed.ncbi.nlm.nih.gov/25605681/), Results | Highest-risk category in the Copenhagen cohort: non-fasting remnant cholesterol of 1.5 mmol/L (58 mg/dL) or more, hazard ratio 2.4 for ischemic heart disease versus the lowest category. Not a guideline cut-off. Converted from the source's mmol/L (1.5) to mg/dL using factor 38.67. Source band >= 58.01; stored as the warn high bound 58.01. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
