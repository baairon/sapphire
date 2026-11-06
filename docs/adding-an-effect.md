# Adding an effect

The conventions every new effect follows, and the parity proof it shows before it is added. The ten effects of [Effects](effects.md) follow them.

## Crate layout

- The effect's ported functions are crates under `crates/effects` (a single port in its own folder under `crates/effects/crates`, a family under `crates/effects/ext`). Each crate is added to `members` in the root `Cargo.toml`.
- A crate whose tests need no stored data and call no platform libm function is also added to `default-members`, so a plain `cargo test --release` runs it. The others stay out, like `fx_eval` and `fx_exec`.
- Effect code calls no platform math library function (`ln`, `log2`, `exp`, `powf`, `sin`, `cos`, `tan`, `atan2` and the like, as methods or as `f32::` and `f64::` paths), because the last bit differs between the C libraries of Linux, macOS and Windows. Use `fx_eval::detmath`, or exact integer code where the crate cannot depend on `fx_eval`. `sqrt`, `abs`, `floor`, `ceil`, `round`, `trunc`, `fract` and `mul_add` are exact IEEE operations and are allowed. `python -I tools/test_effect_rules.py` scans `crates/effects` for these calls outside `detmath.rs` and test code. A deliberate exception is a row of `tools/fx_math_allow.tsv` with its reason, and a row that matches nothing fails the test.
- A fresh NaN is canonicalized to the x86 default NaN (`0xffc00000`) before any operation that reads or flips the sign bit, because an invalid operation gives `0x7fc00000` on ARM. Kernels the executor's rewrite must not touch are listed in `fx_exec::nanrule::NO_OUTPUT_REWRITE`; a kernel on that list canonicalizes its own NaN words, as `sharpen` does in `combine_x86_nan`.
- Nothing is linked from the original plugin, and no file from it is copied into the tree.

## Program constructors

- Each registered style of the effect gets a native constructor function in `crates/effects/fx_eval/src/programs_native.rs`. The constructor builds the style's program graph, together with the helper programs and layout rows it reaches, as Rust constructor code.
- The constructor holds no program text and reads no file. The text parser stays as a test route only.

## Parameter and class facts

- The registered parameters, ranges, input indices, class flags and combination tables the effect needs are Rust definitions in `crates/effects/rtdata`, one file per style under `src/styles/` and shared classes in `src/classes.rs`.
- Every field is named. A definition is kept only if it is keyed by one of the effect's programs, by a class they name, or by a builtin they call. A lookup that has no row still misses.
- Combination tables keep every entry of a used class in its original order, because the first match wins.
- `fx_run --describe-params PROGRAM` prints the new style's parameters from these definitions. Defaults and ranges are never written into docs or data files.

## Computed tables

