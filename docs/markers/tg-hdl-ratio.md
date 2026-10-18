# Triglyceride/HDL-C Ratio (`tg-hdl-ratio`)

Ratio of fasting triglycerides to HDL cholesterol, used as a simple surrogate marker of insulin resistance and atherogenic dyslipidaemia. The numeric cut-off depends on the units of the two inputs (mg/dL-based or mmol/L-based ratios differ).

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | ratio |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | - | 3 | cohort | Nondiabetic overweight adults (BMI 25 kg/m2 or greater) in a general clinical research center cohort (method: mg/dL-based) | [Use of metabolic markers to identify overweight individuals who are insulin resistant.](https://pubmed.ncbi.nlm.nih.gov/14623617/), Abstract (Results) | Optimal cut-point for identifying insulin resistance (top tertile of insulin-mediated glucose disposal), mg/dL-based ratio. Derived from a cross-sectional ROC analysis, not an outcome study. Equivalent to 1.8 in SI units (mmol/L-based). Source band >= 3.0; stored as the warn high bound 3.0. |
| clinical | warn | age 13-17, obesity | - | 0.89 | cohort | Adolescents aged 13-16 years with obesity (method: mmol/L-based) | [Ratio That Reveals Risk: Defining TG:HDL-C Ratio Cutoff for Insulin Resistance in Adolescents with Obesity.](https://pubmed.ncbi.nlm.nih.gov/42650436/), Abstract | Youden-optimal cut-off for insulin resistance (HOMA-IR >= 4.0) in adolescents with obesity, mmol/L-based ratio. Cross-sectional ROC analysis in a single small sample (n = 215); not validated against outcomes and not a guideline threshold. Source band >= 0.89; stored as the warn high bound 0.89. |
| alt-1 | warn | all | - | 2 | cohort | Non-Hispanic Black adults (ethnicity-specific cut-off) | [The Triglyceride/HDL Ratio as a Surrogate Biomarker for Insulin Resistance.](https://pubmed.ncbi.nlm.nih.gov/39062066/), Introduction | Ethnicity-specific cut-off for non-Hispanic Black adults (2.0). Non-Hispanic White and Mexican American adults used 3.0 in the same comparison, as reported in the cited review. Applies only to: ethnicity=black. Source kind: decision. Source band >= 2.0; stored as the warn high bound 2.0. Alternative to the default tg-hdl-review-2024 warn range for the same population. |
| iraqi-adults-2023 | warn | all | - | 3.1 | cohort | Healthy Iraqi adults (single study, ROC best cut-off against insulin resistance) | [Triglyceride/high-density lipoprotein ratio as a predictor for insulin resistance in a sample of healthy Iraqi adults.](https://pubmed.ncbi.nlm.nih.gov/37520476/), Abstract, Results | Study-derived best cut-off (Youden index) in one sample, not a guideline threshold. Sensitivity 83% and specificity 81%. Source kind: decision. Source band >= 3.1; stored as the warn high bound 3.1. |
| tg-hdl-review-2024 | warn | all | - | 3.5 | cohort | Overweight and obese adults, as cited in a 2024 review of the TG/HDL ratio | [The Triglyceride/HDL Ratio as a Surrogate Biomarker for Insulin Resistance.](https://pubmed.ncbi.nlm.nih.gov/39062066/), Introduction | Insulin resistance cut-off of 3.5 (mg/dL units) as reported in the cited 2024 review. The review's sentence does not name the study year explicitly, so check the original McLaughlin paper before relying on the attribution. Source kind: decision. Source band >= 3.5; stored as the warn high bound 3.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
