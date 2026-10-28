"""Tests for tools/gen_docs.py, tools/docgen/, tools/fn_info.py and tools/check_docs.py.

Run: python -I tools/test_docgen.py   (standard library unittest, no skips)
"""

import contextlib
import io
import json
import os
import re
import shutil
import sys
import tempfile
import unittest

TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, TOOLS)

import check_docs  # noqa: E402
import fn_info  # noqa: E402
import gen_docs  # noqa: E402
from docgen import archive, render  # noqa: E402
from docgen.data import ROOT, Data, read_text, read_tsv  # noqa: E402

DATA = os.path.join(ROOT, "docs", "data")
STATE = {}


def setUpModule():
    out, stale, unknown, data = render.build()
    STATE.update(out=out, stale=stale, unknown=unknown, data=data)


def spans(row):
    """Code span contents of one table row, verbatim."""
    out = []
    for m in re.finditer(r"``\s(.+?)\s``|`([^`]*)`", row):
        out.append((m.group(1) if m.group(1) is not None else m.group(2)).replace("\\|", "|"))
    return out


def generated(prefix):
    return [STATE["out"][p] for p in sorted(STATE["out"]) if p.startswith("docs/generated/" + prefix)]


class FactsTest(unittest.TestCase):
    def setUp(self):
        self.f = {k: v["value"] for k, v in STATE["data"].facts.items()}

    def test_parity_figures_match_parity_md(self):
        text = read_text(os.path.join(DATA, "PARITY.md"))
        m = re.search(r"\*\*strict total\*\* \| (\d+) \| (\d+) \| ([\d.]+)%", text)
        self.assertEqual((self.f["strict_fns"], self.f["strict_bytes"], self.f["strict_pct"]),
                         (int(m.group(1)), int(m.group(2)), float(m.group(3))))
        m = re.search(r"\*\*compared-worlds exact total\*\* \| (\d+) \| (\d+) \| ([\d.]+)%", text)
        self.assertEqual((self.f["strictest_fns"], self.f["strictest_pct"]), (int(m.group(1)), float(m.group(3))))
        self.assertEqual(self.f["unproven_fns"] + self.f["strict_fns"], self.f["fn_total"])
        self.assertEqual(len(self.f["unproven_offsets"]), self.f["unproven_fns"])

    def test_name_check_matches_summary(self):
        text = read_text(os.path.join(DATA, "namecheck_summary.md"))
        m = re.search(r"([\d,]+) of [\d,]+ functions have a verdict: ([\d,]+) align \((\d+) high, (\d+) medium, "
                      r"(\d+) low confidence\), (\d+) weak, (\d+) mismatch", text)
        want = [int(g.replace(",", "")) for g in m.groups()]
        got = [self.f[k] for k in ("nc_checked", "nc_align", "nc_align_high", "nc_align_med", "nc_align_low",
                                   "nc_weak", "nc_mismatch")]
        self.assertEqual(got, want)
        m = re.search(r"(\d+) checked, (\d+) align, (\d+) weak, (\d+) mismatch", text)
        self.assertEqual([self.f[k] for k in ("stubs_total", "stub_align", "stub_weak", "stub_mismatch")],
                         [int(g) for g in m.groups()])
        m = re.search(r"(\d+) functions sit in byte identical twin sets.*?(\d+) constant return", text)
        self.assertEqual((self.f["twin_fns"], self.f["const_result_fns"]), (int(m.group(1)), int(m.group(2))))

    def test_names_and_origins(self):
        self.assertEqual(self.f["names_ids_agree"], self.f["fn_total"])
        self.assertEqual(sum(self.f[k] for k in ("origin_bundled", "origin_std", "origin_undetermined",
                                                 "origin_component")), self.f["names_total"])
        self.assertEqual(self.f["effects_total"], len(read_tsv(os.path.join(DATA, "effect_names.tsv"))))

    def test_every_function_has_a_source_line(self):
        rows = STATE["data"].rows
        self.assertEqual(len(rows), self.f["fn_total"])
        self.assertEqual([r["id"] for r in rows if not r["line"]], [])
        self.assertEqual(self.f["tests_scan_total"],
                         sum(c["tests"] for c in STATE["data"].crates.values()))

    def test_aliases_resolve(self):
        d = STATE["data"]
        for alias, key in d.aliases.items():
            self.assertIs(d.fact(alias), d.facts[key])

    def test_lift_definitions(self):
        from docgen.data import DEF_RE
        for line in ("pub fn f_0000abcd(m: &mut M) -> Ret {", "fn f_0000abcd()", "    pub(crate) unsafe fn f_0000abcd()"):
            self.assertEqual(DEF_RE.match(line).group(1), "f_0000abcd", line)
        for line in ("    f_0000abcd(m, r);", "// pub fn f_0000abcd()", "/// fn f_0000abcd", "fn f_0000abcd_rec<E>()",
                     "let x = fn f_0000abcd;"):
            self.assertIsNone(DEF_RE.match(line), line)
        defs = {}
        base = os.path.join(ROOT, "crates")
        for crate in sorted(c for c in os.listdir(base) if c.startswith("lift-")):
            for dirpath, _, files in os.walk(os.path.join(base, crate, "src")):
                for name in files:
                    if name.endswith(".rs"):
                        text = read_text(os.path.join(dirpath, name))
                        for m in re.finditer(r"^[ \t]*(?:pub[ \t]+)?fn[ \t]+(f_[0-9a-f]{8})\b", text, re.M):
                            defs[m.group(1)] = defs.get(m.group(1), 0) + 1
        self.assertEqual(self.f["lift_defs_total"], sum(defs.values()))
        self.assertEqual(self.f["lift_ids_multi"], sum(1 for n in defs.values() if n > 1))
        self.assertEqual(set(defs), {r["id"] for r in STATE["data"].rows})


