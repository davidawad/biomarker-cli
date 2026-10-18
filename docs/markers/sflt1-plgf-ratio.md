# sFlt-1/PlGF Ratio (`sflt1-plgf-ratio`)

Ratio of soluble fms-like tyrosine kinase-1 (sFlt-1) to placental growth factor (PlGF), an angiogenic marker used in suspected pre-eclampsia. A low ratio makes pre-eclampsia within about one week unlikely; a high ratio signals imminent or early-onset disease. Cut-offs are assay-specific.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | ratio |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | pregnant, high-blood-pressure-in-pregnancy | - | 95 | cohort | Chinese pregnant women with high blood pressure presenting at 24+0 to 38+6 weeks, validation cohort (method: immunoassay) | [sFlt-1/PlGF ratio thresholds for diagnosing pre-eclampsia in pregnant women with high blood pressure.](https://pubmed.ncbi.nlm.nih.gov/40992360/), Abstract, Results; [sFlt-1/PlGF ratio thresholds for diagnosing pre-eclampsia in pregnant women with high blood pressure.](https://pubmed.ncbi.nlm.nih.gov/40992360/), Abstract, Results | Late-onset pre-eclampsia (gestational age 34 weeks or more) diagnosed within 1 week of biomarker measurement. Lower sensitivity (36.5%) than the early-onset cut-off. Applies only to: gestational_week_min=34; gestational_week_max=38. Source kind: diagnostic. Source band >= 95; stored as the warn high bound 95. Alternative to the default uog-2025-chinese-cohort warn range for the same population. |
| prognosis-2016 | warn | pregnant, suspected-preeclampsia | 38 | - | cohort | Women with singleton pregnancies at 24 weeks 0 days to 36 weeks 6 days with suspected pre-eclampsia (PROGNOSIS development and validation cohorts, 1050 women); gestational_week_max counts completed weeks (to 36+6) (method: immunoassay) | [Predictive Value of the sFlt-1:PlGF Ratio in Women with Suspected Preeclampsia.](https://pubmed.ncbi.nlm.nih.gov/26735990/), Abstract, Conclusions | Rule-out: a ratio of 38 or lower predicts no pre-eclampsia within the next week (negative predictive value 99.3%). A ratio above 38 has a positive predictive value of 36.7% for pre-eclampsia within 4 weeks. Applies only to: gestational_week_min=24; gestational_week_max=36. Source kind: diagnostic. Source band <= 38; stored as the warn low bound 38. |
| prognosis-asia-2021 | warn | pregnant, suspected-preeclampsia | - | 38 | cohort | Chinese pregnant women with suspected pre-eclampsia (PROGNOSIS Asia sub-analysis) (method: immunoassay) | [Short-Term Prediction of Preeclampsia in Chinese Women Using the Soluble fms-Like Tyrosine Kinase 1/Placental Growth Factor Ratio: A Sub-Analysis of the PROGNOSIS Asia Study.](https://pubmed.ncbi.nlm.nih.gov/34497833/), Abstract, Results | Rule-in cut-off: a ratio above 38 is used to rule in pre-eclampsia within 4 weeks. In this cohort the positive predictive value was modest (35.0%), so a high ratio warns rather than confirms. Source kind: diagnostic. Source band >= 38; stored as the warn high bound 38. |
| prognosis-study | warn | pregnant, suspected-preeclampsia | 38 | - | cohort | Pregnant women with suspected pre-eclampsia in the PROGNOSIS study cohort (Elecsys sFlt-1/PlGF assay) (method: immunoassay) | [Soluble fms-like tyrosine kinase-1 to placental growth factor ratio: ruling out pre-eclampsia for up to 4 weeks and value of retesting.](https://pubmed.ncbi.nlm.nih.gov/30014562/), Abstract | Rule-out cut-off: a ratio at or below 38 rules out pre-eclampsia onset within 1 week of testing (high negative predictive value). Platform-specific; other platforms use different equivalent cut-offs. Source kind: diagnostic. Source band <= 38; stored as the warn low bound 38. |
| uog-2025-chinese-cohort | warn | pregnant, high-blood-pressure-in-pregnancy | - | 74 | cohort | Chinese pregnant women with high blood pressure presenting at 24+0 to 38+6 weeks, validation cohort (method: immunoassay) | [sFlt-1/PlGF ratio thresholds for diagnosing pre-eclampsia in pregnant women with high blood pressure.](https://pubmed.ncbi.nlm.nih.gov/40992360/), Abstract, Results; [sFlt-1/PlGF ratio thresholds for diagnosing pre-eclampsia in pregnant women with high blood pressure.](https://pubmed.ncbi.nlm.nih.gov/40992360/), Abstract, Results | Early-onset pre-eclampsia (gestational age under 34 weeks) diagnosed within 1 week of biomarker measurement. Ratio at or above 74 predicts early-onset disease; the source does not give a 85 cut-off. Applies only to: gestational_week_min=24; gestational_week_max=33. Source kind: diagnostic. Source band >= 74; stored as the warn high bound 74. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
