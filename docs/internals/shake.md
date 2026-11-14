# Internals of `stage apply shake_normal_autogen`, `stage apply shake_twitchy_autogen` and `stage apply shake_jumpy_autogen`

This page covers the code of these stages: where each part lives, how a render runs, how parity is judged, the checks that run from this repository, the device path and the limits. They are the three styles of `S_Shake`. The page also describes the shake offset, the part that `S_DissolveShake` shares ([`S_DissolveShake` internals](dissolve-shake.md)). The user reference is the `S_Shake` section of [Effects](../effects.md); the parameters are described in [Driving the effect parameters](../parameters-guide.md) and the stages in the section "Effects provider" of [Adding a provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md).

## What it computes

On every frame the image is resampled at displaced coordinates: the coordinates of the default xy grid are moved by an offset in x and y, turned by the tilt offset and scaled by `z_dist` plus the depth offset. Each of the four offsets (x, y, depth z and tilt) is the sum of a random part, a noise of the frame time, and a wave part, a sine of the frame time, scaled by `amplitude` and by the random and wave amplitudes of its axis. The offsets depend only on the parameters and the host time values, so the compile folds them to constants of the frame, and the compiled plan holds only image operations. With `motion_blur` on, each pixel is sampled along the move from its coordinates at the frame time toward those one frame later, over the part of the move that `motion_blur_length` gives, instead of at one point; `wrap` sets per axis what the resampling reads outside the frame. With a channel parameter away from its default the shake runs once per colour channel, each with its own amplitude and phase, and the three channels and the alpha are assembled into one image (read from the constructors, INFERRED; the operations are MEASURED below).

The three styles differ in the offsets only. Normal is the continuous shake. Twitchy runs the offsets at ten times `frequency` and multiplies them by a twitch scale: the mean, over the frame and its two neighbours one frame apart, of a random switch of the frame time taken at `twitch_frequency` and compared with `stillness`. In detail, the twitch scale is the mean of a switch at the frame time and at one frame (1 / frames per second) before and after (`defun_smooth_twitch`), and the switch is 1 where the random sawtooth of the time times `twitch_frequency`, seeded by `seed`, is above `stillness` and 0 elsewhere (`defun_twitch`; the sawtooth is `crates/effects/fx_eval/src/sawtooth.rs`). Jumpy sets the route selector of the offset kernel to its hashed route, passes `drift` as that route's fold-back gap, and applies the twitch scale too, with `center_bias` in the place of `stillness` and four times `frequency` as its rate (`eff_shake_jumpy` in `crates/effects/fx_eval/src/programs_native.rs`; read from the constructors, INFERRED).

The offset of one axis is one call of the offset kernel with twelve arguments, which `shake_shift_value` in `crates/effects/fx_eval/src/fold.rs` maps in order to the fields of `Params` in `crates/effects/kernels/shake_shift/src/body.rs`: the axis seed (`seed` plus a fixed offset per axis), the axis time ((frame time minus the axis phase minus `phase`) times `frequency`), the random amplitude (`amplitude` times the axis random amplitude) and rate (the axis random frequency), the sine amplitude (`amplitude` times the axis wave amplitude) and frequency (the axis wave frequency), three arguments of a second noise stage that the per-channel route uses for `RGB_randomness` and `RGB_frequency`, the route selector, the fold-back gap and the gain (the twitch scale, or 1 without it). The kernel returns the first noise stage plus the sine term plus the second noise stage; the sine term is the sine amplitude times the sine of two pi times the sine frequency times the time (`K_SINE`). Each noise stage takes the smooth route, a four by four neighbourhood of hashed values weighted by a cubic filter at the coordinate time times 2 times the rate, or the hashed route, a search of the 1,024 entry prefix-sum table built from the first hash, whose offset is folded back beyond one minus the gap; a seed enters as a hash key, the seed scaled by 4096 and truncated (module and field comments of `body.rs`). The tilt offset is in degrees and is turned into revolutions for `rotate`, and the depth offset is added to `z_dist`, which scales the coordinates (read from the code, INFERRED).

## Styles and stages

| style | program (wrapper, then registered program) | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Normal | `Shake_Normal_Autogen`, then `Shake_Normal` | `stage apply shake_normal_autogen` | temporal | none | false | `cset_opacity` 1 and 3 |
| Twitchy | `Shake_Twitchy_Autogen`, then `Shake_Twitchy` | `stage apply shake_twitchy_autogen` | temporal | none | false | `cset_opacity` 1 and 3 |
| Jumpy | `Shake_Jumpy_Autogen`, then `Shake_Jumpy` | `stage apply shake_jumpy_autogen` | temporal | none | false | `cset_opacity` 1 and 3 |

