# Human Chorionic Gonadotropin, Quantitative (beta-hCG) (`hcg-quantitative`)

Quantitative beta subunit of human chorionic gonadotropin in serum. Used to confirm and date pregnancy, to judge whether an early pregnancy is viable and intrauterine, and to monitor pregnancy of unknown location. Pituitary secretion raises values modestly in perimenopausal and postmenopausal women.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | mIU/mL |
| LOINC | - |
| Specimen | - |
| Aliases | `hcg` |

## Ranges

Bounds in mIU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | not pregnant | - | 5 | reference-interval-study | Conventional adult non-pregnant reference interval, as cited by the authors of a 2026 age-specific hCG study | [Establishing age-specific reference intervals for serum beta-human chorionic gonadotropin using Hoffmann and nonparametric methods: A big data study based on laboratory databases.](https://pubmed.ncbi.nlm.nih.gov/41253215/), Abstract, Background | Conventional upper reference limit for non-pregnant adults. The source authors cite it as the conventional value and show it may be inappropriate for women aged 45 and over. Converted from the source's IU/L (5) to mIU/mL using factor 1. |
| alt-1 | reference | post-menopausal | - | 8.2 | reference-interval-study | Healthy postmenopausal women (n=355 analysed), 460 enrolled, four CLIA systems (method: chemiluminescence immunoassay, Siemens Atellica IM 1600 (System B)) | [Redefining "Normal" for hCG in postmenopausal women: 97.5th percentile reference intervals and 99th percentile thresholds across four chemiluminescence immunoassay systems.](https://pubmed.ncbi.nlm.nih.gov/42607919/), Abstract, Results | Non-parametric 97.5th percentile upper reference limit for System B, which gave the highest limits of the four systems. Converted from the source's IU/L (8.2) to mIU/mL using factor 1. Alternative to the default hcg-postmenopausal-clia-2026 reference range for the same population. |
| alt-2 | reference | post-menopausal | - | 5.7 | reference-interval-study | Healthy postmenopausal women (n=355 analysed), 460 enrolled, four CLIA systems (method: chemiluminescence immunoassay, Roche Cobas e 801 (System C)) | [Redefining "Normal" for hCG in postmenopausal women: 97.5th percentile reference intervals and 99th percentile thresholds across four chemiluminescence immunoassay systems.](https://pubmed.ncbi.nlm.nih.gov/42607919/), Abstract, Results | Non-parametric 97.5th percentile upper reference limit for System C, which gave the lowest limits of the four systems. Converted from the source's IU/L (5.7) to mIU/mL using factor 1. Alternative to the default hcg-postmenopausal-clia-2026 reference range for the same population. |
| alt-3 | reference | post-menopausal | - | 5.9 | reference-interval-study | Healthy postmenopausal women (n=355 analysed), 460 enrolled, four CLIA systems (method: chemiluminescence immunoassay, Abbott ARCHITECT i2000SR (System D)) | [Redefining "Normal" for hCG in postmenopausal women: 97.5th percentile reference intervals and 99th percentile thresholds across four chemiluminescence immunoassay systems.](https://pubmed.ncbi.nlm.nih.gov/42607919/), Abstract, Results | Non-parametric 97.5th percentile upper reference limit for System D. Converted from the source's IU/L (5.9) to mIU/mL using factor 1. Alternative to the default hcg-postmenopausal-clia-2026 reference range for the same population. |
| gestational-sac-discriminatory-2013 | warn | pregnant, trimester 1 | - | 3510 | cohort | Women with first-trimester bleeding or pain, 651 pregnancies (366 viable) with serum beta-hCG and transvaginal ultrasound within 6 hours | [Reevaluation of discriminatory and threshold levels for serum β-hCG in early pregnancy.](https://pubmed.ncbi.nlm.nih.gov/23262929/), Abstract, Results | Discriminatory level at which a gestational sac is predicted to be seen on transvaginal ultrasound 99% of the time. Below this level an empty uterus does not exclude a viable pregnancy. Yolk sac and fetal pole levels are substantially higher. Source kind: diagnostic. Source band >= 3510; stored as the warn high bound 3510. |
| hcg-postmenopausal-clia-2026 | reference | post-menopausal | - | 7.1 | reference-interval-study | Healthy postmenopausal women (n=355 analysed), 460 enrolled, four CLIA systems (method: chemiluminescence immunoassay, Beckman UniCel DxI 800 (System A)) | [Redefining "Normal" for hCG in postmenopausal women: 97.5th percentile reference intervals and 99th percentile thresholds across four chemiluminescence immunoassay systems.](https://pubmed.ncbi.nlm.nih.gov/42607919/), Abstract, Results | Non-parametric 97.5th percentile upper reference limit for System A. Assay-specific; values from other platforms differ. Converted from the source's IU/L (7.1) to mIU/mL using factor 1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
