# Carcinoembryonic Antigen (`cea`)

Carcinoembryonic antigen, a glycoprotein marker of some gastrointestinal and other cancers. It is used mainly to monitor colorectal cancer after treatment; many non-cancer factors, including smoking, can raise it.

| | |
|---|---|
| Category | [tumour](index.md#tumour) |
| Canonical unit | ng/mL |
| LOINC | [2039-6](https://loinc.org/2039-6/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male, age 0-40 | - | 3.7 | reference-interval-study | Apparently healthy Chinese men under 40 (indirect reference-interval study, CLSI C28-A3 non-parametric method) | [Changes of CEA Diagnostic Performance Under the New Reference Interval Stratified by Age and Sex.](https://pubmed.ncbi.nlm.nih.gov/42389829/), Full text, Results | Age- and sex-specific upper reference limit for men under 40, below the previous laboratory interval. Boundary as quoted (< 3.7 ng/mL); derived with the study's own assay. |
| alt-1 | warn | all | - | 10 | consensus | Adults; colorectal cancer context (narrative review) | [Current opinions on CEA in the diagnosis and prognosis of colorectal cancer.](https://pubmed.ncbi.nlm.nih.gov/42214175/), Full text, CEA levels section | Borderline band: CEA 5-10 ng/mL can be produced by benign factors, so a value in this band needs context and repeat measurement. Source band 5-10; stored as the warn high bound 10. Alternative to the default cea-review-2026 warn range for the same population. |
| alt-1 | reference | all | - | 10 | consensus | Adult smokers (narrative review of CEA in focal liver lesions) | [Carcinoembryonic antigen in the diagnosis, treatment, and follow-up of focal liver lesions.](https://pubmed.ncbi.nlm.nih.gov/38690060/), Full text, Introduction | Normal serum CEA in smokers taken as below 10 ng/mL (boundary < 10 as quoted). Smoking raises CEA, so the non-smoker limit of 5 ng/mL does not apply to smokers. Smoking status is not a schema condition, so this is described here and in population. Alternative to the default wjgs-cea-2024 reference range for the same population. |
| cea-review-2026 | warn | all | 5 | - | consensus | Adults; colorectal cancer context (narrative review) | [Current opinions on CEA in the diagnosis and prognosis of colorectal cancer.](https://pubmed.ncbi.nlm.nih.gov/42214175/), Full text, CEA levels section | Commonly used cut-off for CEA elevation. Values are not diagnostic on their own; smoking and other benign conditions can raise CEA. Boundary as quoted (5 ng/mL is the usual upper limit; the review does not state > versus >=). Source kind: decision. Source band <= 5; stored as the warn low bound 5. |
| elderly-tumour-ri-2024 | reference | age 60-99 | - | 4.85 | reference-interval-study | Apparently healthy elderly individuals aged 60 and above in southwestern China (indirect method); sexes combined in the table | [A preliminary study on the reference intervals of serum tumor marker in apparently healthy elderly population in southwestern China using real-world data.](https://pubmed.ncbi.nlm.nih.gov/38811867/), Full text, Table 3 (reference intervals of tumor markers for elderly population) | Upper reference limit (95th percentile) for CEA in adults aged 60 and over, as tabulated. Compare with the non-smoker limit of 5 ng/mL in the ranges above. |
| jcla-cea-2026 | reference | age 40-50 | - | 5 | reference-interval-study | Apparently healthy Chinese adults aged 40 to 50 (indirect reference-interval study, CLSI C28-A3 non-parametric method); sex-specific limits not separately quoted for this band | [Changes of CEA Diagnostic Performance Under the New Reference Interval Stratified by Age and Sex.](https://pubmed.ncbi.nlm.nih.gov/42389829/), Full text, Results | Upper reference limit reported for the 40-50 year group in this study. Boundary as quoted (< 5.0 ng/mL). Sex-specific values differ and are not separately quoted here. |
| wjgs-cea-2024 | reference | all | - | 5 | consensus | Adult non-smokers (narrative review of CEA in focal liver lesions) | [Carcinoembryonic antigen in the diagnosis, treatment, and follow-up of focal liver lesions.](https://pubmed.ncbi.nlm.nih.gov/38690060/), Full text, Introduction | Normal serum CEA in adult non-smokers taken as below 5 ng/mL (boundary < 5 as quoted). Smoking status is not a schema condition here, so the smoker band is a separate range. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
