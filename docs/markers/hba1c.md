# Hemoglobin A1c (`hba1c`)

Glycated haemoglobin, reflecting average glucose over about 2-3 months. Used to diagnose and monitor prediabetes and diabetes, and to set individual treatment goals. Diagnostic and target values are given in % (NGSP) and mmol/mol (IFCC).

| | |
|---|---|
| Category | [metabolic](index.md#metabolic) |
| Canonical unit | % |
| LOINC | [4548-4](https://loinc.org/4548-4/) |
| Specimen | - |
| Aliases | `a1c`, `hemoglobin a1c`, `glycated hemoglobin`, `hb a1c` |

## Unit conversions

Into %: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| mmol/mol | 0.09148 | 2.152 |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 4 | 5.6 | - | - | uncited (legacy) | - |
| clinical | warn | all | - | 6.4 | guideline | Adults without a previous diabetes diagnosis | [The impact of glucose and metabolic disturbances on white matter hyperintensity volume in apparently healthy adults.](https://pubmed.ncbi.nlm.nih.gov/41928882/), Table 1 | Prediabetes by ADA criteria. Source band 5.7-6.4; stored as the warn high bound 6.4. |
| optimal | optimal | all | - | 5.3 | - | - | uncited (legacy) | - |
| blueprint | optimal | all | 4.5 | 5.6 | - | - | uncited (legacy) | - |
| alt-1 | warn | all | - | 6.36 | guideline | Adults without a previous diabetes diagnosis | [Should Prediabetes Be Classified as a Treatable Disease?](https://pubmed.ncbi.nlm.nih.gov/41598648/), Section 2 (Diagnostic criteria) | Prediabetes by ADA criteria, expressed in IFCC units (mmol/mol). Converted from the source's mmol/mol (39-46) to % using factor 0.09148 offset 2.152. Source band 5.72-6.36; stored as the warn high bound 6.36. Alternative to the default clinical warn range for the same population. |
| alt-1 | reference | all | - | 5.7 | guideline | Adults (ADA diagnostic categories as tabulated in a 2026 review) | [Diagnostic utility of fasting versus non-fasting blood glucose: contextualising testing strategies-a narrative review.](https://pubmed.ncbi.nlm.nih.gov/42321865/), Table 3 | Normal HbA1c category, strictly below 5.7%. Alternative to the default clinical reference range for the same population. |
| alt-1 | optimal | diabetes, planning-pregnancy | - | 6.543 | guideline | Women with diabetes planning a pregnancy | [https://www.nice.org.uk/guidance/ng3/chapter/recommendations](https://www.nice.org.uk/guidance/ng3/chapter/recommendations), Recommendation 1.1.18 | Pre-pregnancy HbA1c goal (NICE), if achievable without problematic hypoglycaemia. Equals 6.5% in NGSP units. Converted from the source's mmol/mol (48) to % using factor 0.09148 offset 2.152. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-1 | optimal | all | 5 | 5.5 | cohort | Nondiabetic adults, ARIC cohort (black and white, mean age about 54 years) | [Glycated hemoglobin, diabetes, and cardiovascular risk in nondiabetic adults.](https://pubmed.ncbi.nlm.nih.gov/20200384/), Abstract (Results) | Lowest-risk HbA1c category (reference group, HR 1.00) for diabetes and coronary heart disease in ARIC. The association with all-cause death was J-shaped, so this is not a proven optimum. Alternative to the default optimal optimal range for the same population. |
| alt-2 | warn | all | - | 6.543 | guideline | Adults | [Should Prediabetes Be Classified as a Treatable Disease?](https://pubmed.ncbi.nlm.nih.gov/41598648/), Section 2 (Diagnostic criteria) | Diabetes by HbA1c (ADA), expressed in IFCC units (mmol/mol). Converted from the source's mmol/mol (48) to % using factor 0.09148 offset 2.152. Source kind: diagnostic. Source band >= 6.543; stored as the warn high bound 6.543. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 6.5 | guideline | Adults | [The impact of glucose and metabolic disturbances on white matter hyperintensity volume in apparently healthy adults.](https://pubmed.ncbi.nlm.nih.gov/41928882/), Table 1 | Diabetes by HbA1c (ADA), confirm with a repeat test unless unequivocal symptoms or a second criterion is present. Source kind: diagnostic. Source band >= 6.5; stored as the warn high bound 6.5. |
| target | optimal | not pregnant, diabetes | - | 7 | guideline | Nonpregnant adults with diabetes | [Blood glucose monitoring devices for type 1 diabetes: a journey from the food and drug administration approval to market availability.](https://pubmed.ncbi.nlm.nih.gov/38559693/), Introduction (paragraph citing ADA recommendations) | Glycaemic treatment goal for most nonpregnant adults with diabetes (ADA), cited via a secondary review. Less stringent goals apply to some individuals. Source kind: target. |
| target | optimal | age 65+, diabetes | - | 7.5 | guideline | Older adults with diabetes in good health (few coexisting chronic illnesses, intact cognitive and functional status) | [Management of Glucose-Lowering Therapy in Older Adults with Type 2 Diabetes: Challenges and Opportunities.](https://pubmed.ncbi.nlm.nih.gov/37841649/), Table (Glycemic targets in older adults, ADA 2023 row) | HbA1c goal for healthy older adults with diabetes, as tabulated from ADA 2023 by a review. Less stringent goals apply to complex/intermediate health. Source kind: target. |
| target | optimal | age 65+, complex-intermediate-health, diabetes | - | 8 | guideline | Older adults with diabetes and complex/intermediate health (3 or more chronic illnesses, 2 or more instrumental ADL impairments, or mild-to-moderate cognitive impairment) | [Management of Glucose-Lowering Therapy in Older Adults with Type 2 Diabetes: Challenges and Opportunities.](https://pubmed.ncbi.nlm.nih.gov/37841649/), Table (Glycemic targets in older adults, ADA 2023 row) | HbA1c goal for complex/intermediate older adults with diabetes, as tabulated from ADA 2023 by a review. Limited life expectancy or very complex/poor health are not covered by this quote; the source lists separate goals for them. Source kind: target. |
| population-ri | reference | age 0-45 | 4.82 | 5.92 | reference-interval-study | Healthy Saudi adults over 18 years, under 45 years (IFCC multicenter study, 215 people; LAVE applied to A1C) | [Establishment of reference interval for hemoglobin A1C and other hemoglobin subfractions for healthy Saudi adults.](https://pubmed.ncbi.nlm.nih.gov/38527010/), Table 4 | Parametric reference interval for HbA1c (A1C) in healthy Saudi adults, age partition only (MF = both sexes; no sex or BMI partition). Values are in % as printed in the source table, which gives no unit column. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 45+ | 4.93 | 7.16 | reference-interval-study | Healthy Saudi adults over 18 years, 45 years and older (IFCC multicenter study, 100 people; LAVE applied to A1C) | [Establishment of reference interval for hemoglobin A1C and other hemoglobin subfractions for healthy Saudi adults.](https://pubmed.ncbi.nlm.nih.gov/38527010/), Table 4 | Parametric reference interval for HbA1c (A1C) in healthy Saudi adults aged 45 and over. The upper limit is considerably higher than the under-45 interval; the source says it is the highest upper limit compared with nine countries. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
