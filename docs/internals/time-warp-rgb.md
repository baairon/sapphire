# Internals of `stage apply time_warp_rgb_autogen`

How the code of this stage is built, how a render reaches its kernels, how it is checked and what limits it has. It renders the registered style of `S_TimeWarpRGB`. The user reference is the `S_TimeWarpRGB` section of [Effects](../effects.md); its parameters are described in [Driving the effect parameters](../parameters-guide.md) and its stage in [Effects provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md). The frame requests of the two `S_Retime` stages reuse the frame route described here ([`S_Retime` internals](retime.md)).

## What it computes

Each output frame takes its red, green and blue channels from three frames of the clip: the red channel from the frame `shift_frames_red` frames after the current one, the green channel from the frame `shift_frames_green` frames after it and the blue channel from the frame `shift_frames_blue` frames after it; a negative shift reads an earlier frame. The three channels are assembled into one colour image. When `--param clamp_chroma` is below 1, the colour spread of each pixel (its largest channel minus its smallest) is computed, and the saturation of the pixel is scaled by the smaller of 1 and `--param clamp_chroma` divided by that spread, so a spread above `--param clamp_chroma` is reduced; at 1 or above the assembled image is kept as it is. The registered program then applies its mask processing and its input and output opacity handling. (INFERRED from the program constructors.)

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Time_Warp_RGB_Autogen | Time_Warp_RGB_Autogen, which calls the inner program Time_Warp_RGB | `stage apply time_warp_rgb_autogen` | `temporal` | none | no | none (`-`) |

The stage row is line 12 of `crates/neutral-handoff/data/fx_stages.tsv`; its offered parameters are lines 116 to 121 of `crates/neutral-handoff/data/fx_params.tsv`: `--param clamp_chroma`, `shift_frames_red`, `shift_frames_green` and `shift_frames_blue` (`--set`) and the two opacity popups `cset_input_opacity` and `cset_output_opacity` (`--cset`). Every other parameter of the program keeps its registered default; the mask parameters of the program are fixed by the `fixed` column of the stage row.

