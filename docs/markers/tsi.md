# Thyroid Stimulating Immunoglobulin (TSI) (`tsi`)

Thyroid-stimulating immunoglobulin (TSI) bioassay: functional autoantibody that activates the TSH receptor, reported as a percentage of the assay reference. Tracked to diagnose Graves' disease, to predict relapse after antithyroid drug withdrawal, and to follow thyroid eye disease activity. No published healthy-population reference interval was verified; expected result in a person without Graves' disease is below the assay positivity cut-off.

| | |
|---|---|
| Category | [thyroid](index.md#thyroid) |
| Canonical unit | % |
| LOINC | [30567-2](https://loinc.org/30567-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | graves-disease | - | 66.5 | cohort | Graves' disease patients after antithyroid drug withdrawal (single retrospective cohort, ROC analysis) | [The Prediction Model Using Thyroid-stimulating Immunoglobulin Bioassay For Relapse of Graves' Disease.](https://pubmed.ncbi.nlm.nih.gov/35441120/), Results (Clinical values of the TBII assay and the TSI bioassay) | Relapse prediction cut-off from the cited study's ROC analysis (57.0% sensitivity, 60.9% specificity, AUC 0.557): weak discrimination, not recommended as a decision threshold. Source band >= 66.5; stored as the warn high bound 66.5. |
| diagnostic | warn | thyroid-eye-disease | - | 140 | cohort | Patients with recent-onset symptomatic thyroid eye disease (single cohort, cited study); TSI result expressed as specimen-to-reference ratio | [Epidemiology of thyroid-stimulating immunoglobulin in recent-onset symptomatic thyroid eye disease.](https://pubmed.ncbi.nlm.nih.gov/38847819/), Methods (TSI measurement) | Positivity cut-off used by the cited study (result considered positive at or above 140%). Single-study classification, not a guideline cut-off. Source kind: diagnostic. Source band >= 140; stored as the warn high bound 140. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
