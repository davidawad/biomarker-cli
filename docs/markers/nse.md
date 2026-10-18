# Neuron-Specific Enolase (NSE) (`nse`)

Neuron-specific enolase, a glycolytic enzyme measured in serum by immunoassay. Tracked as a tumour marker for small cell lung cancer and neuroendocrine tumours and as a marker of neuronal damage. Expected result in healthy people: at or below the reference upper limit.

| | |
|---|---|
| Category | [tumor-marker](index.md#tumor-marker) |
| Canonical unit | ng/mL |
| LOINC | [15060-7](https://loinc.org/15060-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 20-80 | - | 25.4 | reference-interval-study | Healthy Chinese adults aged 20-79 years (n = 10,575), Cobas e602 NSE assay; no difference by sex or age | [Distribution of serum neuron-specific enolase and the establishment of a population reference interval in healthy adults.](https://pubmed.ncbi.nlm.nih.gov/30779465/), Methods and Results | Population upper reference limit (unilateral 95th percentile) stated by the source. |
| clinical | reference | male | - | 17.26 | reference-interval-study | Healthy persons from routine health check-up, Chengdu, China (n = 3,293 for NSE); indirect method; no correlation with age | [Establishment of Reference Intervals for Serum Cytokeratin-19 Fragment and Neuron Specific Enolase by Indirect Method Using Data Obtained from Healthy Chinese Population in Chengdu.](https://pubmed.ncbi.nlm.nih.gov/31414742/), Results | Indirect 95th percentile interval, males. Source states the lower limit as 0. |
| clinical | reference | female | - | 16.29 | reference-interval-study | Healthy persons from routine health check-up, Chengdu, China (n = 3,293 for NSE); indirect method; no correlation with age | [Establishment of Reference Intervals for Serum Cytokeratin-19 Fragment and Neuron Specific Enolase by Indirect Method Using Data Obtained from Healthy Chinese Population in Chengdu.](https://pubmed.ncbi.nlm.nih.gov/31414742/), Results | Indirect 95th percentile interval, females. Source states the lower limit as 0. |
| clinical | reference | age 0-6 | - | 36.9 | reference-interval-study | Danish children aged 0.1-17.9 years from residual serum of routine allergy testing (n = 242), Roche Elecsys NSE assay, samples with haemoglobin <10 mg/dL | [Serum neuron-specific enolase - reference interval in Danish children and the impact of preanalytical factors.](https://pubmed.ncbi.nlm.nih.gov/41035379/), Results | Age-partitioned paediatric upper limit, 95th percentile one-sided, age 0-5 years. Source unit ug/L. Converted from the source's ug/L (36.9) to ng/mL using factor 1.0. |
| clinical | reference | age 6-18 | - | 32 | reference-interval-study | Danish children aged 0.1-17.9 years from residual serum of routine allergy testing (n = 242), Roche Elecsys NSE assay, samples with haemoglobin <10 mg/dL | [Serum neuron-specific enolase - reference interval in Danish children and the impact of preanalytical factors.](https://pubmed.ncbi.nlm.nih.gov/41035379/), Results | Age-partitioned paediatric upper limit, 95th percentile one-sided, age 6-17 years. Source unit ug/L. Converted from the source's ug/L (32.0) to ng/mL using factor 1.0. |
| diagnostic | warn | non-small-cell-lung-cancer | - | 20.5 | cohort | Patients with NSCLC in a serum NSE study of 417 lung cancer patients (ROC analysis for SCLC vs NSCLC) | [Cut-off levels of NSE to differentiate SCLC from NSCLC.](https://pubmed.ncbi.nlm.nih.gov/11956631/), Abstract | Strictly above 20.5 ng/mL in NSCLC: source says pathological features must be examined for neuroendocrine differentiation. Source kind: diagnostic. Source band >= 20.5; stored as the warn high bound 20.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
