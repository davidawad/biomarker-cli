# Daily Step Count (`daily-steps`)

Number of steps taken per day, measured by a wearable or pedometer. Mortality risk falls as daily steps rise, with the benefit levelling off at a plateau that depends on age.

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | steps/day |
| LOINC | [55423-8](https://loinc.org/55423-8/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in steps/day. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| ding-2025-steps | optimal | all | 5000 | 7000 | cohort | Adults (all ages pooled; the review notes no age-specific analysis) | [Daily steps and health outcomes in adults: a systematic review and dose-response meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/40713949/), Abstract, Findings | Inflection point of the non-linear dose-response for all-cause mortality, cardiovascular disease incidence, dementia and falls ('around 5000-7000 steps per day'). Pooled across ages. |
| paluch-2022-steps | optimal | age 60+ | 6000 | 8000 | cohort | Adults aged 60 years and older, pooled individual-participant data from 15 cohorts | [Daily steps and all-cause mortality: a meta-analysis of 15 international cohorts.](https://pubmed.ncbi.nlm.nih.gov/35247352/), Abstract, Findings | Plateau of the dose-response for all-cause mortality in adults aged 60 and older (restricted cubic splines, risk decreasing until this level). Not a sedentary cut-off. |
| paluch-2022-steps | optimal | age 0-60 | 8000 | 10000 | cohort | Adults younger than 60 years, pooled individual-participant data from 15 cohorts | [Daily steps and all-cause mortality: a meta-analysis of 15 international cohorts.](https://pubmed.ncbi.nlm.nih.gov/35247352/), Abstract, Findings | Plateau of the dose-response for all-cause mortality in adults younger than 60 (restricted cubic splines). Upper bound 10000 is the top of the published 8000-10 000 range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
