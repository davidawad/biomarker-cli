# Fetal Fibronectin, Cervicovaginal (`fetal-fibronectin`)

Glycoprotein that acts as glue between decidua and fetal membranes; detected in cervicovaginal secretions. A positive test in women with symptoms or a high-risk history signals increased short-term risk of preterm birth or impending labour.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | ng/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| fibronectin-asymptomatic-2016 | warn | pregnant, asymptomatic-high-risk | 10 | - | cohort | Asymptomatic high-risk pregnant women tested at 18 and 22 weeks (quantitative fFN) (specimen: cervicovaginal) | [Quantitative Fetal Fibronectin at 18 Weeks of Gestation to Predict Preterm Birth in Asymptomatic High-Risk Women.](https://pubmed.ncbi.nlm.nih.gov/26942351/), Results | Below 10 ng/mL: high negative predictive value for delivery before 34 weeks. Applies only to: gestational_week_min=18; gestational_week_max=22. Source kind: diagnostic. Source band <= 10; stored as the warn low bound 10. |
| fibronectin-asymptomatic-2016 | critical | pregnant, asymptomatic-high-risk | - | 200 | cohort | Asymptomatic high-risk pregnant women tested at 18 and 22 weeks (quantitative fFN) (specimen: cervicovaginal) | [Quantitative Fetal Fibronectin at 18 Weeks of Gestation to Predict Preterm Birth in Asymptomatic High-Risk Women.](https://pubmed.ncbi.nlm.nih.gov/26942351/), Results | 200 ng/mL or more: high specificity but lower sensitivity for delivery before 34 weeks. Applies only to: gestational_week_min=18; gestational_week_max=22. Source band >= 200; stored as the critical high bound 200. |
| fibronectin-meta-2020 | warn | pregnant | - | 50 | cohort | Pregnant women tested for impending labour or preterm birth (systematic review and meta-analysis) (specimen: cervicovaginal) | [The fetal fibronectin test: 25 years after its development, what is the evidence regarding its clinical utility? A systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/29914277/), Abstract | Qualitative positive test at 50 ng/mL or more between 22 and 36 weeks. Source window is 22 to 36 weeks, not 22 to 35. Applies only to: gestational_week_min=22; gestational_week_max=36. Source kind: diagnostic. Source band >= 50; stored as the warn high bound 50. |
| fibronectin-symptomatic-2013 | warn | pregnant, symptomatic-preterm-labour | 10 | - | cohort | Symptomatic women with threatened preterm labour (quantitative fFN test) (specimen: cervicovaginal) | [Evaluation of a quantitative fetal fibronectin test for spontaneous preterm birth in symptomatic women.](https://pubmed.ncbi.nlm.nih.gov/23164760/), Abstract, Conclusion | Below 10 ng/mL: very low risk of spontaneous preterm birth (negative predictive threshold). Source kind: diagnostic. Source band <= 10; stored as the warn low bound 10. |
| fibronectin-symptomatic-2013 | critical | pregnant, symptomatic-preterm-labour | - | 200 | cohort | Symptomatic women with threatened preterm labour (quantitative fFN test) (specimen: cervicovaginal) | [Evaluation of a quantitative fetal fibronectin test for spontaneous preterm birth in symptomatic women.](https://pubmed.ncbi.nlm.nih.gov/23164760/), Abstract, Conclusion | 200 ng/mL or more: high-risk threshold for spontaneous preterm birth, with high specificity and lower sensitivity. Source band >= 200; stored as the critical high bound 200. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