class CloseRowsTest(unittest.TestCase):
    """docs/data/audit_close.tsv and docs/data/close_sets.tsv against the map and the ledger."""

    def setUp(self):
        self.d = STATE["data"]
        self.by = {r["id"]: r for r in self.d.rows}
        self.ledger = {l.split("\t")[0]: l.split("\t") for l in self.d.ledger_lines if l}

    def test_rows_agree_with_map_ledger_and_sets(self):
        self.assertEqual(len(self.d.audit_close), 7)
        for r in self.d.audit_close:
            row, led = self.by[r["symbol"]], self.ledger[r["symbol"]]
            self.assertEqual((row["tier"], row["status"], row["strict"]), ("P2v", "done", "exact"), r["symbol"])
            self.assertEqual(row["evidence"], "audit2" if r["labels"] == "-" else "assumed", r["symbol"])
            self.assertEqual(led[3:7], ["P2v", "done", "exact", row["evidence"]], r["symbol"])
            self.assertEqual(led[7], "{}/{}".format(r["compared"], r["worlds"]), r["symbol"])
            self.assertGreaterEqual(int(r["compared"]), 900)
            self.assertEqual(int(r["seen"]) + int(r["explained"]), int(r["outcomes"]), r["symbol"])
            sets = [s for s in self.d.close_sets if s["function"] == r["symbol"]]
            self.assertEqual(sum(int(s["compared"]) for s in sets), int(r["compared"]), r["symbol"])
            self.assertEqual(sum(int(s["cases"]) for s in sets), int(r["worlds"]), r["symbol"])
            for label in r["labels"].split(","):
                if label != "-":
                    self.assertIn(label, led[8], r["symbol"])
                    self.assertIn(label + "=", r["label_outcomes"], r["symbol"])

    def test_counts_and_standards(self):
        f = {k: v["value"] for k, v in self.d.facts.items()}
        self.assertEqual(f["close_labelled_rows"], f["ev_assumed_fns"])
        self.assertEqual(f["close_pass_rows"], f["ev_audit2_fns"])
        self.assertEqual(f["close_pass_rows"] + f["close_labelled_rows"], f["close_rows"])
        strictest = sum(1 for r in self.d.fm if r["evidence"] in ("p1", "second", "owner", "audit2"))
        self.assertEqual(f["strictest_fns"], strictest)
        self.assertEqual(f["strict_fns"] - f["strictest_fns"], f["ev_anyworld_fns"] + f["ev_assumed_fns"])
        self.assertTrue(all(len(s["vec_sha256"]) == 64 for s in self.d.close_sets))
        self.assertEqual(f["close_sets_pins"], sum(int(s["pins"]) for s in self.d.close_sets))


