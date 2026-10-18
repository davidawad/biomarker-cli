# Urine Protein, 24 Hour (`urine-protein-24h`)

Total protein excreted in urine over 24 hours, in mg/24h, from a timed urine collection (or estimated from a spot urine protein-to-creatinine ratio). Used to detect and stage proteinuria, a marker of kidney damage used with GFR to define chronic kidney disease. The ranges below give the cited thresholds only: the pregnancy upper limit of normal and the KDIGO 2024 CKD diagnostic threshold. No adult non-pregnant reference interval was verified.

| | |
|---|---|
| Category | [kidney](index.md#kidney) |
| Canonical unit | mg/24h |
| LOINC | [2889-4](https://loinc.org/2889-4/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/24h. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | pregnant | - | 260 | reference-interval-study | Healthy pregnant women aged 35 or under without diabetes, hypertension, pyelonephritis, preeclampsia, or renal or connective tissue disease; 24-hour urine protein | [Normal values of urinary albumin and total protein excretion during pregnancy.](https://pubmed.ncbi.nlm.nih.gov/7943114/), Abstract (Conclusion) | Upper limit of normal in pregnancy (upper 95% confidence limit 259.4 mg per 24 hours; mean 116.9 mg per 24 hours). Source states the value as 260 mg per 24 hours. |
| diagnostic | warn | all | - | 500 | guideline | Adults; 24-hour urine protein excretion (PER) or the approximately equivalent PCR | [KDIGO 2024 Clinical Practice Guideline for the Evaluation and Management of CKD](https://kdigo.org/wp-content/uploads/2024/03/KDIGO-2024-CKD-Guideline.pdf), Figure 48 (circumstances for referral: diagnosis of CKD with significant albuminuria) | Persistent protein excretion of 500 mg/24 h or more (PER), approximately equivalent to PCR 500 mg/g or more, is listed with the diagnosis of CKD with significant albuminuria (KDIGO 2024 Figure 48), written as >=500 in the source. Source kind: diagnostic. Source band >= 500; stored as the warn high bound 500. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