A mathematical table the effect needs is computed from its definition, at build time or at first use, as the sin and cos tables are ([No external data](effects.md#no-external-data)). A table that no documented formula reproduces is not stored: the code path that would read it refuses with a named refusal, documented as a known limit.

## Tests

- A test that needs stored data (recorded plans, emulator rows, copies of tables) reads it only through `VEC_DIR`, under `VEC_DIR/effects/`, and prints a `NOT RUN` line and passes when it is absent.
- Tests write to the system temporary folder, never into the source tree.
- No test source holds text or table bytes from the plugin.

## Stage rows

- The style's host arrangement is one row of `crates/neutral-handoff/data/fx_stages.tsv`: stage id, program, family, kind, spool, layers, source name, hidden, fixed options, host options and placement by size. The arrangement is taken from the recorded runs of its verified comparisons.
- The kind of an effect stage is `temporal`, so that the stage accepts keyframes. Only a stage that reads other frames needs more than that; `time_warp_rgb_autogen` and `trails_autogen` do.
- Its offered parameters are rows of `crates/neutral-handoff/data/fx_params.tsv`: stage id, parameter id, the `fx_run` option form and option name. Type and lanes come from `fx_run --describe-params`.
- **Golden rows.** Each stage has rows in `tools/fx_golden.tsv` in three groups: plain sizes, the special group (size suffix `s`: infinities, both default NaN patterns, subnormal values and operand pairs that make invalid operations) and the threaded group (suffix `t`: the plain sizes rendered with `SAPPHIRE_PAR_MIN=0` and `SAPPHIRE_THREADS=3`, which must equal the plain rows). After building `sapphire`, `python -I tools/fx_golden.py --write` renders every case and rewrites the table; `python -I tools/fx_golden.py` compares. `python -I tools/test_effect_rules.py` fails when a stage of `fx_stages.tsv` lacks a row in any of the three groups.
- `sapphire stage list --json` then lists the stage, and `python -I tools/conformance.py --stage "PATH/sapphire stage apply STAGE"` passes L1 and L3, with only the documented 1x1 size refusals.

## Device

Every operation the executor dispatches (`fx_exec::kernels::SUPPORTED` and `kernels2::WIRED`) is in exactly one of two tables in `crates/effects/fx_exec/src/device_ops.rs`, and the test `every_op_has_a_device_kernel_or_a_cpu_only_entry` fails when an operation is in neither. A new effect that adds an operation therefore adds one of these:

- **A device kernel.** An entry in `DEVICE_OPS` with `accepts` (the tags, operand shapes and parameter values the kernel covers; everything else runs on the CPU for that node), `run` (the launch) and `corpus` (the self-check cases). The OpenCL C source goes in `crates/effects/fx_device/src` and is registered in `kernels::SOURCES` and `PROGRAM_PARTS`. Then add the class to `ALLOWLIST`.
- **A `CPU_ONLY` entry** with the reason the operation has no kernel. Entries are removed when a kernel lands.

Rules for a kernel:

- **Same bits as the CPU kernel.** The CPU code is the reference: keep its operation order, and use `fma()` only where the CPU code calls `mul_add`. The prelude turns off floating point contraction; `mad`, `native_*` and the fast math options are refused by the lint (`fx_device/src/lint.rs`). `min` and `max` follow the x86 operand order, so no `fmin` or `fmax`. Running sums stay serial in the CPU order.
- **A self-check corpus.** The corpus covers one pixel, odd and wide frames, every combination tag, parameters at their edges and the adversarial values (subnormals, negative zero, infinities, NaN with payloads). At start the executor runs every allowlisted kernel on the corpus and compares it bit for bit with `kernels::run_any`; a difference keeps the operation off the device. Add a mutation check of your own: change one fused operation or one guard in the kernel and confirm the self-check refuses it.
- **Decline instead of approximate.** `accepts` or `run` returns a reason for any shape or value the kernel does not reproduce exactly; the node then runs on the CPU.
- **NaN.** A device output that holds a NaN is rerun on the CPU, which applies the default NaN rule, so a kernel need not reproduce NaN payloads.
- **A CPU parity change** to an operation removes its `ALLOWLIST` entry unless the kernel changes in the same patch.

The self-check results are cached per build, device, driver, kernel sources, build options and loaded program binary ([Rendering on a device](guide.md#rendering-on-a-device)). The build identity is a digest of the running executable, so a change to a CPU kernel, to the device glue or to a corpus invalidates the cache automatically on every machine.

- **Serial device tests.** The device integration tests run one at a time (`-- --test-threads=1`) because some OpenCL runtimes crash when two threads create contexts at once; the library also serializes context, queue, program and kernel creation behind one lock.
- **Double precision corpus.** A kernel with a double precision path needs a corpus that covers the full single precision exponent range (subnormal to the largest finite value), both signs and the inputs one ulp from the points where the double precision result rounds differently.
- **Test files.** The device tests are named by operation family: `device_pointwise`, `device_warp`, `device_ripmap` and `device_neighbourhood` (and `device_backend` for the executor). A new kernel goes in the file of its family.
- **No silent skips in CI.** With `SAPPHIRE_REQUIRE_DEVICE=1` a device test that would print `NOT RUN` panics instead. Without the variable, tests that need a device print `NOT RUN` and pass when no OpenCL platform is present.

## Reference section

Add a section to [Effects](effects.md), in the order of the status table, with the same template as the ten:

1. A heading with the effect's exact bank name in code font.
2. What it does, in plain words of its own.
3. A table of its styles and stages (`stage apply STAGE`).
4. A parameter table per stage: `--param ID=V`, type and lanes, as `sapphire stage list --json` prints them. No defaults or ranges.
5. One worked `sapphire stage apply` command on `in.raw`, with values a verified comparison used, run and ending with exit 0.
6. Parity in plain terms: verified comparisons, recorded renders, re-rendered units, any approximate reciprocal bound, and how many counted recorded renders match the stage's host arrangement. Every figure is labelled MEASURED or INFERRED where it is not a plain count.

Add the effect's row to `docs/data/effects_status.tsv`, run `python -I tools/gen_docs.py`, and move its row from "Next ten" to "Done" in [the roadmap](roadmap.md).

## Parity proof

An effect is added only when all of these hold:

- **Byte identity.** Every recorded render of the effect at depth 32 is reproduced byte for byte by the binary built from the tree, by full SHA-256, through `fx_run` and through the stage for the renders that match its host arrangement.
- **No external data.** The renders hold with no `VEC_DIR`, no `EFFECTS_DATA_DIR` and no table argument; `fx_run` opens only its input and output paths.
- **Plugin-data scan.** The change, as a patch and as files, holds no 64-byte window of any data copied from the plugin; shorter matches are program names only.
- **Thread independence.** The output is byte identical at several `SAPPHIRE_THREADS` values (for example 1, 3 and 12) with `SAPPHIRE_PAR_MIN=0`, which makes every kernel split its rows into bands at every size. This is what the threaded golden rows check.
- **CI green.** Every step of `.github/workflows/ci.yml` passes with no stored data, and `python -I tools/check_docs.py` reports 0 findings.
