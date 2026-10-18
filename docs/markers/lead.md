# Blood lead (`lead`)

Lead concentration in whole blood, the standard marker of recent or ongoing lead exposure. Higher values indicate exposure and, in children, risk of neurodevelopmental harm; no level is considered safe.

| | |
|---|---|
| Category | [toxic-metal](index.md#toxic-metal) |
| Canonical unit | µg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | `lead-blood`, `blood-lead`, `blood lead` |

## Ranges

Bounds in µg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 1-6 | - | 3.5 | guideline | US children aged 1-5 years (NHANES 2015-2016 and 2017-2018 cycles) | [Update of the Blood Lead Reference Value - United States, 2021.](https://pubmed.ncbi.nlm.nih.gov/34710078/), Policy Update | Blood lead reference value (BLRV) set at the 97.5th percentile of the child distribution. It is a population benchmark, not a health-based standard or toxicity threshold. Previously 5 µg/dL (2012). |
| cdc-blood-lead-clinical-guidance | warn | age 0-18 | - | 19 | guideline | Children with a venous blood lead level reported to clinicians (CDC clinical guidance for healthcare providers) | [CDC Clinical Guidance for Healthcare Providers: Lead Exposure in Children (web page)](https://www.cdc.gov/lead-prevention/hcp/clinical-guidance/index.html), Clinical guidance table, BLL tiers | Management tier: environmental history, nutrition counseling, follow-up testing and public health reporting. Age limit is an assumption of the pediatric guidance scope. Source band 3.5-19; stored as the warn high bound 19. |
| cdc-blood-lead-clinical-guidance | critical | age 0-18 | - | 45 | guideline | Children with a venous blood lead level, CDC clinical guidance for healthcare providers | [CDC Clinical Guidance for Healthcare Providers: Lead Exposure in Children (web page)](https://www.cdc.gov/lead-prevention/hcp/clinical-guidance/index.html), Clinical guidance table, BLL tiers | Highest management tier; CDC calls for urgent medical management of very high levels. Source band >= 45; stored as the critical high bound 45. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
