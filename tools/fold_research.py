#!/usr/bin/env python3
"""Fold catalog/research-reports/sources/*.toml (research shards) into catalog/markers/.

Run from the repo root: python3 -I tools/fold_research.py [--dry]
Idempotent: removes previously folded (non-legacy) ranges from existing marker
files before appending, and regenerates new-marker files.
"""
import tomllib, glob, os, re, json, sys, collections

SRC = "catalog/research-reports/sources"
MK = "catalog/markers"
EVID = {"guideline": "guideline", "consensus": "consensus", "reference-interval": "reference-interval-study",
        "reference-interval-study": "reference-interval-study", "outcome": "cohort", "outcome-study": "cohort",
        "cohort-study": "cohort", "validation-study": "cohort", "meta-analysis": "cohort",
        "review": "consensus", "textbook": "consensus"}
RANK = {"guideline": 0, "reference-interval-study": 1, "consensus": 2, "cohort": 3, "rct": 3, "lab-convention": 4}
ENGINE_KINDS = {"reference", "optimal", "warn", "critical"}
POP_SET = "population-ri"
POP_NOTE = "Single-population reference interval, kept out of the default clinical set because the marker has a general clinical reference range."
NO_REF = "No general clinical reference interval is cited; see the diagnostic, target and alternative ranges."
SOFT = {"method", "specimen", "setting", "test", "region"}


def norm_unit(u):
    # kUA/L (allergen units) equal kU/L; mU/L equals mIU/L (international units)
    u = u.replace("kUA/L", "kU/L").replace("mU/L", "mIU/L").replace("uL(-1)", "/µL")
    return u.replace("μ", "µ").replace("mIU/l", "mIU/L").replace("/ml", "/mL").strip()


def T(slug, frm, to, f, o=0.0):
    return ((slug, norm_unit(frm)), (to, f, o))


