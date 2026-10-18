# Triglyceride-Glucose Index (`tyg-index`)

Surrogate insulin-resistance index calculated as ln(fasting triglycerides [mg/dL] x fasting glucose [mg/dL] / 2). It was validated against the euglycaemic-hyperinsulinaemic clamp.

| | |
|---|---|
| Category | [derived](index.md#derived) |
| Canonical unit | index |
| LOINC | - |
| Specimen | - |
| Aliases | `tyg index`, `tyg`, `ln(tg x glucose / 2)` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| guerrero-romero-2010 | warn | fasting | - | 4.68 | cohort | Adults in a validation study against the euglycemic-hyperinsulinemic clamp | [The product of triglycerides and glucose, a simple measure of insulin sensitivity. Comparison with the euglycemic-hyperinsulinemic clamp.](https://pubmed.ncbi.nlm.nih.gov/20484475/), Abstract, Results | Best ROC value for insulin resistance (sensitivity 96.5%, specificity 85.0%). Formula in the source: Ln(fasting triglycerides mg/dl) x fasting glucose mg/dl / 2. Applies to fasting values only. Source kind: decision. Source band >= 4.68; stored as the warn high bound 4.68. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
