# Sleep Efficiency (`sleep-efficiency`)

Percentage of time in bed that is spent asleep (total sleep time divided by time in bed), measured by actigraphy, polysomnography or wearable sleep tracking. Low values indicate fragmented or poor-quality sleep.

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | `sleep efficiency index`, `se` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| insomnia-research-criteria-2019 | warn | insomnia | 85 | - | cohort | Adults meeting DSM-5 criteria for insomnia, selected for actigraphy versus in-laboratory polysomnography comparison (method: polysomnography) | [Relation between ambulatory actigraphy and laboratory polysomnography in insomnia practice and research.](https://pubmed.ncbi.nlm.nih.gov/30941838/), Abstract, Methods | Research inclusion criterion for insomnia: in-laboratory polysomnography sleep efficiency below 85% (boundary < 85%). The study used this as an entry threshold; it is not a diagnostic cut-off for insomnia by itself. Source kind: decision. Source band <= 85; stored as the warn low bound 85. |
| jacc-adv-2026-sleep-health-women-mi | warn | all | 85 | - | cohort | Women after myocardial infarction, sleep measured by actigraphy and diary | [Objective and Subjective Multidimensional Sleep Health in Women After Myocardial Infarction.](https://pubmed.ncbi.nlm.nih.gov/42748745/), Full text, Methods (sleep health definitions) | Authors' definition of suboptimal sleep health: efficiency below 85% (boundary < 85%), alongside duration and WASO thresholds. The thresholds are attributed by the authors to AASM/SRS recommendations and prior research; that attribution was not verified against the source documents here. Source band <= 85; stored as the warn low bound 85. |
| sleep-2011-actigraphy-poor-sleep | warn | all | 85 | - | cohort | Healthy middle-aged community adults with 3-day actigraphy in a cross-sectional wave of a follow-up study (method: actigraphy) | [Relationship of sleep quantity and quality with 24-hour urinary catecholamines and salivary awakening cortisol in healthy middle-aged adults.](https://pubmed.ncbi.nlm.nih.gov/21286244/), Abstract, Measurements and Results | Research classification of objectively poor sleepers: actigraphy sleep efficiency below 85% (boundary < 85%). A study-defined threshold, not a clinical guideline cut-off; above 85% counts as good sleep in this comparison. Source kind: decision. Source band <= 85; stored as the warn low bound 85. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
