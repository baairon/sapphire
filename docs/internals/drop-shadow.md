# Internals of `stage apply drop_shadow`

This page covers the code of this stage: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. It is the one style of `S_DropShadow`. The user reference is the `S_DropShadow` section of [Effects](../effects.md), the `S_DropShadow` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md). The shadow is blurred by the blur kernel that [`S_Blur` internals](blur.md) describes in full (passes, gains, threading, host buffer phase); this page names only what differs.

## What it computes

The shadow is made from the alpha of the input: the alpha plane is taken as a matte, joined with the tracker mask (`combine_masks`, `mocha_mask_opacity`), shifted by `shift` through a ripmap and a relative warp, and blurred by three box passes of the blur kernel with the width `shadow_blur` and the gain `shadow_opacity`. The shadow is coloured with `color` and placed under the foreground, the input scaled by `fg_opacity`. With `comp_premult` at 1 the two are combined as premultiplied colour (`comp_premult`), at 0 by a straight blend weighted by `fg_opacity` times the matte (`dissolve`). The stage takes no `Matte` or `Background` layer, so the matte is the input's own alpha and the shadow is composited over nothing else; on an opaque input the foreground hides the shadow and the output equals the input (MEASURED below). A shift of 0 folds the warp away (the blur then reads the base level of the ripmap), and a `shadow_blur` or `shadow_opacity` of 0 folds the blur or the whole shadow branch out of the plan.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Drop_Shadow | Drop_Shadow (registered without a wrapper program) | `stage apply drop_shadow` | `temporal` | none | `false` | `-` |

The values are the row of `crates/neutral-handoff/data/fx_stages.tsv` (line 8); the stage's source input is the program's `Foreground`. The stage offers 10 parameters (rows 77 to 86 of `crates/neutral-handoff/data/fx_params.tsv`): `shadow_blur` and `shift_host` as host values (`--host-param`; `shift_host` is the program parameter `shift` given in host units, its second lane negated), `cset_opacity` as a host popup answer (`--cset`), and `color`, `shadow_opacity`, `shift`, `fg_opacity`, `comp_premult`, `combine_masks` and `mocha_mask_opacity` as record values (`--set`). The `fixed` column holds the other mask inputs of the program at the values of the recorded runs: the matte taken from the alpha and not inverted, and the tracker mask not resized, shifted, inverted or bypassed.

Units (INFERRED from `crates/effects/fx_eval/src/units.rs`): `shift_host` is in pixels, as a host slider shows it; the stage passes it as `--host-param` and `fx_run` converts it to the record value `shift` as host x 2 / W (normalize scale 1), so a record value of 1 is half the frame width, with the second lane negated. The negation is the original's conversion, matched bit for bit by the test `nss1_drop_shadow_shift_row_and_lanes` in units.rs and by `crates/effects/fx_eval/tests/t24_units_lane1.rs` against recorded conversions; it turns the host's downward y into the program's upward y (INFERRED). `shadow_blur` is converted with its normalize scale and the blur's own width factor cancels the frame width, so the shadow radius in pixels equals the host value of `shadow_blur`, as for `blur_amount` in [`S_Blur` internals](blur.md).

Refusals (MEASURED on this build with the commands below):

| input | result | exit |
|---|---|---|
| `--param cset_opacity=1` or `--param cset_opacity=3` | `SOURCE-VARIANT-MISMATCH` | 3 |
| `--param cset_opacity=4` | `bad value for cset_opacity: 4 is outside 1 to 3` | 3 |
| `--param shift=...` together with `--param shift_host=...` | `shift given both as --host-param and --set` | 3 |
| `--layer Matte=PATH`, `--layer Background=PATH` or any other layer | `LAYER-INPUT-UNSUPPORTED` | 3 |
| `--width 1` | `stage apply drop_shadow` refuses it: `needs a width of 2 or more` | 3 |
| a keyed value without `--fps` | `keyframes need --fps` | 2 |

## Code map

