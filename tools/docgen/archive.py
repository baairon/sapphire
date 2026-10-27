"""Build docs/data/archive_refs.tsv: the archived material that docs/data/ and .agents/inputs/ name.

The archive itself is never read. A reference is recorded only where a checked in table, an input note or an
input source copy already names the path, so `archive:PATH` citations in the documents resolve offline.
A folder that a lift test reads from `crates/` and is ignored for want of `sapphire_archive.zip` is recorded
with kind `placed` (the user places it there from the archive).
Rules: Windows separators become `/`; paths deeper than three parts are recorded as their three part directory;
a `tools/` path counts only as a source copied into `.agents/inputs/tools_src/` (recorded under `tools/`, where
those sources lived); a `docs/` path counts when docs/data/ names it or `.agents/inputs/` holds a copy, and the
repository no longer has it.
"""

import os
import re
from collections import OrderedDict

from .data import read_text

TOKEN = re.compile(r"(?<![\w/.-])((?:findings|scripts|original|disasm|index|aggr|fleet|vectors|specs|ledgers|tools|docs)"
                   r"/[A-Za-z0-9_./*-]*[A-Za-z0-9_*])")
DUMP = re.compile(r"(?<![\w/.-])(neutral_\d\.text\.bin)")
HEADER = ["ref", "kind", "mentioned_in", "mentions"]
# A test that reads CARGO_MANIFEST_DIR/../NAME and is ignored for want of the archive names a placed folder.
PLACED = re.compile(r'join\("\.\."\)\.join\("([A-Za-z0-9_-]+)"\)')
SOURCE_EXT = (".md", ".tsv", ".py")


def _sources(root):
    out = []
    for base in ("docs/data", ".agents/inputs"):
        top = os.path.join(root, base)
        for dirpath, dirs, files in os.walk(top):
            dirs[:] = sorted(d for d in dirs if d not in ("twixtor_baseline", "__pycache__"))
            for f in sorted(files):
                if f.endswith(SOURCE_EXT) and f != "archive_refs.tsv":
                    out.append(os.path.relpath(os.path.join(dirpath, f), root).replace(os.sep, "/"))
    return out


def collect(root):
    refs = OrderedDict()

    def add(ref, kind, where):
        if ref not in refs:
            refs[ref] = [kind, where, 0]
        refs[ref][2] += 1

    for rel in _sources(root):
        if rel.startswith(".agents/inputs/tools_src/"):
            add("tools/" + rel.split("/")[-1], "file", rel)
        for i, line in enumerate(read_text(os.path.join(root, rel)).split("\n"), 1):
            line = line.replace("\\", "/")
            line = re.sub(r"/{2,}", "/", line)
            for m in TOKEN.finditer(line):
                tok = m.group(1).rstrip(".")
                parts = tok.split("/")
                if parts[0] == "tools":
                    continue
                if parts[0] == "docs":
                    copy = os.path.join(root, ".agents", "inputs", parts[-1])
                    known = rel.startswith("docs/data/") or os.path.exists(copy)
                    if not known or os.path.exists(os.path.join(root, *parts)) or tok.startswith("docs/data"):
                        continue
                if len(parts) > 3:
                    add("/".join(parts[:3]), "dir", "{}:{}".format(rel, i))
                else:
                    kind = "file" if "." in parts[-1] else "dir"
                    add(tok, kind, "{}:{}".format(rel, i))
            for m in DUMP.finditer(line):
                add(m.group(1), "dump", "{}:{}".format(rel, i))
    crates = os.path.join(root, "crates")
    for name in sorted(os.listdir(crates)) if os.path.isdir(crates) else []:
        tests = os.path.join(crates, name, "tests")
        for f in sorted(os.listdir(tests)) if os.path.isdir(tests) else []:
            if not f.endswith(".rs"):
                continue
            rel = "crates/{}/tests/{}".format(name, f)
            text = read_text(os.path.join(root, rel))
            if "sapphire_archive.zip" not in text:
                continue
            for i, line in enumerate(text.split("\n"), 1):
                for m in PLACED.finditer(line):
                    add("crates/" + m.group(1), "placed", "{}:{}".format(rel, i))
    return refs


def build_refs(root):
    refs = collect(root)
    lines = ["\t".join(HEADER)]
    for ref in sorted(refs):
        kind, where, n = refs[ref]
        lines.append("\t".join([ref, kind, where, str(n)]))
    path = os.path.join(root, "docs", "data", "archive_refs.tsv")
    with open(path, "wb") as h:
        h.write(("\n".join(lines) + "\n").encode("utf-8"))
    return len(refs)


def resolves(ref, rows):
    """True when `ref` names a recorded file, or a path under a recorded directory."""
    for r in rows:
        if ref == r["ref"]:
            return True
        if r["kind"] in ("dir", "placed") and ref.startswith(r["ref"].rstrip("/") + "/"):
            return True
    return False
