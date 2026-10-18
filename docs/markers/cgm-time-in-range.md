# CGM time in range (70-180 mg/dL) (`cgm-time-in-range`)

Percentage of CGM glucose readings (or hours per day) within the target glucose range of 70-180 mg/dL (3.9-10.0 mmol/L). The International Consensus on Time in Range sets targets for people with diabetes; the pregnancy range is 63-140 mg/dL.

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | optimal | diabetes | 50 | - | consensus | Older and/or high-risk adults with type 1 or type 2 diabetes | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Table 3 | Time in range 70-180 mg/dL (3.9-10 mmol/L); lowered target for older and/or high-risk individuals, more than 50 percent of readings (more than 12 h per day). Applies only to: risk=older-or-high-risk. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-1 | optimal | pregnant, diabetes | - | 4 | consensus | Pregnant women with type 1 or type 2 diabetes, CGM data | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Discussion, pregnancy TBR paragraph | Pregnancy time below range (glucose below 63 mg/dL) target: below 4%. Boundary is strictly less than 4%. Alternative to the default battelino-2019-tir optimal range for the same population. |
| alt-2 | optimal | diabetes | - | 25 | consensus | Adults with type 1 or type 2 diabetes (general target), CGM data over 14 days | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Table 3, Guidance for adults with type 1 or type 2 diabetes | Time above range (TAR, glucose above 180 mg/dL) target: below 25% (about 6 h per day). Boundary is strictly less than 25%. Alternative to the default battelino-2019-tir optimal range for the same population. |
| alt-3 | optimal | diabetes | - | 4 | consensus | Adults with type 1 or type 2 diabetes (general target), CGM data over 14 days | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Discussion, Table 3 (TBR targets) | Time below range (TBR, glucose below 70 mg/dL) target: below 4% (under 1 h per day). Boundary is strictly less than 4%. Individualise for older or high-risk adults, where a stricter reduction in time below 70 mg/dL is emphasised. Alternative to the default battelino-2019-tir optimal range for the same population. |
| alt-4 | optimal | diabetes | - | 1 | consensus | Adults with type 1 or type 2 diabetes (general target), CGM data over 14 days | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Discussion, Table 3 (TBR targets) | Time below 54 mg/dL target: below 1% (under 15 min per day). Boundary is strictly less than 1%. Alternative to the default battelino-2019-tir optimal range for the same population. |
| battelino-2019-tir | optimal | diabetes | 50 | - | consensus | Older or high-risk adults with type 1 or type 2 diabetes | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Discussion, paragraph on TIR target for older/high-risk adults | Individualised, more lenient TIR target for older or high-risk adults: above 50% (boundary strictly greater). The consensus lowered this target from >70% to emphasise hypoglycaemia avoidance. |
| battelino-2019-tir | optimal | pregnant, diabetes | 70 | - | consensus | Pregnant women with type 1 or type 2 diabetes, CGM data | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Discussion, pregnancy results paragraph | Pregnancy TIR target: 70% within 63-140 mg/dL (3.5-7.8 mmol/L). The pregnancy band is narrower than the general 70-180 mg/dL band. |
| target | optimal | diabetes | 70 | - | consensus | Adults and children with type 1 or type 2 diabetes (non-pregnant, not older/high-risk) | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Table 3 | Time in range 70-180 mg/dL (3.9-10.0 mmol/L); target is more than 70 percent of readings (more than 16 h 48 min per day). Pediatric exception (age under 25 with A1C goal 7.5 percent) is about 60 percent and is not included. Source kind: target. |
| target | optimal | pregnant, type-1-diabetes | 70 | - | consensus | Pregnant women with type 1 diabetes | [Clinical Targets for Continuous Glucose Monitoring Data Interpretation: Recommendations From the International Consensus on Time in Range.](https://pubmed.ncbi.nlm.nih.gov/31177185/), Table 4 | Pregnancy time-in-range glucose band is 63-140 mg/dL (3.5-7.8 mmol/L), not 70-180; target is more than 70 percent of readings. Type 2 diabetes and GDM in pregnancy have no percentage targets in this consensus. Source kind: target. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
