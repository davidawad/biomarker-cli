#!/usr/bin/env sh
# Replay the README sample session against the synthetic data in examples/ and
# print it as a transcript: each command after a `$ ` prompt, then its real
# output. Uses whichever `biomarker` is first on PATH and a throwaway database.
#
#   scripts/readme-session.sh [full|hero|import]
#
# `full` (default) is the copyable session embedded in README.md and stored in
# docs/readme-session.txt. `hero` shows only the commands marked `*` below
# (the others still run, silently) and is rendered to the hero screenshot.
# `import` is the spreadsheet import example (docs/readme-import.txt, the
# "Import from a spreadsheet" section of README.md).
# Set BIOMARKER_COLOR=always to keep ANSI colours.
# tests/readme.rs fails when docs/readme-session.txt drifts from this output;
# scripts/readme-samples.sh regenerates it, the screenshot and README.md.
set -eu

session=${1:-full}
case "$session" in
  full | hero | import) ;;
  *)
    echo "usage: $0 [full|hero|import]" >&2
    exit 2
    ;;
esac

repo=$(cd "$(dirname "$0")/.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# Isolate from the caller's config and data, and pin everything that would
# otherwise vary between runs (time zone, `generated_at`).
unset BIOMARKER_CONFIG BIOMARKER_PERSON BIOMARKER_FORMAT BIOMARKER_UNITS \
  BIOMARKER_PRECISION BIOMARKER_RANGE_FLAVOR BIOMARKER_DATE_FORMAT \
  BIOMARKER_DEDUPE BIOMARKER_NULL BIOMARKER_QUIET BIOMARKER_VERBOSE NO_COLOR
export HOME="$tmp" XDG_CONFIG_HOME="$tmp/config" XDG_DATA_HOME="$tmp/data"
export BIOMARKER_DB="$tmp/labs.db" BIOMARKER_TZ=UTC SOURCE_DATE_EPOCH=1767225600
export BIOMARKER_COLOR="${BIOMARKER_COLOR:-never}"
# The database is encrypted; use a fixed throwaway test key from the
# environment (not the caller's SSH key).
export BIOMARKER_KEY_SOURCE=env
export BIOMARKER_KEY=raw:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f
cd "$repo"

# `*` = also shown in the hero screenshot, `-` = full session only. A line
# ending in `\` continues on the next one, as in an interactive shell.
first=1
replay() {
  cmd=
  while IFS= read -r line; do
    if [ -z "$cmd" ]; then
      mark=${line%% *}
      line=${line#* }
      shown="\$ $line"
    else
      shown="$shown
$line"
    fi
    cmd="$cmd$line"
    case "$line" in *\\)
      cmd="${cmd%\\}"
      continue
      ;;
    esac
    if [ "$session" = hero ] && [ "$mark" != '*' ]; then
      eval "$cmd" > /dev/null 2>&1 < /dev/null
    else
      [ "$first" = 1 ] || echo
      first=0
      printf '%s\n' "$shown"
      eval "$cmd" 2>&1 < /dev/null
    fi
    cmd=
  done
}

if [ "$session" = import ]; then
  replay << 'EOF'
- sh examples/people.sh
- export BIOMARKER_PERSON=alex
- biomarker import examples/dashboard.xlsx --list-sheets
- biomarker import examples/dashboard.xlsx --mapping examples/dashboard-mapping.toml \
    --create-markers --dry-run
- biomarker import examples/dashboard.xlsx --sheet Body \
    --value-column 'weight=Weight (lbs.):lb' --value-column 'bmi=BMI kg/m²'
- biomarker latest -m weight,bmi --units si --columns taken_at,marker,value,unit
EOF
  exit 0
fi

replay << 'EOF'
- sh examples/people.sh
- export BIOMARKER_PERSON=alex
* biomarker import examples/showcase.csv
- biomarker latest -c lipid --units si
* biomarker flag --latest \
    --columns taken_at,marker,value,unit,ref_high,ref_flag,opt_high,opt_flag
* biomarker trend -m ldl,apob,hba1c,vitamin-d \
    --columns marker,unit,n,first,last,change_pct,slope_per_year,last_flag
* biomarker diff 2023-02-14 2025-03-11 --changed \
    --columns marker,unit,from_value,to_value,change_pct,from_flag,to_flag
- biomarker trend -m apob --format json | head -n 32
EOF
