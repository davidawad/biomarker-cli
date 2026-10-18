# Apnea-Hypopnea Index (`apnea-hypopnea-index`)

Number of apnea and hypopnea events per hour of sleep, the standard measure of obstructive sleep apnea severity. Higher values indicate more severe sleep-disordered breathing.

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | events/h |
| LOINC | [69990-0](https://loinc.org/69990-0/) |
| Specimen | - |
| Aliases | `ahi`, `apnoea-hypopnoea index`, `rei` |

## Ranges

Bounds in events/h. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 5 | guideline | Adults (normal band of the AHI/REI severity grading used for obstructive sleep apnoea) | [Obstructive Sleep Apneas and Cardiovascular Diseases.](https://pubmed.ncbi.nlm.nih.gov/42200975/), Full text, OSA severity paragraph | Normal: AHI below 5 events/h, as graded in the cited 2026 review of OSA and cardiovascular disease. The review gives the grading without a separate source citation in the retrieved text. |
| aasm-osa-severity | warn | all | - | 14 | guideline | Adults with obstructive sleep apnoea | [Optimizing Surgical Choice in Mild and Moderate OSA: Anterior Palatoplasty vs. Radiofrequency Uvulopalatoplasty.](https://pubmed.ncbi.nlm.nih.gov/42073495/), Introduction, paragraph on OSA severity (cites AASM) | Mild OSA band as published (AHI 5-14 events/h). The source attributes this grading to the AASM and ICSD. Boundary as published; the 2026 review's own grading uses 5-15. Source kind: decision. Source band 5-14; stored as the warn high bound 14. |
| alt-1 | warn | all | - | 29 | guideline | Adults with obstructive sleep apnoea | [Optimizing Surgical Choice in Mild and Moderate OSA: Anterior Palatoplasty vs. Radiofrequency Uvulopalatoplasty.](https://pubmed.ncbi.nlm.nih.gov/42073495/), Introduction, paragraph on OSA severity (cites AASM) | Moderate OSA band as published (AHI 15-29 events/h). Source kind: decision. Source band 15-29; stored as the warn high bound 29. Alternative to the default aasm-osa-severity warn range for the same population. |
| alt-2 | warn | all | - | 30 | guideline | Adults with obstructive sleep apnoea | [Optimizing Surgical Choice in Mild and Moderate OSA: Anterior Palatoplasty vs. Radiofrequency Uvulopalatoplasty.](https://pubmed.ncbi.nlm.nih.gov/42073495/), Introduction, paragraph on OSA severity (cites AASM) | Severe OSA band, published as AHI greater than 30 events/h (open upper bound). Boundary at exactly 30 is not explicit in the source; the 2026 review grades severe as '>30' as well. Source kind: decision. Source band >= 30; stored as the warn high bound 30. Alternative to the default aasm-osa-severity warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