class PagesTest(unittest.TestCase):
    @staticmethod
    def column_bytes(name):
        """First column of a bank table, read as raw bytes, one value per line (header dropped)."""
        with open(os.path.join(DATA, name), "rb") as h:
            raw = h.read()
        return b"\n".join(l.split(b"\t")[0] for l in raw.split(b"\n")[1:] if l)

    @staticmethod
    def block_bytes(page):
        m = re.search(r"\n```text\n(.*?)\n```\n", page, re.S)
        return m.group(1).encode("utf-8")

    def test_effects_verbatim(self):
        page = STATE["out"]["docs/generated/effects.md"]
        self.assertEqual(self.block_bytes(page), self.column_bytes("effect_names.tsv"))
        self.assertEqual(len(self.block_bytes(page).split(b"\n")), STATE["data"].facts["effects_total"]["value"])
        self.assertEqual(gen_docs.disk(ROOT, "docs/generated/effects.md"), page)
        self.assertIn("verbatim and in bank order", page)
        self.assertIn("No effect is mapped to a function", page)

    def test_parameters_verbatim(self):
        page = STATE["out"]["docs/generated/parameters.md"]
        self.assertEqual(self.block_bytes(page), self.column_bytes("param_candidates.tsv"))
        self.assertIn("not confirmed parameters", page)

    def test_no_effect_name_beside_a_function(self):
        names = {r["name"] for r in read_tsv(os.path.join(DATA, "effect_names.tsv"))}
        for rel, text in STATE["out"].items():
            if rel == "docs/generated/effects.md":
                continue
            if rel.startswith(("docs/generated/", "docs/index/")):
                found = set(re.findall(r"(?<![\w-])S_[A-Za-z0-9_]*[A-Za-z0-9]", text)) & names
                self.assertEqual(found, set(), rel)

    def test_function_pages(self):
        pages = generated("functions")
        self.assertGreater(len(pages), 1)
        ids, names = [], []
        for page in pages:
            self.assertLessEqual(len(page.encode("utf-8")), render.PAGE_LIMIT)
            for line in page.split("\n"):
                m = re.match(r"\| 0x[0-9a-f]+ \| (.+?) \| (f_[0-9a-f]{8}) \|", line)
                if m:
                    ids.append(m.group(2))
                    names.append(spans(m.group(1))[0])
        fm = read_tsv(os.path.join(DATA, "FUNCTION_MAP.tsv"))
        bank = {r["symbol"]: r["exact_name"] for r in read_tsv(os.path.join(DATA, "names.tsv"))}
        self.assertEqual(ids, [r["name"] for r in fm])
        self.assertEqual(names, [bank[i] for i in ids])

    def test_page_statements(self):
        pages = generated("functions")
        self.assertIn("97.2%", pages[0])
        self.assertIn("about 68%", pages[0])
        self.assertIn("bank name", pages[0])
        for page in pages[1:]:
            self.assertNotIn("97.2%", page)
        names = STATE["out"]["docs/generated/names.md"]
        self.assertIn("name check", names)
        self.assertIn("confirmation", names)

    def test_generated_pages_notice_and_no_source_lines(self):
        for rel, text in STATE["out"].items():
            if rel.startswith("docs/generated/"):
                self.assertTrue(text.startswith(render.DO_NOT_EDIT + "\n# "), rel)
                self.assertEqual(text.count("do not edit"), 1, rel)
                self.assertIsNone(re.search(r"^Source:", text, re.M), rel)
        gen = render.Generator(STATE["data"])
        gen.pages()
        for key, lines in gen.block_cache.items():
            self.assertFalse(any(l.startswith("Source:") for l in lines), key)

    def test_lift_crates_block(self):
        gen = render.Generator(STATE["data"])
        gen.pages()
        self.assertNotIn("owners", gen.block_cache)
        got = {}
        for line in gen.block_cache["lift_crates"][2:]:
            cells = [c.strip() for c in line.strip("|").split("|")]
            got[spans(cells[0])[0]] = (int(cells[1].replace(",", "")), int(cells[2].replace(",", "")))
        want = {}
        for r in read_tsv(os.path.join(DATA, "FUNCTION_MAP.tsv")):
            n, b = want.get(r["source"].split("/")[1], (0, 0))
            want[r["source"].split("/")[1]] = (n + 1, b + int(r["size"]))
        self.assertEqual(got, want)
        self.assertEqual(list(got), sorted(got))
        self.assertEqual(sum(n for n, _ in got.values()), STATE["data"].facts["fn_total"]["value"])

    def test_functions_jsonl(self):
        rows = [json.loads(l) for l in STATE["out"]["docs/index/functions.jsonl"].split("\n") if l]
        self.assertEqual(len(rows), STATE["data"].facts["fn_total"]["value"])
        self.assertEqual(len({r["id"] for r in rows}), len(rows))
        self.assertTrue(all(r["page"] in STATE["out"] for r in rows))

    def test_pages_json_covers_every_md(self):
        entries = json.loads(STATE["out"]["docs/index/pages.json"])
        paths = {e["path"] for e in entries}
        want = (set(render.md_files(ROOT)) - set(STATE["stale"])) | {p for p in STATE["out"] if p.endswith(".md")}
        self.assertEqual(paths, want)
        for e in entries:
            self.assertEqual(set(e), {"path", "title", "line", "bytes", "generated"})
            self.assertEqual(e["generated"], e["path"].startswith("docs/generated/"))

    def test_generated_files_match_disk(self):
        for rel, text in STATE["out"].items():
            if rel.startswith("docs/generated/") or rel in ("docs/index/facts.json", "docs/index/functions.jsonl"):
                self.assertEqual(gen_docs.disk(ROOT, rel), text, rel)

    def test_deterministic(self):
        out2 = render.build()[0]
        for rel in STATE["out"]:
            if rel.startswith(("docs/generated/", "docs/index/f")):
                self.assertEqual(out2[rel], STATE["out"][rel], rel)

    def test_ascii_and_no_dashes(self):
        for rel, text in STATE["out"].items():
            self.assertTrue(all(ord(c) < 128 for c in text), rel)
        for rel in ("gen_docs.py", "check_docs.py", "fn_info.py", "test_docgen.py", "docgen/data.py",
                    "docgen/render.py", "docgen/archive.py", "docgen/__init__.py"):
            text = read_text(os.path.join(TOOLS, rel))
            self.assertTrue(all(ord(c) < 128 for c in text), rel)


