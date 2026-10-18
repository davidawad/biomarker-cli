# Immature Platelet Fraction (`immature-platelet-fraction`)

Percentage of circulating platelets that are immature (reticulated), reported by automated haematology analysers as IPF. Reflects platelet production in the marrow. Raised values suggest consumptive or recovering thrombocytopenia; low values suggest marrow failure. Expected result in healthy adults: within the reference interval (roughly 1-9 percent, depending on analyser).

| | |
|---|---|
| Category | [hematology](index.md#hematology) |
| Canonical unit | % |
| LOINC | [71693-6](https://loinc.org/71693-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 1.6 | 9.6 | reference-interval-study | 153 healthy Caucasian adults from Spain, Sysmex XN-2000 analyser, non-parametric percentile method per CLSI; no significant sex difference | [Reference interval for immature platelet fraction on Sysmex XN haematology analyser in adult population.](https://pubmed.ncbi.nlm.nih.gov/29472803/), Abstract, Results | Analyser-specific (Sysmex XN-2000) reference interval; 153 adults. |
| clinical | reference | male | 0.5 | 3.1 | reference-interval-study | Males from 2152 healthy individuals (1252 men, 900 women), Sysmex XE-2100 analyser, non-parametric percentile method per CLSI | [Establishment of reference interval for immature platelet fraction.](https://pubmed.ncbi.nlm.nih.gov/23286350/), Abstract, Results | Analyser-specific (Sysmex XE-2100) reference interval for males. Source quotes total 0.5-3.3%, men 0.5-3.1%, women 0.5-3.4%. |
| clinical | reference | female | 0.5 | 3.4 | reference-interval-study | Females from 2152 healthy individuals (1252 men, 900 women), Sysmex XE-2100 analyser, non-parametric percentile method per CLSI | [Establishment of reference interval for immature platelet fraction.](https://pubmed.ncbi.nlm.nih.gov/23286350/), Abstract, Results | Analyser-specific (Sysmex XE-2100) reference interval for females. Source quotes total 0.5-3.3%, men 0.5-3.1%, women 0.5-3.4%. |
| alt-1 | reference | all | 1 | 7.3 | reference-interval-study | 2104 healthy individuals analysed on Sysmex XN (non-parametric percentile method per CLSI C28-A3); sexes not separated in the quoted interval | [Reference interval for immature platelet fraction on Sysmex XN hematology analyzer: a comparison study with Sysmex XE-2100.](https://pubmed.ncbi.nlm.nih.gov/25460288/), Abstract, Results | Analyser-specific (Sysmex XN) reference interval from a comparison study with the XE-2100. Alternative to the default clinical reference range for the same population. |
| alt-1 | reference | male | 0.5 | 3.2 | reference-interval-study | Healthy males, 2039 individuals (1161 males, 878 females) including children, 95% confidence interval reference range | [Immature platelet fraction: establishment of a reference interval and diagnostic measure for thrombocytopenia.](https://pubmed.ncbi.nlm.nih.gov/20890075/), Abstract, Results | Reference interval for males from the 2010 study (analyser not named in the quoted sentence). Alternative to the default clinical reference range for the same population. |
| alt-1 | reference | female | 0.4 | 3 | reference-interval-study | Healthy females, 2039 individuals (1161 males, 878 females) including children, 95% confidence interval reference range | [Immature platelet fraction: establishment of a reference interval and diagnostic measure for thrombocytopenia.](https://pubmed.ncbi.nlm.nih.gov/20890075/), Abstract, Results | Reference interval for females from the 2010 study (analyser not named in the quoted sentence). Alternative to the default clinical reference range for the same population. |
| alt-1 | warn | dengue | - | 10.55 | cohort | Dengue patients with thrombocytopenia (platelet count below 50x10^9/L), 54 patients | [Estimation of a cut-off value for immature platelet fraction (IPF) in predicting platelet recovery in dengue patients with thrombocytopenia.](https://pubmed.ncbi.nlm.nih.gov/36591717/), Abstract, Results | Prognostic ROC cut-off for predicting platelet recovery in dengue (sensitivity 69%, specificity 67%). Single small study; a prognostic threshold, not a diagnostic one. Source kind: diagnostic. Source band >= 10.55; stored as the warn high bound 10.55. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | reference | all | 1.3 | 9 | reference-interval-study | 1674 apparently healthy individuals (910 females, 764 males) from the Danish General Suburban Population Study, Sysmex XE-5000 analyser, non-parametric method; no significant age, sex or smoking difference | [Reference intervals for mean platelet volume and immature platelet fraction determined on a sysmex XE5000 hematology analyzer.](https://pubmed.ncbi.nlm.nih.gov/26853453/), Abstract, Results | Analyser-specific (Sysmex XE-5000) reference interval. Alternative to the default clinical reference range for the same population. |
| diagnostic | warn | immune-thrombocytopenia | - | 7.3 | cohort | Patients with immune thrombocytopenic purpura (N=150) and aplastic anaemia (N=51) with platelet counts below 100x10(9)/L | [Immature platelet fraction: establishment of a reference interval and diagnostic measure for thrombocytopenia.](https://pubmed.ncbi.nlm.nih.gov/20890075/), Abstract, Results | ROC cut-off for differentiating ITP from aplastic anaemia (sensitivity 54.0%, specificity 92.2%). Single-study classification, not a guideline cut-off. Source kind: diagnostic. Source band >= 7.3; stored as the warn high bound 7.3. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
