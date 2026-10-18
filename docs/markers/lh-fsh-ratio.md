# LH/FSH Ratio (`lh-fsh-ratio`)

Ratio of serum luteinizing hormone to follicle-stimulating hormone, measured on the same sample. Tracked as a supporting marker for polycystic ovary syndrome (PCOS), where LH is often elevated relative to FSH. Expected result: below the diagnostic cut-offs in the ranges (a ratio at or above the cut-off is reported as increased).

| | |
|---|---|
| Category | [sex-hormone](index.md#sex-hormone) |
| Canonical unit | ratio |
| LOINC | [34434-1](https://loinc.org/34434-1/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | pcos | - | 0.749 | cohort | Sudanese women with PCOS compared with normal ovulatory women; cut-off from ROC analysis | [Diagnostic Performance of Anti-Müllerian Hormone, Luteinizing Hormone to Follicle-Stimulating Hormone Ratio, Testosterone, and Prolactin to Predict Polycystic Ovary Syndrome Among Sudanese Women.](https://pubmed.ncbi.nlm.nih.gov/37275514/), Results; Table 3 | ROC-derived cut-off in a single-country study (AUC 0.932 reported). Not a validated population-wide threshold. Source kind: diagnostic. Source band >= 0.749; stored as the warn high bound 0.749. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | pcos | - | 1 | cohort | Women with polycystic ovary on transvaginal Doppler ultrasound (single-centre study, China) | [[Preliminary study on the value of ratio of serum luteinizing hormone/follicle-stimulating hormone in diagnosis of polycystic ovarian syndrome among women with polycystic ovary].](https://pubmed.ncbi.nlm.nih.gov/21575449/), Abstract, Conclusions | Diagnostic cut-off for PCOS in a single-centre study. Not a validated population-wide threshold. Source kind: diagnostic. Source band >= 1; stored as the warn high bound 1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
