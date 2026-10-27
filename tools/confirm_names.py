"""Confirm the name bank's function names against the evidence in docs/data/ (FLEET section 7 levels).

Usage:
  python -I tools/confirm_names.py            write docs/data/name_confirmation.tsv and print the level counts
  python -I tools/confirm_names.py --check    compare a fresh run with the table; exit 1 on any difference

Inputs (all under docs/data/): names.tsv, stub_names.tsv, classes_names.tsv, FUNCTION_MAP.tsv,
index_functions.jsonl (callers), namecheck_fingerprints.tsv (code shape: argument registers, loads, stores,
vtable stores, `this` offsets, resolved callees and stub targets), namecheck_checks.tsv (signature against
argument registers), namecheck_classes.tsv and namecheck_rest.tsv (recorded observations and flags; their
verdicts are not used as conclusions). The bank is never changed: the name column copies names.tsv.

Evidence ids (repository evidence unless marked knowledge):
  arity     argument registers read equal the signature, two or more registers (`this` counts)
  vtable    a constructor or destructor stores its own class vtable
  base      a constructor or destructor calls the matching base class constructor or destructor
  delete    a destructor calls its own destructor and then the operator delete stub (deleting destructor)
  regs      a getter or setter verb whose body reads or writes registers that way (recorded observation)
  pair      a Get or Set name sharing its register with the partner of the opposite verb (recorded observation)
  fields    a `this` offset above 0x8 shared with another function of the same class and mostly used by it
  calls     a caller or callee of the same class, or sharing a rare type or identifier with the name
  stubs     an import stub target sharing a rare identifier with the name
  shape     the body shape fits the name: recorded destructor, constructor, getter, setter, predicate, string
            builder or static initialiser shape, or the port's own description (Getter, Setter, Returns)
  deep      a recorded deep read that found the name aligned (high or medium confidence)
  know      knowledge of libc++ (allocation and release helpers), not repository evidence; counts toward
            `consistent` only
  consts    constants shared with a named partner: index tables, sibling registers, string or device-id
            constants (written by the names agents' overrides below)
Against: an argument register count unlike the signature (more or fewer), a kind that contradicts the body, a severity
of 2 or more in namecheck_checks.tsv, or a mismatch verdict.

Levels: mismatch (recorded mismatch), stub (constant return, empty virtual default, recorded empty destructor
body, or a body with no loads, stores, calls or branches in 16 bytes or less: the code cannot decide),
twin (byte twin of a differently named function: the code cannot decide), confirmed (two or more repository
pieces, nothing against), consistent (one piece, nothing against), weak (no positive piece, or something
against).

Overrides: docs/data/name_confirmation_a.tsv and name_confirmation_b.tsv hold the names agents' checked-in
rows (same columns, changed rows only). A row replaces the computed one when the name equals names.tsv, the
text is ASCII, every evidence id is known, every cited crates/...:line exists, the computed row is not twin,
stub or mismatch, the new level is not twin or mismatch, a stub comes only from a weak row whose text says
constant return or empty body, and confirmed rests on two repository kinds (know never counts; arity and
consts do not count when the text leans on the SDK; the enum parameter note discounts arity only). A confirmed row short of that becomes consistent; any
other failure keeps the computed row. Every outcome is counted in the printed summary.

Audit: docs/data/name_confirmation_audit.tsv (symbol, name, from, to, reason) holds the independent names audit's
level changes, applied after the a and b overrides. A row applies when the name equals names.tsv, the text is
ASCII, from equals the level after the overrides, from and to are confirmed, consistent or weak and differ, every
cited crates/...:line exists, and a raise starts with a repository evidence id ("calls: ...") and cites a line (a
raise to confirmed needs two repository kinds). Any failure keeps the row. An applied row keeps its evidence ids
(a raise adds its id) and reads "audit: REASON; earlier: OLD TEXT".
Standard library only.
"""

import argparse
import json
import os
import re
import sys
from collections import Counter, defaultdict

TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, TOOLS)

from docgen.data import ROOT, read_tsv  # noqa: E402

