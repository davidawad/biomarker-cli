# Fractional Excretion of Sodium (FENa) (`fena`)

Fractional excretion of sodium: the percentage of filtered sodium excreted in urine, calculated from urine and plasma sodium and creatinine, used to differentiate prerenal azotemia from acute tubular necrosis in oliguric acute kidney injury. Expected result: low in prerenal azotemia, higher in acute tubular necrosis.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | % |
| LOINC | [49135-7](https://loinc.org/49135-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | acute-tubular-necrosis | - | 3 | cohort | Patients in the oliguric phase of acute renal failure (cohort in the cited study) | [The FENa test. Use in the differential diagnosis of acute renal failure.](https://pubmed.ncbi.nlm.nih.gov/947239/), Abstract | Acute tubular necrosis as classified by the source: FENa of more than 3 (strictly above 3). Source kind: diagnostic. Source band >= 3; stored as the warn high bound 3. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | prerenal-azotemia | 1 | - | cohort | Patients in the oliguric phase of acute renal failure (cohort in the cited study) | [The FENa test. Use in the differential diagnosis of acute renal failure.](https://pubmed.ncbi.nlm.nih.gov/947239/), Abstract | Prerenal azotemia as classified by the source: FENa of less than 1 (strictly below 1). Source kind: diagnostic. Source band <= 1; stored as the warn low bound 1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
