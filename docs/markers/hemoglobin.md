# Hemoglobin (`hemoglobin`)

Concentration of the oxygen-carrying protein hemoglobin in whole blood. Low values define anemia; high values suggest polycythemia, dehydration, smoking or altitude.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | g/dL |
| LOINC | [718-7](https://loinc.org/718-7/) |
| Specimen | - |
| Aliases | `hgb`, `hb`, `haemoglobin` |

## Unit conversions

Into g/dL: `value * factor + offset`.

| From | Factor | Offset |
|---|---|---|
| mmol/L | 1.611 | 0 |

## Ranges

Bounds in g/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 13 | 17.7 | - | - | uncited (legacy) | - |
| clinical | reference | female | 11.1 | 15.9 | - | - | uncited (legacy) | - |
| clinical | reference | all | 11.1 | 17.7 | - | - | uncited (legacy) | - |
| braat-2024-fifth-centile | warn | female, age 18-66, not pregnant | 11.97 | - | reference-interval-study | Pooled healthy non-pregnant females aged 18-65 (n=3640) from international population surveys | [Haemoglobin thresholds to define anaemia from age 6 months to 65 years: estimates from international data sources.](https://pubmed.ncbi.nlm.nih.gov/38432242/), Abstract, Findings | Fifth centile of haemoglobin in healthy, iron-replete, non-inflamed individuals (WHO-commissioned pooled analysis informing the 2024 WHO guideline); values below suggest anaemia. Slightly differs from the long-standing WHO 1968 cutoffs. Source kind: diagnostic. Source band <= 11.97; stored as the warn low bound 11.97. |
| braat-2024-fifth-centile | warn | male, age 18-66 | 13.49 | - | reference-interval-study | Pooled healthy males aged 18-65 (n=2377) from international population surveys | [Haemoglobin thresholds to define anaemia from age 6 months to 65 years: estimates from international data sources.](https://pubmed.ncbi.nlm.nih.gov/38432242/), Abstract, Findings | Fifth centile of haemoglobin in healthy, iron-replete, non-inflamed individuals (WHO-commissioned pooled analysis informing the 2024 WHO guideline); values below suggest anaemia. Slightly differs from the long-standing WHO 1968 cutoffs. Source kind: diagnostic. Source band <= 13.49; stored as the warn low bound 13.49. |
| braat-2024-fifth-centile | warn | female, pregnant, trimester 1 | 11.03 | - | reference-interval-study | Healthy pregnant women, first trimester (n=772) | [Haemoglobin thresholds to define anaemia from age 6 months to 65 years: estimates from international data sources.](https://pubmed.ncbi.nlm.nih.gov/38432242/), Abstract, Findings | Fifth centile of haemoglobin in healthy, iron-replete, non-inflamed individuals (WHO-commissioned pooled analysis informing the 2024 WHO guideline); values below suggest anaemia. Slightly differs from the long-standing WHO 1968 cutoffs. Source kind: diagnostic. Source band <= 11.03; stored as the warn low bound 11.03. |
| braat-2024-fifth-centile | warn | female, pregnant, trimester 2 | 10.59 | - | reference-interval-study | Healthy pregnant women, second trimester (n=111) | [Haemoglobin thresholds to define anaemia from age 6 months to 65 years: estimates from international data sources.](https://pubmed.ncbi.nlm.nih.gov/38432242/), Abstract, Findings | Fifth centile of haemoglobin in healthy, iron-replete, non-inflamed individuals (WHO-commissioned pooled analysis informing the 2024 WHO guideline); values below suggest anaemia. Slightly differs from the long-standing WHO 1968 cutoffs. Source kind: diagnostic. Source band <= 10.59; stored as the warn low bound 10.59. |
| norip | reference | male, age 18-90 | 13.4 | 17 | reference-interval-study | 1826 healthy Nordic adults aged 18-90 (NORIP), venous EDTA blood, 12 analyser types | [A multicentre study of reference intervals for haemoglobin, basic blood cell counts and erythrocyte indices in the adult population of the Nordic countries.](https://pubmed.ncbi.nlm.nih.gov/15223702/), Abstract | Non-parametric 2.5th-97.5th percentiles. |
| norip | reference | female, age 18-90 | 11.7 | 15.3 | reference-interval-study | 1826 healthy Nordic adults aged 18-90 (NORIP), venous EDTA blood, 12 analyser types | [A multicentre study of reference intervals for haemoglobin, basic blood cell counts and erythrocyte indices in the adult population of the Nordic countries.](https://pubmed.ncbi.nlm.nih.gov/15223702/), Abstract | Non-parametric 2.5th-97.5th percentiles. |
| who | warn | male, age 15+ | 13 | - | guideline | males over 15 years of age (WHO anaemia definition, as quoted by BSG 2021) | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Anaemia section; [Haemoglobin thresholds to define anaemia from age 6 months to 65 years: estimates from international data sources.](https://pubmed.ncbi.nlm.nih.gov/38432242/), Figure 4 legend | Anaemia if Hb is below this value. The 2024 evidence review (Lancet Haematol) cites the same 130 g/L cutoff for males aged 15 and older. Source kind: diagnostic. Source band <= 13.0; stored as the warn low bound 13.0. |
| who | warn | female, age 15+, not pregnant | 12 | - | guideline | non-pregnant women over 15 years of age (WHO anaemia definition, as quoted by BSG 2021) | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Anaemia section; [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Criteria table, Anemia row | Anaemia if Hb is below this value. Source kind: diagnostic. Source band <= 12.0; stored as the warn low bound 12.0. |
| who | warn | female, pregnant, trimester 2 | 11 | - | guideline | pregnant women in the second trimester (WHO anaemia definition, as quoted by BSG 2021) | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Anaemia section; [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Criteria table, Anemia row | Anaemia if Hb is below this value. No first-trimester WHO value was retrieved, so no first-trimester range is given. Source kind: diagnostic. Source band <= 11.0; stored as the warn low bound 11.0. |
| who | warn | female, pregnant, trimester 3 | 11 | - | guideline | pregnant women in the third trimester (WHO anaemia definition, as quoted by BSG 2021) | [British Society of Gastroenterology guidelines for the management of iron deficiency anaemia in adults.](https://pubmed.ncbi.nlm.nih.gov/34497146/), Anaemia section; [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Criteria table, Anemia row | Anaemia if Hb is below this value. Source kind: diagnostic. Source band <= 11.0; stored as the warn low bound 11.0. |
| who | warn | age 0.5-5 | 11 | - | guideline | children aged 6-59 months (WHO anaemia definition, as quoted in a 2019 AJCN systematic review) | [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Criteria table, Anemia row | Anaemia if Hb is below this value. Age is in years (6 months = 0.5). Source kind: diagnostic. Source band <= 11.0; stored as the warn low bound 11.0. |
| who | warn | age 5-12 | 11.5 | - | guideline | children aged 5-11 years (WHO anaemia definition, as quoted in a 2019 AJCN systematic review) | [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Criteria table, Anemia row | Anaemia if Hb is below this value. Source kind: diagnostic. Source band <= 11.5; stored as the warn low bound 11.5. |
| who | warn | age 12-15 | 12 | - | guideline | children aged 12-14 years (WHO anaemia definition, as quoted in a 2019 AJCN systematic review) | [Improved micronutrient status and health outcomes in low- and middle-income countries following large-scale fortification: evidence from a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/30997493/), Criteria table, Anemia row | Anaemia if Hb is below this value. Same cutoff as the 2024 evidence review for male 12-14 year olds. Source kind: diagnostic. Source band <= 12.0; stored as the warn low bound 12.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
