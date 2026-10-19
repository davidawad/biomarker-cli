# Urine Creatinine (`urine-creatinine`)

Urine Creatinine. Tracked marker with no verified reference range yet, so values can be stored and charted but are not flagged against a range.

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | mg/dL |
| LOINC | [2161-8](https://loinc.org/2161-8/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | 30 | - | guideline | Spot urine samples for occupational biomonitoring (specimen: spot-urine) | [Urinary creatinine concentrations in the U.S. population: implications for urinary biologic monitoring measurements.](https://pubmed.ncbi.nlm.nih.gov/15687057/), Introduction | WHO: too dilute; collect another void. Source band <= 30; stored as the warn low bound 30. |
| clinical | reference | male | 49.56 | 254.4 | reference-interval-study | US males aged 6-90 years (NHANES III), spot urine; 10th-90th percentiles (specimen: spot-urine) | [Urinary creatinine concentrations in the U.S. population: implications for urinary biologic monitoring measurements.](https://pubmed.ncbi.nlm.nih.gov/15687057/), Table 2 | 10th-90th percentile limits (not 2.5-97.5), n=10,610. |
| clinical | reference | female | 27.36 | 217.7 | reference-interval-study | US females aged 6-90 years (NHANES III), spot urine; 10th-90th percentiles (specimen: spot-urine) | [Urinary creatinine concentrations in the U.S. population: implications for urinary biologic monitoring measurements.](https://pubmed.ncbi.nlm.nih.gov/15687057/), Table 2 | 10th-90th percentile limits (not 2.5-97.5), n=11,635. |
| alt-1 | warn | all | - | 300 | guideline | Spot urine samples for occupational biomonitoring (specimen: spot-urine) | [Urinary creatinine concentrations in the U.S. population: implications for urinary biologic monitoring measurements.](https://pubmed.ncbi.nlm.nih.gov/15687057/), Introduction | WHO: too concentrated; collect another void. Source band >= 300; stored as the warn high bound 300. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
