# neutral-parity

Replays P1 oracle vectors against ported functions: a parser for the plain text vector format and a comparison that reports the first mismatches. It is the check behind every P1 row of the parity figures. std only, no dependencies; the `lift-*` crates use it as a dev-dependency. The vector format and the comparison rules: [harness](../../docs/harness.md#vectors).

## API

| Item | Purpose |
|---|---|
| `parse(text)`, `load(path)` | read a vector file into a `VecFile` (header plus one `Case` per vector) |
| `Case` | status, determinism and ABI flags, arguments, expected returns, expected final buffers |
| `Case::int`, `f32`, `f32_bits`, `f64`, `f64_bits` | argument values by index |
| `Case::buf`, `Case::buf_off` | the whole region a buffer argument received, and the pointer's offset inside it |
| `Outcome` | what the port produced: one value per `ret` (narrow values zero-extended) and the final region of each `out` or `inout` buffer |
| `check(vf, f)` | run `f` on every case whose status is `ok` and that is deterministic, compare returns and buffers, return a `Report` |
| `Report::assert_ok(min)` | panic unless every compared case matched and at least `min` were compared; the first 20 mismatches are kept |
| `f32_same`, `f64_same` | float equality: equal bits, or both NaN (payload and sign are not compared) |

## Use

```rust
use neutral_parity::{check, load, Outcome};

let vf = load(std::path::Path::new("f_000260f4.vec")).unwrap();
let rep = check(&vf, |c| {
    let mut buf = c.buf(0).to_vec();
    let off = c.buf_off(0);
    let r = my_port(&mut buf[off..], c.int(1) as u32);
    Outcome { rets: vec![r as u128], bufs: vec![(0, buf)] }
});
rep.assert_ok(450);
```

The stored vectors of the lifted functions are archived, so the P1 tests of the `lift-*` crates are ignored in a plain checkout ([reproduce](../../docs/reproduce.md#what-needs-the-archive)).

## Tests

`cargo test -p neutral-parity` runs the unit tests of the parser and the comparison on inline vector text. No archive and no external tool is needed.
