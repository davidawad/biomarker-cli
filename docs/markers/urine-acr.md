# Urine Albumin-to-Creatinine Ratio (`urine-acr`)

Albumin excreted in urine relative to creatinine, measured on a spot sample. It is the standard screen for albuminuria, the earliest marker of kidney damage in diabetes and hypertension, and an independent cardiovascular risk marker.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | mg/g |
| LOINC | [9318-7](https://loinc.org/9318-7/) |
| Specimen | - |
| Aliases | `urine acr`, `microalbumin-to-creatinine ratio`, `urine albumin/creatinine ratio` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 300 | guideline | People with chronic kidney disease, albuminuria category A3 (severely increased) | [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease; [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Figure caption defining albuminuria categories A1-A3 | KDIGO 2024 A3 band: ACR above 300 mg/g (boundary >300, mg/mmol equivalent >30). Source kind: decision. Source band >= 300; stored as the warn high bound 300. Alternative to the default kdigo-2024 warn range for the same population. |
| alt-2 | warn | all | - | 265.2 | guideline | Adults with chronic kidney disease, ACR category A3 | [https://www.nice.org.uk/guidance/ng203/chapter/Recommendations](https://www.nice.org.uk/guidance/ng203/chapter/Recommendations), Table 1 (ACR categories) | NICE NG203 ACR category A3 (severely increased), above 30 mg/mmol (boundary >30). Converted from the source's mg/mmol (30) to mg/g using factor 8.84. Source kind: decision. Source band >= 265.2; stored as the warn high bound 265.2. Alternative to the default nice-ng203 warn range for the same population. |
| alt-3 | warn | all | - | 26.52 | guideline | Adults and children with a confirmed raised urine ACR | [https://www.nice.org.uk/guidance/ng203/chapter/Recommendations](https://www.nice.org.uk/guidance/ng203/chapter/Recommendations), Recommendation 1.1.13 | NICE NG203 recommendation 1.1.13: a confirmed ACR of 3 mg/mmol or more is clinically important proteinuria (boundary >=3). Replaces the earlier ERBP commentary citation, which restated the same threshold secondhand. Converted from the source's mg/mmol (3) to mg/g using factor 8.84. Source kind: decision. Source band >= 26.52; stored as the warn high bound 26.52. Alternative to the default nice-ng203 warn range for the same population. |
| alt-4 | warn | all | - | 618.8 | guideline | Adults and children with CKD being assessed for proteinuria | [https://www.nice.org.uk/guidance/ng203/chapter/Recommendations](https://www.nice.org.uk/guidance/ng203/chapter/Recommendations), Recommendation 1.1.12 | NICE NG203 recommendation 1.1.12: an initial ACR of 70 mg/mmol or more does not need a confirmatory repeat sample (boundary >=70). Converted from the source's mg/mmol (70) to mg/g using factor 8.84. Source kind: decision. Source band >= 618.8; stored as the warn high bound 618.8. Alternative to the default nice-ng203 warn range for the same population. |
| kdigo-2024 | warn | all | 30 | - | guideline | People with chronic kidney disease, albuminuria category A1 (normal to mildly increased) | [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Table 3 \| Albuminuria categories in chronic kidney disease; [https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Figure caption defining albuminuria categories A1-A3 | KDIGO 2024 A1 band: ACR below 30 mg/g (boundary <30, so 30 itself is not in A1). The mg/mmol equivalent is <3. Source kind: decision. Source band <= 30; stored as the warn low bound 30. |
| nice-ng203 | warn | all | 26.52 | - | guideline | Adults with chronic kidney disease, ACR category A1 | [https://www.nice.org.uk/guidance/ng203/chapter/Recommendations](https://www.nice.org.uk/guidance/ng203/chapter/Recommendations), Table 1 (ACR categories) | NICE NG203 ACR category A1 (normal to mildly increased), boundary less than 3 mg/mmol. Converted from the source's mg/mmol (3) to mg/g using factor 8.84. Source kind: decision. Source band <= 26.52; stored as the warn low bound 26.52. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
