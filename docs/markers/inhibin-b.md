# Inhibin B (`inhibin-b`)

Glycoprotein secreted by Sertoli cells in men and by granulosa cells of growing follicles in women. Suppresses pituitary FSH. In men it is a marker of spermatogenesis; in women it reflects early follicular activity and ovarian reserve.

| | |
|---|---|
| Category | [hormone](index.md#hormone) |
| Canonical unit | pg/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | 62.75 | - | reference-interval-study | Control men with normal total sperm number (WHO 2010), Rome, retrospective study; the 5th percentile was taken as the lower limit | [Inhibin B: are modified ranges needed for orchiectomised testicular cancer patients?](https://pubmed.ncbi.nlm.nih.gov/30531061/), Results, Table 4 | Lower limit (5th percentile) of inhibin B in normozoospermic control men. Value was derived in one centre, so treat as local rather than universal. |
| alt-1 | reference | male | 80 | 350 | reference-interval-study | Men with normal spermatogenesis, as cited by the authors for postpubertal men | [Inhibin B: are modified ranges needed for orchiectomised testicular cancer patients?](https://pubmed.ncbi.nlm.nih.gov/30531061/), Methods, Assay performance | Normal postpubertal range as stated by the authors. The source gives no percentile or sample size for this range, so it is a secondary citation. Alternative to the default inhibin-b-normozoospermic-controls-2019 reference range for the same population. |
| alt-1 | reference | female | 28 | 227 | reference-interval-study | Healthy schoolgirls aged 6-20 years at pubertal stage III (n=403 across puberty stages) | [Serum inhibin A and inhibin B in healthy prepubertal, pubertal, and adolescent girls and adult women: relation to age, stage of puberty, menstrual cycle, follicle-stimulating hormone, luteinizing hormone, and estradiol levels.](https://pubmed.ncbi.nlm.nih.gov/10770209/), Abstract, Results | 95% prediction interval at the peak of inhibin B in puberty (stage III). Median 84 pg/mL. Applies only to: tanner_min=3; tanner_max=3. Alternative to the default puberty-inhibin-b-girls-2000 reference range for the same population. |
| alt-1 | warn | male, nonobstructive-azoospermia | 45.9 | - | cohort | Subfertile Chinese men, 324 patients with different spermatogenesis status | [Clinical application value of Inhibin B alone or in combination with other hormone indicators in subfertile men with different spermatogenesis status: A study of 324 Chinese men.](https://pubmed.ncbi.nlm.nih.gov/34181290/), Abstract, Results | Cut-off to diagnose non-obstructive azoospermia versus obstructive azoospermia; values below this favour NOA. Single-centre. Source kind: diagnostic. Source band <= 45.9; stored as the warn low bound 45.9. Alternative to the default male-infertility-inhibin-b-cutoffs-2021 warn range for the same population. |
| bulgarian-women-inhibin-b-2004 | reference | female, age 20-40 | 33.15 | 190.75 | reference-interval-study | Healthy fertile Bulgarian women aged 20-40 years (n=65), single centre | [[Measurement of basal inhibin B level in Bulgarian women of the childbearing age].](https://pubmed.ncbi.nlm.nih.gov/15518284/), Abstract, Results | 5th-95th percentile in a small, older single-centre study. Cycle phase is not stated in the abstract. Assay-dependent and not validated against newer assays. |
| male-infertility-inhibin-b-cutoffs-2021 | warn | male, severe-oligozoospermia | 58.25 | - | cohort | Subfertile Chinese men, 324 patients with different spermatogenesis status | [Clinical application value of Inhibin B alone or in combination with other hormone indicators in subfertile men with different spermatogenesis status: A study of 324 Chinese men.](https://pubmed.ncbi.nlm.nih.gov/34181290/), Abstract, Results | Upper cut-off for severe oligozoospermia (ISO) in the source. Values at or below this favour impaired spermatogenesis. Single-centre, derived from the same cohort it was tested in. Source kind: diagnostic. Source band <= 58.25; stored as the warn low bound 58.25. |
| micro-tese-inhibin-b-cutoff-2023 | warn | male, non-obstructive-azoospermia | - | 21.51 | cohort | Men with idiopathic non-obstructive azoospermia undergoing micro-TESE | [Inhibin B-to-Anti-Mullerian Hormone Ratio as Noninvasive Predictors of Positive Sperm Retrieval in Idiopathic Non-Obstructive Azoospermia.](https://pubmed.ncbi.nlm.nih.gov/36675430/), Abstract, Results | ROC cut-off for positive sperm retrieval with micro-TESE. Values above this favour a positive retrieval. Single-centre. Source kind: diagnostic. Source band >= 21.51; stored as the warn high bound 21.51. |
| puberty-inhibin-b-girls-2000 | reference | female | - | 100 | reference-interval-study | Healthy prepubertal schoolgirls aged 6-20 years (n=403 across puberty stages) | [Serum inhibin A and inhibin B in healthy prepubertal, pubertal, and adolescent girls and adult women: relation to age, stage of puberty, menstrual cycle, follicle-stimulating hormone, luteinizing hormone, and estradiol levels.](https://pubmed.ncbi.nlm.nih.gov/10770209/), Abstract, Results | Upper 95% prediction limit for prepubertal girls (Tanner stage 1). The lower limit is below 20 pg/mL. Levels are low or undetectable before puberty. Applies only to: tanner_min=1; tanner_max=1. |
| tese-inhibin-b-cutoff-2016 | warn | male, azoospermia | - | 19.1 | cohort | Azoospermia patients undergoing testicular sperm extraction (TESE), Chinese cohort | [[Level of serum inhibin B for predicting the results of testicular sperm extraction in azoospermia patients].](https://pubmed.ncbi.nlm.nih.gov/29282914/), Abstract, Results | ROC cut-off for predicting TESE result. Values at or above this favour sperm retrieval. Single-centre, small sample. Source kind: diagnostic. Source band >= 19.1; stored as the warn high bound 19.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
