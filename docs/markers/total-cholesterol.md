# Total Cholesterol (`total-cholesterol`)

Cholesterol carried in all apolipoprotein B and HDL particles. Interpreted with LDL-C, HDL-C and triglycerides in the lipid profile; adult ATP III cut-points, pediatric NHLBI cut-points, non-fasting flags and pregnancy reference intervals.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | mg/dL |
| LOINC | [2093-3](https://loinc.org/2093-3/) |
| Specimen | - |
| Aliases | `tc`, `cholesterol`, `chol`, `cholesterol, total` |
| Panels | [Cardiovascular risk (lipids and risk enhancers)](../panels/cardiovascular-risk.md), [Diabetes and glycemic control](../panels/diabetes.md) |

## Unit conversions

Into mg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| mmol/L | 38.67 | 0 |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 200 | - | - | uncited (legacy) | - |
| clinical | warn | all | - | 239 | - | - | uncited (legacy) | 200-239 mg/dL: borderline high |
| clinical | reference | age 20+ | - | 200 | guideline | Adults aged 20 years or older | [https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | Desirable: strictly below 200 mg/dL (ATP III). |
| clinical | warn | age 20+ | - | 239 | guideline | Adults aged 20 years or older | [https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | Borderline high total cholesterol (ATP III). Source band 200-239; stored as the warn high bound 239. |
| clinical | reference | age 0-20 | - | 170 | guideline | Children and adolescents (NHLBI expert panel, Table 9-1) | [https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf), Table 9-1 | Acceptable pediatric total cholesterol: strictly below 170 mg/dL. |
| clinical | warn | age 0-20 | - | 199 | guideline | Children and adolescents (NHLBI expert panel, Table 9-1) | [https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf), Table 9-1 | Pediatric borderline-high total cholesterol. Source band 170-199; stored as the warn high bound 199. |
| clinical | warn | non-fasting | - | 190 | consensus | Non-fasting samples (lab flagging, EAS/EFLM consensus) | [Fasting is not routinely required for determination of a lipid profile: clinical and laboratory implications including flagging at desirable concentration cut-points-a joint consensus statement from the European Atherosclerosis Society and European Federation of Clinical Chemistry and Laboratory Medicine.](https://pubmed.ncbi.nlm.nih.gov/27122601/), Abstract | Laboratory flag for abnormal non-fasting total cholesterol. The consensus states the same cut-points apply to fasting samples. Source band >= 190; stored as the warn high bound 190. |
| optimal | optimal | all | - | 180 | - | - | uncited (legacy) | - |
| diagnostic | warn | age 20+ | - | 240 | guideline | Adults aged 20 years or older | [https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 2 | High total cholesterol (>= 240 mg/dL, ATP III). Source kind: diagnostic. Source band >= 240; stored as the warn high bound 240. |
| diagnostic | warn | age 0-20 | - | 200 | guideline | Children and adolescents (NHLBI expert panel, Table 9-1) | [https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf), Table 9-1 | Pediatric high total cholesterol (>= 200 mg/dL). Source kind: diagnostic. Source band >= 200; stored as the warn high bound 200. |
| population-ri | reference | pregnant, trimester 1 | 120.3 | 236.3 | reference-interval-study | Healthy singleton pregnant women, first trimester (population-based prospective study) | [Establishment of trimester-specific reference intervals of serum lipids and the associations with pregnancy complications and adverse perinatal outcomes: a population-based prospective study.](https://pubmed.ncbi.nlm.nih.gov/34498500/), Table 1 | 95% reference interval, first trimester. Numbers are in mmol/L as the source states them. Converted from the source's mmol/L (3.11-6.11) to mg/dL using factor 38.67. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | pregnant, trimester 3 | 176.3 | 360 | reference-interval-study | Healthy singleton pregnant women, third trimester (population-based prospective study) | [Establishment of trimester-specific reference intervals of serum lipids and the associations with pregnancy complications and adverse perinatal outcomes: a population-based prospective study.](https://pubmed.ncbi.nlm.nih.gov/34498500/), Table 1 | 95% reference interval, third trimester. Numbers are in mmol/L as the source states them. Converted from the source's mmol/L (4.56-9.31) to mg/dL using factor 38.67. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 18-45 | 135 | 243 | reference-interval-study | Apparently healthy Egyptian adults aged 18 to under 45 years, both sexes combined (IFCC C-RIDL protocol, serum) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | 95% reference interval, Egypt, under 45 years. Conventional mg/dL values from Table 3 (SI 3.48-6.28 mmol/L). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 45+ | 132 | 271 | reference-interval-study | Apparently healthy Egyptian adults aged 45 and over (study sample aged 18-65 plus some older participants), both sexes combined (IFCC C-RIDL protocol, serum) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | 95% reference interval, Egypt, 45 years and over. Conventional mg/dL values from Table 3 (SI 3.42-7.21 mmol/L). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 0-2 | 88.94 | 215.8 | reference-interval-study | Apparently healthy check-up children in Shenzhen, China, 28 days to under 2 years (sexes not partitioned in this table) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | 95% reference interval, pediatric. Numbers are in mmol/L as the source states them (SI only). Converted from the source's mmol/L (2.3-5.58) to mg/dL using factor 38.67. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 2-10 | 101.7 | 223.5 | reference-interval-study | Apparently healthy check-up children in Shenzhen, China, 2 to under 10 years (sexes not partitioned in this table) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | 95% reference interval, pediatric. Numbers are in mmol/L as the source states them (SI only). Converted from the source's mmol/L (2.63-5.78) to mg/dL using factor 38.67. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | age 10-18 | 96.29 | 199.5 | reference-interval-study | Apparently healthy check-up children in Shenzhen, China, 10 to under 18 years (sexes not partitioned in this table) | [Establishment of reference intervals for 41 serum biochemical analytes in healthy Chinese children.](https://pubmed.ncbi.nlm.nih.gov/42179679/), Table 1 | 95% reference interval, pediatric. Numbers are in mmol/L as the source states them (SI only). Converted from the source's mmol/L (2.49-5.16) to mg/dL using factor 38.67. Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
