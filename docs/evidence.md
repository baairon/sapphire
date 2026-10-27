# Evidence and audit

The files that record each function's tier, status and evidence, their columns, the ledger note grammar, and the audit of the older P2v rows. Tiers and standards are defined in [parity](parity.md#terms).

## Files

| File | Rows | Content |
|---|---|---|
| `docs/data/FUNCTION_MAP.tsv` | one per function | the map: tier, status, strict class, evidence class, crate file |
| `docs/data/ledger_digest.tsv` | one per function | the ledger row behind the map row, with its note |
| `docs/data/EXACT_AUDIT.tsv` | <!--f:exact_audit_rows-->1,702<!--/f--> | the P2v rows recorded under the older any-world accounting, with the start of their note |
| `docs/data/audit_final.tsv` | <!--f:audit_rows-->1,702<!--/f--> | the audit verdict and detail of each of those rows |
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
| `owner` | label of the crate whose ledger row the map keeps; `source` is in that crate |
| `strict` | `p1` (a P1 row), `exact` (a P2v row at strict parity), `unproven` (never matched) |
| `source` | the file of the copy the map names ([lift crates: copies](lifting.md#copies-of-one-function)) |
| `evidence` | the evidence class below |

## Evidence classes

Branch outcomes are counted over compared worlds only (worlds in which the function ran to completion), from complete outcome lists. A P2v row recorded before that rule enters the strictest standard only once a second check passes it. Counts and code shares per class are in [parity](parity.md#standards).

| `evidence` | Meaning |
|---|---|
| `p1` | P1: bit exact on oracle vectors (<!--f:p1_vectors-->117,988<!--/f--> vectors across the <!--f:p1_vector_fns-->236<!--/f--> functions with stored vector files) |
| `second` | P2v: a second check, separate from the row's own ledger entry, replayed its worlds under that rule and recorded PASS; for the rows of `docs/data/EXACT_AUDIT.tsv` that check is the audit below |
| `owner` | P2v: the row's own ledger entry records branch counts over compared worlds; no second check |
| `anyworld` | P2v: counted under the older any-world accounting (outcomes counted in every world, skipped ones included); the audit left it OPEN |
| `-` | none: never matched (unproven) |

Strict parity counts `strict` `p1` and `exact`; the strictest standard counts `evidence` `p1`, `second` and `owner`. Of <!--f:p1_candidates-->606<!--/f--> self-contained P1 candidates, <!--f:p1_downgraded-->57<!--/f--> became P2v because they read pointers loaded from their own buffers or otherwise needed the world harness.

## The audit

Each row of `docs/data/EXACT_AUDIT.tsv` was re-checked one function at a time. The audit passed <!--f:audit_pass-->1,699<!--/f--> of them, now `second`, and left <!--f:audit_open-->3<!--/f--> OPEN, the `anyworld` rows ([parity](parity.md#anyworld-rows)).

| Step | What happens |
|---|---|
| 1. no branches | a function without conditional branches, loops or indirect jumps runs every instruction in each compared world, so PASS needs only 900 or more compared worlds |
| 2. replay | otherwise the vector sets named in the ledger note are replayed with complete coverage; every world's arguments must equal the vector file's, and a set that does not reproduce makes the row OPEN |
| 3. landing pads | outcomes only in blocks unreachable from the entry are removed ([assumptions](assumptions.md#landing-pads)) |
| 4. explanation | an abstract interpreter removes an outcome only when it proves it impossible for every execution the harness can produce; accepted hand proofs remove more ([assumptions](assumptions.md#hand-proofs)) |
| 5. soundness | neither the abstract interpreter nor a hand proof may call impossible an outcome that a compared world observed |
| 6. verdict | PASS when no outcome is left, OPEN when one is left or a set does not reproduce, FAIL with fewer than 900 compared worlds |

Outcomes explained only by the stack protector rule (the guard slot keeps its value until the check) or rule W (a displacement loaded from memory and added to an exact frame pointer is not negative) are counted apart in each detail ([assumptions](assumptions.md#stack-protector-rule-and-rule-w)).

<!--gen:audit_verdicts-->
| verdict | rows |
|---|---|
| OPEN | 3 |
| PASS | 1,699 |
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
| `explained by proof` | decided branches and dead blocks, proved by the abstract interpreter or an accepted hand proof |
| `only under rule W`, `only under the stack protector assumption` | explained only by an assumption rule |
| final number (`N left`) | outcomes left unexplained; any value above 0 makes the row OPEN |
| `ALL-SETS MODE (the engine is not used)` | every outcome was observed in the union of the sets, so the abstract interpreter was not needed |

## ledger_digest columns

| Column | Values |
|---|---|
| `symbol`, `offset`, `owner`, `tier`, `status`, `strict`, `evidence` | as in the map |
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
