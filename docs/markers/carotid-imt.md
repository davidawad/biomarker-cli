# Carotid Intima-Media Thickness (`carotid-imt`)

Ultrasound thickness of the intima-media complex of the common carotid artery; measured as a subclinical atherosclerosis and cardiovascular risk marker. Expected result in healthy adults: a thickness at or below the cut-offs below, and no plaque (plaque is a separate measure).

| | |
|---|---|
| Category | [cardiac](index.md#cardiac) |
| Canonical unit | mm |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mm. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | 0.7 | - | cohort | Healthy middle-aged adults in urban poor settings in Kenya (AWI-Gen Nairobi site, observational study) | [Carotid intima media thickness (CIMT) in adults in the AWI-Gen Nairobi site study: Profiles and predictors.](https://pubmed.ncbi.nlm.nih.gov/36153304/), Discussion (Increased left CIMT values) | Single observational study's classification of left CIMT as high (strictly above 0.70 mm). Not a guideline cut-off. Source band <= 0.7; stored as the warn low bound 0.7. |
| diagnostic | warn | all | - | 0.9 | cohort | Adults (general population), as stated by the Kenyan AWI-Gen observational study | [Carotid intima media thickness (CIMT) in adults in the AWI-Gen Nairobi site study: Profiles and predictors.](https://pubmed.ncbi.nlm.nih.gov/36153304/), Discussion (Normal values of CIMT) | Strictly above 0.90 mm: the study attributes this value to the ESC as indicating organ damage. The ESC document itself was not retrieved and not verified. Source kind: diagnostic. Source band >= 0.9; stored as the warn high bound 0.9. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
