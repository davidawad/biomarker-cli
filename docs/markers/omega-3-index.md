# Omega-3 Index (`omega-3-index`)

Red blood cell EPA plus DHA expressed as a percent of total fatty acids, a long-term marker of omega-3 status. Low values (below 4%) are associated with higher risk of coronary heart disease death; values above 8% are associated with the lowest risk.

| | |
|---|---|
| Category | [fatty-acid](index.md#fatty-acid) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 4 | 8 | consensus | Adults; risk zones proposed for erythrocyte EPA plus DHA from the omega-3 index risk-factor review (specimen: rbc) | [The omega-3 index as a risk factor for coronary heart disease.](https://pubmed.ncbi.nlm.nih.gov/18541601/), Abstract | Proposed intermediate-risk band for the Omega-3 Index (percent of erythrocyte fatty acids). Proposed zones, not a population reference interval. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| harris-2004 | optimal | all | 8 | - | cohort | Adults in the Harris and von Schacky analysis of published primary and secondary prevention studies (CHD death and sudden cardiac death outcomes) (specimen: rbc) | [The Omega-3 Index: a new risk factor for death from coronary heart disease?](https://pubmed.ncbi.nlm.nih.gov/15208005/), Abstract, Results; [Global survey of the omega-3 fatty acids, docosahexaenoic acid and eicosapentaenoic acid in the blood stream of healthy adults.](https://pubmed.ncbi.nlm.nih.gov/27216485/), Abstract | Stated as the level associated with greatest cardioprotection. Stark 2016 uses the same >8% boundary as its high-level cutoff. Quote is from the abstract; the abstract gives only the >=8% and <=4% points, not the intermediate band. |
| harris-2008 | warn | all | 4 | - | consensus | Adults; high-risk zone for erythrocyte EPA plus DHA from the omega-3 index risk-factor review (specimen: rbc) | [The omega-3 index as a risk factor for coronary heart disease.](https://pubmed.ncbi.nlm.nih.gov/18541601/), Abstract | Proposed high-risk zone for the Omega-3 Index. Proposed cutoff, not a population reference interval. Source kind: deficient. Source band <= 4; stored as the warn low bound 4. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
