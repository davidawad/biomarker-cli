# Osmolality, Serum (`osmolality`)

Serum osmolality measures the concentration of osmotically active solutes. No verifiable source for a measured serum osmolality reference interval (commonly quoted 275-295) was found; ranges here are dehydration thresholds in older adults. The osmolal gap is in osmolal-gap.toml.

| | |
|---|---|
| Category | [electrolyte](index.md#electrolyte) |
| Canonical unit | mOsm/kg |
| LOINC | [2692-2](https://loinc.org/2692-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mOsm/kg. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 65+ | 295 | 300 | cohort | Older people aged 65 years and over in five cohorts (community and hospital; directly measured serum/plasma osmolality) | [Diagnostic accuracy of calculated serum osmolarity to predict dehydration in older people: adding value to pathology laboratory reports.](https://pubmed.ncbi.nlm.nih.gov/26490100/), Abstract | Impending dehydration band: osmolality at or above 295 mOsm/kg up to the 300 mOsm/kg current-dehydration threshold (the source gives >=295 and >300 separately; high = 300 marks where the current-dehydration cut-off starts). Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| diagnostic | warn | age 65+ | - | 300 | cohort | Hospitalised adults aged 65 years and over (low-intake dehydration; directly measured serum or plasma osmolality) | [Prevalence of low-intake dehydration in hospitalised older adults: systematic review and meta-analysis.](https://pubmed.ncbi.nlm.nih.gov/42158549/), Abstract (Results) | Low-intake dehydration threshold: measured osmolality strictly above 300 mOsm/kg (low = 300 means strictly greater than 300). Threshold as used by the 19 included studies. Source kind: diagnostic. Source band >= 300; stored as the warn high bound 300. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
