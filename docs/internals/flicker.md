# Internals of `stage apply flicker`

This page covers the code of this stage: where each part lives, how a render runs, how parity is judged, the checks that run from this repository, the device path and the limits. It is the stage of `S_Flicker`. The user reference is the `S_Flicker` section of [Effects](../effects.md); the parameters are described in [Driving the effect parameters](../parameters-guide.md) and the stage in the section "Effects provider" of [Adding a provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md).

## What it computes

Every pixel of a frame is multiplied, per colour channel, by one factor: `brightness` plus `amplitude` times the channel's weight (`red_amp`, `green_amp`, `blue_amp`) times the sum of a random part and a wave part. The random part is a value noise of the frame time at `rand_freq` and `seed`, centred on zero: one value common to the three channels (class `warp_noise`, scaled by `rand_luma_amp`) and one value per channel (class `warp_noise3`, at a seed offset from `seed`, scaled by `rand_color_amp`). The wave part is a sine of the frame time at `wave_freq`, with the phase of each channel (`wave_r_phase`, `wave_g_phase`, `wave_b_phase`), scaled by `wave_amp`. The factor depends only on the parameters and the frame time, so the compile folds it to a three-lane constant of the frame, and the compiled plan holds one `*` node, the source times the factor; the alpha of the input is kept (read from the constructor, INFERRED; the plan is MEASURED below).

The noise calls, from `eff_flicker` in `crates/effects/fx_eval/src/programs_native.rs`: `warp_noise` and `warp_noise3` take the position (frame time, 0), the frequency `rand_freq`, one octave and a seed: `seed` for `warp_noise`, and `seed` plus a fixed offset for `warp_noise3`. In the crate (`crates/effects/kernels/warp_noise/src/lib.rs`, module comment and `eval`) an octave has cells of size 1 / (2^octave * freq), taps the 3 x 3 cells around the point with a hash per cell and a weight from the weight table by squared distance, and the result is remapped to `0.5 * v + 0.5`; the seed enters as a hash key, the seed scaled by 4096 and truncated. The program centres each value on zero and doubles it before the amplitudes. The wave part is the sine of (frame time x `wave_freq` plus a quarter cycle minus the channel phase), with the phases in cycles; the `sin` fold multiplies its argument by two pi (`crates/effects/fx_eval/src/svml_sin_avx2.rs`, module comment) (read from the code, INFERRED).

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Flicker (registered without a wrapper program) | Flicker | `stage apply flicker` | temporal | none | false | none |

The row is line 15 of `crates/neutral-handoff/data/fx_stages.tsv`: the source name is `Source`, the `host` column is the three-plane reader and class 3 route of the recorded runs, the `fixed` column is empty, the placement is empty at both measured sizes (the plan has no default xy reader), and `host_variants` is `-`. The stage offers 14 parameters, each passed with `--set` (`crates/neutral-handoff/data/fx_params.tsv` lines 144 to 157); the program has no opacity popup.

Refusals of this stage (exit codes through `sapphire stage apply`; `fx_run` exits 2 for a refusal):

| Refusal | Exit | When |
|---|---|---|
| `HOST-VALUE-MISSING` | 3 | no `--fps`: the plan calls `get_time` and no value was given; no frame is written (MEASURED below) |
| `unknown param` | 3 | a parameter the stage does not offer, for example `cset_opacity` (MEASURED below) |
| `bad value for` | 3 | a value outside the range `stage list --json` prints |
| `LAYER-INPUT-UNSUPPORTED` | 3 | any `--layer` (MEASURED below) |
| `needs a width of 2 or more` | 3 | a frame 1 pixel wide, the two 1x1 cases of `tools/conformance.py` |
| `HOST-TIME-RANGE` | 3 | a rate or a frame tick outside the host's 32-bit time fields ([Host values](../cli.md#host-values)) |

