# AHA PREVENT 10-year CVD Risk (`prevent-risk`)

Estimated 10-year risk of cardiovascular disease (ASCVD and heart failure) from the American Heart Association PREVENT equations, which use age, sex, lipids, blood pressure, diabetes, smoking, BMI, eGFR and treatments, without race. It is an alternative to the Pooled Cohort Equations for primary-prevention risk estimation.

| | |
|---|---|
| Category | [composite](index.md#composite) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | `prevent`, `prevent cvd risk`, `aha prevent 10-year risk`, `prevent-10y-risk` |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 3 | 5 | guideline | US adults aged 30-79 years without known cardiovascular disease, PREVENT-based 10-year risk | [Risk-Treatment Discordance and Fatal Cardiovascular Disease: A Nationwide Analysis Using PREVENT Thresholds.](https://pubmed.ncbi.nlm.nih.gov/42828255/), Methods, risk category definitions (full text) | Borderline risk, 3 to <5% 10-year PREVENT risk. Boundary semantics: >=3 and <5. Categories as reported in the cited paper (see the low-risk range note). Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| acc-aha-2026-dyslipidemia | warn | all | 3 | - | guideline | US adults aged 30-79 years without known cardiovascular disease, PREVENT-based 10-year risk | [Risk-Treatment Discordance and Fatal Cardiovascular Disease: A Nationwide Analysis Using PREVENT Thresholds.](https://pubmed.ncbi.nlm.nih.gov/42828255/), Methods, risk category definitions (full text) | Low risk, <3% 10-year PREVENT risk. Categories as stated in the cited paper, which attributes them to the 2026 multisociety dyslipidemia guideline; the quote does not name the risk outcome, the paper applies them to PREVENT-ASCVD. Upper limit is exclusive. Source kind: decision. Source band <= 3; stored as the warn low bound 3. |
| alt-1 | warn | all | - | 10 | guideline | US adults aged 30-79 years without known cardiovascular disease, PREVENT-based 10-year risk | [Risk-Treatment Discordance and Fatal Cardiovascular Disease: A Nationwide Analysis Using PREVENT Thresholds.](https://pubmed.ncbi.nlm.nih.gov/42828255/), Methods, risk category definitions (full text) | High risk, >=10% 10-year PREVENT risk. Lower bound only (open above). Source kind: decision. Source band >= 10; stored as the warn high bound 10. Alternative to the default acc-aha-2026-dyslipidemia warn range for the same population. |
| acc-aha-2026-dyslipidemia-5-10 | reference | all | 5 | 10 | guideline | US adults aged 30-79 years without known cardiovascular disease, PREVENT-based 10-year risk | [Risk-Treatment Discordance and Fatal Cardiovascular Disease: A Nationwide Analysis Using PREVENT Thresholds.](https://pubmed.ncbi.nlm.nih.gov/42828255/), Methods, risk category definitions (full text) | Intermediate risk, 5 to <10% 10-year PREVENT risk. The same paper reports this as the 'intermediate-or-higher' threshold for lipid-lowering therapy consideration (>=5%). Boundary semantics: >=5 and <10. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
