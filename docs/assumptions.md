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
| Landing pads | <!--f:audit_landing_rows-->15<!--/f--> audited rows had outcomes only there | exception paths unverified |
| Stack protector rule and rule W | outcomes the audit explains only under them | <!--f:audit_rule_assumption_rows-->0<!--/f--> audited rows pass only under them |
| Hand proofs | <!--f:audit_proof_rows-->30<!--/f--> audited rows passed with explained outcomes, <!--f:audit_hand_proof_rows-->17<!--/f--> of them with hand proofs | an argument under the harness model, not a measurement |
| Stub argument counts | <!--f:stub_arity_sites-->48<!--/f--> call sites in <!--f:stub_arity_stubs-->12<!--/f--> stubs, <!--f:stub_arity_fns-->17<!--/f--> functions | the missing argument values were never compared |
| P1 floats | P1 rows with float returns | NaN payload and sign are not compared |

## Callee model

Callees that are not executed (import stubs, other functions of the dump, indirect targets) are opaque:

| Property | Model |
|---|---|
| Memory | the callee writes nothing, including through the pointers it receives |
| Registers | caller-saved registers are clobbered; `rax`, `rdx` and `xmm0` come back as seeded words: a small value, a heap pointer or a random word, never a stack address |
| Trace | target and arguments are recorded and compared |

A real callee that fills a structure through a pointer, or returns a value the model never produces, could take the original down a path no world exercised. Rows that pass a stack address to a call and read the slot back are the clearest case; their ledger notes say "assumption: verified under opaque callees that write nothing through pointers".

## Declared callee results

A spec may declare that one callee returns one of its arguments plus a constant (`"ret": "arg0+8"`), for example a stream insertion operator that returns the stream it was given. The vector header carries it as an `ovr` line. The shared world adds the constant to the normalised argument, so a stack address comes back as `STK` plus the constant; the `lift-7` and `lift-8` crates return the raw argument register instead, so a stack object passed in keeps its real address.

## Stub write models

A spec may declare a write model for a stub whose role is rated high confidence (`"models": {"stub:0x7b1": "string_ctor"}`): the original run and the port apply the same model after the call, for example the libc++ layout of a short or long `std::string` built from a pointer and a length. The crates with a `models` module are `lift-4`, `lift-5` and `lift-6`.

## Pinned words

A spec may pin chosen 8-byte world words at chosen offsets from the anchor to fixed values (kinds `data`, `heap`, `int`) instead of seeded ones, so that a function reaches code that needs a specific table entry or flag. The original run and the port read the same pinned values before any store.

## Per-world overlays

Some vector sets of the `lift-7` crate carry per-world values in their sidecar, applied to the original run and the port alike: `pinw` world words, `retw` call results and `woutc` callee writes into the stack frame after one call. A `woutc` line replaces the callee model's "writes nothing" for the one call it names.

## No-return declarations

A stub may be declared `noreturn` only when its role (throw, terminate, length error, abort) is rated high or medium and the code after every call site is a dead end (padding, `ud2`, `int3` or an unrelated block). The world ends at that call: the call is in the trace, return values are not compared, and the port must return right after the call; continuing shows up in the trace and writes comparison.

## Landing pads

Blocks reached only through unwind tables (exception landing pads) cannot run in the harness, because no callee ever throws. They are ported as written, listed in the ledger note as `landing pads unverified: N blocks at 0x...`, and removed from the unseen outcomes before an audit verdict. A function is still P2v when every branch outcome outside them is seen.

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
