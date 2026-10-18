# BMI Percentile for Age and Sex (`bmi-percentile`)

BMI-for-age percentile for children and teens aged 2 to 19 years, compared with the sex- and age-specific CDC reference population. Interpreted as underweight, healthy weight, overweight or obesity; not applicable to adults.

| | |
|---|---|
| Category | [pediatric](index.md#pediatric) |
| Canonical unit | percentile |
| LOINC | [59576-9](https://loinc.org/59576-9/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in percentile. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | age 2-20 | - | 95 | guideline | US children and teens aged 2 through 19 years, sex-specific BMI-for-age percentile | [https://www.cdc.gov/bmi/child-teen-calculator/index.html](https://www.cdc.gov/bmi/child-teen-calculator/index.html), BMI Category Percentile Ranges Table; [Defining overweight in children using growth charts.](https://pubmed.ncbi.nlm.nih.gov/15495854/), Abstract | Overweight category (at risk for overweight in the 2004 source's terminology). Source band 85-95; stored as the warn high bound 95. |
| clinical | reference | age 2-20 | 5 | 85 | guideline | US children and teens aged 2 through 19 years, sex-specific BMI-for-age percentile | [https://www.cdc.gov/bmi/child-teen-calculator/index.html](https://www.cdc.gov/bmi/child-teen-calculator/index.html), BMI Category Percentile Ranges Table | Healthy weight category. |
| alt-1 | warn | age 2-20 | 5 | - | guideline | US children and teens aged 2 through 19 years, sex-specific BMI-for-age percentile | [https://www.cdc.gov/bmi/child-teen-calculator/index.html](https://www.cdc.gov/bmi/child-teen-calculator/index.html), BMI Category Percentile Ranges Table | Underweight category: BMI-for-age less than the 5th percentile. Source band <= 5; stored as the warn low bound 5. Alternative to the default clinical warn range for the same population. |
| diagnostic | warn | age 2-20 | - | 95 | guideline | US children and teens aged 2 through 19 years, sex-specific BMI-for-age percentile | [https://www.cdc.gov/bmi/child-teen-calculator/index.html](https://www.cdc.gov/bmi/child-teen-calculator/index.html), BMI Category Percentile Ranges Table; [Defining overweight in children using growth charts.](https://pubmed.ncbi.nlm.nih.gov/15495854/), Abstract | Obesity category. Severe obesity (120 percent of the 95th percentile or greater, or BMI of 35 or greater) is not written as a percentile range. Source kind: diagnostic. Source band >= 95; stored as the warn high bound 95. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
