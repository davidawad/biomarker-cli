# Total Iron Binding Capacity (`tibc`)

Capacity of serum transferrin to bind iron, reported as total iron-binding capacity (TIBC). It rises when transferrin is high, as in iron deficiency and pregnancy, and falls with low transferrin, as in inflammation, malnutrition and liver disease. TIBC is used with serum iron to calculate transferrin saturation.

| | |
|---|---|
| Category | [iron](index.md#iron) |
| Canonical unit | µg/dL |
| LOINC | [2500-7](https://loinc.org/2500-7/) |
| Specimen | - |
| Aliases | - |
| Panels | [Iron deficiency and iron deficiency anaemia](../panels/iron-anemia.md) |

## Unit conversions

Into µg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| µmol/L | 5.585 | 0 |

## Ranges

Bounds in µg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 250 | 450 | - | - | uncited (legacy) | - |
| russian-ifcc-2024 | reference | male | 256.91 | 398.77 | reference-interval-study | Healthy Russian adults aged 18-80 (46% male), IFCC C-RIDL protocol, Beckman Coulter AU 5800 | [Derivation of Russian-specific reference intervals for complete blood count, iron markers and related vitamins.](https://pubmed.ncbi.nlm.nih.gov/39348390/), Table 4, row TIBC, LAVE(+), All, M | 2.5-97.5 percentile limits from the paper's Table 4 (IFCC C-RIDL protocol, single country, Beckman AU 5800; method-specific). The paper labels rows LAVE(+) or LAVE(-), defined in its text as one or more vs no abnormal reference-test values; that does not clearly describe the exclusion applied to this row, so check the supplementary source data before relying on it. |
| russian-ifcc-2024 | reference | female | 262.5 | 437.86 | reference-interval-study | Healthy Russian adults aged 18-80 (54% female), IFCC C-RIDL protocol, Beckman Coulter AU 5800 | [Derivation of Russian-specific reference intervals for complete blood count, iron markers and related vitamins.](https://pubmed.ncbi.nlm.nih.gov/39348390/), Table 4, row TIBC, LAVE(+), All, F | 2.5-97.5 percentile limits from the paper's Table 4 (IFCC C-RIDL protocol, single country, Beckman AU 5800; method-specific). The paper labels rows LAVE(+) or LAVE(-), defined in its text as one or more vs no abnormal reference-test values; that does not clearly describe the exclusion applied to this row, so check the supplementary source data before relying on it. |
| uppsala-75y-2012 | reference | male, age 75-76 | 247.97 | 459.09 | reference-interval-study | Healthy 75-year-old men without diabetes, Sweden (n = 354), including individuals with cardiovascular disease | [Reference values of thirty-one frequently used laboratory markers for 75-year-old males and females.](https://pubmed.ncbi.nlm.nih.gov/22300333/), Methods; Table IV, row TIBC (µmol/L), Males, all | 2.5-97.5 percentile limits (IFCC method) from a Swedish cohort examined at age 75; TIBC was calculated from transferrin (TIBC = 25 x transferrin g/L) on the Vitros platform, as stated in the methods. Older-adult values. |
| uppsala-75y-2012 | reference | female, age 75-76 | 253 | 459.65 | reference-interval-study | Healthy 75-year-old women without diabetes, Sweden (n = 373), including individuals with cardiovascular disease | [Reference values of thirty-one frequently used laboratory markers for 75-year-old males and females.](https://pubmed.ncbi.nlm.nih.gov/22300333/), Table IV, row TIBC (µmol/L), Females, all | 2.5-97.5 percentile limits (IFCC method) from a Swedish cohort examined at age 75; TIBC calculated from transferrin, as stated in the methods. Older-adult values. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
