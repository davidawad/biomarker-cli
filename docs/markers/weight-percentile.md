# Body Weight Percentile for Age (`weight-percentile`)

Body weight expressed as a percentile for the child's age. Low values flag underweight; high values flag overweight or obesity. Category cut-offs are those operationally defined in the cited cohort.

| | |
|---|---|
| Category | [pediatric](index.md#pediatric) |
| Canonical unit | percentile |
| LOINC | [8336-0](https://loinc.org/8336-0/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in percentile. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 0-18 | 5 | 85 | cohort | Children under 18 years, emergency department cohort (1432 patients, tertiary children's hospital) | [Weight-for-Age Percentile as a Pediatric Predictor of Emergency Department Outcome.](https://pubmed.ncbi.nlm.nih.gov/31598543/), Methods, Group Assignments and Operational Definitions | Normal category: weight-for-age percentile from the 5th to the 85th. Cut-off as operationally defined in the cited retrospective cohort. |
| clinical | warn | age 0-18 | - | 95 | cohort | Children under 18 years, emergency department cohort (1432 patients, tertiary children's hospital) | [Weight-for-Age Percentile as a Pediatric Predictor of Emergency Department Outcome.](https://pubmed.ncbi.nlm.nih.gov/31598543/), Methods, Group Assignments and Operational Definitions | Overweight category: weight-for-age percentile from the 85th to below the 95th. Cut-off as operationally defined in the cited retrospective cohort. Source band 85-95; stored as the warn high bound 95. |
| alt-1 | warn | age 0-18 | - | 95 | cohort | Children under 18 years, emergency department cohort (1432 patients, tertiary children's hospital) | [Weight-for-Age Percentile as a Pediatric Predictor of Emergency Department Outcome.](https://pubmed.ncbi.nlm.nih.gov/31598543/), Methods, Group Assignments and Operational Definitions | Obese category: weight-for-age percentile greater than the 95th. Cut-off as operationally defined in the cited retrospective cohort. Source kind: diagnostic. Source band >= 95; stored as the warn high bound 95. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | age 0-18 | 5 | - | cohort | Children under 18 years, emergency department cohort (1432 patients, tertiary children's hospital) | [Weight-for-Age Percentile as a Pediatric Predictor of Emergency Department Outcome.](https://pubmed.ncbi.nlm.nih.gov/31598543/), Methods, Group Assignments and Operational Definitions | Underweight category: weight-for-age percentile less than the 5th. Cut-off as operationally defined in the cited retrospective cohort. Source kind: diagnostic. Source band <= 5; stored as the warn low bound 5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
