# Free β-hCG, Maternal Serum (`free-beta-hcg`)

Free beta subunit of hCG measured in first-trimester maternal serum and expressed as multiples of the median (MoM) for gestational age. Used with PAPP-A, nuchal translucency and maternal age in aneuploidy screening. Abnormal values raise the risk of trisomy 21 (high) or of other chromosomal abnormalities (low).

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | MoM |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in MoM. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| atypical-cma-2021 | warn | pregnant, trimester 1 | 0.37 | - | cohort | 877 singleton pregnancies with first-trimester combined screening data and chromosomal microarray (CMA), 2013-2020 (specimen: serum) | [Significance of Low Maternal Serum Β-hCG Levels in the Assessment of the Risk of Atypical Chromosomal Abnormalities.](https://pubmed.ncbi.nlm.nih.gov/34872079/), Abstract, Results | Low free β-hCG (<= 0.37 MoM) was associated with atypical chromosomal abnormalities on CMA; risk of pathogenic CMA rose from 6% to 20% in the high-risk combined-screening group. It is not specific to trisomy 18 or 13, and no trisomy 18/13 threshold was verifiable. Source band <= 0.37; stored as the warn low bound 0.37. |
| contingent-screening-2023 | warn | pregnant, trimester 1 | - | 2.75 | cohort | First-trimester screening population with a contingent NIPT strategy (BMC Pregnancy Childbirth 2023) (specimen: serum) | [A new contingent screening strategy increased detection rate of trisomy 21 in the first trimester.](https://pubmed.ncbi.nlm.nih.gov/37964244/), Abstract, Results | Cut-off for an abnormal first-trimester result that triggered further NIPT in the contingent strategy: free β-hCG >= 2.75 MoM or PAPP-A <= 0.5 MoM. This is a screening-pathway threshold, not a diagnosis, and MoM values are centre-specific. Source kind: diagnostic. Source band >= 2.75; stored as the warn high bound 2.75. |
| iran-amniocentesis-2026 | warn | pregnant, trimester 1 | - | 1.52 | cohort | First-trimester pregnancies in southern Iran (2021-2022) with abnormal amniocentesis results, cross-sectional study (specimen: serum) | [The Association between Serum Biochemical Markers and Early Amniocentesis in Diagnosing Chromosomal Anomalies: A Cross-Sectional Study in Southern Iran, 2021-2022.](https://pubmed.ncbi.nlm.nih.gov/42039225/), Abstract, Results | ROC-type cut-off for predicting Down syndrome among abnormal amniocentesis cases (sensitivity 76%, specificity 70%). Small single-region cross-sectional study; not a validated screening threshold, and it is higher than the 2.75 MoM contingent rule above. Source kind: diagnostic. Source band >= 1.52; stored as the warn high bound 1.52. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
