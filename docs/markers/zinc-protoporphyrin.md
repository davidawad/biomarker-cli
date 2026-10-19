# Zinc Protoporphyrin (`zinc-protoporphyrin`)

Erythrocyte porphyrin that rises when iron is short for heme synthesis; measured (ZPP/heme) with a portable haematofluorometer to screen for iron deficiency. Expected result for a healthy child: within the age- and sex-specific 2.5th to 97.5th percentile limits in the cited pediatric study; values above the iron-deficiency cut-offs suggest iron deficiency.

| | |
|---|---|
| Category | [iron](index.md#iron) |
| Canonical unit | umol/mol heme |
| LOINC | [33007-6](https://loinc.org/33007-6/) |
| Specimen | - |
| Aliases | - |
| Panels | [Iron deficiency and iron deficiency anaemia](../panels/iron-anemia.md) |

## Ranges

Bounds in umol/mol heme. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | female, age 0-1 | 16.6 | 73.6 | reference-interval-study | Children aged 0 to 12 months, female (pediatric hospital patients, n = 6478 specimens, 0-17 years) | [Pediatric reference ranges for zinc protoporphyrin.](https://pubmed.ncbi.nlm.nih.gov/12554055/), Abstract (Results and Conclusions) | 2.5th to 97.5th percentile (Hoffman approach); reported as 9 to 40 microg/dL in the source. |
| clinical | reference | male, age 0-1 | 15.6 | 63.5 | reference-interval-study | Children aged 0 to 12 months, male (pediatric hospital patients, n = 6478 specimens, 0-17 years) | [Pediatric reference ranges for zinc protoporphyrin.](https://pubmed.ncbi.nlm.nih.gov/12554055/), Abstract (Results and Conclusions) | 2.5th to 97.5th percentile (Hoffman approach); reported as 8.5 to 34.5 microg/dL in the source. |
| alt-1 | warn | iron-deficiency | - | 70 | cohort | Rural Kenyan pregnant women (conventional whole-blood ZPP cut-off, which this study found to overestimate iron deficiency) | [Diagnostic utility of zinc protoporphyrin to detect iron deficiency in Kenyan pregnant women.](https://pubmed.ncbi.nlm.nih.gov/25428714/), Abstract (Results) | Conventional whole-blood ZPP cut-off, strictly above 70 umol/mol heme, cited as giving gross overestimates of iron deficiency prevalence in this population; not validated here. Source kind: diagnostic. Source band >= 70; stored as the warn high bound 70. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | iron-deficiency-anemia | - | 90 | cohort | Community-dwelling Indian preschool children with and without iron deficiency anemia | [Optimizing diagnostic biomarkers of iron deficiency anemia in community-dwelling Indian women and preschool children.](https://pubmed.ncbi.nlm.nih.gov/30093400/), Abstract (Results) | Iron deficiency anemia, high diagnostic likelihood (likelihood ratio 20.2): ZPP/heme strictly above 90 umol/mol heme in children. Source kind: diagnostic. Source band >= 90; stored as the warn high bound 90. |
| diagnostic | warn | female, iron-deficiency-anemia | - | 107 | cohort | Community-dwelling Indian women with and without iron deficiency anemia | [Optimizing diagnostic biomarkers of iron deficiency anemia in community-dwelling Indian women and preschool children.](https://pubmed.ncbi.nlm.nih.gov/30093400/), Abstract (Results) | Iron deficiency anemia, high diagnostic likelihood (likelihood ratio 10.8): ZPP/heme strictly above 107 umol/mol heme in women. Source kind: diagnostic. Source band >= 107; stored as the warn high bound 107. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
