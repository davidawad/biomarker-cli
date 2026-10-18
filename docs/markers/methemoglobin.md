# Methemoglobin (`methemoglobin`)

Percentage of hemoglobin in the oxidized form (methemoglobin, MetHb), which cannot carry oxygen. Measured to evaluate methemoglobinemia (acquired, from drugs or toxins, or inherited) and unexplained cyanosis or low pulse-oximeter readings. Expected result in healthy people: within the reference interval for age; children run higher than adults.

| | |
|---|---|
| Category | [hematology](index.md#hematology) |
| Canonical unit | % |
| LOINC | [2614-6](https://loinc.org/2614-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 6-11 | 3.61 | 6.44 | reference-interval-study | Clinically healthy children aged 6-10 years (n = 78); reference values from the 2.5th and 97.5th percentiles; spectrophotometric method | [Reference values for methemoglobin concentrations in children.](https://pubmed.ncbi.nlm.nih.gov/23049377/), Table 1 (children, Methemoglobin row); Methods (2.5 and 97.5 percentiles) | Pediatric reference interval (2.5th-97.5th percentile). Sample size (n = 78) is below the usual 120 minimum for a 95% reference interval; treat as provisional. The adult values in the same paper (n = 15, 2.51-3.90%) are not stored: too small a sample. |
| clinical | critical | all | - | 70 | consensus | Patients with methemoglobinemia, all ages | [Recommendations for diagnosis and treatment of methemoglobinemia.](https://pubmed.ncbi.nlm.nih.gov/34467556/), Section on management in infancy and childhood (PMC full text) | MetHb strictly above 70% is described as potentially lethal (Delphi-like expert consensus). Table 1 of the same paper lists >70% as severe hypoxemia, death. Source band >= 70; stored as the critical high bound 70. |
| diagnostic | warn | acquired-methemoglobinemia | - | 20 | consensus | Patients with acquired methemoglobinemia | [Recommendations for diagnosis and treatment of methemoglobinemia.](https://pubmed.ncbi.nlm.nih.gov/34467556/), Section on management in infancy and childhood (PMC full text) | Treatment-indication threshold (decision limit, not a diagnostic criterion) for symptomatic patients. The same sentence gives 30% for asymptomatic patients, which is not stored as a separate range. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
