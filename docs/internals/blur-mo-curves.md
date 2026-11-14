# Internals of `stage apply transform_and_blur_autogen` and `stage apply blur_motion_curves_no_transform_autogen`

This page covers the code of these stages: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. They are the two styles of `S_BlurMoCurves`. Their sampler, `blur_warp` over a ripmap from `make_mipmap`, is shared with `S_BlurMotion` and is described in full on the [`S_BlurMotion` internals](blur-motion.md) page (section "The motion blur sampler"); this page covers what differs: the host curves, the two styles and their stage rows. The user reference is the `S_BlurMoCurves` section of [Effects](../effects.md), the `S_BlurMoCurves` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md).

## What it computes

Motion blur along a path that the host supplies as curves. The program reads the host curves `rotate`, `z_dist` and `shift` (`defun_blur_mo_curves_get`, `crates/effects/fx_eval/src/programs_native.rs` line 1752) at two times, the start `shutter_shift + shutter_duration / 2` and the end `shutter_shift - shutter_duration / 2` (`defun_blur_motion_curves_from_to`, line 1761), in frames relative to the current frame, and the host point `center` once. Transform_and_Blur maps each pixel through the transform of each time (the offset from `center` turned by the rotation and scaled by the depth distance, plus `center` minus the shift) and averages the source along the segment between the two mapped positions, so the image follows the curve and is blurred over the shutter interval. Blur_Only takes the difference of the two transforms, divided by twice the current depth distance and turned by the current rotation, as an offset, and averages the source between the pixel position minus and plus that offset, so the image stays in place and is blurred along its motion. The segment's start and end weights are `1 - exposure_bias` and `exposure_bias`, `scale_result` scales the result, `blur_res` sets full, half or quarter resolution, `subpixel` turns sub-pixel sampling on or off and `wrap` sets the edge mode per axis (read from the constructors). The sampler's own formula (sample count, weight ramp, normalization) is in the section "The motion blur sampler" of the [`S_BlurMotion` internals](blur-motion.md) page.

