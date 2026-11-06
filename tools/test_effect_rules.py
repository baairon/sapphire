"""Rules every effect follows, checked from the source tree (standard library only).

  python -I tools/test_effect_rules.py

1. Golden rows: every stage of crates/neutral-handoff/data/fx_stages.tsv has rows in tools/fx_golden.tsv in the plain group,
   the special group (size suffix `s`) and the threaded group (suffix `t`), and each threaded row has a plain row of the same size.
2. No platform math: crates/effects calls no platform math library function (`.ln()`, `.sin()`, `.powf()`, `f32::exp` and the
   like) outside fx_eval's detmath module and test code. An exception is a row of tools/fx_math_allow.tsv with a reason.
   `sqrt`, `abs`, `floor`, `ceil`, `round`, `trunc`, `fract` and `mul_add` are exact IEEE operations and are allowed.
"""
import os
import re
import sys
import tempfile
import unittest

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
STAGES = os.path.join("crates", "neutral-handoff", "data", "fx_stages.tsv")
GOLDEN = os.path.join("tools", "fx_golden.tsv")
ALLOW = os.path.join("tools", "fx_math_allow.tsv")
DETMATH = "crates/effects/fx_eval/src/detmath.rs"

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
            d = os.path.join(t, "crates/effects/crates/x/src")
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
