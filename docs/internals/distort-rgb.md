# Internals of `stage apply distort_rgb_autogen`

This page describes how this stage is built, rendered, checked and changed in this repository: its program, its stage row, the kernels and device kernels it reaches, the checks that cover it and its limits. It renders the registered style of `S_DistortRGB`. The user reference is the `S_DistortRGB` section of [Effects](../effects.md), the `S_DistortRGB` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture the effects share is in [Effect internals](README.md). The ripmap (`make_mipmap`) and the relative warp sampler (`warp_rel`) are described in [`S_Diffuse` internals](diffuse.md); this page adds the footprint path the sampler takes here.

## What it computes

The lens image, which on the stage is the source itself, is reduced to one monochrome plane, blurred by the class `blur` with a width set by `blur_lens`, and its x and y gradient is taken (`grad`); the `grad` call frame derives the blur width and the gradient gain from `blur_lens` and the frame, and its module documentation, `crates/effects/kernels/grad/src/lib.rs`, gives both formulas. The gradient is scaled by a strength, `amount` times the sum of `blur_lens` and a fixed offset (the `scale` binding of `eff_distort_rgb`), by `amount_rel` per axis and by the mask, and rotated by `rotate_warp_dir` when that angle is not 0. Each colour channel is then built into a ripmap of its own and sampled by the relative warp at the pixel position plus the gradient times that channel's share (`warp_red`, `warp_green`, `warp_blue`), with the edge rule of `wrap` per axis; with `filter` on, the sampler picks a ripmap level per pixel from the local size of the displacement (the footprint path). The three warped channels are assembled into the output, whose alpha plane is the source's (MEASURED on the golden synthetic clip, whose alpha varies).

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Distort_RGB_Autogen | `Distort_RGB_Autogen` | `stage apply distort_rgb_autogen` | `temporal` | none | `false` | `-` |

The stage offers 10 parameters (`sapphire stage list --json`); their routes are lines 67 to 76 of `crates/neutral-handoff/data/fx_params.tsv`. `blur_lens` is a host value (`--host-param`), `cset_opacity` a host popup answer (`--cset`, display name `Opacity`), and the others are record values (`--set`).

Refused answers (MEASURED on this build unless marked):

| Input | Result |
|---|---|
| `cset_opacity=1` or `3` | exit 3, `SOURCE-VARIANT-MISMATCH`: the stage's host arrangement covers only the answer 2, Normal (MEASURED with 1; 3 per [Refusal labels](../cli.md#refusal-labels)) |
| `cset_opacity` outside 1 to 3 | exit 3, `bad value for` |
| `--layer Lens=...` or `--layer To=...` | exit 3, `LAYER-INPUT-UNSUPPORTED`, before any frame is read |
| a frame narrower than 2 pixels | exit 3, `needs a width of 2 or more` |
| a frame with infinities and NaN in the source, as the special golden row `40x12s` | exit 1, `stage_failed`: `ERR kernel warp_rel failed: F2(Oob)`; the sampler meets a tap outside the ripmap plane and no frame is written |
| a partial input frame | exit 4 |

`cset_opacity=2` renders the same bytes as an unset `cset_opacity` (MEASURED on the test clip).

Very large finite values end the same way as the special row: a source whose colour columns alternate between 0 and 1e30 at 64x32 ends with `F2(Oob)` and exit 1, while 1e6 and 1e12, and `amount` 1000 and 100000 on the test clip, render (MEASURED). The error is the index check of the sampler (`F1Error::Oob` in `crates/effects/kernels/warp_sampler/src/lib.rs`): a tap whose index falls outside the ripmap planes ends the kernel instead of reading outside them. What the original does with such input is not in this repository.

## Code map

