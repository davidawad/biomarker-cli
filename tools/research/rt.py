#!/usr/bin/env python3
"""rt: research + verification helper for catalog/research-reports/sources/*.toml.

Subcommands
  search  "<pubmed query>" [-n N]      esearch+esummary: PMID, year, journal, DOI, PMCID, title
  epmc    "<europe pmc query>" [-n N]  Europe PMC search (shows open-access flag / PMCID)
  abstract <pmid>                      title + abstract (cached)
  fulltext <pmid|PMCxxxx>              PMC full text -> cache; prints char count (use grep to read)
  url      <url>                       fetch HTML/PDF guideline page -> text cache
  grep     <pmid|PMCxxxx|url> <regex> [-C N]   search all cached text of a source, with context
  show     <pmid|PMCxxxx|url> [--from N --len N]  print a slice of a cached source text
  verify   <file.toml ...>             deterministic verification (exit 1 on any failure)

All NCBI calls share a cross-process lock that keeps the box under 3 req/s.
Text is normalised (unicode dashes/inequalities, whitespace) the same way for
printing and for verification, so quotes copied from this tool's output match.
"""
import argparse, fcntl, hashlib, html, json, os, re, subprocess, sys, time, tomllib, unicodedata
import urllib.parse
import requests

CACHE = os.environ.get("RT_CACHE", "/work/artifacts/cache")
os.makedirs(CACHE, exist_ok=True)
EUTILS = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/"
UA = {"User-Agent": "biomarker-cli-research/0.1 (mailto:research@example.invalid)"}
S = requests.Session()
S.headers.update(UA)


def throttle(name, gap):
    path = os.path.join(CACHE, f".{name}.lock")
    with open(path, "a+") as f:
        fcntl.flock(f, fcntl.LOCK_EX)
        f.seek(0)
        try:
            last = float(f.read().strip() or 0)
        except ValueError:
            last = 0.0
        wait = last + gap - time.time()
        if wait > 0:
            time.sleep(wait)
        f.seek(0)
        f.truncate()
        f.write(str(time.time()))
        f.flush()
        fcntl.flock(f, fcntl.LOCK_UN)


def get(url, params=None, kind="ncbi", tries=4, **kw):
    gap = {"ncbi": 0.4, "epmc": 0.25, "doi": 0.2}.get(kind, 0.5)
    for i in range(tries):
        throttle(kind, gap)
        try:
            r = S.get(url, params=params, timeout=60, **kw)
        except requests.RequestException as e:
            if i == tries - 1:
                raise
            time.sleep(2 * (i + 1))
            continue
        if r.status_code in (429, 500, 502, 503, 504):
            time.sleep(2 * (i + 1))
            continue
        return r
    return r


def norm(s):
    s = html.unescape(s)
    s = unicodedata.normalize("NFKC", s)
    for a, b in {"≤": "<=", "≥": ">=", "≦": "<=", "≧": ">=", "–": "-", "—": "-",
                 "−": "-", "‐": "-", "‑": "-", "‒": "-", " ": " ", " ": " ",
                 " ": " ", "​": "", "­": "", "‘": "'", "’": "'", "“": '"',
                 "”": '"', "·": ".", "×": "x", "µ": "u", "μ": "u"}.items():
        s = s.replace(a, b)
    return re.sub(r"\s+", " ", s).strip()


def key(s):
    """Comparison key: normalised, lowercase, whitespace and spaces around punctuation removed."""
    s = norm(s).lower().replace("|", "")  # table cell separators are rendering, not content
    return re.sub(r"\s+", "", s)


def cpath(name):
    return os.path.join(CACHE, name)


def cached(name, fn):
    p = cpath(name)
    if os.path.exists(p):
        return open(p, encoding="utf-8").read()
    txt = fn()
    if txt is not None:
        open(p, "w", encoding="utf-8").write(txt)
    return txt


# ---------- PubMed ----------

def esummary(pmids):
    pmids = [str(p) for p in pmids]
    out, todo = {}, []
    for p in pmids:
        f = cpath(f"sum-{p}.json")
        if os.path.exists(f):
            out[p] = json.load(open(f))
        else:
            todo.append(p)
    for i in range(0, len(todo), 100):
        chunk = todo[i:i + 100]
        r = get(EUTILS + "esummary.fcgi", {"db": "pubmed", "id": ",".join(chunk), "retmode": "json"})
        res = r.json().get("result", {})
        for p in chunk:
            d = res.get(p)
            if d and "error" not in d:
                json.dump(d, open(cpath(f"sum-{p}.json"), "w"))
                out[p] = d
    return out


