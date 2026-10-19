# GrimAge Acceleration (`grimage-acceleration`)

Epigenetic age acceleration from the DNA methylation GrimAge clock: GrimAge in years, adjusted for chronological age, so positive values mean an older-than-expected mortality-risk profile. It predicts time to death, coronary heart disease and cancer more strongly than earlier DNA methylation clocks.

| | |
|---|---|
| Category | [aging](index.md#aging) |
| Canonical unit | years |
| LOINC | - |
| Specimen | - |
| Aliases | `ageaccelgrim` |
| Panels | [Biological aging](../panels/biological-aging.md) |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in years. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| lu-grimage-2019 | warn | all | - | 8.3 | cohort | Adults in the Framingham and other validation cohorts in Lu et al. 2019; approximate 95th percentile of AgeAccelGrim | [DNA methylation GrimAge strongly predicts lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/30669119/), Results, AgeAccelGrim and mortality (full text) | Fast ageing: AgeAccelGrim at about the top 5% (95th percentile) of the distribution, where the hazard of death is about twice that of the average person in the same stratum (HR about 2.2 from 1.10^8.3). The source text shows '=8.3 years', most likely a symbol lost in extraction for >=8.3. Lower bound only (open above). The bottom 5th percentile (-7.5 years) is not encoded because negative values cannot be matched by the quote check. Source band >= 8.3; stored as the warn high bound 8.3. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
