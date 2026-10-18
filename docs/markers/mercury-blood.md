# Mercury, whole blood (`mercury-blood`)

Mercury concentration in whole blood, reflecting recent exposure, mostly from fish and seafood as methylmercury. High values signal exposure, and in pregnancy they are relevant to fetal neurodevelopment.

| | |
|---|---|
| Category | [toxic-metal](index.md#toxic-metal) |
| Canonical unit | µg/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in µg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | female, age 16-50 | - | 6.7 | reference-interval-study | US women aged 16-49 in NHANES 1999-2000 (n = 1,709) (specimen: whole-blood) | [Blood organic mercury and dietary mercury intake: National Health and Nutrition Examination Survey, 1999 and 2000.](https://pubmed.ncbi.nlm.nih.gov/15064162/), Abstract | Measured value is blood organic (methylmercury) mercury, not total mercury, and reflects fish intake. 95th percentile shown as the upper reference bound. Older survey; later NHANES cycles were not retrieved. |
| nhanes-1999-2000-cutoff | warn | female, age 16-50 | - | 5.8 | cohort | US adult women in NHANES 1999-2000, as used by the cited authors (specimen: whole-blood) | [Blood organic mercury and dietary mercury intake: National Health and Nutrition Examination Survey, 1999 and 2000.](https://pubmed.ncbi.nlm.nih.gov/15064162/), Abstract | Concentration above which the cited authors report the proportion of women; they describe newborns above levels 'without increased risk' of neurodevelopmental effects. The 5.8 µg/L value is widely attributed to EPA/NRC methylmercury benchmarks, but that attribution is not stated in the retrieved abstract, so treat as unverified benchmark context. Blood organic (methyl) mercury basis. Source band >= 5.8; stored as the warn high bound 5.8. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