The labels are listed in [Refusal labels](../cli.md#refusal-labels).

## Code map

| Part | Path | Role |
|---|---|---|
| Program constructor | `crates/effects/fx_eval/src/programs_native.rs`: `eff_flicker` | the factor (`warp_noise`, `warp_noise3`, `sin`, `get_time`, `v3`) and the product with `Source` |
| Layout | the same file: `layout_flicker` | parameter groups |
| Parameter definitions | `crates/effects/rtdata/src/styles/flicker.rs` | registered parameters, host types and ranges |
| Shared class rows | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS`, `COMBOS`, `COMBOS_EXEC` | the rows of `warp_noise`, `warp_noise3`, `sin`, `get_time` and `*` |
| Fold | `crates/effects/fx_eval/src/fold.rs`: `fold_kind`, `fold_lanes`, `noise_tables`, `math1` | folds `get_time` to the host value, `warp_noise` and `warp_noise3` at constant arguments through the `warp_noise` crate, and `sin` through the 8-lane route |
| Noise | `crates/effects/kernels/warp_noise/src/lib.rs` (`Tables`, `eval`, `eval3`) | the value noise with 3 x 3 cell taps per octave, a hash per cell and a weight table, in the AVX2 body's operation order |
| Noise tables | `crates/effects/fx_common/src/shake_natives.rs` (`HashTable`, `build_weights_1`) | the 256 entry hash table and the 4608 entry weight table, computed |
| Sine | `crates/effects/fx_eval/src/svml_sin_avx2.rs` (`sin_rev`, `sin8`, `tables`) | the 8-lane AVX2 and FMA sine of the original, with its window and lookup tables computed at first use |
| `*` | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`), crate `ops_arith` | the one image operation of the plan |
| Executor checks | `crates/effects/fx_exec/src/fxrun.rs` (`HOST-VALUE-MISSING`, `host16_slider`) | the refusal without a time value and the 16.16 slider rule |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS`, `ALLOWLIST`, `NAN_EXACT_COMPILED`), `device_ops_pointwise.rs` (`MUL`) | `*` on a device |
| OpenCL source | `crates/effects/fx_device/src/kernels_pointwise.rs` (`pointwise_bin_f32`) | the device kernel of `*` |
| Stage rows | `crates/neutral-handoff/data/fx_stages.tsv` line 15; `crates/neutral-handoff/data/fx_params.tsv` lines 144 to 157 | host arrangement and offered parameters |
| Stage code | `crates/neutral-handoff/src/fxstage.rs`: `apply_profile`, `run_fx`, `param_args`, `time_values` | the per-frame runner arguments and `get_time` |
| Computed tables | `warp_noise`, `shake_natives.rs` and `svml_sin_avx2.rs` above | see [No external data](../effects.md#no-external-data), "Noise tables" and the sine of `S_Flicker` |

The plan is MEASURED with the device report below: one `*` node per frame, on ordinary input and on the special clip.

## Render flow

1. `sapphire stage apply flicker --width W --height H --depth 32 --fps N[/D] ...` finds the row (line 15), refuses `--layer` (`LAYER-INPUT-UNSUPPORTED`), checks the width, the parameters and their keys, and the rate. A plain value of a 16.16 slider is put on the 1/65536 grid, and the stage prints a `note: ... is put on the 16.16 host grid as ...` line for each value it moves (`apply_profile`, `crates/neutral-handoff/src/fxstage.rs` line 1622; MEASURED in the worked render below). The rule covers every `--host-param` value and every `--set` value whose host type is a slider without a normalize scale (`host16_slider`, `crates/effects/fx_exec/src/fxrun.rs` line 3658), on every effect stage; all 14 parameters of this stage are such `--set` values.
2. Each input frame is handed to the runner in memory as 'ae32' words. The argument list of frame f is the program, `--size`, the source, `--geom`, `--host-frame` and `--ctx-time` from f, the `host` column, the parameters with `--set`, `--host-value get_time=...` (the time of frame f, f32(f) / f32(24) at 24/1) and `--host-value get_frames_per_second=...` from `--fps`, `--ns-default-unbound` and `--do-postdiv` ([Effects provider](../stages.md#effects-provider)).
3. The runner builds the program from `eff_flicker` and the definitions of `effects_rtdata` and compiles the plan. Without a `get_time` value the plan's `get_time` node is refused with `HOST-VALUE-MISSING`, naming the program, the plan node and the getter.
4. The fold evaluates the factor: `get_time` takes the host value; `warp_noise` and `warp_noise3` run the ported noise bodies with the computed hash and weight tables (the per-point and per-cell memo caches of the original are not modelled: a cold cache, as the `assumptions` of the crate state); `sin` runs the 8-lane AVX2 and FMA sine; the arithmetic between them folds as single precision operations. The result is a three-lane constant.
5. The executor runs the one `*` node: the source planes times the factor; the alpha plane is kept.
6. Threading: `*` splits rows over `SAPPHIRE_THREADS` workers above `SAPPHIRE_PAR_MIN` samples; the bytes do not depend on it. Device: with `--device gpu|gpu=N|auto` or `SAPPHIRE_DEVICE` the node runs on the device when `*` passed its self-check.
7. The output words are relaid to the frame format and written. Output frame f depends only on input frame f, the parameters, f and the rate; no layers, frame requests or history.

The direct route `fx_run Flicker --size WxH --out PREFIX ...` takes the same option list; `crates/effects/fx_exec/tests/t_dest_route_label.rs` runs that route with a complete 'ae32' list and checks its destination-route label.

## How parity is judged

An effect counts as rebuilt when every registered style passes seven checks on the final binary: the program text, the builtin classes, the data it reads, its literals, the compiled plan, the combination tables and the execution, each judged against the original running in an emulator ([What rebuilt means](../effects.md#what-rebuilt-means)).

- `S_Flicker` has 24 verified comparisons (12 at 64x32 and 12 at 50x7), 24 recorded renders and 24 re-rendered units (`docs/data/effects_status.tsv`); every re-rendered unit is byte identical to its recorded render by full SHA-256 in the three output files `ae32`, `dest` and `src0`.
- Through the stage, 24 of 24 counted recorded renders match the stage's host arrangement and are byte identical to the original's output (README "Effects and stages", [Effects provider](../stages.md#effects-provider)).
- No comparison differs from the original. The 24 recorded renders are the 24 counted ones (one per comparison, no control render), and all 24 match the stage's host arrangement, so no render lies outside the stage figure (`docs/data/effects_status.tsv`, README "Effects and stages").

## Checks that run from this repository

Run from the repository root after the [Build](../../README.md#build) step; scratch files go to `target/demo/flicker/`.

| Check | Command | Expected result | Exit |
|---|---|---|---|
| Build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | the two binaries | 0 |
| Stage listed | `sapphire stage list --json` filtered | `flicker temporal 14 []` | 0 |
| Parameters | `fx_run --describe-params` with the program name | one JSON member with 14 entries | 0 |
| Worked render | `stage apply flicker` | three output frames of 64x32 and six `note:` lines | 0 |
| Refusals | no `--fps`; `cset_opacity`; `--layer` | `HOST-VALUE-MISSING`; `unknown param`; `LAYER-INPUT-UNSUPPORTED` | 3 |
| Golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 5 rows of this stage | 0 |
| Tests | the `cargo test` commands below | all pass; five `NOT RUN` lines without `VEC_DIR` | 0 |
| Conformance | `tools/conformance.py --stage ".../sapphire stage apply flicker"` | L1 `14 passed, 2 failed` (the 1x1 cases), L3 `33 passed` | 1 |
| Device | golden run and report with `SAPPHIRE_DEVICE=gpu` | as below (needs OpenCL) | 0 |

Build, list the stage and print the parameters:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
target/release/sapphire stage list --json | python -I -c "import json,sys; d=json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in d['stages'] if s['id'] == 'flicker']"
target/release/fx_run --describe-params Flicker
```

