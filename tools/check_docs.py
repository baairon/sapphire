"""Check the documentation of the sapphire repository.

Usage:
  python -I tools/check_docs.py              run every check, print counts and the first findings, exit 1 on any
  python -I tools/check_docs.py --list NAME  print every finding of one check
  python -I tools/check_docs.py --json       print every finding as JSON

Checks: regen (generated files equal a fresh generation, no unknown marker keys), links (relative links and
anchors), paths (inline code paths), ids (every f_XXXXXXXX is in docs/data/FUNCTION_MAP.tsv), flags and exits
(flag tables against --help, exit code tables against contracts/error.schema.json; active once that schema
exists), ascii (and control characters but tab, LF and CR), dashes, secrets, person (first person), claims (forbidden claims), ffmpeg (ffmpeg claims cite a
doc page or a measured command), archive (archive:PATH citations resolve through docs/data/archive_refs.tsv),
effects (an effect is named only by its exact string from docs/data/effect_names.tsv, in code font).

Effects rule scope: an `S_` token must be a bank name, in code font, and never on a line with a function id;
a near miss is a token that equals a bank name or its stem (the name without `S_`) once case, spaces, hyphens
and underscores are ignored, or an `S_` token within a 0.85 spelling ratio of one. Stems are flagged when
written as names: CamelCase ("AutoPaint"), two to four capitalised words ("Band Pass"), a code span holding
only the stem with a capital letter or a separator (`Aurora`, `band_pass`; a lowercase single word in code font
is an identifier), or up to three words next to "effect" ("Aurora effect"). Exempt: exact parameter candidates
(`blur`), lines that mention twixtor (its own effect and stage names), and the bank quotes in generated pages
(only their prose is checked). Standard library only.
"""

import argparse
import difflib
import glob
import json
import os
import re
import subprocess
import sys

TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, TOOLS)

from docgen.archive import resolves  # noqa: E402
from docgen.data import ROOT, read_tsv  # noqa: E402
from docgen.render import SKIP_DIRS, md_files  # noqa: E402

CHECKS = ["regen", "links", "paths", "ids", "flags", "exits", "ascii", "dashes", "secrets", "person", "claims",
          "ffmpeg", "archive", "effects"]
TEXT_EXT = (".md", ".py", ".rs", ".toml", ".json", ".jsonl", ".tsv", ".yml", ".yaml", ".txt", ".html", ".css",
            ".schema")
