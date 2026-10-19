# Urine Sodium (`urine-sodium`)

Urine Sodium. Tracked marker with no verified reference range yet, so values can be stored and charted but are not flagged against a range.

| | |
|---|---|
| Category | [electrolyte](index.md#electrolyte) |
| Canonical unit | mmol/L |
| LOINC | [2955-3](https://loinc.org/2955-3/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | hyponatremia | - | 30 | consensus | Adults with hypotonic hyponatremia (specimen: spot-urine) | [Hyponatremia: Evolving diagnostics and emerging therapeutics in clinical practice.](https://pubmed.ncbi.nlm.nih.gov/41907365/), Urine osmolality, sodium section | Spot urine sodium > 30 supports non-hypovolemic hyponatremia such as SIADH; diuretics and renal dysfunction confound. Source kind: diagnostic. Source band >= 30; stored as the warn high bound 30. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | hyponatremia | 30 | - | consensus | Adults with hypotonic hyponatremia (specimen: spot-urine) | [Hyponatremia: Evolving diagnostics and emerging therapeutics in clinical practice.](https://pubmed.ncbi.nlm.nih.gov/41907365/), Urine osmolality, sodium section | Spot urine sodium < 30 supports hypovolemic hyponatremia (extrarenal loss); diuretics and renal dysfunction confound. Source kind: diagnostic. Source band <= 30; stored as the warn low bound 30. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
