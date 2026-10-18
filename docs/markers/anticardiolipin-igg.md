# Anticardiolipin IgG (`anticardiolipin-igg`)

IgG antibodies against cardiolipin, a phospholipid. Persistent medium or high titres are laboratory criteria for antiphospholipid syndrome and raise thrombotic risk.

| | |
|---|---|
| Category | [autoimmune](index.md#autoimmune) |
| Canonical unit | GPL/mL |
| LOINC | [3181-5](https://loinc.org/3181-5/) |
| Specimen | - |
| Aliases | `acl igg`, `anticardiolipin antibody igg` |

## Ranges

Bounds in GPL/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 40 | 79 | cohort | Patients in a Turkish cohort used to validate the 2023 ACR/EULAR APS classification criteria | [The Validation of the 2023 ACR/EULAR Antiphospholipid Syndrome Classification Criteria in a Cohort from Turkey.](https://pubmed.ncbi.nlm.nih.gov/39410609/), Methods, aPL antibody positivity definition | 2023 ACR/EULAR APS criteria: moderate aPL positivity. The source reports the titre in 'units' (aPL ELISA units) and does not state GPL/mL explicitly; treat the equivalence as unverified. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| acr-eular-2023-aps | warn | all | - | 80 | cohort | Patients in a Turkish cohort used to validate the 2023 ACR/EULAR APS classification criteria | [The Validation of the 2023 ACR/EULAR Antiphospholipid Syndrome Classification Criteria in a Cohort from Turkey.](https://pubmed.ncbi.nlm.nih.gov/39410609/), Methods, aPL antibody positivity definition | 2023 ACR/EULAR APS criteria: high aPL positivity (>=80). The source reports the titre in 'units' and does not state GPL/mL explicitly; treat the equivalence as unverified. Source kind: decision. Source band >= 80; stored as the warn high bound 80. |
| sydney-2006 | warn | all | - | 40 | consensus | Patients suspected of antiphospholipid syndrome (Sydney 2006 laboratory criteria) | [2019 European League Against Rheumatism/American College of Rheumatology Classification Criteria for Systemic Lupus Erythematosus.](https://pubmed.ncbi.nlm.nih.gov/31385462/), Full text, antiphospholipid antibody criterion | Medium or high titre is >40 GPL or MPL, or above the 99th percentile of the laboratory's own control population (percentile not expressed as a number). Boundary is > 40. The quote is taken from the 2019 EULAR/ACR SLE criteria paper, which restates the Sydney criteria; the 2006 consensus full text was not retrievable. Source kind: decision. Source band >= 40; stored as the warn high bound 40. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
