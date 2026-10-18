# hCG 48-Hour Percentage Rise (`hcg-48h-rise`)

Percentage increase in serum hCG over 48 hours in early pregnancy, used to judge whether a pregnancy of unknown location is behaving like a viable intrauterine pregnancy.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | pregnant, trimester 1 | - | 40 | consensus | Viable intrauterine pregnancies after assisted conception; minimum 48-hour hCG rise quoted from prior literature | [Abnormal rate of human chorionic gonadotropin rise: a case series of patients with viable intrauterine pregnancies after embryo transfer.](https://pubmed.ncbi.nlm.nih.gov/34223284/), Discussion | Lowest expected 48-hour percentage rise in a viable intrauterine pregnancy when the initial hCG is 1,500 to 3,000 mIU/mL. Quoted by the source from earlier work. Applies only to: markers=[{"slug": "hcg-quantitative", "op": ">=", "value": 1500, "unit": "mIU/mL"}, {"slug": "hcg-quantitative", "op": "<=", "value": 3000, "unit": "mIU/mL"}]. Source kind: diagnostic. Source band >= 40; stored as the warn high bound 40. Alternative to the default hcg-48h-rise-case-series-2021 warn range for the same population. |
| alt-2 | warn | pregnant, trimester 1 | - | 33 | consensus | Viable intrauterine pregnancies after assisted conception; minimum 48-hour hCG rise quoted from prior literature | [Abnormal rate of human chorionic gonadotropin rise: a case series of patients with viable intrauterine pregnancies after embryo transfer.](https://pubmed.ncbi.nlm.nih.gov/34223284/), Discussion | Lowest expected 48-hour percentage rise in a viable intrauterine pregnancy when the initial hCG is above 3,000 mIU/mL. Quoted by the source from earlier work. Applies only to: markers=[{"slug": "hcg-quantitative", "op": ">", "value": 3000, "unit": "mIU/mL"}]. Source kind: diagnostic. Source band >= 33; stored as the warn high bound 33. Alternative to the default hcg-48h-rise-case-series-2021 warn range for the same population. |
| hcg-48h-rise-case-series-2021 | warn | pregnant, trimester 1 | - | 49 | consensus | Viable intrauterine pregnancies after assisted conception; minimum 48-hour hCG rise quoted from prior literature | [Abnormal rate of human chorionic gonadotropin rise: a case series of patients with viable intrauterine pregnancies after embryo transfer.](https://pubmed.ncbi.nlm.nih.gov/34223284/), Discussion | Lowest 48-hour percentage rise expected in a viable intrauterine pregnancy when the initial hCG is below 1,500 mIU/mL. A rise below this level warrants closer evaluation for ectopic or non-viable pregnancy. The figure is quoted by the source from earlier work, not derived by the source. Applies only to: markers=[{"slug": "hcg-quantitative", "op": "<", "value": 1500, "unit": "mIU/mL"}]. Source kind: diagnostic. Source band >= 49; stored as the warn high bound 49. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
