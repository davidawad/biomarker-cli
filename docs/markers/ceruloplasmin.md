# Ceruloplasmin (`ceruloplasmin`)

Serum ceruloplasmin, the main copper-carrying protein. It is the first-line biochemical marker for Wilson disease, and it is also an acute-phase reactant that rises in inflammation and pregnancy and falls in liver disease. Interpret with copper, urinary copper and clinical findings, not alone.

| | |
|---|---|
| Category | [mineral](index.md#mineral) |
| Canonical unit | mg/dL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mg/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 20 | 40 | consensus | Adults (general, unspecified) | [Challenges and dilemmas in pediatric hepatic Wilson's disease.](https://pubmed.ncbi.nlm.nih.gov/37970614/), Diagnosis section (text) | Normal serum ceruloplasmin as stated in a narrative review of Wilson disease diagnosis. Assay-dependent; not a measured reference-interval study. A value of 20-40 mg/dL does not exclude Wilson disease. |
| clinical | warn | suspected-wilson-disease | 10 | - | consensus | Patients with suspected Wilson disease | [Challenges and dilemmas in pediatric hepatic Wilson's disease.](https://pubmed.ncbi.nlm.nih.gov/37970614/), Table (diagnostic tests, cut-off values) | Intermediate zone: 10 to under 20 mg/dL needs further evaluation (cited review's diagnostic table). Source band 10-20; stored as the warn low bound 10. |
| alt-1 | warn | suspected-wilson-disease | 10 | - | consensus | Patients with suspected Wilson disease | [Challenges and dilemmas in pediatric hepatic Wilson's disease.](https://pubmed.ncbi.nlm.nih.gov/37970614/), Table (diagnostic tests, cut-off values) | Strong evidence for Wilson disease: strictly below 10 mg/dL in the cited review's diagnostic table. Source kind: diagnostic. Source band <= 10; stored as the warn low bound 10. Alternative to the default diagnostic warn range for the same population. |
| alt-1 | warn | wilson-disease | 10 | - | consensus | Patients with Wilson disease (on average, compared with healthy subjects and other liver disease) | [Wilson disease and the differential diagnosis of its hepatic manifestations: a narrative review of clinical, laboratory, and liver histological features.](https://pubmed.ncbi.nlm.nih.gov/34733946/), Serum ceruloplasmin levels (text) | Source states the mean ceruloplasmin in Wilson disease is significantly lower than 0.1 g/L (1 mg/dL is 0.01 g/L; 0.1 g/L = 10 mg/dL). This is a group average with overlap with other liver conditions, not a guideline cut-off. Source unit is g/L, kept as stated. Converted from the source's g/L (0.1) to mg/dL using factor 100. Source kind: diagnostic. Source band <= 10.0; stored as the warn low bound 10.0. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | suspected-wilson-disease | 20 | - | consensus | Patients with suspected Wilson disease | [Wilson disease and the differential diagnosis of its hepatic manifestations: a narrative review of clinical, laboratory, and liver histological features.](https://pubmed.ncbi.nlm.nih.gov/34733946/), Serum ceruloplasmin levels (text); [Challenges and dilemmas in pediatric hepatic Wilson's disease.](https://pubmed.ncbi.nlm.nih.gov/37970614/), Diagnosis section (text) | Strictly below 20 mg/dL is considered diagnostic of Wilson disease in the cited narrative review; the second source says below 20 mg/dL with a Kayser-Fleischer ring is consistent with Wilson disease. Laboratory-dependent cut-off. Source kind: diagnostic. Source band <= 20; stored as the warn low bound 20. |
| diagnostics-2023-table | reference | all | 25 | 45 | consensus | Not stated in the source table; tabulated nephelometric ceruloplasmin interval in a review of Wilson disease monitoring | [Monitoring of Copper in Wilson Disease.](https://pubmed.ncbi.nlm.nih.gov/37296680/), Table (ceruloplasmin row) | Interval for the nephelometric assay as tabulated; no age or sex split. Source notes low values also occur in pregnancy and with oral contraceptive use, and high values with inflammation; the table does not give a number for those groups. |
| wilson-diagnostic-criterion | warn | all | 20 | - | consensus | Suspected Wilson disease; conventional diagnostic criterion as stated by the authors of a genotype-verified ROC study | [Diagnostic accuracy of serum ceruloplasmin in Wilson disease: determination of sensitivity and specificity by ROC curve analysis among ATP7B-genotyped subjects.](https://pubmed.ncbi.nlm.nih.gov/18556333/), Abstract, Background | Conventional criterion. The guideline (EASL/AASLD) figure could not be retrieved in open text, so this is cited from the 2008 ROC study that describes it as conventional. Source kind: diagnostic. Source band <= 20; stored as the warn low bound 20. |
| wilson-roc-2008 | warn | all | 14 | - | cohort | ATP7B-genotyped Wilson disease patients, carriers and wild-type relatives, a validation group, and 690 normal individuals (single nephelometric method) | [Diagnostic accuracy of serum ceruloplasmin in Wilson disease: determination of sensitivity and specificity by ROC curve analysis among ATP7B-genotyped subjects.](https://pubmed.ncbi.nlm.nih.gov/18556333/), Abstract, Conclusions | Local decision threshold from ROC analysis: at 0.14 g/L PPV was 100% and NPV 97.1% in this series. The authors advise each laboratory to set its own threshold. Source kind: diagnostic. Source band <= 14; stored as the warn low bound 14. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
