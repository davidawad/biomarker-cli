# AHA PREVENT 10-year CVD Risk (`prevent-10y-risk`)

Estimated 10-year risk of cardiovascular disease (ASCVD and heart failure) from the American Heart Association PREVENT equations, which use age, sex, lipids, blood pressure, diabetes, smoking, BMI, eGFR and treatments, without race. It is an alternative to the Pooled Cohort Equations for primary-prevention risk estimation.

| | |
|---|---|
| Category | [composite](index.md#composite) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | `prevent`, `prevent cvd risk`, `aha prevent 10-year risk` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| acc-aha-2026-dyslipidemia | warn | all | 3 | - | guideline | US adults aged 30-79 years without known cardiovascular disease, PREVENT-based 10-year risk | [Risk-Treatment Discordance and Fatal Cardiovascular Disease: A Nationwide Analysis Using PREVENT Thresholds.](https://pubmed.ncbi.nlm.nih.gov/42828255/), Methods, risk category definitions (full text) | Low risk, <3% 10-year PREVENT risk. Categories as stated in the cited paper, which attributes them to the 2026 multisociety dyslipidemia guideline; the quote does not name the risk outcome, the paper applies them to PREVENT-ASCVD. Upper limit is exclusive. Source kind: decision. Source band <= 3; stored as the warn low bound 3. |
| alt-1 | warn | all | - | 10 | guideline | US adults aged 30-79 years without known cardiovascular disease, PREVENT-based 10-year risk | [Risk-Treatment Discordance and Fatal Cardiovascular Disease: A Nationwide Analysis Using PREVENT Thresholds.](https://pubmed.ncbi.nlm.nih.gov/42828255/), Methods, risk category definitions (full text) | High risk, >=10% 10-year PREVENT risk. Lower bound only (open above). Source kind: decision. Source band >= 10; stored as the warn high bound 10. Alternative to the default acc-aha-2026-dyslipidemia warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
