# Glucose Challenge Test, 1 Hour (50 g) (`glucose-challenge-1h`)

Plasma glucose one hour after a 50 g oral glucose load in pregnancy. A screening test: a positive result is followed by a diagnostic oral glucose tolerance test, not a diagnosis of gestational diabetes by itself.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | mg/dL |
| LOINC | [1504-0](https://loinc.org/1504-0/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | pregnant | - | 140 | cohort | Pregnant women undergoing the 50 g glucose challenge test for gestational diabetes screening | [Screening and diagnosing gestational diabetes mellitus.](https://pubmed.ncbi.nlm.nih.gov/24423035/), Abstract | Screen-positive at the 140 mg/dL threshold (less sensitive, more specific cut-off). Source band >= 140; stored as the warn high bound 140. |
| clinical | reference | pregnant | 130 | 140 | cohort | Pregnant women undergoing the 50 g glucose challenge test for gestational diabetes screening | [Screening and diagnosing gestational diabetes mellitus.](https://pubmed.ncbi.nlm.nih.gov/24423035/), Abstract | Screen-positive only at the more sensitive 130 mg/dL threshold (values from 130 to strictly below 140 mg/dL). Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | pregnant | - | 135 | cohort | Women with twin pregnancy at 24-28 weeks undergoing the 1-hour 50 g glucose challenge test (single maternal-fetal medicine practice, 475 patients) | [Screening approach for gestational diabetes in twin pregnancies.](https://pubmed.ncbi.nlm.nih.gov/25439813/), Abstract, Conclusion | Optimal screen-positive cut-off in twin pregnancies (single-centre historical cohort): sensitivity 100 percent at 135 mg/dL, specificity 76.4 percent. Source band >= 135; stored as the warn high bound 135. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
