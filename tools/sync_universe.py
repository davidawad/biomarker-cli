#!/usr/bin/env python3
"""Add catalog markers missing from catalog/universe.toml (companion markers made by fold_research.py).

Run from the repo root after tools/fold_research.py: python3 -I tools/sync_universe.py
A companion `<parent>-<unit>` gets its category from its marker file, priority specialist and no LOINC.
"""
import glob
import json
import tomllib

uni = tomllib.load(open("catalog/universe.toml", "rb"))["marker"]
have = {m["slug"]: m for m in uni}
out = []
for f in sorted(glob.glob("catalog/markers/*.toml")):
    d = tomllib.load(open(f, "rb"))
    s = d["slug"]
    if s in have:
        continue
    parent = max((p for p in have if s.startswith(p + "-")), key=len, default=None)
    name = json.dumps(d["name"], ensure_ascii=False)
    out.append(
        f'\n[[marker]]\nslug = "{s}"\nname = {name}\nloinc = ""\n'
        f'loinc_note = "vendor-unit companion of {parent or "its parent marker"}"\n'
        f'category = "{d["category"]}"\npriority = "specialist"\nderived = false\n'
    )
if out:
    open("catalog/universe.toml", "a").write("".join(out))
print(f"universe entries added: {len(out)}")
