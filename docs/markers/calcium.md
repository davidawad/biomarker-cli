# Calcium (`calcium`)

Total serum calcium, the main laboratory measure of calcium status. Values are higher in infancy and early childhood than in adults, and albumin strongly affects the total value. Hypercalcaemia is graded by severity in clinical reviews.

| | |
|---|---|
| Category | [cmp](index.md#cmp) |
| Canonical unit | mg/dL |
| LOINC | [17861-6](https://loinc.org/17861-6/) |
| Specimen | - |
| Aliases | `ca` |
| Panels | [Alzheimer's disease and dementia workup](../panels/alzheimers.md), [Bone health and osteoporosis](../panels/bone-health.md), [Diabetes and glycemic control](../panels/diabetes.md), [Kidney function and CKD](../panels/kidney.md) |

## Unit conversions

Into mg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| mmol/L | 4.008 | 0 |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 8.6 | 10.2 | - | - | uncited (legacy) | - |
| alt-1 | reference | age 18-66 | 8.3 | 10.2 | reference-interval-study | Apparently healthy adults aged 18 years and over in Egypt (IFCC C-RIDL protocol, serum), both sexes pooled (method: albumin-adjusted) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Albumin-adjusted calcium (aCa), not total calcium. Central 95 % reference interval in mg/dL (conventional-unit columns of the source). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. Alternative to the default population-ri reference range for the same population. |
| alt-1 | warn | all | - | 14 | consensus | Adults with hypercalcaemia (moderate grade) | [A Review of Current Clinical Concepts in the Pathophysiology, Etiology, Diagnosis, and Management of Hypercalcemia.](https://pubmed.ncbi.nlm.nih.gov/35217631/), Definition, Epidemiology, and Screening for Hypercalcemia | Moderate hypercalcaemia (12.1 to 14 mg/dL), grading as stated in a narrative review of hypercalcaemia. Source kind: diagnostic. Source band 12.1-14; stored as the warn high bound 14. Alternative to the default diagnostic warn range for the same population. |
| alt-1 | warn | all | - | 14 | consensus | Adults with hypercalcaemia (severe grade) | [A Review of Current Clinical Concepts in the Pathophysiology, Etiology, Diagnosis, and Management of Hypercalcemia.](https://pubmed.ncbi.nlm.nih.gov/35217631/), Definition, Epidemiology, and Screening for Hypercalcemia | Severe hypercalcaemia (strictly above 14 mg/dL), grading as stated in a narrative review of hypercalcaemia. Source kind: diagnostic. Source band >= 14; stored as the warn high bound 14. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 12 | consensus | Adults with hypercalcaemia (mild grade) | [A Review of Current Clinical Concepts in the Pathophysiology, Etiology, Diagnosis, and Management of Hypercalcemia.](https://pubmed.ncbi.nlm.nih.gov/35217631/), Definition, Epidemiology, and Screening for Hypercalcemia | Mild hypercalcaemia (strictly above 10.5 mg/dL, up to 12 mg/dL), grading as stated in a narrative review of hypercalcaemia. Source kind: diagnostic. Source band 10.5-12; stored as the warn high bound 12. |
| population-ri | reference | age 0-2.5 | 9.9 | 11.84 | reference-interval-study | Healthy Iranian neonates, infants and children aged 0 to under 30 months (total serum calcium, CLSI EP28-A3 indirect method) | [Age-specific reference intervals for routine biochemical parameters in healthy neonates, infants, and young children in Iran.](https://pubmed.ncbi.nlm.nih.gov/36524863/), Table 1 | Total calcium in infancy; age partitioning was not required for calcium in this study. Upper limit is above the adult range. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 40-70 | 8.76 | 10.24 | reference-interval-study | UK Biobank adults aged 40 to 69 without CKD or vitamin D deficiency (albumin-adjusted calcium, not total calcium) | [Reference interval for albumin-adjusted calcium based on a large UK population.](https://pubmed.ncbi.nlm.nih.gov/32892370/), Results | 99% reference interval for albumin-adjusted serum calcium, not for total calcium. Upper limits were sex and age dependent in the source (for example 2.59 mmol/L in women aged 55-69, 2.55 mmol/L in men). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 18-66 | 8.7 | 10.7 | reference-interval-study | Apparently healthy adults aged 18 years and over in Egypt (IFCC C-RIDL protocol, serum), both sexes pooled; sex-specific total calcium intervals were not reported | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Total serum calcium, central 95 % reference interval in mg/dL (conventional-unit columns of the source). Age band 18~65 as printed in the source. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 0-2 | 9.379 | 10.94 | reference-interval-study | Healthy Chinese children aged 28 days to under 2 years (Shenzhen, residual serum, CLSI EP28-A3c nonparametric method) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Total calcium in mmol/L as stated in the source (not converted). Sex partitions were not reported for calcium in this age band. Converted from the source's mmol/L (2.34-2.73) to mg/dL using factor 4.008. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 2-5 | 9.579 | 10.74 | reference-interval-study | Healthy Chinese children aged 2 to under 5 years (Shenzhen, residual serum, CLSI EP28-A3c nonparametric method) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Total calcium in mmol/L as stated in the source (not converted). Converted from the source's mmol/L (2.39-2.68) to mg/dL using factor 4.008. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 5-18 | 9.459 | 10.54 | reference-interval-study | Healthy Chinese children aged 5 to under 18 years (Shenzhen, residual serum, CLSI EP28-A3c nonparametric method) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Total calcium in mmol/L as stated in the source (not converted). Converted from the source's mmol/L (2.36-2.63) to mg/dL using factor 4.008. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
