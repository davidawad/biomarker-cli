# Transferrin Saturation (`transferrin-saturation`)

Percentage of transferrin iron-binding sites occupied by iron, calculated from serum iron and transferrin or TIBC. Low values indicate restricted iron supply to the marrow; high values (especially with raised ferritin) are the first-line screen for HFE hemochromatosis.

| | |
|---|---|
| Category | [iron](index.md#iron) |
| Canonical unit | % |
| LOINC | [2502-3](https://loinc.org/2502-3/) |
| Specimen | - |
| Aliases | `tsat`, `iron saturation` |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 15 | 55 | - | - | uncited (legacy) | - |
| optimal | optimal | all | 25 | 45 | - | - | uncited (legacy) | - |
| aasld-2011 | reference | all | 20 | 50 | guideline | Normal subjects, as tabulated in the AASLD 2011 hemochromatosis guideline (typical values, not a sampled reference interval) | [Diagnosis and management of hemochromatosis: 2011 practice guideline by the American Association for the Study of Liver Diseases.](https://pubmed.ncbi.nlm.nih.gov/21452290/), Table 7, Laboratory findings in patients with HH | Typical values table from the guideline. Used as the normal range there; the same table gives 45-100% for asymptomatic HFE haemochromatosis and 80-100% for symptomatic disease. |
| bsg-2021 | warn | all | 20 | - | guideline | Adults with iron deficiency anaemia assessment (UK guideline scope), not limited to CKD | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Discussion of iron restriction and thalassaemia | Guideline describes TSAT below 20% as iron restriction, particularly in thalassaemia carriers. Use alongside ferritin; TSAT is the more useful marker when ferritin is falsely normal. Source kind: deficient. Source band <= 20; stored as the warn low bound 20. |
| bsg-2021 | warn | ferritin <= 100, ckd | 20 | - | guideline | Adults with CKD who are predialysis or on peritoneal dialysis | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Chronic kidney disease section | Absolute iron deficiency in CKD as defined in the guideline; applies only when ferritin is also at or below 100 ug/L. The guideline's 'absolute iron deficiency' definition, not a general population cut-off. Source kind: deficient. Source band <= 20; stored as the warn low bound 20. |
| bsg-2021 | warn | ferritin <= 200, ckd, hemodialysis | 20 | - | guideline | Adults with CKD on haemodialysis | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Chronic kidney disease section | Absolute iron deficiency in haemodialysis patients as defined in the guideline; applies only when ferritin is also at or below 200 ug/L. Source kind: deficient. Source band <= 20; stored as the warn low bound 20. |
| easl-2022 | warn | female, pre-menopausal, ferritin >= 200, hfe-c282y-homozygous | - | 45 | guideline | Premenopausal women homozygous for HFE p.Cys282Tyr, provisional iron overload | [EASL Clinical Practice Guidelines on haemochromatosis.](https://pubmed.ncbi.nlm.nih.gov/35662478/), Abstract | Provisional iron overload criterion, applied together with ferritin above 200 ug/L. The guideline's 'females' wording is read as premenopausal women. The marker check treats 45 as inclusive, while the guideline says above 45%. Source kind: diagnostic. Source band >= 45; stored as the warn high bound 45. |
| easl-2022 | warn | male, ferritin >= 300, hfe-c282y-homozygous | - | 50 | guideline | Men homozygous for HFE p.Cys282Tyr, provisional iron overload | [EASL Clinical Practice Guidelines on haemochromatosis.](https://pubmed.ncbi.nlm.nih.gov/35662478/), Abstract | Provisional iron overload criterion, applied together with ferritin above 300 ug/L. Marker check treats 50 as inclusive; the guideline says above 50%. Source kind: diagnostic. Source band >= 50; stored as the warn high bound 50. |
| easl-2022 | warn | female, post-menopausal, ferritin >= 300, hfe-c282y-homozygous | - | 50 | guideline | Postmenopausal women homozygous for HFE p.Cys282Tyr, provisional iron overload | [EASL Clinical Practice Guidelines on haemochromatosis.](https://pubmed.ncbi.nlm.nih.gov/35662478/), Abstract | Provisional iron overload criterion, applied together with ferritin above 300 ug/L. Marker check treats 50 as inclusive; the guideline says above 50%. Source kind: diagnostic. Source band >= 50; stored as the warn high bound 50. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
