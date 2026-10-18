# Lipoprotein(a) (`lpa`)

Lipoprotein(a), an LDL-like particle with an apolipoprotein(a) component. Concentration is largely genetically determined and is an independent cardiovascular risk factor. Measure at least once in adulthood.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | nmol/L |
| LOINC | [43583-4](https://loinc.org/43583-4/) |
| Specimen | - |
| Aliases | `lp(a)`, `lipoprotein a`, `lipoprotein(a)`, `lpa` |

## Unit conversions

Into nmol/L: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| mg/dL | 2.15 | 0 |

## Ranges

Bounds in nmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | - | 100 | guideline | Adults | [Lipoprotein(a) in Personalized Cardiovascular Prevention: Risk Stratification, Coronary Artery Calcium, and Emerging Lp(a)-Lowering Therapies.](https://pubmed.ncbi.nlm.nih.gov/42739646/), Section on the 2024 NLA focused update (secondary review of the guideline text) | NLA 2024 focused update: Lp(a) above 50 mg/dL or 100-125 nmol/L is a risk-enhancing factor (risk modifier). Lower bound of the band shown as 100 nmol/L. Source band >= 100; stored as the warn high bound 100. |
| clinical | reference | all | - | 75 | consensus | Adults (EAS 2022 consensus on Lp(a) in ASCVD and aortic stenosis) | [Lipoprotein(a) in atherosclerotic cardiovascular disease and aortic stenosis: a European Atherosclerosis Society consensus statement.](https://pubmed.ncbi.nlm.nih.gov/36036785/), Consensus statement, section on Lp(a) cut-offs for rule-out and rule-in | EAS 2022 'rule-out' cut-off: below 75 nmol/L (below 30 mg/dL). |
| optimal | optimal | all | - | 30 | - | - | uncited (legacy) | - |
| alt-1 | warn | all | - | 125 | guideline | Adults; 2026 US multi-society (ACC/AHA) dyslipidemia guideline as summarised in the cited review | [Lipoprotein(a) in Personalized Cardiovascular Prevention: Risk Stratification, Coronary Artery Calcium, and Emerging Lp(a)-Lowering Therapies.](https://pubmed.ncbi.nlm.nih.gov/42739646/), Introduction / risk stratification | 2026 ACC/AHA: Lp(a) >= 125 nmol/L (about 50 mg/dL) is a risk-enhancing factor. Quoted in nmol/L as the source states it; the review notes mg/dL and nmol/L are not interchangeable. Source band >= 125; stored as the warn high bound 125. Alternative to the default clinical warn range for the same population. |
| alt-1 | warn | all | - | 125 | consensus | Adults (EAS 2022 consensus on Lp(a) in ASCVD and aortic stenosis) | [Lipoprotein(a) in atherosclerotic cardiovascular disease and aortic stenosis: a European Atherosclerosis Society consensus statement.](https://pubmed.ncbi.nlm.nih.gov/36036785/), Consensus statement, section on Lp(a) cut-offs for rule-out and rule-in | EAS 2022 grey zone (30-50 mg/dL): relevant when weighing Lp(a)-attributable risk together with other risk factors. Source band 75-125; stored as the warn high bound 125. Alternative to the default clinical warn range for the same population. |
| alt-2 | warn | all | - | 250 | guideline | Adults; 2026 US multi-society (ACC/AHA) dyslipidemia guideline as summarised in the cited review | [Lipoprotein(a) in Personalized Cardiovascular Prevention: Risk Stratification, Coronary Artery Calcium, and Emerging Lp(a)-Lowering Therapies.](https://pubmed.ncbi.nlm.nih.gov/42739646/), Introduction / risk stratification | 2026 ACC/AHA: Lp(a) >= 250 nmol/L is a major risk enhancer. Quoted in nmol/L as the source states it. Source band >= 250; stored as the warn high bound 250. Alternative to the default clinical warn range for the same population. |
| diagnostic | warn | all | - | 125 | consensus | Adults (EAS 2022 consensus on Lp(a) in ASCVD and aortic stenosis) | [Lipoprotein(a) in atherosclerotic cardiovascular disease and aortic stenosis: a European Atherosclerosis Society consensus statement.](https://pubmed.ncbi.nlm.nih.gov/36036785/), Consensus statement, section on Lp(a) cut-offs for rule-out and rule-in | EAS 2022 'rule-in' category: above 125 nmol/L (above 50 mg/dL) indicates elevated Lp(a)-associated risk. Source kind: diagnostic. Source band >= 125; stored as the warn high bound 125. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
