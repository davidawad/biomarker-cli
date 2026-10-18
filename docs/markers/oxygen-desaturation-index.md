# Oxygen Desaturation Index (`oxygen-desaturation-index`)

Number of oxygen desaturations of 4 percent or more per hour of recording, from pulse oximetry. Used as a screening and diagnostic index for obstructive sleep apnea (OSA); it is compared against the apnea-hypopnea index (AHI). Cut-offs below are from diagnostic-accuracy studies, not population reference intervals.

| | |
|---|---|
| Category | [sleep](index.md#sleep) |
| Canonical unit | events/h |
| LOINC | [90567-9](https://loinc.org/90567-9/) |
| Specimen | - |
| Aliases | `odi`, `odi-4` |

## Ranges

Bounds in events/h. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | obstructive-sleep-apnea | - | 10 | cohort | Adults with suspected obstructive sleep apnea (8 studies, 1,924 patients) | [The Value of Oxygen Desaturation Index for Diagnosing Obstructive Sleep Apnea: A Systematic Review.](https://pubmed.ncbi.nlm.nih.gov/32333683/), Abstract, Conclusion | Value at which the systematic review recommends further evaluation for OSA (4 percent ODI). Upper bound not stated in the source, so none is given here. Source band >= 10; stored as the warn high bound 10. |
| clinical | reference | obstructive-sleep-apnea | - | 5 | cohort | Portable monitoring recordings for suspected obstructive sleep apnea (3,413 recordings; 2,281 training, 1,132 test) | [Validation of the oxygen desaturation index in the diagnostic workup of obstructive sleep apnea.](https://pubmed.ncbi.nlm.nih.gov/29564732/), Abstract, Conclusion | Rule-out threshold: ODI strictly below 5 predicts AHI strictly below 5 (AHI under 5 events/h). It is a validated cut-off for excluding AHI of 5 or more, not a population reference interval. |
| diagnostic | warn | obstructive-sleep-apnea | - | 15 | cohort | Adults with suspected obstructive sleep apnea (8 studies, 1,924 patients) | [The Value of Oxygen Desaturation Index for Diagnosing Obstructive Sleep Apnea: A Systematic Review.](https://pubmed.ncbi.nlm.nih.gov/32333683/), Abstract, Conclusion | Systematic review (no pooled estimate) of 4 percent ODI against AHI. The review states that consideration should be given to diagnosing adult OSA at this value, and that further study is needed before a definitive recommendation. Source kind: diagnostic. Source band >= 15; stored as the warn high bound 15. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
