# Creatine Kinase (multiple of upper limit of normal) (`ck-uln-multiple`)

Result expressed as a multiple of the reporting laboratory's upper limit of normal (ULN) rather than in absolute units. Compute it as the measured result divided by the ULN printed on the same laboratory report.

| | |
|---|---|
| Category | [chemistry](index.md#chemistry) |
| Canonical unit | x ULN |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in x ULN. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 4 | 10 | guideline | Adults with asymptomatic or oligosymptomatic hyperCKemia, as grouped in large-scale statin risk studies cited by the EAN 2024 guideline | [EAN 2024 Guideline on the Diagnostic Approach to Oligo/Asymptomatic HyperCKemia.](https://pubmed.ncbi.nlm.nih.gov/41619221/), Full text, statin risk discussion | Statin-risk grouping of hyperCKemia by multiples of the upper limit of normal: up to 3x ULN, between 4x and 10x ULN, and higher values. The 4x to 10x band is the middle group. Multiples are relative to each laboratory's ULN, not absolute U/L values. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| ean-2024-hyperckemia | warn | statin-therapy | 10 | - | guideline | Adults with asymptomatic or oligosymptomatic hyperCKemia who are indicated for statin therapy | [EAN 2024 Guideline on the Diagnostic Approach to Oligo/Asymptomatic HyperCKemia.](https://pubmed.ncbi.nlm.nih.gov/41619221/), Full text, statin risk discussion | Guideline recommendation: statins should not be avoided when indicated in people with CK up to 10x ULN, with close follow-up. Boundary inclusive at 10x, following the wording 'up to 10x'. Source kind: decision. Source band <= 10; stored as the warn low bound 10. |
| chest-2026-rhabdo-review | critical | all | - | 5 | consensus | Adults with suspected rhabdomyolysis (clinically oriented narrative review) | [Updates on Rhabdomyolysis: A Clinically Oriented Narrative Review.](https://pubmed.ncbi.nlm.nih.gov/41619932/), Definition and diagnosis | Commonly used rhabdomyolysis CK cut-off: above 5x the upper limit of normal, or above 1,000 U/L (the review states that no formal consensus definition exists). Boundary is strict (>5x ULN). Source band >= 5; stored as the critical high bound 5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
