# Wake After Sleep Onset (`wake-after-sleep-onset`)

Total time spent awake between sleep onset and final morning awakening, in minutes (WASO). Tracked to measure sleep continuity and as one of the criteria for insomnia. Expected result for most adults is a short total; a middle-of-the-night wake time of 20 minutes or longer is the research cut-off used to classify primary insomnia, so values under that are below it.

| | |
|---|---|
| Category | [sleep](index.md#sleep) |
| Canonical unit | min |
| LOINC | [103215-0](https://loinc.org/103215-0/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in min. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| diagnostic | warn | all | - | 20 | cohort | Adults; average middle-of-the-night wake time over 2 weeks of sleep-log data, primary insomnia vs normal sleepers | [Defining insomnia: quantitative criteria for insomnia severity and frequency.](https://pubmed.ncbi.nlm.nih.gov/16676781/), Abstract, Results | Research classification cut-off for primary insomnia (sensitivity 94.4%, specificity 79.6% in that study); 20 min or longer is the flagged value. Not a population reference interval. Sleep-log measure, not polysomnography. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
