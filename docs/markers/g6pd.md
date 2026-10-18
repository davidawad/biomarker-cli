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

Bounds in U/g Hb. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 6.1 | - | cohort | 987 adults and children with Plasmodium vivax in the Peruvian Amazon, semi-quantitative G6PD activity assay | [Health-care provider compliance with a revised <i>Plasmodium vivax</i> radical cure algorithm incorporating tafenoquine and quantitative G6PD testing in the Peruvian Amazon: a prospective observational study.](https://pubmed.ncbi.nlm.nih.gov/42500357/), Table 2 | Normal category in the programme. Activity thresholds are semi-quantitative and assay-specific. |
| alt-1 | warn | all | 4.1 | - | cohort | 987 adults and children with Plasmodium vivax in the Peruvian Amazon, semi-quantitative G6PD activity assay | [Health-care provider compliance with a revised <i>Plasmodium vivax</i> radical cure algorithm incorporating tafenoquine and quantitative G6PD testing in the Peruvian Amazon: a prospective observational study.](https://pubmed.ncbi.nlm.nih.gov/42500357/), Table 2 | Intermediate category: described as tafenoquine-ineligible in this algorithm; primaquine is the alternative in the Thai protocol. The Peru paper also discusses a 70% threshold for intermediate females. Source band 4.1-6.0; stored as the warn low bound 4.1. Alternative to the default g6pd-peru-2026 warn range for the same population. |
| alt-2 | warn | all | - | 4.1 | cohort | patients with Plasmodium vivax in Thailand receiving radical cure after quantitative G6PD testing (pilot implementation) | [Prospective observational study to assess the feasibility and safety of appropriate Plasmodium vivax radical cure with tafenoquine or primaquine after quantitative G6PD testing during pilot implementation in Thailand.](https://pubmed.ncbi.nlm.nih.gov/40274286/), Methods, treatment allocation | Eligibility for primaquine 15 mg/day for 14 days (G6PD normal or intermediate). Deficient (<=4.0 U/g Hb) patients received weekly primaquine 45 mg for 8 weeks instead. Source kind: diagnostic. Source band >= 4.1; stored as the warn high bound 4.1. Alternative to the default g6pd-thailand-2025 warn range for the same population. |
| g6pd-peru-2026 | warn | all | 4 | - | cohort | 987 adults and children with Plasmodium vivax in the Peruvian Amazon, semi-quantitative G6PD activity assay in a radical-cure programme | [Health-care provider compliance with a revised <i>Plasmodium vivax</i> radical cure algorithm incorporating tafenoquine and quantitative G6PD testing in the Peruvian Amazon: a prospective observational study.](https://pubmed.ncbi.nlm.nih.gov/42500357/), Table 2 | Programme cut-off for deficiency (about the 30% activity threshold per the same paper's discussion). Semi-quantitative point-of-care device, not a reference laboratory assay. Source kind: deficient. Source band <= 4.0; stored as the warn low bound 4.0. |
| g6pd-thailand-2025 | warn | all | - | 6.1 | cohort | patients with Plasmodium vivax in Thailand receiving radical cure after quantitative G6PD testing (pilot implementation) | [Prospective observational study to assess the feasibility and safety of appropriate Plasmodium vivax radical cure with tafenoquine or primaquine after quantitative G6PD testing during pilot implementation in Thailand.](https://pubmed.ncbi.nlm.nih.gov/40274286/), Methods, treatment allocation | Eligibility for tafenoquine 300 mg single dose (G6PD normal). Intermediate patients were allocated primaquine in this protocol. The sources do not convert U/g Hb values to percent of normal; the 2026 Peru paper refers to 30% and 70% activity thresholds without stating the U/g Hb equivalents. Source kind: diagnostic. Source band >= 6.1; stored as the warn high bound 6.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
