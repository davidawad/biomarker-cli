# Oral Glucose Tolerance Test, 2 Hour (75 g), Pregnancy (`ogtt-2h-75g-pregnancy`)

Plasma glucose two hours after a 75 g oral glucose load, performed after an overnight fast at 24-28 weeks in women without previously diagnosed diabetes. Used to classify gestational diabetes.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | mg/dL |
| LOINC | [1518-0](https://loinc.org/1518-0/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | pregnant | - | 153 | guideline | Pregnant women without diabetes diagnosed before pregnancy, 75 g OGTT after overnight fast | [International association of diabetes and pregnancy study groups recommendations on the diagnosis and classification of hyperglycemia in pregnancy.](https://pubmed.ncbi.nlm.nih.gov/20190296/), Table 1 | Normal 2-h value under the IADPSG consensus: strictly below 153 mg/dL. Applies only if the fasting and 1-h values are also below their thresholds. |
| diagnostic | warn | pregnant | - | 153 | guideline | Pregnant women without diabetes diagnosed before pregnancy, 75 g OGTT after overnight fast | [International association of diabetes and pregnancy study groups recommendations on the diagnosis and classification of hyperglycemia in pregnancy.](https://pubmed.ncbi.nlm.nih.gov/20190296/), Table 1 | IADPSG consensus: gestational diabetes if one or more OGTT values equal or exceed the thresholds (fasting 92, 1-h 180, 2-h 153 mg/dL). This shard covers the 2-h value only. Source kind: diagnostic. Source band >= 153; stored as the warn high bound 153. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