LEVELS = ["confirmed", "consistent", "twin", "stub", "weak", "mismatch"]
HEADER = ["symbol", "name", "level", "evidence_ids", "evidence"]
OUT = os.path.join("docs", "data", "name_confirmation.tsv")
OVERRIDES = ["docs/data/name_confirmation_a.tsv", "docs/data/name_confirmation_b.tsv"]
AUDIT = "docs/data/name_confirmation_audit.tsv"
AUDIT_HEADER = ["symbol", "name", "from", "to", "reason"]
AUDIT_LEVELS = ("confirmed", "consistent", "weak")
AUDIT_RAISE = re.compile(r"^([a-z]+): ")
EVIDENCE_IDS = ("arity", "vtable", "base", "delete", "regs", "pair", "fields", "calls", "stubs", "shape", "deep",
                "consts", "know")
CITE = re.compile(r"(crates/[A-Za-z0-9_./-]+):(\d+)")
STUB_TEXT = re.compile(r"(?i)constant return|empty body")
SDK_TEXT = re.compile(r"\bSDK\b")
ENUM_NOTE = re.compile(r"\(enum parameters counted as int registers, SDK knowledge\)")
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
GET_VERBS = ("Get", "Is", "Has", "Read", "Can", "Query")
SET_VERBS = ("Set", "Write", "Enable", "Disable", "Clear", "Reset")
PREDICATE = re.compile(r"^(?:Is|Has|Can)[A-Z0-9]|CanDo")
COMMON = {"std", "__1", "const", "unsigned", "char", "bool", "void", "long", "short", "double", "float", "int",
          "allocator", "basic_string", "char_traits", "operator", "true", "false"}


def base_name(name):
    head = name.split("(")[0]
    return head.split("::")[-1].strip()


def class_of(name):
    head = name.split("(")[0]
    return head.rsplit("::", 1)[0] if "::" in head else ""


def verb(name):
    b = base_name(name)
    m = re.match(r"~|[A-Z][a-z]+|[a-z]+", b)
    return m.group(0) if m else ""


def short(s, n=60):
    return s if len(s) <= n else s[: n - 3] + "..."


def idents(name):
    return {t for t in IDENT.findall(name) if len(t) >= 5 and t not in COMMON}


