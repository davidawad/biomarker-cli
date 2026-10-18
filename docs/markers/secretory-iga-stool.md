# Secretory IgA, Stool (`secretory-iga-stool`)

Secretory immunoglobulin A in stool, a marker of mucosal immune barrier function in the gut. Measured by ELISA on a stool extract; results are assay specific and reported as concentration of the extract (ug/mL in the ImmuChrom kit used in the cited study). Reduced values are read as subnormal, high values as elevated.

| | |
|---|---|
| Category | [stool](index.md#stool) |
| Canonical unit | ug/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ug/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 510 | 2040 | cohort | Adults aged 18-74 years (160 stool samples: 49 men, 111 women) | [Impact of Biometric Patient Data, Probiotic Supplementation, and Selected Gut Microorganisms on Calprotectin, Zonulin, and sIgA Concentrations in the Stool of Adults Aged 18-74 Years.](https://pubmed.ncbi.nlm.nih.gov/36551209/), Table 1 | Normal band as tabulated in the cited observational study (Table 1, reference ranges from the ImmuChrom ELISA kit). The study did not derive these ranges itself. Assay-specific. |
| clinical | warn | all | 510 | - | cohort | Adults aged 18-74 years (160 stool samples: 49 men, 111 women) | [Impact of Biometric Patient Data, Probiotic Supplementation, and Selected Gut Microorganisms on Calprotectin, Zonulin, and sIgA Concentrations in the Stool of Adults Aged 18-74 Years.](https://pubmed.ncbi.nlm.nih.gov/36551209/), Table 1 | Classified as subnormal in the cited study's Table 1 (kit classification). Assay-specific. Source band <= 510; stored as the warn low bound 510. |
| alt-1 | warn | all | - | 2040 | cohort | Adults aged 18-74 years (160 stool samples: 49 men, 111 women) | [Impact of Biometric Patient Data, Probiotic Supplementation, and Selected Gut Microorganisms on Calprotectin, Zonulin, and sIgA Concentrations in the Stool of Adults Aged 18-74 Years.](https://pubmed.ncbi.nlm.nih.gov/36551209/), Table 1 | Classified as elevated in the cited study's Table 1 (kit classification). Assay-specific. No upper limit of quantification given in the source. Source band >= 2040; stored as the warn high bound 2040. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
