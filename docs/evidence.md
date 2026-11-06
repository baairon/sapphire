# Evidence and audit

The files that record each function's tier, status and evidence, their columns, the ledger note grammar, and the audit of the older P2v rows. Tiers and standards are defined in [parity](parity.md#terms), and the terms in the [glossary](glossary.md).

## Files

| File | Rows | Content |
|---|---|---|
| `docs/data/FUNCTION_MAP.tsv` | one per function | the map: tier, status, strict class, evidence class, crate file |
| `docs/data/ledger_digest.tsv` | one per function | the ledger row behind the map row, with its note |
| `docs/data/EXACT_AUDIT.tsv` | <!--f:exact_audit_rows-->1,702<!--/f--> | the P2v rows recorded under the older any-world accounting, with the start of their note |
| `docs/data/audit_final.tsv` | <!--f:audit_rows-->1,702<!--/f--> | the audit verdict and detail of each of those rows |
| `docs/data/audit_close.tsv` | <!--f:close_rows-->10<!--/f--> | the independent audit of the last seven rows: verdict, compared worlds, outcomes seen and explained, and the labels each row rests on ([the last seven rows](#the-last-seven-rows)) |
| `docs/data/close_sets.tsv` | <!--f:close_sets_rows-->62<!--/f--> | one line per world set behind those rows: cases, compared worlds, seed, generator, pins and the SHA-256 of the vector file (the vector files are not in the repository) |
| `docs/data/PARITY.md`, `docs/data/MISSING_DATA.md` | | aggregates generated from the full ledgers (archived) |

## FUNCTION_MAP columns

| Column | Values |
|---|---|
| `offset` | hex offset of the function in the dump |
| `name` | the id `f_XXXXXXXX`; the bank name is in `docs/data/names.tsv` |
| `size` | bytes |
| `group` | `A` or `B`, a fixed split of the functions into two halves; no figure depends on it |
| `tier` | `P1`, `P2v`, `P2` ([parity: tiers](parity.md#tiers)) |
| `status` | `done` or `partial` |
| `crate` | the lift crate whose ledger row the map keeps (`lift-1` to `lift-8`, `lift-a`, `lift-b`); `source` is in that crate |
| `strict` | `p1` (a P1 row), `exact` (a P2v row at strict parity), `unproven` (never matched) |
| `source` | the file of the copy the map names ([lift crates: copies](lifting.md#copies-of-one-function)) |
| `evidence` | the evidence class below |

## Evidence classes

Each [evidence class](glossary.md#evidence-class) names what a row's parity result rests on. [Branch outcomes](glossary.md#branch-outcome) are counted over [compared worlds](glossary.md#compared-world) only (worlds in which the function ran to completion), from complete outcome lists. A P2v row recorded before that rule enters the strictest standard only once a second check passes it. Counts and code shares per class are in [parity](parity.md#standards).

| `evidence` | In plain words | Meaning |
|---|---|---|
| `p1` | bit exact on recorded cases | P1: bit exact on oracle vectors (<!--f:p1_vectors-->117,988<!--/f--> vectors across the <!--f:p1_vector_fns-->236<!--/f--> functions with stored vector files) |
| `second` | confirmed by a second check | P2v: a second check, separate from the row's own ledger entry, replayed its worlds under that rule and recorded PASS; for the rows of `docs/data/EXACT_AUDIT.tsv` that check is the audit below |
| `ledger` | recorded by the port's own check | P2v: the row's own ledger entry records branch counts over compared worlds; no second check |
| `audit2` | independently audited | P2v: the independent audit of the last seven rows ([the last seven rows](#the-last-seven-rows)) recomputed the branch outcomes over compared worlds with its own emulator hook, not with the abstract interpreter of the audit below, replayed every set in its own build with no mismatch and no Rust fault, and re-derived each explained outcome with its own control flow pruning; every explained outcome rests only on the callee model, the zeroed stack and the landing pad category |
| `audit3` | audited under the 2026-11-05 harness rules | P2v: the same independent audit, for a row whose last unexplained outcomes are explained only by the harness rules HR-P2 or HR-FP of 2026-11-05 ([harness](harness.md#harness-rules-hr-p2-and-hr-fp)), each proof re-derived independently; strictest under the revised standard, not under the earlier definition. <!--f:ev_audit3_fns-->4<!--/f--> rows have it at this date (`f_00031780`, `f_000651c2`, `f_0002405b`, `f_00024a39`). The proofs were re-derived independently by a second checker from the archive listings with its own script (files `PROOFS.tsv` and `PROOF_f_*.md`; every store on every path between the zero store and the flag test is classified, 0 unknown stores). Limits of that re-derivation (INFERRED): HR-P2 was read from the rule's wording at the time; calls follow the Calls row of the harness with every caller-saved register clobbered; the successors of indirect jumps are over-approximated by the case labels of the listing |
| `assumed` | audited, given a stated precondition | P2v: the same independent audit, but at least one outcome of the row is explained by, or seen only through, a [precondition label](glossary.md#precondition-label) stated for that row (A1 is the only one in use; F and P2 became the rules HR-FP and HR-P2 and PN is retired, [below](#explanation-kinds-of-the-last-seven-rows)); strict parity, not the strictest standard |
| `anyworld` | counted over all worlds, not audited | P2v: counted under the older any-world accounting (outcomes counted in every world, skipped ones included); the audit left it OPEN |
| `-` | unproven | none: never matched |

Strict parity counts `strict` `p1` and `exact`; the strictest standard under the earlier definition counts `evidence` `p1`, `second`, `ledger` and `audit2`, and under the revised standard of 2026-11-05 also `audit3`. A row with evidence `assumed` is strict but not strictest, because the strictest standard needs every branch outcome seen in a compared world or explained under the harness model alone. Of <!--f:p1_candidates-->606<!--/f--> self-contained P1 candidates, <!--f:p1_downgraded-->57<!--/f--> became P2v because they read pointers loaded from their own buffers or otherwise needed the world harness.

## The audit

Each row of `docs/data/EXACT_AUDIT.tsv` was re-checked one function at a time. The audit passed <!--f:audit_pass-->1,702<!--/f--> of them, now `second`, and left <!--f:audit_open-->0<!--/f--> OPEN, the `anyworld` rows ([parity](parity.md#anyworld-rows)).

| Step | What happens |
|---|---|
| 1. no branches | a function without conditional branches, loops or indirect jumps runs every instruction in each compared world, so PASS needs only 900 or more compared worlds |
| 2. replay | otherwise the vector sets named in the ledger note are replayed with complete coverage; every world's arguments must equal the vector file's, and a set that does not reproduce makes the row OPEN |
| 3. landing pads | outcomes only in blocks unreachable from the entry are removed ([assumptions](assumptions.md#landing-pads)) |
| 4. explanation | an abstract interpreter removes an outcome only when it proves it impossible for every execution the harness can produce; hand proofs re-derived for the row remove more ([assumptions](assumptions.md#hand-proofs)) |
| 5. soundness | neither the abstract interpreter nor a hand proof may call impossible an outcome that a compared world observed |
| 6. verdict | PASS when no outcome is left, OPEN when one is left or a set does not reproduce, FAIL with fewer than 900 compared worlds |

Outcomes explained only by the stack protector rule (the guard slot keeps its value until the check) or rule W (a displacement loaded from memory and added to an exact frame pointer is not negative) are counted apart in each detail ([assumptions](assumptions.md#stack-protector-rule-and-rule-w)).

<!--gen:audit_verdicts-->
| verdict | rows |
|---|---|
| PASS | 1,702 |
<!--/gen:audit_verdicts-->

<!--f:audit_proof_rows-->30<!--/f--> PASS rows needed explained outcomes (decided branches or dead blocks), and <!--f:audit_landing_rows-->15<!--/f--> had outcomes only in landing pads.

## Reading an audit detail

`sets 1+2+3, 1878 ok worlds, 4 conditional branches, 3 unseen outcomes in ok worlds: 2 only in blocks unreachable from the entry (landing pads), 0 explained by proof (none), 0 only under rule W, 0 only under the stack protector assumption, 1`

| Part | Meaning |
|---|---|
| `sets` | the vector sets replayed (`world`, `world2`, `world3`, and others) |
| `ok worlds` | compared worlds across those sets |
| `unseen outcomes in ok worlds` | branch outcomes no compared world took |
| `only in blocks unreachable from the entry` | removed as landing pads |
| `explained by proof` | decided branches and dead blocks, proved by the abstract interpreter or a hand proof |
| `only under rule W`, `only under the stack protector assumption` | explained only by one of these two rules |
| final number (`N left`) | outcomes left unexplained; any value above 0 makes the row OPEN |
| `ALL-SETS MODE (the engine is not used)` | every outcome was observed in the union of the sets, so the abstract interpreter was not needed |

## The last seven rows

Seven functions had never matched ([parity](parity.md#unproven)). A closing round replayed them in new world sets, chosen so that the original completes and takes branch outcomes the earlier sets never took (`docs/data/close_sets.tsv`), and a separate checker then audited the round. That audit recomputed the branch outcomes over compared worlds with its own emulator hook (not with the abstract interpreter or the scripts of the audit above), replayed every set against the ports in its own build, ran every kept world a second time in a fresh emulator, and re-derived each explained outcome. Every row has at least 900 compared worlds, no mismatch and no Rust fault.

Three ports needed a fix before they matched: the self call of `f_0001b438` runs in its own frame, `f_00024a39` dispatches through the jump table at 0x2531f, and `f_00056fa6` returns right after its call to the `noreturn` stub 0x961. The fixes are in the ports. The replay tests list the later sets of these functions, because the stock list stops at set 3; since 2026-11-05 the lists include the sets of round two (sets 4 and 5 for `f_000ac996` and `f_000b4896`, sets 4 to 7 for `f_000b11c0`, set 9 for `f_00056fa6`, set 15 for `f_0001b438`). The vector files stay outside the repository and the tests print NOT RUN without them.

<!--gen:close_rows-->
| bank name | id | offset | bytes | evidence | labels | compared worlds | outcomes seen |
|---|---|---|---|---|---|---|---|
| `void std::__1::__sort<bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&), NTV2DeviceInfo*>(NTV2DeviceInfo*, NTV2DeviceInfo*, bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&))` | f_0001b438 | 0x1b438 | 12,884 | `assumed` | `A1` | 2,486 | 65 of 74 |
| `void std::__1::__insertion_sort_3<bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&), NTV2DeviceInfo*>(NTV2DeviceInfo*, NTV2DeviceInfo*, bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&))` | f_0002405b | 0x2405b | 2,526 | `audit3` | `-` | 2,149 | 12 of 14 |
| `bool std::__1::__insertion_sort_incomplete<bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&), NTV2DeviceInfo*>(NTV2DeviceInfo*, NTV2DeviceInfo*, bool (*&)(NTV2DeviceInfo const&, NTV2DeviceInfo const&))` | f_00024a39 | 0x24a39 | 4,371 | `audit3` | `-` | 2,300 | 21 of 26 |
| `CNTV2AxiSpiFlash::Write(unsigned int, std::__1::vector<unsigned char, std::__1::allocator<unsigned char> >, unsigned int)` | f_00030040 | 0x30040 | 1,570 | `audit2` | `-` | 1,371 | 25 of 64 |
| `void std::__1::vector<unsigned char, std::__1::allocator<unsigned char> >::__push_back_slow_path<unsigned char const&>(unsigned char const&)` | f_00031780 | 0x31780 | 268 | `audit3` | `-` | 2,467 | 7 of 8 |
| `DeviceMap::CheckDriverVersion(unsigned int)` | f_00056fa6 | 0x56fa6 | 3,434 | `audit2` | `-` | 1,326 | 17 of 24 |
| `void std::__1::vector<unsigned short, std::__1::allocator<unsigned short> >::__push_back_slow_path<unsigned short>(unsigned short&&)` | f_000651c2 | 0x651c2 | 274 | `audit3` | `-` | 3,298 | 5 of 6 |
| `RegisterExpert::DecodeAncExtFieldLinesReg::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000ac996 | 0xac996 | 1,344 | `audit2` | `-` | 4,688 | 10 of 10 |
| `RegisterExpert::DecodeHDMIOutHDRControl::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000b11c0 | 0xb11c0 | 1,640 | `audit2` | `-` | 3,261 | 6 of 8 |
| `RegisterExpert::DecodeSDIErrorStatus::operator()(unsigned int, unsigned int, NTV2DeviceID) const` | f_000b4896 | 0xb4896 | 1,292 | `audit2` | `-` | 2,493 | 2 of 2 |
<!--/gen:close_rows-->

The ten rows count <!--f:close_compared-->25,839<!--/f--> compared worlds and <!--f:close_outcomes-->236<!--/f--> branch outcomes: <!--f:close_seen-->170<!--/f--> seen in compared worlds and <!--f:close_explained-->66<!--/f--> explained, <!--f:close_landing_outcomes-->22<!--/f--> of those in landing pads. <!--f:ev_audit2_fns-->5<!--/f--> rows (`f_00030040` and, from the second round of 2026-11-05, `f_00056fa6`, `f_000ac996`, `f_000b11c0`, `f_000b4896`) rest on the original harness model alone and have evidence `audit2`, so they count in the strictest standard under both definitions. <!--f:ev_audit3_fns-->4<!--/f--> rows (`f_00031780`, `f_000651c2`, `f_0002405b`, `f_00024a39`) have evidence `audit3`: they rest on the rules HR-FP or HR-P2 and count under the revised standard only. The other <!--f:close_labelled_rows-->1<!--/f--> row, `f_0001b438`, has evidence `assumed`: its parity rests on the precondition A1, which applies to this row and no other. Neither the stack protector rule nor rule W is used by any of the ten.

`docs/data/audit_close.tsv` has the columns `symbol`, `offset`, `crate`, `verdict`, `compared` and `worlds` (compared and generated worlds), `outcomes`, `seen`, `explained`, `landing_pads` (of the explained outcomes), `labels` (`-` or a comma list of the labels still in use; F, PN and P2 appeared in earlier versions), `label_outcomes` (outcomes per label) and `detail` (in the grammar of an audit detail above). The verdict `PASS` means the row rests on the harness model alone (including HR-P2 and HR-FP); `PASS_LABELLED` means it passes given its labels.

### Explanation kinds of the last seven rows

| Label | Kind | Exact definition |
|---|---|---|
| F (HR-FP) | fault-only proof, now rule HR-FP | An unseen outcome that no compared world can contain, because every execution that takes it faults. The proof shows that the branch is reached with that outcome only when the function stores through an address that no world maps, over all 64 bit values of the operands and with no call between the branch and the store. A world in which the original faults is skipped, not compared ([harness](harness.md#terms)). Rows: `f_00031780` outcome 0x317f3 fallthrough (byte store to 0xffffffffffffffff) and `f_000651c2` outcome 0x6525a taken (two byte store to 0xfffffffffffffffe). The audit ran a grid of 27,648 header combinations for each: the outcome occurred 30 and 50 times, and every such execution faulted |
| PN (retired) | pinned world word plus noreturn | An outcome seen only in compared worlds that carry a `pin ADDR VALUE` case line and end at a call to a stub declared `noreturn` ([assumptions](assumptions.md#pinned-world-words-in-the-case-line)). Row: `f_00056fa6` outcome 0x57936 taken, the failed stack guard check. The pin sets the world word at 0x1403d8, the address the guard is read from, to the address of a stream object store of the function itself, so the guard differs between the entry read and the check; the function then calls stub 0x961 (`___stack_chk_fail`), declared `noreturn`, and the world ends there. Retired 2026-11-05: it was seen only in the 16 pinned worlds of set 8, and set 9 takes it in 23 of 23 worlds without a pin, so the pinned set left the table (<!--f:close_sets_pins-->0<!--/f--> pinned worlds remain). The outcome is seen, not explained, but only in those worlds |
| P2 (HR-P2) | frame disjointness, now rule HR-P2 | A store whose address is built from `rbp` or `rsp`, the function's own frame, never hits the record or buffer array that an argument points to. It is the rule the abstract interpreter already applies (every buffer argument has its own page region, and a store through a pointer of unknown origin cannot hit the frame), written out here because <!--f:close_out_P2-->0<!--/f--> outcomes rest on it. Each is the taken side of a flag byte test that a zero store dominates; the only stores between the zero store and the test that could change the byte go through the frame. Rows: `f_0002405b` 0x243b9 and 0x246f8; `f_00024a39` 0x24d63, 0x25004, 0x25858 and 0x2557c; `f_0001b438` 0x1b8cc, 0x1bba4, 0x1c64e, 0x1cb94, 0x1db61 and 0x1e0b5. The audit attacked the premise for `f_0002405b` and `f_00024a39` with 27,080 and 26,200 worlds that put the array on the function's own frame: 3,145 and 3,058 of them take a flag outcome and none of the completed ones does |
| A1 | valid sort range | The range `[first, last)` that `f_0001b438` sorts holds a whole number of 248 byte records: `last - first` is a multiple of 248, as a valid caller passes it. The function divides the range by 248 exactly (a multiplication by the inverse of 31 after a shift, 0x1b525 to 0x1b533) and never checks the remainder; for an aligned range, the two records of one swap are the same slot or two disjoint slots. An input domain precondition, not a harness fact. Rows: the 3 outcomes 0x1d3bc, 0x1ddc9 and 0x1e3ba taken, which also rest on P2. The outcome 0x1be5e left A1 on 2026-11-05 because set 15 takes it. `f_0001b438` is the one row left that needs a precondition label beyond the harness rules, because no world of any kind takes these three. Worlds with an unaligned range do complete in the harness (set 14, 212 worlds) and take the sibling outcomes 0x1c8a3 and 0x1ce51; those two are therefore seen outcomes and rest on nothing |

The <!--f:close_landing_outcomes-->22<!--/f--> landing pad outcomes of `f_00030040` are the existing landing pad category (blocks from 0x3056f on, reachable only through unwind tables, [assumptions](assumptions.md#landing-pads)); the other 19 explained outcomes of that row (9 decided branches and 10 outcomes behind them) are decided by the abstract interpreter under the callee model and the zeroed stack. The 7 explained outcomes of `f_00056fa6` are decided the same way.

## ledger_digest columns

| Column | Values |
|---|---|
| `symbol`, `offset`, `crate`, `tier`, `status`, `strict`, `evidence` | as in the map |
| `vectors_ok` | compared vectors or worlds, sometimes as `ok/total` |
| `note` | free text; the parts below carry evidence |

## Note grammar

| Part | Example | Meaning |
|---|---|---|
| route | `mechanically lifted from the x86` or `machine-lifted` | a mechanical lift ([lift crates](lifting.md#two-routes)) |
| world counts | `worlds=1200 compared=922 skipped=278` or `1089 worlds ok of 1576` | worlds generated, compared (the function ran to completion), skipped (the function faulted) |
| branches | `branches 32/32` or `branches 0/0 seen both ways` | conditional branches with both outcomes seen, of all |
| coverage | `insn coverage 100.0% (0 uncovered outside landing pads)` | instructions executed, landing pads counted apart |
| landing pads | `landing pads unverified: 2 blocks at 0x14ec3,0x14ed2` | blocks reachable only through unwind tables ([assumptions](assumptions.md#landing-pads)) |
| assumption | `assumption: verified under opaque callees that write nothing through pointers` | the row's check rests on a stated assumption |
| seed | `seed 7` | world seed of the vector set |
| behaviour | `Getter: returns the qword at +0x20.` | what the function computes |
