"""Rules every effect and the repository tree follow, checked from the source tree (standard library only).

  python -I tools/test_effect_rules.py

1. Golden rows: every stage of crates/neutral-handoff/data/fx_stages.tsv has rows in tools/fx_golden.tsv in the plain group,
   the special group (size suffix `s`) and the threaded group (suffix `t`), and each threaded row has a plain row of the same size.
2. No platform math: crates/effects calls no platform math library function (`.ln()`, `.sin()`, `.powf()`, `f32::exp` and the
   like) outside fx_eval's detmath module and test code. An exception is a row of tools/fx_math_allow.tsv with a reason.
   `sqrt`, `abs`, `floor`, `ceil`, `round`, `trunc`, `fract` and `mul_add` are exact IEEE operations and are allowed.
3. Plain technical text: the comments and string literals of crates/ and tools/ hold none of the terms of
   tools/process_terms.py (words and codes that say how the code was made rather than what it does).
4. Crate layout: every crate under crates/effects is an engine crate in its fixed folder or a kernel crate in
   crates/effects/kernels/<name>, whose package and library name is <name>, and no crate name holds a work label.
5. No stray files: no editor or patch backup (*.orig, *.rej, *~, *.bak, *.swp) and no .pyc outside __pycache__ anywhere in the
   tree outside the ignored build, cache and data folders.
An exception to 3 or 4 is a row of tools/process_allow.tsv (path, text, reason); a row without a reason or a row that matches
nothing fails the test.
"""
import os
import re
import sys
import tempfile
import unittest

TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, TOOLS)
import process_terms  # noqa: E402

ROOT = os.path.dirname(TOOLS)
STAGES = os.path.join("crates", "neutral-handoff", "data", "fx_stages.tsv")
GOLDEN = os.path.join("tools", "fx_golden.tsv")
ALLOW = os.path.join("tools", "fx_math_allow.tsv")
DETMATH = "crates/effects/fx_eval/src/detmath.rs"
PROCESS_ALLOW = os.path.join("tools", "process_allow.tsv")
# Files the process scan skips: the term list itself, and the files shared with twixtor, which this repository never edits.
PROCESS_SKIP = {"tools/process_terms.py", "tools/conformance.py", "tools/test_conformance.py", "tools/handoff.py",
                "tools/test_handoff.py"}
SKIP_DIRS = {"target", "__pycache__", "vectors", ".git", ".agents", "handoff", "original", "node_modules"}
# Engine and support crates: folder under crates/effects -> package name. Every other effect crate is a kernel.
ENGINE = {"fx_common": "fx_common", "fx_device": "fx_device", "fx_eval": "fx_eval", "fx_exec": "fx_exec",
          "rtdata": "effects_rtdata", "vecdir": "effects_vecdir"}
KERNELS = "kernels"
BAD_NAME = re.compile(r"wave_|rebuild|(?:^|_)(?:ports?|step\d*|batch\d*|w\d{3,4}|[a-z]{2,5}\d{1,3}[a-z]?)(?=_|$)")
STRAY = re.compile(r"(?:\.orig|\.rej|~|\.bak|\.swp|\.pyc)$")

FUNCS = ("ln log2 log10 log exp exp2 exp_m1 ln_1p powf powi sin cos tan asin acos atan atan2 sinh cosh tanh asinh acosh atanh "
         "cbrt hypot sin_cos").split()
METHOD = re.compile(r"\.(%s)\s*\(" % "|".join(FUNCS))
PATH = re.compile(r"\bf(?:32|64)::(%s)\b" % "|".join(FUNCS))


def table(root, rel):
    with open(os.path.join(root, rel), encoding="utf-8") as f:
        lines = [l.rstrip("\n").split("\t") for l in f if l.strip()]
    return lines[0], lines[1:]


def golden_problems(root):
    _, stages = table(root, STAGES)
    _, rows = table(root, GOLDEN)
    sizes = {}
    for r in rows:
        sizes.setdefault(r[0], set()).add(r[1])
    out = []
    for st in (s[0] for s in stages):
        have = sizes.get(st, set())
        plain = {s for s in have if s[-1].isdigit()}
        special = {s for s in have if s.endswith("s")}
        threaded = {s for s in have if s.endswith("t")}
        for name, grp in (("plain", plain), ("special (s)", special), ("threaded (t)", threaded)):
            if not grp:
                out.append("%s: no %s golden row in %s" % (st, name, GOLDEN))
        for t in threaded:
            if t[:-1] not in plain:
                out.append("%s: threaded row %s has no plain row %s" % (st, t, t[:-1]))
    return out


