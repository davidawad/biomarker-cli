# Newborn Screen TSH (Dried Blood Spot) (`newborn-tsh`)

Thyroid-stimulating hormone measured in a heel-prick dried blood spot on the newborn screening card (congenital hypothyroidism screen). Dried blood spot values run lower than serum TSH, so serum reference intervals do not apply.

| | |
|---|---|
| Category | [pediatric](index.md#pediatric) |
| Canonical unit | mIU/L |
| LOINC | [29575-8](https://loinc.org/29575-8/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mIU/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 0-0.0833 | 0.4 | 4.1 | reference-interval-study | Neonates sampled on days 5-8 of life, newborn screening dried blood spots (PerkinElmer neonatal hTSH kit, GSP analyser) | [Adult and neonate TSH blood spot reference intervals.](https://pubmed.ncbi.nlm.nih.gov/35751155/), Abstract (Results) | Age upper bound (first month) is applied here; the source specifies day 5-8 screening samples. Source states mU/L, which is numerically the same unit as the canonical mIU/L. |
| clinical | reference | age 18+ | 0.49 | 3.07 | reference-interval-study | Euthyroid adults not on thyroid medication with normal haematocrit, dried blood spot TSH (PerkinElmer neonatal hTSH kit, GSP analyser); comparator for the dried blood spot method | [Adult and neonate TSH blood spot reference intervals.](https://pubmed.ncbi.nlm.nih.gov/35751155/), Abstract (Results) | Adult comparator for the same dried blood spot method, not a newborn screening range. Source states mU/L. |
| clinical | warn | age 0-0.0833 | - | 20 | cohort | Neonates screened beyond 24 hours of life, capillary (dried blood spot) TSH, Delhi pilot programme (India) | [An Optimal Capillary Screen Cut-off of Thyroid Stimulating Hormone for Diagnosing Congenital Hypothyroidism: Data from a Pilot Newborn Screening Program in Delhi.](https://pubmed.ncbi.nlm.nih.gov/31064895/), Abstract (Conclusions) | Screen-positive recall threshold from a diagnostic accuracy pilot (102 confirmed cases of 174 referrals). Not a guideline cut-off; population-specific. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
