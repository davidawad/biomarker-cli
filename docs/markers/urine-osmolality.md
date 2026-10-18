# Urine Osmolality (`urine-osmolality`)

Concentration of dissolved particles in urine, measured directly. It shows how well the kidneys concentrate urine and is used to assess hydration, water deprivation tests and the cause of polyuria.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | mOsm/kg |
| LOINC | [2695-5](https://loinc.org/2695-5/) |
| Specimen | - |
| Aliases | `uosm`, `urine osmolarity` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mOsm/kg. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 533 | cohort | Children with polyuria-polydipsia undergoing a water deprivation test (setting: water-deprivation-test) | [Water deprivation test in children: challenging but still necessary for the differential diagnosis of polyuria-polydipsia syndrome.](https://pubmed.ncbi.nlm.nih.gov/40929290/), Abstract, Results | Interrupted-test uOsm above 533 mOsm/kgH2O excluded AVP disorders with 100% sensitivity and specificity in this cohort, as worded in the abstract ('an interrupt uOsm'). Rule-out threshold for this test setting only. Source kind: decision. Source band >= 533; stored as the warn high bound 533. Alternative to the default pediatric-water-deprivation-avp-2025 warn range for the same population. |
| hypohydration-usg-validation-2022 | warn | all | - | 700 | cohort | Athletes, hydration status assessment | [Validation of urine colour L*a*b* for assessing hydration amongst athletes.](https://pubmed.ncbi.nlm.nih.gov/36034925/), Abstract, Results | Hypohydration diagnosis threshold of urine osmolality >=700 mmol/kg as used in the cited validation study, given as an alternative to USG >=1.020. Unit override mmol/kg is used because the source reports mmol/kg. Converted from the source's mmol/kg (700) to mOsm/kg using factor 1. Source kind: decision. Source band >= 700.0; stored as the warn high bound 700.0. |
| pediatric-water-deprivation-avp-2025 | warn | all | - | 279 | cohort | Children with polyuria-polydipsia undergoing a water deprivation test (setting: water-deprivation-test) | [Water deprivation test in children: challenging but still necessary for the differential diagnosis of polyuria-polydipsia syndrome.](https://pubmed.ncbi.nlm.nih.gov/40929290/), Abstract, Results | Baseline uOsm above 279 mOsm/kgH2O excluded arginine vasopressin (AVP) deficiency and/or resistance in all patients in this cohort (sensitivity 100%, specificity 74%). Rule-out threshold, not a general reference range. Source kind: decision. Source band >= 279; stored as the warn high bound 279. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
