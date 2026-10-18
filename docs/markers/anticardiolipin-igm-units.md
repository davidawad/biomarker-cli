# Anticardiolipin IgM Antibody (units) (`anticardiolipin-igm-units`)

Anticardiolipin IgM Antibody reported in units. Results in units are assay specific and are not interchangeable with Anticardiolipin IgM Antibody in MPL.

| | |
|---|---|
| Category | [autoimmune](index.md#autoimmune) |
| Canonical unit | units |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in units. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | antiphospholipid-syndrome | 40 | 79 | cohort | Patients evaluated for primary antiphospholipid syndrome under the 2023 ACR/EULAR classification criteria | [Insights into the 2023 ACR/EULAR antiphospholipid syndrome classification criteria: findings from a cohort of 205 patients with primary APS.](https://pubmed.ncbi.nlm.nih.gov/39689033/), Introduction | Moderate aCL positivity band (40-79 units) as the cited paper describes the 2023 ACR/EULAR criteria. The source gives units only (arbitrary units, not MPL); the number is copied as stated. Isolated IgM positivity is scored low-weight in those criteria. Source kind: diagnostic. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| diagnostic | warn | antiphospholipid-syndrome | - | 80 | cohort | Patients evaluated for primary antiphospholipid syndrome under the 2023 ACR/EULAR classification criteria | [Insights into the 2023 ACR/EULAR antiphospholipid syndrome classification criteria: findings from a cohort of 205 patients with primary APS.](https://pubmed.ncbi.nlm.nih.gov/39689033/), Introduction | High aCL positivity band as the cited paper describes the 2023 ACR/EULAR criteria. The source writes '>80 units' here and '>=80 units' in its Discussion; the number is copied as stated. Units are the source's arbitrary units, not converted. Source kind: diagnostic. Source band >= 80; stored as the warn high bound 80. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