def ids_of(d):
    ids = {a.get("idtype"): a.get("value") for a in d.get("articleids", [])}
    return ids.get("doi"), ids.get("pmc")


def cmd_search(a):
    r = get(EUTILS + "esearch.fcgi", {"db": "pubmed", "term": a.query, "retmax": a.n, "retmode": "json", "sort": a.sort})
    ids = r.json()["esearchresult"].get("idlist", [])
    sums = esummary(ids)
    for p in ids:
        d = sums.get(p, {})
        doi, pmc = ids_of(d)
        print(f"PMID {p} | {d.get('pubdate','')[:4]} | {d.get('source','')} | doi:{doi or '-'} | {pmc or 'no-PMC'}\n   {norm(d.get('title',''))}")


def cmd_epmc(a):
    r = get("https://www.ebi.ac.uk/europepmc/webservices/rest/search",
            {"query": a.query, "format": "json", "pageSize": a.n, "resultType": "lite"}, kind="epmc")
    for x in r.json().get("resultList", {}).get("result", []):
        print(f"PMID {x.get('pmid','-')} | {x.get('pubYear','')} | {x.get('journalTitle','')} | doi:{x.get('doi','-')} | "
              f"{x.get('pmcid','no-PMC')} | OA={x.get('isOpenAccess')}\n   {norm(x.get('title',''))}")


def abstract_text(pmid):
    def fetch():
        r = get(EUTILS + "efetch.fcgi", {"db": "pubmed", "id": pmid, "retmode": "xml"})
        x = r.text
        if "<PubmedArticle" not in x:
            return None
        title = re.search(r"<ArticleTitle[^>]*>(.*?)</ArticleTitle>", x, re.S)
        parts = []
        for m in re.finditer(r"<AbstractText([^>]*)>(.*?)</AbstractText>", x, re.S):
            lab = re.search(r'Label="([^"]*)"', m.group(1))
            body = re.sub(r"<[^>]+>", "", m.group(2))
            parts.append((lab.group(1) + ": " if lab else "") + body)
        t = re.sub(r"<[^>]+>", "", title.group(1)) if title else ""
        return norm(t) + "\n\n" + "\n".join(norm(p) for p in parts)
    return cached(f"abs-{pmid}.txt", fetch)


def cmd_abstract(a):
    t = abstract_text(a.pmid)
    d = esummary([a.pmid]).get(str(a.pmid), {})
    doi, pmc = ids_of(d)
    print(f"PMID {a.pmid} | {d.get('pubdate','')} | {d.get('source','')} | doi:{doi} | {pmc or 'no-PMC'}")
    print(t or "(no abstract)")


def xml_to_text(x):
    x = re.sub(r"<(xref|sup|sub|italic|bold|ext-link)[^>]*>", "", x)
    x = re.sub(r"</(xref|sup|sub|italic|bold|ext-link)>", "", x)
    x = re.sub(r"<(td|th)[^>]*>", " | ", x)
    x = re.sub(r"</(p|title|tr|caption|label|list-item)>", "\n", x)
    x = re.sub(r"<[^>]+>", " ", x)
    return "\n".join(norm(l) for l in x.split("\n") if l.strip())


def pmcid_for(ident):
    if ident.upper().startswith("PMC"):
        return ident.upper()
    d = esummary([ident]).get(str(ident), {})
    return ids_of(d)[1]


def fulltext(ident):
    pmc = pmcid_for(ident)
    if not pmc:
        return None, None
    def fetch():
        r = get(f"https://www.ebi.ac.uk/europepmc/webservices/rest/{pmc}/fullTextXML", kind="epmc")
        if r.status_code == 200 and "<body" in r.text:
            return xml_to_text(r.text)
        r = get(EUTILS + "efetch.fcgi", {"db": "pmc", "id": pmc[3:], "retmode": "xml"})
        if r.status_code == 200 and "<body" in r.text:
            return xml_to_text(r.text)
        return ""
    return pmc, cached(f"pmc-{pmc}.txt", fetch)


def cmd_fulltext(a):
    pmc, t = fulltext(a.id)
    if not pmc:
        print("no PMC id for this record (not in PMC)")
    elif not t:
        print(f"{pmc}: full text body not available (not open access); use abstract or url")
    else:
        print(f"{pmc}: {len(t)} chars cached. Use: rt.py grep {a.id} '<regex>'")


