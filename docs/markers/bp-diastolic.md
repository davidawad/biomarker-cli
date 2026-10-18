# Blood Pressure, Diastolic (`bp-diastolic`)

The pressure in the arteries between heart beats, the lower number of a blood pressure reading. Used with systolic pressure to classify blood pressure and to set treatment targets.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | mmHg |
| LOINC | [8462-4](https://loinc.org/8462-4/) |
| Specimen | - |
| Aliases | `diastolic`, `diastolic bp`, `dbp`, `blood pressure diastolic` |

## Ranges

Bounds in mmHg. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 80 | - | - | uncited (legacy) | - |
| acc-aha-2017-bp-categories | warn | all | 80 | - | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 normal and elevated categories: diastolic below 80 mmHg. The upper limit is strictly below 80. Primary guideline text was not retrievable; the category wording is as stated in the cited paper's Methods. Source kind: decision. Source band <= 80; stored as the warn low bound 80. |
| alt-1 | warn | all | - | 89 | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 stage 1 hypertension: diastolic 80-89 mmHg and/or systolic 130-139 mmHg. Source kind: decision. Source band 80-89; stored as the warn high bound 89. Alternative to the default acc-aha-2017-bp-categories warn range for the same population. |
| alt-2 | warn | all | - | 90 | guideline | Adults; categories as applied in a Chinese adult cohort using the 2017 ACC/AHA guideline | [Stage 1 hypertension defined by the 2017 ACC/AHA blood pressure guideline and cardiometabolic multimorbidity in Chinese adults.](https://pubmed.ncbi.nlm.nih.gov/37710423/), Methods, BP categories | ACC/AHA 2017 stage 2 hypertension: diastolic 90 mmHg or higher and/or systolic 140 mmHg or higher. Source kind: decision. Source band >= 90; stored as the warn high bound 90. Alternative to the default acc-aha-2017-bp-categories warn range for the same population. |
| esc-2024-pharmacotherapy-review | optimal | all | 70 | 79 | guideline | Adults receiving antihypertensive pharmacotherapy (2024 ESC guideline, as summarised) | [2024 ESC Guidelines for the management of elevated blood pressure and hypertension: what is new in pharmacotherapy?](https://pubmed.ncbi.nlm.nih.gov/39439212/), Treatment target paragraph | Diastolic treatment target of 70-79 mmHg (Class 2b) in the 2024 ESC guideline, as stated in the cited summary. The primary guideline text was not retrievable. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
