# Internals of `stage apply warp_transform_autogen`

This page describes how this repository renders the stage `stage apply warp_transform_autogen`: its compiled plan, the code that runs each node, the stage row, the checks that run from the repository alone and the limits. It is the one stage of `S_WarpTransform`. The user reference is the `S_WarpTransform` section of [Effects](../effects.md), its section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The parts every effect shares are described once in [Effect internals](README.md). The ripmap sampler (`make_mipmap` and `warp`) is described in [`S_WarpPerspective` internals](warp-perspective.md); this page names only what differs.

## What it computes

The source image is moved, scaled, turned in its plane, turned about the vertical and the horizontal axis and slanted, then sampled again. One `tilt` node maps the default xy coordinates through the whole transform: `shift`, `rotate`, `scale` (each lane's magnitude held at a floor of the program before it divides `z_dist`, `eff_warp_transform`, `crates/effects/fx_eval/src/programs_native.rs` lines 4822 to 4826), `shear`, the two angles `swivel` and `tilt` with `perspective_amount` for the foreshortening, and `wrap` per axis. The source is built into a ripmap (`make_mipmap`) and sampled at the mapped coordinates (`warp`); `filter` chooses filtered sampling. When the larger magnitude of `swivel` and `tilt` exceeds a threshold of the program (MEASURED below: `tilt=10.01` or `swivel=10.01` adds `tilt_alpha`, while `tilt=10`, and `tilt=8` with `swivel=8`, do not; constructor line 4842), the sample is multiplied (`*`) by an alpha ramp clamped to 0 to 1 (`tilt_alpha`), computed from the coordinates, the two angles and `perspective_amount`. The last node, `postdivide`, divides the colour planes by alpha. These steps are INFERRED from the constructor; the nodes of each plan are MEASURED by the device report below. The formulas are in the crate headers: `crates/effects/kernels/tilt/src/lib.rs` lines 1 to 9 with `crates/effects/kernels/tilt/src/perspective.rs`, and `crates/effects/kernels/tilt_alpha/src/lib.rs` lines 1 to 14 (the ramp, element by element, in the scalar and the 8-wide form).

At the default parameters every argument of `tilt` is its identity, so `tilt` folds, and a `warp` of the raw xy coordinates with `wrap` 0 folds too: the plan holds only `make_mipmap` and `postdivide`, and the output is the input frame (MEASURED below on a clip with alpha 1). The fold rules are in `crates/effects/fx_eval/src/fold.rs` (`tilt`, `motion_blur`) and `crates/effects/fx_eval/src/compile.rs` (the identity `warp`).

## Styles and stages

| Style (program) | Stage | Kind | Layers | Spool | Host variants |
|---|---|---|---|---|---|
| `Warp_Transform_Autogen` | `stage apply warp_transform_autogen` | `temporal` | none | `false` | `cset_opacity` answers 1 and 3 |

The row is line 24 of `crates/neutral-handoff/data/fx_stages.tsv`; the eleven offered parameters are lines 318 to 328 of `crates/neutral-handoff/data/fx_params.tsv` (`scale`, `shift`, `z_dist`, `rotate`, `swivel`, `tilt`, `perspective_amount`, `shear`, `wrap` and `filter` through `--set`, `cset_opacity` through `--cset Opacity`). `fx_run --describe-params Warp_Transform_Autogen` lists 26 registered parameters; the others keep their registered defaults. The `host_variants` entries give the answers 1 and 3 the host options of their own recorded runs (for answer 1 the source plane count, reader head, destination planes, one context word and the class cells and record; for answer 3 the reader head), so all three answers render ([Host variants](../stages.md#effects-provider)).

An unset `shift` renders as `shift=0,0`, the layer centre: the two give the same bytes, and at the default parameters the output equals the input (MEASURED by the commands below; `crates/neutral-handoff/tests/stage_cli.rs` renders this default plan). The recorded host answered an unset Shift XY with its registered (50, 50) pixels; the two default comparisons of [Effects](../effects.md) bind that answer, which moves the image out of a 64x32 or 50x7 frame. The stage does not pass it (see [Open items](#open-items)).

Refused and failing inputs (MEASURED on this build by the commands below):

| Input | Result | Exit |
|---|---|---|
| `--param cset_opacity=4` | `bad value for` `cset_opacity`: outside 1 to 3 | 3 |
| a value outside the range `fx_run --describe-params` prints, for example `perspective_amount=8` | `bad value for` | 3 |
| `--width 1` | `needs a width of 2 or more` | 3 |
| `--layer ID=PATH` | `LAYER-INPUT-UNSUPPORTED` | 3 |
| a keyed value without `--fps` | `keyframes need --fps` | 2 |
| `tilt` or `swivel` such that the cosine of the combined angle over `perspective_amount` is near zero, for example `tilt=90` | `stage_failed`: `tilt_alpha` refuses its host branch (`HOST-BRANCH-REFUSED`), which is not rebuilt | 1 |

The host branch: `tilt_alpha` computes the combined angle of `swivel` and `tilt` and the cosine of that angle divided by `perspective_amount`; when that quotient lies strictly inside the window `K_LO` to `K_HI` around zero (`crates/effects/kernels/tilt_alpha/src/lib.rs` lines 6 and 26 to 28) the original hands the rows to a region-of-interest helper and a host dispatch call instead of its element loops, and the port does not include that route (`Route::HostBranch`, line 56). The executor meets it during the run (`tilt_alpha_k`, `crates/effects/fx_exec/src/kernels2.rs` line 1248) and returns an unsupported-class error; `fx_run` exits 1 for a failed execution ([Exit codes](../cli.md#exit-codes)), which the stage reports as `stage_failed`. No check before the first frame computes the quotient, so the values are not refused with exit 3 (INFERRED from the code).

## Code map

| Part | Path | Role |
|---|---|---|
| Style constructors | `crates/effects/fx_eval/src/programs_native.rs` (`eff_warp_transform`, `eff_warp_transform_autogen`, `layout_warp_transform_autogen`) | the program graph, its wrapper and its layout row |
| Recorded host facts | `crates/effects/rtdata/src/styles/warp_transform.rs` | registered parameters, popup items and host indices, the Shift XY row among them |
| Shared class rows | `crates/effects/rtdata/src/classes.rs` (`CLASS_FLAGS`, `IMG_ONLY_ARGS`, `INT_TYPED_ARGS`, `COMBOS`, `COMBOS_EXEC`) | class flags, typed arguments and combination tables of `tilt`, `tilt_alpha`, `warp`, `make_mipmap`, `postdivide` and `*` |
| Folds | `crates/effects/fx_eval/src/fold.rs`, `crates/effects/fx_eval/src/compile.rs` | the identity `tilt`, `motion_blur` (no shutter) and the identity `warp`; the safe scale folds lane by lane |
| `tilt` | `crates/effects/kernels/tilt` (`lib.rs`: prologue, flags and the loops without the two angles; `perspective.rs`: the loops with them) | the mapped coordinate planes |
| `tilt_alpha` | `crates/effects/kernels/tilt_alpha` | the alpha ramp, with a scalar form and an 8-wide form per element |
| sine and cosine | `crates/effects/kernels/libm_sincos` | the sincosf routine `tilt` and `tilt_alpha` call |
| `make_mipmap`, `warp` | `crates/effects/kernels/make_mipmap`, `crates/effects/kernels/warp_render`, `crates/effects/kernels/warp_driver`, `crates/effects/kernels/warp_sampler`, `crates/effects/kernels/warp_log_gradient` | the ripmap sampler ([`S_WarpPerspective` internals](warp-perspective.md)) |
| `*`, `postdivide` | `crates/effects/kernels/ops_arith`, `crates/effects/fx_exec/src/postdiv.rs` | product with the ramp; colour divided by alpha |
| Executor entries | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`: `*`), `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`: `tilt`, `tilt_alpha`, `warp`, `make_mipmap`, `postdivide`; `tilt_k`, `tilt_alpha_k`, `warp_k`, `rows_of_p`) | operand checks and refusals by name, row chunking with the buffer phase |
| Device entries | `crates/effects/fx_exec/src/device_ops_tilt.rs` (`TILT`), `crates/effects/fx_exec/src/device_ops_tilt_alpha.rs` (`TILT_ALPHA`), `crates/effects/fx_exec/src/device_ops_warp.rs`, `crates/effects/fx_exec/src/device_ops_ripmap.rs`, `crates/effects/fx_exec/src/device_ops_pointwise.rs`, tables in `crates/effects/fx_exec/src/device_ops.rs` | accept tests, host prologues, launches and self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_tilt.rs` (`tilt_f32`), `crates/effects/fx_device/src/kernels_tilt_alpha.rs` (`tilt_alpha_f32`), plus the warp, ripmap and pointwise sources | registered in `SOURCES` and `PROGRAM_PARTS` of `crates/effects/fx_device/src/kernels.rs` |
| Stage row and parameters | `crates/neutral-handoff/data/fx_stages.tsv` line 24, `crates/neutral-handoff/data/fx_params.tsv` lines 318 to 328 | host arrangement, placement, host variants and offered parameters |
| Stage code | `crates/neutral-handoff/src/fxstage.rs` | per-frame runner call, writer phase lists, host variants |
| Golden rows | `tools/fx_golden.tsv` lines 176 to 184; the variants in `VARIANTS` of `tools/fx_golden.py` | nine rows: `40x12s`, `50x7`, `50x7t`, `64x32`, `64x32t`, and `+tl` (`rotate` and `shear` set) and `+ta` (`tilt` and `swivel` set) at both plain sizes |
| Computed tables | `crates/effects/kernels/libm_sincos` | the sincosf table, computed at first use; no table of its own |

Tests that exercise it:

| Test | What it checks |
|---|---|
| `crates/effects/kernels/tilt/tests/t1.rs` | identity, shift, scale, shear, clamp, rotation, NaN and the perspective loops of `tilt` |
| `crates/effects/kernels/tilt_alpha/tests/t1.rs` | the ramp, its sections, the flag and host routes, NaN |
| `crates/effects/fx_exec/tests/tilt_alpha_wired.rs` | `tilt_alpha` through the executor equals the crate; missing host fields, the host branch and a wrong operand shape are refused |
| `crates/effects/fx_exec/tests/t_wt_fold_collapse.rs` | the negative `scale` fold and the collapsed warp at the Opacity answers 1 and 3 |
| `crates/neutral-handoff/tests/stage_cli.rs` (`a_plan_with_no_default_xy_writer_is_staged_without_a_phase_list_and_renders`) | the default plan has no default-xy writer and renders without a phase list |
| `crates/effects/fx_exec/tests/device_tilt.rs`, `device_tilt_alpha.rs`, `device_warp.rs`, `device_ripmap.rs`, `device_pointwise.rs` | device kernels against the crates, stubs and single-edit mutants (`NOT RUN` without an OpenCL device) |

## Render flow

1. `sapphire stage apply warp_transform_autogen` reads the row: `fixed` options (`blur_mask`, `blur_mocha_mask` and `shift_mocha_mask` at 0 as host values, `invert_mask` and `mask_use` at 0), the `host` options (the options of [`S_WarpPerspective` internals](warp-perspective.md), `--palpha-cells` read only where `tilt_alpha` runs) and the `placement`, the same at 64x32 and 50x7: `--xy-base-list 1:0;2:16,0`, one phase list per default-xy writer count. A plan with one writer takes phase 0, a plan with `tilt_alpha` runs two writers and takes 16 and 0, and a plan with no writer (the default) takes no list ([Writer phase lists](../stages.md#effects-provider)). Another size takes the 64x32 entry (INFERRED).
2. Per frame the stage selects the host options by the frame's `cset_opacity` value (the `host` column for 2 or unset, the entry for 1 or 3), checks the offered values, puts `--set` host slider values on the 1/65536 grid with a `note:` line, and calls the runner in memory with the frame's 'ae32' words, size, geometry and frame index (as for every stage, [Effects provider](../stages.md#effects-provider)). The program reads no host time (INFERRED from the constructor).
3. `fx_exec::fxrun` compiles the plan: constant nodes fold, `motion_blur` folds because the host passes no shutter, `tilt` folds at its identity arguments, a negative `scale` lane folds the safe scale lane by lane, and the alpha factor folds to 1 below the angle threshold.
4. The nodes run in plan order: `tilt` (one call over the frame), `make_mipmap`, `warp` (one driver call per row, row bands across `SAPPHIRE_THREADS` workers from 65,536 samples, `SAPPHIRE_PAR_MIN` moves the threshold), `tilt_alpha` (one call per row, row phase from `rows_of_p` with base 0), `*`, `postdivide`. `tilt` and `tilt_alpha` take their angles as constants of the run; a non-constant operand is refused by name.
5. With `--device gpu|gpu=N|auto` or `SAPPHIRE_DEVICE`, `tilt` and `tilt_alpha` run their prologues on the host through the crates themselves and pass the derived constants as words to `tilt_f32` and `tilt_alpha_f32`; other nodes as in [`S_WarpPerspective` internals](warp-perspective.md).
6. The output planes are relaid to frames/2 planes G, B, R, A and written.

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means). For `S_WarpTransform` `docs/data/effects_status.tsv` holds 41 verified comparisons (23 at 64x32 and 18 at 50x7), 78 recorded renders and 78 re-rendered units, all byte identical. Two comparisons, the default parameters at each size, bind the recorded host's Shift XY answer and give an empty output; they verify only that path and have no recorded render. The other 39 have content, and their 78 counted recorded renders are byte identical through `fx_run` and through the stage. 61 of the 78 match the stage's host arrangement; the others equal `fx_run` at the stage's buffer phases ([Effects and stages](../../README.md#effects-and-stages)). A negative `scale` is verified at both sizes; `perspective_amount` enters `tilt` clamped to its registered range (through `fx_run` an infinite value enters as the upper end, negative infinity and 0 as the lower end, MEASURED). The `tilt` kernel's mutation test kills 150 of 150 non-equivalent mutants on route, with 7 equivalent on route out of 168 raw ([Effects](../effects.md), the `S_WarpTransform` section). No comparison of this effect differs from the original. No repository test holds the 61 stage renders: [Effects provider](../stages.md#effects-provider) (docs/stages.md line 192) says each was rendered through `sapphire stage apply` and compared with the original's output; a recheck needs the recorded renders and their host options, which are outside the repository (NEEDS-OUTSIDE). The other 17 differ from the stage's arrangement in their recorded buffer phases, and equal `fx_run` at the stage's phases (docs/effects.md line 894). No recorded render of this effect is a control: its 78 recorded renders are the 78 counted ones.

## Checks that run from this repository

Run from the repository root after the README "Build" step and after the input clip of [`S_WarpPerspective` internals](warp-perspective.md) (`target/demo/warp_in.raw`, made with ffmpeg).

| Check | Command | Expected result and exit |
|---|---|---|
| Stage listed | `sapphire stage list --json` filtered | `warp_transform_autogen temporal 11 0`, exit 0 |
| Parameters | `fx_run --describe-params Warp_Transform_Autogen` | `26 parameters`, exit 0 |
| Worked render | `sapphire stage apply warp_transform_autogen ...` | exit 0, two `note:` lines |
| Unset `shift` | two renders and two `cmp` | `cmp` silent, exit 0 each |
| Plan nodes on a device | worked, default and `tilt`/`swivel` renders with `SAPPHIRE_DEVICE_REPORT=1 --device gpu` | the three `device:` lines below, exit 0 (needs OpenCL) |
| Opacity answers | `cset_opacity=1` and `3` | exit 0 each |
| Golden rows | `python -I tools/fx_golden.py` and the `grep` | `fx_golden: 183 cases, 183 equal, 0 differ`, nine rows, exit 0 |
| Golden rows on a device | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | the same line, exit 0 (needs OpenCL) |
| Conformance | `python -I tools/conformance.py --stage ...` | two `FAIL` lines for the 1x1 cases, L3 `33 passed`, exit 1 |
| Refusals | the commands below | exits 3, 3, 3 and 1 |
| Crate and executor tests | `cargo test` lines below | exit 0 |
| Device tests | `cargo test ... -- --test-threads=1` | exit 0 (needs OpenCL) |

The stage, its parameters and the worked render (the command of [Effects](../effects.md), values of a verified comparison):

```sh
target/release/sapphire stage list --json | python -I -c "import json,sys; m=json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), len(s['layers'])) for s in m['stages'] if s['family'] == 'S_WarpTransform']"
target/release/fx_run --describe-params Warp_Transform_Autogen | python -I -c "import json,sys; d=json.load(sys.stdin); print(len(d['Warp_Transform_Autogen']), 'parameters')"
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0.0,0.0 --param rotate=-22.0 --param shear=-0.149993896484375,0.0 < target/demo/warp_in.raw > target/demo/wt_out.raw
```

Expected: `warp_transform_autogen temporal 11 0`, `26 parameters`, then a render with two standard error lines, `` note: `rotate` -22.0 is put on the 16.16 host grid as -22 `` and `` note: `shear` -0.149993896484375,0.0 is put on the 16.16 host grid as -0.149993896484375,0 ``; exit 0 each.

The unset `shift` against the layer centre, and the default plan against the input:

```sh
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --in target/demo/warp_in.raw --out target/demo/wt_unset.raw
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0,0 --in target/demo/warp_in.raw --out target/demo/wt_centre.raw
cmp target/demo/wt_unset.raw target/demo/wt_centre.raw
cmp target/demo/wt_unset.raw target/demo/warp_in.raw
```

Expected: exit 0 each and no output from either `cmp`. The input clip has alpha 1, so `postdivide` leaves it unchanged.

The nodes of three plans, counted by the device report (an OpenCL platform such as PoCL; without one each command ends with `DEVICE-REQUIRED-UNAVAILABLE`, exit 3):

```sh
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0.0,0.0 --param rotate=-22.0 --param shear=-0.149993896484375,0.0 --device gpu --in target/demo/warp_in.raw --out target/demo/wt_dev.raw
cmp target/demo/wt_out.raw target/demo/wt_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --device gpu --in target/demo/warp_in.raw --out target/demo/wt_default.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0.0,0.0 --param tilt=-14.0 --param swivel=12.0 --device gpu --in target/demo/warp_in.raw --out target/demo/wt_ta.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0,0 --param tilt=10 --device gpu --in target/demo/warp_in.raw --out target/demo/wt_t10.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0,0 --param tilt=10.01 --device gpu --in target/demo/warp_in.raw --out target/demo/wt_t1001.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param shift=0,0 --param tilt=8 --param swivel=8 --device gpu --in target/demo/warp_in.raw --out target/demo/wt_t8s8.raw
```

Expected: exit 0 each; `cmp` prints nothing; the last report lines are `device: make_mipmap 3, postdivide 3, tilt 3, warp 3; fallback: none` (worked command), `device: make_mipmap 3, postdivide 3; fallback: none` (default) and `device: * 3, make_mipmap 3, postdivide 3, tilt 3, tilt_alpha 3, warp 3; fallback: none` (`tilt` and `swivel` set, the values of the `+ta` golden row); then `device: make_mipmap 3, postdivide 3, tilt 3, warp 3; fallback: none` for `tilt=10`, the `+ta` line for `tilt=10.01`, and the `tilt=10` line for `tilt=8` with `swivel=8`.

The opacity answers that take host variants:

```sh
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=1 --param shift=0,0 --param rotate=-22.0 --in target/demo/warp_in.raw --out target/demo/wt_o1.raw
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=3 --param shift=0,0 --param rotate=-22.0 --in target/demo/warp_in.raw --out target/demo/wt_o3.raw
```

Expected: exit 0 each, one `note:` line each.

The golden rows, on the CPU and on the device, and the rows of this stage:

```sh
python -I tools/fx_golden.py
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
grep -P '^warp_transform_autogen\t' tools/fx_golden.tsv | cut -f1,2
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0 for each golden run (the second needs OpenCL); the `grep` prints nine rows: `40x12s`, `50x7`, `50x7+ta`, `50x7+tl`, `50x7t`, `64x32`, `64x32+ta`, `64x32+tl` and `64x32t`. The `+tl` row renders the worked values (`rotate` and `shear`), which reach `tilt`; the `+ta` row sets `tilt` and `swivel`, which reach `tilt_alpha`. Golden reach (nodes MEASURED by the reports above): the plain, threaded and special rows reach only `make_mipmap` and `postdivide`; `+tl` adds `tilt` and `warp`, `+ta` also `tilt_alpha` and `*`. No golden row renders the Opacity answers 1 and 3 of the host variants, a negative `scale`, a `wrap` lane set or `filter` off; the opacity commands above render the answers 1 and 3.

Conformance (L1 and L3):

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply warp_transform_autogen"
```

Expected: exit 1. L1 reports `14 passed, 2 failed`: the two `FAIL` lines are the 1x1 cases, each `exit 3` with `needs a width of 2 or more`, the documented size refusals ([Conformance](../stages.md#conformance)); L3 reports `33 passed`.

Refusals and the failing host branch:

```sh
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=4 --in target/demo/warp_in.raw --out target/demo/wt_r.raw
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param perspective_amount=8 --in target/demo/warp_in.raw --out target/demo/wt_r.raw
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --layer To=target/demo/warp_in.raw --in target/demo/warp_in.raw --out target/demo/wt_r.raw
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param shift=0,0 --param tilt=90 --in target/demo/warp_in.raw --out target/demo/wt_r.raw
```

Expected exits: 3 (`bad value for` `cset_opacity`), 3 (`bad value for` `perspective_amount`), 3 (`LAYER-INPUT-UNSUPPORTED`) and 1 (`error: stage_failed: frame 0: fx_run failed`, with `HOST-BRANCH-REFUSED` in the message).

Crate, executor and stage tests:

```sh
cargo test --release --offline -p tilt -p tilt_alpha -p libm_sincos -p warp_driver -p warp_sampler -p warp_log_gradient -p ops_arith
cargo test --release --offline -p warp_render -p make_mipmap
cargo test --release --offline -p fx_exec --test tilt_alpha_wired --test t_wt_fold_collapse
cargo test --release --offline -p neutral-handoff --test stage_cli a_plan_with_no_default_xy_writer_is_staged_without_a_phase_list_and_renders
```

Expected: exit 0 each; the two `warp_render` tests print `NOT RUN` without `VEC_DIR`. The tests of `tilt`, `tilt_alpha`, `tilt_alpha_wired` and `t_wt_fold_collapse` need no stored data.

Device tests (OpenCL; one thread, as CI runs them):

```sh
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_tilt --test device_tilt_alpha --test device_warp --test device_ripmap --test device_pointwise -- --test-threads=1
```

Expected: exit 0. Without an OpenCL platform each test prints `NOT RUN` and passes when `SAPPHIRE_REQUIRE_DEVICE` is unset, and fails when it is `1`.

CI (`.github/workflows/ci.yml`) runs `cargo test --release` (default members, `tilt` and `tilt_alpha` among them), `python -I tools/fx_golden.py` on Linux, macOS and Windows, the Linux step for `fx_exec`, `make_mipmap` and `warp_render`, and the PoCL job with `device_tilt`, `device_tilt_alpha`, `device_warp`, `device_ripmap` and `device_pointwise` and the device golden run.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders and the stored test data under `VEC_DIR`. From the repository alone the golden rows check only that this build renders as the build that wrote `tools/fx_golden.tsv`. The tests of this page that print `NOT RUN` without stored data are the two `warp_render` tests; the counted recorded renders through the stage are not part of `crates/neutral-handoff/tests/fx_stage_parity.rs` ([Effects provider](../stages.md#effects-provider)). The full report is identified by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified sizes: 64x32 and 50x7, depth 32, constant parameters; other sizes and values run the same code and are INFERRED ([Effects](../effects.md), the `S_WarpTransform` section).
- Host buffer phase: the default xy writer on rows of 29 pixels or more and `tilt_alpha` on lines of 1,200 samples or more take their order or vector form from the buffer phase ([Limits common to the effects](../effects.md#limits-common-to-the-effects)). Below 1,200 samples a `tilt_alpha` row takes the vector form from its first element, so the phase does not matter at the verified sizes; above it the row phase is base 0 plus the row offset modulo 32 bytes, since the stage row passes no `--palpha-base` (`tilt_alpha_k` and `rows_of_p`, `crates/effects/fx_exec/src/kernels2.rs` lines 1248 and 1188, INFERRED from the code). At 1920 pixels a row is 7,680 bytes, a multiple of 32, so every row takes phase 0: an HD render of a plan with `tilt_alpha` rests on phase 0 and on the writer list's phases (INFERRED), while the default plan has neither and does not depend on the phase. What phase a host's allocator gives is not recorded (NEEDS-OUTSIDE).
- The host's motion blur is off: `motion_blur` folds to its argument whatever the host (`crates/effects/fx_eval/src/fold.rs` line 316), because this build models a host with no shutter, and neither the stage nor `fx_run` takes a shutter; a host with motion blur on is not modelled, and its output is NEEDS-OUTSIDE (INFERRED: this build would render it without blur, not refuse it). The mask, matte and crop inputs are not connected and mocha is off; their effect is INFERRED.
- NaN rules: `tilt` and `tilt_alpha` write every two-NaN pick through `fx_common::nanop` in source operand order and a fresh NaN as 0xffc00000. The tilt and the alpha tilt run no approximate reciprocal instruction, so the bound is 0 ulp ([Effects](../effects.md)).
- `ROUTE AVX2-ONLY`: the AVX2 and FMA bodies are ported; the SSE twin is not. The flag route of `tilt` and `tilt_alpha` (host offload) is refused by name, and the host branch of `tilt_alpha` ends the run with exit 1 (above).
- Not verified: a real host, the unset-point answer of a host that centres it, depths 8 and 16.

## Device path

| Operation | Device kernel | `NAN_EXACT` | Runs on the CPU when |
|---|---|---|---|
| `tilt` | `tilt_f32` | yes (`NAN_EXACT_WRITTEN`) | a non-constant parameter operand (the CPU kernel then refuses it by name); a NaN output only on a device whose NaN self-check reports other payloads |
| `tilt_alpha` | `tilt_alpha_f32` | yes (`NAN_EXACT_WRITTEN`) | the host branch or the flag route (the CPU kernel then gives the named error); a NaN output only on a device whose NaN self-check reports other payloads |
| `make_mipmap`, `warp`, `*`, `postdivide` | as in [`S_WarpPerspective` internals](warp-perspective.md) | `make_mipmap`, `*`, `postdivide` yes; `warp` no | as there |

No operation of these plans has a `CPU_ONLY` entry. MEASURED on PoCL only: `fallback: none` on every golden row of this stage, the special row and the `+tl` and `+ta` rows included, and on all 78 counted recorded renders through `fx_run` (314 device nodes, the same at threads 1, 3 and 12, [Effects](../effects.md)); on input with NaN `tilt` and `tilt_alpha` keep their NaN outputs on the device and `warp` reruns its NaN outputs on the CPU. [Speed](../guide.md#speed) gives one 1920x1080 frame of `warp_transform_autogen` 0.91 / 0.88 s on the CPU and 1.27 / 1.11 s on PoCL at the default parameters, whose plan holds only `make_mipmap` and `postdivide`; PoCL shows byte identity, not GPU speed.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` with its `+tl` and `+ta` rows (on the CPU and with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1`), the tests above, `device_tilt` and `device_tilt_alpha` with their mutants, `python -I tools/test_effect_rules.py`, the NaN tests on x86 and arm64 as CI runs them, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- The default plan reaches neither `tilt` nor `tilt_alpha`; only the variant rows and the special row exercise the transform on the golden table. A change to the fold rules of `tilt` or `warp` changes the default rows.
- A golden difference means the output bytes changed; the table holds this repository's own renders, so agreement with the original needs the comparison harness. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` and is used only after such a comparison.
- The kernels keep the original's operation order, `mul_add` exactly where it fuses and `fx_common::nanop` at every two-NaN site; sine and cosine come from `libm_sincos`, never a platform math function ([Crate layout](../adding-an-effect.md#crate-layout)).
- A CPU change to `tilt` or `tilt_alpha` removes it from `ALLOWLIST` unless `tilt_f32` or `tilt_alpha_f32` changes in the same patch, and the device prologue in `device_ops_tilt.rs` or `device_ops_tilt_alpha.rs` must compute the same constants ([Device](../adding-an-effect.md#device)).
- Update the `S_WarpTransform` section of [Effects](../effects.md), [Driving the effect parameters](../parameters-guide.md), the stage row and this page together, then run `python -I tools/gen_docs.py`.

## Open items

- The host branch of `tilt_alpha` is not rebuilt: a `tilt` or `swivel` that brings the cosine term near zero ends the stage with exit 1 (`stage_failed`), not with a refusal of exit 3 (`crates/effects/fx_exec/src/kernels2.rs`, `tilt_alpha_k`).
- The flag routes of `tilt` and `tilt_alpha` and the SSE tier are not ported (`crates/effects/kernels/tilt/src/lib.rs`, `crates/effects/kernels/tilt_alpha/src/lib.rs`).
- `warp` keeps the CPU rerun of NaN outputs on a device ([Rendering on a device](../guide.md#rendering-on-a-device)).
- 61 of the 78 counted recorded renders use the stage's host arrangement; the other 17 are verified through `fx_run` at the stage's buffer phases ([Effects and stages](../../README.md#effects-and-stages)).
