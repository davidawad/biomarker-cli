# 1,25-dihydroxyvitamin D (`calcitriol`)

Circulating 1,25-dihydroxyvitamin D [1,25(OH)2D], the active hormonal form of vitamin D made mainly by the kidney. It has a short half-life and does not reflect vitamin D stores; it is mainly used in calcium, PTH and kidney disorders, and low values in advanced CKD are associated with worse outcomes.

| | |
|---|---|
| Category | [vitamin](index.md#vitamin) |
| Canonical unit | pg/mL |
| LOINC | [1649-3](https://loinc.org/1649-3/) |
| Specimen | - |
| Aliases | `vitamin-d-1-25` |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | age 0-1 | 32.08 | 196.22 | reference-interval-study | Canadian children aged 0-1 year, CALIPER cohort (377 children and adolescents), DiaSorin Liaison XL method, as reported in a 2023 review (method: diasorin-liaison) | [Vitamin D Metabolites: Analytical Challenges and Clinical Relevance.](https://pubmed.ncbi.nlm.nih.gov/35238975/), Text on paediatric 1,25(OH)2D reference intervals | Pediatric interval; levels are higher in children than adults. Converted from pmol/L with factor 0.4166. |
| caliper | reference | age 1-3 | 47.07 | 151.22 | reference-interval-study | Canadian children aged 1-3 years, CALIPER cohort (377 children and adolescents), DiaSorin Liaison XL method, as reported in a 2023 review (method: diasorin-liaison) | [Vitamin D Metabolites: Analytical Challenges and Clinical Relevance.](https://pubmed.ncbi.nlm.nih.gov/35238975/), Text on paediatric 1,25(OH)2D reference intervals | Pediatric interval. Converted from pmol/L with factor 0.4166. |
| caliper | reference | age 3+ | 44.99 | 102.49 | reference-interval-study | Canadian children older than 3 years, CALIPER cohort (377 children and adolescents), DiaSorin Liaison XL method, as reported in a 2023 review (method: diasorin-liaison) | [Vitamin D Metabolites: Analytical Challenges and Clinical Relevance.](https://pubmed.ncbi.nlm.nih.gov/35238975/), Text on paediatric 1,25(OH)2D reference intervals | Pediatric interval for children older than 3 years (the source gives no upper age limit). Converted from pmol/L with factor 0.4166. |
| host-ckd-2012 | warn | ckd | 15 | - | cohort | Patients with advanced CKD (stages 4-5, not yet on dialysis) in the HOST randomized trial, 1,099 veterans, plasma samples | [Associations of plasma 25-hydroxyvitamin D and 1,25-dihydroxyvitamin D concentrations with death and progression to maintenance dialysis in patients with advanced kidney disease.](https://pubmed.ncbi.nlm.nih.gov/22621970/), Abstract, Methods | Statistical tertile cut-off, not a validated clinical threshold. Lowest tertile associated with death and dialysis initiation (HR 1.33 and 1.78 vs highest tertile). Reference-interval values above do not apply to this cut. Source band <= 15; stored as the warn low bound 15. |
| ids-isys | reference | all | 26.25 | 94.98 | consensus | Healthy adults; automated IDS iSYS immunoassay as reported in a 2023 review (method: chemiluminescence-immunoassay) | [Vitamin D Metabolites: Analytical Challenges and Clinical Relevance.](https://pubmed.ncbi.nlm.nih.gov/35238975/), Text on 1,25(OH)2D reference intervals | Method-specific interval, as reported by the review. Converted from pmol/L with factor 0.4166 (MW 416.6 g/mol). |
| ids-ria | reference | all | 17.91 | 69.99 | consensus | Healthy adults; IDS radioimmunoassay (RIA) as reported in a 2023 review (method: ria) | [Vitamin D Metabolites: Analytical Challenges and Clinical Relevance.](https://pubmed.ncbi.nlm.nih.gov/35238975/), Text on 1,25(OH)2D reference intervals | Method-specific interval, as reported by the review. Converted from pmol/L using MW 416.6 g/mol (1 pmol/L = 0.4166 pg/mL). Reference intervals are method-specific. |
| lc-ms-ms-review | reference | all | 24.58 | 66.24 | consensus | Healthy adults; LC-MS/MS as reported in a 2023 review (typically measures 1,25(OH)2D3 only) (method: lc-ms-ms) | [Vitamin D Metabolites: Analytical Challenges and Clinical Relevance.](https://pubmed.ncbi.nlm.nih.gov/35238975/), Text on 1,25(OH)2D reference intervals | Method-specific interval, as reported by the review. Converted from pmol/L with factor 0.4166 (MW 416.6 g/mol). |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
