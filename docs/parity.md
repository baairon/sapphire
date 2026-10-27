# Parity

How many functions of the dump match the original machine code, and under which standard. The comparison itself is in [harness](harness.md).

## Terms

| Term | Meaning |
|---|---|
| P1 | tier of a self-contained function whose port equals the emulated original bit for bit on its oracle vectors |
| P2v | tier of a function that is not self-contained, compared with the original in worlds; status `done` means it passed the world check, `partial` that it did not |
| P2 | tier of a port with the exact control flow and arithmetic whose missing references are placeholders; not compared, so not parity |
| P2-std | tier of a recognised libc++ container, string or smart pointer instantiation mapped to the Rust std type |
| P3 | tier of a function that needs data that cannot be guessed |
| oracle vector | one recorded case of the emulated original: arguments, returns and final buffers |
| world | a seeded model of everything the dump lacks, in which the original and the port both run |
| missing reference | data, an import or a constant the code uses that is not in the dump |
| placeholder | the stand-in a port uses for a missing reference: an `Env` access by its `rel_to_end` (`data_va`, `stub_va`) |
| `partial` | status of a row whose comparison did not pass; every `partial` row is unproven |
| strict parity | P1 rows, plus P2v rows with status `done` |
| strictest | strict parity with every branch outcome counted over compared worlds only: evidence `p1`, `second` or `owner` |
| `anyworld` | evidence class of the P2v rows whose branch outcomes were counted over all worlds, skipped ones included; strict parity but not strictest |
| unproven | a function that never matched; counted in no parity figure |

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
| P2v | 2,148 | 881,796 | 91.4 | 2 |
| P2 | 5 | 24,785 | 2.6 | 5 |
| P2-std | 0 | 0 | 0.0 | 0 |
| P3 | 0 | 0 | 0.0 | 0 |
<!--/gen:tiers-->

## Standards

| Standard | Functions | Bytes | Share of code bytes |
|---|---|---|---|
| strict parity | <!--f:strict_fns-->2,695<!--/f--> of <!--f:fn_total-->2,702<!--/f--> | <!--f:strict_bytes-->939,332<!--/f--> of <!--f:code_bytes-->964,659<!--/f--> | <!--f:strict_pct-->97.4<!--/f-->% |
| strictest | <!--f:strictest_fns-->2,692<!--/f--> | <!--f:strictest_bytes-->935,056<!--/f--> | <!--f:strictest_pct-->96.9<!--/f-->% |
| unproven | <!--f:unproven_fns-->7<!--/f--> | <!--f:unproven_bytes-->25,327<!--/f--> | <!--f:unproven_pct-->2.6<!--/f-->% |

The code is the dump: the raw x86-64 code section of a build of the plugin. The `evidence` column of `docs/data/FUNCTION_MAP.tsv` gives each row's class ([evidence](evidence.md#evidence-classes)):

<!--gen:evidence-->
| evidence | meaning | functions | bytes | share of code |
|---|---|---|---|---|
| `p1` | P1: oracle vectors, bit exact | 549 | 58,078 | 6.0 |
| `second` | P2v: PASS from a second check, separate from the row's own ledger entry | 1,945 | 827,551 | 85.8 |
| `owner` | P2v: the row's own ledger entry, exact branch counts over compared worlds | 198 | 49,427 | 5.1 |
| `anyworld` | P2v: older any-world accounting, not yet audited | 3 | 4,276 | 0.4 |
| `-` | none: never matched (unproven) | 7 | 25,327 | 2.6 |
<!--/gen:evidence-->

## Anyworld rows

The <!--f:ev_anyworld_fns-->3<!--/f--> `anyworld` rows are the difference between strict parity and strictest. Their ports matched the original in every compared world; open is whether every branch outcome was exercised in those worlds.

