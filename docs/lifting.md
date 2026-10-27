# Lift crates

How the Rust in the `lift-*` crates was produced from the dump, how the crates are laid out, and the x86 rules the ports reproduce. What reached parity is in [parity](parity.md).

## The input

| Property | Value |
|---|---|
| Dump | the raw x86-64 code section of a build of the plugin: <!--f:code_bytes-->964,659<!--/f--> bytes, <!--f:fn_total-->2,702<!--/f--> functions |
| Content | code with its jump tables embedded; no symbols, strings, constant tables, vtables, import names or headers ([missing data](missing-data.md)) |
| Origin | per the name bank, <!--f:origin_bundled-->1,983<!--/f--> functions are bundled library code, <!--f:origin_std-->514<!--/f--> std, <!--f:origin_undetermined-->136<!--/f--> undetermined and <!--f:origin_component-->69<!--/f--> component code ([names: origin](names.md#origin)) |
| Location | archived with the other dumps and the full disassembly listing; size and SHA-256 in `docs/data/PROVENANCE_sizes.tsv`; no byte of it is in this repository |
| Index | function boundaries, callers, callees, stub calls and data references: `docs/data/index_functions.jsonl`, `docs/data/index_edges.tsv` |

## Two routes

| Route | Functions | Module | Method |
|---|---|---|---|
| Hand port | every P1 function | `p1` | written function by function from the disassembly with the x86 rules below, then checked bit for bit on oracle vectors ([harness](harness.md)) |
| Mechanical lift | every P2v and P2 function | `p2v` | the lifter translates each instruction into Rust and writes a world spec and a test; the port is checked in the world harness |
| Mechanical lift of a P1 function | most P1 functions of `lift-a`, beside their hand ports | `liftcheck` | the same lifter output, checked on the P1 vectors |

## What the lifter writes

| Output | Content |
|---|---|
| `crates/lift-N/src/p2v/<module>.rs` | one `pub fn f_XXXXXXXX` per function, each instruction of the machine code above its Rust as a comment (for example `// aa711 mov rbp,rsp`) |
| world spec `f_XXXXXXXX.world.json` | argument generators, call arities, no-return stubs (archived) |
| `crates/lift-N/tests/world_<module>.rs` (`tests/world.rs` in `lift-a`) | one `#[ignore]` test per function, replaying its stored vectors |

| Runtime | Crates | Function shape | Stack window |
|---|---|---|---|
| `x86rt.rs` | `lift-1` to `lift-8`, `lift-b` | `f_XXXXXXXX<E: Env>(env, ri, fx, st) -> Ret`: integer and float argument registers, stack arguments | 128 KB |
| `lift_rt.rs` | `lift-a` | typed parameters and returns (`p_0: u64`, ...) | 32 KB |

A lifted function is a transcript of the instructions: the 16 general registers, the xmm registers and the flags are Rust locals, the stack frame is a window at the oracle's stack addresses, and every other memory access and every call goes through `neutral_world::Env` (through the crate's `Mem` model of the P1 buffers in `liftcheck`). In `x86rt.rs` flag semantics follow x86, and QEMU for flags x86 leaves undefined. No source structure, types or names are recovered; names live in tables ([names](names.md)).

In the `x86rt.rs` crates a poison dataflow pass decides each call's arity: registers an earlier call clobbered are set to `POISON` and not passed on. A small table of hand fixes covers cases the register dataflow cannot see, such as a register spilled to a stack slot and reloaded. Arities that disagree with the bank's signatures are the [stub argument count gap](assumptions.md#stub-argument-counts).

## Crate layout

Each lift crate is a standalone library: its only dependencies are `neutral-world`, and `neutral-parity` for tests. A package is named after its folder with an underscore: `crates/lift-1` holds `lift_1`. Functions and bytes below count the map rows whose `source` is in the crate.

<!--gen:lift_crates-->
| crate | functions | bytes |
|---|---|---|
| `lift-1` | 1 | 12,884 |
| `lift-2` | 25 | 4,077 |
| `lift-3` | 457 | 104,591 |
| `lift-4` | 396 | 78,619 |
| `lift-5` | 240 | 83,498 |
| `lift-6` | 58 | 200,071 |
| `lift-7` | 84 | 162,396 |
| `lift-8` | 35 | 107,508 |
| `lift-a` | 381 | 40,495 |
| `lift-b` | 1,025 | 170,520 |
<!--/gen:lift_crates-->

| Module | Content |
|---|---|
| `p1` | hand ports of P1 functions (`lift-a`, `lift-b`; empty in the other crates) |
| `p2v` | mechanical lifts of every P2v and P2 function, including all <!--f:unproven_fns-->7<!--/f--> unproven ones |
| `liftcheck` (`lift-a`, feature `liftcheck`) | mechanical lifts of functions that also have a hand port in `p1` |
| `p2`, `dup`, `missing` | no ports; `missing` declares the placeholder names `stub_<hex rel>` and `UNKNOWN_<hex rel>`, which no port uses ([missing data](missing-data.md)) |
| `x86rt` or `lift_rt` | register, flag and stack runtime of the lifted code |
| `cov` (not in `lift-a`) | branch coverage hooks for the parity tests (feature `cov`, off by default) |
| `pin` (`lift-4` to `lift-8`), `models` (`lift-4` to `lift-6`) | pinned world words and stub write models ([assumptions](assumptions.md#pinned-words)) |

| Convention | Rule |
|---|---|
| Symbol | `f_XXXXXXXX`, the 8 hex digits of the function's offset in the dump; never renamed ([names](names.md#rules)) |
| First doc line | `/// parity: TIER @0xOFFSET size=N` in `p1` and `p2v` |
| Safety | `#![forbid(unsafe_code)]` |
| Struct pointers | raw byte views or plain `u64` pointers read at the offsets the function uses, until a layout is established |
| Features | `cov` (coverage) and `trace` (prints pc and registers at every block), or `liftcheck` in `lift-a` |

## Copies of one function

The crates hold <!--f:lift_defs_total-->5,248<!--/f--> `fn f_` definitions for the <!--f:fn_total-->2,702<!--/f--> ids, and <!--f:lift_ids_multi-->1,605<!--/f--> ids are defined more than once, in several crates or in several modules of one crate. Each copy is a separate port of the same function: no lift crate depends on another, nearly every call between functions goes through `Env` rather than into another port, every copy has its own test in its own crate against the stored vectors of its id, and most copies differ in their code (call helpers, coverage hooks, runtime).

`docs/data/FUNCTION_MAP.tsv` names one copy per id in its `source` column: the first file, in path order, of the crate of the row's `owner` that defines the id; the row's tier, strict class, evidence and ledger note belong to that crate's ledger row, and the other copies count in no figure. In `lift-a` that first file is the `liftcheck` copy wherever one exists, not the hand port in `p1` or the world port in `p2v`.

## x86 rules the ports reproduce

| Construct | Rule |
|---|---|
| 32-bit operations | zero-extend into the 64-bit register; 8-bit and 16-bit writes keep the upper bits |
| Integer arithmetic | `wrapping_*` throughout; `imul` high halves through `u128` or `i128` |
| Shifts and rotates | counts masked (`& 31`, `& 63`); `sar` on the signed type |
| `div`, `idiv` | fault on zero or overflow; the oracle records a fault and skips the case |
| `cvttss2si`, `cvttsd2si` | NaN and out-of-range give 0x80000000 (or the 64-bit equivalent), unlike Rust `as`, which saturates |
| `minss`, `maxss` | the second operand when either is NaN or both are zero: `if a < b { a } else { b }`, not `f32::min` |
| `rcpss`, `rsqrtss` | hardware approximations the emulator does not reproduce; such functions cannot be P1 |
| SIMD | scalar loops over lanes with the exact saturation and rounding of each instruction |
| Jump tables | read from the dump and ported as a `match` |
