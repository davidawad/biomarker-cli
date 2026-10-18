# Urine Protein/Creatinine Ratio (`upcr`)

Urinary protein-to-creatinine ratio on a spot urine sample. Quantifies proteinuria in kidney disease and is used to assess significant proteinuria in pregnancy. Adult PCR staging categories are not stated in the sources reviewed here, so only the thresholds below are included.

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | mg/g |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 0-2 | - | 500 | guideline | Infants aged 6 months to 2 years (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Pediatric CKD text (proteinuria in infants and children) | Spot PCR usually considered normal in infants 6 months to 2 years, strictly below 500 mg/g. |
| clinical | reference | age 2+ | - | 200 | guideline | Children over 2 years of age (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Pediatric CKD text (proteinuria in infants and children) | Spot PCR usually considered normal in children over 2 years, strictly below 200 mg/g. |
| clinical | reference | pregnant | - | 265.2 | consensus | Pregnant women with normal pregnancy (values as tabulated in a review) (specimen: spot-urine) | [Maternal kidney adaptation and function in pregnancy: review and perspectives.](https://pubmed.ncbi.nlm.nih.gov/42731065/), Table 1 | Normal spot protein:creatinine ratio in pregnancy, strictly below 30 mg/mmol (source units kept). The nonpregnant value in the same table is below 20 mg/mmol. Converted from the source's mg/mmol (30) to mg/g using factor 8.84. |
| alt-1 | reference | age 2+ | - | 176.8 | guideline | Children over 2 years of age (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Pediatric CKD text (proteinuria in infants and children) | Spot PCR usually considered normal in children over 2 years, strictly below 20 mg/mmol (the same threshold as below 200 mg/g). Converted from the source's mg/mmol (20) to mg/g using factor 8.84. Alternative to the default clinical reference range for the same population. |
| alt-1 | warn | all | - | 442 | guideline | Adults with suspected or confirmed CKD (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Figure box 'Diagnosis of CKD' (Chapter 3 algorithm) | PCR of 50 mg/mmol or more (the same threshold as 500 mg/g), approximately equivalent to ACR of 30 mg/mmol or more. Converted from the source's mg/mmol (50) to mg/g using factor 8.84. Source kind: diagnostic. Source band >= 442.0; stored as the warn high bound 442.0. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 500 | guideline | Adults with suspected or confirmed CKD (specimen: spot-urine) | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Figure box 'Diagnosis of CKD' (Chapter 3 algorithm) | PCR of 500 mg/g or more, approximately equivalent to ACR of 300 mg/g or more (A3 albuminuria). Part of the KDIGO criteria for diagnosing CKD with albuminuria. Source kind: diagnostic. Source band >= 500; stored as the warn high bound 500. |
| diagnostic | warn | pregnant | - | 300 | cohort | Pregnant women with new-onset hypertension, evaluation for significant proteinuria / preeclampsia (specimen: spot-urine) | [The Prognostic Significance of Proteinuria Severity in Pregnancy: A Retrospective Cohort Study of Maternal and Neonatal Outcomes](https://pubmed.ncbi.nlm.nih.gov/41517598/), Introduction | Spot PCR cut-off of 0.3 mg/mg (equivalent to 300 mg/g or 30 mg/mmol) used to indicate significant proteinuria in pregnancy. The cut-off is quoted from the introduction of this cohort study, which cites its own sources; the ACOG and ISSHP texts were not retrieved, so they are not cited here. Converted from the source's mg/mg (0.3) to mg/g using factor 1000. Source kind: diagnostic. Source band >= 300.0; stored as the warn high bound 300.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
