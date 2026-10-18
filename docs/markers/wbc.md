# White Blood Cells (`wbc`)

Total leukocyte count in blood, the sum of neutrophils, lymphocytes, monocytes, eosinophils and basophils. Low values (leukopenia) occur with marrow failure, drugs, infection, or benign ethnic / Duffy-null neutropenia; high values (leukocytosis) occur with infection, inflammation, stress, steroids or leukaemia.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | 10^3/µL |
| LOINC | [6690-2](https://loinc.org/6690-2/) |
| Specimen | - |
| Aliases | `white blood cell count`, `leukocytes` |

## Ranges

Bounds in 10^3/µL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 3.4 | 10.8 | - | - | uncited (legacy) | - |
| optimal | optimal | all | 4 | 7.5 | - | - | uncited (legacy) | - |
| blueprint | optimal | all | 3.5 | 6 | - | - | uncited (legacy) | - |
| alt-1 | reference | all | 3 | 9.1 | reference-interval-study | Non-Black volunteers from pooled screening/admission data of 35 UK phase I trials, morning sampling | [Time- and Race-Specific Haematological Reference Intervals for Healthy Volunteer Trials: A Retrospective Analysis of Pooled Data From Multiple Phase I Trials.](https://pubmed.ncbi.nlm.nih.gov/32231575/), Table 5, proposed reference intervals | Adjusted race- and time-of-day-specific interval; the evening interval is 3.9-10.9 (not separately encoded). Applies only to: ethnicity=nonblack. Alternative to the default time-race-specific-ri reference range for the same population. |
| barcelona-sysmex-xn | reference | all | 3.9 | 9.5 | reference-interval-study | 191 apparently healthy adults (n = 188 for leukocytes) from the southern metropolitan area of Barcelona, K3EDTA blood on Sysmex XN, CLSI EP28-A3C | [Reference intervals for a complete blood count on an automated haematology analyser Sysmex XN in healthy adults from the southern metropolitan area of Barcelona.](https://pubmed.ncbi.nlm.nih.gov/29765286/), Results, CBC reference intervals table | Analyser-specific interval (Sysmex XN); validation by each laboratory is recommended by the authors. |
| duffy-null-ri | reference | duffy-null | 2.43 | 7.05 | reference-interval-study | 122 healthy adults with Duffy-null phenotype Fy(a-b-), identified from 1695 adults with CBC and RBC phenotyping (2020-2026) | [Establishing Reference Intervals for White Blood Cell and Absolute Neutrophil Counts in Duffy Null Individuals.](https://pubmed.ncbi.nlm.nih.gov/42345734/), Abstract, Results | Non-parametric 95% interval per CLSI; the Duffy-null median WBC (3.24) was significantly below the institutional median (6.65). |
| kenya-urban-adults | reference | all | 3.08 | 7.83 | reference-interval-study | healthy adult urban population in Kenya (Nairobi, PathCare laboratory), Beckman Coulter ACT 5 DIFF CP (region: Kenya) | [Complete blood count reference intervals from a healthy adult urban population in Kenya.](https://pubmed.ncbi.nlm.nih.gov/29879171/), Table, reference intervals | All-participant interval (n = 464) from the reference-interval table; the paper also gives sex-specific intervals. |
| norip | reference | age 18-90 | 3.5 | 8.8 | reference-interval-study | 1826 healthy Nordic adults aged 18-90 (NORIP), EDTA venous blood, 12 analyser types | [A multicentre study of reference intervals for haemoglobin, basic blood cell counts and erythrocyte indices in the adult population of the Nordic countries.](https://pubmed.ncbi.nlm.nih.gov/15223702/), Abstract | B-Lkc, non-parametric 2.5th-97.5th percentiles; no sex-specific interval was derived. |
| time-race-specific-ri | reference | all | 2.7 | 7.3 | reference-interval-study | Black volunteers from pooled screening/admission data of 35 UK phase I trials (7,157 volunteers), morning sampling | [Time- and Race-Specific Haematological Reference Intervals for Healthy Volunteer Trials: A Retrospective Analysis of Pooled Data From Multiple Phase I Trials.](https://pubmed.ncbi.nlm.nih.gov/32231575/), Table 5, proposed reference intervals | Adjusted race- and time-of-day-specific interval; the evening interval for the same group is 2.9-8.7 (not separately encoded). Quoted range is the morning interval. Applies only to: ethnicity=black. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
