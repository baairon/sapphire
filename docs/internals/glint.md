# Internals of `stage apply glint`

This page covers the code of this stage: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. The stage is the one style of `S_Glint`. The user reference is the `S_Glint` section of [Effects](../effects.md), the `S_Glint` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). `alpha_from_rgb_max`, `screen_super` and the output route are shared with `S_Glow` and are described on the [`S_Glow` internals](glow.md) page; the blur core is described on the [`S_Blur` internals](blur.md) page. The architecture every effect shares is in the [index](README.md).

## What it computes

The program keeps the parts of the source above `threshold` (a bright pass) and draws them out into streaks along the rows, the columns and the diagonals: each direction is a chain of box filters whose length comes from `size`, scaled per direction (`size_x`, `size_y`, `size_diag1`) and per colour channel (`size_red`, `size_blue`), and whose strength comes from `brightness`, scaled per direction (`brightness_x`, `brightness_y`). The streaks of each pass are joined into the rays plane by a scaled screen blend. `hue_shift` turns the hue of the rays and keeps their luma (`hue_shift_preserve_luma`), and `glint_from_alpha` takes the rays from the alpha instead of the colour. A combine step joins the rays with the source by a screen blend (`screen_super`), weighted by `scale_source` and `scale_background`, and `affect_alpha` sets how much the rays add to the alpha (`alpha_from_rgb_max`).

Diagonal streaks are rendered on the stage (MEASURED: `size_diag1` changes the output, commands below). The source record of a served frame holds a diagonal mode word of 0 (`SERVED_DIAGONAL_MODE`, `crates/effects/kernels/glint_wire/src/op.rs` line 16), and with that word the worker runs each diagonal pass as a plane copy followed by the box sums along the diagonals of `diagonal_box_sum` (`PassOp::DiagonalBox`, `crates/effects/kernels/glint_worker/src/lib.rs` line 476, mapped in `crates/effects/kernels/glint_wire/src/host.rs` line 210). A nonzero word would select the zigzag diagonal bodies of `glint_diag_box` (`PassOp::DiagonalBody`, worker line 469, host line 181); that is the diagonal route the stage does not serve. On the device the box sums run as `diag_box_sum_f32`, and `glint_diag_band_f32` and `glint_diag_edge_f32` hold the bodies of the route that is not served.

Pass lengths (INFERRED from the code): for a row or column pass of channel c the worker takes the pass scale lane times the size factor of c, rounds it to an integer v by adding one half and truncating, and splits v into three box radii (`radii`, `crates/effects/kernels/glint_worker/src/lib.rs` line 227): the integer third of v, then half of the remainder, then the rest, each clamped to half the line length minus one (`tri`, line 244; a line of one sample gets none). The size factors and the pass scale are lanes the program computes from `size`, `size_x`, `size_y`, `size_diag1`, `size_red` and `size_blue` (constructor `eff_glint`); the pass shift is the truncated pass gain lane (worker lines 434 and 461). The other formulas are in the kernel crates: the box passes in `glint_pass_rows` and `glint_pass_cols`, the diagonal box sums in the header of `crates/effects/kernels/diagonal_box_sum/src/lib.rs`, the blend in the header of `crates/effects/kernels/scaled_screen_blend/src/lib.rs` and the bright pass in `crates/effects/kernels/glow_layer/src/bright.rs`.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Glint (registered program, no wrapper) | Glint | `stage apply glint` | temporal | none | false | `cset_opacity` answer 3 |

The row is line 26 of `crates/neutral-handoff/data/fx_stages.tsv`. Its `fixed` column binds the tracker mask parameters `blur_mocha_mask` and `shift_mocha_mask` to 0 as host values, its `placement` column holds empty entries for 50x7 and 64x32, and its `host_variants` column holds one entry, for the Opacity answer 3: no recorded render holds the answer 1. The 16 offered parameters are lines 349 to 364 of `crates/neutral-handoff/data/fx_params.tsv`: `size` is passed as `--host-param` (host units, converted with the frame width), `cset_opacity` as `--cset Opacity=N`, and the other 14 as `--set`. Matte Type and Combine Masks are not offered: the program never reads Matte Type, and Combine Masks gives the default result at every value when no mask is connected.

Refused (MEASURED unless marked):

