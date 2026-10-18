# Herpes Simplex Virus 2 IgG (`hsv2-igg`)

IgG antibody to herpes simplex virus type 2 (HSV-2), usually a gG2-based ELISA reported as a normalised optical density index. Tracked to establish past HSV-2 (genital herpes) infection, which is often asymptomatic. Expected result: negative (not detected) in most people without past HSV-2 infection.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | index |
| LOINC | [17851-7](https://loinc.org/17851-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.9 | 1.1 | cohort | Adults tested with the HerpeSelect HSV-2 gG2 ELISA (normalised OD index) | [Comparison of focus HerpesSelect and Kalon HSV-2 gG2 ELISA serological assays to detect herpes simplex virus type 2 antibodies in a South African population.](https://pubmed.ncbi.nlm.nih.gov/19837726/), Methods | Equivocal zone for the HerpeSelect HSV-2 ELISA (0.9 to 1.1), per the study's stated interpretation of the manufacturer's bands. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| diagnostic | warn | all | - | 1.1 | cohort | Adults tested with the HerpeSelect HSV-2 gG2 ELISA (normalised OD index) | [Comparison of focus HerpesSelect and Kalon HSV-2 gG2 ELISA serological assays to detect herpes simplex virus type 2 antibodies in a South African population.](https://pubmed.ncbi.nlm.nih.gov/19837726/), Methods | Positive for HSV-2 IgG on the HerpeSelect ELISA (value strictly above 1.1); the cut-off is the manufacturer's, and the study notes higher cut-offs improve specificity. Source kind: diagnostic. Source band >= 1.1; stored as the warn high bound 1.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
