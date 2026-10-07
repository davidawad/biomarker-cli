# biomarker JSON interface (`biomarker/v1`)

This is the stable machine interface for tools such as `health-charts.el`. It
covers `--format json` and `--format jsonl` output, the error object, and the
process exit codes.

## Stability

* The envelope's `schema` field is `"biomarker/v1"`. Within v1, fields are
  only ever **added**. Nothing is removed, renamed, or changed in type or
  meaning. Consumers should ignore fields they don't know.
* A breaking change will bump the schema to `biomarker/v2`.
* Numbers are JSON numbers at full precision. `--precision` only affects
  table/csv/tsv output.
* Dates are ISO 8601 strings. A measurement time (`taken_at`) is either
  `YYYY-MM-DD` (date only) or `YYYY-MM-DDTHH:MM:SS`. The latter is wall-clock
  time in the configured time zone. RFC 3339 inputs with an offset are
  converted to that zone on input. `--date-format` never affects JSON.
* Missing values are `null`. Tags are always arrays of strings.

## Envelope (`--format json`)

```json
{
  "schema": "biomarker/v1",
  "kind": "measurements",
  "generated_at": "2025-01-31T12:00:00Z",
  "count": 2,
  "unit_system": "canonical",
  "range_flavor": "reference",
  "data": [ ... ]
}
```

| field          | type            | notes |
|----------------|-----------------|-------|
| `schema`       | string          | always `biomarker/v1` |
| `kind`         | string          | what `data` contains (see below) |
| `generated_at` | string          | UTC timestamp (`$SOURCE_DATE_EPOCH` pins it for reproducible output) |
| `count`        | integer         | present when `data` is an array |
| `data`         | array \| object | the payload |
| *(extra)*      | any             | command-specific metadata, e.g. `unit_system`, `range_flavor`, `windows`, `person`, `from`, `to`, `config_path`, `current_version` |

## JSON Lines (`--format jsonl`)

JSONL output has no envelope. It prints one `data` element per line, using
the same objects as JSON. A single-object result prints as one line. This
suits streaming and `jq -c`.

## Kinds