class ConfirmationTest(unittest.TestCase):
    def test_levels_follow_the_table(self):
        rows = STATE["data"].rows
        levels = {r["symbol"]: r["level"] for r in read_tsv(os.path.join(DATA, "name_confirmation.tsv"))}
        self.assertEqual({r["id"]: r["confirmation"] for r in rows},
                         {r["id"]: levels.get(r["id"], "missing") for r in rows})

    def test_confirmation_table(self):
        import confirm_names
        rows = read_tsv(os.path.join(DATA, "name_confirmation.tsv"))
        fm = read_tsv(os.path.join(DATA, "FUNCTION_MAP.tsv"))
        names = {r["symbol"]: r["exact_name"] for r in read_tsv(os.path.join(DATA, "names.tsv"))}
        self.assertEqual([r["symbol"] for r in rows], [r["name"] for r in fm])
        self.assertEqual({r["symbol"]: r["name"] for r in rows}, names)
        self.assertTrue({r["level"] for r in rows} <= set(confirm_names.LEVELS))
        counts = {k: sum(r["level"] == k for r in rows) for k in confirm_names.LEVELS}
        facts = STATE["data"].facts
        self.assertEqual({k: facts["conf_" + k]["value"] for k in confirm_names.LEVELS}, counts)
        self.assertEqual(sum(counts.values()), facts["fn_total"]["value"])
        fresh = confirm_names.render(confirm_names.Confirm().rows())
        self.assertEqual(gen_docs.disk(ROOT, "docs/data/name_confirmation.tsv"), fresh)
        for r in rows:
            if r["level"] == "confirmed":
                ids = [e for e in r["evidence_ids"].split(",") if e not in ("know", "-")]
                self.assertGreaterEqual(len(ids), 2, r["symbol"])

    def test_overrides_applied_and_validated(self):
        import confirm_names
        c = confirm_names.Confirm()
        c.rows()
        table = {r["symbol"]: [r[k] for k in confirm_names.HEADER]
                 for r in read_tsv(os.path.join(DATA, "name_confirmation.tsv"))}
        over = c.overrides()
        self.assertEqual(sorted(set(rel for rel, _ in over)), confirm_names.OVERRIDES)
        acts = {sym: act for _, sym, act, _ in c.report}
        self.assertEqual(len(acts), len(over))
        audited = {r[0] for r in c.audit_rows()}
        for _, row in over:
            if row[0] in audited:
                continue
            if acts[row[0]] == "accepted":
                self.assertEqual(table[row[0]], row)
            elif acts[row[0]] == "downgraded":
                self.assertEqual(table[row[0]][2], "consistent")
            else:
                self.assertNotEqual(table[row[0]], row)
        sym, name = over[0][1][0], over[0][1][1]
        weak = [sym, name, "weak", "-", ""]
        good = [sym, name, "confirmed", "regs,pair", "test row"]
        self.assertEqual(c.check_override(good, weak), (good, None))
        rejected = [
            ([sym, name + " x", "consistent", "calls", "test"], weak),
            (good, [sym, name, "twin", "-", ""]),
            (good, [sym, name, "stub", "-", ""]),
            ([sym, name, "stub", "-", "nothing here"], weak),
            ([sym, name, "stub", "-", "constant return"], [sym, name, "consistent", "calls", ""]),
            ([sym, name, "consistent", "calls", "at crates/no-such/src/x.rs:1"], weak),
            ([sym, name, "consistent", "calls", "a " + chr(0x2014) + " b"], weak),
            ([sym, name, "consistent", "guess", "test"], weak),
            ([sym, name, "twin", "-", "test"], weak),
        ]
        for row, computed in rejected:
            self.assertIsNone(c.check_override(row, computed)[0], row)
        self.assertIsNone(c.check_override([sym, name, "stub", "-", "constant return body"], weak)[1])
        for ids, text in (("calls,know", "test"), ("arity,regs", "enum widths, SDK knowledge"),
                          ("regs,consts", "the NTV2 SDK declares it")):
            down, why = c.check_override([sym, name, "confirmed", ids, text], weak)
            self.assertEqual((down[2], why), ("consistent", "confirmed on fewer than two repository kinds"))
        kept = [sym, name, "confirmed", "regs,arity,consts",
                "reads 2 (enum parameters counted as int registers, SDK knowledge); index table shared"]
        self.assertEqual(c.check_override(kept, weak), (kept, None))
        cited = next(r for _, r in over if confirm_names.CITE.search(r[4]) and acts[r[0]] == "accepted")
        m = confirm_names.CITE.search(cited[4])
        self.assertTrue(c.cite_ok(m.group(1), int(m.group(2))))
        self.assertFalse(c.cite_ok(m.group(1), 10 ** 7))
        self.assertEqual(STATE["data"].facts["conf_decode_dtor_shared"]["value"], 44)

    def test_audit_applied_and_validated(self):
        import confirm_names
        c = confirm_names.Confirm()
        c.rows()
        table = {r["symbol"]: r for r in read_tsv(os.path.join(DATA, "name_confirmation.tsv"))}
        audit = c.audit_rows()
        self.assertEqual((len(audit), sum(r[2] == "confirmed" for r in audit), sum(r[2] == "weak" for r in audit)),
                         (41, 37, 4))
        self.assertEqual([(s, a) for s, a, _ in c.audit_report], [(r[0], "applied") for r in audit])
        for sym, name, old, new, reason in audit:
            self.assertEqual((table[sym]["name"], table[sym]["level"]), (name, new))
            self.assertTrue(table[sym]["evidence"].startswith("audit: " + reason))
        sym, name = audit[0][0], audit[0][1]
        conf = [sym, name, "confirmed", "calls,shape", "old"]
        weak = [sym, name, "weak", "-", ""]
        self.assertEqual(c.check_audit([sym, name, "confirmed", "consistent", "one piece"], conf),
                         ([sym, name, "consistent", "calls,shape", "audit: one piece; earlier: old"], None))
        promo = next(r for r in audit if r[2] == "weak")
        raised = c.check_audit([sym, name, "weak", "consistent", promo[4]], weak)[0]
        self.assertEqual(raised[2:4], ["consistent", promo[4].split(":")[0]])
        rejected = [
            ([sym, name + " x", "confirmed", "consistent", "t"], conf),
            ([sym, name, "weak", "consistent", "t"], conf),
            ([sym, name, "confirmed", "twin", "t"], conf),
            ([sym, name, "confirmed", "confirmed", "t"], conf),
            ([sym, name, "confirmed", "consistent", ""], conf),
            ([sym, name, "confirmed", "consistent", "a " + chr(0x2014) + " b"], conf),
            ([sym, name, "confirmed", "consistent", "at crates/no-such/src/x.rs:1"], conf),
            ([sym, name, "confirmed", "consistent", "t"], [sym, name, "stub", "-", ""]),
            ([sym, name, "weak", "consistent", "calls: no cited line"], weak),
            ([sym, name, "weak", "consistent", "know: " + promo[4]], weak),
            ([sym, name, "weak", "consistent", "guess: " + promo[4]], weak),
            ([sym, name, "weak", "confirmed", promo[4]], weak),
        ]
        for row, computed in rejected:
            self.assertIsNone(c.check_audit(row, computed)[0], row)
        know = sum(r["level"] == "consistent" and r["evidence_ids"] == "know" for r in table.values())
        self.assertEqual(STATE["data"].facts["conf_consistent_know_only"]["value"], know)

    def test_regenerates_with_a_table(self):
        tmp = tempfile.mkdtemp()
        try:
            path = os.path.join(tmp, "name_confirmation.tsv")
            lines = ["symbol\tname\tlevel\tevidence_ids\tevidence"]
            for i, r in enumerate(STATE["data"].rows):
                lines.append("\t".join([r["id"], r["bank_name"], "confirmed" if i % 2 else "twin", "e1", "test row"]))
            with open(path, "w", encoding="utf-8", newline="\n") as h:
                h.write("\n".join(lines) + "\n")
            out, _, _, data = render.build(confirmation_path=path)
            self.assertEqual(data.facts["conf_confirmed"]["value"] + data.facts["conf_twin"]["value"],
                             data.facts["fn_total"]["value"])
            for rel, text in out.items():
                if rel.startswith("docs/generated/functions"):
                    self.assertNotIn("| pending |", text)
            gen = render.Generator(data)
            gen.pages()
            filled, unknown = gen.fill("<!--f:conf_twin-->x<!--/f-->")
            self.assertEqual(unknown, [])
            self.assertIn(data.facts["conf_twin"]["text"], filled)
        finally:
            shutil.rmtree(tmp)