<!--gen:anyworld-->
| bank name | id | offset | bytes | audit verdict |
|---|---|---|---|---|
| `RegisterExpert::DecodeAncExtFieldLinesReg::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000ac996 | 0xac996 | 1,344 | OPEN |
| `RegisterExpert::DecodeHDMIOutHDRControl::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000b11c0 | 0xb11c0 | 1,640 | OPEN |
| `RegisterExpert::DecodeSDIErrorStatus::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000b4896 | 0xb4896 | 1,292 | OPEN |
<!--/gen:anyworld-->

OPEN: the audit found branch outcomes that no compared world took and no sound rule explained ([evidence: the audit](evidence.md#the-audit)). Bank names here and below are strings from the name bank, not evidence of behaviour.

## Unproven

<!--f:unproven_fns-->7<!--/f--> functions never matched the original and are counted in no parity figure: <!--f:unproven_p2_fn-->5<!--/f--> are tier P2 (<!--f:unproven_p2_bytes-->24,785<!--/f--> bytes) and <!--f:unproven_p2v_fn-->2<!--/f--> are P2v rows with status `partial` (<!--f:unproven_p2v_bytes-->542<!--/f--> bytes). They remain in the crates. Matching them needs the archive ([reproduce](reproduce.md#what-needs-the-archive)).

<!--gen:unproven-->
| bank name | id | offset | bytes | tier | status | origin | source |
|---|---|---|---|---|---|---|---|
| `void std::__1::__sort<bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&), NTV2DeviceInfo*>(NTV2DeviceInfo*, NTV2DeviceInfo*, bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&))` | f_0001b438 | 0x1b438 | 12,884 | P2 | partial | std | `crates/lift-1/src/p2v/b3.rs:11` |
| `void std::__1::__insertion_sort_3<bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&), NTV2DeviceInfo*>(NTV2DeviceInfo*, NTV2DeviceInfo*, bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&))` | f_0002405b | 0x2405b | 2,526 | P2 | partial | std | `crates/lift-3/src/p2v/u11001.rs:9022` |
| `bool std::__1::__insertion_sort_incomplete<bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&), NTV2DeviceInfo*>(NTV2DeviceInfo*, NTV2DeviceInfo*, bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&))` | f_00024a39 | 0x24a39 | 4,371 | P2 | partial | std | `crates/lift-3/src/p2v/u11001.rs:34415` |
| `CNTV2AxiSpiFlash::Write(unsigned int, std::__1::vector<unsigned char, std::__1::allocator<unsigned char> >, unsigned int)` | f_00030040 | 0x30040 | 1,570 | P2 | partial | std | `crates/lift-3/src/p2v/u10001.rs:886` |
| `void std::__1::vector<unsigned char, std::__1::allocator<unsigned char> >::__push_back_slow_path<unsigned char const&>(unsigned char const&)` | f_00031780 | 0x31780 | 268 | P2v | partial | std | `crates/lift-5/src/p2v/w003.rs:5124` |
| `DeviceMap::CheckDriverVersion(unsigned int)` | f_00056fa6 | 0x56fa6 | 3,434 | P2 | partial | bundled_library | `crates/lift-3/src/p2v/u11001.rs:26808` |
| `void std::__1::vector<unsigned short, std::__1::allocator<unsigned short> >::__push_back_slow_path<unsigned short>(unsigned short&&)` | f_000651c2 | 0x651c2 | 274 | P2v | partial | std | `crates/lift-5/src/p2v/w003.rs:5395` |
<!--/gen:unproven-->

## Caveats

The limits on every parity figure are in [assumptions](assumptions.md).

## Looking up one function

| Question | Ask |
|---|---|
| Tier, status, strict class, evidence, crate and file of a function | `python -I tools/fn_info.py ID` (an `f_XXXXXXXX` id, a bank name or an offset), or its row in `docs/data/FUNCTION_MAP.tsv` |
| The ledger note behind a row | `docs/data/ledger_digest.tsv` |
| The audit verdict of an older row | `docs/data/audit_final.tsv` |
| All rows of one tier or evidence class | the generated pages under `docs/generated/` |