def strip_code(text):
    """Source without comments, string literals and `#[cfg(test)]` items."""
    text = re.sub(r"/\*.*?\*/", lambda m: "\n" * m.group(0).count("\n"), text, flags=re.S)
    lines = []
    for l in text.split("\n"):
        l = re.sub(r'"(?:\\.|[^"\\])*"', '""', l)
        lines.append(re.sub(r"//.*", "", l))
    out, i = [], 0
    while i < len(lines):
        if re.match(r"\s*#\[cfg\(test\)\]", lines[i]):
            depth, started, j = 0, False, i + 1
            while j < len(lines):
                depth += lines[j].count("{") - lines[j].count("}")
                started = started or "{" in lines[j]
                if (started and depth <= 0) or (not started and lines[j].rstrip().endswith(";")):
                    break
                j += 1
            out.extend([""] * (j - i + 1))
            i = j + 1
        else:
            out.append(lines[i])
            i += 1
    return out


def math_problems(root):
    _, allow = table(root, ALLOW)
    problems = [ALLOW + ": entry without a reason: " + "\t".join(a) for a in allow if len(a) < 3 or not a[2].strip()]
    allowed = {(a[0], a[1]) for a in allow if len(a) >= 3 and a[2].strip()}
    used = set()
    base = os.path.join(root, "crates", "effects")
    for d, ds, fs in os.walk(base):
        ds[:] = [x for x in ds if x not in ("target", "tests", "benches", "examples", "__pycache__")]
        for f in fs:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(d, f)
            rel = os.path.relpath(p, root).replace(os.sep, "/")
            if rel == DETMATH:
                continue
            with open(p, encoding="utf-8", errors="replace") as fh:
                code = strip_code(fh.read())
            for n, l in enumerate(code, 1):
                for m in list(METHOD.finditer(l)) + list(PATH.finditer(l)):
                    hit = next((a for a in allowed if a[0] == rel and a[1] in l), None)
                    if hit:
                        used.add(hit)
                    else:
                        problems.append("%s:%d: platform math call `%s` (use fx_eval::detmath or exact integer code)"
                                        % (rel, n, m.group(0).strip()))
    problems += [ALLOW + ": entry matches nothing: %s %s" % a for a in sorted(allowed - used)]
    return problems


# ---- 3. plain technical text --------------------------------------------------------------------------------------------

def allow_rows(root):
    """(rows with a reason, problems) of tools/process_allow.tsv."""
    if not os.path.exists(os.path.join(root, PROCESS_ALLOW)):
        return [], []
    _, rows = table(root, PROCESS_ALLOW)
    bad = [PROCESS_ALLOW + ": entry without a reason: " + "\t".join(r) for r in rows if len(r) < 3 or not r[2].strip()]
    return [(r[0], r[1]) for r in rows if len(r) >= 3 and r[2].strip()], bad


def sources(root):
    out = []
    for top in ("crates", "tools"):
        for d, ds, fs in os.walk(os.path.join(root, top)):
            ds[:] = sorted(x for x in ds if x not in SKIP_DIRS)
            for f in sorted(fs):
                if f.endswith((".rs", ".py")) or f == "Cargo.toml":
                    rel = os.path.relpath(os.path.join(d, f), root).replace(os.sep, "/")
                    if rel not in PROCESS_SKIP:
                        with open(os.path.join(root, rel), encoding="utf-8", errors="replace") as h:
                            out.append((rel, h.read()))
    return out


def process_problems(root, allowed, used):
    files = sources(root)
    idents = process_terms.identifiers(files)
    out = []
    for rel, src in files:
        lines = src.split("\n")
        for n, kind, text in process_terms.source_hits(rel, src, idents):
            hit = next((a for a in allowed if a[0] == rel and a[1] in lines[n - 1]), None)
            if hit:
                used.add(hit)
            else:
                out.append("%s:%d: %s `%s` (state what the code does; see docs/adding-an-effect.md)" % (rel, n, kind, text))
    return out


# ---- 4. crate layout ------------------------------------------------------------------------------------------------------

