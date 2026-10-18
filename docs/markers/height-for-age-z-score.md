# Height-for-Age Z-Score (`height-for-age-z-score`)

Child length or height for age, expressed as a standard-deviation score (z-score) against the WHO Child Growth Standards median for the same age and sex. It is the standard measure of linear growth and of stunting. A z-score below -2 defines stunting and below -3 defines severe stunting.

| | |
|---|---|
| Category | [pediatric](index.md#pediatric) |
| Canonical unit | z-score |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in z-score. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | -3 | - | consensus | Children, length/height-for-age z-score against the WHO Child Growth Standards for the same age and sex | [Childhood stunting: a global perspective.](https://pubmed.ncbi.nlm.nih.gov/27187907/), Full text, paragraph defining stunting by length/height-for-age cut-offs | Severe stunting: z-score strictly below -3 (the bound is exclusive). Source kind: diagnostic. Source band <= -3.0; stored as the warn low bound -3.0. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | -2 | - | consensus | Children, length/height-for-age z-score against the WHO Child Growth Standards for the same age and sex | [Childhood stunting: a global perspective.](https://pubmed.ncbi.nlm.nih.gov/27187907/), Full text, paragraph defining stunting by length/height-for-age cut-offs | Stunting: z-score strictly below -2 (the bound is exclusive). Source kind: diagnostic. Source band <= -2.0; stored as the warn low bound -2.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
