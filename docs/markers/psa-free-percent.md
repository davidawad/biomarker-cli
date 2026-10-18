# Free PSA Percentage (`psa-free-percent`)

The percentage of total PSA that is unbound (free) in serum. A low free fraction raises the likelihood of prostate cancer, so it helps decide whether to biopsy men with a borderline total PSA.

| | |
|---|---|
| Category | [tumour](index.md#tumour) |
| Canonical unit | % |
| LOINC | [12841-3](https://loinc.org/12841-3/) |
| Specimen | - |
| Aliases | `%free psa`, `free/total psa ratio`, `percent free psa` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| jama-1998-free-psa | warn | all | 25 | - | cohort | Men with total PSA 4.0-10.0 ng/mL and a palpably benign prostate in a prospective multicenter trial | [Use of the percentage of free prostate-specific antigen to enhance differentiation of prostate cancer from benign prostatic disease: a prospective multicenter clinical trial.](https://pubmed.ncbi.nlm.nih.gov/9605898/), Abstract, Conclusions | Biopsy cut-off: free PSA at or below 25% is abnormal (biopsy recommended) for total PSA 4.0-10.0 ng/mL with a benign-feeling gland. Applies only when total PSA is 4.0-10.0 ng/mL; boundary is <= 25%. Applies only to: markers={"psa": ">=4"}. Source kind: decision. Source band <= 25; stored as the warn low bound 25. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
