# Placental Growth Factor (`plgf`)

Angiogenic protein produced by the placenta. Low circulating PlGF in suspected pre-eclampsia reflects placental dysfunction and is used to rule pre-eclampsia in or out, in addition to blood pressure and proteinuria.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | pg/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | pregnant, suspected-preeclampsia | 12 | 100 | cohort | Women with suspected pre-eclampsia or fetal growth restriction (MAPPLE cohort) | [Placental Growth Factor informed management of suspected pre-eclampsia or fetal growth restriction: The MAPPLE cohort study.](https://pubmed.ncbi.nlm.nih.gov/29627351/), Results | Low PlGF category (12 to 100 pg/mL), a borderline zone between the very low and normal categories. Assay-dependent. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| mapple-2018 | critical | pregnant, suspected-preeclampsia | 12 | - | cohort | Women with suspected pre-eclampsia or fetal growth restriction (MAPPLE cohort) | [Placental Growth Factor informed management of suspected pre-eclampsia or fetal growth restriction: The MAPPLE cohort study.](https://pubmed.ncbi.nlm.nih.gov/29627351/), Results | Very low PlGF category (below 12 pg/mL). Assay-dependent; values are not directly comparable across PlGF platforms. Source band <= 12; stored as the critical low bound 12. |
| pelican-stepped-wedge-2019 | warn | pregnant, suspected-preeclampsia | 100 | - | cohort | Women presenting with suspected pre-eclampsia before 35 weeks of gestation, concealed-PlGF arm of a stepped-wedge cluster RCT | [Placental growth factor testing to assess women with suspected pre-eclampsia: a multicentre, pragmatic, stepped-wedge cluster-randomised controlled trial.](https://pubmed.ncbi.nlm.nih.gov/30948284/), Findings | Low PlGF (below 100 pg/mL) predicts pre-eclampsia requiring delivery within 14 days: high sensitivity and negative predictive value, lower specificity. Decision cut-off, assay-dependent. Applies only to: gestational_week_min=20; gestational_week_max=34. Source kind: diagnostic. Source band <= 100; stored as the warn low bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