| part | path | role |
|---|---|---|
| style constructor | `crates/effects/fx_eval/src/programs_native.rs`: `eff_drop_shadow` (line 725), `layout_drop_shadow` (2096) | the program graph and its layout row |
| helper programs | the same file: `defun_drop_shadow_shadow` (1793), `defun_drop_shadow_combine` (1778), `defun_mocha_mask` (1652), `defun_combine_masks` (1628) | the shadow branch, the composite, the tracker mask and the mask join |
| parameter and class facts | `crates/effects/rtdata/src/styles/drop_shadow.rs`; class rows in `crates/effects/rtdata/src/classes.rs` | registered parameters, host indices, normalize scales, combination tables of `coerce_to_alpha`, `make_mipmap`, `warp_rel`, `blur`, `coerce_to_v3`, `comp_premult`, `dissolve`, `*`, `max` and `postdivide` |
| op `coerce_to_alpha`, `coerce_to_v3` | `crates/effects/kernels/ops_coerce` | the matte from the alpha plane, the foreground colour planes; executor `coerce_k` (`crates/effects/fx_exec/src/kernels2.rs` line 360) |
| op `make_mipmap` | `crates/effects/kernels/make_mipmap` | the ripmap of the matte that the shift samples; `make_mipmap_k` (kernels2.rs line 114) |
| op `warp_rel` | `crates/effects/kernels/warp_rel` with `crates/effects/kernels/warp_rel_sampler` and `crates/effects/kernels/warp_sampler` | the relative shift of the matte; `warp_rel_k` (kernels2.rs line 243) |
| op `blur` | `crates/effects/kernels/blur` and its pass crates | the shadow blur, see [`S_Blur` internals](blur.md); `blur_k` (kernels2.rs line 975), which also takes a ripmap operand at its base level |
| op `*` | `crates/effects/kernels/ops_arith` | the colour times the shadow, `fg_opacity` times the foreground and its alpha; `binary_k` (`crates/effects/fx_exec/src/kernels.rs` line 197) |
| op `comp_premult` | `crates/effects/kernels/comp_premult` | the premultiplied composite; `comp_premult_k` (kernels2.rs line 833) |
| op `dissolve` | `crates/effects/kernels/dissolve` | the straight composite at `comp_premult` 0; `dissolve_k` (kernels2.rs line 1086) |
| op `max` | `crates/effects/kernels/ops_minmax` | the mask join when the tracker mask differs from 1 |
| op `postdivide` | `crates/effects/fx_exec/src/postdiv.rs` | the division by the alpha that the 'ae32' route adds; `postdivide_k` (kernels2.rs line 1375) |
| op tables | `SUPPORTED` (`crates/effects/fx_exec/src/kernels.rs` line 10), `WIRED` (kernels2.rs line 13) | `*` and `max` are in `SUPPORTED`; the others are in `WIRED` |
| threading | `crates/effects/fx_exec/src/par.rs` (`threads_for`), used by `warp_rel_k` (kernels2.rs line 301); the blur's plane split | row bands of the warp and planes of the blur on frames of 65,536 samples or more |
| device entries | `crates/effects/fx_exec/src/device_ops_pointwise.rs` (`MUL` and `MAX` lines 1963 and 1965, `COERCE_TO_ALPHA` and `COERCE_TO_V3` lines 2035 and 2036, `device_ops_pointwise::DISSOLVE` 2096, `COMP_PREMULT` 2103, `POSTDIVIDE` 2117), `crates/effects/fx_exec/src/device_ops_ripmap.rs` (`make_mipmap_accepts` line 72), `crates/effects/fx_exec/src/device_ops_warp.rs` (`warp_rel_accepts` line 98), `crates/effects/fx_exec/src/device_ops_neighbourhood.rs` (`device_ops_neighbourhood::BLUR` line 20) | device kernels and self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_ripmap.rs` (`RIPMAP_MIPMAP` line 19), `crates/effects/fx_device/src/kernels_warp.rs` (`WARP_HELPERS` 16, `WARP_WARP` 120), `crates/effects/fx_device/src/kernels_neighbourhood.rs` (`NEIGHBOURHOOD_BLUR` 15), `crates/effects/fx_device/src/kernels_pointwise.rs` | the device programs |
| stage rows | `crates/neutral-handoff/data/fx_stages.tsv` line 8; `crates/neutral-handoff/data/fx_params.tsv` lines 77 to 86 | host arrangement and offered parameters |
| computed tables | none | the ripmap and the warp read no table |
| tests | unit tests in `crates/effects/kernels/comp_premult/src/lib.rs` and `crates/effects/kernels/comp_premult/tests/nan_x86.rs`, `crates/effects/kernels/ops_arith/src/lib.rs`, `crates/effects/kernels/make_mipmap/src/lib.rs` and `src/mm2.rs`, `crates/effects/fx_exec/tests/dsz1_blur_mip.rs`, `crates/effects/fx_exec/tests/nan_x86.rs`, `crates/effects/fx_exec/tests/t_set_grid_cli.rs`, `crates/effects/fx_exec/tests/t_dx2_claim_d64.rs`, `crates/effects/fx_eval/tests/t11_dsw3.rs`, `crates/effects/fx_eval/tests/t_host_param_ns1.rs`, `crates/effects/fx_exec/tests/device_ripmap.rs`, `crates/effects/fx_exec/tests/device_warp.rs`, `tools/fx_golden.tsv` lines 50 to 54 | see the checks below |

The ops of the compiled plan are MEASURED with the device report and with `fx_run --plan-text`. At 64x32 with the defaults the plan holds `coerce_to_alpha`, `make_mipmap`, `warp_rel`, `blur`, `coerce_to_v3`, `*`, `comp_premult` and `postdivide`. MEASURED variants on PoCL: `comp_premult=0` replaces `comp_premult` by `dissolve`; `shift=0,0` removes `warp_rel`; `shadow_blur=0` removes `blur`; `shadow_opacity=0` leaves `coerce_to_alpha`, `coerce_to_v3`, `comp_premult` and `postdivide`; `mocha_mask_opacity=0.5` adds `max`. The golden rows run the defaults: they reach the eight ops of the default plan, and no golden row reaches `dissolve` or `max`. On the golden clip (alpha between 0.05 and 1.05) the default output equals the output with `shadow_opacity=0` (MEASURED below), so a fault in the shadow branch (`make_mipmap`, `warp_rel`, `blur`, the colour product) need not change the stage's golden rows; a clip with transparent pixels shows the shadow (MEASURED below).

## Render flow

1. `sapphire stage apply drop_shadow` reads the stage row: program Drop_Shadow, source name `Foreground`, the `fixed` mask options, the `host` options (four source planes) and the `placement` column (one placement at every measured size).
2. Before any frame, as for every effect stage ([`S_Blur` internals](blur.md), render flow steps 2 and 3): `--layer` is refused, a width below 2 is refused, keyed values need `--fps`, and every host slider value (`shadow_blur`, `shift_host`, `shadow_opacity`, `fg_opacity` and the other host type 2 sliders) is put on the 16.16 grid with a `note:` line.
3. Per frame the input words go to the runner as `Foreground`. The program's `Matte` input defaults to `Foreground` and its `Background` input to the constant 0; the stage binds neither.
4. The runner converts `shadow_blur` (and `shift_host`) with the frame width and the registered normalize scale, compiles the plan and folds constant nodes. The stage learns `--do-postdiv 1` from the first `POSTDIV-MISMATCH`, as for `S_Blur`.
5. The executor runs, in order: `coerce_to_alpha` of the matte, the mask join (folded when the tracker mask is 1), `make_mipmap` of the matte, `warp_rel` by the negated `shift`, `blur` of the shifted matte, the colour times the shadow (`*`), `coerce_to_v3` of the foreground and the `fg_opacity` products, then `comp_premult` (or `dissolve`) and `postdivide`.
6. Threads: `warp_rel` splits its rows into bands and `blur` its planes on frames of 65,536 samples or more (`SAPPHIRE_THREADS`, `SAPPHIRE_PAR_MIN`); the bytes do not depend on the thread count.
7. Device: every node runs on the device when its kernel is admitted; `make_mipmap` leaves its pyramid on the device for the device samplers (`crates/effects/fx_exec/tests/device_ripmap.rs`). A declined node or a NaN output of an op that is not NaN exact runs on the CPU.
8. The output words are relaid to the frames/2 planes. The output frame depends only on input frame N.

`fx_run Drop_Shadow` takes the same argument list; the plan text command below builds it from the stage row.

## How parity is judged

The seven checks of [What rebuilt means](../effects.md#what-rebuilt-means) apply to the registered program. For `S_DropShadow`: 24 verified comparisons, 26 recorded renders and 26 re-rendered units, all equal in both renders (`docs/data/effects_status.tsv` line 8; the columns are described in [Effect internals](README.md)). Of the 26 recorded renders 24 are counted and 2 are controls, recorded and not counted; the repository does not name them. Through the stage 20 of the 24 counted recorded renders are byte identical, the ones that use the stage's host arrangement; the other 4 used another host arrangement, whose options the repository does not list, and the row's `host_variants` is `-`, so serving them would need a stage row of its own (the `S_DropShadow` row of the table in the README section [Effects and stages](../../README.md#effects-and-stages); [Effects provider](../stages.md#effects-provider)). None of the approximate reciprocal instructions in the effect's code is reached on the verified render path (MEASURED for 11 sites; the logarithm sites INFERRED from the plan constants; `S_DropShadow` section of [Effects](../effects.md)). In the repository's code the only logarithm route of this plan is the footprint path of `warp_rel` (`crates/effects/kernels/warp_log_gradient`), which `warp_rel_k` takes only for image coordinates with a nonzero argument 3 (`footprint_taken`, `crates/effects/fx_exec/src/kernels2.rs` lines 71 and 264); the stage passes the shift as a two-lane constant of the plan, so no parameter value of the stage reaches it (INFERRED). Which instruction sites the emulator saw is recorded outside the repository. No comparison differs from the original.

## Checks that run from this repository

From the repository root after [Build](../../README.md#build); scratch files in `target/demo/drop-shadow/`. The device commands need an OpenCL platform such as PoCL.

| check | command | expected result | exit |
|---|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | both binaries | 0 |
| stage listed | `sapphire stage list --json` filtered by family | `drop_shadow temporal 10 []` | 0 |
| parameters | `fx_run --describe-params Drop_Shadow` | one JSON member with 19 registered parameters | 0 |
| worked render | `sapphire stage apply drop_shadow` on a 64x32 clip | 98,304 bytes, three `note:` lines, output equal to the opaque input | 0 |
| plan ops on the CPU | `fx_run ... --plan-text` | the eight ops of the default plan | 0 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 5 rows of the stage | 0 |
| conformance | `tools/conformance.py --stage` | L1 `14 passed, 2 failed` (1x1), L3 `33 passed` | 1 |
| refusals | the refusal block | the messages of the refusal table | 3 |
| crate and executor tests | `cargo test` | `test result: ok`; stored-data tests print `NOT RUN` | 0 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 ... --device gpu` | `device: * 3, blur 3, coerce_to_alpha 3, coerce_to_v3 3, comp_premult 3, make_mipmap 3, postdivide 3, warp_rel 3; fallback: none` | 0 |
| device golden rows and tests | as for `S_Blur` | `183 equal`; `test result: ok` | 0 |

