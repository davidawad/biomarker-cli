# NAFLD Fibrosis Score (`nafld-fibrosis-score`)

Composite score from age, BMI, impaired glucose status, platelet count, albumin and AST/ALT ratio that estimates advanced fibrosis in NAFLD. Scores below the low cut-off exclude advanced fibrosis and scores above the high cut-off suggest it.

| | |
|---|---|
| Category | [derived](index.md#derived) |
| Canonical unit | score |
| LOINC | - |
| Specimen | - |
| Aliases | `nfs`, `nafld-fs` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in score. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 0.676 | cohort | Adults with biopsy-proven NAFLD (derivation and validation cohorts) | [The NAFLD fibrosis score: a noninvasive system that identifies liver fibrosis in patients with NAFLD.](https://pubmed.ncbi.nlm.nih.gov/17393509/), Abstract, Results | High cut-off: scores above 0.676 indicate advanced fibrosis (PPV 90% and 82%). The upper cut-off is unchanged for age 65 and over. Source kind: decision. Source band >= 0.676; stored as the warn high bound 0.676. Alternative to the default angulo-2007 warn range for the same population. |
| angulo-2007 | warn | all | -1.455 | - | cohort | Adults with biopsy-proven NAFLD (derivation and validation cohorts) | [The NAFLD fibrosis score: a noninvasive system that identifies liver fibrosis in patients with NAFLD.](https://pubmed.ncbi.nlm.nih.gov/17393509/), Abstract, Results | Low cut-off: scores below -1.455 exclude advanced fibrosis with high accuracy (NPV 93% and 88%). Not validated for under 35 and for 65 and over, where age-adjusted cut-offs apply. Source kind: decision. Source band <= -1.455; stored as the warn low bound -1.455. |
| mcpherson-2017 | warn | age 65+ | 0.12 | - | cohort | Adults aged 65 and over with biopsy-proven NAFLD | [Age as a Confounding Factor for the Accurate Non-Invasive Diagnosis of Advanced NAFLD Fibrosis.](https://pubmed.ncbi.nlm.nih.gov/27725647/), Results, Discussion; [Age as a Confounding Factor for the Accurate Non-Invasive Diagnosis of Advanced NAFLD Fibrosis.](https://pubmed.ncbi.nlm.nih.gov/27725647/), Results, Discussion | Age-adjusted lower cut-off for 65 and over: NFS below 0.12 is used to exclude advanced fibrosis, per the paper's proposed thresholds (sensitivity 80%, specificity 70%). Upper cut-off unchanged. Source kind: decision. Source band <= 0.12; stored as the warn low bound 0.12. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
