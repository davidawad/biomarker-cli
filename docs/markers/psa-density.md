# PSA Density (`psa-density`)

Serum PSA divided by prostate volume (cm3) measured by imaging. It helps judge whether a raised PSA is explained by a large prostate or suggests clinically significant cancer.

| | |
|---|---|
| Category | [tumour](index.md#tumour) |
| Canonical unit | ng/mL/cm3 |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL/cm3. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| psa-density-2026 | warn | male | - | 0.15 | cohort | Men with suspected prostate cancer who had MRI-derived prostate volume (single-centre retrospective cohort) | [PSA density for clinically significant prostate cancer detection: a retrospective cohort study on the influence of age and prostate volume.](https://pubmed.ncbi.nlm.nih.gov/42599322/), Abstract, Purpose | PSA density (PSA divided by prostate volume) threshold of 0.15 ng/mL/cm3 used in current practice for clinically significant cancer. The study evaluated its performance across volumes and ages; boundary not stated in the quoted sentence. Not a PSA mass-concentration threshold; the unit is ng/mL per cm3. Source kind: decision. Source band >= 0.15; stored as the warn high bound 0.15. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
