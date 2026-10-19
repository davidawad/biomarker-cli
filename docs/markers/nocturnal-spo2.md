# Nocturnal SpO2 (`nocturnal-spo2`)

Nocturnal SpO2. Tracked marker with no verified reference range yet, so values can be stored and charted but are not flagged against a range.

| | |
|---|---|
| Category | [wearable](index.md#wearable) |
| Canonical unit | % |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in %. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | 90 | - | consensus | Adults, overnight oximetry | [Excessive daytime sleepiness in myotonic dystrophy: a narrative review.](https://pubmed.ncbi.nlm.nih.gov/39011358/), Diagnostic criteria | Surrogate criterion for sleep hypoventilation when CO2 is unavailable: SpO2 below 90% for more than 10% of total sleep time (or <=88% for 5 consecutive minutes). Source kind: diagnostic. Source band <= 90; stored as the warn low bound 90. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | 88 | - | consensus | Adults, overnight oximetry | [Insomnia and sleep-disordered breathing in FKRP-related limb-girdle muscular dystrophy R9. The Norwegian LGMDR9 cohort study (2020).](https://pubmed.ncbi.nlm.nih.gov/37695533/), Methods | Sleep-related hypoxemia: SpO2 at or below 88% for more than 5 consecutive minutes (ICSD-3 / AASM definition as applied in the cited cohort study). Source kind: diagnostic. Source band <= 88; stored as the warn low bound 88. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
