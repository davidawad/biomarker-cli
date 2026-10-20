# Universe markers without a verified range (pending)

Each slug below has a marker file so values can be stored, with `no_reference = "pending: no verified range yet (see UNCOVERED.md)"`.
`biomarker catalog lint` reports them under the `pending` code. Research found no source that met the rules (every bound verbatim in a cited quote with its direction stated, resolving PMID/DOI, no derived bands, no min-max or mean +/- SD of small samples). Retried with Sonnet for every core and common slug, and again for all slugs in 8h7.5 (2026-10-19). Access rechecked 2026-10-20: OpenAlex OA locations were queried for cited PMIDs; Europe PMC fullTextXML, PMC supplementary files, and accessible repository or publisher PDFs were attempted where available.

| slug | priority | why no range yet |
|---|---|---|
| basal-metabolic-rate | common | only prediction-equation validation papers; no reference interval; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| bcaa-total | common | no source states a total BCAA interval (per-analyte only); OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| bmd-femoral-neck | common | no g/cm2 interval; WHO T-score cut-offs are on bmd-femoral-neck-t-score; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| bmd-lumbar-spine | common | no g/cm2 interval; WHO T-score cut-offs are on bmd-lumbar-spine-t-score; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| body-water-percent | common | only BIA phase-angle/body-composition papers; no healthy-adult % range; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| estimated-average-glucose | common | ADAG (PMID 18540046) gives only the eAG regression and an HbA1c mapping, not a cut-off; ADA Standards not open access; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| hip-circumference | common | only waist-hip-ratio papers; no hip-only interval; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| lean-body-mass | common | EWGSOP2 limits (PMID 30312372) are appendicular muscle mass, not total lean mass; FFM percentile papers not accessible; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| light-sleep-duration | common | meta-analyses (15586779, 31006560) give only per-decade changes; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| omega6-omega3-ratio | common | no numeric target or threshold in accessible text; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| phenoage | common | no published threshold or reference interval; only NHANES association studies; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| phenoage-acceleration | common | cohort residual; any band would be derived; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| time-awake | common | meta-analyses give only per-decade WASO changes; no wearable threshold; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| 5-nucleotidase | specialist | only a 1984 pregnancy paper (PMID 6516805) with no numbers in abstract; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| afib-burden | specialist | ESC/EHRA consensus (PMID 40073206) states actionable burden thresholds are not defined; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| asparagine | specialist | only Lepage 1997 graphical quantiles, no numbers in text; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| basal-body-temperature | specialist | only small-study mean +/- SD (PMID 41624353); OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| base-excess | specialist | PMID 21619466 has no numeric abstract bounds; the Aarhus repository page provided an abstract but no PDF/full text. No verifiable BE interval. |
| beta-globulin | specialist | SPE RI studies (36039308, 39736375) give beta-1/beta-2 only; a total would be derived; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| betaine | specialist | Northern China RI study (PMID 40157700) has no numbers in abstract and no open full text; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| biotin | specialist | only method papers and small diet-study means; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| bone-age | specialist | no consensus numeric cut-off found; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| estriol | specialist | only MoM-based screening papers; non-pregnant LC-MS/MS shows estriol undetectable (PMID 31720688); OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| ft3-rt3-ratio | specialist | no ratio reference interval published; computing one is not allowed; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| grimage | specialist | same as phenoage; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| hdl-large-p | specialist | NMR subclass thresholds found belong to ldl-p/ldl-size; PMID 37442713 no numbers or full text; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| horvath-epigenetic-clock | specialist | same as phenoage; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| klemera-doubal-biological-age | specialist | same as phenoage; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| ldl-small-p | specialist | same as hdl-large-p; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| magnesium-rbc | specialist | PMIDs 3608148, 10612712 no numeric intervals accessible; only small-sample mean +/- SD; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| molybdenum | specialist | CALIPER whole-blood study (PMID 37207688) has no numbers in abstract and no open full text; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| niacin | specialist | only method papers and small diet-study means; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| pantothenic-acid | specialist | only method papers and small diet-study means; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| s-adenosylhomocysteine | specialist | same as s-adenosylmethionine; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| s-adenosylmethionine | specialist | only small samples or mean +/- SD (PMIDs 35629877, 15919880, 23499573); OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| stool-ph | specialist | no source stating a limit; only microbiome and diet studies; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| t3-uptake | specialist | no % interval; a TBI interval (PMID 41008732) is on t3-uptake-tbi; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| trans-fatty-acid-index | specialist | no numeric cut-off or reference interval found; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| urine-granular-casts | specialist | no healthy-adult per-LPF limit; only case reports and method papers; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| urine-phosphorus | specialist | spot mg/dL interval not found; 24h excretion (PMID 40080792) is on urine-phosphorus-24h; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| urine-pyruvate | specialist | no source with a numeric limit; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| urine-succinate | specialist | no source with a numeric limit; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| urine-urea-nitrogen | specialist | spot mg/dL interval not found; 24h excretion (PMID 40080792) is on urine-urea-24h; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| vldl-size | specialist | same as hdl-large-p; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| wrist-temperature-deviation | specialist | wearable papers (38544080, 41840158) are detection models with no deviation cut-off; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| p-tau217-ratio | specialist | named in the Alzheimer's Association blood-biomarker guideline (PMID 40729527); no verbatim cut-off found; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| p-tau181 | specialist | no verbatim cut-off or reference interval found; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| p-tau231 | specialist | no verbatim cut-off or reference interval found; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
| abeta42-40-ratio | specialist | cut-offs are assay-specific; guideline cautions against single cut-offs; none stated verbatim; OpenAlex/Europe PMC full-text and OA-location recheck (2026-10-20) found no additional eligible quote-backed range |
