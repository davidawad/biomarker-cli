# Immunoglobulin A (`iga`)

The main antibody on mucosal surfaces such as the gut and airways. Very low serum IgA (selective IgA deficiency) is the commonest primary immunodeficiency and affects how coeliac disease serology is interpreted.

| | |
|---|---|
| Category | [immune](index.md#immune) |
| Canonical unit | g/L |
| LOINC | [2458-8](https://loinc.org/2458-8/) |
| Specimen | - |
| Aliases | `immunoglobulin a, serum` |

## Ranges

Bounds in g/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 18-56 | 0.72 | 3.75 | reference-interval-study | Iranian healthy adult blood donor volunteers aged 18-55 years (n=914), measured by nephelometry | [Normal value of immunoglobulins IgA, IgG, and IgM in Iranian healthy adults, measured by nephelometry.](https://pubmed.ncbi.nlm.nih.gov/14677654/), Abstract, Methods; [Normal value of immunoglobulins IgA, IgG, and IgM in Iranian healthy adults, measured by nephelometry.](https://pubmed.ncbi.nlm.nih.gov/14677654/), Abstract, Results | Central reference interval as reported (mg/dL). Single-country blood-donor cohort. Converted from the source's mg/dL (72-375) to g/L using factor 0.01. |
| hemochromatosis-probands-2026 | reference | all | 0.91 | 4.14 | consensus | Adults in a hemochromatosis proband study; the limits are the 1996 consensus reference limits it adopted, not hemochromatosis-specific data | [Serum IgA and IgM levels in hemochromatosis probands with HFE p.C282Y homozygosity.](https://pubmed.ncbi.nlm.nih.gov/42340960/), Full text, Methods | Serum IgA reference limits (91-414 mg/dL) as adopted from 1996 consensus guidelines. Sexes and ages not partitioned in the quoted limits. |
| iga-coeliac-flowchart-2026 | warn | all | 0.2 | - | cohort | Patients with suspected coeliac disease undergoing serology (flowchart in a 2026 scoping review) | [Immunoglobulin a Deficiency in Coeliac Disease: A Scoping Review.](https://pubmed.ncbi.nlm.nih.gov/42820738/), Full text, Figure legend (flowchart) | Serum IgA cut-off used in the review's coeliac testing flowchart because IgA-based antibody tests may give false negatives. The quote states the cut-off value but not its direction; the review applies it to low serum IgA results. Evidence is a scoping review, not a meta-analysis. Source kind: decision. Source band <= 0.2; stored as the warn low bound 0.2. |
| iga-deficiency-review-2026 | warn | all | 0.07 | - | cohort | People older than 4 years with suspected selective IgA deficiency (as defined in a 2026 scoping review) | [Immunoglobulin a Deficiency in Coeliac Disease: A Scoping Review.](https://pubmed.ncbi.nlm.nih.gov/42820738/), Full text, Introduction | Selective IgA deficiency: serum IgA below 0.07 g/L (strict) with normal IgM and IgG. Evidence is a scoping review of the literature, not a meta-analysis; the definition is the one it cites as current. Source kind: decision. Source band <= 0.07; stored as the warn low bound 0.07. |
| spain-multicentre-2007 | reference | all | 0.89 | 4.8 | reference-interval-study | Adults (women and men) from 21 Spanish clinical laboratories, measured with Tina-Quant reagents on RD/Hitachi analysers | [Multicentre physiological reference intervals for serum concentrations of immunoglobulins A, G and M, complement C3c and C4 measured with Tina-Quant reagents systems.](https://pubmed.ncbi.nlm.nih.gov/17378738/), Abstract, Results and conclusions | Multicentre adult reference interval (sexes combined). Assay-specific; check against the local assay's own interval. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
