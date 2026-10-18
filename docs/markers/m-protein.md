# M-Protein (Serum Protein Electrophoresis) (`m-protein`)

Monoclonal (M) protein: a single, clonal immunoglobulin band seen on serum protein electrophoresis (SPEP) and quantified in g/dL. It is the marker used to detect and monitor monoclonal gammopathies such as MGUS, smoldering multiple myeloma and multiple myeloma. Expected result in a person without a plasma cell disorder: no M-protein band detected (negative).

| | |
|---|---|
| Category | [protein](index.md#protein) |
| Canonical unit | g/dL |
| LOINC | [33358-3](https://loinc.org/33358-3/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in g/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | mgus | 3 | - | cohort | Adults in a cohort study; M-protein case definition for monoclonal gammopathy of undetermined significance (SPEP and immunofixation positive) | [A nested case-control study of monoclonal gammopathy of undetermined significance in the Multiethnic Cohort study.](https://pubmed.ncbi.nlm.nih.gov/41529215/), Full text, MGUS case definition paragraph (Methods) | Strictly below 3 g/dL: the case definition of MGUS in the cited cohort study. Source kind: diagnostic. Source band <= 3; stored as the warn low bound 3. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | smoldering-multiple-myeloma | - | 3 | consensus | Adults with smoldering multiple myeloma criteria (IMWG 2014 definition as stated in the cited paper) | [Estimated Whole Blood Viscosity in Smoldering Multiple Myeloma: Preliminary Data from a Retrospective Single Center Analysis.](https://pubmed.ncbi.nlm.nih.gov/42519532/), Full text, Introduction (paragraph on SMM definition) | Serum M-protein of 3 g/dL or more, as stated for the IMWG 2014 smoldering multiple myeloma definition in the cited paper; the same paper states further required criteria that are not part of this range. Source kind: diagnostic. Source band >= 3; stored as the warn high bound 3. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
