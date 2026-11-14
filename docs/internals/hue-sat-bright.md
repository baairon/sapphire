# Internals of `stage apply hue_sat_bright_autogen`

This page describes how this stage is built, rendered, checked and changed in this repository: its program, its stage row, the kernels and device kernels it reaches, the checks that cover it and its limits. It renders the registered style of `S_HueSatBright`. The user reference is the `S_HueSatBright` section of [Effects](../effects.md), the `S_HueSatBright` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture the effects share is in [Effect internals](README.md).

## What it computes

The colour of each pixel is adjusted in four steps. The hue is rotated by `hue_shift`, a fraction of a full turn (`hue_shift`), or, with `preserve_luma` on, rotated so that each pixel keeps its luma (`hue_shift_preserve_luma`). The saturation is then scaled by `saturation` (`sat_scale`). Each channel is next multiplied by a light factor formed from `brightness` times the channel's lane of `scale_colors` and reduced by `offset_darks`, and `offset_darks` is added to it (`*` and `+`). The result is mixed with the source by the mask, which is 1 on the stage unless a tracker mask parameter changes it; the output alpha plane is the source's (MEASURED on the golden synthetic clip, whose alpha varies).

The colour models, as the kernel crates write them: the plain rotation converts each pixel to value, saturation and hue (HSV, with a sign flip for negative values), adds the shift to the hue, wraps it with floor and converts back by sector (module documentation of `crates/effects/kernels/hue_shift/src/lib.rs`). The luma preserving rotation does the same and then compares the clamped luma of the rotated colour with the luma of the source and restores it; its module documentation, `crates/effects/kernels/hue_shift_preserve_luma/src/lib.rs`, gives the luma weights. `sat_scale` mixes each colour with its luma, out = s x colour + (1 - s) x luma(colour), where the luma is a weighted sum computed in double precision with fused steps and rounded to f32 for every pixel (`crates/effects/kernels/sat_scale/src/lib.rs`, lines 4, 5 and 71 to 80).

At the registered defaults every step is an identity, so the compiled plan folds to the source frame: it holds no node and the output equals the input byte for byte (MEASURED). Each step that a parameter leaves at its identity value folds away in the same way (`crates/effects/fx_eval/src/fold.rs`).

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Hue_Sat_Bright_Autogen | `Hue_Sat_Bright_Autogen` | `stage apply hue_sat_bright_autogen` | `temporal` | none | `false` | `-` |

The stage offers 8 parameters (`sapphire stage list --json`); their routes are lines 87 to 94 of `crates/neutral-handoff/data/fx_params.tsv`, all `--set` record values. It offers no opacity popup. `hue_shift`, `saturation`, `brightness`, `offset_darks` and `mocha_mask_opacity` are host sliders that hold a 16.16 value, so the stage puts a plain value on that grid and prints a `note:` line for each value that moves (`scale_colors` is not moved), for example ``note: `hue_shift` 0.419998169 is put on the 16.16 host grid as 0.4199981689453125`` (MEASURED; `host16_slider` in `crates/effects/fx_exec/src/fxrun.rs`, `apply_profile` in `crates/neutral-handoff/src/fxstage.rs`).

Refused answers (MEASURED on this build):

| Input | Result |
|---|---|
| `cset_opacity` or any id the stage does not offer | exit 3, `unknown param` |
| a value outside the printed range, for example `hue_shift=1.5` or `brightness=-1` | exit 3, `bad value for` |
| `scale_colors` with a lane count other than 3 | exit 3, `bad value for`, `expects 3 numbers separated by commas` |
| `--layer` with any id | exit 3, `LAYER-INPUT-UNSUPPORTED` (as for every stage that is not a transition) |
| a frame narrower than 2 pixels | exit 3, `needs a width of 2 or more` |
| a partial input frame | exit 4 |

## Code map

