# Thyroid Peroxidase Antibodies (`tpo-ab`)

Autoantibodies against thyroid peroxidase, the main marker of autoimmune (Hashimoto) thyroid disease. Positivity cut-offs vary by assay and are associated with higher risk of pregnancy complications and miscarriage.

| | |
|---|---|
| Category | [thyroid](index.md#thyroid) |
| Canonical unit | IU/mL |
| LOINC | [8099-4](https://loinc.org/8099-4/) |
| Specimen | - |
| Aliases | `tpo`, `anti-tpo` |
| Panels | [Thyroid function and autoimmunity](../panels/thyroid.md) |

## Ranges

Bounds in IU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 34 | - | - | uncited (legacy) | - |
| cn-maternal-tpo-cohort-2026 | warn | pregnant | - | 34 | cohort | 10,154 Chinese mother-newborn pairs, maternal serum sampled before birth (retrospective cohort) | [Maternal TPO-Ab status modulates associations between gestational thyroid hormones and neonatal TSH in 10,154 Chinese mother-newborn pairs.](https://pubmed.ncbi.nlm.nih.gov/42137608/), Methods | Positivity definition used by the study for maternal TPO-Ab; assay-specific cut-off. Source kind: diagnostic. Source band >= 34; stored as the warn high bound 34. |
| cn-preconception-cohort-2026 | reference | all | - | 9 | reference-interval-study | Chinese women in routine preconception evaluation (retrospective cohort, n = 256) | [Correlation analysis between serum 25-hydroxyvitamin D levels and thyroid peroxidase antibody positivity in preconception women.](https://pubmed.ncbi.nlm.nih.gov/42143362/), Methods | Assay reference range as stated by the study (not a population-derived interval); assay dependent. |
| cn-preconception-cohort-2026 | warn | all | - | 34 | cohort | Chinese women in routine preconception evaluation (retrospective cohort, n = 256) | [Correlation analysis between serum 25-hydroxyvitamin D levels and thyroid peroxidase antibody positivity in preconception women.](https://pubmed.ncbi.nlm.nih.gov/42143362/), Methods | Positivity threshold prespecified by the study for the primary analysis; the same study reports an assay reference range of 0-9 IU/mL. Source kind: diagnostic. Source band >= 34; stored as the warn high bound 34. |
| cn-tpoab-gdm-cohort-2026 | warn | pregnant | - | 12 | cohort | Pregnant women in early pregnancy, single centre (Chinese cohort, gestational diabetes study) (method: chemiluminescence immunoassay (Architect i2000SR)) | [Lipid profiles and risk of gestational diabetes mellitus in women with isolated TPOAb positivity during early pregnancy.](https://pubmed.ncbi.nlm.nih.gov/42156488/), Methods | Study defines positivity as the manufacturer's upper limit of normal for this assay; not a guideline cut-off and assay dependent. Source kind: diagnostic. Source band >= 12; stored as the warn high bound 12. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
