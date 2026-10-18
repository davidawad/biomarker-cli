# Retinol (vitamin A) (`vitamin-a`)

Serum retinol, the circulating form of vitamin A, which reflects liver vitamin A stores when inflammation is absent. Low values indicate vitamin A deficiency, which causes night blindness and xerophthalmia and raises infection risk.

| | |
|---|---|
| Category | [vitamin](index.md#vitamin) |
| Canonical unit | µg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in µg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| fortification-meta-2019 | warn | all | 30.08 | - | consensus | Lactating mothers, as a vitamin A deficiency outcome definition in a 2019 meta-analysis of LMIC fortification studies | [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Table 2, Definitions of outcomes | Outcome definition used by the meta-analysis (1.05 µmol/L converted with factor 28.65). Outcome definition used by the meta-analysis; not presented as a general clinical threshold. Applies only to: lactating=True. Source kind: deficient. Source band <= 30.08; stored as the warn low bound 30.08. |
| who | warn | all | 20.06 | - | consensus | Preschool children and general population, as the WHO cut-off for biochemical vitamin A deficiency (VAD), as quoted in a 2018 review | [Vitamin A Deficiency and the Lung.](https://pubmed.ncbi.nlm.nih.gov/30134568/), Introduction, para 1 | WHO cut-off 0.70 µmol/L converted to µg/dL with factor 28.65. Value is 'below' the cut-off. Inflammation can lower retinol; retinol-binding protein is an alternative marker. Source kind: deficient. Source band <= 20.06; stored as the warn low bound 20.06. |
| who | critical | all | 10.03 | - | consensus | General population; WHO severe vitamin A deficiency cut-off, as quoted in a 2018 review | [Vitamin A Deficiency and the Lung.](https://pubmed.ncbi.nlm.nih.gov/30134568/), Introduction, para 1 | Severe deficiency cut-off 0.35 µmol/L converted to µg/dL with factor 28.65. Associated with marked increases in risk of clinical manifestations such as xerophthalmia and nyctalopia. Source band <= 10.03; stored as the critical low bound 10.03. |
| who | warn | pregnant | 20.06 | - | consensus | Pregnant women; WHO definition of VAD in pregnancy, as quoted in a 2019 narrative review | [Vitamin A and Pregnancy: A Narrative Review.](https://pubmed.ncbi.nlm.nih.gov/30909386/), Text on vitamin A deficiency in pregnancy | Same 0.70 µmol/L cut-off as for the general population, converted with factor 28.65. Source kind: deficient. Source band <= 20.06; stored as the warn low bound 20.06. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
