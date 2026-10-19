# Apolipoprotein B (`apob`)

Apolipoprotein B, one protein per atherogenic lipoprotein particle (LDL, VLDL, IDL, Lp(a)). Used as a secondary treatment target and as a risk marker alongside LDL cholesterol.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | mg/dL |
| LOINC | [1884-6](https://loinc.org/1884-6/) |
| Specimen | - |
| Aliases | `apo b`, `apolipoprotein b`, `apo-b` |
| Panels | [Cardiovascular risk (lipids and risk enhancers)](../panels/cardiovascular-risk.md) |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 90 | - | - | uncited (legacy) | - |
| clinical | warn | all | - | 130 | guideline | Adults (2018 AHA/ACC risk-enhancing factor) | [Apolipoprotein B in cardiovascular risk assessment: beyond traditional lipid markers.](https://pubmed.ncbi.nlm.nih.gov/42876934/), Introduction / risk-enhancing factors paragraph | Risk-enhancing factor in the 2018 AHA/ACC cholesterol guideline, particularly with triglycerides >= 200 mg/dL (as summarised in the cited review). Superseded by the 2026 ACC/AHA dyslipidemia guideline, whose apoB text could not be retrieved. Source band >= 130; stored as the warn high bound 130. |
| clinical | warn | all | - | 130 | guideline | Adults; apoB as a risk-enhancing biomarker (as restated in a 2026 review) | [Beyond LDL-C: Modern Lipid Management for ASCVD Prevention in Primary Care.](https://pubmed.ncbi.nlm.nih.gov/42629510/), Lipid management section | apoB >= 130 mg/dL as a risk-enhancing biomarker supporting therapy initiation or intensification, as restated in the cited 2026 review. The review does not name the guideline edition; the 2026 ACC/AHA text itself was not retrievable. Source band >= 130; stored as the warn high bound 130. |
| optimal | optimal | all | - | 80 | - | - | uncited (legacy) | - |
| alt-1 | optimal | all | - | 80 | guideline | Adults with high cardiovascular risk | [Apolipoprotein B in cardiovascular risk assessment: beyond traditional lipid markers.](https://pubmed.ncbi.nlm.nih.gov/42876934/), Introduction / lipid guideline paragraph | ESC/EAS 2019 secondary target: apoB strictly below 80 mg/dL (as summarised in the cited review). Applies only to: risk=high-cardiovascular-risk. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-1 | warn | all | - | 100 | consensus | Adults; laboratory flagging of lipid results | [Fasting is not routinely required for determination of a lipid profile: clinical and laboratory implications including flagging at desirable concentration cut-points-a joint consensus statement from the European Atherosclerosis Society and European Federation of Clinical Chemistry and Laboratory Medicine.](https://pubmed.ncbi.nlm.nih.gov/27122601/), Abstract (laboratory flagging recommendation) | EAS/EFLM 2016 laboratory flag: apoB at or above 1.0 g/L (100 mg/dL). apoB is not affected by fasting status per the same consensus. Converted from the source's g/L (1.0) to mg/dL using factor 100. Source band >= 100.0; stored as the warn high bound 100.0. Alternative to the default clinical warn range for the same population. |
| alt-2 | optimal | all | - | 100 | guideline | Adults with moderate cardiovascular risk | [Apolipoprotein B in cardiovascular risk assessment: beyond traditional lipid markers.](https://pubmed.ncbi.nlm.nih.gov/42876934/), Introduction / lipid guideline paragraph | ESC/EAS 2019 secondary target: apoB strictly below 100 mg/dL (as summarised in the cited review). Applies only to: risk=moderate-cardiovascular-risk. Source kind: target. Alternative to the default target optimal range for the same population. |
| alt-3 | optimal | all | - | 90 | reference-interval-study | FINRISK 2007 reference sample, Finnish adults (n = 2828) | [Apolipoprotein A-I, apolipoprotein B, and apolipoprotein B/apolipoprotein A-I ratio: reference intervals compared with values in different pathophysiological conditions from the FINRISK 2007 study.](https://pubmed.ncbi.nlm.nih.gov/21419755/), Abstract (Results) | Therapeutic apoB goal for low-risk individuals derived in FINRISK 2007 (0.9 g/L, coinciding with LDL-C 3.0 mmol/L); study-derived, not a guideline target. Converted from the source's g/L (0.9) to mg/dL using factor 100. Applies only to: risk=low-cardiovascular-risk. Source kind: target. Alternative to the default target optimal range for the same population. |
| target | optimal | all | - | 65 | guideline | Adults with very high cardiovascular risk | [Apolipoprotein B in cardiovascular risk assessment: beyond traditional lipid markers.](https://pubmed.ncbi.nlm.nih.gov/42876934/), Introduction / lipid guideline paragraph | ESC/EAS 2019 secondary target: apoB strictly below 65 mg/dL (as summarised in the cited review). Lower bound not stated. Applies only to: risk=very-high-cardiovascular-risk. Source kind: target. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