| Part | Path | Role |
|---|---|---|
| Style constructor | `crates/effects/fx_eval/src/programs_native.rs`, `eff_distort_rgb_autogen` | the registered program: the mask processing, the inner program and the tracker mask switch |
| Inner program | same file, `eff_distort_rgb` | the gradient of the blurred monochrome lens image, its scale and rotation, the per-channel relative warps and the channel assembly |
| Helper programs | same file, `defun_process_masks`, `defun_mocha_mask`, `defun_combine_masks`, `defun_warp_mask` | the mask input and the tracker mask |
| Layout row | same file, `layout_distort_rgb_autogen` | parameter grouping of the style |
| Recorded host facts | `crates/effects/rtdata/src/styles/distort_rgb.rs` | registered parameters, ranges, input indices and normalize scales |
| Class rows | `crates/effects/rtdata/src/classes.rs` | flag words, argument kinds and combination tables of the classes of the plan |
| `monochrome` kernel | `crates/effects/kernels/monochrome` | the luma plane of the lens image |
| `grad` kernel | `crates/effects/kernels/grad`, using `crates/effects/kernels/blur` | three blur passes of the plane, then the x and y differences scaled by the gain |
| `rotate` kernel | `crates/effects/kernels/rotate2d` | the rotation of the gradient vectors, only when `rotate_warp_dir` is not 0 |
| `element_of` kernel | `crates/effects/kernels/ops_element` | one channel of the source |
| `make_mipmap` kernel | `crates/effects/kernels/make_mipmap` | the ripmap of each channel ([`S_Diffuse` internals](diffuse.md)) |
| `warp_rel` kernel | `crates/effects/kernels/warp_rel` and its samplers; `crates/effects/kernels/warp_log_gradient` | the relative warp; on the footprint path the per-pixel level map of the coordinate planes |
| `*` and `v3` kernels | `crates/effects/kernels/ops_arith`, `crates/effects/kernels/ops_vec` | the per-channel share of the displacement, and the three-channel assembly |
| `postdivide` | `crates/effects/fx_exec/src/postdiv.rs` | the last node of the host route; `postdivide_k` takes its copy-alpha arm when the `copy_alpha` operand is 0 and the element and output counts are 3 (`ROUTE_COPY_ALPHA`, `ROUTE_ELNUM` and `ROUTE_NOUT`, lines 161 to 163), and that arm writes the source's alpha plane |
| Executor entries | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`: `*`, `element_of`, `v3`), `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`: `monochrome_k`, `grad_k`, `rotate_k`, `make_mipmap_k`, `warp_rel_k`, `postdivide_k`; `footprint_taken`) | the call frame of each class into its crate |
| Device entries | `crates/effects/fx_exec/src/device_ops_pointwise.rs` (the entries of `monochrome`, `element_of`, `v3`, `postdivide` and `*`), `crates/effects/fx_exec/src/device_ops_neighbourhood.rs` (`GRAD`), `crates/effects/fx_exec/src/device_ops_warp.rs` (`warp_rel_run`, `rotate_run`), `crates/effects/fx_exec/src/device_ops_ripmap.rs` (`make_mipmap_run`), `crates/effects/fx_exec/src/device_ops_footprint.rs` (the footprint prefill and warp) | the device kernel of each op |
| OpenCL sources | `crates/effects/fx_device/src/kernels_pointwise.rs`, `crates/effects/fx_device/src/kernels_neighbourhood.rs` (`NEIGHBOURHOOD_BLUR`, `grad_stencil_f32`), `crates/effects/fx_device/src/kernels_warp.rs` (`warp_warp_f32`, `warp_rotate_f32`), `crates/effects/fx_device/src/kernels_ripmap.rs`, `crates/effects/fx_device/src/kernels_footprint.rs` (`FOOTPRINT_PREFILL`, `FOOTPRINT_WARP`) | kernel text, registered in `crates/effects/fx_device/src/kernels.rs` |
| Stage row | `crates/neutral-handoff/data/fx_stages.tsv` line 7 | program, family, kind, source name, fixed options, host options, placement |
| Parameter rows | `crates/neutral-handoff/data/fx_params.tsv` lines 67 to 76 | id, option form and program parameter of each offered parameter |
| Stage provider | `crates/neutral-handoff/src/fxstage.rs` (`apply_profile`, `run_fx`) | the per-frame runner command line and the in-process run |
| Golden rows | `tools/fx_golden.tsv` lines 45 to 49 | the special row `40x12s` (recorded as `rc1`), the plain and the threaded rows at 50x7 and 64x32 |

The ops of the compiled plan, per frame (MEASURED with the device report below): `monochrome` 1, `grad` 1, `*` 2 to 4, `element_of` 3, `make_mipmap` 3, `warp_rel` 3, `v3` 1 and `postdivide` 1, plus `rotate` 1 when `rotate_warp_dir` is not 0. The number of `*` nodes depends on which shares fold to constants: 2 at the registered defaults, 4 with the worked command's values. The alpha warp of the inner program and the `v3_to_v4_if4` assembly are pruned, because the source has three planes on this host arrangement. The mask processing folds away with the unconnected mask. No table is computed for this effect.

## Render flow

1. `sapphire stage apply distort_rgb_autogen --width W --height H --depth D [--fps N[/D]] [--param ID=V]...` finds the row of `crates/neutral-handoff/data/fx_stages.tsv` and checks, before any frame is read: no `--layer`, the host variants column (`-`), a width of 2 or more, the parameters against the ranges `fx_run --describe-params` prints and the opacity answer against 1 to 3.
2. Each input frame is handed to the runner in memory as 'ae32' words, as for every effect stage ([Effects provider](../stages.md#effects-provider)).
3. `run_fx` builds the runner's command line: the program, `--size`, the source and output words, `--geom`, `--host-frame` and `--ctx-time` from the frame index, then the `host` column (three source planes, `--src-planes 3`, and the class-3 source route options: `--class-record Source=3`, the recorded class byte of the bound source, class 3 for a three-plane input, and `--class3-record`, the recorded plane-count builder hits, image count, element count and a program cell; the runner compares both with what it derives and refuses a difference by name, `crates/effects/fx_exec/src/fxrun.rs` lines 2152 to 2166), the empty `placement` column, the `fixed` column (the three mask parameters, and the blur and shift of the tracker mask), the parameters, `--ns-default-unbound` and `--do-postdiv`.
4. The runner builds and compiles the program. The `Lens` input is unbound, and its text default names `Source`, so it takes the source's planes (`INPUT-DEFAULT-ALIAS`, tested by `crates/effects/fx_exec/tests/t26_input_alias.rs`). The plan runs `monochrome`, `grad`, the `*` of the scale, `rotate` when set, then for each colour channel `element_of`, `make_mipmap`, a `*` of the channel's share and `warp_rel`, then `v3` and `postdivide`.
5. `warp_rel` takes the footprint path when its filter operand truncates to a non-zero integer and the coordinates are an image (`footprint_taken` in `crates/effects/fx_exec/src/kernels2.rs`): a per-pixel map of the log2 of the local coordinate gradient (`crates/effects/kernels/warp_log_gradient`) selects the ripmap level of each sample. With `filter` 0 it samples without the map.
6. Threading: `warp_rel` splits its rows into bands (`SAPPHIRE_THREADS`, `SAPPHIRE_PAR_MIN`, `crates/effects/fx_exec/src/par.rs`), and the blur inside `grad` splits its planes across threads by the same two variables (`crates/effects/kernels/blur/src/lib.rs`); the bytes do not depend on the thread count, which the threaded golden rows check.
7. Device: with `--device gpu` or `SAPPHIRE_DEVICE=gpu`, every node runs on the device when its op passed the self-check, and the ripmaps and the footprint maps stay on the device ([Device path](#device-path)).
8. The output words are relaid to the frame format and written. Frame N depends only on input frame N.

`fx_run Distort_RGB_Autogen --size WxH --out PREFIX ...` runs the same library from a shell, with the host context given explicitly ([Build and run](../effects.md#build-and-run)).

## How parity is judged

An effect counts as rebuilt when each registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means): the program text, the builtin classes, the data read, the literals, the compiled plan, the combination tables and the execution, judged against the original running in an emulator.

| Figure | Value | Source |
|---|---|---|
| verified comparisons | 30 | `docs/data/effects_status.tsv` line 7 |
| recorded renders | 30 | same row |
| re-rendered units | 30, all byte identical to their recorded render | same row; [What rebuilt means](../effects.md#what-rebuilt-means) |
| byte identical through the stage | 26 of 30 counted recorded renders | README [Effects and stages](../../README.md#effects-and-stages) |

All 30 comparisons are equal: 28 are byte identical, and in 2 the output holds NaN values, which are equal as NaN values while their payload bits are not compared. No approximate reciprocal instruction is used on the verified render path (MEASURED, the `S_DistortRGB` section of [Effects](../effects.md)). The inputs of the two NaN renders are kept with the recorded renders outside this repository. All 30 recorded renders are counted, with no control. The other 4 use a different host arrangement and are not claimed for the stage; the host options in which they differ from the stage row, and the arrangement a host application uses, are records of the harness, not of this repository (docs/stages.md line 341 names the kinds of difference).

## Checks that run from this repository

| Check | Command | Expected result and exit code |
|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | exit 0 |
| test clip and manifest | the first block below | 98,304 bytes; `Distort_RGB_Autogen S_DistortRGB temporal 10 []`; exit 0 (needs ffmpeg) |
| parameters | `fx_run --describe-params Distort_RGB_Autogen` | JSON, exit 0 |
| worked render | `sapphire stage apply distort_rgb_autogen ...` (below) | exit 0; 98,304 bytes |
| refusals | the refusal block (below) | exit 3, 3 and 1 |
| golden hashes | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0 |
| conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply distort_rgb_autogen"` | the two documented 1x1 `FAIL` lines, exit 1 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 ... --device gpu` (below) | `fallback: none` on every frame, output equal to the CPU output, exit 0 (needs OpenCL) |
| device golden | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | 183 equal, exit 0 (needs OpenCL) |
| kernel crates | `cargo test --release --offline -p monochrome -p grad -p rotate2d -p ops_element -p ops_vec -p ops_arith -p make_mipmap` | exit 0 |
| executor and evaluator tests | `cargo test --release --offline -p fx_exec --test t26_input_alias --test t35_g3r --test t17_arm_mut --test first_set_plan_probe` and `cargo test --release --offline -p fx_eval --test t28_lym3` | exit 0; tests that need stored data print `NOT RUN` (shown with `-- --nocapture`) |
| stage command line | `cargo test --release --offline -p neutral-handoff --test stage_cli layer_on_an_effect_stage_is_refused_with_exit_3_before_rendering` | exit 0 |
| device tests | `SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_neighbourhood --test device_warp --test device_ripmap --test device_footprint --test device_backend -- --test-threads=1` | exit 0 (needs OpenCL) |
| stage parity | `cargo test --release --offline -p neutral-handoff --test fx_stage_parity` | exit 0; with `-- --nocapture` it prints `NOT RUN: VEC_DIR is not set` |

Build, from the repository root:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
```

