# Glycated Albumin (`glycated-albumin`)

Glycated albumin reflects average glycaemia over about 2-3 weeks. Reference intervals and diabetes cut-offs come from population studies and are not standardised across assays or ethnic groups.

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | female, not pregnant | 10 | 13.3 | reference-interval-study | Non-pregnant Euro-Brazilian women (adult cohort, mean age 43.5 years, n=290) | [Reference Interval for Glycated Albumin, 1,5-AG/GA, and GA/HbA1c Ratios and Cut-Off Values for Type 1, Type 2, and Gestational Diabetes: A Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/39767558/), Abstract, Results | Reference interval, non-pregnant (ref. 39767558 reports it for non-pregnant women). |
| clinical | reference | female, pregnant | 10.6 | 14.7 | reference-interval-study | Pregnant Euro-Brazilian women (26.5 +/- 3.1 gestational weeks, n=406) | [Reference Interval for Glycated Albumin, 1,5-AG/GA, and GA/HbA1c Ratios and Cut-Off Values for Type 1, Type 2, and Gestational Diabetes: A Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/39767558/), Abstract, Results | Reference interval in pregnancy. |
| clinical | warn | all | - | 14.6 | reference-interval-study | Community residents of Yi-lan County, Taiwan (mean age 60.1 years, n=2192) | [A comparison of glycated albumin and glycosylated hemoglobin for the screening of diabetes mellitus in Taiwan.](https://pubmed.ncbi.nlm.nih.gov/26247684/), Abstract, Results | Prediabetes cut-point at the 75th percentile of the normal population (Taiwan). Source band >= 14.6; stored as the warn high bound 14.6. |
| clinical | reference | all | - | 16.1 | reference-interval-study | Non-diabetic controls, community residents of Yi-lan County, Taiwan (mean age 60.1 years) | [A comparison of glycated albumin and glycosylated hemoglobin for the screening of diabetes mellitus in Taiwan.](https://pubmed.ncbi.nlm.nih.gov/26247684/), Abstract, Results | 95th percentile in non-diabetic controls (Taiwan); upper limit only, not a central 95% interval. |
| alt-1 | warn | all | - | 16.5 | reference-interval-study | Japanese blood donors (as reported in ref. 39767558) | [Reference Interval for Glycated Albumin, 1,5-AG/GA, and GA/HbA1c Ratios and Cut-Off Values for Type 1, Type 2, and Gestational Diabetes: A Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/39767558/), Discussion (text) | Cited second-hand (Al-Lahham et al. citing Araki et al.): higher diabetes risk in Japanese blood donors with GA at or above this value. Not verified against the primary Japanese source. Source band >= 16.5; stored as the warn high bound 16.5. Alternative to the default clinical warn range for the same population. |
| alt-1 | warn | all | - | 16.5 | reference-interval-study | Community residents of Yi-lan County, Taiwan (mean age 60.1 years, n=2192) | [A comparison of glycated albumin and glycosylated hemoglobin for the screening of diabetes mellitus in Taiwan.](https://pubmed.ncbi.nlm.nih.gov/26247684/), Abstract, Results | GA 16.5% corresponds to HbA1c 6.5%, diagnostic of diabetes (Taiwanese screening study). Source kind: diagnostic. Source band >= 16.5; stored as the warn high bound 16.5. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | warn | all | - | 14.8 | reference-interval-study | Euro-Brazilian adults with type 1 diabetes versus controls (n=81 cases, 290 controls) | [Reference Interval for Glycated Albumin, 1,5-AG/GA, and GA/HbA1c Ratios and Cut-Off Values for Type 1, Type 2, and Gestational Diabetes: A Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/39767558/), Table 7 | ROC cut-off for type 1 diabetes in adults versus controls (Euro-Brazilian; Table 7). Not a standardised guideline threshold. Source kind: diagnostic. Source band >= 14.8; stored as the warn high bound 14.8. Alternative to the default diagnostic warn range for the same population. |
| alt-3 | warn | all | - | 15.2 | reference-interval-study | Japanese community residents (908 nondiabetic subjects; 176 with an OGTT) | [Glycated albumin as a diagnostic tool for diabetes in a general Japanese population.](https://pubmed.ncbi.nlm.nih.gov/25817605/), Abstract, Results | ROC-optimal cut-off for diabetes (OGTT-defined) in this population; low sensitivity and specificity, not a guideline threshold. Source kind: diagnostic. Source band >= 15.2; stored as the warn high bound 15.2. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 14.2 | reference-interval-study | Euro-Brazilian adults with type 2 diabetes versus controls (n=283 cases, 290 controls) | [Reference Interval for Glycated Albumin, 1,5-AG/GA, and GA/HbA1c Ratios and Cut-Off Values for Type 1, Type 2, and Gestational Diabetes: A Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/39767558/), Table 7 | ROC cut-off for type 2 diabetes versus controls (Euro-Brazilian; Table 7 gives sensitivity and specificity). Note: not a standardised guideline threshold. Source kind: diagnostic. Source band >= 14.2; stored as the warn high bound 14.2. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
