# Urine Protein-to-Creatinine Ratio (`urine-pcr`)

Total protein excreted in urine relative to creatinine, measured on a spot sample. It estimates 24-hour proteinuria and is used to diagnose and monitor pre-eclampsia and other proteinuric kidney disease.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | mg/g |
| LOINC | - |
| Specimen | - |
| Aliases | `spot urine protein-creatinine ratio`, `urine p/c ratio` |

## Ranges

Bounds in mg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 141.7 | reference-interval-study | Healthy adults in Dalian, China (random morning spot urine), all adults combined | [Reference values of urine protein/creatinine ratio in healthy Dalian adults.](https://pubmed.ncbi.nlm.nih.gov/34617627/), Abstract, Results | Upper reference limit, taken as the 95th percentile of UPCR in healthy adults. The overall value is 141.7 mg/g; sex-specific values are given in the next two ranges. Single-centre Chinese cohort, so not a universal limit. |
| alt-1 | warn | all | - | 200 | guideline | Children and adolescents: threshold for referral to specialist kidney care | [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Practice Point 5.1.2 (pediatric referral criteria) | KDIGO 2024 Practice Point 5.1.2 (pediatric considerations): refer when an ACR of 30 mg/g or a PCR of 200 mg/g or more is confirmed on a repeat first-morning sample. The 200 mg/g (20 mg/mmol) PCR is the PCR criterion. Boundary >=200. Source kind: decision. Source band >= 200; stored as the warn high bound 200. Alternative to the default kdigo-2024 warn range for the same population. |
| kdigo-2024 | warn | all | - | 500 | guideline | Adults with chronic kidney disease: heavy proteinuria threshold for CKD diagnosis and specialist referral | [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Figure 48 (referral circumstances), CKD diagnosis box | KDIGO 2024 states a PCR of 500 mg/g (50 mg/mmol) is approximately equivalent to an ACR of 300 mg/g (A3, heavily increased albuminuria). Boundary >=500 as printed. Source kind: decision. Source band >= 500; stored as the warn high bound 500. |
| pregnancy-proteinuria-0-3 | warn | pregnant | - | 300 | consensus | Pregnant women | [Proteinuria during pregnancy: definition, pathophysiology, methodology, and clinical significance.](https://pubmed.ncbi.nlm.nih.gov/32882208/), Abstract | Classic cut-off for significant proteinuria in pregnancy: a urine protein-to-creatinine ratio of at least 0.3 (mg/mg, i.e. 300 mg/g). The source is a narrative review citing this cut-off, not a guideline. Converted from the source's mg/mg (0.3) to mg/g using factor 1000. Source kind: decision. Source band >= 300.0; stored as the warn high bound 300.0. |
| upcr-0-3-preeclampsia-validation | warn | pregnant | - | 300 | cohort | Women with suspected pre-eclampsia (setting: suspected-pre-eclampsia) | [Diagnostic accuracy of random urinary protein-to-creatinine ratio for proteinuria in patients with suspected pre-eclampsia.](https://pubmed.ncbi.nlm.nih.gov/33386413/), Abstract, Results | UPCR of 0.3 (mg/mg) as the threshold for predicting proteinuria >=300 mg/day in suspected pre-eclampsia, with sensitivity 90.1% and specificity 63.3%. The cited text also reports a UPCR >0.30 predictive accuracy. Converted from the source's mg/mg (0.3) to mg/g using factor 1000. Source kind: decision. Source band >= 300.0; stored as the warn high bound 300.0. |
| upcr-reference-dalian-2021 | reference | male | - | 128.7 | reference-interval-study | Healthy adult men in Dalian, China (random morning spot urine) | [Reference values of urine protein/creatinine ratio in healthy Dalian adults.](https://pubmed.ncbi.nlm.nih.gov/34617627/), Abstract, Results | Upper reference limit (95th percentile) for men: 128.7 mg/g. |
| upcr-reference-dalian-2021 | reference | female | - | 150.8 | reference-interval-study | Healthy adult women in Dalian, China (random morning spot urine) | [Reference values of urine protein/creatinine ratio in healthy Dalian adults.](https://pubmed.ncbi.nlm.nih.gov/34617627/), Abstract, Results | Upper reference limit (95th percentile) for women: 150.8 mg/g. Women had higher values than men in the same age group in this cohort. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
