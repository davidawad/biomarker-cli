# 24-Hour Urine Volume (`urine-volume-24h`)

Total urine volume collected over 24 hours. Tracked to judge adequacy of fluid intake and to assess kidney stone risk, since low urine volume raises urinary supersaturation. Expected result: within the reference range; low volume prompts a fluid-intake target.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | L/day |
| LOINC | [3167-4](https://loinc.org/3167-4/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in L/day. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | 1.5 | - | guideline | Adults, 24-hour urine collection (Brazilian Society of Nephrology guideline, Table 4) | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 4, Urinary laboratory parameters in adults (Volume row); [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Section on urinary laboratory parameters (text preceding Table 4) | Attention level: strictly below 1.5 L/day, per guideline Table 4. The guideline states therapeutic decisions could be applied from the attention level. Source band <= 1.5; stored as the warn low bound 1.5. |
| clinical | reference | all | 2 | 2.5 | guideline | Adults, 24-hour urine collection (Brazilian Society of Nephrology guideline, Table 4) | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 4, Urinary laboratory parameters in adults (Volume row) | Guideline reference range for 24-hour urine volume, written in the source as '>2.0-2.5 L/day (25-30 mL/kg/day)'. The guideline states reference values are a suggestion that varies by laboratory standardization. |
| diagnostic | warn | all | 1 | - | guideline | Adults, 24-hour urine collection (Brazilian Society of Nephrology guideline, Table 4) | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 4, Urinary laboratory parameters in adults (Volume row); [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Section on urinary laboratory parameters (text preceding Table 4) | Action level: strictly below 1 L/day, per guideline Table 4. The guideline states therapeutic decisions are recommended from the action level. Source kind: diagnostic. Source band <= 1.0; stored as the warn low bound 1.0. |
| target | optimal | nephrolithiasis | 2.5 | 3 | guideline | Adults with kidney stones (nephrolithiasis) and low urinary volume | [Brazilian Guidelines on evaluation and clinical management of Nephrolithiasis: Brazilian Society of Nephrology.](https://pubmed.ncbi.nlm.nih.gov/40080792/), Table 5, Nutritional recommendations according to risk factors and metabolic disorders (Low Urinary Volume row) | Treatment target for low urinary volume in nephrolithiasis: urine output 2.5 to 3.0 L/day through increased fluid intake (guideline recommendation level A). Source kind: target. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
