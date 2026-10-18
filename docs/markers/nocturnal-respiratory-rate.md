# Nocturnal Respiratory Rate (`nocturnal-respiratory-rate`)

Mean breathing rate during sleep, measured by polysomnography, chest belts, Holter waveforms or consumer wearables. Tracked as a cardiopulmonary risk marker and as a possible early-warning signal. No verified adult reference interval was found in the sources checked; the cited cut-offs are 16 breaths/min (older adults, mortality risk) and 18 breaths/min (cardiomyopathy, ICD benefit).

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | breaths/min |
| LOINC | [9286-6](https://loinc.org/9286-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in breaths/min. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | - | 16 | cohort | Community-dwelling older men and women (MrOS and SOF cohorts), overnight polysomnography | [Mean nocturnal respiratory rate predicts cardiovascular and all-cause mortality in community-dwelling older men and women.](https://pubmed.ncbi.nlm.nih.gov/31151958/), Abstract (Conclusion) | Prognostic cut-off (dichotomised mean nocturnal respiratory rate at 16 breaths/min) for long-term cardiovascular and all-cause mortality. Not a diagnostic threshold. Source band >= 16; stored as the warn high bound 16. |
| clinical | warn | cardiomyopathy | - | 18 | cohort | Patients with ischaemic or non-ischaemic cardiomyopathy meeting primary prophylactic ICD implantation criteria (EU-CERT-ICD sub-study), nocturnal respiratory rate from pre-implantation 24-hour Holter | [Nocturnal respiratory rate predicts ICD benefit: A prospective, controlled, multicentre cohort study.](https://pubmed.ncbi.nlm.nih.gov/33554086/), Interpretation | Threshold at which the mortality benefit of prophylactic ICD implantation was not demonstrated (interaction analysis, not a diagnostic value). Source band >= 18; stored as the warn high bound 18. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
