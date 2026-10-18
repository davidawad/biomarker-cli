# Lymphocytes (Absolute) (`lymphocytes-abs`)

Absolute count of circulating lymphocytes (T, B and NK cells). Low values (lymphopenia) occur with viral infection, steroids, chemotherapy, HIV or autoimmune disease; high values (lymphocytosis) with viral infection, pertussis or lymphoproliferative disorders.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | 10^3/µL |
| LOINC | [731-0](https://loinc.org/731-0/) |
| Specimen | - |
| Aliases | `lymphocytes`, `lymphs (absolute)` |

## Ranges

Bounds in 10^3/µL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.7 | 3.1 | - | - | uncited (legacy) | - |
| alt-1 | reference | all | 0.9 | 3 | reference-interval-study | Non-Black volunteers from pooled screening/admission data of 35 UK phase I trials, morning sampling | [Time- and Race-Specific Haematological Reference Intervals for Healthy Volunteer Trials: A Retrospective Analysis of Pooled Data From Multiple Phase I Trials.](https://pubmed.ncbi.nlm.nih.gov/32231575/), Table 5, proposed reference intervals | Race- and time-specific adjusted interval; evening interval is 1.0-3.4 (not separately encoded). Applies only to: ethnicity=nonblack. Alternative to the default time-race-specific-ri reference range for the same population. |
| barcelona-sysmex-xn | reference | all | 1.3 | 3.4 | reference-interval-study | 187 apparently healthy adults from the southern metropolitan area of Barcelona, Sysmex XN | [Reference intervals for a complete blood count on an automated haematology analyser Sysmex XN in healthy adults from the southern metropolitan area of Barcelona.](https://pubmed.ncbi.nlm.nih.gov/29765286/), Results, CBC reference intervals table | Analyser-specific interval (Sysmex XN). |
| kenya-urban-adults | reference | all | 1.29 | 3.4 | reference-interval-study | healthy adult urban population in Kenya (Nairobi), Beckman Coulter ACT 5 DIFF CP (region: Kenya) | [Complete blood count reference intervals from a healthy adult urban population in Kenya.](https://pubmed.ncbi.nlm.nih.gov/29879171/), Table, reference intervals | All-participant interval (n = 461) from the reference-interval table. |
| time-race-specific-ri | reference | all | 1 | 3 | reference-interval-study | Black volunteers from pooled screening/admission data of 35 UK phase I trials, morning sampling | [Time- and Race-Specific Haematological Reference Intervals for Healthy Volunteer Trials: A Retrospective Analysis of Pooled Data From Multiple Phase I Trials.](https://pubmed.ncbi.nlm.nih.gov/32231575/), Table 5, proposed reference intervals | Race- and time-specific adjusted interval; evening interval is 1.0-3.4 (not separately encoded). Black volunteers had higher mean lymphocyte counts than non-Black volunteers in this study. Applies only to: ethnicity=black. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