def url_text(url):
    h = hashlib.sha1(url.encode()).hexdigest()[:16]
    def fetch():
        r = get(url, kind="web", tries=2, headers={"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) research"}, allow_redirects=True)
        if r.status_code >= 400:
            print(f"HTTP {r.status_code} for {url}", file=sys.stderr)
            return None
        if "RecaptchaChallengePageUi" in r.text[:5000] or "captcha" in r.text[:3000].lower():
            print(f"captcha/bot wall for {url}", file=sys.stderr)
            return None
        ct = r.headers.get("content-type", "")
        if "pdf" in ct or r.content[:4] == b"%PDF":
            pdf = cpath(f"url-{h}.pdf")
            open(pdf, "wb").write(r.content)
            out = subprocess.run(["pdftotext", "-layout", pdf, "-"], capture_output=True, text=True).stdout
            return "\n".join(norm(l) for l in out.split("\n") if l.strip())
        x = r.text
        x = re.sub(r"(?is)<(script|style|noscript)[^>]*>.*?</\1>", " ", x)
        return xml_to_text(x)
    return cached(f"url-{h}.txt", fetch)


def cmd_url(a):
    t = url_text(a.url)
    print(f"{len(t)} chars cached" if t else "fetch failed")


def source_texts(ident):
    """All cached/fetchable text for a source id (pmid, PMCid or url)."""
    out = []
    if ident.startswith("http"):
        t = url_text(ident)
        if t:
            out.append(("url", t))
    else:
        if not ident.upper().startswith("PMC"):
            t = abstract_text(ident)
            if t:
                out.append(("abstract", t))
        pmc, t = fulltext(ident)
        if t:
            out.append((pmc, t))
    return out


def cmd_grep(a):
    rx = re.compile(a.regex, re.I)
    n = 0
    for name, t in source_texts(a.id):
        for m in rx.finditer(t):
            s, e = max(0, m.start() - a.C), min(len(t), m.end() + a.C)
            print(f"--- [{name} @{m.start()}] ...{t[s:e]}...")
            n += 1
            if n >= a.max:
                return
    if n == 0:
        print("no match")


def cmd_show(a):
    for name, t in source_texts(a.id):
        print(f"=== {name} ({len(t)} chars)\n{t[a.start:a.start + a.len]}")


# ---------- verification ----------

SETS = {"reference", "clinical", "optimal"}
KINDS = {"reference", "optimal", "warn", "diagnostic", "target", "critical"}
EVIDENCE = {"guideline", "consensus", "reference-interval-study", "outcome-study", "meta-analysis", "textbook", "regulatory"}
WHEN_KEYS = {"sex", "age_min", "age_max", "pregnant", "trimester", "postpartum", "menopause", "cycle_phase", "fasting",
             "ethnicity", "conditions", "risk", "markers", "specimen", "method"}
CATEGORY_RE = re.compile(r"[a-z][a-z0-9]*(-[a-z0-9]+)*")  # any kebab-case category; universe.toml owns the vocabulary
DERIVED_RE = re.compile(r"derived from (the )?[^.]*(cut-?offs?|thresholds?|bands?)|derived (band|bound|range)|(bound|band|range)s? (is|are) derived|\binferred\b|\bextrapolated\b|\binterpolated\b|computed from the|complement of", re.I)
NUM = re.compile(r"(?<![\w.])(\d{1,3}(?:,\d{3})+|\d*\.\d+|\d+)(?![\d])")


def numbers_in(text):
    vals = set()
    t = norm(text)
    for m in NUM.finditer(t):
        s = m.group(1)
        try:
            vals.add(float(s.replace(",", "")))
        except ValueError:
            pass
        for part in s.split(","):
            try:
                vals.add(float(part))
            except ValueError:
                pass
    # decimal comma (European): 5,7
    for m in re.finditer(r"(?<![\d,])(\d+),(\d{1,2})(?![\d,])", t):
        vals.add(float(f"{m.group(1)}.{m.group(2)}"))
    return vals


def has_num(v, vals):
    # numbers_in() collects magnitudes (a leading minus is read as a dash), so compare magnitudes
    v = abs(v)
    return any(abs(v - x) < 1e-9 * max(1, abs(v)) for x in vals)


def title_match(want, got):
    a, b = key(want).rstrip("."), key(got).rstrip(".")
    a, b = re.sub(r"[^a-z0-9]", "", a), re.sub(r"[^a-z0-9]", "", b)
    return a == b or (len(a) > 30 and (a in b or b in a))


