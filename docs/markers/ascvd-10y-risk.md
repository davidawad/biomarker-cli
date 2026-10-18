# 10-year ASCVD Risk (Pooled Cohort Equations) (`ascvd-10y-risk`)

Estimated 10-year risk of a first atherosclerotic cardiovascular event (heart attack, coronary death, or stroke), calculated from the Pooled Cohort Equations using age, sex, race, cholesterol, blood pressure, diabetes, smoking and treatment. It guides primary-prevention decisions such as statin therapy.

| | |
|---|---|
| Category | [composite](index.md#composite) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | `pce 10-year risk`, `pooled cohort equations risk`, `10-year ascvd risk` |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 5 | 7.5 | guideline | US adults 40 to 75 years of age evaluated for primary ASCVD prevention, without diabetes | [2018 AHA/ACC/AACVPR/AAPA/ABC/ACPM/ADA/AGS/APhA/ASPC/NLA/PCNA Guideline on the Management of Blood Cholesterol: A Report of the American College of Cardiology/American Heart Association Task Force on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/30586774/), Full text, Recommendations | Borderline risk, 5% to <7.5% 10-year ASCVD risk. Risk-enhancing factors may favour statin therapy in this band. Boundary semantics: >=5 and <7.5. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| acc-aha-2019-primary-prevention | warn | all | - | 20 | guideline | US adults 40 to 75 years of age in primary prevention | [2019 ACC/AHA Guideline on the Primary Prevention of Cardiovascular Disease: Executive Summary: A Report of the American College of Cardiology/American Heart Association Task Force on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/30894319/), Full text, Recommendations | High risk, >=20% 10-year ASCVD risk. The 2018 guideline recommends a >=50% LDL-C reduction at this level. Lower bound only (open above). Source kind: decision. Source band >= 20; stored as the warn high bound 20. |
| acc-aha-2018-cholesterol-7-5-19-9 | reference | age 40-76 | 7.5 | 19.9 | guideline | US adults 40 to 75 years of age without diabetes mellitus, primary prevention | [2018 AHA/ACC/AACVPR/AAPA/ABC/ACPM/ADA/AGS/APhA/ASPC/NLA/PCNA Guideline on the Management of Blood Cholesterol: A Report of the American College of Cardiology/American Heart Association Task Force on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/30586774/), Full text, Recommendations | Intermediate risk, 7.5% to <20% 10-year ASCVD risk. Risk-enhancing factors favour statin initiation; if uncertain, CAC may be measured. Boundary semantics: >=7.5 and <20 (19.9 stated as the upper limit of the band). Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
