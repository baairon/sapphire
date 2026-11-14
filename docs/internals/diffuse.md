# Internals of `stage apply diffuse_autogen`

This page describes how this stage is built, rendered, checked and changed in this repository: its program, its stage row, the kernels and device kernels it reaches, the checks that cover it and its limits. It renders the registered style of `S_Diffuse`. The user reference is the `S_Diffuse` section of [Effects](../effects.md), the `S_Diffuse` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture the effects share is in [Effect internals](README.md). The ripmap and the relative warp sampler described here are also used by `S_DistortRGB` ([`S_DistortRGB` internals](distort-rgb.md)).

## What it computes

Each output pixel is sampled from the source at a position moved by a per-pixel offset. The offsets are two planes of a pseudo random pattern (`static2`): pixel i of the frame takes the hash of i plus a seed, four rounds over a hash table computed at first use (`HashTable` and `hash_1` in `crates/effects/fx_common/src/shake_natives.rs`, the function h of docs/effects.md line 949), and two bit fields of that word give the x and the y offset, evenly spaced values between -0.5 and 0.5 (the exact formula is the module documentation of `crates/effects/kernels/static2/src/lib.rs`). The offsets are scaled by `diffuse_amount`, by the per-axis `rel_amount` and by the mask. In pixels, a host value of `diffuse_amount` is the largest displacement (MEASURED: 8 at 64x32 spreads a one-pixel column over up to 8 pixels), and a record value r (`diffuse_amount_host`) moves pixels by up to about r x W / 4 on a frame W pixels wide (MEASURED: 0.2 spreads the column over up to 4, 7 and 13 pixels at widths 64, 128 and 256, bilinear sampling included). The source is first built into a ripmap (a pyramid of planes halved along x and along y independently) and sampled at the moved positions with the edge rule of `wrap` on each axis (No, Tile or Reflect). All four planes of the source, alpha included, are scattered the same way (MEASURED: on the golden synthetic clip, whose alpha varies, the output alpha differs from the input alpha). The seed is the frame's pixel count W x H times the host frame index, a 32-bit product with wrapping (`crates/effects/fx_exec/src/fxrun.rs` line 800); the stage passes the 0-based index of the frame in the clip it is given as `--host-frame`, and `fx_run --static2-seed N` sets the seed instead (line 686). Each frame of a clip therefore takes a different pattern (MEASURED: three equal input frames give three different output frames).

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Diffuse_Autogen | `Diffuse_Autogen` | `stage apply diffuse_autogen` | `temporal` | none | `false` | `-` |

The stage offers 15 parameters (`sapphire stage list --json`); their routes are lines 52 to 66 of `crates/neutral-handoff/data/fx_params.tsv`. `diffuse_amount` and `blur_mocha_mask` are host values (`--host-param`), and `diffuse_amount_host` and `blur_mocha_mask_host` give the same two program parameters as record values (`--set`). The stage offers no opacity popup.

Refused answers (MEASURED on this build):

| Input | Result |
|---|---|
| a frame narrower than 2 pixels | exit 3, `needs a width of 2 or more` |
| `--layer` with any id | exit 3, `LAYER-INPUT-UNSUPPORTED`, before any frame is read |
| `cset_opacity` or any id the stage does not offer | exit 3, `unknown param` |
| `diffuse_amount` and `diffuse_amount_host` together | exit 3, `given both as --host-param and --set` |
| a value outside the printed range of a one-lane parameter | exit 3, `bad value for` |
| a partial input frame | exit 4 |
| a keyed value without `--fps` | exit 2, `keyframes need --fps` |

A plain host value off the 16.16 grid is not refused by the stage: it is put on the grid and the stage prints a `note:` line, for example ``note: `diffuse_amount` 4.00001 is put on the 16.16 host grid as 4.0000152587890625`` (MEASURED; `apply_profile` in `crates/neutral-handoff/src/fxstage.rs`); docs/cli.md line 359 lists `HOST-VALUE-OFF-GRID` for `fx_run` only.

## Code map

