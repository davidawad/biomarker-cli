# catalog/research/<slug>.toml format (shard r1)

The epic text was not shipped to this box, so this follows the field list in the
bead brief (slug, name, loinc, category, unit, description, [[range]] with set,
kind, low, high, when, population, evidence, citations=[{pmid, doi, url,
locator, quote}], note), aligned to the existing engine (src/ranges.rs
RangeKind reference/optimal/warn, sex any/male/female, age bands, named sets).

```toml
slug = "hba1c"                     # existing slug from src/seed.rs if present; else lowercase-kebab
name = "Hemoglobin A1c"
loinc = "4548-4"                   # omit if unknown; never guess
category = "glycemic"              # glycemic | lipid | liver | kidney | electrolyte | mineral | chemistry
unit = "%"                         # canonical unit (seed.rs unit if the marker exists)
description = "1-3 sentences."

[[range]]
set = "clinical"                   # reference | clinical | optimal   (see below)
kind = "warn"                      # reference | optimal | warn | diagnostic | target | critical
low = 5.7                          # either bound may be omitted (one-sided)
high = 6.4
unit = "%"                         # OPTIONAL: only when the source states the numbers in a
                                   # different unit than the file's canonical unit; numbers are
                                   # copied exactly as the source states them, never converted
when = { }                         # conditions; omit keys that do not apply (empty = everyone)
population = "Nonpregnant adults"  # who the source's numbers apply to, in words
evidence = "guideline"             # guideline | consensus | reference-interval-study |
                                   # outcome-study | meta-analysis | textbook | regulatory
note = "Prediabetes (increased risk of diabetes)."
citations = [
  { pmid = "39651986", doi = "10.2337/dc25-S002", title = "<exact PubMed title>",
    url = "https://...", locator = "Table 2.1", quote = "<verbatim text containing 5.7 and 6.4>" },
]
```

## set
- `reference`: population reference interval (central 95 % of healthy people, e.g. CALIPER,
  NORIP, NHANES-derived, CLSI EP28 studies, Tietz).
- `clinical`: thresholds that guidelines attach meaning to: diagnostic cut-offs, risk
  categories, treatment targets, decision limits (ADA, ACC/AHA, ESC/EAS, KDIGO, AASLD, NICE...).
- `optimal`: tighter "best outcome" range justified by outcome studies / meta-analyses.

## kind
- `reference`: the normal interval.
- `optimal`: the optimal band.
- `warn`: borderline / approaching zone (prediabetes, borderline-high LDL, A2 albuminuria).
- `diagnostic`: values meeting a diagnostic or disease-category criterion (diabetes HbA1c >= 6.5,
  hypertriglyceridaemia, CKD G3a...). `note` names the category.
- `target`: treatment goal for people with a condition (`when.conditions` / `when.risk` set).
- `critical`: panic / critical values.

Inequalities: ">= 6.5" means `low = 6.5`; "< 100" means `high = 100` (the note can
say "strictly below"). Categories like "100-125" become `low = 100, high = 125`.

## when (all optional)
| key | type | example |
|---|---|---|
| sex | "male" / "female" | `sex = "female"` |
| age_min, age_max | years, half-open [min, max) | `age_min = 18, age_max = 65` |
| pregnant | bool | `pregnant = true` |
| trimester | 1 / 2 / 3 | `trimester = 2` |
| postpartum | bool | |
| menopause | "pre" / "peri" / "post" | |
| cycle_phase | "follicular" / "ovulatory" / "luteal" | |
| fasting | bool | `fasting = true` |
| ethnicity | string | `ethnicity = "south-asian"` |
| conditions | list of kebab strings | `conditions = ["diabetes", "ckd"]` |
| risk | kebab string | `risk = "very-high-cardiovascular-risk"` |
| markers | list of other-marker thresholds | `markers = [{ marker = "egfr", op = "<", value = 60 }]` |
| specimen | string | `specimen = "spot-urine"`, `"serum"`, `"plasma"`, `"24h-urine"` |
| method | string | `method = "nmr"`, `"ifcc"` |

## citations
Each citation needs `quote` (verbatim text copied from rt.py output; "..." allowed to
join two verbatim pieces of the same source), `locator` (abstract / Table 3 / section
name / page), and at least one of `pmid`, `doi`, `url`. With a `pmid`, `title` must be
the PubMed title. Every number in `low`, `high` and `when.markers[].value` must appear
in at least one quote of that range. Never invent a citation, a number or a quote.

## Panels (catalog/panels/<slug>.toml)

A panel is a cited preset of catalog markers for one condition or goal (diabetes, Alzheimer's, biological aging...).

```toml
slug = "diabetes"
name = "Diabetes and glycemic control"
description = "Screening, diagnosis and monitoring of prediabetes and diabetes."
aliases = ["glycemic"]            # optional

[[member]]
marker = "hba1c"                  # must exist as catalog/markers/<marker>.toml
role = "core"                     # core = the guideline's named test set; extended = supported add-on
reason = "Diagnostic criterion (>= 6.5%) and the main monitoring test."
term = "A1C"                      # optional: the word the quote uses for this marker, if not its name/alias/slug
citations = [
  { pmid = "...", title = "...", url = "...", locator = "Abstract", quote = "verbatim text naming the marker and its role" },
]
```

Rules: every member's citation quote must name the marker (its catalog name, an alias, the slug, or `term`)
and state why it belongs (diagnosis, screening, monitoring, risk, clock input...). Prefer guidelines and
consensus statements; for aging clocks cite the paper that defines the clock's inputs. `rt.py verify
catalog/panels/<slug>.toml` checks markers exist, PMIDs/titles/DOIs resolve, and quotes are verbatim.
