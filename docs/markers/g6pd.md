# G6PD activity (`g6pd`)

Activity of the red-cell enzyme glucose-6-phosphate dehydrogenase, expressed per gram of haemoglobin. Low activity (G6PD deficiency) predisposes to oxidant-induced haemolysis, for example after primaquine or tafenoquine, so the result decides whether these 8-aminoquinoline drugs can be given.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | U/g Hb |
| LOINC | [2357-2](https://loinc.org/2357-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in U/g Hb. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | all | - | 4.1 | cohort | patients with Plasmodium vivax in Thailand receiving radical cure after quantitative G6PD testing (pilot implementation) | [Prospective observational study to assess the feasibility and safety of appropriate Plasmodium vivax radical cure with tafenoquine or primaquine after quantitative G6PD testing during pilot implementation in Thailand.](https://pubmed.ncbi.nlm.nih.gov/40274286/), Methods, treatment allocation | Eligibility for primaquine 15 mg/day for 14 days (G6PD normal or intermediate). Deficient (<=4.0 U/g Hb) patients received weekly primaquine 45 mg for 8 weeks instead. Source kind: diagnostic. Source band >= 4.1; stored as the warn high bound 4.1. Alternative to the default g6pd-thailand-2025 warn range for the same population. |
| g6pd-thailand-2025 | warn | all | - | 6.1 | cohort | patients with Plasmodium vivax in Thailand receiving radical cure after quantitative G6PD testing (pilot implementation) | [Prospective observational study to assess the feasibility and safety of appropriate Plasmodium vivax radical cure with tafenoquine or primaquine after quantitative G6PD testing during pilot implementation in Thailand.](https://pubmed.ncbi.nlm.nih.gov/40274286/), Methods, treatment allocation | Eligibility for tafenoquine 300 mg single dose (G6PD normal). Intermediate patients were allocated primaquine in this protocol. The sources do not convert U/g Hb values to percent of normal; the 2026 Peru paper refers to 30% and 70% activity thresholds without stating the U/g Hb equivalents. Source kind: diagnostic. Source band >= 6.1; stored as the warn high bound 6.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