# (slug, from unit) -> (to unit, factor, offset); canonical = v*factor + offset
CONV = dict([
    T("glucose", "mmol/L", "mg/dL", 18.016), T("hdl-c", "mmol/L", "mg/dL", 38.67), T("ldl-c", "mmol/L", "mg/dL", 38.67),
    T("non-hdl-c", "mmol/L", "mg/dL", 38.67), T("total-cholesterol", "mmol/L", "mg/dL", 38.67),
    T("remnant-cholesterol", "mmol/L", "mg/dL", 38.67),
    T("ldl-c-direct", "mmol/L", "mg/dL", 38.67), T("ldl-c-martin-hopkins", "mmol/L", "mg/dL", 38.67), T("ldl-c-sampson", "mmol/L", "mg/dL", 38.67), T("triglycerides", "mmol/L", "mg/dL", 88.57),
    T("creatinine", "µmol/L", "mg/dL", 0.01131), T("bun", "mmol/L", "mg/dL", 2.801),
    T("bilirubin-total", "µmol/L", "mg/dL", 0.05848), T("bilirubin-direct", "µmol/L", "mg/dL", 0.05848),
    T("uric-acid", "µmol/L", "mg/dL", 0.01681), T("calcium", "mmol/L", "mg/dL", 4.008),
    T("magnesium", "mmol/L", "mg/dL", 2.431), T("phosphorus", "mmol/L", "mg/dL", 3.097),
    T("albumin", "g/L", "g/dL", 0.1), T("total-protein", "g/L", "g/dL", 0.1),
    T("apoa1", "g/L", "mg/dL", 100), T("apob", "g/L", "mg/dL", 100),
    T("igg", "mg/dL", "g/L", 0.01), T("igm", "mg/dL", "g/L", 0.01), T("iga", "mg/dL", "g/L", 0.01),
    T("c3", "mg/dL", "g/L", 0.01), T("c4", "mg/dL", "g/L", 0.01), T("fibrinogen", "mg/dL", "g/L", 0.01),
    T("ceruloplasmin", "g/L", "mg/dL", 100), T("hba1c", "mmol/mol", "%", 0.09148, 2.152),
    T("alt", "IU/L", "U/L", 1), T("ck", "IU/L", "U/L", 1),
    T("cortisol", "nmol/L", "µg/dL", 0.03625), T("estradiol", "pmol/L", "pg/mL", 0.2724),
    T("estrone", "pmol/L", "pg/mL", 0.2703), T("free-t3", "pmol/L", "pg/mL", 0.651),
    T("free-t4", "pmol/L", "ng/dL", 0.0777), T("t3-total", "nmol/L", "ng/dL", 65.1),
    T("testosterone-total", "nmol/L", "ng/dL", 28.84), T("testosterone-total", "ng/mL", "ng/dL", 100),
    T("testosterone-free", "pmol/L", "pg/mL", 0.2884),
    T("dhea-s", "µmol/L", "µg/dL", 36.85), T("dhea-s", "nmol/L", "µg/dL", 0.03685), T("dhea", "nmol/L", "ng/dL", 28.84),
    T("dihydrotestosterone", "nmol/L", "ng/dL", 29.0), T("progesterone", "nmol/L", "ng/mL", 0.3145),
    T("17-ohp", "nmol/L", "ng/dL", 33.05), T("17-ohp", "ng/mL", "ng/dL", 100),
    T("androstenedione", "nmol/L", "ng/dL", 28.64), T("pregnenolone", "nmol/L", "ng/dL", 31.6),
    T("c-peptide", "nmol/L", "ng/mL", 3.02), T("ctx", "ng/L", "ng/mL", 0.001), T("nt-probnp", "ng/L", "pg/mL", 1),
    T("procalcitonin", "µg/L", "ng/mL", 1), T("osteocalcin", "µg/L", "ng/mL", 1), T("p1np", "µg/L", "ng/mL", 1),
    T("igfbp-3", "ng/mL", "mg/L", 0.001), T("igfbp-3", "µg/mL", "mg/L", 1), T("gh", "µg/L", "ng/mL", 1),
    T("prolactin", "mIU/L", "ng/mL", 0.04717), T("prolactin", "µIU/mL", "ng/mL", 0.04717),
    T("lh", "IU/L", "mIU/mL", 1), T("lh", "U/L", "mIU/mL", 1), T("fsh", "IU/L", "mIU/mL", 1),
    T("hcg-quantitative", "IU/L", "mIU/mL", 1), T("ca-19-9", "IU/mL", "U/mL", 1), T("ige-total", "IU/mL", "kU/L", 1),
    T("acth", "ng/L", "pg/mL", 1), T("aldosterone", "ng/L", "ng/dL", 0.1), T("aldosterone", "pmol/L", "ng/dL", 0.0361),
    T("amh", "pmol/L", "ng/mL", 0.14),
    T("metanephrine-free", "nmol/L", "pg/mL", 197.2), T("metanephrine-free", "pmol/L", "pg/mL", 0.1972),
    T("normetanephrine", "nmol/L", "pg/mL", 183.2), T("normetanephrine", "pmol/L", "pg/mL", 0.1832),
    T("tbg", "nmol/L", "mg/L", 0.054), T("bsap", "ng/mL", "µg/L", 1),
    T("salivary-cortisol", "nmol/L", "ng/dL", 3.625), T("salivary-cortisol", "µg/L", "ng/dL", 100),
    T("urine-free-cortisol", "nmol/24 h", "µg/24 h", 0.3625), T("urine-free-cortisol", "µg/day", "µg/24 h", 1),
    T("uacr", "mg/mmol", "mg/g", 8.84), T("upcr", "mg/mmol", "mg/g", 8.84), T("urine-acr", "mg/mmol", "mg/g", 8.84),
    T("upcr", "mg/mg", "mg/g", 1000), T("urine-pcr", "mg/mg", "mg/g", 1000),
    T("urine-osmolality", "mmol/kg", "mOsm/kg", 1),
    # ng/L per mU/L to ng/dL per mIU/L: 1 ng/L = 0.1 ng/dL; mU/L and mIU/L are the same unit
    T("arr-direct-renin", "ng/L/mU/L", "ng/dL per mIU/L", 0.1),
    # 1 MET = 3.5 mL O2/kg/min by definition
    T("vo2max", "METs", "mL/kg/min", 3.5),
    T("corrected-calcium", "mmol/L", "mg/dL", 4.008),
    # 8-OHdG MW 283.24 g/mol, creatinine MW 113.12 g/mol: 1 nmol/mmol creatinine = 2.504 ng/mg creatinine
    T("urine-8-ohdg", "nmol/mmol", "ng/mg creatinine", 2.504), T("urine-8-ohdg", "pmol/umol(creat)", "ng/mg creatinine", 2.504),
    T("immature-granulocytes-abs", "/µL", "10^9/L", 0.001),  # 1000 per uL = 1 x10^9/L
    T("bilirubin-transcutaneous", "µmol/L", "mg/dL", 0.05848), T("bilirubin-indirect", "µmol/L", "mg/dL", 0.05848),
    T("testosterone-free-calculated", "nmol/L", "pg/mL", 288.42),  # testosterone MW 288.42
    # ng/L and pg/mL are numerically equal
    T("pth-intact", "ng/L", "pg/mL", 1),
    # total T4 MW 776.87: 1 nmol/L = 0.0777 ug/dL
    T("t4-total", "nmol/L", "µg/dL", 0.0777),
])
CONV = {(s, norm_unit(u)): v for (s, u), v in CONV.items()}


