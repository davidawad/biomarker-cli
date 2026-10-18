# Alpha-1 Antitrypsin (`alpha-1-antitrypsin`)

Serum alpha-1 antitrypsin (AAT), the main circulating inhibitor of neutrophil elastase. Measured first when screening for alpha-1 antitrypsin deficiency (AATD). AAT is an acute-phase protein, so levels can rise during inflammation. Expected result: above 1.1 g/L (levels below 1.1 g/L prompt confirmatory testing; levels at or below 0.57 g/L indicate severe deficiency).

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | g/L |
| LOINC | [1825-9](https://loinc.org/1825-9/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in g/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | 1.1 | - | consensus | Adults undergoing initial screening for alpha-1 antitrypsin deficiency | [Alpha-1 Antitrypsin Deficiency-Associated Chronic Obstructive Pulmonary Disease.](https://pubmed.ncbi.nlm.nih.gov/42075511/), Diagnosis section, paragraph after the CRP/acute-phase discussion | Confirmatory-testing threshold, as stated in the same 2026 narrative review, which also says these thresholds are not absolute biological cut-offs. Source band <= 1.1; stored as the warn low bound 1.1. |
| clinical | reference | all | 0.57 | 1.1 | consensus | Adults undergoing initial screening for alpha-1 antitrypsin deficiency | [Alpha-1 Antitrypsin Deficiency-Associated Chronic Obstructive Pulmonary Disease.](https://pubmed.ncbi.nlm.nih.gov/42075511/), Diagnosis section, initial screening paragraph | Milder AAT deficiency band, as stated in the same 2026 narrative review: strictly above 0.57 g/L and strictly below 1.1 g/L. Source kind: diagnostic. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| diagnostic | warn | all | 0.57 | - | consensus | Adults undergoing initial screening for alpha-1 antitrypsin deficiency | [Alpha-1 Antitrypsin Deficiency-Associated Chronic Obstructive Pulmonary Disease.](https://pubmed.ncbi.nlm.nih.gov/42075511/), Diagnosis section, initial screening paragraph | Severe AAT deficiency, as stated in a 2026 narrative review (the review's reference [19] was not retrieved). Upper bound is inclusive (<=). Source kind: diagnostic. Source band <= 0.57; stored as the warn low bound 0.57. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