```sh
target/release/sapphire stage list --json | python -I -c "import json,sys; [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in json.load(sys.stdin)['stages'] if s['family'] == 'S_DropShadow']"
target/release/fx_run --describe-params Drop_Shadow | python -I -c "import json,sys; print({k: len(v) for k, v in json.load(sys.stdin).items()})"
```

Both exit 0: the first prints `drop_shadow temporal 10 []`, the second the parameter count from the JSON description, `{'Drop_Shadow': 19}` (the 8 program parameters the stage offers, `cset_opacity` and `shift_host` being stage forms, plus `invert_matte`, `matte_use`, `mocha_project`, `blur_mocha_mask`, `invert_mocha_mask`, `resize_mocha_mask`, `resize_mocha_rel_x`, `resize_mocha_rel_y`, `shift_mocha_mask`, `bypass_mocha_mask` and `show_mocha_mask_only`, which the stage fixes or leaves at their defaults).

Worked render with values a verified comparison used:

```sh
mkdir -p target/demo/drop-shadow
ffmpeg -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/drop-shadow/in.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --param shadow_blur=0.291702271 --param color=0.356862754,0.713725507,0.184313729 --param shadow_opacity=0.679092407 --param shift=0.00299692154,-0.00117826462 --param fg_opacity=0.736099243 --param comp_premult=1 --in target/demo/drop-shadow/in.raw --out target/demo/drop-shadow/out.raw
wc -c target/demo/drop-shadow/out.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --in target/demo/drop-shadow/in.raw --out target/demo/drop-shadow/default.raw
cmp target/demo/drop-shadow/in.raw target/demo/drop-shadow/default.raw
```