def doi_ok(doi):
    f = cpath("doi-" + hashlib.sha1(doi.lower().encode()).hexdigest()[:16] + ".json")
    if os.path.exists(f):
        return json.load(open(f)).get("responseCode") == 1
    r = get("https://doi.org/api/handles/" + urllib.parse.quote(doi, safe="/:;()"), kind="doi")
    try:
        j = r.json()
    except ValueError:
        return False
    if j.get("responseCode") in (1, 100):
        json.dump(j, open(f, "w"))
    return j.get("responseCode") == 1


def check_citations(cits, tag, errs, stats):
    """Verify a citations list (PMID/title/DOI resolve, quote verbatim); return numbers found in quotes."""
    if not cits:
        errs.append(f"{tag}: no citations")
    quote_nums = set()
    for j, c in enumerate(cits):
        ct = f"{tag}.citations[{j}]"
        stats["citations"] += 1
        ok = True
        extra = set(c) - {"pmid", "doi", "url", "title", "locator", "quote"}
        if extra:
            errs.append(f"{ct}: unknown keys {sorted(extra)}"); ok = False
        pmid, doi, url = c.get("pmid"), c.get("doi"), c.get("url")
        if not (pmid or doi or url):
            errs.append(f"{ct}: needs pmid, doi or url"); ok = False
        q = c.get("quote")
        if not isinstance(q, str) or len(q.strip()) < 8:
            errs.append(f"{ct}: quote required"); ok = False
            q = ""
        if not c.get("locator"):
            errs.append(f"{ct}: locator required"); ok = False
        texts = []
        if pmid:
            pmid = str(pmid)
            if not pmid.isdigit():
                errs.append(f"{ct}: pmid must be digits"); ok = False
            else:
                d = esummary([pmid]).get(pmid)
                if not d:
                    errs.append(f"{ct}: PMID {pmid} does not resolve"); ok = False
                else:
                    if not c.get("title"):
                        errs.append(f"{ct}: title required with pmid"); ok = False
                    elif not title_match(c["title"], d.get("title", "")):
                        errs.append(f"{ct}: title mismatch for PMID {pmid}: esummary={norm(d.get('title',''))!r}"); ok = False
                    edoi, _ = ids_of(d)
                    if doi and edoi and edoi.lower() != str(doi).lower():
                        errs.append(f"{ct}: DOI {doi} != PubMed DOI {edoi} for PMID {pmid}"); ok = False
                    texts += [t for _, t in source_texts(pmid)]
        if doi:
            if not doi_ok(str(doi)):
                errs.append(f"{ct}: DOI {doi} does not resolve"); ok = False
        if url:
            if not str(url).startswith("http"):
                errs.append(f"{ct}: url must be http(s)"); ok = False
            elif not pmid or not texts:
                t = url_text(url)
                if t:
                    texts.append(t)
                elif not pmid:
                    errs.append(f"{ct}: url {url} could not be fetched"); ok = False
        if q and texts:
            kq = key(q).replace("...", "\x00")
            pieces = [p for p in kq.split("\x00") if p]
            joined = [key(t) for t in texts]
            if not all(any(p in jt for jt in joined) for p in pieces):
                errs.append(f"{ct}: quote not found verbatim in retrieved source text: {q[:90]!r}"); ok = False
        elif q and not texts:
            errs.append(f"{ct}: no retrievable source text to check quote against"); ok = False
        quote_nums |= numbers_in(q)
        stats["cit_ok" if ok else "cit_fail"] += 1
    return quote_nums