_MASS = {"": 1.0, "k": 1e3, "m": 1e-3, "µ": 1e-6, "u": 1e-6, "mc": 1e-6, "n": 1e-9, "p": 1e-12, "f": 1e-15}
_VOL = {"": 1.0, "d": 0.1, "c": 0.01, "m": 1e-3, "µ": 1e-6, "u": 1e-6}
_MV = re.compile(r"^(mc|k|m|µ|u|n|p|f)?g/(d|c|m|µ|u)?L$")


_IU = re.compile(r"^(k|m|µ|u)?(IU|U)/(d|c|m|µ|u)?L$", re.I)
# IU (WHO reference standard) and U (arbitrary assay unit) are equal only for enzyme activity
ENZYME_SLUGS = {"alt", "ast", "alp", "ggt", "ldh", "ck", "amylase", "lipase"}


def generic_factor(frm, to, enzyme=False):
    """Exact factor between two mass-per-volume units (g, mg, ug, ng, pg over L, dL, mL) or two
    international-unit-per-volume units (IU/L = U/L = mIU/mL), else None."""
    iu_a, iu_b = _IU.match(norm_unit(frm)), _IU.match(norm_unit(to))
    if iu_a and iu_b and (enzyme or iu_a.group(2).upper() == iu_b.group(2).upper()):
        pre = lambda m: _MASS[(m.group(1) or "").lower()] if (m.group(1) or "").lower() != "m" else 1e-3
        vol = lambda m: _VOL[(m.group(3) or "").lower()]
        return (pre(iu_a) / vol(iu_a)) / (pre(iu_b) / vol(iu_b))
    a, b = _MV.match(norm_unit(frm)), _MV.match(norm_unit(to))
    if not (a and b):
        return None
    mass = lambda m: _MASS[m.group(1) or ""]
    vol = lambda m: _VOL[m.group(2) or ""]
    return (mass(a) / vol(a)) / (mass(b) / vol(b))


def sig(x, n=4):
    return float(f"{x:.{n}g}")


