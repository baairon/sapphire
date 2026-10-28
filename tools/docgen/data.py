"""Load docs/data/ and crates/ and derive the facts used by every generated page."""

import csv
import json
import os
import re
from collections import Counter, OrderedDict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = "docs/data"

TIER_ORDER = ["P1", "P2v", "P2", "P2-std", "P3"]
VERDICT_RANK = {"mismatch": 0, "weak": 1, "align": 2}
CONF_RANK = {"low": 0, "med": 1, "high": 2}
CONF_LEVELS = ["confirmed", "consistent", "twin", "stub", "weak", "mismatch"]
# Meanings of the evidence classes of docs/data/PARITY.md ("How the strict rows were checked").
EVIDENCE_LABEL = OrderedDict([
    ("p1", "P1: oracle vectors, bit exact"),
    ("second", "P2v: PASS from a second check, separate from the row's own ledger entry"),
    ("owner", "P2v: the row's own ledger entry, exact branch counts over compared worlds"),
    ("audit2", "P2v: PASS from the independent audit of the last seven rows, with its own coverage recompute"),
    ("assumed", "P2v: PASS from the same audit, resting on labelled assumptions the maintainer accepted"),
    ("anyworld", "P2v: older any-world accounting, not yet audited"),
    ("-", "none: never matched (unproven)"),
])
STRICT_LABEL = OrderedDict([
    ("p1", "P1 verified (oracle vectors, bit exact)"),
    ("exact", "P2v verified under the exact check (seeded worlds)"),
    ("unproven", "unproven (never matched)"),
])
# Alternative key names (used by pages written before the key list existed) and their canonical keys.
ALIASES = OrderedDict([
    ("strict_fn", "strict_fns"), ("p1_fn", "p1_fns"), ("p2v_fn", "p2v_exact_fns"), ("p2v_pct", "p2v_exact_pct"),
    ("exact_fn", "strictest_fns"), ("exact_bytes", "strictest_bytes"), ("exact_pct", "strictest_pct"),
    ("second_fn", "ev_second_fns"), ("owner_fn", "ev_owner_fns"), ("anyworld_fns", "anyworld_fn"),
    ("unproven_fn", "unproven_fns"), ("names_fn", "names_total"), ("class_cntv2card_fn", "cntv2card_fns"),
    ("stub_names", "stubs_total"), ("classes_local", "classes_total"), ("effect_names", "effects_total"),
    ("param_candidates", "params_total"), ("arity_lifted", "arity_fit_base"),
    ("ctor_vtable", "ctor_dtor_own_vtable"), ("twin_fn", "twin_fns"), ("const_return_fn", "const_result_fns"),
])
# A `fn f_XXXXXXXX` definition at the start of a line, after optional visibility and qualifiers.
DEF_RE = re.compile(r"[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?(?:(?:const|async|unsafe|extern(?:[ \t]+\"[^\"]*\")?)[ \t]+)*"
                    r"fn[ \t]+(f_[0-9a-f]{8})\b")


def read_tsv(path):
    with open(path, encoding="utf-8", newline="") as h:
        return list(csv.DictReader(h, delimiter="\t", quoting=csv.QUOTE_NONE))


def read_text(path):
    with open(path, encoding="utf-8", newline="") as h:
        return h.read()


def fmt_int(n):
    return "{:,}".format(n)


def fmt_pct(part, whole):
    return "{:.1f}".format(100.0 * part / whole) if whole else "0.0"


def hexoff(n):
    return "0x{:x}".format(n)


def fid(n):
    return "f_{:08x}".format(n)


def _must(pattern, text, source):
    m = re.search(pattern, text, re.M)
    if not m:
        raise ValueError("pattern not found in {}: {}".format(source, pattern))
    return m


def module_header(text):
    """First paragraph of the leading `//!` block, joined to one line."""
    out = []
    for line in text.split("\n"):
        s = line.strip()
        if s.startswith("//!"):
            body = s[3:].strip()
            if not body:
                if out:
                    break
                continue
            out.append(body)
        elif s == "" and not out:
            continue
        elif s.startswith("#![") and not out:
            continue
        else:
            break
    return " ".join(out)


