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
| `generated_at` | string          | UTC timestamp |
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
* `ref_flag`/`opt_flag` are `"low"`, `"normal"`, `"high"`, or `null` when no
  range applies.
* `flag` is the overall flag for the active `range_flavor`. `reference` uses
  `ref_flag` and `optimal` uses `opt_flag`. `both` uses `ref_flag` if it is
  out of range, otherwise `opt_flag`.
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
  "last_flag": "normal",
  "windows": {
    "6m": {"days": 184, "change": -10.0, "change_pct": -10.0},
    "1y": {"days": 366, "change": -10.0, "change_pct": -10.0}
  },
  "ref_low": null, "ref_high": 100.0, "opt_low": null, "opt_high": 70.0,
  "points": [
    {"id": 1, "taken_at": "2023-01-01", "value": 140.0, "qualifier": null, "flag": "high"}
  ]
}
```

* All values are in the display unit.
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

### Person / Marker / Range

* Person: `{id, slug, name, sex, dob, notes, tags, created_at}`.
  `person show` adds `measurements, markers, first_date, last_date`.
* Marker: `{id, slug, name, category, unit, loinc, description, builtin,
  aliases}`. `marker show` adds `measurements, convertible_units, conversions,
  ranges`.
* Range: `{id, marker, kind, sex, age_min, age_max, low, high, unit, note}`.
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
