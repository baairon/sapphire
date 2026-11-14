# Internals of `stage apply blur_motion_autogen` and `stage apply blur_motion_mono_autogen`

This page covers the code of these stages: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. They are the two styles of `S_BlurMotion`. The page also describes in full the motion blur sampler (`blur_warp` over a ripmap from `make_mipmap`) that `S_BlurMoCurves` shares; the [`S_BlurMoCurves` internals](blur-mo-curves.md) page links here for it. The user reference is the `S_BlurMotion` section of [Effects](../effects.md), the `S_BlurMotion` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md).

## What it computes

Motion blur along a camera move. For each output pixel the program computes two positions: the pixel position turned about `center` by `from_rotate`, scaled by `from_z_dist` and moved by `from_shift` (the start of the move), and the same with `to_rotate`, `to_z_dist` and `to_shift` (the end of the move). The `blur_warp` operation averages the source along the straight segment between the two positions, reading a ripmap of the source built by `make_mipmap`, with the edge mode of `wrap` per axis (none, tile or reflect). In the constructors the start position is `R(from_rotate) (xy - center) x from_z_dist + center - from_shift`, with R the turn by the angle, and the end position the same with the `to_` parameters (`eff_blur_motion`, `crates/effects/fx_eval/src/programs_native.rs` line 4414). The pair (`1 - exposure_bias`, `exposure_bias`) is the start and end weight of the segment, and the sampler ramps the weight linearly between them (see [The motion blur sampler](#the-motion-blur-sampler)). `blur_res` runs the sampler at full, half or quarter resolution, and `subpixel` turns sub-pixel sampling on or off. The mono style converts the source to a luma plane and its alpha plane before the ripmap, so the blur runs on two channels instead of four.

## Styles and stages

| style (program) | stage | kind | layers | spool | host variants | stage row |
|---|---|---|---|---|---|---|
| Blur_Motion_Autogen | `stage apply blur_motion_autogen` | temporal | none | false | `cset_opacity` 1 and 3 | `crates/neutral-handoff/data/fx_stages.tsv` line 22 |
| Blur_Motion_Mono_Autogen | `stage apply blur_motion_mono_autogen` | temporal | none | false | `cset_opacity` 1 and 3 | `crates/neutral-handoff/data/fx_stages.tsv` line 23 |

Both stages offer the same 12 parameters (`crates/neutral-handoff/data/fx_params.tsv` lines 294 to 305 and 306 to 317): `blur_res`, `wrap`, `subpixel`, `center`, `from_z_dist`, `from_rotate`, `from_shift`, `to_z_dist`, `to_shift`, `to_rotate`, `exposure_bias` and `cset_opacity`. `from_shift` and `to_shift` take host values (`--host-param`), the opacity answer goes through `--cset Opacity`, and the others are record values (`--set`). The registered programs hold 27 parameters each (`fx_run --describe-params`); the rest stay at their registered defaults or are fixed by the stage.

The two styles are the two answers of the Effect popup. The stage of each style runs that style only and offers no popup for it. `fx_run` judges a declared Effect answer before anything else (`crates/effects/fx_exec/src/style.rs`, `STYLE_TABLE` and `select`): a run of one style's program with the popup set to the other style is refused with `STYLE-SELECT-MISMATCH`, a program count that differs from the table with `STYLE-SELECT-COUNT-MISMATCH`, and an answer outside the table with `STYLE-INDEX-OUT-OF-TABLE` (exit 2 from `fx_run`; a stage does not reach them).

The fixed column of both rows binds the mask parameters (`blur_mask`, `blur_mocha_mask`, `shift_mocha_mask`, `invert_mask`, `mask_use`) and sets `show_mocha_mask_only` off; the mask input is not connected and mocha is off. These parameters are not offered: `--param show_mocha_mask_only=1` ends with exit 3 (`unknown param`).

Refused answers and values, each before any frame is written (MEASURED on this tree, commands below):

| input | result |
|---|---|
| `--param cset_opacity=4` | exit 3, `bad value for` (`cset_opacity`: 4 is outside 1 to 3) |
| `--param blur_res=3` | exit 3, `bad value for` (`blur_res`: 3 is outside 0 to 2) |
| `--layer To=PATH` | exit 3, `LAYER-INPUT-UNSUPPORTED` |
| a frame 1 pixel wide | exit 3, `needs a width of 2 or more` |
| a keyed value without `--fps` | exit 2, `keyframes need --fps` |
| an id the stage does not offer | exit 3, `unknown param` |

All three opacity answers render on both stages: answer 2 runs the `host` column, and answers 1 and 3 run the `host_variants` entries of the row (another source plane count, reader head, destination planes and class cells for answer 1, another reader head for answer 3). These options describe the host's input record of the run, not the image: `--src-planes` is the plane count of the source image, `--src-head` the reader head that converts the host's words, `--ae32-dest PLANES,MODE` the destination planes and their mode, `--ctx-1ae` a word of the host context, `--class-cells` the bytes of the style's input-class record and `--class3-record` the counts of the three-plane source route (`crates/effects/fx_exec/src/fxrun.rs` lines 2143 to 2159). The runner checks the derivable ones against the opacity answer (`SOURCE-VARIANT-MISMATCH`, `MODE-BYTE-MISMATCH`). The entries were taken from the original's recorded runs at the answers 1 and 3 ([Effects provider](../stages.md#effects-provider), "Host variants"); those records are outside the repository. The terms are gathered in [Effect internals](README.md).

## Code map

| part | path | role |
|---|---|---|
| style constructors | `crates/effects/fx_eval/src/programs_native.rs`: `eff_blur_motion` (line 4414), `eff_blur_motion_autogen` (4481), `eff_blur_motion_mono` (4550), `eff_blur_motion_mono_autogen` (4629), `layout_blur_motion_autogen` (4946), `layout_blur_motion_mono_autogen` (4984) | the program graphs and layout rows of both styles; the `_Autogen` wrapper adds the mask and mocha group around the inner program |
| style facts | `crates/effects/rtdata/src/styles/blur_motion.rs`, `crates/effects/rtdata/src/styles/blur_motion_mono.rs` (registered in `crates/effects/rtdata/src/styles/mod.rs` lines 26, 27, 60 and 61) | registered parameters, ranges, input indices of both programs |
| class facts | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS` (line 7), `IMG_ONLY_ARGS` (112), `INT_TYPED_ARGS` (134), `COMBOS` (168), `COMBOS_EXEC` (1209); the `blur_warp` rows at lines 18, 114, 137 to 139, 333 and 1374 | flag words, argument kinds and combination tables of `blur_warp`, `make_mipmap`, `rotate`, `monochrome_a`, `postdivide` and the arithmetic classes |
| style selection | `crates/effects/fx_exec/src/style.rs` (`STYLE_TABLE` line 35, `style_program` 47, `select` 62) | the Effect popup rule and the program of each style |
| `blur_warp` | executor `blur_warp_k` in `crates/effects/fx_exec/src/kernels2.rs` line 156; crates `crates/effects/kernels/blur_warp`, `blur_warp_1ch`, `blur_warp_2ch`, `blur_warp_3ch`, `blur_warp_4ch`, `blur_warp_point`, `libm_fmod` | the segment sampler; see [The motion blur sampler](#the-motion-blur-sampler) |
| `make_mipmap` | executor `make_mipmap_k` in `crates/effects/fx_exec/src/kernels2.rs` line 114; crate `crates/effects/kernels/make_mipmap` (`mm2.rs`) | the ripmap of the source the sampler reads |
| `rotate` | executor `rotate_k` in `crates/effects/fx_exec/src/kernels2.rs` line 901; crate `crates/effects/kernels/rotate2d` with `libm_sincos` | turns the pixel offsets about `center` when a rotation is set |
| `monochrome_a` (mono style) | executor `monochrome_a_k` in `crates/effects/fx_exec/src/kernels2.rs` line 558; crate `crates/effects/kernels/monochrome_a` | luma plane plus the alpha plane |
| `+`, `-`, `*` | executor `binary_k` in `crates/effects/fx_exec/src/kernels.rs` line 197 (dispatch line 409); crate `crates/effects/kernels/ops_arith` | the position arithmetic |
| `postdivide` | executor `postdivide_k` in `crates/effects/fx_exec/src/kernels2.rs` line 1375; `crates/effects/fx_exec/src/postdiv.rs`; inserted by the compiler (`crates/effects/fx_eval/src/compile.rs` line 1196) | divides the colour planes by alpha at the end of the opacity route |
| downsampled block (`blur_res` 1 and 2) | `crates/effects/fx_exec/src/exec.rs` `execute_downsampled` line 355; `crates/effects/fx_exec/src/downsampled.rs`; factor in `crates/effects/fx_eval/src/cpuplan.rs` line 84 | halves the inputs once or twice, runs the body on the small planes, doubles the result back |
| default xy writer | `default_xy` in `crates/effects/fx_exec/src/kernels2.rs` line 50; placement option `--xy-base-list` | the plane of pixel positions the program reads as `xy` |
| executor lists | `SUPPORTED` in `crates/effects/fx_exec/src/kernels.rs` line 10, `WIRED` in `crates/effects/fx_exec/src/kernels2.rs` line 13 | every operation above is listed in one of them |
| device entries | `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS` line 71, `ALLOWLIST` 163, `NAN_EXACT` 169, `NAN_EXACT_WRITTEN` 180, `NAN_EXACT_COMPILED` 192, `CPU_ONLY` 205); `device_ops_ripmap.rs` (`make_mipmap`, `blur_warp`), `device_ops_warp.rs` (`rotate`), `device_ops_pointwise.rs` (`+`, `-`, `*` at lines 1961 to 1963, `POSTDIVIDE` 2117, `MONOCHROME_A` 2286) | the device kernels and their self-check cases |
| OpenCL sources | `crates/effects/fx_device/src/kernels_ripmap.rs` (`RIPMAP_MIPMAP`, `RIPMAP_BLUR_WARP`), `kernels_warp.rs` (`WARP_ROTATE`), `kernels_pointwise.rs`; registered in `crates/effects/fx_device/src/kernels.rs` (`SOURCES` line 94, `PROGRAM_PARTS` 137) | the device code |
| stage rows | `crates/neutral-handoff/data/fx_stages.tsv` lines 22 and 23; `crates/neutral-handoff/data/fx_params.tsv` lines 294 to 317 | host arrangement, placement, fixed options, host variants, offered parameters |
| stage runner | `crates/neutral-handoff/src/fxstage.rs` | builds the runner arguments per frame, serves the writer phase list (`parse_xy_writers` line 857) and the postdivide value |
| golden rows | `tools/fx_golden.tsv` lines 12 to 16 and 22 to 26 | five rows per stage: `40x12s`, `50x7`, `50x7t`, `64x32`, `64x32t` |
| tests | see [Checks that run from this repository](#checks-that-run-from-this-repository) | |

No computed tables: the sampler, the ripmap and the arithmetic read no table. The sine and cosine of `rotate` come from `libm_sincos`, whose tables are computed at first use (the "No external data" section of [Effects](../effects.md)).

The ops of the compiled plans are MEASURED with the device report (`SAPPHIRE_DEVICE_REPORT=1`, commands below). Per frame, on a 64x32 clip: `blur_motion_autogen` with `center` set runs `*` 1, `+` 2, `-` 2, `blur_warp` 1, `make_mipmap` 1 and `postdivide` 1; a rotation (`from_rotate` set) adds `rotate` 1; the mono style adds `monochrome_a` 1. At the default parameters the plan holds `*`, `blur_warp`, `make_mipmap` and `postdivide` (and `monochrome_a` on the mono style): the other arithmetic folds to constants at compile time. With the mask input unconnected and mocha off, the mask and mocha arms of the `_Autogen` wrapper fold away and no mask node runs.

### The motion blur sampler

This part is shared with `S_BlurMoCurves`.

- `make_mipmap` builds the ripmap the sampler reads. Its third argument, the filter, is the constant 0 in every plan of these stages (constructor lines 428, 1415, 4459 and 4607): at filter 0 the ripmap holds one level, the source plane with padding rows and columns written for the edge mode of `wrap` (`build_m`, `crates/effects/kernels/make_mipmap/src/lib.rs` line 347, returns after the base pads at line 352; `alloc`, `crates/effects/kernels/make_mipmap/src/mm2.rs` line 124, allocates one level in x and y). A non-zero filter builds the full grid in which plane (l, j) is the input halved l times in x and j times in y, ceil(log2 size) + 2 levels per axis (`level_count`, line 97); the programs of `S_DistortRGB`, `S_WarpPerspective` and `S_WarpTransform` pass their `filter` parameter there, and those of `S_SwishPan` and `S_Retime` the constant 1 (read from the constructors). In these plans the ripmap is read only by `blur_warp`, so no level is chosen: every sample reads level (0, 0).
- `blur_warp` takes the ripmap, the two segment end points (each a constant pair or a two-channel image of per-pixel positions), the pair of end weights, the scale of the result, the edge modes, the sub-pixel flag and the resolution. The executor calls the body once per output row, with a count equal to the frame width (`blur_warp_k`); the resolution argument is not read there, since `blur_res` acts through the downsampled block.
- Per output sample the body converts the start point to pixel coordinates and wraps it by the edge mode (none; tile, through `libm_fmod`; reflect, by the parity of the integer quotient), and computes the end point from the difference of the two points (`wrap`, `s2_x`, `s3_y`, `crates/effects/kernels/blur_warp_1ch/src/lib.rs` lines 187 to 230). When both points are the same pixel position it reads that one point (`blur_warp_point::render_point`) and multiplies it by the scale (line 271). Otherwise it walks the segment along its major axis, one sample per pixel column (x-major, `x_major_sample`, `crates/effects/kernels/blur_warp_1ch/src/p2.rs` line 260) or row (y-major, `y_major_sample`, line 45) the segment crosses, at most 1,000 samples (lines 305 and 394), with the minor coordinate stepped along the line. The weight of sample k is the start weight plus k times (end weight - start weight) / n for n samples (line 294); with `subpixel` on, the first and last samples are weighted by the fraction of their pixel the segment covers and the minor coordinate keeps its fraction, with `subpixel` off the walk steps whole pixels (the two branches of `x_major_sample`). The result is the scale times the weighted sum over the sum of the weights (line 464). These statements are read from the kernel code (INFERRED); the crates' own tests check the arithmetic of each section.
- The crate `blur_warp` is the class entry: it reads the class object and calls one of four bodies by the channel count of the ripmap. `blur_warp_1ch` is the one-channel body and also holds the prologue the four bodies share; `blur_warp_2ch`, `blur_warp_3ch` and `blur_warp_4ch` are the two, three and four channel bodies written as differences against it. `blur_warp_point` is the single point routine they share, and `libm_fmod` is the double precision remainder of the tile and reflect modes.
- The colour style runs the four-channel body and the mono style, whose ripmap holds the luma and alpha planes of `monochrome_a`, the two-channel body.
- On the device, `blur_warp` runs one work item per output sample (the samples are independent) and `make_mipmap` keeps the ripmap as one device buffer that the device sampler reads in place.

## Render flow

From `sapphire stage apply blur_motion_autogen` (or `blur_motion_mono_autogen`) to the output planes:

1. The stage reads the row of `fx_stages.tsv` and the offered parameters of `fx_params.tsv`, checks every given value against the range `fx_run --describe-params` prints (exit 3 outside it), refuses `--layer` (exit 3) and a width under 2 (exit 3), and reads keyed values (keys need `--fps`).
2. For each frame it hands the runner, in memory, the input frame as 'ae32' words (at depth 32 a byte permutation of the planes G, B, R, A), the options of the `host` column (or of the `host_variants` entry of the frame's `cset_opacity` answer), the placement of the frame size, the options of the `fixed` column, `--size` and `--geom` from the frame, `--host-frame` and `--ctx-time` from the frame index, and the offered parameters (`--set`, `--host-param`, `--cset`).
3. Placement: the row holds entries for 64x32 and 50x7; other sizes take the 64x32 entry. Each entry gives the downsample phase (`--ds-phase`) and one phase list per default-xy writer count (`--xy-base-list COUNT:B,...`). The stage passes the list for the writer count of the compiled plan: one writer at `blur_res` 1 and 2, two otherwise. A writer count the entry does not list ends with `XY-BASE-LIST-UNSERVED` (exit 3).
4. The runner checks a declared Effect answer (`style.rs`), compiles the program built by the constructor of the style (`fx_eval`), folds the constant arms (the mask group, the zero rotations, the parameters left at their defaults), and adds `postdivide` for the opacity route. The stage sends `--do-postdiv 0` and, when the runner refuses with `POSTDIV-MISMATCH`, takes the derived value from the run's result and repeats the frame (the "Two inputs that the runner reveals" paragraph of [Effects provider](../stages.md#effects-provider)).
5. The plan runs its nodes in dependency order: the default xy plane, the position arithmetic (`-`, `*`, `+`, and `rotate` when a rotation is set), `monochrome_a` on the mono style, `make_mipmap` of the source, `blur_warp`, `postdivide`. At `blur_res` 1 or 2 the body runs inside the downsampled block: each input plane is halved once or twice (`downsampled.rs`, at the downsample phase of the placement), the body runs on the small planes, and the result is doubled back to the frame size.
6. Threading: none of these operations splits its rows into bands (only `warp`, `warp_rel` and `blur` use `crates/effects/fx_exec/src/par.rs`), so `SAPPHIRE_THREADS` and `SAPPHIRE_PAR_MIN` do not change the work of these plans (INFERRED from the code); the threaded golden rows equal the plain rows (MEASURED).
7. Device: with `--device gpu`, `gpu=N` or `auto`, or `SAPPHIRE_DEVICE`, each node whose operation is allowlisted and passed its self-check runs on the device; a node the kernel declines, and a NaN output of an operation off `NAN_EXACT` (here `blur_warp`, `rotate` and `monochrome_a`), runs again on the CPU, so the bytes are the CPU's.
8. The output words are relaid as frames/2 planes and written. The frame time is not read by these programs, and no other frame is requested: output frame N depends on input frame N only.

From `fx_run PROGRAM`: the runner takes the same options directly (the full list is the header of `crates/effects/fx_exec/src/fxrun.rs`; `crates/effects/fx_exec/src/bin/fx_run.rs` is the wrapper that adds `--device`); the host context options are required and have no default, and the stage fills them for the verified render path. `fx_run --describe-params PROGRAM` is the only `fx_run` form a user needs; rendering needs only `sapphire`.

## How parity is judged

An effect counts as rebuilt when every registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means): program text, builtin classes, the data it reads, literals, compiled plan, combination tables and execution, each against the original running in an emulator.

| figure | `S_BlurMotion` | source |
|---|---|---|
| verified comparisons | 88 | `docs/data/effects_status.tsv` line 4 |
| recorded renders | 352 | `docs/data/effects_status.tsv` line 4 |
| re-rendered units | 352 | `docs/data/effects_status.tsv` line 4 |
| byte identical through the stages | 232 of 352 (112 of 168 for Blur_Motion_Autogen, 120 of 184 for Blur_Motion_Mono_Autogen) | README.md, "Effects and stages"; the table of stage renders in [Effects provider](../stages.md#effects-provider) |

Of the 88 comparisons, 46 are of the colour style (22 at 64x32, 24 at 50x7, four of them with the Effect popup set to the mono style, which renders through Blur_Motion_Mono_Autogen) and 42 of the mono style (20 at 64x32, 22 at 50x7). No comparison differs from the original. There are no control renders: the 352 recorded renders are the 352 counted ones. All 352 were rendered again through `fx_run` at their recorded buffer phases and equal the recorded renders, which equal the original. Through the stages, the 232 that match the stage's host arrangement equal the original; the other 120 were recorded at buffer phases the stage does not pass (another `--ds-phase` or `--xy-base-list`), and through the stage they were compared with `fx_run` run at the stage's own phases, a check of the stage against the runner, not against the original. Through `sapphire stage apply` those 120 are not claimed equal to the original (the `S_BlurMotion` section of [Effects](../effects.md)). At 50x7 with `blur_res` Half or Quarter the stage's outputs equal the original's at every recorded placement (16 of 16, MEASURED); the stage uses the 64x32 placement there, and that it is the placement of a real host at 50x7 is INFERRED. The opacity answers 1, 2 and 3 are verified for both styles. The counted routes run no approximate reciprocal instruction: the sampler, the rotation, the mono conversion and `postdivide` hold none, and the four in `make_mipmap` of the original sit in the logarithm of its level count, which only a non-zero filter computes (see the sampler section; this repository computes that logarithm with exact operations, `log_over_ln2`, `crates/effects/kernels/make_mipmap/src/mm2.rs` line 80). The emulator's own result for those instructions is outside the repository. The figures are those of the `S_BlurMotion` section of [Effects](../effects.md).

## Checks that run from this repository

Run from the repository root after the README "Build" step. Scratch files go to the git ignored `target/demo/blur-motion/`. ffmpeg is needed only to make the test clip; the device rows need an OpenCL platform such as PoCL.

| check | command | expected result and exit code |
|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | exit 0 |
| stage list | `sapphire stage list --json` filtered to the two stages | two lines, `temporal 12 0`; exit 0 |
| parameters | `fx_run --describe-params Blur_Motion_Autogen Blur_Motion_Mono_Autogen` | 27 entries per program; exit 0 |
| worked renders | `sapphire stage apply` on a 3 frame 64x32 clip | exit 0, 98,304 bytes out |
| opacity answers | `cset_opacity` 1 and 3, then 4 | exit 0, exit 0, exit 3 |
| refusals | `blur_res=3`, `--layer`, width 1, keyed value without `--fps`, `show_mocha_mask_only` | exit 3, 3, 3, 2, 3 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; exit 0 |
| golden rows on a device | the same with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` | the same line; exit 0 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 ... --device gpu`, then `cmp` with the CPU output | `fallback: none`; exit 0 |
| NaN rerun | the special clip of `tools/fx_golden.py` on the device | `fallback: blur_warp 3 (nan_output 3)`, hash of the `40x12s` row; exit 0 |
| conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply STAGE"` | 2 `FAIL` lines (the 1x1 cases), L3 33 passed; exit 1 |
| kernel crates | `cargo test` of the sampler, arithmetic and colour crates | exit 0 |
| executor tests | `cargo test -p fx_exec` for style selection and the xy writers | exit 0 |
| device tests | `cargo test -p fx_exec` device files, one thread | exit 0 |

Make the clip and list the stages:

```sh
mkdir -p target/demo/blur-motion
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/blur-motion/in.raw
target/release/sapphire stage list --json | python -I -c "import json, sys; d = json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), len(s['layers'])) for s in d['stages'] if s['id'] in ('blur_motion_autogen', 'blur_motion_mono_autogen')]"
target/release/fx_run --describe-params Blur_Motion_Autogen Blur_Motion_Mono_Autogen | python -I -c "import json, sys; d = json.load(sys.stdin); [print(p, len(v)) for p, v in d.items()]"
```

Each command exits 0. The clip is 98,304 bytes (3 frames of 64x32 at 16 bytes a pixel). The stage list prints `blur_motion_autogen temporal 12 0` and `blur_motion_mono_autogen temporal 12 0`; the parameter list prints `Blur_Motion_Autogen 27` and `Blur_Motion_Mono_Autogen 27`. Without ffmpeg on `PATH` the second command fails and the renders below have no input.

Worked renders, the opacity answers and the refusals:

```sh
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param center=0.5625,-1.0625 < target/demo/blur-motion/in.raw > target/demo/blur-motion/out.raw
target/release/sapphire stage apply blur_motion_mono_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param center=0.5625,-1.0625 --param from_rotate=-21.0 < target/demo/blur-motion/in.raw > target/demo/blur-motion/mono.raw
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=1 < target/demo/blur-motion/in.raw > target/demo/blur-motion/o1.raw
target/release/sapphire stage apply blur_motion_mono_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=3 < target/demo/blur-motion/in.raw > target/demo/blur-motion/o3.raw
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24 --param cset_opacity=4 < target/demo/blur-motion/in.raw > target/demo/blur-motion/r.raw
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24 --param blur_res=3 < target/demo/blur-motion/in.raw > target/demo/blur-motion/r.raw
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24 --layer To=target/demo/blur-motion/in.raw < target/demo/blur-motion/in.raw > target/demo/blur-motion/r.raw
target/release/sapphire stage apply blur_motion_autogen --width 1 --height 32 --depth 32 --fps 24 < target/demo/blur-motion/in.raw > target/demo/blur-motion/r.raw
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --param "from_rotate=0@0;-17@2" < target/demo/blur-motion/in.raw > target/demo/blur-motion/r.raw
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24 --param show_mocha_mask_only=1 < target/demo/blur-motion/in.raw > target/demo/blur-motion/r.raw
```

The first four exit 0 and write 98,304 bytes each; the mono render prints a `note:` line on standard error saying that the `from_rotate` value is put on the 16.16 host grid as -21. The fifth to tenth exit 3 (`bad value for`, 4 is outside 1 to 3), 3 (`bad value for`, 3 is outside 0 to 2), 3 (`LAYER-INPUT-UNSUPPORTED`), 3 (`needs a width of 2 or more`), 2 (`keyframes need --fps`) and 3 (`unknown param`), and write no frame.

Golden rows, CPU and device. The table holds five rows for each stage; the second command lists them:

```sh
python -I tools/fx_golden.py
awk -F'\t' '$1 == "blur_motion_autogen" || $1 == "blur_motion_mono_autogen"' tools/fx_golden.tsv
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

The first and third commands end with `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0 (the device run on PoCL); the second prints 10 rows and exits 0. `tools/fx_golden.py` renders every stage of `fx_stages.tsv` and has no stage filter. The table has no variant row for these stages: no golden row reaches the `cset_opacity` 1 and 3 entries, which do change the output (on the golden clip the answers 1, 2 and 3 give three different hashes, MEASURED). On the golden clip at the default parameters the plans run `*`, `blur_warp`, `make_mipmap` and `postdivide`, and `monochrome_a` on the mono style (MEASURED with the device report); no golden row reaches `+`, `-` or `rotate` (they need `center`, a shift or a rotation), the downsampled block (`blur_res` 1 and 2), an edge mode of `wrap` other than none, or `subpixel` 0. The special rows reach the NaN rerun of `blur_warp`. Without an OpenCL platform every case of the third command is refused (`DEVICE-REQUIRED-UNAVAILABLE`), the tool prints `fx_golden: 183 cases, 0 equal, 183 differ` and exits 1.

Device report, CPU equality, and the NaN rerun on the special clip of `tools/fx_golden.py` (needs an OpenCL platform):

```sh
target/release/sapphire stage apply blur_motion_mono_autogen --width 64 --height 32 --depth 32 --fps 24 --param center=0.5625,-1.0625 --param from_rotate=-21.0 < target/demo/blur-motion/in.raw > target/demo/blur-motion/cpu.raw
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply blur_motion_mono_autogen --width 64 --height 32 --depth 32 --fps 24 --param center=0.5625,-1.0625 --param from_rotate=-21.0 --device gpu < target/demo/blur-motion/in.raw > target/demo/blur-motion/dev.raw
cmp target/demo/blur-motion/cpu.raw target/demo/blur-motion/dev.raw
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden as g; sys.stdout.buffer.write(g.synth_special(g.SPECIAL_SIZE[0], g.SPECIAL_SIZE[1], g.FRAMES, g.SEED))" > target/demo/blur-motion/special.raw
SAPPHIRE_REQUIRE_DEVICE=1 SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply blur_motion_autogen --width 40 --height 12 --depth 32 --fps 24 --frames 3 --device gpu < target/demo/blur-motion/special.raw > target/demo/blur-motion/special_dev.raw
python -I -c "import hashlib; print(hashlib.sha256(open('target/demo/blur-motion/special_dev.raw', 'rb').read()).hexdigest())"
```

Every command exits 0 and `cmp` prints nothing. The last report line of the device render is `device: * 3, + 6, - 6, blur_warp 3, make_mipmap 3, monochrome_a 3, postdivide 3, rotate 3; fallback: none`. On the special clip it is `device: * 3, make_mipmap 3, postdivide 3; fallback: blur_warp 3 (nan_output 3)`, and the printed hash equals the `40x12s` row of `blur_motion_autogen` in `tools/fx_golden.tsv`. Without a device the two device renders exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`.

Conformance of each stage (L1 and L3):

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply blur_motion_autogen"
python -I tools/conformance.py --stage "target/release/sapphire stage apply blur_motion_mono_autogen"
```

Each prints `L1 stage matrix FAIL 14 passed, 2 failed` and `L3 failure injection pass 33 passed` and exits 1. The two `FAIL` lines are the documented 1x1 size refusals (`needs a width of 2 or more`, exit 3); see [Conformance](../stages.md#conformance).

Crate, executor and device tests (cargo):

```sh
cargo test --release --offline -p blur_warp -p blur_warp_1ch -p blur_warp_2ch -p blur_warp_3ch -p blur_warp_4ch -p blur_warp_point -p libm_fmod -p libm_sincos -p monochrome_a -p ops_arith
cargo test --release --offline -p make_mipmap -p rotate2d
cargo test --release --offline -p fx_exec --test t27_style_select --test t_xyh1_head_form --test t_xwp2_writers
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_ripmap --test device_warp --test device_pointwise -- --test-threads=1
cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
```

Each exits 0. The first list are default members (a plain `cargo test --release` runs them too); `make_mipmap` and `rotate2d` are outside the default members and run in the Linux step of CI with `fx_eval` and `fx_exec`. `t27_style_select` checks the Effect popup rule on `fx_run` (`STYLE-SELECT-MISMATCH`, `STYLE-SELECT-COUNT-MISMATCH`, `STYLE-INDEX-OUT-OF-TABLE`, both styles tabled); `t_xyh1_head_form` checks the head form of the default-xy writers; `t_xwp2_writers` checks the writers of the placement model on Blur_Motion_Autogen and, without the stored program corpus under `VEC_DIR`, prints a `NOT RUN` line and passes (cargo shows the line with `-- --nocapture`). The `libm_sincos` test that compares its computed tables with stored copies prints `NOT RUN` the same way. `device_ripmap` runs the self-check of `make_mipmap` and `blur_warp` and feeds a device-built ripmap to the device samplers, `device_warp` covers `rotate` and `device_pointwise` covers `+`, `-`, `*`, `postdivide` and `monochrome_a`. Without an OpenCL platform the device tests print `NOT RUN` and pass when `SAPPHIRE_REQUIRE_DEVICE` is unset; with `SAPPHIRE_REQUIRE_DEVICE=1` they fail instead.

CI (`.github/workflows/ci.yml`): `cargo test --release` (line 77), `python -I tools/fx_golden.py` on Linux, macOS and Windows (line 88), the non-default crates on Linux (line 101), `tools/test_effect_rules.py` (line 132), and in the PoCL job the device library tests, the device test files and the device golden run (lines 191 to 193).

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders and the stored test data. From the repository alone the checks above confirm that the code renders its own recorded hashes (`tools/fx_golden.tsv`), that the device equals the CPU, and that the refusals hold; they cannot show that the original agrees. The 88 comparisons, the 352 recorded renders and their re-render through `fx_run` and the stages are not in the repository. Tests that read stored data print `NOT RUN` without `VEC_DIR` (here `t_xwp2_writers`). The full report is kept outside the repository; its SHA-256 is printed in the "Evidence" section of [Effects](../effects.md).

## Verified scope and limits

- Sizes: 64x32 and 50x7, at a pixel aspect of 1.0 (the stage passes 1.0). Other sizes run the same code with the 64x32 placement (INFERRED).
- Frame time and rate: not read; `--fps` is needed only for keyed values.
- Host buffer phase: the default xy writer (rows of 29 pixels or more) and the downsampled planes (small planes 31 pixels or more wide) take their order from a host buffer phase above these thresholds. The stage passes the recorded placement (`--ds-phase` and `--xy-base-list` of the row); at another phase the original's own output can differ ("Limits common to the effects" in [Effects](../effects.md)). The "blur windows" that [Effects](../effects.md) names for the mono style belong to the `blur` of the mask in `eff_blur_motion_mono` (line 4550); with the mask input unconnected that arm folds away and no stage plan holds a `blur` node (MEASURED with the device report), so a stage never reaches them. `monochrome_a` chooses its vector peel from a destination phase for planes of 797 samples or more (`split`, `crates/effects/kernels/monochrome_a/src/lib.rs` line 62); the executor passes phase 0 (`crates/effects/fx_exec/src/kernels2.rs` line 570), and every section computes the same per sample formula (the crate's header), so its output does not depend on that phase (INFERRED).
- A 1920x1080 frame is above the default-xy and downsampled-plane thresholds. The stage passes the phases of its 64x32 entry (16), which are recorded phases at 64x32 and 50x7 and INFERRED for a real host at 1920x1080; such a render is not verified and not phase independent.
- NaN: the CPU kernels keep the x86 NaN rules; `blur_warp`'s NaN words are the compiled CPU instruction's picks, and the device reruns its NaN outputs on the CPU. NaN payload and sign bits are outside the verified cases.
- Routes not rebuilt: `ROUTE AVX2-ONLY` (the tier of CPUs with AVX2 and FMA only); the mask input and mocha (the stage fixes the mask off); a real host's placement at sizes other than the recorded ones.
- Refusals: the table under [Styles and stages](#styles-and-stages); `SOURCE-VARIANT-MISMATCH` does not apply, since both rows carry entries for the answers 1 and 3. The labels are listed in [Refusal labels](../cli.md#refusal-labels).
- INFERRED: other sizes, parameter values, host buffer phases and keyed values between keys run the same code path and are not measured.

## Device path

| op | device kernel | `NAN_EXACT` | NaN outputs |
|---|---|---|---|
| `blur_warp` | `ripmap_blur_warp_f32` (`kernels_ripmap.rs`) | no | rerun on the CPU |
| `make_mipmap` | the `ripmap_mm_*` kernels (`kernels_ripmap.rs`) | yes, `NAN_EXACT_COMPILED` | kept on the device after an equal self-check |
| `rotate` | `warp_rotate_f32` (`kernels_warp.rs`) | no | rerun on the CPU |
| `monochrome_a` | pointwise (`kernels_pointwise.rs`) | no | rerun on the CPU |
| `+`, `*` | `pointwise_bin_f32` | yes, `NAN_EXACT_COMPILED` | kept after an equal self-check |
| `-` | `pointwise_bin_f32` | yes, `NAN_EXACT_WRITTEN` | kept after an equal self-check |
| `postdivide` | `pointwise_postdiv_f32` | yes, `NAN_EXACT_WRITTEN` | kept after an equal self-check |

No operation of these plans has a `CPU_ONLY` entry (the table is empty). A node falls back to the CPU when its kernel declines the call (`unsupported`), when the operation failed its self-check on the device in use (`not_allowlisted`), when the device budget would be exceeded (`memory`), on a driver error (`opencl_error`) and for a NaN output of an operation that keeps the CPU rerun (`nan_output`). MEASURED on PoCL only: every node on the device with `fallback: none` on the plain and threaded golden rows of both stages and on all 352 counted recorded renders through `fx_run` (1,376 and 1,704 device nodes, the same at threads 1, 3 and 12, from the "Device" paragraph of the `S_BlurMotion` section of [Effects](../effects.md)); on the special golden rows `blur_warp` reruns 3 nodes, with the CPU's bytes. Speed on a real GPU is INFERRED. The timing rows in [Speed](../guide.md#speed), one 1920x1080 frame: `blur_motion_autogen` 6.35 / 6.93 s on the CPU and 1.34 / 1.43 s on PoCL, `blur_motion_mono_autogen` 3.95 / 3.93 s and 1.05 / 1.25 s.

## Changing the code safely

- Gates that stay green: `python -I tools/fx_golden.py` (all four groups, also on a device), the crate and executor tests above, `python -I tools/test_effect_rules.py`, the device self-check and device tests on PoCL, the NaN tests as CI runs them, `python -I tools/gen_docs.py --check` and `python -I tools/check_docs.py`.
- The NaN tests of these crates: `crates/effects/kernels/libm_fmod/tests/nan_x86.rs` (a default member: `cargo test --release` runs it on Linux, macOS and Windows, `.github/workflows/ci.yml` line 77), `crates/effects/kernels/rotate2d/tests/nan_x86.rs` and `crates/effects/fx_exec/tests/nan_x86.rs` (a ripmap from infinite samples) in the Linux step (line 101), and the special golden rows on all three systems (line 88). The macOS job is the one arm64 runner (comment at line 103); there the default members, the golden rows and `t_random_sawtooth_nan` (line 107) run, while the Linux-only NaN tests run on x86 only.
- A golden difference means that the output bytes of a stage changed. The table holds this repository's own renders, so the repository alone cannot tell whether the original agrees with the new bytes. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` with the current renders; it is used only after a deliberate change whose new output was checked by other means, never as a check.
- The CPU kernels are the reference. A change to `blur_warp`, `make_mipmap`, `rotate2d`, `monochrome_a` or `ops_arith` keeps the operation order, `mul_add` exactly where the code fuses, the x86 operand order of min and max, and NaN picks through `fx_common::nanop` or the `fx_common::x86` operations; it calls no platform math function (`fx_eval::detmath` instead; `tools/test_effect_rules.py` scans for them). A CPU parity change removes the operation's `ALLOWLIST` entry unless its device kernel changes in the same patch ("Device" in [Adding an effect](../adding-an-effect.md)).
- A change to the stage rows (`fx_stages.tsv`, `fx_params.tsv`) changes the host arrangement and is checked by the golden rows and the stage tests of `crates/neutral-handoff`.
- Docs and data to update with a change: the `S_BlurMotion` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), `docs/data/effects_status.tsv` for new comparisons, the timing rows of [Speed](../guide.md#speed) when they change, then `python -I tools/gen_docs.py`.

## Open items

- No golden row reaches the `cset_opacity` 1 and 3 entries of these stages: the variant group of `tools/fx_golden.py` (`VARIANTS`) holds none for them, so a change to those entries is not caught by `fx_golden.py`. The documented route is the variant group of [Adding an effect](../adding-an-effect.md) ("Golden rows": suffixes `+o1` and `+o3`): add the two `VARIANTS` entries and rewrite the table with `--write` on a build whose output is checked. Until then, render the answers 1 and 3 with the build before and after a change and compare the files with `cmp`.
- `blur_warp` keeps the CPU rerun of its NaN outputs on the device: its NaN words are the compiled CPU instruction's picks at each site of the bodies, which its kernel does not copy ([Rendering on a device](../guide.md#rendering-on-a-device)).
- The 50x7 placement of `blur_res` Half and Quarter is the 64x32 one; that a real host uses it at 50x7 is INFERRED (the `S_BlurMotion` section of [Effects](../effects.md)).
- `ROUTE AVX2-ONLY`: the tiers of CPUs without AVX2 and FMA are not reproduced ("Limits common to the effects" in [Effects](../effects.md)).
- The "Host buffer phase" line of the `S_BlurMotion` section of [Effects](../effects.md) names blur windows on the mono style; no stage plan reaches them, because the mask arm that holds the `blur` folds away (see [Verified scope and limits](#verified-scope-and-limits)).
- The device test files of these operations (`device_ripmap.rs`, `device_warp.rs`, `device_pointwise.rs`) hold no mutation test of the `make_mipmap`, `blur_warp` and `rotate` kernels; the self-check is their guard.
