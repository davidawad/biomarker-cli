# DHEA-Sulfate (`dhea-s`)

Sulfated adrenal androgen produced mainly by the zona reticularis; it reflects adrenal androgen output and rises at adrenarche. Used to assess adrenal androgen excess, hyperandrogenism in women, and adrenal disorders.

| | |
|---|---|
| Category | [hormone](index.md#hormone) |
| Canonical unit | µg/dL |
| LOINC | [2191-5](https://loinc.org/2191-5/) |
| Specimen | - |
| Aliases | `dheas`, `dhea sulfate` |

## Unit conversions

Into µg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| µmol/L | 36.81 | 0 |

## Ranges

Bounds in µg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 35 | 430 | - | - | uncited (legacy) | - |
| adrenal-steroids-lcms-2017 | reference | male | 33.79 | 369.3 | reference-interval-study | 232 men aged 18-81, with and without hypertension; morning fasting supine sampling (specimen: plasma; method: lc-ms/ms) | [Reference intervals for plasma concentrations of adrenal steroids measured by LC-MS/MS: Impact of gender, age, oral contraceptives, body mass index and blood pressure status.](https://pubmed.ncbi.nlm.nih.gov/28479316/), Table 3 (columns: ALL \| Males \| Females; median then [2.5th-97.5th percentile]) | 2.5th-97.5th percentile in nmol/L (917-10,021 nmol/L = 0.92-10.0 umol/L); falls with age. Converted from the source's nmol/L (917-10021) to µg/dL using factor 0.03685. |
| adrenal-steroids-lcms-2017 | reference | female | 31.32 | 274 | reference-interval-study | 293 women aged 18-81, with and without hypertension; morning fasting supine sampling (specimen: plasma; method: lc-ms/ms) | [Reference intervals for plasma concentrations of adrenal steroids measured by LC-MS/MS: Impact of gender, age, oral contraceptives, body mass index and blood pressure status.](https://pubmed.ncbi.nlm.nih.gov/28479316/), Table 3 (columns: ALL \| Males \| Females; median then [2.5th-97.5th percentile]) | 2.5th-97.5th percentile in nmol/L, pooled across ages and menopausal status; postmenopausal women are lower. Converted from the source's nmol/L (850-7435) to µg/dL using factor 0.03685. |
| caliper | reference | age 9-13 | 15.85 | 207.1 | reference-interval-study | Healthy children and adolescents of both sexes aged 9 to under 13 years (CALIPER reference population, as reproduced in a Croatian verification study on the Beckman Coulter DxI600 immunoassay) | [Verification of the Canadian Laboratory Initiative on Paediatric Reference Intervals (CALIPER) reference values in Croatian children and adolescents.](https://pubmed.ncbi.nlm.nih.gov/32550818/), Table 1 | CALIPER 2.5th-97.5th percentile interval as printed in the verification study's table (values in umol/L). The upper age bound is exclusive in the source ('<13 years'). Immunoassay-specific; CALIPER values were not re-derived by the verification study. Converted from the source's µmol/L (0.43-5.62) to µg/dL using factor 36.85. |
| roche-cobas-2023 | reference | female, age 20-25 | 134.5 | 470.2 | reference-interval-study | Healthy women aged 20-25 y, indirect (partitioned) reference interval from Roche Cobas electrochemiluminescence immunoassay data (3500 subjects aged 20-45 y) (method: immunoassay) | [Androgens in women: Establishing reference intervals for dehydroepiandrostenedione sulphate and androstenedione on the Roche Cobas.](https://pubmed.ncbi.nlm.nih.gov/37324111/), Results | 95% reference interval (2.5th-97.5th percentile) in the source unit umol/L; population-specific for this immunoassay, not for LC-MS/MS. Band boundaries as printed in the source. Converted from the source's µmol/L (3.65-12.76) to µg/dL using factor 36.85. |
| roche-cobas-2023 | reference | female, age 25-35 | 109.4 | 423.8 | reference-interval-study | Healthy women aged 25-35 y, indirect reference interval from Roche Cobas electrochemiluminescence immunoassay data (method: immunoassay) | [Androgens in women: Establishing reference intervals for dehydroepiandrostenedione sulphate and androstenedione on the Roche Cobas.](https://pubmed.ncbi.nlm.nih.gov/37324111/), Results | 95% reference interval in the source unit umol/L; immunoassay-specific. Band boundaries as printed in the source. Converted from the source's µmol/L (2.97-11.5) to µg/dL using factor 36.85. |
| roche-cobas-2023 | reference | female, age 35-45 | 84.75 | 362.2 | reference-interval-study | Healthy women aged 35-45 y, indirect reference interval from Roche Cobas electrochemiluminescence immunoassay data (method: immunoassay) | [Androgens in women: Establishing reference intervals for dehydroepiandrostenedione sulphate and androstenedione on the Roche Cobas.](https://pubmed.ncbi.nlm.nih.gov/37324111/), Results | 95% reference interval in the source unit umol/L; immunoassay-specific. Band boundaries as printed in the source. Converted from the source's µmol/L (2.3-9.83) to µg/dL using factor 36.85. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