| Part | Path | Role |
|---|---|---|
| Style constructor | `crates/effects/fx_eval/src/programs_native.rs`, `eff_hue_sat_bright_autogen` | the registered program: the mask processing, the inner program, the dissolve with the source by the mask and the tracker mask switch |
| Inner program | same file, `eff_hue_sat_bright` | the hue rotation (plain or luma preserving), the saturation scale, then the light factor and the dark offset |
| Helper programs | same file, `defun_scale_lights_offset_darks`, `defun_scale_lights`, `defun_process_masks`, `defun_mocha_mask`, `defun_combine_masks` | the light factor and the dark offset; the mask and the tracker mask |
| Layout row | same file, `layout_hue_sat_bright_autogen` | parameter grouping of the style |
| Recorded host facts | `crates/effects/rtdata/src/styles/hue_sat_bright.rs` | registered parameters, ranges, input indices and normalize scales |
| Class rows | `crates/effects/rtdata/src/classes.rs` | flag words, argument kinds and combination tables of the classes of the plan |
| `hue_shift` kernel | `crates/effects/kernels/hue_shift` | the hue rotation |
| `hue_shift_preserve_luma` kernel | `crates/effects/kernels/hue_shift_preserve_luma` | the luma preserving rotation; its vector loops divide with an approximate reciprocal and one Newton step, modelled as the exact quotient (APPROX-MODEL) |
| `sat_scale` kernel | `crates/effects/kernels/sat_scale` | the saturation scale; its luma is computed in double precision for every pixel |
| `*` and `+` kernels | `crates/effects/kernels/ops_arith` | the light factor and the dark offset |
| `dissolve` kernel | `crates/effects/kernels/dissolve` | the mix with the source, reached only when the mask is not 1 |
| Folding | `crates/effects/fx_eval/src/fold.rs` | the identity collapses of `hue_shift`, `hue_shift_preserve_luma` and `sat_scale` and the constant folding that empties the plan at the defaults |
| Executor entries | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`; `hue_shift_k`, `hsp_k`, `sat_scale_k`, `binary_k`), `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`; `dissolve_k`) | the call frame of each class into its crate |
| Device entries | `crates/effects/fx_exec/src/device_ops_pointwise.rs` (the entries of `hue_shift`, `hue_shift_preserve_luma`, `sat_scale`, `+`, `*` and `dissolve`) | the device kernel of each op |
| OpenCL sources | `crates/effects/fx_device/src/kernels_pointwise.rs` (`pointwise_hue_f32`, `pointwise_hsp_f32`, `pointwise_sat_f32`, `pointwise_bin_f32`, `pointwise_dissolve_f32`) | kernel text, registered in `crates/effects/fx_device/src/kernels.rs` |
| Stage row | `crates/neutral-handoff/data/fx_stages.tsv` line 9 | program, family, kind, source name, fixed options, host options, placement |
| Parameter rows | `crates/neutral-handoff/data/fx_params.tsv` lines 87 to 94 | id, option form and program parameter of each offered parameter |
| Stage provider | `crates/neutral-handoff/src/fxstage.rs` (`apply_profile`, `run_fx`) | the per-frame runner command line and the in-process run |
| Golden rows | `tools/fx_golden.tsv` lines 87 to 95; the variants in `VARIANTS` of `tools/fx_golden.py` | the special, plain and threaded rows, and the variant rows `+hs` and `+pl` |
| Comparison tools | `crates/effects/fx_exec/src/bin/hsb_cmp.rs`, `crates/effects/fx_exec/src/bin/hsb_e2e.rs` | bit comparisons of the native render with stored planes of the original's runs; built with `fx_exec`, each reads the files named on its command line (usage in the first lines of the source), not `VEC_DIR`, and no test or CI step runs them |

The ops of the compiled plan depend on the parameters (MEASURED with the device report on the worked clip): none at the defaults (`device: none`); `hue_shift_preserve_luma`, `sat_scale` and `*` with the worked command below; `hue_shift`, `sat_scale`, `*` and `+` with the parameters of the golden variant `+hs`; and `hue_shift` and `dissolve` with `hue_shift=0.25` and `mocha_mask_opacity=0.5`, while `bypass_mocha_mask=1` removes the `dissolve` again. No table is computed for this effect.

## Render flow

1. `sapphire stage apply hue_sat_bright_autogen --width W --height H --depth D [--fps N[/D]] [--param ID=V]...` finds the row of `crates/neutral-handoff/data/fx_stages.tsv` and checks, before any frame is read: no `--layer`, the host variants column (`-`), a width of 2 or more, and the parameters against the ranges `fx_run --describe-params` prints. Plain host slider values are put on the 16.16 grid.
2. Each input frame is handed to the runner in memory as 'ae32' words ([Effects provider](../stages.md#effects-provider)).
3. `run_fx` builds the runner's command line: the program, `--size`, the source and output words, `--geom`, `--host-frame` and `--ctx-time` from the frame index, then the `host` column (three source planes, `--src-planes 3`, and the class-3 source route options `--class-record` and `--class3-record`), the empty `placement` column, the `fixed` column (the three mask parameters, and the blur and shift of the tracker mask), the parameters, `--ns-default-unbound` and `--do-postdiv`.
4. The runner builds and compiles the program and folds every constant step. What remains runs per pixel, in this order: the hue rotation, the saturation scale, the `*` of the light factor, the `+` of the dark offset, and the `dissolve` with the source when the mask is not 1. With the three-plane source the output alpha is the source's.
5. Threading: none of these ops splits its rows; `SAPPHIRE_THREADS` and `SAPPHIRE_PAR_MIN` do not change the work, and the threaded golden rows equal the plain rows.
6. Device: with `--device gpu` or `SAPPHIRE_DEVICE=gpu`, each node runs on the device when its op passed the self-check; a NaN output of `hue_shift`, `hue_shift_preserve_luma` or `sat_scale` is rerun on the CPU ([Device path](#device-path)). A plan with no node reports `device: none`.
7. The output words are relaid to the frame format and written. Frame N depends only on input frame N.

`fx_run Hue_Sat_Bright_Autogen --size WxH --out PREFIX ...` runs the same library from a shell, with the host context given explicitly ([Build and run](../effects.md#build-and-run)).

## How parity is judged

An effect counts as rebuilt when each registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means): the program text, the builtin classes, the data read, the literals, the compiled plan, the combination tables and the execution, judged against the original running in an emulator.

| Figure | Value | Source |
|---|---|---|
| verified comparisons | 130 | `docs/data/effects_status.tsv` line 13 |
| recorded renders | 196 | same row |
| re-rendered units | 196, all byte identical to their recorded render | same row; [What rebuilt means](../effects.md#what-rebuilt-means) |
| byte identical through the stage | 118 of 130 counted recorded renders | README [Effects and stages](../../README.md#effects-and-stages) |

No comparison of `S_HueSatBright` differs from the original. The luma preserving rotation uses an approximate reciprocal instruction, so its output may differ on real CPUs; the largest change measured stays within Intel's stated bound, 25,244 ulp (the `S_HueSatBright` section of [Effects](../effects.md), line 555). The model behind the comparisons: both sides computed each reciprocal estimate of the rotation as the exact quotient (APPROX-MODEL), so the native output is that of a CPU whose estimate is exact. A real CPU's estimate lies within Intel's stated relative error; the test `approx_bound_rcp_perturbation` perturbs every estimate by up to that error before the code's own Newton step and prints the largest change of an output in ulp. An ulp scales with the output: between 0.5 and 1 one ulp is 2^-24, so 25,244 ulp is at most about 0.0015 there, under one 8-bit code value and about 1.5 10-bit code values (INFERRED arithmetic). What a given Intel or AMD CPU returns is not measured in this repository.

The other 12 counted recorded renders use a different host arrangement and are not claimed for the stage. Of the 196 recorded renders, 130 are counted and 66 are controls, recorded and not counted ([What rebuilt means](../effects.md#what-rebuilt-means)); which renders are the controls, and the host options in which the 12 differ from the stage row, are kept with the recorded renders outside this repository (docs/stages.md line 341 names the kinds of difference).

## Checks that run from this repository

| Check | Command | Expected result and exit code |
|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | exit 0 |
| test clip and manifest | the first block below | 98,304 bytes; `Hue_Sat_Bright_Autogen S_HueSatBright temporal 8 []`; exit 0 (needs ffmpeg) |
| parameters | `fx_run --describe-params Hue_Sat_Bright_Autogen` | JSON, exit 0 |
| worked render | `sapphire stage apply hue_sat_bright_autogen ...` (below) | three `note:` lines, exit 0, 98,304 bytes |
| identity at the defaults | the default render and `cmp` (below) | `cmp` silent, exit 0 |
| refusals | the refusal block (below) | exit 3 each |
| golden hashes | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0 |
| conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply hue_sat_bright_autogen"` | the two documented 1x1 `FAIL` lines, exit 1 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 ... --device gpu` (below) | `fallback: none`; `device: none` at the defaults; exit 0 (needs OpenCL) |
| device golden | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | 183 equal, exit 0 (needs OpenCL) |
| kernel crates | `cargo test --release --offline -p hue_shift -p hue_shift_preserve_luma -p sat_scale -p ops_arith -p dissolve` | exit 0 |
| APPROX-BOUND | `cargo test --release --offline -p hue_shift_preserve_luma approx_bound_rcp_perturbation -- --nocapture` | an `APPROX-BOUND max ulp change` line, exit 0 |
| evaluator and executor tests | `cargo test --release --offline -p fx_eval --lib fold`, `cargo test --release --offline -p fx_eval --test t8` and `cargo test --release --offline -p fx_exec --test t_int24_ops --test t32_input_class` | exit 0; tests that need stored data print `NOT RUN` (shown with `-- --nocapture`) |
| stage tests | `cargo test --release --offline -p neutral-handoff --test fx_run_state` | exit 0 |
| device tests | `SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_backend -- --test-threads=1` | exit 0 (needs OpenCL) |
| stage parity | `cargo test --release --offline -p neutral-handoff --test fx_stage_parity` | exit 0; with `-- --nocapture` it prints `NOT RUN: VEC_DIR is not set` |

Build, from the repository root:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
```

