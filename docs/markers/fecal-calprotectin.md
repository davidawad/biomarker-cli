# Faecal Calprotectin (`fecal-calprotectin`)

Calprotectin is a neutrophil-derived protein measured in stool. Raised faecal calprotectin indicates intestinal mucosal inflammation and is used to triage suspected inflammatory bowel disease and to monitor IBD activity. Values are higher in young children and vary with assay.

| | |
|---|---|
| Category | [stool](index.md#stool) |
| Canonical unit | µg/g |
| LOINC | [38445-3](https://loinc.org/38445-3/) |
| Specimen | - |
| Aliases | `faecal calprotectin`, `fecal calprotectin`, `fc`, `fcp`, `stool calprotectin` |

## Ranges

Bounds in µg/g. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 50 | cohort | Adults and children over 4 years (typical reference values as reported in a systematic review) | [Comparison between fecal calprotectin and magnetic resonance enterography in ileal Crohn's disease for activity disease assessment: a systematic review.](https://pubmed.ncbi.nlm.nih.gov/41216276/), Full text, Introduction | Typical reference value is below 50 µg/g; values at or above 50 µg/g are above the typical reference range. Children under 4 years have higher normal values, so this band should not be applied to that age group. |
| espghan-fc-250-as-cited | warn | all | - | 250 | consensus | Children with suspected inflammatory bowel disease (ESPGHAN cut-off as cited by a UK paediatric cohort) | [Guidance on the interpretation of faecal calprotectin levels in children.](https://pubmed.ncbi.nlm.nih.gov/33571226/), Methods, cut-off levels | Cut-off above which faecal calprotectin was suggested to predict mucosal inflammation in IBD. The number is taken from ESPGHAN as cited in the 2021 paper, not from the ESPGHAN document itself. Quoted as above 250 µg/g. Source kind: decision. Source band >= 250; stored as the warn high bound 250. |
| fc-mre-systematic-review-2025 | reference | age 65+ | 100 | 112 | cohort | Individuals aged 65 years and older (reference range as recommended in a systematic review, which attributes it to age-related mucosal change) | [Comparison between fecal calprotectin and magnetic resonance enterography in ileal Crohn's disease for activity disease assessment: a systematic review.](https://pubmed.ncbi.nlm.nih.gov/41216276/), Full text, Introduction | Age-adjusted reference band for older adults reported by the systematic review. The figure is quoted from the review rather than from a primary reference-interval study. |
| fc-mre-systematic-review-2025 | warn | all | - | 100 | cohort | Adults with Crohn's disease or ulcerative colitis; activity cut-offs as reported across studies in a systematic review | [Comparison between fecal calprotectin and magnetic resonance enterography in ileal Crohn's disease for activity disease assessment: a systematic review.](https://pubmed.ncbi.nlm.nih.gov/41216276/), Abstract, Results | IBD activity cut-offs reported across studies ranged from above 100 to above 430 µg/g. The lower end of that range is shown here; this is not a single guideline threshold, so treat it as indicative only. Source kind: decision. Source band >= 100; stored as the warn high bound 100. |
| uk-paeds-fc-cohort-2021 | reference | all | - | 50 | cohort | UK paediatric patients referred for suspected IBD, measured with a commercial calprotectin kit | [Guidance on the interpretation of faecal calprotectin levels in children.](https://pubmed.ncbi.nlm.nih.gov/33571226/), Abstract, Results and Conclusion | Laboratory reference value used in this paediatric cohort. Results above 50 µg/g were common and many were not due to IBD, so the authors suggest a higher threshold for referral. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