def manifest_names(path):
    """(package name, lib name or None) of a Cargo.toml."""
    section, names = "", {}
    with open(path, encoding="utf-8") as h:
        for line in h:
            m = re.match(r"\s*\[([^\]]+)\]", line)
            if m:
                section = m.group(1).strip()
            m = re.match(r'\s*name\s*=\s*"([^"]+)"', line)
            if m and section in ("package", "lib") and section not in names:
                names[section] = m.group(1)
    return names.get("package"), names.get("lib")


def layout_problems(root, allowed, used):
    base = os.path.join(root, "crates", "effects")
    out = []
    if not os.path.isdir(base):
        return out
    for d, ds, fs in os.walk(base):
        ds[:] = sorted(x for x in ds if x not in SKIP_DIRS)
        rel = os.path.relpath(d, root).replace(os.sep, "/")
        parts = rel.split("/")[2:]
        if len(parts) == 1 and parts[0] not in ENGINE and parts[0] != KERNELS:
            out.append("%s: crates/effects holds only %s and %s/" % (rel, ", ".join(sorted(ENGINE)), KERNELS))
            ds[:] = []
            continue
        if len(parts) == 2 and parts[0] == KERNELS and "Cargo.toml" not in fs:
            out.append("%s: a folder in %s/ is a crate (no Cargo.toml)" % (rel, KERNELS))
        if "Cargo.toml" not in fs:
            continue
        manifest = rel + "/Cargo.toml"
        package, lib = manifest_names(os.path.join(d, "Cargo.toml"))
        if len(parts) == 1 and parts[0] in ENGINE:
            if package != ENGINE[parts[0]]:
                out.append("%s: the package of crates/effects/%s is %s" % (manifest, parts[0], ENGINE[parts[0]]))
            continue
        if not (len(parts) == 2 and parts[0] == KERNELS):
            out.append("%s: a kernel crate lives in crates/effects/%s/<name>, one level deep" % (manifest, KERNELS))
            continue
        name = parts[1]
        if package != name or (lib is not None and lib != name):
            out.append("%s: package %s and lib %s must both equal the folder name %s" % (manifest, package, lib, name))
        for n in {name, package or "", lib or ""} - {""}:
            if BAD_NAME.search(n):
                hit = next((a for a in allowed if a[0] == manifest and a[1] == n), None)
                if hit:
                    used.add(hit)
                else:
                    out.append("%s: the name %s holds a work label (wave, rebuild, port, step, batch, a code or a W-number);"
                               " name the crate for what it computes" % (manifest, n))
    return out


# ---- 5. stray files -------------------------------------------------------------------------------------------------------

def stray_problems(root):
    out = []
    for d, ds, fs in os.walk(root):
        ds[:] = sorted(x for x in ds if x not in SKIP_DIRS)
        for f in sorted(fs):
            if STRAY.search(f):
                out.append("%s: stray file (backup or bytecode)" % os.path.relpath(os.path.join(d, f), root).replace(os.sep, "/"))
    return out


def tree_problems(root):
    """Problems of rules 3 and 4 plus the allowlist rows that lack a reason or match nothing."""
    allowed, out = allow_rows(root)
    used = set()
    out += process_problems(root, allowed, used) + layout_problems(root, allowed, used)
    out += [PROCESS_ALLOW + ": entry matches nothing: %s %s" % a for a in sorted(set(allowed) - used)]
    return out


