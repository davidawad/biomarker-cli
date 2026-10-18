# Phosphatidylethanol (PEth) (`peth`)

Direct blood biomarker of recent alcohol intake: phosphatidylethanol 16:0/18:1 forms in red blood cells only when ethanol is present. Used to monitor abstinence, alcohol use disorder treatment, and forensic or fitness-to-drive assessment. Expected result for an abstinent or low-consumption person: below 20 ng/mL. Cut-offs are classification thresholds, not reference intervals.

| | |
|---|---|
| Category | [metabolic](index.md#metabolic) |
| Canonical unit | ng/mL |
| LOINC | [74661-0](https://loinc.org/74661-0/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 200 | consensus | General adults, PEth in whole blood | [Assessing alcohol consumption in an era of direct alcohol markers.](https://pubmed.ncbi.nlm.nih.gov/42248449/), Section on PEth cut-offs | Approximately 200 ng/mL: commonly considered indicative of high-risk alcohol consumption (per review; cited as in agreement with the Consensus of Basel). Secondary source: narrative review, not the primary consensus. Source kind: diagnostic. Source band >= 200; stored as the warn high bound 200. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 20 | consensus | General adults, PEth 16:0/18:1 in whole blood | [Assessing alcohol consumption in an era of direct alcohol markers.](https://pubmed.ncbi.nlm.nih.gov/42248449/), Section on PEth cut-offs | Cut-off of 20 ng/mL (US laboratory practice, per review): at or above it indicates deliberate alcohol consumption rather than abstinence or incidental exposure. Secondary source: narrative review, not the primary Consensus of Basel (full text not accessible). Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |
| diagnostic | warn | pregnant | - | 8 | cohort | Pregnant women, Western Cape Province, South Africa (n = 222), PEth from dried blood spots | [Assessing the sensitivity and specificity of phosphatidylethanol (PEth) cutoffs to identify alcohol exposed pregnancies.](https://pubmed.ncbi.nlm.nih.gov/37102125/), Abstract | Research-use positive cut-off of 8 ng/mL for identifying alcohol-consuming pregnant women in this single sample, judged against self-report and AUDIT; not a validated clinical threshold. Source kind: diagnostic. Source band >= 8; stored as the warn high bound 8. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
