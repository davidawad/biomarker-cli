# Research runbook (catalog coverage boxes)

Goal: add a cited marker for every assigned universe slug (`catalog/universe.toml`). Work from the
repo root. Do NOT hand-edit `catalog/markers/` for new markers: they are generated from research
shards by `tools/fold_research.py`.

## Layout
- One shard per marker: `catalog/research-reports/sources/<slug>.toml`, format in
  `tools/research/FORMAT.md`. The slug is the universe slug, exactly.
- `python3 -I tools/fold_research.py` folds all shards into `catalog/markers/*.toml`, `catalog/sets.toml`,
  `catalog/conditions.toml` and rewrites `catalog/research-reports/FOLD-DROPPED.tsv`. It is idempotent.
- Research tools and verifier: `tools/research/rt.py` (see `tools/research/preamble.md` for exact usage).
  The verifier checks: every PMID resolves via esummary with a matching title; every DOI resolves;
  every quote appears verbatim in retrieved text; every number in low/high/when.markers appears in a quote.
  Set `RT_CACHE` to a directory under the box artifacts dir so retrieved text is saved with the run.

## Method
1. Fan out Haiku subagents (Agent tool, model haiku), AT MOST 3 AT A TIME (more gets the box OOM-killed).
   Give each 3-6 slugs, with `tools/research/preamble.md` and `tools/research/FORMAT.md` as its
   instructions. Work core priority first, then common, then specialist.
2. A subagent writes only its shards and must get `rt.py verify` to print OK for each file.
3. The lead (you) then audits every range by hand for semantic correctness (citations can verify while
   the coding is wrong): sex/age partitions, one-sided bounds, reference vs warn vs diagnostic kinds,
   single means or IQRs mis-coded as reference intervals, bands borrowed from a neighbouring band.
   Drop or fix what is wrong. Then re-run `rt.py verify` on every touched shard.
4. Units: the shard `unit` is the canonical unit. If a source states numbers in another unit, set the
   range-level `unit` and make sure `tools/fold_research.py` CONV has a conversion for (slug, unit);
   add one only when the factor is exact and well known (comment it). If the quantity is genuinely
   different (percent vs absolute, multiples of ULN, activity vs mass, per-HPF vs per-uL), make it a
   separate companion marker slug and say so in the shard description. After folding,
   `FOLD-DROPPED.tsv` must list zero rows (header only). Two-sided bands that cannot be oriented are
   kept automatically as reference ranges in their own sets.
5. A marker with no verifiable source after a real effort: write no shard, list it as skipped with the
   reason in your report. Never invent a citation, PMID, DOI, quote or number.
6. A new marker slug you need that is not in the universe: add its entry (same fields as
   `catalog/universe.toml`; `loinc` only if verified through the NLM Clinical Tables LOINC API,
   otherwise `loinc = ""` with a `loinc_note`) to `catalog/research-reports/universe-additions-<bead>.toml`
   (do not edit `catalog/universe.toml`; the lead merges those).
7. Save shards and verifier output (`rt.py verify` over every touched shard) as box artifacts, and write
   a report `catalog/research-reports/<bead>-REPORT.md`: per category, markers added, ranges, evidence
   mix, skipped markers with reasons.
8. Commit shards, folded markers, sets/conditions, the report and FOLD-DROPPED.tsv with
   conventional-commit subjects, no attribution lines.

## Gate (on the box, before finishing)
`python3 -I tools/fold_research.py && cargo fmt --check && cargo clippy --all-targets -- -D warnings &&
cargo test && biomarker catalog lint --universe`, then regenerate `docs/markers` with
`biomarker catalog docs --out docs/markers`. Save cargo test and lint outputs (also `--json`) as box
artifacts. Fix every lint error; report warning counts.
