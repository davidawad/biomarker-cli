# Folate (`folate`)

Serum folate reflects recent dietary folate intake and is the first-line test for folate status. Low values indicate folate deficiency or insufficiency; the metabolic consequence of low folate is raised homocysteine.

| | |
|---|---|
| Category | [vitamin](index.md#vitamin) |
| Canonical unit | ng/mL |
| LOINC | [2284-8](https://loinc.org/2284-8/) |
| Specimen | - |
| Aliases | `folic acid` |
| Panels | [Alzheimer's disease and dementia workup](../panels/alzheimers.md), [Iron deficiency and iron deficiency anaemia](../panels/iron-anemia.md) |

## Unit conversions

Into ng/mL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| nmol/L | 0.4413 | 0 |

## Ranges

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 3 | - | - | - | uncited (legacy) | - |
| nutrients-2017-review | warn | all | 5.9 | - | consensus | Women of reproductive age and pregnant women in Ethiopia, Kenya, Nigeria and South Africa (systematic review; cut-offs quoted from a reference cited in the review) | [Micronutrient Status and Dietary Intake of Iron, Vitamin A, Iodine, Folate and Zinc in Women of Reproductive Age and Pregnant Women in Ethiopia, Kenya, Nigeria and South Africa: A Systematic Review of Data from 2005 to 2015.](https://pubmed.ncbi.nlm.nih.gov/28981457/), Methods, cut-off definitions (full text) | Quoted as possible deficiency (6.8-13.4 nmol/L). The same review gives severe deficiency as below 3 ng/mL (6.8 nmol/L). These bands overlap the WHO deficient band (<4.4 ng/mL) above, so pick one set for a given lab. Source kind: insufficient. Source band 3-5.9; stored as the warn low bound 5.9. |
| who-1972-historic | reference | all | 6 | 20 | consensus | Historical normal range from early folate studies, which formed the basis for WHO serum folate guidance in 1972 (Lactobacillus microbiological assay era) | [Folate status assessment history: implications for measurement of biomarkers in NHANES.](https://pubmed.ncbi.nlm.nih.gov/21593497/), Results or history section (full text) | Historical reference only. Current assays and calibrators give different values, so this band should not be used for modern assays without local validation. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