Expected: exit 0.

A test clip, the manifest entry and the parameters (ffmpeg on `PATH`; without it the clip is not written and the later renders have no input):

```sh
mkdir -p target/demo/hue-sat-bright
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/hue-sat-bright/in.raw
target/release/sapphire stage list --json | python -I -c "import json,sys; s=[x for x in json.load(sys.stdin)['stages'] if x['id']=='hue_sat_bright_autogen'][0]; print(s['name'], s['family'], s['kind'], len(s['params']), s['layers'])"
target/release/fx_run --describe-params Hue_Sat_Bright_Autogen
```

Expected: exit 0 for each; 98,304 bytes in `in.raw`; the line `Hue_Sat_Bright_Autogen S_HueSatBright temporal 8 []`; the parameters as JSON.

The worked render, with values that verified comparisons used, and the render at the defaults:

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param hue_shift=0.419998169 --param saturation=1.1499939 --param brightness=0.800003052 --param preserve_luma=1 < target/demo/hue-sat-bright/in.raw > target/demo/hue-sat-bright/out.raw
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --fps 25/1 < target/demo/hue-sat-bright/in.raw > target/demo/hue-sat-bright/default.raw
cmp target/demo/hue-sat-bright/in.raw target/demo/hue-sat-bright/default.raw
```

Expected: exit 0 for each. The first command prints three `note:` lines, for `hue_shift`, `saturation` and `brightness` put on the 16.16 grid, and writes 98,304 bytes; `cmp` prints nothing, because at the defaults the output equals the input.

Refusals:

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param hue_shift=1.5 < target/demo/hue-sat-bright/in.raw > target/demo/hue-sat-bright/range.raw
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param cset_opacity=2 < target/demo/hue-sat-bright/in.raw > target/demo/hue-sat-bright/opacity.raw
```

