# Cobalt, whole blood (`cobalt`)

Cobalt concentration in whole blood. In people without metal exposure it is very low; cobalt released from chrome-cobalt hip implants raises it, and high values can signal implant wear and systemic cobalt toxicity.

| | |
|---|---|
| Category | [toxic-metal](index.md#toxic-metal) |
| Canonical unit | µg/L |
| LOINC | - |
| Specimen | - |
| Aliases | `cobalt-blood` |

## Ranges

Bounds in µg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 0.6 | consensus | People without abnormal cobalt exposure, as summarised in a 2019 systematic review of systemic cobaltism after chrome-cobalt hip implant wear (specimen: whole-blood) | [A Systematic Review of Systemic Cobaltism After Wear or Corrosion of Chrome-Cobalt Hip Implants.](https://pubmed.ncbi.nlm.nih.gov/26076080/), Introduction, para 2 | Background level as reported secondhand in the cited review (95th percentile of people without abnormal exposure). Not from the primary reference study. |
| acgih-bei-as-cited | warn | occupational-exposure | - | 1 | consensus | Occupationally exposed workers (biologic exposure index), as summarised in a 2019 systematic review (specimen: whole-blood) | [A Systematic Review of Systemic Cobaltism After Wear or Corrosion of Chrome-Cobalt Hip Implants.](https://pubmed.ncbi.nlm.nih.gov/26076080/), Introduction, para 2 | Occupational biologic exposure threshold quoted secondhand in the cited review. Not a clinical or hip-implant limit. Source band >= 1; stored as the warn high bound 1. |
| mhra-mom-cutoff | warn | metal-on-metal-hip | - | 7 | guideline | Patients with a unilateral metal-on-metal (chrome-cobalt) hip replacement, case-control study of failed vs well-functioning hips (specimen: whole-blood) | [Sensitivity and specificity of blood cobalt and chromium metal ions for predicting failure of metal-on-metal hip replacement.](https://pubmed.ncbi.nlm.nih.gov/21969427/), Abstract | MHRA cut-off for the higher of blood cobalt or chromium, used to prompt review of the implant. In the cited study the 7 ppb cut-off had specificity 89% and sensitivity 52% for unexplained failure, so it is a screening trigger rather than a diagnosis. Applies to the maximum of cobalt or chromium. Source band >= 7; stored as the warn high bound 7. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
