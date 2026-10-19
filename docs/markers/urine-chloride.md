# Urine Chloride (`urine-chloride`)

Urine Chloride. Tracked marker with no verified reference range yet, so values can be stored and charted but are not flagged against a range.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | mmol/L |
| LOINC | [2078-4](https://loinc.org/2078-4/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | metabolic-alkalosis | 20 | - | consensus | Adults with metabolic alkalosis (specimen: spot-urine) | [Mastering blood gas interpretation: A practical guide for primary care providers.](https://pubmed.ncbi.nlm.nih.gov/40336441/), Step 3, metabolic alkalosis | Chloride-responsive metabolic alkalosis (volume depletion, vomiting, diuretics); respond to NaCl. Source band <= 20; stored as the warn low bound 20. |
| diagnostic | warn | metabolic-alkalosis | - | 20 | consensus | Adults with metabolic alkalosis (specimen: spot-urine) | [Mastering blood gas interpretation: A practical guide for primary care providers.](https://pubmed.ncbi.nlm.nih.gov/40336441/), Step 3, metabolic alkalosis | Chloride-resistant metabolic alkalosis (e.g. mineralocorticoid excess); does not respond to NaCl. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
