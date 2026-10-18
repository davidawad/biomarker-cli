# Aldosterone-to-Renin Ratio (`aldosterone-renin-ratio`)

Ratio of plasma aldosterone to renin, the primary screening index for primary aldosteronism in hypertensive patients. A high ratio suggests renin-independent aldosterone excess; cut-offs depend on the renin assay, aldosterone method and age.

| | |
|---|---|
| Category | [adrenal](index.md#adrenal) |
| Canonical unit | ng/dL per ng/mL/h |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/dL per ng/mL/h. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 19 | cohort | Chinese adults at high risk of primary aldosteronism at seven tertiary centres (117 PA, 354 essential hypertension) (method: lc-ms/ms; test: seated saline infusion test) | [[Establishing LC-MS/MS-based cutoffs for the confirmatory testing for primary aldosteronism: a prospective multi-center study].](https://pubmed.ncbi.nlm.nih.gov/41633645/), Abstract, Results | ARR after the seated saline infusion test, LC-MS/MS (sensitivity 64%, specificity 85%). Study-derived threshold. Source kind: diagnostic. Source band >= 19.0; stored as the warn high bound 19.0. Alternative to the default china-multicenter-pa-2026 warn range for the same population. |
| china-multicenter-pa-2026 | warn | all | - | 9.3 | cohort | Chinese adults at high risk of primary aldosteronism at seven tertiary centres (117 PA, 354 essential hypertension) (method: lc-ms/ms; test: captopril challenge test, 2 h post-dose) | [[Establishing LC-MS/MS-based cutoffs for the confirmatory testing for primary aldosteronism: a prospective multi-center study].](https://pubmed.ncbi.nlm.nih.gov/41633645/), Abstract, Results | ARR two hours after the captopril challenge, LC-MS/MS (sensitivity 61%, specificity 85%). Study-derived threshold; the 19.0 seated-saline cut-off is a separate range. Source kind: diagnostic. Source band >= 9.3; stored as the warn high bound 9.3. |
| endocrine-society-2025 | warn | all | - | 15 | consensus | Adults screened for primary aldosteronism; ARR with PAC and PRA by LC-MS/MS (Endocrine Society 2025 criteria as summarised in a 2026 analysis) (method: lc-ms/ms) | [Evaluation of Current Screening Thresholds for the Detection of Primary Aldosteronism.](https://pubmed.ncbi.nlm.nih.gov/42751802/), Abstract, Background | Endocrine Society 2025 screening criterion: ARR above 15, applied together with PAC at or above 7.5 ng/dL and PRA at or below 1 ng/mL/h. Cited via the Hypertension 2026 analysis, not the guideline full text. Applies only to: markers=[{"slug": "aldosterone", "op": ">=", "value": 7.5, "unit": "ng/dL"}, {"slug": "renin-activity", "op": "<=", "value": 1, "unit": "ng/mL/h"}]. Source kind: diagnostic. Source band >= 15; stored as the warn high bound 15. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
