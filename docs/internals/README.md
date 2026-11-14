# Effect internals

One page per rebuilt effect, for changing, verifying and extending its code from this repository alone. Each page maps the effect to its code, follows a render from the command line to the output planes, lists every check that covers it with its command and expected result, and states its device path, its limits and what a change must keep green. The user reference is elsewhere: [Effects](../effects.md) (one section per effect, with a worked command and its parity figures), [Driving the effect parameters](../parameters-guide.md), the effects provider section of [Adding a provider](../stages.md#effects-provider) and the [Command-line reference](../cli.md). The conventions a new effect follows are in [Adding an effect](../adding-an-effect.md).

## Contents

- [Shared architecture](#shared-architecture)
- [From a stage command to a kernel](#from-a-stage-command-to-a-kernel)
- [Rules every kernel keeps](#rules-every-kernel-keeps)
- [Checks](#checks)
- [What lives outside the repository](#what-lives-outside-the-repository)
- [Conventions on these pages](#conventions-on-these-pages)
- [Runner options the stage rows set](#runner-options-the-stage-rows-set)
- [Stage figures and widening a stage](#stage-figures-and-widening-a-stage)
- [Stages in the pipeline](#stages-in-the-pipeline)
- [Page template](#page-template)
- [The pages](#the-pages)

## Shared architecture

The effects use no code of the function record ([how the two parts relate](../../README.md#effects-and-stages)). Their code is in these places:

| Part | Path | Role |
|---|---|---|
| Program constructors | `crates/effects/fx_eval/src/programs_native.rs` | One Rust function per program row the styles reach (`def_effect`, `defun`, `layout_effect`). `Corpus::native()` builds the same structures the program text parser builds, node for node, and holds no program text. |
| Compiler | `crates/effects/fx_eval/src/compile.rs` | `compile` turns a program, its parameter values and the combination tables into a compiled plan: an ordered list of nodes, each an operation class with a combination tag and its operands. The per-class collapse and constant folding are in `crates/effects/fx_eval/src/fold.rs`. |
| Recorded host facts | `crates/effects/rtdata` (package `effects_rtdata`) | Rust definitions of the registered parameters, ranges, input indices, class flags and combination tables: one file per style in `src/styles/`, the classes several styles share in `src/classes.rs`. |
| Runner | `crates/effects/fx_exec/src/fxrun.rs` | The library behind the `fx_run` executable (`src/bin/fx_run.rs`) and behind every effect stage: parses the options, builds the program from the native corpus, binds the parameters and host values, compiles, executes and writes or returns the output. |
| Executor | `crates/effects/fx_exec/src/exec.rs` | `compile_for` checks the host context and compiles; `execute_compiled` runs the plan's nodes in order and keeps their values. |
| Kernel dispatch | `crates/effects/fx_exec/src/kernels.rs`, `crates/effects/fx_exec/src/kernels2.rs` | `kernels::SUPPORTED` and `kernels2::WIRED` list every operation class the executor runs; `kernels::run_any` sends a `WIRED` class to `kernels2::run_wired` and every other class to its kernel wrapper. |
| Effect specific executor modules | `crates/effects/fx_exec/src/` | For example `history.rs` (the render history of `S_Trails`), `twframe.rs` (the frame window), `downsampled.rs` (downsampled planes), `opacity_conv.rs` (the opacity answers); each internals page names the ones its effect reaches. |
| Kernel crates | `crates/effects/kernels/<name>` | One crate per ported kernel, class render entry, library routine or member of a kernel family, named by what it computes ([Crate layout](../adding-an-effect.md#crate-layout)). |
| Shared helpers | `crates/effects/fx_common`, `crates/effects/fx_eval/src/detmath.rs` | The x86 operation rules (`fx_common::x86`, `fx_common::nanop`), plane helpers, and the deterministic math routines that replace platform math functions. |
| Threads | `crates/effects/fx_exec/src/par.rs` | Row bands across threads: `SAPPHIRE_THREADS` sets the worker count (1 is serial), `SAPPHIRE_PAR_MIN` the sample count below which a kernel stays serial. A band runs the same per-row operations in the same order, so the output does not depend on the thread count. |
| Device path | `crates/effects/fx_device`, `crates/effects/fx_exec/src/backend.rs`, `crates/effects/fx_exec/src/device_ops.rs` and the other `device_ops_*.rs` files | The OpenCL loader, probe, self-check, caches and kernel sources (`crates/effects/fx_device/src/kernels*.rs`, registered in `fx_device::kernels::SOURCES` and `fx_device::kernels::PROGRAM_PARTS`); the per-operation launch tables `DEVICE_OPS`, `CPU_ONLY`, `ALLOWLIST` and `NAN_EXACT`. |
| NaN rewrite | `crates/effects/fx_exec/src/nanrule.rs` | On a processor whose default NaN differs from x86's, sets the fresh NaN of an invalid operation in a kernel output to the x86 default NaN; the kernels of `NO_OUTPUT_REWRITE` already write x86 words. |
| Stage provider | `crates/neutral-handoff/src/fxstage.rs` | One frames/2 stage per style, from the rows of `crates/neutral-handoff/data/fx_stages.tsv` (stage id, program, family, kind, spool, layers, source name, hidden, fixed options, host options, placement by size, notes, host variants) and `crates/neutral-handoff/data/fx_params.tsv` (the offered parameters). |
| Stored-data helper | `crates/effects/vecdir` (package `effects_vecdir`) | Finds the stored test data under `VEC_DIR`; a test that needs it prints `NOT RUN` and passes when it is absent. |
| Tools | `tools/fx_golden.py` with `tools/fx_golden.tsv`, `tools/conformance.py`, `tools/test_effect_rules.py`, `tools/check_docs.py`, `tools/gen_docs.py` | Golden hashes of every stage, the frames/2 conformance matrix, the source rules of [Adding an effect](../adding-an-effect.md), the page checks and the generated pages. |

## From a stage command to a kernel

1. `sapphire stage apply STAGE` reads whole frames/2 frames on standard input. The stage provider (`crates/neutral-handoff/src/fxstage.rs`) takes the stage's row of `fx_stages.tsv` and its parameter rows of `fx_params.tsv`; the type, lanes, range and registered default of each parameter come from the native definitions through `fx_exec::fxrun::describe_params_json`, the same text `fx_run --describe-params` prints.
2. Per frame, the provider relays the planes into 'ae32' words (A, R, G, B per pixel, f32 bits, straight alpha) in memory and builds the runner's command line: the options of the `host` column, the buffer placement of the `placement` column for the frame size, the options of the `fixed` column, then `--size` and `--geom` from the frame, `--host-frame` and `--ctx-time` from the frame index, the parameters and, when an offered parameter has a host variant at the frame's value, that variant's options ([Effects provider](../stages.md#effects-provider)).
3. The runner (`fx_exec::fxrun`, one runner thread with a large stack per clip) builds the program graph from `Corpus::native()`, binds the parameters through the definitions of `effects_rtdata`, and compiles it with `compile_for`. A host context the run does not give, a value outside the recorded cases or a route that is not rebuilt is refused here, by name.
4. `execute_compiled` runs the plan's nodes in order. Each kernel node is offered to the installed backend first; the CPU backend declines every node, and the executor runs `kernels::run_any`, which calls the kernel crate of the node's class. Kernels split their rows into bands across threads (`par.rs`).
5. With a device installed (`--device`, or `SAPPHIRE_DEVICE` through the pipeline), a node runs on the device only when its operation is on `ALLOWLIST`, passed its self-check on that device, and its kernel `accepts` the combination tag, operand shapes and parameters; otherwise that one node runs on the CPU with the same arguments. A device output that holds a NaN is rerun on the CPU unless the operation is NaN exact on that device ([Rendering on a device](../guide.md#rendering-on-a-device)).
6. The output words are relaid into frames/2 planes and written to standard output. Depth 32 is a pure byte permutation; depths 8 and 16 convert through f32 and are outside the verified render path.

`fx_run PROGRAM --size WxH --out PREFIX ...` enters at step 3 with input planes from files (`--in`, `--synth`) and every host option on its command line. The options are listed in the header of `crates/effects/fx_exec/src/fxrun.rs`, which the executable `crates/effects/fx_exec/src/bin/fx_run.rs` calls ([Build and run](../effects.md#build-and-run)).

## Rules every kernel keeps

- **Host buffer phase.** Some loops of the original choose their summation order, or which elements take the vector form, from the address of a buffer modulo 32 or 64 bytes. The native code takes that phase as an input: 0, or the placement the stage row records for the frame size. Every counted render reaches these loops below their thresholds or at a recorded phase; the thresholds and what holds above them are in [Limits common to the effects](../effects.md#limits-common-to-the-effects), and each internals page names the loops its effect reaches.
- **NaN rules.** The CPU kernels keep the x86 NaN rules: a fresh NaN is the x86 default NaN `0xffc00000`, and an operation whose operands are both NaN returns the first source operand of the original instruction, quieted (`fx_common::nanop`, `fx_common::x86`). `nanrule.rs` makes the bytes the same on processors whose default NaN differs ([Adding an effect](../adding-an-effect.md#crate-layout)). NaN payload and sign bits are outside the verified comparisons ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- **No platform math.** Effect code calls no platform math library function; it uses `fx_eval::detmath` or exact integer code, and `tools/test_effect_rules.py` scans for such calls (`tools/fx_math_allow.tsv` lists the exceptions).
- **Device self-check.** Before an operation runs on a device, its kernel runs on a corpus of adversarial inputs (subnormals, negative zero, infinities, NaN, odd and wide frames, every combination tag) and is compared bit for bit with the CPU kernel; an operation that differs stays on the CPU. Passes are cached per build, device, driver and kernel source ([Device](../adding-an-effect.md#device), [Rendering on a device](../guide.md#rendering-on-a-device)).
- **Thread independence.** The output is byte identical at every `SAPPHIRE_THREADS` value; the threaded golden rows check it at 3 threads with `SAPPHIRE_PAR_MIN=0`.

## Checks

Run from the repository root after the README [Build](../../README.md#build) step. These are the commands of the README [Tests](../../README.md#tests) section; each internals page lists the subset and the extra commands that cover its effect.

```sh
cargo test --release --offline -- --test-threads=2
python -I tools/fx_golden.py
python -I tools/test_handoff.py
python -I tools/test_conformance.py
python -I tools/test_docgen.py
python -I tools/gen_docs.py --check
python -I tools/check_docs.py
python -I tools/test_effect_rules.py
python -I tools/conformance.py --quick
python -I tools/handoff.py doctor --conformance
```

| Command | Expected result |
|---|---|
| `cargo test --release --offline -- --test-threads=2` | exit 0; every test binary ends `test result: ok`. Without ffmpeg the live `neutral-ffmpeg` and pipeline tests print `SKIPPED`, and one `neutral-ffmpeg` test always prints `SKIPPED` ([Tests](../../README.md#tests)) |
| `python -I tools/fx_golden.py` | exit 0; last line `fx_golden: 183 cases, 183 equal, 0 differ` |
| `python -I tools/test_handoff.py`, `python -I tools/test_conformance.py`, `python -I tools/test_docgen.py`, `python -I tools/test_effect_rules.py` | exit 0; last line `OK` |
| `python -I tools/gen_docs.py --check` | exit 0; the line ends `0 drifted, 0 stale, 0 unknown keys` |
| `python -I tools/check_docs.py` | exit 0; first line `check_docs: 0 findings; ...` |
| `python -I tools/conformance.py --quick` | ffmpeg on `PATH`. With twixtor checked out beside the repository, exit 0. Without it, exit 6 and `result: not run (exit 6): 2 passed, 0 failed, 4 not run`; with `--allow-absent-peer` (as CI runs it) exit 0 and `result: pass with absent peer twixtor` |
| `python -I tools/handoff.py doctor --conformance` | exit 0 and `doctor: all required checks passed` with twixtor beside the repository; without it, exit 6 and `[FAIL] conformance: tools/conformance.py --quick exited 6` |

The effect crates outside the default members and the device checks run as the GitHub workflow runs them on Linux (`.github/workflows/ci.yml`). The device commands need an OpenCL platform such as PoCL:

```sh
cargo test --release --offline -p fx_eval -p fx_exec -p edge_aware_sharpen_weights -p edge_aware_sharpen_jobs -p edge_aware_sharpen_blur_core -p edge_aware_sharpen_box_filter -p edge_aware_sharpen_render -p edge_aware_sharpen -p sharpen -p warp_render -p make_mipmap -p rotate2d -p interp -p effects_rtdata -p effects_vecdir
cargo test --release --offline -p fx_device -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_backend --test device_pointwise --test device_warp --test device_ripmap --test device_neighbourhood --test device_glow --test device_glow_ops --test device_footprint --test device_edge --test device_edge_mutants --test device_tilt --test device_tilt_alpha --test device_glint --test device_glint_diag --test device_glint_passes --test device_retime -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE=gpu python -I tools/fx_golden.py
```

| Command | Expected result |
|---|---|
| `cargo test ... -p fx_eval -p fx_exec ...` | exit 0; a test that needs stored data prints a `NOT RUN` line and passes ([Tests](../effects.md#tests)) |
| `cargo test ... -p fx_device` | exit 0; without an OpenCL platform the tests that need a device print `NOT RUN` |
| the two `fx_exec` device commands | exit 0 on an OpenCL platform; with `SAPPHIRE_REQUIRE_DEVICE=1` and no platform, a test that would print `NOT RUN` panics instead and the command fails ([Device](../adding-an-effect.md#device)) |
| `fx_golden.py` on the device | exit 0 and `fx_golden: 183 cases, 183 equal, 0 differ`: the device output equals the CPU hashes |

## What lives outside the repository

- **The comparison harness.** Every verified comparison ran the original plugin's code in an emulator that follows Intel's specification, outside this repository; no file of it is here, and the repository alone cannot rerun a comparison with the original ([What rebuilt means](../effects.md#what-rebuilt-means)).
- **The recorded renders.** The outputs recorded as the expected result of each case are kept outside the repository; the stage figures of the README [Effects and stages](../../README.md#effects-and-stages) table were measured against them. `tools/fx_golden.tsv` holds this repository's own renders of synthetic clips, not the original's.
- **Stored test data.** Tests that compare definitions or computed tables with copies of the plugin's data, or that replay recorded plans and emulator rows, read them under `VEC_DIR/effects/` and print a `NOT RUN` line and pass without it ([Tests](../effects.md#tests)).
- **The full report.** The report with a paragraph per effect and the runs behind it is kept outside the repository; [Evidence](../effects.md#evidence) prints its SHA-256.

## Conventions on these pages

- **MEASURED and INFERRED.** MEASURED marks a statement checked by running code, with its command on the page. INFERRED marks a statement not checked by a run, in one of two senses that the sentence names: a fact about this repository read from its code, data or docs ("INFERRED from the code"), or a statement about the original plugin outside the verified comparisons, such as another size, rate or host phase ("INFERRED", with what was not compared). The definitions are in the conventions of [Driving the effect parameters](../parameters-guide.md).
- **Counts.** `docs/data/effects_status.tsv` stores the three counts of each effect under its own column names: `counted_rows` is the number of verified comparisons, `frozen_members` the number of recorded renders and `landing_rerun_units` the number of re-rendered units; `tools/gen_docs.py` prints them under those three names in the status table of [Effects](../effects.md#what-rebuilt-means) (`tools/docgen/render.py`, the effects status table). The stage figure of each effect, the recorded renders byte identical through its stages, is in the README [Effects and stages](../../README.md#effects-and-stages) table.
- **Code lines.** A code map cites a file and, where it helps, a line on the tree the pages were checked on; a line number can move with a later change, the file and the named item cannot.

## Runner options the stage rows set

The `host`, `placement` and `fixed` columns of `crates/neutral-handoff/data/fx_stages.tsv` hold `fx_run` options that describe the host arrangement of the recorded runs. Each option is parsed in `crates/effects/fx_exec/src/fxrun.rs`, where its comment or its error message states its fields; most have no default, and a call that omits a required one is refused by name ([Build and run](../effects.md#build-and-run)). The terms the internals pages use for them:

| Term | Options | What the runner takes (INFERRED from the code) |
|---|---|---|
| 'ae32' words | `--ae32-src`, `--ae32-prior`, `--ae32-dest`, `--ae32-out` | input and output planes as A, R, G, B words per pixel (f32 bits, straight alpha); the stage hands them over in memory ([Effects provider](../stages.md#effects-provider)) |
| prior fill | `--prior-fill` (host column) | the fill word of the prior frame input of a style, fixed per style; read from the host column by `crates/neutral-handoff/src/fxstage.rs` |
| input class record | `--class-record`, `--class-cells`, `--class3-record` | the recorded class byte of each bound input, the cells of the style's input class record, and for the class 3 source route the plane count builder hits, the image count and the element count of the recorded allocation |
| reader head | `--src-head` | the recorded head word of the source reader, or `unrecorded` |
| opacity arm | `--owner-arm`, `--owner-opacity` | the recorded inputs of the Opacity loop: the arm word, its bit 2 and the exit of the mode byte, and the answer of the program's opacity popup minus 1 (the runner derives `--owner-opacity` from `cset_opacity`) |
| context words | `--ctx-time`, `--ctx-word2`, `--ctx-has-data`, `--bound-mask` | the host context of the call: the frame time, the second context word (word 2 = 1 is compiled by the preserve-group model of `crates/effects/fx_eval/src/preserve.rs`), the data flag and the mask of bound inputs |
| buffer placement, writer phase lists | `--xy-base`, `--xy-base-list`, `--xy-render`, `--persp-base`, `--palpha-base`, `--palpha-cells` | the host buffer phase (bytes modulo 32) of each default-xy writer in execution order, of the perspective plane and of the perspective alpha plane; `--xy-render 1` or `2` names the render whose image a preserve writer reuses ([Writer phase lists](../stages.md#effects-provider)) |
| head form and plane form | none (from the plan) | a default-xy writer allocated outside every exec block is a plane whose rows take their own address phase; one allocated in the line list of the block that writes it is one row buffer (`head_form` in `crates/effects/fx_exec/src/exec.rs`) |
| frame route | `--tw-route`, `--frame-req`, `--frame-words` | seven fields from the run's record: the clip length in frames, the limit cell, the local time step, the host id and three route flags; no field has a default. `--frame-req` gives the host frame that answers a request |
| render history | `--host-times`, `--host-step`, `--host-alloc`, `--host-depth`, `--host-id`, `--host-sizes` | the renders of the history route in order, the local time step, the allocator cell (nonzero present, 0 the free path), the history depth and the host id (the verified render path is ported for id 1); `crates/effects/fx_exec/src/history.rs` decides per render whether the previous destination is kept |

## Stage figures and widening a stage

- **Which test reproduces the stage figures.** `crates/neutral-handoff/tests/fx_stage_parity.rs` renders every stored member of `$VEC_DIR/fx_parity/members.tsv` through `sapphire stage apply` and compares it by full SHA-256; it covers the stages of the ten effects named in [Effects provider](../stages.md#effects-provider) (382 of 382) and prints `NOT RUN` without `VEC_DIR` and an `fx_run`. The figures of the other stages were measured by rendering their counted renders through `sapphire stage apply` with stored data that is not in the repository; no repository test reproduces them. A test for those 14 stages would need their counted members in a stored `members.tsv`; it is an open item.
- **What a stage offers.** A stage runs the host arrangement of the recorded runs of its verified comparisons ([Stage rows](../adding-an-effect.md#stage-rows)): its parameters are the rows of `crates/neutral-handoff/data/fx_params.tsv`, its Opacity answers and other parameter values with other host options are `host_variants` entries, and its sizes beyond the measured ones use the 64x32 placement entry (INFERRED). A value, answer or layer that no recorded run holds is refused by name, for example `SOURCE-VARIANT-MISMATCH` ([Refusal labels](../cli.md#refusal-labels)).
- **Widening a stage.** New recorded runs at a new value come from the comparison harness, outside the repository. With them, the change is: a `host_variants` entry with the options of those runs, a placement entry for a new size or a row of `fx_params.tsv` for a new parameter; golden rows for the new case (the variant group of `tools/fx_golden.py`, then `python -I tools/fx_golden.py --write`, which rewrites `tools/fx_golden.tsv`); and the docs of the effect ([Adding an effect](../adding-an-effect.md)).

## Stages in the pipeline

`sapphire INPUT --stage @ID` runs an effect stage as a `sapphire stage apply` child process with `--width`, `--height`, `--depth`, `--fps` and the parameters, and without `--frames` or `--layer`, so the transition stages `swishpan` and `stage apply dissolve_shake` refuse there with `FRAMES-REQUIRED` (exit 3) and run only through `stage apply` ([the pipeline command](../cli.md#sapphire-input-effect_id-flags)). The stages that read clip frames other than the current one read the whole input clip before they write the first frame: the two `S_Retime` stages and `trails_autogen` ([Stage kinds, spool and keyframes](../cli.md#stage-kinds-spool-and-keyframes)) and `time_warp_rgb_autogen` (the temporal stages paragraph of [Adding a provider](../stages.md#effects-provider)); each internals page states what its stages hold in memory.

## Page template

Each internals page has these sections, in this order; a section with nothing to state says so in one sentence.

1. What it computes: the operation on the image.
2. Styles and stages: style, program, stage, kind, layers, spool and host variants; the refused styles and answers with their labels and exit codes.
3. Code map: constructors, `effects_rtdata` style files, the plan's operations and their kernel crates, executor entries, device entries, stage and parameter rows, computed tables, tests.
4. Render flow: from `sapphire stage apply` and from `fx_run` to the output planes.
5. How parity is judged: the verified comparisons, recorded renders and re-rendered units, the stage figure, and every comparison that differs from the original.
6. Checks that run from this repository: command, expected result and exit code.
7. What needs the comparison harness: the checks that cannot run from the repository and the tests that print `NOT RUN`.
8. Verified scope and limits: sizes, rates, host times, host buffer phase loops, NaN rules, routes not rebuilt, refusals.
9. Device path: per operation, its kernel or `CPU_ONLY` entry, `NAN_EXACT` membership, fallback and what is MEASURED.
10. Changing the code safely: the gates, what a golden difference means, and the docs and data to update.
11. Open items: technical limits a later change could address, each with its source.

## The pages

Each page below follows the template above. The crates in the last column are named in the code maps of this page and of at least one other page; the operator crates (`ops_*`), which most plans use, are left out. A part that several effects share is described once and linked from the others: the blur kernel on [blur.md](blur.md), the ripmap and the relative warp sampler on [diffuse.md](diffuse.md), the motion blur sampler on [blur-motion.md](blur-motion.md), the shake offset on [shake.md](shake.md), the glow helpers and `screen_super` on [glow.md](glow.md), and the frame route of the clip requests on [time-warp-rgb.md](time-warp-rgb.md).

| Effect | Page | Stages | Kernel crates also used by another effect |
|---|---|---|---|
| `S_Blur` | [blur.md](blur.md) | `stage apply blur_color_autogen`, `stage apply blur_mono_autogen` | `blur`, `blur_coef`, `blur_rows`, `blur_rows_y`, `interp`, `monochrome_a` |
| `S_BlurMoCurves` | [blur-mo-curves.md](blur-mo-curves.md) | `stage apply transform_and_blur_autogen`, `stage apply blur_motion_curves_no_transform_autogen` | `blur_warp`, `make_mipmap` |
| `S_BlurMotion` | [blur-motion.md](blur-motion.md) | `stage apply blur_motion_autogen`, `stage apply blur_motion_mono_autogen` | `blur_warp`, `blur_warp_1ch`, `blur_warp_4ch`, `libm_fmod`, `libm_sincos`, `make_mipmap`, `monochrome_a`, `rotate2d` |
| `S_Diffuse` | [diffuse.md](diffuse.md) | `stage apply diffuse_autogen` | `libm_fmod`, `make_mipmap`, `warp_rel`, `warp_rel_sampler`, `warp_sampler` |
| `S_DissolveShake` | [dissolve-shake.md](dissolve-shake.md) | `stage apply dissolve_shake` | `blur_warp`, `dissolve`, `interp`, `make_mipmap` |
| `S_DistortRGB` | [distort-rgb.md](distort-rgb.md) | `stage apply distort_rgb_autogen` | `blur`, `make_mipmap`, `monochrome`, `rotate2d`, `v3_to_v4_if4`, `warp_log_gradient`, `warp_rel` |
| `S_DropShadow` | [drop-shadow.md](drop-shadow.md) | `stage apply drop_shadow` | `blur`, `dissolve`, `make_mipmap`, `warp_rel`, `warp_rel_sampler`, `warp_sampler` |
| `S_Flicker` | [flicker.md](flicker.md) | `stage apply flicker` | none |
| `S_Glint` | [glint.md](glint.md) | `stage apply glint` | `alpha_from_rgb_max`, `blur`, `glow_layer`, `hue_shift`, `hue_shift_preserve_luma`, `interp`, `screen_super` |
| `S_Glow` | [glow.md](glow.md) | `stage apply glow` | `alpha_from_rgb_max`, `blur`, `blur_coef`, `blur_rows`, `blur_rows_y`, `comp_rgba`, `get_alpha`, `glow_layer`, `interp`, `overlay_super`, `screen`, `screen_super`, `v3_to_v4_if4` |
| `S_GlowEdges` | [glow-edges.md](glow-edges.md) | `stage apply glow_edges` | `alpha_from_rgb_max`, `blur`, `blur_coef`, `blur_rows`, `blur_rows_y`, `comp_rgba`, `get_alpha`, `interp`, `monochrome`, `overlay_super`, `screen`, `screen_super`, `v3_to_v4_if4` |
| `S_HueSatBright` | [hue-sat-bright.md](hue-sat-bright.md) | `stage apply hue_sat_bright_autogen` | `dissolve`, `hue_shift`, `hue_shift_preserve_luma`, `sat_scale` |
| `S_Retime` | [retime.md](retime.md) | `stage apply retime_speed`, `stage apply retime_longer` | `dissolve`, `make_mipmap`, `monochrome`, `warp_rel`, `warp_rel_sampler` |
| `S_Shake` | [shake.md](shake.md) | `stage apply shake_normal_autogen`, `stage apply shake_twitchy_autogen`, `stage apply shake_jumpy_autogen` | `blur_warp`, `blur_warp_1ch`, `blur_warp_4ch`, `get_alpha`, `interp`, `make_mipmap`, `rotate2d`, `warp_log_gradient`, `warp_render` |
| `S_Sharpen` | [sharpen.md](sharpen.md) | `stage apply sharpen_best_quality_autogen`, `stage apply sharpen_fast_autogen` | `blur`, `monochrome` |
| `S_SwishPan` | [swish-pan.md](swish-pan.md) | `stage apply swishpan` | `blur`, `blur_coef`, `blur_rows`, `blur_rows_y`, `make_mipmap`, `screen_super`, `warp_driver`, `warp_log_gradient`, `warp_render`, `warp_sampler` |
| `S_TimeWarpRGB` | [time-warp-rgb.md](time-warp-rgb.md) | `stage apply time_warp_rgb_autogen` | `dissolve`, `sat_scale` |
| `S_Trails` | [trails.md](trails.md) | `stage apply trails_autogen` | `blur`, `blur_coef`, `blur_rows`, `blur_rows_y`, `comp_rgba`, `dissolve`, `hue_shift`, `screen_super` |
| `S_WarpPerspective` | [warp-perspective.md](warp-perspective.md) | `stage apply warp_perspective_autogen` | `libm_sincos`, `make_mipmap`, `rotate2d`, `warp_driver`, `warp_log_gradient`, `warp_render`, `warp_sampler` |
| `S_WarpTransform` | [warp-transform.md](warp-transform.md) | `stage apply warp_transform_autogen` | `libm_sincos`, `make_mipmap`, `warp_driver`, `warp_log_gradient`, `warp_render`, `warp_sampler` |
