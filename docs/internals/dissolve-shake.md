# Internals of `stage apply dissolve_shake`

This page covers the code of this stage: where each part lives, how a render runs, how parity is judged, the checks that run from this repository, the device path and the limits. It is the stage of `S_DissolveShake`. Its shake is the Normal style of `S_Shake`, whose offset, coordinates and resampling are described in [`S_Shake` internals](shake.md); this page describes the transition around it. The user reference is the `S_DissolveShake` section of [Effects](../effects.md); the parameters are described in [Driving the effect parameters](../parameters-guide.md), the stage in the section "Effects provider" of [Adding a provider](../stages.md#effects-provider) and the transition inputs in [Transition stages](../cli.md#transition-stages). The architecture every effect shares is in [Effect internals](README.md).

## What it computes

A transition from a first clip to a second clip. One progress value drives it: with `auto_trans` 1 the host's percent time of the frame, which runs from 0 on the first frame of the clip to 1 on its last, and with `auto_trans` 0 the value of `dissolve_amt`. `transition_dir` swaps which clip leaves and which arrives. The dissolve weight is the progress moved away from the middle of the transition by the factor `dissolve_speed`; the dissolve gives the first clip where the weight is 0 or less, the second clip where it is 1 or more, and between them a fused linear mix per channel. The shake amplitude is `amplitude` times a triangle of the progress, zero at the start, full at the middle and zero again at the end, and the shake of `S_Shake` (program `Shake_Normal`) resamples the mixed image with it; `mo_blur_length` is its motion blur length. Where the weight shows only one clip, before the dissolve starts or after it ends, the compiled plan reads only that clip (read from the constructors, INFERRED; the operations are MEASURED below).

The expressions, with p the progress: the dissolve weight is w = 1/2 + `dissolve_speed` x (p - 1/2) (`defun_delay_dissolve` in `crates/effects/fx_eval/src/programs_native.rs`), and the dissolve gives A for w <= 0, B for w >= 1 and fma(w, B - A, A) per channel between, with A the leaving clip and B the arriving one (module comment of `crates/effects/kernels/dissolve/src/lib.rs`); the shake amplitude is `amplitude` times 2p for p up to 1/2 and times 2 - 2p above it (an `if>` in `eff_dissolve_shake`), so it is 0 at p = 0 and p = 1 and `amplitude` at p = 1/2. The shake offsets are those of [`S_Shake` internals](shake.md) (read from the code, INFERRED).

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Dissolve_Shake (registered without a wrapper program) | Dissolve_Shake | `stage apply dissolve_shake` | temporal | `To`, optional | false | `cset_opacity` 1 and 3 |

The row is line 21 of `crates/neutral-handoff/data/fx_stages.tsv`: the source name is `From` (the stage input), the `layers` column `To?` makes it a transition stage with an optional second clip, the `fixed` column is empty (the program has no mask input), and the placement is `--xy-base 0` at 50x7 and at 64x32 (served from the 64x32 entry elsewhere). The `host_variants` entries for the Opacity answers 1 and 3 replace the reader options of those answers; at answer 1 the entry also sets the class records of `From` and `To`: `--class-record NAME=CLASS` gives the class byte recorded for each bound input (`crates/effects/fx_exec/src/fxrun.rs` line 2161), the class the runner also derives from the input-class rule (`crates/effects/fx_exec/src/input_class.rs`; class 4 is the four-lane class the kernels implement, `CLASS_IMPLEMENTED`), and at answer 1 both clips are three-plane sources, class 3, whose route the entry's `--class3-record` and `--class-cells` options describe. The [Effect internals](README.md) index defines these host terms. The stage offers 36 parameters (`stage list` in [Effects provider](../stages.md#effects-provider)). `stage list` prints the registered default of `auto_trans` as a zero-based index, and `fx_run --describe-params` prints the program text default; an unset `auto_trans` takes the registered one ([Parameter discovery](../cli.md#parameter-discovery)).

Refusals of this stage (exit codes through `sapphire stage apply`):

| Refusal | Exit | When |
|---|---|---|
| `FRAMES-REQUIRED` | 3 | no `--frames N`; also the pipeline stage `--stage @dissolve_shake`, since the pipeline passes no `--frames` (MEASURED below for the stage) |
| `FRAMES-TOO-SMALL` | 3 | `--frames 1` (MEASURED below) |
| `FPS-UNGIVEN` | 3 | no `--fps`: the shake reads `get_frames_per_second` (MEASURED below) |
| `LAYER-ID-UNKNOWN` | 3 | a layer id other than `To`, for example `From` (MEASURED below) |
| `LAYER-DUPLICATE`, `LAYER-FILE-MISSING`, `LAYER-SIZE-MISMATCH` | 3 | a layer given twice, a file that cannot be opened, a file that is not whole frames of the input size and depth (a named pipe reports 0 bytes) |
| `the input holds` | 4 | `--frames` differs from the number of input frames, after the frames were written (MEASURED below) |
| `frames but the input has more` | 4 | a layer of more than one frame that is shorter than the input |
| `unknown param`, `bad value for` | 3 | a parameter the stage does not offer (for example `phase`, MEASURED below) or a value outside its range |
| `needs a width of 2 or more` | 3 | a frame 1 pixel wide |
| `HOST-TIME-RANGE` | 3 | a rate or a frame tick outside the host's 32-bit time fields |

Each label is in [Refusal labels](../cli.md#refusal-labels). The refusals of the frame count and of the layer come before any frame is read.

## Code map

| Part | Path | Role |
|---|---|---|
| Program constructor | `crates/effects/fx_eval/src/programs_native.rs`: `eff_dissolve_shake` | inputs `From` and `To`, the progress, the dissolve and the call of `Shake_Normal` |
| Transition helpers | the same file: `defun_transition_amount`, `defun_transition_swap_inputs`, `defun_delay_dissolve` | the progress from `auto_trans`, the swap by `transition_dir`, the dissolve weight from `dissolve_speed` |
| Shake | the same file: `eff_shake_normal` and its helpers | see [`S_Shake` internals](shake.md) |
| Layout | the same file: `layout_dissolve_shake` | parameter groups |
| Parameter definitions | `crates/effects/rtdata/src/styles/dissolve_shake.rs` | registered parameters, host types, ranges, normalize scales and input records |
| Shared class rows | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS`, `COMBOS`, `COMBOS_EXEC` | the rows of `dissolve` and of the shake classes |
| `dissolve` | `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`, `dissolve_k`), crate `dissolve` (`src/lib.rs`, `src/consts.rs`) | the clamped linear mix per channel, each operand a scalar or a plane |
| Shake operations | `make_mipmap`, `+`, `*`, `rotate`, `warp`, `interp`, `blur_warp`, `postdivide` and the channel operations | as in the code map of [`S_Shake` internals](shake.md) |
| Percent time | `crates/neutral-handoff/src/fxstage.rs`: `check_time`, `percent_time`, `time_values` | `--frames` checks and the host values `get_percent_time`, `get_total_frames`, `get_aspect_ratio`, `get_time` and `get_frames_per_second` of each frame |
| Second clip | the same file: `load_layers`, `run_fx`, `unread_source`, `drop_source` | reads the layer, hands it to the runner as `To`, and renders a frame again without the clip that its plan does not read |
| Unread inputs | `crates/effects/fx_exec/src/srcindep.rs` (`not_an_input`), `crates/effects/fx_exec/src/input_class.rs` (`check_unread_inputs`) | the runner's refusal of a given clip that the compiled plan does not read |
| Device entries | `crates/effects/fx_exec/src/device_ops_pointwise.rs` (`device_ops_pointwise::DISSOLVE`), with the shake entries of [`S_Shake` internals](shake.md) | |
| OpenCL source | `crates/effects/fx_device/src/kernels_pointwise.rs` (`pointwise_dissolve_f32`) | the dissolve kernel |
| Stage rows | `crates/neutral-handoff/data/fx_stages.tsv` line 21; `crates/neutral-handoff/data/fx_params.tsv` lines 258 to 293 | host arrangement and offered parameters |
| Computed tables | the shake tables of [`S_Shake` internals](shake.md) | |

The operations of the compiled plans are MEASURED with the device report of the commands below: the worked render (`auto_trans` 0, `dissolve_amt` between the start and the end of the dissolve) runs `+` (two nodes), `blur_warp`, `dissolve`, `interp`, `make_mipmap` and `postdivide` per frame. With `auto_trans` 1 over three frames, frame 0 runs `blur_warp`, `interp`, `make_mipmap` and `postdivide` only (the progress is 0: the plan reads only the first clip and the offset folds to zero), frame 1 adds `dissolve` and two `+` nodes, and frame 2 again runs no `dissolve` (the plan reads only the second clip).

## Render flow

1. `sapphire stage apply dissolve_shake --width W --height H --depth 32 --fps N[/D] --frames N [--layer To=PATH] ...` finds the row, checks `--frames` (`FRAMES-REQUIRED`, `FRAMES-TOO-SMALL`) and then the layer (`LAYER-*`) before any frame is read (`check_time`, `load_layers`), checks the `host_variants` column, the width, the parameters and the rate. Values of 16.16 sliders are put on the 1/65536 grid with a `note:` line, as on the `S_Shake` stages.
2. The layer file is one frame, held for the whole clip, or one frame per input frame; each layer frame is handed to the runner in memory as 'ae32' words under the id `To`. Without a layer the program reads no second clip and the first clip dissolves into transparent black ([Effects](../effects.md), the `S_DissolveShake` section).
3. Frame f gets `get_percent_time` f32(f) / f32(N - 1), `get_total_frames` f32(N), `get_aspect_ratio` f32(H) / f32(W), and `get_time` and `get_frames_per_second` from `--fps` (`time_values`; [Host values](../cli.md#host-values)). The argument list is built as for `S_Shake` (host column or variant entry, placement, parameters, `--do-postdiv`).
4. The runner compiles the plan. The fold computes the progress, the weight and the shake offsets as constants of the frame (the `shake_shift` crate). Where the weight selects one clip, the plan holds no `dissolve` node and does not read the other clip; the runner then refuses the clip it was given (`not_an_input`), and the stage renders the frame again without it, as a host hands a plan only the inputs it reads (`unread_source`, `drop_source`). With the Opacity answer 1 entry the count of three-plane source images in the host options drops by one with that clip ([Effects provider](../stages.md#effects-provider)).
5. The executor runs `dissolve` on the two clips with the folded weight, then the shake chain of [`S_Shake` internals](shake.md) on its output: `make_mipmap`, `+` on the default xy, `interp` and `blur_warp` with motion blur (or `warp` without it), and `postdivide` at the default Opacity answer 2 (the answers 1 and 3 run none, MEASURED by the coverage loop below).
6. Threading, device dispatch and output are as for `S_Shake`. Output frame f depends on input frame f, layer frame f, the parameters, N and the rate; there are no frame requests and no history.

The direct route `fx_run Dissolve_Shake --size WxH --out PREFIX ...` takes the same list; `fx_run` itself keeps refusing a given clip that the compiled plan does not read.

## How parity is judged

An effect counts as rebuilt when every registered style passes seven checks on the final binary: the program text, the builtin classes, the data it reads, its literals, the compiled plan, the combination tables and the execution, each judged against the original running in an emulator ([What rebuilt means](../effects.md#what-rebuilt-means)).

- `S_DissolveShake` has 38 verified comparisons, 304 recorded renders and 304 re-rendered units (`docs/data/effects_status.tsv`): each of the 38 parameter sets is recorded at the eight base phases 0, 4, ..., 28 of the default xy buffer.
- Through the stage, 27 of the 36 verified renders that equal the original are byte identical to it (README "Effects and stages"), among them the 8 whose plan reads only one of the two clips (6 only the first, 2 only the second). The other 9 are 50x7 renders recorded at phase 16, verified through `fx_run` at that phase and through the stage against the runner's render at phase 0 ([Effects provider](../stages.md#effects-provider)).
- Two verified comparisons differ from the original, one at each size, in every word of the output (8192 of 8192 at 64x32, 1400 of 1400 at 50x7). Their recorded renders were made with two faults: the vertical random amplitude was converted by the frame height where the original converts every axis by the frame width, and the sine scale of the shake offset was pi where the original's is 2 pi. The code has the original's sine scale, and with the amplitude converted by the width the same parameter sets give the original's output byte for byte (MEASURED after the original's output was known, so both stay counted as different; [Effects](../effects.md), the `S_DissolveShake` section).
- Two more parameter sets are not counted and are not among the recorded renders: their vertical wave amplitude was handed over converted by the height with the sine scale pi, so their plans differ from the original's in four folded constants; their output is a constant image.
- 16 re-rendered units of `S_DissolveShake` changed with the sine scale of the shake offset: one counted unit for each of the two comparisons that differ, the rest control units ([What rebuilt means](../effects.md#what-rebuilt-means)).
- Controls: the 304 recorded renders are the 38 parameter sets at 8 phases; each set counts once, at the phase of its verified run, and its other members are control renders (INFERRED from the counts; the list of members is outside the repository).
- The 9 renders outside the stage figure differ from the stage's host arrangement only in the placement option `--xy-base` (phase 16 against the stage's 0); a host variant entry cannot carry a placement option (`HOST-VARIANT-PLACEMENT`). At 50x7 the stage's output equals the original's for the renders recorded at phase 0 (the 27 above) and is checked against the runner at phase 0 for the 9.
- The "40 counted recorded renders" of the device paragraph are the 38 verified comparisons and the 2 uncounted parameter sets, all rendered through `fx_run` (INFERRED from the counts of the `S_DissolveShake` section of [Effects](../effects.md)).
- The fault of the two differing comparisons lay in the value handed to the runner (the vertical amplitude converted by the height), not in the runner or the stage (README "Status"); counting them as equal needs new recorded renders made in the harness. Two repository tests guard against both faults: `t_host_param_ns1` checks that every axis amplitude is converted by the frame width at 64x32, 50x7 and 49x9 (its feature `ns1_height_mut` makes the 50x7 tests fail), and `k_sine_is_the_measured_cell` in `shake_shift`'s `t1_body` pins the two pi sine scale (commands below).

## Checks that run from this repository

Run from the repository root after the [Build](../../README.md#build) step; scratch files go to `target/demo/dissolve-shake/`.

| Check | Command | Expected result | Exit |
|---|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | the two binaries | 0 |
| Stage listed | `sapphire stage list --json` filtered | `temporal 36 [{'id': 'To', 'optional': True}]` | 0 |
| Parameters | `fx_run --describe-params` with the program name | one JSON member | 0 |
| Worked render | `stage apply dissolve_shake ... --frames 3 --layer To=...` | three output frames of 64x32 at depth 32 | 0 |
| Refusals | no `--frames`; `--frames 1`; no `--fps`; `--layer From=...`; `--frames 5` on 3 frames; `--param phase=0.5` | `FRAMES-REQUIRED`; `FRAMES-TOO-SMALL`; `FPS-UNGIVEN`; `LAYER-ID-UNKNOWN`; `the input holds 3 frames`; `unknown param` | 3, 3, 3, 3, 4, 3 |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 13 rows of this stage | 0 |
| Tests | the `cargo test` commands below | all pass; six `NOT RUN` lines (all of `t12`) without `VEC_DIR` | 0 |
| Conformance | `tools/conformance.py --stage ".../sapphire stage apply dissolve_shake --frames 12"` | L1 `4 passed, 7 failed`, L3 `32 passed, 1 failed` | 1 |
| Device | golden run and report with `SAPPHIRE_DEVICE=gpu` | as below (needs OpenCL) | 0 |

Build, list the stage and print the parameters:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
target/release/sapphire stage list --json | python -I -c "import json,sys; d=json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in d['stages'] if s['id'] == 'dissolve_shake']"
target/release/fx_run --describe-params Dissolve_Shake
```

The second command prints `dissolve_shake temporal 36 [{'id': 'To', 'optional': True}]`. Defaults and ranges come from these commands only.

The worked render of the `S_DissolveShake` section of [Effects](../effects.md) with three frames of a second clip (ffmpeg on `PATH`), then the refusals:

```sh
mkdir -p target/demo/dissolve-shake
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/dissolve-shake/in.raw
ffmpeg -v error -y -f lavfi -i testsrc2=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/dissolve-shake/to.raw
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer To=target/demo/dissolve-shake/to.raw --param auto_trans=0 --param transition_dir=0 --param dissolve_amt=0.3700000047683716 --param amplitude=1.6999969482421875 --param frequency=5.5 --param x_phase=-0.350006103515625 --param seed=5.0 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/out.raw
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/noframes.raw # rc 3
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 --frames 1 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/frames1.raw # rc 3
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --frames 3 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/nofps.raw # rc 3
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer From=target/demo/dissolve-shake/to.raw < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/layerid.raw # rc 3
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 --frames 5 --layer To=target/demo/dissolve-shake/to.raw < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/frames5.raw # rc 4
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --param phase=0.5 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/phase.raw # rc 3
```

The render writes three frames of 64x32 at depth 32 and prints `note: ... seed ... is put on the 16.16 host grid as 5`. The refusals print, in order, the labels `FRAMES-REQUIRED`, `FRAMES-TOO-SMALL`, `FPS-UNGIVEN` and `LAYER-ID-UNKNOWN`, the message `the input holds 3 frames, but --frames says 5` (exit 4, after the three frames were written) and ``unknown param `phase` ``, each on an `error:` line of standard error.

The golden rows: plain `64x32` and `50x7` (no second clip), threaded `64x32t` and `50x7t`, special `40x12s`, and the variants `+o1` and `+o3` (Opacity answers 1 and 3), `+to` (a `To` layer from the seed plus one) and `+ch` (`auto_trans` 0, `dissolve_amt` inside the dissolve and `red_amplitude` set, which reaches the channel operations):

```sh
python -I tools/fx_golden.py
grep -E '^dissolve_shake\s' tools/fx_golden.tsv | cut -f1,2
```

The first prints `fx_golden: 183 cases, 183 equal, 0 differ`; the second prints the 13 rows of this stage.

Which operations the golden rows reach (the plan depends on the parameters, the times, the size, the opacity answer and whether a clip is read, not on the pixel values, INFERRED; needs OpenCL):

```sh
for v in "" "--layer To=target/demo/dissolve-shake/to.raw" "--param cset_opacity=1" "--param cset_opacity=3" "--param auto_trans=0 --param transition_dir=0 --param dissolve_amt=0.5099999904632568 --param red_amplitude=0.5"; do printf "[%s] " "$v"; SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply dissolve_shake --device gpu --width 64 --height 32 --depth 32 --fps 24 --frames 3 $v < target/demo/dissolve-shake/in.raw 2>&1 >/dev/null | tail -1; done
```

| Golden group | Operations over the 3 frames (MEASURED by the loop) |
|---|---|
| plain, threaded, special and `+to` | `+` (2), `blur_warp` (3), `dissolve` (1, the middle frame), `interp` (3), `make_mipmap` (3), `postdivide` (3) |
| `+o1`, `+o3` | as plain without `postdivide` |
| `+ch` | `*` (3), `+` (24), `blur_warp` (9), `copy_alpha` (9), `dissolve` (3), `element_of` (18), `get_alpha` (9), `interp` (9), `make_mipmap` (9), `postdivide` (3), `v3` (3), `v3_to_v4_if4_a` (3) |

The golden rows leave `motion_blur` unset and all reach `interp` and `blur_warp`; no golden row of this stage reaches `warp` or `rotate`. With `auto_trans` unset the first and last frames of a 3 frame clip read one clip each, so the plain rows reach the stage's removal of an unread clip.

The tests of the transition, the dissolve and the shake (`-- --nocapture` shows the `NOT RUN` lines):

```sh
cargo test --release --offline -p neutral-handoff --lib -- transition percent_time unread_clip
cargo test --release --offline -p neutral-handoff --test stage_cli a_stage_that_reads_the_frame_rate_is_refused_without_fps
cargo test --release --offline -p fx_exec --test t12 --test t_unread_class3 --test popup_default -- --nocapture
cargo test --release --offline -p dissolve -p shake_shift -p blur_warp
cargo test --release --offline -p fx_eval --test t_shake_shift_fold --test t_frame_rate --test t_get_time_fold --test t_host_param_ns1
cargo test --release --offline -p shake_shift --test t1_body k_sine_is_the_measured_cell
```

All end with exit 0. The library filter runs the stage's unit tests whose names contain `transition`, `percent_time` or `unread_clip`, among them `a_transition_frame_is_rendered_without_the_clip_its_plan_does_not_read` (on a 3 frame clip at 16x8, frame 0's plan does not read `To` and frame 2's does not read `From`, and the stage renders both) and `percent_time_formula`. `stage_cli` checks `FPS-UNGIVEN` on this stage with `--frames 2`. `t12` holds six tests of the `dissolve` class (three by hand values, three over the stored program corpus); every one reads stored data under `VEC_DIR` (the hand tests read the style parameter index), so without `VEC_DIR` all six print a `NOT RUN` line and pass without checking (MEASURED on check29, `logs/a5_t12_check29.log`); `t_unread_class3` checks the class of `From` and `To` and the check of an unread `To` (at Opacity answer 1 the plan can drop the dissolve); `popup_default` checks that an unset popup, `auto_trans` included, compiles its registered default. The crate `dissolve` holds no test of its own; without `VEC_DIR` its CPU kernel is checked only by the golden rows with a dissolve (the middle frame of every row of `stage apply dissolve_shake`, and the `+ch` row on all three frames) and by the device self-check, which compares the device kernel with it. The shake tests are listed in [`S_Shake` internals](shake.md).

Conformance needs the clip length in the stage command:

```sh
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply dissolve_shake --frames 12" # rc 1
```

It prints `L1 stage matrix FAIL 4 passed, 7 failed`, `L3 failure injection FAIL 32 passed, 1 failed` and `result: fail (exit 1)`. The `FAIL` lines are the documented refusals: the two 1x1 cases (`needs a width of 2 or more`, exit 3), the cases of 1 or 2 frames (`the input holds N frames, but --frames says 12`, exit 4) and the empty input of L3 (exit 4) ([Conformance](../stages.md#conformance)).

The device checks (an OpenCL platform such as PoCL; without one they behave as described in [`S_Shake` internals](shake.md)):

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply dissolve_shake --device gpu --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer To=target/demo/dissolve-shake/to.raw --param auto_trans=0 --param transition_dir=0 --param dissolve_amt=0.3700000047683716 --param amplitude=1.6999969482421875 --param frequency=5.5 --param x_phase=-0.350006103515625 --param seed=5.0 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/out_dev.raw
cmp target/demo/dissolve-shake/out.raw target/demo/dissolve-shake/out_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply dissolve_shake --device gpu --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer To=target/demo/dissolve-shake/to.raw --param amplitude=1.25 --param frequency=7 < target/demo/dissolve-shake/in.raw > target/demo/dissolve-shake/auto_dev.raw
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; open('target/demo/dissolve-shake/special.raw', 'wb').write(fx_golden.synth_special(40, 12, 3, 20261106))"
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply dissolve_shake --device gpu --width 40 --height 12 --depth 32 --fps 24 --frames 3 < target/demo/dissolve-shake/special.raw > target/demo/dissolve-shake/special_dev.raw
target/release/sapphire stage apply dissolve_shake --width 40 --height 12 --depth 32 --fps 24 --frames 3 < target/demo/dissolve-shake/special.raw > target/demo/dissolve-shake/special.out
cmp target/demo/dissolve-shake/special.out target/demo/dissolve-shake/special_dev.raw
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_ripmap --test device_footprint -- --test-threads=1
```

The golden run prints `fx_golden: 183 cases, 183 equal, 0 differ`. The worked render's report ends `device: + 6, blur_warp 3, dissolve 3, interp 3, make_mipmap 3, postdivide 3; fallback: none` and `cmp` prints nothing. The `auto_trans` 1 report shows the per-frame plans described under "Code map", ending `device: + 2, blur_warp 3, dissolve 1, interp 3, make_mipmap 3, postdivide 3; fallback: none`. The special clip is the input of the golden row `40x12s` (holding infinities, NaN and subnormal values); its report ends `fallback: blur_warp 2 (nan_output 2)`, and the CPU render equals the device render (`cmp` silent).

The pipeline route: `--stage @dissolve_shake` is refused, and the whole stage command with `--frames` equal to the clip length runs (ffmpeg on `PATH`; the clip has 3 frames):

```sh
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -pix_fmt yuv420p target/demo/dissolve-shake/clip.mp4
target/release/sapphire target/demo/dissolve-shake/clip.mp4 --depth 32 --stage @dissolve_shake -o target/demo/dissolve-shake/pipe1.mp4 --force # rc 3
target/release/sapphire target/demo/dissolve-shake/clip.mp4 --depth 32 --stage "$PWD/target/release/sapphire stage apply dissolve_shake --width {width} --height {height} --depth {depth} --fps {fps} --frames 3" -o target/demo/dissolve-shake/pipe2.mp4 --force
```

The second command ends `error: unsupported: pre1 exited with code 3: ... FRAMES-REQUIRED ...`; the third writes `pipe2.mp4` (64x32, 24 fps) with the transition from the clip into transparent black, since the command gives no layer.

CI (`.github/workflows/ci.yml`) runs `cargo test --release` (the default members, among them `neutral-handoff`, `dissolve`, `shake_shift` and `blur_warp`), the non-default crates step on Linux (`fx_eval`, `fx_exec`), `python -I tools/fx_golden.py` on Linux, macOS and Windows, `conformance.py --quick --allow-absent-peer`, `test_effect_rules.py`, and in the job "device backend on PoCL" the device tests and the device golden run.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders and the stored test data. The 38 verified comparisons and the 304 re-rendered units cannot be rerun from here. Without `VEC_DIR`, all six tests of `t12` print `NOT RUN` (the hand tests read the style parameter index, the others the program corpus), and the shake tests that read recorded cases print `NOT RUN` as listed in [`S_Shake` internals](shake.md). Whether the host's percent time for a transition is f / (N - 1) over the clip given (in and out points, a transition cut from a longer clip, a rate with a non-integer step), the phase a host allocator gives at 50x7, and the output at Opacity answer 3 with a second clip, of the channel shake at answer 3 and of the late `auto_trans` 1 branch can only be judged in the harness. The full report is identified by the SHA-256 printed in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Sizes 64x32 and 50x7; each verified render is one frame of a transition of 240 frames at 24 frames a second, at the frames 0 to 190, with the second clip absent or present. The time values of other clip lengths and rates are INFERRED ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- The transition spans exactly the clip the stage is given, from 0 on its first frame to 1 on its last; a second clip that starts at another time, or a clip cut from a longer transition, is not modelled (a choice of the stage, INFERRED; [Effects provider](../stages.md#effects-provider)).
- Opacity answers 1, 2 and 3 are verified at both sizes with input alpha other than 1; answer 3 only without a second clip (an all-zero `To` layer), so its output with a second clip is INFERRED. No verified render sets a channel parameter at answer 3. The channel alpha drop at answer 1 is verified by a render at 50x7 only (INFERRED at 64x32).
- With `auto_trans` 1, after the dissolve ends the plan renders the second clip alone through the shake; that branch is verified only with no second clip, so its output for other second clips is INFERRED.
- Host buffer phase: as for `S_Shake`, the default xy writer takes its order from its phase on rows of 29 pixels or more; at 50x7 the output depends on it, and the stage passes phase 0 at every size. A 1920x1080 render is above the threshold and rests on the stage's phase 0 (INFERRED); the phase a host gives is outside the repository.
- Pipeline: `--stage @dissolve_shake` is refused with `FRAMES-REQUIRED`, because the pipeline passes no `--frames`. The pipeline runs the transition when `--stage` carries the whole stage command with `--frames` equal to the clip length (MEASURED below); docs/roadmap.md lists no change to the `@` form.
- No approximate reciprocal instruction runs on the verified render path (MEASURED by a static scan of the original's code; that the route reaches none is INFERRED). `ROUTE AVX2-ONLY` applies to the shake offset as in [`S_Shake` internals](shake.md).
- NaN: on the device, NaN outputs of `blur_warp` are rerun on the CPU (2 nodes on the special golden row, MEASURED above). Depths 8 and 16 are not verified. Refusals: the table under "Styles and stages".

## Device path

| Operation | Device kernel | `NAN_EXACT` | Fallback to the CPU |
|---|---|---|---|
| `dissolve` | `device_ops_pointwise::DISSOLVE`, `pointwise_dissolve_f32` | yes (`NAN_EXACT_COMPILED`) | a declined shape |
| `+`, `make_mipmap`, `postdivide` | as in [`S_Shake` internals](shake.md) | yes | a declined shape |
| `interp`, `blur_warp`, `warp` | as in [`S_Shake` internals](shake.md) | no | NaN outputs (`nan_output`) |
| channel operations (`+ch` row) | as in [`S_Shake` internals](shake.md) | yes, `*` as well | a declined shape |

No operation of these plans has a `CPU_ONLY` entry. MEASURED on PoCL: every node runs on the device with the CPU's bytes and `fallback: none` on the plain, threaded and variant golden rows and on all 40 counted recorded renders through `fx_run`; on input with NaN, `blur_warp` reruns its NaN outputs on the CPU ([Effects](../effects.md), the `S_DissolveShake` section). [Speed](../guide.md#speed) holds 1.26 / 1.23 s on the CPU and 0.53 / 0.52 s on PoCL for one 1920x1080 frame of `stage apply dissolve_shake` (a 3 frame clip divided by 3; MEASURED on a 12 core server under shared load; GPU speed is INFERRED).

## Changing the code safely

- Gates that must stay green: `python -I tools/fx_golden.py` (all groups, the `+to` and `+ch` variants included), the tests above and those of [`S_Shake` internals](shake.md), `python -I tools/test_effect_rules.py`, the device self-check and device tests on PoCL, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes changed; the table holds this repository's own renders, so the repository alone cannot say whether the original agrees. `python -I tools/fx_golden.py --write` rewrites the whole table and is used only after a change whose new output was judged against the original outside the repository.
- The `dissolve` kernel uses a fused multiply add at its mix (`f32::mul_add`); the crate's feature `mutant_nofma` replaces it with a separate multiply and add as a mutation check. No test or CI step enables the feature (`.github/workflows/ci.yml`), so no repository check shows the mutant detected (open item). A change to its CPU result removes `dissolve` from `ALLOWLIST` unless its device kernel changes in the same patch; the NaN and platform math rules of [Adding an effect](../adding-an-effect.md) apply as for `S_Shake`.
- A change to the transition inputs keeps the stage's order of checks (`--frames`, then the layer, before any frame) and the removal of an unread clip; the unit tests named above check both.
- What to update: the `S_DissolveShake` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), [Transition stages](../cli.md#transition-stages) for a change to the inputs, line 21 of `fx_stages.tsv` and the rows of `fx_params.tsv`, `tools/fx_golden.tsv` (after the judgement above), `docs/data/effects_status.tsv`, this page, then `python -I tools/gen_docs.py`.

## Open items

- The two verified comparisons that differ stay counted as different although the current code reproduces the original's output for them ([Effects](../effects.md), the `S_DissolveShake` section).
- Opacity answer 3 with a second clip, the channel shake at answer 3, the channel alpha drop at 64x32 and the late branch of `auto_trans` 1 with a second clip have no verified render ([Effects](../effects.md)).
- Transitions that do not span the clip given to the stage are not modelled ([Effects provider](../stages.md#effects-provider)).
- The crate `dissolve` holds no unit test of its own (`crates/effects/kernels/dissolve`), and no repository check enables its `mutant_nofma` feature.
- The `@dissolve_shake` pipeline form is refused (`FRAMES-REQUIRED`); the pipeline passes no clip length to a stage.
- `blur_warp` is not on the `NAN_EXACT` list, so its NaN outputs are rerun on the CPU (`crates/effects/fx_exec/src/device_ops.rs`).
