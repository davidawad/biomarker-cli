# biomarker-cli

Rust CLI for tracking biomarkers (lab results) across any number of people,
backed by [fsqlite](https://crates.io/crates/fsqlite) (FrankenSQLite, a
pure-Rust SQLite reimplementation). It builds a single binary, `biomarker`.

* Any number of people, with sex and date of birth for sex- and age-specific
  ranges.
* A built-in catalog of 61 markers you can extend: lipid panel, ApoB,
  Lp(a), CMP, CBC, thyroid, hormones, HbA1c, insulin, hs-CRP, homocysteine,
  vitamin D/B12, ferritin and iron studies. Each has aliases and LOINC codes.
* Reference ranges and separate *optimal* ranges, by sex and age band.
* Unit conversions per marker (mg/dL↔mmol/L, ng/dL↔nmol/L, HbA1c %↔mmol/mol,
  …). The raw value and unit are stored **and** a normalized canonical value.
* Import from CSV, TSV, JSON and JSONL, in long or wide layout. Column
  mapping comes from flags or a TOML file. Imports support dry-run and a
  dedupe policy, and record an import batch.
* Query, latest, flag, trend (min/max/mean/median/stddev/slope, % change over
  windows) and diff.
* Every command supports `--format table|json|jsonl|csv|tsv` and `--output FILE`.
  JSON uses a stable, versioned envelope (`biomarker/v1`, see
  [docs/json-schema.md](docs/json-schema.md)) so tools like `health-charts.el`
  can consume it.
* Layered configuration: defaults < TOML file < `BIOMARKER_*` env < flags.
  `config show --effective` shows where each value came from.

> **Not medical advice.** The built-in ranges are typical adult values for
> orientation only. Labs differ, so set your lab's ranges with
> `biomarker range set`.

## Build

fsqlite 0.4 uses `#![feature(...)]` on x86_64, so the crate builds with a
**nightly** toolchain. `rust-toolchain.toml` selects it automatically under
rustup.

```sh
cargo build --release          # target/release/biomarker
cargo install --path .         # or install into ~/.cargo/bin
```

Debug builds of fsqlite use deep async state machines. The binary runs its
work on a thread with a 256 MiB stack, so no `ulimit` tweaks are needed.

## Quick start

```sh
biomarker db init
biomarker person add alex --name "Alex Example" --sex male --dob 1984-06-01
biomarker person add sam  --sex female --dob 1991-09-23
biomarker config set default_person alex

# single measurements (unit defaults to the marker's canonical unit)
biomarker add ldl 96 --date 2024-03-05 --lab "Synthetic Labs" --fasting
biomarker add glucose 5.1 mmol/L --date 2024-03-05      # stored as 91.9 mg/dL + raw 5.1 mmol/L
biomarker add hscrp "<0.5" --date 2024-03-05            # censored values keep their qualifier
biomarker add ferritin 38 --person sam --date 2023-11-20 --note "after iron supplement"

biomarker query                          # default person's measurements
biomarker query --all-people -c lipid --from 2023-01-01
biomarker latest --units si              # latest value per marker, SI units
biomarker flag --range-flavor both       # outside reference or optimal ranges
biomarker trend -m ldl,apob --windows 6m,1y
biomarker diff 2023-02-14 2024-03-05 --changed
```

Example table output:

```
ID  PERSON  TAKEN_AT    MARKER   QUALIFIER  VALUE  UNIT   REF_LOW  REF_HIGH  FLAG    LAB
──  ──────  ──────────  ───────  ─────────  ─────  ─────  ───────  ────────  ──────  ──────────────
 3  alex    2024-03-05  hscrp    <           0.50  mg/L                3.00  normal
 1  alex    2024-03-05  ldl-c               96.00  mg/dL             100.00  normal  Synthetic Labs
 2  alex    2024-03-05  glucose             91.88  mg/dL    70.00     99.00  normal
```

## Commands