| Part | Path | Role |
|---|---|---|
| Style constructor | `crates/effects/fx_eval/src/programs_native.rs`, `eff_diffuse_autogen` | the registered program: the mask processing, the inner scatter program and the tracker mask switch |
| Inner program | same file, `eff_diffuse` | `warp_rel` of the ripmap of the source by `static2` times the amount, the relative amount and the mask |
| Helper programs | same file, `defun_process_masks`, `defun_mocha_mask`, `defun_combine_masks`, `defun_warp_mask` | the mask input and the tracker mask |
| Layout row | same file, `layout_diffuse_autogen` | parameter grouping of the style |
| Recorded host facts | `crates/effects/rtdata/src/styles/diffuse.rs` | registered parameters, ranges, input indices and normalize scales of the two programs |
| Class rows | `crates/effects/rtdata/src/classes.rs` | flag words, argument kinds and combination tables of `static2`, `warp_rel`, `make_mipmap` and `*` |
| `static2` kernel | `crates/effects/kernels/static2` | the two offset planes from the hash of the pixel index plus the seed |
| `make_mipmap` kernel | `crates/effects/kernels/make_mipmap` | the ripmap of the source, with its padded plane layout |
| `warp_rel` kernel | `crates/effects/kernels/warp_rel`, `crates/effects/kernels/warp_rel_sampler`, `crates/effects/kernels/warp_sampler`, `crates/effects/kernels/libm_fmod` | the relative warp: per row, sample the ripmap at the pixel position plus the offset, with the edge rule of each axis |
| `*` kernel | `crates/effects/kernels/ops_arith` | the product of the offsets and the folded scale |
| Executor entries | `crates/effects/fx_exec/src/kernels.rs` (`SUPPORTED`, `binary_k`), `crates/effects/fx_exec/src/kernels2.rs` (`WIRED`, `static2_k`, `make_mipmap_k`, `warp_rel_k`) | the call frame of each class into its crate |
| Seed | `crates/effects/fx_exec/src/fxrun.rs` | the `static2` seed from `--host-frame` (the pixel count times the frame index, 32-bit wrapping) or `--static2-seed` |
| Device entries | `crates/effects/fx_exec/src/device_ops.rs` (`DEVICE_OPS`, `ALLOWLIST`, `NAN_EXACT`), `crates/effects/fx_exec/src/device_ops_neighbourhood.rs` (`STATIC2`), `crates/effects/fx_exec/src/device_ops_ripmap.rs` (`make_mipmap_accepts`, `make_mipmap_run`), `crates/effects/fx_exec/src/device_ops_warp.rs` (`warp_rel_accepts`, `warp_rel_run`), `crates/effects/fx_exec/src/device_ops_pointwise.rs` (`MUL`) | the device kernel of each op |
| OpenCL sources | `crates/effects/fx_device/src/kernels_neighbourhood.rs` (`static2_f32`), `crates/effects/fx_device/src/kernels_ripmap.rs` (`RIPMAP_MIPMAP`), `crates/effects/fx_device/src/kernels_warp.rs` (`warp_warp_f32`), `crates/effects/fx_device/src/kernels_pointwise.rs` (`pointwise_bin_f32`) | kernel text, registered in `crates/effects/fx_device/src/kernels.rs` |
| Stage row | `crates/neutral-handoff/data/fx_stages.tsv` line 6 | program, family, kind, source name, fixed options, host options, placement |
| Parameter rows | `crates/neutral-handoff/data/fx_params.tsv` lines 52 to 66 | id, option form and program parameter of each offered parameter |
| Stage provider | `crates/neutral-handoff/src/fxstage.rs` (`apply_profile`, `run_fx`) | the per-frame runner command line and the in-process run |
| Golden rows | `tools/fx_golden.tsv` lines 27 to 31 | the special row `40x12s`, the plain rows `50x7` and `64x32`, the threaded rows `50x7t` and `64x32t` |
| Tests | see [Checks that run from this repository](#checks-that-run-from-this-repository) | |

The ops of the compiled plan are `static2`, `*`, `make_mipmap` and `warp_rel`, one node each per frame (MEASURED with the device report below: `device: * 1, make_mipmap 1, static2 1, warp_rel 1; fallback: none` after the first frame). The mask processing folds away: the stage connects no mask and no tracker project, so the mask is a constant and the product of the amount, the relative amount and the mask becomes one constant operand of `*`. Setting `mocha_mask_opacity` changes that constant and adds no node (MEASURED). No table is computed for this effect: the `static2` hash is exact integer code.

## Render flow

1. `sapphire stage apply diffuse_autogen --width W --height H --depth D [--fps N[/D]] [--param ID=V]...` finds the row of `crates/neutral-handoff/data/fx_stages.tsv` and checks, before any frame is read: no `--layer`, the host variants column (`-` here), a width of 2 or more, the parameters against the ranges `fx_run --describe-params` prints, and the frame rate.
2. Each input frame is read whole and handed to the runner in memory as 'ae32' words (A, R, G, B per pixel, f32, straight alpha); at depth 32 this is a byte permutation of the planar G, B, R, A frame, and depths 8 and 16 convert through f32 ([Effects provider](../stages.md#effects-provider)).
3. `run_fx` builds the runner's command line: the program, `--size`, the source and output words, `--geom` from the frame size, `--host-frame` and `--ctx-time` from the 0-based frame index, then the `host` column (four source planes, `--src-planes 4`), the `placement` column for the size (empty at the measured sizes 50x7 and 64x32, and the same elsewhere), the `fixed` column (the three mask parameters of the unconnected mask input), the parameters, `--ns-default-unbound` and `--do-postdiv`.
4. The runner (`fx_exec::fxrun`, the same code as `fx_run`) builds the program from the constructors, compiles it for the binding and folds the constant subgraphs. The plan holds `static2` (seeded from `--host-frame`), `*` (the offsets times the folded scale), `make_mipmap` (the ripmap of the source with the `wrap` modes) and `warp_rel` (the sampler). `--do-postdiv` starts at 0; when the runner refuses it with `POSTDIV-MISMATCH`, the stage takes the derived value from the runner's result and repeats the frame ([Effects provider](../stages.md#effects-provider)).
5. Threading: `warp_rel` splits its output rows into bands across threads above `SAPPHIRE_PAR_MIN` samples, with `SAPPHIRE_THREADS` workers (1 is serial); the bands only choose which thread computes which rows, so the bytes do not depend on the thread count (`crates/effects/fx_exec/src/par.rs`). The other three ops run on one thread.
6. Device: with `--device gpu` or `SAPPHIRE_DEVICE=gpu`, each node runs on the device when its op passed the self-check; a node the kernel declines, or a NaN output of `static2` or `warp_rel` (neither is NaN exact), runs on the CPU with the same bytes ([Device path](#device-path)).
7. The output 'ae32' words are relaid to the frame format and written; the stage writes as many frames as it read. Frame N depends only on input frame N and the frame index.

`fx_run Diffuse_Autogen --size WxH --out PREFIX ...` runs the same library from a shell. It takes the host context explicitly (the options of the stage row and the per-frame options above have no defaults, and a missing one is refused by name), and `--synth SEED` fills unbound inputs ([Build and run](../effects.md#build-and-run)).

## How parity is judged

An effect counts as rebuilt when each registered style passes the seven checks of [What rebuilt means](../effects.md#what-rebuilt-means): the program text, the builtin classes, the data read, the literals, the compiled plan, the combination tables and the execution, judged against the original running in an emulator.

| Figure | Value | Source |
|---|---|---|
| verified comparisons | 31 | `docs/data/effects_status.tsv` line 5 |
| recorded renders | 54 | same row |
| re-rendered units | 54, all byte identical to their recorded render | same row; [What rebuilt means](../effects.md#what-rebuilt-means) |
| byte identical through the stage | 31 of 42 counted recorded renders | README [Effects and stages](../../README.md#effects-and-stages) |

No comparison of `S_Diffuse` differs from the original. The `S_Diffuse` section of [Effects](../effects.md) (line 215) counts one approximate reciprocal instruction, inside a logarithm, with an effect of 0 ulp on the output (MEASURED) under the bound of the approximate-instruction model: each estimate perturbed by up to Intel's stated relative error before the code's own Newton step. That logarithm is the level count of the `make_mipmap` allocator (`level_count` and `alloc` in `crates/effects/kernels/make_mipmap/src/mm2.rs`), an integer from a rounded-up logarithm of the frame size, which this repository computes from exact operations; the unit test `level_count_matches_the_libm_route_for_every_size` checks it against the library route. It is taken only for a nonzero filter operand, and this plan passes 0, so the stage's ripmap holds one level and no logarithm runs (INFERRED from `alloc`; the same site as on the [`S_BlurMotion` internals](blur-motion.md) page). The output therefore does not depend on a CPU's reciprocal estimates; a CPU outside Intel's bound is not measured. The other 11 counted recorded renders use a different host arrangement and are not claimed for the stage. Of the 54 recorded renders, 42 are counted and 12 are controls, recorded and not counted ([What rebuilt means](../effects.md#what-rebuilt-means)). Which renders are the controls, and the host options in which the 11 differ from the stage row, are kept with the recorded renders outside this repository; docs/stages.md line 341 names the kinds of difference (another host arrangement, a fractional frame, a `--prev` file outside the history chain).

## Checks that run from this repository

| Check | Command | Expected result and exit code |
|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | exit 0; `target/release/sapphire` and `target/release/fx_run` |
| test clip | `ffmpeg ... target/demo/diffuse/in.raw` (below) | exit 0; 98,304 bytes (needs ffmpeg) |
| manifest | `sapphire stage list --json` (below) | `Diffuse_Autogen S_Diffuse temporal 15 []`, exit 0 |
| parameters | `fx_run --describe-params Diffuse_Autogen` | JSON with the program's parameters, exit 0 |
| worked render | `sapphire stage apply diffuse_autogen ...` (below) | exit 0; 98,304 bytes |
| refusals | the two commands of the refusal block (below) | exit 3 each |
| golden hashes | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`, exit 0 |
| conformance | `python -I tools/conformance.py --stage "target/release/sapphire stage apply diffuse_autogen"` | the two documented 1x1 `FAIL` lines, exit 1 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 ... --device gpu` (below) | `fallback: none` on every frame, output equal to the CPU output, exit 0 (needs OpenCL) |
| device golden | `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py` | 183 equal, exit 0 (needs OpenCL) |
| kernel crates | `cargo test --release --offline -p static2 -p make_mipmap -p ops_arith` | exit 0 |
| executor and evaluator tests | `cargo test --release --offline -p fx_exec --test t9 --test t27_style_select` and `cargo test --release --offline -p fx_eval --test t23_checkout` | exit 0; tests that need stored data print `NOT RUN` (shown with `-- --nocapture`) |
| device tests | `SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_neighbourhood --test device_ripmap --test device_warp --test device_pointwise --test device_backend -- --test-threads=1` | exit 0 (needs OpenCL) |
| stage parity | `cargo test --release --offline -p neutral-handoff --test fx_stage_parity` | exit 0; with `-- --nocapture` it prints `NOT RUN: VEC_DIR is not set` |

Build, from the repository root:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
```

Expected: exit 0.

A test clip of three 64x32 frames at depth 32 and the stage's manifest entry (ffmpeg and ffprobe on `PATH`; without ffmpeg the first command fails and the later commands have no input):

```sh
mkdir -p target/demo/diffuse
ffmpeg -y -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/diffuse/in.raw
target/release/sapphire stage list --json | python -I -c "import json,sys; s=[x for x in json.load(sys.stdin)['stages'] if x['id']=='diffuse_autogen'][0]; print(s['name'], s['family'], s['kind'], len(s['params']), s['layers'])"
target/release/fx_run --describe-params Diffuse_Autogen
```

Expected: exit 0 for each; `target/demo/diffuse/in.raw` holds 98,304 bytes; the manifest line is `Diffuse_Autogen S_Diffuse temporal 15 []`; the last command prints the parameters as JSON, with their ranges and registered defaults.

The worked render, with values that verified comparisons used:

```sh
target/release/sapphire stage apply diffuse_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param diffuse_amount=48 --param rel_amount=1,1 --param wrap=2,2 < target/demo/diffuse/in.raw > target/demo/diffuse/out.raw
```

Expected: exit 0; `target/demo/diffuse/out.raw` holds 98,304 bytes.

Two refusals:

```sh
target/release/sapphire stage apply diffuse_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param diffuse_amount=48 --param diffuse_amount_host=0.5 < target/demo/diffuse/in.raw > target/demo/diffuse/both.raw
target/release/sapphire stage apply diffuse_autogen --width 64 --height 32 --depth 32 --fps 25/1 --layer To=target/demo/diffuse/in.raw < target/demo/diffuse/in.raw > target/demo/diffuse/layer.raw
```

Expected: exit 3 for each, with `diffuse_amount given both as --host-param and --set` and with `LAYER-INPUT-UNSUPPORTED`.

The golden rows of the stage and the full golden check:

```sh
grep '^diffuse_autogen' tools/fx_golden.tsv | cut -f1,2
python -I tools/fx_golden.py
```

Expected: five rows, `40x12s`, `50x7`, `50x7t`, `64x32` and `64x32t` (the special, plain and threaded groups; the stage has no variant row). The rows set no parameter, so each runs the plan of the defaults with all four ops, the special row included; then `fx_golden: 183 cases, 183 equal, 0 differ` and exit 0. A difference prints a `DIFFERS diffuse_autogen SIZE` line and exit 1.

Conformance of the stage:

```sh
python -I tools/conformance.py --stage "target/release/sapphire stage apply diffuse_autogen"
```

Expected: exit 1 with `L1 stage matrix FAIL 14 passed, 2 failed`, the two `FAIL` lines of the 1x1 cases (`needs a width of 2 or more`, exit 3), and `L3 failure injection pass 33 passed`. These are the documented size refusals of every effect stage ([Conformance](../stages.md#conformance)).

The device path (an OpenCL platform such as PoCL; without one the first command ends with exit 3 and `DEVICE-REQUIRED-UNAVAILABLE`):

```sh
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply diffuse_autogen --device gpu --width 64 --height 32 --depth 32 --fps 25/1 --param diffuse_amount=48 --param rel_amount=1,1 --param wrap=2,2 < target/demo/diffuse/in.raw > target/demo/diffuse/dev.raw
cmp target/demo/diffuse/out.raw target/demo/diffuse/dev.raw
SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py
```

Expected: exit 0 for each. The report names the device, then prints `device: * 1, make_mipmap 1, static2 1, warp_rel 1; fallback: none` and the running counts up to `* 3, make_mipmap 3, static2 3, warp_rel 3`; `cmp` prints nothing; the golden check prints `183 equal`. The first device run of a build pays the self-check of every allowlisted op ([Rendering on a device](../cli.md#rendering-on-a-device)).

The crate and executor tests (cargo, offline):

```sh
cargo test --release --offline -p static2 -p make_mipmap -p ops_arith
cargo test --release --offline -p fx_exec --test t9 --test t27_style_select
cargo test --release --offline -p fx_eval --test t23_checkout
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_neighbourhood --test device_ripmap --test device_warp --test device_pointwise --test device_backend -- --test-threads=1
cargo test --release --offline -p neutral-handoff --test fx_stage_parity
```

Expected: exit 0 for each. `static2`, `ops_arith` and the `warp_*` crates are default members; `make_mipmap`, `fx_eval` and `fx_exec` are not, so a plain `cargo test --release` does not run them. `crates/effects/fx_exec/tests/t9.rs` runs the wired classes, `static2` among them, and program-level runs of the inner program; tests in it that need stored data print `NOT RUN` and pass. `crates/effects/fx_eval/tests/t23_checkout.rs` checks which parameters of `Diffuse_Autogen` the host checks out with no mask connected. `device_neighbourhood` holds the self-check of the `static2` kernel, `device_ripmap` that of `make_mipmap`, `device_warp` that of `warp_rel`, `device_pointwise` that of `*`, and `device_backend` renders every registered program on the device and the CPU and compares the bytes. Without an OpenCL platform the device tests print `NOT RUN` and pass; with `SAPPHIRE_REQUIRE_DEVICE=1` they fail instead. The stage parity test needs `VEC_DIR` ([What needs the comparison harness](#what-needs-the-comparison-harness)). Cargo captures the output of a passing test, so the `NOT RUN` and `T2 PENDING` lines named here show only with `-- --nocapture` after the command; the exit codes are the same either way.

In CI (`.github/workflows/ci.yml`): the `tests on ...` job runs `cargo test --release` and `python -I tools/fx_golden.py` on Linux, macOS and Windows, the `fx_eval`, `fx_exec` and `make_mipmap` tests on Linux, and `tools/conformance.py --quick`; the `device backend on PoCL` job runs the device tests above and the device golden check with `SAPPHIRE_REQUIRE_DEVICE=1`.

## What needs the comparison harness

The comparison with the original runs in an emulator harness kept outside this repository, and the recorded renders of the parity record are not in it. From the repository alone the checks above confirm that this build renders its own golden hashes, that the device path equals the CPU path and that the kernels pass their unit tests; they do not compare with the original.

- `crates/neutral-handoff/tests/fx_stage_parity.rs` renders the 31 counted recorded renders that match the stage's host arrangement and compares them by full SHA-256. It reads them from `VEC_DIR/fx_parity/members.tsv` and an `fx_run` named by `SAPPHIRE_FX_RUN` or beside the test binaries; without them it prints `NOT RUN` and passes.
- Tests of `fx_eval` and `fx_exec` that compare with recorded plans, emulator rows or the program corpus read `VEC_DIR/effects/` and print a `NOT RUN` line without it ([Tests](../effects.md#tests)).
- Which recorded renders are controls, the host options of the 11 counted renders outside the stage, and the frame value a host passes to the seed are records of the harness, not of this repository.
- The full report of the comparisons is kept outside the repository; its SHA-256 is in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, the frame sizes 64x32 and 50x7 of the stage row's measured placement, the parameter values listed in the `S_Diffuse` section of [Driving the effect parameters](../parameters-guide.md), one band, full-frame geometry, the source with four planes and no mask connected. Other sizes and values run the same code and are INFERRED.
- Frame time: the plan reads no host time getter (INFERRED from the constructors); the frame index reaches the output only through the seed of the pattern.
- Host buffer phase: none of the plan's kernels takes a buffer phase: the call frames of `static2`, `make_mipmap` and `warp_rel` in `crates/effects/fx_exec/src/kernels2.rs` read no phase option, and the stage row's placement column is empty, so the output at 1920x1080 does not depend on a host buffer phase (INFERRED). [Limits common to the effects](../effects.md#limits-common-to-the-effects) lists no loop of this effect.
- NaN: a NaN output of `static2` or `warp_rel` on the device is rerun on the CPU, which keeps the x86 NaN rules; the special golden row `40x12s` (infinities, NaN, subnormal values) renders on both paths with `fallback: none` (MEASURED by the device golden check).
- `ROUTE AVX2-ONLY`: the kernels reproduce the original's AVX2 and FMA code tier; other tiers are not reproduced ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- `wrap` is a two-lane popup with the items 0 to 2. The stage range checks only one-lane values, and the evaluator clamps each lane to the registered range before the plan, as the original's compiler clamps an enforced bound (`crates/effects/rtdata/src/styles/diffuse.rs` line 54; the tests `clamp_is_lane_wise_on_vectors` and `compile_clamps_every_bound_scalar_before_the_plan` in `crates/effects/fx_eval/tests/t8.rs`): a lane of 3 renders as 2 and a lane of -1 as 0 (MEASURED). Values outside 0 to 2 are outside the verified comparisons.
- The mask input is not offered; mask tracking data is not part of the rebuild, so the tracker mask parameters act on the mask the program builds without a project (the tracker mask group of [Driving the effect parameters](../parameters-guide.md)).
- Depths 8 and 16 convert through f32 and are not verified.
- Refusals and exit codes: the table under [Styles and stages](#styles-and-stages) and [Refusal labels](../cli.md#refusal-labels).

## Device path

| Op | Device kernel | `NAN_EXACT` | Note |
|---|---|---|---|
| `static2` | `STATIC2` in `crates/effects/fx_exec/src/device_ops_neighbourhood.rs` | no | a NaN output reruns on the CPU |
| `*` | `MUL` in `crates/effects/fx_exec/src/device_ops_pointwise.rs` | yes, `NAN_EXACT_COMPILED` | NaN words of the compiled CPU instruction |
| `make_mipmap` | `make_mipmap_run` in `crates/effects/fx_exec/src/device_ops_ripmap.rs` | yes, `NAN_EXACT_COMPILED` | the ripmap stays on the device for `warp_rel` |
| `warp_rel` | `warp_rel_run` in `crates/effects/fx_exec/src/device_ops_warp.rs` | no | its kernel takes each NaN from the device's own instructions, not from tests in the CPU's operand order, so a device can return another NaN word than the CPU; NaN outputs rerun on the CPU (docs/guide.md line 120) |

All four ops are on `ALLOWLIST` and none has a `CPU_ONLY` entry. A node falls back to the CPU when its kernel declines the tag, operand shape or parameter (`unsupported`), when its op failed the self-check on the device (`not_allowlisted`), when the device budget would be exceeded (`memory`), on a driver error (`opencl_error`) or when a non NaN exact op wrote a NaN (`nan_output`); the bytes are the CPU's in every case. MEASURED on PoCL only: `fallback: none` on every golden row of the stage, the special row included (the `S_Diffuse` section of [Effects](../effects.md)), and on the worked render above. The `diffuse_autogen` row of [Speed](../guide.md#speed) holds the timing of one 1920x1080 frame: 2.13 / 1.90 s on the CPU and 0.71 / 0.62 s on PoCL; PoCL runs on the CPU, so the device cells show the cost of the path, not GPU speed.

## Changing the code safely

- Keep green: `python -I tools/fx_golden.py` (all groups), the crate tests above, `python -I tools/test_effect_rules.py`, the device tests with `SAPPHIRE_REQUIRE_DEVICE=1`, `python -I tools/check_docs.py` and `python -I tools/gen_docs.py --check`, and the CI steps that run the NaN tests on x86 and arm64.
- A golden difference means the output bytes of the stage changed. The table holds this repository's own renders, so the repository alone cannot say whether the original agrees; only the comparison harness can. `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` from the current build and is used only after a change has been judged against the original.
- A change to the CPU kernel of `static2`, `make_mipmap`, `warp_rel` or `*` removes the op from `ALLOWLIST` unless its device kernel changes in the same patch, and the device self-check then compares the two bit for bit ([Device](../adding-an-effect.md#device)).
- Effect code calls no platform math library function; the NaN rules of [Crate layout](../adding-an-effect.md#crate-layout) apply to any new arithmetic in these crates (`fx_common::nanop` for two-NaN picks, a fresh NaN canonicalized to `0xffc00000`), and `tools/test_effect_rules.py` scans for both.
- A change to the stage row or the offered parameters updates `crates/neutral-handoff/data/fx_stages.tsv` and `fx_params.tsv`, the `S_Diffuse` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), and then `python -I tools/gen_docs.py` regenerates `docs/index/`.

## Open items

- A lane of `wrap` outside 0 to 2 is clamped, not refused (MEASURED above); refusing it, as the stage refuses an out of range one-lane value ([Parameter discovery](../cli.md#parameter-discovery)), would be a change of the stage.
- The seed takes the frame index of the clip the stage is given, counted from 0; the stage has no option for a frame offset, so a clip cut from a longer timeline takes other patterns than the same frames at their place in it (`fx_run --host-frame` or `--static2-seed` set the seed of one run). Which frame value a host application passes is not recorded in this repository.
- Sizes other than 64x32 and 50x7, and the original's other code tiers, are INFERRED or not reproduced (`ROUTE AVX2-ONLY`).
