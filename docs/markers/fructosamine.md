# Fructosamine (`fructosamine`)

Glycated serum proteins (mainly albumin), reflecting average glucose over an intermediate period (about 2-4 weeks). Used as an intermediate-term glycaemic marker alongside HbA1c and fasting glucose.

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | µmol/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in µmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 194.8 | 258 | reference-interval-study | Healthy US Black and White adults in the ARIC community cohort (1799 people, mean age 55 years, 51% women) | [Establishment of Community-Based Reference Intervals for Fructosamine, Glycated Albumin, and 1,5-Anhydroglucitol.](https://pubmed.ncbi.nlm.nih.gov/29436378/), Abstract (Results) | Central 95 % interval from the 2.5th and 97.5th percentiles in a healthy reference population. The source writes the values as umol/L; they are copied exactly. Distributions differed by race, sex and BMI, and no partition is given in the abstract. |
| alt-1 | warn | all | - | 261.7 | reference-interval-study | Adults (derived from the ARIC healthy reference population) | [Establishment of Community-Based Reference Intervals for Fructosamine, Glycated Albumin, and 1,5-Anhydroglucitol.](https://pubmed.ncbi.nlm.nih.gov/29436378/), Abstract (Results) | Fructosamine concentration equivalent to a fasting glucose of 126 mg/dL (the 93.9th percentile of this reference population). This is a derived equivalent, not an official ADA diagnostic criterion. Source kind: diagnostic. Source band >= 261.7; stored as the warn high bound 261.7. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 270.2 | reference-interval-study | Adults (derived from the ARIC healthy reference population) | [Establishment of Community-Based Reference Intervals for Fructosamine, Glycated Albumin, and 1,5-Anhydroglucitol.](https://pubmed.ncbi.nlm.nih.gov/29436378/), Abstract (Results) | Fructosamine concentration equivalent to an HbA1c of 6.5 % (the 96.5th percentile of this reference population). This is a derived equivalent, not an official ADA diagnostic criterion. Source kind: diagnostic. Source band >= 270.2; stored as the warn high bound 270.2. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
