# Iron, Serum (`iron`)

Iron circulating bound to transferrin, measured as serum iron. It varies widely through the day and with recent intake, so a single value is weak evidence on its own; low values with low transferrin saturation and ferritin point to iron deficiency, while high values with high transferrin saturation point to iron overload.

| | |
|---|---|
| Category | [iron](index.md#iron) |
| Canonical unit | µg/dL |
| LOINC | [2498-4](https://loinc.org/2498-4/) |
| Specimen | - |
| Aliases | `serum iron`, `fe` |

## Unit conversions

Into µg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| µmol/L | 5.585 | 0 |

## Ranges

Bounds in µg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 38 | 169 | - | - | uncited (legacy) | - |
| clinical | reference | female | 27 | 159 | - | - | uncited (legacy) | - |
| clinical | reference | all | 27 | 169 | - | - | uncited (legacy) | - |
| russian-ifcc-2024 | reference | male | 54.73 | 196.03 | reference-interval-study | Healthy Russian adults aged 18-80 (506 participants, 46% male), IFCC C-RIDL protocol, Beckman Coulter AU 5800 | [Derivation of Russian-specific reference intervals for complete blood count, iron markers and related vitamins.](https://pubmed.ncbi.nlm.nih.gov/39348390/), Table 4, row Fe, LAVE(-), All, M | 2.5-97.5 percentile limits from the paper's Table 4 (IFCC C-RIDL protocol, single country, Beckman AU 5800; method-specific). The paper labels rows LAVE(+) or LAVE(-), defined in its text as one or more vs no abnormal reference-test values; that does not clearly describe the exclusion applied to this row, so check the supplementary source data before relying on it. |
| russian-ifcc-2024 | reference | female | 40.21 | 170.34 | reference-interval-study | Healthy Russian adults aged 18-80 (506 participants, 54% female), IFCC C-RIDL protocol, Beckman Coulter AU 5800 | [Derivation of Russian-specific reference intervals for complete blood count, iron markers and related vitamins.](https://pubmed.ncbi.nlm.nih.gov/39348390/), Table 4, row Fe, LAVE(+), All, F | 2.5-97.5 percentile limits from the paper's Table 4 (IFCC C-RIDL protocol, single country, Beckman AU 5800; method-specific). The paper labels rows LAVE(+) or LAVE(-), defined in its text as one or more vs no abnormal reference-test values; that does not clearly describe the exclusion applied to this row, so check the supplementary source data before relying on it. |
| uppsala-75y-2012 | reference | male, age 75-76 | 46.36 | 191.57 | reference-interval-study | Healthy 75-year-old men without diabetes, Sweden (n = 354), including individuals with cardiovascular disease | [Reference values of thirty-one frequently used laboratory markers for 75-year-old males and females.](https://pubmed.ncbi.nlm.nih.gov/22300333/), Table IV, row Iron (µmol/L), Males, all | 2.5-97.5 percentile limits (IFCC method) from a Swedish cohort examined at age 75, Ortho Vitros assay. Older-adult values; not a general adult interval. The paper also reports a subgroup without cardiovascular disease that is nearly identical (8.4-34.8 umol/L). |
| uppsala-75y-2012 | reference | female, age 75-76 | 49.15 | 160.85 | reference-interval-study | Healthy 75-year-old women without diabetes, Sweden (n = 373), including individuals with cardiovascular disease | [Reference values of thirty-one frequently used laboratory markers for 75-year-old males and females.](https://pubmed.ncbi.nlm.nih.gov/22300333/), Table IV, row Iron (µmol/L), Females, all | 2.5-97.5 percentile limits (IFCC method) from a Swedish cohort examined at age 75, Ortho Vitros assay. Older-adult values; not a general adult interval. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
