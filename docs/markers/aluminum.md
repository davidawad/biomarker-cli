# Aluminum (serum) (`aluminum`)

Aluminium concentration in serum, used to detect exposure in dialysis patients (from dialysate, water or aluminium-containing phosphate binders). Higher values are linked to aluminium-associated bone disease, encephalopathy and anaemia.

| | |
|---|---|
| Category | [toxic-metal](index.md#toxic-metal) |
| Canonical unit | µg/L |
| LOINC | - |
| Specimen | - |
| Aliases | `aluminum-serum` |

## Ranges

Bounds in µg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | ckd | - | 20 | consensus | Maintenance hemodialysis patients (Taiwan); the authors state a threshold for aluminium toxicity risk | [Association Between Serum Aluminum Level and Uremic Pruritus in Hemodialysis Patients.](https://pubmed.ncbi.nlm.nih.gov/30467375/), Methods, Laboratory section | Serum aluminium of 2 µg/dL or more is the level above which the authors say toxicity is possible (they cite an earlier source for this threshold). Below it, toxicity is described as unlikely. |
| aluminum-osteomalacia-1986 | warn | ckd | - | 100 | cohort | Patients on maintenance hemodialysis or CAPD, 28 of whom had bone biopsy with aluminium staining | [Interpretation of serum aluminum values in dialysis patients.](https://pubmed.ncbi.nlm.nih.gov/3776914/), Abstract | Reliable indicator of aluminium-associated osteomalacia on bone biopsy in this 1986 series. Lower levels did not exclude low-turnover or mixed bone disease. ng/mL is numerically equal to µg/L. Source kind: toxic. Source band >= 100; stored as the warn high bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
