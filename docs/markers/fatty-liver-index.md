# Fatty Liver Index (`fatty-liver-index`)

Score from 0 to 100 calculated from BMI, waist circumference, triglycerides and GGT that estimates the likelihood of fatty liver on ultrasound. A non-imaging screen, not a diagnosis. Below 30 rules out fatty liver; 60 or above rules it in.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | index |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 30 | cohort | General population adults (Dionysos Nutrition & Liver Study), ultrasound-defined fatty liver | [The Fatty Liver Index: a simple and accurate predictor of hepatic steatosis in the general population.](https://pubmed.ncbi.nlm.nih.gov/17081293/), Abstract (Results) | Strictly below 30: rules out fatty liver (negative likelihood ratio 0.2). |
| clinical | warn | all | - | 60 | cohort | General population adults (Dionysos Nutrition & Liver Study), ultrasound-defined fatty liver | [The Fatty Liver Index: a simple and accurate predictor of hepatic steatosis in the general population.](https://pubmed.ncbi.nlm.nih.gov/17081293/), Abstract (Results) | 30 up to 60: between the rule-out and rule-in cut-offs. The source gives no classification for this band. Source band 30-60; stored as the warn high bound 60. |
| diagnostic | warn | all | - | 60 | cohort | General population adults (Dionysos Nutrition & Liver Study), ultrasound-defined fatty liver | [The Fatty Liver Index: a simple and accurate predictor of hepatic steatosis in the general population.](https://pubmed.ncbi.nlm.nih.gov/17081293/), Abstract (Results) | 60 or above: rules in fatty liver (positive likelihood ratio 4.3). Source kind: diagnostic. Source band >= 60; stored as the warn high bound 60. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
