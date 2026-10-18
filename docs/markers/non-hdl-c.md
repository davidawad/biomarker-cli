# Non-HDL Cholesterol (`non-hdl-c`)

Total cholesterol minus HDL-C, capturing all atherogenic apolipoprotein B particles. Used as a secondary target when triglycerides are elevated (ATP III: goal set 30 mg/dL above the LDL-C goal), with the NICE secondary-prevention target, pediatric NHLBI cut-points and the EAS/EFLM non-fasting and fasting flags.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | mg/dL |
| LOINC | [43396-1](https://loinc.org/43396-1/) |
| Specimen | - |
| Aliases | `non-hdl`, `non hdl cholesterol` |

## Unit conversions

Into mg/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| mmol/L | 38.67 | 0 |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 130 | - | - | uncited (legacy) | - |
| clinical | reference | age 0-20 | - | 120 | guideline | Children and adolescents (NHLBI expert panel, Table 9-1) | [https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf), Table 9-1 | Acceptable pediatric non-HDL-C: strictly below 120 mg/dL. |
| clinical | warn | age 0-20 | - | 144 | guideline | Children and adolescents (NHLBI expert panel, Table 9-1) | [https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf), Table 9-1 | Pediatric borderline-high non-HDL-C. Source band 120-144; stored as the warn high bound 144. |
| clinical | warn | non-fasting | - | 150 | consensus | Non-fasting samples (lab flagging, EAS/EFLM consensus) | [Fasting is not routinely required for determination of a lipid profile: clinical and laboratory implications including flagging at desirable concentration cut-points-a joint consensus statement from the European Atherosclerosis Society and European Federation of Clinical Chemistry and Laboratory Medicine.](https://pubmed.ncbi.nlm.nih.gov/27122601/), Abstract | Laboratory flag for abnormal non-fasting non-HDL-C. The consensus full text gives 3.9 mmol/L (155 mg/dL) in one place and 3.9 mmol/L (150 mg/dL) in the abstract; the abstract value is used here. Source band >= 150; stored as the warn high bound 150. |
| clinical | warn | fasting | - | 145 | consensus | Fasting samples (lab flagging, EAS/EFLM consensus) | [Fasting is not routinely required for determination of a lipid profile: clinical and laboratory implications including flagging at desirable concentration cut-points-a joint consensus statement from the European Atherosclerosis Society and European Federation of Clinical Chemistry and Laboratory Medicine.](https://pubmed.ncbi.nlm.nih.gov/27122601/), Results / flagging section (full text) | Laboratory flag for abnormal fasting non-HDL-C. Source band >= 145; stored as the warn high bound 145. |
| optimal | optimal | all | - | 100 | - | - | uncited (legacy) | - |
| alt-1 | optimal | all | - | 160 | guideline | Adults with two or more risk factors and 10-year CHD risk of 20% or less | [https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 9 | ATP III non-HDL-C goal (<160 mg/dL) for multiple (2+) risk factors. Applies only to: risk=two-or-more-risk-factors. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-2 | optimal | all | - | 190 | guideline | Adults with zero to one risk factor | [https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 9 | ATP III non-HDL-C goal (<190 mg/dL) for zero to one risk factor. Applies only to: risk=zero-to-one-risk-factor. Source kind: target. Alternative to the default target optimal range for the same population. |
| diagnostic | warn | age 0-20 | - | 145 | guideline | Children and adolescents (NHLBI expert panel, Table 9-1) | [https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/peds_guidelines_full.pdf), Table 9-1 | Pediatric high non-HDL-C (>= 145 mg/dL). Source kind: diagnostic. Source band >= 145; stored as the warn high bound 145. |
| target | optimal | all | - | 130 | guideline | Adults with coronary heart disease or CHD risk equivalents | [https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf](https://www.nhlbi.nih.gov/files/docs/guidelines/atp3xsum.pdf), Table 9 | ATP III non-HDL-C goal (<130 mg/dL), set 30 mg/dL above the LDL-C goal of <100. ATP III (2001) is superseded; the 2018 AHA/ACC text was not retrievable here. Applies only to: risk=chd-or-risk-equivalent. Source kind: target. |
| target | optimal | established-cvd | - | 100.5 | guideline | Adults with established cardiovascular disease (secondary prevention) | [https://www.nice.org.uk/guidance/ng238/chapter/recommendations](https://www.nice.org.uk/guidance/ng238/chapter/recommendations), Recommendation 1.7.1 | NICE NG238 secondary-prevention non-HDL-C target, stated in mmol/L; numbers kept as the source states them. Converted from the source's mmol/L (2.6) to mg/dL using factor 38.67. Source kind: target. |
| population-ri | reference | male, age 18-66 | 73 | 220 | reference-interval-study | Apparently healthy Egyptian men aged 18-65 years (IFCC C-RIDL protocol, serum) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | 95% reference interval, Egyptian men, 18-65 years. Conventional mg/dL values from Table 3 (SI 1.88-5.69 mmol/L). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | female, age 18-45 | 71 | 187 | reference-interval-study | Apparently healthy Egyptian women aged 18 to under 45 years (IFCC C-RIDL protocol, serum) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | 95% reference interval, Egyptian women under 45 years. Conventional mg/dL values from Table 3 (SI 1.83-4.84 mmol/L). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |
| population-ri | reference | female, age 45+ | 89 | 226 | reference-interval-study | Apparently healthy Egyptian women aged 45 and over (IFCC C-RIDL protocol, serum) | [Establishment of reference intervals of clinical chemistry analytes for the adult population in Egypt.](https://pubmed.ncbi.nlm.nih.gov/33740794/), Table 3 | 95% reference interval, Egyptian women, 45 years and over. Conventional mg/dL values from Table 3 (SI 2.31-5.85 mmol/L). Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
