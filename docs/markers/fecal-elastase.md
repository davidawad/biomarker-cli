# Faecal Elastase-1 (`fecal-elastase`)

Faecal elastase-1 is a pancreas-specific enzyme measured in a stool sample. Low concentrations indicate exocrine pancreatic insufficiency (PEI), and the result is graded as mild, moderate or severe. It is not affected by pancreatic enzyme replacement therapy because it measures only human elastase.

| | |
|---|---|
| Category | [stool](index.md#stool) |
| Canonical unit | µg/g |
| LOINC | [25907-7](https://loinc.org/25907-7/) |
| Specimen | - |
| Aliases | `fe-1`, `fel-1`, `faecal elastase`, `fecal elastase`, `stool elastase`, `pancreatic elastase` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in µg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| uk-pei-consensus-2021 | warn | all | - | 200 | consensus | Adults with suspected exocrine pancreatic insufficiency (UK practical consensus) | [Consensus for the management of pancreatic exocrine insufficiency: UK practical guidelines.](https://pubmed.ncbi.nlm.nih.gov/34140324/), Section on diagnosis, Statement 1.2 and supporting text | Result at or above 200 µg/g is not in the moderate-PEI band. The consensus gives <200 as moderate and <100 as severe, so 200 is the boundary. The consensus does not state a normal range in these words; the lower limit is taken by implication. Source kind: decision. Source band >= 200; stored as the warn high bound 200. |
| uk-pei-consensus-2021 | critical | all | 100 | - | consensus | Adults with suspected exocrine pancreatic insufficiency (UK practical consensus) | [Consensus for the management of pancreatic exocrine insufficiency: UK practical guidelines.](https://pubmed.ncbi.nlm.nih.gov/34140324/), Section on diagnosis, Statement 1.2 and supporting text | Severe PEI band (below 100 µg/g), per the UK consensus wording '<100 µg/g suggests severe PEI'. Source band <= 100; stored as the critical low bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
