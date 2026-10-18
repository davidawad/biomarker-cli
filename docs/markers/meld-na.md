# MELD-Na Score (`meld-na`)

Model for End-Stage Liver Disease score with serum sodium added (MELD-Na), calculated from creatinine, bilirubin, INR and sodium. Used for liver transplant allocation and to estimate short-term mortality in cirrhosis. There is no normal range; a higher score means more severe liver disease and higher mortality risk.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | points |
| LOINC | [100348-2](https://loinc.org/100348-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in points. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| diagnostic | warn | cirrhosis | - | 28 | cohort | Adults with cirrhosis listed for liver transplantation on the UNOS waiting list (2002-2019), single retrospective cohort | [MELD-Na Accurately Predicts 6-Month Mortality in Patients With Decompensated Cirrhosis: Potential Trigger for Hospice Referral.](https://pubmed.ncbi.nlm.nih.gov/34802021/), Discussion (limitations paragraph, PMC full text) | Score of 28 or more used as the trigger for hospice referral discussion (6-month mortality). Single observational study; a decision trigger, not a diagnostic category. Source kind: diagnostic. Source band >= 28; stored as the warn high bound 28. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