def verify_file(path, stats):
    errs, warns = [], []
    try:
        doc = tomllib.load(open(path, "rb"))
    except Exception as e:
        return [f"TOML parse error: {e}"], []
    slug = os.path.basename(path)[:-5]
    for k in ("slug", "name", "category", "unit", "description"):
        if not isinstance(doc.get(k), str) or not doc.get(k):
            errs.append(f"missing/invalid top-level {k}")
    if doc.get("slug") != slug:
        errs.append(f"slug {doc.get('slug')!r} != filename {slug!r}")
    if not re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)*", slug):
        errs.append("slug not lowercase-kebab")
    if not CATEGORY_RE.fullmatch(str(doc.get("category", ""))):
        errs.append(f"category {doc.get('category')!r} is not a kebab-case category")
    extra = set(doc) - {"slug", "name", "loinc", "category", "unit", "description", "aliases", "range", "no_reference"}
    if extra:
        errs.append(f"unknown top-level keys {sorted(extra)}")
    ranges = doc.get("range", [])
    if not ranges and not doc.get("no_reference"):
        errs.append("no [[range]] entries and no no_reference reason")
    for i, r in enumerate(ranges):
        tag = f"range[{i}]"
        extra = set(r) - {"set", "kind", "low", "high", "unit", "when", "population", "evidence", "citations", "note"}
        if extra:
            errs.append(f"{tag}: unknown keys {sorted(extra)}")
        if r.get("set") not in SETS:
            errs.append(f"{tag}: set {r.get('set')!r} not in {sorted(SETS)}")
        if r.get("kind") not in KINDS:
            errs.append(f"{tag}: kind {r.get('kind')!r} not in {sorted(KINDS)}")
        if r.get("evidence") not in EVIDENCE:
            errs.append(f"{tag}: evidence {r.get('evidence')!r} not in {sorted(EVIDENCE)}")
        lo, hi = r.get("low"), r.get("high")
        if lo is None and hi is None:
            errs.append(f"{tag}: needs low and/or high")
        for v in (lo, hi):
            if v is not None and not isinstance(v, (int, float)):
                errs.append(f"{tag}: low/high must be numbers")
        if isinstance(lo, (int, float)) and isinstance(hi, (int, float)) and lo > hi:
            errs.append(f"{tag}: low > high")
        if not isinstance(r.get("population"), str) or not r.get("population"):
            errs.append(f"{tag}: population required")
        when = r.get("when", {})
        if not isinstance(when, dict):
            errs.append(f"{tag}: when must be a table")
            when = {}
        bad = set(when) - WHEN_KEYS
        if bad:
            errs.append(f"{tag}: unknown when keys {sorted(bad)}")
        if "sex" in when and when["sex"] not in ("male", "female"):
            errs.append(f"{tag}: when.sex must be male/female (omit for any)")
        if "trimester" in when and when["trimester"] not in (1, 2, 3):
            errs.append(f"{tag}: trimester must be 1, 2 or 3")
        if "menopause" in when and when["menopause"] not in ("pre", "peri", "post"):
            errs.append(f"{tag}: menopause must be pre/peri/post")
        for b in ("pregnant", "fasting", "postpartum"):
            if b in when and not isinstance(when[b], bool):
                errs.append(f"{tag}: when.{b} must be boolean")
        if "conditions" in when and not (isinstance(when["conditions"], list) and all(isinstance(c, str) for c in when["conditions"])):
            errs.append(f"{tag}: when.conditions must be a list of strings")
        mk = when.get("markers", [])
        thresholds = []
        for m in mk if isinstance(mk, list) else [None]:
            if not (isinstance(m, dict) and isinstance(m.get("marker"), str) and m.get("op") in ("<", "<=", ">", ">=")
                    and isinstance(m.get("value"), (int, float))):
                errs.append(f"{tag}: when.markers entries need marker, op (<,<=,>,>=), value[, unit]")
            else:
                thresholds.append(m["value"])
        quote_nums = check_citations(r.get("citations", []), tag, errs, stats)
        for name, v in (("low", lo), ("high", hi)):
            sf = r.get("source_factor")
            src_ok = isinstance(v, (int, float)) and isinstance(sf, (int, float)) and sf and any(abs(float(v) / sf - x) <= 1e-3 * max(1.0, abs(x)) for x in quote_nums)
            if isinstance(v, (int, float)) and not has_num(float(v), quote_nums) and not src_ok:
                errs.append(f"{tag}: {name}={v} not present in any citation quote")
        if DERIVED_RE.search(str(r.get("note", "")) + " " + str(r.get("population", ""))):
            errs.append(f"{tag}: note says the bounds are derived/inferred; every bound must be stated verbatim in the cited source")
        for v in thresholds:
            if not has_num(float(v), quote_nums):
                errs.append(f"{tag}: when.markers threshold {v} not present in any citation quote")
        for k in ("age_min", "age_max"):
            if k in when and not has_num(float(when[k]), quote_nums):
                warns.append(f"{tag}: when.{k}={when[k]} not in quotes (check population text)")
    return errs, warns


PANEL_ROLES = {"core", "extended"}