ffmpeg must be on `PATH`. Every command exits 0. The render prints three `note:` lines (`shadow_blur`, `shadow_opacity` and `fg_opacity` put on the 16.16 host grid) and `wc` prints 98304. The last `cmp` prints nothing: the `testsrc` clip is opaque, so at the defaults the shadow lies wholly under the foreground and the output frames equal the input frames.

The shadow on the golden clip and on a clip with transparent pixels (ffmpeg for the second clip):

```sh
python -I -c "import sys; sys.path.insert(0, 'tools'); import fx_golden as g; open('target/demo/drop-shadow/synth.raw', 'wb').write(g.synth_clip(64, 32, 3, g.SEED))"
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 24 --frames 3 --in target/demo/drop-shadow/synth.raw --out target/demo/drop-shadow/synth_default.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 24 --frames 3 --param shadow_opacity=0 --in target/demo/drop-shadow/synth.raw --out target/demo/drop-shadow/synth_noshadow.raw
cmp target/demo/drop-shadow/synth_default.raw target/demo/drop-shadow/synth_noshadow.raw
ffmpeg -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -vf "format=rgba,geq=r='r(X,Y)':g='g(X,Y)':b='b(X,Y)':a='if(lt(X,32),255,0)'" -f rawvideo -pix_fmt gbrapf32le target/demo/drop-shadow/half.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --param shift_host=4,4 --param color=0.2,0.5,0.8 --in target/demo/drop-shadow/half.raw --out target/demo/drop-shadow/half_shadow.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --param shift_host=4,4 --param color=0.2,0.5,0.8 --param shadow_opacity=0 --in target/demo/drop-shadow/half.raw --out target/demo/drop-shadow/half_noshadow.raw
cmp -s target/demo/drop-shadow/half_shadow.raw target/demo/drop-shadow/half_noshadow.raw
```