class Confirm:
    def __init__(self, root=ROOT):
        d = os.path.join(root, "docs", "data")
        self.root = root
        self._lines = {}
        self.report = []
        t = lambda f: read_tsv(os.path.join(d, f))
        self.fm = t("FUNCTION_MAP.tsv")
        self.names = {r["symbol"]: r for r in t("names.tsv")}
        self.fp = {r["symbol"]: r for r in t("namecheck_fingerprints.tsv")}
        self.ck = {r["symbol"]: r for r in t("namecheck_checks.tsv")}
        self.ncc = {r["symbol"]: r for r in t("namecheck_classes.tsv")}
        self.ncr = {r["symbol"]: r for r in t("namecheck_rest.tsv")}
        self.classes = {r["class_name"]: r for r in t("classes_names.tsv")}
        self.stub_names = [r["exact_target"] for r in t("stub_names.tsv")]
        self.callers = {}
        off2sym = {int(r["offset"], 16): r["name"] for r in self.fm}
        with open(os.path.join(d, "index_functions.jsonl"), encoding="utf-8") as h:
            for line in h:
                if line.strip():
                    j = json.loads(line)
                    self.callers[j["name"]] = [off2sym[c] for c in j["callers"] if c in off2sym]
        self._prepare()

    def name(self, sym):
        return self.names[sym]["exact_name"]

    def _prepare(self):
        df = Counter()
        for n in [r["exact_name"] for r in self.names.values()] + self.stub_names:
            df.update(idents(n))
        limit = max(3, int(0.015 * (len(self.names) + len(self.stub_names))))
        self.rare = {k for k, v in df.items() if v <= limit}
        self.by_name = defaultdict(list)
        for s, r in self.names.items():
            self.by_name[r["exact_name"]].append(s)
        # `this` offsets per class
        self.off_users = defaultdict(Counter)
        for s, f in self.fp.items():
            c = self.ck.get(s, {}).get("class", "")
            for o in f["this_offsets"].split():
                self.off_users[o][c] += 1
        # fingerprint key for the byte twin cross-check
        self.key = {}
        groups = defaultdict(list)
        for s, f in self.fp.items():
            k = tuple(f[c] for c in ("size", "n_insn", "n_blocks", "n_load", "n_store", "n_cond_jumps",
                                     "n_stub_calls", "n_code_calls", "n_indirect_calls", "this_offsets",
                                     "stub_targets"))
            self.key[s] = k
            groups[k].append(s)
        self.groups = groups

    def nc_rows(self, sym):
        return [t[sym] for t in (self.ncc, self.ncr) if sym in t]

    def evidence(self, sym):
        name = self.name(sym)
        kind = self.names[sym]["kind"]
        f, ck = self.fp[sym], self.ck[sym]
        cls = ck.get("class", "") or class_of(name)
        v = verb(name)
        pos, know, against, notes = [], [], [], []
        nc = self.nc_rows(sym)
        nc_ev = " ".join(r["evidence"] for r in nc)
        flags = ";".join(r["flags"] for r in nc)

        # arity
        try:
            di, ds, du, ai = int(ck["decl_int"]), int(ck["decl_sse"]), int(ck["decl_unk"]), int(ck["arg_int"])
        except ValueError:
            di = ds = du = ai = -1
        if ck["sig_ok"] == "1" and du == 0 and ck["arity_delta"] == "0" and (ai + int(ck["arg_xmm"] or 0) >= 2):
            pos.append("arity")
            notes.append("reads {} int and {} sse args as the signature".format(ai, ck["arg_xmm"]))
        if ck["arity_mismatch"] not in ("", "0"):
            try:
                delta = int(ck["arity_delta"])
            except ValueError:
                delta = 0
            if delta == 0 and min(di, ai) >= 0:
                delta = ai + int(ck["arg_xmm"] or 0) - (di + ds + du)
            against.append("fewer argument registers read than the signature declares" if delta < 0 else
                           "more argument registers read than the signature allows")
        if ck["kind_mismatch"] not in ("", "0"):
            against.append("kind contradicts the body")
        if ck["severity"] not in ("", "0", "1"):
            against.append("check severity " + ck["severity"])

        # constructor and destructor shape
        base = self.classes.get(cls, {}).get("base_class", "")
        callees = [c.strip() for c in f["callees"].split("|") if c.strip()]
        stubs = [c.strip() for c in f["stub_targets"].split("|") if c.strip()]
        if kind in ("constructor", "destructor"):
            if f["vtable_store"] == "1" and f["vtable_refs"].split("|")[0].strip() == cls:
                pos.append("vtable")
                notes.append("stores the {} vtable".format(short(cls, 30)))
            want = ("{0}::{0}(" if kind == "constructor" else "{0}::~{0}(").format(base) if base else None
            if want and any(c.startswith(want) for c in callees):
                pos.append("base")
                notes.append("calls the base " + kind)
            own = "{0}::~{1}(".format(cls, cls.split("::")[-1]) if cls else None
            if kind == "destructor" and own and any(c.startswith(own) for c in callees) and \
                    any("operator delete" in s for s in stubs):
                pos.append("delete")
                notes.append("deleting destructor: own destructor then operator delete")

        # accessors (recorded observations of the name check scripts)
        if v in SET_VERBS and "Set-like: calls register write" in nc_ev or \
                v in GET_VERBS and "Get-like: calls register read" in nc_ev:
            pos.append("regs")
            notes.append("{} name with matching register access".format(v))
        b = base_name(name)
        for cond, pattern, what in [
            (kind == "destructor", r"destructor releases or chains", "destructor body releases or chains"),
            (kind == "constructor", r"constructor initialises fields or chains", "constructor body initialises"),
            (v in GET_VERBS, r"short getter shape", "getter shape"),
            (v in SET_VERBS, r"setter shape", "setter shape"),
            (bool(PREDICATE.search(b)), r"branch table predicate|Classifier", "predicate shape"),
            (bool(re.search(r"String|ostream|operator<<", name)), r"builds string via string stubs", "builds a string"),
            (kind == "initialiser", r"runs static constructors", "static initialiser shape"),
        ]:
            if cond and (re.search(pattern, nc_ev) or re.match(pattern, f["doc"])):
                pos.append("shape")
                notes.append(what + " (recorded)")
                break
        if v in ("Get", "Set", "Is") and "shared with partner" in nc_ev:
            pos.append("pair")
            m = re.search(r"shared with partner (\S+)", nc_ev)
            notes.append("register shared with " + short(m.group(1), 40) if m else "partner register")

        # fields
        if cls:
            for o in f["this_offsets"].split():
                if int(o, 16) > 8:
                    users = self.off_users[o]
                    if users[cls] >= 2 and users[cls] * 2 > sum(users.values()):
                        pos.append("fields")
                        notes.append("field +{} shared within {}".format(o, short(cls, 30)))
                        break

        # calls (callers and callees, resolved through the name tables)
        mine = idents(name) & self.rare
        partners = callees + [self.name(c) for c in self.callers.get(sym, [])]
        hit = None
        for p in partners:
            if p == name:
                continue
            if cls and class_of(p) == cls:
                hit = "same class as " + short(p, 50)
                break
            shared = mine & idents(p)
            if shared:
                hit = "shares {} with {}".format(sorted(shared)[0], short(p, 40))
                break
        if hit:
            pos.append("calls")
            notes.append(hit)
        shared = [(s, mine & idents(s)) for s in stubs]
        shared = [(s, x) for s, x in shared if x]
        if shared:
            pos.append("stubs")
            notes.append("stub {} shares {}".format(short(shared[0][0], 40), sorted(shared[0][1])[0]))

        # shape from the port's description
        doc = f["doc"]
        if doc and ((v in ("Get", "Is", "Has") and re.match(r"(Getter|Returns|Reads)", doc)) or
                    (v == "Set" and re.match(r"(Setter|Writes|Stores)", doc))):
            pos.append("shape")
            notes.append("port: " + short(doc, 40))

        # deep reads recorded by the name check
        if any(r["reviewed"] == "deep" and r["verdict"] == "align" and r["confidence"] in ("high", "med") for r in nc):
            pos.append("deep")
            notes.append("deep read aligned")
        if any(r["verdict"] == "mismatch" for r in nc):
            against.append("recorded mismatch: " + short(nc_ev, 70))

        # knowledge of libc++ (not repository evidence)
        if self.names[sym]["origin"] == "std":
            b = base_name(name)
            if re.search(r"allocate|__append|push_back|reserve|__construct|resize|insert|emplace", b) and \
                    any("operator new" in s for s in stubs + callees):
                know.append("know")
            elif re.search(r"~|deallocate|destroy|clear|__release", b) and \
                    any("operator delete" in s for s in stubs + callees):
                know.append("know")
            if know:
                notes.append("libc++ knowledge: allocation or release helper as expected")
        return pos, know, against, notes, flags

    def body_is_constant(self, sym):
        f = self.fp[sym]
        try:
            busy = sum(int(f[c]) for c in ("n_load", "n_store", "n_code_calls", "n_stub_calls", "n_indirect_calls",
                                          "n_cond_jumps"))
        except ValueError:
            return False
        return busy == 0 and int(f["size"]) <= 16

    def twin(self, sym, flags):
        if "twin_interchangeable" not in flags:
            return None
        name = self.name(sym)
        others = [s for s in self.groups[self.key[sym]] if s != sym and self.name(s) != name]
        return others

    def level(self, sym):
        pos, know, against, notes, flags = self.evidence(sym)
        pos = list(dict.fromkeys(pos))
        if any(r["verdict"] == "mismatch" for r in self.nc_rows(sym)):
            return "mismatch", pos + know, ["mismatch: " + "; ".join(against)] + notes
        empty = any(re.search(r"(?i)empty destructor body", r["evidence"]) for r in self.nc_rows(sym))
        if "CONSTANT_RESULT" in flags or "EMPTY_VIRTUAL_DEFAULT" in flags or empty or self.body_is_constant(sym):
            return "stub", pos + know, ["constant return or empty body, undecidable by code"] + notes
        tw = self.twin(sym, flags)
        if tw is not None:
            extra = "fingerprint equal to {}".format(tw[0]) if tw else "no fingerprint equal partner in the tables"
            return "twin", pos + know, ["byte twin of a differently named function, undecidable by code; " + extra] + notes
        if against:
            return "weak", pos + know, ["against: " + "; ".join(against)] + notes
        if len(pos) >= 2:
            return "confirmed", pos + know, notes
        if pos or know:
            return "consistent", pos + know, notes
        return "weak", [], ["no positive evidence in the tables"]

    def rows(self):
        out = []
        for r in self.fm:
            sym = r["name"]
            level, ids, notes = self.level(sym)
            text = "; ".join(notes) if notes else ""
            text = re.sub(r"[\t\n]", " ", text)
            out.append([sym, self.name(sym), level, ",".join(ids) or "-", short(text, 240)])
        index = {r[0]: i for i, r in enumerate(out)}
        self.report = []
        seen = set()
        for rel, row in self.overrides():
            sym = row[0]
            if sym in seen or sym not in index:
                self.report.append((rel, sym, "rejected", "symbol repeated" if sym in seen else "unknown symbol"))
                continue
            seen.add(sym)
            new, reason = self.check_override(row, out[index[sym]])
            if new is None:
                self.report.append((rel, sym, "rejected", reason))
            else:
                out[index[sym]] = new
                self.report.append((rel, sym, "downgraded" if reason else "accepted", reason or ""))
        self.audit_report = []
        seen = set()
        for row in self.audit_rows():
            sym = row[0]
            if sym in seen or sym not in index:
                self.audit_report.append((sym, "rejected", "symbol repeated" if sym in seen else "unknown symbol"))
                continue
            seen.add(sym)
            new, reason = self.check_audit(row, out[index[sym]])
            if new is None:
                self.audit_report.append((sym, "rejected", reason))
            else:
                out[index[sym]] = new
                self.audit_report.append((sym, "applied", ""))
        return out

    def overrides(self):
        """The names agents' checked-in rows as (file, [symbol, name, level, evidence_ids, evidence])."""
        out = []
        for rel in OVERRIDES:
            path = os.path.join(self.root, *rel.split("/"))
            if os.path.exists(path):
                out.extend((rel, [r[c] for c in HEADER]) for r in read_tsv(path))
        return out

    def audit_rows(self):
        """The independent names audit's level changes as [symbol, name, from, to, reason]."""
        path = os.path.join(self.root, *AUDIT.split("/"))
        if not os.path.exists(path):
            return []
        return [[r[c] for c in AUDIT_HEADER] for r in read_tsv(path)]

    def check_audit(self, row, computed):
        """(new row, None) applied or (None, reason) rejected; computed is the row after the a and b overrides."""
        sym, name, old, new, reason = row
        if sym not in self.names or name != self.name(sym):
            return None, "name differs from names.tsv"
        if any(ord(ch) > 127 for ch in "".join(row)):
            return None, "non-ASCII text"
        if old not in AUDIT_LEVELS or new not in AUDIT_LEVELS or old == new:
            return None, "from and to not two different levels of confirmed, consistent and weak"
        if computed[2] != old:
            return None, "from differs from the current level " + computed[2]
        if not reason.strip():
            return None, "no reason"
        cites = list(CITE.finditer(reason))
        for m in cites:
            if not self.cite_ok(m.group(1), int(m.group(2))):
                return None, "cited line missing"
        ids = [i for i in computed[3].split(",") if i not in ("", "-")]
        if LEVELS.index(new) < LEVELS.index(old):
            m = AUDIT_RAISE.match(reason)
            if not m or m.group(1) not in EVIDENCE_IDS or m.group(1) == "know" or not cites:
                return None, "a raise starts with a repository evidence id and cites a line"
            ids = list(dict.fromkeys(ids + [m.group(1)]))
            if new == "confirmed" and len(self.repo_kinds(ids, reason)) < 2:
                return None, "confirmed on fewer than two repository kinds"
        text = "audit: {}; earlier: {}".format(reason, computed[4]) if computed[4] else "audit: " + reason
        return [sym, name, new, ",".join(ids) or "-", text], None

    def cite_ok(self, path, line):
        if path not in self._lines:
            full = os.path.join(self.root, *path.split("/"))
            n = -1
            if os.path.isfile(full):
                with open(full, "rb") as h:
                    n = h.read().count(b"\n") + 1
            self._lines[path] = n
        return 1 <= line <= self._lines[path]

    @staticmethod
    def repo_kinds(ids, text):
        kinds = set(ids) - {"know"}
        if SDK_TEXT.search(text):
            kinds.discard("arity")
        if SDK_TEXT.search(ENUM_NOTE.sub("", text)):
            kinds.discard("consts")
        return kinds

    def check_override(self, row, computed):
        """(row, None) accepted, (row, reason) downgraded, (None, reason) rejected (the computed row stays)."""
        sym, name, level, ids_text, text = row
        if sym not in self.names or name != self.name(sym):
            return None, "name differs from names.tsv"
        if any(ord(ch) > 127 for ch in "".join(row)):
            return None, "non-ASCII text"
        ids = [i for i in ids_text.split(",") if i not in ("", "-")]
        if any(i not in EVIDENCE_IDS for i in ids):
            return None, "unknown evidence id"
        if level not in LEVELS or level in ("twin", "mismatch"):
            return None, "level not open to overrides"
        if computed[2] in ("twin", "stub", "mismatch"):
            return None, "computed row is " + computed[2]
        if level == "stub" and not (computed[2] == "weak" and STUB_TEXT.search(text)):
            return None, "stub without a weak row and a constant return or empty body"
        for m in CITE.finditer(text):
            if not self.cite_ok(m.group(1), int(m.group(2))):
                return None, "cited line missing"
        if level in ("confirmed", "consistent") and not ids:
            return None, "no evidence id"
        if level == "confirmed" and len(self.repo_kinds(ids, text)) < 2:
            return [sym, name, "consistent", ids_text,
                    "downgraded from confirmed: fewer than two repository kinds; " + text], \
                "confirmed on fewer than two repository kinds"
        return list(row), None

    def shared_fingerprint_destructors(self, rows):
        """Destructors left consistent or confirmed whose code fingerprint equals a differently named one."""
        level = {r[0]: r[2] for r in rows}
        groups = defaultdict(set)
        for s, k in self.key.items():
            if "~" in self.name(s) and level.get(s) in ("consistent", "confirmed"):
                groups[k].add(s)
        return sorted(s for g in groups.values() if len({self.name(x) for x in g}) > 1 for s in g)


