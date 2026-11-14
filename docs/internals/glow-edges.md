# Internals of `stage apply glow_edges`

This page covers the code of this stage: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. The stage is the one style of `S_GlowEdges`. The user reference is the `S_GlowEdges` section of [Effects](../effects.md), the `S_GlowEdges` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The combine step, `alpha_from_rgb_max`, `screen_super` and the output route are shared with `S_Glow` and are described once on the [`S_Glow` internals](glow.md) page; the blur passes are described on the [`S_Blur` internals](blur.md) page. The architecture every effect shares is in the [index](README.md).

## What it computes

The program turns the source into one luminance plane (`monochrome`), finds its edges with an edge detector that blurs the plane by `edges_smooth` and takes the gradient magnitude (`edge_detect`), scales the edges by `edges_brightness`, subtracts `edges_threshold` and clamps the result (`-`, `clamp`). A channel blur (`blur_channels`) spreads the edge image into three colour planes, its size set by `width` and scaled per axis (`width_x`, `width_y`) and per colour channel (`width_red`, `width_green`, `width_blue`), its gain by `glow_brightness` and `color`. The combine step of `S_Glow` then joins that light with the source by one of five blend modes (`combine`), weighted by `glow_under_source`, `scale_source`, `scale_background`, `light_bg` and `affect_alpha`; `glow_from_alpha` takes the edges from the alpha, and `show` outputs either the result or the edge image. The formulas are in the kernel crates: the gradient body and the blur gain of the edge detector in the header of `crates/effects/kernels/edge_detect/src/lib.rs` (`sigma_gain` line 108, `body_for` line 62), the per-plane sigmas and gains of the channel blur in `blur_channels::channel_sigmas` and `channel_gain` (`crates/effects/kernels/blur_channels/src/lib.rs` lines 54 and 38), the blur in the `blur` crates ([blur page](blur.md)) and the combine step on the [`S_Glow` internals](glow.md) page.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Glow_Edges (registered program, no wrapper) | Glow_Edges | `stage apply glow_edges` | temporal | none | false | `cset_opacity` answers 1 and 3 |

