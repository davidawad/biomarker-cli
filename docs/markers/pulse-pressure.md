# Pulse Pressure (`pulse-pressure`)

Systolic minus diastolic blood pressure. Tracked as a marker of arterial stiffness and cardiovascular risk. No population reference interval was verified for this shard; the ranges below are cut-offs from single observational studies in specific patient groups.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | mmHg |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mmHg. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | heart-failure | 30 | - | cohort | Adults admitted with heart failure (retrospective cohort, 21,768 patients, China) | [Low admission pulse pressure and increased in-hospital mortality in patients with heart failure.](https://pubmed.ncbi.nlm.nih.gov/42147065/), Abstract (Results) | Low admission pulse pressure: strictly below 30 mmHg, associated with higher in-hospital mortality. Single-study cut-off. Source band <= 30; stored as the warn low bound 30. |
| alt-1 | warn | heart-failure | - | 60 | cohort | Adults admitted with heart failure (retrospective cohort, 21,768 patients, China) | [Low admission pulse pressure and increased in-hospital mortality in patients with heart failure.](https://pubmed.ncbi.nlm.nih.gov/42147065/), Results (pulse pressure categories) | High admission pulse pressure: strictly above 60 mmHg. Study category, not a clinical threshold. Source band >= 60; stored as the warn high bound 60. Alternative to the default clinical warn range for the same population. |
| alt-1 | warn | stable-coronary-artery-disease | - | 60 | cohort | Adults with stable coronary artery disease and normal ejection fraction (prospective cohort, 7,027 patients) | [Pulse pressure associates with severity and worse outcomes in patients with stable coronary artery disease.](https://pubmed.ncbi.nlm.nih.gov/42082683/), Figure 1 legend | Highest pulse-pressure quartile (Q4) in the study cohort. Quartile boundary, not a clinical threshold. Source band >= 60; stored as the warn high bound 60. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
