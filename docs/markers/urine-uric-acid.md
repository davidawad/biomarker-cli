# Urine Uric Acid (`urine-uric-acid`)

24-hour urinary uric acid excretion. Tracked to assess uric acid kidney stone risk (uric acid stones) and urinary uric acid handling, interpreted together with urinary pH. Expected result: within the reference range; values above the action level prompt evaluation.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | mg/day |
| LOINC | [3086-6](https://loinc.org/3086-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/day. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 250 | 750 | guideline | Adults, 24-hour urine collection (Brazilian Society of Nephrology guideline, Table 4) | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 4, Urinary laboratory parameters in adults (Uric Acid row) | Guideline reference range for 24-hour urinary uric acid. The guideline states reference values are a suggestion that varies by laboratory standardization. |
| diagnostic | warn | female | - | 750 | guideline | Adult women, 24-hour urine collection (Brazilian Society of Nephrology guideline, Table 4) | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 4, Urinary laboratory parameters in adults (Uric Acid row); [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Section on urinary laboratory parameters (text preceding Table 4) | Action level for women: above 750 mg/day (strictly greater than), per guideline Table 4. The guideline states therapeutic decisions are recommended from the action level. Source kind: diagnostic. Source band >= 750; stored as the warn high bound 750. |
| diagnostic | warn | male | - | 800 | guideline | Adult men, 24-hour urine collection (Brazilian Society of Nephrology guideline, Table 4) | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 4, Urinary laboratory parameters in adults (Uric Acid row); [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Section on urinary laboratory parameters (text preceding Table 4) | Action level for men: above 800 mg/day (strictly greater than), per guideline Table 4. The guideline states therapeutic decisions are recommended from the action level. Source kind: diagnostic. Source band >= 800; stored as the warn high bound 800. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
