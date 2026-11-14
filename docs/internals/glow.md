# Internals of `stage apply glow`

This page covers the code of this stage: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. The stage is the one style of `S_Glow`. The user reference is the `S_Glow` section of [Effects](../effects.md), the `S_Glow` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). This page also describes the parts that `S_GlowEdges` and `S_Glint` share with it: the combine helper, `alpha_from_rgb_max`, `screen_super` and the output route. The architecture every effect shares is in the [index](README.md).

## What it computes

The program keeps the parts of the source above a threshold, spreads them with a separable blur whose size is set by `width`, scaled per axis (`width_x`, `width_y`) and per colour channel (`width_red`, `width_green`, `width_blue`), and scales the spread light by `brightness`. A combine step then joins the light with the source by one of five blend modes (`combine`), weighted by `glow_under_source`, `scale_source`, `scale_background`, `light_bg` and `affect_alpha`, and writes the alpha from the largest colour channel of the light. `show` outputs either that result or the thresholded image the light is made from, and `edge_mode` chooses how the blur treats the border of the frame. The atmosphere noise of the program is not rendered: a plan that keeps it is refused (see [Verified scope and limits](#verified-scope-and-limits)). The formulas are in the kernel crates: the bright pass in `crates/effects/kernels/glow_layer/src/bright.rs`, the blur sigmas of each plane in `glow_layer::pass_args` (`crates/effects/kernels/glow_layer/src/full.rs` line 73), the blur itself in the `blur` crates ([blur page](blur.md)), the alpha step in the header of `crates/effects/kernels/alpha_from_rgb_max/src/lib.rs` and the screen blend in the header of `crates/effects/kernels/screen_super/src/lib.rs`.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Glow (registered program, no wrapper) | Glow | `stage apply glow` | temporal | none | false | `cset_opacity` answers 1 and 3 |

The row is line 16 of `crates/neutral-handoff/data/fx_stages.tsv`. Its `fixed` column is empty, its `placement` column holds empty entries for 50x7 and 64x32 (no buffer placement is passed: no executed route reads the default xy buffer), and its `host_variants` column holds one entry for each of the Opacity answers 1 and 3. The 17 offered parameters are lines 158 to 174 of `crates/neutral-handoff/data/fx_params.tsv`: `width` is passed as `--host-param` (host units, converted with the frame width), `cset_opacity` as `--cset Opacity=N`, and the other 15 as `--set`.

Refused on this stage (MEASURED, commands in [Checks that run from this repository](#checks-that-run-from-this-repository)):

| Input | Result | Exit |
|---|---|---|
| `--layer` with any id (the background and matte inputs are not connected) | `LAYER-INPUT-UNSUPPORTED` before any frame is read | 3 |
| `--param atmosphere_amp=V` or any other parameter of the program that the stage does not offer | `unknown param` | 3 |
| `--param cset_opacity=4` (outside 1 to 3) | `bad value for` | 3 |
| `--param edge_mode=2` (outside the popup's range) | `bad value for` | 3 |
| a frame narrower than 2 pixels | `needs a width of 2 or more` | 3 |
| `fx_run` with `atmosphere_amp` other than 0 | `SIMPLEX-GRADIENT-TABLE-ABSENT`, no output written | 2 |
| `fx_run` with a `--host-param` value off the 16.16 grid | `HOST-VALUE-OFF-GRID` | 2 |

The stage itself puts every slider value on the 16.16 grid of the host before the run and prints one `note:` line per value it moves, so a stage run does not reach `HOST-VALUE-OFF-GRID` (MEASURED). The labels are listed in [Refusal labels](../cli.md#refusal-labels).

## Code map

| Part | Path | Role |
|---|---|---|
| Program constructor | `crates/effects/fx_eval/src/programs_native.rs` (`eff_glow`, line 2396) | the program graph of Glow: the source with the optional alpha route (`interp`, `coerce_to_alpha`), the mask product, the atmosphere factor (`RAYS_ATMOSPHERE`), the `glow` class and the combine helper |
| Combine helper | `crates/effects/fx_eval/src/programs_native.rs` (`defun_glow_rgba_combine`, line 2694) | the brightness and alpha step (`alpha_from_rgb_max`) and the five blend modes (`*`, `+`, `screen_super`, the `screen` route with `diff`, `get_alpha`, `remove_alpha` and `v3_to_v4_if4`, and `overlay_super`); shared with `S_GlowEdges` |
| Mask and atmosphere helpers | `crates/effects/fx_eval/src/programs_native.rs` (`defun_process_color_masks` line 2606, `defun_rays_atmosphere` line 2627, `defun_maybe_add_alpha` line 1870) | mask and atmosphere sub-programs; they fold to constants on the stage's host arrangement |
| Layout and registry | `crates/effects/fx_eval/src/programs_native.rs` (`layout_glow` line 2851; `effects` line 5731, `defuns` line 5785, `layouts` line 5825) | the parameter layout and the registration of the program |
| Recorded host facts | `crates/effects/rtdata/src/styles/glow.rs` (`STYLE`) | registered parameters, host indices and types, ranges, input rows and normalize_scale rows of Glow |
| Shared class rows | `crates/effects/rtdata/src/classes.rs` | `glow` rows at lines 49 (`CLASS_FLAGS`), 118 (`IMG_ONLY_ARGS`), 142 to 144 (`INT_TYPED_ARGS`), 640 (`COMBOS`) and 1668 (`COMBOS_EXEC`); `alpha_from_rgb_max` at 14, 264 and 1305; `screen_super` at 87, 1035 and 2063; `screen` at 86, 1001 and 2029; `overlay_super` at 75, 916 and 1944 |
| `glow` kernel | `crates/effects/kernels/glow_layer` (`full.rs`: `render_full`, `pass_args`, `GLOW_PASSES`; `bright.rs`: the bright pass) | copies the source planes, applies the threshold (bright pass) and, unless `show` selects the thresholded image, blurs each plane with `blur::blur_core` |
| Blur passes | `crates/effects/kernels/blur`, `blur_coef`, `blur_rows`, `blur_rows_y` | the separable blur the glow passes run; described on the [`S_Blur` internals](blur.md) page |
| `alpha_from_rgb_max` kernel | `crates/effects/kernels/alpha_from_rgb_max` | copies the three colour planes and writes the alpha as the scaled largest of them (mode 4) |
| `screen_super` kernel | `crates/effects/kernels/screen_super` | the screen blend of two images, per channel, with the operand layout of the arithmetic operators |
| Other blend kernels | `crates/effects/kernels/ops_arith` (`*`, `+`), `screen`, `overlay_super`, `ops_diff`, `get_alpha`, `v3_to_v4_if4`, `ops_coerce` | the blend modes other than the default one and the plane assembly of the fourth mode |
| Output route | `crates/effects/fx_exec/src/postdiv.rs` (`postdivide`), `crates/effects/fx_exec/src/opacity_conv.rs` | the division by alpha of the 'ae32' output at the Opacity answer 2 (`postdivide`), and the conversion of the host's Opacity answer to the value of the program parameter `opacity` |
| Executor entries | `crates/effects/fx_exec/src/kernels2.rs` (`WIRED` line 13; `run_wired` line 1627; `glow_k` line 1447, `alpha_from_rgb_max_k` line 1538, `postdivide_k` line 1375, `get_alpha_k` line 1426) and `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED` line 10; `run_class` line 399; `screen_k` line 222, `screen2_k` line 251, `overlay_k` line 236, `binary_k` line 197, `v3_to_v4_if4_k` line 177) | the wrappers that read the plan operands and call the kernel crates |
| Atmosphere refusal | `crates/effects/fx_exec/src/fxrun.rs` (line 1477) | `SIMPLEX-GRADIENT-TABLE-ABSENT` when the compiled plan keeps `simplex_noise` on its route |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS` line 71: `device_ops_pointwise::MUL` line 88, `device_ops_pointwise::SCREEN_SUPER` line 114, `device_ops_pointwise::POSTDIVIDE` line 119, `device_ops_glow::GLOW` line 145, `device_ops_glow_ops::ALPHA_FROM_RGB_MAX` line 143, `device_ops_glow_ops::SCREEN` and `device_ops_glow_ops::OVERLAY_SUPER` lines 141 and 142), `crates/effects/fx_exec/src/device_ops_glow.rs`, `crates/effects/fx_exec/src/device_ops_glow_ops.rs`, `crates/effects/fx_exec/src/device_ops_pointwise.rs` | the device launches and their self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_glow.rs` (bright pass), `crates/effects/fx_device/src/kernels_glow_ops.rs` (`screen`, `overlay_super`, `alpha_from_rgb_max`), `crates/effects/fx_device/src/kernels_neighbourhood.rs` (blur passes), `crates/effects/fx_device/src/kernels_pointwise.rs` | registered in `SOURCES` and `PROGRAM_PARTS` of `crates/effects/fx_device/src/kernels.rs` |
| Stage rows | `crates/neutral-handoff/data/fx_stages.tsv` line 16, `crates/neutral-handoff/data/fx_params.tsv` lines 158 to 174 | the host arrangement and the offered parameters |
| Stage code | `crates/neutral-handoff/src/fxstage.rs` | frame conversion, host options and variants, the 16.16 grid (`snap_text`), the layer and width refusals, the in-process run |
| Golden rows | `tools/fx_golden.tsv` lines 69 to 81; variants in `tools/fx_golden.py` lines 47 to 50 | 13 rows: plain, `t`, `s`, `+o1`, `+o3`, `+c4`, `+c5` |
| Tests | kernel crate tests under `crates/effects/kernels/glow_layer/tests`, `screen_super`, `screen`, `overlay_super`, `alpha_from_rgb_max`; `crates/effects/fx_exec/tests/glow_operands.rs`, `t_glow_output_operand.rs`, `atmosphere_wired.rs`, `alpha_from_rgb_max_wired.rs`, `t_refusals.rs`, `device_glow.rs`, `device_glow_ops.rs` | see [Checks that run from this repository](#checks-that-run-from-this-repository) |

Computed tables: none. The atmosphere noise needs gradient tables that no documented formula reproduces; none is stored, and the route that would read them is refused ([No external data](../effects.md#no-external-data)).

Ops of the compiled plan (MEASURED with the device report, one node of each per frame at 64x32): at the default parameters `glow`, `*`, `alpha_from_rgb_max`, `screen_super` and `postdivide`. `combine` 0 replaces `screen_super` by a second `*`; `combine` 1 replaces it by `+`; `combine` 3 runs `diff`, `get_alpha`, `remove_alpha`, `screen` and `v3_to_v4_if4` in its place; `combine` 4 runs `overlay_super`; the Opacity answer 1 leaves `glow` and `screen_super`, the answer 3 drops `postdivide`; `show` 1 runs `glow`, `coerce_up_a1` and `postdivide`; `glow_under_source` adds `comp_rgba` and `interp`, `light_bg` adds `+`, `comp_rgba`, `interp` and a second `*`, and `scale_source` adds a second `*`. The order of the executed nodes at the default parameters is `glow`, `*`, `alpha_from_rgb_max`, `screen_super_if_needed` (which the device report counts as `screen_super`) and `postdivide` (MEASURED: the `EVALTEXT` line that `fx_run --plan-text` writes, command below). The `*` multiplies the `glow` output by a factor that the fold of the mask and atmosphere sub-programs makes constant; `brightness` reaches the `glow` node as a folded operand (INFERRED from the constructor).

## Render flow

1. `sapphire stage apply glow` reads whole frames from standard input or `--in`. At depth 32 each frame is permuted into 'ae32' words (A, R, G, B per pixel, straight alpha) in memory; depths 8 and 16 convert through f32 and are not verified ([Effects provider](../stages.md#effects-provider)).
2. `--layer` is refused before any frame is read, and a width under 2 is refused before the run. Each `--param` is checked against the stage's range, and each slider value is put on the 16.16 grid (`snap_text` in `crates/neutral-handoff/src/fxstage.rs`). A keyed value (`V@F;V@F`) is read per frame.
3. The runner options are built per frame, in order: the `host` column of line 16, the `host_variants` entry of the frame's `cset_opacity` answer (1 or 3) where one applies, the `placement` entry of the frame size (empty here) and the `fixed` column (empty), then `--size`, `--geom` (`-0.5,W-0.5,1,0,(H-1-W)/2,0,0`), `--host-frame` and `--ctx-time` from the frame index, the parameters (`--set`, `--host-param width=V`, `--cset Opacity=N`) and `--ns-default-unbound`. With `--fps` the stage also passes `get_time`; Glow reads it only on the refused atmosphere route, and a run without `--fps` gives the same bytes (MEASURED).
4. The run is in process (`fx_exec::fxrun`): the program graph is built from the constructor, compiled against the combination tables of `crates/effects/rtdata/src/classes.rs` and folded. At `atmosphere_amp` 0 the atmosphere factor and the mask folds leave the nodes listed in [Code map](#code-map). `--do-postdiv` is learned from the runner: the stage sends 0, reads the value of a `POSTDIV-MISMATCH` refusal and repeats the frame.
5. The nodes run through `fx_exec::exec::execute_compiled` and `fx_exec::kernels::run_any`, which calls `run_class` (`kernels.rs`) or `run_wired` (`kernels2.rs`). The `glow` node copies the source planes, applies the threshold and blurs each plane in turn; its blur takes the render scale from the frame width and the aspect from the host record, the `--aspect-triple` option of the `host` column, which holds a pixel aspect of 1.0. No stage option sets another aspect, so a clip with non-square pixels renders as if its pixels were square; `fx_run` accepts another `--aspect-triple` and renders (MEASURED with the `fx_run` commands below), but no other aspect is verified.
6. Threading: no op of these plans splits its work by thread. The row bands of `crates/effects/fx_exec/src/par.rs` serve `warp` and `warp_rel`, and the per-plane split of `blur::render` is not called by `glow_layer`, which calls `blur::blur_core` per plane (INFERRED from the code). `SAPPHIRE_THREADS` and `SAPPHIRE_PAR_MIN` therefore change no byte, which the threaded golden rows check (MEASURED).
7. Device: with `--device gpu` or `SAPPHIRE_DEVICE=gpu` each node whose op is on the allowlist and passes the self-check runs on the device; a node the kernel declines, and a node whose device output holds a NaN that `glow` may not keep, run on the CPU ([Device path](#device-path)).
8. The output words are relaid into the frames/2 planes and written; one output frame per input frame. Frame N depends only on input frame N.

From `fx_run PROGRAM` the same plan runs when the caller passes the host options of the stage row, as the `fx_run` command below does. `fx_run` writes the 'ae32' output to the `--ae32-out` path and `PREFIX.dest.f32`, `PREFIX.src0.f32` and `PREFIX.dest.meta.txt`, which lists every label of the run ([Build and run](../effects.md#build-and-run)).

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means): the program text, the builtin classes, the data read, the literals, the compiled plan, the combination tables and the execution, each compared with the original running in an emulator. For `S_Glow` (`docs/data/effects_status.tsv` line 11): 36 verified comparisons, 36 recorded renders and 36 re-rendered units, all byte identical by full SHA-256. The comparisons are 18 at 64x32 and 18 at 50x7; the Opacity answers 1, 2 and 3 are each verified at both sizes with input alpha other than 1. Through `stage apply glow` 36 of 36 counted recorded renders match the stage's host arrangement and are byte identical (README "Effects and stages"); the counted renders at the Opacity answers 1 and 3 (12 on `glow`) run through its `cset_opacity` entries ([Effects provider](../stages.md#effects-provider)). No comparison of `S_Glow` differs from the original, no counted render lies outside the stage figure, and none of the 36 recorded renders is a control: the recorded renders and the counted comparisons are the same 36 (`docs/data/effects_status.tsv` line 11). No approximate reciprocal instruction runs on the verified render path (the `S_Glow` section of [Effects](../effects.md)): the repository holds no model or bound for one on this stage, and what the emulator returns for the twelve sites of the blur code is outside it. The terms are defined in the [index](README.md).

## Checks that run from this repository

Run from the repository root after the README [Build](../../README.md#build) step. Every exit code below is MEASURED on this build unless the row says otherwise.

| Check | Command | Expected result | Exit |
|---|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| Stage listed | `sapphire stage list --json` filtered to `glow` | `glow S_Glow temporal 17 0` | 0 |
| Parameters | `fx_run --describe-params Glow` | a JSON object with the key Glow, one entry per registered parameter | 0 |
| Worked render | `stage apply glow` on a 64x32 `testsrc` clip | three `note:` lines, 98304 bytes written | 0 |
| Refusals | `--layer`, `atmosphere_amp`, `cset_opacity=4`, width 1 | the messages of [Styles and stages](#styles-and-stages) | 3 each |
| `fx_run` route | `fx_run Glow` with the stage row's host options | four output files; with `atmosphere_amp` set, `SIMPLEX-GRADIENT-TABLE-ABSENT` and no output | 0, then 2 |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` | 0 |
| Device golden rows | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | the same line | 0 |
| Device report | `stage apply glow --device gpu` with `SAPPHIRE_DEVICE_REPORT=1` | `fallback: none`, output equal to the CPU | 0 |
| NaN rerun | the special clip through the device | `fallback: glow 3 (nan_output 3)`, the hash of the `40x12s` golden row | 0 |
| Kernel crates | `cargo test --release --offline -p glow_layer ...` | `test result: ok` for every crate | 0 (documented) |
| Executor | `cargo test --release --offline -p fx_exec --test glow_operands ...` | `test result: ok` | 0 (documented) |
| Device tests | `cargo test ... --test device_glow --test device_glow_ops -- --test-threads=1` | `test result: ok`; `NOT RUN` lines without an OpenCL device | 0 (documented) |
| Conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply glow"` | L1 14 passed and 2 `FAIL` lines (the 1x1 cases, exit 3), L3 33 passed | 1 |
| Rules | `python -I tools/test_effect_rules.py` | `OK` | 0 |

The stage, its parameters and one worked render (ffmpeg and ffprobe on `PATH`; without them the clip is not made and the render has no input):

```sh
target/release/sapphire stage list --json | python -I -c "import json, sys; [print(s['id'], s['family'], s['kind'], len(s['params']), len(s['layers'])) for s in json.load(sys.stdin)['stages'] if s['id'] == 'glow']"
target/release/fx_run --describe-params Glow
mkdir -p target/demo/glow
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/glow/in.raw
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24/1 --param width=12.5 --param brightness=1.3500061 --param threshold=0.300003052 --param width_x=1.5 --param width_red=0.800003052 --param edge_mode=0 --in target/demo/glow/in.raw --out target/demo/glow/out.raw
wc -c target/demo/glow/out.raw
```

Expected: `glow S_Glow temporal 17 0`; the parameter JSON; `note:` lines for `brightness`, `threshold` and `width_red`, which the stage puts on the 16.16 grid; `98304 target/demo/glow/out.raw` (three frames of 64x32 at 16 bytes a pixel). Every command exits 0.

The refusals (each exits 3 with empty output):

```sh
python -I -c "import sys; sys.stdout.buffer.write(bytes(16 * 1 * 32 * 3))" > target/demo/glow/w1.raw
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --layer Background=target/demo/glow/in.raw --in target/demo/glow/in.raw --out target/demo/glow/r.raw
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --param atmosphere_amp=0.5 --in target/demo/glow/in.raw --out target/demo/glow/r.raw
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=4 --in target/demo/glow/in.raw --out target/demo/glow/r.raw
target/release/sapphire stage apply glow --width 1 --height 32 --depth 32 --fps 24 --in target/demo/glow/w1.raw --out target/demo/glow/r.raw
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --param edge_mode=2 --in target/demo/glow/in.raw --out target/demo/glow/r.raw
```

Expected standard error lines: `LAYER-INPUT-UNSUPPORTED: glow takes no ...`, ``unknown param `atmosphere_amp` ``, ``bad value for `cset_opacity`: 4 is outside 1 to 3``, `glow needs a width of 2 or more (the host grid needs two columns)` and ``bad value for `edge_mode`: 2 is outside 0 to 1``.

The `fx_run` route, with the options of the stage row (its `--prior-fill` option is the stage's own and is left out) and a 16x8 source:

```sh
python -I -c "import struct, sys; sys.stdout.buffer.write(b''.join(struct.pack('<f', (k * 7 % 31) / 31.0) for k in range(4 * 16 * 8)))" > target/demo/glow/src.u32
python -I -c "import sys; sys.stdout.buffer.write(bytes([0xaa]) * (16 * 16 * 8))" > target/demo/glow/prior.u32
HOST=$(awk -F '\t' '$1 == "glow" {print $10}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' '\n' | grep -v '^--prior-fill' | tr '\n' ' ')
target/release/fx_run Glow --size 16x8 --out target/demo/glow/fx --ae32-src Source=target/demo/glow/src.u32 --ae32-prior target/demo/glow/prior.u32 --ae32-out target/demo/glow/fx.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $HOST --ns-default-unbound --do-postdiv 1
ls target/demo/glow/fx.*
target/release/fx_run Glow --size 16x8 --out target/demo/glow/atm --ae32-src Source=target/demo/glow/src.u32 --ae32-prior target/demo/glow/prior.u32 --ae32-out target/demo/glow/atm.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $HOST --ns-default-unbound --do-postdiv 1 --set atmosphere_amp=0.600006103515625 --host-value get_time=0x00000000
FX="target/release/fx_run Glow --size 16x8 --ae32-src Source=target/demo/glow/src.u32 --ae32-prior target/demo/glow/prior.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 --ns-default-unbound --do-postdiv 1"
$FX $HOST --out target/demo/glow/pt --ae32-out target/demo/glow/pt.ae32.u32 --plan-text target/demo/glow/plan.txt
grep '^EVALTEXT' target/demo/glow/plan.txt
$FX $HOST --out target/demo/glow/og --ae32-out target/demo/glow/og.ae32.u32 --host-param width=4.00001
for e in 0 -1 1 2 5; do $FX $HOST --out target/demo/glow/e$e --ae32-out target/demo/glow/e$e.ae32.u32 --set edge_mode=$e; sha256sum target/demo/glow/e$e.ae32.u32 | cut -c1-16; done
$FX $(echo "$HOST" | sed 's/--aspect-triple [^ ]*/--aspect-triple 0,40000000,3f800000/') --out target/demo/glow/as --ae32-out target/demo/glow/as.ae32.u32
```

Expected: the first `fx_run` exits 0 and `ls` lists `fx.ae32.u32`, `fx.dest.f32`, `fx.dest.meta.txt` and `fx.src0.f32`; the second exits 2 with `SIMPLEX-GRADIENT-TABLE-ABSENT (simplex_noise: ...)` on standard error and writes no file. The plan run exits 0 and its `EVALTEXT` line reads `EVALTEXT	Eval list:\nSource(0)\n_glow(8)\n_*(10)\n_alpha_from_rgb_max(13)\n_screen_super_if_needed(14)\n_postdivide(16)`: the executed nodes in order (the plan file holds the compiled values and is not reproduced here). The off-grid width exits 2 with `HOST-VALUE-OFF-GRID (Glow width: ...)`. The loop exits 0 for every value and prints one hash for 0 and -1 and another for 1, 2 and 5: `fx_run` puts the popup value into its range. The run with a pixel aspect of 2.0 in `--aspect-triple` exits 0.

The golden rows (`python -I tools/fx_golden.py` renders every stage; the `awk` line lists the 13 rows of `glow`: `40x12s`, `50x7`, `50x7+c4`, `50x7+c5`, `50x7+o1`, `50x7+o3`, `50x7t`, `64x32`, `64x32+c4`, `64x32+c5`, `64x32+o1`, `64x32+o3`, `64x32t`):

```sh
python -I tools/fx_golden.py
awk -F '\t' '$1 == "glow" {print $2}' tools/fx_golden.tsv
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0. A difference prints `DIFFERS glow SIZE: expected ..., rendered ...` and exits 1.

What the golden rows reach (the ops MEASURED by the device report loop below at the same parameter values):

| Golden rows | Ops of the plan |
|---|---|
| plain, `t`, `40x12s` | `glow`, `*`, `alpha_from_rgb_max`, `screen_super`, `postdivide` |
| `+c4` (`combine` 3) | `glow`, `*`, `alpha_from_rgb_max`, `diff`, `get_alpha`, `remove_alpha`, `screen`, `v3_to_v4_if4`, `postdivide` |
| `+c5` (`combine` 4) | `glow`, `*`, `alpha_from_rgb_max`, `overlay_super`, `postdivide` |
| `+o1`, `+o3` | `glow` and `screen_super` at the answer 1; `glow`, `*`, `alpha_from_rgb_max` and `screen_super` at the answer 3 |
| no row | `+` (`combine` 1 and `light_bg`), the second `*` (`combine` 0, `scale_source`, `light_bg`), `comp_rgba` and `interp` (`glow_under_source`, `light_bg`), `coerce_up_a1` (`show` 1) and the edge mode other than the registered one |

A change to an op of the last row fails no golden row; a `VARIANTS` entry in `tools/fx_golden.py` and a `--write` run add one ([Stage rows](../adding-an-effect.md#stage-rows)).

The device checks (an OpenCL platform such as PoCL; without one, `SAPPHIRE_REQUIRE_DEVICE=1` makes each `stage apply` exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`, and the device tests panic instead of printing `NOT RUN`):

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --device gpu --in target/demo/glow/in.raw --out target/demo/glow/dev.raw
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --in target/demo/glow/in.raw --out target/demo/glow/cpu.raw
cmp target/demo/glow/dev.raw target/demo/glow/cpu.raw
for v in combine=0 combine=1 combine=3 combine=4 glow_under_source=0.75 light_bg=0.300003052 cset_opacity=1 cset_opacity=3 show=1; do SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24 --device gpu --param $v --in target/demo/glow/in.raw --out target/demo/glow/v.raw 2>&1 | tail -n 1; done
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; sys.stdout.buffer.write(fx_golden.synth_special(40, 12, 3, 20261106))" > target/demo/glow/special.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glow --width 40 --height 12 --depth 32 --fps 24 --frames 3 --device gpu --in target/demo/glow/special.raw --out target/demo/glow/special.out
sha256sum target/demo/glow/special.out
awk -F '\t' '$1 == "glow" && $2 == "40x12s" {print $5}' tools/fx_golden.tsv
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ`; the last report line of the device render is `device: * 3, alpha_from_rgb_max 3, glow 3, postdivide 3, screen_super 3; fallback: none`, and `cmp` prints nothing; the loop prints one report line per value, each ending `fallback: none`, with the ops listed in [Code map](#code-map); the special clip ends `fallback: glow 3 (nan_output 3)`, and its SHA-256 equals the `40x12s` row. Every command exits 0. The first device run of a build pays the self-check of every allowlisted op (minutes on PoCL under load); later runs read the cache ([Rendering on a device](../guide.md#rendering-on-a-device)).

The Rust tests (none of them needs `VEC_DIR`; the kernel crates are default members, `fx_exec` is not):

```sh
cargo test --release --offline -p glow_layer -p alpha_from_rgb_max -p screen_super -p screen -p overlay_super -p ops_arith -p ops_diff -p get_alpha
cargo test --release --offline -p fx_exec --test glow_operands --test t_glow_output_operand --test atmosphere_wired --test alpha_from_rgb_max_wired --test t_refusals
cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_glow --test device_glow_ops -- --test-threads=1
```

Expected: `test result: ok` for every test binary, exit 0. `t_refusals` holds the atmosphere refusal (`a_glow_atmosphere_is_refused_by_name_and_renders_nothing`) and the 32-bit frame size refusal of the `glow` kernel. `device_glow` checks the full op, the bright pass alone, a declined call and single-edit mutants of the bright pass source; `device_glow_ops` checks `screen`, `overlay_super` and `alpha_from_rgb_max` and their mutants. Without `SAPPHIRE_REQUIRE_DEVICE` and without an OpenCL platform the device tests print `NOT RUN` and pass.

The conformance run and the rules:

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply glow"
python -I tools/test_effect_rules.py
```

Expected: the L1 line reports 14 passed and 2 failed, with two `FAIL` lines for the 1x1 cases (`code unsupported, exit 3`), the L3 line reports 33 passed, and the run ends `result: fail (exit 1)`: these are the documented size refusals ([Conformance](../stages.md#conformance)). `test_effect_rules.py` prints `OK` and exits 0.

CI (`.github/workflows/ci.yml`): `cargo test --release` (line 77) runs the kernel crates on Linux, macOS and Windows; `python -I tools/fx_golden.py` (line 88) renders the golden rows on all three; the `fx_exec` tests run on Linux (line 101); the device job runs the `device` library tests and `device_glow` and `device_glow_ops` on PoCL (lines 191 and 192) and the device golden rows (line 193).

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the original's recorded renders and the stored test data. Without them the 36 verified comparisons cannot be rerun here; the repository holds the effect's own renders as golden hashes. Tests that read stored data under `VEC_DIR` (`effects/rtdata/`, the recorded plans and the program text) print a `NOT RUN` line and pass when it is absent; among them `crates/effects/fx_exec/tests/t_native_defs.rs`, which compares the definitions of `crates/effects/rtdata` with the stored copies of the host tables. The full report is checked by its SHA-256 in [Evidence](../effects.md#evidence). The atmosphere route would need the original's gradient tables, which no documented formula or published source reproduces; rendering it within [No external data](../effects.md#no-external-data) needs such a source, and none is known.

## Verified scope and limits

- Sizes 64x32 and 50x7, depth 32, at a pixel aspect of 1.0; Opacity answers 1, 2 and 3; no mask, no matte and no background. Other sizes and values run the same code and are INFERRED ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- Atmosphere is not rendered: `atmosphere_amp` keeps its registered default, which leaves it off; the stage does not offer it or the four parameters that shape the noise; `fx_run` refuses a Glow plan with `atmosphere_amp` other than 0 (`SIMPLEX-GRADIENT-TABLE-ABSENT`, exit 2). No crate ports `simplex_noise`.
- Host buffer phase: the blur passes of `glow` run the window of the blur kernel; a window of 64 samples or more, which needs a frame 128 pixels or more wide or high, sums in an order the host buffer phase selects. The stage passes phase 0; at another phase the original's own output can differ (INFERRED above the threshold). A 1920x1080 render reaches that threshold whenever its blur window does, so it rests on the phase 0 the code passes (`DEFAULT_BASE_PHASE`, `crates/effects/kernels/blur/src/lib.rs` line 30), INFERRED; it is not verified at HD.
- Edge mode: the stage refuses a value outside the popup's range (`bad value for`, exit 3); `fx_run --set edge_mode=V` puts the value into that range before the plan is built (MEASURED with the `fx_run` loop below: 2 and 5 render as 1, -1 as 0); `blur::Edge::from_i32` (`crates/effects/kernels/blur/src/lib.rs` line 19) accepts 0 (zero padding) and 1 (mirror) and refuses any other value when the crate is called directly.
- Time: the plan reads no host time at `atmosphere_amp` 0, so `--fps` changes no byte (MEASURED). Keyed values are plain runs at the keyed value per frame; values between keys are INFERRED.
- NaN: the CPU kernels keep the x86 NaN rules; `glow` is not NaN exact on the device, so NaN outputs of a `glow` node are rerun on the CPU. NaN payload and sign bits are not part of the verified comparisons.
- `ROUTE AVX2-ONLY`: the native code reproduces the original's AVX2 and FMA code tier only.
- Not verified: a bound mask or matte, the tracker mask, mocha with tracking data, a background layer, depths 8 and 16, a real host application. The stage does not offer `combine_masks`, which has no effect without a mask.
- Refusals: `LAYER-INPUT-UNSUPPORTED`, `unknown param`, `bad value for` and the width refusal through the stage (exit 3); `SIMPLEX-GRADIENT-TABLE-ABSENT`, `HOST-VALUE-OFF-GRID` and `FRAME-SIZE-OVER-I32` through `fx_run` (exit 2).

## Device path

| Op | Device entry | `NAN_EXACT` | Reached by |
|---|---|---|---|
| `glow` | `device_ops_glow::GLOW` | no | every plan |
| `*` | `device_ops_pointwise::MUL` | yes (`NAN_EXACT_COMPILED`) | every plan but the Opacity answer 1 and `show` 1 |
| `alpha_from_rgb_max` | `device_ops_glow_ops::ALPHA_FROM_RGB_MAX` | yes (`NAN_EXACT_WRITTEN`) | the same plans |
| `screen_super` | `device_ops_pointwise::SCREEN_SUPER` | yes (`NAN_EXACT_COMPILED`) | the default `combine` |
| `postdivide` | `device_ops_pointwise::POSTDIVIDE` | yes (`NAN_EXACT_WRITTEN`) | the Opacity answer 2 |
| `+` | `device_ops_pointwise::ADD` | yes (`NAN_EXACT_COMPILED`) | `combine` 1, `light_bg` |
| `comp_rgba`, `interp` | `device_ops_pointwise::COMP_RGBA`, `INTERP` | no | `glow_under_source`, `light_bg` |
| `screen`, `overlay_super` | `device_ops_glow_ops::SCREEN`, `OVERLAY_SUPER` | no | `combine` 3 and 4 |
| `diff`, `get_alpha`, `remove_alpha`, `v3_to_v4_if4`, `coerce_up_a1` | `device_ops_pointwise`, `device_ops_plane::GET_ALPHA` | yes (`NAN_EXACT_WRITTEN`) | `combine` 3; `show` 1 |

No op of these plans has a `CPU_ONLY` entry (the table is empty). The `glow` kernel copies the source planes on the device, runs the bright pass (`glow_bright1_f32` or `glow_bright3_f32`) in place and then the separable blur of each plane through the neighbourhood blur route; the host computes the sigmas and the bright pass scale with the CPU crate's own functions (`glow_layer::pass_args`, `glow_layer::bright::scale`). An edge mode the CPU refuses, or a blur call the blur kernels decline, returns `unsupported` and the node runs on the CPU. Its blur passes use the NaN-exact `blur` kernel, but `glow` as a whole is not on `NAN_EXACT`, so a `glow` node whose device output holds a NaN is rerun on the CPU (`nan_output`). No step is known to write a NaN word the device kernel does not copy: the CPU bright pass computes its two-NaN operations through `fx_common::nanop` (`crates/effects/kernels/glow_layer/src/bright.rs`) and the device bright pass through the `nanx_*` helpers (`crates/effects/fx_device/src/kernels_glow.rs`). The op is not admitted: it is on neither NaN-exact list, so its self-check adds no NaN cases, and a unit test pins that it keeps no NaN output (`crates/effects/fx_exec/src/device_ops.rs` line 453). Admitting it takes an entry in `NAN_EXACT` and `NAN_EXACT_WRITTEN`, the change of that assertion and a self-check on the device that reports equal NaN payloads (INFERRED from the code, not measured).

MEASURED on PoCL only: `fallback: none` on the plain, threaded and variant golden rows and on all 36 counted recorded renders through `fx_run` (the `S_Glow` section of [Effects](../effects.md)); `fallback: glow 3 (nan_output 3)` on the special row, whose bytes equal the CPU's. Speed of one 1920x1080 frame through the stage ([Speed](../guide.md#speed)): 0.92 / 0.92 s on the CPU and 0.78 / 0.80 s on PoCL, which is CPU OpenCL and not GPU speed; speed on a GPU is INFERRED.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (all groups: plain, `s`, `t` and the `+o1`, `+o3`, `+c4`, `+c5` variants of `glow`), the kernel crate tests and the `fx_exec` tests above, `python -I tools/test_effect_rules.py`, the device self-check and mutation tests (`device_glow`, `device_glow_ops`) with `-- --test-threads=1`, and on CI the same golden rows on Linux, macOS (64-bit ARM) and Windows, where the special rows check the NaN words across processors.
- A golden difference means the output bytes of the stage changed. The table holds this repository's own renders, so the repository alone cannot tell whether the original agrees; a changed byte needs the comparison harness before it is accepted. `python -I tools/fx_golden.py --write` renders every case and rewrites `tools/fx_golden.tsv`; it records a change, it does not check one.
- Kernel rules ([Crate layout](../adding-an-effect.md#crate-layout) and [Device](../adding-an-effect.md#device)): no platform libm call in `glow_layer` or the blend crates (`detmath` or exact code); a fresh NaN is canonicalized to `0xffc00000`; a two-NaN operation goes through `fx_common::nanop`. A CPU parity change to `glow`, `screen`, `overlay_super`, `alpha_from_rgb_max` or `screen_super` removes its `ALLOWLIST` entry unless its device kernel changes in the same patch; a new op of the plan needs a device kernel or a `CPU_ONLY` entry, which `every_op_has_a_device_kernel_or_a_cpu_only_entry` checks.
- The blur passes are shared with `S_Blur`, `S_GlowEdges` and `S_Glint`: a change to the blur crates reruns their golden rows too ([`S_Blur` internals](blur.md)). The combine helper is shared with `S_GlowEdges` ([`S_GlowEdges` internals](glow-edges.md)).
- Docs and data to update with a change: the `S_Glow` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), the stage rows, this page, and `docs/data/effects_status.tsv` with a new comparison; then `python -I tools/gen_docs.py` (rewrites `docs/generated/`, `docs/index/` and the figure markers) and `python -I tools/check_docs.py`.

## Open items

- Atmosphere: `SIMPLEX-GRADIENT-TABLE-ABSENT` (`crates/effects/fx_exec/src/fxrun.rs` line 1477; [No external data](../effects.md#no-external-data)). The noise needs gradient tables that no documented formula reproduces.
- The background, matte and tracker mask inputs are not connected, and no verified render binds them (`LAYER-INPUT-UNSUPPORTED`; the `S_Glow` section of [Effects](../effects.md)).
- `glow` is not on `NAN_EXACT` (`crates/effects/fx_exec/src/device_ops.rs` line 169): its NaN outputs are rerun on the CPU.
- Blur windows of 64 samples or more at a host buffer phase other than 0 are not verified ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
