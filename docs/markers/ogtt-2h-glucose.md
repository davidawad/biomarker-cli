# Glucose, 2-hour OGTT (75 g) (`ogtt-2h-glucose`)

Plasma glucose 2 hours after a 75 g oral glucose load. Used to classify impaired glucose tolerance and diabetes in adults, and gestational diabetes in pregnancy (IADPSG thresholds).

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | mg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | `glucose-ogtt-2h` |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 140 | guideline | Adults without a previous diabetes diagnosis | [The impact of glucose and metabolic disturbances on white matter hyperintensity volume in apparently healthy adults.](https://pubmed.ncbi.nlm.nih.gov/41928882/), Table 1 | Normal glucose tolerance: 2-hour glucose strictly below 140 mg/dL. |
| clinical | warn | all | - | 200 | guideline | Adults without a previous diabetes diagnosis | [The impact of glucose and metabolic disturbances on white matter hyperintensity volume in apparently healthy adults.](https://pubmed.ncbi.nlm.nih.gov/41928882/), Table 1 | Impaired glucose tolerance (prediabetes) by ADA criteria. Source band 140-200; stored as the warn high bound 200. |
| diagnostic | warn | all | - | 200 | guideline | Adults, 75 g OGTT | [The impact of glucose and metabolic disturbances on white matter hyperintensity volume in apparently healthy adults.](https://pubmed.ncbi.nlm.nih.gov/41928882/), Table 1 | Diabetes by 2-hour glucose during a 75 g OGTT (ADA). Source kind: diagnostic. Source band >= 200; stored as the warn high bound 200. |
| diagnostic | warn | pregnant | - | 153 | consensus | Pregnant women, 75 g OGTT (IADPSG criteria) | [International association of diabetes and pregnancy study groups recommendations on the diagnosis and classification of hyperglycemia in pregnancy.](https://pubmed.ncbi.nlm.nih.gov/20190296/), Threshold table (consensus thresholds) | Gestational diabetes if any one 75 g OGTT value meets or exceeds the threshold (IADPSG 2010). Source kind: diagnostic. Source band >= 153; stored as the warn high bound 153. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
