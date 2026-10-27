"""Generate the reference pages, the marker values and the machine readable index from docs/data/.

Usage:
  python -I tools/gen_docs.py            write docs/generated/, docs/index/ and every marker in the .md files
  python -I tools/gen_docs.py --check    regenerate in memory and report drift; exit 1 on any drift
  python -I tools/gen_docs.py --archive-refs   rebuild docs/data/archive_refs.tsv

Markers: `<!--f:KEY-->value<!--/f-->` for a figure, `<!--gen:KEY-->` ... `<!--/gen:KEY-->` for a table.
Keys and sources: docs/index/facts.json. Standard library only.
"""

import argparse
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from docgen import archive  # noqa: E402
from docgen.data import ROOT  # noqa: E402
from docgen.render import build  # noqa: E402


def disk(root, rel):
    path = os.path.join(root, rel)
    if not os.path.exists(path):
        return None
    with open(path, "rb") as h:
        return h.read().decode("utf-8", errors="replace")


def drift(root=ROOT, confirmation_path=None):
    """Return (outputs, changed, stale, unknown) for the current tree."""
    outputs, stale, unknown, _ = build(root, confirmation_path)
    changed = [rel for rel, text in outputs.items() if disk(root, rel) != text]
    return outputs, changed, stale, unknown


def main(argv=None):
    ap = argparse.ArgumentParser(description="Generate the sapphire reference documentation from docs/data/.")
    ap.add_argument("--check", action="store_true", help="regenerate in memory and exit 1 on drift")
    ap.add_argument("--archive-refs", action="store_true", help="rebuild docs/data/archive_refs.tsv")
    ap.add_argument("--root", default=ROOT, help=argparse.SUPPRESS)
    args = ap.parse_args(argv)
    root = args.root
    if args.archive_refs:
        n = archive.build_refs(root)
        print("gen_docs: docs/data/archive_refs.tsv written, {} refs".format(n))
        return 0
    outputs, changed, stale, unknown = drift(root)
    pages = sum(1 for p in outputs if p.startswith("docs/generated/"))
    if args.check:
        print("gen_docs --check: {} outputs ({} generated pages), {} drifted, {} stale, {} unknown keys".format(
            len(outputs), pages, len(changed), len(stale), len(unknown)))
        for item in (changed + ["stale: " + s for s in stale] + unknown)[:20]:
            print("  " + item)
        return 1 if (changed or stale or unknown) else 0
    for rel in changed:
        path = os.path.join(root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as h:
            h.write(outputs[rel].encode("utf-8"))
    for rel in stale:
        os.remove(os.path.join(root, rel))
    print("gen_docs: {} outputs ({} generated pages), {} written, {} stale removed, {} unknown keys".format(
        len(outputs), pages, len(changed), len(stale), len(unknown)))
    for item in unknown[:20]:
        print("  " + item)
    return 1 if unknown else 0


if __name__ == "__main__":
    sys.exit(main())
