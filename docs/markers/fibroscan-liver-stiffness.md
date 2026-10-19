# Liver Stiffness (FibroScan Transient Elastography) (`fibroscan-liver-stiffness`)

Liver stiffness measured by FibroScan transient elastography, in kilopascals (kPa). A non-invasive estimate of liver fibrosis (scarring), staged against biopsy in the source studies. Higher values indicate more fibrosis; below 7.1 kPa is the range below the significant-fibrosis cut-off.

| | |
|---|---|
| Category | [liver](index.md#liver) |
| Canonical unit | kPa |
| LOINC | [77615-3](https://loinc.org/77615-3/) |
| Specimen | - |
| Aliases | - |
| Panels | [Liver health and MASLD](../panels/liver.md) |

## Ranges

Bounds in kPa. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | hepatitis-c | - | 7.1 | cohort | Adults with chronic hepatitis C, biopsy-staged (METAVIR); prospective study. Cut-offs not validated for other etiologies. | [Prospective comparison of transient elastography, Fibrotest, APRI, and liver biopsy for the assessment of fibrosis in chronic hepatitis C.](https://pubmed.ncbi.nlm.nih.gov/15685546/), Abstract (Results) | Below 7.1 kPa: F0-F1 (below the F2-or-higher cut-off). Another hepatitis C study (Hepatology 2005) reports different cut-offs (8.7 and 14.5 kPa). |
| diagnostic | warn | hepatitis-c | - | 12.5 | cohort | Adults with chronic hepatitis C, biopsy-staged (METAVIR); prospective study. Cut-offs not validated for other etiologies. | [Prospective comparison of transient elastography, Fibrotest, APRI, and liver biopsy for the assessment of fibrosis in chronic hepatitis C.](https://pubmed.ncbi.nlm.nih.gov/15685546/), Abstract (Results) | 12.5 kPa or above: F4 (cirrhosis) by the source's F4 cut-off. Source kind: diagnostic. Source band >= 12.5; stored as the warn high bound 12.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
