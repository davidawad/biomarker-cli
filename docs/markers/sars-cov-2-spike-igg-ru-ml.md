# SARS-CoV-2 Spike IgG Antibody (RU/mL) (`sars-cov-2-spike-igg-ru-ml`)

SARS-CoV-2 Spike IgG Antibody reported in RU/mL. Results in RU/mL are assay specific and are not interchangeable with SARS-CoV-2 Spike IgG Antibody in BAU/mL.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | RU/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in RU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 8 | 11 | cohort | Egyptian community sera (n = 650), cross-sectional, January-June 2021 (method: euroimmun-quantivac-elisa) | [Determining the SARS-CoV-2 Anti-Spike Cutoff Level Denoting Neutralizing Activity Using Two Commercial Kits.](https://pubmed.ncbi.nlm.nih.gov/36423048/), Methods | Borderline on EUROIMMUN QuantiVac ELISA (IgG); the cited study counted borderline as positive. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | all | - | 11 | cohort | Egyptian community sera (n = 650), cross-sectional, January-June 2021 (method: euroimmun-quantivac-elisa) | [Determining the SARS-CoV-2 Anti-Spike Cutoff Level Denoting Neutralizing Activity Using Two Commercial Kits.](https://pubmed.ncbi.nlm.nih.gov/36423048/), Methods | Positive on EUROIMMUN QuantiVac ELISA (IgG), per manufacturer categories as reported in the cited study. Source kind: diagnostic. Source band >= 11; stored as the warn high bound 11. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | 8 | - | cohort | Egyptian community sera (n = 650), cross-sectional, January-June 2021 (method: euroimmun-quantivac-elisa) | [Determining the SARS-CoV-2 Anti-Spike Cutoff Level Denoting Neutralizing Activity Using Two Commercial Kits.](https://pubmed.ncbi.nlm.nih.gov/36423048/), Methods | Negative on EUROIMMUN QuantiVac ELISA (IgG), per manufacturer categories as reported in the cited study. Source kind: diagnostic. Source band <= 8; stored as the warn low bound 8. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