class MarkerTest(unittest.TestCase):
    def setUp(self):
        self.gen = render.Generator(STATE["data"])
        self.gen.pages()

    def test_figures_and_aliases(self):
        text, unknown = self.gen.fill("a <!--f:fn_total-->0<!--/f--> b <!--f:strict_fn-->0<!--/f--> "
                                      "c <!--f:no_such_key-->7<!--/f-->")
        self.assertIn("<!--f:fn_total-->2,702<!--/f-->".replace("2,702", STATE["data"].facts["fn_total"]["text"]), text)
        self.assertIn("<!--f:strict_fn-->" + STATE["data"].facts["strict_fns"]["text"] + "<!--/f-->", text)
        self.assertIn("<!--f:no_such_key-->7<!--/f-->", text)
        self.assertEqual(unknown, ["f:no_such_key"])

    def test_blocks_both_closings_and_crlf(self):
        src = "x\r\n<!--gen:tiers-->\r\nold\r\n<!--/gen-->\r\ny\r\n<!--gen:anyworld-->\r\n<!--/gen:anyworld-->\r\n"
        text, unknown = self.gen.fill(src)
        self.assertEqual(unknown, [])
        self.assertNotIn("old", text)
        self.assertIn("| tier |", text)
        self.assertIn("| bank name |", text)
        self.assertNotIn("\n", text.replace("\r\n", ""))
        again, _ = self.gen.fill(text)
        self.assertEqual(again, text)