def verify_panel(path, stats):
    """catalog/panels/<slug>.toml: a cited preset of catalog markers (see tools/research/FORMAT.md)."""
    errs, warns = [], []
    try:
        doc = tomllib.load(open(path, "rb"))
    except Exception as e:
        return [f"TOML parse error: {e}"], []
    slug = os.path.basename(path)[:-5]
    for k in ("slug", "name", "description"):
        if not isinstance(doc.get(k), str) or not doc.get(k):
            errs.append(f"missing/invalid top-level {k}")
    if doc.get("slug") != slug:
        errs.append(f"slug {doc.get('slug')!r} != filename {slug!r}")
    extra = set(doc) - {"slug", "name", "description", "aliases", "member"}
    if extra:
        errs.append(f"unknown top-level keys {sorted(extra)}")
    members = doc.get("member", [])
    if not members:
        errs.append("no [[member]] entries")
    mk_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(path))), "markers")
    seen = set()
    for i, m in enumerate(members):
        tag = f"member[{i}]"
        extra = set(m) - {"marker", "role", "reason", "term", "citations"}
        if extra:
            errs.append(f"{tag}: unknown keys {sorted(extra)}")
        mk = m.get("marker")
        if mk in seen:
            errs.append(f"{tag}: duplicate marker {mk!r}")
        seen.add(mk)
        mp = os.path.join(mk_dir, f"{mk}.toml")
        if not isinstance(mk, str) or not os.path.exists(mp):
            errs.append(f"{tag}: marker {mk!r} is not in catalog/markers")
            continue
        if m.get("role") not in PANEL_ROLES:
            errs.append(f"{tag}: role {m.get('role')!r} not in {sorted(PANEL_ROLES)}")
        if not isinstance(m.get("reason"), str) or not m.get("reason"):
            errs.append(f"{tag}: reason required")
        check_citations(m.get("citations", []), tag, errs, stats)
        md = tomllib.load(open(mp, "rb"))
        terms = [md.get("name", ""), mk, mk.replace("-", " ")] + list(md.get("aliases", []))
        if m.get("term"):
            terms.append(m["term"])
        quotes = " ".join(key(str(c.get("quote", ""))) for c in m.get("citations", []))
        if not any(key(x) and key(x) in quotes for x in terms):
            errs.append(f"{tag}: no citation quote names {mk!r} (name, alias, slug or term)")
    return errs, warns


def cmd_verify(a):
    stats = {"citations": 0, "cit_ok": 0, "cit_fail": 0}
    bad = 0
    for p in a.files:
        errs, warns = (verify_panel if os.sep + "panels" + os.sep in os.path.abspath(p) else verify_file)(p, stats)
        status = "FAIL" if errs else "OK"
        bad += bool(errs)
        print(f"{status} {p}")
        for e in errs:
            print(f"   ERROR {e}")
        if not a.quiet:
            for w in warns:
                print(f"   warn  {w}")
    print(f"\n{len(a.files)-bad}/{len(a.files)} files OK; citations checked {stats['citations']}, ok {stats['cit_ok']}, failed {stats['cit_fail']}")
    if a.json:
        json.dump(stats, open(a.json, "w"))
    sys.exit(1 if bad else 0)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sp = ap.add_subparsers(dest="cmd", required=True)
    p = sp.add_parser("search"); p.add_argument("query"); p.add_argument("-n", type=int, default=10)
    p.add_argument("--sort", default="relevance"); p.set_defaults(f=cmd_search)
    p = sp.add_parser("epmc"); p.add_argument("query"); p.add_argument("-n", type=int, default=10); p.set_defaults(f=cmd_epmc)
    p = sp.add_parser("abstract"); p.add_argument("pmid"); p.set_defaults(f=cmd_abstract)
    p = sp.add_parser("fulltext"); p.add_argument("id"); p.set_defaults(f=cmd_fulltext)
    p = sp.add_parser("url"); p.add_argument("url"); p.set_defaults(f=cmd_url)
    p = sp.add_parser("grep"); p.add_argument("id"); p.add_argument("regex"); p.add_argument("-C", type=int, default=250)
    p.add_argument("--max", type=int, default=12); p.set_defaults(f=cmd_grep)
    p = sp.add_parser("show"); p.add_argument("id"); p.add_argument("--start", "--from", type=int, default=0)
    p.add_argument("--len", type=int, default=3000); p.set_defaults(f=cmd_show)
    p = sp.add_parser("verify"); p.add_argument("files", nargs="+"); p.add_argument("-q", "--quiet", action="store_true")
    p.add_argument("--json"); p.set_defaults(f=cmd_verify)
    a = ap.parse_args()
    a.f(a)


if __name__ == "__main__":
    main()
