# Appendicular Lean Mass Index (ALM/height2) (`appendicular-lean-mass-index`)

Appendicular lean mass (muscle mass of the arms and legs, usually from DXA or bioimpedance) divided by height squared. Used as the muscle-quantity criterion for sarcopenia; low values indicate low muscle quantity.

| | |
|---|---|
| Category | [body](index.md#body) |
| Canonical unit | kg/m2 |
| LOINC | - |
| Specimen | - |
| Aliases | `alm/height2`, `appendicular skeletal muscle index`, `asm/height2`, `smi` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in kg/m2. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | female | 5.7 | - | consensus | Asian older adults (AWGS 2019 consensus) (method: bia) | [Asian Working Group for Sarcopenia: 2019 Consensus Update on Sarcopenia Diagnosis and Treatment.](https://pubmed.ncbi.nlm.nih.gov/32033882/), Abstract, Results | Low muscle mass if below 5.7 kg/m2 in women by bioimpedance (strict less-than boundary). Source kind: decision. Source band <= 5.7; stored as the warn low bound 5.7. Alternative to the default awgs-2019 warn range for the same population. |
| awgs-2019 | warn | male | 7 | - | consensus | Asian older adults (AWGS 2019 consensus) (method: dxa) | [Asian Working Group for Sarcopenia: 2019 Consensus Update on Sarcopenia Diagnosis and Treatment.](https://pubmed.ncbi.nlm.nih.gov/32033882/), Abstract, Results | Low muscle mass if below 7.0 kg/m2 in men by DXA (strict less-than boundary). Source kind: decision. Source band <= 7.0; stored as the warn low bound 7.0. |
| awgs-2019 | warn | female | 5.4 | - | consensus | Asian older adults (AWGS 2019 consensus) (method: dxa) | [Asian Working Group for Sarcopenia: 2019 Consensus Update on Sarcopenia Diagnosis and Treatment.](https://pubmed.ncbi.nlm.nih.gov/32033882/), Abstract, Results | Low muscle mass if below 5.4 kg/m2 in women by DXA (strict less-than boundary). Source kind: decision. Source band <= 5.4; stored as the warn low bound 5.4. |
| awgs-2019 | warn | male | 7 | - | consensus | Asian older adults (AWGS 2019 consensus) (method: bia) | [Asian Working Group for Sarcopenia: 2019 Consensus Update on Sarcopenia Diagnosis and Treatment.](https://pubmed.ncbi.nlm.nih.gov/32033882/), Abstract, Results | Low muscle mass if below 7.0 kg/m2 in men by bioimpedance (strict less-than boundary). Source kind: decision. Source band <= 7.0; stored as the warn low bound 7.0. |
| ewgsop2-2019 | warn | male | 7 | - | consensus | Older adults (European consensus); cut-offs for low muscle quantity in men | [Sarcopenia: revised European consensus on definition and diagnosis.](https://pubmed.ncbi.nlm.nih.gov/30312372/), Table 3, EWGSOP2 cut-off points for low muscle quantity | Low muscle quantity if below 7.0 kg/m2 in men (strict less-than boundary). Confirms sarcopenia when combined with low strength. Measurement method not specified in the quoted text. Source kind: decision. Source band <= 7.0; stored as the warn low bound 7.0. |
| ewgsop2-2019 | warn | female | 5.5 | - | consensus | Older adults (European consensus); cut-offs for low muscle quantity in women | [Sarcopenia: revised European consensus on definition and diagnosis.](https://pubmed.ncbi.nlm.nih.gov/30312372/), Table 3, EWGSOP2 cut-off points for low muscle quantity | Low muscle quantity if below 5.5 kg/m2 in women (strict less-than boundary). Measurement method not specified in the quoted text. Source kind: decision. Source band <= 5.5; stored as the warn low bound 5.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
