# Serum Free Light Chain Kappa/Lambda Ratio (`kappa-lambda-ratio`)

The ratio of kappa to lambda free light chains in serum. A ratio far from the normal range suggests a monoclonal plasma-cell disorder such as myeloma, but kidney function changes the normal range.

| | |
|---|---|
| Category | [immune](index.md#immune) |
| Canonical unit | ratio |
| LOINC | [48378-4](https://loinc.org/48378-4/) |
| Specimen | - |
| Aliases | `sflc ratio`, `kappa/lambda ratio`, `kappa:lambda free light chain ratio`, `free-light-chain-ratio` |

## Ranges

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0.26 | 1.65 | reference-interval-study | Reference serum samples (n=282, USA) as cited in a 2020 renal free light chain study | [The renal range of the κ/λ sFLC ratio: best strategy to evaluate multiple myeloma in patients with chronic kidney disease.](https://pubmed.ncbi.nlm.nih.gov/32234026/), Full text, Introduction | Polyclonal reference range for the kappa/lambda ratio. Taken from a 2020 paper that cites Katzmann et al. 2002; the primary Katzmann paper was not retrieved. |
| afr-descent-2025 | reference | all | 0.686 | 2.1 | reference-interval-study | Individuals of African descent with normal renal function (Blood 2025 multi-population study) | [Serum free light chains in a racially diverse population including African Americans and populations from South Africa.](https://pubmed.ncbi.nlm.nih.gov/39571144/), Abstract, Results | Proposed ratio range for people of African descent with normal kidney function, to reduce overdiagnosis of light-chain gammopathy. A proposal from one study rather than a guideline. Applies only to: ethnicity=black. |
| hutchison-renal-cited-2020 | reference | ckd | 0.37 | 3.1 | reference-interval-study | Patients with chronic kidney disease (polyclonal range for CKD) | [The renal range of the κ/λ sFLC ratio: best strategy to evaluate multiple myeloma in patients with chronic kidney disease.](https://pubmed.ncbi.nlm.nih.gov/32234026/), Full text, Introduction | Renal-adjusted polyclonal range, as cited in a 2020 paper. That paper states the range has not been validated in a large representative sample; the original Hutchison et al. paper was not retrieved. |
| imwg-2016 | warn | all | - | 100 | consensus | Patients with suspected multiple myeloma (IMWG myeloma-defining event criteria) | [Updated Diagnostic Criteria and Staging System for Multiple Myeloma.](https://pubmed.ncbi.nlm.nih.gov/27249749/), Abstract | Myeloma-defining biomarker: involved/uninvolved FLC ratio at or above 100, provided the involved free light chain is at least 100 mg/L. The mg/L condition is not captured by this ratio range; check it separately. The cited text is a 2016 review of the IMWG criteria. Source kind: decision. Source band >= 100; stored as the warn high bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