| command | purpose |
|---------|---------|
| `person add/list/show/edit/rm` | manage people (`--sex`, `--dob`, `--notes`, `--tag`) |
| `marker add/list/show/edit/rm/alias/categories` | manage the catalog. `edit --unit` rescales stored values and ranges |
| `range set/list/rm` | reference/optimal ranges by `--sex` and `--age-min/--age-max`. `--unit` converts the bounds |
| `unit list [--symbols]`, `unit add-conversion`, `unit convert` | conversion table (`to = from * factor + offset`) |
| `add MARKER VALUE [UNIT]` | record one measurement (`--person --date --lab --fasting --note --tag --dedupe`) |
| `rm ID...` | delete measurements |
| `import FILE` | CSV/TSV/JSON/JSONL import (see below) |
| `export` | re-importable CSV (default), JSON or JSONL. Accepts the same filters as `query` |
| `query` / `list` | filters: `-p/--person` (repeatable), `--all-people`, `-m/--marker`, `-c/--category`, `--from/--to`, `--last 90d`, `--lab`, `--tag`, `--batch`, `--flagged`, `--latest`, `--sort date\|person\|marker\|category\|value`, `-r`, `-n` |
| `latest` | latest value per person × marker |
| `trend` / `stats` | per-series statistics and `--windows` % change. JSON includes the points |
| `flag` | out-of-range values (`--latest`, `--exit-code` → status 10) |
| `diff FROM TO` | compare values at two dates (`--exact`, `--changed`) |
| `db path/init/migrate/backup/vacuum/check/info` | maintenance. Migrations are versioned and applied automatically |
| `config show [--effective]/set/unset/path/keys` | configuration |
| `completions bash\|zsh\|fish\|nushell\|elvish\|powershell` | shell completions |
| `man [--dir DIR]` | roff man page(s) |

Global flags work on every command: `--format`, `--output`, `--units`,
`--precision`, `--date-format`, `--tz`, `--color`, `--delimiter`, `--quote`,
`--no-header`, `--null`, `--range-flavor`, `--columns`, `--db`, `--config`,
`-q`, `-v`.

## Importing

Long format is one measurement per row. The default columns are `person,
marker, value, unit, date`, plus optional `time, lab, fasting, note, tags,
qualifier`. Common synonyms such as `result`, `test`, `collected` and
`patient` are recognized automatically.

```sh
biomarker import examples/measurements.csv --dry-run
biomarker import examples/measurements.csv --dedupe replace
biomarker import examples/measurements.json         # array, or a biomarker/v1 export envelope
biomarker import examples/measurements.jsonl
cat labs.csv | biomarker import - --input-format csv --person sam
```

* Map columns with `--map FIELD=COLUMN` (repeatable or comma-separated), or
  with a TOML mapping file (`--mapping`) that can also set defaults, a date
  format and marker renames. See
  [`examples/lab-report-mapping.toml`](examples/lab-report-mapping.toml):

  ```sh
  biomarker import examples/lab-report.csv --mapping examples/lab-report-mapping.toml
  biomarker import export.csv --input-delimiter ';' --map marker=Analyte,date=Drawn --input-date-format %d.%m.%Y
  ```
* `--wide` reads one row per date with one column per marker. A header like
  `Glucose (mmol/L)` or `LDL [mg/dL]` sets the unit
  ([`examples/wide.csv`](examples/wide.csv)).
* `--dedupe skip|replace|error` decides what happens when a measurement
  already exists for the same person, marker and time. The default comes
  from config `dedupe`, else `skip`.
* `--dry-run` runs the whole import in a transaction and rolls it back, so
  the counts are exact.
* An invalid row aborts the import and writes nothing. Pass `--skip-invalid`
  to import the valid rows instead.
* `--create-people` and `--create-markers` add unknown slugs on the fly.
* `--person` sets the person for rows without one, and `--tag` tags every
  imported row. Each import gets a batch id, so you can query it with
  `query --batch ID`.
* Values may carry qualifiers (`<0.5`, `>= 90`), thousands separators
  (`1,000`) or a decimal comma (`2,1`).

Round trip: `biomarker export > all.csv`, then importing `all.csv` into an
empty database (with `--create-people`) reproduces the same export. The same
holds for `-f json` and `-f jsonl`.

## Configuration

Settings resolve in this order, lowest to highest precedence:

