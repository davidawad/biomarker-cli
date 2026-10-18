# Heart Rate Recovery (1 min) (`cardio-recovery`)

Fall in heart rate during the first minute after stopping exercise, computed as peak exercise heart rate minus the heart rate one minute after exercise ends. Tracked as a marker of vagal reactivation and cardiovascular prognosis. Expected result: a fall greater than 12 bpm; a fall of 12 bpm or less is classed as abnormal in the cited cohort.

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | bpm |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in bpm. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | 12 | - | cohort | Adults without a history of heart failure or coronary revascularization and without pacemakers, undergoing symptom-limited exercise testing (mean age 57 years, 63 percent men) | [Heart-rate recovery immediately after exercise as a predictor of mortality.](https://pubmed.ncbi.nlm.nih.gov/10536127/), Abstract (Methods) | Abnormal (prognostic) heart rate recovery at one minute after exercise, cut-off 12 bpm or less (inclusive). Cohort measured on exercise-test ECG, not wearable data. Not a diagnostic threshold. Source band <= 12; stored as the warn low bound 12. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
