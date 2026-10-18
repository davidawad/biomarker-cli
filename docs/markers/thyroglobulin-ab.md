# Thyroglobulin Antibodies (`thyroglobulin-ab`)

Autoantibodies against thyroglobulin. In differentiated thyroid cancer follow-up they can interfere with serum thyroglobulin immunoassays, so they are measured alongside Tg.

| | |
|---|---|
| Category | [thyroid](index.md#thyroid) |
| Canonical unit | IU/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in IU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 10 | consensus | Patients with differentiated thyroid cancer (review of TgAb measurement and interpretation) | [Thyroglobulin and thyroglobulin antibodies in differentiated thyroid cancer: interpretation, challenges, and future perspectives.](https://pubmed.ncbi.nlm.nih.gov/42047636/), Results | Institutional cut-off (ICO) for TgAb positivity, based on NACB guidance as cited; higher-titre TgAb interferes with immunoassay Tg. Assay dependent. Source kind: diagnostic. Source band >= 10; stored as the warn high bound 10. Alternative to the default tgab-cutoffs-dtc-review-2026 warn range for the same population. |
| ptc-lymph-node-cohort-2026 | warn | papillary-thyroid-carcinoma | 115 | - | cohort | Patients with persistent or recurrent papillary thyroid carcinoma in cervical lymph nodes (retrospective cohort) | [Predictors of false-negative serum thyroglobulin in persistent/recurrent Papillary Thyroid Carcinoma cervical lymph nodes.](https://pubmed.ncbi.nlm.nih.gov/42388622/), Results | Study-defined TgAb-negative range (<=115 IU/mL); used to stratify false-negative serum Tg, not a general reference limit. Source kind: diagnostic. Source band <= 115; stored as the warn low bound 115. |
| tgab-cutoffs-dtc-review-2026 | warn | all | - | 4.11 | consensus | Patients with differentiated thyroid cancer (review of TgAb measurement and interpretation) | [Thyroglobulin and thyroglobulin antibodies in differentiated thyroid cancer: interpretation, challenges, and future perspectives.](https://pubmed.ncbi.nlm.nih.gov/42047636/), Results | Manufacturer cut-off (MCO) for TgAb positivity as quoted in the review; assay dependent. Source kind: diagnostic. Source band >= 4.11; stored as the warn high bound 4.11. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
