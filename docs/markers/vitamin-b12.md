# Vitamin B12 (`vitamin-b12`)

Total serum vitamin B12 (cobalamin), including the inactive haptocorrin-bound fraction. Low values indicate deficiency or depletion, but serum B12 alone misses many cases of functional deficiency, so it is usually read together with methylmalonic acid, holotranscobalamin or homocysteine.

| | |
|---|---|
| Category | [vitamin](index.md#vitamin) |
| Canonical unit | pg/mL |
| LOINC | [2132-9](https://loinc.org/2132-9/) |
| Specimen | - |
| Aliases | `b12`, `cobalamin` |

## Unit conversions

Into pg/mL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| pmol/L | 1.355 | 0 |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 232 | 1245 | - | - | uncited (legacy) | - |
| optimal | optimal | all | 500 | - | - | - | uncited (legacy) | - |
| clarke-2003 | warn | age 65+, methylmalonic-acid >= 350 | 271 | - | cohort | Older persons aged 65 and over in Oxford (population study, n=1562), borderline B12 with raised MMA or homocysteine | [Screening for vitamin B-12 and folate deficiency in older persons.](https://pubmed.ncbi.nlm.nih.gov/12716678/), Design, para 1 (Abstract) | Borderline band 150-200 pmol/L counts as high risk of B12 deficiency only when MMA is above 0.35 micromol/L (350 nmol/L) or homocysteine is above 15 micromol/L (see the MMA and homocysteine files). The marker condition here encodes the MMA arm only. Source band 203.3-271.0; stored as the warn low bound 271.0. |
| epi-review-2011 | warn | all | 200.5 | - | consensus | Clinical B12 deficiency threshold as summarised in an epidemiologic review of cobalamin biomarkers (adults, megaloblastic anaemia studies) | [Biomarkers of cobalamin (vitamin B-12) status in the epidemiologic setting: a critical overview of context, applications, and performance characteristics of cobalamin, methylmalonic acid, and holotranscobalamin II.](https://pubmed.ncbi.nlm.nih.gov/21593511/), Performance characteristics, para 1 (full text) | 148 pmol/L is the traditional cut-off (about 200 pg/mL). It is about 95-97% sensitive for clinical deficiency but only 38-39% sensitive against metabolic (MMA or homocysteine) criteria in non-clinical surveys. Higher cut-offs of 221 or 258 pmol/L have been proposed; they raise sensitivity and lower specificity. The 1.355 factor is 1/0.7378 (1 pg/mL = 0.7378 pmol/L). Source kind: deficient. Source band <= 200.5; stored as the warn low bound 200.5. |
| frontiers-2016 | reference | all | 338.8 | - | consensus | General adult population; normal range used by most clinical chemistry laboratories, as cited from Clarke 2003 and others | [Biomarkers and Algorithms for the Diagnosis of Vitamin B12 Deficiency.](https://pubmed.ncbi.nlm.nih.gov/27446930/), Total serum vitamin B12 section, para 1 (full text) | Normal lower limit >250 pmol/L (about 339 pg/mL) as quoted in a 2016 review. Values 150-249 pmol/L are labelled low. Many labs use different assays, so local ranges should be checked. |
| nhanes-roundtable-2011 | warn | all | 271 | - | consensus | US adults in NHANES (roundtable on B12 biomarker cut-offs; general population) | [Biomarkers of vitamin B-12 status in NHANES: a roundtable summary.](https://pubmed.ncbi.nlm.nih.gov/21593512/), Roundtable discussion, cut-off relevance (full text) | Band between the 3-SD (148 pmol/L) and 2-SD (about 200 pmol/L) cut-offs. The roundtable notes that a 148 pmol/L cut-off misses 3-5% of clinically deficient patients while about 200 pmol/L catches all of them but gives more false positives. Source band 200.5-271.0; stored as the warn low bound 271.0. |
| pregnancy-prevalence-2019 | warn | pregnant | 284.6 | - | consensus | Pregnant women in the PREFORM prospective cohort (Toronto, n=368), sampled at 12-16 gestational weeks and at delivery | [The Effects of Vegetarian and Vegan Diet during Pregnancy on the Health of Mothers and Offspring.](https://pubmed.ncbi.nlm.nih.gov/30845641/), Maternal B12 status section, para 1 (full text) | No pregnancy-specific B12 threshold was found in the retrieved sources; this is the suboptimal cut-off used in a pregnancy cohort, cited in a review. Deficiency at <148 pmol/L was 17% at 12-16 weeks and 38% at delivery in the same cohort. Source kind: insufficient. Source band <= 284.6; stored as the warn low bound 284.6. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
