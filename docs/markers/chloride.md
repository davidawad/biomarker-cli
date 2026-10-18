# Chloride (`chloride`)

Serum chloride tracks sodium and acid-base status. Both low and high values are associated with mortality in hospital cohorts, with the lowest risk at intermediate values.

| | |
|---|---|
| Category | [cmp](index.md#cmp) |
| Canonical unit | mmol/L |
| LOINC | [2075-0](https://loinc.org/2075-0/) |
| Specimen | - |
| Aliases | `cl` |

## Ranges

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 96 | 106 | - | - | uncited (legacy) | - |
| clinical | warn | all | 99 | - | cohort | Adult hospital survivors at discharge (tertiary referral hospital cohort) | [Serum Chloride Levels at Hospital Discharge and One-Year Mortality among Hospitalized Patients.](https://pubmed.ncbi.nlm.nih.gov/32438557/), Abstract | Discharge chloride at or below 99 mmol/L (strictly, <=99) was associated with higher one-year mortality, with HR 1.45 for 97-99 and 1.94 for <=96 mmol/L versus 103-105 mmol/L (reference); HR 1.41 for >=109. Source band <= 99; stored as the warn low bound 99. |
| optimal | optimal | all | 100 | 108 | cohort | Adult hospital survivors at discharge (tertiary referral hospital cohort) | [Serum Chloride Levels at Hospital Discharge and One-Year Mortality among Hospitalized Patients.](https://pubmed.ncbi.nlm.nih.gov/32438557/), Results | Discharge chloride 100-108 mmol/L was not associated with different one-year mortality; the nadir was at 103-105 mmol/L. |
| population-ri | reference | male | 98 | 108 | reference-interval-study | Adults (Turkish population, indirect reference interval, reflimR, male) | [Verification of reference intervals for routine biochemical tests using the reflimR in Turkish adults.](https://pubmed.ncbi.nlm.nih.gov/41671238/), Table 2 | Sex-specific indirect reference interval (reflimR); the manufacturer interval was 101-109 mmol/L. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | female | 99 | 108 | reference-interval-study | Adults (Turkish population, indirect reference interval, reflimR, female) | [Verification of reference intervals for routine biochemical tests using the reflimR in Turkish adults.](https://pubmed.ncbi.nlm.nih.gov/41671238/), Table 2 | Sex-specific indirect reference interval (reflimR); the manufacturer interval was 101-109 mmol/L. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 18-66 | 94 | 111 | reference-interval-study | Apparently healthy adults aged 18 years and over in Egypt (IFCC C-RIDL protocol, serum), both sexes pooled | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Central 95 % reference interval, conventional columns of the source (identical SI values). Age band 18~65 as printed in the source. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 0.077-0.5 | 103 | 114 | reference-interval-study | Healthy Chinese children aged 28 days to under 6 months (Shenzhen, check-up children; CLSI EP28) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Sub-year pediatric 95% interval from a 2026 Chinese reference-interval study (Table 1). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 0.5-18 | 102 | 111 | reference-interval-study | Healthy Chinese children aged 6 months to under 18 years (Shenzhen, check-up children; CLSI EP28) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Sub-year pediatric 95% interval from a 2026 Chinese reference-interval study (Table 1). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
