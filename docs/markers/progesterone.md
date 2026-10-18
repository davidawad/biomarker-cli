# Progesterone (`progesterone`)

Steroid produced mainly by the corpus luteum after ovulation and by the placenta in pregnancy. Used to judge cycle phase and ovulation, and as a single-value marker of early pregnancy viability. Values are cycle-phase and assay dependent.

| | |
|---|---|
| Category | [hormone](index.md#hormone) |
| Canonical unit | ng/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | - | 0.86 | reference-interval-study | healthy men (n = 220, aged 20-68 years) measured on Advia Centaur, CLSI EP28-A3c method (method: immunoassay) | [Establishing reference intervals for sex hormones on the analytical platforms Advia Centaur and Immulite 2000XP.](https://pubmed.ncbi.nlm.nih.gov/26522760/), Table 1 | Upper reference limit (Table 1, Advia Centaur). Lower limit not reported for this group. |
| alt-1 | reference | female, post-menopausal | - | 0.6 | reference-interval-study | postmenopausal women (n = 103; amenorrhea for at least 12 months) measured on Advia Centaur, CLSI EP28-A3c method (method: immunoassay) | [Establishing reference intervals for sex hormones on the analytical platforms Advia Centaur and Immulite 2000XP.](https://pubmed.ncbi.nlm.nih.gov/26522760/), Table 1 | Upper reference limit in postmenopausal women (Table 1). Lower limit not reported. Alternative to the default clsi-ep28-advia-2016 reference range for the same population. |
| alt-1 | warn | pregnant, trimester 1, symptomatic-first-trimester | 6.3 | - | cohort | symptomatic first-trimester pregnant patients (pain or bleeding); 15,878 patients in the pooled studies | [Performance of single serum progesterone in the evaluation of symptomatic first-trimester pregnant patients: a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/35921048/), Abstract, Results | Single serum progesterone below about 6.3 ng/mL (about 20 nmol/L) makes a non-viable pregnancy likely (specificity 99.2% in the review). Pregnancy-viability decision value, not a reference interval. Source kind: diagnostic. Source band <= 6.3; stored as the warn low bound 6.3. Alternative to the default viability-meta-2022 warn range for the same population. |
| clsi-ep28-advia-2016 | reference | female, luteal phase | 1.14 | 20.6 | reference-interval-study | women in the luteal phase (n = 87; days 21-23 after menses onset) measured on Advia Centaur, CLSI EP28-A3c method (method: immunoassay) | [Establishing reference intervals for sex hormones on the analytical platforms Advia Centaur and Immulite 2000XP.](https://pubmed.ncbi.nlm.nih.gov/26522760/), Table 1 | Luteal-phase reference interval (Table 1), 2.5th-97.5th percentile as reported. Wide interval; a single value cannot confirm ovulation or cycle day without timing. Not an ovulation-confirmation threshold. |
| progesterone-elecsys-natural-cycle-2021 | reference | female, luteal phase | 4.12 | 14.56 | reference-interval-study | 85 apparently healthy women over one standardised natural menstrual cycle (US and EU), Elecsys Progesterone III immunoassay (method: immunoassay) | [Extensive monitoring of the natural menstrual cycle using the serum biomarkers estradiol, luteinizing hormone and progesterone.](https://pubmed.ncbi.nlm.nih.gov/33869706/), Abstract, Results | 5th percentile for the whole luteal phase, not a mid-luteal-only threshold; the 95th percentile is 46.3 nmol/L. A value at or above 13.1 nmol/L is consistent with luteal activity after ovulation, but one value does not confirm ovulation or cycle day. Source unit nmol/L, not converted. Phase order in the source: follicular, ovulation, luteal. Converted from the source's nmol/L (13.1-46.3) to ng/mL using factor 0.3145. |
| viability-meta-2022 | warn | pregnant, trimester 1, symptomatic-first-trimester | - | 20 | cohort | symptomatic first-trimester pregnant patients (pain or bleeding); 15,878 patients in the pooled studies | [Performance of single serum progesterone in the evaluation of symptomatic first-trimester pregnant patients: a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/35921048/), Abstract, Results | Single serum progesterone at or above about 20 ng/mL (about 63.6 nmol/L) makes a viable pregnancy likely (NPV 0.89-0.99 across the prevalences reported in the review). Pregnancy-viability decision value, not a reference interval. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
