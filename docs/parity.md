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
| strictest | strict parity with every branch outcome counted over compared worlds only: evidence `p1`, `second`, `owner` or `audit2` |
| labelled assumption | one of F, PN, P2 and A1 ([assumptions](assumptions.md#summary)): a stated condition the maintainer accepted for named rows; a row that rests on one has evidence `assumed`, is strict, and is not strictest |
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
| of which resting on labelled assumptions | <!--f:close_labelled_rows-->6<!--/f--> | <!--f:close_labelled_bytes-->23,757<!--/f--> | <!--f:close_labelled_pct-->2.5<!--/f-->% |
| strictest | <!--f:strictest_fns-->2,693<!--/f--> | <!--f:strictest_bytes-->936,626<!--/f--> | <!--f:strictest_pct-->97.1<!--/f-->% |
| unproven | <!--f:unproven_fns-->0<!--/f--> | <!--f:unproven_bytes-->0<!--/f--> | <!--f:unproven_pct-->0.0<!--/f-->% |

Strict parity is not an unqualified full match of every input: <!--f:close_labelled_rows-->6<!--/f--> of the <!--f:fn_total-->2,702<!--/f--> strict rows rest on a labelled assumption (F <!--f:close_rows_F-->2<!--/f--> rows, PN <!--f:close_rows_PN-->1<!--/f-->, P2 <!--f:close_rows_P2-->3<!--/f-->, A1 <!--f:close_rows_A1-->1<!--/f-->; `f_0001b438` carries both P2 and A1). They are listed with their worlds and outcomes in [evidence: the last seven rows](evidence.md#the-last-seven-rows), and each assumption is defined in [assumptions](assumptions.md#summary). The other row of that group, `f_00030040`, rests on the accepted harness assumptions alone and counts as strictest.

The code is the dump: the raw x86-64 code section of a build of the plugin. The `evidence` column of `docs/data/FUNCTION_MAP.tsv` gives each row's class ([evidence](evidence.md#evidence-classes)):

<!--gen:evidence-->
| evidence | meaning | functions | bytes | share of code |
|---|---|---|---|---|
| `p1` | P1: oracle vectors, bit exact | 549 | 58,078 | 6.0 |
| `second` | P2v: PASS from a second check, separate from the row's own ledger entry | 1,945 | 827,551 | 85.8 |
| `owner` | P2v: the row's own ledger entry, exact branch counts over compared worlds | 198 | 49,427 | 5.1 |
| `audit2` | P2v: PASS from the independent audit of the last seven rows, with its own coverage recompute | 1 | 1,570 | 0.2 |
| `assumed` | P2v: PASS from the same audit, resting on labelled assumptions the maintainer accepted | 6 | 23,757 | 2.5 |
| `anyworld` | P2v: older any-world accounting, not yet audited | 3 | 4,276 | 0.4 |
| `-` | none: never matched (unproven) | 0 | 0 | 0.0 |
<!--/gen:evidence-->

## Anyworld rows

The <!--f:ev_anyworld_fns-->3<!--/f--> `anyworld` rows and the <!--f:ev_assumed_fns-->6<!--/f--> `assumed` rows ([evidence](evidence.md#the-last-seven-rows)) are the difference between strict parity and strictest. The ports of the `anyworld` rows matched the functions they replace in every compared world; open is whether every branch outcome was exercised in those worlds. For the `assumed` rows every outcome is seen or explained, under their labelled assumptions.

<!--gen:anyworld-->
| bank name | id | offset | bytes | audit verdict |
|---|---|---|---|---|
| `RegisterExpert::DecodeAncExtFieldLinesReg::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000ac996 | 0xac996 | 1,344 | OPEN |
| `RegisterExpert::DecodeHDMIOutHDRControl::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000b11c0 | 0xb11c0 | 1,640 | OPEN |
| `RegisterExpert::DecodeSDIErrorStatus::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000b4896 | 0xb4896 | 1,292 | OPEN |
<!--/gen:anyworld-->

OPEN: the audit found branch outcomes that no compared world took and no sound rule explained ([evidence: the audit](evidence.md#the-audit)). Bank names here and below are strings from the name bank, not evidence of behaviour.

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
