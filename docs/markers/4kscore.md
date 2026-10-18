# 4Kscore (`4kscore`)

Blood test combining four kallikreins with clinical information, reported as a percent risk of Gleason 7 or higher (significant) prostate cancer on biopsy. Used to decide whether a man needs a prostate biopsy. Expected result: a low percent risk, below the cut-offs in the ranges below.

| | |
|---|---|
| Category | [tumor-marker](index.md#tumor-marker) |
| Canonical unit | % |
| LOINC | [90740-2](https://loinc.org/90740-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | prostate-cancer | - | 7.5 | cohort | Men considered for prostate biopsy (meta-analysis background) | [4Kscore diagnostic value in patients with high-grade prostate cancer using cutoff values of 7.5% to 10%: A meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/33685800/), Abstract (Background) | Cut-off below which the source describes low risk of high-grade cancer. The same meta-analysis concludes the 7.5% to 10% cut-offs need confirmation in larger studies. Source band >= 7.5; stored as the warn high bound 7.5. |
| clinical | reference | prostate-cancer | 5 | 23 | cohort | Men scheduled for prostate biopsy (US prospective validation study) | [Developing an effective strategy to improve the detection of significant prostate cancer by combining the 4Kscore and multiparametric MRI.](https://pubmed.ncbi.nlm.nih.gov/31378585/), Abstract (Results) | Intermediate band 5% to 23% for high-grade prostate cancer; the source places <5% in the low band and >23% in the high band. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