Expected: exit 0.

A test clip, the manifest entry and the parameters (ffmpeg on `PATH`; without it the clip is not written and the later renders have no input):

```sh
mkdir -p target/demo/distort-rgb
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/distort-rgb/in.raw
target/release/sapphire stage list --json | python -I -c "import json,sys; s=[x for x in json.load(sys.stdin)['stages'] if x['id']=='distort_rgb_autogen'][0]; print(s['name'], s['family'], s['kind'], len(s['params']), s['layers'])"
target/release/fx_run --describe-params Distort_RGB_Autogen
```

Expected: exit 0 for each; 98,304 bytes in `in.raw`; the line `Distort_RGB_Autogen S_DistortRGB temporal 10 []`; the parameters as JSON.

The worked render, with values that verified comparisons used:

```sh
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param amount=1.3125 --param amount_rel=1.1875,0.8125 --param warp_red=0.9375 --param warp_green=0.4375 --param warp_blue=0.1875 --param blur_lens=44 < target/demo/distort-rgb/in.raw > target/demo/distort-rgb/out.raw
```

Expected: exit 0; 98,304 bytes in `out.raw`.

Refusals, the last one on the special clip of `tools/fx_golden.py`:

```sh
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param cset_opacity=1 < target/demo/distort-rgb/in.raw > target/demo/distort-rgb/opacity1.raw
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --layer Lens=target/demo/distort-rgb/in.raw < target/demo/distort-rgb/in.raw > target/demo/distort-rgb/lens.raw
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; sys.stdout.buffer.write(fx_golden.synth_special(40, 12, 3, 20261106))" > target/demo/distort-rgb/special.raw
target/release/sapphire stage apply distort_rgb_autogen --width 40 --height 12 --depth 32 --fps 24 --frames 3 < target/demo/distort-rgb/special.raw > target/demo/distort-rgb/special_out.raw
```