The second command prints `flicker temporal 14 []`; the third prints one JSON object whose one member, Flicker, lists its 14 parameters. Defaults and ranges come from these commands only.

The worked render of the `S_Flicker` section of [Effects](../effects.md) (ffmpeg on `PATH` makes the input), then the refusals:

```sh
mkdir -p target/demo/flicker
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/flicker/in.raw
target/release/sapphire stage apply flicker --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=0.330001831 --param rand_luma_amp=0.5 --param rand_color_amp=0.800003052 --param rand_freq=12 --param seed=0.770004272 --param blue_amp=1.69999695 --param brightness=1.19999695 --param wave_g_phase=-0.899993896 --param wave_amp=0.25 < target/demo/flicker/in.raw > target/demo/flicker/out.raw
target/release/sapphire stage apply flicker --width 64 --height 32 --depth 32 --param amplitude=0.5 < target/demo/flicker/in.raw > target/demo/flicker/nofps.raw # rc 3
target/release/sapphire stage apply flicker --width 64 --height 32 --depth 32 --fps 24/1 --param cset_opacity=1 < target/demo/flicker/in.raw > target/demo/flicker/cset.raw # rc 3
target/release/sapphire stage apply flicker --width 64 --height 32 --depth 32 --fps 24/1 --layer To=target/demo/flicker/in.raw < target/demo/flicker/in.raw > target/demo/flicker/layer.raw # rc 3
```

