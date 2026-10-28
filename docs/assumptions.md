# Assumptions and limits

What the parity figures rest on, and how many rows depend on each assumption. Parity is strong measured evidence on seeded inputs under these assumptions, not a proof for every input or every callee.

## Summary

| Assumption | Applies to | Effect on the claim |
|---|---|---|
| Seeded worlds | every P2v row | equal in every compared world (at least 900 per function); inputs no world produced were not compared |
| Missing data | every row that reads a missing reference | the <!--f:missing_refs-->19,563<!--/f--> missing references take seeded values, never their real ones ([missing data](missing-data.md)) |
| Callee model | every P2v row; stated in <!--f:assume_write_nothing_rows-->8<!--/f--> ledger notes | a callee that writes through a pointer, or returns what the model never produces, was not compared |
| Declared callee results | rows whose specs declare a callee result (`ovr`) | verified under the declared result |
| Stub write models | rows whose specs declare `models` | verified under the model |
| Pinned words | rows whose specs declare `pins` | verified only for worlds with the pinned values |
| Per-world overlays | <!--f:audit_callee_writes_rows-->11<!--/f--> audit rows whose vector sets "carry per world callee writes" | verified under the per-world values |
| No-return declarations | rows that call a stub declared `noreturn` | return values are not compared |
| Landing pads | <!--f:audit_landing_rows-->15<!--/f--> audited rows and <!--f:close_landing_rows-->1<!--/f--> of the last seven had outcomes only there | exception paths unverified |
| Stack protector rule and rule W | outcomes the audit explains only under them | <!--f:audit_rule_assumption_rows-->0<!--/f--> audited rows pass only under them, and none of the last seven uses them |
| Hand proofs | <!--f:audit_proof_rows-->30<!--/f--> audited rows passed with explained outcomes, <!--f:audit_hand_proof_rows-->17<!--/f--> of them with hand proofs | an argument under the harness model, not a measurement |
| Fault-only proofs (F) | <!--f:close_rows_F-->2<!--/f--> rows, <!--f:close_out_F-->2<!--/f--> outcomes | an outcome that only faulting executions take is explained; maintainer-accepted for these rows ([F](#fault-only-proofs)) |
| Pinned world word in the case line (PN) | <!--f:close_rows_PN-->1<!--/f--> row, <!--f:close_sets_pins-->16<!--/f--> pinned worlds | one outcome is seen only in worlds that carry the pin and end at a no-return call ([PN](#pinned-world-words-in-the-case-line)) |
| Frame disjointness (P2) | <!--f:close_rows_P2-->3<!--/f--> rows, <!--f:close_out_P2-->12<!--/f--> outcomes | stores through the function's own frame never hit the record array ([P2](#frame-disjointness)) |
| Valid sort range (A1) | <!--f:close_rows_A1-->1<!--/f--> row, <!--f:close_out_A1-->4<!--/f--> outcomes | the sorted range is a whole number of records ([A1](#valid-sort-range)) |
| Stub argument counts | <!--f:stub_arity_sites-->48<!--/f--> call sites in <!--f:stub_arity_stubs-->12<!--/f--> stubs, <!--f:stub_arity_fns-->17<!--/f--> functions | the missing argument values were never compared |
| P1 floats | P1 rows with float returns | NaN payload and sign are not compared |

Of the <!--f:fn_total-->2,702<!--/f--> strict rows, <!--f:close_labelled_rows-->6<!--/f--> rest on one of the labelled assumptions F, PN, P2 and A1 (F <!--f:close_rows_F-->2<!--/f--> rows, PN <!--f:close_rows_PN-->1<!--/f-->, P2 <!--f:close_rows_P2-->3<!--/f-->, A1 <!--f:close_rows_A1-->1<!--/f-->; `f_0001b438` carries both P2 and A1, so the four counts add to seven), and <!--f:close_pass_rows-->1<!--/f--> row of the last seven, `f_00030040`, rests on the accepted harness assumptions alone. The maintainer accepted F, PN, P2 and A1 for the rows listed under each heading below and for no others. Their evidence class is `assumed` ([evidence](evidence.md#the-last-seven-rows)).

## Callee model

Callees that are not executed (import stubs, other functions of the dump, indirect targets) are opaque:

| Property | Model |
|---|---|
| Memory | the callee writes nothing, including through the pointers it receives |
| Registers | caller-saved registers are clobbered; `rax`, `rdx` and `xmm0` come back as seeded words: a small value, a heap pointer or a random word, never a stack address |
| Trace | target and arguments are recorded and compared |

A real callee that fills a structure through a pointer, or returns a value the model never produces, could take the function down a path no world exercised. Rows that pass a stack address to a call and read the slot back are the clearest case; their ledger notes say "assumption: verified under opaque callees that write nothing through pointers".

## Declared callee results

A spec may declare that one callee returns one of its arguments plus a constant (`"ret": "arg0+8"`), for example a stream insertion operator that returns the stream it was given. The vector header carries it as an `ovr` line. The shared world adds the constant to the normalised argument, so a stack address comes back as `STK` plus the constant; the `lift-7` and `lift-8` crates return the raw argument register instead, so a stack object passed in keeps its real address.

## Stub write models

A spec may declare a write model for a stub whose role is rated high confidence (`"models": {"stub:0x7b1": "string_ctor"}`): the function and its port apply the same model after the call, for example the libc++ layout of a short or long `std::string` built from a pointer and a length. The crates with a `models` module are `lift-4`, `lift-5` and `lift-6`.

## Pinned words

A spec may pin chosen 8-byte world words at chosen offsets from the anchor to fixed values (kinds `data`, `heap`, `int`) instead of seeded ones, so that a function reaches code that needs a specific table entry or flag. The function and its port read the same pinned values before any store.

## Pinned world words in the case line

A case of a world vector may carry `pin ADDR VALUE` lines: the 8-byte world word at the aligned address ADDR then reads as VALUE in that world, before any store, on both sides. The oracle (`archive:tools/world.py`) and `neutral_world` (`Layout.pins`, filled from the case; `Layout::word` checks the pins before the seeded word) apply the same lookup. A case without a pin line reads exactly as before: the oracle with an empty pin list reproduced 479 stored worlds of earlier sets, the Rust replays of six other functions give the same totals with and without the change, and `crates/neutral-world/tests/pin.rs` checks the parse and the lookup.

Only `f_00056fa6` uses it: the <!--f:close_sets_pins-->16<!--/f--> worlds of set 8 pin the word at 0x1403d8, and stub 0x961 (`___stack_chk_fail`) is declared `noreturn` for that function ([no-return declarations](#no-return-declarations)); its single call site is followed by a landing pad block. The declaration is used for no other function; the other 58 call sites of the stub in the dump were not examined. Set 8 without its pin lines fails in all 16 worlds (the call trace lacks the call to stub 0x961), so the pins are applied.

Difference from [pinned words](#pinned-words): those pins are declared in a function's spec, at offsets from the anchor, hold for every world of the spec and are applied by the `pin` module of the crates that have one; the `pinw` overlays of `lift-7` ([per-world overlays](#per-world-overlays)) are sidecar lines applied by that crate's wrapper. The `pin` line is part of the vector's own case, names an absolute address and is read by `neutral_world` itself, so every crate that loads a vector sees it without a sidecar. All three declare world data, not a callee result, and restrict the claim to worlds with the pinned values; the pinned datum is a missing reference, so any value is a legitimate world.

## Per-world overlays

Some vector sets of the `lift-7` crate carry per-world values in their sidecar, applied to the function and its port alike: `pinw` world words, `retw` call results and `woutc` callee writes into the stack frame after one call. A `woutc` line replaces the callee model's "writes nothing" for the one call it names.

## No-return declarations

A stub may be declared `noreturn` only when its role (throw, terminate, length error, abort) is rated high or medium and the code after every call site is a dead end (padding, `ud2`, `int3` or an unrelated block). The world ends at that call: the call is in the trace, return values are not compared, and the port must return right after the call; continuing shows up in the trace and writes comparison.

## Fault-only proofs

Label F. An unseen outcome counts as explained when every execution that takes it faults, so no compared world can contain it. Two rows use it: `f_00031780` outcome 0x317f3 (fallthrough) and `f_000651c2` outcome 0x6525a (taken). In both, a vector header with zero capacity and an end pointer one or two bytes below the begin pointer leads to a store at the end of the address space (0xffffffffffffffff and 0xfffffffffffffffe), which no world maps; the arithmetic is exact over all 64 bit values and no call lies between the branch and the store. The claim rests on the list of mapped addresses of the harness (code image, entry stack, argument buffers, world pages) and on no callee behaviour.

## Frame disjointness

Label P2. A store whose address is built from `rbp` or `rsp` never hits the record or buffer array that an argument points to. The abstract interpreter already applies it as a model rule (each buffer argument has its own page region; a store through a pointer of unknown origin cannot hit the frame); it was not listed here before. The <!--f:close_out_P2-->12<!--/f--> outcomes that rest on it are flag byte tests with a dominating zero store, in `f_0002405b`, `f_00024a39` and `f_0001b438`; without P2 a frame store could set the byte, but in the attack worlds that place the array on the frame the outcomes occurred only in executions that fault or never complete ([evidence](evidence.md#explanation-kinds-of-the-last-seven-rows)). The ports on such worlds were not replayed.

## Valid sort range

Label A1. For `f_0001b438` the length of the sorted range is a multiple of 248, as a valid caller passes it. It is a precondition on the input, not a harness fact: the function divides by 248 exactly and never checks the remainder, and worlds with an unaligned range complete in the harness (set 14). The <!--f:close_out_A1-->4<!--/f--> outcomes that rest on it (0x1be5e, 0x1d3bc, 0x1ddc9, 0x1e3ba taken) were not reached in 7,491 attack worlds with overlapping records; the two sibling outcomes 0x1c8a3 and 0x1ce51 were reached there and count as seen.

## Landing pads

Blocks reached only through unwind tables (exception landing pads) cannot run in the harness, because no callee ever throws. They are ported as written, listed in the ledger note as `landing pads unverified: N blocks at 0x...`, and removed from the unseen outcomes before an audit verdict. A function is still P2v when every branch outcome outside them is seen. In `f_00030040`, <!--f:close_landing_outcomes-->20<!--/f--> of the 39 explained outcomes are landing pad outcomes (blocks from 0x3056f on).

## Stack protector rule and rule W

| Rule | Assumption |
|---|---|
| Stack protector | the guard slot holds its value at the check; a runaway world could overwrite it |
| Rule W | a displacement loaded from memory and added to an exact frame pointer is not negative |

The audit reports outcomes explained only under these rules apart from the sound ones.

## Hand proofs

Where the abstract interpreter could not decide a branch outcome, a hand proof could: a decided branch or a dead block, with the reason. The audit accepted a hand proof only when re-derived for the row in question, and the soundness pass reports any proof that calls an outcome a compared world observed impossible. Some proofs rest on "callees write nothing".

## Stub argument counts

The lifter decides each call's arity from register dataflow. At these call sites it passed fewer integer arguments than the stub's bank signature declares (`docs/data/namecheck_stub_arity_defects.tsv`). The trace compares only the arguments passed. Fixing it needs the archive ([reproduce](reproduce.md#what-needs-the-archive)).
