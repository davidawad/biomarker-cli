# AST/ALT Ratio (De Ritis) (`ast-alt-ratio`)

Ratio of aspartate aminotransferase to alanine aminotransferase (AST/ALT), the De Ritis ratio. Used to help separate alcohol-related liver injury from nonalcoholic steatohepatitis and to flag possible cirrhosis. Expected result in a healthy adult is about 0.7 to 2.1; values below 1 suggest NASH and values of 2 or above suggest alcoholic liver disease.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | ratio |
| LOINC | [1916-6](https://loinc.org/1916-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.7 | 2.1 | reference-interval-study | Healthy adults, Gandaki Province, Nepal (combined sexes) | [Establishment of dry-chemistry-based reference intervals of routine liver function tests for the adult population of Gandaki Province, Nepal.](https://pubmed.ncbi.nlm.nih.gov/38722987/), Abstract | Combined 95% reference interval for AST/ALT ratio in healthy adults. |
| alt-1 | warn | all | - | 2 | cohort | Adults with NASH or alcoholic liver disease (matched case-control study) | [The ratio of aspartate aminotransferase to alanine aminotransferase: potential value in differentiating nonalcoholic steatohepatitis from alcoholic liver disease.](https://pubmed.ncbi.nlm.nih.gov/10201476/), Abstract (Conclusions) | 2 or above: strongly suggestive of alcoholic liver disease. Source kind: diagnostic. Source band >= 2; stored as the warn high bound 2. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | warn | all | - | 1 | cohort | Adults with biopsy-staged chronic nonalcoholic liver disease (single study) | [Ratio of serum aspartate to alanine aminotransferase in chronic hepatitis. Relationship to cirrhosis.](https://pubmed.ncbi.nlm.nih.gov/3135226/), Abstract | Strictly above 1.0 in nonalcoholic liver disease: suggests cirrhosis. The source notes the ratio is less helpful in the presence of cirrhosis. Not validated for other etiologies. Source kind: diagnostic. Source band >= 1.0; stored as the warn high bound 1.0. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | 1 | - | cohort | Adults with NASH or alcoholic liver disease (matched case-control study) | [The ratio of aspartate aminotransferase to alanine aminotransferase: potential value in differentiating nonalcoholic steatohepatitis from alcoholic liver disease.](https://pubmed.ncbi.nlm.nih.gov/10201476/), Abstract (Conclusions) | Below 1: pattern suggesting NASH rather than alcoholic liver disease. Discriminates between the two; not a standalone diagnosis. Source kind: diagnostic. Source band <= 1; stored as the warn low bound 1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
