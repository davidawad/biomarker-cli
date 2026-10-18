# Immunoglobulin E, Total (`ige-total`)

Total IgE antibody in blood, raised in allergic and some parasitic or fungal conditions. Interpretation depends on age and assay, and very high levels are used as a criterion in allergic bronchopulmonary aspergillosis.

| | |
|---|---|
| Category | [immune](index.md#immune) |
| Canonical unit | kU/L |
| LOINC | [19113-0](https://loinc.org/19113-0/) |
| Specimen | - |
| Aliases | `total ige`, `ige, total serum` |

## Ranges

Bounds in kU/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 302 | reference-interval-study | Healthy Norwegian adults recruited through a regional blood donation programme (n=252), ImmunoCAP assay | [Updated total IgE reference intervals in Norwegian adults.](https://pubmed.ncbi.nlm.nih.gov/36705407/), Abstract, Results | Upper 95% reference limit only (CLSI-based). The authors report that older manufacturer intervals (upper limit about 114 kU/L) are substantially lower, so check the assay's own interval. |
| abpa-review-2026 | warn | all | - | 500 | consensus | Patients with asthma evaluated for allergic bronchopulmonary aspergillosis (ABPA) | [Asthma and Allergic Bronchopulmonary Aspergillosis: Understanding, Insights, and State-of-the-Art.](https://pubmed.ncbi.nlm.nih.gov/41835114/), Full text, ABPA diagnosis section | Diagnostic criterion for ABPA: total IgE at or above 500 IU/mL (rule-in, together with fungal sensitisation and other features). The same review also says a level above 500 IU/mL is considered positive, and that lower levels may be acceptable with other supportive features. Cited as a summary of guideline criteria; the primary ISHAM document was not retrieved. Converted from the source's IU/mL (500) to kU/L using factor 1. Source kind: decision. Source band >= 500.0; stored as the warn high bound 500.0. |
| japanese-abpa-criteria-cited-2026 | warn | all | - | 417 | cohort | Asthma patients with allergic bronchopulmonary mycosis (ABPA/ABPM) by Japanese diagnostic criteria | [High total serum IgE level at diagnosis was associated with a progressive decline in lung function in asthmatic patients with allergic bronchopulmonary mycosis.](https://pubmed.ncbi.nlm.nih.gov/41796390/), Full text, Methods | Japanese diagnostic criterion for ABPA/ABPM: elevated total serum IgE at or above 417 IU/mL, one of ten criteria (at least six needed). The number is the study's disease-specific cut-off. Converted from the source's IU/mL (417) to kU/L using factor 1. Source kind: decision. Source band >= 417.0; stored as the warn high bound 417.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