def render(rows):
    return "\n".join(["\t".join(HEADER)] + ["\t".join(r) for r in rows]) + "\n"


def main(argv=None):
    ap = argparse.ArgumentParser(description="Confirm the bank's function names against docs/data/.")
    ap.add_argument("--check", action="store_true", help="compare a fresh run with the table; exit 1 on drift")
    ap.add_argument("--root", default=ROOT, help=argparse.SUPPRESS)
    args = ap.parse_args(argv)
    c = Confirm(args.root)
    rows = c.rows()
    text = render(rows)
    path = os.path.join(args.root, OUT)
    counts = Counter(r[2] for r in rows)
    ev = Counter(e for r in rows for e in r[3].split(",") if e != "-")
    summary = "confirm_names: {} functions; ".format(len(rows)) + ", ".join(
        "{} {}".format(k, counts[k]) for k in LEVELS)
    acts = Counter(a for _, _, a, _ in c.report)
    why = Counter(a + ": " + w for _, _, a, w in c.report if w)
    summary += "\n  overrides: {} rows; accepted {}, downgraded {}, rejected {}".format(
        len(c.report), acts["accepted"], acts["downgraded"], acts["rejected"])
    for k, v in sorted(why.items()):
        summary += "\n    {} {}".format(v, k)
    for rel, sym, a, w in c.report:
        if a != "accepted":
            summary += "\n    {} {} {} ({})".format(a, sym, rel.rsplit("/", 1)[-1], w)
    aud = Counter(a for _, a, _ in c.audit_report)
    summary += "\n  audit: {} rows; applied {}, rejected {}".format(len(c.audit_report), aud["applied"], aud["rejected"])
    for sym, a, w in c.audit_report:
        if a != "applied":
            summary += "\n    rejected {} audit ({})".format(sym, w)
    shared = c.shared_fingerprint_destructors(rows)
    summary += "\n  destructors sharing a code fingerprint, not byte twins: {} (RegisterExpert::Decode* {})".format(
        len(shared), sum(c.name(s).startswith("RegisterExpert::Decode") for s in shared))
    if args.check:
        with open(path, "rb") as h:
            same = h.read().decode("utf-8") == text
        print(summary + ("; table current" if same else "; table differs"))
        return 0 if same else 1
    with open(path, "wb") as h:
        h.write(text.encode("utf-8"))
    print(summary)
    print("  evidence: " + ", ".join("{} {}".format(k, v) for k, v in sorted(ev.items())))
    tw = [r for r in rows if r[2] == "twin"]
    print("  twin rows with a fingerprint equal partner: {} of {}".format(
        sum("fingerprint equal to" in r[4] for r in tw), len(tw)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