The first `cmp` exits 0 and prints nothing: on the golden clip the shadow does not change the output. The second clip is opaque on its left half and transparent on its right half; there the shadow, shifted by 4 pixels, shows, and the last `cmp` exits 1 (the two outputs differ). The other commands exit 0. The values of the last two renders are not verified values (INFERRED).

The plan on the CPU (zero source words; the plan does not depend on them):

```sh
head -c 32768 /dev/zero > target/demo/drop-shadow/zero.u32
target/release/fx_run Drop_Shadow --size 64x32 --out target/demo/drop-shadow/fx --ae32-src Foreground=target/demo/drop-shadow/zero.u32 --ae32-prior target/demo/drop-shadow/zero.u32 --ae32-out target/demo/drop-shadow/fx.u32 --geom -0.5,63.5,1,0,-16.5,0,0 --host-frame 0 --ctx-time 0 $(awk -F'\t' '$1=="drop_shadow"{print $10}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' '\n' | grep -v '^--prior-fill' | tr '\n' ' ') $(awk -F'\t' '$1=="drop_shadow"{print $9}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' ' ') --ns-default-unbound --do-postdiv 1 --plan-text target/demo/drop-shadow/plan.txt
grep '^NODE' target/demo/drop-shadow/plan.txt | grep -E 'kind=(func|auto)' | cut -f3
```

Both exit 0; the last prints the eight ops of the default plan, one a line: `coerce_to_alpha`, `make_mipmap`, `warp_rel`, `blur`, `*`, `coerce_to_v3`, `comp_premult` and `postdivide` in plan order.

Golden rows, conformance and refusals:

```sh
python -I tools/fx_golden.py
grep -E '^drop_shadow' tools/fx_golden.tsv
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply drop_shadow"
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --param shift=0.1,0.1 --param shift_host=4,4 --in target/demo/drop-shadow/in.raw --out target/demo/drop-shadow/r1.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --layer Matte=target/demo/drop-shadow/in.raw --in target/demo/drop-shadow/in.raw --out target/demo/drop-shadow/r2.raw
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --param cset_opacity=3 --in target/demo/drop-shadow/in.raw --out target/demo/drop-shadow/r3.raw
```

