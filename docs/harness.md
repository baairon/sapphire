# Harness

How a Rust port is compared with the original machine code. The original runs in the Unicorn emulator under the archived Python oracle and world generator; the Rust side is `crates/neutral-parity` (P1 vectors) and `crates/neutral-world` (world vectors), which parse the vector files, run the port on the same inputs and compare. The tiers this produces are in [parity](parity.md).

## Terms

| Term | Meaning |
|---|---|
| oracle | the emulator run of one self-contained function on arguments generated from its spec |
| spec | the JSON description of a function's entry, arguments, returns, call arities and declarations |
| vector set | one vector file of a function; world sets are `world`, `world2`, `world3` and up, each with its own seeds |
| compared world | a world whose case status is `ok`: the original ran to completion; only these are compared and counted |
| skipped world | a world whose case status is not `ok` (the original faulted or timed out); not compared |
| branch outcome | one direction, taken or not taken, of one conditional branch |
| call trace | target, integer arguments and float arguments of every call, in order |
| opaque callee | a callee that is not executed: it writes nothing and returns seeded words |
| Rust fault | a `Fault` or panic raised while the port runs; counts as a failure |
| `STK` | `0x5afe57ac`, the value that stands for any stack address passed to a call |

## Oracle

`archive:tools/oracle.py` runs one self-contained function on arguments generated from its spec, twice per case in fresh emulators, and records the returns, the final buffers and whether callee-saved registers and `rsp` were restored. Pointers stored inside buffers, callbacks, calls that leave the dump, thread-local storage and system calls are not supported, so a function that needs them is never P1.

## World

A function that is not self-contained runs in seeded worlds from `archive:tools/world.py`: everything the dump lacks becomes an input drawn from the world seed, the same for the original and the port.

