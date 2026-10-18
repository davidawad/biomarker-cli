# Heart Rate Variability (`hrv`)

Beat-to-beat variation in heart rate reflecting cardiac autonomic (vagal) activity. Lower short-term and 24-hour SDNN values are associated with higher mortality after myocardial infarction.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | ms |
| LOINC | [80404-7](https://loinc.org/80404-7/) |
| Specimen | - |
| Aliases | `heart rate variability` |

## Ranges

No clinical reference range: legacy catalog entry without a reference range; needs research

Bounds in ms. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 100 | cohort | Survivors of acute myocardial infarction (808 patients), 24-hour Holter recordings made 11 +/- 3 days after AMI (method: sdnn-24h; setting: post-acute-mi) | [Decreased heart rate variability and its association with increased mortality after acute myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/3812275/), Abstract, Results | 24-hour SDNN above 100 ms was the low-risk comparator group in this cohort (the reference against which the 5.3-fold mortality risk was computed). It is an observed comparator, not a validated health target. Source kind: decision. Source band >= 100; stored as the warn high bound 100. Alternative to the default kleiger-1987-sdnn-post-mi warn range for the same population. |
| alt-1 | reference | all | 32 | 93 | cohort | Healthy adults in 27 short-term (about 5 min) HRV studies pooled by Nunan et al. (reviewed in Shaffer 2017, Table 6); mostly adult samples, not age- or sex-specific (method: sdnn-short-term) | [An Overview of Heart Rate Variability Metrics and Norms.](https://pubmed.ncbi.nlm.nih.gov/29034226/), Table 6 (Nunan et al. short-term norms) | Indicative spread only: 32-93 ms is the range of study-level mean SDNN across the pooled studies (pooled mean 50 ms, SD 16), not an individual reference interval or a diagnostic cut-off. Alternative to the default nunan-2010-short-term-norms reference range for the same population. |
| kleiger-1987-sdnn-post-mi | warn | all | 50 | - | cohort | Survivors of acute myocardial infarction (808 patients), 24-hour Holter recordings made 11 +/- 3 days after AMI (method: sdnn-24h; setting: post-acute-mi) | [Decreased heart rate variability and its association with increased mortality after acute myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/3812275/), Abstract, Results | 24-hour SDNN below 50 ms is a high-mortality-risk band (risk cut-off from the cohort, not a guideline). Boundary as published: 'less than 50 ms'. Source kind: decision. Source band <= 50; stored as the warn low bound 50. |
| nunan-2010-short-term-norms | reference | all | 19 | 75 | cohort | Healthy adults in 15 short-term (about 5 min) HRV studies pooled by Nunan et al. (reviewed in Shaffer 2017, Table 6); mostly adult samples, not age- or sex-specific (method: rmssd-short-term) | [An Overview of Heart Rate Variability Metrics and Norms.](https://pubmed.ncbi.nlm.nih.gov/29034226/), Table 6 (Nunan et al. short-term norms) | Indicative spread only: 19-75 ms is the range of study-level mean RMSSD across the pooled studies (pooled mean 42 ms, SD 15), not an individual reference interval or a diagnostic cut-off. Ultra-short recordings and free vs paced breathing change values. |
| shaffer-2017-sdnn-24h | warn | all | - | 100 | cohort | Adults with 24-hour ambulatory ECG monitoring, as classified in the review by Shaffer 2017 (the review attributes these thresholds to the post-MI cohort work it cites) (method: sdnn-24h) | [An Overview of Heart Rate Variability Metrics and Norms.](https://pubmed.ncbi.nlm.nih.gov/29034226/), Section 'Twenty-Four Hour Measurement Norms' / SDNN description | Compromised-health band: 24-hour SDNN from 50 to 100 ms (boundaries as stated in the review, below 50 ms unhealthy and above 100 ms healthy). The sub-50 ms boundary is also covered by the cohort cut-off above. Source band 50-100; stored as the warn high bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
