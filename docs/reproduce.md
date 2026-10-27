# Reproduce

What reruns from this repository alone, what needs the archive `sapphire_archive.zip`, and how the parity evidence is re-created from it.

## What reruns here

Without the archive the code builds, the default members test, and the documentation regenerates and checks ([README: tests](../README.md#tests)). The `lift-*` tests stay ignored until the stored vectors are placed. One more command covers the lift crates:

| Command | Checks |
|---|---|
| `cargo check --workspace --all-targets --offline` | every crate compiles, including the <!--f:lift_crates-->10<!--/f--> `lift-*` crates and their ignored tests |

On a Windows machine without the MSVC linker the GNU toolchain builds the same code: `cargo +stable-x86_64-pc-windows-gnu check --workspace --all-targets --offline`, and the same `+stable-x86_64-pc-windows-gnu` for every cargo command on this page.

Every figure in the documentation regenerates from `docs/data/`; `docs/data/MANIFEST.tsv` holds the SHA-256 of each data file.

## What needs the archive

| Item | Used for |
|---|---|
| the dumps, among them `original/neutral_3.text.bin`, and the full disassembly listing | the oracle and world generator run the dump in the emulator; the lift tests read it for reads of the code itself |
| stored vectors (`f_XXXXXXXX.vec`, `f_XXXXXXXX.world.vec`, `world2`, `world3`) | the `lift-*` tests |
| specs (`f_XXXXXXXX.json`, `f_XXXXXXXX.world.json`) | generating vectors with new seeds |
| the oracle and world generator (`archive:tools/oracle.py`, `archive:tools/world.py`) | running a function of the dump on a spec and recording its vectors |
| the replay tools (`archive:tools/fuzz.py`, `archive:tools/aggr_run.py`, `archive:tools/mutate.py`, `archive:scripts/fullcov.py`) | coverage-guided P1 inputs, a crate's tests on fresh vectors, mutation checks, complete branch coverage |
| the lifter, the abstract interpreter and the audit scripts | regenerating ports and audit verdicts |
| the emulator | Unicorn 2.1.4 in a Python virtual environment; not a dependency of this repository |
| the full ledgers, hand proofs and coverage files | the notes behind `docs/data/ledger_digest.tsv` |

Without these, the parity figures stand as recorded evidence: they can be read and regenerated into documents, but not re-measured.

## Re-creating the evidence

With the archive unpacked next to a checkout:

| Step | Action |
|---|---|
| 1. dump | place the dump at `original/neutral_3.text.bin` and check its SHA-256 against `docs/data/PROVENANCE_sizes.tsv` |
| 2. vectors | place the stored vectors in `crates/vectors/`, where the lift tests read them |
| 3. replay | `cargo test --release --offline -p lift_1 -- --ignored --test-threads=2`, one lift crate at a time: packages `lift_1` to `lift_8`, `lift_a`, `lift_b` (`--features liftcheck` adds the `liftcheck` tests of `lift_a`) |
| 4. fresh worlds | from the unpacked archive, in its virtual environment: `python tools/world.py gen ROOT DUMP SPEC CASES SEED OUT`, with ROOT the archive folder and DUMP the base name of the dump's file; then step 3 on the new vectors |
| 5. coverage | in the same place: `python -I scripts/fullcov.py -j K VEC...` rewrites each vector file's `.cov.json` with the complete list of unseen branch outcomes |

A world test binary can hold gigabytes of vectors per thread: keep two test threads, run one crate at a time, and start only with enough free memory.
