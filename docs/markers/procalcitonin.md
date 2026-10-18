# Procalcitonin (`procalcitonin`)

Precursor of calcitonin that rises markedly in systemic bacterial infection and sepsis, and falls as infection resolves. Used mainly to support decisions about starting or stopping antibiotics.

| | |
|---|---|
| Category | [inflammation](index.md#inflammation) |
| Canonical unit | ng/mL |
| LOINC | [33959-8](https://loinc.org/33959-8/) |
| Specimen | - |
| Aliases | `pct`, `pct test` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | 0.5 | - | consensus | Critically ill adults with acute respiratory infection or sepsis in intensive care (setting: icu) | [Procalcitonin-guided antibiotic therapy algorithms for different types of acute respiratory infections based on previous trials.](https://pubmed.ncbi.nlm.nih.gov/29969320/), Abstract, Results | Consensus algorithm cut-off for discontinuing antibiotics in ICU patients (<= 0.5 ug/L; strong recommendation <= 0.25). Converted from the source's µg/L (0.5) to ng/mL using factor 1. Source kind: decision. Source band <= 0.5; stored as the warn low bound 0.5. Alternative to the default pct-algorithms-review-2018 warn range for the same population. |
| alt-2 | warn | all | - | 0.5 | cohort | Adults with suspected community-acquired pneumonia in the emergency department (algorithm from a randomised trial, as summarised in a review) | [Procalcitonin in respiratory disease: use as a biomarker for diagnosis and guiding antibiotic therapy.](https://pubmed.ncbi.nlm.nih.gov/31803264/), Section on antibiotic guidance, para describing trial algorithm | Antibiotics strongly encouraged above 0.5 ug/L (boundary > 0.5). 1 ug/L equals 1 ng/mL. Converted from the source's µg/L (0.5) to ng/mL using factor 1. Source kind: decision. Source band >= 0.5; stored as the warn high bound 0.5. Alternative to the default pct-ed-algorithm-review-2019 warn range for the same population. |
| balkan-pct-consensus-2023 | warn | all | 0.25 | - | consensus | Adults with mild disease and diagnostic uncertainty about bacterial infection (setting: mild-disease-diagnostic-uncertainty) | [PROCALCITONIN GUIDED ANTIBIOTIC STEWARDSHIP: A BALKAN EXPERT CONSENSUS STATEMENT.](https://pubmed.ncbi.nlm.nih.gov/38304372/), Full text, recommendations | Expert consensus: with PCT below 0.25 ug/L the probability of bacterial infection is low and antibiotics are withheld with a repeat PCT in 6-12 h. 1 ug/L equals 1 ng/mL. Converted from the source's µg/L (0.25) to ng/mL using factor 1. Source kind: decision. Source band <= 0.25; stored as the warn low bound 0.25. |
| pct-algorithms-review-2018 | warn | all | 0.25 | - | consensus | Adults with acute respiratory infection in emergency departments and inpatient wards (setting: ed-or-ward) | [Procalcitonin-guided antibiotic therapy algorithms for different types of acute respiratory infections based on previous trials.](https://pubmed.ncbi.nlm.nih.gov/29969320/), Abstract, Results | Consensus algorithm cut-off for discontinuing antibiotics in ED and inpatients (<= 0.25 ug/L; strong recommendation <= 0.1). 1 ug/L equals 1 ng/mL. Converted from the source's µg/L (0.25) to ng/mL using factor 1. Source kind: decision. Source band <= 0.25; stored as the warn low bound 0.25. |
| pct-ed-algorithm-review-2019 | warn | all | 0.1 | - | cohort | Adults with suspected community-acquired pneumonia in the emergency department (algorithm from a randomised trial, as summarised in a review) | [Procalcitonin in respiratory disease: use as a biomarker for diagnosis and guiding antibiotic therapy.](https://pubmed.ncbi.nlm.nih.gov/31803264/), Section on antibiotic guidance, para describing trial algorithm | Antibiotics strongly discouraged when PCT is below 0.1 ug/L (1 ug/L equals 1 ng/mL). Boundary < 0.1. Converted from the source's µg/L (0.1) to ng/mL using factor 1. Source kind: decision. Source band <= 0.1; stored as the warn low bound 0.1. |
| pct-sepsis-meta-2023 | warn | all | 0.5 | - | cohort | Adults with sepsis in trials of procalcitonin-guided antibiotic therapy (meta-analysis) (setting: sepsis-liberal-protocol) | [Procalcitonin-guided antibiotic therapy may shorten length of treatment and may improve survival-a systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/37833778/), Methods, subgroup analyses (protocol definitions) | Liberal protocol threshold: stop antibiotics if PCT falls below 0.5 ng/mL (or drops more than 80% from peak). Boundary <. Source kind: decision. Source band <= 0.5; stored as the warn low bound 0.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
