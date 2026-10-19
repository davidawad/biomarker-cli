# Vitamin D, 25-Hydroxy (`vitamin-d`)

Circulating 25-hydroxyvitamin D [25(OH)D], the main marker of vitamin D status from skin synthesis and diet. Low values indicate deficiency or insufficiency relative to bone health; very high values indicate risk of vitamin D toxicity with hypercalcemia.

| | |
|---|---|
| Category | [vitamin](index.md#vitamin) |
| Canonical unit | ng/mL |
| LOINC | [1989-3](https://loinc.org/1989-3/) |
| Specimen | - |
| Aliases | `25-oh vitamin d`, `vit d`, `25(oh)d`, `vitamin d, 25-hydroxy`, `25-hydroxyvitamin d` |
| Panels | [Bone health and osteoporosis](../panels/bone-health.md), [Diabetes and glycemic control](../panels/diabetes.md) |

## Unit conversions

Into ng/mL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| nmol/L | 0.4006 | 0 |

## Ranges

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 30 | 100 | - | - | uncited (legacy) | - |
| optimal | optimal | all | 40 | 60 | - | - | uncited (legacy) | - |
| alt-1 | warn | all | - | 50 | guideline | North American general population; IOM DRI scope | [Dietary Reference Intakes for Calcium and Vitamin D (IOM 2011, NAP 13050)](https://nap.nationalacademies.org/read/13050/chapter/2), Chapter 2 (NAP online text) | IOM: reason for concern about adverse effects above this level; not a diagnosis of toxicity. Source kind: excess. Source band >= 50; stored as the warn high bound 50. Alternative to the default iom-2011 warn range for the same population. |
| alt-2 | warn | all | 21 | - | guideline | Adults and children at risk of deficiency (Endocrine Society 2011 clinical practice guideline, as quoted by Wacker and Holick 2013) | [Vitamin D - effects on skeletal and extraskeletal health and the need for supplementation.](https://pubmed.ncbi.nlm.nih.gov/23306192/), Section: Vitamin D deficiency (citing the Endocrine Society 2011 guideline, ref 16) | Endocrine Society 2011 insufficiency band (21-29 ng/mL). Source kind: insufficient. Source band 21-29; stored as the warn low bound 21. Alternative to the default endocrine-society-2011 warn range for the same population. |
| endo-rev-2024-consensus | optimal | all | 20 | 30 | consensus | Adults and general population; summary of worldwide vitamin D guidelines cited in a 2024 consensus statement | [Consensus Statement on Vitamin D Status Assessment and Supplementation: Whys, Whens, and Hows.](https://pubmed.ncbi.nlm.nih.gov/38676447/), Vitamin D Metabolism and Mechanism of Action, para on optimal levels | Range as summarized by the consensus authors from guidelines (radioimmunoassay era); the consensus itself says optimal levels remain debated. |
| endocrine-society-2011 | warn | all | 20 | - | guideline | Adults and children at risk of deficiency (Endocrine Society 2011 clinical practice guideline, as quoted by Wacker and Holick 2013) | [Vitamin D - effects on skeletal and extraskeletal health and the need for supplementation.](https://pubmed.ncbi.nlm.nih.gov/23306192/), Section: Vitamin D deficiency (citing the Endocrine Society 2011 guideline, ref 16) | Endocrine Society 2011 deficiency cutoff; the guideline itself is paywalled, so the numbers are cited via a review by the guideline chair. Source kind: deficient. Source band <= 20; stored as the warn low bound 20. |
| endocrine-society-2011 | reference | all | 30 | - | guideline | Adults and children at risk of deficiency (Endocrine Society 2011 clinical practice guideline, as quoted by Wacker and Holick 2013) | [Vitamin D - effects on skeletal and extraskeletal health and the need for supplementation.](https://pubmed.ncbi.nlm.nih.gov/23306192/), Section: Vitamin D deficiency (citing the Endocrine Society 2011 guideline, ref 16) | Endocrine Society 2011 sufficiency (>30 ng/mL). The 2024 Endocrine Society guideline (PMID 38828931) no longer endorses a target threshold. |
| endocrine-society-2011 | optimal | all | 40 | 60 | guideline | Adults and children at risk of deficiency (Endocrine Society 2011 clinical practice guideline, as quoted by Wacker and Holick 2013) | [Vitamin D - effects on skeletal and extraskeletal health and the need for supplementation.](https://pubmed.ncbi.nlm.nih.gov/23306192/), Section: Vitamin D deficiency (citing the Endocrine Society 2011 guideline, ref 16) | Preferred range per the same review; opinion-level, not an outcome-trial-derived target. |
| iom-2011 | warn | all | 12 | - | guideline | North American general population; IOM DRI scope (all life-stage groups, risk of deficiency relative to bone health) | [Dietary Reference Intakes for Calcium and Vitamin D (IOM 2011, NAP 13050)](https://nap.nationalacademies.org/read/13050/chapter/2), Chapter 2 (NAP online text) | Upper bound of deficiency (below 30 nmol/L). Value 12 ng/mL is the boundary; source quote gives it as 'below'. Source kind: deficient. Source band <= 12; stored as the warn low bound 12. |
| iom-2011 | warn | all | 12 | - | guideline | North American general population; IOM DRI scope (all life-stage groups) | [Dietary Reference Intakes for Calcium and Vitamin D (IOM 2011, NAP 13050)](https://nap.nationalacademies.org/read/13050/chapter/2), Chapter 2 (NAP online text) | IOM 'inadequacy' band, 30-50 nmol/L. Some but not all persons are potentially at risk. Source kind: insufficient. Source band 12-20; stored as the warn low bound 12. |
| iom-2011 | reference | all | 20 | - | guideline | North American general population; IOM DRI scope (all life-stage groups) | [Dietary Reference Intakes for Calcium and Vitamin D (IOM 2011, NAP 13050)](https://nap.nationalacademies.org/read/13050/chapter/2), Chapter 2 (NAP online text) | Sufficiency threshold (at least 50 nmol/L). The IOM gives no upper bound for sufficiency; see the excess range for concern above 50 ng/mL. |
| vitamin-d-toxicity-review-2018 | warn | all | - | 150 | consensus | Adults with vitamin D overdosing (exogenous vitamin D toxicity, VDT) | [Vitamin D Toxicity-A Clinical Perspective.](https://pubmed.ncbi.nlm.nih.gov/30294301/), Introduction, para 1 | Serum 25(OH)D above 150 ng/mL is the hallmark of exogenous VDT, usually with hypercalcemia and hypercalciuria; the review states that the IOM and the Endocrine Society also concluded that levels must exceed this value for toxicity. Source kind: toxic. Source band >= 150; stored as the warn high bound 150. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
