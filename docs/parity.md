# Parity

How many functions of the dump match the build they replace, and under which standard. The comparison itself is in [harness](harness.md).

## Terms

| Term | Meaning |
|---|---|
| P1 | tier of a self-contained function whose port equals the function it replaces bit for bit on its oracle vectors |
| P2v | tier of a function that is not self-contained, compared with the function it replaces in worlds; status `done` means it passed the world check, `partial` that it did not |
| P2 | tier of a port with the exact control flow and arithmetic whose missing references are placeholders; not compared, so not parity |
| P2-std | tier of a recognised libc++ container, string or smart pointer instantiation mapped to the Rust std type |
| P3 | tier of a function that needs data that cannot be guessed |
| oracle vector | one recorded case of the function a port replaces: arguments, returns and final buffers |
| world | a seeded model of everything the dump lacks, in which the function and its port both run |
| missing reference | data, an import or a constant the code uses that is not in the dump |
| placeholder | the stand-in a port uses for a missing reference: an `Env` access by its `rel_to_end` (`data_va`, `stub_va`) |
| `partial` | status of a row whose comparison did not pass; every `partial` row is unproven |
| strict parity | P1 rows, plus P2v rows with status `done` |
| strictest | strict parity with every branch outcome seen in a compared world or explained under the harness model. Under the revised standard of 2026-11-05 the harness model includes the rules HR-P2 and HR-FP ([harness](harness.md#harness-rules-hr-p2-and-hr-fp)) and the evidence is `p1`, `second`, `owner`, `audit2` or `audit3`; under the earlier definition `audit3` is not counted |
| labelled assumption | A1 is the one still in use; F and P2 became the harness rules HR-FP and HR-P2 and PN is retired ([assumptions](assumptions.md#summary)): a stated condition the maintainer accepted for named rows; a row that rests on one has evidence `assumed`, is strict, and is not strictest |
| `anyworld` | evidence class of the P2v rows whose branch outcomes were counted over all worlds, skipped ones included; strict parity but not strictest |
| unproven | a function that never matched; counted in no parity figure; none remains |

## Tiers

Every function has exactly one tier. What each tier compared ([harness: comparison](harness.md#comparison)):

| Tier | Compared |
|---|---|
| P1 | return registers and every `out` or `inout` buffer, on oracle vectors |
| P2v | return registers, final memory writes outside the stack and the full call trace, in every compared world |
| P2 | nothing |

<!--gen:tiers-->
| tier | functions | bytes | share of code | of which partial |
|---|---|---|---|---|
| P1 | 549 | 58,078 | 6.0 | 0 |
| P2v | 2,153 | 906,581 | 94.0 | 0 |
| P2 | 0 | 0 | 0.0 | 0 |
| P2-std | 0 | 0 | 0.0 | 0 |
| P3 | 0 | 0 | 0.0 | 0 |
<!--/gen:tiers-->

## Standards

| Standard | Functions | Bytes | Share of code bytes |
|---|---|---|---|
| strict parity | <!--f:strict_fns-->2,702<!--/f--> of <!--f:fn_total-->2,702<!--/f--> | <!--f:strict_bytes-->964,659<!--/f--> of <!--f:code_bytes-->964,659<!--/f--> | <!--f:strict_pct-->100.0<!--/f-->% |
| of which resting on labelled assumptions | <!--f:close_labelled_rows-->1<!--/f--> | <!--f:close_labelled_bytes-->12,884<!--/f--> | <!--f:close_labelled_pct-->1.3<!--/f-->% |
| strictest, revised standard of 2026-11-05 (all classes) | <!--f:strictest_rev_fns-->2,701<!--/f--> | <!--f:strictest_rev_bytes-->951,775<!--/f--> | <!--f:strictest_rev_pct-->98.7<!--/f-->% |
| strictest, earlier definition (`audit3` not counted) | <!--f:strictest_fns-->2,697<!--/f--> | <!--f:strictest_bytes-->944,336<!--/f--> | <!--f:strictest_pct-->97.9<!--/f-->% |
| unproven | <!--f:unproven_fns-->0<!--/f--> | <!--f:unproven_bytes-->0<!--/f--> | <!--f:unproven_pct-->0.0<!--/f-->% |

Strict parity is not an unqualified full match of every input: <!--f:close_labelled_rows-->1<!--/f--> of the <!--f:fn_total-->2,702<!--/f--> strict rows rests on a labelled assumption (F <!--f:close_rows_F-->0<!--/f--> rows, PN <!--f:close_rows_PN-->0<!--/f-->, P2 <!--f:close_rows_P2-->0<!--/f-->, A1 <!--f:close_rows_A1-->1<!--/f-->: `f_0001b438`, whose six P2 outcomes are now explained under HR-P2). They are listed with their worlds and outcomes in [evidence: the last seven rows](evidence.md#the-last-seven-rows), and each assumption is defined in [assumptions](assumptions.md#summary). The other <!--f:close_pass_rows-->9<!--/f--> rows audited so far rest on the harness model alone (evidence `audit2` or `audit3`) and count as strictest.

The standard was revised on 2026-11-05. The two rules HR-P2 and HR-FP replace the labelled assumptions P2 and F once their proofs are independently re-derived; until a row has that audit it keeps its label. PN is retired. A1 is kept for `f_0001b438`, the one row left: its outcomes 0x1d3bc, 0x1ddc9 and 0x1e3ba need a valid sort range, a caller precondition that is not a harness fact, and no world of any kind takes them.

The code is the dump: the raw x86-64 code section of a build of the plugin. The `evidence` column of `docs/data/FUNCTION_MAP.tsv` gives each row's class ([evidence](evidence.md#evidence-classes)):

<!--gen:evidence-->
| evidence | meaning | functions | bytes | share of code |
|---|---|---|---|---|
| `p1` | P1: oracle vectors, bit exact | 549 | 58,078 | 6.0 |
| `second` | P2v: PASS from a second check, separate from the row's own ledger entry | 1,945 | 827,551 | 85.8 |
| `owner` | P2v: the row's own ledger entry, exact branch counts over compared worlds | 198 | 49,427 | 5.1 |
| `audit2` | P2v: PASS from the independent audit of the last seven rows, with its own coverage recompute | 5 | 9,280 | 1.0 |
| `audit3` | P2v: PASS from the independent audit under the revised harness model of 2026-11-05 (rules HR-P2 and HR-FP) | 4 | 7,439 | 0.8 |
| `assumed` | P2v: PASS from the same audit, resting on labelled assumptions the maintainer accepted | 1 | 12,884 | 1.3 |
| `anyworld` | P2v: older any-world accounting, not yet audited | 0 | 0 | 0.0 |
| `-` | none: never matched (unproven) | 0 | 0 | 0.0 |
<!--/gen:evidence-->

## Anyworld rows

The <!--f:ev_anyworld_fns-->0<!--/f--> `anyworld` rows and the <!--f:ev_assumed_fns-->1<!--/f--> `assumed` row ([evidence](evidence.md#the-last-seven-rows)) are the difference between strict parity and strictest. The ports of the `anyworld` rows matched the functions they replace in every compared world; open is whether every branch outcome was exercised in those worlds. For the `assumed` rows every outcome is seen or explained, under their labelled assumptions.

<!--gen:anyworld-->
| bank name | id | offset | bytes | audit verdict |
|---|---|---|---|---|
<!--/gen:anyworld-->

OPEN: the audit found branch outcomes that no compared world took and no sound rule explained ([evidence: the audit](evidence.md#the-audit)). Bank names here and below are strings from the name bank, not evidence of behaviour.

## Known port divergences outside the harness model

These are measured differences between a port and its original in worlds that violate HR-P2 (a buffer argument inside the function's own frame). The harness model excludes such worlds, so they are not compared worlds of any standard and no figure changes. Each is a port follow-up for the maintainer.

| Function | Worlds | Result | File | Follow-up |
|---|---|---|---|---|
| `f_0001b438` | 1,609 stack-resident worlds compared (sorted array on the function's own stack, `rdi = 0x7ffff000 + x`) | 32 differ, 0 Rust faults; the first is world 55, call event 6 to 0x100ec014, whose 4th integer argument is 0x0 in the port and 0xc3bd000000000000 in the original; world 108 also returns a different `rax` and takes one extra call | `FINDING_STACK_RESIDENT.md` (kept in the recovery workspace, not in this repository) | model the return address push of the lifted stub calls in the lifted world (INFERRED cause: only the self call writes it, so an array overlapping the dead slot `rsp-8` reads different words) |
| `f_0002405b` (0x246f8) and `f_00024a39` (0x25004) | 11 stack-resident worlds that take these outcomes | the port faults differently from the original: at 0x246f8 the original writes at 0x0 and the port writes at 0x18; at 0x25004 the original writes at 0x0 and the port reads at 0x0 | `FAULT_PARITY.tsv` and `WORLDS.tsv` (kept in the recovery workspace) | make the faulting access of the port match the original in these worlds |

The other stack-resident worlds of `f_0002405b` and `f_00024a39` (2,551 and 7,646 compared) show 0 mismatches, and set 15 of `f_0001b438` (unaligned, heap arena) is unaffected (274 of 274). All counts are MEASURED by the builders named in the workspace files.

## Unproven

<!--f:unproven_fns-->0<!--/f--> functions are unproven: each of the <!--f:fn_total-->2,702<!--/f--> matched the build it replaces in every compared world. Seven were unproven until a closing round matched them in new world sets and an independent audit re-checked them ([evidence: the last seven rows](evidence.md#the-last-seven-rows)); their rows carry the assumptions they rest on. A function that stopped matching would return here and count in no parity figure.

## Caveats

The limits on every parity figure are in [assumptions](assumptions.md).

## Looking up one function

| Question | Ask |
|---|---|
| Tier, status, strict class, evidence, crate and file of a function | `python -I tools/fn_info.py ID` (an `f_XXXXXXXX` id, a bank name or an offset), or its row in `docs/data/FUNCTION_MAP.tsv` |
| The ledger note behind a row | `docs/data/ledger_digest.tsv` |
| The audit verdict of an older row | `docs/data/audit_final.tsv` |
| All rows of one tier or evidence class | the generated pages under `docs/generated/` |