def first_sentence(text, limit=150):
    m = re.match(r"(.+?\.)(\s|$)", text)
    s = m.group(1) if m else text
    if len(s) > limit:
        s = s[: limit - 3].rstrip() + "..."
    return s


class Data:
    """All inputs, loaded once. `root` is the repository root."""

    def __init__(self, root=ROOT, confirmation_path=None):
        self.root = root
        self.d = os.path.join(root, DATA)
        self.confirmation_path = confirmation_path or os.path.join(self.d, "name_confirmation.tsv")
        self._load()
        self._scan_crates()
        self._merge()
        self._facts()

    def p(self, name):
        return os.path.join(self.d, name)

    def _load(self):
        self.fm = read_tsv(self.p("FUNCTION_MAP.tsv"))
        self.names = {r["symbol"]: r for r in read_tsv(self.p("names.tsv"))}
        self.checks = {r["symbol"]: r for r in read_tsv(self.p("namecheck_checks.tsv"))}
        self.nc_classes = {r["symbol"]: r for r in read_tsv(self.p("namecheck_classes.tsv"))}
        self.nc_rest = {r["symbol"]: r for r in read_tsv(self.p("namecheck_rest.tsv"))}
        self.stubs = read_tsv(self.p("stub_names.tsv"))
        self.stubcheck = {r["symbol"]: r for r in read_tsv(self.p("namecheck_stubs.tsv"))}
        self.stubhyp = {r["stub_offset"]: r for r in read_tsv(self.p("namecheck_stub_hypotheses.tsv"))}
        self.stub_arity = read_tsv(self.p("namecheck_stub_arity_defects.tsv"))
        self.classes = read_tsv(self.p("classes_names.tsv"))
        self.effects = read_tsv(self.p("effect_names.tsv"))
        self.params = read_tsv(self.p("param_candidates.tsv"))
        self.exact_audit = read_tsv(self.p("EXACT_AUDIT.tsv"))
        self.audit_final = read_tsv(self.p("audit_final.tsv"))
        self.audit_close = read_tsv(self.p("audit_close.tsv"))
        self.close_sets = read_tsv(self.p("close_sets.tsv"))
        self.pipeline = read_tsv(self.p("pipeline_parity.tsv"))
        self.risks = read_tsv(self.p("RISKS.tsv"))
        self.manifest = read_tsv(self.p("MANIFEST.tsv"))
        self.parity_md = read_text(self.p("PARITY.md"))
        self.summary_md = read_text(self.p("namecheck_summary.md"))
        self.missing_md = read_text(self.p("MISSING_DATA.md"))
        self.ledger_lines = read_text(self.p("ledger_digest.tsv")).split("\n")[1:]
        self.index = {}
        with open(self.p("index_functions.jsonl"), encoding="utf-8") as h:
            for line in h:
                if line.strip():
                    j = json.loads(line)
                    self.index[j["off"]] = j
        self.confirmation = {r["symbol"]: r for r in read_tsv(self.confirmation_path)}
        refs = self.p("archive_refs.tsv")
        self.archive_refs = read_tsv(refs) if os.path.exists(refs) else []

    def _scan_crates(self):
        """Text scan of crates/: module headers, #[test] lines, fn f_XXXXXXXX definitions."""
        self.crates = OrderedDict()
        self.defs = {}
        self.lift_defs = {}
        base = os.path.join(self.root, "crates")
        for name in sorted(os.listdir(base)):
            cdir = os.path.join(base, name)
            if not os.path.isfile(os.path.join(cdir, "Cargo.toml")):
                continue
            info = {"name": name, "role": "", "files": 0, "tests": 0, "test_files": 0, "modules": []}
            for dirpath, dirs, files in os.walk(cdir):
                dirs[:] = sorted(d for d in dirs if d != "target")
                for f in sorted(files):
                    if not f.endswith(".rs"):
                        continue
                    full = os.path.join(dirpath, f)
                    rel = os.path.relpath(full, self.root).replace(os.sep, "/")
                    text = read_text(full)
                    info["files"] += 1
                    n = len(re.findall(r"^[ \t]*#\[test\]", text, re.M))
                    info["tests"] += n
                    info["test_files"] += 1 if n else 0
                    head = module_header(text)
                    if head:
                        info["modules"].append((rel, head))
                    if rel.endswith(("/src/lib.rs", "/src/main.rs")) and not info["role"]:
                        info["role"] = "Lifted functions." if name.startswith("lift-") else head
                    if "fn f_" in text:
                        lift_src = name.startswith("lift-") and rel.startswith("crates/{}/src/".format(name))
                        for i, line in enumerate(text.split("\n"), 1):
                            m = DEF_RE.match(line)
                            if m:
                                self.defs.setdefault(m.group(1), []).append((rel, i))
                                if lift_src:
                                    self.lift_defs.setdefault(m.group(1), []).append((rel, i))
            self.crates[name] = info

    def source_line(self, sym, source):
        for rel, line in self.defs.get(sym, []):
            if rel == source:
                return line
        return None

    def _merge(self):
        """One name check verdict per function: worst verdict, then highest confidence."""
        self.nc = {}
        for sym in self.checks:
            rows = [t[sym] for t in (self.nc_classes, self.nc_rest) if sym in t]
            if not rows:
                continue
            worst = min(VERDICT_RANK[r["verdict"]] for r in rows)
            cand = [r for r in rows if VERDICT_RANK[r["verdict"]] == worst]
            best = max(cand, key=lambda r: CONF_RANK.get(r["confidence"], -1))
            flags = ";".join(r["flags"] for r in rows if r["flags"])
            self.nc[sym] = dict(best, flags=flags)
        self.stub_callers = Counter()
        for j in self.index.values():
            for s in set(j["stubs"]):
                self.stub_callers[s] += 1
        off_to_sym = {int(r["offset"], 16): r["name"] for r in self.fm}
        self.rows = []
        for r in self.fm:
            sym, off = r["name"], int(r["offset"], 16)
            n = self.names.get(sym, {})
            ck = self.checks.get(sym, {})
            nc = self.nc.get(sym)
            j = self.index.get(off, {})
            line = self.source_line(sym, r["source"])
            conf = self.confirmation.get(sym, {}).get("level", "missing")
            self.rows.append(OrderedDict([
                ("offset", hexoff(off)), ("id", sym), ("bank_name", n.get("exact_name", "")),
                ("rust_label", n.get("rust_label", "")), ("kind", n.get("kind", "")),
                ("origin", n.get("origin", "")), ("class", ck.get("class", "")),
                ("size", int(r["size"])), ("group", r["group"]), ("tier", r["tier"]),
                ("status", r["status"]), ("strict", r["strict"]), ("evidence", r["evidence"]),
                ("name_check", "{} {}".format(nc["verdict"], nc["confidence"]) if nc else "none"),
                ("name_check_reviewed", nc["reviewed"] if nc else ""),
                ("confirmation", conf),
                ("callers", [off_to_sym.get(c, hexoff(c)) for c in j.get("callers", [])]),
                ("callees", [off_to_sym.get(c, hexoff(c)) for c in j.get("calls", []) + j.get("tails", [])]),
                ("stubs", [hexoff(s) for s in j.get("stubs", [])]),
                ("source", r["source"]), ("line", line),
            ]))
        self.by_id = {r["id"]: r for r in self.rows}

    def _facts(self):
        f = OrderedDict()

        def put(key, value, source, text=None):
            if text is None:
                text = fmt_int(value) if isinstance(value, int) else str(value)
            f[key] = {"value": value, "text": text, "source": source}

        FM = "docs/data/FUNCTION_MAP.tsv"
        total_b = sum(int(r["size"]) for r in self.fm)
        n = len(self.fm)
        put("fn_total", n, FM + " rows")
        put("code_bytes", total_b, FM + " sum of size")

        def sel(pred):
            rs = [r for r in self.fm if pred(r)]
            return len(rs), sum(int(r["size"]) for r in rs)

        for key, pred, src in [
            ("strict", lambda r: r["strict"] in ("p1", "exact"), "strict in {p1, exact}"),
            ("p1", lambda r: r["tier"] == "P1", "tier P1"),
            ("p2v_exact", lambda r: r["strict"] == "exact", "strict exact"),
            ("strictest", lambda r: r["evidence"] in ("p1", "second", "owner", "audit2"),
             "evidence in {p1, second, owner, audit2}"),
            ("unproven", lambda r: r["strict"] == "unproven", "strict unproven"),
        ]:
            c, b = sel(pred)
            put(key + "_fns", c, FM + " " + src)
            put(key + "_bytes", b, FM + " " + src)
            put(key + "_pct", float(fmt_pct(b, total_b)), FM + " " + src + ", share of code bytes", fmt_pct(b, total_b))
        ev = Counter(r["evidence"] for r in self.fm)
        put("ev_second_fns", ev["second"], FM + " evidence second")
        put("ev_owner_fns", ev["owner"], FM + " evidence owner")
        put("ev_anyworld_fns", ev["anyworld"], FM + " evidence anyworld")
        put("ev_audit2_fns", ev["audit2"], FM + " evidence audit2")
        put("ev_assumed_fns", ev["assumed"], FM + " evidence assumed")
        put("partial_fns", sum(r["status"] == "partial" for r in self.fm), FM + " status partial")
        unp = [r["offset"] for r in self.fm if r["strict"] == "unproven"]
        put("unproven_offsets", unp, FM + " strict unproven", ", ".join("`{}`".format(o) for o in unp))
        m = _must(r"P1 vectors compared in total: (\d+) across (\d+) functions", self.parity_md, "PARITY.md")
        put("p1_vectors", int(m.group(1)), "docs/data/PARITY.md P1 vectors line")
        put("p1_vector_fns", int(m.group(2)), "docs/data/PARITY.md P1 vectors line")

        NM = "docs/data/names.tsv"
        put("names_total", len(self.names), NM + " rows")
        put("names_ids_agree", sum(1 for r in self.fm if r["name"] in self.names
                                   and self.names[r["name"]]["offset"] == r["offset"]), NM + " symbol and offset against " + FM)
        org = Counter(r["origin"] for r in self.names.values())
        for k, o in [("origin_bundled", "bundled_library"), ("origin_std", "std"),
                     ("origin_undetermined", "undetermined"), ("origin_component", "component")]:
            put(k, org[o], NM + " origin " + o)
        kinds = Counter(r["kind"] for r in self.names.values())
        for k in ["method", "std", "function", "destructor", "constructor", "initialiser"]:
            put("kind_" + k, kinds[k], NM + " kind " + k)
        put("cntv2card_fns", sum(r["class"] == "CNTV2Card" for r in self.checks.values()),
            "docs/data/namecheck_checks.tsv class CNTV2Card")

        NC = "docs/data/namecheck_classes.tsv and namecheck_rest.tsv, merged (worst verdict, then highest confidence)"
        vc = Counter((r["verdict"], r["confidence"]) for r in self.nc.values())
        put("nc_checked", len(self.nc), NC)
        put("nc_align", sum(v for (a, _), v in vc.items() if a == "align"), NC)
        put("nc_align_high", vc[("align", "high")], NC)
        put("nc_align_med", vc[("align", "med")], NC)
        put("nc_align_low", vc[("align", "low")], NC)
        put("nc_weak", sum(v for (a, _), v in vc.items() if a == "weak"), NC)
        put("nc_mismatch", sum(v for (a, _), v in vc.items() if a == "mismatch"), NC)
        mm = sorted(s for s, r in self.nc.items() if r["verdict"] == "mismatch")
        put("nc_mismatch_id", ", ".join(mm), NC)
        put("nc_unchecked", len(self.checks) - len(self.nc), NC + ", functions without a verdict")
        rv = Counter(r["reviewed"] for r in self.nc.values())
        put("nc_deep", rv["deep"], NC + ", reviewed deep in the row that gives the verdict")
        put("nc_sampled", rv["sample"], NC + ", reviewed sample in the row that gives the verdict")
        put("nc_script", rv["script"], NC + ", reviewed script in the row that gives the verdict")

        SM = "docs/data/namecheck_summary.md"
        m = _must(r"for ([\d.]+)% of ([\d,]+) lifted functions", self.summary_md, SM)
        put("arity_fit_pct", float(m.group(1)), SM, m.group(1))
        put("arity_fit_base", int(m.group(2).replace(",", "")), SM)
        m = _must(r"about (\d+) to (\d+)% if shifted", self.summary_md, SM)
        put("arity_shift_pct", int(m.group(2)), SM + " (stated as about " + m.group(1) + " to " + m.group(2) + "%)")
        m = _must(r"All (\d+) class hierarchies match", self.summary_md, SM)
        put("class_hier_match", int(m.group(1)), SM)
        m = _must(r"(\d+) of (\d+) constructors and destructors", self.summary_md, SM)
        put("ctor_dtor_own_vtable", int(m.group(1)), SM)
        put("classes_total", len(self.classes), "docs/data/classes_names.tsv rows")
        flags = Counter()
        for r in self.nc_rest.values():
            for fl in set(x for x in r["flags"].split(";") if x):
                flags[fl] += 1
        put("twin_fns", flags["twin_interchangeable"], "docs/data/namecheck_rest.tsv flag twin_interchangeable")
        put("const_result_fns", flags["CONSTANT_RESULT"], "docs/data/namecheck_rest.tsv flag CONSTANT_RESULT")

        ST = "docs/data/namecheck_stubs.tsv verdict"
        sv = Counter(r["verdict"] for r in self.stubcheck.values())
        put("stubs_total", len(self.stubs), "docs/data/stub_names.tsv rows")
        put("stub_align", sv["align"], ST)
        put("stub_weak", sv["weak"], ST)
        put("stub_mismatch", sv["mismatch"], ST)
        hy = Counter(r["outcome"] for r in self.stubhyp.values())
        HY = "docs/data/namecheck_stub_hypotheses.tsv outcome"
        put("stub_hyp_total", hy["confirmed"] + hy["refined"] + hy["refuted"], HY + " other than no_hypothesis")
        put("stub_hyp_confirmed", hy["confirmed"], HY)
        put("stub_hyp_refined", hy["refined"], HY)
        put("stub_hyp_refuted", hy["refuted"], HY)
        SA = "docs/data/namecheck_stub_arity_defects.tsv"
        put("stub_arity_sites", len(self.stub_arity), SA + " rows")
        put("stub_arity_stubs", len(set(r["stub_offset"] for r in self.stub_arity)), SA + " distinct stub_offset")
        put("effects_total", len(self.effects), "docs/data/effect_names.tsv rows")
        put("params_total", len(self.params), "docs/data/param_candidates.tsv rows")
        m = _must(r"^(\d+) references recorded", self.missing_md, "MISSING_DATA.md")
        put("missing_refs", int(m.group(1)), "docs/data/MISSING_DATA.md")
        put("exact_audit_rows", len(self.exact_audit), "docs/data/EXACT_AUDIT.tsv rows")
        av = Counter(r["verdict"] for r in self.audit_final)
        put("audit_pass", av["PASS"], "docs/data/audit_final.tsv verdict PASS")
        put("audit_open", av["OPEN"], "docs/data/audit_final.tsv verdict OPEN")
        put("pipeline_parity_rows", len(self.pipeline), "docs/data/pipeline_parity.tsv rows")
        put("pipeline_parity_equal", sum(r["equal"] == "true" for r in self.pipeline), "docs/data/pipeline_parity.tsv equal true")
        put("risks_total", len(self.risks), "docs/data/RISKS.tsv rows")
        CS = "text scan of crates/ (#[test] lines), not a test run"
        put("crates_total", len(self.crates), "crates/ folders with a Cargo.toml")
        put("lift_crates", sum(1 for c in self.crates if c.startswith("lift-")), "crates/lift-* folders")
        put("tests_scan_total", sum(c["tests"] for c in self.crates.values()), CS)
        for c, info in self.crates.items():
            put("tests_scan_" + c.replace("-", "_"), info["tests"], CS)
        LD = "text scan of crates/lift-*/src/ for fn f_XXXXXXXX definitions"
        put("lift_defs_total", sum(len(v) for v in self.lift_defs.values()), LD)
        put("lift_ids_multi", sum(1 for v in self.lift_defs.values() if len(v) > 1),
            LD + ", ids with more than one definition")
        for key, pred, src in [
            ("anyworld", lambda r: r["evidence"] == "anyworld", "evidence anyworld"),
            ("unproven_p2v", lambda r: r["strict"] == "unproven" and r["tier"] == "P2v", "strict unproven, tier P2v"),
            ("unproven_p2", lambda r: r["strict"] == "unproven" and r["tier"] == "P2", "strict unproven, tier P2"),
            ("group_a", lambda r: r["group"] == "A", "group A"),
            ("group_b", lambda r: r["group"] == "B", "group B"),
        ]:
            c, b = sel(pred)
            put(key + "_fn", c, FM + " " + src)
            put(key + "_bytes", b, FM + " " + src)
        m = _must(r"(\d+) functions were self-contained candidates; (\d+) are verified P1; (\d+) were downgraded",
                  self.parity_md, "PARITY.md")
        put("p1_candidates", int(m.group(1)), "docs/data/PARITY.md P1 candidates line")
        put("p1_downgraded", int(m.group(3)), "docs/data/PARITY.md P1 candidates line")
        AF = "docs/data/audit_final.tsv detail"
        put("audit_rows", len(self.audit_final), "docs/data/audit_final.tsv rows")

        def count_detail(pattern):
            n = 0
            for r in self.audit_final:
                m = re.search(pattern, r["detail"])
                n += 1 if m and int(m.group(1)) > 0 else 0
            return n

        put("audit_proof_rows", count_detail(r"(\d+) explained by proof"), AF + ", 'explained by proof' above 0")
        put("audit_landing_rows", count_detail(r"(\d+) only in blocks unreachable from the entry"),
            AF + ", 'only in blocks unreachable from the entry' above 0")
        put("audit_rule_w_rows", count_detail(r"(\d+) only under rule W"), AF + ", 'only under rule W' above 0")
        put("audit_stack_rows", count_detail(r"(\d+) only under the stack protector assumption"),
            AF + ", 'only under the stack protector assumption' above 0")
        put("audit_rule_assumption_rows", sum(
            1 for r in self.audit_final
            if any(int(m) > 0 for m in re.findall(r"(\d+) only under (?:rule W|the stack protector assumption)",
                                                    r["detail"]))),
            AF + ", 'only under rule W' or 'only under the stack protector assumption' above 0")
        put("audit_hand_proof_rows", count_detail(r"hand proofs (\d+)"), AF + ", 'hand proofs' above 0")
        put("audit_callee_writes_rows", sum("callee writes" in r["detail"] for r in self.audit_final),
            AF + ", rows mentioning 'callee writes'")
        put("assume_write_nothing_rows", sum("write nothing through pointers" in l for l in self.ledger_lines),
            "docs/data/ledger_digest.tsv note, 'write nothing through pointers'")
        AC = "docs/data/audit_close.tsv"
        ac = self.audit_close
        size = {r["id"]: r["size"] for r in self.rows}
        labelled = [r for r in ac if r["labels"] != "-"]
        lbytes = sum(size[r["symbol"]] for r in labelled)
        put("close_rows", len(ac), AC + " rows")
        put("close_pass_rows", sum(r["verdict"] == "PASS" for r in ac), AC + " verdict PASS")
        put("close_labelled_rows", len(labelled), AC + " rows with a label other than -")
        put("close_labelled_bytes", lbytes, AC + " rows with a label other than -, sum of size")
        put("close_labelled_pct", float(fmt_pct(lbytes, total_b)), AC + " rows with a label other than -, share of code bytes",
            fmt_pct(lbytes, total_b))
        put("close_compared", sum(int(r["compared"]) for r in ac), AC + " sum of compared")
        put("close_outcomes", sum(int(r["outcomes"]) for r in ac), AC + " sum of outcomes")
        put("close_seen", sum(int(r["seen"]) for r in ac), AC + " sum of seen")
        put("close_explained", sum(int(r["explained"]) for r in ac), AC + " sum of explained")
        put("close_landing_outcomes", sum(int(r["landing_pads"]) for r in ac), AC + " sum of landing_pads")
        put("close_landing_rows", sum(int(r["landing_pads"]) > 0 for r in ac), AC + " rows with landing_pads above 0")
        for label in ("F", "PN", "P2", "A1"):
            rs = [r for r in ac if label in r["labels"].split(",")]
            counts = [dict(kv.split("=") for kv in r["label_outcomes"].split(";") if "=" in kv) for r in rs]
            put("close_rows_" + label, len(rs), AC + " rows whose labels include " + label)
            put("close_out_" + label, sum(int(c.get(label, 0)) for c in counts), AC + " label_outcomes " + label)
        CS = "docs/data/close_sets.tsv"
        put("close_sets_rows", len(self.close_sets), CS + " rows")
        put("close_sets_pins", sum(int(r["pins"]) for r in self.close_sets), CS + " sum of pins")
        put("stub_arity_fns", len(set(r["function"] for r in self.stub_arity)), SA + " distinct function")
        CF = "docs/data/name_confirmation.tsv level"
        lv = Counter(r["level"] for r in self.confirmation.values())
        for level in CONF_LEVELS:
            put("conf_" + level, lv[level], CF)
        KO = CF + " consistent rows whose only evidence id is know (library knowledge, no repository evidence)"
        put("conf_consistent_know_only", sum(1 for r in self.confirmation.values() if r["level"] == "consistent"
                                             and set(r["evidence_ids"].split(",")) == {"know"}), KO)
        # RegisterExpert::Decode* destructors that share one code fingerprint but carry no byte twin flag
        fpt = [r for r in read_tsv(self.p("namecheck_fingerprints.tsv"))
               if r["exact_name"].startswith("RegisterExpert::Decode") and "~" in r["exact_name"]]
        shape_cols = ["size", "n_insn", "n_blocks", "n_load", "n_store", "n_cond_jumps", "n_stub_calls", "n_code_calls",
                      "n_indirect_calls", "this_offsets", "stub_targets", "callees", "tail"]
        FPS = ("docs/data/namecheck_fingerprints.tsv RegisterExpert::Decode* destructors, consistent or confirmed in "
               "docs/data/name_confirmation.tsv, that share their most common code fingerprint")
        # the two port styles write an absent count as "" or "0"
        shapes = Counter(tuple((r[c] or "0") if c.startswith("n_") else r[c] for c in shape_cols) for r in fpt
                         if self.confirmation.get(r["symbol"], {}).get("level") in ("consistent", "confirmed"))
        put("conf_decode_dtor_shared", shapes.most_common(1)[0][1] if shapes else 0, FPS)
        self.facts = f
        self.aliases = OrderedDict((a, k) for a, k in ALIASES.items() if k in f)
        for c in self.crates:
            self.aliases["tests_" + c.replace("-", "_")] = "tests_scan_" + c.replace("-", "_")

    def fact(self, key):
        """Fact by key or alias, or None."""
        return self.facts.get(self.aliases.get(key, key))
