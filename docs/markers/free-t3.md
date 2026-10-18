# Free T3 (`free-t3`)

Unbound triiodothyronine, the most active thyroid hormone. Measured with TSH and FT4 to evaluate hyperthyroidism and T3 toxicosis. Most sources report pmol/L, which is kept in that unit on those ranges.

| | |
|---|---|
| Category | [thyroid](index.md#thyroid) |
| Canonical unit | pg/mL |
| LOINC | [3051-0](https://loinc.org/3051-0/) |
| Specimen | - |
| Aliases | `ft3`, `t3, free`, `free triiodothyronine` |

## Unit conversions

Into pg/mL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| pmol/L | 0.651 | 0 |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 2 | 4.4 | - | - | uncited (legacy) | - |
| ankara-multicenter-2026 | reference | male, age 18+ | 2.252 | 3.932 | reference-interval-study | Adults aged 18 and over from three Ankara hospitals, routine laboratory data, Maglumi chemiluminescence immunoassay (method: immunoassay) | [Age- and Sex-Specific Reference Intervals for TSH, FT4, and FT3 Derived from the Turkish Multi-Center Cohort.](https://pubmed.ncbi.nlm.nih.gov/42351459/), Abstract, Results | Indirect refineR estimate from routine data. Reported in pmol/L. Converted from the source's pmol/L (3.46-6.04) to pg/mL using factor 0.651. |
| ankara-multicenter-2026 | reference | female, age 18+ | 2.252 | 3.75 | reference-interval-study | Adults aged 18 and over from three Ankara hospitals, routine laboratory data, Maglumi chemiluminescence immunoassay (method: immunoassay) | [Age- and Sex-Specific Reference Intervals for TSH, FT4, and FT3 Derived from the Turkish Multi-Center Cohort.](https://pubmed.ncbi.nlm.nih.gov/42351459/), Abstract, Results | Indirect refineR estimate from routine data. Reported in pmol/L. Converted from the source's pmol/L (3.46-5.76) to pg/mL using factor 0.651. |
| chinese-pediatric-2021 | reference | male, age 0-0.0833 | 1.927 | 4.609 | reference-interval-study | Healthy Chinese children aged 1 day to 12 years (n=1,279), Mindray CL-6000i immunoassay | [Age- and sex-specific reference intervals for thyroid hormones in a Chinese pediatrics: a prospective observational study of 1,279 healthy children.](https://pubmed.ncbi.nlm.nih.gov/34765471/), Abstract, Results | First month of life, male infants; reported in pmol/L. Converted from the source's pmol/L (2.96-7.08) to pg/mL using factor 0.651. |
| chinese-pediatric-2021 | reference | female, age 0-0.0833 | 1.53 | 4.733 | reference-interval-study | Healthy Chinese children aged 1 day to 12 years (n=1,279), Mindray CL-6000i immunoassay | [Age- and sex-specific reference intervals for thyroid hormones in a Chinese pediatrics: a prospective observational study of 1,279 healthy children.](https://pubmed.ncbi.nlm.nih.gov/34765471/), Abstract, Results | First month of life, female infants; reported in pmol/L. Converted from the source's pmol/L (2.35-7.27) to pg/mL using factor 0.651. |
| iodine-sufficient-trimester-2019 | reference | pregnant, trimester 1 | 2.213 | 3.672 | reference-interval-study | TPOAb-negative pregnant women in an iodine-sufficient area, first trimester | [Reference Intervals for Thyroid Hormones During the First Trimester of Gestation: A Report from an Area with a Sufficient Iodine Level.](https://pubmed.ncbi.nlm.nih.gov/30861562/), Abstract, Results | 2.5th-97.5th percentile. Reported in pmol/L. Converted from the source's pmol/L (3.4-5.64) to pg/mL using factor 0.651. |
| iodine-sufficient-trimester-2019 | reference | female, not pregnant | 2.409 | 3.613 | reference-interval-study | TPOAb-negative non-pregnant women in an iodine-sufficient area | [Reference Intervals for Thyroid Hormones During the First Trimester of Gestation: A Report from an Area with a Sufficient Iodine Level.](https://pubmed.ncbi.nlm.nih.gov/30861562/), Abstract, Results | 2.5th-97.5th percentile, comparison group for the first-trimester study. Converted from the source's pmol/L (3.7-5.55) to pg/mL using factor 0.651. |
| multi-ethnic-uk-2025 | reference | pregnant, trimester 1 | 2.213 | 3.646 | reference-interval-study | Multi-ethnic pregnant women in the UK (49% non-white), Abbott Alinity platform (method: immunoassay) | [Thyroid reference ranges in pregnancy utilizing an Abbott Alinity platform in a multi-ethnic population in the UK.](https://pubmed.ncbi.nlm.nih.gov/40156169/), Abstract, Results | Reported in pmol/L. Converted from the source's pmol/L (3.4-5.6) to pg/mL using factor 0.651. |
| multi-ethnic-uk-2025 | reference | pregnant, trimester 2 | 2.148 | 3.581 | reference-interval-study | Multi-ethnic pregnant women in the UK (49% non-white), Abbott Alinity platform (method: immunoassay) | [Thyroid reference ranges in pregnancy utilizing an Abbott Alinity platform in a multi-ethnic population in the UK.](https://pubmed.ncbi.nlm.nih.gov/40156169/), Abstract, Results | Reported in pmol/L. Converted from the source's pmol/L (3.3-5.5) to pg/mL using factor 0.651. |
| multi-ethnic-uk-2025 | reference | pregnant, trimester 3 | 2.018 | 3.255 | reference-interval-study | Multi-ethnic pregnant women in the UK (49% non-white), Abbott Alinity platform (method: immunoassay) | [Thyroid reference ranges in pregnancy utilizing an Abbott Alinity platform in a multi-ethnic population in the UK.](https://pubmed.ncbi.nlm.nih.gov/40156169/), Abstract, Results | Reported in pmol/L. Converted from the source's pmol/L (3.1-5.0) to pg/mL using factor 0.651. |
| second-trimester-ethnicity-2007 | reference | pregnant, trimester 2 | 2.48 | 3.87 | reference-interval-study | Pregnant women in the US (Asian, Black, Hispanic and White), antibody-negative with normal TSH, ARCHITECT immunoassay | [Second-trimester reference intervals for thyroid tests: the role of ethnicity.](https://pubmed.ncbi.nlm.nih.gov/17712003/), Abstract, Results | Nonparametric 2.5th-97.5th percentile for all participants, pg/mL as printed. Source also gives 3.82-5.96 pmol/L. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