1. built-in defaults
2. the config file: `--config FILE`, else `$BIOMARKER_CONFIG`, else
   `$XDG_CONFIG_HOME/biomarker-cli/config.toml` (default
   `~/.config/biomarker-cli/config.toml`)
3. environment variables `BIOMARKER_*` (`NO_COLOR` is honoured too)
4. command-line flags

| key | env | default | values |
|-----|-----|---------|--------|
| `db_path` | `BIOMARKER_DB` | `$XDG_DATA_HOME/biomarker-cli/biomarker.db` | path (`~` expanded) |
| `default_person` | `BIOMARKER_PERSON` | — | person slug |
| `format` | `BIOMARKER_FORMAT` | `table` | `table json jsonl csv tsv` |
| `date_format` | `BIOMARKER_DATE_FORMAT` | `%Y-%m-%d` | strftime. Used for table/csv/tsv display, and accepted on input |
| `timezone` | `BIOMARKER_TZ` | `local` | `local`, `UTC`, IANA name, `+HH:MM` |
| `unit_system` | `BIOMARKER_UNITS` | `canonical` | `canonical us si` |
| `color` | `BIOMARKER_COLOR` | `auto` | `auto always never` |
| `precision` | `BIOMARKER_PRECISION` | `2` | decimals in table/csv/tsv |
| `csv_delimiter` | `BIOMARKER_CSV_DELIMITER` | `,` | one char or `tab` |
| `csv_quote` | `BIOMARKER_CSV_QUOTE` | `"` | one char |
| `csv_header` | `BIOMARKER_CSV_HEADER` | `true` | bool |
| `null` | `BIOMARKER_NULL` | empty | text for missing values |
| `range_flavor` | `BIOMARKER_RANGE_FLAVOR` | `reference` | `reference optimal both` |
| `dedupe` | `BIOMARKER_DEDUPE` | `skip` | `skip replace error` (import) |
| `quiet` / `verbose` | `BIOMARKER_QUIET` / `BIOMARKER_VERBOSE` | `false` | bool |

```sh
biomarker config set unit_system si
biomarker config show --effective
biomarker config keys
```

```
KEY             VALUE                                     SOURCE
───             ─────                                     ──────
db_path         /home/me/.local/share/biomarker-cli/...   default
default_person  alex                                      file
format          table                                     default
unit_system     si                                        file
precision       3                                         env
...
```

The config file is flat TOML. One level of tables is flattened, so
`[csv] delimiter = ";"` is the same as `csv_delimiter = ";"`. See
[`examples/config.toml`](examples/config.toml).

## Machine interface

`biomarker query --format json` and `biomarker trend --format json` return a
versioned envelope:

```json
{"schema": "biomarker/v1", "kind": "trend", "count": 1, "data": [{"person": "alex", "marker": "ldl-c", "n": 4, "slope_per_year": -24.9, "points": [...]}]}
```

The fields, all `kind`s, the error object, and the exit codes (0 ok, 1 error,
2 usage, 3 not found, 4 invalid data, 5 database, 6 io, 7 config, 10 flagged
values with `flag --exit-code`) are documented in
[docs/json-schema.md](docs/json-schema.md).

## Data model

SQLite schema, managed by versioned migrations in `src/migrations.rs` and
tracked in `schema_migrations` and `PRAGMA user_version`:

* `people(id, slug, name, sex, dob, notes, tags, created_at)`
* `markers(id, slug, name, category, unit, loinc, description, builtin)` and
  `marker_aliases(alias, marker_id)`
* `units(symbol, system)` and `unit_conversions(marker_id|0 = generic,
  from_unit, to_unit, factor, offset)`
* `ranges(marker_id, kind reference|optimal, sex any|male|female, age_min,
  age_max, low, high, note)`
* `measurements(person_id, marker_id, taken_at, value_raw, unit_raw, value
  (canonical), qualifier, lab, fasting, note, tags, batch_id, created_at)`.
  It is unique on (person, marker, taken_at).
* `import_batches(id, source, format, created_at, row_count)`

## Development

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test        # unit tests + assert_cmd integration tests (tests/cli.rs)
```

All data in `examples/` is synthetic.
