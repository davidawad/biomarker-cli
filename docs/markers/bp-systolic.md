# Blood Pressure, Systolic (`bp-systolic`)

The pressure in the arteries during the heart's contraction, the upper number of a blood pressure reading. Used to classify blood pressure and to set treatment targets.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | mmHg |
| LOINC | [8480-6](https://loinc.org/8480-6/) |
| Specimen | - |
| Aliases | `systolic`, `systolic bp`, `sbp`, `blood pressure systolic` |

## Ranges

Bounds in mmHg. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 120 | - | - | uncited (legacy) | - |
| clinical | warn | all | - | 129 | - | - | uncited (legacy) | 120-129 mmHg: elevated |
| acc-aha-2017-bp-categories | warn | all | 120 | - | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 normal blood pressure: systolic below 120 mmHg (and diastolic below 80 mmHg). The upper limit is strictly below 120. Primary guideline text was not retrievable; the category wording is as stated in the cited paper's Methods. Source kind: decision. Source band <= 120; stored as the warn low bound 120. |
| alt-1 | warn | all | - | 129 | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 elevated blood pressure: systolic 120-129 mmHg with diastolic below 80 mmHg. Source kind: decision. Source band 120-129; stored as the warn high bound 129. Alternative to the default acc-aha-2017-bp-categories warn range for the same population. |
| alt-2 | warn | all | - | 139 | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 stage 1 hypertension: systolic 130-139 mmHg and/or diastolic 80-89 mmHg. Source kind: decision. Source band 130-139; stored as the warn high bound 139. Alternative to the default acc-aha-2017-bp-categories warn range for the same population. |
| alt-3 | warn | all | - | 140 | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 stage 2 hypertension: systolic 140 mmHg or higher and/or diastolic 90 mmHg or higher (or on antihypertensive drugs, not captured by this numeric range). Source kind: decision. Source band >= 140; stored as the warn high bound 140. Alternative to the default acc-aha-2017-bp-categories warn range for the same population. |
| esc-2024-pharmacotherapy-review | optimal | all | 120 | 129 | guideline | Adults receiving antihypertensive pharmacotherapy (2024 ESC guideline, as summarised) | [2024 ESC Guidelines for the management of elevated blood pressure and hypertension: what is new in pharmacotherapy?](https://pubmed.ncbi.nlm.nih.gov/39439212/), Treatment target paragraph | Treatment target for most adults on pharmacotherapy (Class I), supported by BP-lowering treatment trials. When not tolerated, the guideline advises targeting a systolic BP as low as reasonably achievable. The primary guideline text was not retrievable; the target wording is from the cited summary. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
