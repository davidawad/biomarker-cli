# Calcitonin (`calcitonin`)

Hormone secreted by thyroid C-cells; the tumour marker for medullary thyroid carcinoma (MTC). Basal levels are sex-dependent and assay-dependent, and values in the 10-100 pg/mL range need follow-up rather than a diagnosis of MTC.

| | |
|---|---|
| Category | [thyroid](index.md#thyroid) |
| Canonical unit | pg/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in pg/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male | - | 8.4 | reference-interval-study | Basal serum calcitonin reference limits as reported for a clinical laboratory, Germany (pre-June 2019 assay) (method: immunoassay (Siemens Immulite 2000)) | [The Pitfalls of Calcitonin as a Tumor Marker: Real-Life Data of Patients with Elevated Basal Calcitonin Levels but Without Evidence of Medullary Thyroid Carcinoma.](https://pubmed.ncbi.nlm.nih.gov/41976801/), Methods | Upper normal limit reported by the cohort laboratory; assay-specific and not an independently derived reference interval. |
| alt-1 | reference | male | - | 11.8 | reference-interval-study | Basal serum calcitonin reference limits as reported for a clinical laboratory, Germany (from June 2019 assay) (method: immunoassay (DiaSorin LIAISON Calcitonin II Gen)) | [The Pitfalls of Calcitonin as a Tumor Marker: Real-Life Data of Patients with Elevated Basal Calcitonin Levels but Without Evidence of Medullary Thyroid Carcinoma.](https://pubmed.ncbi.nlm.nih.gov/41976801/), Methods | Upper normal limit reported by the cohort laboratory after the assay change; assay-specific. Alternative to the default calcitonin-lab-ranges-2019 reference range for the same population. |
| alt-1 | reference | female | - | 4.8 | reference-interval-study | Basal serum calcitonin reference limits as reported for a clinical laboratory, Germany (from June 2019 assay) (method: immunoassay (DiaSorin LIAISON Calcitonin II Gen)) | [The Pitfalls of Calcitonin as a Tumor Marker: Real-Life Data of Patients with Elevated Basal Calcitonin Levels but Without Evidence of Medullary Thyroid Carcinoma.](https://pubmed.ncbi.nlm.nih.gov/41976801/), Methods | Upper normal limit reported by the cohort laboratory after the assay change; assay-specific. Alternative to the default calcitonin-lab-ranges-2019 reference range for the same population. |
| calcitonin-lab-ranges-2019 | reference | female | - | 5 | reference-interval-study | Basal serum calcitonin reference limits as reported for a clinical laboratory, Germany (pre-June 2019 assay) (method: immunoassay (Siemens Immulite 2000)) | [The Pitfalls of Calcitonin as a Tumor Marker: Real-Life Data of Patients with Elevated Basal Calcitonin Levels but Without Evidence of Medullary Thyroid Carcinoma.](https://pubmed.ncbi.nlm.nih.gov/41976801/), Methods | Upper normal limit reported by the cohort laboratory; assay-specific. |
| german-mtc-guidance-as-cited | warn | female | - | 30 | consensus | Adults with thyroid nodules and elevated basal calcitonin (German guidance as summarised in the review) | [Laboratory diagnostics and follow-up of medullary thyroid cancer.](https://pubmed.ncbi.nlm.nih.gov/41642846/), Reference ranges and cutoff levels | Sex-specific action threshold per German recommendations as cited by the review; intended to balance unnecessary surgery against missed micro-MTC. Not verified against the primary German guideline. Source kind: diagnostic. Source band >= 30; stored as the warn high bound 30. |
| german-mtc-guidance-as-cited | warn | male | - | 60 | consensus | Adults with thyroid nodules and elevated basal calcitonin (German guidance as summarised in the review) | [Laboratory diagnostics and follow-up of medullary thyroid cancer.](https://pubmed.ncbi.nlm.nih.gov/41642846/), Reference ranges and cutoff levels | Sex-specific action threshold per German recommendations as cited by the review; not verified against the primary German guideline. Source kind: diagnostic. Source band >= 60; stored as the warn high bound 60. |
| mtc-lab-review-2026 | reference | all | - | 10 | consensus | Healthy individuals (narrative review of MTC laboratory diagnostics) | [Laboratory diagnostics and follow-up of medullary thyroid cancer.](https://pubmed.ncbi.nlm.nih.gov/41642846/), Reference ranges and cutoff levels | Typical basal calcitonin in healthy people, as stated in the review; assay dependent. |
| mtc-lab-review-2026 | warn | all | - | 100 | consensus | Adults with elevated basal calcitonin (narrative review of MTC laboratory diagnostics) | [Laboratory diagnostics and follow-up of medullary thyroid cancer.](https://pubmed.ncbi.nlm.nih.gov/41642846/), Reference ranges and cutoff levels | Gray zone: values in this range are more often C-cell hyperplasia or non-specific than MTC; the review advises further work-up (e.g., FNA washout calcitonin). Source band 10-100; stored as the warn high bound 100. |
| mtc-lab-review-2026 | warn | all | - | 100 | consensus | Adults with basal calcitonin elevation (narrative review of MTC laboratory diagnostics) | [Laboratory diagnostics and follow-up of medullary thyroid cancer.](https://pubmed.ncbi.nlm.nih.gov/41642846/), Reference ranges and cutoff levels | Level above which basal calcitonin is considered indicative of MTC; the review stresses cut-offs are only indicative and assay-dependent. Source kind: diagnostic. Source band >= 100; stored as the warn high bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
