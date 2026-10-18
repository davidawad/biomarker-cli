# Immunoglobulin G (`igg`)

The most abundant immunoglobulin in blood, made by plasma cells and the main antibody against systemic infections. Low levels (hypogammaglobulinaemia) raise infection risk, while very high levels can signal a monoclonal gammopathy.

| | |
|---|---|
| Category | [immune](index.md#immune) |
| Canonical unit | g/L |
| LOINC | [2465-3](https://loinc.org/2465-3/) |
| Specimen | - |
| Aliases | `immunoglobulin g, serum` |

## Ranges

Bounds in g/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 18-56 | 6.36 | 15.18 | reference-interval-study | Iranian healthy adult blood donor volunteers aged 18-55 years (n=914), measured by nephelometry | [Normal value of immunoglobulins IgA, IgG, and IgM in Iranian healthy adults, measured by nephelometry.](https://pubmed.ncbi.nlm.nih.gov/14677654/), Abstract, Methods; [Normal value of immunoglobulins IgA, IgG, and IgM in Iranian healthy adults, measured by nephelometry.](https://pubmed.ncbi.nlm.nih.gov/14677654/), Abstract, Results | Central reference interval as reported (mg/dL). Single-country blood-donor cohort; check against the local assay's own reference interval. Converted from the source's mg/dL (636-1518) to g/L using factor 0.01. |
| brazil-children-2024 | reference | age 1-11 | 7.92 | 17.71 | reference-interval-study | Healthy children of both sexes aged 1-10 years living in central Brazil | [Reference intervals for serum immunoglobulins G, M and E levels in Brazilian children aged 1 to 10 years: a population-based study.](https://pubmed.ncbi.nlm.nih.gov/38145629/), Abstract, Results | Population-based rank-method reference interval (mg/dL). Paediatric values; no sex partition reported for IgG. Converted from the source's mg/dL (792-1771) to g/L using factor 0.01. |
| immunodeficiency-review-2026 | warn | all | 4 | - | consensus | Adults with suspected immunodeficiency, as summarised in a 2026 review (Immunodeficiencies in Adults) | [Immunodeficiencies in Adults: Key Considerations for Diagnosis and Management.](https://pubmed.ncbi.nlm.nih.gov/41107625/), Full text, Diagnosis section | A total serum IgG below 4 g/L is described as a strong indicator for further immunological evaluation (boundary < 4 g/L). The review stresses that ESID diagnostic criteria need a broader assessment; this is not itself a diagnostic cut-off. Source kind: decision. Source band <= 4; stored as the warn low bound 4. |
| immunodeficiency-review-2026 | warn | common-variable-immunodeficiency | 3 | - | consensus | Patients with common variable immunodeficiency (CVID), as summarised in a 2026 review | [Immunodeficiencies in Adults: Key Considerations for Diagnosis and Management.](https://pubmed.ncbi.nlm.nih.gov/41107625/), Full text, Diagnosis section | In CVID cohorts the risk of infection, particularly pneumonia, rises significantly when IgG falls below 3.0 g/L (boundary < 3.0 g/L). Cited by the review from an underlying CVID study that was not itself verified. Source band <= 3; stored as the warn low bound 3. |
| spain-multicentre-2007 | reference | all | 6.5 | 14.3 | reference-interval-study | Adults (women and men) from 21 Spanish clinical laboratories, measured with Tina-Quant reagents on RD/Hitachi analysers | [Multicentre physiological reference intervals for serum concentrations of immunoglobulins A, G and M, complement C3c and C4 measured with Tina-Quant reagents systems.](https://pubmed.ncbi.nlm.nih.gov/17378738/), Abstract, Results and conclusions | Multicentre adult reference interval (sexes combined), derived non-parametrically from each laboratory's reference individuals. Assay-specific; check against the local assay's own interval. |
| study-definition-2026 | warn | all | 6 | - | cohort | Adults undergoing antibody-deficiency screening in a 2026 cohort (study reference standard) | [Improving screening for antibody deficiency using calculated globulin and serum protein electrophoresis.](https://pubmed.ncbi.nlm.nih.gov/41929515/), Full text, Results | Hypogammaglobulinaemia as used by the study as its reference standard (IgG below 6 g/L, strict). This is a study definition, not a guideline threshold; paediatric comparisons used manufacturer ranges. Source kind: decision. Source band <= 6; stored as the warn low bound 6. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
