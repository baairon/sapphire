# Internals of `stage apply blur_color_autogen` and `stage apply blur_mono_autogen`

This page covers the code of these stages: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. They are the two styles of `S_Blur`. The user reference is the `S_Blur` section of [Effects](../effects.md), the `S_Blur` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md). The blur kernel described here is also the shadow blur of [`S_DropShadow`](drop-shadow.md) and the low-pass of [`S_Sharpen`](sharpen.md).

## What it computes

`S_Blur` blurs the image with a separable box filter repeated one, two or three times (`filter`: Box, Triangle or Gauss), with a radius per axis set by `blur_amount` times the lane of `blur_rel` for that axis. The blurred planes are scaled in the same pass by a light gain from `brightness` and `offset_darks` and by an alpha gain `scale_opacity`; `offset_darks` is then added back to the colour, and the result is mixed back towards the unblurred image by `mix_with_source`. The colour style blurs the R, G, B and A planes and mixes towards the source; the mono style first turns the image into one luma plane and its alpha (`monochrome_a`), blurs those two planes and mixes towards that luma image. Where the tracker mask parameters make the effect mask differ from the constant 1, the blur is the mask weighted `masked_blur` instead of `blur`. A node whose parameters make it an identity (for example `mix_with_source` at 0 or `offset_darks` at 0) is folded out of the compiled plan, so the plan at the defaults holds only the blur and the alpha division (MEASURED below).

