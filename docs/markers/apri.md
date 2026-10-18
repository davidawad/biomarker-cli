# AST to Platelet Ratio Index (`apri`)

APRI is a simple serum score calculated from the AST level and platelet count. It is used as a low-cost, non-invasive estimate of liver fibrosis and cirrhosis, mainly in chronic hepatitis B and C.

| | |
|---|---|
| Category | [derived](index.md#derived) |
| Canonical unit | index |
| LOINC | - |
| Specimen | - |
| Aliases | `ast-to-platelet ratio index`, `ast/plt ratio` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in index. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 1 | guideline | Adults with chronic hepatitis B; thresholds from the WHO 2024 hepatitis B guidelines | [The Impact of Access to Clinical Guidelines on LLM-Based Treatment Recommendations for Chronic Hepatitis B.](https://pubmed.ncbi.nlm.nih.gov/40891225/), Full text, guideline excerpt quoted in Methods | Cirrhosis (F4) threshold: APRI greater than 1 (boundary > 1), as reproduced in a 2025 paper quoting the WHO 2024 wording. The source paper is about LLM treatment recommendations, so this is a secondary reproduction of the guideline text. A supporting meta-analysis (PMID 39983746) gives the same cut-off (greater than 1.0) but its quote uses a decimal-comma style that the checker cannot match. Source kind: decision. Source band >= 1; stored as the warn high bound 1. Alternative to the default who-2024-hbv-apri warn range for the same population. |
| who-2024-hbv-apri | warn | all | - | 0.5 | guideline | Adults with chronic hepatitis B; thresholds from the WHO 2024 hepatitis B guidelines | [Guidelines for the Prevention, Diagnosis, Care and Treatment for People with Chronic Hepatitis B Infection (Text Extract): Executive Summary.](https://pubmed.ncbi.nlm.nih.gov/39391287/), Full text, treatment recommendations | Significant fibrosis (F2 or above) threshold: APRI greater than 0.5 (boundary > 0.5), used with or without HBV DNA or ALT levels to decide on treatment. Quoted from the WHO 2024 guideline text extract. Hepatitis C thresholds were not retrieved. Source kind: decision. Source band >= 0.5; stored as the warn high bound 0.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