| command                                | kind           | `data` |
|----------------------------------------|----------------|--------|
| `query` / `list`                       | `measurements` | array of [Measurement](#measurement) |
| `latest`, `query --latest`             | `latest`       | array of Measurement |
| `flag`                                 | `flags`        | array of Measurement |
| `add`                                  | `measurement`  | Measurement + `"outcome": "inserted" \| "replaced" \| "skipped"` |
| `trend` / `stats`                      | `trend`        | array of [Series](#series-trend) |
| `diff`                                 | `diff`         | array of [Diff row](#diff-row) |
| `export`                               | `export`       | array of [Export row](#export-row) |
| `import`                               | `import`       | [Import summary](#import-summary) |
| `import FILE --list-sheets`            | `sheets`       | `{index, name, rows, columns, dimensions}` |
| `observations`                         | `observations` | array of [Observation](#observation) |
| `person list` / `person show` / `add` / `edit` | `people` / `person` | Person |
| `marker list` / `show` / `add` / `edit` | `markers` / `marker` | Marker |
| `marker alias`                         | `aliases`      | `{marker, aliases}` |
| `marker categories`                    | `categories`   | `{category, markers}` |
| `range list` / `set`                   | `ranges` / `range` | Range |
| `unit list`                            | `conversions`  | `{id, marker, from, to, factor, offset}` |
| `unit list --symbols`                  | `units`        | `{unit, system}` |
| `unit convert`                         | `conversion_result` | `{marker, value, from, result, to}` |
| `*  rm`                                | `removed`      | what was removed |
| `config show`                          | `config`       | `{key, value, source}`; `source` ∈ `default, file, env, flag` |
| `config keys` / `path` / `set` / `unset` | `config_keys` / `config_path` / `config_set` / `config_unset` | |
| `db path/init/migrate/backup/vacuum/check/info` | `db_path` / `db_init` / `migrations` / `db_backup` / `db_vacuum` / `db_check` / `db_info` | |

### Measurement

```json
{
  "id": 12,
  "person": "alex",
  "marker": "ldl-c",
  "marker_name": "LDL Cholesterol",
  "category": "lipid",
  "taken_at": "2024-03-05",
  "qualifier": null,
  "value": 96.0,
  "unit": "mg/dL",
  "value_raw": 96.0,
  "unit_raw": "mg/dL",
  "value_canonical": 96.0,
  "unit_canonical": "mg/dL",
  "ref_low": null,
  "ref_high": 100.0,
  "ref_flag": "normal",
  "opt_low": null,
  "opt_high": 70.0,
  "opt_flag": "high",
  "flag": "normal",
  "ref_level": "normal",
  "opt_level": "high",
  "level": "normal",
  "warn_low": null,
  "warn_high": 159.0,
  "status": "in-range",
  "sex": "male",
  "age": 39.76,
  "range_set": {
    "reference": {"id": 2, "source": "catalog", "sex": "any", "age_min": 0.0, "age_max": 200.0, "note": null},
    "optimal": {"id": 3, "source": "catalog", "sex": "any", "age_min": 0.0, "age_max": 200.0, "note": null},
    "warn": {"id": 90, "source": "catalog", "sex": "any", "age_min": 0.0, "age_max": 200.0,
             "note": "100-159 mg/dL: near optimal to borderline high"}
  },
  "lab": "Synthetic Labs",
  "fasting": true,
  "note": null,
  "tags": ["annual"],
  "batch": "b20250131T120000-1a2b3"
}
```

* `value`/`unit` are in the **display unit** chosen by `--units` /
  `unit_system`. `canonical` is the marker's catalog unit. `us` and `si` pick
  a convertible unit of that system when one exists.
* `value_raw`/`unit_raw` are what was recorded. `value_canonical` is the
  normalized value in `unit_canonical`.
* `ref_*`/`opt_*` are the applicable reference/optimal range bounds, in the
  display unit. The range is chosen by the person's sex and age at
  measurement time; the most specific match wins.
* `sex` is the person's sex and `age` their age in years at `taken_at`
  (from the birth date, truncated to two decimals). Both are `null` when the
  person has none. Each row's ranges are resolved per draw, so a series that
  crosses an age band (`range set --age-min 50`) switches ranges at that draw.
* `range_set` tells which range applied for each kind (`reference`,
  `optimal`, `warn`): `{id, source, sex, age_min, age_max, note}` or `null`.
  `source` follows the range precedence (as in `profile show`): `"person"`
  for a person's own range (person profile file, then `range set --person`;
  it beats everything else at any age), `"set:<name>"` for an entry of the
  range set in effect, or `"catalog"` for the most specific catalog row.
  `id` is the `range list` id (`range list --person` for a person's database
  range) and `null` for entries from profile or range-set files.
* `warn_low`/`warn_high` are the near-limit bounds (range kind `warn`, display
  unit) for markers with a clinically meaningful "approaching" zone. The
  catalog ships them for glucose (fasting 100-125 mg/dL), HbA1c (5.7-6.4 %),
  total cholesterol, LDL-C, triglycerides, eGFR (60-89) and systolic blood
  pressure. Set or override them with `range set MARKER --kind warn` (with
  `--sex`/`--age-min`/`--age-max`, or `--person`) or with `kind = "warn"`
  entries in a range set or person profile, which are selected by sex, age
  band, lab and person exactly like reference and optimal ranges. `null` means the marker
  has no such zone; a consumer may then apply its own margin as a fallback.
* `status` is `"low"`, `"near-low"`, `"in-range"`, `"near-high"`, `"high"`
  or `"unknown"` (no reference range). On each side the reference limit and
  the warn bound are two cut points: beyond the outer one is `low`/`high`,
  between them is `near-low`/`near-high`. A warn bound inside the reference
  range marks a margin before the limit (eGFR reference low 60, warn low 90:
  60-89 is `near-low`). One outside it marks an approaching zone before the
  clinical cut-off (HbA1c reference high 5.6, warn high 6.4: 5.7-6.4 is
  `near-high`, 6.5 and above `high`); there `ref_flag` and `flag` still say
  `"high"`. Without a warn bound, `status` follows the reference range.
  Censored values follow the flag rules. `status` always uses the reference
  range, whatever the `range_flavor`. It is independent of `ref_level`:
  the borderline band of `borderline_margin` does not affect it.
* `ref_flag`/`opt_flag` are `"low"`, `"normal"`, `"high"`, or `null` when no
  range applies.
* `flag` is the overall flag for the active `range_flavor`. `reference` uses
  `ref_flag` and `optimal` uses `opt_flag`. `both` uses `ref_flag` if it is
  out of range, otherwise `opt_flag`.
* `ref_level`/`opt_level`/`level` (added in 0.6, additive) are finer than the
  flags: `"critical-low"`, `"low"`, `"borderline-low"`, `"normal"`,
  `"borderline-high"`, `"high"`, `"critical-high"`, or `null`. Critical comes
  from `critical_low`/`critical_high` in a range set or person profile;
  borderline from the `borderline_margin` setting (percent of the range width
  inside a bound, off at 0). `level` follows `range_flavor`; with `both` it is
  the more severe of the two. The `*_flag` fields never change meaning.
* `unit_system` (top-level metadata) is the name of the unit preset in use:
  a built-in (`canonical`, `us`, `si`, `uk`) or one you defined.
* `qualifier` is `"<"`, `">"`, `"<="`, `">="` or `null` for censored results
  such as `<0.5`. When flagging, `<x` is never "high" and `>x` is never "low".

### Series (trend)

```json
{
  "person": "alex", "marker": "ldl-c", "marker_name": "LDL Cholesterol",
  "category": "lipid", "unit": "mg/dL",
  "n": 4, "first_date": "2023-01-01", "last_date": "2025-01-01",
  "min": 90.0, "max": 140.0, "mean": 112.5, "median": 110.0, "stddev": 22.17,
  "first": 140.0, "last": 90.0, "change": -50.0, "change_pct": -35.71,
  "slope_per_year": -24.9,
  "last_flag": "normal", "last_status": "in-range",
  "windows": {
    "6m": {"days": 184, "change": -10.0, "change_pct": -10.0},
    "1y": {"days": 366, "change": -10.0, "change_pct": -10.0}
  },
  "ref_low": null, "ref_high": 100.0, "opt_low": null, "opt_high": 70.0,
  "warn_low": null, "warn_high": 159.0, "range_set": { ... },
  "points": [
    {"id": 1, "taken_at": "2023-01-01", "value": 140.0, "qualifier": null, "flag": "high", "status": "near-high"}
  ]
}
```

* All values are in the display unit.
* The range fields (`ref_*`, `opt_*`, `warn_*`, `range_set`) and
  `last_status` are those of the last measurement. Each point's `status` is
  computed with the ranges that applied at that draw.
* `slope_per_year` is the least-squares slope of value over time, in value
  units per year. It is `null` with fewer than two distinct dates.
* `windows[w]` compares the last value with the most recent value at or
  before the date `w` before the last measurement. `m` and `y` are calendar
  months and years (`1y` before 2024-01-01 is 2023-01-01); `d` and `w` are
  days and weeks. `days` is the span in days. The value is `null` when the
  series doesn't reach back that far.
* `--no-points` omits `points`. In csv/tsv/table output, windows are
  flattened into `change_pct_<w>` columns and points are omitted.
* The envelope also carries `"windows": ["3m", "6m", "1y"]`.

### Diff row

`{person, marker, category, unit, from_date, from_value, to_date, to_value,
change, change_pct, from_flag, to_flag}`. By default each side is the latest
measurement on or before the date. With `--exact`, it must be on that date.
Missing sides are `null`.

### Export row

`{person, marker, date, qualifier, value, unit, lab, fasting, note, tags}`.
With `--with-ids`, `id` and `batch` are added. Values are the raw recorded
value and unit, so `import` of an export reproduces the data exactly. CSV
export joins `tags` with commas and writes missing values as empty cells.
`export --format json` adds a top-level `observations` array (the same
filters apply). It sits outside `data`, so the envelope still re-imports as
measurements.

### Observation

`{id, person, marker, marker_name, category, taken_at, text, flag, range_low,
range_high, note, lab, batch}`: a qualitative result such as `"Negative"` or
`"6-10 Abnormal"`. `flag` is `"abnormal"` when the text says so, else `null`.
`range_low`/`range_high` hold a numeric range from the text (`6-10`) and
`note` describes it (`"range 6-10"`).

### Import summary

```json
{
  "batch": "b20250131T120000-1a2b3", "source": "labs.csv", "format": "csv",
  "dry_run": false, "dedupe": "skip",
  "rows": 30, "measurements": 30,
  "inserted": 28, "replaced": 0, "skipped": 1, "invalid": 1,
  "people_created": [], "markers_created": [],
  "errors": [{"line": 7, "error": "cannot parse numeric value 'n/a'"}]
}
```

Every import summary also has `layout` (`long`, `wide`, `transposed`) and these
report fields, which are mostly filled by spreadsheet imports:
`matched` (`[{source, marker}]`), `unmatched` (`[{source, cells}]`, skipped),
`skipped_names` (from the mapping's `[skip]`), `qualitative` (count) and
`qualitative_values` (`[{line, marker, date, text}]`), `observations`
(`{inserted, replaced, skipped}` with `--qualitative store`),
`date_corrections` (`[{from, to, cells}]`), `cells_per_date`
(`{date: values}`), `ranges_set` (`[{marker, person, low, high, unit}]` with
`ranges = "sheet"`) and `warnings`. Spreadsheet imports add `sheet` and
`header_row` (1-based).

### Person / Marker / Range

* Person: `{id, slug, name, sex, dob, notes, tags, created_at}`.
  `person show` adds `measurements, markers, first_date, last_date`.
* Marker: `{id, slug, name, category, unit, loinc, description, builtin,
  aliases}`. `marker show` adds `measurements, convertible_units, conversions,
  ranges`.
* Range: `{id, person, marker, kind, sex, age_min, age_max, low, high, unit,
  note}`. `kind` ∈ `reference, optimal, warn`. `person` is set for person-specific ranges (`range set --person`)
  and `null` for catalog ranges.
  Bounds are in the marker's canonical unit. `sex` ∈ `any, male, female`.
  The age band is `[age_min, age_max)` in years.

## Errors

On failure, a human-readable message goes to stderr. If the effective format is `json` or
`jsonl` (from the flag, environment, or config file), stderr instead gets a
single JSON line:

```json
{"schema": "biomarker/v1", "kind": "error", "error": {"kind": "not_found", "code": 3, "message": "no such person 'ghost' ..."}}
```

## Exit codes

| code | kind        | meaning |
|------|-------------|---------|
| 0    |             | success |
| 1    | `general`   | unspecified failure |
| 2    | `usage`     | bad arguments or invalid flag combination |
| 3    | `not_found` | unknown person, marker, measurement, range |
| 4    | `invalid`   | invalid data: unparsable value/date, unknown unit conversion, duplicate (with `--dedupe error`), import aborted because of invalid rows |
| 5    | `database`  | database engine error, or `db check` failed |
| 6    | `io`        | file read/write failure |
| 7    | `config`    | bad config file, key or value |
| 8    | `key`       | encrypted database cannot be unlocked: no key available or wrong key |
| 10   |             | `flag --exit-code` found at least one flagged value |

## Emacs example

```elisp
(defun health-charts--biomarker (&rest args)
  (with-temp-buffer
    (let ((code (apply #'call-process "biomarker" nil t nil
                       (append args '("--format" "json")))))
      (unless (zerop code) (error "biomarker exited %s" code))
      (goto-char (point-min))
      (let ((json (json-parse-buffer :object-type 'alist :null-object nil)))
        (unless (equal (alist-get 'schema json) "biomarker/v1")
          (error "unsupported biomarker schema"))
        (alist-get 'data json)))))

;; (health-charts--biomarker "trend" "--person" "alex" "--marker" "ldl-c")
;; (health-charts--biomarker "query" "--person" "alex" "--category" "lipid" "--from" "2023-01-01")
```