The rows are lines 18 to 20 of `crates/neutral-handoff/data/fx_stages.tsv`. All three rows have the source name `Source`, the same `host` column (a four-plane reader of the recorded runs), the placement `--xy-base 0` at 50x7 and at 64x32 (served from the 64x32 entry at every other size), and a `fixed` column that sets the three parameters of the unconnected mask input, `mask_use`, `blur_mask` and `invert_mask`, to the values the original holds for an absent input ([Effects provider](../stages.md#effects-provider), "Mask input"). The `host_variants` entries for the Opacity answers 1 and 3 replace the reader options for those answers; the default answer 2 runs the `host` column. The stages offer 25, 25 and 23 parameters (`stage list` in [Effects provider](../stages.md#effects-provider)); every registered style of `S_Shake` has a stage, so no style is refused. A stage offers a parameter when a counted recorded render set it ([Effects provider](../stages.md#effects-provider), "Parameters"); by that rule the three stages offer different subsets and leave out `z_wave_amp` and the `z_rand_amp` of Normal.

Refusals of these stages (exit codes through `sapphire stage apply`; `fx_run` exits 2 for a refusal):

| Refusal | Exit | When |
|---|---|---|
| `FPS-UNGIVEN` | 3 | no `--fps`: every style reads `get_frames_per_second`, so the compile refuses before any frame is written (MEASURED below) |
| `unknown param` | 3 | a parameter the stage does not offer: `z_wave_amp` of every style, `z_rand_amp` of Normal, the mask parameters (MEASURED for `z_wave_amp`) |
| `bad value for` | 3 | a value outside the range `stage list --json` prints, for example `cset_opacity` outside 1 to 3 |
| `LAYER-INPUT-UNSUPPORTED` | 3 | any `--layer`; the `Mask` input of the programs is not offered as a layer |
| `needs a width of 2 or more` | 3 | a frame 1 pixel wide, the two 1x1 cases of `tools/conformance.py` |
| `HOST-TIME-RANGE` | 3 | a rate or a frame tick outside the host's 32-bit time fields ([Host values](../cli.md#host-values)) |
| `SHAKE-SHIFT-UNFOLDED` | 2 (`fx_run` only) | a shake_shift node that the fold leaves in the plan; the renderer has no run-time kernel for it. A stage run does not reach it, since the offsets fold |
| `XY-BASE-UNGIVEN` | 2 (`fx_run` only) | a direct `fx_run` run whose plan reads the default xy without `--xy-base`; the stages pass the base from their placement column |

The labels are listed in [Refusal labels](../cli.md#refusal-labels).

## Code map

| Part | Path | Role |
|---|---|---|
| Wrapper constructors | `crates/effects/fx_eval/src/programs_native.rs`: `eff_shake_normal_autogen`, `eff_shake_twitchy_autogen`, `eff_shake_jumpy_autogen` | the registered style programs with the `Mask` input, which call the programs below |
| Program constructors | the same file: `eff_shake_normal`, `eff_shake_twitchy`, `eff_shake_jumpy` | the shake programs; Twitchy scales `frequency`, Jumpy sets the jump route |
| Shared helpers | the same file: `defun_shake`, `defun_shake_channels`, `defun_shake_internal`, `defun_smooth_twitch`, `defun_twitch`, `defun_eq` | `defun_shake` chooses the plain or the per-channel route; `defun_shake_internal` holds the eight offsets (four axes at the frame time and one frame later), the coordinates and the `warp` or `blur_warp` output; the twitch helpers build the twitch scale from `random_sawtooth` |
| Layouts | the same file: `layout_shake_normal_autogen`, `layout_shake_twitchy_autogen`, `layout_shake_jumpy_autogen` | parameter groups of the styles |
| Parameter definitions | `crates/effects/rtdata/src/styles/shake_normal.rs`, `shake_twitchy.rs`, `shake_jumpy.rs` | registered parameters, host types, ranges, normalize scales and input records; `fx_run --describe-params` prints them |
| Shared class rows | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS`, `IMG_ONLY_ARGS`, `INT_TYPED_ARGS`, `COMBOS`, `COMBOS_EXEC` | the class flags and combination tables of `shake_shift`, `random_sawtooth`, `make_mipmap`, `warp`, `blur_warp`, `interp`, `postdivide` and the channel classes |
| Offset fold | `crates/effects/fx_eval/src/fold.rs`: `fold_kind`, `shake_shift_value`, `fold_lanes` | folds `get_time`, `get_frames_per_second`, every `shake_shift` node and every `random_sawtooth` node to constants of the frame |
| Offset kernel | `crates/effects/kernels/shake_shift` (`body.rs`, `hash.rs`, `weights.rs`, `sin.rs`, `tables.rs`, `exact.rs`, `fill.rs`, `dispatch.rs`) | the render body of one offset value (AVX2 route, `f32::mul_add` at the fused sites) with its hash, weight table, sine chain and fill routine; every table computed from its definition |
| Random switch | `crates/effects/fx_eval/src/sawtooth.rs` | the constant path of `random_sawtooth` (the twitch switch) and its 1,024 row table, computed |
| Default xy | `crates/effects/fx_eval/src/xyplace.rs`, `crates/effects/fx_exec/src/kernels2.rs` (`default_xy`) | the placement of each default-xy writer in the plan (a plane with a phase per row, or one row buffer) and the coordinates it writes from `--xy-base` |
| `+`, `*` | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`), crate `ops_arith` | the offset added to the default xy and the depth scale |
| `rotate` | `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`, `rotate_k`), crate `rotate2d` | the tilt, present when a tilt offset is not zero |
| `make_mipmap` | `kernels2.rs` (`make_mipmap_k`), crate `make_mipmap` | the source pyramid with the `wrap` modes |
| `warp` | `kernels2.rs` (`warp_k`), crates `warp_log_gradient` and `warp_render` | the resampling without motion blur |
| `interp`, `blur_warp` | `kernels.rs` (`interp_k`), crate `interp`; `kernels2.rs` (`blur_warp_k`), crates `blur_warp`, `blur_warp_1ch` to `blur_warp_4ch` | the end point of the motion segment and the sampling along it; `blur_warp` is described in [`S_BlurMotion` internals](blur-motion.md) |
| `postdivide` | `kernels2.rs` (`postdivide_k`), `crates/effects/fx_exec/src/postdiv.rs` | the final division by alpha at the Opacity answer 2 |
| Channel assembly | `kernels.rs` (`element_k`, `v3`), crates `ops_element` and `ops_vec`; `kernels2.rs` (`copy_alpha_k`, `get_alpha_k`, `v3_to_v4_if4_a_k`), crates `copy_alpha`, `get_alpha`, `v3_to_v4_if4_a` | the per-channel route |
| Executor checks | `crates/effects/fx_exec/src/exec.rs` (`FPS-UNGIVEN`), `crates/effects/fx_exec/src/fxrun.rs` (`SHAKE-SHIFT-UNFOLDED`, `host16_slider`) | refusals and the 16.16 slider rule |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS`, `ALLOWLIST`, `NAN_EXACT`, `CPU_ONLY`); `device_ops_pointwise.rs` (`ADD`, `MUL`, `ELEMENT_OF`, `V3`, `INTERP`, `POSTDIVIDE`), `device_ops_warp.rs` (`warp_accepts`, `rotate_accepts`), `device_ops_ripmap.rs` (`make_mipmap_accepts`, `blur_warp_accepts`), `device_ops_plane.rs` (`COPY_ALPHA`, `GET_ALPHA`, `V3_TO_V4_IF4_A`) | which nodes run on a device and under which checks |
| OpenCL sources | `crates/effects/fx_device/src/kernels_pointwise.rs`, `kernels_warp.rs`, `kernels_ripmap.rs`, `kernels.rs` | the device kernels of these operations |
| Stage rows | `crates/neutral-handoff/data/fx_stages.tsv` lines 18 to 20; `crates/neutral-handoff/data/fx_params.tsv` lines 185 to 257 | host arrangement and offered parameters |
| Stage code | `crates/neutral-handoff/src/fxstage.rs`: `apply_profile`, `run_fx`, `host_at`, `placement_for`, `param_args`, `time_values`, `snap_text` | the per-frame runner arguments |
| Computed tables | the `shake_shift` crate and `sawtooth.rs` above | see [No external data](../effects.md#no-external-data), "Shake tables" |
| Tests | the files named under "Checks that run from this repository" | |

The operations of the compiled plans are MEASURED with the device report of the commands below (`SAPPHIRE_DEVICE_REPORT=1`, every node on the device, so every executed node is counted): the plain Normal render runs `*`, `+`, `make_mipmap`, `postdivide` and `warp` per frame; Twitchy and Jumpy at the parameters below run `+`, `make_mipmap`, `postdivide` and `warp`; `motion_blur=1` runs `+` (two nodes), `interp`, `blur_warp`, `make_mipmap` and `postdivide`; a tilt amplitude adds `rotate`; the Opacity answers 1 and 3 drop `postdivide`; the channel parameters of the golden `+ch` row run `*`, `+` (five nodes), `copy_alpha` (three), `element_of` (six), `get_alpha` (three), `make_mipmap` (three), `postdivide`, `v3`, `v3_to_v4_if4_a` and `warp` (three).

## Render flow

1. `sapphire stage apply shake_normal_autogen --width W --height H --depth 32 --fps N[/D] ...` finds the stage row, refuses `--layer` (`LAYER-INPUT-UNSUPPORTED`), checks the `host_variants` column, refuses a width under 2, reads the parameters and their keys, and checks the rate (`HOST-TIME-RANGE`). A plain value of a parameter that the host shows as a 16.16 slider (every `--host-param` value, and a `--set` value whose host type is a slider without a normalize scale: `host16_slider`, `crates/effects/fx_exec/src/fxrun.rs` line 3658) is put on the 1/65536 grid, and the stage prints `note: ... is put on the 16.16 host grid as ...` on standard error (`apply_profile`, `crates/neutral-handoff/src/fxstage.rs` line 1622; MEASURED in the Twitchy and Jumpy commands below, whose `amplitude`, `stillness`, `drift` and `center_bias` are `--set` values). The rule is the same for every effect stage; [Driving the effect parameters](../parameters-guide.md) describes it for `--host-param` values.
2. Each input frame is handed to the runner in memory as 'ae32' words (`frame_to_words`). The argument list of frame f (`run_fx`) is the program, `--size`, the source, `--geom` from the frame size, `--host-frame` and `--ctx-time` from f, the `host` column or the host variant entry that the frame's `cset_opacity` selects (`host_at`), the placement for the frame size (`placement_for`), the `fixed` column, the parameters (`--set`, `--host-param` for the x, y and z amplitudes, `--cset` for the opacity answer), `--host-value get_time=...` and `--host-value get_frames_per_second=...` from `--fps` (`time_values`), `--ns-default-unbound` and `--do-postdiv`, learned from the runner's `POSTDIV-MISMATCH` answer ([Effects provider](../stages.md#effects-provider)).
3. The runner (`fx_exec::fxrun`) builds the program from the native constructors and the definitions of `effects_rtdata`, converts the host-unit amplitudes with the frame width on both axes (`crates/effects/fx_eval/src/units.rs`), and compiles the plan. A program that reads `get_frames_per_second` with no rate is refused with `FPS-UNGIVEN` here.
4. The fold evaluates `get_time` (f32(f) / f32(24) at 24/1), `get_frames_per_second`, the time one frame later used by the motion blur and the twitch, every `shake_shift` node through the `shake_shift` crate and, on Twitchy, every `random_sawtooth` node through `sawtooth.rs`. A `shake_shift` node that does not fold is refused with `SHAKE-SHIFT-UNFOLDED`.
5. The plan reads the default xy grid. Each default-xy writer is placed by `xyplace.rs` (a top-level plane whose rows take their own address phase, or one row buffer in the line list of its block), and `--xy-base 0` gives its base phase.
6. The executor runs the nodes of the plan in order: `make_mipmap` builds the source pyramid with the `wrap` modes; `+` adds the folded offset to the default xy; `rotate` turns it when the tilt offset is not zero; `*` scales it when the depth scale is not 1; `warp` resamples the pyramid at the coordinates, or, with motion blur, `interp` computes the end point and `blur_warp` samples along the segment; `postdivide` divides by alpha at the default Opacity answer 2 (the answers 1 and 3 run no `postdivide` node, MEASURED by the golden coverage loop below). The per-channel route runs one such chain per colour channel and assembles the result with `element_of`, `copy_alpha`, `get_alpha`, `v3` and `v3_to_v4_if4_a`.
7. Threading: the parallel-safe kernels split rows over `SAPPHIRE_THREADS` workers when the frame holds at least `SAPPHIRE_PAR_MIN` samples; the output bytes do not depend on either ([Environment variables](../cli.md#environment-variables)).
8. Device: with `--device gpu|gpu=N|auto` or `SAPPHIRE_DEVICE`, each node whose operation passed the self-check runs on the device; a node the kernel declines, and a NaN output of an operation that is not admitted as NaN exact, run on the CPU with the same bytes (see "Device path" below).
9. The runner's output words are relaid to the frame format (`words_to_frame`) and written. Output frame f depends only on input frame f, the parameters, f and the rate; there are no layers, no frame requests and no history.

The direct route is `fx_run PROGRAM --size WxH --out PREFIX` with the same options; the stage builds that list and calls the same library in process, so no `fx_run` process starts. The list is the internal runner interface ([`fx_run --describe-params`](../cli.md#fx_run---describe-params)); `crates/effects/fx_exec/tests/t_dest_route_label.rs` shows a complete 'ae32' list for another program.

## How parity is judged

An effect counts as rebuilt when every registered style passes seven checks on the final binary: the program text, the builtin classes, the data it reads, its literals, the compiled plan, the combination tables and the execution, each judged against the original running in an emulator ([What rebuilt means](../effects.md#what-rebuilt-means)).

- `S_Shake` has 68 verified comparisons, 2,056 recorded renders and 2,056 re-rendered units (`docs/data/effects_status.tsv`). 65 parameter sets are recorded at the eight base phases 0, 4, ..., 28 of the default xy buffer, and three channel sets at 50x7 at the 512 combinations of the phases of their three writers.
- Through the stages, 54 of the 66 verified renders that equal the original are byte identical to it (17 of 22 Normal, 18 of 22 Twitchy and 19 of 22 Jumpy; README "Effects and stages", [Effects provider](../stages.md#effects-provider)). The other 12, 9 recorded at a phase other than 0 and the three channel renders at 50x7, are verified through `fx_run` at their recorded phases and through the stages against the runner's render at phase 0.
- Two verified comparisons differ from the original: two Twitchy renders at 50x7 at still frames, in 178 and 302 of 1400 words. At such a frame the offset is exactly zero, the plan writes the default xy as a plane that a later block reads, and the original gives each row of that plane its own buffer phase; the recorded renders were made with the former rule, one row buffer for every plan without a perspective node. The head-form rule of `xyplace.rs` gives the original's output for the same commands (MEASURED after the original's output was known, so both stay counted as different; [Effects](../effects.md), the `S_Shake` section).
- 16 re-rendered units of `S_Shake` changed with the default-xy head form: one counted unit for each of the two comparisons that differ, the rest control units ([What rebuilt means](../effects.md#what-rebuilt-means)).
- Controls: the 2,056 recorded renders are the 65 parameter sets at 8 phases and the 3 channel sets at 512 phase combinations; each set counts once, at the phase of its verified run, and its other members are control renders (INFERRED from the counts and [What rebuilt means](../effects.md#what-rebuilt-means); the list of members is outside the repository).
- The renders outside the stage figure differ from the stage's host arrangement in the placement options only: 9 were recorded with `--xy-base` at a phase other than 0 and the 3 channel renders with their three writers at different phases, while the placement column passes `--xy-base 0`. A host variant entry cannot carry a placement option (`HOST-VARIANT-PLACEMENT`), so no entry can serve them ([Effects provider](../stages.md#effects-provider)).
- The "74 counted recorded renders" of the device paragraph are the rows of the counted runs: the 65 parameter sets and the nine channel renders recorded first at 50x7, which are not compared (INFERRED from the counts of the `S_Shake` section of [Effects](../effects.md)).

## Checks that run from this repository

Run from the repository root after the [Build](../../README.md#build) step; scratch files go to `target/demo/shake/`.

| Check | Command | Expected result | Exit |
|---|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| Stages listed | `sapphire stage list --json` filtered | three lines, `temporal`, 25, 25 and 23 parameters, no layers | 0 |
| Parameters | `fx_run --describe-params` for the three programs | one JSON member per program | 0 |
| Worked render | `stage apply shake_normal_autogen` | three output frames of 64x32 at depth 32 | 0 |
| Refusals | without `--fps`; with `z_wave_amp` | `FPS-UNGIVEN`, empty output; `unknown param` | 3 |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 29 rows of these stages | 0 |
| Crate tests | `cargo test` on the offset, fold and kernel crates | all pass; two `NOT RUN` lines without `VEC_DIR` | 0 |
| Stage test | `cargo test -p neutral-handoff --test stage_cli` filtered | 1 passed | 0 |
| Conformance | `tools/conformance.py --stage` per stage | L1 `14 passed, 2 failed` (the 1x1 cases), L3 `33 passed` | 1 |
| Device golden | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ` (needs OpenCL) | 0 |
| Device report | `--device gpu` with `SAPPHIRE_DEVICE_REPORT=1` | `fallback: none` on every frame; `cmp` silent | 0 |
| Device tests | `cargo test -p fx_exec` device tests, one thread | all pass (needs OpenCL with `SAPPHIRE_REQUIRE_DEVICE=1`) | 0 |

Build, list the stages and print the parameters:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
target/release/sapphire stage list --json | python -I -c "import json,sys; d=json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in d['stages'] if s['id'].startswith('shake_')]"
target/release/fx_run --describe-params Shake_Normal_Autogen Shake_Twitchy_Autogen Shake_Jumpy_Autogen
```

The second command prints `shake_normal_autogen temporal 25 []`, `shake_twitchy_autogen temporal 25 []` and `shake_jumpy_autogen temporal 23 []`; the third prints one JSON object with a member per program. Defaults and ranges come from these commands only.

One worked render per style, and the two refusals (ffmpeg on `PATH` makes the input; the first render is the command of the `S_Shake` section of [Effects](../effects.md)):

```sh
mkdir -p target/demo/shake
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/shake/in.raw
target/release/sapphire stage apply shake_normal_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=1.6999969482421875 --param frequency=5.5 --param phase=-0.350006103515625 --param x_rand_amp=0.4499969482421875 --param y_rand_freq=2.5 --param z_dist=2.5 < target/demo/shake/in.raw > target/demo/shake/out.raw
target/release/sapphire stage apply shake_twitchy_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=1.4 --param stillness=0.35 --param twitch_frequency=4.5 --param x_rand_amp=0.5 < target/demo/shake/in.raw > target/demo/shake/twitchy.raw
target/release/sapphire stage apply shake_jumpy_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=1.3 --param drift=0.65 --param center_bias=0.8 --param frequency=3.5 < target/demo/shake/in.raw > target/demo/shake/jumpy.raw
target/release/sapphire stage apply shake_normal_autogen --width 64 --height 32 --depth 32 < target/demo/shake/in.raw > target/demo/shake/nofps.raw # rc 3
target/release/sapphire stage apply shake_normal_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param z_wave_amp=0.125 < target/demo/shake/in.raw > target/demo/shake/zwave.raw # rc 3
```

Each render writes three frames of 64x32 at depth 32. The Twitchy and Jumpy commands print `note:` lines for the values they put on the 16.16 grid (`1.4` becomes `1.399993896484375`). The fifth command prints `error: unsupported: frame 0: fx_run refused: bad plan: compile: FPS-UNGIVEN (...)` and writes no frame; the sixth prints ``error: unsupported: unknown param `z_wave_amp` ``.

The golden rows. `tools/fx_golden.py` renders every stage of `crates/neutral-handoff/data/fx_stages.tsv` on fixed synthetic clips and compares the SHA-256 of each output with `tools/fx_golden.tsv`; it has no filter, so it checks all 183 cases. The rows of these stages are the plain sizes `64x32` and `50x7`, the threaded group (`64x32t`, `50x7t`), the special size `40x12s`, and the variants `+o1` and `+o3` (Opacity answers 1 and 3) of all three stages and `+ch` (channel parameters) of `shake_normal_autogen`:

```sh
python -I tools/fx_golden.py
grep -c -E '^shake_(normal|twitchy|jumpy)_autogen' tools/fx_golden.tsv
grep -E '^shake_normal_autogen' tools/fx_golden.tsv | cut -f1,2
```

The first prints `fx_golden: 183 cases, 183 equal, 0 differ`, the second `29`, the third the 11 rows of `shake_normal_autogen` by size. A difference prints `DIFFERS STAGE SIZE: expected ..., rendered ...` and exit 1.

Which operations the golden rows reach. The compiled plan depends on the parameters, the frame time, the size and the opacity answer, not on the pixel values (the offsets fold from parameters and host values), so the worked input stands for the synthetic clip of the golden rows; the loop prints the last report line of each stage at the options of each golden group (needs OpenCL):

```sh
for v in "" "--param cset_opacity=1" "--param cset_opacity=3" "--param red_amplitude=1.5 --param green_phase=0.0800018310546875 --param blue_amplitude=0.6999969482421875"; do for s in shake_normal_autogen shake_twitchy_autogen shake_jumpy_autogen; do printf "%s [%s] " $s "$v"; SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply $s --device gpu --width 64 --height 32 --depth 32 --fps 24 --frames 3 $v < target/demo/shake/in.raw 2>&1 >/dev/null | tail -1; done; done
```

| Golden group | Operations per frame (MEASURED by the loop) |
|---|---|
| plain, threaded (`t`) and special (`s`) rows of the three stages | `+`, `make_mipmap`, `postdivide`, `warp` |
| `+o1`, `+o3` | `+`, `make_mipmap`, `warp` |
| `+ch` (`shake_normal_autogen` only) | `*`, `+` (5), `copy_alpha` (3), `element_of` (6), `get_alpha` (3), `make_mipmap` (3), `postdivide`, `v3`, `v3_to_v4_if4_a`, `warp` (3) |

No golden row of these stages reaches `rotate`, `interp`, `blur_warp` or the `*` of a depth scale; the rows of `stage apply dissolve_shake` reach `interp` and `blur_warp`. Every row runs the `shake_shift` fold, on the hashed route in the Jumpy rows, and the `random_sawtooth` fold in the Twitchy and Jumpy rows (read from the constructors, INFERRED). The special rows run the plain plan on infinities, NaN and subnormal values (INFERRED: same options).

The crate, fold and executor tests that cover the offset and the operations of these plans (`-- --nocapture` shows the `NOT RUN` lines):

```sh
cargo test --release --offline -p shake_shift -p ops_arith -p ops_element -p ops_vec -p copy_alpha -p get_alpha -p v3_to_v4_if4_a -p blur_warp -p warp_log_gradient
cargo test --release --offline -p make_mipmap -p interp -p rotate2d -p warp_render
cargo test --release --offline -p fx_eval --test t_shake_shift_fold --test t_shake_shift_cases --test t_random_sawtooth --test t_random_sawtooth_nan --test t_frame_rate --test t_get_time_fold -- --nocapture
cargo test --release --offline -p fx_exec --test t_frame_rate_refusal --test t_refusals --test t_xyh1_head_form --test t_copy_alpha --test copy_alpha_source_alpha
cargo test --release --offline -p neutral-handoff --test stage_cli a_stage_that_reads_the_frame_rate_is_refused_without_fps
```

All end with exit 0. Without `VEC_DIR`, `t_shake_shift_cases` (the recorded cases of the offset body) prints `NOT RUN: VEC_DIR is not set` and `t_random_sawtooth` prints the same line in its recorded-case test; every other test runs on data it builds itself. `shake_shift`'s own tests check the hash table, the dispatch targets, the sine values, the weight table and the fill routine; `t_shake_shift_fold` checks that the fold equals the crate body; `t_random_sawtooth_nan` checks the x86 default NaN of the switch on every processor (CI also runs it on the macOS arm64 runner); `t_frame_rate_refusal` checks `FPS-UNGIVEN`; `t_refusals` checks `SHAKE-SHIFT-UNFOLDED`; `t_xyh1_head_form` checks the head-form rule of the default xy.

Conformance on each stage (the command names `sapphire` by absolute path):

```sh
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply shake_normal_autogen" # rc 1
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply shake_twitchy_autogen" # rc 1
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply shake_jumpy_autogen" # rc 1
```

Each prints `L1 stage matrix FAIL 14 passed, 2 failed`, two `FAIL` lines for the 1x1 cases (`needs a width of 2 or more`, exit 3), `L3 failure injection pass 33 passed` and `result: fail (exit 1)`. These are the documented size refusals ([Conformance](../stages.md#conformance)).

The device checks need an OpenCL platform such as PoCL. Without one, every case of the golden run is refused with `DEVICE-REQUIRED-UNAVAILABLE` (`fx_golden: 183 cases, 0 equal, 183 differ`, exit 1), the report commands end with that label and exit 3, and the device tests fail with `SAPPHIRE_REQUIRE_DEVICE=1` or print `NOT RUN` and pass without it (MEASURED with `OCL_ICD_VENDORS=/nonexistent` for the first two):

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply shake_normal_autogen --device gpu --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=1.6999969482421875 --param frequency=5.5 --param phase=-0.350006103515625 --param x_rand_amp=0.4499969482421875 --param y_rand_freq=2.5 --param z_dist=2.5 < target/demo/shake/in.raw > target/demo/shake/out_dev.raw
cmp target/demo/shake/out.raw target/demo/shake/out_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply shake_normal_autogen --device gpu --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=1.6999969482421875 --param frequency=5.5 --param motion_blur=1 --param motion_blur_length=3 < target/demo/shake/in.raw > target/demo/shake/mb_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply shake_normal_autogen --device gpu --width 64 --height 32 --depth 32 --fps 24/1 --param red_amplitude=1.5 --param green_phase=0.0800018310546875 --param blue_amplitude=0.6999969482421875 < target/demo/shake/in.raw > target/demo/shake/ch_dev.raw
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --lib device -- --test-threads=1
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_backend --test device_pointwise --test device_warp --test device_ripmap -- --test-threads=1
```

The golden run prints `fx_golden: 183 cases, 183 equal, 0 differ`. The first report command prints the device line, then `device: * 1, + 1, make_mipmap 1, postdivide 1, warp 1; fallback: none` after frame 0 and the running counts after frames 1 and 2, and `cmp` prints nothing. The motion blur command ends `device: + 6, blur_warp 3, interp 3, make_mipmap 3, postdivide 3; fallback: none`; the channel command ends `device: * 3, + 15, copy_alpha 9, element_of 18, get_alpha 9, make_mipmap 9, postdivide 3, v3 3, v3_to_v4_if4_a 3, warp 9; fallback: none`. The first device run of a build pays the self-check of every allowlisted operation ([Rendering on a device](../guide.md#rendering-on-a-device), "Caches").

CI (`.github/workflows/ci.yml`) runs these checks on every push to `main` and every pull request: `cargo test --release` (the default members, among them `shake_shift`, `ops_arith`, `copy_alpha`, `get_alpha`, `v3_to_v4_if4_a`, `blur_warp` and `neutral-handoff`), the non-default crates step on Linux (`fx_eval`, `fx_exec`, `make_mipmap`, `interp`, `rotate2d`, `warp_render`), the sawtooth NaN test on macOS, `python -I tools/fx_golden.py` on Linux, macOS and Windows, `conformance.py --quick --allow-absent-peer`, `test_effect_rules.py`, and in the job "device backend on PoCL" the `--lib device` tests, the device test files and the golden run with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1`.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, together with the recorded renders and the stored test data. From this repository alone the checks above confirm that the build renders this repository's own golden bytes; they cannot say whether the original agrees.

- The recorded renders of the 68 verified comparisons and the 2,056 re-rendered units are not in the repository.
- The output the original gives at 50x7 at the phases a host allocator gives, at frame widths that are not a multiple of 8 in the plane form, and for the channel shake at Opacity answers 1 and 3 can only be judged in the harness.
- `VEC_DIR` names a folder with the stored test data (`effects/` under it). Without it, `t_shake_shift_cases` and the recorded-case test of `t_random_sawtooth` print `NOT RUN` and pass; with it they compare the offset body and the switch with the original's recorded cases bit for bit.
- The full report, with a paragraph per effect and the runs behind it, is identified by the SHA-256 printed in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Sizes 64x32 and 50x7; whole frames from 0 to 139 at 24 frames a second; the Opacity answers 1, 2 and 3 at both sizes with input alpha other than 1. Other sizes, rates, times between frames and parameter values run the same code and are INFERRED ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- Frame time: the stages pass `get_time` and `get_frames_per_second` from `--fps`; at a rate whose reduced denominator is not 1 the values follow the original's arithmetic for a host with time scale num and local step den, and equality with a host's values is INFERRED ([Host values](../cli.md#host-values)).
- Host buffer phase: the default xy writer takes its order from its buffer phase on rows of 29 pixels or more. At 64x32 the phases do not change the output; at 50x7 they do, and the stages pass phase 0 for every writer at every size. The placement at sizes other than 64x32 and 50x7 is served from the 64x32 entry (INFERRED). A 1920x1080 render is above the threshold, so its output rests on the stage's phase 0 (INFERRED); the phase a host's allocator gives is outside the repository.
- Head form: a writer allocated outside every exec block is a plane whose rows start at the base plus the row times the frame width times 4 bytes (PerRow), and a writer allocated in the line list of its block is one row buffer for every row (OneHead) (`crates/effects/fx_eval/src/xyplace.rs`, module comment). A shake plan writes the plane form when its offset folds to zero, as at a still frame of Twitchy, because the plane is then read by a later block (INFERRED from the `S_Shake` section of [Effects](../effects.md)). When the width is a multiple of 8 every row starts at the base phase modulo 32 bytes and the two forms agree; at other widths the row phases step through the residues, and no verified render judges the original's output there.
- The channel shake at 50x7 is verified through `fx_run` by three renders recorded at all 512 combinations of the three writers' phases; the stages run phase 0 for all three.
- No verified render sets a channel parameter at Opacity answer 3, and none covers the channel shake at answer 1 (where the compile drops the channel alpha copies and the final alpha assembly, by element counts only): both are INFERRED.
- `z_wave_amp` of every style and `z_rand_amp` of Normal are not offered, because no verified render sets them; the same depth offset code is reached with other values through the other styles and `S_DissolveShake` (INFERRED for these four parameters).
- The unconnected mask input: the stages fix its three parameters at the values the original holds; on the direct `fx_run` route an unbound `blur_mask` is converted from host units instead, and the node that reads it is not reached from the output (MEASURED on the verified renders).
- A search of the random sawtooth table that ends past its last row (a NaN or positive infinite position, a position of 2^31 or more, or a position less than 0.0004 below a multiple of 1024 of magnitude at most 4096) reads a zero row: the original reads the 12 bytes after its table there, which read zero in a recorded render (MEASURED; [Effects](../effects.md), the `S_Shake` section).
- NaN: the CPU kernels keep the x86 NaN rules; on the device, NaN outputs of `warp`, `blur_warp`, `interp` and `rotate` are rerun on the CPU (see "Device path"). The `random_sawtooth` fold gives the x86 default NaN on every processor.
- `ROUTE AVX2-ONLY`: the offset kernel and the sine route reproduce the original's AVX2 and FMA tier; the SSE render body of the offset is not ported, and the original's results on CPUs without AVX2 and FMA are not reproduced. No approximate reciprocal instruction runs on the verified render path (MEASURED by a static scan of the original's code; that the route reaches none is INFERRED; [Effects](../effects.md)).
- Approximate instructions: no kernel of these plans carries an approximate-instruction model; the `random_sawtooth` fold is labelled APPROX-FREE (`crates/effects/fx_eval/src/sawtooth.rs` line 3). What the emulator returns for such instructions is outside the repository.
- Depths 8 and 16 convert through float and are not verified. Refusals: the table under "Styles and stages".

## Device path

| Operation | Device kernel | `NAN_EXACT` | Fallback to the CPU |
|---|---|---|---|
| `+`, `*` | `device_ops_pointwise.rs` (`ADD`, `MUL`), `pointwise_bin_f32` | yes (`NAN_EXACT_COMPILED`) | a declined shape |
| `make_mipmap` | `device_ops_ripmap.rs`, `RIPMAP_MIPMAP` | yes (`NAN_EXACT_COMPILED`) | a declined shape |
| `warp`, `rotate` | `device_ops_warp.rs`, `WARP_WARP`, `WARP_ROTATE` | no | NaN outputs (`nan_output`) |
| `interp` | `device_ops_pointwise.rs` (`INTERP`) | no | NaN outputs |
| `blur_warp` | `device_ops_ripmap.rs`, `RIPMAP_BLUR_WARP` | no | NaN outputs |
| `postdivide`, `element_of`, `v3` | `device_ops_pointwise.rs` | yes (`NAN_EXACT_WRITTEN`) | a declined shape |
| `copy_alpha`, `get_alpha`, `v3_to_v4_if4_a` | `device_ops_plane.rs` (`copy_f32`, `pointwise_fill_f32`) | yes (`NAN_EXACT_WRITTEN`) | a declined shape |

No operation of these plans has a `CPU_ONLY` entry; `CPU_ONLY` is empty. MEASURED on PoCL: every golden row of the three stages, the special rows `40x12s` included, gives the CPU's bytes with `fallback: none`, and so do the 74 counted recorded renders through `fx_run` ([Effects](../effects.md), the `S_Shake` section); the commands above show `fallback: none` for the plain, motion blur and channel routes. The timing rows of [Speed](../guide.md#speed) for one 1920x1080 frame are 1.62 / 2.04 s on the CPU and 0.68 / 0.66 s on PoCL for `shake_normal_autogen`, 1.78 / 1.80 s and 0.92 / 0.92 s for `shake_twitchy_autogen`, and 2.03 / 2.17 s and 0.67 / 0.76 s for `shake_jumpy_autogen` (MEASURED on a 12 core server under shared load; PoCL is an OpenCL runtime on the CPU, not GPU speed, which is INFERRED). The self-check and the NaN admission are described in [Rendering on a device](../guide.md#rendering-on-a-device).

## Changing the code safely

- Gates that must stay green: `python -I tools/fx_golden.py` (all groups, on every platform CI runs), the crate, fold, executor and stage tests listed above, `python -I tools/test_effect_rules.py` (golden rows for every stage in the first three groups, no platform math in effect code, the comment rules), the device self-check and the device tests on PoCL, the NaN tests as CI runs them (the sawtooth NaN test on the arm64 runner), `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes of a stage changed. `tools/fx_golden.tsv` holds this repository's own renders, so the repository alone cannot say whether the original agrees with the new bytes. `python -I tools/fx_golden.py --write` rewrites the whole table from the current build; it is used only after a change whose new output was judged against the original outside the repository.
- The offset kernel and the sine route follow the original's operation order: keep `f32::mul_add` exactly at the fused sites, the cvtt truncation semantics and the computed tables, and add no platform math call (`fx_eval::detmath` or exact integer code only; `tools/fx_math_allow.tsv` lists the reasoned exceptions). The `shake_shift` tests pin the bits of each route and stage.
- A fresh NaN is the x86 default NaN on every processor, through `fx_common::nanop` and `fx_common::x86` ([Adding an effect](../adding-an-effect.md)).
- A change to the CPU result of `+`, `*`, `make_mipmap`, `warp`, `rotate`, `interp`, `blur_warp`, `postdivide` or a channel operation removes that operation's `ALLOWLIST` entry unless its device kernel changes in the same patch, and a new operation gets a device kernel or a `CPU_ONLY` entry ([Adding an effect](../adding-an-effect.md), "Device").
- What to update when the output or the interface changes: the `S_Shake` section of [Effects](../effects.md), the `S_Shake` section of [Driving the effect parameters](../parameters-guide.md), rows 18 to 20 of `fx_stages.tsv` and the rows of `fx_params.tsv`, `tools/fx_golden.tsv` (after the judgement above), `docs/data/effects_status.tsv`, this page, and `python -I tools/gen_docs.py` to regenerate `docs/generated/` and `docs/index/`.

## Open items

- The two Twitchy comparisons at 50x7 stay counted as different from the original although the current code reproduces the original's output for them ([Effects](../effects.md), the `S_Shake` section).
- `z_wave_amp` of every style and `z_rand_amp` of Normal are not offered by the stages ([Effects](../effects.md)).
- The channel shake at Opacity answers 1 and 3 and the plane form of the default xy at frame widths that are not a multiple of 8 have no verified render ([Effects](../effects.md)).
- The host buffer phase is a host value: the stages pass phase 0, and at 50x7 the original's own output depends on it ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- `ROUTE AVX2-ONLY`: the SSE render body of the offset kernel is not ported (`assumptions` in `crates/effects/kernels/shake_shift/src/lib.rs`).
- `warp`, `rotate`, `interp` and `blur_warp` are not on the `NAN_EXACT` list, so their NaN outputs are rerun on the CPU (`crates/effects/fx_exec/src/device_ops.rs`).
- Past the last row of the random sawtooth table the native reads a zero row; whether the original's bytes there are image data or host memory is outside the repository.
- No golden row of these stages reaches `rotate`, `interp`, `blur_warp` or the `*` of a depth scale (the golden coverage under "Checks that run from this repository"); a change to the tilt or motion blur route is checked only by the device and crate tests and by the rows of `stage apply dissolve_shake` for `interp` and `blur_warp`.
