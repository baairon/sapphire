# Internals of `stage apply swishpan`

This page covers the code of this stage: where each part lives, how a render runs, how its parity is judged, the checks that run from this repository, the device path and the limits. It is the stage of `S_SwishPan`, a transition stage registered without a wrapper program. The user reference is the `S_SwishPan` section of [Effects](../effects.md); its parameters are described in [Driving the effect parameters](../parameters-guide.md) and its stage in the section "Effects provider" of [Adding a provider](../stages.md#effects-provider). The parts every effect shares are described once in [Effect internals](README.md).

## What it computes

A transition between two clips. The first clip (the program input `From`, which is the stage input) and the second clip (the program input `To`, which is the stage layer) are each resampled with an offset along the move, so that the first clip leaves the frame and the second follows it in; the two resampled clips are combined with `screen_super`, and the result is blurred along the direction of the move. The offset is the transition progress times a shift of one frame width to the left or right, or of one frame height up or down (`direction`), multiplied by one minus `overlap`; the vertical shift is scaled by the frame height over its width (`get_aspect_ratio`). The progress is the host's percent time (`get_percent_time`, 0 on the first frame of the clip and 1 on the last) when `auto_trans` is 1, or the Wipe Percent sample of the host curve when it is 0, eased by `slow_in` and `slow_out`; `transition_dir` swaps the two clips. The blur width is `blur_amount` times the speed of the eased progress: its derivative when `auto_trans` or `swish_auto_trans` is above 0, otherwise the difference of the eased Wipe Percent half a frame after and half a frame before the frame, times the clip length (`get_total_frames`) (INFERRED from the constructor `eff_swishpan`).

In formulas (INFERRED from `eff_swishpan` and its two helper programs; coordinates are the program's normalized units, in which the frame width spans 2):

- progress p: `get_percent_time` when `auto_trans` is 1, else the Wipe Percent sample W(t) of the host curve at the frame's tick; eased e = slow_in_out(p, `slow_in`, `slow_out`), the constant fold `fx_eval::fold::slow_in_out` (`crates/effects/fx_eval/src/fold.rs` line 995);
- shift S: (2, 0) for `direction` 0, (-2, 0) for 1, (0, -2 H/W) for 2 and (0, 2 H/W) for 3, with H/W from `get_aspect_ratio`; D = S x (1 - `overlap`);
- the first clip is sampled at xy + D x e and the second at xy + D x e - D, each by `warp` of its `make_mipmap` pyramid; `transition_dir` 1 swaps the two clips;
- output: `blur` of `screen_super`(first, second), along x for `direction` 0 and 1 and along y for 2 and 3, of width `blur_amount` x e'(p) (the derivative form of slow_in_out) when max(`auto_trans`, `swish_auto_trans`) is above 0, else `blur_amount` x (e(W(t + 0.5)) - e(W(t - 0.5))) x N with N from `get_total_frames`.

The arithmetic of each op is in its kernel crate (Code map): the box passes of `blur` in `crates/effects/kernels/blur`, the sampler of `warp` in `crates/effects/kernels/warp_render`.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| registered without a wrapper program | the registered program, `program` column of `fx_stages.tsv` line 17 (`fx_run --describe-params SwishPan`) | `stage apply swishpan` | `temporal` | `To`, optional | false | entries for `cset_opacity=1` and `cset_opacity=3` |

The stage is one of the two transition stages of this build, with the `S_DissolveShake` stage ([Transition stages](../cli.md#transition-stages)). It takes `--frames N` (the clip length, which sets the time values of every frame) and an optional `--layer To=PATH` (the second clip: one frame held for the whole clip, or one frame per input frame). Without the layer the `To` input is a held frame of zero words. Keyed values are accepted, with `--fps` as their time base. The program reads no frame time or rate, so `--fps` is optional: a run without it gives the bytes of a run with `--fps 24` (MEASURED). All three answers of `cset_opacity` render: answer 2 runs the `host` column and answers 1 and 3 run the `host_variants` entries.

Driving the Wipe Percent (`auto_trans` 0): `host_curve_values` names a curve file in the text format that the header of `crates/effects/fx_eval/src/hostcurve.rs` describes (`host`, `value` and `point` lines). For this program the file holds the `host` lines that the stage's own constant file carries (`constant_curve`, `crates/neutral-handoff/src/fxstage.rs` line 698) and one `value wipe_amt 6 TICK 1 24 0 RAW` line per frame, with TICK the frame index from 0 and RAW the Wipe Percent as a fraction times 65536 (32768 for 50 percent). A file with the ticks 0, 1 and 2 at 32768 renders a 3 frame clip (MEASURED: exit 0). A file without the tick of a frame ends at that frame with exit 3, after the earlier frames are written, and the runner's refusal names the sample it needs: `HOST-CURVE-VALUE-MISSING (SwishPan: index 6, tick 1, step 1, scale 24)` (MEASURED with tick 0 only). Without the parameter the stage's constant file gives a Wipe Percent of 0 on every frame, and the output equals the input clip (MEASURED on a 3 frame clip with a `To` layer). The repository holds no curve file of this form, and no golden row runs `auto_trans` 0.

Refused inputs, each before any frame is written unless the row says otherwise (MEASURED on this build at 64x32 with a 3 frame input, unless marked):

| input | label or message | exit |
|---|---|---|
| no `--frames` | `FRAMES-REQUIRED` | 3 |
| `--frames 1` | `FRAMES-TOO-SMALL` | 3 |
| `--frames 0` | `expects a positive integer` | 2 |
| `--frames` other than the input length | `the input holds N frames, but --frames says M`, after the frames were written | 4 |
| empty input | `the input holds 0 frames, but --frames says N` | 4 |
| `--layer` with an id other than `To` | `LAYER-ID-UNKNOWN` | 3 |
| the same layer twice | `LAYER-DUPLICATE` | 3 |
| a layer file that cannot be opened | `LAYER-FILE-MISSING` | 3 |
| a layer file that is not a whole number of frames | `LAYER-SIZE-MISMATCH` | 3 |
| a layer of more than one frame that ends before the input | `layer To holds N frames but the input has more`, at the first frame it lacks | 4 |
| a frame narrower than 2 pixels | `swishpan needs a width of 2 or more` | 3 |
| an unknown parameter | `unknown param` | 3 |
| `cset_opacity` outside 1 to 3, a value out of range or not a number | `bad value for` | 3 |
| `--stage @swishpan` in a pipeline, which passes no `--frames` | `FRAMES-REQUIRED`, passed through by the pipeline | 3 |
| a host curve sample that neither the given file nor the identity curve serves | `HOST-CURVE-SAMPLE-UNSERVED` (CODE, [Refusal labels](../cli.md#refusal-labels)) | 3 |
| an `--fps` whose time fields do not fit 32 bits | `HOST-TIME-RANGE` ([Host values](../cli.md#host-values)) | 3 |

## Code map

| part | path | role |
|---|---|---|
| program constructor | `crates/effects/fx_eval/src/programs_native.rs`: `eff_swishpan` (line 2499), `defun_transition_amount` (line 2813), `defun_transition_swap_inputs` (line 2818), `layout_swishpan` (line 2903) | builds the program graph, its two helper programs and its layout row as Rust constructor calls; registered in the constructor lists at lines 5760 and 5842 |
| constant folds | `crates/effects/fx_eval/src/fold.rs`: `slow_in_out` (line 995), the host getter fold (line 381) | the easing curve and the host values `get_percent_time`, `get_total_frames` and `get_aspect_ratio` fold to constants of the compiled plan |
| host curve | `crates/effects/fx_eval/src/hostcurve.rs` | `get_curve_val` of the Wipe Percent through the parameter's tickmap rows, from the `--host-curve-values` file |
| default xy placement | `crates/effects/fx_eval/src/xyplace.rs` | where the default xy writer of the plan is placed; its base phase is `fx_run --xy-base` |
| recorded host facts | `crates/effects/rtdata/src/styles/swishpan.rs`, listed in `crates/effects/rtdata/src/styles/mod.rs` line 54 | the style's registered parameters, tickmap rows, ranges, input indices, input records and normalize scale rows |
| shared class rows | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS`, `IMG_ONLY_ARGS`, `INT_TYPED_ARGS`, `COMBOS`, `COMBOS_EXEC` | class flags and the combination tables of the classes the plan calls |
| `make_mipmap` | `crates/effects/kernels/make_mipmap`; executor `make_mipmap_k` (`crates/effects/fx_exec/src/kernels2.rs` line 114) | the ripmap pyramid of each clip; with the filter argument 1 that this program passes, `alloc` (`crates/effects/kernels/make_mipmap/src/mm2.rs` line 124) sizes it by `level_count` (line 97), ceil(log(size) / ln 2) + 2 levels per axis |
| `warp` | `crates/effects/kernels/warp_render` (`slot53_warp`, over `warp_driver` and `warp_sampler`) and `crates/effects/kernels/warp_log_gradient` (the footprint prefill); executor `warp_k` (`kernels2.rs` line 1277) | samples a pyramid at the moved coordinates, rows split into bands across threads |
| `+`, `-` | `crates/effects/kernels/ops_arith` | the coordinates `xy` plus the offset, and minus the shift |
| `screen_super` | `crates/effects/kernels/screen_super`; `kernels::normalize_class` maps the plan's `screen_super_if_needed` to it | combines the two resampled clips |
| `blur` | `crates/effects/kernels/blur` (with `blur_coef`, `blur_rows` and `blur_rows_y`); executor `blur_k` (`kernels2.rs` line 975) | the directional blur; planes split across threads |
| `postdivide` | `crates/effects/fx_exec/src/postdiv.rs`; executor `postdivide_k` (`kernels2.rs` line 1375) | the host's opacity handling at the Opacity answer 2; no `postdivide` node runs at the answers 1 and 3 (MEASURED) |
| executor tables | `crates/effects/fx_exec/src/kernels.rs` `SUPPORTED` (line 10: `+`, `-`, `screen_super`); `crates/effects/fx_exec/src/kernels2.rs` `WIRED` (line 13: `make_mipmap`, `blur`, `warp`, `postdivide`) | the ops the executor dispatches |
| device entries | `crates/effects/fx_exec/src/device_ops.rs` `DEVICE_OPS` (line 71), `ALLOWLIST` (line 163), `NAN_EXACT` (line 169); `device_ops_warp.rs`, `device_ops_footprint.rs`, `device_ops_ripmap.rs`, `device_ops_pointwise.rs`, `device_ops_neighbourhood.rs` | the device kernel of each op, its self-check corpus and its NaN class |
| OpenCL sources | `crates/effects/fx_device/src/kernels_warp.rs`, `kernels_footprint.rs`, `kernels_ripmap.rs`, `kernels_pointwise.rs`, `kernels_neighbourhood.rs` | the device kernels of the seven ops |
| stage row | `crates/neutral-handoff/data/fx_stages.tsv` line 17 | program, family, kind, layers (`To?`), source name `From`, host options, placement (`--xy-base` per size) and host variants |
| parameter rows | `crates/neutral-handoff/data/fx_params.tsv` lines 175 to 184 | the 10 offered parameters and their `fx_run` routes (`--set`, `--host-param` for `blur_amount`, `--cset` for `cset_opacity`, `--host-curve-values`) |
| stage code | `crates/neutral-handoff/src/fxstage.rs`: `apply_profile` (line 1596), `check_time` (line 1523), `load_layers` (line 1537), `time_values` (line 1505), `run_fx` (line 979), `constant_curve` (line 698), `identity_sample` (line 754) | the transition checks, the layer, the host values, the constant curve file and the runner call |
| computed tables | none | the plan reads no computed table |

The ops of the plan are MEASURED with the device report on PoCL (section Device path below): `+`, `-`, `blur`, `make_mipmap`, `postdivide`, `screen_super` and `warp`. The shared kernels are described once, on these pages: `blur` in [`S_Blur` internals](blur.md), `warp` and `make_mipmap` in [`S_WarpPerspective` internals](warp-perspective.md) and `screen_super` in [`S_Glow` internals](glow.md); the other transition stage is described in [`S_DissolveShake` internals](dissolve-shake.md).

Tests that exercise the effect (the ones that need stored data, a case file or an OpenCL device print `NOT RUN` without it):

| test | covers |
|---|---|
| `crates/effects/fx_exec/tests/transition_second_input.rs` | the second input through the executor: a bound `To` image and a `To` of zero pixels both render, and differ where the transition reads `To` |
| `crates/effects/fx_exec/tests/popup_default.rs` (`swishpan_auto_trans_unbound_is_one_and_bound_zero_is_zero`) | the registered default of an unset `auto_trans` |
| `crates/effects/fx_exec/tests/t_plan_text.rs` | `fx_run --plan-text PATH` on a run of the program (the plan text and node rows, no change to the output), and the refusals of a missing, repeated or non-finite host value |
| `crates/effects/fx_exec/tests/first_set_plan_probe.rs` | the linewise `make_mipmap` rewrite on the programs added after the first set, this one among them |
| `crates/effects/fx_eval/tests/t_slow_in_out.rs` | `slow_in_out` against recorded results (`NOT RUN` without `SLOW_IN_OUT_CASES`) |
| `crates/neutral-handoff/src/fxstage.rs` unit tests (`frames_are_required_by_name_for_a_time_live_stage`, `to_layer_is_loaded_and_refused_by_name`, `absent_optional_layer_is_a_zero_frame`, `to_layer_and_host_values_reach_fx_run`, `layer_lengths`, `run_fx_refuses_an_offered_curve_parameter_and_serves_a_hidden_one`, `an_offered_parameter_is_never_served_by_the_identity_curve`, `every_landed_row_parses_its_variants_and_the_table_checks`, `manifest_default_of_an_unbound_popup_is_the_registered_default`) | the transition inputs, the curve file, the host variants and the listed default |
| `crates/neutral-handoff/tests/stage_cli.rs` | the manifest layers and the `--layer` refusals through the binary |
| `crates/effects/fx_exec/tests/device_warp.rs`, `device_footprint.rs`, `device_ripmap.rs`, `device_pointwise.rs`, `device_neighbourhood.rs`, `device_backend.rs` | the device kernels of its ops (`NOT RUN` without an OpenCL device) |

## Render flow

1. `sapphire stage apply swishpan --width W --height H --depth D --frames N [--fps R] [--layer To=PATH] [--param ID=V]...` checks, in this order: `--frames` (`FRAMES-REQUIRED`, `FRAMES-TOO-SMALL`), the `host_variants` column, a width of 2 or more, the keyed values and the rate, then the layer file (`load_layers`). A refusal writes no frame.
2. For each input frame f, the frame is handed to the runner in memory as 'ae32' words under the source name `From`, and the layer frame f (or the held frame, or a frame of zero words when no layer is given) under `To`. Depths 8 and 16 convert through f32 and are not verified.
3. `time_values` gives the host values `get_percent_time` f32(f) / f32(N - 1), `get_total_frames` f32(N) and `get_aspect_ratio` f32(H) / f32(W), and with `--fps` also `get_time` and `get_frames_per_second` ([Host values](../cli.md#host-values)).
4. `run_fx` builds the runner's command line: the program and size, the `host` column of the stage row (or, at `cset_opacity` 1 or 3, the options of its `host_variants` entry in their place), the placement for the frame size (`--xy-base 0` at both measured sizes, the 64x32 entry elsewhere), the parameters by their routes, the host values, `--ns-default-unbound`, and a constant curve file with a Wipe Percent of 0 when `host_curve_values` is not given (a given file is passed as it is). `--do-postdiv` starts at 0 and takes the value the runner derives when it refuses with `POSTDIV-MISMATCH`.
5. The runner (`fx_exec`, the same code as `fx_run SwishPan`) builds the program from `eff_swishpan`, compiles it with the facts of `effects_rtdata` and folds every constant: the host values, the eased progress, the shift, the offset and the blur width are constants of the plan. A frame whose plan does not read one of the two clips is rendered again without that clip, as for every transition stage.
6. The plan's nodes in order: one `make_mipmap` per clip; the coordinates `xy` plus the offset (`+`) for the first clip and minus the shift (`-`) for the second; one `warp` per clip; `screen_super` of the two; `blur` along the move; `postdivide`. Nodes whose inputs fold to constants are not run: on frame 0 of an `auto_trans` 1 run the offset is 0, at the first and last frames the eased progress has zero speed, and with `auto_trans` 0 and the constant curve file the blur width is 0 (INFERRED from the node counts MEASURED in Device path).
7. `warp` splits its rows into bands and `blur` its planes across threads above `SAPPHIRE_PAR_MIN` samples (`SAPPHIRE_THREADS` workers); the output bytes do not depend on either (MEASURED below, and by the threaded golden rows).
8. With `--device` or `SAPPHIRE_DEVICE`, each node runs on the device when its op is admitted and its kernel accepts the call; any other node, and a `warp` output that holds a NaN, runs on the CPU with the same bytes.
9. The output words are relaid to the frames/2 planes and written; with `--frames` different from the input length the run ends with exit 4 after the frames.

From `fx_run`: `fx_run SwishPan --size WxH --out PREFIX` with the options above renders one frame; the host context options are required and have no default ([Build and run](../effects.md#build-and-run)). The stage fills them from its row, so the stage is the supported entry point.

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means) on the final binary, judged against the original running in an emulator. For `S_SwishPan`, `docs/data/effects_status.tsv` holds 36 verified comparisons (18 at 64x32 and 18 at 50x7), 288 recorded renders and 288 re-rendered units, all equal; none of the 38 comparisons that differ from the original is of this effect. Each verified render is one frame of a transition of 100 to 480 frames at 24 frames a second, at frames 0 to 120, with the second clip absent or present; the 288 recorded renders are the 36 renders at the eight base phases 0, 4, ..., 28 of the default xy buffer. The README table "Effects and stages" gives 32 of 36 counted recorded renders byte identical through the stage; the other four are 50x7 renders recorded at base phase 16, verified through `fx_run` at that phase, while the stage runs phase 0 ([Effects provider](../stages.md#effects-provider)). The counted renders with `auto_trans` 0 run through the stage with a curve file that holds their recorded Wipe Percent.

The four counted renders outside the stage figure differ from the stage's host arrangement only in the base phase of the default xy buffer: they were recorded at phase 16, while the `placement` column gives `--xy-base 0` at 50x7 (`crates/neutral-handoff/data/fx_stages.tsv` line 17). A host variant entry cannot serve them, because an entry may not carry an option of the `placement` column (`HOST-VARIANT-PLACEMENT`, [Effects provider](../stages.md#effects-provider)). Of the 288 recorded renders, the 36 at the phase each original run used are counted; the other 252 are the same renders at the seven other phases, recorded and not counted (DERIVED from the eight phases above). The repository names no other control render of this effect.

## Checks that run from this repository

| check | command | expected result | exit |
|---|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| stage listed | `target/release/sapphire stage list --json` | the `swishpan` entry with `"kind": "temporal"`, `"spool": false`, `"layers": [{"id": "To", "optional": true}]` and 10 parameters | 0 |
| parameters | `target/release/fx_run --describe-params SwishPan` | JSON with the 10 registered parameters of the program | 0 |
| worked render | the `sh` block below | 98,304 bytes, 3 frames of 64x32 | 0 |
| refusals | the `sh` block below | the labels of Styles and stages | 3, 3, 4 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 11 rows of `swishpan` | 0 |
| conformance | `python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply swishpan --frames 12"` | L1 `4 passed, 7 failed`, L3 `32 passed, 1 failed`, the documented `FAIL` lines | 1 |
| device report | the device block below | the report line of Device path; output equal to the CPU | 0 |
| device golden rows | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` | 0 |
| crate and executor tests | `cargo test --release --offline -p fx_eval -p fx_exec` | tests pass; tests that need stored data print `NOT RUN` | 0 |
| stage tests | `cargo test --release --offline -p neutral-handoff` | tests pass; tests that need ffmpeg print `SKIPPED` without it | 0 |
| device tests | `SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_warp --test device_footprint --test device_ripmap --test device_pointwise --test device_neighbourhood --test device_backend -- --test-threads=1` | tests pass on an OpenCL device | 0 |

Build, then make the inputs (ffmpeg on `PATH`; scratch files under the git ignored `target/demo/`):

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
mkdir -p target/demo
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/in.raw
ffmpeg -v error -y -f lavfi -i testsrc2=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/to.raw
target/release/sapphire stage list --json | python -I -c "import json,sys; s=[x for x in json.load(sys.stdin)['stages'] if x['id']=='swishpan'][0]; print(s['kind'], s['spool'], s['layers'], len(s['params']))"
target/release/fx_run --describe-params SwishPan
```

Each command ends with exit 0; the python line prints `temporal False [{'id': 'To', 'optional': True}] 10`. Without ffmpeg the two input files are not made and the later commands that read them fail.

The worked render of the `S_SwishPan` section of [Effects](../effects.md), with the second clip:

```sh
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer To=target/demo/to.raw --param transition_dir=0 --param auto_trans=1 --param direction=2 --param blur_amount=300.0 --param overlap=0.1999969482421875 --param slow_in=0.3000030517578125 --param slow_out=0.8000030517578125 --param swish_auto_trans=0.0 < target/demo/in.raw > target/demo/swish.raw
```

Exit 0 and 98,304 bytes (3 frames of 64x32 at 16 bytes a pixel); standard error holds two `note:` lines, that `blur_amount` and `swish_auto_trans` are put on the 16.16 host grid (as 300 and 0, the same values).

Three refusals:

```sh
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24 < target/demo/in.raw > target/demo/r1.raw
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24 --frames 1 < target/demo/in.raw > target/demo/r2.raw
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24 --frames 5 < target/demo/in.raw > target/demo/r3.raw
```

The first exits 3 with `FRAMES-REQUIRED` and the second 3 with `FRAMES-TOO-SMALL`, each with no output; the third writes the 3 frames and exits 4 with `the input holds 3 frames, but --frames says 5`.

The golden rows and the grep that lists this effect's rows:

```sh
python -I tools/fx_golden.py
grep -E '^swishpan\s' tools/fx_golden.tsv | cut -f1,2
```

`fx_golden.py` ends with `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0 (it renders every stage; a difference is a `DIFFERS` line and exit 1). The grep prints 11 rows: the plain sizes `64x32` and `50x7`, the special row `40x12s`, the threaded rows `64x32t` and `50x7t` and the variant rows `+o1`, `+o3` (the `cset_opacity` answers 1 and 3) and `+to` (with a `To` layer) at both plain sizes. Every golden case runs with `--fps 24 --frames 3`.

What the golden rows reach (MEASURED by a device report of each of the 129 golden cases other than the threaded rows, on PoCL on the check build): the plain and `+to` rows `device: + 2, - 3, blur 1, make_mipmap 6, postdivide 3, screen_super 3, warp 5; fallback: none`, the `+o1` and `+o3` rows the same nodes without `postdivide`, and the special row the same nodes with `fallback: warp 2 (nan_output 2)`. Every op of the plan is reached. No golden row runs `auto_trans` 0 (the curve route), `direction` 1 to 3, `transition_dir` 1 or a set `overlap`, `slow_in` or `slow_out` (`VARIANTS` in `tools/fx_golden.py`); those values change only the constants of the plan, so a change confined to their branch of the constant folds moves no golden row of this stage (INFERRED).

Conformance on the stage (the command must name the binary by absolute path):

```sh
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply swishpan --frames 12"
```

Exit 1, with L1 `4 passed, 7 failed` and L3 `32 passed, 1 failed`: the `FAIL` lines are the two 1x1 cases (exit 3), the L1 cases of 1 or 2 frames (exit 4, `the input holds N frames, but --frames says 12`) and the L3 empty input (exit 4). These are the documented refusals of [Conformance](../stages.md#conformance).

The device path (an OpenCL platform such as PoCL; without one `SAPPHIRE_REQUIRE_DEVICE=1` makes the first command exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24 --frames 3 --device gpu < target/demo/in.raw > target/demo/swish_dev.raw
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24 --frames 3 < target/demo/in.raw > target/demo/swish_cpu.raw
cmp target/demo/swish_dev.raw target/demo/swish_cpu.raw
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

Each ends with exit 0 and `cmp` prints nothing. The last `device: ` line of the first command is `device: + 2, - 3, blur 1, make_mipmap 6, postdivide 3, screen_super 3, warp 5; fallback: none`. The first device run of a build pays the self-check of every allowlisted op ([Rendering on a device](../guide.md#rendering-on-a-device)).

The cargo tests (the device ones on an OpenCL device):

```sh
cargo test --release --offline -p fx_eval -p fx_exec
cargo test --release --offline -p neutral-handoff
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_warp --test device_footprint --test device_ripmap --test device_pointwise --test device_neighbourhood --test device_backend -- --test-threads=1
```

Each ends with exit 0. Without stored data the tests that read it print `NOT RUN` and pass; without an OpenCL device and without `SAPPHIRE_REQUIRE_DEVICE=1` the device tests print `NOT RUN` and pass, and with the variable set they fail instead.

In CI (`.github/workflows/ci.yml`), `cargo test --release` (the default members, `neutral-handoff` among them), `python -I tools/fx_golden.py` on Linux, macOS and Windows, `cargo test --release -p fx_eval -p fx_exec ...` on Linux, and in the job `device backend on PoCL` the self-check tests (`cargo test --release -p fx_exec --lib device`), the device test files (among them `device_backend`, `device_pointwise`, `device_warp`, `device_ripmap`, `device_neighbourhood` and `device_footprint`, each with `--test-threads=1`) and `fx_golden.py` with `SAPPHIRE_DEVICE=gpu` run these checks.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders and the recorded runs of the original; the repository holds the native code, the stage rows taken from those runs and this repository's own golden hashes. The 36 verified comparisons, the 288 recorded renders and the 32 renders checked through the stage cannot be rerun from the repository alone. `crates/effects/fx_eval/tests/t_slow_in_out.rs` prints `NOT RUN` without its case file, and the tests that read stored data under `VEC_DIR` print `NOT RUN` without it ([Tests](../effects.md#tests)). The full report is kept outside the repository; its SHA-256 is printed in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, 64x32 and 50x7, one frame each of transitions of 100 to 480 frames at 24 frames a second, frames 0 to 120, the Opacity answers 1, 2 and 3, the second clip absent or present, `auto_trans` 0 with recorded Wipe Percent curves of 35 and 50 percent and `auto_trans` 1. Other sizes, lengths, rates and values run the same code and are INFERRED; depths 8 and 16 are not verified.
- The time values describe a transition that spans exactly the clip the stage is given; a clip cut from a longer transition, or a second clip that starts at another time, is not modelled ([Host values](../cli.md#host-values)).
- Host buffer phase: the default xy writer (rows of 29 pixels or more) and the blur windows (a window of 64 samples or more, which needs a frame 128 pixels or more wide or high) take their order from the phase ([Limits common to the effects](../effects.md#limits-common-to-the-effects)). The stage passes `--xy-base 0` at every size; above the thresholds the output equals the original's when the host's phase equals that value (INFERRED). Four counted 50x7 renders depend on the phase and are verified at phase 16 through `fx_run` only.
- At 1920x1080 a render passes both thresholds: the default xy writer runs on rows of 1,920 pixels at the stage's phase 0, and a blur window of 64 samples or more takes the blur scratch phase, 0 by default (`base_phase`, assumption B-A4 in `crates/effects/kernels/blur/src/lib.rs` line 240). Such a render equals the original's when the host gives those phases (INFERRED); no render at that size was compared.
- One approximate reciprocal instruction runs on the verified render path, inside the original's logarithm that sizes the mipmap levels: `make_mipmap` with the filter argument 1 computes ceil(log(size) / ln 2) + 2 levels per axis (`level_count`, `crates/effects/kernels/make_mipmap/src/mm2.rs` line 97). The native level count uses exact operations only (`log_over_ln2`, line 80) and equals the former platform-library route for every size from -2 to 8192 and for sampled sizes up to 2^24 (`level_count_matches_the_libm_route_for_every_size`, line 320); only the integer level count reaches the output, and the docs give its effect as 0 ulp (MEASURED on that logarithm; that the route enters it, and that the reciprocal estimates of the blur are not reached, is INFERRED). The repository holds no table of approximate-instruction sites per effect beyond the parity paragraphs of [Effects](../effects.md); what the emulator returns for the instruction is recorded outside the repository.
- NaN: the CPU kernels keep the x86 NaN rules; NaN payload and sign bits are not verified against the original. The special golden row `40x12s` fixes the NaN words of this repository's own render on each CI platform.
- `ROUTE AVX2-ONLY`: the tier of the original that runs on CPUs with AVX2 and FMA is reproduced; other tiers are not.
- Host curves other than the identity curve and the recorded Wipe Percent curves are INFERRED.
- An unset popup compiles its registered default, f32(max(v - 1, 0)) for v the style's registered checkout integer (`fx_eval::units::registered_popup_default`, `crates/effects/fx_eval/src/units.rs` line 229); that is the answer a host's unset popup gives (INFERRED) and the default `stage list` prints (`popup_manifest_default`, `crates/effects/fx_exec/src/fxrun.rs` line 289). `fx_run --describe-params SwishPan` prints the default of the program's own parameter declaration (`describe_params_json`, line 295), which for `auto_trans` is the other item; of the offered parameters only `auto_trans` differs (MEASURED: an unset `auto_trans` gives the bytes of a run at the value `stage list` prints; [Parameter discovery](../cli.md#parameter-discovery)).
- `blur_amount` is a host-unit value (`--host-param`), converted with the frame width; the `default` printed for it is the record value, not a host value.

## Device path

| op | device kernel | NaN class | on NaN output |
|---|---|---|---|
| `+` | `device_ops_pointwise::ADD` | `NAN_EXACT_COMPILED` | stays on the device |
| `-` | `device_ops_pointwise::SUB` | `NAN_EXACT_WRITTEN` | stays on the device |
| `blur` | `device_ops_neighbourhood::BLUR` | `NAN_EXACT_WRITTEN` | stays on the device |
| `make_mipmap` | `device_ops_ripmap`, pyramid left on the device | `NAN_EXACT_COMPILED` | stays on the device |
| `postdivide` | `device_ops_pointwise::POSTDIVIDE` | `NAN_EXACT_WRITTEN` | stays on the device |
| `screen_super` | `device_ops_pointwise::SCREEN_SUPER` | `NAN_EXACT_COMPILED` | stays on the device |
| `warp` | `device_ops_warp`, with the footprint prefill of `device_ops_footprint` | not NaN exact | rerun on the CPU (`nan_output`) |

No op of the plan has a `CPU_ONLY` entry (the table is empty). A node runs on the CPU when its op fails the self-check on the device in use (`not_allowlisted`), when its kernel declines the call (`unsupported`), when the device budget would be exceeded (`memory`) or on a driver error (`opencl_error`); the bytes are the CPU's in every case. MEASURED on PoCL on this build, 64x32, 3 frames of `testsrc`: the default run reports `device: + 2, - 3, blur 1, make_mipmap 6, postdivide 3, screen_super 3, warp 5; fallback: none`; with the worked parameters and the `To` layer the same counts; with `cset_opacity=1` or `cset_opacity=3` the same nodes without `postdivide`; with `auto_trans=0` and the constant curve file `device: - 3, make_mipmap 6, postdivide 3, screen_super 3, warp 3; fallback: none`; each output equals the CPU output. The section of `S_SwishPan` in [Effects](../effects.md) records `fallback: none` on the plain, threaded and variant golden rows and on the 36 counted recorded renders through `fx_run`, and 2 `warp` nodes rerun for NaN outputs on the special golden row (MEASURED on PoCL). Speed in [Speed](../guide.md#speed): one 1920x1080 frame (a 3 frame clip divided by 3) takes 2.32 and 2.55 s on the CPU and 1.16 and 0.90 s on PoCL, which is correctness on a CPU runtime, not GPU speed.

## Changing the code safely

- Gates that stay green: `python -I tools/fx_golden.py` (the 11 `swishpan` rows in every group, and the rows of every other stage that shares an op), the device run of the same table, `cargo test --release -p fx_eval -p fx_exec`, `cargo test --release` (default members), the device tests with `SAPPHIRE_REQUIRE_DEVICE=1` and `--test-threads=1`, `every_op_has_a_device_kernel_or_a_cpu_only_entry` in `device_ops.rs`, `python -I tools/test_effect_rules.py`, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes of the stage changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees; that takes the comparison harness. `python -I tools/fx_golden.py --write` renders every case and rewrites all of `tools/fx_golden.tsv` with whatever the current build gives, so it is a record of the build, not a check.
- The shared ops (`warp`, `blur`, `make_mipmap`, `screen_super`, `+`, `-`, `postdivide`) are used by other effects: a change to one of them moves those effects' golden rows as well.
- A CPU parity change to an op removes its `ALLOWLIST` entry unless its device kernel changes in the same patch; a kernel keeps the CPU's operation order, uses `fma()` only where the CPU calls `mul_add`, and declines what it does not reproduce ([Device](../adding-an-effect.md#device)).
- No platform math library call in effect code (`fx_eval::detmath` or exact integer code instead); a fresh NaN is canonicalized to `0xffc00000` before a sign operation, and a two-NaN pick goes through `fx_common::nanop`; `test_effect_rules.py` scans for the calls ([Crate layout](../adding-an-effect.md#crate-layout)). The special golden row and the arm64 macOS runner catch a NaN word that differs between processors.
- A change to the stage row or the parameter rows changes the host arrangement on which the 32 matching recorded renders were checked through the stage. No test of the repository rechecks those renders (they need the comparison harness); `fx_golden.py` checks only this repository's own renders.
- Docs and data to update with a change: the `S_SwishPan` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), the tables of [Adding a provider](../stages.md#effects-provider), [Transition stages](../cli.md#transition-stages) when the inputs change, `docs/data/effects_status.tsv` when a count changes, and this page; then `python -I tools/gen_docs.py`.

## Open items

- The stage runs the default xy buffer at base phase 0 at every size, so four counted 50x7 renders recorded at phase 16 are verified through `fx_run` only (`crates/neutral-handoff/data/fx_stages.tsv` line 17, placement column; the `S_SwishPan` section of [Effects](../effects.md)).
- `warp` has no NaN-exact device kernel: its NaN outputs rerun on the CPU ([Rendering on a device](../guide.md#rendering-on-a-device); `NAN_EXACT` in `crates/effects/fx_exec/src/device_ops.rs` line 169).
- The pipeline passes no `--frames`, so `--stage @swishpan` is refused with `FRAMES-REQUIRED` and the effect runs through `sapphire stage apply` only, with ffmpeg decoding and encoding around it (`check_time` in `crates/neutral-handoff/src/fxstage.rs` line 1523). The repository records no change that would pass the probed frame count to a transition stage.
- A given curve file that lacks the sample of a frame ends with the runner's refusal `TICKMAP-CURVE-DIRECT-READ-UNSOURCED` (`crates/effects/fx_exec/src/fxrun.rs` line 1196) (exit 3 through the stage, [Refusal labels](../cli.md#refusal-labels)); `HOST-CURVE-SAMPLE-UNSERVED` of the same table covers the stage's own constant file. A curve file is served only at the ticks it holds.
- No curve file for `auto_trans` 0 is in the repository and no golden row runs that route (`VARIANTS` in `tools/fx_golden.py`).
- The phase of the default xy buffer that a real host's allocator gives is not recorded; the stage's phase 0 is its own choice (`crates/neutral-handoff/data/fx_stages.tsv` line 17, `placement` column).
- A transition cut from a longer one, or a second clip that starts at another time, is not modelled ([Host values](../cli.md#host-values)).
- The two parameter listings differ for `auto_trans` ([Parameter discovery](../cli.md#parameter-discovery)).
- `ROUTE AVX2-ONLY` and the unverified host settings of [Limits common to the effects](../effects.md#limits-common-to-the-effects).