Each curve read is one host checkout at the tick trunc((t + F) x M): t is the time above, F the frame (`--host-frame`, the 0-based frame index on the stages) and M the ticks per frame of the curve file's `time_mult` line (doubled under the host's doubling rule); the sum and the product are f32 and the conversion truncates toward zero (`shutter_tick`, `crates/effects/kernels/tickmap/src/lib.rs` line 373; `time_multiplier`, line 363). The file must hold a `value` line for exactly that parameter index and tick: a time between two ticks reads the tick its truncation gives, and nothing is interpolated between ticks (a host's keyframe interpolation happens before it answers and is not modelled here). The value is the line's raw 16.16 number times 2^-16, and the rotation enters the program negated (`crates/effects/fx_eval/tests/t_tpu2b.rs` lines 1 to 3). At frame 0 with the default `shutter_duration` both times truncate to tick 0, so that frame shows no motion whatever the curve (INFERRED from `shutter_tick`).

## Styles and stages

| style (program) | stage | kind | layers | spool | host variants | stage row |
|---|---|---|---|---|---|---|
| Transform_and_Blur_Autogen | `stage apply transform_and_blur_autogen` | temporal | none | false | none | `crates/neutral-handoff/data/fx_stages.tsv` line 4 |
| `Blur_Motion_Curves_No_Transform_Autogen` (the Blur_Only answer of the Effect popup) | `stage apply blur_motion_curves_no_transform_autogen` | temporal | none | false | none | `crates/neutral-handoff/data/fx_stages.tsv` line 5 |

Both stages offer the same 9 parameters (`crates/neutral-handoff/data/fx_params.tsv` lines 36 to 43 and 142 for `transform_and_blur_autogen`, lines 44 to 51 and 143 for `blur_motion_curves_no_transform_autogen`): `shutter_duration`, `scale_result`, `subpixel`, `exposure_bias`, `shutter_shift`, `blur_res` and `wrap` as record values (`--set`), `cset_opacity` as a host answer (`--cset Opacity`) and `host_curve_values` as a file path (`--host-curve-values`). The registered programs hold 25 parameters each (`fx_run --describe-params`). The curve parameters themselves are not offered: their values come from the curve file. The fixed column binds the mask and mocha parameters (`blur_mocha_mask`, `shift_mocha_mask`, `blur_mask`, `invert_mask`, `mask_use`); the mask input is not connected.

The Blur_Only style has no registered program of its own in the plugin: the Effect popup answer 2 selects it, and its plan equals the compile of `Blur_Motion_Curves_No_Transform_Autogen` (`crates/effects/fx_exec/src/style.rs`, `STYLE_TABLE` and `style_program`). The row of `blur_motion_curves_no_transform_autogen` therefore declares the answer in its `host` column (`--effect-answer 2`, `--effect-count 2`), and `fx_run` checks it before anything else; a wrong declaration is refused by name (`STYLE-SELECT-MISMATCH`, `STYLE-SELECT-COUNT-MISMATCH`, `STYLE-INDEX-OUT-OF-TABLE`, exit 2 from `fx_run`; a stage does not reach them).

Refused answers and values, each before any frame is written (MEASURED on this tree, commands below):

| input | result |
|---|---|
| `--param cset_opacity=1` or `3` | exit 3, `SOURCE-VARIANT-MISMATCH`: only the answer 2 (Normal) has a recorded host arrangement on these stages |
| `--param host_curve_values=PATH` with an empty file | exit 3, `TICKMAP-POINT-HOST-INPUT-UNSOURCED` |
| `--param blur_res=3` | exit 3, `bad value for` |
| `--layer To=PATH` | exit 3, `LAYER-INPUT-UNSUPPORTED` |
| a frame 1 pixel wide | exit 3, `needs a width of 2 or more` |
| `--param host_curve_values=PATH` with a file that lacks a tick the frame reads | exit 3, `HOST-CURVE-SAMPLE-UNSERVED`, naming the missing sample as `HOST-CURVE-VALUE-MISSING (PROGRAM: index I, tick T, step S, scale C)` |

Without `host_curve_values` the stage adds the samples its identity curve lacks (see [Render flow](#render-flow)); there `HOST-CURVE-SAMPLE-UNSERVED` ends only a sample of a parameter the stage offers or of another index.

## Code map

| part | path | role |
|---|---|---|
| style constructors | `crates/effects/fx_eval/src/programs_native.rs`: `eff_transform_and_blur` (line 1368), `eff_transform_and_blur_autogen` (1427), `eff_blur_motion_curves_no_transform` (380), `eff_blur_motion_curves_no_transform_autogen` (440), the curve helpers `defun_blur_mo_curves_get` (1752) and `defun_blur_motion_curves_from_to` (1761), the layout rows `layout_transform_and_blur_autogen` (1958) and `layout_blur_motion_curves_no_transform_autogen` (1995) | the program graphs; the `_Autogen` wrappers add the mask and mocha group |
| style facts | `crates/effects/rtdata/src/styles/transform_and_blur.rs`, `crates/effects/rtdata/src/styles/blur_motion_curves_no_transform.rs` (registered in `crates/effects/rtdata/src/styles/mod.rs` lines 8, 17, 42 and 51) | registered parameters, ranges, input indices and the curve parameter records (field `tickmap`) |
| class facts | `crates/effects/rtdata/src/classes.rs` | the class rows of `blur_warp`, `make_mipmap`, `postdivide` and the arithmetic classes (line numbers on the [`S_BlurMotion` internals](blur-motion.md) page) |
| host curves | `crates/effects/fx_eval/src/hostcurve.rs` (the curve file format in its header; `fold` line 102); crate `crates/effects/kernels/tickmap` (`time_multiplier` line 363, `shutter_tick` 373) | serves each `get_curve_val` sample from the curve file through `tickmap::map_param`, at compile time |
| identity curve | `crates/neutral-handoff/src/fxstage.rs`: `constant_curve` (line 698), `identity_sample` (754), `parse_curve_missing` (769), `CURVE_RERUNS` (786), the repeat loop (1108) | the curve file the stage writes when none is given, and the samples it adds |
| style selection | `crates/effects/fx_exec/src/style.rs` (`STYLE_TABLE` line 35, `style_program` 47, `select` 62) | the Effect popup rule |
| plan ops and kernels | `blur_warp`, `make_mipmap`, `+`, `-`, `*`, `postdivide`: executor entries and crates as on the [`S_BlurMotion` internals](blur-motion.md) page | the sampler and the position arithmetic |
| downsampled block (`blur_res` 1 and 2) | `crates/effects/fx_exec/src/exec.rs` `execute_downsampled` line 355; `crates/effects/fx_exec/src/downsampled.rs` | runs the body on halved planes |
| device entries | `crates/effects/fx_exec/src/device_ops.rs`, `device_ops_ripmap.rs`, `device_ops_pointwise.rs`; OpenCL in `crates/effects/fx_device/src/kernels_ripmap.rs` and `kernels_pointwise.rs` | as on the [`S_BlurMotion` internals](blur-motion.md) page |
| stage rows | `crates/neutral-handoff/data/fx_stages.tsv` lines 4 and 5; `crates/neutral-handoff/data/fx_params.tsv` lines 36 to 51, 142 and 143 | host arrangement, placement, fixed options, offered parameters |
| golden rows | `tools/fx_golden.tsv` lines 17 to 21 and 166 to 170 | five rows per stage: `40x12s`, `50x7`, `50x7t`, `64x32`, `64x32t` |
| tests | see [Checks that run from this repository](#checks-that-run-from-this-repository) | |

No computed tables: the curve, the sampler and the arithmetic read no table.

The ops of the compiled plans are MEASURED with the device report (`SAPPHIRE_DEVICE_REPORT=1`, commands below), per frame on a 64x32 clip with the identity curve: `transform_and_blur_autogen` runs `+` 2, `-` 2, `blur_warp` 1, `make_mipmap` 1 and `postdivide` 1; `blur_motion_curves_no_transform_autogen` with `shutter_duration=2` runs `*` 4, `+` 1, `-` 4, `blur_warp` 1, `make_mipmap` 1 and `postdivide` 1. On the identity curve the rotations are zero and fold away. A curve that turns adds `rotate` and `*` nodes: with the curve file of the commands below, `transform_and_blur_autogen` runs `*` 2, `+` 2, `-` 2, `blur_warp` 1, `make_mipmap` 1, `postdivide` 1 and `rotate` 2 on its frame, with `fallback: none` and the CPU's bytes (MEASURED). No golden row runs that route; the `rotate` kernel on its own is covered by `device_warp` and the `rotate2d` tests (listed on the [`S_BlurMotion` internals](blur-motion.md) page).

## Render flow

From `sapphire stage apply transform_and_blur_autogen` (or `blur_motion_curves_no_transform_autogen`) to the output planes:

1. The stage reads its rows of `fx_stages.tsv` and `fx_params.tsv`, checks the given values against the ranges `fx_run --describe-params` prints (exit 3 outside them), refuses `--layer` and a width under 2 (exit 3), and reads keyed values (keys need `--fps`).
2. Curve file: with `--param host_curve_values=PATH` the stage passes that file unchanged to every frame, and a sample the frame reads that the file lacks ends the run with `HOST-CURVE-SAMPLE-UNSERVED` (exit 3, MEASURED). Without it, the stage writes an identity curve for the frame in the recorded format (rotation 0, shift 0, depth distance 1, centre 50 by 50, `constant_curve`) with the ticks F - 1, F and F + 1 (tick 0 only at frame 0). When `fx_run` names a sample the curve lacks (`HOST-CURVE-VALUE-MISSING`) of the indices 22 to 25 of the stage's own program, the stage adds an identity sample for it and renders the frame again in process, at most 64 times a frame (`CURVE_RERUNS`); a learned sample is kept for the clip as an offset from the frame, so later frames need no repeat (`crates/neutral-handoff/src/fxstage.rs` lines 1108 to 1157). Any `shutter_duration` therefore runs on the identity curve: a 3 frame 64x32 clip of `transform_and_blur_autogen` takes 0.07 s at `shutter_duration` 1 and 0.09 s at 200 (MEASURED on a 12 core server under load). A sample of an offered parameter or of another index ends with `HOST-CURVE-SAMPLE-UNSERVED` (exit 3).
3. For each frame the stage hands the runner the input frame as 'ae32' words, the options of the `host` column, the placement of the frame size, the `fixed` options, `--size`, `--geom`, `--host-frame` and `--ctx-time` from the frame, the offered parameters and `--host-curve-values`.
4. Placement: `transform_and_blur_autogen` runs one placement at every size (`--ds-phase 0`, `--xy-base 0`). `blur_motion_curves_no_transform_autogen` has a per size table: at 50x7 it passes `--xy-base-list 0,0` and `--xy-render 1`, at 64x32 and elsewhere the 64x32 entry. `--ds-phase` is the 32-byte phase of the downsampled planes; `--xy-base-list B,B` gives the base phase in bytes (0 to 31) of each default-xy writer of the placement model in execution order, here two; `--xy-render 1` says the run is the first render, the one in which a preserve writer runs (`crates/effects/fx_exec/src/fxrun.rs` lines 1062 to 1063). The `notes` column is a text for the reader (here: a table at the measured sizes, the 64x32 placement elsewhere, INFERRED) and is not read by the stage. A placement entry for another size, such as 1920x1080, comes only from recorded runs of the original at that size, outside this repository; without one the stage uses the 64x32 entry.
5. The runner checks the declared Effect answer (Blur_Only only), compiles the program, serves each `get_curve_val` from the curve file through `fx_eval::hostcurve` (constants of the run), folds the constant arms (the mask group, zero rotations) and adds `postdivide` for the opacity route; `--do-postdiv` is learned from `POSTDIV-MISMATCH` as on the other stages ([Effects provider](../stages.md#effects-provider)).
6. The plan computes the two end points (or the offset of Blur_Only) with `-`, `*` and `+`, builds the ripmap of the source with `make_mipmap`, samples it with `blur_warp` and ends with `postdivide`; at `blur_res` 1 or 2 the body runs inside the downsampled block. The sampler is described on the [`S_BlurMotion` internals](blur-motion.md) page.
7. Threading: none of these operations splits its rows into bands, so `SAPPHIRE_THREADS` and `SAPPHIRE_PAR_MIN` do not change the work (INFERRED from the code); the threaded golden rows equal the plain rows (MEASURED).
8. Device: as on the [`S_BlurMotion` internals](blur-motion.md) page; every node runs on the device, and a NaN output of `blur_warp` reruns on the CPU.
9. The output is relaid as frames/2 planes. Frame N of the output depends on input frame N and on the curve at the two times of frame N; no other clip frame is read.

From `fx_run PROGRAM`: the runner takes the same options directly, among them `--host-curve-values PATH`; a missing sample is `HOST-CURVE-VALUE-MISSING`, which only the stage answers.

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means): program text, builtin classes, the data it reads, literals, compiled plan, combination tables and execution, each against the original running in an emulator.

| figure | `S_BlurMoCurves` | source |
|---|---|---|
| verified comparisons | 41 (24 Transform_and_Blur, 17 Blur_Only) | `docs/data/effects_status.tsv` line 3; the `S_BlurMoCurves` section of [Effects](../effects.md) |
| recorded renders | 69 | `docs/data/effects_status.tsv` line 3 |
| re-rendered units | 69 | `docs/data/effects_status.tsv` line 3 |
| byte identical through the stages | 26 of 67 (13 of 35 for Transform_and_Blur_Autogen, 13 of 32 for Blur_Motion_Curves_No_Transform_Autogen) | README.md, "Effects and stages"; the table of stage renders in [Effects provider](../stages.md#effects-provider) |

All 41 comparisons equal the original; no comparison differs. The renders through the stages are checked by the stage provider's parity run, `crates/neutral-handoff/tests/fx_stage_parity.rs`, which needs the stored data (382 of 382 over the stages of ten effects, these two among them). That test hands each stored member's parameters to the stage as `--param` values (lines 133 to 135), so a member that carries `host_curve_values=PATH` renders with its recorded curve file; which members carry one is in the stored members list, not in the repository. Of the 69 recorded renders, 67 are counted and 2 are controls (recorded and not counted, "What rebuilt means" in [Effects](../effects.md)); the repository does not list which. Of the 67 counted renders, the 41 outside the stage figure use a host arrangement other than the row's `host`, `placement` and `fixed` options; the repository names only the kinds of difference (a different host arrangement, a fractional frame, a `--prev` file outside the history chain, in the last paragraph of [Effects provider](../stages.md#effects-provider)), and the row has no `host_variants` entry that could serve them. The curve values of the comparisons come from a stand-in for the host; 2 of the 13 recorded renders of each style that match the stage's host arrangement have a recorded curve file equal to the identity curve text at frame 0 (MEASURED; other curve values and later frames are INFERRED). The entry code of the route's builtins uses no approximate reciprocal instruction (MEASURED); the kernel bodies behind it were not listed, so whether one is reached is not measured (INFERRED to be none). The `blur_warp` crates model no approximate instruction: their assumption lists name fused multiply-add sites only (`crates/effects/kernels/blur_warp_1ch/src/lib.rs` lines 155 and 156), so the repository states no ulp bound for this route; the emulator's own result for a body that would reach one is outside the repository.

Since the default-xy head form, `blur_motion_curves_no_transform_autogen` at its default parameters renders differently than before that form at some widths that are not a multiple of 8, where the stage serves the size from its 64x32 entry: 1,342 of 5,292 words differ at 49x9, 647 of 2,220 at 37x5 and 144 of 1,872 at 52x3, and the output is identical at 64x32 and 50x7 (MEASURED against the build before that form). That the new output is the original's at those sizes is INFERRED. The head form is the shape in which the runner writes the default xy plane that the Blur_Only plan reads: as one row buffer that every row shares (one head) or as a plane whose rows each take their own address phase (per row). The plan of Blur_Only at its defaults writes it as one row buffer, and Transform_and_Blur at its defaults has no default-xy writer (`crates/effects/fx_exec/tests/t_xyh1_head_form.rs` lines 147 to 152). In a per row plane row r starts at the base plus 4 r W bytes, so its phase modulo 32 bytes stays the same on every row only when W is a multiple of 8, and the writer's vector loop takes its order from that phase (`crates/effects/fx_exec/src/types.rs` lines 517 to 524). At such a width the two forms give the same words; at another width served from the 64x32 entry (`--xy-base 0`) they can differ, which is where the change shows, while the 50x7 entry passes its recorded phase list and gives the same output in both builds (INFERRED from the code).

## Checks that run from this repository

Run from the repository root after the README "Build" step. Scratch files go to the git ignored `target/demo/blur-mo-curves/`. ffmpeg is needed only to make the test clip; the device rows need an OpenCL platform such as PoCL.

| check | command | expected result and exit code |
|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | exit 0 |
| stage list | `sapphire stage list --json` filtered to the two stages | two lines, `temporal 9 0`; exit 0 |
| parameters | `fx_run --describe-params Transform_and_Blur_Autogen Blur_Motion_Curves_No_Transform_Autogen` | 25 entries per program; exit 0 |
| worked renders | `sapphire stage apply` on a 3 frame 64x32 clip | exit 0; the shutter render equals its input |
| refusals | `cset_opacity` 1 and 3, an empty curve file, `blur_res=3`, `--layer`, width 1 | exit 3 each |
| curve file | a curve that turns, on one frame, CPU and device; a tick the file lacks | `rotate 2` in the report, `cmp` against the input exit 1; exit 3 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; exit 0 |
| golden rows on a device | the same with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` | the same line; exit 0 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 ... --device gpu`, then `cmp` | `fallback: none`; exit 0 |
| NaN rerun | the special clip of `tools/fx_golden.py` on the device | `fallback: blur_warp 3 (nan_output 3)`; exit 0 |
| conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply STAGE"` | 2 `FAIL` lines (1x1), L3 33 passed; exit 1 |
| curve tests | `cargo test` of `fx_eval` curve tests, `tickmap` and the stage's curve unit tests | exit 0 |
| stage parity | `cargo test -p neutral-handoff --test fx_stage_parity` | `NOT RUN` without `VEC_DIR`; exit 0 |

Make the clip, list the stages and print the parameter counts:

```sh
mkdir -p target/demo/blur-mo-curves
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/blur-mo-curves/in.raw
target/release/sapphire stage list --json | python -I -c "import json, sys; d = json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), len(s['layers'])) for s in d['stages'] if s['id'] in ('transform_and_blur_autogen', 'blur_motion_curves_no_transform_autogen')]"
target/release/fx_run --describe-params Transform_and_Blur_Autogen 'Blur_Motion_Curves_No_Transform_Autogen' | python -I -c "import json, sys; d = json.load(sys.stdin); [print(p, len(v)) for p, v in d.items()]"
```

Each command exits 0. The stage list prints `transform_and_blur_autogen temporal 9 0` and `blur_motion_curves_no_transform_autogen temporal 9 0`; the parameter list prints `Transform_and_Blur_Autogen 25` and the same count, 25, for `Blur_Motion_Curves_No_Transform_Autogen`. Without ffmpeg on `PATH` the second command fails and the renders below have no input.

Worked renders on the identity curve and the refusals:

```sh
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param scale_result=0.765625 < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/out.raw
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --frames 3 --param shutter_duration=2 --param shutter_shift=0.5 --param exposure_bias=0.25 --param wrap=1,2 --in target/demo/blur-mo-curves/in.raw --out target/demo/blur-mo-curves/shutter.raw
cmp target/demo/blur-mo-curves/in.raw target/demo/blur-mo-curves/shutter.raw
target/release/sapphire stage apply blur_motion_curves_no_transform_autogen --width 64 --height 32 --depth 32 --frames 3 --param shutter_duration=2 --param blur_res=1 --param scale_result=0.75 --in target/demo/blur-mo-curves/in.raw --out target/demo/blur-mo-curves/blur_only.raw
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=1 < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/r.raw
target/release/sapphire stage apply blur_motion_curves_no_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=3 < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/r.raw
printf '' > target/demo/blur-mo-curves/empty_curve.txt
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --fps 24 --param host_curve_values=target/demo/blur-mo-curves/empty_curve.txt < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/r.raw
target/release/sapphire stage apply blur_motion_curves_no_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --param blur_res=3 < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/r.raw
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --fps 24 --layer To=target/demo/blur-mo-curves/in.raw < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/r.raw
target/release/sapphire stage apply transform_and_blur_autogen --width 1 --height 32 --depth 32 --fps 24 < target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/r.raw
```

The first four commands and the `printf` exit 0. On the identity curve the shutter render returns its input unchanged, so `cmp` prints nothing (the first render scales the result and differs from its input). The six other stage commands exit 3 with `SOURCE-VARIANT-MISMATCH`, `SOURCE-VARIANT-MISMATCH`, `TICKMAP-POINT-HOST-INPUT-UNSOURCED`, `bad value for`, `LAYER-INPUT-UNSUPPORTED` and `needs a width of 2 or more`, and write no frame.

A curve that turns, shifts and changes depth. The `python` command writes a curve file in the format of `constant_curve`: its `host`, `unit` and `point` lines, read from `crates/neutral-handoff/src/fxstage.rs`, then one `value` line per parameter index and tick from -2 to 2, in the form `value NAME INDEX TICK STEP SCALE STATUS RAW` of the header of `crates/effects/fx_eval/src/hostcurve.rs`: `rotate` (index 23) 5 degrees a tick, `shift` x (index 24) 0.25 a tick, `shift` y (index 25) 0, and `z_dist` (index 22) 1 plus 0.05 a tick, each RAW a 16.16 number. The renders use one frame, so the times of `shutter_duration=4` read the ticks -2 and 2:

```sh
head -c 32768 target/demo/blur-mo-curves/in.raw > target/demo/blur-mo-curves/in1.raw
python -I -c 's = open("crates/neutral-handoff/src/fxstage.rs").read(); f = s[s.index("\"host effect_ref") + 1:s.index("{values}\",")].replace("\\n\\\n", "\n").replace("\\n", "\n"); v = "".join("value %s %d %d 1 24 0 %d\n" % (n, i, t, r) for t in range(-2, 3) for (n, i, r) in (("rotate", 23, 327680 * t), ("shift", 24, 16384 * t), ("shift", 25, 0), ("z_dist", 22, 65536 + 3277 * t))); open("target/demo/blur-mo-curves/curve.txt", "w").write(f.replace("{w}", "64").replace("{h}", "32").replace("{t}", "0") + v)'
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --param shutter_duration=4 --param host_curve_values=target/demo/blur-mo-curves/curve.txt --in target/demo/blur-mo-curves/in1.raw --out target/demo/blur-mo-curves/curve_cpu.raw
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --param shutter_duration=4 --param host_curve_values=target/demo/blur-mo-curves/curve.txt --device gpu --in target/demo/blur-mo-curves/in1.raw --out target/demo/blur-mo-curves/curve_dev.raw
cmp target/demo/blur-mo-curves/curve_cpu.raw target/demo/blur-mo-curves/curve_dev.raw
cmp target/demo/blur-mo-curves/in1.raw target/demo/blur-mo-curves/curve_cpu.raw
target/release/sapphire stage apply blur_motion_curves_no_transform_autogen --width 64 --height 32 --depth 32 --param shutter_duration=4 --param host_curve_values=target/demo/blur-mo-curves/curve.txt --in target/demo/blur-mo-curves/in1.raw --out target/demo/blur-mo-curves/curve_bo.raw
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --param shutter_duration=6 --param host_curve_values=target/demo/blur-mo-curves/curve.txt --in target/demo/blur-mo-curves/in1.raw --out target/demo/blur-mo-curves/r.raw
```

The first five commands exit 0; the device render (needs an OpenCL platform) ends with the report line `device: * 2, + 2, - 2, blur_warp 1, make_mipmap 1, postdivide 1, rotate 2; fallback: none`, and the first `cmp` prints nothing. The second `cmp` reports that the files differ and exits 1: the curve moves the image. The Blur_Only render exits 0. The last command reads the ticks -3 and 3, which the file lacks, and exits 3 with `HOST-CURVE-SAMPLE-UNSERVED` naming `HOST-CURVE-VALUE-MISSING (Transform_and_Blur_Autogen: index 23, tick 3, step 1, scale 24)`. The repository holds no expected hash for a curve render: no golden row and no test renders a curve file.

Golden rows, CPU and device. The table holds five rows for each stage; the second command lists them:

```sh
python -I tools/fx_golden.py
awk -F'\t' '$1 == "transform_and_blur_autogen" || $1 == "blur_motion_curves_no_transform_autogen"' tools/fx_golden.tsv
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

The first and third commands end with `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0 (the device run on PoCL); the second prints 10 rows and exits 0. The golden clip has no curve file, so every golden row of these stages renders on the identity curve; the `64x32` rows of the two stages hold the same hash (`tools/fx_golden.tsv` lines 20 and 169). On the golden clip at the default parameters the plans run `+`, `-`, `blur_warp`, `make_mipmap` and `postdivide`, and `*` on Blur_Only (MEASURED with the device report). No golden row reaches `rotate`, a curve file, the downsampled block (`blur_res` 1 and 2), an edge mode of `wrap` other than none, `subpixel` 0, or another `exposure_bias`, `shutter_shift` or `scale_result`; the special rows reach the NaN rerun of `blur_warp`. Without an OpenCL platform every case of the third command is refused (`DEVICE-REQUIRED-UNAVAILABLE`), the tool prints `fx_golden: 183 cases, 0 equal, 183 differ` and exits 1.

Device report, CPU equality and the NaN rerun (needs an OpenCL platform):

```sh
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply blur_motion_curves_no_transform_autogen --width 64 --height 32 --depth 32 --frames 3 --param shutter_duration=2 --param blur_res=1 --param scale_result=0.75 --device gpu --in target/demo/blur-mo-curves/in.raw --out target/demo/blur-mo-curves/blur_only_dev.raw
cmp target/demo/blur-mo-curves/blur_only.raw target/demo/blur-mo-curves/blur_only_dev.raw
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden as g; sys.stdout.buffer.write(g.synth_special(g.SPECIAL_SIZE[0], g.SPECIAL_SIZE[1], g.FRAMES, g.SEED))" > target/demo/blur-mo-curves/special.raw
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply transform_and_blur_autogen --width 40 --height 12 --depth 32 --fps 24 --frames 3 --device gpu < target/demo/blur-mo-curves/special.raw > target/demo/blur-mo-curves/special_dev.raw
python -I -c "import hashlib; print(hashlib.sha256(open('target/demo/blur-mo-curves/special_dev.raw', 'rb').read()).hexdigest())"
```

Every command exits 0 and `cmp` prints nothing. The last report line of the first render is `device: * 12, + 3, - 12, blur_warp 3, make_mipmap 3, postdivide 3; fallback: none`. On the special clip it is `device: + 6, - 6, make_mipmap 3, postdivide 3; fallback: blur_warp 3 (nan_output 3)`, and the printed hash equals the `40x12s` row of `transform_and_blur_autogen` in `tools/fx_golden.tsv`. Without a device the device renders exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`.

Conformance of each stage (L1 and L3):

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply transform_and_blur_autogen"
python -I tools/conformance.py --stage "target/release/sapphire stage apply blur_motion_curves_no_transform_autogen"
```

Each prints `L1 stage matrix FAIL 14 passed, 2 failed` and `L3 failure injection pass 33 passed` and exits 1. The two `FAIL` lines are the documented 1x1 size refusals (`needs a width of 2 or more`, exit 3); see [Conformance](../stages.md#conformance).

Curve, stage and sampler tests (cargo):

```sh
cargo test --release --offline -p fx_eval --test t_tpr --test t_tpu --test t_tpu2 --test t_tpu2b --test t_pw_point --test t32_hcv_doubling --test t20_hcnoif
cargo test --release --offline -p tickmap
cargo test --release --offline -p neutral-handoff --lib curve
cargo test --release --offline -p neutral-handoff --test fx_stage_parity
```

Each exits 0. The `fx_eval` tests check the curve fold of both programs (the parameter records, the point route of `center`, the unit route of `shift`, the refusals by name, and that a `get_curve_val` with no curve interface is a compile error); `tickmap` is the host fetch mapping; the `neutral-handoff` unit tests filtered by `curve` check the identity curve's shape, the parse of a missing sample, and that an offered parameter is never served by the identity curve. `fx_stage_parity` prints `NOT RUN: VEC_DIR is not set` without the stored data and passes (cargo shows the line with `-- --nocapture`). The sampler, arithmetic and device tests are listed on the [`S_BlurMotion` internals](blur-motion.md) page.

CI (`.github/workflows/ci.yml`): `cargo test --release` (line 77, which runs `tickmap` and the `neutral-handoff` tests), `python -I tools/fx_golden.py` on Linux, macOS and Windows (line 88), `fx_eval` on Linux (line 101), and the device golden run and device tests on PoCL (lines 191 to 193).

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders, their recorded curve files and the stored test data. From the repository alone the checks above confirm the code's own hashes, device and CPU equality and the refusals, not agreement with the original. The 41 comparisons and the 69 recorded renders are not in the repository; `crates/neutral-handoff/tests/fx_stage_parity.rs` renders the counted recorded renders through the stages only with `VEC_DIR` and prints `NOT RUN` without it. The full report is kept outside the repository; its SHA-256 is printed in the "Evidence" section of [Effects](../effects.md). The host stand-in that produced the curve values of the comparisons, and the curve interface of a real host (its tick base and keyframe interpolation), are outside the repository as well; the runner reads only the file format in the header of `crates/effects/fx_eval/src/hostcurve.rs`.

## Verified scope and limits

- Sizes: 64x32 and 50x7. Other sizes run the same code with the 64x32 placement of the per size table (INFERRED), and at some widths that are not a multiple of 8 `blur_motion_curves_no_transform_autogen` renders differently than before the default-xy head form (above).
- Curves: the curve values of the verified comparisons came from a stand-in for the host. The identity curve at frame 0 is verified for 2 of the 13 renders of each style; other curves and later frames are INFERRED. The frame time enters only through the curve file's ticks.
- Host buffer phase: the downsampled planes (small planes 31 pixels or more wide) and the default xy writer of Blur_Only (rows of 29 pixels or more) take their order from a host buffer phase above those thresholds; the stage passes phase 0 from its placement ("Limits common to the effects" in [Effects](../effects.md)).
- A 1920x1080 frame: `transform_and_blur_autogen` at its defaults (no default-xy writer, `blur_res` 0) reaches none of these loops, so its output does not depend on a host buffer phase (INFERRED). `blur_motion_curves_no_transform_autogen`, and both stages at `blur_res` 1 or 2, reach them; the stage passes phase 0 from its 64x32 entry, which is a recorded phase at 64x32 and INFERRED for a real host at 1920x1080. No 1920x1080 render is verified.
- Opacity: only the answer 2 renders; 1 and 3 are refused (`SOURCE-VARIANT-MISMATCH`, exit 3), since no recorded render sets an opacity popup.
- NaN: as on the [`S_BlurMotion` internals](blur-motion.md) page; NaN payload and sign bits are outside the verified cases.
- Routes not rebuilt: `ROUTE AVX2-ONLY`; the mask input and mocha; keyed host curves from a real host. The labels are listed in [Refusal labels](../cli.md#refusal-labels).
- INFERRED: other sizes, parameter values, curves and keyed values between keys.

## Device path

Every operation of these plans has a device kernel and no `CPU_ONLY` entry: `blur_warp` and `make_mipmap` (`kernels_ripmap.rs`), `+`, `-`, `*` and `postdivide` (`kernels_pointwise.rs`). `NAN_EXACT` membership and the fallback reasons are those of the table on the [`S_BlurMotion` internals](blur-motion.md) page: `blur_warp` reruns its NaN outputs on the CPU; `make_mipmap`, `+`, `-`, `*` and `postdivide` keep theirs on a device whose self-check reports equal NaN payloads. MEASURED on PoCL only: every node on the device with `fallback: none` on the plain and threaded golden rows of both stages, and 3 `blur_warp` nodes rerun on each special golden row, with the CPU's bytes. Timing rows in [Speed](../guide.md#speed), one 1920x1080 frame: `transform_and_blur_autogen` 1.03 / 1.01 s on the CPU and 0.73 / 0.69 s on PoCL, `blur_motion_curves_no_transform_autogen` 1.09 / 1.10 s and 0.74 / 0.73 s; the earlier route took 11.26 / 11.87 s and 52.84 / 51.09 s. Speed on a real GPU is INFERRED.

## Changing the code safely

- Gates that stay green: `python -I tools/fx_golden.py` (all groups, also on a device), the curve, stage and sampler tests above and on the [`S_BlurMotion` internals](blur-motion.md) page, `python -I tools/test_effect_rules.py`, the device tests on PoCL, `python -I tools/gen_docs.py --check` and `python -I tools/check_docs.py`.
- A golden difference means that the output bytes changed; the table holds this repository's own renders, so the repository alone cannot tell whether the original agrees. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` and is used only after a deliberate change checked by other means. The golden rows render on the identity curve only: a change to the curve fold (`hostcurve.rs`, `tickmap`) is checked by the curve tests above, not by the golden rows.
- The NaN, platform math and `ALLOWLIST` rules of [Adding an effect](../adding-an-effect.md) apply to the shared kernels as described on the [`S_BlurMotion` internals](blur-motion.md) page.
- Docs and data to update with a change: the `S_BlurMoCurves` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), `docs/data/effects_status.tsv`, the timing rows of [Speed](../guide.md#speed) when they change, then `python -I tools/gen_docs.py`.

## Open items

- Only the Normal opacity answer renders on these stages (`SOURCE-VARIANT-MISMATCH` for 1 and 3): no recorded render sets an opacity popup ([Driving the effect parameters](../parameters-guide.md), "The opacity popup").
- No golden row and no test renders a curve file: `tools/fx_golden.py` has no curve case, so a change to the curve fold is caught only by the `fx_eval` curve tests, and a curve render has no expected hash.
- The 2 control renders, and the host arrangements of the 41 counted renders outside the stage figure, are not listed in the repository (the stored data and the full report hold them).
- A placement entry for a size other than 64x32 and 50x7 needs recorded runs of the original at that size; the stage serves every other size from the 64x32 entry (the notes column of the row).
- Curves other than the identity curve, and later frames of the identity curve, are INFERRED ([Effects provider](../stages.md#effects-provider), "Host curves").
- The output of `blur_motion_curves_no_transform_autogen` at widths that are not a multiple of 8 changed with the default-xy head form; that the new output is the original's there is INFERRED (the `S_BlurMoCurves` section of [Effects](../effects.md)).
- Whether the kernel bodies behind the entry code reach an approximate reciprocal is not measured (the `S_BlurMoCurves` section of [Effects](../effects.md)).
- `blur_warp` keeps the CPU rerun of its NaN outputs on the device ([Rendering on a device](../guide.md#rendering-on-a-device)).