The window, read from the code (INFERRED): the stage converts the host value of `blur_amount` to the record value with the frame width and the registered normalize scale ([How the stage passes a value to fx_run](../parameters-guide.md#how-the-stage-passes-a-value-to-fx_run), `crates/effects/fx_eval/src/units.rs`), and `lane_sigma` (`crates/effects/kernels/blur/src/lib.rs` line 137) multiplies the record value by a fixed factor and the frame width again; the two width factors cancel, so the radius of an axis in pixels equals the host value of `blur_amount` times that axis's lane of `blur_rel`. `radii` (`crates/effects/kernels/blur_coef/src/coef.rs`) rounds the radius to the nearest integer when `subpixel` is 0; when `subpixel` is 1 it keeps the integer part A and the ceiling B and blends a window of radius A with one of radius B by the fraction. `pass_count` (blur/src/lib.rs line 115) gives `filter` plus 1 passes (Box 1, Triangle 2, Gauss 3), and `distribute` splits the radius over the passes, each pass taking the remainder divided by the passes left, truncated, and at most half the extent minus 1. Each pass is a box of 2r+1 samples; the final gain divides by the product of the window lengths (`coeffs`, `final_scale`).

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| colour | Blur_Color_Autogen | `stage apply blur_color_autogen` | `temporal` | none | `false` | `-` |
| mono | Blur_Mono_Autogen | `stage apply blur_mono_autogen` | `temporal` | none | `false` | `-` |

The values are the rows of `crates/neutral-handoff/data/fx_stages.tsv` (lines 2 and 3). Both stages offer the same 17 parameters (rows 2 to 35 of `crates/neutral-handoff/data/fx_params.tsv`): `blur_amount` as a host value (`--host-param`), `cset_opacity` as a host popup answer (`--cset`), the other 15 as record values (`--set`). The two styles are the two items of the effect's style popup. The mono row's host options carry the popup answer 2 of 2 (`--effect-answer 2`, `--effect-count 2`), and the runner takes the program at index answer minus 1 (`style_index` in `crates/effects/fx_exec/src/style.rs`); the colour row carries no answer and takes index 0.

Refusals (all MEASURED on this build with the commands of [Checks that run from this repository](#checks-that-run-from-this-repository) and its refusal block):

| input | result | exit |
|---|---|---|
| `--param cset_opacity=1` or `--param cset_opacity=3` | `SOURCE-VARIANT-MISMATCH` (the host arrangement covers only Normal) | 3 |
| `--param cset_opacity=4` | `bad value for cset_opacity: 4 is outside 1 to 3` | 3 |
| `--param filter=3` | `bad value for filter: 3 is outside 0 to 2` | 3 |
| `--width 1` | `blur_color_autogen needs a width of 2 or more` | 3 |
| `--layer To=PATH` (any id) | `LAYER-INPUT-UNSUPPORTED` | 3 |
| a parameter the stage does not offer, such as `mask_use` | `unknown param` | 3 |
| a keyed value without `--fps` | `keyframes need --fps` | 2 |

The same refusal through `fx_run` is exit 2 ([Exit codes](../cli.md#exit-codes)).

## Code map

| part | path | role |
|---|---|---|
| style constructors | `crates/effects/fx_eval/src/programs_native.rs`: `eff_blur_color` (line 135), `eff_blur_color_autogen` (197), `eff_blur_mono` (257), `eff_blur_mono_autogen` (320), `layout_blur_color_autogen` (1890), `layout_blur_mono_autogen` (1924) | the program graphs of the two styles, their wrapper programs and layout rows |
| shared helper programs | the same file: `defun_combine_masks` (1628), `defun_mocha_mask` (1652), `defun_process_masks` (1674), `defun_scale_lights` (1704), `defun_scale_lights_offset_darks` (1712) | the tracker mask chain and the light gain; registered in `effects`, `defuns` and `layouts` (lines 5731, 5785, 5825) |
| parameter and class facts | `crates/effects/rtdata/src/styles/blur_color.rs`, `crates/effects/rtdata/src/styles/blur_mono.rs`; class rows in `crates/effects/rtdata/src/classes.rs` (`CLASS_FLAGS`, `IMG_ONLY_ARGS`, `INT_TYPED_ARGS`, `COMBOS`, `COMBOS_EXEC`) | registered parameters, host indices and types, popup counts, normalize scales, combination tables of `blur`, `masked_blur`, `monochrome_a`, `postdivide`, `add_rgb`, `interp` and the other classes the plans name |
| op `blur` | `crates/effects/kernels/blur` with `crates/effects/kernels/blur_coef` (pass plan and gains), `crates/effects/kernels/blur_rows` (row pass), `crates/effects/kernels/blur_rows_y` (column pass) | the separable box passes; executor entry `blur_k` (`crates/effects/fx_exec/src/kernels2.rs` line 975) |
| op `masked_blur` | `crates/effects/kernels/masked_blur` | mask weighted blur on the tracker mask route; `masked_blur_k` (kernels2.rs line 776) |
| op `monochrome_a` | `crates/effects/kernels/monochrome_a` | luma plane plus alpha plane of the mono style; `monochrome_a_k` (kernels2.rs line 558) |
| op `add_rgb` | `crates/effects/kernels/add_rgb` | adds `offset_darks` to the colour planes; `add_rgb_k` (kernels2.rs line 742) |
| op `interp` | `crates/effects/kernels/interp` | the mix with the source; `interp_k` (`crates/effects/fx_exec/src/kernels.rs` line 383) |
| op `postdivide` | `crates/effects/fx_exec/src/postdiv.rs` | the division of the colour by the alpha that the 'ae32' route adds after the plan; `postdivide_k` (kernels2.rs line 1375) |
| op tables | `SUPPORTED` (`crates/effects/fx_exec/src/kernels.rs` line 10), `WIRED` (kernels2.rs line 13) | `interp` is in `SUPPORTED`; `blur`, `masked_blur`, `monochrome_a`, `add_rgb` and `postdivide` are in `WIRED` |
| style selection | `crates/effects/fx_exec/src/style.rs` | the program index from the style popup answer |
| threading | `crates/effects/kernels/blur/src/lib.rs` (`PAR_MIN_SAMPLES`, `par_min_samples`, `plane_threads`, lines 145 to 160) | one thread per plane on frames of 65,536 samples or more |
| device entries | `crates/effects/fx_exec/src/device_ops_neighbourhood.rs` (`device_ops_neighbourhood::BLUR` line 20, `MASKED_BLUR` line 21), `crates/effects/fx_exec/src/device_ops_pointwise.rs` (`INTERP` 2089, `POSTDIVIDE` 2117, `ADD_RGB` 2234, `MONOCHROME_A` 2286), tables in `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS` 71, `ALLOWLIST` 163, `NAN_EXACT` 169, `CPU_ONLY` 205) | the device kernels and their self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_neighbourhood.rs` (`NEIGHBOURHOOD_BLUR` line 15, `NEIGHBOURHOOD_MASKED` line 379), `crates/effects/fx_device/src/kernels_pointwise.rs` (`POINTWISE_PW` 19, `POINTWISE_PW2` 463, `POINTWISE_FMA` 505), registered in `SOURCES` and `PROGRAM_PARTS` of `crates/effects/fx_device/src/kernels.rs` (lines 94 and 137) | the device programs |
| stage rows | `crates/neutral-handoff/data/fx_stages.tsv` lines 2 and 3; `crates/neutral-handoff/data/fx_params.tsv` lines 2 to 35 | host arrangement and offered parameters |
| stage code | `crates/neutral-handoff/src/fxstage.rs` (`apply_profile`, `run_fx`, `snap_text`) | frame loop, runner arguments, 16.16 grid of host slider values |
| computed tables | none | the blur reads no table; its pass plan is computed per call from the amount and the frame width (`blur_coef`) |
| tests | `crates/effects/kernels/masked_blur/tests/nan_pair.rs`, unit tests in `crates/effects/kernels/monochrome_a/src/lib.rs`, `crates/effects/fx_eval/tests/t27_unbound_default.rs`, `crates/effects/fx_eval/tests/t_fld3_form.rs`, `crates/effects/fx_eval/tests/t_brd2_broadcast.rs`, `crates/effects/fx_exec/tests/t27_style_select.rs`, `crates/effects/fx_eval/tests/t31_lym1.rs` (the layout rows of all 13 registered programs of the first ten effects, the two `S_Blur` programs among them), `crates/effects/fx_exec/tests/device_neighbourhood.rs`, `crates/effects/fx_exec/tests/device_pointwise.rs`, `crates/neutral-handoff/tests/fx_stage_parity.rs`, `crates/neutral-handoff/tests/stage_cli.rs`, `tools/fx_golden.tsv` lines 2 to 11 | see [Checks that run from this repository](#checks-that-run-from-this-repository) |

The ops of the compiled plans are MEASURED with the device report and with `fx_run --plan-text` (commands below). At 64x32 with the defaults the plan of Blur_Color_Autogen holds `blur` and `postdivide`, and that of Blur_Mono_Autogen `monochrome_a`, `blur` and `postdivide`. `mix_with_source` other than 0 adds `interp`, `offset_darks` other than 0 adds `add_rgb`, and a tracker mask parameter that makes the mask differ from the constant 1 replaces `blur` by `masked_blur` (MEASURED on PoCL with the device report at `mocha_mask_opacity=0.5` and at `invert_mocha_mask=1`; `combine_masks`, `bypass_mocha_mask`, `resize_mocha_mask` and `shift_mocha_mask` alone keep `blur`). The crates `blur`, `blur_coef`, `blur_rows`, `blur_rows_y`, `masked_blur`, `monochrome_a`, `add_rgb` and `interp` hold the CPU kernels; all but `interp` are default members of the workspace.

Golden coverage by op: the plain, threaded and special rows run the defaults, so they reach `blur` and `postdivide` on both stages and `monochrome_a` on the mono stage. No golden row reaches `masked_blur`, `add_rgb` or `interp`. On the CPU those three are checked by their crate tests (`masked_blur`: two-NaN picks; `add_rgb` and `interp`: unit tests in `src/`), and on a device by `device_neighbourhood` and `device_pointwise` against the CPU kernels. The parameter values that reach them are verified values ([Driving the effect parameters](../parameters-guide.md) lists `mocha_mask_opacity` 0.5, `invert_mocha_mask` 1, `offset_darks` and `mix_with_source`), and the plan depends only on the parameters, so the counted renders at those values ran the `masked_blur`, `add_rgb` and `interp` nodes (INFERRED from the plans MEASURED above at the same values).

## Render flow

1. `sapphire stage apply blur_color_autogen` (or `blur_mono_autogen`) reads the stage row: program, source name `Source`, the `fixed` options (the mask inputs of the program held at no mask: `blur_mocha_mask`, `invert_mask` and `mask_use`), the `host` options and the `placement` column (the same empty placement at every size).
2. Before any frame: `--layer` is refused, the `host_variants` column (`-`) is checked, a width below 2 is refused, keyed values are parsed (they need `--fps`), and every value of a host slider is put on the 16.16 grid of the host, round half to even; a value that moves prints `note: ... is put on the 16.16 host grid as ...` on standard error (`apply_profile` in `crates/neutral-handoff/src/fxstage.rs`).
3. Per frame: the frame is read and turned into 'ae32' words (at depth 32 a pure byte permutation of the G, B, R, A planes; depths 8 and 16 convert through float and are not verified).
4. The stage calls the runner in process (`fx_exec::fxrun`) with the program, `--size`, the source words, the geometry `-0.5,W-0.5,1,0,(H-1-W)/2,0,0`, the frame index as `--host-frame` and `--ctx-time`, then the `host`, placement and `fixed` options, then the offered parameters (`--set`, `--host-param`, `--cset`), `get_time` and `get_frames_per_second` from `--fps` as host values, `--ns-default-unbound` and `--do-postdiv` (`run_fx` in fxstage.rs). The first frame runs with `--do-postdiv 0`; the runner refuses it with `POSTDIV-MISMATCH`, the stage takes the derived value 1 from the result and repeats the frame, and keeps 1 for the clip.
5. The runner builds the program from the native constructors, binds the parameters (a host value of `blur_amount` is converted with the frame width and the registered normalize scale, [How the stage passes a value to fx_run](../parameters-guide.md#how-the-stage-passes-a-value-to-fx_run)), selects the style from the popup answer, compiles the plan and folds every constant node.
6. The executor runs the nodes in order. `blur` receives the source planes (4 on the colour style, the luma and alpha planes of `monochrome_a` on the mono style), the two widths, the pass count from `filter`, the per plane gain and `subpixel`. Each plane runs the row pass with all its repeats, then the column pass with all its repeats, with mirrored edges (the runner's `--blur-edge` default, 1), then the gain; a zero width on one axis skips that pass (`drive` in `crates/effects/kernels/blur_coef/src/driver.rs`).
7. Threads: `blur` runs its planes on separate threads when the frame holds 65,536 samples or more and `SAPPHIRE_THREADS` is not 1; `SAPPHIRE_PAR_MIN` lowers that threshold. Each plane is computed the same way on any thread, so the bytes do not depend on the thread count (the threaded golden rows check it).
8. Device: with `--device gpu` (or `SAPPHIRE_DEVICE`), each node whose kernel is allowlisted and passed its self-check runs on the device; any other node, and a NaN output of an op that is not NaN exact on that device, runs on the CPU ([Rendering on a device](../guide.md#rendering-on-a-device)).
9. `postdivide` turns the result into straight alpha and the 'ae32' output words are relaid to the frames/2 planes and written. The output frame depends only on input frame N; `--fps` is needed only for keyed values.

`fx_run Blur_Color_Autogen` takes the same arguments directly. A run without the host context options is refused by name (for example `--ctx-time is required (compile context, no default)`, exit 2); the plan text command below shows a complete argument list built from the stage row.

## How parity is judged

An effect counts as rebuilt when each registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means) on the final binary, judged against the original's code in an emulator. For `S_Blur`: 7 verified comparisons, 52 recorded renders and 52 re-rendered units, all equal (`docs/data/effects_status.tsv` line 2; the columns are described in [Effect internals](README.md)). Of the 52 recorded renders 48 are counted, 24 per style; the other 4 are controls, recorded and not counted ([What rebuilt means](../effects.md#what-rebuilt-means)). The repository does not name the controls or what they control for; that is held with the comparison record outside it. Through the stages 36 of the 48 counted recorded renders are byte identical, the ones that use the stage's host arrangement: 18 of 24 on each style (the `S_Blur` row of the table in the README section [Effects and stages](../../README.md#effects-and-stages) and [Effects provider](../stages.md#effects-provider)). The other 12, 6 per style, used another host arrangement and are not claimed for the stages. The repository does not list their options. Both stage rows carry `-` in `host_variants`, so no offered parameter value selected another arrangement in the recorded runs; a render that differs in a host option no offered parameter selects (such as the source plane count `--src-planes`, the source reader head `--src-head` or the destination route `--ae32-dest` of the `host` column) needs a stage row of its own, not a host variant entry ([Effects provider](../stages.md#effects-provider), host variants rules; INFERRED). No approximate reciprocal instruction is used on the verified render path (MEASURED, `S_Blur` section of [Effects](../effects.md)). No comparison of `S_Blur` differs from the original.

## Checks that run from this repository

Run from the repository root after [Build](../../README.md#build). Scratch files go to `target/demo/blur/`. The device commands need an OpenCL platform such as PoCL; without one `SAPPHIRE_REQUIRE_DEVICE=1` makes them exit 3 (`fx_run` 2) instead of rendering on the CPU.

| check | command | expected result | exit |
|---|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| stages listed | `sapphire stage list --json` filtered by family | `blur_color_autogen temporal 17 []` and `blur_mono_autogen temporal 17 []` | 0 |
| parameters | `fx_run --describe-params Blur_Color_Autogen` (and Blur_Mono_Autogen) | one JSON member with 21 registered parameters | 0 |
| worked renders | `sapphire stage apply` on a 64x32 clip of 3 frames | 98,304 bytes each; `note:` lines on standard error for the values the stage puts on the host grid | 0 |
| plan ops on the CPU | `fx_run ... --plan-text` | `monochrome_a`, `blur`, `postdivide` | 0 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; the 10 `S_Blur` rows among them | 0 |
| conformance | `tools/conformance.py --stage` per stage | L1 `14 passed, 2 failed` (the two 1x1 cases), L3 `33 passed`, `result: fail (exit 1)` | 1 |
| refusals | the refusal block | exit 3 and the messages of [Styles and stages](#styles-and-stages) | 3 |
| crate tests | `cargo test` of the kernel crates | `test result: ok` | 0 |
| executor and evaluator tests | `cargo test -p fx_eval -p fx_exec` (named tests) | `test result: ok`; tests that need stored data print `NOT RUN` and pass | 0 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 ... --device gpu` | `device: blur 3, monochrome_a 3, postdivide 3; fallback: none`, output equal to the CPU | 0 |
| device golden rows | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` | 0 |
| device tests | `cargo test -p fx_exec --test device_neighbourhood --test device_pointwise -- --test-threads=1` | `test result: ok`; `NOT RUN` without a device unless `SAPPHIRE_REQUIRE_DEVICE=1` | 0 |

Build, stages and parameters:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
target/release/sapphire stage list --json | python -I -c "import json,sys; [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in json.load(sys.stdin)['stages'] if s['family'] == 'S_Blur']"
target/release/fx_run --describe-params Blur_Color_Autogen | python -I -c "import json,sys; print({k: len(v) for k, v in json.load(sys.stdin).items()})"
target/release/fx_run --describe-params Blur_Mono_Autogen | python -I -c "import json,sys; print({k: len(v) for k, v in json.load(sys.stdin).items()})"
```

Each command exits 0. The second prints `blur_color_autogen temporal 17 []` and `blur_mono_autogen temporal 17 []`; the last two print the parameter count of each program from its JSON description, `{'Blur_Color_Autogen': 21}` and `{'Blur_Mono_Autogen': 21}` (the 17 offered ones less `cset_opacity`, plus `invert_mask`, `mask_use`, `mocha_project`, `blur_mocha_mask` and `show_mocha_mask_only`).

Worked renders with values the verified comparisons used:

```sh
mkdir -p target/demo/blur
ffmpeg -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/blur/in.raw
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param blur_amount=4.2183990478515625 --param blur_rel=0.573104858,1.25630188 --param brightness=1.36999512 --param mix_with_source=0.337097168 --in target/demo/blur/in.raw --out target/demo/blur/color.raw
target/release/sapphire stage apply blur_mono_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param blur_amount=4.2183990478515625 --param offset_darks=0.628295898 --in target/demo/blur/in.raw --out target/demo/blur/mono.raw
wc -c target/demo/blur/color.raw target/demo/blur/mono.raw
```

ffmpeg must be on `PATH` for the second line. Every command exits 0 and `wc` prints 98304 for each file (3 frames of 64x32 at 16 bytes a pixel). The colour command prints three lines such as ``note: `brightness` 1.36999512 is put on the 16.16 host grid as 1.3699951171875`` (for `blur_rel`, `brightness` and `mix_with_source`) and the mono command one (for `offset_darks`): these parameters are host sliders, and the stage passes their values on the host grid.

The plan of a style on the CPU, without a device (the source words are zeros; the plan does not depend on them):

```sh
head -c 32768 /dev/zero > target/demo/blur/zero.u32
target/release/fx_run Blur_Mono_Autogen --size 64x32 --out target/demo/blur/fx --ae32-src Source=target/demo/blur/zero.u32 --ae32-prior target/demo/blur/zero.u32 --ae32-out target/demo/blur/fx.u32 --geom -0.5,63.5,1,0,-16.5,0,0 --host-frame 0 --ctx-time 0 $(awk -F'\t' '$1=="blur_mono_autogen"{print $10}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' '\n' | grep -v '^--prior-fill' | tr '\n' ' ') $(awk -F'\t' '$1=="blur_mono_autogen"{print $9}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' ' ') --ns-default-unbound --do-postdiv 1 --plan-text target/demo/blur/plan.txt
grep '^NODE' target/demo/blur/plan.txt | grep -E 'kind=(func|auto)' | cut -f3
```

Both exit 0; the last prints `monochrome_a`, `blur` and `postdivide`, one a line. The `fx_run` line passes the stage row's `host` and `fixed` options in the order the stage uses (`--prior-fill` is a stage option and is left out); `--do-postdiv 0` instead is refused with `POSTDIV-MISMATCH` and exit 2.

Golden rows (no ffmpeg needed):

```sh
python -I tools/fx_golden.py
grep -E '^blur_(color|mono)_autogen' tools/fx_golden.tsv
```

The first exits 0 with `fx_golden: 183 cases, 183 equal, 0 differ`; a difference prints `DIFFERS STAGE SIZE`, the styles that differ, and exits 1. The grep lists the 10 rows of the effect, 5 per stage: the plain sizes `64x32` and `50x7`, the threaded rows `64x32t` and `50x7t` (`SAPPHIRE_PAR_MIN=0`, `SAPPHIRE_THREADS=3`, equal to the plain rows) and the special row `40x12s` (infinities, both default NaN patterns, subnormals). The effect has no variant rows. The two `40x12s` rows hold the same hash because the special clip drives every output plane of both stages to one word: the blur spreads the infinities and NaNs of the clip over the whole frame, so each colour plane is one NaN word and each alpha plane one value (MEASURED with the block below). The plain rows tell the styles apart: their hashes differ (`tools/fx_golden.tsv` lines 5 and 10).

```sh
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden as g; open('target/demo/blur/special.raw', 'wb').write(g.synth_special(40, 12, 3, g.SEED))"
target/release/sapphire stage apply blur_color_autogen --width 40 --height 12 --depth 32 --fps 24 --frames 3 --in target/demo/blur/special.raw --out target/demo/blur/special_color.raw
target/release/sapphire stage apply blur_mono_autogen --width 40 --height 12 --depth 32 --fps 24 --frames 3 --in target/demo/blur/special.raw --out target/demo/blur/special_mono.raw
cmp target/demo/blur/special_color.raw target/demo/blur/special_mono.raw
python -I -c "import struct; d = open('target/demo/blur/special_color.raw', 'rb').read(); n = 40 * 12; w = struct.unpack('<%dI' % (len(d) // 4), d); print([len(set(w[k * n:(k + 1) * n])) for k in range(len(w) // n)])"
```

Every command exits 0; `cmp` prints nothing and the last prints a list of twelve 1s (one distinct word in each of the 4 planes of the 3 frames).

Conformance of each stage command (the program path must be absolute):

```sh
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply blur_color_autogen"
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply blur_mono_autogen"
```

Each exits 1 by design: L1 prints `14 passed, 2 failed`, the two `FAIL` lines being the 1x1 cases (`needs a width of 2 or more (the host grid needs two columns)`, exit 3), L3 prints `33 passed`, and the run ends `result: fail (exit 1)`. These are the documented size refusals of [Conformance](../stages.md#conformance).

Refusals:

```sh
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param cset_opacity=1 --in target/demo/blur/in.raw --out target/demo/blur/r1.raw
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param filter=3 --in target/demo/blur/in.raw --out target/demo/blur/r2.raw
target/release/sapphire stage apply blur_mono_autogen --width 1 --height 32 --depth 32 --fps 25/1 --in target/demo/blur/in.raw --out target/demo/blur/r3.raw
target/release/sapphire stage apply blur_mono_autogen --width 64 --height 32 --depth 32 --fps 25/1 --layer To=target/demo/blur/in.raw --in target/demo/blur/in.raw --out target/demo/blur/r4.raw
```

Each exits 3 with no frame written: `SOURCE-VARIANT-MISMATCH`, `bad value for filter`, `needs a width of 2 or more`, `LAYER-INPUT-UNSUPPORTED`.

Kernel crates, executor and evaluator tests:

```sh
cargo test --release --offline -p blur -p blur_coef -p blur_rows -p blur_rows_y -p masked_blur -p monochrome_a -p add_rgb -p interp
cargo test --release --offline -p fx_eval --test t27_unbound_default --test t_fld3_form --test t_brd2_broadcast --test t31_lym1
cargo test --release --offline -p fx_exec --test t27_style_select
cargo test --release --offline -p neutral-handoff --test fx_stage_parity
```

Each exits 0. The blur crates hold no unit tests of their own (the blur kernel is checked through the golden rows, the device self-check and the device tests); `masked_blur` checks its two-NaN picks (`tests/nan_pair.rs`), `monochrome_a` its plane layout. `t27_unbound_default` checks the default host value of `blur_amount`, `t_fld3_form` and `t_brd2_broadcast` the folded `v3_to_v4_if4` gain node of these plans, and `t31_lym1` the layout rows of the registered programs. `t31_lym1` and `t27_style_select` compare with stored data: without `VEC_DIR` they print `NOT RUN` lines (`t27_style_select` returns without a line) and pass. `fx_stage_parity` prints `NOT RUN: VEC_DIR is not set` and passes; with the stored data and an `fx_run` it renders the 382 counted recorded renders of the stages of ten effects through `sapphire stage apply`, the 36 of `S_Blur` among them, and compares them by full SHA-256.

Device (PoCL or a GPU):

```sh
target/release/sapphire stage apply blur_mono_autogen --width 64 --height 32 --depth 32 --fps 25/1 --in target/demo/blur/in.raw --out target/demo/blur/mono_cpu.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply blur_mono_autogen --width 64 --height 32 --depth 32 --fps 25/1 --device gpu --in target/demo/blur/in.raw --out target/demo/blur/mono_gpu.raw
cmp target/demo/blur/mono_cpu.raw target/demo/blur/mono_gpu.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --fps 25/1 --device gpu --param mocha_mask_opacity=0.5 --in target/demo/blur/in.raw --out target/demo/blur/masked_gpu.raw
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_neighbourhood --test device_pointwise -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
```

Every command exits 0 on PoCL. The mono report ends `device: blur 3, monochrome_a 3, postdivide 3; fallback: none` and `cmp` prints nothing; the masked report ends `device: masked_blur 3, postdivide 3; fallback: none`. The first run of a build pays the self-check of every allowlisted op (about a minute or more on PoCL, [Rendering on a device](../guide.md#rendering-on-a-device)); later runs read it from the cache. Without a device the two `sapphire` device commands print `DEVICE-REQUIRED-UNAVAILABLE` and exit 3, and the cargo device tests panic instead of printing `NOT RUN` because of `SAPPHIRE_REQUIRE_DEVICE=1`.

The NaN-exact mutation check of the neighbourhood kernels is an ignored test, run by hand on a device:

```sh
cargo test --release --offline -p fx_exec --lib nan_exact_neighbourhood_source_mutants_are_refused -- --ignored --test-threads=1
```

It exits 0 when every source mutant of the `blur`, `masked_blur`, `sharpen` and `edge_aware_sharpen` kernels is refused by the self-check (`MUTANT ... KILLED` lines); without a device it prints `NOT RUN`.

CI (`.github/workflows/ci.yml`): `cargo test --release` (the default members, among them every `S_Blur` crate but `interp`) and `python -I tools/fx_golden.py` on Linux, macOS and Windows; `cargo test -p fx_eval -p fx_exec -p interp ...` on Linux; and in the PoCL job `cargo test -p fx_exec --lib device`, the device test files and `fx_golden.py` with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1`.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, together with the recorded renders and the stored test data under `VEC_DIR` ([Tests](../effects.md#tests)). Without them `crates/neutral-handoff/tests/fx_stage_parity.rs` and the stored-data tests of `fx_eval` and `fx_exec` print `NOT RUN` and pass. The golden table holds this repository's own renders and cannot say whether the original agrees. The full report with the effect's exclusion tables is identified by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, the verified render path (one band, full frame geometry, constant parameters, one thread, Normal opacity) at the sizes and values of the counted recorded renders that use the stage's host arrangement; the verified parameter values are listed in the `S_Blur` section of [Driving the effect parameters](../parameters-guide.md). Other sizes and values run the same code and are INFERRED.
- Host buffer phase: the row and column passes take their summation order from the address of a scratch buffer modulo 32 bytes once a window holds 64 samples or more, which needs a frame 128 pixels or more wide or high. In the code the phase enters the first window sum of a pass when the pass radius is 63 or more (`s0` in `crates/effects/kernels/blur_rows/src/lib.rs`, `prefix_sum` and `column_window` in `crates/effects/kernels/blur_rows_y/src/lib.rs`). With the radius in pixels of [What it computes](#what-it-computes), a pass reaches it at a radius of 63 pixels with Box, about 126 with Triangle and about 189 with Gauss, on any frame of 128 pixels or more on that axis (INFERRED): a 1920x1080 or 3840x2160 render with a smaller radius is independent of the phase, and a larger one rests on the phase. The kernel takes that phase as an input and the stage passes 0 (`DEFAULT_BASE_PHASE` in `crates/effects/kernels/blur/src/lib.rs`). Above the threshold the output equals the original's when the host's phase equals 0 (INFERRED; [Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- No golden row and no test reaches that path: the golden sizes are below 128 pixels. The repository can check it only for self-consistency, CPU against device, with a frame of 256x256 and a Box radius of 100 (block below); a change to the summation order needs the comparison harness to be judged against the original.
- `ROUTE AVX2-ONLY`: the native code reproduces the AVX2 and FMA tier of the original only.
- NaN: the CPU kernels keep the x86 NaN rules; `blur` and `masked_blur` write their two-NaN picks through `fx_common::nanop`; NaN payload and sign bits are not verified against the original ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- Opacity: only the answer 2 (Normal) renders; 1 and 3 are refused with `SOURCE-VARIANT-MISMATCH` (exit 3).
- Mask: the program's `Mask` input is not offered and stays the constant 1; the tracker mask parameters act on the mask the plan builds without a tracker project ([The tracker mask group](../parameters-guide.md#the-tracker-mask-group)).
- Depths 8 and 16 render (exit 0) through a float conversion that is not verified.
- Keyed values: each frame is a plain run at the value of its keys (MEASURED); values between keys are INFERRED.
- Refusals: the table in [Styles and stages](#styles-and-stages); every label is listed in [Refusal labels](../cli.md#refusal-labels).
- Host values: `sapphire stage apply` puts every `--host-param` value on the 16.16 grid and prints a `note:` line for a value that moves; `HOST-VALUE-OFF-GRID` is an `fx_run` refusal only (the `HOST-VALUE-OFF-GRID` row of [Refusal labels](../cli.md#refusal-labels)).

The phase path, CPU against device (needs ffmpeg and an OpenCL platform):

```sh
ffmpeg -v error -f lavfi -i testsrc=size=256x256:rate=25 -frames:v 1 -f rawvideo -pix_fmt gbrapf32le target/demo/blur/big.raw
target/release/sapphire stage apply blur_color_autogen --width 256 --height 256 --depth 32 --fps 25/1 --param blur_amount=100 --param filter=0 --in target/demo/blur/big.raw --out target/demo/blur/big_cpu.raw
SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply blur_color_autogen --width 256 --height 256 --depth 32 --fps 25/1 --param blur_amount=100 --param filter=0 --device gpu --in target/demo/blur/big.raw --out target/demo/blur/big_gpu.raw
cmp target/demo/blur/big_cpu.raw target/demo/blur/big_gpu.raw
```

Every command exits 0 on PoCL and `cmp` prints nothing.

## Device path

| op | device kernel | NaN exact | falls back to the CPU when |
|---|---|---|---|
| `blur` | `device_ops_neighbourhood::BLUR` (`blur_x_f32`, `blur_y_f32` in `NEIGHBOURHOOD_BLUR`) | yes (`NAN_EXACT_WRITTEN`) | the amount, filter shape, scale or subpixel is not a constant, the tag is outside 1 to 4, or the frame holds more samples than an i32 index |
| `masked_blur` | `MASKED_BLUR` (`NEIGHBOURHOOD_MASKED`) | yes (`NAN_EXACT_WRITTEN`) | the mask is not an image, or the parameters are not constants |
| `monochrome_a` | `MONOCHROME_A` (pointwise) | no | a NaN output (rerun on the CPU, `nan_output`) |
| `add_rgb` | `ADD_RGB` (pointwise) | no | a NaN output |
| `interp` | `INTERP` (pointwise) | no | a NaN output |
| `postdivide` | `POSTDIVIDE` (`pointwise_postdiv_f32`) | yes (`NAN_EXACT_WRITTEN`) | the kernel declines the shape |

No op of the effect has a `CPU_ONLY` entry; the table is empty. MEASURED on PoCL (an OpenCL runtime on the CPU) only: every golden row of both stages, the special rows included, gives the CPU's bytes with `fallback: none` ([Rendering on a device](../cli.md#rendering-on-a-device)), and the device report gives `fallback: none` at 64x32 on the default plans, on the plans with `offset_darks` and `mix_with_source` set and on the tracker mask plan (MEASURED). The `blur` self-check on PoCL reports equal NaN payloads. Speed on PoCL for one 1920x1080 frame: 0.79 / 0.73 s (`blur_color_autogen`) and 0.64 / 0.70 s (`blur_mono_autogen`) against 0.63 / 0.73 s and 0.59 / 0.57 s on the CPU ([Speed](../guide.md#speed)); speed on a GPU is INFERRED.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (every group, and with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` on a device), the crate tests above, `cargo test -p fx_eval -p fx_exec`, the device tests and the mutation check, `python -I tools/test_effect_rules.py`, `python -I tools/gen_docs.py --check` and `python -I tools/check_docs.py`.
- A golden difference means the output bytes of a stage changed. The table holds this repository's own Linux renders, so it says nothing about the original: a change that is meant to keep parity must keep every row, and a change of the output needs the comparison harness to be judged. `python -I tools/fx_golden.py --write` renders every case and rewrites `tools/fx_golden.tsv`; it is used only after an intended and judged output change.
- The blur crates call no platform math library function (`python -I tools/test_effect_rules.py` scans for it; exceptions are rows of `tools/fx_math_allow.tsv`). A two-NaN pick goes through `fx_common::nanop`; a fresh NaN is the x86 default NaN ([Crate layout](../adding-an-effect.md#crate-layout)).
- The row and column passes keep their summation order; the device kernels copy that order (`window sum order` is one of the mutants). A CPU change to `blur`, `masked_blur`, `monochrome_a`, `add_rgb`, `interp` or `postdivide` removes the op from `ALLOWLIST` in `crates/effects/fx_exec/src/device_ops.rs` unless its device kernel changes in the same patch ([Device](../adding-an-effect.md#device)).
- A change that moves an output updates `tools/fx_golden.tsv`, the `S_Blur` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), and `docs/data/effects_status.tsv` when a count changes, then `python -I tools/gen_docs.py`.

## Open items

- `cset_opacity` answers 1 and 3 are refused: no recorded render of the effect holds them, so the stage rows carry no host variant entry (`SOURCE-VARIANT-MISMATCH`, [Refusal labels](../cli.md#refusal-labels)).
- Above the 64 sample window the stage passes host buffer phase 0 only; a host whose allocator gives another phase is not modelled ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- The other code tiers of the original (CPUs without AVX2 and FMA) are not rebuilt (`ROUTE AVX2-ONLY`).
- The program's `Mask` input is not offered by the stages and stays the constant 1.
- No golden row reaches `masked_blur`, `add_rgb` or `interp`, nor the phase dependent windows; a golden variant row at a tracker mask, light or mix value, and a row of 128 pixels or more, would cover them (`tools/fx_golden.py`, `VARIANTS`).
- The buffer phase a host's allocator gives the scratch buffer is not known from this repository; the stage's 0 is an assumption above the threshold (NEEDS-OUTSIDE).
- How the original's other code tiers differ on CPUs without AVX2 and FMA (rounding, summation order) is not measured here (NEEDS-OUTSIDE).
