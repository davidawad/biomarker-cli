# Tetanus Antitoxin IgG (`tetanus-antitoxin-igg`)

Serum IgG antibody to tetanus toxin (tetanus antitoxin), measured against WHO international standards. It indicates protection against tetanus; levels fall with time after vaccination, so adults often have low or undetectable titres.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | IU/mL |
| LOINC | - |
| Specimen | - |
| Aliases | `tetanus igg`, `anti-tetanus toxoid igg`, `tetanus toxin antibody`, `tetanus antibody`, `tetanus-antitoxin` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in IU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 0.1 | cohort | Healthy individuals in Zhejiang, China (cross-sectional study of 3,659 people in 2024) (method: elisa) | [Population waning trajectories of vaccine-induced tetanus immunity in Zhejiang, China.](https://pubmed.ncbi.nlm.nih.gov/42266706/), Abstract, Methods | Seroprotection: IgG at or above 0.1 IU/mL by ELISA (boundary >= 0.1). Source kind: decision. Source band >= 0.1; stored as the warn high bound 0.1. Alternative to the default zhejiang-2026-tetanus-waning warn range for the same population. |
| tetanus-vaccine-phase-iii-2025 | warn | all | - | 1 | cohort | Healthy adults aged 18-44 years in a randomised phase I/III single-dose tetanus vaccine trial | [Safety and Immunogenicity of Single-Dose of Adsorbed Tetanus Vaccine in Adults Aged 18-44 Years: Randomized, Double-Blind, Positive-Controlled Phase I/III Clinical Trial.](https://pubmed.ncbi.nlm.nih.gov/41012175/), Methods, immunogenicity endpoints | Long-term seroprotection: IgG at or above 1.0 IU/mL (boundary >= 1.0), as defined by the trial authors (cited to a separate reference in the source). Source kind: decision. Source band >= 1.0; stored as the warn high bound 1.0. |
| zhejiang-2026-tetanus-waning | warn | all | - | 0.01 | cohort | Healthy individuals in Zhejiang, China (cross-sectional study of 3,659 people in 2024) (method: elisa) | [Population waning trajectories of vaccine-induced tetanus immunity in Zhejiang, China.](https://pubmed.ncbi.nlm.nih.gov/42266706/), Abstract, Methods | Seropositivity: IgG at or above 0.01 IU/mL by ELISA (boundary >= 0.01). Indicates detectable antibody only; the WHO position paper threshold could not be retrieved as text, so this is taken from the population study that adopted it. Source kind: decision. Source band >= 0.01; stored as the warn high bound 0.01. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
