# ZnT8 Autoantibodies (`znt8-antibodies`)

IgG autoantibodies against zinc transporter 8 (ZnT8), a beta-cell antigen, measured by immunoassay (ELISA or radiobinding). Tracked as one of the islet autoantibodies (with GADA, IA-2A, IAA) to screen for and classify type 1 diabetes, and to tell autoimmune diabetes from type 2 diabetes in young people. Expected result: negative (below the age-specific cut-off).

| | |
|---|---|
| Category | [autoimmune](index.md#autoimmune) |
| Canonical unit | WHO U/mL |
| LOINC | [76651-9](https://loinc.org/76651-9/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in WHO U/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 0-30 | - | 18 | reference-interval-study | Control sera from people without diabetes and with HbA1c below 6.0% (<42 mmol/mol), aged 0-83 years; participants tested under 30 years | [Zinc transporter 8 autoantibody testing requires age-related cut-offs.](https://pubmed.ncbi.nlm.nih.gov/34348918/), Abstract, Results | 97.5th centile of control sera for people tested under 30 years, as stated by the source. |
| clinical | reference | age 30+ | - | 9 | reference-interval-study | Control sera from people without diabetes and with HbA1c below 6.0% (<42 mmol/mol), aged 0-83 years; participants tested 30 years and over | [Zinc transporter 8 autoantibody testing requires age-related cut-offs.](https://pubmed.ncbi.nlm.nih.gov/34348918/), Abstract, Results | 97.5th centile of control sera for people tested 30 years and over, as stated by the source. |
| diagnostic | warn | age 0-30 | 127 | - | reference-interval-study | Control sera from people without diabetes and with HbA1c below 6.0% (<42 mmol/mol), aged 0-83 years; participants tested under 30 years | [Zinc transporter 8 autoantibody testing requires age-related cut-offs.](https://pubmed.ncbi.nlm.nih.gov/34348918/), Abstract, Conclusions | 99th centile of control sera for people tested under 30 years, which the source recommends as the positivity cut-off for ZnT8A testing in this age group. Upper bound stated as a cut-off; the source does not state whether the boundary is >= or >. Source kind: diagnostic. Source band <= 127; stored as the warn low bound 127. |
| diagnostic | warn | age 30+ | 21 | - | reference-interval-study | Control sera from people without diabetes and with HbA1c below 6.0% (<42 mmol/mol), aged 0-83 years; participants tested 30 years and over | [Zinc transporter 8 autoantibody testing requires age-related cut-offs.](https://pubmed.ncbi.nlm.nih.gov/34348918/), Abstract, Conclusions | 99th centile of control sera for people tested 30 years and over, which the source recommends as the positivity cut-off for ZnT8A testing in this age group. Upper bound stated as a cut-off; the source does not state whether the boundary is >= or >. Source kind: diagnostic. Source band <= 21; stored as the warn low bound 21. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
