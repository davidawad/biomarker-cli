# Urinary iodine concentration (`iodine-urine`)

Iodine concentration in a spot urine sample, the main indicator of recent iodine intake. Values are interpreted as population medians rather than for an individual; low medians indicate deficiency and high medians indicate excess intake.

| | |
|---|---|
| Category | [trace-element](index.md#trace-element) |
| Canonical unit | µg/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in µg/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | age 6-12 | - | 300 | guideline | Population median urinary iodine in school-age children | [GLOBAL ENDOCRINOLOGY: Global perspectives in endocrinology: coverage of iodized salt programs and iodine status in 2020.](https://pubmed.ncbi.nlm.nih.gov/33989173/), Table (WHO iodine intake categories) | Population-median criterion. The table writes '>300' for excessive; encoded as 300 (boundary exactly 300 is not separately stated). Source kind: excess. Source band >= 300; stored as the warn high bound 300. Alternative to the default who-2007 warn range for the same population. |
| who-2007 | warn | age 6-12 | 100 | - | guideline | Population median urinary iodine in school-age children (WHO/UNICEF/ICCIDD criteria, as reproduced in a 2021 review) | [GLOBAL ENDOCRINOLOGY: Global perspectives in endocrinology: coverage of iodized salt programs and iodine status in 2020.](https://pubmed.ncbi.nlm.nih.gov/33989173/), Table (WHO iodine intake categories) | Population-median criterion, not an individual cut-off. The table lists sub-bands 50-99 (mild), 20-49 (moderate), <20 (severe) deficiency, all insufficient. Source kind: insufficient. Source band <= 100; stored as the warn low bound 100. |
| who-2007 | warn | pregnant | 150 | - | guideline | Population median urinary iodine in pregnant women (WHO table) | [GLOBAL ENDOCRINOLOGY: Global perspectives in endocrinology: coverage of iodized salt programs and iodine status in 2020.](https://pubmed.ncbi.nlm.nih.gov/33989173/), Table (WHO iodine intake categories) | Population-median criterion for pregnancy. Pregnancy categories differ from school-age children. Source kind: insufficient. Source band <= 150; stored as the warn low bound 150. |
| who-2007 | optimal | pregnant | 150 | 249 | guideline | Population median urinary iodine in pregnant women | [Iodine and Iodine Deficiency: A Comprehensive Review of a Re-Emerging Issue.](https://pubmed.ncbi.nlm.nih.gov/36079737/), Introduction | Population-median criterion for pregnancy (the median, not individual values, is classified). |
| who-2007 | optimal | age 6+, not pregnant | 100 | 199 | guideline | Population median urinary iodine in non-pregnant, non-lactating adults and children aged 6 years or older | [Iodine and Iodine Deficiency: A Comprehensive Review of a Re-Emerging Issue.](https://pubmed.ncbi.nlm.nih.gov/36079737/), Introduction | Population-median criterion. Other sources use 100-299 for school-age children; the adult/child range here is 100-199 as stated in the review. Applies only to: lactating=False. |
| who-2013 | optimal | age 6-12 | 100 | 299 | consensus | National or subnational population median urinary iodine in school-aged children (global review of 2013 status) | [Global iodine nutrition: Where do we stand in 2013?](https://pubmed.ncbi.nlm.nih.gov/23472655/), Summary | Population-median criterion. Adequate intake defined at median 100-299 µg/L in school-aged children. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