The render writes three frames of 64x32 at depth 32 and prints six `note:` lines, one for each value it puts on the 16.16 grid (`0.330001831` becomes `0.3300018310546875`). The refusals print `error: unsupported: frame 0: fx_run refused: HOST-VALUE-MISSING (Flicker: plan node 3 calls get_time and no --host-value get_time=VALUE was given)` with no frame written, ``unknown param `cset_opacity` `` and `LAYER-INPUT-UNSUPPORTED: flicker takes no ...`.

The golden rows are the plain sizes `64x32` and `50x7`, the threaded group (`64x32t`, `50x7t`) and the special size `40x12s`; the stage has no variant rows:

```sh
python -I tools/fx_golden.py
grep -E '^flicker\s' tools/fx_golden.tsv | cut -f1,2
```

The first prints `fx_golden: 183 cases, 183 equal, 0 differ`; the second prints the five rows. Every golden row runs the one `*` node per frame (the device report commands below, on the worked input and on the special clip), and the folds of `get_time`, `warp_noise`, `warp_noise3` and `sin` at the frame times 0, 1/24 and 2/24; the golden rows keep every parameter at its registered default.

The noise, sine, fold and route tests (`-- --nocapture` shows the `NOT RUN` lines):

```sh
cargo test --release --offline -p warp_noise -p ops_arith
cargo test --release --offline -p fx_eval --test t_warp_noise_fold --test t_sin_fold_pin --test t_sin_avx2 --test t_get_time_fold -- --nocapture
cargo test --release --offline -p fx_exec --test t_dest_route_label
```

All end with exit 0. `warp_noise`'s tests check the ends of the weight table, one octave at the origin by hand, the output range, determinism, the x86 default NaN for a NaN coordinate and the out-of-table report; `t_warp_noise_fold` checks the folded factor against recorded bindings of the program, held in the test as bit patterns; `t_sin_fold_pin` checks that the `sin` fold is the AVX2 route at arguments where it differs from the SSE route; `t_sin_avx2` runs its hand values without data and prints `NOT RUN ...: VEC_DIR/effects/sine_avx2 data is absent` in its five tests that compare the computed tables and the sine with stored emulator data; `t_get_time_fold` checks that `get_time` has no default; `t_dest_route_label` runs `fx_run Flicker` on the 'ae32' route.

Conformance:

```sh
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply flicker" # rc 1
```

