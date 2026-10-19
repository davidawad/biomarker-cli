# Plasma p-tau217 (`p-tau217`)

Plasma phosphorylated tau 217, the leading blood biomarker of Alzheimer's amyloid pathology. Values and cut-offs are strongly assay-specific; only a Lumipulse two-cut-off example is encoded.

| | |
|---|---|
| Category | [neurology](index.md#neurology) |
| Canonical unit | pg/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |
| Panels | [Alzheimer's disease and dementia workup](../panels/alzheimers.md) |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.229 | 0.516 | cohort | Subjective cognitive decline, MCI and AD-dementia patients (Lumipulse G600II); gray zone between the lower and upper cut-off (specimen: plasma; method: lumipulse-g600ii) | [The two cut-offs approach for plasma p-tau217 in detecting Alzheimer's disease in subjective cognitive decline and mild cognitive impairment.](https://pubmed.ncbi.nlm.nih.gov/40357140/), Results | Gray zone of the two-cut-off approach: neither amyloid-positive nor amyloid-negative (Core 1) classification; confirm with PET or CSF. Assay- and cohort-specific, not transferable to other assays. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| diagnostic | warn | all | - | 0.516 | cohort | Subjective cognitive decline, MCI and AD-dementia patients (Lumipulse G600II) (specimen: plasma; method: lumipulse-g600ii) | [The two cut-offs approach for plasma p-tau217 in detecting Alzheimer's disease in subjective cognitive decline and mild cognitive impairment.](https://pubmed.ncbi.nlm.nih.gov/40357140/), Results | Above the upper cut-off (specificity 96%): predicts Core 1 positivity (amyloid PET / CSF). Assay- and cohort-specific. Source kind: diagnostic. Source band >= 0.516; stored as the warn high bound 0.516. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
