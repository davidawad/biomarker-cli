# Faecal Immunochemical Test (Haemoglobin) (`fit-hemoglobin`)

Faecal immunochemical test (FIT) detects human haemoglobin in stool. Quantitative results in µg haemoglobin per gram of faeces are compared with programme or laboratory cut-offs to decide on referral or colonoscopy for possible colorectal neoplasia. The threshold is method- and programme-specific.

| | |
|---|---|
| Category | [stool](index.md#stool) |
| Canonical unit | µg Hb/g |
| LOINC | [29771-3](https://loinc.org/29771-3/) |
| Specimen | - |
| Aliases | `fit`, `faecal haemoglobin`, `f-hb`, `fhb`, `quantitative fit`, `qfit`, `fecal immunochemical test` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in µg Hb/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| dutch-crc-screening-fit-2026 | warn | all | - | 47 | cohort | Participants in the Dutch national colorectal cancer screening programme | [Evaluating the predictive value of multiple rounds of faecal haemoglobin measurement for the prediction of advanced neoplasia and colorectal cancer: an analysis of data from the Dutch national colorectal cancer screening programme.](https://pubmed.ncbi.nlm.nih.gov/42772943/), Methods, Screening | Programme cut-off: results at or above 47 µg Hb/g are positive and referred for colonoscopy. The cut-off is programme-specific. Source kind: decision. Source band >= 47; stored as the warn high bound 47. |
| florence-fit-screening-2026 | warn | all | - | 20 | cohort | Participants in the Florence colorectal cancer screening programme with a negative FIT | [Three-year risk of colorectal cancer by faecal haemoglobin concentration in subjects with a negative FIT.](https://pubmed.ncbi.nlm.nih.gov/42742245/), Abstract, Methods | Programme positivity cut-off. Results at or above 20 µg Hb/g are positive and referred for colonoscopy, per the study's description of the programme. Source kind: decision. Source band >= 20; stored as the warn high bound 20. |
| multicutoff-fit-2026 | warn | all | - | 17 | cohort | Adults in a colorectal cancer screening cohort at younger screening ages (method: manufacturer-recommended quantitative FIT) | [Colorectal Cancer Screening at Younger Ages: A Multicutoff Analysis of FIT Performance.](https://pubmed.ncbi.nlm.nih.gov/42509105/), Abstract, Results | Manufacturer-recommended cut-off for this quantitative FIT, evaluated for advanced neoplasia detection. Sensitivity was low at this cut-off (about 27 percent), so it is not a general-purpose threshold. Source kind: decision. Source band >= 17; stored as the warn high bound 17. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
