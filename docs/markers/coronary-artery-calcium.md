# Coronary Artery Calcium Score (Agatston) (`coronary-artery-calcium`)

Coronary artery calcium score measured by non-contrast cardiac CT and quantified with the Agatston method. It reflects the burden of coronary atherosclerotic calcium and refines ASCVD risk, especially when the 10-year risk estimate is uncertain.

| | |
|---|---|
| Category | [composite](index.md#composite) |
| Canonical unit | Agatston units |
| LOINC | - |
| Specimen | - |
| Aliases | `cac score`, `agatston score`, `coronary calcium score` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in Agatston units. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| acc-aha-2018-cholesterol | warn | all | - | 100 | guideline | Adults with a coronary artery calcium score measured for a statin decision | [2018 AHA/ACC/AACVPR/AAPA/ABC/ACPM/ADA/AGS/APhA/ASPC/NLA/PCNA Guideline on the Management of Blood Cholesterol: A Report of the American College of Cardiology/American Heart Association Task Force on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/30586774/), Full text, Recommendations on coronary artery calcium | CAC of 100 Agatston units or higher: statin therapy is indicated unless deferred after clinician-patient risk discussion. The same recommendation also accepts a score at or above the 75th percentile as an equivalent cut-off; that percentile is not a numeric Agatston range and is not encoded here. Lower bound only (open above). Source kind: decision. Source band >= 100; stored as the warn high bound 100. |
| scct-cac-drs-2018 | warn | all | - | 300 | consensus | Adults undergoing coronary artery calcium scoring (CAC-DRS reporting categories) | [Beyond the Agatston calcium score: role of calcium density and other calcified plaque markers for cardiovascular disease prediction.](https://pubmed.ncbi.nlm.nih.gov/39445716/), Full text, CAC-DRS description | CAC-DRS category A3, severely increased (>=300 Agatston units). Same caveat as the 100-299 range: the category is taken from the review cited here. Lower bound only (open above). Source kind: decision. Source band >= 300; stored as the warn high bound 300. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
