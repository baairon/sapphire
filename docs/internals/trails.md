# Internals of `stage apply trails_autogen`

This page covers the code of this stage: where each part lives, how a render runs, how its parity is judged, the checks that run from this repository, the device path and the limits. It is the stage of `S_Trails`, the style Trails_Autogen. The user reference is the `S_Trails` section of [Effects](../effects.md); its parameters are described in [Driving the effect parameters](../parameters-guide.md) and its stage in the section "Effects provider" of [Adding a provider](../stages.md#effects-provider). The parts every effect shares are described once in [Effect internals](README.md).

## What it computes

A feedback blend over the clip. Each render multiplies the current frame by `new_opacity` times `new_color`, takes the previous render's output (the previous destination), blurs it by `blur_amount` times `blur_rel` per axis, turns its hue by `prev_hue_shift` and multiplies it by `fade` times `fade_color`, then combines the two by `combine`: the current frame weighted by one minus `fade` added to the trail (0), the maximum (1), `screen_super` (2), the sum (3), `comp_rgba` in one operand order (4) or the other (5), or the minimum (6). The result is dissolved with the source frame by the effect's mask, which on the stage is the tracker mask group alone (the `Mask` input is not connected). The first render has no previous destination and starts from the weighted current frame (at `combine` 0 and 6) or from zero (at 1 to 5) (INFERRED from the constructors `eff_trails` and `eff_trails_autogen` and their helpers).

In formulas (INFERRED from `eff_trails`, `eff_trails_autogen`, `defun_feedback_combine` and `defun_feedback_init_prev`), with F the current frame, P the previous destination and M the mask:

- s = F x (`new_opacity` x `new_color`);
- the trail T = hue_shift(blur(P, `blur_amount` x `blur_rel`), `prev_hue_shift`) x (`fade` x `fade_color`);
- the combination C by `combine`: T + s x max(0, 1 - `fade`) (0), max(T, s) (1), screen_super(T, s) (2), T + s (3), comp_rgba(T, s) (4), comp_rgba(s, T) (5), min(T, s) (6);
- the output: dissolve(F, C, M), the mix of F and C by M, or the tracker mask alone when `show_mocha_mask_only` is set;
- on a restarted render P is s at `combine` 0 and 6, and zero at 1 to 5.

The arithmetic of each op is in its kernel crate (Code map), for example the box passes of `blur` in `crates/effects/kernels/blur` and the mix in `crates/effects/kernels/dissolve`.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Trails_Autogen | `Trails_Autogen`, wrapping the program of `eff_trails` | `stage apply trails_autogen` | `temporal` | none | true | none (`-`) |

`spool: true` means the stage reads the whole input before it writes the first frame, and renders the clip in one run of the history route ([Stage kinds, spool and keyframes](../cli.md#stage-kinds-spool-and-keyframes)). Output frame N depends on input frames 0 to N: the first two frames of a 3 frame render equal the render of the first two input frames (MEASURED). The program reads no host time value, so `--fps` changes nothing: renders without `--fps`, with `--fps 24` and with `--fps 30` give the same bytes (MEASURED). Only the Opacity answer 2 renders.

Refused inputs (MEASURED on this build at 64x32 with a 3 frame input, unless marked):

| input | label or message | exit |
|---|---|---|
| a keyed value (`V@F;V@F`) | `bad value for`, `renders the whole clip in one fx_run run` | 3 |
| `cset_opacity=1` or `cset_opacity=3` | `SOURCE-VARIANT-MISMATCH` in the standard error tail of the runner; the message reads `render 1 failed with rc 2` | 3 |
| `--layer` | `LAYER-INPUT-UNSUPPORTED`, before any frame is read | 3 |
| a frame narrower than 2 pixels | `trails_autogen needs a width of 2 or more` | 3 |
| an unknown parameter | `unknown param` | 3 |
| `cset_opacity` outside 1 to 3, a value out of range or not a number | `bad value for` | 3 |
| an input that ends in part of a frame | `partial_frame` | 4 |
| `--frames` other than the input length | `the input holds N frames, but --frames says M`, after the frames were written | 4 |
| empty input | no refusal: no frame is written | 0 |

A host-unit value off the 1/65536 grid, such as `blur_amount=4.00001`, is put on the 16.16 host grid by the stage, with a `note:` line on standard error, and renders (MEASURED: exit 0), as the row `HOST-VALUE-OFF-GRID` of [Refusal labels](../cli.md#refusal-labels) states; `fx_run` refuses the same value with that label.

## Code map

| part | path | role |
|---|---|---|
| program constructors | `crates/effects/fx_eval/src/programs_native.rs`: `eff_trails` (line 1272), `eff_trails_autogen` (line 1303), `defun_feedback_combine` (line 1818), `defun_feedback_init_prev` (line 1860), `defun_process_masks` (line 1674) with the mask helpers above it, `layout_trails_autogen` (line 2262) | the wrapper program, the program it calls, the two feedback helpers, the mask processing and the layout row; registered in the constructor lists at lines 5752, 5753 and 5838 |
| recorded host facts | `crates/effects/rtdata/src/styles/trails.rs`, listed in `crates/effects/rtdata/src/styles/mod.rs` line 50 | the style's registered parameters, tickmap rows, ranges, input indices, input records and normalize scale rows |
| shared class rows | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS`, `IMG_ONLY_ARGS`, `INT_TYPED_ARGS`, `COMBOS`, `COMBOS_EXEC` | class flags and the combination tables of the classes the plan calls |
| render history | `crates/effects/fx_exec/src/history.rs`: `FrameRecord` (line 26), `decide` (line 56), `begin` (line 81), `save` (line 98), `window_limit` (line 46) | the rule that keeps or restarts the history at each render, and the save at its end |
| history driver | `crates/effects/fx_exec/src/fxrun.rs`: the `--host-times` switch (line 396), `run_history` (line 3313), `history_required` (line 3443) | runs the renders in order, hands each kept render the previous destination, writes `PREFIX.history.tsv` |
| previous destination | `crates/effects/fx_exec/src/kernels.rs` `HOST_CLASSES` (line 17); `crates/effects/fx_exec/src/exec.rs` line 685 | `get_prev_dest` and `save_prev_dest`, served by the executor from the host frames |
| opacity answer | `crates/effects/fx_exec/src/opacity_conv.rs` | the Opacity answer to the program's opacity value, with unit tests on this style |
| `*`, `+` | `crates/effects/kernels/ops_arith` | the colour and fade products and the sum |
| `max`, `min` | `crates/effects/kernels/ops_minmax` | `combine` 1 and 6 |
| `screen_super` | `crates/effects/kernels/screen_super` | `combine` 2 |
| `comp_rgba` | `crates/effects/kernels/comp_rgba` (with `plane_service`) | `combine` 4 and 5 |
| `blur` | `crates/effects/kernels/blur` (with `blur_coef`, `blur_rows` and `blur_rows_y`); executor `blur_k` (`crates/effects/fx_exec/src/kernels2.rs` line 975) | the trail blur, when `blur_amount` is above 0; planes split across threads |
| `hue_shift` | `crates/effects/kernels/hue_shift` | the trail hue turn, when `prev_hue_shift` is not 0 |
| `dissolve` | `crates/effects/kernels/dissolve` | the mask dissolve, when the tracker mask is not constant 1 (for example `mocha_mask_opacity` 0.5) |
| `postdivide` | `crates/effects/fx_exec/src/postdiv.rs`; executor `postdivide_k` (`kernels2.rs` line 1375) | the host's opacity handling |
| executor tables | `crates/effects/fx_exec/src/kernels.rs` `SUPPORTED` (line 10); `crates/effects/fx_exec/src/kernels2.rs` `WIRED` (line 13) | the ops the executor dispatches |
| device entries | `crates/effects/fx_exec/src/device_ops.rs` `DEVICE_OPS` (line 71), `ALLOWLIST` (line 163), `NAN_EXACT` (line 169); `device_ops_pointwise.rs`, `device_ops_neighbourhood.rs` | the device kernel of each op, its self-check corpus and its NaN class |
| OpenCL sources | `crates/effects/fx_device/src/kernels_pointwise.rs`, `kernels_neighbourhood.rs` | the device kernels of the ops above |
| stage row | `crates/neutral-handoff/data/fx_stages.tsv` line 13 | program, family, kind, `spool` true, source name, the `fixed` column (the mask parameters of the unconnected `Mask` input, two parameters of the tracker mask group, and the history inputs the driver requires), the `host` column and the placement (empty at both measured sizes) |
| parameter rows | `crates/neutral-handoff/data/fx_params.tsv` lines 122 to 132 | the 11 offered parameters and their `fx_run` routes (`--host-param` for `blur_amount`, `--cset` for `cset_opacity`, `--set` for the rest) |
| stage code | `crates/neutral-handoff/src/fxstage.rs`: `apply_profile` (line 1596, the `spool` branch at line 1694), `run_fx` (line 979), `Scratch` (line 562) | buffers the clip, runs the history route once in a scratch folder under the system temporary folder, removed on every exit path |
| comparison tool | `crates/effects/fx_exec/src/bin/trails_cmp.rs` | compares the executor with the stored input, previous destination and output planes of an original run, read from a programs table and a planes file named on its command line (not through `VEC_DIR`; usage in its header); `cargo build --release -p fx_exec` builds it with the other binaries of the package, and no test or CI step runs it (INFERRED from the tests and `.github/workflows/ci.yml`) |
| computed tables | none | the plan reads no computed table |

The ops of the plan are MEASURED with the device report on PoCL (section Device path below): at the default parameters `*`, `+` and `postdivide`; the other ops appear with the parameters named in the table. The shared kernels are described once, on these pages: `blur` in [`S_Blur` internals](blur.md), `screen_super` in [`S_Glow` internals](glow.md) and `hue_shift` in [`S_HueSatBright` internals](hue-sat-bright.md).

Tests that exercise the effect (the ones that need stored data print `NOT RUN <test>: needs the stored effect data ...` without `VEC_DIR`):

| test | covers |
|---|---|
| `crates/effects/fx_exec/src/history.rs` unit tests (`first_call_restarts_and_stores_size`, `keep_at_advance_inside_window`, `equal_time_restarts`, `time_back_restarts`, `beyond_window_restarts`, `size_change_restarts_then_stores_new_size`, `grow_and_depth_change_reallocate`, `free_path_when_allocator_cell_is_zero`, `flag_clear_never_keeps_and_leaves_record`, `fused_limit_discriminates`, `flag_byte_after_every_save`, `limit_is_fused_and_k_is_double_19`) | the keep and restart rule and the save |
| `crates/effects/fx_exec/src/fxrun.rs` (`history_inputs_required`) | the refusals of the history driver |
| `crates/effects/fx_exec/src/opacity_conv.rs` unit tests | the Opacity conversion on this style |
| `crates/effects/fx_exec/tests/t2.rs` (`trails_default_blends_prev_dest_and_source_and_saves_the_result`), `t12.rs`, `t10.rs`, `t1.rs`, `t9.rs` | the program on the executor against stored plans and planes (`VEC_DIR`) |
| `crates/effects/fx_eval/tests/t7.rs`, `t8.rs`, `t28_lym3.rs`, `t31_lym1.rs`, `t_int28_pi.rs`, `t_int32_df.rs` | the compile of the style: coerce folds, registered parameters, input order and indices, unbound defaults |
| `crates/neutral-handoff/tests/fx_stage_parity.rs` | the stage against the recorded renders (`NOT RUN` without `VEC_DIR` and an `fx_run`) |
| `crates/neutral-handoff/tests/frames2.rs`, `crates/neutral-handoff/src/fxstage.rs` (`tables_parse_for_twenty_seven_stages`) | the manifest's `spool` flag and the stage row |
| `crates/effects/fx_exec/tests/device_pointwise.rs`, `device_neighbourhood.rs`, `device_backend.rs` | the device kernels of its ops (`NOT RUN` without an OpenCL device) |

## Render flow

1. `sapphire stage apply trails_autogen --width W --height H --depth D [--param ID=V]...` refuses a `--layer`, checks the width and the parameter values, and refuses a keyed value: the clip is rendered in one run, so its parameters are constant over the clip.
2. The stage reads every input frame into memory as 'ae32' words (the `spool` branch of `apply_profile`). Memory grows with the clip: about 33 bytes a pixel for every frame (MEASURED in [Stage kinds, spool and keyframes](../cli.md#stage-kinds-spool-and-keyframes)). An empty input renders nothing and exits 0.
3. `run_fx` creates a scratch folder, holds the frames in memory as `src1.u32` to `srcN.u32` and builds one runner command line: the program `Trails_Autogen`, `--host-times 1,2,...,N`, `--host-frame %R` and `--ctx-time %R` (the render number), the `host` column, the empty placement, the `fixed` column (the mask parameters of the unconnected `Mask` input, two parameters of the tracker mask group, and the host step, program flag, depth, allocator cell and host id the history driver requires) and the parameters. No host value (`get_time` and the others) is passed.
4. The runner's history driver (`run_history`) refuses a missing history input by name: `HISTORY-HOST-DATA-MISSING` (no host step), `HISTORY-FLAG-UNRECORDED` (no program flag), `HISTORY-HOST-CELL-UNRECORDED` (no depth, allocator cell or host id), and the host id must be 1. The stage always passes them. Their meaning (the comment above `run_history`, `crates/effects/fx_exec/src/fxrun.rs` lines 3306 to 3312): `--host-step` is the host's local time step, the program flag option the program's flag byte (its bit value 4 turns the history on), `--host-alloc` the allocator cell (nonzero keeps a buffer, 0 is the free path), `--host-depth` the allocated depth and `--host-id` the host id, for which the verified render path is ported. The `fixed` column holds the values of the recorded runs; the values a real host passes are not recorded.
5. For render k (clip frame k - 1 at time k), `FrameRecord::begin` decides KEEP or RESTART: a render keeps the history when the program flag carries the history bit, the previous render's time is below the current time, the current time is below the window limit (`window_limit`, one fused multiply-add of the host step and the previous time) and the frame size is unchanged; otherwise it restarts. The first render restarts; with the consecutive times and the host step the stage gives, every later render keeps (INFERRED from the rule and the unit tests of `history.rs`). Times below 2^24 are exact in f32, so up to there every render after the first keeps the history; at time 16,777,216 (2^24) the f32 rounding of the limit equals the time and the render restarts (INFERRED by evaluating `window_limit` and `decide` with the stage's host step of 1). Whether the original restarts there with a real host's times is not recorded.
6. The runner compiles `Trails_Autogen` with the facts of `effects_rtdata` and folds every constant. A kept render reads the previous render's destination (`get_prev_dest`, from the `PREFIX.r(k-1).prev.f32` file in the scratch folder); a restarted render reads the starting value of `defun_feedback_init_prev`. The plan then runs the products (`*`), the blur and hue turn of the trail when their parameters are set, the combine op, the mask dissolve when the mask is not constant, and `postdivide`; its destination is saved as `PREFIX.rk.prev.f32` (`save_prev_dest`), and the record is saved (`FrameRecord::save`).
7. `blur` runs its planes on separate threads above `SAPPHIRE_PAR_MIN` samples (`SAPPHIRE_THREADS` workers); the output bytes do not depend on either (MEASURED with `blur_amount=2` at `SAPPHIRE_THREADS=1` and at `SAPPHIRE_THREADS=3 SAPPHIRE_PAR_MIN=0`).
8. With `--device` or `SAPPHIRE_DEVICE`, each node runs on the device when its op is admitted and its kernel accepts the call; any other node runs on the CPU with the same bytes.
9. The stage relays the N outputs to the frames/2 planes and writes them in order; the scratch folder is removed on every exit path.

From `fx_run`: `fx_run Trails_Autogen --host-times 1,2,3 ...` runs the same driver with the history inputs of the `fixed` column; outputs go to `PREFIX.rK.*`, the decision of each render to `PREFIX.history.tsv` and the route's labels to `PREFIX.history.labels.txt`. Any `--in`, `--frame` or `--prev` value may contain `%R`, replaced by the render number; a given `--prev` is only a check against the driver's own previous destination (`HISTORY-INPUT-MISMATCH`, exit 1). Without `--host-times` the program renders one frame with the previous destination given by `--prev`.

Why the stage spools: the history driver takes the whole list of renders in one call (`--host-times`, with `%R` naming each render's input) and keeps the frame record and the previous destination inside that call (`run_history`, `crates/effects/fx_exec/src/fxrun.rs` line 3313), so the stage hands it the whole clip. A stage that streams frame by frame would carry `FrameRecord` and the previous destination across runner calls, which the code does not do (INFERRED); the repository records no such change.

Per-frame parameters: the driver gives every render the same arguments apart from `%R`, and each render is a run of its own (`run_locked` once per render in `run_history`), so a value per render would need a per-render argument list in the driver. The code has none, and the repository records no plan for one (INFERRED from `run_history`).

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means) on the final binary, judged against the original running in an emulator. For `S_Trails`, `docs/data/effects_status.tsv` holds 78 verified comparisons, 170 recorded renders and 220 re-rendered units (a second render for the cases that render twice), all equal in both renders; none of the 38 comparisons that differ from the original is of this effect. The README table "Effects and stages" gives 70 of 120 counted recorded renders byte identical through the stage, measured by `crates/neutral-handoff/tests/fx_stage_parity.rs`; the other counted renders use a different host arrangement and are not claimed for the stage ([Effects provider](../stages.md#effects-provider)). No approximate reciprocal instruction is used on the verified render path (MEASURED).

How the counts relate: of the 170 recorded renders, 120 are counted and 50 are controls, recorded and not counted, and the 220 re-rendered units are the 170 renders plus a second render of the 50 cases that render twice (DERIVED from the columns of `docs/data/effects_status.tsv`, the README stage figure and the definitions of [Effects](../effects.md)). Which counted renders belong to each of the 78 comparisons (one comparison may hold several renders of a history), which renders are the controls, and the host options in which the 50 counted renders outside the stage figure differ from the `host` and `fixed` columns are recorded outside the repository; the docs name the kinds of difference only: another host arrangement, a fractional frame, or a `--prev` file given outside the history chain ([Effects provider](../stages.md#effects-provider)).

## Checks that run from this repository

| check | command | expected result | exit |
|---|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| stage listed | `target/release/sapphire stage list --json` | the `trails_autogen` entry with `"kind": "temporal"`, `"spool": true`, `"layers": []` and 11 parameters | 0 |
| parameters | `target/release/fx_run --describe-params Trails_Autogen` | JSON with the 22 registered parameters of the program | 0 |
| worked render | the `sh` block below | 98,304 bytes, 3 frames of 64x32 | 0 |
| refusals | the `sh` block below | the labels of Styles and stages | 3, 3, 3 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 5 rows of `trails_autogen` | 0 |
| conformance | `python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply trails_autogen"` | L1 `14 passed, 2 failed` (the two 1x1 cases), L3 `33 passed` | 1 |
| device report | the device block below | the report line of Device path; output equal to the CPU | 0 |
| device golden rows | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` | 0 |
| crate and executor tests | `cargo test --release --offline -p fx_eval -p fx_exec` | tests pass; tests that need stored data print `NOT RUN` | 0 |
| stage tests | `cargo test --release --offline -p neutral-handoff` | tests pass; `fx_stage_parity` prints `NOT RUN` without `VEC_DIR` | 0 |
| device tests | `SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_neighbourhood --test device_backend -- --test-threads=1` | tests pass on an OpenCL device | 0 |

Build, then make the input (ffmpeg on `PATH`; scratch files under the git ignored `target/demo/`):

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
mkdir -p target/demo
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/in.raw
target/release/sapphire stage list --json | python -I -c "import json,sys; s=[x for x in json.load(sys.stdin)['stages'] if x['id']=='trails_autogen'][0]; print(s['kind'], s['spool'], s['layers'], len(s['params']))"
target/release/fx_run --describe-params Trails_Autogen
```

Each command ends with exit 0; the python line prints `temporal True [] 11`. Without ffmpeg the input file is not made and the later commands that read it fail.

The worked render of the `S_Trails` section of [Effects](../effects.md):

```sh
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param blur_amount=1.5625 --param mocha_mask_opacity=0.5 < target/demo/in.raw > target/demo/trails.raw
```

Exit 0 and 98,304 bytes (3 frames of 64x32 at 16 bytes a pixel).

Three refusals:

```sh
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 24 --param "blur_amount=1@0;2@10" < target/demo/in.raw > target/demo/r1.raw
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=1 --json < target/demo/in.raw > target/demo/r2.raw
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 24 --layer To=target/demo/in.raw < target/demo/in.raw > target/demo/r3.raw
```

Each exits 3 with no output: the first with `renders the whole clip in one fx_run run`, the second with the error object on standard error whose `stderr_tail` holds the `SOURCE-VARIANT-MISMATCH` line, the third with `LAYER-INPUT-UNSUPPORTED`.

The golden rows and the grep that lists this effect's rows:

```sh
python -I tools/fx_golden.py
grep -E '^trails_autogen\s' tools/fx_golden.tsv | cut -f1,2
```

`fx_golden.py` ends with `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0 (a difference is a `DIFFERS` line and exit 1). The grep prints 5 rows: the plain sizes `64x32` and `50x7`, the special row `40x12s` and the threaded rows `64x32t` and `50x7t`; the stage has no variant rows. Every golden case runs with `--fps 24 --frames 3` and the default parameters, whose plan holds `*`, `+` and `postdivide`.

What the golden rows reach (MEASURED by a device report of each of the 129 golden cases other than the threaded rows, on PoCL on the check build): the plain rows and the special row of this stage `device: * 6, + 3, postdivide 3; fallback: none`. The ops `blur`, `hue_shift`, `dissolve`, `max`, `min`, `screen_super` and `comp_rgba` of this plan are reached by no golden row of this stage; `blur`, `hue_shift`, `dissolve` and `screen_super` are reached by golden rows of other stages, and `max`, `min` and `comp_rgba` by no golden row at all. A variant row for this stage (an entry of `VARIANTS` in `tools/fx_golden.py`, then `--write`) would be a repository change; the repository records no plan for it.

Conformance on the stage (the command must name the binary by absolute path):

```sh
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply trails_autogen"
```

Exit 1, with L1 `14 passed, 2 failed` and L3 `33 passed`: the two `FAIL` lines are the 1x1 cases, refused with exit 3 (`trails_autogen needs a width of 2 or more`), the documented size refusals of [Conformance](../stages.md#conformance).

The device path (an OpenCL platform such as PoCL; without one `SAPPHIRE_REQUIRE_DEVICE=1` makes the first command exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 24 --device gpu < target/demo/in.raw > target/demo/trails_dev.raw
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 24 < target/demo/in.raw > target/demo/trails_cpu.raw
cmp target/demo/trails_dev.raw target/demo/trails_cpu.raw
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

Each ends with exit 0 and `cmp` prints nothing. The last `device: ` line of the first command is `device: * 6, + 3, postdivide 3; fallback: none`.

The cargo tests (the device ones on an OpenCL device):

```sh
cargo test --release --offline -p fx_eval -p fx_exec
cargo test --release --offline -p neutral-handoff
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise --test device_neighbourhood --test device_backend -- --test-threads=1
```

Each ends with exit 0. Without stored data the tests that read it print `NOT RUN` and pass; without an OpenCL device and without `SAPPHIRE_REQUIRE_DEVICE=1` the device tests print `NOT RUN` and pass, and with the variable set they fail instead.

In CI (`.github/workflows/ci.yml`), `cargo test --release` (the default members, `neutral-handoff` among them), `python -I tools/fx_golden.py` on Linux, macOS and Windows, `cargo test --release -p fx_eval -p fx_exec ...` on Linux, and in the job `device backend on PoCL` the self-check tests (`cargo test --release -p fx_exec --lib device`), the device test files (among them `device_backend`, `device_pointwise` and `device_neighbourhood`, with `--test-threads=1`) and `fx_golden.py` with `SAPPHIRE_DEVICE=gpu` run these checks.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders and the recorded runs of the original, histories included; the repository holds the native code, the stage row and the history inputs taken from those runs, and this repository's own golden hashes. The 78 verified comparisons and the 170 recorded renders cannot be rerun from the repository alone. With the stored data under `VEC_DIR` (and an `fx_run`), `crates/neutral-handoff/tests/fx_stage_parity.rs` renders the 70 matching recorded renders through the stage; without it the test prints `NOT RUN: VEC_DIR is not set` and passes. The executor tests of the program (`t2.rs`, `t12.rs` and the others above) and `trails_cmp` need stored plans and planes ([Tests](../effects.md#tests)). The full report is kept outside the repository; its SHA-256 is printed in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32 at the sizes, parameter values and history lengths of the recorded renders. The time base starting at 1 is a per style host choice, measured for two frame histories; trails longer than the recorded histories are INFERRED ([Effects provider](../stages.md#effects-provider)). Other sizes and values run the same code and are INFERRED; depths 8 and 16 are not verified.
- Memory: about 33 bytes a pixel for every frame of the clip (MEASURED), so a 1080p clip of 100 frames needs about 6.8 GB (INFERRED from that rate).
- Keyed values are refused: the stage renders the clip in one run, at one set of parameter values.
- Only the Opacity answer 2 renders on the stage's host arrangement; the answers 1 and 3 are refused (`SOURCE-VARIANT-MISMATCH`, exit 3), because no recorded render of this style sets the popup.
- Host buffer phase: the trail's blur windows take their order from the phase above the thresholds (a window of 64 samples or more, which needs a frame 128 pixels or more wide or high; [Limits common to the effects](../effects.md#limits-common-to-the-effects)). The placement column is empty: the stage passes no phase.
- At 1920x1080 at the default parameters the plan holds no blur (`*`, `+`, `postdivide`), so no host buffer phase enters the render (INFERRED from the plan). With `blur_amount` set, a blur window of 64 samples or more takes the blur scratch phase, 0 by default (assumption B-A4, `crates/effects/kernels/blur/src/lib.rs` line 240), and the render equals the original's when the host gives that phase (INFERRED). No render at that size was compared.
- Approximate instructions: none is used on the verified render path (MEASURED, the `S_Trails` section of [Effects](../effects.md)), which holds renders with the trail blur and the hue turn set, so no approximate-instruction model or bound applies to this effect.
- The `Mask` input is not offered; mask tracking data is not part of the rebuild ([The tracker mask group](../parameters-guide.md#the-tracker-mask-group)).
- NaN: the CPU kernels keep the x86 NaN rules; NaN payload and sign bits are not verified against the original. The special golden row `40x12s` fixes the NaN words of this repository's own render on each CI platform.
- `ROUTE AVX2-ONLY`: the tier of the original that runs on CPUs with AVX2 and FMA is reproduced; other tiers are not.
- `blur_amount` is a host-unit value (`--host-param`), converted with the frame width; the `default` printed for it is the record value, not a host value.

## Device path

| op | device kernel | NaN class | on NaN output |
|---|---|---|---|
| `*` | `device_ops_pointwise::MUL` | `NAN_EXACT_COMPILED` | stays on the device |
| `+` | `device_ops_pointwise::ADD` | `NAN_EXACT_COMPILED` | stays on the device |
| `postdivide` | `device_ops_pointwise::POSTDIVIDE` | `NAN_EXACT_WRITTEN` | stays on the device |
| `blur` | `device_ops_neighbourhood::BLUR` | `NAN_EXACT_WRITTEN` | stays on the device |
| `hue_shift` | `device_ops_pointwise::HUE_SHIFT` | not NaN exact | rerun on the CPU (`nan_output`) |
| `dissolve` | `device_ops_pointwise::DISSOLVE` | `NAN_EXACT_COMPILED` | stays on the device |
| `max`, `min` | `device_ops_pointwise::MAX`, `MIN` | `NAN_EXACT_WRITTEN` | stays on the device |
| `screen_super` | `device_ops_pointwise::SCREEN_SUPER` | `NAN_EXACT_COMPILED` | stays on the device |
| `comp_rgba` | `device_ops_pointwise::COMP_RGBA` | not NaN exact | rerun on the CPU (`nan_output`) |

No op of the plan has a `CPU_ONLY` entry (the table is empty). A node runs on the CPU when its op fails the self-check on the device in use (`not_allowlisted`), when its kernel declines the call (`unsupported`), when the device budget would be exceeded (`memory`) or on a driver error (`opencl_error`); the bytes are the CPU's in every case. MEASURED on PoCL on this build, 64x32, 3 frames of `testsrc`, `fallback: none` on every run and each output equal to the CPU output:

| parameters | report line |
|---|---|
| default | `device: * 6, + 3, postdivide 3; fallback: none` |
| `blur_amount=1.5625 mocha_mask_opacity=0.5` (the worked render) | `device: * 6, + 3, blur 3, dissolve 3, postdivide 3; fallback: none` |
| `prev_hue_shift=0.25 blur_amount=2 blur_rel=0.5,1.75` | `device: * 6, + 3, blur 3, hue_shift 3, postdivide 3; fallback: none` |
| `combine=1` | `device: * 3, max 3, postdivide 3; fallback: none` |
| `combine=2` | `device: * 3, postdivide 3, screen_super 3; fallback: none` |
| `combine=3` | `device: * 3, + 3, postdivide 3; fallback: none` |
| `combine=4` or `combine=5` | `device: * 3, comp_rgba 3, postdivide 3; fallback: none` |
| `combine=6` | `device: * 3, min 3, postdivide 3; fallback: none` |

The section of `S_Trails` in [Effects](../effects.md) records `fallback: none` on every golden row, the special row included (MEASURED on PoCL); the golden rows run the default parameters. Speed in [Speed](../guide.md#speed): one 1920x1080 frame takes 0.86 and 0.82 s on the CPU and 0.94 and 0.93 s on PoCL, against 1.26 and 1.16 s before the in-process route; the PoCL figures show correctness on a CPU runtime, not GPU speed.

## Changing the code safely

- Gates that stay green: `python -I tools/fx_golden.py` (the 5 `trails_autogen` rows, and the rows of every other stage that shares an op), the device run of the same table, `cargo test --release -p fx_eval -p fx_exec` (the history unit tests among them), `cargo test --release` (default members), the device tests with `SAPPHIRE_REQUIRE_DEVICE=1` and `--test-threads=1`, `every_op_has_a_device_kernel_or_a_cpu_only_entry` in `device_ops.rs`, `python -I tools/test_effect_rules.py`, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- The golden rows run the default parameters only, so they reach `*`, `+` and `postdivide`; a change to `blur`, `hue_shift`, `dissolve`, `max`, `min`, `screen_super` or `comp_rgba` is caught for this effect by the comparison harness, by the golden rows of the other stages that use those ops, and by the device tests, not by this effect's golden rows.
- A golden difference means the output bytes of the stage changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees; that takes the comparison harness. `python -I tools/fx_golden.py --write` renders every case and rewrites all of `tools/fx_golden.tsv` with whatever the current build gives, so it is a record of the build, not a check.
- The keep and restart rule of `history.rs` is pinned by its unit tests, among them the fused form of the window limit (`fused_limit_discriminates`); the mutation features of `crates/effects/fx_exec/Cargo.toml` that change the rule exist only to show that those tests fail, and are never part of a shipped build.
- A CPU parity change to an op removes its `ALLOWLIST` entry unless its device kernel changes in the same patch; a kernel keeps the CPU's operation order, uses `fma()` only where the CPU calls `mul_add`, and declines what it does not reproduce ([Device](../adding-an-effect.md#device)).
- No platform math library call in effect code (`fx_eval::detmath` or exact integer code instead); a fresh NaN is canonicalized to `0xffc00000` before a sign operation, and a two-NaN pick goes through `fx_common::nanop`; `test_effect_rules.py` scans for the calls ([Crate layout](../adding-an-effect.md#crate-layout)).
- A change to the stage row (the `fixed` history inputs among them) or the parameter rows changes the host arrangement: `fx_stage_parity.rs` with `VEC_DIR` rechecks the 70 matching recorded renders, and `fx_golden.py` must still pass.
- Docs and data to update with a change: the `S_Trails` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), the tables of [Adding a provider](../stages.md#effects-provider), [Stage kinds, spool and keyframes](../cli.md#stage-kinds-spool-and-keyframes) when the memory or keyframe behaviour changes, `docs/data/effects_status.tsv` when a count changes, and this page; then `python -I tools/gen_docs.py`.

## Open items

- Keyed values are refused because the clip renders in one run (the `spool` check in `apply_profile`, `crates/neutral-handoff/src/fxstage.rs`).
- Memory grows with the clip length ([Stage kinds, spool and keyframes](../cli.md#stage-kinds-spool-and-keyframes)).
- The Opacity answers 1 and 3 have no host variant entry (`host_variants` `-` in `crates/neutral-handoff/data/fx_stages.tsv` line 13).
- The golden table has no variant rows for this stage: `blur`, `hue_shift`, `dissolve`, `max`, `min`, `screen_super` and `comp_rgba` are checked for this effect only by the comparison harness, and `max`, `min` and `comp_rgba` by no golden row of any stage (`VARIANTS` in `tools/fx_golden.py`).
- `hue_shift` and `comp_rgba` are not NaN exact on the device: their NaN outputs rerun on the CPU (`NAN_EXACT` in `crates/effects/fx_exec/src/device_ops.rs` line 169).
- With consecutive times the history restarts at time 2^24, where the f32 limit of `window_limit` (`crates/effects/fx_exec/src/history.rs` line 46) stops exceeding the time (INFERRED); real host times are not recorded.
- The mapping of comparisons to counted renders, the control renders and the host options of the 50 counted renders outside the stage figure are recorded outside the repository (`docs/data/effects_status.tsv` line 19 gives the counts only).
- Trails longer than the recorded histories, and the time base starting at 1 beyond two frame histories, are INFERRED ([Effects provider](../stages.md#effects-provider)).
- `ROUTE AVX2-ONLY` and the unverified host settings of [Limits common to the effects](../effects.md#limits-common-to-the-effects).