Refused answers and inputs (each MEASURED on this tree with the command in [Checks that run from this repository](#checks-that-run-from-this-repository) or cited from the reference pages):

| request | result | exit |
|---|---|---|
| `cset_input_opacity` 1 or 3 | `SOURCE-VARIANT-MISMATCH` (the row has no host variant for those answers) | 3 |
| `cset_output_opacity` 1 or 2 | renders | 0 |
| `--layer ID=PATH` | `LAYER-INPUT-UNSUPPORTED` | 3 |
| a frame narrower than 2 pixels | `needs a width of 2 or more` | 3 |
| an unknown parameter, a value outside the range `fx_run --describe-params` prints | `unknown param`, `bad value for` | 3 |

A keyed value is accepted on every offered parameter ([Keyframed values](../parameters-guide.md#keyframed-values)); the shifts are integers and hold until the next key.

## Code map

| part | path | role |
|---|---|---|
| program constructors | `crates/effects/fx_eval/src/programs_native.rs:1168` (`eff_time_warp_rgb`), `:1215` (`eff_time_warp_rgb_autogen`), `:2231` (`layout_time_warp_rgb_autogen`) | the inner program, the registered wrapper and its layout row, built as Rust constructor code |
| mask helpers | `crates/effects/fx_eval/src/programs_native.rs:1628`, `:1652`, `:1674` (`defun_combine_masks`, `defun_mocha_mask`, `defun_process_masks`) | the mask processing the wrapper calls |
| parameter and class facts | `crates/effects/rtdata/src/styles/time_warp_rgb.rs`, listed in `crates/effects/rtdata/src/styles/mod.rs:49` | registered parameters, ranges, input records and the style's popup answers |
| shared class rows | `crates/effects/rtdata/src/classes.rs`: `CLASS_FLAGS` (line 7), `COMBOS` (line 168), `COMBOS_EXEC` (line 1209), rows for `element_of`, `v3`, `max_element`, `min_element` and `sat_scale` | class flags and the combination tables the compiler reads |
| executor, pointwise ops | `crates/effects/fx_exec/src/kernels.rs:10` (`SUPPORTED`: `element_of`, `v3`, `sat_scale`, `-`, `/`, `min`) | kernel wrappers of the pointwise ops |
| executor, wired ops | `crates/effects/fx_exec/src/kernels2.rs:13` (`WIRED`: `max_element`, `min_element`) | kernel wrappers of the wired ops |
| frame service | `crates/effects/fx_exec/src/kernels.rs:16` (`HOST_CLASSES`), `crates/effects/fx_exec/src/exec.rs:660` (`get_frame`), `crates/effects/fx_exec/src/twframe.rs:269` (`route_request`) | a `get_frame` node is answered from the frames the caller hands in; with `--tw-route` the request is the frame and stored time the frame route computes |
| `fx_run` options | `crates/effects/fx_exec/src/fxrun.rs:596` (`--tw-route`), `:601` (`--frame-words`) | the frame route fields and the served frames |
| kernel crates | `crates/effects/kernels/ops_element` (`element_of`, `max_element`, `min_element`), `crates/effects/kernels/ops_vec` (`v3`), `crates/effects/kernels/sat_scale`, `crates/effects/kernels/ops_arith` (`-`, `/`), `crates/effects/kernels/ops_minmax` (`min`) | the CPU kernels |
| device entries | `crates/effects/fx_exec/src/device_ops.rs:87`, `:89`, `:91` (`SUB`, `DIV`, `MIN`), `:99` (`V3`), `:111` to `:113` (`ELEMENT_OF`, `MAX_ELEMENT`, `MIN_ELEMENT`), `:123` (`SAT_SCALE`); launches in `crates/effects/fx_exec/src/device_ops_pointwise.rs` | the device kernels of the plan's ops |
| stage | `crates/neutral-handoff/src/fxstage.rs:1647` (the clip loop of this stage), `:1410` (`tw_shifts`) | reads the clip, hands each frame the frames at its shifts |
| stage and parameter rows | `crates/neutral-handoff/data/fx_stages.tsv:12`, `crates/neutral-handoff/data/fx_params.tsv:116` to `:121` | host arrangement and offered parameters |
| computed tables | none | the plan reads no computed table |
| tests | `crates/effects/fx_exec/tests/t2.rs`, `t_int29_tw.rs`, `t_int29_twf.rs`, `device_pointwise.rs`; `crates/neutral-handoff/src/fxstage.rs` unit tests `time_warp_route_duration_is_the_clip_length` and `requested_route_equals_the_time_warp_route`; `crates/neutral-handoff/tests/fx_stage_parity.rs` | see [Checks that run from this repository](#checks-that-run-from-this-repository) |

The nodes of the compiled plan are MEASURED with the device report (below). With `--param clamp_chroma` 0.5 each frame runs `element_of` three times and `v3`, `max_element`, `min_element`, `-`, `/`, `min` and `sat_scale` once; at the default parameters, as on the golden rows, the plan holds `element_of` three times and `v3` once a frame (MEASURED with the defaults report command below: `device: element_of 9, v3 3; fallback: none` after 3 frames). The `dissolve` and `coerce_up` of the wrapper and the `if>` select are in the program but not among the nodes of these plans (INFERRED: they fold at compile time with the constant mask). The registered defaults of two of the three shifts are not 0 (`fx_run --describe-params Time_Warp_RGB_Autogen` prints them), so the plain, threaded and special golden rows read the neighbouring frames and reach the clamp at the first and last frame; the registered `--param clamp_chroma` default leaves the clamp branch out, so no golden row reaches `max_element`, `min_element`, `-`, `/`, `min` or `sat_scale` (open item).

## Render flow

1. `sapphire stage apply time_warp_rgb_autogen --width W --height H --depth D [--fps N[/D]] [--param ID=V]...` checks every parameter against the list `stage list --json` prints (built from `fx_run --describe-params` and `fx_params.tsv`) and refuses a bad value, an unknown id or a `--layer` with exit 3 before any frame is read.
2. The stage reads the whole clip into memory, each frame converted to 'ae32' words (a byte permutation at depth 32, a conversion through f32 at depths 8 and 16) (`crates/neutral-handoff/src/fxstage.rs:1647`). MEASURED: the peak resident memory of a 320x180 render grows by about the size of the clip from 3 to 200 frames (the memory command under [Checks that run from this repository](#checks-that-run-from-this-repository)); the "Temporal stages" paragraph of [Effects provider](../stages.md#effects-provider) states the same. The clip is held as 16 bytes a pixel: 33,177,600 bytes a 1920x1080 frame, about 8.0 GB for 240 frames; there is no streaming mode and no limit, and an allocation the system refuses ends the process (INFERRED).
3. For each output frame f it takes the three shifts at that frame (a given or keyed value, else the registered default) and hands the runner, for each shift s, the clip frame min(max(f + s, 0), last) as an in-memory `--frame-words` entry: a request before the first frame reads frame 0 and a request after the last frame reads the last frame.
4. The runner arguments are the program, `--size`, `--geom` and the frame index (`--host-frame`, `--ctx-time`), then the options of the `host` column, the placement entry of the `placement` column (empty at 50x7 and 64x32) and the options of the `fixed` column, among them the mask parameters and `--tw-route N,0,1,1,0,0,1` with N replaced by the clip length; with `--fps` the stage also hands `get_time` ([Effects provider](../stages.md#effects-provider)).
5. `fx_exec` builds the program graph from the native constructors and compiles the plan. Each `get_frame` node asks the frame route for its request (frame and stored time) and is answered from the served frame words; an unanswered request ends the run with a named error.
6. The kernels run node by node: `element_of` takes channel 0, 1 or 2 of each served frame, `v3` assembles them, and with `--param clamp_chroma` below 1 `max_element`, `min_element`, the arithmetic ops and `sat_scale` limit the spread. The parallel kernels split rows over `SAPPHIRE_THREADS` workers for frames of at least `SAPPHIRE_PAR_MIN` samples; the output does not depend on either (the `t` golden rows).
7. With `--device` or `SAPPHIRE_DEVICE` naming a device, each node whose op passed the self-check runs on the device; a node the kernel declines, or a NaN output of an op that is not NaN exact on that device, runs on the CPU ([Device path](#device-path)).
8. The output words are relaid to the frames/2 planes and written, one output frame per input frame.

`fx_run Time_Warp_RGB_Autogen` runs the same plan with the same options given by hand; the stage builds them in `crates/neutral-handoff/src/fxstage.rs` and needs no `fx_run` process.

## How parity is judged

An effect counts as rebuilt when every registered style passes seven checks on the final binary: program text, builtin classes, data read, literals, compiled plan, combination tables and execution ([What rebuilt means](../effects.md#what-rebuilt-means)).

- 22 verified comparisons, 22 recorded renders and 22 re-rendered units, all byte identical by full SHA-256 (`docs/data/effects_status.tsv`, line 18, and the status table of [Effects](../effects.md)). No comparison differs from the original.
- Through the stage: 10 of the 22 counted recorded renders use the stage's host arrangement and are byte identical through `sapphire stage apply` at depth 32 (README "Effects and stages" table; [Effects provider](../stages.md#effects-provider)). The other 12 use a different host arrangement and are not claimed for the stage. `crates/neutral-handoff/tests/fx_stage_parity.rs` renders those 10 when the stored data is present.
- No approximate reciprocal instruction is used on the verified render path (MEASURED, the `S_TimeWarpRGB` section of [Effects](../effects.md)). One effect instance runs per process, with a fresh frame cache per render. The repository runs one `Time_Warp_RGB_Autogen` instance a stage frame, so the frame route's frame cache (`FrameCache`, `crates/effects/fx_exec/src/twframe.rs:229`: request entries keyed by the request's cache key, released at the end of a render unless the host context keeps them, `end_of_render` at line 247) starts empty for every render (INFERRED from the code). Whether a host that keeps the entries across renders changes the output is not recorded in the repository.
- The other 12 counted recorded renders differ from the stage in host options that the repository does not record: `crates/neutral-handoff/data/fx_stages.tsv` holds only the arrangement the stage runs, and the 22 recorded renders are outside the repository. None of the 22 is a control (`docs/data/effects_status.tsv` line 18: 22 counted, 22 recorded).

## Checks that run from this repository

Run from the repository root after the README "Build" step. Scratch files go to `target/demo/` (git ignored). The worked render needs ffmpeg on `PATH`; the device checks need an OpenCL platform such as PoCL and print `NOT RUN` (or, with `SAPPHIRE_REQUIRE_DEVICE=1`, fail) without one.

| check | command | expected result | exit |
|---|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | `target/release/sapphire` and `target/release/fx_run` | 0 |
| stage listed | `stage list --json` filtered to the stage | one stage, kind `temporal`, 6 parameters, no layer | 0 |
| parameters | `fx_run --describe-params Time_Warp_RGB_Autogen` | JSON with 18 parameters | 0 |
| worked render | `stage apply time_warp_rgb_autogen` on 3 frames of 64x32 | 98304 bytes written | 0 |
| refusals | `cset_input_opacity=1`, `--layer` | `SOURCE-VARIANT-MISMATCH`, `LAYER-INPUT-UNSUPPORTED` | 3, 3 |
| golden hashes | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 5 rows of this stage | 0 |
| crate and executor tests | `cargo test --release --offline -p ops_element -p ops_vec -p sat_scale -p ops_arith -p ops_minmax` and the `fx_exec` and `neutral-handoff` tests below | every test passes; tests that need stored data print `NOT RUN` | 0 |
| conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply time_warp_rgb_autogen"` | L1 14 passed, 2 failed (the two 1x1 cases, exit 3), L3 33 passed | 1 |
| memory | `/usr/bin/time -f 'maxrss_kb=%M'` around the stage at 3 and 200 frames of 320x180 | the peak grows with the clip | 0 |
| device golden hashes | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | the same 183 hashes | 0 |
| device tests | `cargo test --release --offline -p fx_exec --test device_pointwise -- --test-threads=1` | every pointwise op passes its self-check | 0 |

```sh
target/release/sapphire stage list --json | python -I -c "import json,sys; d=json.load(sys.stdin); [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in d['stages'] if s['id']=='time_warp_rgb_autogen']"
target/release/fx_run --describe-params Time_Warp_RGB_Autogen
```

The first prints `time_warp_rgb_autogen temporal 6 []`; the second prints one JSON entry per registered parameter (18) with `name`, `type`, `lanes`, `min`, `max` and `default`. Both exit 0 (MEASURED).

```sh
mkdir -p target/demo
ffmpeg -v error -y -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/twrgb_in.raw
target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param shift_frames_red=2 --param shift_frames_green=-2 --param shift_frames_blue=1 --param clamp_chroma=0.5 < target/demo/twrgb_in.raw > target/demo/twrgb_out.raw
target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param cset_input_opacity=1 --in target/demo/twrgb_in.raw --out target/demo/twrgb_r1.raw # rc 3
target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --layer To=target/demo/twrgb_in.raw --in target/demo/twrgb_in.raw --out target/demo/twrgb_r2.raw # rc 3
```

The render exits 0 and writes 98304 bytes (3 frames of 64x32 at 16 bytes a pixel). The two refusals exit 3 with `SOURCE-VARIANT-MISMATCH` and `LAYER-INPUT-UNSUPPORTED` on standard error and write no frame (MEASURED).

```sh
python -I tools/fx_golden.py
grep -E '^time_warp_rgb_autogen' tools/fx_golden.tsv
```

`fx_golden.py` renders all 27 stages and ends with `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0 (MEASURED). The grep lists the 5 rows of this stage: `50x7` and `64x32` (plain), `50x7t` and `64x32t` (threaded, equal to the plain rows) and `40x12s` (special values). The stage has no variant row. A difference prints `DIFFERS time_warp_rgb_autogen SIZE` and exit 1.

```sh
cargo test --release --offline -p ops_element -p ops_vec -p sat_scale -p ops_arith -p ops_minmax
cargo test --release --offline -p fx_exec --test t2 --test t_int29_tw --test t_int29_twf
cargo test --release --offline -p neutral-handoff
```

The kernel crates are default members (`cargo test --release` runs them). `t2` holds the hand-evaluated `get_frame` cases of the inner program, `t_int29_tw` the frame request of the verified render path and `t_int29_twf` the frame route port against stored emulator cases; the tests that need stored data print a `NOT RUN` line and pass. The `neutral-handoff` run includes the stage unit tests `time_warp_route_duration_is_the_clip_length` and `requested_route_equals_the_time_warp_route` and `tests/fx_stage_parity.rs`, which prints `NOT RUN` without `VEC_DIR`. Expected exit 0 for each ([Tests](../effects.md#tests)).

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply time_warp_rgb_autogen"
```

L1 reports 14 passed and 2 failed: the `8 bit 1x1` and `16 bit 1x1` cases, each `needs a width of 2 or more`, exit 3 (the documented size refusal); L3 reports 33 passed; the tool prints `result: fail (exit 1)` and exits 1 (MEASURED).

```sh
python -I -c "import struct,sys; sys.stdout.buffer.write(struct.pack('<f',0.5)*(320*180*4*200))" > target/demo/twrgb_long.raw
head -c 2764800 target/demo/twrgb_long.raw > target/demo/twrgb_3.raw
/usr/bin/time -f 'maxrss_kb=%M' target/release/sapphire stage apply time_warp_rgb_autogen --width 320 --height 180 --depth 32 --fps 24 --in target/demo/twrgb_3.raw --out target/demo/twrgb_3_out.raw
/usr/bin/time -f 'maxrss_kb=%M' target/release/sapphire stage apply time_warp_rgb_autogen --width 320 --height 180 --depth 32 --fps 24 --in target/demo/twrgb_long.raw --out target/demo/twrgb_long_out.raw
```

The memory check needs GNU `time` (Linux). Both renders exit 0 and print a `maxrss_kb=` line, about 29,500 KiB for 3 frames and about 208,000 KiB for 200 frames (MEASURED; the figures move by a few hundred KiB from run to run), a growth of about 178,000 KiB, about 182 MB. A clip of 200 frames of 320x180 holds 184,320,000 bytes at 16 bytes a pixel, so the growth is the clip read whole.

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_pointwise -- --test-threads=1
SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param shift_frames_red=2 --param shift_frames_green=-2 --param shift_frames_blue=1 --param clamp_chroma=0.5 --device gpu --in target/demo/twrgb_in.raw --out target/demo/twrgb_dev.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --device gpu --in target/demo/twrgb_in.raw --out target/demo/twrgb_def.raw
cmp target/demo/twrgb_out.raw target/demo/twrgb_dev.raw
```

With an OpenCL platform the device run of `fx_golden.py` gives the same 183 hashes, exit 0; the first run of a build pays the self-check of every allowlisted op ([Rendering on a device](../guide.md#rendering-on-a-device)). The report run prints one `device: ` line naming the device and one line per frame with the running node counts, the last `device: - 3, / 3, element_of 9, max_element 3, min 3, min_element 3, sat_scale 3, v3 3; fallback: none`, exit 0, and `cmp` prints nothing, exit 0 (MEASURED on PoCL). Without a device the first command exits 1 with the device refusal of every case and the second fails, because `SAPPHIRE_REQUIRE_DEVICE=1` turns `NOT RUN` into a failure.

CI (`.github/workflows/ci.yml`): `cargo test --release` (line 77, the default members), `python -I tools/fx_golden.py` on Linux, macOS and Windows (line 88), the `fx_eval` and `fx_exec` tests on Linux (line 101), and in the device job on PoCL the `fx_exec` device tests including `device_pointwise` (line 192) and the device run of `fx_golden.py` (line 193).

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, and the recorded renders and the stored test data are not in it. Without `VEC_DIR`, `crates/neutral-handoff/tests/fx_stage_parity.rs` (the 10 stage renders against their recorded renders), the stored cases of `t_int29_twf` and the stored-data cases of `t2` and `t_int29_tw` print `NOT RUN` and pass. The full report, with the runs behind each figure, is identified by its SHA-256 in [Evidence](../effects.md#evidence). The same holds for what a host returns when a shift reaches before the first or past the last frame of its layer (the stage serves frame 0 or the last frame), for the host options of the 12 counted renders that the stage does not run, and for the effect of a frame cache kept across renders.

## Verified scope and limits

- Verified: the 22 recorded renders at the sizes of the stage row's `placement` column (50x7 and 64x32), with the shifts and `--param clamp_chroma` values listed in the `S_TimeWarpRGB` section of [Driving the effect parameters](../parameters-guide.md), at depth 32. Other sizes, values, frame rates and clip lengths run the same code and are INFERRED.
- Frames outside the clip: before the first frame the route clamps to frame 0 and past the last frame the stage serves the last frame; this is a choice of the stage and is INFERRED beyond the measured shifts ([Effects provider](../stages.md#effects-provider)).
- Host buffer phase: none of the phase-dependent loops listed in [limits common to the effects](../effects.md#limits-common-to-the-effects) is on this plan (INFERRED from that list and the plan's ops). A 1920x1080 render is therefore phase independent (INFERRED).
- NaN: the `40x12s` golden row renders a clip with infinities, both default NaN patterns and subnormal values; NaN payload and sign bits are not verified against the original ([limits common to the effects](../effects.md#limits-common-to-the-effects)).
- `ROUTE AVX2-ONLY`: the kernels reproduce the original's AVX2 and FMA tier.
- Opacity: only `cset_input_opacity` 2 renders; `cset_output_opacity` renders both of its items (MEASURED).
- Depths 8 and 16 go through a conversion that is not verified.

## Device path

| op | device kernel | NaN exact | note |
|---|---|---|---|
| `element_of` | `ELEMENT_OF`, `crates/effects/fx_exec/src/device_ops_pointwise.rs` | yes (`NAN_EXACT_WRITTEN`) | |
| `v3` | `V3`, same file | yes (`NAN_EXACT_WRITTEN`) | |
| `max_element`, `min_element` | `MAX_ELEMENT`, `MIN_ELEMENT`, same file | no | NaN outputs rerun on the CPU |
| `sat_scale` | `SAT_SCALE`, same file | no | NaN outputs rerun on the CPU |
| `-`, `/`, `min` | `SUB`, `DIV`, `MIN`, same file | yes (`NAN_EXACT_WRITTEN`) | |

No op of this plan has a `CPU_ONLY` entry (`crates/effects/fx_exec/src/device_ops.rs`, `CPU_ONLY` is empty). A node runs on the CPU when its kernel declines the tag or shape (`unsupported`), when its op failed the self-check on that device (`not_allowlisted`), when the device budget would be exceeded (`memory`) or the driver reports an error (`opencl_error`), and for a NaN output of an op that is not NaN exact there (`nan_output`). MEASURED on PoCL only: every node runs on the device with the CPU's bytes and `fallback: none` on every golden row, the special row included (the `S_TimeWarpRGB` section of [Effects](../effects.md)). Timing of one 1920x1080 frame ([Speed](../guide.md#speed), PoCL is an OpenCL runtime on the CPU and shows correctness, not GPU speed): CPU 0.74 / 0.79 s, PoCL 0.89 / 0.80 s.

## Changing the code safely

- Gates that stay green: `python -I tools/fx_golden.py` (all groups, on the CPU and with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1`), the crate and executor tests above, `python -I tools/test_effect_rules.py`, the device self-check and mutation tests (`cargo test --release -p fx_exec --lib device` and the device test files, one thread), `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`.
- A golden difference means the output bytes changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees; the recorded renders under `VEC_DIR` and `fx_stage_parity.rs` can. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` from the current build; it is used only after a deliberate change has been checked against the recorded renders, never as a check.
- Kernel rules ([Adding an effect](../adding-an-effect.md)): no platform math library call in effect code (`tools/test_effect_rules.py`, exceptions in `tools/fx_math_allow.tsv`); a fresh NaN canonicalized to 0xffc00000 before a sign operation; two-NaN picks through `fx_common::nanop`. A CPU parity change to `element_of`, `v3`, `max_element`, `min_element`, `sat_scale` or the arithmetic ops removes the op from `ALLOWLIST` in `crates/effects/fx_exec/src/device_ops.rs` unless its device kernel changes in the same patch.
- Docs and data to update with a change: the `S_TimeWarpRGB` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), the stage row and [Effects provider](../stages.md#effects-provider), `docs/data/effects_status.tsv` (then `python -I tools/gen_docs.py`), and this page.

## Open items

- The stage keeps the whole clip in memory (about 8.0 GB for 240 frames at 1920x1080) and has no streaming mode; a window of the largest positive plus the largest negative shift plus 1 frames would bound it (`crates/neutral-handoff/src/fxstage.rs:1647`).
- No golden row sets the shifts or `--param clamp_chroma`: the clamp ops (`max_element`, `min_element`, `-`, `/`, `min`, `sat_scale`) are reached by no golden row and are checked in the repository only by the device report command above and by `device_pointwise`; the repository states no plan for a variant row.
- 12 of the 22 counted recorded renders use a host arrangement the stage does not run, so they are not claimed through the stage (README "Effects and stages" table).
- `cset_input_opacity` 1 and 3 are refused: the stage row has no host variant for them, because no recorded render sets them (`crates/neutral-handoff/data/fx_stages.tsv:12`, `host_variants` `-`).
- Shifts beyond the measured ones and frames requested outside the clip are INFERRED ([Effects provider](../stages.md#effects-provider)).
