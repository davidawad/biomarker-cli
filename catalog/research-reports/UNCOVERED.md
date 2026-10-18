# Universe markers without a verified range (pending)

Each slug below has a marker file so values can be stored, with `no_reference = "pending: no verified range yet (see UNCOVERED.md)"`.
`biomarker catalog lint` reports them under the `pending` code. Research found no source that met the rules (every bound verbatim in a cited quote with its direction stated, resolving PMID/DOI, no derived bands, no min-max or mean +/- SD of small samples). Retried with Sonnet for every core and common slug.

| slug | priority | why no range yet |
|---|---|---|
| phenoage | common | quantitative biological age; no verified reference interval |
| phenoage-acceleration | common | quantitative; no verified reference interval |
| alanine | common | only neonatal DBS 0-28 d interval found (PMID 42739196); no plasma or adult interval |
| bcaa-total | common | no total BCAA interval retrievable (per-analyte values only in supplements) |
| cysteine | common | only control means and cohort quartiles found; no reference limits |
| isoleucine | common | same as glycine |
| lysine | common | same as glycine |
| methionine | common | same as glycine |
| tryptophan | common | no numeric source found |
| basal-metabolic-rate | common | quantitative; no verified reference interval (moved from no_reference) |
| body-water-percent | common | only abstract-level total body water data, no retrievable numeric values |
| hip-circumference | common | no verified interval for hip circumference alone |
| lean-body-mass | common | no verified reference interval |
| bmd-femoral-neck | common | T-score cut-offs identified (PMIDs 21234807, 24577348) but not yet written; rerun with the fixed verifier |
| bmd-lumbar-spine | common | same as bmd-femoral-neck |
| omega6-omega3-ratio | common | quantitative ratio; sources give no numeric target |
| estimated-average-glucose | common | quantitative formula of HbA1c; no independent verified interval |
| light-sleep-duration | common | only per-decade changes in abstracts; no absolute reference values retrievable |
| time-awake | common | no numeric threshold defined for time awake as a wearable metric |
| dopamine | specialist | no numeric plasma reference interval found |
| epinephrine | specialist | no numeric plasma reference interval found (only a single-cohort mean +/- SD) |
| norepinephrine | specialist | no numeric plasma reference interval found |
| grimage | specialist | quantitative epigenetic age; no verified reference interval |
| horvath-epigenetic-clock | specialist | quantitative epigenetic age; no verified reference interval |
| klemera-doubal-biological-age | specialist | quantitative biological age; no verified reference interval |
| asparagine | specialist | only observed min-max in 95 volunteers; no reference interval |
| histidine | specialist | only observed min-max in 95 volunteers; no reference interval |
| ornithine | specialist | only mean/SD and min-max in small samples; no reference interval |
| proline | specialist | same as ornithine |
| serine | specialist | same as ornithine |
| threonine | specialist | same as ornithine |
| myoglobin | specialist | only pediatric interval source has no numbers in abstract and no open full text |
| urine-potassium | specialist | no urine potassium reference interval retrievable (only mean excretion and dietary intake) |
| urine-sodium | specialist | no urine sodium interval in mmol/L retrievable (only estimated intake in g/day) |
| trans-fatty-acid-index | specialist | quantitative index; only group means found |
| sit-to-stand | specialist | norms only in unreachable full text; other sources are mean +/- SD or not in abstract |
| homa-b | specialist | only cut-offs found (PMID 25794879) whose risk direction is not stated in the abstract; removed rather than inferred |
| igg-subclass-1 | specialist | no verifiable numeric IgG1 interval in retrievable text |
| urine-creatinine | specialist | no verifiable numeric reference interval found |
| apoc3 | specialist | reference-interval papers have values only in inaccessible tables |
| hdl-large-p | specialist | no numeric large-HDL-P interval in accessible text |
| ldl-hdl-ratio | specialist | quantitative ratio; no verifiable cut-off |
| ldl-small-p | specialist | no verifiable numeric reference or cut-off found |
| vldl-size | specialist | no VLDL particle size interval found |
| 5-nucleotidase | specialist | no verifiable interval found |
| adiponectin | specialist | only a pediatric paper with no numbers in abstract and no open full text |
| base-excess | specialist | only venous interval found; marker is arterial |
| ph | specialist | no verifiable numeric arterial pH interval |
| betaine | specialist | only a 1994 statement without sample size or percentile basis |
| s-adenosylhomocysteine | specialist | only mean +/- 2 SD of small samples |
| s-adenosylmethionine | specialist | only mean +/- SD or small-sample ranges |
| magnesium-rbc | specialist | only mean +/- SD (n=20) or small observed min-max |
| urine-pyruvate | specialist | no numeric urine reference interval found |
| urine-succinate | specialist | only a group comparison, no reference interval |
| bone-age | specialist | no verifiable numeric reference interval or cut-off |
| sflt-1 | specialist | only sFlt-1/PlGF ratio cut-offs or a threshold with no stated direction |
| beta-globulin | specialist | only separate beta-1 and beta-2 intervals; no total beta bound |
| estriol | specialist | only small or historical pregnancy studies; no verifiable interval |
| stool-ph | specialist | only mean +/- SD of 14 volunteers |
| ft3-rt3-ratio | specialist | calculated ratio; no verifiable source |
| reverse-t3 | specialist | only source (PMID 32523672) gives an implausible upper limit (195 ng/dL); held back pending a second source |
| t3-uptake | specialist | no numeric reference interval found |
| molybdenum | specialist | blood-specific data is an observed range only; serum intervals are the wrong specimen |
| ca-27-29 | specialist | no verifiable numeric reference interval or cut-off |
| urine-chloride | specialist | no verifiable numeric urine chloride interval |
| urine-granular-casts | specialist | no verifiable interval |
| urine-phosphorus | specialist | only source blocked or mean +/- SD |
| urine-urea-nitrogen | specialist | no verifiable numeric reference found |
| basal-body-temperature | specialist | no verifiable reference interval; only phase-curve endpoints from 57 women |
| biotin | specialist | only a statement without sample size or derivation |
| niacin | specialist | no numeric source found |
| pantothenic-acid | specialist | only mean +/- SD of 63 |
| afib-burden | specialist | no published burden cut-off; only episode-duration thresholds |
| nocturnal-spo2 | specialist | no verified nocturnal SpO2 cut-off or reference interval |
| wrist-temperature-deviation | specialist | vendor-defined deviation from personal baseline; no numeric cut-off |
| arachidonic-acid | common | only a Table 4 quote (PMID 22254080) that is not in retrievable text; range dropped |
| dha | common | same as arachidonic-acid |
| epa | common | same as arachidonic-acid |
| linoleic-acid | common | same as arachidonic-acid |
| trab-uln-multiple | specialist | its single multiple-of-ULN range lacked a verifiable bound in its quote |
