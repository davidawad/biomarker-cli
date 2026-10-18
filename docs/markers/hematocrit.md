# Hematocrit (`hematocrit`)

Percentage of whole blood volume occupied by red blood cells (packed cell volume). Low values indicate anemia; high values indicate erythrocytosis (for example polycythemia vera) or dehydration.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | % |
| LOINC | [4544-3](https://loinc.org/4544-3/) |
| Specimen | - |
| Aliases | `hct`, `haematocrit` |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 37.5 | 51 | - | - | uncited (legacy) | - |
| clinical | reference | female | 34 | 46.6 | - | - | uncited (legacy) | - |
| clinical | reference | all | 34 | 51 | - | - | uncited (legacy) | - |
| norip | reference | male, age 18-90 | 39.5 | 50 | reference-interval-study | 1826 healthy Nordic adults aged 18-90 (NORIP), venous EDTA blood, 12 analyser types | [A multicentre study of reference intervals for haemoglobin, basic blood cell counts and erythrocyte indices in the adult population of the Nordic countries.](https://pubmed.ncbi.nlm.nih.gov/15223702/), Abstract | Non-parametric 2.5th-97.5th percentiles; NORIP reports EVF as a fraction. |
| norip | reference | female, age 18-90 | 34.8 | 45.9 | reference-interval-study | 1826 healthy Nordic adults aged 18-90 (NORIP), venous EDTA blood, 12 analyser types | [A multicentre study of reference intervals for haemoglobin, basic blood cell counts and erythrocyte indices in the adult population of the Nordic countries.](https://pubmed.ncbi.nlm.nih.gov/15223702/), Abstract | Non-parametric 2.5th-97.5th percentiles; NORIP reports EVF as a fraction. |
| who | warn | male, age 18+ | - | 49 | guideline | adult men, WHO 2016 criteria for polycythaemia vera (as quoted in a 2021 Annals of Hematology paper) | [Diagnosis and management of non-clonal erythrocytosis remains challenging: a single centre clinical experience.](https://pubmed.ncbi.nlm.nih.gov/34013406/), Introduction | Haematocrit above this value is a WHO 2016 diagnostic criterion for polycythaemia vera (with Hb above 16.5 g/dL in men). Not a general reference limit. Source kind: diagnostic. Source band >= 49; stored as the warn high bound 49. |
| who | warn | female, age 18+ | - | 48 | guideline | adult women, WHO 2016 criteria for polycythaemia vera (as quoted in a 2021 Annals of Hematology paper) | [Diagnosis and management of non-clonal erythrocytosis remains challenging: a single centre clinical experience.](https://pubmed.ncbi.nlm.nih.gov/34013406/), Introduction | Haematocrit above this value is a WHO 2016 diagnostic criterion for polycythaemia vera (with Hb above 16.0 g/dL in women). Not a general reference limit. Source kind: diagnostic. Source band >= 48; stored as the warn high bound 48. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
