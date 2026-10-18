# Varicella Zoster IgG (`varicella-igg`)

IgG antibody to varicella-zoster virus (VZV), measured by ELISA. Indicates prior varicella infection or vaccination. Reported against an assay cut-off as positive, borderline or negative. Expected result: positive.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | IU/mL |
| LOINC | [19162-7](https://loinc.org/19162-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in IU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 8 | 11 | cohort | Children and adults tested with EUROIMMUN anti-VZV IgG ELISA (Northern Lebanon cross-sectional study, n = 180, aged 1-18 years) | [Seroprevalence and Vaccination Determinants of Varicella Zoster Virus Among Pediatric and Adolescent Populations in Northern Lebanon.](https://pubmed.ncbi.nlm.nih.gov/41295539/), 2.3. Serological Testing | Borderline by the EUROIMMUN ELISA classification (>= 8 and strictly below 11 IU/mL). Assay-specific. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | all | 8 | - | cohort | Children and adults tested with EUROIMMUN anti-VZV IgG ELISA (Northern Lebanon cross-sectional study, n = 180, aged 1-18 years) | [Seroprevalence and Vaccination Determinants of Varicella Zoster Virus Among Pediatric and Adolescent Populations in Northern Lebanon.](https://pubmed.ncbi.nlm.nih.gov/41295539/), 2.3. Serological Testing | Negative by the EUROIMMUN ELISA classification (strictly below 8 IU/mL). Assay-specific. Source kind: diagnostic. Source band <= 8; stored as the warn low bound 8. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 11 | cohort | Children and adults tested with EUROIMMUN anti-VZV IgG ELISA (Northern Lebanon cross-sectional study, n = 180, aged 1-18 years) | [Seroprevalence and Vaccination Determinants of Varicella Zoster Virus Among Pediatric and Adolescent Populations in Northern Lebanon.](https://pubmed.ncbi.nlm.nih.gov/41295539/), 2.3. Serological Testing | Positive by the EUROIMMUN ELISA classification (>= 11 IU/mL). Assay-specific manufacturer criterion; not a general clinical cut-off. Source kind: diagnostic. Source band >= 11; stored as the warn high bound 11. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
