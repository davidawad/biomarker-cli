# ApoB/ApoA-I Ratio (`apob-apoa1-ratio`)

Ratio of apolipoprotein B to apolipoprotein A-I, a balance index of atherogenic versus antiatherogenic lipoprotein particles. Used as a cardiovascular risk marker.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | ratio |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 0.3 | 1 | reference-interval-study | FINRISK 2007 reference sample, Finnish adults (n = 2828) | [Apolipoprotein A-I, apolipoprotein B, and apolipoprotein B/apolipoprotein A-I ratio: reference intervals compared with values in different pathophysiological conditions from the FINRISK 2007 study.](https://pubmed.ncbi.nlm.nih.gov/21419755/), Abstract (Results) | Sex-specific reference interval for the apoB/apoA-I ratio (both apolipoproteins in g/L). |
| clinical | reference | female | 0.3 | 0.8 | reference-interval-study | FINRISK 2007 reference sample, Finnish adults (n = 2828) | [Apolipoprotein A-I, apolipoprotein B, and apolipoprotein B/apolipoprotein A-I ratio: reference intervals compared with values in different pathophysiological conditions from the FINRISK 2007 study.](https://pubmed.ncbi.nlm.nih.gov/21419755/), Abstract (Results) | Sex-specific reference interval for the apoB/apoA-I ratio (both apolipoproteins in g/L). |
| clinical | warn | all | - | 0.61 | cohort | Adults, men and women combined (AMORIS cohort) | [Long-term risk of a major cardiovascular event by apoB, apoA-1, and the apoB/apoA-1 ratio-Experience from the Swedish AMORIS cohort: A cohort study.](https://pubmed.ncbi.nlm.nih.gov/34851955/), Results, 'The apoB/apoA-1 ratio and its relations to levels of apoB and apoA-1' section | Tentative medium-risk band of the apoB/apoA-1 ratio (AMORIS, Swedish cohort). Not sex-specific; the source calls the levels tentative. Source band 0.61-0.9; stored as the warn high bound 0.61. |
| alt-1 | warn | all | - | 0.91 | cohort | Adults, men and women combined (AMORIS cohort) | [Long-term risk of a major cardiovascular event by apoB, apoA-1, and the apoB/apoA-1 ratio-Experience from the Swedish AMORIS cohort: A cohort study.](https://pubmed.ncbi.nlm.nih.gov/34851955/), Results, 'The apoB/apoA-1 ratio and its relations to levels of apoB and apoA-1' section | Tentative high-risk band of the apoB/apoA-1 ratio (AMORIS, Swedish cohort). Not sex-specific. Source band >= 0.91; stored as the warn high bound 0.91. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
