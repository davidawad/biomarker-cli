# Uric Acid (`uric-acid`)

Serum urate. Hyperuricaemia is the main risk factor for gout; treat-to-target urate-lowering therapy aims for a serum urate below 6 mg/dL in people with gout. Reference intervals are sex-specific.

| | |
|---|---|
| Category | [metabolic](index.md#metabolic) |
| Canonical unit | mg/dL |
| LOINC | [3084-1](https://loinc.org/3084-1/) |
| Specimen | - |
| Aliases | `urate` |

## Unit conversions

Into mg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| µmol/L | 0.01681 | 0 |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 3.7 | 8 | - | - | uncited (legacy) | - |
| clinical | reference | female | 2.5 | 7.1 | - | - | uncited (legacy) | - |
| clinical | reference | all | 2.5 | 8 | - | - | uncited (legacy) | - |
| optimal | optimal | all | - | 6 | - | - | uncited (legacy) | - |
| alt-1 | optimal | gout | - | 5 | guideline | Adults with severe gout receiving urate-lowering therapy | [2016 updated EULAR evidence-based recommendations for the management of gout.](https://pubmed.ncbi.nlm.nih.gov/27457514/), Abstract (Results) | Stricter serum urate target below 5 mg/dL (300 umol/L) in severe gout (2016 EULAR gout recommendations). Source kind: target. Alternative to the default target optimal range for the same population. |
| diagnostic | warn | all | - | 6.8 | guideline | Adults with asymptomatic hyperuricaemia (no prior gout flares or subcutaneous tophi) | [2020 American College of Rheumatology Guideline for the Management of Gout.](https://pubmed.ncbi.nlm.nih.gov/32391934/), Scope of the guideline | Asymptomatic hyperuricaemia as defined in the 2020 ACR gout guideline (serum urate of 6.8 mg/dL or higher). Sex-specific hyperuricaemia cut-offs (>7 men, >6 women) were not found in retrieved text and are not included. Source kind: diagnostic. Source band >= 6.8; stored as the warn high bound 6.8. |
| target | optimal | gout | - | 6 | guideline | Adults with gout receiving urate-lowering therapy | [2020 American College of Rheumatology Guideline for the Management of Gout.](https://pubmed.ncbi.nlm.nih.gov/32391934/), Treat-to-target recommendation | Treat-to-target serum urate goal below 6 mg/dL (ACR 2020 gout guideline). Source kind: target. |
| target | optimal | gout | - | 6 | guideline | Adults with gout receiving urate-lowering therapy | [2016 updated EULAR evidence-based recommendations for the management of gout.](https://pubmed.ncbi.nlm.nih.gov/27457514/), Abstract (Results) | Serum urate target below 6 mg/dL (360 umol/L) from the 2016 EULAR gout recommendations. Source kind: target. |
| population-ri | reference | male, age 18-66 | 2.8 | 7.9 | reference-interval-study | Adults aged 18 to 65 years, apparently healthy Egyptian volunteers (IFCC C-RIDL protocol, parametric method); males | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Values in mg/dL as stated in the source table (the source gives the matching umol/L limits as 168-479). Male RI is not age-partitioned in the source. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | female, age 18-45 | 2.4 | 5.8 | reference-interval-study | Adults aged 18 to under 45 years, apparently healthy Egyptian volunteers (IFCC C-RIDL protocol); females | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Female urate is partitioned at 45 years in the source; values in mg/dL as stated in the source table. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | female, age 45+ | 2.6 | 7.2 | reference-interval-study | Females aged 45 years and older, apparently healthy Egyptian volunteers (IFCC C-RIDL protocol) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | Values in mg/dL as stated in the source table. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 0.077-1 | 1.679 | 6.495 | reference-interval-study | Healthy Chinese children aged 28 days to under 1 year (Shenzhen, check-up children; CLSI EP28) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Sub-year pediatric 95% interval from a 2026 Chinese reference-interval study (Table 1). Converted from the source's µmol/L (99.91-386.35) to mg/dL using factor 0.01681. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 1-11 | 2.689 | 6.192 | reference-interval-study | Healthy Chinese children aged 1 to under 11 years (Shenzhen, check-up children; CLSI EP28) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Sub-year pediatric 95% interval from a 2026 Chinese reference-interval study (Table 1). Converted from the source's µmol/L (159.96-368.38) to mg/dL using factor 0.01681. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | male, age 11-18 | 3.274 | 8.085 | reference-interval-study | Healthy Chinese male children aged 11 to under 18 years (Shenzhen, check-up children; CLSI EP28) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Sex-specific pediatric 95% interval from a 2026 Chinese reference-interval study (Table 1). Converted from the source's µmol/L (194.76-480.95) to mg/dL using factor 0.01681. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | female, age 11-18 | 2.083 | 6.651 | reference-interval-study | Healthy Chinese female children aged 11 to under 18 years (Shenzhen, check-up children; CLSI EP28) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | Sex-specific pediatric 95% interval from a 2026 Chinese reference-interval study (Table 1). Converted from the source's µmol/L (123.9-395.65) to mg/dL using factor 0.01681. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
