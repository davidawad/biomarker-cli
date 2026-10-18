# Systemic Immune-Inflammation Index (`sii`)

Systemic immune-inflammation index (SII) is platelet count multiplied by neutrophil count divided by lymphocyte count, a computed index from a complete blood count. It is studied as a prognostic marker of inflammation in cancer, cardiovascular disease and mortality. No established clinical cut-off exists. Expected result in healthy people: within the reference interval, which varies by study, sex and analyser.

| | |
|---|---|
| Category | [hematology](index.md#hematology) |
| Canonical unit | 10^9/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in 10^9/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 162 | 811 | reference-interval-study | Healthy adults, Jiangsu region, Eastern China (males, all ages), Sysmex platform | [Establishment of Reference Intervals for SII, NLR, PLR, and LMR in Healthy Adults in Jiangsu Region in Eastern China.](https://pubmed.ncbi.nlm.nih.gov/37145057/), Results | Non-parametric 2.5th-97.5th percentile interval, males; the study reports no significant age differences. Analyser-specific. |
| clinical | reference | female | 165 | 792 | reference-interval-study | Healthy adults, Jiangsu region, Eastern China (females, all ages), Sysmex platform | [Establishment of Reference Intervals for SII, NLR, PLR, and LMR in Healthy Adults in Jiangsu Region in Eastern China.](https://pubmed.ncbi.nlm.nih.gov/37145057/), Results | Non-parametric 2.5th-97.5th percentile interval, females; the study reports no significant age differences. Analyser-specific. |
| clinical | reference | all | 142 | 804 | reference-interval-study | Healthy Han Chinese adults aged 18-79, western China, sexes pooled (CLSI method, multi-centre) | [Normal Reference Intervals of Neutrophil-To-Lymphocyte Ratio, Platelet-To-Lymphocyte Ratio, Lymphocyte-To-Monocyte Ratio, and Systemic Immune Inflammation Index in Healthy Adults: a Large Multi-Center Study from Western China.](https://pubmed.ncbi.nlm.nih.gov/30868857/), Abstract (Results) | Pooled-sex interval as reported by the study. The same study reports sex-related differences in SII, so the sex-specific range above may be preferred. |
| clinical | warn | all | - | 655.56 | cohort | US adults free of pregnancy, cardiovascular disease and cancer, NHANES 1999-2018 (n = 42,875) | [Systemic Immune Inflammation Index (SII), System Inflammation Response Index (SIRI) and Risk of All-Cause Mortality and Cardiovascular Mortality: A 20-Year Follow-Up Cohort Study of 42,875 US Adults.](https://pubmed.ncbi.nlm.nih.gov/36769776/), Abstract (Results) | Single observational NHANES cohort: SII above 655.56 compared with below 335.36 was associated with higher all-cause mortality (HR 1.29). Not an established clinical cut-off. Source band >= 655.56; stored as the warn high bound 655.56. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
