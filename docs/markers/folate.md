# Folate (`folate`)

Serum folate reflects recent dietary folate intake and is the first-line test for folate status. Low values indicate folate deficiency or insufficiency; the metabolic consequence of low folate is raised homocysteine.

| | |
|---|---|
| Category | [vitamin](index.md#vitamin) |
| Canonical unit | ng/mL |
| LOINC | [2284-8](https://loinc.org/2284-8/) |
| Specimen | - |
| Aliases | `folic acid` |

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
| clarke-2003 | warn | age 65+, homocysteine >= 15 | 2.2 | - | cohort | Older persons aged 65 and over in Oxford (population study, n=1562); borderline folate with raised homocysteine | [Screening for vitamin B-12 and folate deficiency in older persons.](https://pubmed.ncbi.nlm.nih.gov/12716678/), Design, para 1 (Abstract) | Borderline band (5-7 nmol/L) counts as high risk of folate deficiency only with homocysteine above 15 micromol/L. In the same study, folate below 5 nmol/L (about 2.2 ng/mL) put people in the high-risk group without a homocysteine condition. Source band 2.2-3.1; stored as the warn low bound 2.2. |
| nutrients-2017-review | warn | all | 5.9 | - | consensus | Women of reproductive age and pregnant women in Ethiopia, Kenya, Nigeria and South Africa (systematic review; cut-offs quoted from a reference cited in the review) | [Micronutrient Status and Dietary Intake of Iron, Vitamin A, Iodine, Folate and Zinc in Women of Reproductive Age and Pregnant Women in Ethiopia, Kenya, Nigeria and South Africa: A Systematic Review of Data from 2005 to 2015.](https://pubmed.ncbi.nlm.nih.gov/28981457/), Methods, cut-off definitions (full text) | Quoted as possible deficiency (6.8-13.4 nmol/L). The same review gives severe deficiency as below 3 ng/mL (6.8 nmol/L). These bands overlap the WHO deficient band (<4.4 ng/mL) above, so pick one set for a given lab. Source kind: insufficient. Source band 3-5.9; stored as the warn low bound 5.9. |
| who-1972-historic | reference | all | 6 | 20 | consensus | Historical normal range from early folate studies, which formed the basis for WHO serum folate guidance in 1972 (Lactobacillus microbiological assay era) | [Folate status assessment history: implications for measurement of biomarkers in NHANES.](https://pubmed.ncbi.nlm.nih.gov/21593497/), Results or history section (full text) | Historical reference only. Current assays and calibrators give different values, so this band should not be used for modern assays without local validation. |
| who-2018-review | warn | all | 4.4 | - | consensus | Serum folate cut-offs derived from NHANES III folate data (Bio-Rad radioassay) and used in WHO folate-status assessment | [Global folate status in women of reproductive age: a systematic review with emphasis on methodological issues.](https://pubmed.ncbi.nlm.nih.gov/30239016/), Table 1 footnote (full text) | 10 nmol/L is about 4.4 ng/mL. The WHO-authored review says rising homocysteine appears at higher serum folate levels (10 or 14 nmol/L). Cut-offs depend strongly on the assay: microbiological assays with 5-methyl-THF calibration give lower values than folic acid calibration. Source kind: deficient. Source band <= 4.4; stored as the warn low bound 4.4. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