`fx_golden.py` exits 0 with `183 cases, 183 equal, 0 differ`; the grep lists the 5 rows of the stage (`64x32`, `64x32t`, `50x7`, `50x7t`, `40x12s`; no variant rows). Conformance exits 1 with L1 `14 passed, 2 failed` (the two 1x1 cases, exit 3) and L3 `33 passed`. The three refusals exit 3 with `shift given both as --host-param and --set` (after a `note:` line that puts the `shift` lanes on the host lattice), `LAYER-INPUT-UNSUPPORTED` and `SOURCE-VARIANT-MISMATCH`, and write no frame.

Crate, executor and evaluator tests:

```sh
cargo test --release --offline -p comp_premult -p ops_arith -p ops_coerce -p ops_minmax -p dissolve -p warp_rel -p warp_rel_sampler -p warp_sampler -p make_mipmap
cargo test --release --offline -p fx_exec --test dsz1_blur_mip --test nan_x86 --test t_set_grid_cli --test t_dx2_claim_d64
cargo test --release --offline -p fx_eval --test t11_dsw3 --test t_host_param_ns1
```

Each exits 0. `comp_premult` checks its arms and its x86 NaN words (`tests/nan_x86.rs`), `make_mipmap` its ripmap levels, `dsz1_blur_mip` that `blur` on a ripmap operand equals `blur` on its base image (the shift 0 plan), `nan_x86` that a ripmap of an infinite source holds the x86 default NaN, `t_host_param_ns1` the host route of `shift`. `t_set_grid_cli`, `t_dx2_claim_d64` and `t11_dsw3` read stored data and print `NOT RUN` lines without `VEC_DIR`, then pass.

Device (PoCL or a GPU):

```sh
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --device gpu --in target/demo/drop-shadow/in.raw --out target/demo/drop-shadow/gpu.raw
cmp target/demo/drop-shadow/default.raw target/demo/drop-shadow/gpu.raw
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_ripmap --test device_warp --test device_pointwise --test device_neighbourhood -- --test-threads=1
```

Each exits 0 on PoCL; the report ends `device: * 3, blur 3, coerce_to_alpha 3, coerce_to_v3 3, comp_premult 3, make_mipmap 3, postdivide 3, warp_rel 3; fallback: none` and `cmp` prints nothing. The device golden rows run as on the `S_Blur` page (`SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1 python -I tools/fx_golden.py`, exit 0). Without a device the first command exits 3 with `DEVICE-REQUIRED-UNAVAILABLE`.

CI (`.github/workflows/ci.yml`) runs the default members (`comp_premult`, `ops_arith`, `ops_coerce`, `ops_minmax`, `dissolve`, `warp_rel`, `warp_rel_sampler`, `warp_sampler`) and `fx_golden.py` on Linux, macOS and Windows; `fx_exec`, `fx_eval` and `make_mipmap` on Linux; the device tests and the device golden rows in the PoCL job.

## What needs the comparison harness

