# von Willebrand factor antigen (VWF:Ag) (`von-willebrand-antigen`)

Quantity of von Willebrand factor protein in plasma, measured by immunoassay. Low levels suggest von Willebrand disease or acquired von Willebrand syndrome; levels are raised by inflammation, pregnancy and stress, and depend on ABO blood group.

| | |
|---|---|
| Category | [coagulation](index.md#coagulation) |
| Canonical unit | IU/dL |
| LOINC | [27816-7](https://loinc.org/27816-7/) |
| Specimen | - |
| Aliases | `vwf-antigen` |

## Ranges

Bounds in IU/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | blood-group-o | 36 | 157 | consensus | Healthy individuals of blood group O (large cohort of 1117 healthy individuals, as summarised in a 2007 review) | [Relationship between ABO blood group and von Willebrand factor levels: from biology to clinical implications.](https://pubmed.ncbi.nlm.nih.gov/17894864/), ABO and VWF:Ag normal range section, full text | Mean +/- 2SD range for blood group O, quoted from a review that cites the Gill et al. cohort; the source states the group O lower limit extends below 50 IU/dL, so the 50 IU/dL cutoff used for all groups is not appropriate for group O. Not a pooled adult reference interval. |
| alt-1 | warn | all | 50 | - | consensus | Adults and children with low VWF levels suspected of von Willebrand disease (as summarised in a 2017 review) | [Diagnosis and Treatment of von Willebrand Disease and Rare Bleeding Disorders.](https://pubmed.ncbi.nlm.nih.gov/28394285/), Full text, type 1 VWD paragraph | Described as 'low VWF' with a mild bleeding risk, as attributed to guidelines by the review. Not a diagnostic threshold. Source band 30-50; stored as the warn low bound 50. Alternative to the default vwd-guidelines-as-cited-jcm-2017 warn range for the same population. |
| vwd-guidelines-as-cited-jcm-2017 | warn | all | 30 | - | consensus | Adults and children suspected of von Willebrand disease (as summarised in a 2017 review of VWD diagnosis and treatment) | [Diagnosis and Treatment of von Willebrand Disease and Rare Bleeding Disorders.](https://pubmed.ncbi.nlm.nih.gov/28394285/), Full text, type 1 VWD paragraph | Threshold attributed by the review to 'several guidelines' (its ref 16); the 2021 ASH/ISTH/NHF/WFH guideline text itself was not retrievable, and the 2025 monitoring report full text contains no numeric cutoffs. Values below this level are diagnostic of VWD. Source kind: diagnostic. Source band <= 30; stored as the warn low bound 30. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
