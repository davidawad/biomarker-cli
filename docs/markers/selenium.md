# Selenium, Serum/Plasma (`selenium`)

Serum or plasma selenium, the marker of selenium status used with selenoprotein P and glutathione peroxidase activity. Low status is linked to mortality, immune and cognitive outcomes, and high status is not automatically beneficial, so the U-shaped relation with status matters.

| | |
|---|---|
| Category | [mineral](index.md#mineral) |
| Canonical unit | µg/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in µg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 90 | 125 | consensus | Healthy people (plasma or serum selenium), as described in a critical-care review | [Pharmaconutrition revisited for critically ill patients with coronavirus disease 2019 (COVID-19): Does selenium have a place?](https://pubmed.ncbi.nlm.nih.gov/33049573/), Discussion (selenium kinetics paragraph) | The source describes 90-125 ug/L as the normal physiologic range associated with maximal GPX3 activity (daily intake 55-70 ug). It is a narrative citation of earlier work, not a measured reference interval. Assay and selenoprotein P status are not stated. |
| clinical | warn | all | - | 120 | cohort | Adults (observational and dose-response data) | [Selenium Species in Diabetes Mellitus Type 2.](https://pubmed.ncbi.nlm.nih.gov/37880477/), Discussion (dose-response paragraph) | Blood selenium at or above 120 ug/L was associated with increased risk of type 2 diabetes in the source's pooled observational data. This is a risk signal from observational studies, not a treatment or toxicity limit. Source band >= 120; stored as the warn high bound 120. |
| combs-2015 | optimal | all | 50 | 70 | consensus | Adults in general; a review of selenium biomarkers based on plasma and selenoprotein data (specimen: plasma) | [Biomarkers of selenium status.](https://pubmed.ncbi.nlm.nih.gov/25835046/), Section on plasma selenium and selenoproteins | Plasma selenium at which GPX3 and SEPP1 selenoprotein expression reaches its maximum (plateau). The same review also quotes a point value of about 80 ng/mL and gives no deficiency cut-off. Not a clinical reference interval. |
| epic-reference-2020 | warn | all | 45.7 | - | cohort | Adults with COVID-19 compared with the EPIC reference population (n=1915) for total serum selenium (specimen: serum) | [Selenium Deficiency Is Associated with Mortality Risk from COVID-19.](https://pubmed.ncbi.nlm.nih.gov/32708526/), Results, paragraph 1 | Authors' low-status cut-off: below the 2.5th percentile of the EPIC reference population. It is a reference-derived lower limit, not a clinical deficiency cut-off. The same paper gives only mean and SD for the reference group. Source kind: deficient. Source band <= 45.7; stored as the warn low bound 45.7. |
| us-epa-2002 | warn | all | - | 1000 | guideline | Adults with selenium exposure; threshold taken from the US EPA (2002) value quoted in a 2020 review (specimen: whole-blood) | [Selenium intake, status, and health: a complex relationship.](https://pubmed.ncbi.nlm.nih.gov/31388899/), Section on mercury exposure in the Brazilian Amazon | Whole-blood selenium considered toxic. Value cited secondhand from a review (Rayman 2020); the primary EPA document was not retrieved. Source kind: toxic. Source band >= 1000; stored as the warn high bound 1000. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
