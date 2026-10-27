# neutral-world

The Rust side of the world harness: a deterministic pseudo-random model of everything the dump lacks (data sections, heap, imports, callees), the `Env` trait every lifted function is written against, the world vector parser, and the comparison of a port with the recorded behaviour of the function it replaces. std only, no dependencies. The harness as a whole: [harness](../../docs/harness.md#world).

## API

| Item | Purpose |
|---|---|
| `Env` | memory reads and writes (`r8` to `r128`, `rf32`, `rf64`, `w8` to `w128`, `wf32`, `wf64`), calls (`call`, `call_f`), addresses (`code_va`, `data_va`, `stub_va`) |
| `World` | the `Env` of the model: seeded words outside the dump, `ret` bytes in the stub zone, a 64-byte-slot heap, zero reads of the stack, seeded call results, recorded writes and call trace |
| `Layout` | the address zones and the seeded word function `word(seed, address)` |
| `STK` | the value a port passes for a pointer into its own stack frame |
| `CallRet` | `rax`, `rdx`, `xmm0_lo`, `xmm0_hi` returned by a call |
| `parse`, `load` | read a world vector file into `WorldVec` (`Header` plus `WCase` per world) |
| `check(vf, code, f)` | run `f` once per compared world and compare returns, final writes and call trace; returns a `Report` |
| `Report::assert_ok(min)` | panic unless every compared world matched and at least `min` were compared |
| `sm64`, `mix` | the hash shared with the Python world generator |

A port that touches memory the model does not provide raises a `Fault`, counted as a Rust fault and a failure, not a panic of the test harness.

## Use

```rust
use neutral_world::{check, load, Env};

let vf = load(std::path::Path::new("f_00010128.world.vec")).unwrap();
let rep = check(&vf, None, |w, c| {
    let r = my_port(w, c.int(0));
    vec![r as u128]
});
rep.assert_ok(900);
```

Pass `Some(&code)` instead of `None` when the port reads the code image itself. The `lift-*` tests need the archived code image and vector sets ([reproduce](../../docs/reproduce.md#what-needs-the-archive)).

## Tests

`cargo test -p neutral-world` runs the unit tests and the synthetic cross check in `tests/synthetic.rs`: small synthetic functions recorded by the Python world generator (`tests/fixtures/*.vec`), replayed through ports written against `Env`. They cover data reads, stub calls, opaque callees, indirect calls, pointer chasing, heap writes, fetch then read, no-return calls and call result overrides, so a change to the model that breaks agreement with the Python side fails here. No archive is needed.
