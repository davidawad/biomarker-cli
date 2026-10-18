# Resting Heart Rate (`resting-hr`)

Heart rate measured at rest, in beats per minute. Higher resting rates in the general population are associated with greater all-cause and cardiovascular mortality.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | bpm |
| LOINC | [40443-4](https://loinc.org/40443-4/) |
| Specimen | - |
| Aliases | `resting heart rate`, `rhr`, `resting hr` |

## Ranges

Bounds in bpm. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 60 | 100 | - | - | uncited (legacy) | - |
| cmaj-2016-rhr-mortality | warn | all | - | 80 | cohort | General population adults in prospective cohorts (meta-analysis of 46 studies) | [Resting heart rate and all-cause and cardiovascular mortality in the general population: a meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/26598376/), Abstract, Results | Resting heart rate above 80 beats/min: relative risk 1.45 for all-cause mortality compared with the lowest category in the meta-analysis. The 80 bpm cut-off is the one stated by the source; the boundary is read as strictly greater than 80. Source band >= 80; stored as the warn high bound 80. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
