# Anti-Beta2-Glycoprotein I IgG (`b2gp1-igg`)

IgG antibodies against beta2-glycoprotein I, a plasma protein that binds phospholipids. Persistent medium or high titres are antiphospholipid syndrome criteria and are linked to thrombosis risk.

| | |
|---|---|
| Category | [autoimmune](index.md#autoimmune) |
| Canonical unit | U/mL |
| LOINC | [44448-9](https://loinc.org/44448-9/) |
| Specimen | - |
| Aliases | `anti-beta2gpi igg`, `anti-β2gpi igg`, `anti-beta2-glycoprotein i igg`, `anti-beta2-glycoprotein-igg` |

## Ranges

Bounds in U/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 40 | 79 | cohort | Patients in a Turkish cohort used to validate the 2023 ACR/EULAR APS classification criteria | [The Validation of the 2023 ACR/EULAR Antiphospholipid Syndrome Classification Criteria in a Cohort from Turkey.](https://pubmed.ncbi.nlm.nih.gov/39410609/), Methods, aPL antibody positivity definition | 2023 ACR/EULAR APS criteria: moderate aPL positivity, applied to anti-beta2GPI as well as anticardiolipin. The source reports the titre in 'units' and does not state U/mL explicitly; treat the equivalence as unverified. Sydney 2006 uses a >99th percentile cut-off that is not expressed numerically in the retrieved text. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| acr-eular-2023-aps | warn | all | - | 80 | cohort | Patients in a Turkish cohort used to validate the 2023 ACR/EULAR APS classification criteria | [The Validation of the 2023 ACR/EULAR Antiphospholipid Syndrome Classification Criteria in a Cohort from Turkey.](https://pubmed.ncbi.nlm.nih.gov/39410609/), Methods, aPL antibody positivity definition | 2023 ACR/EULAR APS criteria: high aPL positivity (>=80), applied to anti-beta2GPI as well as anticardiolipin. The source reports the titre in 'units' and does not state U/mL explicitly; treat the equivalence as unverified. Source kind: decision. Source band >= 80; stored as the warn high bound 80. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
