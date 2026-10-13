# Interventions: supplements, drugs, regimens and interactions

biomarker records what each person *takes* next to what their labs *show*.
That way a marker series can be read against the supplements and drugs that
were started or stopped around it. There is no separate database: the
catalog, the regimens and the interaction edges are tables in the same
encrypted database as the measurements.

Status: design for `biomarker-cli-iy9` (schema version 6).

## 1. Concepts

* **Intervention**: a catalog entry for something a person can take, such as a
  supplement (`magnesium-glycinate`), a drug (`atorvastatin`), a food or a
  lifestyle change. It is shared by everyone, like the marker catalog.
* **Regimen**: one person taking one intervention at a dose and schedule from
  a start date until an optional stop date. A dose change is modeled as
  stopping one regimen and starting another. History is kept, never rewritten.
* **Interaction edge**: a directed statement "intervention A has a relation
  of *kind* with target B". B is another intervention or a non-catalog
  target: a drug class, a condition, a lab test or a genotype.

## 2. Data model (migration 6)

Migration 6, "interventions, regimens and interactions", adds four tables.
Like every migration it runs once inside a transaction and is recorded in
`schema_migrations`. `PRAGMA user_version` becomes 6; it follows the
shipped migration 5 (near-limit warn zones, #3). Older binaries refuse the
newer database with the usual "schema version is newer" error.

```sql
CREATE TABLE interventions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    category    TEXT NOT NULL DEFAULT 'supplement',
    rx_status   TEXT NOT NULL DEFAULT 'unknown',  -- otc | rx | controlled | unknown
    ingredients TEXT NOT NULL DEFAULT '[]',       -- JSON [{name, amount, unit}]
    classes     TEXT NOT NULL DEFAULT '[]',       -- JSON ["anticoagulant", ...]
    description TEXT,
    source      TEXT,                             -- where an imported entry came from
    created_at  TEXT NOT NULL
);
CREATE TABLE intervention_aliases (
    alias           TEXT PRIMARY KEY,
    intervention_id INTEGER NOT NULL REFERENCES interventions(id) ON DELETE CASCADE
);
CREATE TABLE regimens (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id       INTEGER NOT NULL REFERENCES people(id) ON DELETE CASCADE,
    intervention_id INTEGER NOT NULL REFERENCES interventions(id),
    dose            REAL,
    unit            TEXT,
    frequency       TEXT,        -- free text: daily, 2x/day, weekly, prn ...
    time_slot       TEXT,        -- free text: morning, evening, bedtime, with-meals ...
    started_on      TEXT NOT NULL,   -- YYYY-MM-DD, first day taken
    stopped_on      TEXT,            -- YYYY-MM-DD, first day NOT taken; NULL = ongoing
    note            TEXT,
    created_at      TEXT NOT NULL
);
CREATE TABLE interactions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    a_id        INTEGER NOT NULL REFERENCES interventions(id) ON DELETE CASCADE,
    b_kind      TEXT NOT NULL,   -- intervention | drug-class | condition | lab-test | genotype
    b_ref       TEXT NOT NULL,   -- normalised slug of the target
    kind        TEXT NOT NULL,   -- contraindicated | avoid | caution | monitor | synergy | lab-interference
    severity    TEXT NOT NULL DEFAULT 'unknown',  -- major | moderate | minor | unknown
    mechanism   TEXT,
    citations   TEXT NOT NULL DEFAULT '[]',       -- JSON array (strings or objects, kept as given)
    note        TEXT,
    source      TEXT,
    created_at  TEXT NOT NULL,
    UNIQUE (a_id, b_kind, b_ref, kind)
);
CREATE INDEX idx_regimens_person ON regimens(person_id);
CREATE INDEX idx_interactions_b ON interactions(b_kind, b_ref);
```

Design decisions:

* **Targets are stored by slug, not by id** (`b_ref`). An imported
  `interactions.json` for supplement X often names drugs that are not in the
  catalog yet. Storing the slug lets the edge exist before its target does.
  The target is resolved when the edge is read, through the intervention
  catalog (slug or alias) for `intervention` targets, or the marker catalog
  for `lab-test` targets. Unresolved targets are reported with
  `"resolved": false`; they are not errors. Other target kinds have no
  catalog and report `"resolved": null`.
* **Intervention-intervention edges are symmetric when read.** The edge
  `A -> intervention:B` is found when you ask about A or about B. It is
  stored once. Import dedupes on `(a, b_kind, b_ref, kind)`.
* **Drug classes link edges to catalog entries.** An intervention lists its
  `classes` (for example `warfarin` has `anticoagulant`). The edge
  `fish-oil -> drug-class:anticoagulant` then applies to the pair
  (fish-oil, warfarin) in `interaction grid`, reported as `via:
  "drug-class:anticoagulant"`.
* **Dates are whole days.** `started_on` is the first day taken.
  `stopped_on` is the first day not taken. A regimen is *active on* day D
  when `started_on <= D` and (`stopped_on` is NULL or `D < stopped_on`). A
  measurement's day is the date part of its `taken_at`. A regimen whose
  start is after D is *planned* as of D.
* **No uniqueness on regimens.** A person may take the same intervention
  twice in a day (different slots), or restart it later. `regimen add`
  warns when it overlaps an active regimen of the same intervention for the
  same person, but still adds it.
* **Deletion.** `person rm` deletes the person's regimens with the person.
  `intervention rm` refuses while regimens reference the intervention, unless
  `--force` is given, which deletes them. It always deletes the
  intervention's own edges (`a_id`) and edges that target it by slug.
  Deletion is explicit in code, as it is for the existing tables, rather
  than relying on foreign-key cascades.

### Ordering of kinds and severities

Kinds are ranked so a grid cell can show the most important relation:

| rank | kind               | meaning |
|------|--------------------|---------|
| 6    | `contraindicated`  | do not combine |
| 5    | `avoid`            | avoid unless supervised |
| 4    | `caution`          | combine with care (timing, dose) |
| 3    | `monitor`          | combine, but watch a marker or symptom |
| 2    | `lab-interference` | distorts a lab result (for example biotin and immunoassays) |
| 1    | `synergy`          | beneficial or additive combination |

Severity is `major` > `moderate` > `minor` > `unknown`. Import maps the
synonyms `severe`/`high`/`serious` to `major`, `medium`/`significant` to
`moderate` and `low`/`mild` to `minor`. It maps the kind synonyms
`contraindication` to `contraindicated`, `interference`/`lab` to
`lab-interference`, `beneficial`/`synergistic` to `synergy` and
`warning`/`precaution` to `caution`. Unknown kinds are invalid and abort
the import (or are skipped with `--skip-invalid`).

## 3. CLI surface

Every command accepts the global flags (`--format`, `--output`, `--columns`,
...). Mutations print a status line to stderr and, in non-table formats,
the record that was written (`emit_mutation`, as `add` does).

### Catalog: `biomarker intervention` (alias `iv`)

```
intervention add SLUG [--name N] [--category C] [--rx-status otc|rx|controlled|unknown]
                      [--ingredient NAME[=AMOUNT[UNIT]]]... [--class CLASS]... [--alias A]...
                      [--description TEXT]
intervention ls  [--category C] [--search TEXT]          (alias: list)
intervention show INTERVENTION                            (aliases, ingredients, classes, edges, regimen count)
intervention rm  INTERVENTION [--force]
intervention import PATH... [--dry-run] [--skip-invalid]
```

`INTERVENTION` resolves like markers do: slug, alias, display name, then
slugified name. `--ingredient magnesium=120mg` stores
`{"name": "magnesium", "amount": 120, "unit": "mg"}`. A bare
`--ingredient magnesium` has null amount and unit.

`intervention import` accepts files and directories. A directory is searched
for `*/interactions.json` (the health-cli `research/supplements/<slug>/`
layout) plus an `interactions.json` directly inside it.

### Regimens: `biomarker regimen` (alias `rx`)

```
regimen add INTERVENTION [-p PERSON] [--dose N] [--unit U] [--frequency F] [--slot S]
                         [--start DATE (default today)] [--stop DATE] [--note TEXT]
regimen stop TARGET [-p PERSON] [--on DATE (default today)]
regimen ls [-p PERSON | --all-people] [--as-of DATE (default today)]
           [--status taking|stopped|planned|all (default all)] [--intervention I]
regimen rm ID...
regimen impact INTERVENTION [-p PERSON] [-m MARKER]... [-c CATEGORY]... [--window 90d]
```

* `TARGET` for `stop` is a regimen id, or an intervention. An intervention
  stops every regimen of it that is active for the person on `--on`. It is
  an error if none is.
* `regimen ls` reports a `status` per row as of `--as-of`: `taking`,
  `stopped` or `planned`. This is the "taking vs not as of a date" view.
  `--status taking --as-of 2025-06-01` lists what the person was taking on
  that day.
* `regimen impact` is the before/after join. For every start and stop event
  of the person's regimens of INTERVENTION, and every marker with values
  near the event, it reports the values in the window before the event,
  `(event - window, event]`, and in the window after it,
  `(event, event + window]`. The event day counts as *before*, since a
  baseline draw is usually taken on the day the supplement starts. `before`
  is the value nearest the event and `after` is the last value in the after
  window (closest to steady state). Means, counts and change are also
  reported. Values are in the display unit (`--units`).

### Interactions: `biomarker interaction` (alias `ix`)

```
interaction add A TARGET --kind K [--severity S] [--mechanism TEXT] [--cite TEXT]... [--note TEXT]
interaction ls [--intervention I] [--kind K]... [--min-severity S] [--target-kind T]
interaction grid [-p PERSON] [--as-of DATE] [--catalog]
interaction rm ID...
```

`TARGET` is `intervention-slug`, or `TYPE:slug` with TYPE one of
`intervention`, `drug-class`, `condition`, `lab-test`, `genotype`
(`drug-class:anticoagulant`, `lab-test:tsh`, `genotype:mthfr-c677t`).

`interaction grid` is the contraindication matrix for one person on one
day:

* Rows are the interventions the person is taking on `--as-of` (default
  today).
* Columns are the same interventions. With `--catalog`, they also include
  every catalog intervention that has an edge with one of the rows ("what
  should this person not add?").
* A cell holds the highest-ranked kind among the edges between the row and
  the column, matched directly in either direction or through a drug class
  of either side. Text formats show the kind (`contraindicated`,
  `caution`, ...) or the null text. The JSON form lists every cell with all
  its edges.
* Edges from a row to a condition, a genotype, a lab test or a drug class
  that no column belongs to are listed separately as `other` (JSON) and
  printed after the matrix in table mode as a second report. These are the
  items to review against the person's history.

### Joins with measurements

* `query --with-regimens` (also `latest`) adds two fields to each
  measurement: `regimens`, the slugs of the interventions the person was
  taking on the measurement's day, and `interferences`, the slugs of those
  interventions with a `lab-interference` edge to this marker (`lab-test:`
  target resolving to the marker by slug or alias). Without the flag the
  record is unchanged.
* `trend --with-regimens` adds `regimens` to each point and `events` to
  each series: `[{date, event: "start"|"stop", intervention, regimen_id}]`
  for the person's regimen starts and stops inside the series' date span.
  Charts can draw these as vertical rules.
* `regimen impact` (above) is the summary form.

## 4. JSON (`biomarker/v1`)

All new output uses the existing envelope (`schema`, `kind`,
`generated_at`, `count`, `data`). The fields below are added to v1. Nothing
existing changes meaning. The only additions to existing kinds are the
opt-in `regimens`/`interferences`/`events` fields described above, plus
`interventions`, `regimens` and `interactions` counts in `db info`.

| command | kind | `data` |
|---------|------|--------|
| `intervention ls` / `show` / `add` | `interventions` / `intervention` | Intervention |
| `intervention import` | `intervention_import` | Import summary |
| `regimen ls` / `add` / `stop` | `regimens` / `regimen` / `regimens` | Regimen |
| `regimen impact` | `regimen_impact` | Impact row |
| `interaction ls` / `add` | `interactions` / `interaction` | Interaction |
| `interaction grid` | `interaction_grid` | Grid object |
| `* rm` | `removed` | what was removed |

* **Intervention**: `{id, slug, name, category, rx_status, ingredients:
  [{name, amount, unit}], classes, aliases, description, source,
  created_at}`. `show` adds `regimens` (count), `active_regimens` (count
  today) and `interactions` (Interaction array).
* **Regimen**: `{id, person, intervention, intervention_name, dose, unit,
  frequency, time_slot, started_on, stopped_on, note, status, as_of,
  days}`. `status` ∈ `taking, stopped, planned` as of `as_of`. `days` is
  the number of days taken up to `as_of` (0 when planned).
* **Interaction**: `{id, a, a_name, target_kind, target, target_name,
  resolved, kind, severity, mechanism, citations, note, source}`. `a` and
  `target` are slugs. `target_name` is the resolved catalog name, or null.
* **Impact row**: `{person, intervention, regimen_id, event, event_date,
  window_days, marker, unit, n_before, before, before_date, before_mean,
  n_after, after, after_date, after_mean, change, change_pct}`. `change`
  and `change_pct` are null unless both sides have values.
* **Grid object**:

```json
{
  "person": "alex", "as_of": "2025-06-01",
  "rows": ["fish-oil", "warfarin"],
  "columns": ["fish-oil", "warfarin", "vitamin-k2"],
  "cells": [
    {"row": "fish-oil", "column": "warfarin", "kind": "caution", "severity": "moderate",
     "edges": [{"id": 3, "kind": "caution", "severity": "moderate", "via": "drug-class:anticoagulant"}]}
  ],
  "other": [
    {"intervention": "fish-oil", "target_kind": "lab-test", "target": "tsh", "kind": "lab-interference",
     "severity": "minor", "id": 7}
  ]
}
```

  `cells` only lists pairs that have an edge. The diagonal is never filled.
  The table form has one row per regimen intervention, one column per
  column intervention and the kind as the cell value.

* **Import summary**: `{dry_run, files, interventions_created,
  interventions_updated, interactions_inserted, interactions_updated,
  interactions_unchanged, invalid, errors: [{file, index, error}]}`.

## 5. Importing `supplement-interactions/1`

health-cli writes one document per supplement to
`research/supplements/<slug>/interactions.json`. biomarker reads the
following shape. Fields not listed are ignored, so the format can grow.

```json
{
  "schema": "supplement-interactions/1",
  "supplement": {
    "slug": "fish-oil",
    "name": "Fish oil (EPA/DHA)",
    "aliases": ["omega-3", "epa/dha"],
    "category": "supplement",
    "rx_status": "otc",
    "classes": ["omega-3-fatty-acid"],
    "ingredients": [{"name": "epa", "amount": 500, "unit": "mg"}, {"name": "dha", "amount": 250, "unit": "mg"}]
  },
  "interactions": [
    {
      "with": {"type": "drug-class", "slug": "anticoagulant", "name": "Anticoagulants"},
      "kind": "caution",
      "severity": "moderate",
      "mechanism": "additive antiplatelet effect",
      "citations": ["PMID:00000000", {"title": "Synthetic review", "url": "https://example.org/x"}],
      "note": "monitor INR"
    },
    {"with": "intervention:vitamin-e", "kind": "monitor", "severity": "minor"},
    {"with": {"type": "lab-test", "slug": "triglycerides"}, "kind": "lab-interference"}
  ]
}
```

Leniency rules (the producer is a research pipeline, not a fixed schema):

* `schema` must start with `supplement-interactions/1`. A missing schema is
  accepted with a warning. Any other schema is rejected.
* The subject may be called `supplement`, `subject` or `intervention`. It
  may be a string (the slug). Without one, the slug is taken from the parent
  directory name (`<slug>/interactions.json`).
* A target (`with`, `target` or `b`) is either a string `type:slug` (a bare
  slug means an intervention) or an object with `type`/`kind`/`target_kind`
  and `slug`/`id`/`name`. Type spellings `drug`, `supplement` and
  `intervention` all mean intervention. `class` means `drug-class`, `lab`
  and `marker` mean `lab-test`, `gene` and `snp` mean `genotype`. Slugs are
  normalised with the same `slugify` as markers.
* `citations` may be a single string, an array of strings, or an array of
  objects. It is stored as an array, exactly as given.
* A top-level array of such documents is accepted too.

What an import does, in one transaction across every input file:

1. Upsert the subject intervention by slug. A new one is created with the
   document's fields. An existing one keeps its fields; only empty ones are
   filled (an empty name, `unknown` rx_status, empty ingredient or class
   lists). Aliases are merged, and aliases owned by another intervention are
   skipped with a warning.
2. Upsert each edge on `(a, b_kind, b_ref, kind)`. Edges with a new
   severity, mechanism, citations or note are updated, the rest are
   unchanged. Edges are never deleted by import, so re-running an import is
   idempotent.
3. Targets of type intervention are **not** created. They resolve once the
   target has its own document imported or is added with `intervention
   add`.

`--dry-run` runs everything in a transaction that is rolled back and
reports the same summary. Invalid entries (an unknown kind, a missing
target) abort the import with exit code 4 and the list of errors, unless
`--skip-invalid` is given.

## 6. Encryption and audit

Nothing new is needed and nothing is weakened:

* The four tables live inside the sealed database image, so they are
  encrypted at rest under the DEK and re-sealed on every committing
  statement like measurements. `db encrypt` verifies their row counts along
  with the other tables, since it enumerates all tables. No sidecar or temp
  file holds plaintext.
* Every new command opens the database through `Ctx::db`, so it is audited
  like `add` and `query`. The audit record holds the command path
  (`regimen add`, `interaction grid`, ...), counts and the outcome, never
  intervention names, doses or notes.
* Read-only commands (`ls`, `show`, `grid`, `impact`) do not re-seal the
  file.
* `--output` warns on plaintext health data and `--encrypt-output` works
  as everywhere else.

## 7. Tests

All fixtures are synthetic (`examples/interactions/*/interactions.json`,
people `alex`/`sam` as in the existing tests):

* Unit tests: date activity rules, kind and severity parsing and synonyms,
  target parsing, document parsing leniency, grid cell ranking and drug-class
  matching, impact windows.
* Integration tests (`tests/interventions.rs`): catalog add/ls/show/rm,
  import of a directory (idempotent on re-run, `--dry-run` writes nothing,
  invalid kinds abort), regimen add/stop/ls `--as-of` status, grid with and
  without `--catalog`, `query --with-regimens` annotation and interference,
  `trend --with-regimens` events, `regimen impact` before/after values, the
  migration from a version 4 database, no plaintext in written files, and
  audit entries for the new commands.
