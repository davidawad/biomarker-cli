# Hemoglobin A2 (`hba2`)

Share of total haemoglobin present as HbA2 (alpha2 delta2 globin), measured by electrophoresis, HPLC or capillary methods. Tracked mainly for beta-thalassemia carrier screening, where a raised HbA2 is the key finding. Expected result: within the reference interval for the population and method.

| | |
|---|---|
| Category | [hematology](index.md#hematology) |
| Canonical unit | % |
| LOINC | [4551-8](https://loinc.org/4551-8/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 2.3 | 3.1 | reference-interval-study | 832 individuals of reproductive age without hypochromic microcytic anaemia, Guizhou population (China); overall normal individuals, sex and pregnancy status reported to have no obvious influence | [Reference intervals for HbA2 and HbF and cut-off value of HbA2 for β-thalassemia carrier screening in a Guizhou population of reproductive age.](https://pubmed.ncbi.nlm.nih.gov/30503531/), Abstract, Results | Single-centre reference interval study. |
| alt-1 | reference | all | 2.3 | 3.4 | reference-interval-study | 3306 individuals aged 3 to 92 years included in the reference interval calculation, Northern Alberta (retrospective laboratory data, Alberta) | [Incidence of hemoglobinopathies and thalassemias in Northern Alberta. Establishment of reference intervals for HbF and HbA2.](https://pubmed.ncbi.nlm.nih.gov/25869492/), Abstract, Results | Retrospective laboratory-based reference interval study. Alternative to the default clinical reference range for the same population. |
| diagnostic | warn | male, beta-thalassemia-carrier | - | 4.4 | cohort | Males of reproductive age, Guizhou population (China), with HbA2 and beta-globin gene analysis | [Reference intervals for HbA2 and HbF and cut-off value of HbA2 for β-thalassemia carrier screening in a Guizhou population of reproductive age.](https://pubmed.ncbi.nlm.nih.gov/30503531/), Abstract, Results | ROC-derived HbA2 cut-off for beta-thalassemia carrier screening in males. Single-study classification, not a guideline cut-off. Source kind: diagnostic. Source band >= 4.4; stored as the warn high bound 4.4. |
| diagnostic | warn | female, not pregnant, beta-thalassemia-carrier | - | 3.75 | cohort | Non-pregnant women of reproductive age, Guizhou population (China), with HbA2 and beta-globin gene analysis | [Reference intervals for HbA2 and HbF and cut-off value of HbA2 for β-thalassemia carrier screening in a Guizhou population of reproductive age.](https://pubmed.ncbi.nlm.nih.gov/30503531/), Abstract, Results | ROC-derived HbA2 cut-off for beta-thalassemia carrier screening in non-pregnant women. Single-study classification, not a guideline cut-off. Source kind: diagnostic. Source band >= 3.75; stored as the warn high bound 3.75. |
| diagnostic | warn | female, pregnant, beta-thalassemia-carrier | - | 3.7 | cohort | Pregnant women, Guizhou population (China), with HbA2 and beta-globin gene analysis | [Reference intervals for HbA2 and HbF and cut-off value of HbA2 for β-thalassemia carrier screening in a Guizhou population of reproductive age.](https://pubmed.ncbi.nlm.nih.gov/30503531/), Abstract, Results | ROC-derived HbA2 cut-off for beta-thalassemia carrier screening in pregnant women. Single-study classification, not a guideline cut-off. Source kind: diagnostic. Source band >= 3.7; stored as the warn high bound 3.7. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
