# Methylmalonic acid (MMA) (`methylmalonic-acid`)

Serum methylmalonic acid (MMA) rises when vitamin B12 is functionally short, because B12 is a cofactor for methylmalonyl-CoA mutase. It is a more specific functional marker of B12 status than total B12, but it also rises with reduced kidney function and with age, so results need renal and age context.

| | |
|---|---|
| Category | [organic-acid](index.md#organic-acid) |
| Canonical unit | nmol/L |
| LOINC | [13964-2](https://loinc.org/13964-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in nmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 20-86 | 80 | 280 | reference-interval-study | 126 women (ages 20-85, median 49) and 109 men (ages 20-84, median 50) with plasma MMA measured; central 95% interval (specimen: plasma) | [Age- and gender-specific reference intervals for total homocysteine and methylmalonic acid in plasma before and after vitamin supplementation.](https://pubmed.ncbi.nlm.nih.gov/8605683/), Abstract, Results | Small early-period sample, before folic acid or cyanocobalamin supplementation. Age and sex bands were not separated for MMA in the quoted abstract. Assay not named in the abstract, so check against the lab's method. |
| clarke-2003 | warn | age 65+ | - | 350 | cohort | Older persons aged 65 and over in Oxford (population study, n=1562); used to flag high risk of B12 deficiency among borderline B12 results | [Screening for vitamin B-12 and folate deficiency in older persons.](https://pubmed.ncbi.nlm.nih.gov/12716678/), Design, para 1 (Abstract) | Elevated-MMA cut-off used in the study's high-risk classification, not a stand-alone diagnostic threshold. Published MMA cut-offs range from about 210 to 480 nmol/L across studies (NHANES roundtable). Frequency of abnormal MMA falls as the cut-off rises (Hordaland data, 71-74 y: 17.7% above 260 nmol/L, 5.2% above 370 nmol/L). Renal impairment can raise MMA independently of B12. Source band >= 350; stored as the warn high bound 350. |
| nhanes-1999-2004 | reference | all | 69.6 | 297 | reference-interval-study | US population sample in NHANES 1999-2004 (all subjects in Table 2, n=18,569; not restricted to healthy people) (specimen: serum) | [Population Reference Values for Serum Methylmalonic Acid Concentrations and Its Relationship with Age, Sex, Race-Ethnicity, Supplement Use, Kidney Function and Serum Vitamin B12 in the Post-Folic Acid Fortification Period.](https://pubmed.ncbi.nlm.nih.gov/29329201/), Table 2, All subjects row (full text) | Table 2 shows 5th and 95th percentiles for all subjects. Geometric mean 121.0 nmol/L at age 21-30 vs 196.9 nmol/L over 70 (abstract). Serum MMA starts to rise at about 18-20 years (abstract). Higher MMA with impaired kidney function and lower B12, and higher values in non-Hispanic white than black adults. Fasting time also affects MMA (2019 NHANES 2011-2014 analysis). |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
