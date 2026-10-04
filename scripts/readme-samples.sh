#!/usr/bin/env sh
# Regenerate every README sample from scratch with a fresh release build:
#
#   docs/readme-session.txt      the copyable session (plain text)
#   docs/screenshots/hero.png    the hero screenshot (needs uv)
#   README.md                    the session block between the readme-session markers
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
mkdir -p docs/screenshots
BIOMARKER_COLOR=always scripts/readme-session.sh hero |
    uv run --quiet scripts/render-terminal.py docs/screenshots/hero.png \
        --title "biomarker — synthetic example data"

# Splice the session into README.md between the markers.
tmp=$(mktemp)
awk -v session=docs/readme-session.txt '
    /<!-- readme-session:begin -->/ {
        print; print "```console"
        while ((getline line < session) > 0) print line
        print "```"; skip = 1; next
    }
    /<!-- readme-session:end -->/ { skip = 0 }
    !skip
' README.md >"$tmp"
cat "$tmp" >README.md
rm -f "$tmp"

echo "regenerated docs/readme-session.txt, docs/screenshots/hero.png and README.md"
