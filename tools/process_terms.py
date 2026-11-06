"""Terms that say how the repository was made instead of what it does, for the documentation and code checks.

tools/check_docs.py runs `prose_hits` on every page (check `process`); tools/test_effect_rules.py runs `source_hits` on
the comments and string literals of crates/ and tools/. Standard library only.

A hit is one of:
- a term of TERMS (process vocabulary, numbered work items, paths of working folders);
- a code: a run of capitals and digits such as AB12, AB12-C3 or AB-12, or of lowercase letters and digits followed by
  `_word` or `/` such as ab12_word, unless it is a standard technical name (TECHNICAL), an identifier of the code (a
  name the Rust, Python or Cargo sources define or use), an assumption id (`...-A12`) or a register (`XMM0`).
In source files a string literal that is one word, one code or one relative path is a key and not prose: it is skipped.
"""
import os
import re

TERMS = [
    ("work item number", re.compile(r"\bW\d{3,4}\b")),
    # wave_ is followed by a digit in a working folder name; effect parameters such as wave_freq are not matched
    ("working folder", re.compile(r"\bwave_[a-z]*\d\w*|\bwave_(?:glow_blur|shake)\b|\bnext_wave\b|\.agents\b|\brebuild_[a-z0-9]\w*")),
    ("fleet", re.compile(r"\bfleet\b", re.I)),
    ("coordinator", re.compile(r"\bcoordinators?\b", re.I)),
    ("ruling", re.compile(r"\brulings?\b", re.I)),
    ("decision", re.compile(r"\b(?:owner|maintainer)'?s? decisions?\b|\bdecided by (?:the )?(?:owner|maintainer|coordinator)\b", re.I)),
    ("maintainer", re.compile(r"\bmaintainers?\b", re.I)),
    ("builder", re.compile(r"\bby (?:the |a )?builders?\b|\bbuilders? (?:report|agent|role|wrote|ran)\b|\bBUILDERS?\b")),
    ("brief", re.compile(r"\bthe brief\b", re.I)),
    ("step label", re.compile(r"(?<![\w-])[Ss]teps? (?:[A-Z][A-Z0-9]+[a-z]?\d*[a-z]?\b|[A-Z](?=[:)]|'s)|s\d+_\w+)")),
    ("piece label", re.compile(r"\b[Pp]ieces? [A-Z][\w-]*\d")),
    ("batch", re.compile(r"\b[Bb]atch(?:es)? \d+")),
    ("wave", re.compile(r"\b[Ww]aves? (?:[Ss]?\d+|[A-Z]\d*)\b")),
    ("work note", re.compile(r"\b[Ss]cratch (?:std-only|only|crate|port|copy)\b|\bScratch[,.]|\blean effect rebuild\b|"
                             r"\brelaunch\w*|\bhigh-effort\b")),
    ("work document", re.compile(r"\b(?:DECISIONS|RESUME|MORNING|ROADMAP)_\w+|"
                                 r"\b(?:SPEC|MAPPING|NEXT|API|SELECTOR|NOTES|CELLS|LANES|PRED|BRIEF|VERIFY)(?:_[A-Z0-9_]+)?\.(?:md|tsv)\b|"
                                 r"\b[\w-]+\.lst\b")),
    ("person", re.compile(r"\bAle\b|\bAlejandro\b|\bVPS\b")),
    ("booked", re.compile(r"\bbooked\b", re.I)),
    ("frozen members", re.compile(r"\bfrozen members?\b", re.I)),
    ("counted rows", re.compile(r"\bcounted rows\b", re.I)),
    ("claim route", re.compile(r"\bclaim routes?\b", re.I)),
    ("in profile", re.compile(r"\bin profile\b", re.I)),
    ("labelled assumption", re.compile(r"\blabell?ed assumptions?\b", re.I)),
]
# A date in a comment or string of crates/ dates the work, not the code (tools/docgen dates a revised standard: allowed).
DATE_IN_SOURCE = ("date stamp", re.compile(r"\b20\d\d-[01]\d-[0-3]\d\b"))

# Standard technical names that have the shape of a code, with the reason each one stays. Only names that occur are listed;
# a capital letter with digits (S5, K1, F2, T2, R0) is never a code: it labels a listing section, a kernel, a test kind or a
# register inside one file.
TECHNICAL = {
    "AVX2": "x86 instruction set extension",
    "SSE2": "x86 instruction set extension",
    "FNV64": "64-bit FNV hash",
    "SHA-256": "hash function",
    "UTF-8": "text encoding",
    "UTF-16": "text encoding",
    "FNV-1a": "hash function",
    "CRC-32": "checksum (PNG chunks)",
    "BIT2": "field name of the fx_run --owner-arm argument",
    "EXIT2": "field name of the fx_run --owner-arm argument",
    "VB4": "the four-plane variant-B reader head, part of the labels CLASS3-VB4-HANDOVER-REFUSED and SOURCE-HEAD-VB4-ON-CLASS3",
}
UPPER = re.compile(r"(?<![\w\-./`$#])([A-Z]{2,6}\d{1,3}[a-z]?)(?:-([A-Z]{1,2}\d{1,3}[a-z]?))?(?![\w\-/.]\w|\w|`)")
HYPHEN = re.compile(r"(?<![\w\-./`])([A-Z]{2,5}-\d{1,3}[a-z]?)(?![\w`]|[\-/.]\w)")
LOWER = re.compile(r"(?<![\w\-.`\\])([a-z]{2,5}\d{1,2}[a-z]?)(?:_[a-z][a-z0-9_]*|(?=/))")
REGISTER = re.compile(r"(?:XMM|YMM|ZMM)\d+", re.I)
# A string literal that is one word or one relative path is a key (a label, a mutant or format name, a stored-data
# path), not prose: [\w.{}*+-]+ with optional /-separated parts. A literal that is only a code is a key too.
KEY = re.compile(r"[A-Z]{1,6}\d{1,3}[a-z]?(?:-[A-Z]{1,2}\d{1,3}[a-z]?)?|[\w.{}*+-]+(?:/[\w.{}*+-]*)*")


