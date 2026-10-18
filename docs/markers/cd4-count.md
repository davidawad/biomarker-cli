# CD4+ T-cell Count (`cd4-count`)

Number of CD4-positive T lymphocytes per microlitre of blood. It is the main marker of immune function in HIV, used to stage disease and guide opportunistic infection prophylaxis.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | cells/µL |
| LOINC | [24467-3](https://loinc.org/24467-3/) |
| Specimen | - |
| Aliases | `cd4 count`, `cd4+ t-lymphocyte count`, `cd4 t-cell count` |

## Ranges

Bounds in cells/µL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 421.4 | 1523.8 | reference-interval-study | 351 healthy Brazilian adults in ELSA-Brasil; reference intervals from the 2.5th and 97.5th percentiles (age and sex differences were reported but are not split in the abstract) | [Establishing the reference intervals for subpopulations of T lymphocytes, B lymphocytes, and natural killer cells: insight from the Brazilian Longitudinal Study of Adult Health (ELSA-Brasil).](https://pubmed.ncbi.nlm.nih.gov/41124450/), Abstract, Results | Absolute CD4+ T-cell reference interval in healthy adults (2.5th to 97.5th percentile), flow cytometry. Population-specific to this Brazilian cohort; interval boundaries are inclusive. |
| alt-1 | warn | hiv | - | 500 | guideline | HIV-infected adults and adolescents, CDC clinical classification system | [https://www.cdc.gov/mmwr/preview/mmwrhtml/00018871.htm](https://www.cdc.gov/mmwr/preview/mmwrhtml/00018871.htm), Section: CD4+ T-Lymphocyte Categories | CDC category 1: CD4 count of 500 cells/µL or more (boundary >=500). The source text prints the unit as cells/mL in this category label; the threshold itself is the cells/µL figure used in the category table. Source kind: decision. Source band >= 500; stored as the warn high bound 500. Alternative to the default cdc-1992-hiv-classification warn range for the same population. |
| cdc-1992-hiv-classification | warn | hiv | 200 | - | guideline | HIV-infected adults and adolescents, CDC clinical classification system | [https://www.cdc.gov/mmwr/preview/mmwrhtml/00018871.htm](https://www.cdc.gov/mmwr/preview/mmwrhtml/00018871.htm), Section: CD4+ T-Lymphocyte Categories | CDC category 3 (AIDS threshold): CD4 count below 200 cells/µL (boundary <200). Category 3 is the threshold that also defines the AIDS case definition. Source kind: decision. Source band <= 200; stored as the warn low bound 200. |
| cdc-1992-hiv-classification | warn | hiv | 200 | - | guideline | HIV-infected adults and adolescents, CDC clinical classification system | [https://www.cdc.gov/mmwr/preview/mmwrhtml/00018871.htm](https://www.cdc.gov/mmwr/preview/mmwrhtml/00018871.htm), Section: CD4+ T-Lymphocyte Categories | CDC category 2: CD4 count 200 to 499 cells/µL. Source kind: decision. Source band 200-499; stored as the warn low bound 200. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
