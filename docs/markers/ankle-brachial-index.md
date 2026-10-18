# Ankle-Brachial Index (`ankle-brachial-index`)

Ratio of the systolic pressure at the ankle to the systolic brachial pressure, measured with a cuff and Doppler. It is the first-line physiological test for peripheral artery disease (PAD).

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | ratio |
| LOINC | - |
| Specimen | - |
| Aliases | `abi`, `ankle brachial pressure index`, `ankle-brachial pressure index` |

## Ranges

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 1 | 1.4 | guideline | Adults with suspected or known lower extremity peripheral artery disease, resting ABI in each leg | [2024 ACC/AHA/AACVPR/APMA/ABC/SCAI/SVM/SVN/SVS/SIR/VESS Guideline for the Management of Lower Extremity Peripheral Artery Disease: A Report of the American College of Cardiology/American Heart Association Joint Committee on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/38752899/), Full text, Diagnosis: resting ABI reporting statement | Normal resting ABI band (1.00 to 1.40, boundaries inclusive) as defined by the guideline. |
| acc-aha-2024-pad | warn | all | 0.9 | - | guideline | Adults with suspected or known lower extremity peripheral artery disease, resting ABI in each leg | [2024 ACC/AHA/AACVPR/APMA/ABC/SCAI/SVM/SVN/SVS/SIR/VESS Guideline for the Management of Lower Extremity Peripheral Artery Disease: A Report of the American College of Cardiology/American Heart Association Joint Committee on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/38752899/), Full text, Diagnosis: resting ABI reporting statement | Abnormal resting ABI (<=0.90, boundary inclusive); PAD is suspected at this level. Sensitivity is reduced in diabetes. Source kind: decision. Source band <= 0.9; stored as the warn low bound 0.9. |
| alt-1 | warn | all | 0.91 | - | guideline | Adults with suspected or known lower extremity peripheral artery disease, resting ABI in each leg | [2024 ACC/AHA/AACVPR/APMA/ABC/SCAI/SVM/SVN/SVS/SIR/VESS Guideline for the Management of Lower Extremity Peripheral Artery Disease: A Report of the American College of Cardiology/American Heart Association Joint Committee on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/38752899/), Full text, Diagnosis: resting ABI reporting statement | Borderline resting ABI (0.91 to 0.99). Exercise treadmill ABI is suggested for suspected chronic symptomatic PAD in this band and in the normal band. Source band 0.91-0.99; stored as the warn low bound 0.91. Alternative to the default acc-aha-2024-pad warn range for the same population. |
| alt-2 | warn | all | - | 1.4 | guideline | Adults with suspected or known lower extremity peripheral artery disease, resting ABI in each leg | [2024 ACC/AHA/AACVPR/APMA/ABC/SCAI/SVM/SVN/SVS/SIR/VESS Guideline for the Management of Lower Extremity Peripheral Artery Disease: A Report of the American College of Cardiology/American Heart Association Joint Committee on Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/38752899/), Full text, Diagnosis: resting ABI reporting statement | Noncompressible vessels (resting ABI above 1.40, boundary exclusive). The ABI is not interpretable here; toe pressure or toe-brachial index is recommended. Common in diabetes and CKD. Source kind: decision. Source band >= 1.4; stored as the warn high bound 1.4. Alternative to the default acc-aha-2024-pad warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
