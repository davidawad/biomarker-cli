# BUN/Creatinine Ratio (`bun-creatinine-ratio`)

Ratio of blood urea nitrogen to serum creatinine (both in mg/dL). Traditionally used to distinguish prerenal azotemia from intrinsic kidney injury in acute kidney injury; its diagnostic value is debated. No reference interval was retrievable from the sources reviewed.

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | ratio |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| bcr-prerenal-threshold | warn | all | - | 20 | consensus | Adults with acute kidney injury (hospitalised patients, as discussed in a critical-care nephrology commentary) | [The meaning of the blood urea nitrogen/creatinine ratio in acute kidney injury.](https://pubmed.ncbi.nlm.nih.gov/29497527/), Full text, Introduction | Traditional pre-renal azotaemia threshold: BUN:creatinine ratio above 20 (boundary > 20). The source describes it as the traditional threshold and notes that about half of AKI patients in its cohort exceeded it, so a high ratio does not reliably separate pre-renal from tubular causes. A 10 to 20 reference interval was not retrievable and is not included. Source kind: decision. Source band >= 20; stored as the warn high bound 20. |
| diagnostic | warn | all | - | 20 | consensus | Adults and children with acute kidney injury | [The meaning of the blood urea nitrogen/creatinine ratio in acute kidney injury.](https://pubmed.ncbi.nlm.nih.gov/29497527/), Abstract, Background | Traditional threshold: a ratio strictly above 20 is used to suggest prerenal azotemia rather than acute tubular necrosis. The source (an expert opinion article) reports little evidence that the ratio reliably separates the two conditions. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