Expected: exit 3 with `SOURCE-VARIANT-MISMATCH`; exit 3 with `LAYER-INPUT-UNSUPPORTED`; exit 0 writing the special clip; exit 1 with `error: stage_failed: frame 0: fx_run failed with Some(1): ERR kernel warp_rel failed: F2(Oob)` and an empty `special_out.raw`, the outcome the golden row `40x12s` records as `rc1`.

The golden rows of the stage and the full golden check:

```sh
grep '^distort_rgb_autogen' tools/fx_golden.tsv | cut -f1,2
python -I tools/fx_golden.py
```

Expected: five rows, `40x12s`, `50x7`, `50x7t`, `64x32` and `64x32t` (no variant row). The plain and threaded rows run every op of the plan but `rotate`, whose angle they leave at 0; no golden row reaches `rotate`, and the special row stops at `warp_rel`; then `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0.

Conformance of the stage:

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply distort_rgb_autogen"
```

Expected: exit 1 with `L1 stage matrix FAIL 14 passed, 2 failed`, the two `FAIL` lines of the 1x1 cases, and `L3 failure injection pass 33 passed` ([Conformance](../stages.md#conformance)).

The device path (an OpenCL platform such as PoCL; without one the first command ends with exit 3 and `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply distort_rgb_autogen --device gpu --width 64 --height 32 --depth 32 --fps 25/1 --param amount=1.3125 --param amount_rel=1.1875,0.8125 --param warp_red=0.9375 --param warp_green=0.4375 --param warp_blue=0.1875 --param blur_lens=44 < target/demo/distort-rgb/in.raw > target/demo/distort-rgb/dev.raw
cmp target/demo/distort-rgb/out.raw target/demo/distort-rgb/dev.raw
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

Expected: exit 0 for each. After the device line the report prints `device: * 4, element_of 3, grad 1, make_mipmap 3, monochrome 1, postdivide 1, v3 1, warp_rel 3; fallback: none` and the running counts of the next two frames; `cmp` prints nothing; the golden check prints `183 equal` (the special row ends with `rc1` on the device as on the CPU).

The crate and executor tests (cargo, offline):

```sh
cargo test --release --offline -p monochrome -p grad -p rotate2d -p ops_element -p ops_vec -p ops_arith -p make_mipmap
cargo test --release --offline -p fx_exec --test t26_input_alias --test t35_g3r --test t17_arm_mut --test first_set_plan_probe
cargo test --release --offline -p fx_eval --test t28_lym3
cargo test --release --offline -p neutral-handoff --test stage_cli layer_on_an_effect_stage_is_refused_with_exit_3_before_rendering
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_neighbourhood --test device_warp --test device_ripmap --test device_footprint --test device_backend -- --test-threads=1
cargo test --release --offline -p neutral-handoff --test fx_stage_parity
```

Expected: exit 0 for each. `grad`'s test `t2_against_emulated_original` compares with stored emulator runs and prints `T2 PENDING` without them. `t26_input_alias` checks the alias of an unbound input to the input its default names (the `Lens` input), `t35_g3r` the class-3 source route of `Distort_RGB_Autogen`, `t17_arm_mut` the arm mutants of the `grad` call frame, `first_set_plan_probe` that no `make_mipmap` node of these plans is read only linewise, and `t28_lym3` the input group of the layout rows. The `stage_cli` test refuses `Lens` and `To` layers on this stage with exit 3. `device_footprint` holds the self-checks and mutants of the footprint path; `device_backend` renders every registered program on the device and the CPU and compares the bytes. `make_mipmap`, `rotate2d`, `fx_eval` and `fx_exec` are not default members; the others are. Without an OpenCL platform the device tests print `NOT RUN` and pass; with `SAPPHIRE_REQUIRE_DEVICE=1` they fail instead. Cargo captures the output of a passing test, so the `NOT RUN` and `T2 PENDING` lines named here show only with `-- --nocapture` after the command; the exit codes are the same either way.

In CI (`.github/workflows/ci.yml`) the `tests on ...` job runs `cargo test --release` and `python -I tools/fx_golden.py` on three systems and the `fx_eval`, `fx_exec`, `make_mipmap` and `rotate2d` tests on Linux; the `device backend on PoCL` job runs the device tests above and the device golden check.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, and the recorded renders are not in it.

- `crates/neutral-handoff/tests/fx_stage_parity.rs` renders the 26 counted recorded renders that match the stage's host arrangement and compares them by full SHA-256; it needs `VEC_DIR/fx_parity/members.tsv` and an `fx_run`, and prints `NOT RUN` without them.
- `grad`'s `t2_against_emulated_original` reads stored emulator runs and prints `T2 PENDING` without them; tests of `fx_eval` and `fx_exec` that read `VEC_DIR/effects/` print `NOT RUN` ([Tests](../effects.md#tests)).
- The full report of the comparisons is kept outside the repository; its SHA-256 is in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, the frame sizes 64x32 and 50x7 of the stage row's measured placement, the parameter values listed in the `S_DistortRGB` section of [Driving the effect parameters](../parameters-guide.md), the source with three planes, the Opacity answer 2 and no mask connected. Other sizes and values run the same code and are INFERRED.
- Host buffer phase: the blur windows inside `grad` take their summation order from the phase of a host buffer above the thresholds of [Limits common to the effects](../effects.md#limits-common-to-the-effects): a blur window of 64 samples or more, which needs a frame 128 pixels or more wide or high. Every counted render reaches them below the thresholds or at a recorded phase (MEASURED); above them the native output equals the original's when the host's phase equals the one the native code takes (INFERRED). A 1920x1080 frame with a `blur_lens` that makes a window of 64 samples or more is above the threshold; the stage then runs phase 0, its placement column being empty, and equality with a host's output is INFERRED.
- NaN: two counted comparisons hold NaN outputs, compared as NaN values without their payload bits. A source with infinities and NaN, as in the special golden row, ends with `F2(Oob)` and exit 1 on the CPU and on the device; the original's behaviour on that input is not part of the verified comparisons.
- `ROUTE AVX2-ONLY`: the kernels reproduce the AVX2 and FMA code tier only ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- The `Lens` and `Mask` inputs are not offered; the lens image is always the source.
- Depths 8 and 16 convert through f32 and are not verified.
- Refusals and exit codes: the table under [Styles and stages](#styles-and-stages) and [Refusal labels](../cli.md#refusal-labels).

## Device path

| Op | Device kernel | `NAN_EXACT` |
|---|---|---|
| `monochrome` | its entry in `crates/effects/fx_exec/src/device_ops_pointwise.rs` | no |
| `grad` | `GRAD`, `crates/effects/fx_exec/src/device_ops_neighbourhood.rs`: the blur passes, then `grad_stencil_f32` | no |
| `rotate` | `rotate_run`, `crates/effects/fx_exec/src/device_ops_warp.rs` | no |
| `*` | `MUL`, `device_ops_pointwise.rs` | yes, `NAN_EXACT_COMPILED` |
| `element_of` | `ELEMENT_OF`, `device_ops_pointwise.rs` | yes, `NAN_EXACT_WRITTEN` |
| `make_mipmap` | `make_mipmap_run`, `crates/effects/fx_exec/src/device_ops_ripmap.rs` | yes, `NAN_EXACT_COMPILED` |
| `warp_rel` | `warp_rel_run`, `device_ops_warp.rs`, with the footprint prefill and warp of `crates/effects/fx_exec/src/device_ops_footprint.rs` | no |
| `v3` | `V3`, `device_ops_pointwise.rs` | yes, `NAN_EXACT_WRITTEN` |
| `postdivide` | `POSTDIVIDE`, `device_ops_pointwise.rs`, the copy-alpha arm included | yes, `NAN_EXACT_WRITTEN` |

Every op is on `ALLOWLIST` and none has a `CPU_ONLY` entry. A node falls back to the CPU for the reasons of [Rendering on a device](../cli.md#rendering-on-a-device), always with the CPU's bytes; a NaN output of `monochrome`, `grad`, `rotate` or `warp_rel` is rerun on the CPU. MEASURED on PoCL only: `fallback: none` on the plain and threaded golden rows and on the worked render; the special row ends with `rc1` on both paths. The `distort_rgb_autogen` row of [Speed](../guide.md#speed) holds the timing of one 1920x1080 frame: 4.80 / 4.45 s on the CPU at the default thread count under load, 1.7 to 2.0 s with `SAPPHIRE_THREADS=1`, and 1.02 / 1.00 s on PoCL; the CPU cell is a contention figure, and PoCL runs on the CPU, so no cell is GPU speed. Measured again on this build at a one-minute load of 14 to 15 on the same 12 core machine: 4.01 s at the default thread count and 2.02 to 2.08 s with `SAPPHIRE_THREADS=1`, two runs each, outputs equal (MEASURED); a run on an idle machine is not measured. The blur inside `grad` splits its work by plane and has one plane here, so the threads go to the row bands of `warp_rel` (INFERRED from `crates/effects/kernels/blur/src/lib.rs` and `crates/effects/fx_exec/src/par.rs`).

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (all groups, the `rc1` of the special row included), the crate tests above, `python -I tools/test_effect_rules.py`, the device tests with `SAPPHIRE_REQUIRE_DEVICE=1`, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes, or the exit status of the special row, changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` from the current build and is used only after a change has been judged against the original.
- A change to a CPU kernel of the plan removes its op from `ALLOWLIST` unless the device kernel changes in the same patch; the self-check, with its mutants for the footprint path, then compares the two bit for bit ([Device](../adding-an-effect.md#device)).
- Effect code calls no platform math library function; two-NaN picks go through `fx_common::nanop` and a fresh NaN is canonicalized ([Crate layout](../adding-an-effect.md#crate-layout)).
- A change to the stage row or the offered parameters updates `crates/neutral-handoff/data/fx_stages.tsv` and `fx_params.tsv`, the `S_DistortRGB` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), and then `python -I tools/gen_docs.py`.

## Open items

- A tap outside the ripmap ends the render with exit 1 (`F2(Oob)`) for the special golden row and for very large finite source values (MEASURED above; docs/effects.md line 314). The stage could refuse such a frame by name before rendering; what the original does with that input is not recorded in this repository.
- The Opacity answers 1 and 3 are refused with `SOURCE-VARIANT-MISMATCH`: no recorded render of the effect sets them, so the stage row has no host variant for them.
- Frames 128 pixels or more wide or high pass the host buffer phase thresholds of the gradient's blur (INFERRED equality at the native phase).