It prints `L1 stage matrix FAIL 14 passed, 2 failed`, two `FAIL` lines for the 1x1 cases (`needs a width of 2 or more`, exit 3), `L3 failure injection pass 33 passed` and `result: fail (exit 1)`: the documented size refusals ([Conformance](../stages.md#conformance)). With `--full` L1 refuses its 27 cases at 1x1 and passes the other 216 ([Effects provider](../stages.md#effects-provider), "Exits").

The device checks (an OpenCL platform such as PoCL; without one they behave as described in [`S_Shake` internals](shake.md)):

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply flicker --device gpu --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=0.330001831 --param rand_luma_amp=0.5 --param rand_color_amp=0.800003052 --param rand_freq=12 --param seed=0.770004272 --param blue_amp=1.69999695 --param brightness=1.19999695 --param wave_g_phase=-0.899993896 --param wave_amp=0.25 < target/demo/flicker/in.raw > target/demo/flicker/out_dev.raw
cmp target/demo/flicker/out.raw target/demo/flicker/out_dev.raw
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden; open('target/demo/flicker/special.raw', 'wb').write(fx_golden.synth_special(40, 12, 3, 20261106))"
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply flicker --device gpu --width 40 --height 12 --depth 32 --fps 24 --frames 3 < target/demo/flicker/special.raw > target/demo/flicker/special_dev.raw
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise -- --test-threads=1
```

The golden run prints `fx_golden: 183 cases, 183 equal, 0 differ`. The report prints the device line and then `device: * 1; fallback: none`, `device: * 2; fallback: none` and `device: * 3; fallback: none`; `cmp` prints nothing. On the special clip of the golden row `40x12s` (infinities, NaN and subnormal values) the report also ends `device: * 3; fallback: none`.

CI (`.github/workflows/ci.yml`) runs `cargo test --release` (the default members, among them `warp_noise`, `ops_arith` and `fx_common`), the non-default crates step on Linux (`fx_eval`, `fx_exec`), `python -I tools/fx_golden.py` on Linux, macOS and Windows, `conformance.py --quick --allow-absent-peer`, `test_effect_rules.py`, and in the job "device backend on PoCL" the device tests and the device golden run.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, with the recorded renders and the stored test data. The 24 verified comparisons and the 24 re-rendered units cannot be rerun from here. Without `VEC_DIR`, the five tests of `t_sin_avx2` that compare with stored emulator data print `NOT RUN` and pass. Frame times other than whole frames of a 24 frames a second clock, rates such as 30000/1001, and whether the original's warm noise caches can return another value than a fresh evaluation can only be judged in the harness. The full report is identified by the SHA-256 printed in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Sizes 64x32 and 50x7; the verified times are the frames 0, 2, 3, 5, 11, 13, 19 and 29 of a 24 frames a second clock (whole frames from 0 to 29). Other rates, times between frames, sizes and parameter values run the same code and are INFERRED ([Limits common to the effects](../effects.md#limits-common-to-the-effects), [Driving the effect parameters](../parameters-guide.md)).
- The output alpha is the input alpha, verified with input alpha other than 1 at both sizes.
- `ROUTE AVX2-ONLY`: the sine of the wave part follows the original's AVX2 and FMA route on every CPU. On a CPU without AVX2 and FMA the original takes another sine routine, which differs from it on 10.7 percent of argument bit patterns by at most 2 ulp (MEASURED on the original's two routines); those results are not reproduced. The noise follows the AVX2 body of the original.
- No approximate reciprocal instruction is used on the verified render path (MEASURED; [Effects](../effects.md), the `S_Flicker` section).
- Host buffer phase: the plan holds only `*`, which reaches none of the loops of [Limits common to the effects](../effects.md#limits-common-to-the-effects) whose order depends on a buffer phase, so no threshold applies (INFERRED from the plan above).
- Frame time: at a rate whose reduced denominator is not 1, `get_time` follows the original's arithmetic for a host with time scale num and local step den, and equality with a host's value is INFERRED ([Host values](../cli.md#host-values)).
- Noise range: `warp_noise` reports an index outside its weight table only for an overflowing coordinate (a scaled coordinate of magnitude above about 1.7e38; `Oob` in `crates/effects/kernels/warp_noise/src/lib.rs`), and the fold then leaves the node in the plan (`t_warp_noise_fold`); the executor has no kernel for it and refuses the run (`no kernel wrapper in fx_exec`, `crates/effects/fx_exec/src/exec.rs` line 547). Through the stage the coordinate is the frame time times `rand_freq`: frame 100000 at 24 frames a second and `rand_freq` 12 gives 50000, far below that bound, so ordinary clips do not reach it (INFERRED).
- HD frames: the plan holds only `*`, so a 1920x1080 render does not depend on a host buffer phase (INFERRED). Approximate instructions: none on the verified render path (MEASURED in the `S_Flicker` section of [Effects](../effects.md)); the kernels carry no approximate-instruction model.
- NaN: `*` keeps the x86 NaN rules on the CPU and is NaN exact on the device (`NAN_EXACT_COMPILED`), so a NaN input stays on the device after the self-check (MEASURED on the special clip above). Depths 8 and 16 are not verified. Refusals: the table under "Styles and stages".

## Device path

| Operation | Device kernel | `NAN_EXACT` | Fallback to the CPU |
|---|---|---|---|
| `*` | `device_ops_pointwise.rs` (`MUL`), `pointwise_bin_f32` | yes (`NAN_EXACT_COMPILED`) | a declined shape; a NaN output only if the device's self-check reports other NaN payloads |

`*` has no `CPU_ONLY` entry. MEASURED on PoCL: the node runs on the device with the CPU's bytes and `fallback: none` on every golden row and on all 24 counted recorded renders through `fx_run` ([Effects](../effects.md), the `S_Flicker` section), and on the commands above. [Speed](../guide.md#speed) holds 0.49 / 0.45 s on the CPU and 0.54 / 0.57 s on PoCL for one 1920x1080 frame of `flicker` (MEASURED on a 12 core server under shared load; most of the time is moving the frame in and out; GPU speed is INFERRED).

## Changing the code safely

- Gates that must stay green: `python -I tools/fx_golden.py` (all groups), the tests above, `python -I tools/test_effect_rules.py`, the device self-check and device tests on PoCL, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes changed; the table holds this repository's own renders, so the repository alone cannot say whether the original agrees. `python -I tools/fx_golden.py --write` rewrites the whole table and is used only after a change whose new output was judged against the original outside the repository.
- The noise and the sine follow the original's operation order with `mul_add` at the fused sites; their tables are computed from their definitions ([No external data](../effects.md#no-external-data)). No platform math call may enter them (`fx_eval::detmath` or exact integer code; `python -I tools/test_effect_rules.py` scans for it), and a fresh NaN is the x86 default NaN through `fx_common::nanop` ([Adding an effect](../adding-an-effect.md)).
- A change to the CPU result of `*` removes it from `ALLOWLIST` unless its device kernel changes in the same patch.
- What to update: the `S_Flicker` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), line 15 of `fx_stages.tsv` and the rows of `fx_params.tsv`, `tools/fx_golden.tsv` (after the judgement above), `docs/data/effects_status.tsv`, this page, then `python -I tools/gen_docs.py`.

## Open items

- `ROUTE AVX2-ONLY`: the original's sine and noise routes for CPUs without AVX2 and FMA are not reproduced ([Effects](../effects.md), the `S_Flicker` section).
- The memo caches of the noise are not modelled; the evaluation assumes a cold cache (`assumptions` in `crates/effects/kernels/warp_noise/src/lib.rs`), and each frame's factor is computed afresh by the fold.
- Frame times other than whole frames of a 24 frames a second clock have no verified render ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