def write(root, rel, text):
    path = os.path.join(root, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as h:
        h.write(text)


def kernel(root, folder, package):
    write(root, "crates/effects/kernels/%s/Cargo.toml" % folder, '[package]\nname = "%s"\nversion = "0.0.0"\n' % package)
    write(root, "crates/effects/kernels/%s/src/lib.rs" % folder, "//! Adds two planes.\n")


class TreeRules(unittest.TestCase):
    def test_the_tree_is_clean(self):
        self.assertEqual(tree_problems(ROOT), [])

    def test_no_stray_files(self):
        self.assertEqual(stray_problems(ROOT), [])

    def test_planted_process_terms_fail(self):
        with tempfile.TemporaryDirectory() as t:
            kernel(t, "add", "add")
            write(t, "crates/effects/kernels/add/src/x.rs",
                  "// built in " + "wa" + "ve 9 by the " + "fle" + "et, see " + "SP" + "EC.md\n"
                  "pub const A: &str = \"done at " + "st" + "ep DX" + "2\";\n"
                  "pub const B: &str = \"rebuild" + "_x/y\";\n")
            write(t, "tools/process_allow.tsv", "path\ttext\treason\n")
            got = tree_problems(t)
            self.assertEqual(sorted(p.split("`")[1] for p in got), ["DX2", "SP" + "EC.md", "fle" + "et", "st" + "ep DX" + "2", "wa" + "ve 9"], got)

    def test_planted_layout_fails(self):
        with tempfile.TemporaryDirectory() as t:
            kernel(t, "add", "add")
            kernel(t, "wave" + "_t1_glow", "wave" + "_t1_glow")
            kernel(t, "es" + "h9_job", "es" + "h9_job")
            kernel(t, "blur_x", "blur_y")
            write(t, "crates/effects/" + "ext/x/Cargo.toml", '[package]\nname = "x"\n')
            write(t, "crates/effects/kernels/add/port/Cargo.toml", '[package]\nname = "add_port"\n')
            write(t, "tools/process_allow.tsv", "path\ttext\treason\n")
            got = layout_problems(t, [], set())
            self.assertEqual(len(got), 5, got)

    def test_allowlist_rows_need_a_reason_and_a_match(self):
        with tempfile.TemporaryDirectory() as t:
            kernel(t, "add", "add")
            write(t, "tools/process_allow.tsv", "path\ttext\treason\na.rs\tx\t\nb.rs\ty\tsome reason\n")
            got = tree_problems(t)
            self.assertTrue(any("without a reason" in p for p in got), got)
            self.assertTrue(any("matches nothing" in p for p in got), got)

    def test_planted_stray_files_fail(self):
        with tempfile.TemporaryDirectory() as t:
            for rel in ("crates/a/src/lib.rs.orig", "tools/x.py.rej", "docs/a.md~", "README.md.bak", "tools/.x.swp",
                        "tools/x.pyc", "tools/__pycache__/x.cpython-312.pyc", "target/debug/x.orig", "crates/a/src/lib.rs"):
                write(t, rel, "x\n")
            got = stray_problems(t)
            self.assertEqual(len(got), 6, got)


class Rules(unittest.TestCase):
    def test_every_stage_has_golden_rows(self):
        self.assertEqual(golden_problems(ROOT), [])

    def test_golden_check_fails_without_a_group(self):
        with tempfile.TemporaryDirectory() as t:
            os.makedirs(os.path.join(t, "crates/neutral-handoff/data"))
            os.makedirs(os.path.join(t, "tools"))
            with open(os.path.join(t, STAGES), "w") as f:
                f.write("stage_id\nfoo\n")
            with open(os.path.join(t, GOLDEN), "w") as f:
                f.write("stage\tsize\nfoo\t8x8\nfoo\t8x8t\n")
            self.assertEqual(len(golden_problems(t)), 1)

    def test_no_platform_math(self):
        self.assertEqual(math_problems(ROOT), [])

    def test_planted_call_fails(self):
        with tempfile.TemporaryDirectory() as t:
            d = os.path.join(t, "crates/effects/kernels/x/src")
            os.makedirs(d)
            os.makedirs(os.path.join(t, "tools"))
            with open(os.path.join(t, ALLOW), "w") as f:
                f.write("path\ttext\treason\n")
            with open(os.path.join(d, "lib.rs"), "w") as f:
                f.write("pub fn f(x: f32) -> f32 {\n    x.sqrt() + (x as f64).ln() as f32 + f32::exp(x)\n}\n"
                        "#[cfg(test)]\nmod tests {\n    fn g(x: f32) -> f32 { x.sin() }\n}\n")
            got = math_problems(t)
            self.assertEqual(len(got), 2, got)

    def test_allowlist_entry_needs_a_reason(self):
        with tempfile.TemporaryDirectory() as t:
            os.makedirs(os.path.join(t, "crates/effects"))
            os.makedirs(os.path.join(t, "tools"))
            with open(os.path.join(t, ALLOW), "w") as f:
                f.write("path\ttext\treason\na.rs\tx.sin()\t\n")
            self.assertTrue(any("without a reason" in p for p in math_problems(t)))


if __name__ == "__main__":
    unittest.main()
