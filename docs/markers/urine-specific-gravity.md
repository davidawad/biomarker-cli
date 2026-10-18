# Urine Specific Gravity (`urine-specific-gravity`)

Density of urine relative to water, a quick index of urine concentration and hydration status. Higher values mean more concentrated urine and relatively less fluid intake.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | ratio |
| LOINC | [5811-5](https://loinc.org/5811-5/) |
| Specimen | - |
| Aliases | `usg`, `urine relative density` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| hypohydration-usg-runners-2021 | warn | all | - | 1.03 | consensus | Recreational runners in a tropical setting, hydration status assessment | [Hydration Status, Fluid Intake, Sweat Rate, and Sweat Sodium Concentration in Recreational Tropical Native Runners.](https://pubmed.ncbi.nlm.nih.gov/33923890/), Methods, Urine Specific Gravity (USG) | Severe hypohydration threshold of USG >1.030 as stated in this field study, which cites its own reference [34] for the bands (that source was not retrieved). Boundary >1.030 as printed. Paired with the USG >1.020 hypohydration band in the same sentence. Source kind: decision. Source band >= 1.03; stored as the warn high bound 1.03. |
| hypohydration-usg-validation-2022 | warn | all | - | 1.02 | cohort | Athletes, hydration status assessment | [Validation of urine colour L*a*b* for assessing hydration amongst athletes.](https://pubmed.ncbi.nlm.nih.gov/36034925/), Abstract, Results | Hypohydration diagnosis threshold of USG >=1.020 as used in the cited validation study (the source writes >=; the >1.020 wording in the sports consensus hint was not verified). Compared against urine osmolality, a threshold of 700 mmol/kg is given in the same sentence. Source kind: decision. Source band >= 1.02; stored as the warn high bound 1.02. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
