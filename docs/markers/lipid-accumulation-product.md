# Lipid Accumulation Product (`lipid-accumulation-product`)

Computed index of visceral adiposity from waist circumference (cm) and fasting triglycerides (mmol/L): LAP = (WC - 58) x TG for women; LAP = (WC - 65) x TG for men (formula as stated in PMID 34045594, Methods). Tracked as a simple marker of visceral obesity and cardiometabolic risk. Expected result: below the cut-off for the person's sex and age band.

| | |
|---|---|
| Category | [body-composition](index.md#body-composition) |
| Canonical unit | cm mmol/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: Computed score with no established reference interval; the source states that reference and cut-off values for LAP have not been defined. The ranges below are ROC-derived hypertension cut-offs from one cross-sectional study.

Bounds in cm mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| diagnostic | warn | female, hypertension | - | 27 | cohort | Adults aged 20-64 years, hypertensive cases and normotensive controls, cross-sectional case-control study (1960 subjects, 1052 women) | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Women | ROC cut-off point for predicting hypertension in the whole female group (20-64 years), with sensitivity 70.6% and specificity 75.8% per the source. Source kind: diagnostic. Source band >= 27.0; stored as the warn high bound 27.0. |
| diagnostic | warn | female, age 20-35, hypertension | - | 13.8 | cohort | Women aged 20-34 years, hypertensive cases and normotensive controls, cross-sectional case-control study | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Women, 20-34 | Age-band ROC cut-off point for predicting hypertension in women aged 20-34, with sensitivity 75.0% and specificity 73.5% per the source. Source kind: diagnostic. Source band >= 13.8; stored as the warn high bound 13.8. |
| diagnostic | warn | female, age 35-50, hypertension | - | 27 | cohort | Women aged 35-49 years, hypertensive cases and normotensive controls, cross-sectional case-control study | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Women, 35-49 | Age-band ROC cut-off point for predicting hypertension in women aged 35-49, with sensitivity 69.2% and specificity 72.3% per the source. Source kind: diagnostic. Source band >= 27.0; stored as the warn high bound 27.0. |
| diagnostic | warn | female, age 50-65, hypertension | - | 44.6 | cohort | Women aged 50-64 years, hypertensive cases and normotensive controls, cross-sectional case-control study | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Women, 50-64 | Age-band ROC cut-off point for predicting hypertension in women aged 50-64, with sensitivity 46.8% and specificity 72.5% per the source. Source kind: diagnostic. Source band >= 44.6; stored as the warn high bound 44.6. |
| diagnostic | warn | male, hypertension | - | 27.6 | cohort | Adults aged 20-64 years, hypertensive cases and normotensive controls, cross-sectional case-control study (1960 subjects, 908 men) | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Men | ROC cut-off point for predicting hypertension in the whole male group (20-64 years), with sensitivity 63.6% and specificity 68.3% per the source. AUROC 0.694 (95% CI 0.650-0.738). Source kind: diagnostic. Source band >= 27.6; stored as the warn high bound 27.6. |
| diagnostic | warn | male, age 20-35, hypertension | - | 24 | cohort | Men aged 20-34 years, hypertensive cases and normotensive controls, cross-sectional case-control study | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Men, 20-34 | Age-band ROC cut-off point for predicting hypertension in men aged 20-34, with sensitivity 68.4% and specificity 74.7% per the source. Source kind: diagnostic. Source band >= 24.0; stored as the warn high bound 24.0. |
| diagnostic | warn | male, age 35-50, hypertension | - | 36.4 | cohort | Men aged 35-49 years, hypertensive cases and normotensive controls, cross-sectional case-control study | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Men, 35-49 | Age-band ROC cut-off point for predicting hypertension in men aged 35-49, with sensitivity 61.2% and specificity 78.4% per the source. Source kind: diagnostic. Source band >= 36.4; stored as the warn high bound 36.4. |
| diagnostic | warn | male, age 50-65, hypertension | - | 26.7 | cohort | Men aged 50-64 years, hypertensive cases and normotensive controls, cross-sectional case-control study | [Age-adjusted cut-off values of lipid accumulation product (LAP) for predicting hypertension.](https://pubmed.ncbi.nlm.nih.gov/34045594/), Table 3, Men, 50-64; Results text | Age-band ROC cut-off point for predicting hypertension in men aged 50-64, with sensitivity 64.4% and specificity 53.3% per the source. The source reports AUROC 0.551 (95% CI 0.474-0.629) for this band and states the men's 50+ confidence intervals include 0.5 (no predictive performance). Source kind: diagnostic. Source band >= 26.7; stored as the warn high bound 26.7. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
