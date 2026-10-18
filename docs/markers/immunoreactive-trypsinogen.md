# Immunoreactive Trypsinogen (newborn screen) (`immunoreactive-trypsinogen`)

Trypsinogen measured in newborn dried blood spots as the first-tier test in cystic fibrosis newborn screening. A raised value triggers CFTR genetic testing or a sweat test; it is a screening marker, not a diagnostic test.

| | |
|---|---|
| Category | [pediatric](index.md#pediatric) |
| Canonical unit | ng/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| brazil-irt-irt-2024 | warn | all | - | 70 | cohort | Newborns in a Northeastern Brazil cystic fibrosis newborn screening program, using the IRT/IRT two-tier protocol | [IRT/IRT as a newborn cystic fibrosis screening method: optimal cutoff points for a mixed population.](https://pubmed.ncbi.nlm.nih.gov/39194088/), Abstract | Second-tier IRT cut-off of the current protocol in that program (70 ng/mL). The same study found that raising it to 90 ng/mL performed similarly with fewer false positives. This value is program-specific and not a general standard. Source kind: diagnostic. Source band >= 70; stored as the warn high bound 70. |
| georgia-cf-nbs-algorithm | warn | all | - | 55 | cohort | Newborns screened in the Georgia (US) cystic fibrosis newborn screening program (specimen: dried-blood-spot) | [Evaluating Georgia's Cystic Fibrosis Newborn Screening Algorithm to Inform Improvement Recommendations.](https://pubmed.ncbi.nlm.nih.gov/41133699/), Abstract | Program-specific first-tier cut-off. Values at or above 55 ng/mL trigger CFTR genetic testing. Cut-offs differ between programs and need local validation, so this is not a universal threshold. Source kind: diagnostic. Source band >= 55; stored as the warn high bound 55. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
