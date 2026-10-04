#!/usr/bin/env sh
# Regenerate every README sample from scratch with a fresh release build:
#
#   docs/readme-session.txt      the copyable session (plain text)
#   docs/readme-import.txt       the spreadsheet import example (plain text)
#   docs/screenshots/hero.png    the hero screenshot (needs uv)
#   README.md                    the blocks between the readme-session / readme-import markers
#
# tests/readme.rs fails when the committed session drifts from the binary.
set -eu

repo=$(cd "$(dirname "$0")/.." && pwd)
cd "$repo"

cargo build --release --quiet
bin_dir=$(cargo metadata --format-version 1 --no-deps |
    sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')/release
export PATH="$bin_dir:$PATH"

BIOMARKER_COLOR=never scripts/readme-session.sh full >docs/readme-session.txt
BIOMARKER_COLOR=never scripts/readme-session.sh import >docs/readme-import.txt
mkdir -p docs/screenshots
BIOMARKER_COLOR=always scripts/readme-session.sh hero |
    uv run --quiet scripts/render-terminal.py docs/screenshots/hero.png \
        --title "biomarker — synthetic example data"

# Splice the sessions into README.md between their markers.
splice() {
    tmp=$(mktemp)
    awk -v session="$2" -v name="$1" '
        $0 == "<!-- " name ":begin -->" {
            print; print "```console"
            while ((getline line < session) > 0) print line
            print "```"; skip = 1; next
        }
        $0 == "<!-- " name ":end -->" { skip = 0 }
        !skip
    ' README.md >"$tmp"
    cat "$tmp" >README.md
    rm -f "$tmp"
}
splice readme-session docs/readme-session.txt
splice readme-import docs/readme-import.txt

echo "regenerated docs/readme-session.txt, docs/readme-import.txt, docs/screenshots/hero.png and README.md"
