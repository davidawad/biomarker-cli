# METS-IR (Metabolic Score for Insulin Resistance) (`mets-ir`)

Computed, non-insulin-based insulin resistance score from fasting glucose, triglycerides, HDL-C and BMI. Tracked as a surrogate for insulin resistance and cardiometabolic risk. Expected result: no established normal range; interpret against study-specific cut-offs.

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | index |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: Computed score with no established reference interval; the ranges below are single-study or pooled cut-offs for specific outcomes, not normal limits.

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | pregnant, preeclampsia | - | 38.644 | cohort | Singleton pregnant women delivering at participating hospitals (multicenter cohort) | [Association of metabolic score for insulin resistance and preeclampsia: a multicenter cohort study.](https://pubmed.ncbi.nlm.nih.gov/41094658/), Abstract (Results) | ROC-optimal cut-off for predicting preeclampsia in one cohort (AUC 0.649). Predictive threshold, not a diagnostic criterion. Source band >= 38.644; stored as the warn high bound 38.644. |
| clinical | warn | diabetes | - | 33.33 | cohort | Adults with diabetes (cohort study) | [Association Between a Novel Metabolic Score for Insulin Resistance and Mortality in People With Diabetes.](https://pubmed.ncbi.nlm.nih.gov/35647046/), Abstract (Results) | Threshold-effect inflection for all-cause and CVD mortality; risk is associated only above this value per the source. Source band >= 33.33; stored as the warn high bound 33.33. |
| clinical | warn | all | - | 38.24 | cohort | Adults in pooled cohort studies of cardiovascular outcomes | [Metabolic score for insulin resistance and the incidence of cardiovascular disease: a meta-analysis of cohort studies.](https://pubmed.ncbi.nlm.nih.gov/41180178/), Results, dose-response analysis (Figure 8C) | Inflection point of the pooled dose-response curve (Figure 8C); risk rises beyond it per the source. Not a diagnostic cut-off. Source band >= 38.24; stored as the warn high bound 38.24. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