The comparison with the original, the recorded renders and the stored data under `VEC_DIR` are kept outside this repository ([Tests](../effects.md#tests)). Without them `crates/neutral-handoff/tests/fx_stage_parity.rs`, `t_set_grid_cli`, the `t_dx2_*` tests of `fx_exec` and `t11_dsw3` of `fx_eval` print `NOT RUN` and pass. The golden table holds this repository's own renders only. The full report is identified by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, Normal opacity, the verified render path, at the sizes and values of the 20 counted recorded renders that use the stage's host arrangement; the verified values are listed in the `S_DropShadow` section of [Driving the effect parameters](../parameters-guide.md). Other sizes and values are INFERRED.
- Host buffer phase: the shadow's blur runs the window of the blur kernel (MEASURED on the code), with the threshold and the phase 0 of [`S_Blur` internals](blur.md); [Limits common to the effects](../effects.md#limits-common-to-the-effects) names no other phase dependent loop on this stage's plan. The shadow blur has three passes, so at a radius below about 189 pixels (`shadow_blur` in host units, see Units above) a 1920x1080 render is independent of the phase (INFERRED).
- `ROUTE AVX2-ONLY`; NaN payload and sign bits not verified against the original; depths 8 and 16 not verified.
- Matte and background: the stage takes no layer, so the shadow comes from the input's own alpha and is composited over nothing.
- Host values: through the stage every host slider value is put on the 16.16 grid (`note:` line) and renders; `shift_host` and `shadow_blur` off the grid exit 0 (MEASURED with `4.00001`). `fx_run` takes an off-grid `shift_host` as given and is not reached by the stage with one (the `HOST-VALUE-OFF-GRID` row of [Refusal labels](../cli.md#refusal-labels)).
- Opacity: only the answer 2 renders.

## Device path

| op | device kernel | NaN exact | falls back to the CPU when |
|---|---|---|---|
| `coerce_to_alpha`, `coerce_to_v3` | pointwise coerce kernels | yes (`NAN_EXACT_WRITTEN`) | the kernel declines the shape |
| `make_mipmap` | `RIPMAP_MIPMAP`; the pyramid stays on the device | yes (`NAN_EXACT_COMPILED`) | the kernel declines the shape |
| `warp_rel` | `warp_warp_f32` (`WARP_WARP`) | no | a NaN output (rerun on the CPU) or a declined shape |
| `blur` | `device_ops_neighbourhood::BLUR` | yes (`NAN_EXACT_WRITTEN`) | see [`S_Blur` internals](blur.md) |
| `*` | pointwise `MUL` | yes (`NAN_EXACT_COMPILED`) | the kernel declines the shape |
| `comp_premult` | pointwise `COMP_PREMULT` | no | a NaN output |
| `dissolve` | `device_ops_pointwise::DISSOLVE` | yes (`NAN_EXACT_COMPILED`) | the kernel declines the shape |
| `max` | pointwise `MAX` | yes (`NAN_EXACT_WRITTEN`) | the kernel declines the shape |
| `postdivide` | `POSTDIVIDE` | yes (`NAN_EXACT_WRITTEN`) | the kernel declines the shape |

No op has a `CPU_ONLY` entry. MEASURED on PoCL only: `fallback: none` on every golden row of the stage, the special row included ([`S_DropShadow` section of Effects](../effects.md)), and on the commands above. PoCL time for one 1920x1080 frame: 0.63 / 0.64 s against 1.63 / 1.50 s on the CPU ([Speed](../guide.md#speed)); GPU speed is INFERRED.

## Changing the code safely

- Keep green: `fx_golden.py` (every group, on the CPU and on a device), the crate tests above, `cargo test -p fx_eval -p fx_exec`, the device tests, `test_effect_rules.py`, `gen_docs.py --check` and `check_docs.py`.
- A golden difference means the stage's output bytes changed; the table cannot say whether the original agrees, and `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` only after an intended and judged change.
- A change to the blur affects `S_Blur`, `S_Sharpen` and other effects as well: run the whole golden table. A CPU change to an op removes it from `ALLOWLIST` unless its device kernel changes in the same patch. `comp_premult` pins its x86 NaN words in `tests/nan_x86.rs`; a new or changed kernel follows the NaN rules of [Crate layout](../adding-an-effect.md#crate-layout) (`fx_common::nanop` for two-NaN picks, the x86 default NaN for a fresh NaN).
- A change that moves an output updates `tools/fx_golden.tsv`, the `S_DropShadow` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), `docs/data/effects_status.tsv` when a count changes, and the generated pages (`python -I tools/gen_docs.py`).

## Open items

- No `Matte` or `Background` layer is offered: the stage covers the recorded arrangement in which both inputs were unbound (`LAYER-INPUT-UNSUPPORTED`, [Refusal labels](../cli.md#refusal-labels)). Offering them needs recorded runs with the inputs bound, which the repository does not hold (NEEDS-OUTSIDE).
- `cset_opacity` 1 and 3 are refused (`SOURCE-VARIANT-MISMATCH`): no recorded render holds them.
- `warp_rel` and `comp_premult` are not NaN exact on the device: their NaN outputs rerun on the CPU (`NAN_EXACT` in `crates/effects/fx_exec/src/device_ops.rs`).
- No golden row renders a visible shadow (MEASURED above); a golden variant row on a clip with transparent pixels would cover the shadow branch (`tools/fx_golden.py`, `VARIANTS`).
