# GAD65 Autoantibodies (`gad65-antibodies`)

Autoantibodies against glutamic acid decarboxylase 65 (GAD65), measured by radioimmunoassay, ELISA or chemiluminescence immunoassay. Used to support a diagnosis of type 1 diabetes (including latent autoimmune diabetes in adults) and to evaluate GAD antibody-spectrum neurological syndromes. Results are assay-specific. Expected result: negative (below the upper reference limit).

| | |
|---|---|
| Category | [autoimmune](index.md#autoimmune) |
| Canonical unit | IU/mL |
| LOINC | [13926-1](https://loinc.org/13926-1/) |
| Specimen | - |
| Aliases | - |
| Panels | [Diabetes and glycemic control](../panels/diabetes.md) |

## Ranges

Bounds in IU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 5.1 | reference-interval-study | Healthy Danish children (0-18 years) and adults (18-70 years) (method: maglumi) | [Reference limits for GAD65 and IA-2 autoantibodies by chemiluminescence immunoassay in Northern European adults and children.](https://pubmed.ncbi.nlm.nih.gov/30727763/), Abstract | Upper reference limit for GAD65 antibodies on the MAGLUMI 800 chemiluminescence assay, established per CLSI guidance. The study found a substantial discrepancy from the manufacturer's limit. |
| alt-1 | warn | gad-antibody-neurological-disorder | - | 10000 | cohort | Patients with suspected GAD antibody-spectrum neurological disease, serum, tested on GAD65 ELISA (method: elisa; specimen: serum) | [GAD65 Antibody ELISA With Extended Reportable Range: Validation and Guidance for Neurological Practice.](https://pubmed.ncbi.nlm.nih.gov/41902651/), Results | Neurological serum cut-off for the GAD65 ELISA (single validation study, correlated with RIA neurological cut-off). The study states this value is informative but requires clinical context. Source kind: diagnostic. Source band >= 10000; stored as the warn high bound 10000. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | gad-antibody-neurological-disorder | - | 6778 | cohort | Patients with suspected autoimmune diabetes or GAD antibody-spectrum neurological syndromes (n = 90), tested on Maglumi and RIA (method: maglumi) | [Clinical relevance of the Maglumi immunoassay for anti-GAD65 in diabetes and neurological disorders: Analytical challenges and diagnostic insights.](https://pubmed.ncbi.nlm.nih.gov/41076178/), Abstract (Results) | Maglumi-specific cut-off (ROC analysis, single study) separating neurological from diabetic patients; values at or above this cut-off indicate the neurological group. Source kind: diagnostic. Source band >= 6778; stored as the warn high bound 6778. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