| Element | Model |
|---|---|
| Memory not in the dump | 8-byte words from `Layout::word(seed, address)`: a world heap pointer with chance `ptrn` in 16 (header field, default 6), a small byte 3 in 16, a 16-bit value 2 in 16, zero 2 in 16, otherwise a random word |
| World heap | 65,536 slots of 64 bytes; page-aligned with header `heap=aligned` |
| Import stubs | the stub zone after the dump reads as `ret` bytes (0xc3) |
| Calls | every call to a stub, to another function of the dump or to an indirect target is trapped, recorded in the call trace, and answered with seeded `rax`, `rdx` and `xmm0` from (seed, target, call index, normalised arguments); the callee writes no memory ([assumptions: callee model](assumptions.md#callee-model)) |
| Fetch into a world page | with header `fetch=byte` only the target byte becomes `ret`; without it the whole page does |
| Stack | the original runs on a zeroed stack; stack reads outside the port's own locals return zero; a stack address passed to a call is normalised to `STK` |
| Registers at entry | registers the harness never writes are 0; `rbx`, `rbp`, `r12` to `r15` hold fixed constants |
| Call result override | `ovr TARGET ARG DELTA`: the declared callee returns normalised argument ARG plus DELTA in `rax` |
| No-return call | a target declared `noreturn` ends the world (`x noreturn`); returns are not compared |

A P2v row passes with at least 900 compared worlds, no mismatch, no Rust fault, and every branch outcome seen in a compared world or soundly explained ([evidence](evidence.md)).

## The Env trait

Every lifted P2v function is generic over `neutral_world::Env`: memory outside the function's own locals and every call go through it. `World` implements it, and a port that touches memory the model does not provide raises a `Fault`.

| Method | Purpose |
|---|---|
| `r8`, `w8` (required) | byte read and write |
| `r16`, `r32`, `r64`, `r128`, `rf32`, `rf64` | little-endian reads built on `r8` |
| `w16`, `w32`, `w64`, `w128`, `wf32`, `wf64` | little-endian writes built on `w8` |
| `call_f(target, iargs, fargs)` (required) | a call with integer arguments and float arguments as raw bits; returns `CallRet { rax, rdx, xmm0_lo, xmm0_hi }` |
| `call(target, iargs)` | `call_f` without float arguments; the slice length is the call's arity |
| `code_va(off)` (required) | address of the code at offset `off` of the dump |
| `data_va(rel)` (required) | address of a target given as `rel_to_end` |
| `stub_va(rel)` | address of an import stub, `data_va` of a non-negative `rel` |

## Vectors

Both formats are plain text, one token line per item. P1 files start with `# oracle vectors v1`; world files start with a `world` header line.

| P1 line | Meaning |
|---|---|
| `fn NAME entry=0x.. args=N rets=N seed=S` | header |
| `case K ok` (or `fault ...`, `timeout`, `NONDETERMINISTIC`) | case status; only `ok` cases are compared; `NONDETERMINISTIC` means the two runs differed |
| `a I int BITS 0x..`, `a I f32 0x..`, `a I f64 0x..`, `a I buf SIZE OFF DIR 0x..` | argument values; a buffer carries its whole initial region |
| `abi ok` | callee-saved registers restored (`abi BAD` otherwise) |
| `r I REG 0x..` | expected return |
| `b I 0x..` | final region of an `out` or `inout` buffer |
| `end` | end of case |

| World line | Meaning |
|---|---|
| `world NAME entry=0x.. n=0x.. seed=0x.. ptrn=N ...` | header; also `anchor`, `stub_zone`, `heap=aligned`, `fetch=byte` |
| `ovr TARGET ARG DELTA` | call result override |
| `case ...` | world status and seed |
| `a ...` | arguments, with buffer pointers |
| `r ...` | expected returns |
| `w 0xADDR 0xBYTES` | one run of final memory writes |
| `t ...` | one call trace event (target, integer and float arguments) |
| `x noreturn 0xTARGET` | the world ended at a declared no-return call |
| `end` | end of case |

## Comparison

| Item | P1 (`neutral_parity::check`) | World (`neutral_world::check`) |
|---|---|---|
| Cases compared | status `ok`, deterministic | status `ok` |
| Returns | each `ret`, narrow values masked to their width | each `ret`, masked; none when the world ended at a no-return call |
| Floats | equal bits, or both NaN (payload and sign not compared) | same rule for `:f32` and `:f64` returns |
| Memory | final region of every `out` and `inout` buffer | coalesced runs of bytes that differ from the initial world, outside the stack |
| Calls | none (calls are not supported) | the full call trace |
| Faults | a panicking port fails the case | a Rust fault fails the world |
| Report | `assert_ok(min)` panics unless every compared case matched and at least `min` were compared; first 20 mismatches kept | the same, with `rust_faults` and `skipped` |

## Replay

The tests of each lift crate (`crates/lift-*/tests/`) replay the stored vector sets of each function against its port. These replay tests are `#[ignore]` ("needs the stored vectors from sapphire_archive.zip") and the lift crates are not default workspace members ([reproduce](reproduce.md)).

| Input | Path the tests read |
|---|---|
| Oracle vectors | `crates/vectors/f_XXXXXXXX.vec` |
| World vector sets | `crates/vectors/f_XXXXXXXX.world.vec`, then `world2`, `world3` and up |
| Sidecars | `.pins` and `.models` files beside a vector set, in the crates with a `pin` or `models` module |
| Code image | `original/neutral_3.text.bin` at the repository root, for reads of the code itself |

| Variable | Effect |
|---|---|
| `WORLD_MIN` | replaces the pass rule's minimum of compared worlds, summed over a function's sets |
| `DBG_SET` | replay one set only: empty for `world`, `2` for `world2`, and so on |
| `DBG_CASE` | replay one world, by index |

## Synthetic cross check

`crates/neutral-world/tests/synthetic.rs` replays vectors that `archive:tools/world.py` produced for small synthetic functions (`crates/neutral-world/tests/fixtures/`, not dump bytes) and checks that the Rust world model reproduces the Python one: data reads, stub calls, opaque callees, indirect calls, pointer chasing, heap writes, fetch then read, no-return calls and call result overrides. It runs in every `cargo test`.

## Aggressive replay and mutation checks

`archive:tools/aggr_run.py` reran a crate's tests on freshly generated vectors (P1 inputs from the coverage-guided fuzzer `archive:tools/fuzz.py`, worlds from new seeds), and `archive:tools/mutate.py` put one deliberate bug at a time into a port to test whether its vectors catch it.