Expected: exit 3 for each, with a ``bad value for `hue_shift` `` line that names the range, and with ``unknown param `cset_opacity` ``.

The golden rows of the stage and the full golden check:

```sh
grep '^hue_sat_bright_autogen' tools/fx_golden.tsv | cut -f1,2
python -I tools/fx_golden.py
```

Expected: nine rows, `40x12s`, `50x7`, `50x7+hs`, `50x7+pl`, `50x7t`, `64x32`, `64x32+hs`, `64x32+pl` and `64x32t`; then `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0. The plain, special and threaded rows render at the defaults, where the plan folds to the source frame; the variant rows set colour parameters (`+hs`: hue, saturation, brightness and offset darks; `+pl`: hue with preserved luma) so that the golden check, and the device golden check, reach the colour ops: `+hs` runs `hue_shift`, `sat_scale`, `*` and `+`, and `+pl` runs `hue_shift_preserve_luma` alone (MEASURED with the device report on the golden clip). `dissolve` is reached by no golden row.

Conformance of the stage:

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply hue_sat_bright_autogen"
```

Expected: exit 1 with `L1 stage matrix FAIL 14 passed, 2 failed`, the two `FAIL` lines of the 1x1 cases, and `L3 failure injection pass 33 passed` ([Conformance](../stages.md#conformance)).

The device path (an OpenCL platform such as PoCL; without one each command ends with exit 3 and `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply hue_sat_bright_autogen --device gpu --width 64 --height 32 --depth 32 --fps 25/1 --param hue_shift=0.419998169 --param saturation=1.1499939 --param brightness=0.800003052 --param preserve_luma=1 < target/demo/hue-sat-bright/in.raw > target/demo/hue-sat-bright/dev.raw
cmp target/demo/hue-sat-bright/out.raw target/demo/hue-sat-bright/dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply hue_sat_bright_autogen --device gpu --width 64 --height 32 --depth 32 --fps 25/1 < target/demo/hue-sat-bright/in.raw > target/demo/hue-sat-bright/dev_default.raw
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

Expected: exit 0 for each. The first report prints `device: * 1, hue_shift_preserve_luma 1, sat_scale 1; fallback: none` and the running counts of the next two frames; `cmp` prints nothing; the run at the defaults prints `device: none; fallback: none` for each frame; the golden check prints `183 equal`.

The crate, evaluator, executor and stage tests (cargo, offline):

```sh
cargo test --release --offline -p hue_shift -p hue_shift_preserve_luma -p sat_scale -p ops_arith -p dissolve
cargo test --release --offline -p hue_shift_preserve_luma approx_bound_rcp_perturbation -- --nocapture
cargo test --release --offline -p fx_eval --lib fold
cargo test --release --offline -p fx_eval --test t8
cargo test --release --offline -p fx_exec --test t_int24_ops --test t32_input_class
cargo test --release --offline -p neutral-handoff --test fx_run_state
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_backend -- --test-threads=1
cargo test --release --offline -p neutral-handoff --test fx_stage_parity
```

Expected: exit 0 for each. `approx_bound_rcp_perturbation` perturbs every reciprocal estimate of the luma preserving rotation by up to 1.5 x 2^-12 relative before the code's own Newton step and prints the largest output change as an `APPROX-BOUND max ulp change` line (MEASURED on the port; no hardware). The `fold` unit tests cover the folding rules, `t8` the parameter range table and its clamp rule, `t_int24_ops` a compile of `Hue_Sat_Bright_Autogen`, `t32_input_class` the class-3 source route of its record, and `fx_run_state` the in-process stage runs and the refusal of key times beyond 2^53 on this stage. `device_pointwise` holds the self-check of every pointwise op, these five among them, and `device_backend` renders every registered program on the device and the CPU and compares the bytes. The kernel crates are default members; `fx_eval` and `fx_exec` are not. Without an OpenCL platform the device tests print `NOT RUN` and pass; with `SAPPHIRE_REQUIRE_DEVICE=1` they fail instead. Cargo captures the output of a passing test, so the `NOT RUN` and `T2 PENDING` lines named here show only with `-- --nocapture` after the command; the exit codes are the same either way.

In CI (`.github/workflows/ci.yml`) the `tests on ...` job runs `cargo test --release` (the kernel crates and `neutral-handoff`) and `python -I tools/fx_golden.py` on three systems and the `fx_eval` and `fx_exec` tests on Linux; the `device backend on PoCL` job runs the device tests and the device golden check, which reaches the colour ops through the `+hs` and `+pl` rows.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, and the recorded renders are not in it.

- `crates/neutral-handoff/tests/fx_stage_parity.rs` renders the 118 counted recorded renders that match the stage's host arrangement and compares them by full SHA-256; it needs `VEC_DIR/fx_parity/members.tsv` and an `fx_run`, and prints `NOT RUN` without them.
- `hsb_cmp` and `hsb_e2e`, binaries of `fx_exec`, compare the native render with stored planes and host bytes of the original's runs; those files are not in the repository.
- Which recorded renders are controls and the host options of the 12 counted renders outside the stage are records of the harness, not of this repository.
- Tests of `fx_eval` and `fx_exec` that read `VEC_DIR/effects/` print `NOT RUN` without it ([Tests](../effects.md#tests)).
- The full report of the comparisons is kept outside the repository; its SHA-256 is in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, the frame sizes 64x32 and 50x7 of the stage row's measured placement, the parameter values listed in the `S_HueSatBright` section of [Driving the effect parameters](../parameters-guide.md), the source with three planes and no mask connected. Other sizes and values run the same code and are INFERRED. `brightness` has a registered range whose lower bound is enforced and whose upper record bound is not (`crates/effects/rtdata/src/styles/hue_sat_bright.rs` line 43). The stage refuses a value outside the range it prints; through `fx_run --set`, a value below the lower bound is clamped to it before the plan, as the compiler clamps every enforced bound (`compile_clamps_every_bound_scalar_before_the_plan` in `crates/effects/fx_eval/tests/t8.rs`), and a value above the record bound passes unclamped (`rule_record_bound_of_plus_minus_1e6_is_not_enforced`). Neither is verified.
- The approximate reciprocal of the luma preserving rotation is modelled as the exact quotient on both sides of every comparison; a real CPU's estimate can move the output by up to the bound above (APPROX-MODEL, APPROX-BOUND).
- Host buffer phase: [Limits common to the effects](../effects.md#limits-common-to-the-effects) names no loop of this effect whose order depends on a buffer phase, and its placement column is empty.
- NaN: a NaN output of `hue_shift`, `hue_shift_preserve_luma` or `sat_scale` on the device is rerun on the CPU, which keeps the x86 NaN rules. The special golden row renders at the defaults, where the plan holds no node.
- `ROUTE AVX2-ONLY`: the kernels reproduce the AVX2 and FMA code tier only ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- The mask input is not offered, and mask tracking data is not part of the rebuild; `mocha_mask_opacity` and `bypass_mocha_mask` act on the mask the program builds without a project (the tracker mask group of [Driving the effect parameters](../parameters-guide.md)).
- Depths 8 and 16 convert through f32 and are not verified.
- Refusals and exit codes: the table under [Styles and stages](#styles-and-stages) and [Refusal labels](../cli.md#refusal-labels).

## Device path

| Op | Device kernel | `NAN_EXACT` |
|---|---|---|
| `hue_shift` | `HUE_SHIFT`, `pointwise_hue_f32` | no |
| `hue_shift_preserve_luma` | `HUE_SHIFT_PRESERVE_LUMA`, `pointwise_hsp_f32` | no |
| `sat_scale` | `SAT_SCALE`, `pointwise_sat_f32` | no |
| `*` | `MUL`, `pointwise_bin_f32` | yes, `NAN_EXACT_COMPILED` |
| `+` | `ADD`, `pointwise_bin_f32` | yes, `NAN_EXACT_COMPILED` |
| `dissolve` | its entry, `pointwise_dissolve_f32` | yes, `NAN_EXACT_COMPILED` |

The entries are in `crates/effects/fx_exec/src/device_ops_pointwise.rs` and the kernels in `crates/effects/fx_device/src/kernels_pointwise.rs`. Every op is on `ALLOWLIST` and none has a `CPU_ONLY` entry. A node falls back to the CPU for the reasons of [Rendering on a device](../cli.md#rendering-on-a-device), always with the CPU's bytes. `sat_scale` computes its luma in double precision for every pixel, and its device kernel does the same (`pointwise_sat_f32`, `crates/effects/fx_device/src/kernels_pointwise.rs` line 707); it can run slower on GPUs with a low double precision rate (INFERRED, [Rendering on a device](../guide.md#rendering-on-a-device)). MEASURED on PoCL only: `fallback: none` on the variant golden rows `+hs` and `+pl` (the `S_HueSatBright` section of [Effects](../effects.md)) and on the worked render. The `hue_sat_bright_autogen` row of [Speed](../guide.md#speed) holds the timing of one 1920x1080 frame at the defaults: 0.41 / 0.44 s on the CPU and 0.50 / 0.50 s on PoCL, where the plan holds no node and the report reads `device: none`; most of that time is moving the frame in and out.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (all groups, the `+hs` and `+pl` variants included), the crate tests above, `python -I tools/test_effect_rules.py`, the device tests with `SAPPHIRE_REQUIRE_DEVICE=1`, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- The plain golden rows of this stage render at the defaults and check only the folding to the source frame; a change to a colour kernel shows in the `+hs` and `+pl` rows. A change that needs another parameter combination in the golden check adds a row to `VARIANTS` in `tools/fx_golden.py`.
- A golden difference means the output bytes changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` from the current build and is used only after a change has been judged against the original.
- A change to the CPU kernel of an op removes the op from `ALLOWLIST` unless its device kernel changes in the same patch ([Device](../adding-an-effect.md#device)). A change in `hue_shift_preserve_luma` reruns its APPROX-BOUND test and restates the bound if it moves.
- Effect code calls no platform math library function; two-NaN picks go through `fx_common::nanop` and a fresh NaN is canonicalized ([Crate layout](../adding-an-effect.md#crate-layout)).
- A change to the stage row or the offered parameters updates `crates/neutral-handoff/data/fx_stages.tsv` and `fx_params.tsv`, the `S_HueSatBright` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), and then `python -I tools/gen_docs.py`.

## Open items

- The luma preserving rotation is exact only under the exact reciprocal model; on real CPUs its output can differ from the native output within 25,244 ulp (the `S_HueSatBright` section of [Effects](../effects.md)).
- A brightness below the enforced bound given through `fx_run` is clamped and one above the record bound passes unclamped; neither, nor sizes other than 64x32 and 50x7 or the original's other code tiers, is verified.
- `hsb_cmp` and `hsb_e2e` have no test and no stored-data layout in this repository; they run only with the stored planes of the original's runs.