def code_hits(text, idents):
    """Codes in `text` that are not technical names, identifiers, assumption ids or registers."""
    out = []
    for m in UPPER.finditer(text):
        t = m.group(1)
        if t in TECHNICAL or t in idents or REGISTER.fullmatch(t) or (m.group(2) and re.fullmatch(r"A\d+", m.group(2))):
            continue
        out.append((m.start(), "code", m.group(0)))
    for m in HYPHEN.finditer(text):
        if m.group(1) not in TECHNICAL:
            out.append((m.start(), "code", m.group(1)))
    for m in LOWER.finditer(text):
        word = m.group(0)
        if word in idents or m.group(1).upper() in TECHNICAL or REGISTER.fullmatch(m.group(1)):
            continue
        out.append((m.start(), "code", word))
    return out


def term_hits(text, dated):
    out = []
    terms = TERMS + [DATE_IN_SOURCE] if dated else TERMS
    for name, rx in terms:
        for m in rx.finditer(text):
            if name == "ruling" and re.search(r"\bruling(?:s)? (?:out|in)\b", text[m.start():m.end() + 4]):
                continue
            out.append((m.start(), name, m.group(0)))
    return out


# ---- pages ---------------------------------------------------------------------------------------------------------

SPAN = re.compile(r"``\s?(.+?)\s?``|`([^`\n]+)`|\]\([^)]*\)|<!--.*?-->")


def prose_hits(line):
    """Hits in one page line outside code spans, link targets and HTML comments."""
    masked = SPAN.sub(lambda m: " " * len(m.group(0)), line)
    return sorted(term_hits(masked, False) + code_hits(masked, set()))


# ---- sources -------------------------------------------------------------------------------------------------------

RS_START = re.compile(r"//|/\*|b?r#*\"|\"|'")
PY_START = re.compile(r"#|\"\"\"|\'\'\'|\"|'")
CHAR = re.compile(r"'(?:\\.|\\u\{[0-9a-fA-F]+\}|[^\\'\n])'")


def segments(src, rust):
    """(kind, start, end) of comment bodies ('c') and string literal bodies ('s') of a .rs, .py or .toml source.
    Rust strings may span lines; Python and TOML quotes end at the line end unless tripled."""
    i, n = 0, len(src)
    start = RS_START if rust else PY_START
    while True:
        m = start.search(src, i)
        if not m:
            return
        i, tok = m.start(), m.group(0)
        if tok in ("//", "#"):
            j = src.find("\n", i)
            j = n if j < 0 else j
            yield "c", i, j
            i = j
        elif tok == "/*":
            j = src.find("*/", i + 2)
            j = n if j < 0 else j
            yield "c", i + 2, j
            i = j + 2
        elif tok == "'" and rust:
            c = CHAR.match(src, i)
            i = c.end() if c else i + 1
        elif rust and "r" in tok:
            if i > 0 and (src[i - 1].isalnum() or src[i - 1] == "_"):
                i += 1
                continue
            hashes = tok.count("#")
            s = i + len(tok)
            j = src.find('"' + "#" * hashes, s)
            j = n if j < 0 else j
            yield "s", s, j
            i = j + 1 + hashes
        elif tok in ('"""', "'''"):
            j = src.find(tok, i + 3)
            j = n if j < 0 else j
            yield "s", i + 3, j
            i = j + 3
        else:
            q, j = tok[-1], i + 1
            while j < n and src[j] != q and (rust or src[j] != "\n"):
                j += 2 if src[j] == "\\" else 1
            yield "s", i + 1, min(j, n)
            i = j + 1


IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
TOML_KEY = re.compile(r"^\s*([A-Za-z0-9_-]+)\s*=", re.M)


def identifiers(files):
    """Every name used in the code (outside comments and strings) of the given (path, text) sources, and every
    Cargo.toml key: package, dependency and feature names."""
    out = set()
    for path, src in files:
        if path.endswith(".toml"):
            out.update(TOML_KEY.findall(src))
        last = 0
        for _, a, b in segments(src, path.endswith(".rs")):
            out.update(IDENT.findall(src, last, a))
            last = b
        out.update(IDENT.findall(src, last))
    return out


def source_hits(path, src, idents):
    """(line, kind, text) of every hit in the comments and string literals of one source file."""
    out = []
    for kind, a, b in segments(src, path.endswith(".rs")):
        body = src[a:b]
        if kind == "s" and KEY.fullmatch(body.strip()):
            continue
        for pos, name, text in term_hits(body, path.startswith("crates/")) + code_hits(body, idents):
            out.append((src.count("\n", 0, a + pos) + 1, name, text))
    return sorted(out)
