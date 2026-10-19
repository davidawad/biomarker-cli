# Universe markers without a verified range (pending)

Each slug below has a marker file so values can be stored, with `no_reference = "pending: no verified range yet (see UNCOVERED.md)"`.
`biomarker catalog lint` reports them under the `pending` code. Research found no source that met the rules (every bound verbatim in a cited quote with its direction stated, resolving PMID/DOI, no derived bands, no min-max or mean +/- SD of small samples). Retried with Sonnet for every core and common slug, and again for all slugs in 8h7.5 (2026-10-19).

| slug | priority | why no range yet |
|---|---|---|
| alanine | common | only neonatal dried-blood-spot intervals (PMIDs 42739196, 34977148); no plasma or adult interval |
| arachidonic-acid | common | only PMID 37746532 (n=56/108, limits look like CIs, not open access) |
| basal-metabolic-rate | common | only prediction-equation validation papers; no reference interval |
| bcaa-total | common | no source states a total BCAA interval (per-analyte only) |
| bmd-femoral-neck | common | no g/cm2 interval; WHO T-score cut-offs are on bmd-femoral-neck-t-score |
| bmd-lumbar-spine | common | no g/cm2 interval; WHO T-score cut-offs are on bmd-lumbar-spine-t-score |
| body-water-percent | common | only BIA phase-angle/body-composition papers; no healthy-adult % range |
| cysteine | common | only cardiovascular-risk cohorts; Lepage 1997 (PMID 9439460) gives cystine in graphs only |
| dha | common | same as arachidonic-acid; EPA+DHA combined zones live on omega-3-index |
| epa | common | same as dha |
| estimated-average-glucose | common | ADAG (PMID 18540046) gives only the eAG regression and an HbA1c mapping, not a cut-off; ADA Standards not open access |
| hip-circumference | common | only waist-hip-ratio papers; no hip-only interval |
| lean-body-mass | common | EWGSOP2 limits (PMID 30312372) are appendicular muscle mass, not total lean mass; FFM percentile papers not accessible |
| light-sleep-duration | common | meta-analyses (15586779, 31006560) give only per-decade changes |
| linoleic-acid | common | no % of total fatty acids interval; a mol% phospholipid target (cystic fibrosis) is on linoleic-acid-mol |
| lysine | common | PMID 42568356 gives mean/SD only; GA-I guideline gives no lysine limits |
| omega6-omega3-ratio | common | no numeric target or threshold in accessible text |
| phenoage | common | no published threshold or reference interval; only NHANES association studies |
| phenoage-acceleration | common | cohort residual; any band would be derived |
| time-awake | common | meta-analyses give only per-decade WASO changes; no wearable threshold |
| tryptophan | common | no numeric reference limits found |
| 5-nucleotidase | specialist | only a 1984 pregnancy paper (PMID 6516805) with no numbers in abstract |
| afib-burden | specialist | ESC/EHRA consensus (PMID 40073206) states actionable burden thresholds are not defined |
| apoc3 | specialist | PMID 11580913 mean +/- SD only; 16205021 no numbers in abstract |
| asparagine | specialist | only Lepage 1997 graphical quantiles, no numbers in text |
| basal-body-temperature | specialist | only small-study mean +/- SD (PMID 41624353) |
| base-excess | specialist | PMID 21619466 has arterial BE intervals but no numbers in abstract and no full text |
| beta-globulin | specialist | SPE RI studies (36039308, 39736375) give beta-1/beta-2 only; a total would be derived |
| betaine | specialist | Northern China RI study (PMID 40157700) has no numbers in abstract and no open full text |
| biotin | specialist | only method papers and small diet-study means |
| bone-age | specialist | no consensus numeric cut-off found |
| estriol | specialist | only MoM-based screening papers; non-pregnant LC-MS/MS shows estriol undetectable (PMID 31720688) |
| ft3-rt3-ratio | specialist | no ratio reference interval published; computing one is not allowed |
| grimage | specialist | same as phenoage |
| hdl-large-p | specialist | NMR subclass thresholds found belong to ldl-p/ldl-size; PMID 37442713 no numbers or full text |
| histidine | specialist | only Lepage 1997 graphical quantiles, no numbers in text |
| horvath-epigenetic-clock | specialist | same as phenoage |
| klemera-doubal-biological-age | specialist | same as phenoage |
| ldl-small-p | specialist | same as hdl-large-p |
| magnesium-rbc | specialist | PMIDs 3608148, 10612712 no numeric intervals accessible; only small-sample mean +/- SD |
| molybdenum | specialist | CALIPER whole-blood study (PMID 37207688) has no numbers in abstract and no open full text |
| niacin | specialist | only method papers and small diet-study means |
| ornithine | specialist | PMID 26407660 is min-max of n=95; HHH review (25874378) gives no limits |
| pantothenic-acid | specialist | only method papers and small diet-study means |
| proline | specialist | large RI studies (25829462, 28278231, 40469518) keep values in tables not retrievable as text |
| s-adenosylhomocysteine | specialist | same as s-adenosylmethionine |
| s-adenosylmethionine | specialist | only small samples or mean +/- SD (PMIDs 35629877, 15919880, 23499573) |
| serine | specialist | same as proline |
| stool-ph | specialist | no source stating a limit; only microbiome and diet studies |
| t3-uptake | specialist | no % interval; a TBI interval (PMID 41008732) is on t3-uptake-tbi |
| threonine | specialist | same as proline |
| trans-fatty-acid-index | specialist | no numeric cut-off or reference interval found |
| urine-granular-casts | specialist | no healthy-adult per-LPF limit; only case reports and method papers |
| urine-phosphorus | specialist | spot mg/dL interval not found; 24h excretion (PMID 40080792) is on urine-phosphorus-24h |
| urine-pyruvate | specialist | no source with a numeric limit |
| urine-succinate | specialist | no source with a numeric limit |
| urine-urea-nitrogen | specialist | spot mg/dL interval not found; 24h excretion (PMID 40080792) is on urine-urea-24h |
| vldl-size | specialist | same as hdl-large-p |
| wrist-temperature-deviation | specialist | wearable papers (38544080, 41840158) are detection models with no deviation cut-off |
| p-tau217-ratio | specialist | named in the Alzheimer's Association blood-biomarker guideline (PMID 40729527); no verbatim cut-off found |
| p-tau181 | specialist | no verbatim cut-off or reference interval found |
| p-tau231 | specialist | no verbatim cut-off or reference interval found |
| abeta42-40-ratio | specialist | cut-offs are assay-specific; guideline cautions against single cut-offs; none stated verbatim |
