# Atherogenic Index of Plasma (`atherogenic-index-plasma`)

Atherogenic Index of Plasma (AIP) is the base-10 logarithm of the triglyceride to HDL cholesterol ratio, with both lipids in mmol/L. It is used as a marker of atherogenic dyslipidaemia and cardiovascular risk.

| | |
|---|---|
| Category | [derived](index.md#derived) |
| Canonical unit | index |
| LOINC | - |
| Specimen | - |
| Aliases | `aip`, `log(tg/hdl-c)`, `log tg/hdl ratio`, `atherogenic-index-of-plasma` |

## Ranges

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.11 | 0.21 | consensus | Adults in cardiovascular and metabolic risk studies; zones attributed to Dobiasova and Frohlich | [Atherogenic Index of Plasma Relationship with Cardiovascular Risk Factors and Frailty and Value as Determinant of Mortality in Elderly Patients with Severe Aortic Stenosis.](https://pubmed.ncbi.nlm.nih.gov/42187998/), Full text, Results (AIP categories) | Intermediate atherogenic risk: AIP 0.11 to 0.21 inclusive of the lower boundary (described as moderate risk in the source). Risk zone, not a diagnostic cut-off. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | all | - | 0.21 | consensus | Adults in cardiovascular and metabolic risk studies; zones attributed to Dobiasova and Frohlich | [Atherogenic Index of Plasma Relationship with Cardiovascular Risk Factors and Frailty and Value as Determinant of Mortality in Elderly Patients with Severe Aortic Stenosis.](https://pubmed.ncbi.nlm.nih.gov/42187998/), Full text, Results (AIP categories) | High atherogenic risk: AIP above 0.21 (boundary > 0.21). Risk zone, not a diagnostic cut-off. A 2026 Cureus paper quoting the same authors uses a different upper split (0.21-0.24 high, above 0.24 very high), which is not used here. Source kind: decision. Source band >= 0.21; stored as the warn high bound 0.21. Alternative to the default dobiasova-frohlich-aip-bands warn range for the same population. |
| dobiasova-frohlich-aip-bands | warn | all | 0.11 | - | consensus | Adults in cardiovascular and metabolic risk studies; zones attributed to Dobiasova and Frohlich | [Atherogenic Index of Plasma Relationship with Cardiovascular Risk Factors and Frailty and Value as Determinant of Mortality in Elderly Patients with Severe Aortic Stenosis.](https://pubmed.ncbi.nlm.nih.gov/42187998/), Full text, Results (AIP categories) | Low atherogenic risk: AIP below 0.11 (boundary < 0.11). Band is reported as a risk zone, not a diagnostic cut-off. Original Dobiasova and Frohlich paper not retrievable; band taken from a 2026 Metabolites paper. Source kind: decision. Source band <= 0.11; stored as the warn low bound 0.11. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
