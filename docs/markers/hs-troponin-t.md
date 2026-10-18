# High-Sensitivity Cardiac Troponin T (`hs-troponin-t`)

High-sensitivity cardiac troponin T, a protein released from injured heart muscle. Measured with a high-sensitivity assay it is used to rule acute myocardial infarction in or out, typically with a 0/1-hour algorithm, and to detect myocardial injury.

| | |
|---|---|
| Category | [cardiac](index.md#cardiac) |
| Canonical unit | ng/L |
| LOINC | [67151-1](https://loinc.org/67151-1/) |
| Specimen | - |
| Aliases | `hs-ctnt`, `hs-tnt`, `troponin t high sensitivity` |

## Ranges

Bounds in ng/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 14 | reference-interval-study | Healthy reference population used for the Roche cobas 8000 hs-cTnT assay (overall, not sex-specific) | [Rapid risk stratification of acute coronary syndrome: adoption of an adapted European Society of Cardiology 0/1-hour troponin algorithm in a real-world setting.](https://pubmed.ncbi.nlm.nih.gov/36032815/), Methods, assay description | 99th percentile upper reference limit (URL) for the Roche hs-cTnT assay as stated by the study. Values above the URL indicate myocardial injury; this single URL is not sex-specific. |
| alt-1 | warn | all | - | 52 | cohort | Adults presenting to the emergency department or ambulatory emergency care with suspected acute coronary syndrome (setting: suspected-acute-mi; method: roche-cobas-8000) | [Rapid risk stratification of acute coronary syndrome: adoption of an adapted European Society of Cardiology 0/1-hour troponin algorithm in a real-world setting.](https://pubmed.ncbi.nlm.nih.gov/36032815/), Results, risk categories (Figure 1) | Adapted ESC 0/1-hour hs-cTnT algorithm, high-risk ('ACS likely') band: initial hs-cTnT of 52 ng/L or more, inclusive (>=52). Concentrations between 5 and 51 ng/L require a 1-hour repeat (rule-in if delta >=5 ng/L). Source kind: decision. Source band >= 52; stored as the warn high bound 52. Alternative to the default ehjopen-2022-acs-0-1h warn range for the same population. |
| alt-2 | warn | all | - | 5 | cohort | Adults presenting with suspected myocardial infarction in a single-centre cohort, with the ESC 0/1-hour algorithm logic applied to the predecessor Elecsys hs-cTnT Gen 5 assay (setting: suspected-acute-mi; method: elecsys-troponin-t-hs-gen5) | [Diagnostic validation of a novel high-sensitivity cardiac troponin T assay.](https://pubmed.ncbi.nlm.nih.gov/42008514/), Methods, Statistical analysis | ESC 0/1-hour algorithm, 1-hour rule-in for Elecsys hs-cTnT Gen 5: absolute change of 5 ng/L or more (inclusive, >=5) at 1 hour. The paper gives this as the logic it applied to the Gen 5 assay. Source kind: decision. Source band >= 5; stored as the warn high bound 5. Alternative to the default ehjacc-2026-tnt-gen6-validation warn range for the same population. |
| ehjacc-2026-tnt-gen6-validation | warn | all | 3 | - | cohort | Adults presenting with suspected myocardial infarction in a single-centre cohort, with the ESC 0/1-hour algorithm logic applied to the predecessor Elecsys hs-cTnT Gen 5 assay (setting: suspected-acute-mi; method: elecsys-troponin-t-hs-gen5) | [Diagnostic validation of a novel high-sensitivity cardiac troponin T assay.](https://pubmed.ncbi.nlm.nih.gov/42008514/), Methods, Statistical analysis | ESC 0/1-hour algorithm, 1-hour rule-out for Elecsys hs-cTnT Gen 5: absolute change below 3 ng/L (strict <3), with a baseline below 12 ng/L. The paper states this as the logic it applied to the Gen 5 assay for comparison; it is not the manufacturer's Gen 5 label. Source kind: decision. Source band <= 3; stored as the warn low bound 3. |
| ehjopen-2022-acs-0-1h | warn | all | 5 | - | cohort | Adults presenting to the emergency department or ambulatory emergency care with suspected acute coronary syndrome (setting: suspected-acute-mi; method: roche-cobas-8000) | [Rapid risk stratification of acute coronary syndrome: adoption of an adapted European Society of Cardiology 0/1-hour troponin algorithm in a real-world setting.](https://pubmed.ncbi.nlm.nih.gov/36032815/), Results, risk categories | Adapted ESC 0/1-hour hs-cTnT algorithm, low-risk band: initial hs-cTnT below 5 ng/L (strict '<5') when chest pain onset was at least 3 h before presentation. Rule-out boundary is strict (<5 ng/L). Source kind: decision. Source band <= 5; stored as the warn low bound 5. |
| gen6-ref-tsix-2026 | reference | female | - | 18 | reference-interval-study | Healthy participants in the global REF-TSIX reference study, Elecsys hs-cTnT Gen 6 assay (method: elecsys-troponin-t-hs-gen6) | [Establishing Reference Values in Healthy Participants for the Cardiac Troponin T High-Sensitivity Gen 6 Assay: REF-TSIX Global Reference Study.](https://pubmed.ncbi.nlm.nih.gov/41564003/), Abstract, Results | Sex-specific 99th percentile upper reference limit for females on the Gen 6 assay (95% CI 16-23 ng/L). Applies only to the Gen 6 assay, not to the Roche Gen 5 URL of 14 ng/L. |
| gen6-ref-tsix-2026 | reference | male | - | 32 | reference-interval-study | Healthy participants in the global REF-TSIX reference study, Elecsys hs-cTnT Gen 6 assay (method: elecsys-troponin-t-hs-gen6) | [Establishing Reference Values in Healthy Participants for the Cardiac Troponin T High-Sensitivity Gen 6 Assay: REF-TSIX Global Reference Study.](https://pubmed.ncbi.nlm.nih.gov/41564003/), Abstract, Results | Sex-specific 99th percentile upper reference limit for males on the Gen 6 assay (95% CI 28-35 ng/L). Applies only to the Gen 6 assay. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