class FnInfoTest(unittest.TestCase):
    def setUp(self):
        self.rows = [json.loads(l) for l in STATE["out"]["docs/index/functions.jsonl"].split("\n") if l]

    def test_lookup(self):
        r = self.rows[5]
        self.assertEqual(fn_info.find(self.rows, r["id"]), [r])
        self.assertEqual(fn_info.find(self.rows, r["offset"]), [r])
        inside = "0x{:x}".format(int(r["offset"], 16) + 1)
        self.assertEqual(fn_info.find(self.rows, inside), [r])
        self.assertIn(r, fn_info.find(self.rows, r["bank_name"]))
        self.assertIn(r, fn_info.find(self.rows, r["rust_label"]))
        self.assertEqual(fn_info.find(self.rows, "no such name at all"), [])

    def test_main(self):
        r = self.rows[0]
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            self.assertEqual(fn_info.main([r["id"], "--json"]), 0)
        self.assertEqual(json.loads(buf.getvalue())[0]["id"], r["id"])
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            self.assertEqual(fn_info.main([r["id"]]), 0)
        self.assertIn(r["bank_name"], buf.getvalue())
        with contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(fn_info.main(["f_ffffffff"]), 1)


class CheckDocsTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.mkdtemp()
        os.makedirs(os.path.join(self.tmp, "docs", "data"))
        os.makedirs(os.path.join(self.tmp, "tools"))
        with open(os.path.join(self.tmp, "docs", "data", "FUNCTION_MAP.tsv"), "w", encoding="utf-8") as h:
            h.write("offset\tname\tsize\tstrict\n0x10\tf_00000010\t4\texact\n0x1b438\tf_0001b438\t9\tunproven\n")
        with open(os.path.join(self.tmp, "docs", "data", "archive_refs.tsv"), "w", encoding="utf-8") as h:
            h.write("ref\tkind\tmentioned_in\tmentions\nfindings/fleet-8\tdir\tx:1\t1\nscripts/lift.py\tfile\tx:2\t1\n")

    def tearDown(self):
        shutil.rmtree(self.tmp)

    def write(self, rel, text):
        path = os.path.join(self.tmp, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8", newline="\n") as h:
            h.write(text)

    def run_checks(self, *names):
        return check_docs.run(self.tmp, list(names))

    def test_links_and_anchors(self):
        self.write("docs/a.md", "# A\n\n## Some Heading, here\n\n[ok](b.md#target-one) [bad](b.md#nope) "
                                "[gone](c.md) [self](#some-heading-here) [web](https://example.com)\n")
        self.write("docs/b.md", "# B\n\n## Target one\n")
        found = self.run_checks("links")["links"]
        self.assertEqual(sorted(f[3] for f in found), ["missing anchor b.md#nope", "missing target c.md"])

    def test_paths_ids_archive(self):
        self.write("docs/a.md", "# A\n\n`docs/a.md` `tools/missing.py:12` `crates/x/src/*.rs` `f_00000010` "
                                "f_00000099 `archive:findings/fleet-8/SUMMARY.md` `archive:scripts/other.py`\n"
                                "twixtor `tools/apply.py`\n")
        r = self.run_checks("paths", "ids", "archive")
        self.assertEqual([f[3] for f in r["paths"]], ["missing path tools/missing.py"])
        self.assertEqual([f[3] for f in r["ids"]], ["unknown id f_00000099"])
        self.assertEqual([f[3] for f in r["archive"]], ["unresolved archive:scripts/other.py"])

    def test_paths_listed_as_archived_or_placed(self):
        with open(os.path.join(self.tmp, "docs", "data", "archive_refs.tsv"), "a", encoding="utf-8") as h:
            h.write("crates/vectors\tplaced\tcrates/x/tests/t.rs:3\t1\n")
        self.write("docs/a.md", "# A\n\n`crates/vectors/` `crates/vectors/f_00000010.json` `crates/other/`\n")
        self.assertEqual([f[3] for f in self.run_checks("paths")["paths"]], ["missing path crates/other/"])

    def test_control_characters(self):
        nl = chr(10)
        self.write("docs/a.md", "# A" + nl + nl + "tab" + chr(9) + "ok" + chr(13) + nl + "bad" + chr(1) + "byte" + nl
                   + "del" + chr(127) + nl)
        found = self.run_checks("ascii")["ascii"]
        self.assertEqual([(f[1], f[2], f[3]) for f in found],
                         [("docs/a.md", 4, "control character 0x01"), ("docs/a.md", 5, "control character 0x7f")])

    def test_prose_checks(self):
        dash = chr(0x2014)
        key = "gh" + "p_" + "A" * 36
        self.write("docs/a.md", "# A\n\nWe ran it" + dash + "twice.\n\n`my code` is fine.\n\n"
                                "The f_0001b438 port is proven.\n\nA clean" + " room rewrite.\n\n"
                                "Never write clean" + " room. " + check_docs.ALLOW + "\n\ntoken " + key + "\n\n"
                                "```\nwe are in a fence\n```\n")
        r = self.run_checks("person", "dashes", "claims", "secrets", "ascii")
        self.assertEqual(len(r["person"]), 1)
        self.assertEqual(len(r["dashes"]), 1)
        self.assertEqual(len(r["ascii"]), 1)
        self.assertEqual(sorted(f[3] for f in r["claims"]),
                         ["'proven' next to an unproven function", "forbidden claim: clean room"])
        self.assertEqual([f[3] for f in r["secrets"]], ["github token"])

    def test_ffmpeg_claims(self):
        self.write("docs/a.md", "# A\n\n## One\n\nffmpeg exits 0 on a truncated file.\n\n"
                                "## Two\n\nffmpeg exits 0 on a truncated file (`ffmpeg -v error -i x.mkv -f null -`).\n")
        found = self.run_checks("ffmpeg")["ffmpeg"]
        self.assertEqual([f[2] for f in found], [5])

    def test_exit_and_flag_tables(self):
        schema = {"oneOf": [{"properties": {"code": {"const": "usage"}, "exit_code": {"const": 2}}},
                            {"properties": {"code": {"const": "unsupported"}, "exit_code": {"const": 3}}}],
                  "x-exits": {"usage": 2, "unsupported": 3, "bad_input": 4}}
        self.write("contracts/error.schema.json", json.dumps(schema))
        self.assertEqual(check_docs.schema_exits(os.path.join(self.tmp, "contracts", "error.schema.json")),
                         {"usage": 2, "unsupported": 3, "bad_input": 4})
        self.write("docs/a.md", "# A\n\n| code | exit |\n|---|---|\n| `usage` | 2 |\n| unsupported | 4 |\n"
                                "| nope | 1 |\n\n| flag | meaning |\n|---|---|\n| `--describe` | x |\n")
        r = self.run_checks("exits", "flags")
        self.assertEqual(sorted(f[3] for f in r["exits"]),
                         ["code nope not in the error schema", "code unsupported exits 3 in the schema, 4 here"])
        self.assertEqual([f[3] for f in r["flags"]], ["flag --describe not in --help"])

    def effect_bank(self):
        with open(os.path.join(self.tmp, "docs", "data", "effect_names.tsv"), "w", encoding="utf-8") as h:
            h.write("name\tfile_offset\tvirtual_address\tsection\n")
            for n in ("S_Aurora", "S_AutoPaint", "S_BandPass", "S_Blur", "S_Color"):
                h.write(n + "\t0x1\t0x1\ts\n")
        with open(os.path.join(self.tmp, "docs", "data", "param_candidates.tsv"), "w", encoding="utf-8") as h:
            h.write("candidate\tfile_offset\tvirtual_address\tsection\nblur\t0x1\t0x1\ts\naurora\t0x1\t0x1\ts\n")

    def test_effect_near_misses(self):
        self.effect_bank()
        self.write("docs/a.md", "# A\n\nThe Aurora effect.\n\nA Band Pass pass.\n\nUse `s_aurora`.\n\n"
                                "Use `S_AUTOPAINT`.\n\nUse `S_Auroraa`.\n\nUse `AutoPaint`.\n\nPlain S_Blur here.\n")
        msgs = [(f[2], f[3].split(":")[0]) for f in self.run_checks("effects")["effects"]]
        self.assertEqual(msgs, [(3, "near miss (stem written as an effect name) of S_Aurora"),
                                (5, "near miss (stem written as an effect name) of S_BandPass"),
                                (7, "near miss (case) of S_Aurora"),
                                (9, "near miss (case) of S_AutoPaint"),
                                (11, "near miss (spelling) of S_Aurora"),
                                (13, "near miss (stem written as an effect name) of S_AutoPaint"),
                                (15, "effect name not in code font")])

    def test_effect_not_in_bank(self):
        self.effect_bank()
        self.write("docs/a.md", "# A\n\nThe `S_Nebula` effect.\n\n`S_Aurora` runs as f_00000010 here.\n")
        msgs = [f[3] for f in self.run_checks("effects")["effects"]]
        self.assertEqual(msgs, ["not in the effect bank: S_Nebula", "effect name beside a function id implies a mapping"])

    def test_effect_rule_exemptions(self):
        self.effect_bank()
        self.write("docs/a.md", "# A\n\n`S_Aurora` and `S_BandPass` by name.\n\nThe candidate `blur` and the "
                                "aurora effect parameter.\n\nThe twixtor `retime` effect and `selftest-invert`; "
                                "twixtor Color effect.\n\nA colour matrix, a Color space, a blur radius.\n")
        self.write("docs/generated/effects.md", "# E\n\n```text\nS_Aurora\nS_Blur\n```\n")
        self.assertEqual(self.run_checks("effects")["effects"], [])

    def test_slug(self):
        self.assertEqual(check_docs.slug("Strict parity (900+ worlds)"), "strict-parity-900-worlds")
        self.assertEqual(check_docs.anchors("# T\n\n## Same\n\n## Same\n"), {"t", "same", "same-1"})

    def test_archive_refs_table(self):
        rows = read_tsv(os.path.join(DATA, "archive_refs.tsv"))
        self.assertGreater(len(rows), 0)
        self.assertEqual([r["ref"] for r in rows], sorted({r["ref"] for r in rows}))
        for r in rows:
            self.assertIn(r["kind"], ("file", "dir", "dump", "placed"))
            self.assertTrue(archive.resolves(r["ref"], rows))
        self.assertFalse(archive.resolves("nowhere/at/all.py", rows))


if __name__ == "__main__":
    unittest.main(verbosity=1)