ROOT_FILES = {"README.md", "Cargo.toml", "Cargo.lock", "LICENSE", ".gitignore", ".gitattributes"}
PATH_HEAD = re.compile(r"^(crates|tools|docs|contracts|\.github|\.forgejo)/")
DASHES = (chr(0x2013), chr(0x2014))
# Paths that later phases create; mentioning them before they exist is not a finding.
OPTIONAL_PATHS = {"docs/data/name_confirmation.tsv"}
SECRET_PATTERNS = [
    ("private key", re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----")),
    ("github token", re.compile(r"\b(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,})")),
    ("api key", re.compile(r"\b(?:sk-[A-Za-z0-9_-]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}|AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{35})")),
    ("assigned secret", re.compile(r"(?i)\b(?:password|passwd|secret|api[_-]?key|token)\s*[:=]\s*['\"][^'\"\s]{8,}['\"]")),
    ("email address", re.compile(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)*\.[A-Za-z]{2,}\b")),
    ("home path", re.compile(r"(?i)\b[A-Z]:[\\/]+Users[\\/]+[^\\/\s'\x22]{2,}|/home/[a-z][\w.-]*/|/Users/[A-Za-z][\w.-]*/")),
    ("phone number", re.compile(r"\(\d{3}\)\s?\d{3}-\d{4}")),
]
EMAIL_OK = re.compile(r"@(?:example\.(?:com|org|net)|users\.noreply\.github\.com|bairon\.dev)$")
PERSON = re.compile(r"\b(?:I|I'm|I've|I'll|I'd)\b|\b(?:[Mm]e|[Mm]y|[Mm]ine|[Ww]e|[Ww]e're|[Ww]e've|[Oo]ur|[Oo]urs|us|"
                    r"[Ll]et's)\b")
CLEAN = re.compile(r"(?i)clean[\s-]?room")
PROVEN = re.compile(r"(?i)(?<!un)\bproven\b|\bprovably correct\b")
FF_CLAIM = re.compile(r"(?i)\b(?:ffmpeg|ffprobe)\b[^.\n]{0,80}?\b(?:exits?|accepts?|rejects?|writes?|reads?|returns?|"
                      r"supports?|requires?|refuses?|ignores?|drops?|keeps?|stores?|prints?|hangs?|fails?|outputs?|"
                      r"produces?|treats?|converts?|rounds?|truncates?|detects?|stalls?|emits?)\b")
FF_CITE = re.compile(r"(?i)[\w-]+\.html\b|ffmpeg\.org|`(?:ffmpeg|ffprobe)\s[^`]+`|"
                     r"docs/data/(?:contract\w*|pipeline_parity)\.tsv|handoff-bench\.tsv|conformance(?:\.py)?\b")
ARCHIVE = re.compile(r"archive:([A-Za-z0-9_./*-]+[A-Za-z0-9_*])")
FID = re.compile(r"\bf_[0-9a-f]{8}\b")
LINK = re.compile(r"!?\[([^\]]*)\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
SPAN = re.compile(r"``\s?(.+?)\s?``|`([^`\n]+)`")
COMMENT = re.compile(r"<!--.*?-->")
ALLOW = "<!--allow:claim-->"


class Ctx:
    def __init__(self, root):
        self.root = root
        self._text = {}
        self.md = [p for p in md_files(root) if not p.startswith("docs/data/")]

    def text(self, rel):
        if rel not in self._text:
            with open(os.path.join(self.root, rel), "rb") as h:
                self._text[rel] = h.read().decode("utf-8", errors="replace")
        return self._text[rel]

    def text_files(self):
        out = []
        for dirpath, dirs, files in os.walk(self.root):
            dirs[:] = sorted(d for d in dirs if d not in SKIP_DIRS)
            for f in sorted(files):
                if f.endswith(TEXT_EXT):
                    out.append(os.path.relpath(os.path.join(dirpath, f), self.root).replace(os.sep, "/"))
        return out


def lines_outside_fences(text):
    """Yield (line number, line) for lines outside fenced code blocks."""
    fence = None
    for i, line in enumerate(text.split("\n"), 1):
        s = line.strip()
        m = re.match(r"(```|~~~)", s)
        if m:
            if fence is None:
                fence = m.group(1)
            elif s.startswith(fence):
                fence = None
            continue
        if fence is None:
            yield i, line


def prose(line):
    return SPAN.sub(" ", COMMENT.sub(" ", line))


def slug(heading):
    h = COMMENT.sub("", heading)
    h = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", h).strip().lower()
    h = re.sub(r"[^\w\- ]", "", h)
    return h.replace(" ", "-")


def anchors(text):
    out, seen = set(), {}
    for _, line in lines_outside_fences(text):
        m = re.match(r"#{1,6}\s+(.*?)\s*#*\s*$", line)
        if m:
            s = slug(m.group(1))
            n = seen.get(s, 0)
            out.add(s if n == 0 else "{}-{}".format(s, n))
            seen[s] = n + 1
        for a in re.finditer(r"<a\s+(?:id|name)=\"([^\"]+)\"", line):
            out.add(a.group(1))
    return out


# ---- checks ---------------------------------------------------------------

def check_regen(ctx):
    import gen_docs
    _, changed, stale, unknown = gen_docs.drift(ctx.root)
    out = [("regen", p, 0, "differs from a fresh generation") for p in changed]
    out += [("regen", p, 0, "stale generated page") for p in stale]
    out += [("regen", u.split(": ")[0], 0, "unknown marker key " + u.split(": ")[1]) for u in unknown]
    return out


def check_links(ctx):
    out, cache = [], {}
    for rel in ctx.md:
        base = os.path.dirname(rel)
        for i, line in lines_outside_fences(ctx.text(rel)):
            for m in LINK.finditer(SPAN.sub(" ", line)):
                target = m.group(2)
                if re.match(r"(?i)[a-z][a-z0-9+.-]*:", target):
                    continue
                path, _, anchor = target.partition("#")
                full = rel if not path else os.path.normpath(os.path.join(base, path)).replace(os.sep, "/")
                if path.startswith("/"):
                    full = path.lstrip("/")
                if not os.path.exists(os.path.join(ctx.root, full)):
                    out.append(("links", rel, i, "missing target " + target))
                    continue
                if anchor and full.endswith(".md"):
                    if full not in cache:
                        cache[full] = anchors(ctx.text(full))
                    if anchor not in cache[full]:
                        out.append(("links", rel, i, "missing anchor " + target))
    return out


def check_paths(ctx):
    """Inline code paths exist, or are listed in docs/data/archive_refs.tsv (archived or placed by the user)."""
    refs_path = os.path.join(ctx.root, "docs", "data", "archive_refs.tsv")
    refs = read_tsv(refs_path) if os.path.exists(refs_path) else []
    out, seen = [], {}
    for rel in ctx.md:
        for i, line in lines_outside_fences(ctx.text(rel)):
            if "twixtor" in line.lower():
                continue
            for m in SPAN.finditer(line):
                s = (m.group(1) or m.group(2)).strip().replace("\\", "/")
                s = re.sub(r":\d+(?:-\d+)?$", "", s.split("#")[0]).rstrip(":")
                if not (PATH_HEAD.match(s) or s in ROOT_FILES) or " " in s:
                    continue
                if any(c in s for c in "*<>{}$|") or "NNN" in s or "XXXX" in s or "..." in s or s in OPTIONAL_PATHS:
                    continue
                if s not in seen:
                    seen[s] = os.path.exists(os.path.join(ctx.root, s.rstrip("/"))) or resolves(s.rstrip("/"), refs)
                if not seen[s]:
                    out.append(("paths", rel, i, "missing path " + s))
    return out


def check_ids(ctx):
    ids = {r["name"] for r in read_tsv(os.path.join(ctx.root, "docs", "data", "FUNCTION_MAP.tsv"))}
    out = []
    for rel in ctx.md:
        for i, line in enumerate(ctx.text(rel).split("\n"), 1):
            for m in FID.finditer(line):
                if m.group(0) not in ids:
                    out.append(("ids", rel, i, "unknown id " + m.group(0)))
    return out


def tables(text):
    """Yield (line of the header, preceding comment, header cells, rows of cells)."""
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        if lines[i].lstrip().startswith("|") and i + 1 < len(lines) and re.match(r"^\s*\|?\s*:?-{3,}", lines[i + 1]):
            head = [c.strip().strip("`").lower() for c in lines[i].strip().strip("|").split("|")]
            prev = lines[i - 1].strip() if i > 0 else ""
            if not prev and i > 1:
                prev = lines[i - 2].strip()
            rows, j = [], i + 2
            while j < len(lines) and lines[j].lstrip().startswith("|"):
                rows.append((j + 1, [c.strip() for c in re.split(r"(?<!\\)\|", lines[j].strip().strip("|"))]))
                j += 1
            yield i + 1, prev, head, rows
            i = j
        else:
            i += 1


def schema_exits(path):
    """code -> exit code from contracts/error.schema.json, read generically."""
    with open(path, encoding="utf-8") as h:
        doc = json.load(h)
    found = {}

    def one(v):
        if isinstance(v, dict):
            if "const" in v:
                return v["const"]
            if isinstance(v.get("enum"), list) and len(v["enum"]) == 1:
                return v["enum"][0]
        return None

    def walk(node):
        if isinstance(node, dict):
            props = node.get("properties")
            if isinstance(props, dict) and "code" in props and "exit_code" in props:
                c, e = one(props["code"]), one(props["exit_code"])
                if isinstance(c, str) and isinstance(e, int):
                    found[c] = e
            if "code" in node and "exit_code" in node and isinstance(node["code"], str) and isinstance(node["exit_code"], int):
                found[node["code"]] = node["exit_code"]
            ints = {k: v for k, v in node.items() if isinstance(v, int) and not isinstance(v, bool)}
            if len(ints) >= 3 and len(ints) == len(node) and all(re.match(r"^[a-z_]+$", k) for k in ints):
                found.update(ints)
            for v in node.values():
                walk(v)
        elif isinstance(node, list):
            for v in node:
                walk(v)

    walk(doc)
    return found


def help_texts(root):
    """Help output of the tools that exist, keyed by command string."""
    out = {}

    def run(key, cmd):
        try:
            p = subprocess.run(cmd, cwd=root, capture_output=True, text=True, timeout=30)
            out[key] = (p.stdout or "") + (p.stderr or "")
        except (OSError, subprocess.TimeoutExpired):
            out[key] = ""

    for tool in ("handoff.py", "conformance.py", "gen_docs.py", "fn_info.py"):
        path = os.path.join(root, "tools", tool)
        if os.path.exists(path):
            key = "python -I tools/" + tool
            run(key, [sys.executable, "-I", path, "--help"])
            m = re.search(r"\{([a-z0-9_,-]+)\}", out[key])
            for sub in (m.group(1).split(",") if m else []):
                run(key + " " + sub, [sys.executable, "-I", path, sub, "--help"])
    # the exact name: target/*/deps also holds sapphire-<hash>, the test harness of the binary
    name = "sapphire.exe" if os.name == "nt" else "sapphire"
    for exe in sorted(glob.glob(os.path.join(root, "target", "**", name), recursive=True)):
        if os.path.isfile(exe) and os.access(exe, os.X_OK):
            run("sapphire", [exe, "--help"])
            break
    return out


def check_flags(ctx):
    if not os.path.exists(os.path.join(ctx.root, "contracts", "error.schema.json")):
        return []
    helps = help_texts(ctx.root)
    union = "\n".join(helps.values())
    out = []
    for rel in ctx.md:
        for ln, prev, head, rows in tables(ctx.text(rel)):
            if not head or head[0] not in ("flag", "flags", "option", "options"):
                continue
            m = re.match(r"<!--help:(.+?)-->", prev)
            text = union
            if m:
                text = helps.get(m.group(1).strip(), "")
                if not text:
                    out.append(("flags", rel, ln, "no help output for " + m.group(1).strip()))
                    continue
            for row_ln, cells in rows:
                for flag in re.findall(r"--[a-z0-9][a-z0-9-]*", cells[0]):
                    if not re.search(re.escape(flag) + r"(?![a-z0-9-])", text):
                        out.append(("flags", rel, row_ln, "flag {} not in --help".format(flag)))
    return out


def check_exits(ctx):
    path = os.path.join(ctx.root, "contracts", "error.schema.json")
    if not os.path.exists(path):
        return []
    try:
        mapping = schema_exits(path)
    except (OSError, ValueError) as e:
        return [("exits", "contracts/error.schema.json", 0, "unreadable: {}".format(e))]
    if not mapping:
        return [("exits", "contracts/error.schema.json", 0, "no code to exit code mapping found")]
    out = []
    for rel in ctx.md:
        for ln, _, head, rows in tables(ctx.text(rel)):
            ci = next((k for k, h in enumerate(head) if h in ("code", "error code", "kind", "error.code")), None)
            ei = next((k for k, h in enumerate(head) if h in ("exit", "exit code", "exit_code", "exit status")), None)
            if ci is None or ei is None:
                continue
            for row_ln, cells in rows:
                if max(ci, ei) >= len(cells):
                    continue
                code = cells[ci].strip().strip("`")
                ex = re.match(r"^`?(\d+)`?$", cells[ei].strip())
                if not re.match(r"^[a-z][a-z_]*$", code) or not ex:
                    continue
                if code not in mapping:
                    out.append(("exits", rel, row_ln, "code {} not in the error schema".format(code)))
                elif mapping[code] != int(ex.group(1)):
                    out.append(("exits", rel, row_ln, "code {} exits {} in the schema, {} here".format(
                        code, mapping[code], ex.group(1))))
    return out


def check_ascii(ctx):
    out = []
    files = ctx.md + [p for p in ctx.text_files() if p.startswith(("tools/", "contracts/")) and not p.endswith(".md")]
    for rel in files:
        for i, line in enumerate(ctx.text(rel).split("\n"), 1):
            if any(ord(c) > 127 for c in line):
                out.append(("ascii", rel, i, "non-ASCII character"))
            ctrl = sorted({ord(c) for c in line if (ord(c) < 32 and ord(c) not in (9, 13)) or ord(c) == 127})
            if ctrl:
                out.append(("ascii", rel, i, "control character " + ", ".join("0x%02x" % o for o in ctrl)))
    return out


def check_dashes(ctx):
    out = []
    for rel in ctx.text_files():
        text = ctx.text(rel)
        if not any(d in text for d in DASHES):
            continue
        for i, line in enumerate(text.split("\n"), 1):
            if any(d in line for d in DASHES):
                out.append(("dashes", rel, i, "em or en dash"))
    return out


def check_secrets(ctx):
    out = []
    for rel in ctx.text_files():
        text = ctx.text(rel)
        for name, pat in SECRET_PATTERNS:
            for m in pat.finditer(text):
                if name == "email address" and EMAIL_OK.search(m.group(0)):
                    continue
                out.append(("secrets", rel, text.count("\n", 0, m.start()) + 1, name))
    return out


def check_person(ctx):
    out = []
    for rel in ctx.md:
        for i, line in lines_outside_fences(ctx.text(rel)):
            m = PERSON.search(prose(line))
            if m:
                out.append(("person", rel, i, "first person: " + m.group(0)))
    return out


def check_claims(ctx):
    data = read_tsv(os.path.join(ctx.root, "docs", "data", "FUNCTION_MAP.tsv"))
    unproven = [(r["name"], r["offset"]) for r in data if r["strict"] == "unproven"]
    pats = [re.compile(r"\b(?:{}|{})\b".format(re.escape(n), re.escape(o))) for n, o in unproven]
    out = []
    files = ctx.md + [p for p in ctx.text_files() if p.startswith(("tools/", "contracts/")) and not p.endswith(".md")]
    for rel in files:
        if rel.startswith("tools/") and rel.endswith(".py"):
            continue
        for i, line in enumerate(ctx.text(rel).split("\n"), 1):
            if ALLOW in line:
                continue
            if CLEAN.search(line):
                out.append(("claims", rel, i, "forbidden claim: clean room"))
            if PROVEN.search(line) and any(p.search(line) for p in pats):
                out.append(("claims", rel, i, "'proven' next to an unproven function"))
    return out


def check_ffmpeg(ctx):
    out = []
    for rel in ctx.md:
        if rel.startswith("docs/generated/"):
            continue
        sections, cur = [], [0, []]
        for i, line in enumerate(ctx.text(rel).split("\n"), 1):
            if re.match(r"#{1,6}\s", line):
                sections.append(cur)
                cur = [i, []]
            cur[1].append((i, line))
        sections.append(cur)
        for start, lines in sections:
            body = "\n".join(l for _, l in lines)
            claim = next((i for i, l in lines_outside_fences(body) if FF_CLAIM.search(prose(l))), None)
            prose_lines = {i for i, _ in lines_outside_fences(body)}
            fenced = [l for i, l in enumerate(lines, 1) if i not in prose_lines]
            fenced = [l for _, l in fenced]
            command = any(re.match(r"\s*(?:\$\s*)?(?:ffmpeg|ffprobe)(?:\.exe)?\s", l) for l in fenced)
            if claim is not None and not FF_CITE.search(body) and not command:
                out.append(("ffmpeg", rel, lines[claim - 1][0], "ffmpeg claim without a doc page or measured command"))
    return out


def check_archive(ctx):
    path = os.path.join(ctx.root, "docs", "data", "archive_refs.tsv")
    rows = read_tsv(path) if os.path.exists(path) else []
    out = []
    for rel in ctx.md:
        for i, line in enumerate(ctx.text(rel).split("\n"), 1):
            for m in ARCHIVE.finditer(line):
                if not rows:
                    out.append(("archive", rel, i, "docs/data/archive_refs.tsv missing"))
                elif not resolves(m.group(1), rows):
                    out.append(("archive", rel, i, "unresolved archive:" + m.group(1)))
    return out


E_PREFIXED = re.compile(r"(?<![\w-])S_[A-Za-z0-9_]*[A-Za-z0-9]")
E_LOWER = re.compile(r"(?<![\w-])s_[A-Za-z0-9_]*[A-Za-z0-9]")
E_SPACED = re.compile(r"(?<![\w-])S[ -]([A-Z][A-Za-z0-9]*(?:[ -][A-Z][A-Za-z0-9]*){0,2})(?![\w])")
E_CAMEL = re.compile(r"(?<![\w-])[A-Z][a-z0-9]*[A-Z][A-Za-z0-9]*(?![\w-])")
E_MULTI = re.compile(r"(?<![\w-])[A-Z][A-Za-z0-9]*(?:[ -][A-Z][A-Za-z0-9]*){1,3}(?![\w-])")
E_WORD = re.compile(r"[A-Za-z0-9]+")


def ekey(s):
    return re.sub(r"[^a-z0-9]", "", s.lower())


class EffectBank:
    """Effect names and parameter candidates of docs/data/, for the near miss rules."""

    def __init__(self, root):
        d = os.path.join(root, "docs", "data")
        self.names = [r["name"] for r in read_tsv(os.path.join(d, "effect_names.tsv"))]
        path = os.path.join(d, "param_candidates.tsv")
        self.cands = {r["candidate"] for r in read_tsv(path)} if os.path.exists(path) else set()
        self.exact = set(self.names)
        self.lower = {n.lower(): n for n in self.names}
        self.key = {ekey(n): n for n in self.names}
        self.stem = {ekey(n[2:]): n for n in self.names if n.startswith("S_")}

    def prefixed(self, tok):
        """Message for an S_ token that is not a bank name."""
        if tok.lower() in self.lower:
            return "near miss (case) of {}: {}".format(self.lower[tok.lower()], tok)
        if ekey(tok) in self.key:
            return "near miss (spacing) of {}: {}".format(self.key[ekey(tok)], tok)
        close = difflib.get_close_matches(tok, self.names, 1, 0.85)
        if close:
            return "near miss (spelling) of {}: {}".format(close[0], tok)
        return "not in the effect bank: " + tok

    def stem_hit(self, words):
        text = " ".join(words)
        if text in self.cands or text in self.exact:
            return None
        n = self.stem.get(ekey(text))
        return "near miss (stem written as an effect name) of {}: {}".format(n, text) if n else None


def effect_line(bank, line, generated):
    """Findings for one line outside fences. Generated pages are checked in prose only."""
    out = []
    twixtor = "twixtor" in line.lower()
    spans = [(m.group(1) or m.group(2)).strip() for m in SPAN.finditer(COMMENT.sub(" ", line))]
    text = prose(line)
    for tok in E_PREFIXED.findall(text):
        out.append("effect name not in code font: " + tok if tok in bank.exact else bank.prefixed(tok))
    for tok in E_LOWER.findall(text) + [m.group(0) for m in E_SPACED.finditer(text)]:
        n = bank.key.get(ekey(tok))
        if n:
            out.append("near miss (case or spacing) of {}: {}".format(n, tok))
    if not generated:
        for s in spans:
            for tok in E_PREFIXED.findall(s):
                if tok not in bank.exact:
                    out.append(bank.prefixed(tok))
            for tok in E_LOWER.findall(s):
                if bank.key.get(ekey(tok)):
                    out.append("near miss (case) of {}: {}".format(bank.key[ekey(tok)], tok))
            # A lowercase single word in code font is an identifier (`effect` is a handoff role, `grid` a field).
            styled = s != s.lower() or re.search(r"[ _-]", s)
            if not twixtor and styled and re.fullmatch(r"[A-Za-z][A-Za-z0-9 _-]*", s) and not s.startswith(("S_", "s_")):
                hit = bank.stem_hit([s])
                if hit:
                    out.append(hit)
    if twixtor:
        return out
    seen = set()
    for m in E_CAMEL.finditer(text):
        hit = bank.stem_hit([m.group(0)])
        if hit:
            seen.add(m.group(0))
            out.append(hit)
    for m in E_MULTI.finditer(text):
        words = re.split(r"[ -]", m.group(0))
        for i in range(len(words)):
            for j in range(i + 2, len(words) + 1):
                hit = bank.stem_hit(words[i:j])
                if hit:
                    out.append(hit)
    words = E_WORD.findall(text)
    for i, w in enumerate(words):
        if w.lower() not in ("effect", "effects"):
            continue
        for seq in [words[max(0, i - k):i] for k in (1, 2, 3)] + [words[i + 1:i + 1 + k] for k in (1, 2, 3)]:
            if seq and " ".join(seq) not in seen:
                hit = bank.stem_hit(seq)
                if hit and hit not in out:
                    out.append(hit)
    return out


def check_effects(ctx):
    """Effects are named only by their exact bank string in code font, and never beside a function id."""
    bank = EffectBank(ctx.root)
    out = []
    for rel in ctx.md:
        generated = rel.startswith("docs/generated/")
        for i, line in lines_outside_fences(ctx.text(rel)):
            for msg in effect_line(bank, line, generated):
                out.append(("effects", rel, i, msg))
            if FID.search(line) and any(t in bank.exact for t in E_PREFIXED.findall(line)):
                out.append(("effects", rel, i, "effect name beside a function id implies a mapping"))
    return out


def run(root=ROOT, names=None):
    ctx = Ctx(root)
    results = {}
    for name in names or CHECKS:
        results[name] = globals()["check_" + name](ctx)
    return results


def main(argv=None):
    ap = argparse.ArgumentParser(description="Check the sapphire documentation.")
    ap.add_argument("--list", metavar="CHECK", choices=CHECKS, help="print every finding of one check")
    ap.add_argument("--json", action="store_true", help="print every finding as JSON")
    ap.add_argument("--only", metavar="CHECK", action="append", choices=CHECKS, help="run only these checks")
    ap.add_argument("--root", default=ROOT, help=argparse.SUPPRESS)
    args = ap.parse_args(argv)
    results = run(args.root, args.only)
    total = sum(len(v) for v in results.values())
    if args.json:
        print(json.dumps({k: [list(f) for f in v] for k, v in results.items()}, indent=1))
        return 1 if total else 0
    print("check_docs: {} findings; ".format(total) + ", ".join("{} {}".format(k, len(v)) for k, v in results.items()))
    if not os.path.exists(os.path.join(args.root, "contracts", "error.schema.json")):
        print("  flags and exits inactive: contracts/error.schema.json not present yet")
    for name, found in results.items():
        shown = found if args.list == name else (found[:5] if not args.list else [])
        for _, path, line, msg in shown:
            print("  {} {}:{} {}".format(name, path, line, msg))
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
