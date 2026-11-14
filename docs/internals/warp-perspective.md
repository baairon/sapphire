# Internals of `stage apply warp_perspective_autogen`

This page describes how this repository renders the stage `stage apply warp_perspective_autogen`: its compiled plan, the code that runs each node, the stage row, the checks that run from the repository alone and the limits. It is the one stage of `S_WarpPerspective`. The user reference is the `S_WarpPerspective` section of [Effects](../effects.md), its section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The parts every effect shares (program constructors, recorded host facts, executor, device path, tools) are described once in [Effect internals](README.md). The ripmap sampler described here (`make_mipmap` and `warp`) is the same code that the plans of `S_WarpTransform` run ([`S_WarpTransform` internals](warp-transform.md)).

## What it computes

The source image is laid on a plane seen in perspective and sampled again. The default xy coordinates of the frame are turned by the `roll` angle, projected through a plane at the distance `z_dist`, tilted by `latitude` and seen through a lens of width `tele_lens_width` (`perspective`), then turned by the `swing` angle (`rotate`). The source is built into a ripmap (`make_mipmap`) and sampled at the projected coordinates (`warp`), with `wrap` choosing per axis what fills the area outside the image and `filter` choosing filtered sampling. The sampled image is multiplied (`*`) by a horizon ramp (`perspective_alpha`) that clears the area above the horizon; with `wrap_above_horizon` set, or with a `latitude` whose magnitude is below a threshold of the program, the ramp folds to 1 and the area is filled from the image (MEASURED below: the ramp node is present at `latitude=10` and absent at `latitude=9.99`; the comparison is in `eff_warp_perspective`, `crates/effects/fx_eval/src/programs_native.rs` line 1534). The last node, `postdivide`, divides the colour planes by alpha before the output words are written. These steps are INFERRED from the constructor; the nodes of each plan are MEASURED by the device report in [Checks that run from this repository](#checks-that-run-from-this-repository). The formula of each kernel is in its crate header: `crates/effects/kernels/perspective/src/lib.rs` lines 4 to 11 (the two output planes and the reciprocal of the divisor), `crates/effects/kernels/perspective_alpha/src/lib.rs` lines 3 to 14 (the ramp and its clamp), `crates/effects/kernels/rotate2d/src/lib.rs` lines 4 to 9 (the rotation about a centre) and `crates/effects/kernels/make_mipmap/src/lib.rs` lines 3 to 9 (the ripmap layout).

## Styles and stages

| Style (program) | Stage | Kind | Layers | Spool | Host variants |
|---|---|---|---|---|---|
| `Warp_Perspective_Autogen` | `stage apply warp_perspective_autogen` | `temporal` | none | `false` | none (`-`) |

The row is line 14 of `crates/neutral-handoff/data/fx_stages.tsv`; the nine offered parameters are lines 133 to 141 of `crates/neutral-handoff/data/fx_params.tsv` (`filter`, `z_dist`, `tele_lens_width`, `swing`, `roll`, `latitude`, `wrap_above_horizon` and `wrap` through `--set`, `cset_opacity` through `--cset Opacity`). The style registers more parameters than the stage offers (`fx_run --describe-params Warp_Perspective_Autogen` lists 24); a parameter that is not offered keeps its registered default, the program's own `shift` among them.

Refused answers and values (MEASURED on this build by the commands below):

| Input | Result | Exit |
|---|---|---|
| `--param cset_opacity=1` or `3` | `SOURCE-VARIANT-MISMATCH`: the stage row has no host variant for these answers | 3 |
| `--param cset_opacity=4` | `bad value for` `cset_opacity`: outside 1 to 3 | 3 |
| a value outside the range `fx_run --describe-params` prints, for example `latitude=90` | `bad value for` | 3 |
| `--width 1` | `needs a width of 2 or more` (the host grid needs two columns) | 3 |
| `--layer ID=PATH` | `LAYER-INPUT-UNSUPPORTED` | 3 |
| a keyed value without `--fps` | `keyframes need --fps` | 2 |
| `SAPPHIRE_REQUIRE_DEVICE=1` with `--device gpu` and no usable OpenCL device | `DEVICE-REQUIRED-UNAVAILABLE` | 3 |

A `wrap` lane outside 0 to 2 is not refused: `wrap=3,0` renders the bytes of `wrap=2,0` and `wrap=-1,0` those of `wrap=0,0` (MEASURED). The compile clamps each given lane into the parameter's range before folding: `crates/effects/fx_eval/src/compile.rs` lines 795 to 806 apply the range clamp of `crates/effects/fx_eval/src/ranges.rs` line 155, or, for a popup with no range row, the popup rule 0 to the item count minus 1 (INFERRED from the code). The code comment calls this the original's own range pass, so the clamp is the intended behaviour and a refusal by the stage would be a new choice. Every label is described in [Refusal labels](../cli.md#refusal-labels).

## Code map

| Part | Path | Role |
|---|---|---|
| Style constructors | `crates/effects/fx_eval/src/programs_native.rs` (`eff_warp_perspective`, `eff_warp_perspective_autogen`, `layout_warp_perspective_autogen`) | the program graph of the style, its wrapper and its layout row, as Rust constructor code |
| Recorded host facts | `crates/effects/rtdata/src/styles/warp_perspective.rs` | the registered parameters, popup items and host indices of the style |
| Shared class rows | `crates/effects/rtdata/src/classes.rs` (`CLASS_FLAGS`, `IMG_ONLY_ARGS`, `INT_TYPED_ARGS`, `COMBOS`, `COMBOS_EXEC`) | class flags, typed arguments and combination tables of `perspective`, `perspective_alpha`, `rotate`, `warp`, `make_mipmap`, `postdivide` and `*` |
| Host angle rule | `crates/effects/fx_eval/src/units.rs` (`angle_host_lane`) | a host value of `swing` or `roll` enters the program negated |
| Input index rule | `crates/effects/fx_eval/src/ranges.rs` (`PARAM_INPUT_INDEX_BY_RECORD`) | the mask parameters of `Warp_Perspective_Autogen` name input 1 |
| `perspective` | `crates/effects/kernels/perspective` | the two output planes of the projected coordinates; `run_with_rcp` runs the body with an injectable reciprocal estimate |
| `perspective_alpha` | `crates/effects/kernels/perspective_alpha` | the horizon ramp plane |
| `rotate` | `crates/effects/kernels/rotate2d` | turns a two-lane vector about a centre (present when `swing` or `roll` is not zero) |
| sine and cosine | `crates/effects/kernels/libm_sincos` | the sincosf routine the three kernels above call |
| `make_mipmap` | `crates/effects/kernels/make_mipmap` (`mm2.rs`) | the ripmap: a grid of planes halved along x and along y, with pad rows and columns |
| `warp` | `crates/effects/kernels/warp_render` (`slot53_warp`), `crates/effects/kernels/warp_driver`, `crates/effects/kernels/warp_sampler` | the per-row driver and the lattice sampler over the ripmap planes |
| `warp` footprint | `crates/effects/kernels/warp_log_gradient` | with `filter` set and plane coordinates, the log2 gradient maps the driver's footprint branch reads |
| `perspective` and `perspective_alpha` glue | `crates/effects/kernels/warp_render` (`slot52_perspective`, `slot52_perspective_alpha`) | the dispatcher over each kernel body |
| `*` | `crates/effects/kernels/ops_arith` | the product of the sampled image and the ramp |
| `postdivide` | `crates/effects/fx_exec/src/postdiv.rs` | divides the colour planes by alpha |
| Executor entries | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`: `*`), `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`: `make_mipmap`, `rotate`, `perspective`, `perspective_alpha`, `warp`, `postdivide`; `perspective_k`, `perspective_alpha_k`, `rotate_k`, `warp_k`, `make_mipmap_k`, `postdivide_k`, `rows_of_p`) | operand checks, row chunking with the buffer phase, the call into each crate |
| Default xy writer | `crates/effects/fx_exec/src/kernels2.rs` (`default_xy`), `crates/effects/fx_exec/src/types.rs` | the coordinate plane the plan reads as `xy`, in the head form its placement in the plan gives |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS`, `ALLOWLIST`, `NAN_EXACT`, `CPU_ONLY`), `crates/effects/fx_exec/src/device_ops_warp.rs`, `crates/effects/fx_exec/src/device_ops_ripmap.rs`, `crates/effects/fx_exec/src/device_ops_footprint.rs`, `crates/effects/fx_exec/src/device_ops_pointwise.rs` | accept tests, launches and self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_warp.rs` (`warp_warp_f32`, `warp_rotate_f32`, `warp_persp_f32`, `warp_persp_alpha_f32`), `crates/effects/fx_device/src/kernels_ripmap.rs`, `crates/effects/fx_device/src/kernels_footprint.rs`, `crates/effects/fx_device/src/kernels_pointwise.rs` | the device kernels, registered in `SOURCES` and `PROGRAM_PARTS` of `crates/effects/fx_device/src/kernels.rs` |
| Stage row and parameters | `crates/neutral-handoff/data/fx_stages.tsv` line 14, `crates/neutral-handoff/data/fx_params.tsv` lines 133 to 141 | host arrangement, placement and offered parameters |
| Stage code | `crates/neutral-handoff/src/fxstage.rs` | the per-frame runner call, placement and value handling |
| Golden rows | `tools/fx_golden.tsv` lines 171 to 175 | five rows: `40x12s`, `50x7`, `50x7t`, `64x32`, `64x32t` |
| Computed tables | `crates/effects/kernels/libm_sincos` | the sincosf table, computed at first use ([No external data](../effects.md#no-external-data)); no table of its own |

Tests that exercise it:

| Test | What it checks |
|---|---|
| `crates/effects/kernels/perspective/tests/nan_x86.rs`, `crates/effects/kernels/perspective_alpha/tests/nan_x86.rs` | fresh NaN words are the x86 words on every processor |
| `crates/effects/kernels/make_mipmap/src/lib.rs` (unit tests) | ripmap levels, pads, formats, mutants |
| `crates/effects/kernels/warp_render/tests/warp53_entry.rs`, `crates/effects/kernels/warp_render/tests/alpha_entry_planes.rs` | the `warp` and `perspective_alpha` glue against stored emulator runs (`NOT RUN` without `VEC_DIR`) |
| `crates/effects/fx_eval/tests/t_wsr1.rs` | the negated host lane of `swing` and `roll` |
| `crates/effects/fx_eval/tests/t_int28_pi.rs`, `crates/effects/fx_eval/tests/t_input_index_order.rs` | the input index of the mask rows (`t_input_index_order` prints `NOT RUN` without `VEC_DIR`) |
| `crates/effects/fx_exec/tests/t_xyh1_head_form.rs` | the head form of the default xy writer, including this program with `swing` and `roll` set |
| `crates/neutral-handoff/tests/fx_run_state.rs` | `warp_perspective_autogen` gives the same bytes in one process after other stages as in a fresh process |
| `crates/neutral-handoff/tests/fx_stage_parity.rs` | the recorded renders through the stage (`NOT RUN` without `VEC_DIR`) |
| `crates/effects/fx_exec/tests/device_warp.rs`, `device_ripmap.rs`, `device_footprint.rs`, `device_pointwise.rs`, `device_backend.rs` | device self-checks and mutants (`NOT RUN` without an OpenCL device) |

## Render flow

1. `sapphire stage apply warp_perspective_autogen` reads the row: `fixed` options (`--host-param blur_mask=0`, `--host-param blur_mocha_mask=0`, `--set shift_mocha_mask=0,-0`), the `host` options (`--src-planes` the source plane count, `--src-head` the reader head of the source, `--owner-arm` the host's opacity arm from which the runner derives `do_postdiv`, `--ae32-dest` the destination planes and mode, `--ctx-word2`, `--ctx-1ae` and `--ctx-has-data` words of the recorded host context, `--palpha-cells` three host record cells that `perspective_alpha` reads, the frame width and two fixed cells; each is parsed with its field named in `crates/effects/fx_exec/src/fxrun.rs` lines 2180 to 2290, and docs/stages.md line 256 and docs/effects.md line 918 describe `--owner-arm` and the context options) and the `placement` options, the same at 64x32 and 50x7: `--persp-base 16`, `--palpha-base 16` and `--xy-base 16`. Another size takes the 64x32 entry (INFERRED). The `host_variants` column is `-`.
2. Offered values are checked against the ranges `fx_run --describe-params` prints; `tele_lens_width` and the other host slider values given through `--set` are put on the 1/65536 grid and the stage prints a `note:` line (MEASURED on the worked command). A keyed value is read per frame ([Keyframes](../stages.md#effects-provider)).
3. Per frame the stage hands the frame to the runner in memory as 'ae32' words with `--size`, `--geom`, `--host-frame` and `--ctx-time` of the frame, `get_time` and `get_frames_per_second` when `--fps` is given, `--ns-default-unbound` and `--do-postdiv` (learned from `POSTDIV-MISMATCH` on the first frame). The program reads no host time (INFERRED from the constructor), so the output of a frame depends only on that frame and the parameter values at it.
4. `fx_exec::fxrun` builds the program from the constructor, binds the parameters and compiles the plan (`crates/effects/fx_eval/src/compile.rs`, `crates/effects/fx_eval/src/fold.rs`): constant nodes fold, `rotate` by a zero angle folds, and the ramp folds to 1 when `wrap_above_horizon` is set or the latitude is low.
5. The nodes run in plan order (`crates/effects/fx_exec/src/exec.rs`); by the constructor (INFERRED) the data flow is the default xy writer, `rotate` (roll), `perspective`, `rotate` (swing), `make_mipmap`, `warp`, `perspective_alpha`, `*` and `postdivide`. `perspective` and `perspective_alpha` run one call per row; the row's output phase is the placement base plus the row offset modulo 32 bytes (`rows_of_p`). `warp` runs one driver call per row and splits the rows into bands across `SAPPHIRE_THREADS` workers when the frame holds 65,536 samples or more (`SAPPHIRE_PAR_MIN` moves that threshold); the other nodes run on one thread. Bands decide only which thread computes which rows, so the bytes do not depend on the thread count.
6. With `filter` set the `warp` node takes the footprint route: the coordinate planes become log2 gradient maps (`warp_log_gradient`), which select the ripmap level of each sample.
7. With `--device gpu|gpu=N|auto` or `SAPPHIRE_DEVICE`, each node runs on the device when its operation passed the self-check and its `accepts` test holds; otherwise that node runs on the CPU with the same arguments ([Device path](#device-path)).
8. The output planes are relaid to frames/2 planes G, B, R, A and written; depths 8 and 16 convert through f32 and are not verified.

From `fx_run` the same plan runs with the options given on its command line: `fx_run Warp_Perspective_Autogen --size WxH --out PREFIX` needs the host context options the stage row holds; a missing one is refused by name ([Build and run](../effects.md#build-and-run)).

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means) on the final binary. For `S_WarpPerspective` `docs/data/effects_status.tsv` holds 19 verified comparisons, 256 recorded renders and 256 re-rendered units. 232 units are byte identical in both renders, every counted one among them; the other 24 are control renders at 50x7 whose recorded renders predate the current default-xy head form (one row buffer where the plan writes the default xy inside its first exec block), and each changed unit's `ae32` equals the original's render or the counted recorded render of the same binding at the same buffer phases ([Effects](../effects.md), the `S_WarpPerspective` section). Through the stage, 10 of the 224 counted recorded renders match the stage's host arrangement, and those 10 are byte identical through `sapphire stage apply` ([Effects and stages](../../README.md#effects-and-stages); they are part of the 382 of `crates/neutral-handoff/tests/fx_stage_parity.rs`). No comparison of this effect differs from the original. The repository does not record the host options of the other 214 counted renders (NEEDS-OUTSIDE: the full report holds them); the stage row holds one placement per size, so only the renders recorded with that placement match it (INFERRED). The controls are the 32 recorded renders that are not counted (256 recorded, 224 counted); docs/effects.md line 854 describes the 24 of them that changed with the head form, and which records the other 8 are is NEEDS-OUTSIDE.

## Checks that run from this repository

Run from the repository root after the README "Build" step. Scratch files go under `target/demo/`.

| Check | Command | Expected result and exit |
|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | exit 0 |
| Input clip | `ffmpeg ... testsrc ... target/demo/warp_in.raw` | exit 0, 98,304 bytes (needs ffmpeg) |
| Stage listed | `sapphire stage list --json` filtered | `warp_perspective_autogen temporal 9 0`, exit 0 |
| Parameters | `fx_run --describe-params Warp_Perspective_Autogen` | `24 parameters`, exit 0 |
| Worked render | `sapphire stage apply warp_perspective_autogen ...` | exit 0, one `note:` line, 98,304 bytes |
| Plan nodes on a device | the worked render with `SAPPHIRE_DEVICE_REPORT=1 --device gpu` | `device: * 3, make_mipmap 3, perspective 3, perspective_alpha 3, postdivide 3, warp 3; fallback: none`, equal bytes, exit 0 (needs OpenCL) |
| `rotate` nodes | the same with `swing` and `roll` set | adds `rotate 6`, equal bytes, exit 0 (needs OpenCL) |
| Special values on a device | special clip with `--device gpu` | `fallback: warp 3 (nan_output 3)`, SHA-256 equal to the `40x12s` golden row, exit 0 (needs OpenCL) |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0 |
| Golden rows on a device | the same with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` | the same line, exit 0 (needs OpenCL) |
| Conformance | `python -I tools/conformance.py --stage ...` | two `FAIL` lines for the 1x1 cases, L3 `33 passed`, exit 1 |
| Refusals | the commands below | exits 3, 3, 3, 3 and 2 |
| Crate tests | `cargo test` lines below | exit 0; the tests named above print `NOT RUN` without `VEC_DIR` |
| Device tests | `cargo test ... -- --test-threads=1` | exit 0 (needs OpenCL; `NOT RUN` without it) |

The build, then a three-frame clip at 64x32 (ffmpeg on `PATH`; without it the `ffmpeg` command fails with the shell's command-not-found exit 127):

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
mkdir -p target/demo
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/warp_in.raw
```

Expected: exit 0 each; `target/demo/warp_in.raw` holds 98,304 bytes (3 frames of 64 x 32 pixels, 16 bytes a pixel).

The stage and its parameters:

```sh
target/release/sapphire stage list --json | python -I -c "import json,sys; m=json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), len(s['layers'])) for s in m['stages'] if s['family'] == 'S_WarpPerspective']"
target/release/fx_run --describe-params Warp_Perspective_Autogen | python -I -c "import json,sys; d=json.load(sys.stdin); print(len(d['Warp_Perspective_Autogen']), 'parameters')"
```

Expected: `warp_perspective_autogen temporal 9 0`, then `24 parameters`; exit 0 each.

One worked render (the command of [Effects](../effects.md), values of a verified comparison):

```sh
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param z_dist=2.5 --param latitude=80 --param tele_lens_width=1.6000061 --param filter=1 < target/demo/warp_in.raw > target/demo/wp_out.raw
```

Expected: exit 0; standard error holds one line, `` note: `tele_lens_width` 1.6000061 is put on the 16.16 host grid as 1.600006103515625 ``; `target/demo/wp_out.raw` holds 98,304 bytes.

The nodes of the plan, counted by the device report, and the device output against the CPU output (an OpenCL platform such as PoCL; without one the first command ends with `DEVICE-REQUIRED-UNAVAILABLE`, exit 3):

```sh
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param z_dist=2.5 --param latitude=80 --param tele_lens_width=1.6000061 --param filter=1 --device gpu --in target/demo/warp_in.raw --out target/demo/wp_dev.raw
cmp target/demo/wp_out.raw target/demo/wp_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --param latitude=60 --param swing=-12 --param roll=30 --param z_dist=2.5 --param wrap=0,2 --device gpu --in target/demo/warp_in.raw --out target/demo/wp_sr_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --param latitude=10 --device gpu --in target/demo/warp_in.raw --out target/demo/wp_lat10.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --param latitude=9.99 --device gpu --in target/demo/warp_in.raw --out target/demo/wp_lat999.raw
```

Expected: exit 0 each. The first command's last line is `device: * 3, make_mipmap 3, perspective 3, perspective_alpha 3, postdivide 3, warp 3; fallback: none`; `cmp` prints nothing; the third command's last line is `device: * 3, make_mipmap 3, perspective 3, perspective_alpha 3, postdivide 3, rotate 6, warp 3; fallback: none`; the fourth (`latitude=10`) ends `device: * 3, make_mipmap 3, perspective 3, perspective_alpha 3, postdivide 3, warp 3; fallback: none` and the fifth (`latitude=9.99`) `device: make_mipmap 3, perspective 3, postdivide 3, warp 3; fallback: none`, without the ramp. The first run of a build on a device pays the self-check of every allowlisted operation (minutes on PoCL with an empty cache folder); later runs read the cache ([Rendering on a device](../guide.md#rendering-on-a-device)).

The special-value clip of the golden table on the device:

```sh
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden as g; open('target/demo/warp_special.raw', 'wb').write(g.synth_special(40, 12, 3, g.SEED))"
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply warp_perspective_autogen --width 40 --height 12 --depth 32 --fps 24 --frames 3 --device gpu --in target/demo/warp_special.raw --out target/demo/wp_special.raw
sha256sum target/demo/wp_special.raw
grep -P '^warp_perspective_autogen\t40x12s\t' tools/fx_golden.tsv | cut -f5
```

Expected: exit 0 each; the report ends `fallback: warp 3 (nan_output 3)`, and the two hashes printed are equal.

The golden rows, on the CPU and on the device, and the rows of this stage:

```sh
python -I tools/fx_golden.py
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
grep -P '^warp_perspective_autogen\t' tools/fx_golden.tsv | cut -f1,2
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0 for each golden run (the second needs OpenCL); the `grep` prints five rows, sizes `40x12s`, `50x7`, `50x7t`, `64x32` and `64x32t`. `tools/fx_golden.py` renders every stage; it has no filter by stage. The `t` rows render with `SAPPHIRE_PAR_MIN=0 SAPPHIRE_THREADS=3` and must equal the plain rows. This stage has no variant rows. Golden reach (INFERRED from the plan and the registered defaults, nodes MEASURED by the reports above): the plain, threaded and special rows reach `*`, `make_mipmap`, `perspective`, `perspective_alpha`, `postdivide` and `warp` on its footprint route (`filter` is on by default); no golden row of this stage reaches `rotate` (`swing` and `roll` at zero), the plan without the ramp (`wrap_above_horizon` set or a low latitude) or `warp` with `filter` off. The device commands above with `swing`, `roll` and the latitude set reach `rotate` and the plan without the ramp.

Conformance (L1 and L3):

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply warp_perspective_autogen"
```

Expected: exit 1. L1 reports `14 passed, 2 failed`: the two `FAIL` lines are the 1x1 cases (`8 bit 1x1 x1 opaque` and `16 bit 1x1 x12 zero`), each `exit 3` with `needs a width of 2 or more`, the documented size refusals ([Conformance](../stages.md#conformance)). L3 reports `33 passed`.

Refusals:

```sh
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=1 --in target/demo/warp_in.raw --out target/demo/wp_r.raw
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=4 --in target/demo/warp_in.raw --out target/demo/wp_r.raw
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --param latitude=90 --in target/demo/warp_in.raw --out target/demo/wp_r.raw
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 24 --layer To=target/demo/warp_in.raw --in target/demo/warp_in.raw --out target/demo/wp_r.raw
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --param 'latitude=60@0;80@2' --in target/demo/warp_in.raw --out target/demo/wp_r.raw
```

Expected exits: 3 (`SOURCE-VARIANT-MISMATCH`), 3 (`bad value for` `cset_opacity`), 3 (`bad value for` `latitude`), 3 (`LAYER-INPUT-UNSUPPORTED`) and 2 (`keyframes need --fps`).

Crate, evaluator, executor and stage tests (cargo builds the tests first):

```sh
cargo test --release --offline -p perspective -p perspective_alpha -p libm_sincos -p warp_driver -p warp_sampler -p warp_log_gradient -p ops_arith
cargo test --release --offline -p warp_render -p make_mipmap -p rotate2d
cargo test --release --offline -p fx_eval --test t_wsr1 --test t_int28_pi --test t_input_index_order
cargo test --release --offline -p fx_exec --test t_xyh1_head_form
cargo test --release --offline -p neutral-handoff --test fx_run_state --test fx_stage_parity
```

Expected: exit 0 each. Without `VEC_DIR` the two `warp_render` tests, `t_input_index_order` and `fx_stage_parity` print a `NOT RUN` line that names the missing stored data and pass. The first line covers default members, which a plain `cargo test --release` also runs; the second line covers crates outside the default members.

Device tests (OpenCL; one thread, as CI runs them):

```sh
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_warp --test device_ripmap --test device_footprint --test device_pointwise --test device_backend -- --test-threads=1
```

Expected: exit 0. Without an OpenCL platform each test prints `NOT RUN` and passes when `SAPPHIRE_REQUIRE_DEVICE` is unset, and fails when it is `1`.

CI (`.github/workflows/ci.yml`) runs these on every push to `main` and every pull request: `cargo test --release` (default members), `python -I tools/fx_golden.py` on Linux, macOS and Windows, the Linux step for the crates outside the default members (`make_mipmap`, `warp_render`, `rotate2d`, `fx_eval`, `fx_exec` and others), and the PoCL job (`device_warp`, `device_ripmap`, `device_footprint`, `device_pointwise`, `device_backend` and the device golden run).

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository; the recorded renders and the stored test data under `VEC_DIR` are kept outside it too. From the repository alone, the golden rows check that this build renders as the build that wrote `tools/fx_golden.tsv`; they do not compare with the original. The tests that print `NOT RUN` without stored data are `crates/effects/kernels/warp_render/tests/warp53_entry.rs`, `crates/effects/kernels/warp_render/tests/alpha_entry_planes.rs`, `crates/effects/fx_eval/tests/t_input_index_order.rs` and `crates/neutral-handoff/tests/fx_stage_parity.rs` ([Tests](../effects.md#tests)). The full report with the runs behind the figures above is identified by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified sizes: 64x32 and 50x7, at depth 32, one band, constant parameters ([Effects](../effects.md), the `S_WarpPerspective` section). Other sizes and values run the same code and are INFERRED.
- Host buffer phase: the default xy writer and the `perspective` and `perspective_alpha` planes take their summation order or vector form from the buffer phase on rows of 29 pixels or more ([Limits common to the effects](../effects.md#limits-common-to-the-effects)). The stage passes phase 16 for the three at every size, the recorded value at both verified sizes; at other sizes it is the 64x32 entry (INFERRED). At another host phase the original's own output can differ. A 1920x1080 frame passes the threshold on every row, so an HD render rests on the stage's phase 16 (INFERRED); a row of 7,680 bytes is a multiple of 32, so every row keeps that phase (`rows_of_p`, `crates/effects/fx_exec/src/kernels2.rs` line 1188). What base a host's allocator gives at other sizes is not recorded (NEEDS-OUTSIDE).
- The program reads no host time (INFERRED from the constructor); `--fps` is needed only for keyed values.
- Approximate reciprocals: `perspective` uses the estimate instruction at six sites (the six variants of `RcpSite`, `crates/effects/kernels/perspective/src/lib.rs` line 107); both sides are modelled with the exact f32 reciprocal (`crates/effects/kernels/perspective/src/lib.rs`), and on real CPUs the output may differ by up to 6 ulp in the first output plane and 12,972 ulp in the second, where values near zero cancel ([Effects](../effects.md), the `S_WarpPerspective` section).
- NaN rules: `perspective` and `perspective_alpha` canonicalize a fresh NaN to the x86 default NaN on every processor (their `nan_x86.rs` tests). The `warp` sampler follows the hardware NaN rule on the CPU and on a device.
- `ROUTE AVX2-ONLY`: the crates reproduce the AVX2 and FMA tier; a dispatcher tier other than that one is reported as `RouteNotRebuilt` by `crates/effects/kernels/warp_render` and is never run. The host offload flag route of `perspective` is not ported.
- `perspective_alpha` refuses its zero-fill route (a ramp scale of 0), which the stage's parameter ranges do not reach (INFERRED from the range of `latitude` and `z_dist`).
- Not verified: the mask, matte and background inputs (not offered; a connected mask is refused by `fx_run`), a real host, depths 8 and 16, and the opacity answers 1 and 3 (refused).
- Refusals with their labels and exits: [Styles and stages](#styles-and-stages) and [Refusal labels](../cli.md#refusal-labels).

## Device path

| Operation | Device kernel | `NAN_EXACT` | Runs on the CPU when |
|---|---|---|---|
| `perspective` | `warp_persp_f32` | no | the output holds a NaN (rerun of that node) |
| `perspective_alpha` | `warp_persp_alpha_f32` | no | the output holds a NaN; the host cells are not given; the zero-fill route |
| `rotate` | `warp_rotate_f32` | no | the output holds a NaN |
| `make_mipmap` | `ripmap_mm_base`, `ripmap_mm_fill`, `ripmap_mm_padcols`, `ripmap_mm_padrows`, `ripmap_mm_hor`, `ripmap_mm_ver` | yes (`NAN_EXACT_COMPILED`) | a NaN output, only on a device whose NaN self-check of the operation reports other payloads |
| `warp` | `warp_warp_f32`; with `filter` set `fp_prefill_u32` and `fp_warp_f32` | no | the output holds a NaN (special golden row: 3 nodes) |
| `*` | `pointwise_bin_f32` | yes (`NAN_EXACT_COMPILED`) | a NaN output, only on a device whose NaN self-check of the operation reports other payloads |
| `postdivide` | `pointwise_postdiv_f32` | yes (`NAN_EXACT_WRITTEN`) | a NaN output, only on a device whose NaN self-check of the operation reports other payloads |

No operation of this plan has a `CPU_ONLY` entry. MEASURED on PoCL only: every node runs on the device with the CPU's bytes and `fallback: none` on the plain and threaded golden rows and on the worked commands above; on the special row `warp` reruns its NaN outputs on the CPU and the bytes equal the golden row. [Speed](../guide.md#speed) gives one 1920x1080 frame of `warp_perspective_autogen` 1.41 / 1.39 s on the CPU and 0.93 / 0.93 s on PoCL (MEASURED under shared load, two runs); PoCL is an OpenCL runtime on the CPU, so it shows byte identity and not GPU speed.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (all groups, on the CPU and with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1`), the crate and executor tests above, `python -I tools/test_effect_rules.py`, the device self-check and mutation tests, the NaN tests on x86 and on the arm64 runner as CI runs them, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees; a change of the bytes needs the comparison harness. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` from the current build and is used only after such a comparison.
- The kernels keep the operation order of the original and `mul_add` exactly where it fuses; a new two-NaN site goes through `fx_common::nanop`, and a fresh NaN is the x86 default NaN ([Crate layout](../adding-an-effect.md#crate-layout)). No platform math function: sine and cosine come from `libm_sincos`.
- A CPU change to `perspective`, `perspective_alpha`, `rotate`, `warp`, `make_mipmap` or `postdivide` removes the operation from `ALLOWLIST` unless its OpenCL kernel changes in the same patch ([Device](../adding-an-effect.md#device)). `warp` and `make_mipmap` are shared with other stages, so their golden rows change too.
- A change of the stage row or of the placement changes `crates/neutral-handoff/data/fx_stages.tsv`; the parameter list lives in `crates/neutral-handoff/data/fx_params.tsv`. Update the `S_WarpPerspective` section of [Effects](../effects.md), [Driving the effect parameters](../parameters-guide.md) and this page with it, then run `python -I tools/gen_docs.py`.

## Open items

- The approximate reciprocal bound of `perspective` (6 and 12,972 ulp) is stated in [Effects](../effects.md); the sensitivity test that measured it is not in the repository. It can be added without the emulator: `perspective::run_with_rcp` (`crates/effects/kernels/perspective/src/lib.rs` line 225) takes the reciprocal as a function, so a Rust test can perturb each `RcpSite` by up to 1.5 x 2^-12 relative (the estimate bound the crate's assumption text gives, line 24) and report the largest ulp change. A 12,972 ulp change of a value near zero is a small absolute change; its effect on the sampled image was not measured. What the emulator returns for the estimate instruction is outside the repository (NEEDS-OUTSIDE).
- `ROUTE AVX2-ONLY` and the host offload route of `perspective` (not ported; `crates/effects/kernels/perspective/src/lib.rs`).
- The `perspective_alpha` zero-fill route is refused by name (`crates/effects/fx_exec/src/kernels2.rs`, `perspective_alpha_k`).
- `warp` keeps the CPU rerun of NaN outputs on a device: its sampler has no NaN-exact kernel ([Rendering on a device](../guide.md#rendering-on-a-device)).
- The opacity answers 1 and 3 are refused with `SOURCE-VARIANT-MISMATCH`: the stage row has no host variants ([Refusal labels](../cli.md#refusal-labels)).
- Only 10 of the 224 counted recorded renders use the stage's host arrangement; the others are verified through `fx_run` with their own arrangements ([Effects and stages](../../README.md#effects-and-stages)).
