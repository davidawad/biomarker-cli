# Beta-2 Microglobulin (`beta-2-microglobulin`)

Beta-2 microglobulin, a small protein on the surface of most cells that is released into blood. It is cleared by the kidneys, so it rises with reduced kidney function, and in multiple myeloma and some lymphomas it is used for prognostic staging.

| | |
|---|---|
| Category | [tumour](index.md#tumour) |
| Canonical unit | mg/L |
| LOINC | [1952-1](https://loinc.org/1952-1/) |
| Specimen | - |
| Aliases | `b2m`, `beta2-microglobulin`, `β2-microglobulin`, `beta-2 microglobulin` |

## Ranges

Bounds in mg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 29-50 | 1.02 | 2.46 | reference-interval-study | Healthy blood donors aged 29-49 years (immunonephelometry) | [Serum beta2-microglobulin measured by immunonephelometry: expression patterns and reference intervals in healthy adults.](https://pubmed.ncbi.nlm.nih.gov/19397485/), Abstract, Results | Non-parametric reference interval for younger adults, measured by immunonephelometry in 183 healthy blood donors aged 29-75 years. Method-specific; the upper limit rises with age (see the 50-75 year band). |
| alt-1 | warn | all | - | 5.5 | cohort | Newly diagnosed multiple myeloma patients (International Staging System derivation and validation) | [International staging system for multiple myeloma.](https://pubmed.ncbi.nlm.nih.gov/15809451/), Abstract, Results | ISS stage III (highest risk) is beta-2 microglobulin >= 5.5 mg/L regardless of albumin. Boundary is >= 5.5 mg/L. Median survival 29 months in the original cohort. Source kind: decision. Source band >= 5.5; stored as the warn high bound 5.5. Alternative to the default iss-2005 warn range for the same population. |
| b2m-healthy-adults-2009 | reference | age 50-76 | 1.29 | 2.7 | reference-interval-study | Healthy blood donors aged 50-75 years (immunonephelometry) | [Serum beta2-microglobulin measured by immunonephelometry: expression patterns and reference intervals in healthy adults.](https://pubmed.ncbi.nlm.nih.gov/19397485/), Abstract, Results | Non-parametric reference interval for older adults, measured by immunonephelometry in healthy blood donors. Method-specific. |
| iss-2005 | warn | all | 3.5 | - | cohort | Newly diagnosed multiple myeloma patients (International Staging System derivation and validation) | [International staging system for multiple myeloma.](https://pubmed.ncbi.nlm.nih.gov/15809451/), Abstract, Results | ISS stage I (lowest risk) requires beta-2 microglobulin below 3.5 mg/L together with serum albumin >= 3.5 g/dL. Boundary is < 3.5 mg/L (strictly less than). Stage I median survival 62 months in the original cohort. Applies only to: markers={"albumin": ">=3.5"}. Source kind: decision. Source band <= 3.5; stored as the warn low bound 3.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
