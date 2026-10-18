# Body Fat (`body-fat`)

Body fat as a percentage of total body mass, estimated by methods such as DXA, bioelectrical impedance (BIA) or skinfolds. Used alongside BMI to assess adiposity, which BMI alone may misclassify.

| | |
|---|---|
| Category | [body](index.md#body) |
| Canonical unit | % |
| LOINC | [41982-0](https://loinc.org/41982-0/) |
| Specimen | - |
| Aliases | `body fat`, `body fat %`, `body fat percentage`, `bf%` |

## Ranges

No clinical reference range: legacy catalog entry without a reference range; needs research

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| blueprint | optimal | all | - | 10 | - | - | uncited (legacy) | - |
| jes-2026-percent-bf-obesity | warn | male | - | 25 | cohort | Adults (obesity threshold for percent body fat as cited in a 2026 Journal of the Endocrine Society brief report on DXA body composition) (method: dxa) | [Percent body fat misclassifies patients with sarcopenia as obese: support for fat mass index in obesity assessment.](https://pubmed.ncbi.nlm.nih.gov/42453567/), Full text, Introduction (obesity definitions) | Obesity by percent body fat in men: 25 percent or more (boundary >= 25). The report cites its own reference 5 for this threshold, which was not retrieved; the report describes %BF as reported on DXA. Source kind: decision. Source band >= 25; stored as the warn high bound 25. |
| jes-2026-percent-bf-obesity | warn | female | - | 35 | cohort | Adults (obesity threshold for percent body fat as cited in a 2026 Journal of the Endocrine Society brief report on DXA body composition) (method: dxa) | [Percent body fat misclassifies patients with sarcopenia as obese: support for fat mass index in obesity assessment.](https://pubmed.ncbi.nlm.nih.gov/42453567/), Full text, Introduction (obesity definitions) | Obesity by percent body fat in women: 35 percent or more (boundary >= 35). The report cites its own reference 5 for this threshold, which was not retrieved; the report describes %BF as reported on DXA. Source kind: decision. Source band >= 35; stored as the warn high bound 35. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
