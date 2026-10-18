# Adrenocorticotropic Hormone (`acth`)

Pituitary hormone that stimulates cortisol production by the adrenal cortex. Measured in the morning (plasma, cold-collected) to help separate primary from secondary or tertiary adrenal insufficiency and ACTH-dependent from ACTH-independent Cushing's syndrome.

| | |
|---|---|
| Category | [pituitary](index.md#pituitary) |
| Canonical unit | pg/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 20-69 | 8 | 93 | reference-interval-study | 50 healthy subjects from the general population aged 20-69 years, equally distributed by sex and age (specimen: plasma) | [Establishment of reference values for endocrine tests. Part IV: Adrenal insufficiency.](https://pubmed.ncbi.nlm.nih.gov/16397312/), Abstract, Results | 09:00 plasma ACTH reference interval (mean +/- 2 SD after log transformation). ng/L is numerically equal to pg/mL. 2005 study, older assay; use the local lab range where available. Converted from the source's ng/L (8-93) to pg/mL using factor 1. Applies only to: time_of_day=morning. |
| alt-1 | warn | cushing-syndrome | - | 20 | consensus | Patients with suspected hypercortisolism (Cushing syndrome), as restated in a laboratory-medicine review | [Pitfalls in the Diagnosis and Management of Hypercortisolism (Cushing Syndrome) in Humans; A Review of the Laboratory Medicine Perspective.](https://pubmed.ncbi.nlm.nih.gov/37189516/), Full text (Cushing diagnosis section) | ACTH above about 20 pg/mL (or an intermediate value) indicates an ACTH-dependent cause of hypercortisolism. Restates a review's threshold; not verified against the primary guideline. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. Alternative to the default cushing-acth-review-2023 warn range for the same population. |
| cushing-acth-review-2023 | warn | cushing-syndrome | 10 | - | consensus | Patients with suspected hypercortisolism (Cushing syndrome), as restated in a laboratory-medicine review | [Pitfalls in the Diagnosis and Management of Hypercortisolism (Cushing Syndrome) in Humans; A Review of the Laboratory Medicine Perspective.](https://pubmed.ncbi.nlm.nih.gov/37189516/), Full text (Cushing diagnosis section) | ACTH below about 10 pg/mL suggests an adrenal (ACTH-independent) cortisol source once exogenous glucocorticoids are excluded. The source notes that the level varies between assays. Restates a review's threshold; not verified against the primary guideline. Source kind: diagnostic. Source band <= 10; stored as the warn low bound 10. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
