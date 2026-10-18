# C-Reactive Protein (standard assay) (`crp-standard`)

Conventional (standard-sensitivity) C-reactive protein, an acute-phase protein made by the liver. Higher values indicate acute inflammation, most often from infection or tissue injury. Distinct from the high-sensitivity assay used for cardiovascular risk.

| | |
|---|---|
| Category | [inflammation](index.md#inflammation) |
| Canonical unit | mg/L |
| LOINC | [1988-5](https://loinc.org/1988-5/) |
| Specimen | - |
| Aliases | `standard crp` |

## Ranges

Bounds in mg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 5 | reference-interval-study | Women in sports (HemSter study); the 5.0 mg/L value is the proposed healthy-population cut-off cited by the authors, alongside their own calculated interval (<2.9 mg/L) | [Women in sports: the applicability of common national reference intervals for inflammatory and coagulation biomarkers (HemSter Study).](https://pubmed.ncbi.nlm.nih.gov/33380889/), Abstract, Results | Conventional CRP upper reference limit of 5.0 mg/L (boundary < 5.0 mg/L) for a healthy population, as proposed in the source. The study's own calculated interval was lower (<2.9 mg/L) in a female sports cohort, so the value is not a sex-neutral or universal limit. |
| emej-2025-sepsis-crp | warn | all | - | 48 | cohort | Patients in diagnostic studies of sepsis (44 studies, 10 755 patients), Sepsis-2 and Sepsis-3 definitions (setting: suspected-sepsis) | [Diagnostic performances of procalcitonin and C-reactive protein for sepsis: a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/40214293/), Abstract, Results | Optimal (sensitivity-weighted) CRP cut-off for sepsis, 48 mg/L with 72% sensitivity and 55% specificity. Pooled AUC for CRP was 0.67, so discrimination is limited; this is not a rule-in threshold on its own. Source kind: decision. Source band >= 48; stored as the warn high bound 48. |
| ijid-bmc-2021-crp-viral-bacterial | warn | age 5+ | 10 | - | cohort | Patients aged 5 years and over with non-malaria acute undifferentiated fever in tropical areas (individual patient data) | [Value of C-reactive protein in differentiating viral from bacterial aetiologies in patients with non-malaria acute undifferentiated fever in tropical areas: a meta-analysis and individual patient data study.](https://pubmed.ncbi.nlm.nih.gov/33644814/), Abstract, Results | CRP below 10 mg/L favours a viral aetiology (52% sensitivity, 95% specificity for viral infection in this study). Boundary is strictly less than 10 mg/L. Not a stand-alone diagnosis. Source kind: decision. Source band <= 10; stored as the warn low bound 10. |
| ijid-bmc-2021-crp-viral-bacterial | warn | all | - | 40 | cohort | Patients aged 5 years and over with non-malaria acute undifferentiated fever in tropical areas (individual patient data) | [Value of C-reactive protein in differentiating viral from bacterial aetiologies in patients with non-malaria acute undifferentiated fever in tropical areas: a meta-analysis and individual patient data study.](https://pubmed.ncbi.nlm.nih.gov/33644814/), Abstract, Results | CRP above 40 mg/L favours a bacterial aetiology (74% sensitivity, 84% specificity for bacterial infection in this study). Boundary is strictly greater than 40 mg/L. Source kind: decision. Source band >= 40; stored as the warn high bound 40. |
| pediatr-2026-crp-fever-without-source | warn | all | - | 20 | cohort | Children aged 36 months or younger with fever without source (systematic review and meta-analysis) | [Identifying the thresholds of C-reactive protein, procalcitonin, and interleukin-6 among children ≤36 months' old with fever without source at risk of serious bacterial infections: a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/41837198/), Abstract, Conclusion | Best-performing CRP cut-off band for serious bacterial infection in this group (10 to 20 mg/L). The source reports the band as the optimum, not a single threshold. Source kind: decision. Source band 10-20; stored as the warn high bound 20. |
| pregnancy-crp-ri-2021 | reference | pregnant | - | 19 | reference-interval-study | Healthy pregnant women (n=322) in a longitudinal study; pregnancy-specific upper reference limit, similar across trimesters | [Pregnancy-specific reference intervals for C-reactive protein improve diagnostic accuracy for infection: A longitudinal study.](https://pubmed.ncbi.nlm.nih.gov/33647266/), Abstract, Results | Pregnancy-specific upper reference limit for CRP of 19 mg/L (boundary above 19 mg/L). CRP rises in pregnancy and also increases with body mass index, so the general-population limits do not apply. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
