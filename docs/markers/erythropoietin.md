# Erythropoietin (`erythropoietin`)

Glycoprotein hormone that drives red cell production, measured by immunoassay. Used in the workup of anemia and erythrocytosis, including polycythemia vera versus secondary erythrocytosis. Expected result: within the reference interval; subnormal values favour polycythemia vera over secondary causes.

| | |
|---|---|
| Category | [hematology](index.md#hematology) |
| Canonical unit | mIU/mL |
| LOINC | [15061-5](https://loinc.org/15061-5/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mIU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 5.3 | 26.3 | reference-interval-study | Healthy donors (n=393); age not stated in the abstract | [Diagnostic Performance of Serum Erythropoietin to Discriminate Polycythemia Vera from Secondary Erythrocytosis through Established Subnormal Limits.](https://pubmed.ncbi.nlm.nih.gov/39272689/), Abstract (Results) | 2.5th-97.5th percentile of healthy donors. IU/L is numerically equal to mIU/mL. Converted from the source's IU/L (5.3-26.3) to mIU/mL using factor 1.0. |
| clinical | reference | age 0-17 | 9.1 | 27.6 | reference-interval-study | Healthy children aged 1 month to 16 years | [Serum immunoreactive erythropoietin in healthy normal children.](https://pubmed.ncbi.nlm.nih.gov/3191035/), Abstract (Results) | Pediatric 95% range (geometric-mean study, n=130). Adult values from the same study (n=22) omitted as too small a sample. Converted from the source's miu/mL (9.1-27.6) to mIU/mL using factor 1.0. |
| diagnostic | warn | polycythemia-vera | 3.1 | - | cohort | Patients with erythrocytosis (41 polycythemia vera, 49 secondary erythrocytosis) | [Diagnostic Performance of Serum Erythropoietin to Discriminate Polycythemia Vera from Secondary Erythrocytosis through Established Subnormal Limits.](https://pubmed.ncbi.nlm.nih.gov/39272689/), Abstract (Results) | Clinical decision limit from ROC analysis: values below this favour polycythemia vera (sensitivity 80.5%, specificity 87.8%). Single study. Converted from the source's IU/L (3.1) to mIU/mL using factor 1.0. Source kind: diagnostic. Source band <= 3.1; stored as the warn low bound 3.1. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
