# Glucose Coefficient of Variation (CGM) (`cgm-coefficient-of-variation`)

The coefficient of variation (CV) of glucose readings from a continuous glucose monitor, calculated as the standard deviation divided by the mean glucose. It measures glycaemic variability, with lower values indicating more stable glucose levels.

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | `glucose cv`, `%cv`, `glycemic variability cv`, `glycaemic variability` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | optimal | all | - | 33 | consensus | People with diabetes receiving insulin or sulfonylurea therapy (hypoglycaemia risk) | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Full text, clinical targets table, footnote | Lower CV target of below 33 percent (boundary < 33). The consensus footnote says some studies suggest this gives additional protection against hypoglycaemia for people on insulin or sulfonylureas. Presented as a suggestion from studies, not a firm target. Alternative to the default battelino-2019-tir-cv optimal range for the same population. |
| battelino-2019-tir-cv | optimal | all | - | 36 | consensus | Adults and children with diabetes using continuous glucose monitoring (international consensus on time in range) | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Full text, clinical targets table, Glycemic variability row | Glycaemic variability target: CV at or below 36 percent (boundary <= 36). Consensus target for people with diabetes; not validated as a target for people without diabetes. |
| hgv-cvw-2021 | warn | all | - | 36 | cohort | Adults with type 1 diabetes; within-day CV (%CVw) computed from 14 days of CGM data | [Is the Consensual Threshold for Defining High Glucose Variability Implementable in Clinical Practice?](https://pubmed.ncbi.nlm.nih.gov/34099517/), Abstract, Objective | High glycaemic variability is defined as CV above 36 percent (boundary > 36). The source found that CGM devices report only total CV, which can exceed 36 percent while within-day CV stays at or below 36 percent, so a total CV above 36 percent is not always high within-day variability. Source band >= 36; stored as the warn high bound 36. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
