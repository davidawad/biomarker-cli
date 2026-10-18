# Life's Essential 8 Cardiovascular Health Score (`lifes-essential-8`)

Overall cardiovascular health score from 0 to 100 built from eight metrics: diet, physical activity, nicotine exposure, sleep, body mass index, blood lipids, blood glucose and blood pressure. Higher scores indicate better cardiovascular health.

| | |
|---|---|
| Category | [composite](index.md#composite) |
| Canonical unit | score |
| LOINC | - |
| Specimen | - |
| Aliases | `le8`, `life's essential 8`, `cvh score` |

## Ranges

Bounds in score. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 80 | 100 | consensus | US and general adult populations; overall Life's Essential 8 cardiovascular health score | [Life's Essential 8: Updating and Enhancing the American Heart Association's Construct of Cardiovascular Health: A Presidential Advisory From the American Heart Association.](https://pubmed.ncbi.nlm.nih.gov/35766027/), Full text, Overall CVH score section | High cardiovascular health, score 80-100. Category cut-offs recommended by the writing group; the score is 0-100 points. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| aha-life-essential-8-2022 | warn | all | 49 | - | consensus | US and general adult populations; overall Life's Essential 8 cardiovascular health score | [Life's Essential 8: Updating and Enhancing the American Heart Association's Construct of Cardiovascular Health: A Presidential Advisory From the American Heart Association.](https://pubmed.ncbi.nlm.nih.gov/35766027/), Full text, Overall CVH score section | Low cardiovascular health, score 0-49. Upper bound only; the lower bound is open because scores cannot go below 0. Source band <= 49; stored as the warn low bound 49. |
| aha-life-essential-8-2022-50-79 | reference | all | 50 | 79 | consensus | US and general adult populations; overall Life's Essential 8 cardiovascular health score | [Life's Essential 8: Updating and Enhancing the American Heart Association's Construct of Cardiovascular Health: A Presidential Advisory From the American Heart Association.](https://pubmed.ncbi.nlm.nih.gov/35766027/), Full text, Overall CVH score section | Moderate cardiovascular health, score 50-79. Boundary semantics: >=50 and <=79 as written in the source. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