The row is line 25 of `crates/neutral-handoff/data/fx_stages.tsv`. Its `fixed` column binds the two tracker mask parameters `blur_mocha_mask` and `shift_mocha_mask` to 0 as host values, its `placement` column holds empty entries for 50x7 and 64x32, and its `host_variants` column holds one entry for each of the Opacity answers 1 and 3. The 20 offered parameters are lines 329 to 348 of `crates/neutral-handoff/data/fx_params.tsv`: `width` and `edges_smooth` are passed as `--host-param` (host units, converted with the frame width), `cset_opacity` as `--cset Opacity=N`, and the other 17 as `--set`. An unbound `width` is passed as its registered default host value (the notes column of the row). This is the rule of `--ns-default-unbound`, which every stage passes: an unbound slider with a normalize scale takes the 16.16 value the host registers for it (`unbound_ns_host`, `crates/effects/fx_eval/src/units.rs` line 258), converted with the frame width, and not the decimal default of the program text (unit test `unbound_glow_width_takes_its_registered_bits_without_the_text_rule`, line 684). Without the flag `fx_run` refuses such a run; a decimal fed in place of the grid value is the kind of feed behind the 34 comparisons that differ ([How parity is judged](#how-parity-is-judged)).

Refused (MEASURED, commands below):

| Input | Result | Exit |
|---|---|---|
| `--layer` with any id | `LAYER-INPUT-UNSUPPORTED` before any frame is read | 3 |
| `--param combine_masks=V` or any other parameter of the program that the stage does not offer | `unknown param` | 3 |
| a frame narrower than 2 pixels | `needs a width of 2 or more` | 3 |
| `fx_run` at a width of 1 | the edge detector refuses (`width 1: the body reads one float past the plane`), no output | 1 |
| `fx_run` with a `--host-param` value off the 16.16 grid | `HOST-VALUE-OFF-GRID` | 2 |
| `fx_run` with a `--set` slider lane off the 16.16 grid | `SET-VALUE-OFF-GRID` | 2 |

The stage puts every slider value on the 16.16 grid of the host before the run and prints one `note:` line per value it moves, so a stage run reaches neither grid refusal (MEASURED). The labels are listed in [Refusal labels](../cli.md#refusal-labels).

## Code map

| Part | Path | Role |
|---|---|---|
| Program constructor | `crates/effects/fx_eval/src/programs_native.rs` (`eff_glow_edges`, line 4698) | the program graph: the optional alpha route (`interp`, `coerce_to_alpha`), `monochrome`, `edge_detect`, the threshold (`-`, `clamp`), the mask product, `blur_channels` and the combine helper |
| Combine helper | `crates/effects/fx_eval/src/programs_native.rs` (`defun_glow_rgba_combine`, line 2694) | shared with `S_Glow` ([`S_Glow` internals](glow.md)) |
| Layout and registry | `crates/effects/fx_eval/src/programs_native.rs` (`layout_glow_edges` line 5022; registered in `effects` line 5731 and `layouts` line 5825) | the parameter layout and the registration of the program |
| Recorded host facts | `crates/effects/rtdata/src/styles/glow_edges.rs` (`STYLE`) | registered parameters, host indices and types, ranges, input rows and normalize_scale rows of Glow_Edges |
| Shared class rows | `crates/effects/rtdata/src/classes.rs` | `edge_detect` rows at lines 33 (`CLASS_FLAGS`), 116 (`IMG_ONLY_ARGS`), 140 (`INT_TYPED_ARGS`), 538 (`COMBOS`) and 1579 (`COMBOS_EXEC`); `blur_channels` at 17, 309 and 1350; `clamp` at 19, 342 and 1383; `monochrome` at 68, 857 and 1885; the rows of the combine step as on the `S_Glow` page |
| `edge_detect` kernel | `crates/effects/kernels/edge_detect` (`render`, line 116 of `src/lib.rs`) | blurs the plane with `blur::blur_core` and computes the gradient magnitude per pixel (central differences inside the plane, one-sided at the first and last column, a row factor on the vertical differences) |
| `blur_channels` kernel | `crates/effects/kernels/blur_channels` (`render`, line 63 of `src/lib.rs`) | one blur per colour plane with that plane's width factor and gain; a plane beyond the source plane count starts as a copy of the plane before it |
| `monochrome`, `clamp`, arithmetic | `crates/effects/kernels/monochrome`, `ops_clamp`, `ops_arith` | luminance, the clamp and the threshold subtraction |
| Blur passes | `crates/effects/kernels/blur`, `blur_coef`, `blur_rows`, `blur_rows_y` | described on the [`S_Blur` internals](blur.md) page |
| Grid checks | `crates/effects/fx_eval/src/units.rs` (`host_value_off_grid` line 269, `check_host_value` line 281, `check_set_lanes` line 362) | `HOST-VALUE-OFF-GRID`, `SET-VALUE-OFF-GRID` and `SET-LANE-OFF-GRID` of `fx_run` |
| Executor entries | `crates/effects/fx_exec/src/kernels2.rs` (`WIRED` line 13; `run_wired` line 1627; `edge_detect_k` line 664, `blur_channels_k` line 1015, `monochrome_k` line 343) and `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED` line 10; `clamp_k` line 458, `binary_k` line 197) | the wrappers; `edge_detect_k` refuses a width of 1 and the two- and three-element source rows by name |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`device_ops_edge::EDGE_DETECT` line 150, `device_ops_blur_channels::BLUR_CHANNELS` line 152, `device_ops_pointwise::CLAMP` line 97, `device_ops_pointwise::MONOCHROME` line 126, `device_ops_pointwise::SUB` line 87), `crates/effects/fx_exec/src/device_ops_edge.rs`, `crates/effects/fx_exec/src/device_ops_blur_channels.rs` (tests in `device_ops_blur_channels_tests.rs`), `crates/effects/fx_exec/src/device_ops_pointwise.rs` | the device launches and their self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_edge.rs` (`edge_grad_f32`), `crates/effects/fx_device/src/kernels_neighbourhood.rs` (blur passes), `crates/effects/fx_device/src/kernels_pointwise.rs` | registered in `SOURCES` and `PROGRAM_PARTS` of `crates/effects/fx_device/src/kernels.rs` |
| Stage rows | `crates/neutral-handoff/data/fx_stages.tsv` line 25, `crates/neutral-handoff/data/fx_params.tsv` lines 329 to 348 | the host arrangement and the offered parameters |
| Stage code | `crates/neutral-handoff/src/fxstage.rs` (`snap_text`, `param_args`; test `host_slider_values_are_put_on_the_16_16_grid`) | the 16.16 grid of every slider value, the host options and variants, the in-process run |
| Golden rows | `tools/fx_golden.tsv` lines 82 to 86 | 5 rows: `40x12s`, `50x7`, `50x7t`, `64x32`, `64x32t` |
| Tests | `crates/effects/kernels/edge_detect/tests/t1.rs`, `crates/effects/kernels/blur_channels/tests/t1.rs`, the crate tests of `ops_clamp` and `monochrome`; `crates/effects/fx_exec/tests/edge_detect_wired.rs`, `blur_channels_narrow.rs`, `t_set_grid_cli.rs`, `device_edge.rs`, `device_edge_mutants.rs`; the unit tests of `crates/effects/fx_eval/src/units.rs` | see [Checks that run from this repository](#checks-that-run-from-this-repository) |

Computed tables: none.

Ops of the compiled plan (MEASURED with the device report, one node of each per frame at 64x32): at the default parameters `monochrome`, `edge_detect`, `-`, `clamp`, `blur_channels`, `*`, `alpha_from_rgb_max`, `screen_super` and `postdivide`. The blend parameters change the tail as on `S_Glow`: `combine` 0 adds a second `*`, 1 runs `+`, 3 runs `diff`, `get_alpha`, `remove_alpha`, `screen` and `v3_to_v4_if4`, 4 runs `overlay_super`; `glow_under_source` adds `comp_rgba` and `interp`; `light_bg` adds `+`, `comp_rgba`, `interp` and a second `*`. `glow_from_alpha` adds `coerce_to_alpha` and `interp`; `show` 1 runs `monochrome`, `edge_detect`, `-`, `clamp`, `coerce_up_a1` and `postdivide`; the Opacity answer 1 leaves `monochrome`, `edge_detect`, `-`, `clamp`, `blur_channels` and `screen_super`, and the answer 3 drops `postdivide`. The order of the nodes follows the constructor (INFERRED).

## Render flow

1. `sapphire stage apply glow_edges` reads whole frames and permutes each depth 32 frame into 'ae32' words in memory ([Effects provider](../stages.md#effects-provider)).
2. `--layer` is refused before any frame is read and a width under 2 before the run. Each `--param` is checked against its range, and each slider value is put on the 16.16 grid of the host (`snap_text`); a keyed value is read per frame.
3. Per frame the runner options are the `host` column of line 25, the `host_variants` entry of the frame's Opacity answer (1 or 3) where one applies, the `placement` entry (empty), the `fixed` column (`blur_mocha_mask` and `shift_mocha_mask` at 0), then `--size`, `--geom`, `--host-frame`, `--ctx-time`, the parameters (`--set`, `--host-param width=V`, `--host-param edges_smooth=V`, `--cset Opacity=N`) and `--ns-default-unbound`. The plan reads no host time; a run without `--fps` gives the same bytes (MEASURED).
4. The run is in process (`fx_exec::fxrun`): the program graph is built from the constructor, compiled against the combination tables of `crates/effects/rtdata/src/classes.rs` and folded; `--do-postdiv` is learned from a `POSTDIV-MISMATCH` refusal and the frame repeated, as for every stage.
5. The nodes run through `fx_exec::exec::execute_compiled` and `fx_exec::kernels::run_any`. `edge_detect` blurs the luminance plane and writes the gradient magnitude; `blur_channels` blurs the clamped edge plane once per colour plane, starting the second and third planes as copies of the first; the combine step follows ([`S_Glow` internals](glow.md)).
6. Threading: `edge_detect` and `blur_channels` call `blur::blur_core` per plane in sequence, and no other op of the plan splits its work by thread (INFERRED from the code), so `SAPPHIRE_THREADS` and `SAPPHIRE_PAR_MIN` change no byte; the threaded golden rows check it (MEASURED).
7. Device: each allowlisted op runs on the device after its self-check; a declined node and a NaN output that the op may not keep run on the CPU ([Device path](#device-path)).
8. The output words are relaid into frames/2 planes, one output frame per input frame.

From `fx_run PROGRAM` the same plan runs with the stage row's `host` and `fixed` options, as the `fx_run` commands below do. `fx_run` refuses what the stage would have put on the grid.

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means). For `S_GlowEdges` (`docs/data/effects_status.tsv` line 12): 68 verified comparisons, 34 recorded renders and 34 re-rendered units. 34 comparisons equal the original and 34 differ: the 34 that differ (DIFF-FEED-OFF-GRID) were run with slider values passed as decimals, where a host passes each slider as its nearest 16.16 fixed-point value; at the host's grid values the output equals the original in every word, measured after the output was seen, so they stay counted as different. The comparisons are 34 at 64x32 and 34 at 50x7, 17 equal and 17 different at each size. The 34 equal comparisons give the 34 recorded renders and 34 re-rendered units, byte identical by full SHA-256 through `fx_run` and through the stage; 34 of 34 counted recorded renders match the stage's host arrangement (README "Effects and stages"). The stage puts every slider value on the grid, and `fx_run` refuses a value off the grid, the commands of the 34 different comparisons among them (the `S_GlowEdges` section of [Effects](../effects.md)). The route runs no approximate reciprocal instruction. No counted render lies outside the stage figure, and none of the 34 recorded renders is a control: the recorded renders are the 34 equal comparisons (`docs/data/effects_status.tsv` line 12; the 34 different comparisons have no recorded render). The terms are defined in the [index](README.md).

## Checks that run from this repository

Run from the repository root after the README [Build](../../README.md#build) step. Exit codes are MEASURED on this build unless the row says otherwise.

| Check | Command | Expected result | Exit |
|---|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| Stage listed | `sapphire stage list --json` filtered to the stage | `glow_edges S_GlowEdges temporal 20 0` | 0 |
| Parameters | `fx_run --describe-params Glow_Edges` | a JSON object keyed by the program name | 0 |
| Worked render | `stage apply glow_edges` on a 64x32 `testsrc` clip | 98304 bytes written | 0 |
| Refusals | `--layer`, `combine_masks`, width 1 | the messages of [Styles and stages](#styles-and-stages) | 3 each |
| `fx_run` route | `fx_run Glow_Edges` with the row's options; then width 1, an off-grid `--host-param` and an off-grid `--set` lane | four output files; then the refusals | 0, 1, 2, 2 |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` | 0 |
| Device golden rows | the same with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` | the same line | 0 |
| Device report | `stage apply glow_edges --device gpu`, the variants and the special clip | `fallback: none` on each, output equal to the CPU | 0 |
| Kernel crates | `cargo test --release --offline -p edge_detect -p blur_channels -p ops_clamp -p monochrome` | `test result: ok` | 0 (documented) |
| Executor and grid | `cargo test ... -p fx_exec --test edge_detect_wired --test blur_channels_narrow --test t_set_grid_cli`, `-p fx_eval --lib units`, `-p neutral-handoff --lib fxstage` | `test result: ok`; `t_set_grid_cli` prints `NOT RUN` without `VEC_DIR` | 0 (documented) |
| Device tests | `cargo test ... --lib device` and `--test device_edge --test device_edge_mutants` with `-- --test-threads=1` | `test result: ok`; `NOT RUN` without an OpenCL device | 0 (documented) |
| Conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply glow_edges"` | L1 14 passed and 2 `FAIL` lines (1x1, exit 3), L3 33 passed | 1 |

The stage, its parameters, one worked render and the refusals (ffmpeg on `PATH` for the clip):

```sh
target/release/sapphire stage list --json | python -I -c "import json, sys; [print(s['id'], s['family'], s['kind'], len(s['params']), len(s['layers'])) for s in json.load(sys.stdin)['stages'] if s['id'] == 'glow_edges']"
target/release/fx_run --describe-params Glow_Edges
mkdir -p target/demo/glow-edges
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/glow-edges/in.raw
target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24/1 --param affect_alpha=0.2899932861328125 --in target/demo/glow-edges/in.raw --out target/demo/glow-edges/out.raw
wc -c target/demo/glow-edges/out.raw
python -I -c "import sys; sys.stdout.buffer.write(bytes(16 * 1 * 32 * 3))" > target/demo/glow-edges/w1.raw
target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24 --layer Matte=target/demo/glow-edges/in.raw --in target/demo/glow-edges/in.raw --out target/demo/glow-edges/r.raw
target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24 --param combine_masks=1 --in target/demo/glow-edges/in.raw --out target/demo/glow-edges/r.raw
target/release/sapphire stage apply glow_edges --width 1 --height 32 --depth 32 --fps 24 --in target/demo/glow-edges/w1.raw --out target/demo/glow-edges/r.raw
```

Expected: `glow_edges S_GlowEdges temporal 20 0`; the parameter JSON; `98304 target/demo/glow-edges/out.raw`; these commands exit 0. The last three exit 3 with `LAYER-INPUT-UNSUPPORTED: glow_edges takes no ...`, ``unknown param `combine_masks` `` and `glow_edges needs a width of 2 or more (the host grid needs two columns)`.

The `fx_run` route with the options of the stage row (its `--prior-fill` option is the stage's own and is left out), then the three refusals of the runner:

```sh
python -I -c "import struct, sys; sys.stdout.buffer.write(b''.join(struct.pack('<f', (k * 7 % 31) / 31.0) for k in range(4 * 16 * 8)))" > target/demo/glow-edges/src.u32
python -I -c "import struct, sys; sys.stdout.buffer.write(b''.join(struct.pack('<f', (k * 7 % 31) / 31.0) for k in range(4 * 1 * 8)))" > target/demo/glow-edges/src1.u32
python -I -c "import sys; sys.stdout.buffer.write(bytes([0xaa]) * (16 * 16 * 8))" > target/demo/glow-edges/prior.u32
python -I -c "import sys; sys.stdout.buffer.write(bytes([0xaa]) * (16 * 1 * 8))" > target/demo/glow-edges/prior1.u32
OPTS=$(awk -F '\t' '$1 == "glow_edges" {print $10 "|" $9}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' '\n' | grep -v -e '^--prior-fill' -e '^$' | tr '\n' ' ')
target/release/fx_run Glow_Edges --size 16x8 --out target/demo/glow-edges/fx --ae32-src Source=target/demo/glow-edges/src.u32 --ae32-prior target/demo/glow-edges/prior.u32 --ae32-out target/demo/glow-edges/fx.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $OPTS --ns-default-unbound --do-postdiv 1
ls target/demo/glow-edges/fx.*
target/release/fx_run Glow_Edges --size 1x8 --out target/demo/glow-edges/w1 --ae32-src Source=target/demo/glow-edges/src1.u32 --ae32-prior target/demo/glow-edges/prior1.u32 --ae32-out target/demo/glow-edges/w1.ae32.u32 --geom -0.5,0.5,1,0,3,0,0 --host-frame 0 --ctx-time 0 $OPTS --ns-default-unbound --do-postdiv 1
target/release/fx_run Glow_Edges --size 16x8 --out target/demo/glow-edges/og --ae32-src Source=target/demo/glow-edges/src.u32 --ae32-prior target/demo/glow-edges/prior.u32 --ae32-out target/demo/glow-edges/og.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $OPTS --ns-default-unbound --do-postdiv 1 --host-param width=4.00001
target/release/fx_run Glow_Edges --size 16x8 --out target/demo/glow-edges/os --ae32-src Source=target/demo/glow-edges/src.u32 --ae32-prior target/demo/glow-edges/prior.u32 --ae32-out target/demo/glow-edges/os.ae32.u32 --geom -0.5,15.5,1,0,-4.5,0,0 --host-frame 0 --ctx-time 0 $OPTS --ns-default-unbound --do-postdiv 1 --set edges_threshold=0.3499999940395355
```

Expected: the first `fx_run` exits 0 and `ls` lists `fx.ae32.u32`, `fx.dest.f32`, `fx.dest.meta.txt` and `fx.src0.f32`; the width 1 run exits 1 with `width 1: the body reads one float past the plane (host memory)`; the off-grid `--host-param` exits 2 with `HOST-VALUE-OFF-GRID (Glow_Edges width: ...)`; the off-grid `--set` lane exits 2 with `SET-VALUE-OFF-GRID (Glow_Edges edges_threshold: ...)`.

The golden rows (the `awk` line lists the 5 rows of the stage):

```sh
python -I tools/fx_golden.py
awk -F '\t' '$1 == "glow_edges" {print $2}' tools/fx_golden.tsv
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0, and the sizes `40x12s`, `50x7`, `50x7t`, `64x32`, `64x32t`.

What the golden rows reach (the ops MEASURED by the device report loop below at the same parameter values):

| Golden rows | Ops of the plan |
|---|---|
| plain, `t`, `40x12s` | `monochrome`, `edge_detect`, `-`, `clamp`, `blur_channels`, `*`, `alpha_from_rgb_max`, `screen_super`, `postdivide` |
| no row | the Opacity answers 1 and 3, `+`, `comp_rgba` and `interp` (`combine` 1, `glow_under_source`, `light_bg`), the second `*` (`combine` 0), `diff`, `get_alpha`, `remove_alpha`, `screen` and `v3_to_v4_if4` (`combine` 3), `overlay_super` (`combine` 4), `coerce_to_alpha` (`glow_from_alpha`) and `coerce_up_a1` (`show` 1) |

The device checks (an OpenCL platform such as PoCL; without one, `SAPPHIRE_REQUIRE_DEVICE=1` makes `stage apply` exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24 --device gpu --in target/demo/glow-edges/in.raw --out target/demo/glow-edges/dev.raw
target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24 --in target/demo/glow-edges/in.raw --out target/demo/glow-edges/cpu.raw
cmp target/demo/glow-edges/dev.raw target/demo/glow-edges/cpu.raw
for v in combine=0 combine=1 combine=3 combine=4 glow_under_source=0.75 light_bg=0.300003052 glow_from_alpha=1 show=1 cset_opacity=1 cset_opacity=3; do SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24 --device gpu --param $v --in target/demo/glow-edges/in.raw --out target/demo/glow-edges/v.raw 2>&1 | tail -n 1; done
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; sys.stdout.buffer.write(fx_golden.synth_special(40, 12, 3, 20261106))" > target/demo/glow-edges/special.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply glow_edges --width 40 --height 12 --depth 32 --fps 24 --frames 3 --device gpu --in target/demo/glow-edges/special.raw --out target/demo/glow-edges/special.out
sha256sum target/demo/glow-edges/special.out
awk -F '\t' '$1 == "glow_edges" && $2 == "40x12s" {print $5}' tools/fx_golden.tsv
```

Expected: `fx_golden: 183 cases, 183 equal, 0 differ`; the last report line of the device render is `device: * 3, - 3, alpha_from_rgb_max 3, blur_channels 3, clamp 3, edge_detect 3, monochrome 3, postdivide 3, screen_super 3; fallback: none`, and `cmp` prints nothing; the loop prints one line per value, each ending `fallback: none`, with the ops listed in [Code map](#code-map); the special clip ends `fallback: none` and its SHA-256 equals the `40x12s` row. Every command exits 0.

The Rust tests (the kernel crates are default members; `fx_eval` and `fx_exec` are not):

```sh
cargo test --release --offline -p edge_detect -p blur_channels -p ops_clamp -p monochrome -p ops_arith
cargo test --release --offline -p fx_exec --test edge_detect_wired --test blur_channels_narrow --test t_set_grid_cli
cargo test --release --offline -p fx_eval --lib units
cargo test --release --offline -p neutral-handoff --lib fxstage
cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_edge --test device_edge_mutants -- --test-threads=1
```

Expected: `test result: ok` for every test binary, exit 0. `edge_detect_wired` asserts the width 1 and the two- and three-element refusals by their text; `blur_channels_narrow` checks the copy of the first plane into the planes beyond the source; the `units` tests of `fx_eval` check the grid refusals of Glow_Edges; `t_set_grid_cli` (`sg_cli_edges_threshold_off_grid_is_refused`) needs the program text under `VEC_DIR` and prints `NOT RUN: the program text is absent` without it. The `device` library tests hold the `blur_channels` compare driver with its mutants; `device_edge` checks the `edge_detect` self-check and a stub gradient kernel it must refuse, and `device_edge_mutants` single-edit mutants of the gradient kernel source.

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply glow_edges"
```

Expected: the L1 line reports 14 passed and 2 failed, with two `FAIL` lines for the 1x1 cases (`code unsupported, exit 3`), the L3 line reports 33 passed, and the run ends `result: fail (exit 1)`, the documented size refusals ([Conformance](../stages.md#conformance)).

CI (`.github/workflows/ci.yml`): `cargo test --release` (line 77) and the golden rows (line 88) on Linux, macOS and Windows; the `fx_eval` and `fx_exec` tests on Linux (line 101); the device job runs the `device` library tests, `device_edge` and `device_edge_mutants` on PoCL (lines 191 and 192) and the device golden rows (line 193).

## What needs the comparison harness

The 68 comparisons with the original ran in an emulator harness kept outside this repository, with the original's recorded renders and the stored test data; the 34 different ones and the grid-value reruns that equal the original need that harness to be rerun. Tests that read stored data under `VEC_DIR` print `NOT RUN` and pass when it is absent: `crates/effects/fx_exec/tests/t_set_grid_cli.rs` (the program text) and `crates/effects/fx_exec/tests/t_native_defs.rs` (the stored copies of the host tables). The full report is checked by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Sizes 64x32 and 50x7, depth 32, a pixel aspect of 1.0 (the stage passes 1.0); Opacity answers 1, 2 and 3; the mask inputs not connected, mocha off. Other sizes and values are INFERRED ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- The second item of `show` and `glow_from_alpha` on: the outputs of the verified comparisons equal the original's, but the compiled plan places two images differently from the original's plan, so these settings are not counted as verified. In the native plan (`fx_run --plan-text`) an image is allocated inside its line-wise block when every consumer of the compiled, pruned node list sits in that block (`crates/effects/fx_eval/src/cpuplan.rs` line 480 with `lgpass::is_group_local`, `crates/effects/fx_eval/src/lgpass.rs` line 339); the pruned list keeps no consumer of the `show` branch that the collapse drops (`PrunedRec`, `crates/effects/fx_eval/src/compile.rs` line 134). The difference lies in where images are allocated and freed, not in the computed values (INFERRED); which images the original's plan places otherwise, and whether that changes an output above the host buffer phase thresholds, needs the recorded plans outside the repository.
- Slider values: the stage passes each on the 16.16 grid; a value given off the grid is moved and a `note:` line names it. Values between keys are INFERRED.
- Host buffer phase: the edge detector and the channel blur run the window of the blur kernel; a window of 64 samples or more (a frame 128 pixels or more wide or high) sums in an order the host buffer phase selects. The stage passes phase 0; at another phase the original's own output can differ (INFERRED above the threshold). A 1920x1080 render reaches that threshold whenever a blur window does, so it rests on the phase 0 the code passes (`DEFAULT_BASE_PHASE`, `crates/effects/kernels/blur/src/lib.rs` line 30), INFERRED; it is not verified at HD.
- NaN: `edge_detect` and `blur_channels` compute each two-NaN pick through `fx_common::nanop` and are NaN exact on the device; `clamp` and `monochrome` are not. NaN payload and sign bits are not part of the verified comparisons.
- `ROUTE AVX2-ONLY`: the AVX2 and FMA tier only; the original's blur code holds approximate reciprocal instructions outside the tier the port reproduces, and the `S_GlowEdges` section of [Effects](../effects.md) states them as found by a static scan of the original's code and not reached on this route (INFERRED, since the scan reads the code and no run counts them). The blur route this stage runs is the one the [blur page](blur.md) describes; the repository holds no model or bound for these instructions, and what the emulator returns for them is outside it.
- Refusals: `LAYER-INPUT-UNSUPPORTED`, `unknown param`, `bad value for` and the width refusal through the stage (exit 3); the edge detector at width 1 (exit 1), `HOST-VALUE-OFF-GRID` and `SET-VALUE-OFF-GRID` through `fx_run` (exit 2).

## Device path

| Op | Device entry | `NAN_EXACT` | Reached by |
|---|---|---|---|
| `edge_detect` | `device_ops_edge::EDGE_DETECT` | yes (`NAN_EXACT_WRITTEN`) | every plan |
| `blur_channels` | `device_ops_blur_channels::BLUR_CHANNELS` | yes (`NAN_EXACT_WRITTEN`) | every plan but `show` 1 |
| `monochrome`, `clamp` | `device_ops_pointwise::MONOCHROME`, `CLAMP` | no | every plan |
| `-` | `device_ops_pointwise::SUB` | yes (`NAN_EXACT_WRITTEN`) | every plan |
| the combine and output ops | as on the [`S_Glow` internals](glow.md) page | as there | the blend parameters and the Opacity answer |

No op of these plans has a `CPU_ONLY` entry. On the device `edge_detect` runs the blur of `edge_detect::render_rows` through the neighbourhood blur route and then the gradient kernel `edge_grad_f32`; the host computes the aspect block, the blur scalars and the body factors with the CPU crate's functions, and `accepts` refuses what `edge_detect_k` refuses by name, so the CPU path then reports its own result. `blur_channels` launches the blur route once per plane with that plane's sigma pair and gain, which the host computes with `blur_channels::channel_sigmas`, `channel_gain` and `blur::pass_count`. On input with NaN both keep their NaN outputs on the device when their self-check reports equal NaN payloads.

MEASURED on PoCL only: `fallback: none` on every golden row, the special row included, and on all 34 counted recorded renders through `fx_run` (330 device nodes, the same at threads 1, 3 and 12; the `S_GlowEdges` section of [Effects](../effects.md)). Speed of one 1920x1080 frame through the stage ([Speed](../guide.md#speed)): 0.89 / 0.91 s on the CPU and 0.77 / 0.79 s on PoCL, which is CPU OpenCL and not GPU speed.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (plain, `s` and `t` rows), the kernel crate tests, the `fx_exec`, `fx_eval` and `neutral-handoff` tests above, `python -I tools/test_effect_rules.py`, the device self-check and mutation tests with `-- --test-threads=1`, and on CI the golden rows on Linux, macOS (64-bit ARM) and Windows, where the special row checks the NaN words across processors.
- A golden difference means the output bytes changed; the table holds this repository's own renders, so the repository alone cannot say whether the original agrees. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` and records a change, it does not check one.
- Kernel rules ([Crate layout](../adding-an-effect.md#crate-layout), [Device](../adding-an-effect.md#device)): no platform libm call; a fresh NaN is `0xffc00000`; each two-NaN pick through `fx_common::nanop`, which keeps `edge_detect` and `blur_channels` NaN exact. A CPU parity change to `edge_detect`, `blur_channels`, `clamp` or `monochrome` removes its `ALLOWLIST` entry unless the device kernel changes in the same patch.
- The grid handling lives in two places that must agree: the stage's `snap_text` (`crates/neutral-handoff/src/fxstage.rs`) and the runner's checks in `crates/effects/fx_eval/src/units.rs`. A change to either reruns `host_slider_values_are_put_on_the_16_16_grid` and the `units` tests.
- The blur passes are shared with `S_Blur`, `S_Glow` and `S_Glint`, the combine step with `S_Glow`: a change there reruns their golden rows too.
- Docs and data to update: the `S_GlowEdges` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), the stage rows, this page and `docs/data/effects_status.tsv`; then `python -I tools/gen_docs.py` and `python -I tools/check_docs.py`.

## Open items

- 34 counted comparisons differ from the original (DIFF-FEED-OFF-GRID, `docs/data/effects_status.tsv` line 12 and the `S_GlowEdges` section of [Effects](../effects.md)): a new run at the host's grid values would be a new comparison, made in the comparison harness, and the 34 stay counted as different by the counting rule of that section.
- `show` at its second item and `glow_from_alpha` on are not counted as verified: the compiled plan places two images differently from the original's plan ([Verified scope and limits](#verified-scope-and-limits)). Removing the difference is a change of the locality rule in `crates/effects/fx_eval/src/cpuplan.rs` and `compile.rs`, checked against the recorded plans outside the repository.
- The edge detector refuses a width of 1 and the two- and three-element source rows (`crates/effects/fx_exec/src/kernels2.rs` line 664).
- `monochrome` and `clamp` are not on `NAN_EXACT` (`crates/effects/fx_exec/src/device_ops.rs` line 169): their NaN outputs are rerun on the CPU.
- No golden row selects the Opacity answers 1 or 3, a `combine` other than the default, `show` or `glow_from_alpha` on this stage (`tools/fx_golden.py`, `VARIANTS`); those plans are covered by the counted recorded renders, which need the comparison harness. The repository's remedy is a `VARIANTS` entry per mode in `tools/fx_golden.py` and a `--write` run, as `glow` has `+o1`, `+o3`, `+c4` and `+c5` ([Stage rows](../adding-an-effect.md#stage-rows)).
- Blur windows of 64 samples or more at a host buffer phase other than 0 are not verified ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
