# B-type Natriuretic Peptide (`bnp`)

B-type natriuretic peptide, a hormone released by the heart ventricles in response to stretch. It is used to help diagnose or rule out heart failure, with cut-offs that depend on setting, body weight and other conditions.

| | |
|---|---|
| Category | [cardiac](index.md#cardiac) |
| Canonical unit | pg/mL |
| LOINC | [30934-4](https://loinc.org/30934-4/) |
| Specimen | - |
| Aliases | `brain natriuretic peptide` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 100 | guideline | Adults presenting to the emergency department with acute dyspnoea (setting: acute-dyspnoea) | [Heart failure with reduced ejection fraction: a systematic review of clinical practice guidelines and recommendations.](https://pubmed.ncbi.nlm.nih.gov/41778370/), Full text, guideline diagnostic criteria table | Emergency-setting heart failure diagnostic criterion: BNP at or above 100 pg/mL (boundary inclusive). Source kind: decision. Source band >= 100; stored as the warn high bound 100. Alternative to the default hf-guideline-review-2026 warn range for the same population. |
| alt-1 | warn | severe-obesity | - | 54 | cohort | Patients with acute heart failure in the Breathing Not Properly multinational study, severely or morbidly obese | [How obesity affects the cut-points for B-type natriuretic peptide in the diagnosis of acute heart failure. Results from the Breathing Not Properly Multinational Study.](https://pubmed.ncbi.nlm.nih.gov/16644321/), Abstract, Results and Conclusion | Obesity-adjusted BNP cut-point for severely or morbidly obese patients, chosen to keep 90% sensitivity. The study recommends this lower cut-point in severe obesity to preserve sensitivity (boundary inclusive, >=54). Source kind: decision. Source band >= 54; stored as the warn high bound 54. Alternative to the default breathing-not-properly-2006 warn range for the same population. |
| alt-2 | warn | all | - | 500 | cohort | Patients presenting to the emergency department with acute dyspnoea (Breathing Not Properly multinational study) (setting: acute-dyspnoea) | [Impact of the history of congestive heart failure on the utility of B-type natriuretic peptide in the emergency diagnosis of heart failure: results from the Breathing Not Properly Multinational Study.](https://pubmed.ncbi.nlm.nih.gov/16431187/), Abstract, Results | High-range rule-in band in the emergency department: BNP above 500 pg/mL (strict >500). Reported positive predictive value of 87%. Values from 100 to 500 pg/mL are the ambiguous medium range. Source kind: decision. Source band >= 500; stored as the warn high bound 500. Alternative to the default breathing-not-properly-2006 warn range for the same population. |
| breathing-not-properly-2006 | warn | all | 100 | - | cohort | Patients presenting to the emergency department with acute dyspnoea (Breathing Not Properly multinational study) (setting: acute-dyspnoea) | [Impact of the history of congestive heart failure on the utility of B-type natriuretic peptide in the emergency diagnosis of heart failure: results from the Breathing Not Properly Multinational Study.](https://pubmed.ncbi.nlm.nih.gov/16431187/), Abstract, Results | Low-range rule-out band in the emergency department: BNP below 100 pg/mL (strict <100). Reported negative predictive value of 90% in the study. Source kind: decision. Source band <= 100; stored as the warn low bound 100. |
| breathing-not-properly-2006 | warn | overweight-obesity | - | 110 | cohort | Patients with acute heart failure in the Breathing Not Properly multinational study, overweight or obese (not severely obese) | [How obesity affects the cut-points for B-type natriuretic peptide in the diagnosis of acute heart failure. Results from the Breathing Not Properly Multinational Study.](https://pubmed.ncbi.nlm.nih.gov/16644321/), Abstract, Results | Obesity-adjusted BNP cut-point for overweight or obese patients, chosen to keep 90% sensitivity for heart failure. Boundary inclusive (>=110). Lean patients need a higher cut-point (170 pg/mL reported in the same study). Source kind: decision. Source band >= 110; stored as the warn high bound 110. |
| hf-guideline-review-2026 | warn | all | - | 35 | guideline | Adults assessed for suspected heart failure in the community (non-acute setting) (setting: ambulatory) | [Heart failure with reduced ejection fraction: a systematic review of clinical practice guidelines and recommendations.](https://pubmed.ncbi.nlm.nih.gov/41778370/), Full text, guideline diagnostic criteria table | Community (non-acute) heart failure diagnostic criterion: BNP at or above 35 pg/mL (boundary inclusive). Criteria are as tabulated in a systematic review of HF guideline recommendations. Source kind: decision. Source band >= 35; stored as the warn high bound 35. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