| Input | Result | Exit |
|---|---|---|
| `--param cset_opacity=1` | `SOURCE-VARIANT-MISMATCH` from the runner | 3 |
| `--param blur_glint=V`, `--param mask_type=V` or any other parameter of the program the stage does not offer | `unknown param` | 3 |
| `--layer` with any id | `LAYER-INPUT-UNSUPPORTED` before any frame is read | 3 |
| a frame narrower than 2 pixels | `needs a width of 2 or more` | 3 |
| `fx_run` with a `--host-param` value off the 16.16 grid | `HOST-VALUE-OFF-GRID` | 2 |
| a kernel guard of the `glint` op (`GLINT-LAYOUT-REFUSED`, `GLINT-BLUR-FLAG-REFUSED`, `GLINT-REGION-SELECTOR-REFUSED`, `GLINT-DIAG-EXTENT-REFUSED`, `GLINT-DIAG-BAND-REFUSED`, `GLINT-SHIFT-REFUSED`) | an execution failure of `fx_run` with the name on standard error and no output; no input the stage serves reaches one; the `S_Glint` section of [Effects](../effects.md) (`docs/effects.md` line 427) gives exit 1 through `fx_run` as INFERRED from its exit map | not measured |

The stage and `fx_run` labels are listed in [Refusal labels](../cli.md#refusal-labels); the six `GLINT-*` labels are guards inside the kernel, documented in the `S_Glint` section of [Effects](../effects.md) and in `crates/effects/kernels/glint_wire/src/refusal.rs`, and since no stage route or `fx_run` argument reaches one, no exit is measured for them. The stage puts every slider value on the 16.16 grid and prints a `note:` line for each value it moves (MEASURED).

## Code map

| Part | Path | Role |
|---|---|---|
| Program constructor | `crates/effects/fx_eval/src/programs_native.rs` (`eff_glint`, line 5106) | the program graph: the optional alpha route (`interp`, `coerce_to_alpha`), the `glint` class with its thresholds, strength, sizes and pass factors, the optional blur of the rays (`blur_glint`, not offered), `hue_shift_preserve_luma` and the combine helper |
| Combine helper | `crates/effects/fx_eval/src/programs_native.rs` (`defun_glint_rgba_combine`, line 5204) | `alpha_from_rgb_max`, the screen blend and the source and background scales |
| Layout and registry | `crates/effects/fx_eval/src/programs_native.rs` (`layout_glint` line 5225; registered in `effects` line 5731, `defuns` line 5785 and `layouts` line 5825) | the parameter layout and the registration of the program |
| Recorded host facts | `crates/effects/rtdata/src/styles/glint.rs` (`STYLE`) | registered parameters, host indices and types, ranges, input rows and normalize_scale rows of Glint |
| Shared class rows | `crates/effects/rtdata/src/classes.rs` | `glint` rows at lines 48 (`CLASS_FLAGS`), 117 (`IMG_ONLY_ARGS`), 620 (`COMBOS`) and 1648 (`COMBOS_EXEC`); `hue_shift_preserve_luma` at 53, 679 and 1707; `alpha_from_rgb_max` and `screen_super` as on the `S_Glow` page |
| Op entry | `crates/effects/kernels/glint_wire` (`op.rs`: `render` line 99, `source_record`, `SERVED_DIAGONAL_MODE`; `refusal.rs`; `mapped.rs`; `host.rs`: `Align`) | takes the plan's three source planes and six float arguments, builds the source record, runs the wide-vector wrapper and the three-plane worker against a composed host that maps every typed call to a ported crate |
| Dispatcher | `crates/effects/kernels/glint_dispatch` (`refusal_of` line 139) | the wrapper of the AVX2 route: reads the layer, decides the value refusals, calls the worker |
| Worker | `crates/effects/kernels/glint_worker` | the orchestration of the passes: the blur and fill requests, the bright pass, and per channel up to four passes (rows, columns and two diagonal ones), each turning a sigma into three box radii and a call record |
| Pass bodies | `crates/effects/kernels/glint_pass_rows`, `glint_pass_cols`, `glint_diag_box`, `diagonal_box_sum` | the row and column box passes over two ping-pong buffers, the diagonal box bodies and the diagonal box sums |
| Blend and bright pass | `crates/effects/kernels/scaled_screen_blend`, `crates/effects/kernels/glow_layer` (`bright.rs`) | the scaled screen blend of a pass into the rays plane; the bright pass shared with `S_Glow` |
| Blur core | `crates/effects/kernels/blur` (`blur_core_phase`) | the blur the worker requests; described on the [`S_Blur` internals](blur.md) page |
| Executor entries | `crates/effects/fx_exec/src/kernels2.rs` (`WIRED` line 13; `glint_lanes` line 1498, `glint_k` line 1506, `alpha_from_rgb_max_k` line 1538, `postdivide_k` line 1375) and `crates/effects/fx_exec/src/kernels.rs` (`hsp_k` line 149, `interp_k` line 383, `screen_k` line 222) | the wrappers; `glint_k` renders the three-plane layout only and passes the alignment phases of `KParams::glint_align` (`crates/effects/fx_exec/src/types.rs` line 307), 0 on every run |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`device_ops_glint::GLINT` line 154, `device_ops_pointwise::HUE_SHIFT_PRESERVE_LUMA` line 125, `device_ops_pointwise::INTERP` line 115, `device_ops_pointwise::COERCE_TO_ALPHA` line 104), `crates/effects/fx_exec/src/device_ops_glint.rs`, `crates/effects/fx_exec/src/device_ops_glint_passes.rs`, `crates/effects/fx_exec/src/device_ops_glint_diag.rs` | the device host of the worker and the pass launches |
| OpenCL sources | `crates/effects/fx_device/src/kernels_glint.rs` (`glint_blend_f32`), `crates/effects/fx_device/src/kernels_glint_passes.rs` (`glint_lines_f32`), `crates/effects/fx_device/src/kernels_glint_diag.rs` (`glint_diag_band_f32`, `glint_diag_edge_f32`, `diag_box_sum_f32`), `crates/effects/fx_device/src/kernels_glow.rs` (`glow_bright3_f32`), `crates/effects/fx_device/src/kernels_neighbourhood.rs` (blur passes) | registered in `SOURCES` and `PROGRAM_PARTS` of `crates/effects/fx_device/src/kernels.rs` |
| Stage rows | `crates/neutral-handoff/data/fx_stages.tsv` line 26, `crates/neutral-handoff/data/fx_params.tsv` lines 349 to 364 | the host arrangement and the offered parameters |
| Golden rows | `tools/fx_golden.tsv` lines 60 to 68; variants in `tools/fx_golden.py` lines 69 and 70 | 9 rows: plain, `t`, `s`, `+ga`, `+hu` |
| Tests | the tests of `glint_wire` (`op.rs`, `precheck.rs`, `refusals.rs`), `glint_dispatch`, `glint_worker`, `glint_pass_rows`, `glint_pass_cols`, `glint_diag_box`, `diagonal_box_sum`, `scaled_screen_blend`; `crates/effects/fx_exec/tests/glint_operands.rs`, `device_glint.rs`, `device_glint_passes.rs`, `device_glint_diag.rs` | see [Checks that run from this repository](#checks-that-run-from-this-repository) |

Computed tables: none.

Ops of the compiled plan (MEASURED with the device report, one node of each per frame at 64x32): at the default parameters `glint`, `alpha_from_rgb_max`, `screen_super` and `postdivide`. `glint_from_alpha` adds `coerce_to_alpha` and `interp`; `hue_shift` adds `hue_shift_preserve_luma`; `scale_source` with `scale_background` adds `*`; the Opacity answer 3 drops `postdivide`. `affect_alpha` changes no node. The blur of the rays folds away at the registered `blur_glint`, which the stage does not offer.

## Render flow

1. `sapphire stage apply glint` reads whole frames and permutes each depth 32 frame into 'ae32' words in memory ([Effects provider](../stages.md#effects-provider)).
2. `--layer` is refused before any frame is read and a width under 2 before the run; each `--param` is range checked and each slider value is put on the 16.16 grid; a keyed value is read per frame. `cset_opacity=1` passes the stage's range check and is refused by the runner (`SOURCE-VARIANT-MISMATCH`), since the `host` column holds the reader head of the answer 2 and no entry serves the answer 1.
3. Per frame the runner options are the `host` column of line 26, the answer 3 entry of `host_variants` when the frame's answer is 3, the empty `placement` entry, the `fixed` column, then `--size`, `--geom`, `--host-frame`, `--ctx-time`, the parameters (`--set`, `--host-param size=V`, `--cset Opacity=N`) and `--ns-default-unbound`. The plan reads no host time; a run without `--fps` gives the same bytes (MEASURED).
4. The run is in process (`fx_exec::fxrun`): the program graph is built from the constructor, compiled against the combination tables and folded; `--do-postdiv` is learned from a `POSTDIV-MISMATCH` refusal as for every stage.
5. The `glint` node (`glint_k`) reads its six float arguments by meaning (thresholds, strength, the two factor triples, the pass scale and the pass gain), uses the frame width as the render scale and calls `glint_wire::op::render`: the source record of the served frame (selector 0, diagonal mode 0), the dispatcher's refusal check, then the worker's calls in order: the blur and zero fill of the scratch planes, the bright pass, and per channel the row, column and diagonal passes, each blended into the rays plane by the scaled screen blend. `hue_shift_preserve_luma`, `alpha_from_rgb_max`, `screen_super` and `postdivide` follow.
6. Threading: the `glint` passes run in sequence on one thread, and no other op of the plan splits its work (INFERRED from the code); `SAPPHIRE_THREADS` and `SAPPHIRE_PAR_MIN` change no byte, which the threaded golden rows check (MEASURED).
7. Device: the `glint` op runs the CPU's dispatcher and worker on the host and launches each pass on the device ([Device path](#device-path)).
8. The output words are relaid into frames/2 planes, one output frame per input frame.

From `fx_run PROGRAM` the same plan runs with the stage row's `host` and `fixed` options, as the `fx_run` commands below do.

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means). For `S_Glint` (`docs/data/effects_status.tsv` line 10): 23 verified comparisons (12 at 64x32 and 11 at 50x7), 23 recorded renders and 23 re-rendered units, all byte identical by full SHA-256. Nine comparisons, five at 64x32 and four at 50x7, use parameter sets whose original output had not been seen when the native output was recorded. Through `stage apply glint` 22 of 23 counted recorded renders match the stage's host arrangement (README "Effects and stages"): the 23rd sets Brightness Diag1, which the stage does not offer. At the registered `threshold` the counted test images give no rays, so the two comparisons at the default parameters verify only that the source passes through; the others set `threshold` to the 16.16 grid value of 0.25, 0.3 or 0.35. All 23 are equal through `fx_run` at the default thread count, at threads 1 and 12 and on the forced device; the 22 the stage offers also at threads 3. The rendered route runs no approximate reciprocal instruction, and on x86_64 and 64-bit ARM the `glint` kernel gives the same output words on 9,976 generated cases at every NaN level (MEASURED under emulation; the `S_Glint` section of [Effects](../effects.md)). No comparison of `S_Glint` differs from the original, and none of the 23 recorded renders is a control: the recorded renders and the counted comparisons are the same 23 (`docs/data/effects_status.tsv` line 10). The counted render outside the stage figure differs from the stage in one parameter, Brightness Diag1, and not in its host arrangement. The repository holds no model or bound for an approximate instruction on this stage, since the rendered route runs none; what the emulator returns for the original's sites is outside it. The terms are defined in the [index](README.md).

## Checks that run from this repository

Run from the repository root after the README [Build](../../README.md#build) step. Exit codes are MEASURED on this build unless the row says otherwise.

| Check | Command | Expected result | Exit |
|---|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| Stage listed | `sapphire stage list --json` filtered to `glint` | `glint S_Glint temporal 16 0` | 0 |
| Parameters | `fx_run --describe-params Glint` | a JSON object keyed by the program name | 0 |
| Worked render | `stage apply glint` on a 64x32 `testsrc` clip | a `note:` line for `size_blue`, 98304 bytes written | 0 |
| Refusals | `cset_opacity=1`, `blur_glint`, `mask_type`, width 1 | the messages of [Styles and stages](#styles-and-stages) | 3 each |
| `fx_run` route | `fx_run Glint` with the row's options; then an off-grid `--host-param` | four output files; then `HOST-VALUE-OFF-GRID` | 0, 2 |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` | 0 |
| Device golden rows | the same with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` | the same line | 0 |
| Device report | `stage apply glint --device gpu`, the variants and the special clip | `fallback: none`, output equal to the CPU | 0 |
| Kernel crates | `cargo test --release --offline -p glint_wire -p glint_dispatch ...` | `test result: ok` | 0 (documented) |
| Executor | `cargo test --release --offline -p fx_exec --test glint_operands` | `test result: ok` | 0 (documented) |
| Device tests | `cargo test ... --test device_glint --test device_glint_diag --test device_glint_passes -- --test-threads=1` | `test result: ok`; `NOT RUN` without an OpenCL device | 0 (documented) |
| Conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply glint"` | L1 14 passed and 2 `FAIL` lines (1x1, exit 3), L3 33 passed | 1 |

Run the blocks of this section in page order from the repository root: later blocks read the files earlier ones write under `target/demo/glint`. The stage, its parameters, one worked render and the refusals (ffmpeg on `PATH` for the clip):

```sh
target/release/sapphire stage list --json | python -I -c "import json, sys; [print(s['id'], s['family'], s['kind'], len(s['params']), len(s['layers'])) for s in json.load(sys.stdin)['stages'] if s['id'] == 'glint']"
target/release/fx_run --describe-params Glint
mkdir -p target/demo/glint
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/glint/in.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24/1 --param threshold=0.350006103515625 --param size=300 --param size_red=0.5 --param size_blue=2.0 --in target/demo/glint/in.raw --out target/demo/glint/out.raw
wc -c target/demo/glint/out.raw
python -I -c "import sys; sys.stdout.buffer.write(bytes(16 * 1 * 32 * 3))" > target/demo/glint/w1.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=1 --in target/demo/glint/in.raw --out target/demo/glint/r.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --param blur_glint=0.5 --in target/demo/glint/in.raw --out target/demo/glint/r.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --param mask_type=1 --in target/demo/glint/in.raw --out target/demo/glint/r.raw
target/release/sapphire stage apply glint --width 1 --height 32 --depth 32 --fps 24 --in target/demo/glint/w1.raw --out target/demo/glint/r.raw
```

Expected: `glint S_Glint temporal 16 0`; the parameter JSON; ``note: `size_blue` 2.0 is put on the 16.16 host grid as 2``; `98304 target/demo/glint/out.raw`; these commands exit 0. The last four exit 3 with `frame 0: fx_run refused: shape error: SOURCE-VARIANT-MISMATCH: ...`, ``unknown param `blur_glint` ``, ``unknown param `mask_type` `` and `glint needs a width of 2 or more (the host grid needs two columns)`.

The `fx_run` route with the options of the stage row (its `--prior-fill` option is the stage's own and is left out):

```sh
python -I -c "import struct, sys; sys.stdout.buffer.write(b''.join(struct.pack('<f', (k * 7 % 31) / 31.0) for k in range(4 * 16 * 8)))" > target/demo/glint/src.u32
python -I -c "import sys; sys.stdout.buffer.write(bytes([0xaa]) * (16 * 16 * 8))" > target/demo/glint/prior.u32
OPTS=$(awk -F '\t' '$1 == "glint" {print $10 "|" $9}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' '\n' | grep -v -e '^--prior-fill' -e '^$' | tr '\n' ' ')
target/release/fx_run Glint --size 16x8 --out target/demo/glint/fx --ae32-src Source=target/demo/glint/src.u32 --ae32-prior target/demo/glint/prior.u32 --ae32-out target/demo/glint/fx.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $OPTS --ns-default-unbound --do-postdiv 1
ls target/demo/glint/fx.*
target/release/fx_run Glint --size 16x8 --out target/demo/glint/og --ae32-src Source=target/demo/glint/src.u32 --ae32-prior target/demo/glint/prior.u32 --ae32-out target/demo/glint/og.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $OPTS --ns-default-unbound --do-postdiv 1 --host-param size=4.00001
```

Expected: the first `fx_run` exits 0 and `ls` lists `fx.ae32.u32`, `fx.dest.f32`, `fx.dest.meta.txt` and `fx.src0.f32`; the second exits 2 with `HOST-VALUE-OFF-GRID (Glint size: ...)` and writes no output.

The golden rows (the `awk` line lists the 9 rows of `glint`: `40x12s`, `50x7`, `50x7+ga`, `50x7+hu`, `50x7t`, `64x32`, `64x32+ga`, `64x32+hu`, `64x32t`):

```sh
python -I tools/fx_golden.py
awk -F '\t' '$1 == "glint" {print $2}' tools/fx_golden.tsv
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0.

The golden clip draws rays at the registered `threshold` (its samples lie in [0.05, 1.05), `tools/fx_golden.py`), so the plain rows check the ray passes and not only the pass-through; `+ga` and `+hu` set `threshold` as `tools/fx_golden.py` lines 69 and 70 give it. The commands render the golden clip, compare it with the output, and show that `size_diag1` changes the output:

```sh
mkdir -p target/demo/glint && python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; sys.stdout.buffer.write(fx_golden.synth_clip(64, 32, 3, 20261106))" > target/demo/glint/synth.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --in target/demo/glint/synth.raw --out target/demo/glint/g0.raw
cmp -s target/demo/glint/g0.raw target/demo/glint/synth.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --param threshold=0.3000030517578125 --in target/demo/glint/synth.raw --out target/demo/glint/g1.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --param threshold=0.3000030517578125 --param size_diag1=0.25 --in target/demo/glint/synth.raw --out target/demo/glint/g2.raw
cmp -s target/demo/glint/g1.raw target/demo/glint/g2.raw
```

Expected: the renders exit 0; each `cmp -s` exits 1, since the files differ: the default parameters draw rays on the golden clip, and `size_diag1` changes the diagonal streaks.

What the golden rows reach (the ops MEASURED by the device report loop below at the same parameter values):

| Golden rows | Ops of the plan |
|---|---|
| plain, `t`, `40x12s` | `glint`, `alpha_from_rgb_max`, `screen_super`, `postdivide` |
| `+ga` | adds `coerce_to_alpha` and `interp` |
| `+hu` | adds `hue_shift_preserve_luma` |
| no row | `*` (`scale_source`, `scale_background`) and the Opacity answer 3 tail (no `postdivide`) |

The device checks (an OpenCL platform such as PoCL; without one, `SAPPHIRE_REQUIRE_DEVICE=1` makes `stage apply` exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --device gpu --param threshold=0.350006103515625 --param size=300 --in target/demo/glint/in.raw --out target/demo/glint/dev.raw
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --param threshold=0.350006103515625 --param size=300 --in target/demo/glint/in.raw --out target/demo/glint/cpu.raw
cmp target/demo/glint/dev.raw target/demo/glint/cpu.raw
for v in glint_from_alpha=1.0 hue_shift=-0.5 scale_source=0.75 cset_opacity=3; do SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --device gpu --param threshold=0.3000030517578125 --param $v --in target/demo/glint/in.raw --out target/demo/glint/v.raw 2>&1 | tail -n 1; done
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; sys.stdout.buffer.write(fx_golden.synth_special(40, 12, 3, 20261106))" > target/demo/glint/special.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glint --width 40 --height 12 --depth 32 --fps 24 --frames 3 --device gpu --in target/demo/glint/special.raw --out target/demo/glint/special.out
sha256sum target/demo/glint/special.out
awk -F '\t' '$1 == "glint" && $2 == "40x12s" {print $5}' tools/fx_golden.tsv
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ`; the last report line of the device render is `device: alpha_from_rgb_max 3, glint 3, postdivide 3, screen_super 3; fallback: none`, and `cmp` prints nothing; the loop prints one line per value, each ending `fallback: none`, with the ops listed in [Code map](#code-map); the special clip ends `fallback: none` and its SHA-256 equals the `40x12s` row. Every command exits 0.

The Rust tests (all the crates below but `interp` and `fx_exec` are default members; none needs `VEC_DIR`):

```sh
cargo test --release --offline -p glint_wire -p glint_dispatch -p glint_worker -p glint_pass_rows -p glint_pass_cols -p glint_diag_box -p diagonal_box_sum -p scaled_screen_blend -p glow_layer -p hue_shift_preserve_luma -p interp
cargo test --release --offline -p fx_exec --test glint_operands
cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_glint --test device_glint_diag --test device_glint_passes -- --test-threads=1
```

Expected: `test result: ok` for every test binary, exit 0. `glint_wire` holds the refusal contract (`precheck.rs`, `refusals.rs`: each refusal named, no plane written, no host call) and `served_frames_never_set_the_diagonal_word` (`tests/op.rs`), which checks that the source record of every served frame holds diagonal mode 0 for 120 combinations of width (1 to 4096), height (1 to 2160) and count. `glint_operands` checks the seven operands, the three-plane layout and the lanes of the pass scale and pass gain. `device_glint` checks the full corpus, a stub driver, the declined calls, the NaN check and mutants of the blend kernel; `device_glint_passes` and `device_glint_diag` compare the pass helpers word for word with the CPU crates and refuse their mutants. Without `SAPPHIRE_REQUIRE_DEVICE` and without an OpenCL platform the device tests print `NOT RUN` and pass.

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply glint"
```

Expected: the L1 line reports 14 passed and 2 failed, with two `FAIL` lines for the 1x1 cases (`code unsupported, exit 3`), the L3 line reports 33 passed, and the run ends `result: fail (exit 1)`, the documented size refusals ([Conformance](../stages.md#conformance)).

CI (`.github/workflows/ci.yml`): `cargo test --release` (line 77) and the golden rows (line 88) on Linux, macOS and Windows; `interp` and the `fx_exec` tests on Linux (line 101); the device job runs the `device` library tests, `device_glint`, `device_glint_diag` and `device_glint_passes` on PoCL (lines 191 and 192) and the device golden rows (line 193).

## What needs the comparison harness

The 23 comparisons with the original, the recorded calls that show the alignment phases do not change the result, and the 9,976 generated kernel cases ran in an emulator harness kept outside this repository, with the original's recorded renders and the stored test data. Tests that read stored data under `VEC_DIR` print `NOT RUN` and pass when it is absent, `crates/effects/fx_exec/tests/t_native_defs.rs` (the stored copies of the host tables) among them; no test of the glint crates needs it. The full report is checked by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Sizes 64x32 and 50x7, depth 32, a pixel aspect of 1.0 (the stage passes 1.0); Opacity answers 2 and 3; the background, sparks matte and mask inputs not connected, mocha off; input alpha between 0.01 and 1. Other sizes and values are INFERRED ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- The kernel renders one route: the three-plane layout, the blur flag at 0, a zero region selector and no diagonal pass. The stage sets the diagonal mode to 0; the pass shift is 0 in every counted render, and every shift element of the recorded plans is a finite literal (MEASURED); that no other value the stage serves reaches the diagonal route or a shift refusal is INFERRED. What would reach each guard (INFERRED from the code): `GLINT-LAYOUT-REFUSED` a `glint` node with a combination tag other than 3, which no plan of the stage compiles; `GLINT-BLUR-FLAG-REFUSED` a flag word set, which `glint_k` never sets (`crates/effects/fx_exec/src/kernels2.rs` line 1529); `GLINT-REGION-SELECTOR-REFUSED` and `GLINT-DIAG-EXTENT-REFUSED` a selector or diagonal word that `glint_wire::op::source_record` never writes; `GLINT-DIAG-BAND-REFUSED` a band outside the host's own partition; `GLINT-SHIFT-REFUSED` a pass gain lane of 2^30 or more, which the program computes from its own literals. The tests pin the served words (`served_frames_never_set_the_diagonal_word` and `op_entry_builds_selector_zero_for_every_served_size` in `crates/effects/kernels/glint_wire/tests`) and the guards' names; no test sweeps the offered parameter ranges.
- Host buffer phase: three alignment phases of the original's scratch memory select the order of some sums (the pass radii, the box sum rows and the blur scratch). They are host values and the code passes each as 0. On the 22 counted renders whose calls were observed the result does not depend on the phase: the product of the pass radii is exact (66 calls), every box sum window is shorter than the peeled loop (176 calls) and every blur window is shorter than 64 samples (154 calls) (MEASURED). A blur or box sum window of 64 samples or more, which larger frames and sizes give, sums in the order the phase selects, and its result at a phase other than 0 is not verified. A 1920x1080 render at the sizes that give such windows rests on the phase 0 the code passes (INFERRED); it is not verified at HD.
- NaN: `glint` computes each two-NaN pick through `fx_common::nanop` and is NaN exact on the device; `hue_shift_preserve_luma` and `interp` are not. NaN payload and sign bits are not part of the verified comparisons.
- `ROUTE AVX2-ONLY`: the wide-vector route only; the narrower vector arm, the scalar arm and the one-time initialisation of the dispatcher are not rebuilt. The eight approximate reciprocal instructions of the original's glint workers lie on the refused flag route and in the refused one-plane layout.
- Refusals: `SOURCE-VARIANT-MISMATCH`, `unknown param`, `LAYER-INPUT-UNSUPPORTED` and the width refusal through the stage (exit 3); `HOST-VALUE-OFF-GRID` through `fx_run` (exit 2); the six `GLINT-*` guards as an execution failure of `fx_run` (exit 1, INFERRED, not reached).

## Device path

| Op | Device entry | `NAN_EXACT` | Reached by |
|---|---|---|---|
| `glint` | `device_ops_glint::GLINT` | yes (`NAN_EXACT_WRITTEN`) | every plan |
| `alpha_from_rgb_max`, `screen_super`, `postdivide`, `*` | as on the [`S_Glow` internals](glow.md) page | yes | every plan; `postdivide` at the answer 2; `*` with `scale_source` |
| `hue_shift_preserve_luma` | `device_ops_pointwise::HUE_SHIFT_PRESERVE_LUMA` | no | `hue_shift` |
| `coerce_to_alpha` | `device_ops_pointwise::COERCE_TO_ALPHA` | yes (`NAN_EXACT_WRITTEN`) | `glint_from_alpha` |
| `interp` | `device_ops_pointwise::INTERP` | no | `glint_from_alpha` |

No op of these plans has a `CPU_ONLY` entry. The `glint` op runs the CPU's own dispatcher and worker on the host against a device host, so every call, every host scalar (the radii, the strengths) and every pass order is the CPU's, and it launches each call on the session's in-order queue: the blur (the neighbourhood blur route), the zero fill, the bright pass (`glow_bright3_f32`), the plane copy, the row and column box passes (`glint_lines_f32`), the diagonal box sums (`diag_box_sum_f32` and the band kernels) and the scaled screen blend (`glint_blend_f32`). The value refusals of `glint_dispatch::refusal_of` are decided before any allocation; a refused or declined call, and a blur alignment phase other than 0, return `unsupported` and the node runs on the CPU, which raises the named error.

MEASURED on PoCL only: `fallback: none` on every golden row, the special row and the variant rows `+ga` and `+hu` included, and on all 23 counted recorded renders through `fx_run` (96 device nodes, the same at threads 1, 3 and 12); the self-check on PoCL reports equal NaN payloads (the `S_Glint` section of [Effects](../effects.md)). Speed of one 1920x1080 frame through the stage ([Speed](../guide.md#speed)): 1.40 / 1.46 s on the CPU and 1.01 / 1.05 s on PoCL, which is CPU OpenCL and not GPU speed.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (plain, `s`, `t`, `+ga` and `+hu` rows), the crate tests and `glint_operands`, `python -I tools/test_effect_rules.py`, the device self-check and mutation tests with `-- --test-threads=1`, and on CI the golden rows on Linux, macOS (64-bit ARM) and Windows.
- A golden difference means the output bytes changed; the table holds this repository's own renders, so the repository alone cannot say whether the original agrees. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` and records a change, it does not check one.
- Kernel rules ([Crate layout](../adding-an-effect.md#crate-layout), [Device](../adding-an-effect.md#device)): no platform libm call in the glint crates; a fresh NaN is `0xffc00000`; each two-NaN pick through `fx_common::nanop`, which keeps `glint` NaN exact. A CPU parity change to a glint crate removes `glint` from `ALLOWLIST` unless `device_ops_glint` and its pass helpers change in the same patch: the device op replays the CPU's worker, so the device launches must follow every change of a pass body, the dispatcher or the worker.
- A new refusal belongs in the worker's one predicate (`glint_worker`) and the op entry (`glint_wire::refusal`), with a precheck test that it writes no plane.
- The bright pass and the blur core are shared with `S_Glow`, `S_GlowEdges` and `S_Blur`: a change there reruns their golden rows too.
- Docs and data to update: the `S_Glint` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), the stage rows, this page and `docs/data/effects_status.tsv`; then `python -I tools/gen_docs.py` and `python -I tools/check_docs.py`.

## Open items

- The Opacity answer 1 is refused (`SOURCE-VARIANT-MISMATCH`): no recorded render holds it (`crates/neutral-handoff/data/fx_stages.tsv` line 26 and [Refusal labels](../cli.md#refusal-labels)).
- Brightness Diag1, the blur of the rays (`blur_glint`), Matte Type and Combine Masks are not offered; one counted recorded render sets Brightness Diag1 and is verified through `fx_run` only. Offering it takes a row `glint`, `brightness_diag1`, `--set`, `brightness_diag1` in `crates/neutral-handoff/data/fx_params.tsv` ([Stage rows](../adding-an-effect.md#stage-rows)) and a render of that counted case through the stage; offering `blur_glint` would reach the refused flag route (INFERRED).
- The six `GLINT-*` guards: the one-plane layout, the blur flag route, a nonzero region selector, the diagonal route and large pass shifts are not rendered (`crates/effects/kernels/glint_wire/src/refusal.rs`).
- Alignment phases other than 0 and box sum or blur windows of 64 samples or more are not verified; `KParams::glint_align` has no `fx_run` option. A change to the pass radii or box sum code is exercised at nonzero phases by the crate tests (`crates/effects/kernels/glint_pass_rows/tests/radii_align.rs`; `align_defaults_to_zero_and_is_forwarded` in `crates/effects/kernels/glint_wire/tests/op.rs`) and by the device corpus, which varies `glint_align` over six cases (`crates/effects/fx_exec/src/device_ops_glint.rs` line 447; the device op declines a blur phase other than 0, line 56). What the original gives at the phases a host allocator produces at HD sizes needs the comparison harness.
- `hue_shift_preserve_luma` and `interp` are not on `NAN_EXACT` (`crates/effects/fx_exec/src/device_ops.rs` line 169).
- No golden row selects the Opacity answer 3 on this stage (`tools/fx_golden.py`, `VARIANTS`); it is covered by the counted recorded renders, which need the comparison harness. The repository's remedy is a `+o3` entry in `VARIANTS` and a `--write` run, as `glow` has ([Stage rows](../adding-an-effect.md#stage-rows)).
