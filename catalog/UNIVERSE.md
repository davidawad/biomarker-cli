# Biomarker universe

`catalog/universe.toml` is the master list of biomarkers the catalog should eventually cover:
human, in vivo, measurable today in clinical or consumer settings. `biomarker catalog lint
--universe` reports universe entries that have no `catalog/markers/<slug>.toml` file yet. Each
entry has `slug`, `name`, `loinc`, `category`, `priority` (core | common | specialist) and
`derived`. An entry with an empty `loinc` also has a `loinc_note` explaining why.

## Sources

- **Slugs:** the 71 existing markers in `src/seed.rs` (`MARKERS`, `VITALS_MARKERS`) keep their
  slugs unchanged (for example `hscrp`, `lpa`, `co2`, `egfr`).
- **Scope:** standard US/EU lab menus (CMP, lipid, CBC with differential, thyroid, hormone, iron,
  coagulation, urinalysis and serology panels), preventive and consumer test panels (ApoB, Lp(a),
  omega-3 index, NMR particles, GlycA, toxic metals, amino and organic acids), pregnancy and
  newborn screening, and wearable and sleep metrics.
- **Codes:** every LOINC code comes from the NLM Clinical Tables LOINC API
  (`https://clinicaltables.nlm.nih.gov/api/loinc_items/v3/search`).

Exotic research-only assays are excluded (for example stool zonulin and cholesterol efflux
capacity).

## Method

1. **Drafting:** twelve Haiku subagents ran in parallel, one per category group, and each wrote a
   JSON fragment. A shared lookup helper served them; it caches results and holds all callers to
   one API request per 0.35 s. The agents had to confirm each code's `LONG_COMMON_NAME`
   (analyte, specimen, property) before using it.
2. **Merge:**
   - Entries were deduplicated by slug (first fragment wins) and by LOINC code.
   - An entry whose code belonged to an existing seed marker was renamed to that marker's slug.
   - All 71 seed slugs were confirmed present.
   - Slug format, category vocabulary and priority were checked.
3. **Verification:**
   - Each code was resolved through the API.
   - Its long name was compared with the marker name by token overlap.
   - The 40 token mismatches were all synonyms (urate/uric acid, lutropin/LH, thyrotropin/TSH and
     so on) and were checked by hand.
   - The full list was then reviewed by hand for specimen and property errors.
   - Finally, a fresh pass with no cache re-fetched all codes: every one resolves and none is
     deprecated.
4. **Corrections made during review:**
   - `vitamin-d`: 1989-3 (25-OH vitamin D3 only) → 62292-8 (25-OH D2+D3 total).
     **`src/seed.rs` still has 1989-3.**
   - `factor-viii`: blank → 3209-4 (activity, actual/normal).
   - `protein-c`: 4671-4 (antigen) → 27818-4 (activity).
   - `ige-timothy-grass`: 61296-0 (ratio to total IgE) → 6265-3 (IgE Ab, units/volume).
   - `afp-maternal-serum`: 1834-1 (same code as `afp`) → 20450-3 (MoM).
   - `cac-score`, `bone-age`, `hip-circumference`: blanked. Only procedure or protocol codes
     exist (79087-3, 85151-9, 62409-8).
   - `metabolic-syndrome-score`: blanked. 49898-0 is a presence flag, not a score.
   - `rdw-sd`: blank. 21000-5 is deprecated and has no replacement.
   - Duplicates dropped: `total-iga` (same as `iga`), `fat-free-mass` (same as
     `lean-body-mass`), `hrv-sdnn` (`hrv` is already SDNN, 80404-7).
   - Salivary cortisol is a single entry: LOINC has no code for collection time.
   - Added: `hrv-rmssd`, `peth`, `apoc3`, `pregnenolone`, `hbs`.
5. **Approximate codes:** some codes are the closest available fit rather than exact. These are
   NMR particle counts (`ldl-p`, `hdl-p`, `ldl-small-p` use the Lipoprotein.beta/alpha codes),
   `homa-ir` (the generic HOMA code), the CGM metrics (interstitial-fluid codes) and
   `lupus-anticoagulant` (an interpretation code). Each was kept because LOINC has nothing closer.
   The catalog marker files can refine them.

Derived entries (ratios, formulas, risk scores, clocks) have `derived = true`. Most have no LOINC
code. Where LOINC defines a calculated code (eGFR, FIB-4, MELD, KFRE, HOMA, ROMA, PHI, 4Kscore,
FAI, the BMI percentile), that code is used.

## Counts

627 entries; 561 with a verified LOINC code, 66 without (each has a loinc_note).

| Category | core | common | specialist | derived | total |
|---|---:|---:|---:|---:|---:|
| metabolic | 1 | 3 | 9 | 2 | 13 |
| glycemic | 2 | 8 | 10 | 11 | 20 |
| lipid | 7 | 9 | 13 | 11 | 29 |
| liver | 5 | 8 | 7 | 7 | 20 |
| pancreas | 0 | 0 | 3 | 0 | 3 |
| kidney | 4 | 5 | 6 | 8 | 15 |
| electrolyte | 6 | 1 | 3 | 4 | 10 |
| mineral | 1 | 2 | 3 | 0 | 6 |
| protein | 2 | 2 | 9 | 3 | 13 |
| hematology | 11 | 17 | 13 | 3 | 41 |
| coagulation | 0 | 4 | 9 | 1 | 13 |
| iron | 4 | 2 | 4 | 2 | 10 |
| vitamin | 3 | 8 | 8 | 0 | 19 |
| trace-element | 0 | 3 | 5 | 0 | 8 |
| toxic-metal | 1 | 0 | 10 | 0 | 11 |
| thyroid | 3 | 3 | 9 | 2 | 15 |
| adrenal | 1 | 1 | 19 | 1 | 21 |
| sex-hormone | 2 | 4 | 13 | 3 | 19 |
| pituitary | 0 | 1 | 3 | 0 | 4 |
| growth | 0 | 1 | 2 | 0 | 3 |
| bone | 0 | 5 | 7 | 3 | 12 |
| inflammation | 3 | 2 | 7 | 0 | 12 |
| immune | 0 | 6 | 10 | 1 | 16 |
| autoimmune | 0 | 4 | 29 | 0 | 33 |
| allergy | 0 | 9 | 2 | 0 | 11 |
| cardiac | 1 | 9 | 9 | 2 | 19 |
| tumor-marker | 0 | 1 | 16 | 4 | 17 |
| infectious | 0 | 16 | 23 | 0 | 39 |
| urinalysis | 11 | 3 | 16 | 1 | 30 |
| stool | 0 | 4 | 3 | 0 | 7 |
| fatty-acid | 0 | 5 | 4 | 3 | 9 |
| amino-acid | 0 | 15 | 10 | 2 | 25 |
| organic-acid | 0 | 1 | 9 | 0 | 10 |
| methylation | 1 | 0 | 4 | 0 | 5 |
| pregnancy | 5 | 1 | 9 | 2 | 15 |
| pediatric | 4 | 1 | 2 | 2 | 7 |
| body-composition | 5 | 8 | 5 | 7 | 18 |
| vitals | 4 | 5 | 1 | 2 | 10 |
| wearable | 2 | 4 | 4 | 3 | 10 |
| sleep | 1 | 7 | 2 | 1 | 10 |
| fitness | 1 | 0 | 3 | 1 | 4 |
| aging | 0 | 3 | 5 | 7 | 8 |
| genetic | 0 | 0 | 7 | 0 | 7 |
| **total** | 91 | 191 | 345 | 99 | 627 |
