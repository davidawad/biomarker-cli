# Plasma Renin Activity (`renin-activity`)

Measure of the enzymatic generation of angiotensin I by renin in plasma. Low (suppressed) renin with high aldosterone points to primary aldosteronism; renin is also monitored during mineralocorticoid receptor antagonist therapy.

| | |
|---|---|
| Category | [adrenal](index.md#adrenal) |
| Canonical unit | ng/mL/h |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL/h. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| endocrine-society-2025 | warn | all | 1 | - | consensus | Adults screened for primary aldosteronism; PRA and PAC by LC-MS/MS (Endocrine Society 2025 criteria as summarised in a 2026 analysis) (method: lc-ms/ms) | [Evaluation of Current Screening Thresholds for the Detection of Primary Aldosteronism.](https://pubmed.ncbi.nlm.nih.gov/42751802/), Abstract, Background | Suppressed renin for screening: PRA at or below 1 ng/mL/h, applied together with PAC at or above 7.5 ng/dL and ARR above 15. Applies only to: markers=[{"slug": "aldosterone", "op": ">=", "value": 7.5, "unit": "ng/dL"}]. Source kind: diagnostic. Source band <= 1; stored as the warn low bound 1. |
| renin-status-outcomes-2025 | optimal | all | - | 1 | cohort | Adults with primary aldosteronism on medical treatment with mineralocorticoid receptor antagonists (24 studies, 6621 patients) | [Post-treatment renin status and cardiovascular, renal, and mortality outcomes in medically treated primary aldosteronism: a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/41235994/), Abstract, Findings | Renin cut-off used in most studies to classify post-treatment renin as suppressed (below) or unsuppressed (at or above). Unsuppressed renin was associated with fewer cardiovascular events; the authors suggest renin normalisation as a possible treatment target, which prospective trials still need to confirm. Applies only to: on_treatment=mineralocorticoid receptor antagonist. Source kind: target. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