def tv(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return json.dumps(v, ensure_ascii=False)
    if isinstance(v, list):
        return "[" + ", ".join(tv(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {tv(x)}" for k, x in v.items()) + " }"
    raise TypeError(v)


def slugify(s):
    return re.sub(r"-+", "-", re.sub(r"[^a-z0-9]+", "-", s.lower())).strip("-")


def strip_folded(text):
    parts = re.split(r"(?=^\[\[range\]\]\n)", text, flags=re.M)
    keep = [parts[0]] + [p for p in parts[1:] if re.search(r"^legacy = true", p, re.M)]
    return "".join(keep).rstrip() + "\n"


def range_key(r):
    num = lambda v: None if v is None else float(v)
    when = {k: (sorted(v) if k == "conditions" else [float(x) for x in v] if k == "age" else v) for k, v in r.get("when", {}).items()}
    return (r.get("set", "clinical"), r.get("kind", "reference"), json.dumps(when, sort_keys=True), num(r.get("low")), num(r.get("high")))


def drop_superseded(text, final):
    """Remove legacy ranges that a cited range now states exactly (same set, kind, when and bounds)."""
    cited = {range_key(r) for r in final}
    parts = re.split(r"(?=^\[\[range\]\]\n)", text, flags=re.M)
    keep = [parts[0]] + [p for p in parts[1:] if range_key(tomllib.loads(p)["range"][0]) not in cited]
    return "".join(keep).rstrip() + "\n", len(parts) - len(keep)


def convert_when(w, known):
    out = {}
    restrict = []
    soft = []
    conds = []
    for k, v in w.items():
        if k == "sex" and v in ("male", "female"):
            out["sex"] = v
        elif k in ("pregnant", "pregnancy") and isinstance(v, bool):
            out["pregnant"] = v
        elif k == "trimester" and v in (1, 2, 3):
            out["trimester"] = v
        elif k == "fasting" and isinstance(v, bool):
            out["fasting"] = v
        elif k == "menopause" and v in ("pre", "peri", "post"):
            out["menopause"] = v
        elif k == "cycle_phase":
            v2 = {"midcycle": "ovulatory"}.get(v, v)
            if v2 in ("menstrual", "follicular", "ovulatory", "luteal"):
                out["cycle_phase"] = v2
            else:
                restrict.append(f"cycle_phase={v}")
        elif k in ("conditions", "condition"):
            conds += [slugify(c) for c in (v if isinstance(v, list) else [v])]
        elif k == "markers":
            ms = []
            ok = isinstance(v, list)
            for m in (v if isinstance(v, list) else []):
                if not isinstance(m, dict):
                    ok = False
                    break
                slug = m.get("marker") or m.get("slug")
                if "op" in m and "value" in m and slug and "unit" not in m:
                    ms.append({"marker": slug, "op": m["op"], "value": m["value"]})
                elif slug and "high" in m and len(m) == 2:
                    ms.append({"marker": slug, "op": "<=", "value": m["high"]})
                elif slug and "low" in m and len(m) == 2:
                    ms.append({"marker": slug, "op": ">=", "value": m["low"]})
                else:
                    ok = False
            if ok and ms and all(m["marker"] in known and m["op"] in ("<", "<=", ">", ">=", "=") for m in ms):
                out["markers"] = ms
            else:
                restrict.append("markers=" + json.dumps(v, ensure_ascii=False))
        elif k in ("age_min", "age_max", "age_unit"):
            pass
        elif k in SOFT:
            soft.append(f"{k}: {v}")
        else:
            restrict.append(f"{k}={v}")
    if "age_min" in w or "age_max" in w:
        div = {"months": 12.0, "days": 365.25, "weeks": 52.18, "years": 1.0}.get(w.get("age_unit", "years"), 1.0)
        lo = float(w.get("age_min", 0)) / div
        hi = float(w["age_max"]) / div if "age_max" in w else 200
        if "age_max" in w and float(w["age_max"]) == float(w.get("age_min", 0)):
            # A point on an age curve ("at age 22") covers that whole year of age: [22, 23).
            hi = (float(w["age_max"]) + 1) / div
        out["age"] = [round(lo, 4), round(hi, 4) if "age_max" in w else 200]
    if conds:
        out["conditions"] = sorted(set(conds))
    return out, restrict, soft


def spec(rng):
    w = rng["when"]
    # Same specificity as the engine (Cond::count): each marker and condition counts once.
    n = sum(len(v) if k in ("markers", "conditions") else 1 for k, v in w.items() if k not in ("sex", "age"))
    a = w.get("age", [0, 200])
    return (n, 1 if "sex" in w else 0, -(a[1] - a[0]))


def can_comatch(a, b):
    wa, wb = a["when"], b["when"]
    if wa.get("sex") and wb.get("sex") and wa["sex"] != wb["sex"]:
        return False
    aa = wa.get("age", [0, 200])
    ab = wb.get("age", [0, 200])
    if aa[1] <= ab[0] or ab[1] <= aa[0]:
        return False
    for k in ("pregnant", "trimester", "fasting", "menopause", "cycle_phase"):
        if k in wa and k in wb and wa[k] != wb[k]:
            return False
    if (wa.get("pregnant") is False and wb.get("trimester")) or (wb.get("pregnant") is False and wa.get("trimester")):
        return False
    return True


def conflicts(a, b):
    if a["kind"] != b["kind"] or a["set"] != b["set"]:
        return False
    if (a.get("low"), a.get("high")) == (b.get("low"), b.get("high")):
        return False
    return spec(a) == spec(b) and can_comatch(a, b)


def bound_side(zone, ref):
    """Which side of the reference range a two-sided band [a, b] lies on: above, below, near-low or near-high."""
    (a, b), lo, hi = zone, ref.get("low"), ref.get("high")
    m = (a + b) / 2
    if hi is not None and m >= hi:
        return "above"
    if lo is not None and m <= lo:
        return "below"
    if lo is not None and hi is not None:
        return "near-low" if m - lo < hi - m else "near-high"
    if lo is not None:
        return "near-low" if a <= lo else "above"
    if hi is not None:
        return "near-high" if b >= hi else "below"
    return None


def to_cut_points(c, refs):
    """Research bands are zones ("[low, high] is where the label applies"); the engine's warn and
    critical ranges are cut points beside the reference limits (docs/json-schema.md). Returns the
    engine bounds and a note, or None when a two-sided band cannot be oriented."""
    a, b = c.get("low"), c.get("high")
    crit = c["kind"] == "critical"
    if b is None:
        out = {"high": a}
    elif a is None:
        out = {"low": b}
    else:
        same = [r for r in refs if r["when"] == c["when"]] or [r for r in refs if can_comatch(r, c)] or refs
        side = bound_side((a, b), same[0]) if same else None
        if side is None or (crit and side.startswith("near")):
            return None
        out = {"above": {"high": a if crit else b}, "below": {"low": b if crit else a},
               "near-low": {"low": b}, "near-high": {"high": a}}[side]
    (k, v), = out.items()
    band = f"{'' if a is None else a}-{'' if b is None else b}" if a is not None and b is not None else (f">= {a}" if b is None else f"<= {b}")
    return out, f"Source band {band}; stored as the {c['kind']} {k} bound {v}."


def build():
    ex = {os.path.basename(f)[:-5]: tomllib.load(open(f, "rb")) for f in glob.glob(f"{MK}/*.toml")}
    research = {}
    for f in sorted(glob.glob(f"{SRC}/*.toml")):
        d = tomllib.load(open(f, "rb"))
        research[d["slug"]] = d
    # A range whose source unit cannot be converted to the marker's unit measures a different
    # quantity (vendor units, mass vs activity, ...). Move it to a companion marker in that unit
    # instead of dropping it.
    for slug in sorted(research):
        rd = research[slug]
        dest = norm_unit(ex[slug]["unit"] if slug in ex else rd["unit"])
        keep, moved = [], {}
        for r in rd.get("range", []):
            u = norm_unit(r.get("unit", rd["unit"]))
            gf = None if (u == dest or (slug, u) in CONV) else generic_factor(u, dest, slug in ENZYME_SLUGS)
            if gf is not None:
                CONV[(slug, u)] = (dest, gf, 0.0)
            if u == dest or (slug, u) in CONV:
                keep.append(r)
            else:
                moved.setdefault(u, []).append(r)
        for u, rs in moved.items():
            comp = f"{slug}-{slugify(u)}"
            c = research.setdefault(comp, {
                "slug": comp, "name": f"{rd['name']} ({u})", "category": rd["category"], "unit": u,
                "description": f"{rd['name']} reported in {u}. Results in {u} are assay specific and are not interchangeable with {rd['name']} in {dest}.",
                "range": []})
            c["range"] += [{k: v for k, v in r.items() if k != "unit"} for r in rs]
        rd["range"] = keep
    known = set(ex) | set(research)
    sets = {}
    conditions = {}
    dropped = collections.Counter()
    drop_log = []
    stats = {"ranges_in": 0, "ranges_out": 0, "markers_new": 0, "markers_ext": 0}
    alias_taken = {}
    for s, d in ex.items():
        alias_taken[s] = s
        for a in d.get("aliases", []):
            alias_taken.setdefault(a.lower(), s)
    for s in research:
        alias_taken.setdefault(s, s)
    out_files = {}
    for slug, rd in sorted(research.items()):
        e = ex.get(slug)
        dest_unit = e["unit"] if e else rd["unit"]
        file_unit = rd["unit"]
        cands = []
        for r in rd.get("range", []):
            stats["ranges_in"] += 1

            def drop(why, r=r):
                dropped[why] += 1
                drop_log.append((slug, why, r.get("low"), r.get("high")))

            low, high = r.get("low"), r.get("high")
            if low is None and high is None:
                drop("no bounds")
                continue
            note_bits = []
            u = norm_unit(r.get("unit", file_unit))
            if u != norm_unit(dest_unit):
                c = CONV.get((slug, u))
                if not c or norm_unit(c[0]) != norm_unit(dest_unit):
                    drop(f"unit {u}->{dest_unit} not convertible")
                    continue
                _, f, o = c
                src = "-".join(str(x) for x in (low, high) if x is not None)
                low = None if low is None else sig(low * f + o)
                high = None if high is None else sig(high * f + o)
                note_bits.append(f"Converted from the source's {u} ({src}) to {dest_unit} using factor {f}" + (f" offset {o}" if o else "") + ".")
            if low is not None and high is not None and low > high:
                drop("low>high")
                continue
            cits = []
            for c in r.get("citations", []):
                cc = {}
                if re.fullmatch(r"[0-9]+", str(c.get("pmid", ""))):
                    cc["pmid"] = str(c["pmid"])
                if re.fullmatch(r"10\.[0-9]{4,9}/\S+", str(c.get("doi", ""))):
                    cc["doi"] = c["doi"]
                if re.match(r"https?://", str(c.get("url", ""))):
                    cc["url"] = c["url"]
                if not cc:
                    continue
                if c.get("title"):
                    cc["title"] = c["title"]
                if c.get("locator"):
                    cc["locator"] = c["locator"]
                cits.append(cc)
            if not cits:
                drop("no usable citation")
                continue
            when, restrict, soft = convert_when(r.get("when", {}), known)
            for cd in when.get("conditions", []):
                conditions[cd] = True
            k0, s0 = r["kind"], r.get("set", "clinical")
            kind = k0 if k0 in ENGINE_KINDS else ("optimal" if k0 in ("target", "therapeutic") else "warn")
            if s0 in ("clinical", "reference"):
                if k0 in ("reference", "warn", "critical"):
                    sname = "clinical"
                elif k0 == "optimal":
                    sname = "optimal"
                elif k0 in ("diagnostic", "decision"):
                    sname = "diagnostic"
                else:
                    sname = k0
            elif s0 == "optimal":
                sname = "optimal"
            else:
                sname = slugify(s0)
            if restrict and sname in ("clinical", "optimal"):
                sname = "contextual"
            pop = r.get("population", "")
            soft += [f"{k}: {r[k]}" for k in ("specimen", "region") if k in r]
            if soft:
                pop = pop + (" " if pop else "") + "(" + "; ".join(soft) + ")"
            if restrict:
                note_bits.append("Applies only to: " + "; ".join(restrict) + ".")
            if k0 not in ENGINE_KINDS:
                note_bits.append(f"Source kind: {k0}.")
            note = " ".join(([r["note"]] if r.get("note") else []) + note_bits)
            rng = {"set": sname, "kind": kind, "_src": s0}
            if low is not None:
                rng["low"] = low
            if high is not None:
                rng["high"] = high
            rng["when"] = when
            rng["population"] = pop
            rng["evidence"] = EVID.get(r.get("evidence"), "consensus")
            rng["citations"] = cits
            if note:
                rng["note"] = note
            cands.append(rng)
        # Only legacy ranges survive strip_folded; ranges folded by an earlier run are rebuilt here.
        kept = [r for r in e.get("range", []) if r.get("legacy")] if e else []
        # A marker this script created on an earlier run is treated as new again.
        created = not e or (not kept and e.get("no_reference", NO_REF) == NO_REF)
        # A single study's reference interval (source set "reference") must not shadow a
        # general clinical reference: an age band would make it the more specific match.
        general = any(r.get("set", "clinical") == "clinical" and r.get("kind", "reference") == "reference"
                      for r in kept) or any(c["_src"] == "clinical" and c["set"] == "clinical" and c["kind"] == "reference" for c in cands)
        if general:
            for c in cands:
                if c["_src"] == "reference" and c["set"] == "clinical" and c["kind"] == "reference":
                    c["set"] = POP_SET
                    c["note"] = (c.get("note", "") + " " + POP_NOTE).strip()
        refs = [dict(r, when=dict(r.get("when", {}))) for r in kept if r.get("set", "clinical") == "clinical" and r.get("kind", "reference") == "reference"]
        refs += [c for c in cands if c["set"] == "clinical" and c["kind"] == "reference"]
        # without a clinical reference, any cited reference interval shows where normal is
        refs = refs or [c for c in cands if c["kind"] == "reference"]
        for c in list(cands):
            if c["kind"] not in ("warn", "critical"):
                continue
            cut = to_cut_points(c, refs)
            if cut is None:
                # A two-sided band that cannot be oriented against a reference range is kept as a
                # reference range in a set of its own (the set name carries the source and the bounds).
                base = slugify(c["_src"]) if c["_src"] not in ("clinical", "reference", "optimal") else f"{slug}-bands"
                c["kind"] = "reference"
                c["set"] = slugify(f"{base}-{c.get('low')}-{c.get('high')}")
                c["note"] = (c.get("note", "") + " Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point.").strip()
                stats["bands_kept"] = stats.get("bands_kept", 0) + 1
                continue
            c.pop("low", None)
            c.pop("high", None)
            c.update(cut[0])
            c["note"] = (c.get("note", "") + " " + cut[1]).strip()
        placed = []
        if kept:
            for r in kept:
                placed.append({"set": r.get("set", "clinical"), "kind": r.get("kind", "reference"),
                               "low": r.get("low"), "high": r.get("high"), "when": dict(r.get("when", {}))})
        cands.sort(key=lambda c: (RANK.get(c["evidence"], 9), 0 if c["set"] in ("clinical", "optimal") else 1, -len(c["citations"])))
        final = []
        for c in cands:
            tgt = c
            n = 0
            while any(conflicts(tgt, p) for p in placed):
                n += 1
                tgt = dict(c)
                tgt["set"] = f"alt-{n}"
                if n > 60:
                    break
            if tgt is not c:
                tgt["note"] = (c.get("note", "") + f" Alternative to the default {c['set']} {c['kind']} range for the same population.").strip()
            placed.append(tgt)
            final.append(tgt)
        has_clin = lambda: any(r["set"] == "clinical" and r["kind"] == "reference" for r in final) or any(r.get("set", "clinical") == "clinical" and r.get("kind", "reference") == "reference" for r in kept)
        if created and not has_clin():
            for a in final:
                if a["kind"] == "reference" and a["set"] not in ("optimal", "contextual", "diagnostic"):
                    t = dict(a, set="clinical")
                    if not any(conflicts(t, p) for p in final if p is not a):
                        a["set"] = "clinical"
                        break
        for r in final:
            sets[r["set"]] = True
        lines = []
        for r in final:
            lines.append("[[range]]")
            for k in ("set", "kind", "low", "high"):
                if k in r:
                    lines.append(f"{k} = {tv(r[k])}")
            if r["when"]:
                lines.append(f"when = {tv(r['when'])}")
            lines.append(f"population = {tv(r['population'])}")
            lines.append(f"evidence = {tv(r['evidence'])}")
            lines.append("citations = [")
            for c in r["citations"]:
                lines.append(f"  {tv(c)},")
            lines.append("]")
            if r.get("note"):
                lines.append(f"note = {tv(r['note'])}")
            lines.append("")
        stats["ranges_out"] += len(final)
        if e:
            text, n = drop_superseded(strip_folded(open(f"{MK}/{slug}.toml").read()), final)
            stats["legacy_superseded"] = stats.get("legacy_superseded", 0) + n
            if created:
                text = text.replace(f"no_reference = {tv(NO_REF)}\n", "")
                if not any(r["set"] == "clinical" and r["kind"] == "reference" for r in final):
                    text = text.rstrip() + f"\nno_reference = {tv(rd.get('no_reference', NO_REF))}\n"
            if "description" not in e and rd.get("description"):
                text = re.sub(r"^(unit = .*\n)", lambda m: m.group(1) + f"description = {tv(rd['description'])}\n", text, count=1, flags=re.M)
            if "loinc" not in e and rd.get("loinc") and re.fullmatch(r"[0-9]{1,7}-[0-9]", rd["loinc"]):
                text = re.sub(r"^(unit = .*\n)", lambda m: m.group(1) + f"loinc = {tv(rd['loinc'])}\n", text, count=1, flags=re.M)
            body = text.rstrip() + "\n" + ("\n" + "\n".join(lines) if lines else "")
            stats["markers_ext"] += 1
        else:
            hdr = [f"slug = {tv(slug)}", f"name = {tv(rd['name'])}", f"category = {tv(rd['category'])}", f"unit = {tv(rd['unit'])}"]
            if rd.get("loinc") and re.fullmatch(r"[0-9]{1,7}-[0-9]", rd["loinc"]):
                hdr.append(f"loinc = {tv(rd['loinc'])}")
            if rd.get("description"):
                hdr.append(f"description = {tv(rd['description'])}")
            al = []
            for a in rd.get("aliases", []):
                if a.lower() not in alias_taken and a.lower() not in al:
                    al.append(a.lower())
                    alias_taken[a.lower()] = slug
            if al:
                hdr.append(f"aliases = {tv(al)}")
            if not any(r["set"] == "clinical" and r["kind"] == "reference" for r in final):
                hdr.append(f"no_reference = {tv(rd.get('no_reference', NO_REF))}")
            body = "\n".join(hdr) + "\n\n" + "\n".join(lines)
            stats["markers_new"] += 1
        out_files[slug] = body.rstrip() + "\n"
    return out_files, sets, conditions, dropped, drop_log, stats


def main():
    dry = "--dry" in sys.argv
    files, sets, conds, dropped, drop_log, stats = build()
    print(stats)
    print(dict(dropped))
    if dry:
        return
    for s, b in files.items():
        open(f"{MK}/{s}.toml", "w").write(b)
    st = open("catalog/sets.toml").read()
    have = set(re.findall(r'^name = "([^"]+)"', st, re.M))
    desc = {"contextual": "Cited ranges that apply only under a condition the engine cannot express (see each range's note)",
            "diagnostic": "Diagnostic and decision cut-offs from the literature, kept apart from reference intervals",
            "target": "Treatment and monitoring targets",
            POP_SET: "Reference intervals from single-population studies (one country, cohort or analyser); select with range_sets", "therapeutic": "Therapeutic ranges",
            "deficient": "Deficiency cut-offs", "insufficient": "Insufficiency cut-offs",
            "toxic": "Toxicity cut-offs", "excess": "Excess cut-offs"}
    add = ""
    for s in sorted(sets):
        if s in have:
            continue
        d = desc.get(s) or ("Alternative cited reference or cut-off for a population another range already covers" if s.startswith("alt-") else f"Published source set: {s}")
        add += f'\n[[set]]\nname = "{s}"\ndescription = {json.dumps(d)}\n'
    open("catalog/sets.toml", "w").write(st.rstrip() + "\n" + add)
    ct = open("catalog/conditions.toml").read()
    havec = set(re.findall(r'^slug = "([^"]+)"', ct, re.M))
    addc = "".join(f'\n[[condition]]\nslug = "{c}"\nname = {json.dumps(c.replace("-", " ").capitalize())}\n' for c in sorted(conds) if c not in havec)
    open("catalog/conditions.toml", "w").write(ct.rstrip() + "\n" + addc)
    open("catalog/research-reports/FOLD-DROPPED.tsv", "w").write("slug\treason\tlow\thigh\n" + "".join(f"{a}\t{b}\t{c}\t{d}\n" for a, b, c, d in drop_log))


main()
