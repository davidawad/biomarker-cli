# Globulin (calculated) (`globulin`)

Calculated globulin (total protein minus albumin). Raised globulin reflects immunoglobulin and acute-phase protein levels and is associated with higher mortality in dialysis patients.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | g/dL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in g/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male, age 18+ | 2.6 | 4.1 | reference-interval-study | Healthy adult males (Gandaki Province, Nepal; dry-chemistry method) | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Results (sex-specific 95% RIs) | - |
| clinical | reference | female, age 18+ | 2.7 | 4 | reference-interval-study | Healthy adult females (Gandaki Province, Nepal; dry-chemistry method) | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Results (sex-specific 95% RIs) | - |
| clinical | reference | age 18+ | 2.7 | 4.2 | reference-interval-study | Healthy adults (Gandaki Province, Nepal; dry-chemistry method), sexes combined | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Results (combined 95% RIs) | - |
| clinical | reference | age 18-66 | 2 | 3.7 | reference-interval-study | Adults aged 18 to 65 years, apparently healthy Egyptian volunteers (IFCC C-RIDL protocol); sexes combined | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Sex-combined adult 95% reference interval (calculated analyte in the source). The conventional-unit block is labelled g/L, but the values are g/dL (the SI block is 20-37 g/L); used as g/dL, not converted. |
| clinical | warn | hemodialysis | - | 3.8 | cohort | Incident hemodialysis patients (retrospective cohort, 2007-2011) | [Association of serum globulin with all-cause mortality in incident hemodialysis patients.](https://pubmed.ncbi.nlm.nih.gov/34617572/), Abstract (Results) | Globulin above 3.8 g/dL was associated with higher all-cause and infection-related mortality (fully adjusted HR 1.11 for all-cause) versus the 3.0 to <3.2 g/dL reference group. Association only; not a treatment target. Source band >= 3.8; stored as the warn high bound 3.8. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
