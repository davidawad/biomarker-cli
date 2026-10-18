# Respiratory Rate (`respiratory-rate`)

Number of breaths taken per minute at rest. A raised respiratory rate is an early sign of acute illness, sepsis or respiratory distress, and it is one of the parameters in the NEWS2 early warning score and the qSOFA sepsis screen.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | breaths/min |
| LOINC | [9279-1](https://loinc.org/9279-1/) |
| Specimen | - |
| Aliases | `rr`, `breathing rate` |

## Ranges

Bounds in breaths/min. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 12 | 20 | consensus | Adults, as used in the NEWS2 early warning score chart (score 0 band for respiration rate) | [https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf](https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf), Chart 1: The NEWS scoring system (Respiration rate, per minute) | Respiration rate 12-20 breaths/min is the NEWS2 score-0 band, i.e. the normal range used for early-warning scoring. It is a scoring band, not a population reference interval; boundaries are inclusive as printed in the chart. |
| alt-1 | critical | all | - | 25 | consensus | Adults, as used in the NEWS2 early warning score chart (score 3 band for respiration rate) | [https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf](https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf), Chart 1: The NEWS scoring system (Respiration rate, per minute) | NEWS2 score-3 band: respiration rate of 25 breaths/min or more. Boundary as printed in the chart. Source band >= 25; stored as the critical high bound 25. Alternative to the default rcp-news2-2017 critical range for the same population. |
| alt-1 | warn | all | - | 24 | consensus | Adults, as used in the NEWS2 early warning score chart (score 2 band for respiration rate) | [https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf](https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf), Chart 1: The NEWS scoring system (Respiration rate, per minute) | NEWS2 score-2 band: respiration rate 21-24 breaths/min. Boundaries inclusive as printed. Source band 21-24; stored as the warn high bound 24. Alternative to the default rcp-news2-2017 warn range for the same population. |
| rcp-news2-2017 | critical | all | 8 | - | consensus | Adults, as used in the NEWS2 early warning score chart (score 3 band for respiration rate) | [https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf](https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf), Chart 1: The NEWS scoring system (Respiration rate, per minute) | NEWS2 score-3 band: respiration rate of 8 breaths/min or fewer (scores 3 for a single parameter; a reduced rate indicates CNS depression or narcosis). Boundary as printed in the chart. Source band <= 8; stored as the critical low bound 8. |
| rcp-news2-2017 | warn | all | 9 | - | consensus | Adults, as used in the NEWS2 early warning score chart (score 1 band for respiration rate) | [https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf](https://www.rcp.ac.uk/media/a4ibkkbf/news2-final-report_0_0.pdf), Chart 1: The NEWS scoring system (Respiration rate, per minute) | NEWS2 score-1 band: respiration rate 9-11 breaths/min (low-ish rate that adds one point). Boundaries inclusive as printed. Source band 9-11; stored as the warn low bound 9. |
| sepsis-3-2016-qsofa | warn | all | - | 22 | consensus | Adults with suspected infection outside the ICU (out-of-hospital, emergency department or general ward) in the Sepsis-3 consensus | [The Third International Consensus Definitions for Sepsis and Septic Shock (Sepsis-3).](https://pubmed.ncbi.nlm.nih.gov/26903338/), Abstract | qSOFA criterion: respiratory rate of 22/min or greater (one of three bedside criteria; two or more positive criteria indicate higher risk of poor outcome). A screening threshold, not a diagnostic cut-off for sepsis. Source kind: decision. Source band >= 22; stored as the warn high bound 22. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
