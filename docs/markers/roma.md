# ROMA (Risk of Ovarian Malignancy Algorithm) (`roma`)

Risk of Ovarian Malignancy Algorithm: a calculated index from HE4 and CA125 combined with menopausal status, used to estimate ovarian malignancy risk in women with a pelvic mass. Expected result: a percentage; a low-risk result is below the cited menopause-specific cut-off.

| | |
|---|---|
| Category | [tumor-marker](index.md#tumor-marker) |
| Canonical unit | % |
| LOINC | [104551-7](https://loinc.org/104551-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | female, pre-menopausal, pelvic-mass | 6 | - | cohort | Premenopausal women with a pelvic mass scheduled for conservative management or surgery (cohort in the cited study) | [Assessing the risk of ovarian malignancy algorithm for the conservative management of women with a pelvic mass.](https://pubmed.ncbi.nlm.nih.gov/26364809/), Full text (PMC4868067), ROMA score low-risk statement | Premenopausal low-risk cut-off as stated by the source: ROMA below 6% (strictly below 6 is low risk). Source band <= 6; stored as the warn low bound 6. |
| clinical | warn | female, post-menopausal, pelvic-mass | 10 | - | cohort | Postmenopausal women with a pelvic mass scheduled for conservative management or surgery (cohort in the cited study) | [Assessing the risk of ovarian malignancy algorithm for the conservative management of women with a pelvic mass.](https://pubmed.ncbi.nlm.nih.gov/26364809/), Full text (PMC4868067), ROMA score low-risk statement | Postmenopausal low-risk cut-off as stated by the source: ROMA below 10% (strictly below 10 is low risk). Source band <= 10; stored as the warn low bound 10. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
