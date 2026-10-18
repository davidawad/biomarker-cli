# QUICKI (Quantitative Insulin Sensitivity Check Index) (`quicki`)

Computed insulin sensitivity index from fasting insulin and fasting glucose: 1/[log(fasting insulin) + log(fasting glucose)]. Lower values indicate insulin resistance. Expected result: no single established normal range; interpret against study-specific cut-offs.

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | index |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: Computed index with no established reference interval; the only range below is a single-study cut-off for metabolic syndrome classification.

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| diagnostic | warn | metabolic-syndrome | 0.32 | - | cohort | Southern Indian adults aged 18 to 65 years (75 with metabolic syndrome, 75 controls) | [Cut-off Values and Clinical Utility of Surrogate Markers for Insulin Resistance and Beta-Cell Function to Identify Metabolic Syndrome and Its Components among Southern Indian Adults.](https://pubmed.ncbi.nlm.nih.gov/33229629/), Discussion (cut-off values) | Cut-off for identifying metabolic syndrome (value at or below 0.32 flagged), derived by ROC analysis in one cross-sectional study. Single study; not a general insulin-resistance cut-off. Source kind: diagnostic. Source band <= 0.32; stored as the warn low bound 0.32. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
