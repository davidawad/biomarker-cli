# High-Sensitivity Cardiac Troponin I (`hs-troponin-i`)

High-sensitivity cardiac troponin I, a protein released from injured heart muscle. Measured with a high-sensitivity assay it is used to rule acute myocardial infarction in or out, with sex-specific 99th percentile limits and 0/1-hour algorithms, and to detect myocardial injury.

| | |
|---|---|
| Category | [cardiac](index.md#cardiac) |
| Canonical unit | ng/L |
| LOINC | [89579-7](https://loinc.org/89579-7/) |
| Specimen | - |
| Aliases | `hs-ctni`, `hs-tni`, `troponin i high sensitivity` |

## Ranges

Bounds in ng/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | female | - | 16 | reference-interval-study | Healthy reference population used to derive sex-specific 99th percentile limits for the Abbott Architect hs-cTnI (CLIA) assay (method: abbott-architect-stat) | [Troponin in early presenters to rule out myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/37350492/), Abstract, Methods | Sex-specific 99th percentile upper reference limit for women on the Abbott Architect hs-cTnI assay. Values above the limit indicate myocardial injury. |
| alt-1 | warn | all | 4 | - | cohort | Adults with suspected myocardial infarction in a multicentre prospective Chinese cohort applying the ESC 0/1-hour algorithm (Architect STAT hs-cTnI) (setting: suspected-acute-mi; method: abbott-architect-stat) | [A multicenter prospective study on the ESC algorithm for the early diagnosis of non-ST-elevation myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/39262795/), Introduction, ESC algorithm thresholds | 2020 ESC 0/1-hour algorithm, direct rule-out at presentation: hs-cTnI below 4 ng/L (strict <4) when chest pain onset was more than 3 hours before presentation. Source kind: decision. Source band <= 4; stored as the warn low bound 4. Alternative to the default iscience-2024-esc-algorithm warn range for the same population. |
| alt-2 | warn | all | 2 | - | cohort | Adults with suspected myocardial infarction in a multicentre prospective Chinese cohort applying the ESC 0/1-hour algorithm (Architect STAT hs-cTnI) (setting: suspected-acute-mi; method: abbott-architect-stat) | [A multicenter prospective study on the ESC algorithm for the early diagnosis of non-ST-elevation myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/39262795/), Methods, ESC algorithm thresholds | ESC 0/1-hour algorithm, 1-hour rule-out: absolute change from baseline below 2 ng/L (strict <2) for hs-cTnI, as described for the algorithm in the study methods. Source kind: decision. Source band <= 2; stored as the warn low bound 2. Alternative to the default iscience-2024-esc-algorithm warn range for the same population. |
| alt-3 | warn | all | - | 6 | cohort | Adults with suspected myocardial infarction in a multicentre prospective Chinese cohort applying the ESC 0/1-hour algorithm (Architect STAT hs-cTnI) (setting: suspected-acute-mi; method: abbott-architect-stat) | [A multicenter prospective study on the ESC algorithm for the early diagnosis of non-ST-elevation myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/39262795/), Results | ESC 0/1-hour algorithm, 1-hour rule-in: hs-cTnI change of 6 ng/L or more (inclusive, >=6) from baseline to 1 hour. Source kind: decision. Source band >= 6; stored as the warn high bound 6. Alternative to the default iscience-2024-esc-algorithm warn range for the same population. |
| eurheartj-2023-troponin-early | reference | male | - | 34 | reference-interval-study | Healthy reference population used to derive sex-specific 99th percentile limits for the Abbott Architect hs-cTnI (CLIA) assay (method: abbott-architect-stat) | [Troponin in early presenters to rule out myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/37350492/), Abstract, Methods | Sex-specific 99th percentile upper reference limit for men on the Abbott Architect hs-cTnI assay. |
| eurheartj-2023-troponin-early | warn | all | 5 | - | cohort | Patients presenting with suspected myocardial infarction, including those presenting within 3 hours of symptom onset (setting: suspected-acute-mi; method: abbott-architect-stat) | [Troponin in early presenters to rule out myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/37350492/), Abstract, Results | Optimised single-sample rule-out threshold for hs-cTnI at presentation. Boundary is strict (<5 ng/L). The same study reports a 2 ng/L limit of detection threshold with higher sensitivity. Source kind: decision. Source band <= 5; stored as the warn low bound 5. |
| high-steacs-2017 | warn | all | 5 | - | cohort | Patients with suspected acute coronary syndrome in the High-STEACS-derived pathway (setting: suspected-acute-mi; method: abbott-architect-stat) | [Single High-Sensitivity Cardiac Troponin I to Rule Out Acute Myocardial Infarction.](https://pubmed.ncbi.nlm.nih.gov/28344141/), Abstract, Results | High-STEACS hs-cTnI rule-out threshold at presentation (strict <5 ng/L). Used alone or with a normal ECG. Source kind: decision. Source band <= 5; stored as the warn low bound 5. |
| iscience-2024-esc-algorithm | warn | all | - | 64 | cohort | Adults with suspected myocardial infarction in a multicentre prospective ESC-algorithm cohort (Architect STAT hs-cTnI) (setting: suspected-acute-mi; method: abbott-architect-stat) | [A multicenter prospective study on the ESC algorithm for the early diagnosis of non-ST-elevation myocardial infarction.](https://pubmed.ncbi.nlm.nih.gov/39262795/), Results | ESC 0/1-hour algorithm: direct rule-in at presentation for hs-cTnI of 64 ng/L or more (inclusive, >=64). Concentrations below this are triaged by the 1-hour change (rule-in for a 0/1-hour change of 6 ng/L or more). Source kind: decision. Source band >= 64; stored as the warn high bound 64. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
