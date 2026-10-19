# Mean Corpuscular Volume (`mcv`)

Average volume of a red blood cell. Low values (microcytosis) suggest iron deficiency or thalassaemia trait; high values (macrocytosis) suggest B12 or folate deficiency, alcohol use, liver disease or reticulocytosis.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | fL |
| LOINC | [787-2](https://loinc.org/787-2/) |
| Specimen | - |
| Aliases | - |
| Panels | [Biological aging](../panels/biological-aging.md), [Iron deficiency and iron deficiency anaemia](../panels/iron-anemia.md) |

## Ranges

Bounds in fL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 79 | 97 | - | - | uncited (legacy) | - |
| am-fam-physician-2009 | warn | all | - | 100 | consensus | adults with an incidental raised MCV on a complete blood count (macrocytosis evaluation review) | [Evaluation of macrocytosis.](https://pubmed.ncbi.nlm.nih.gov/19202968/), Abstract | Review (American Family Physician) defining macrocytosis as MCV above 100 fL; abstract-level citation, not a guideline threshold. No microcytosis (low MCV) cutoff was found in retrievable open-access text, so none is encoded. Source band >= 100; stored as the warn high bound 100. |
| norip | reference | age 18-90 | 82 | 98 | reference-interval-study | 1826 healthy Nordic adults aged 18-90 (NORIP), venous EDTA blood, 12 analyser types; no sex-specific interval reported | [A multicentre study of reference intervals for haemoglobin, basic blood cell counts and erythrocyte indices in the adult population of the Nordic countries.](https://pubmed.ncbi.nlm.nih.gov/15223702/), Abstract | Non-parametric 2.5th-97.5th percentiles. The NORIP abstract reports one interval for both sexes. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
