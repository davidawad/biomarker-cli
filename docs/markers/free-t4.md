# Free T4 (`free-t4`)

Unbound thyroxine in serum, the biologically active thyroid hormone fraction. Used with TSH to classify hypo- and hyperthyroidism. Many sources report pmol/L; ranges in pmol/L are kept in that unit.

| | |
|---|---|
| Category | [thyroid](index.md#thyroid) |
| Canonical unit | ng/dL |
| LOINC | [3024-7](https://loinc.org/3024-7/) |
| Specimen | - |
| Aliases | `ft4`, `t4, free`, `free thyroxine` |

## Unit conversions

Into ng/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| pmol/L | 0.0777 | 0 |

## Ranges

Bounds in ng/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.82 | 1.77 | - | - | uncited (legacy) | - |
| ankara-multicenter-2026 | reference | age 18+ | 1.002 | 1.671 | reference-interval-study | Adults aged 18 and over from three Ankara hospitals, routine laboratory data, Maglumi chemiluminescence immunoassay (method: immunoassay) | [Age- and Sex-Specific Reference Intervals for TSH, FT4, and FT3 Derived from the Turkish Multi-Center Cohort.](https://pubmed.ncbi.nlm.nih.gov/42351459/), Abstract, Results | Approximate adult interval (indirect refineR estimate, sex not separated for FT4). Reported in pmol/L. Converted from the source's pmol/L (12.9-21.5) to ng/dL using factor 0.0777. |
| chinese-pediatric-2021 | reference | male, age 0-0.0833 | 1.037 | 2.226 | reference-interval-study | Healthy Chinese children aged 1 day to 12 years (n=1,279), Mindray CL-6000i immunoassay | [Age- and sex-specific reference intervals for thyroid hormones in a Chinese pediatrics: a prospective observational study of 1,279 healthy children.](https://pubmed.ncbi.nlm.nih.gov/34765471/), Abstract, Results | First month of life, male infants; reported in pmol/L. Converted from the source's pmol/L (13.34-28.65) to ng/dL using factor 0.0777. |
| chinese-pediatric-2021 | reference | female, age 0-0.0833 | 1.074 | 2.473 | reference-interval-study | Healthy Chinese children aged 1 day to 12 years (n=1,279), Mindray CL-6000i immunoassay | [Age- and sex-specific reference intervals for thyroid hormones in a Chinese pediatrics: a prospective observational study of 1,279 healthy children.](https://pubmed.ncbi.nlm.nih.gov/34765471/), Abstract, Results | First month of life, female infants; reported in pmol/L. Converted from the source's pmol/L (13.82-31.83) to ng/dL using factor 0.0777. |
| gestational-japan-2022 | reference | pregnant, trimester 1 | 1.1 | 1.87 | reference-interval-study | Japanese pregnant women with negative thyroglobulin and TPO antibodies | [Gestational trimester-specific reference ranges for serum thyrotropin and free thyroxine in Japanese.](https://pubmed.ncbi.nlm.nih.gov/35811133/), Abstract, Results | 2.5th-97.5th percentile, antibody-negative samples; immunoassay. |
| gestational-japan-2022 | reference | pregnant, trimester 2 | 0.76 | 1.56 | reference-interval-study | Japanese pregnant women with negative thyroglobulin and TPO antibodies | [Gestational trimester-specific reference ranges for serum thyrotropin and free thyroxine in Japanese.](https://pubmed.ncbi.nlm.nih.gov/35811133/), Abstract, Results | 2.5th-97.5th percentile, antibody-negative samples. |
| gestational-japan-2022 | reference | pregnant, trimester 3 | 0.76 | 1.14 | reference-interval-study | Japanese pregnant women with negative thyroglobulin and TPO antibodies | [Gestational trimester-specific reference ranges for serum thyrotropin and free thyroxine in Japanese.](https://pubmed.ncbi.nlm.nih.gov/35811133/), Abstract, Results | 2.5th-97.5th percentile, antibody-negative samples. |
| iodine-sufficient-trimester-2019 | reference | pregnant, trimester 1 | 0.6993 | 1.4 | reference-interval-study | TPOAb-negative pregnant women in an iodine-sufficient area, first trimester | [Reference Intervals for Thyroid Hormones During the First Trimester of Gestation: A Report from an Area with a Sufficient Iodine Level.](https://pubmed.ncbi.nlm.nih.gov/30861562/), Abstract, Results | 2.5th-97.5th percentile. Reported in pmol/L. Converted from the source's pmol/L (9.0-18.02) to ng/dL using factor 0.0777. |
| iodine-sufficient-trimester-2019 | reference | female, not pregnant | 0.7397 | 1.5 | reference-interval-study | TPOAb-negative non-pregnant women in an iodine-sufficient area | [Reference Intervals for Thyroid Hormones During the First Trimester of Gestation: A Report from an Area with a Sufficient Iodine Level.](https://pubmed.ncbi.nlm.nih.gov/30861562/), Abstract, Results | 2.5th-97.5th percentile, comparison group for the first-trimester study. Converted from the source's pmol/L (9.52-19.3) to ng/dL using factor 0.0777. |
| multi-ethnic-uk-2025 | reference | pregnant, trimester 1 | 0.7692 | 1.189 | reference-interval-study | Multi-ethnic pregnant women in the UK (49% non-white), Abbott Alinity platform (method: immunoassay) | [Thyroid reference ranges in pregnancy utilizing an Abbott Alinity platform in a multi-ethnic population in the UK.](https://pubmed.ncbi.nlm.nih.gov/40156169/), Abstract, Results | Reported in pmol/L; kept in source unit. Converted from the source's pmol/L (9.9-15.3) to ng/dL using factor 0.0777. |
| multi-ethnic-uk-2025 | reference | pregnant, trimester 2 | 0.6605 | 1.119 | reference-interval-study | Multi-ethnic pregnant women in the UK (49% non-white), Abbott Alinity platform (method: immunoassay) | [Thyroid reference ranges in pregnancy utilizing an Abbott Alinity platform in a multi-ethnic population in the UK.](https://pubmed.ncbi.nlm.nih.gov/40156169/), Abstract, Results | Reported in pmol/L; kept in source unit. Converted from the source's pmol/L (8.5-14.4) to ng/dL using factor 0.0777. |
| multi-ethnic-uk-2025 | reference | pregnant, trimester 3 | 0.5905 | 0.9557 | reference-interval-study | Multi-ethnic pregnant women in the UK (49% non-white), Abbott Alinity platform (method: immunoassay) | [Thyroid reference ranges in pregnancy utilizing an Abbott Alinity platform in a multi-ethnic population in the UK.](https://pubmed.ncbi.nlm.nih.gov/40156169/), Abstract, Results | Reported in pmol/L; kept in source unit. Converted from the source's pmol/L (7.6-12.3) to ng/dL using factor 0.0777. |
| second-trimester-ethnicity-2007 | reference | pregnant, trimester 2 | 0.72 | 1.18 | reference-interval-study | Pregnant women in the US (Asian, Black, Hispanic and White), antibody-negative with normal TSH, ARCHITECT immunoassay | [Second-trimester reference intervals for thyroid tests: the role of ethnicity.](https://pubmed.ncbi.nlm.nih.gov/17712003/), Abstract, Results | Nonparametric 2.5th-97.5th percentile for all participants. The source also gives the value in pmol/L (9.3-15.2). |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
