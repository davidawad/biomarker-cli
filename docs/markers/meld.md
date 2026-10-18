# MELD Score (Model for End-Stage Liver Disease) (`meld`)

Model for End-Stage Liver Disease score, calculated from serum creatinine, total bilirubin and INR. Used to rank liver transplant candidates and to estimate short-term (3-month) waiting-list mortality. There is no normal range; a higher score means more severe liver disease and higher mortality risk.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | points |
| LOINC | [44760-7](https://loinc.org/44760-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in points. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | cirrhosis | - | 40 | cohort | Adult liver transplant candidates with chronic liver disease on the OPTN waiting list (2A/2B status, 1999-2001) | [Model for end-stage liver disease (MELD) and allocation of donor livers.](https://pubmed.ncbi.nlm.nih.gov/12512033/), Abstract (Results) | Highest MELD risk band: 3-month mortality 71.3% (score 40 or more). Risk category, not a diagnosis. Source kind: diagnostic. Source band >= 40; stored as the warn high bound 40. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | cirrhosis | 9 | - | cohort | Adult liver transplant candidates with chronic liver disease on the OPTN waiting list (2A/2B status, 1999-2001) | [Model for end-stage liver disease (MELD) and allocation of donor livers.](https://pubmed.ncbi.nlm.nih.gov/12512033/), Abstract (Results) | Lowest MELD risk band: 3-month mortality 1.9%. Strictly below 9 (the source says '<9'). Risk category, not a diagnosis. Source kind: diagnostic. Source band <= 9; stored as the warn low bound 9. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
