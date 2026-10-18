# Immature Granulocytes (Absolute) (`immature-granulocytes-abs`)

Absolute count of immature granulocytes (myelocytes, metamyelocytes and promyelocytes) per litre of blood, reported by automated haematology analysers. Raised counts are seen with bacterial infection, sepsis, inflammation, pregnancy, glucocorticoid or chemotherapy use, and haematologic malignancy. Expected result in healthy people: within the reference interval, typically a very small count.

| | |
|---|---|
| Category | [hematology](index.md#hematology) |
| Canonical unit | 10^9/L |
| LOINC | [51584-1](https://loinc.org/51584-1/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in 10^9/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.01 | 0.04 | reference-interval-study | 25,431 routine laboratory records (16,122 females, 9309 males) from a tertiary-care hospital, Sysmex XN-1000, indirect laboratory-information-system derivation with 2.5th-97.5th percentiles; sexes pooled | [Local reference intervals for MacroR, MicroR, IG% and IG# on Sysmex XN-1000: A retrospective indirect LIS-based study.](https://pubmed.ncbi.nlm.nih.gov/42470060/), Abstract, Results | Indirect LIS-based local reference interval; the authors state it is without individual clinical verification. Sexes pooled by the source. |
| clinical | reference | age 0-11 | - | 0.04 | reference-interval-study | Outpatients up to the age of 10 years (source split at 10 years, encoded as integer ages below 11); 1-tailed upper reference limit recommended by the source for routine outpatient use; no male-female difference reported | [Age-dependent reference ranges for automated assessment of immature granulocytes and clinical significance in an outpatient setting.](https://pubmed.ncbi.nlm.nih.gov/21466364/), Abstract, Results | Upper reference limit in per-microlitre units as the source states it (40.0 per uL equals 0.04 x 10^9/L). Age split encoded as age_max = 11 for ages up to and including 10. Based on more than 2400 outpatient samples. Converted from the source's /µL (40.0) to 10^9/L using factor 0.001. |
| clinical | reference | age 11+ | - | 0.07 | reference-interval-study | Outpatients above the age of 10 years (source split at 10 years, encoded as integer ages from 11); 1-tailed upper reference limit recommended by the source for routine outpatient use; no male-female difference reported | [Age-dependent reference ranges for automated assessment of immature granulocytes and clinical significance in an outpatient setting.](https://pubmed.ncbi.nlm.nih.gov/21466364/), Abstract, Results | Upper reference limit in per-microlitre units as the source states it (70.0 per uL equals 0.07 x 10^9/L). Age split encoded as age_min = 11 for ages above 10. Based on more than 2400 outpatient samples. Converted from the source's /µL (70.0) to 10^9/L using factor 0.001. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
