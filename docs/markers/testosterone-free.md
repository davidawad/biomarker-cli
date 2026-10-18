# Testosterone, Free (`testosterone-free`)

Unbound (free) testosterone in serum, measured by equilibrium dialysis or calculated from total testosterone and SHBG. Preferred when SHBG is altered or total testosterone is near the lower limit of normal.

| | |
|---|---|
| Category | [hormone](index.md#hormone) |
| Canonical unit | pg/mL |
| LOINC | [2991-8](https://loinc.org/2991-8/) |
| Specimen | - |
| Aliases | `free testosterone`, `ft` |

## Unit conversions

Into pg/mL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| pmol/L | 0.2884 | 0 |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 46 | 224 | - | - | uncited (legacy) | - |
| clinical | reference | female | 0 | 4.2 | - | - | uncited (legacy) | - |
| ft-ed-standardized-2023 | reference | male | 66 | 309 | reference-interval-study | Healthy nonobese men aged 19 and older (n=145; age 19 or older per the source), free testosterone by standardized 16-h equilibrium dialysis with CDC-certified LC-MS/MS (method: equilibrium-dialysis) | [Reference intervals for free testosterone in adult men measured using a standardized equilibrium dialysis procedure.](https://pubmed.ncbi.nlm.nih.gov/36251328/), Conclusion | Normative range in all men of the source population (percentile-based). Source gives both pg/ml and pmol/L; this entry uses the canonical pg/mL. |
| ft-ed-standardized-2023 | reference | male, age 19-39 | 120 | 368 | reference-interval-study | Healthy nonobese men aged 19-39 (same cohort as the all-men range), free testosterone by standardized equilibrium dialysis with LC-MS/MS (method: equilibrium-dialysis) | [Reference intervals for free testosterone in adult men measured using a standardized equilibrium dialysis procedure.](https://pubmed.ncbi.nlm.nih.gov/36251328/), Conclusion | Young-men reference interval. Values fall with age, BMI and SHBG in the same cohort. |
| mft-age-strat-2026 | reference | male, age 18-39 | 53.07 | 216 | reference-interval-study | Healthy, nonobese (BMI under 30) men aged 18-39 from large community cohorts (1202 men), free testosterone by equilibrium dialysis with LC-MS/MS (method: equilibrium-dialysis) | [Age-Stratified Reference Ranges for Directly Measured Serum Free Testosterone in Community-Dwelling and Healthy Men.](https://pubmed.ncbi.nlm.nih.gov/40929374/), Abstract, Results | 2.5th to 97.5th percentile (CLSI guidelines). Source unit is pmol/L; canonical here is pg/mL. Values are age-decline dependent (about -4.5 pmol/L per year). Converted from the source's pmol/L (184-749) to pg/mL using factor 0.2884. |
| premenopausal-women-2011 | reference | female, age 30-31 | 1.2 | 6.4 | reference-interval-study | Normally cycling healthy premenopausal women aged 18-49 (n=161); percentiles estimated for a typical 30-year-old woman, pooled across menstrual cycle phases | [Testosterone reference ranges in normally cycling healthy premenopausal women.](https://pubmed.ncbi.nlm.nih.gov/21771278/), Abstract, Results | 5th to 95th percentile for a 30-year-old woman (measured free testosterone). Assay/method not specified in the abstract. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
