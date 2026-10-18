# Mumps IgG (`mumps-igg`)

Serum IgG antibody against mumps virus, measured by ELISA and reported as an assay-specific index or titer. Tracked to confirm immunity after mumps infection or MMR vaccination, and to check susceptibility before exposure. Expected result: positive (seropositive, immune); negative or equivocal results indicate possible lack of immunity.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | RU/mL |
| LOINC | [22415-4](https://loinc.org/22415-4/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in RU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 16 | 22 | cohort | Community-based cross-sectional sample, Vellore, South India, mumps IgG by EUROIMMUN ELISA (method: euroimmun-elisa) | [Mumps Seroprevalence in Vellore, South India: A Community-Based Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/40795852/), Methods | Equivocal mumps IgG on EUROIMMUN ELISA (16 to <22 RU/mL). Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | all | 16 | - | cohort | Community-based cross-sectional sample, Vellore, South India, mumps IgG by EUROIMMUN ELISA (method: euroimmun-elisa) | [Mumps Seroprevalence in Vellore, South India: A Community-Based Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/40795852/), Methods | Seronegative (no detectable immunity) on EUROIMMUN ELISA (<16 RU/mL). Source kind: diagnostic. Source band <= 16; stored as the warn low bound 16. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 22 | cohort | Community-based cross-sectional sample, Vellore, South India, mumps IgG by EUROIMMUN ELISA (method: euroimmun-elisa) | [Mumps Seroprevalence in Vellore, South India: A Community-Based Cross-Sectional Study.](https://pubmed.ncbi.nlm.nih.gov/40795852/), Methods | Seropositive (immune) on EUROIMMUN ELISA, per manufacturer's guidelines as applied in the cited study. Source kind: diagnostic. Source band >= 22; stored as the warn high bound 22. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
