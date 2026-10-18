# Copper, Serum (`copper`)

Total serum copper. Most circulating copper is bound to ceruloplasmin, so serum copper tracks ceruloplasmin and is raised by acute-phase states. Reference limits depend on the laboratory assay and the population.

| | |
|---|---|
| Category | [mineral](index.md#mineral) |
| Canonical unit | µg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in µg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 30 | 181.65 | reference-interval-study | Healthy adults, MASHAD cohort (Iran), total population; CLSI EP28 2.5-97.5 percentile limits | [Age- and sex-specific reference intervals for superoxide dismutase enzyme and several minerals in a healthy adult cohort.](https://pubmed.ncbi.nlm.nih.gov/34273186/), Results (reference interval table, total population); [Age- and sex-specific reference intervals for superoxide dismutase enzyme and several minerals in a healthy adult cohort.](https://pubmed.ncbi.nlm.nih.gov/34273186/), Abstract (Results) | Serum copper reference limits from a healthy Iranian adult cohort. The source found that sex partitioning was not required for copper (only magnesium was partitioned), so one interval is given. Limits are assay-specific. No pregnancy or oral-contraceptive interval was found in the sources checked. |
| diagnostics-2023-table | reference | all | 70 | 155 | consensus | Adults in general, as tabulated for serum total copper by atomic absorption spectroscopy in a review of Wilson disease copper monitoring; age and sex are not split in the table | [Monitoring of Copper in Wilson Disease.](https://pubmed.ncbi.nlm.nih.gov/37296680/), Table (serum total copper row) | Tabulated laboratory interval for serum total copper (AAS). No sex or pregnancy split is given in the source. |
| epic-serum-copper-2021 | reference | all | 89.78 | 190.6 | reference-interval-study | Healthy subjects from the cross-sectional EPIC study (European cohort), 2.5th-97.5th percentiles of total serum copper; sex and age are not split in the quote | [Relation of Serum Copper Status to Survival in COVID-19.](https://pubmed.ncbi.nlm.nih.gov/34072977/), Section 3.2 (Cu status of COVID-19 patients) | Reported in µg/L in the source (897.8-1906.0 µg/L), converted to µg/dL. Its upper limit is higher than the 70-155 µg/dL tabulated interval in the other range; the two sources differ. |
| pediatric-ri-us | reference | age 0.5-10.3 | 75 | 153 | reference-interval-study | Clinically healthy US children aged 0.5-18 y (n=2115), serum, ICP-MS; this band is children younger than 10.3 y | [Pediatric reference intervals for serum copper and zinc.](https://pubmed.ncbi.nlm.nih.gov/22192859/), Abstract, Results | Central 95% interval. Age-dependent, no sex difference reported in this study. |
| pediatric-ri-us | reference | age 10.3-12.5 | 64 | 132 | reference-interval-study | Clinically healthy US children aged 10.3-12.5 y (n=2115 children, 0.5-18 y) | [Pediatric reference intervals for serum copper and zinc.](https://pubmed.ncbi.nlm.nih.gov/22192859/), Abstract, Results | Central 95% interval, ICP-MS. |
| pediatric-ri-us | reference | age 12.5-18 | 57 | 129 | reference-interval-study | Clinically healthy US adolescents older than 12.5 y (n=2115 children, 0.5-18 y) | [Pediatric reference intervals for serum copper and zinc.](https://pubmed.ncbi.nlm.nih.gov/22192859/), Abstract, Results | Central 95% interval, ICP-MS. Only pediatric data in this study; no adult serum copper interval was retrieved. |
| wilson-diagnostic-2025 | warn | all | 59.1 | - | cohort | Patients with suspected Wilson disease in Spanish and Danish cohorts, newly diagnosed WD versus non-Wilsonian hepatic disease and acute liver failure | [Relative Exchangeable Copper, Exchangeable Copper and Total Copper in the Diagnosis of Wilson Disease.](https://pubmed.ncbi.nlm.nih.gov/40198317/), Discussion | ROC cut-off for total serum copper (AUC 0.966, sensitivity 86.5%, specificity 95.6% in the combined cohort). Not a general reference interval; the paper reports the same cut-off as 59.1 µg/dL. Discrimination was lower in acute liver failure cases. Source kind: diagnostic. Source band <= 59.1; stored as the warn low bound 59.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
