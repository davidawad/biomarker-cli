# LDL Particle Size (`ldl-size`)

Average diameter of LDL particles in nanometres, calculated from LDL composition. In one cohort study, both larger and smaller average LDL diameters than the intermediate category were associated with higher mortality. No population reference interval was verified; the ranges below are single-study outcome classification cut-offs.

| | |
|---|---|
| Category | [lipid](index.md#lipid) |
| Canonical unit | nm |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in nm. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | 16.5 | - | cohort | Patients referred to coronary angiography, not on lipid-lowering drugs (Ludwigshafen Risk and Cardiovascular Health Study, n = 1643) (method: beta-quantification) | [Low-density lipoprotein particle diameter and mortality: the Ludwigshafen Risk and Cardiovascular Health Study.](https://pubmed.ncbi.nlm.nih.gov/24569029/), Abstract, Results | Small average LDL diameter (strictly below 16.5 nm) versus intermediate reference category (16.5-16.8 nm). Diameter calculated from beta-quantification LDL composition, not NMR. Hazard ratio for cardiovascular death 1.54 (95% CI 1.06-2.12). Source band <= 16.5; stored as the warn low bound 16.5. |
| alt-1 | warn | all | - | 16.8 | cohort | Patients referred to coronary angiography, not on lipid-lowering drugs (Ludwigshafen Risk and Cardiovascular Health Study, n = 1643) (method: beta-quantification) | [Low-density lipoprotein particle diameter and mortality: the Ludwigshafen Risk and Cardiovascular Health Study.](https://pubmed.ncbi.nlm.nih.gov/24569029/), Abstract, Results | Large average LDL diameter (strictly above 16.8 nm) versus intermediate reference category (16.5-16.8 nm). Diameter calculated from beta-quantification LDL composition, not NMR. Hazard ratio for cardiovascular death 1.89 (95% CI 1.32-2.70). Source band >= 16.8; stored as the warn high bound 16.8. Alternative to the default clinical warn range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
