# Tissue Transglutaminase IgA (multiple of upper limit of normal) (`tissue-transglutaminase-iga-uln-multiple`)

Result expressed as a multiple of the reporting laboratory's upper limit of normal (ULN) rather than in absolute units. Compute it as the measured result divided by the ULN printed on the same laboratory report.

| | |
|---|---|
| Category | [autoimmune](index.md#autoimmune) |
| Canonical unit | x ULN |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in x ULN. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | 10 | - | guideline | Children and adolescents with symptoms suspicious of coeliac disease | [European Society Paediatric Gastroenterology, Hepatology and Nutrition Guidelines for Diagnosing Coeliac Disease 2020.](https://pubmed.ncbi.nlm.nih.gov/31568151/), Abstract, Recommendations | Below 10x ULN the no-biopsy route does not apply: at least 4 biopsies from the distal duodenum and at least 1 from the bulb are required. Upper boundary is exclusive (<10x ULN). Source kind: decision. Source band <= 10; stored as the warn low bound 10. Alternative to the default espghan-2020-coeliac warn range for the same population. |
| espghan-2020-coeliac | warn | all | - | 10 | guideline | Children and adolescents with symptoms suspicious of coeliac disease | [European Society Paediatric Gastroenterology, Hepatology and Nutrition Guidelines for Diagnosing Coeliac Disease 2020.](https://pubmed.ncbi.nlm.nih.gov/31568151/), Abstract, Recommendations | Biopsy-free diagnosis may be applied if TGA-IgA is >=10x ULN, endomysial antibodies (EMA-IgA) are positive in a second blood sample and the family agrees. Total serum IgA should be measured with TGA-IgA; an IgG-based test is needed only if total IgA is low or undetectable. No numeric total-IgA cut-off was retrieved. Source kind: decision. Source band >= 10; stored as the warn high bound 10. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
