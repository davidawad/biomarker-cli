# DXA Z-Score (Lumbar Spine) (`dxa-z-score`)

Lumbar spine areal bone mineral density (aBMD) Z-score from DXA: the standard deviation difference from the mean of an age-, sex- and ethnicity-matched reference population. Used to flag bone density below the expected range for age, and preferred over the T-score in premenopausal women and in men under 50. A calculated, not a measured, score. Expected result: within the expected range for age (Z-score above -2.0).

| | |
|---|---|
| Category | [bone](index.md#bone) |
| Canonical unit | z-score |
| LOINC | [85394-5](https://loinc.org/85394-5/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in z-score. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | -2 | - | consensus | Adults and children with DXA aBMD Z-scores against age-, sex- and ethnicity-matched reference data (ISCD definition) | [Bone Health and Psychological Wellbeing in Turner Syndrome: A Tertiary Healthcare Centre Experience from North India.](https://pubmed.ncbi.nlm.nih.gov/42788133/), Methods (DXA analysis) | Z-score above -2.0: within the expected range for age (ISCD). |
| diagnostic | warn | all | -2 | - | consensus | Adults and children with DXA aBMD Z-scores against age-, sex- and ethnicity-matched reference data (ISCD definition) | [Bone Health and Psychological Wellbeing in Turner Syndrome: A Tertiary Healthcare Centre Experience from North India.](https://pubmed.ncbi.nlm.nih.gov/42788133/), Methods (DXA analysis); [Osteoporosis Diagnosis and Management: A Review from an Orthopedic Perspective.](https://pubmed.ncbi.nlm.nih.gov/42661519/), Section 2 (T-scores and Z-scores) | Z-score -2.0 or lower: below the expected range for age (ISCD). Prompts evaluation for secondary causes. Source kind: diagnostic. Source band <= -2.0; stored as the warn low bound -2.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
