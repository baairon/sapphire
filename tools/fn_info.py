"""Show one function of the dump: bank name, tier, evidence, name check, confirmation, callers, stubs, source.

Usage:
  python -I tools/fn_info.py f_0001b438         by id
  python -I tools/fn_info.py 0x1b438            by offset (an offset inside a body finds the enclosing function)
  python -I tools/fn_info.py SetEveryFrame      by bank name or Rust label (exact, else case-insensitive part)
  add --json for machine readable output

Reads docs/index/functions.jsonl (written by tools/gen_docs.py) and docs/data/stub_names.tsv.
Exit codes: 0 found, 1 not found, 2 usage. Standard library only.
"""

import argparse
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INDEX = os.path.join(ROOT, "docs", "index", "functions.jsonl")
STUBS = os.path.join(ROOT, "docs", "data", "stub_names.tsv")
MAX_LIST = 20


def load(path=INDEX):
    with open(path, encoding="utf-8") as h:
        return [json.loads(line) for line in h if line.strip()]


def stub_names(path=STUBS):
    out = {}
    if os.path.exists(path):
        with open(path, encoding="utf-8") as h:
            next(h)
            for line in h:
                parts = line.rstrip("\n").split("\t")
                if len(parts) >= 2:
                    out[parts[0]] = parts[1]
    return out


def find(rows, query):
    """Rows matching an id, an offset or a name."""
    q = query.strip()
    if re.fullmatch(r"f_[0-9a-fA-F]{8}", q):
        return [r for r in rows if r["id"] == q.lower()]
    if re.fullmatch(r"0x[0-9a-fA-F]+", q):
        off = int(q, 16)
        exact = [r for r in rows if int(r["offset"], 16) == off]
        if exact:
            return exact
        return [r for r in rows if int(r["offset"], 16) <= off < int(r["offset"], 16) + r["size"]]
    exact = [r for r in rows if q in (r["bank_name"], r["rust_label"])]
    if exact:
        return exact
    if re.fullmatch(r"[0-9a-fA-F]{3,}", q):
        hit = find(rows, "0x" + q)
        if hit:
            return hit
    low = q.lower()
    return [r for r in rows if low in r["bank_name"].lower() or low in r["rust_label"].lower()]


def describe(r, stubs):
    src = r["source"] + (":{}".format(r["line"]) if r["line"] else "")
    lines = [
        "{}  {}".format(r["bank_name"], r["id"]),
        "  offset        {}  size {}".format(r["offset"], r["size"]),
        "  kind          {}  origin {}  class {}".format(r["kind"], r["origin"], r["class"] or "-"),
        "  rust label    {}".format(r["rust_label"]),
        "  tier          {}  status {}  strict {}  evidence {}".format(r["tier"], r["status"], r["strict"],
                                                                     r["evidence"]),
        "  name check    {} ({})".format(r["name_check"], r["name_check_reviewed"] or "-"),
        "  confirmation  {}".format(r["confirmation"]),
        "  source        {}".format(src),
        "  page          {}".format(r.get("page", "")),
        "  callers       {}".format(", ".join(r["callers"]) or "none"),
        "  callees       {}".format(", ".join(r["callees"]) or "none"),
    ]
    if r["stubs"]:
        lines.append("  stubs")
        for s in r["stubs"]:
            lines.append("    {}  {}".format(s, stubs.get(s, "")))
    else:
        lines.append("  stubs         none")
    return "\n".join(lines)


def main(argv=None):
    ap = argparse.ArgumentParser(description="Show one function of the dump by id, offset or name.")
    ap.add_argument("query", metavar="ID|NAME|OFFSET")
    ap.add_argument("--json", action="store_true", help="print the matching rows as JSON")
    ap.add_argument("--index", default=INDEX, help=argparse.SUPPRESS)
    args = ap.parse_args(argv)
    if not os.path.exists(args.index):
        print("fn_info: {} missing; run python -I tools/gen_docs.py".format(os.path.relpath(args.index, ROOT)),
              file=sys.stderr)
        return 1
    hits = find(load(args.index), args.query)
    if args.json:
        print(json.dumps(hits[:MAX_LIST] if len(hits) > 1 else hits, indent=1))
        return 0 if hits else 1
    if not hits:
        print("fn_info: no function matches {!r}".format(args.query), file=sys.stderr)
        return 1
    if len(hits) == 1:
        print(describe(hits[0], stub_names()))
        return 0
    print("{} functions match {!r}{}:".format(len(hits), args.query,
                                             " (first {})".format(MAX_LIST) if len(hits) > MAX_LIST else ""))
    for r in hits[:MAX_LIST]:
        print("  {:>8}  {}  {}".format(r["offset"], r["id"], r["bank_name"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
