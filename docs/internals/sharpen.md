# Internals of `stage apply sharpen_best_quality_autogen` and `stage apply sharpen_fast_autogen`

This page covers the code of these stages: where each part lives, how a render runs, the checks that run from this repository, the device path and the limits. They are the two styles of `S_Sharpen`. The user reference is the `S_Sharpen` section of [Effects](../effects.md), the `S_Sharpen` section of [Driving the effect parameters](../parameters-guide.md) and [Effects provider](../stages.md#effects-provider). The architecture every effect shares is in [Effect internals](README.md). The low-pass of the Fast style is the blur kernel of [`S_Blur` internals](blur.md).

## What it computes

Both styles split the colour image into a luma plane (`monochrome`) and a chroma image (the source minus the luma). When `sharpen_luma` equals `sharpen_chroma` the whole image is sharpened in one pass with the per-channel gains `sharpen_red`, `sharpen_green` and `sharpen_blue` times `sharpen_amp`; otherwise the luma and the chroma are sharpened separately, with `sharpen_luma` and `sharpen_chroma` times `sharpen_amp`, and added. The Fast style (`sharpen`) is an unsharp mask: each plane is blurred by the blur kernel (three box passes, mirrored edges) with a radius proportional to `sharpen_width` and the frame width, and the output is the plane plus the gain times the plane minus its blur; a radius of 0 or a gain of 0 copies the plane. The Best Quality style (`edge_aware_sharpen`) separates the image into four detail levels whose sizes start from half of `small_detail_size`, scales each level by `scale_tiny_details`, `scale_small_details`, `scale_medium_details` and `scale_large_details`, and weights the sharpening by the local edge strength so that it follows edges, against `edge_threshold`. Only the colour planes are processed (the stages pass a three-plane source); the alpha division that the 'ae32' route adds reads the alpha of the source frame.

Read from the code (INFERRED): the radius of the Fast blur in pixels equals the host value of `sharpen_width` (the conversion's frame width and normalize scale cancel against `radius` in `crates/effects/kernels/sharpen/src/plan.rs`, as for `blur_amount` in [`S_Blur` internals](blur.md)). The Best Quality weights follow the domain transform form, as the docs of `crates/effects/kernels/edge_aware_sharpen_weights/src/lib.rs` and `crates/effects/kernels/edge_aware_sharpen_jobs/src/lib.rs` name it: weight planes from the absolute derivatives along lines and columns, running sums and a guided box filter per level. The program passes four levels and a tiny level size of half `small_detail_size` (`eff_sharpen_best_quality`); `edge_aware_sharpen` hands the render the level count, the tiny size and `edge_threshold` (`crates/effects/kernels/edge_aware_sharpen/src/lib.rs` line 143). The sizes of the further levels and the use of the threshold are computed in `crates/effects/kernels/edge_aware_sharpen_render/src/lib.rs` (with `powf`); the repository states no closed formula for them.

## Styles and stages

| style | program | stage | kind | layers | spool | host variants |
|---|---|---|---|---|---|---|
| Best Quality | Sharpen_Best_Quality_Autogen | `stage apply sharpen_best_quality_autogen` | `temporal` | none | `false` | `-` |
| Fast | Sharpen_Fast_Autogen | `stage apply sharpen_fast_autogen` | `temporal` | none | `false` | `-` |

The values are the rows of `crates/neutral-handoff/data/fx_stages.tsv` (lines 10 and 11). `sharpen_best_quality_autogen` offers 13 parameters (rows 95 to 107 of `crates/neutral-handoff/data/fx_params.tsv`): `small_detail_size` as a host value (`--host-param`), `cset_opacity` as a host popup answer, the other 11 as record values. `sharpen_fast_autogen` offers 8 (rows 108 to 115): `sharpen_width` as a host value, `cset_opacity`, and 6 record values. Both rows pass a three-plane source (`--src-planes 3`) and fix the mask and tracker mask parameters of the programs at the values of the recorded runs in their `fixed` column, so the effect mask is the constant 1.

Refusals (MEASURED on this build):

| input | result | exit |
|---|---|---|
| `--param cset_opacity=1` or `--param cset_opacity=3` | `SOURCE-VARIANT-MISMATCH` | 3 |
| `--param cset_opacity=4` | `bad value for cset_opacity: 4 is outside 1 to 3` | 3 |
| `--param show=1` (a program parameter the stage does not offer) | `unknown param` | 3 |
| `--width 1` | `needs a width of 2 or more` | 3 |
| `--layer To=PATH` | `LAYER-INPUT-UNSUPPORTED` | 3 |
| a keyed value without `--fps` | `keyframes need --fps` | 2 |

## Code map

| part | path | role |
|---|---|---|
| style constructors | `crates/effects/fx_eval/src/programs_native.rs`: `eff_sharpen_best_quality` (line 874), `eff_sharpen_best_quality_autogen` (976), `eff_sharpen_fast` (1047), `eff_sharpen_fast_autogen` (1109), `layout_sharpen_best_quality_autogen` (2159), `layout_sharpen_fast_autogen` (2198) | the program graphs, wrapper programs and layout rows |
| helper programs | the same file: `defun_process_masks` (1674), `defun_mocha_mask` (1652), `defun_combine_masks` (1628) | the mask chain, constant on the stages |
| parameter and class facts | `crates/effects/rtdata/src/styles/sharpen_best_quality.rs`, `crates/effects/rtdata/src/styles/sharpen_fast.rs`; class rows in `crates/effects/rtdata/src/classes.rs` | registered parameters, host indices, normalize scales; combination tables of `monochrome`, `-`, `+`, `sharpen`, `edge_aware_sharpen` and `postdivide` |
| op `monochrome` | `crates/effects/kernels/monochrome` | the luma plane; `monochrome_k` (`crates/effects/fx_exec/src/kernels2.rs` line 343) |
| ops `-`, `+` | `crates/effects/kernels/ops_arith` | chroma and the sum of the two sharpened parts; `binary_k` (`crates/effects/fx_exec/src/kernels.rs` line 197) |
| op `sharpen` | `crates/effects/kernels/sharpen` (`plan.rs`, `combine.rs`, `assume.rs`), using `crates/effects/kernels/blur` | the unsharp mask of the Fast style; `sharpen_k` (kernels2.rs line 462) |
| op `edge_aware_sharpen` | `crates/effects/kernels/edge_aware_sharpen` (entry: operands and named errors), `crates/effects/kernels/edge_aware_sharpen_render` (render entry and plane services), `crates/effects/kernels/edge_aware_sharpen_weights` (weight planes of the edge-aware recursive filter), `crates/effects/kernels/edge_aware_sharpen_jobs` (weight planes from the channel differences, running sums, the exp2 based transform), `crates/effects/kernels/edge_aware_sharpen_blur_core` (the blur core in its edge-aware mode), `crates/effects/kernels/edge_aware_sharpen_box_filter` (the guided variable window box filter along rows and columns), `crates/effects/kernels/libm_powf` (`powf`) | the Best Quality style; `edge_aware_sharpen_k` (kernels2.rs line 500) |
| op `postdivide` | `crates/effects/fx_exec/src/postdiv.rs` | the alpha division of the 'ae32' route; on this three-plane route it takes the alpha from the source frame (the copy-alpha arm, `postdiv_copy_alpha_into`) |
| op tables | `SUPPORTED` (`crates/effects/fx_exec/src/kernels.rs` line 10), `WIRED` (kernels2.rs line 13) | `+` and `-` are in `SUPPORTED`; `monochrome`, `sharpen`, `edge_aware_sharpen` and `postdivide` in `WIRED` |
| computed tables | `crates/effects/kernels/libm_powf/src/tables.rs` | the log2 and exp2 tables of `powf`, computed from their definitions at first use (`libm_sincos::exact`); the exp2 approximation of the edge-aware blur core and box filter uses named constants in `exp2tab.rs` of those two crates, no table |
| device entries | `crates/effects/fx_exec/src/device_ops_neighbourhood.rs` (`device_ops_neighbourhood::SHARPEN` line 23, `EDGE_AWARE_SHARPEN` line 24), `crates/effects/fx_exec/src/device_ops_pointwise.rs` (`ADD`, `SUB` lines 1961 and 1962, `device_ops_pointwise::MONOCHROME` 2280, `POSTDIVIDE` 2117) | device kernels and self-check corpora |
| OpenCL sources | `crates/effects/fx_device/src/kernels_neighbourhood.rs` (`NEIGHBOURHOOD_SHARPEN` line 493 with `sharpen_combine_f32`, `NEIGHBOURHOOD_ESH` line 537 with the `esh_*` kernels, `NEIGHBOURHOOD_BLUR` line 15), `crates/effects/fx_device/src/kernels_pointwise.rs` | the device programs |
| stage rows | `crates/neutral-handoff/data/fx_stages.tsv` lines 10 and 11; `crates/neutral-handoff/data/fx_params.tsv` lines 95 to 115 | host arrangement and offered parameters |
| tests | `crates/effects/kernels/sharpen/tests/`, `crates/effects/kernels/edge_aware_sharpen/tests/accept.rs`, the `tests/` folders of the five `edge_aware_sharpen_*` crates, `crates/effects/fx_exec/tests/nan_x86_esh.rs`, `crates/effects/fx_exec/tests/device_neighbourhood.rs`, `crates/effects/fx_eval/tests/t_int32_df.rs`, `tools/fx_golden.tsv` lines 135 to 144 | see the checks below |

The ops of the compiled plans are MEASURED with the device report and with `fx_run --plan-text`. At 64x32 with the defaults Sharpen_Best_Quality_Autogen holds `edge_aware_sharpen` and `postdivide`, and Sharpen_Fast_Autogen `sharpen` and `postdivide`: the luma and chroma are equal at the defaults, so the one-pass branch is taken. With `sharpen_luma` other than `sharpen_chroma` the plans hold `monochrome`, `-`, two `edge_aware_sharpen` or two `sharpen` nodes, `+` and `postdivide`. MEASURED on PoCL: `small_detail_size=0` folds the Best Quality plan to `postdivide` alone (output equal to the input); `sharpen_width=0` keeps the `sharpen` node, which then copies its planes (output equal to the input). The golden rows run the defaults, so they reach the one-pass branch only: `edge_aware_sharpen`, `sharpen` and `postdivide`; no golden row reaches `monochrome`, `-` or `+` (the two-pass branch), which the device commands below and the `monochrome` and `ops_arith` crate tests cover.

The executor entry of `edge_aware_sharpen` names its errors as printed: `ERROR-PATH FLAG-NONZERO`, `ERROR-PATH TAG-NEGATIVE`, `NCH-RANGE`, `ERROR-PATH ITERS-ABOVE-7` and `MASK-UNBOUND` (`crates/effects/fx_exec/src/kernels2.rs` lines 542 to 546); the stage plans do not reach them (INFERRED from that entry and the plans above).

## Render flow

1. `sapphire stage apply sharpen_best_quality_autogen` (or `sharpen_fast_autogen`) reads the stage row: program, source `Source`, the `fixed` mask options, the `host` options (a three-plane source and the options of its input class) and the `placement` column (one placement at every measured size).
2. Before any frame, as on every effect stage ([`S_Blur` internals](blur.md), render flow steps 2 and 3): `--layer`, a width below 2 and an unknown parameter are refused; keyed values need `--fps`; host slider values are put on the 16.16 grid with a `note:` line.
3. Per frame the runner receives the colour planes as the source and the frame's alpha for the copy-alpha arm; `small_detail_size` or `sharpen_width` is converted with the frame width and the normalize scale; the stage learns `--do-postdiv 1` from the first `POSTDIV-MISMATCH`.
4. The runner compiles the plan; the comparison of `sharpen_luma` and `sharpen_chroma` is folded, so only one branch is in the plan.
5. Fast: `sharpen` computes the radius from its amount and the frame width; for each plane with a radius and a gain other than 0 it runs the blur kernel (three passes, mirrored edges, gain 1) and combines the plane with its blur by one fused multiply-add per sample.
6. Best Quality: `edge_aware_sharpen` runs its render entry on one thread: weight planes from the channel differences along lines and columns, running sums, the guided box filter passes per detail level, the exp2 based transform with the per-level scales and `edge_threshold`, and the sum of the levels.
7. Threads: the blur inside `sharpen` splits its planes over threads on frames of 65,536 samples or more (`SAPPHIRE_THREADS`, `SAPPHIRE_PAR_MIN`); `edge_aware_sharpen` runs on one thread. The bytes do not depend on the thread count.
8. Device: each admitted node runs on the device; `sharpen` and `edge_aware_sharpen` are NaN exact, so their NaN outputs stay on the device after a self-check that reports equal payloads.
9. `postdivide` divides the colour by the source alpha (its copy-alpha arm); the output words are relaid to the frames/2 planes. On the opaque `testsrc` clip the output alpha planes equal the input's (MEASURED).

`fx_run Sharpen_Fast_Autogen` and `fx_run Sharpen_Best_Quality_Autogen` take the same argument list; the plan text command below builds it from the stage row.

## How parity is judged

The seven checks of [What rebuilt means](../effects.md#what-rebuilt-means) apply to both styles. For `S_Sharpen`: 43 verified comparisons, 114 recorded renders and 114 re-rendered units, all equal in both renders (`docs/data/effects_status.tsv` line 16; the columns are described in [Effect internals](README.md)). Of the 114 recorded renders 78 are counted (42 Best Quality, 36 Fast, [Effects provider](../stages.md#effects-provider)) and 36 are controls, recorded and not counted; the repository does not name them. Through the stages 35 of the 78 counted recorded renders are byte identical, the ones that use the stage's host arrangement: 19 of 42 (Best Quality) and 16 of 36 (Fast); the other 43 used another host arrangement, whose options the repository does not list. Both rows carry `-` in `host_variants`, so a render whose arrangement differs in a host option that no offered parameter selects (for example the destination route `--ae32-dest` of the `host` column) needs a stage row of its own (INFERRED) (the `S_Sharpen` row of the table in the README section [Effects and stages](../../README.md#effects-and-stages); [Effects provider](../stages.md#effects-provider)). The edge-aware core of Best Quality is sensitive to approximate reciprocals, with a bound of 6 ulp (INFERRED); Fast uses none (INFERRED) (`S_Sharpen` section of [Effects](../effects.md)). The two approximate sites of the edge-aware crates, a reciprocal estimate in the weights and a reciprocal square root estimate in the blur core, are modelled as the exact IEEE operation followed by the code's own Newton step (`crates/effects/kernels/edge_aware_sharpen_weights/src/lib.rs` line 63, `crates/effects/kernels/edge_aware_sharpen_blur_core/src/lib.rs` line 61), and have Rust-only sensitivity tests that perturb the estimate within the SDM bound before the code's own Newton step (`esh2_approx_equiv`, `esh5_approx`). No comparison differs from the original.

## Checks that run from this repository

From the repository root after [Build](../../README.md#build); scratch files in `target/demo/sharpen/`. The device commands need an OpenCL platform such as PoCL.

| check | command | expected result | exit |
|---|---|---|---|
| build | `cargo build --release --offline -p neutral-handoff -p fx_exec` | both binaries | 0 |
| stages listed | `sapphire stage list --json` filtered by family | `sharpen_best_quality_autogen temporal 13 []`, `sharpen_fast_autogen temporal 8 []` | 0 |
| parameters | `fx_run --describe-params` per program | 27 (Best Quality) and 21 (Fast) registered parameters | 0 |
| worked renders | `sapphire stage apply` on a 64x32 clip | 98,304 bytes each | 0 |
| plan ops on the CPU | `fx_run ... --plan-text` | `edge_aware_sharpen`, `postdivide` | 0 |
| golden rows | `python -I tools/fx_golden.py` | `fx_golden: 183 cases, 183 equal, 0 differ`; 10 rows of the effect | 0 |
| conformance | `tools/conformance.py --stage` per stage | L1 `14 passed, 2 failed` (1x1), L3 `33 passed` | 1 |
| refusals | the refusal block | the messages of the refusal table | 3 |
| crate tests | `cargo test` of the sharpen crates | `test result: ok`; stored-data tests print `NOT RUN` | 0 |
| device report | `SAPPHIRE_DEVICE_REPORT=1 ... --device gpu` | `fallback: none`, output equal to the CPU | 0 |
| device golden rows and tests | as for `S_Blur` | `183 equal`; `test result: ok` | 0 |

```sh
target/release/sapphire stage list --json | python -I -c "import json,sys; [print(s['id'], s['kind'], len(s['params']), s['layers']) for s in json.load(sys.stdin)['stages'] if s['family'] == 'S_Sharpen']"
target/release/fx_run --describe-params Sharpen_Best_Quality_Autogen | python -I -c "import json,sys; print({k: len(v) for k, v in json.load(sys.stdin).items()})"
target/release/fx_run --describe-params Sharpen_Fast_Autogen | python -I -c "import json,sys; print({k: len(v) for k, v in json.load(sys.stdin).items()})"
```

Each exits 0. The first prints `sharpen_best_quality_autogen temporal 13 []` and `sharpen_fast_autogen temporal 8 []`; the other two print the parameter counts from the JSON descriptions, `{'Sharpen_Best_Quality_Autogen': 27}` and `{'Sharpen_Fast_Autogen': 21}` (the offered program parameters plus `show`, the mask and the tracker mask parameters, which the stage fixes or leaves at their defaults).

Worked renders with values the verified comparisons used:

```sh
mkdir -p target/demo/sharpen
ffmpeg -v error -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le target/demo/sharpen/in.raw
target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param sharpen_width=100 --param sharpen_amp=0 --param sharpen_luma=0.5 --param sharpen_chroma=1.5 --in target/demo/sharpen/in.raw --out target/demo/sharpen/fast.raw
target/release/sapphire stage apply sharpen_best_quality_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param small_detail_size=4 --param edge_threshold=0.25 --param sharpen_amp=1.25 --in target/demo/sharpen/in.raw --out target/demo/sharpen/best.raw
wc -c target/demo/sharpen/fast.raw target/demo/sharpen/best.raw
```

ffmpeg must be on `PATH`. Every command exits 0 and `wc` prints 98304 for each file; all values are on the 16.16 grid, so no `note:` line is printed. The Fast command uses `sharpen_amp=0`: both `sharpen` nodes copy their planes, and the output differs from the input only through the sum of the luma and the chroma (the first `cmp` below exits 1, MEASURED). A render whose gain is not 0, with values each verified on its own (the combination INFERRED), shows the unsharp mask:

```sh
cmp -s target/demo/sharpen/in.raw target/demo/sharpen/fast.raw
target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param sharpen_width=26.879989624023438 --param sharpen_amp=1.25 --in target/demo/sharpen/in.raw --out target/demo/sharpen/fast_gain.raw
cmp -s target/demo/sharpen/in.raw target/demo/sharpen/fast_gain.raw
```

Both `cmp` commands exit 1 and the render exits 0: each output differs from the input.

The plan on the CPU (zero source words; the plan does not depend on them):

```sh
head -c 32768 /dev/zero > target/demo/sharpen/zero.u32
target/release/fx_run Sharpen_Best_Quality_Autogen --size 64x32 --out target/demo/sharpen/fx --ae32-src Source=target/demo/sharpen/zero.u32 --ae32-prior target/demo/sharpen/zero.u32 --ae32-out target/demo/sharpen/fx.u32 --geom -0.5,63.5,1,0,-16.5,0,0 --host-frame 0 --ctx-time 0 $(awk -F'\t' '$1=="sharpen_best_quality_autogen"{print $10}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' '\n' | grep -v '^--prior-fill' | tr '\n' ' ') $(awk -F'\t' '$1=="sharpen_best_quality_autogen"{print $9}' crates/neutral-handoff/data/fx_stages.tsv | tr '|' ' ') --ns-default-unbound --do-postdiv 1 --plan-text target/demo/sharpen/plan.txt
grep '^NODE' target/demo/sharpen/plan.txt | grep -E 'kind=(func|auto)' | cut -f3
```

Both exit 0; the last prints `edge_aware_sharpen` and `postdivide`.

Golden rows, conformance and refusals:

```sh
python -I tools/fx_golden.py
grep -E '^sharpen_(best_quality|fast)_autogen' tools/fx_golden.tsv
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply sharpen_best_quality_autogen"
python -I tools/conformance.py --stage "$PWD/target/release/sapphire stage apply sharpen_fast_autogen"
target/release/sapphire stage apply sharpen_best_quality_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param show=1 --in target/demo/sharpen/in.raw --out target/demo/sharpen/r1.raw
target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param cset_opacity=1 --in target/demo/sharpen/in.raw --out target/demo/sharpen/r2.raw
```

`fx_golden.py` exits 0 with `183 cases, 183 equal, 0 differ`; the grep lists 10 rows, 5 per stage (`64x32`, `64x32t`, `50x7`, `50x7t`, `40x12s`; no variant rows). Each conformance run exits 1 with L1 `14 passed, 2 failed` (the two 1x1 cases, exit 3) and L3 `33 passed`. The two refusals exit 3 with `unknown param` and `SOURCE-VARIANT-MISMATCH`.

Crate, executor and evaluator tests:

```sh
cargo test --release --offline -p sharpen -p edge_aware_sharpen -p edge_aware_sharpen_render -p edge_aware_sharpen_weights -p edge_aware_sharpen_jobs -p edge_aware_sharpen_blur_core -p edge_aware_sharpen_box_filter -p libm_powf -p monochrome -p ops_arith
cargo test --release --offline -p fx_exec --test nan_x86_esh
cargo test --release --offline -p fx_eval --test t_int32_df
```

Each exits 0. Without `VEC_DIR` the tests that compare with stored emulator rows print `NOT RUN` lines and pass (in `sharpen`: `t1`, `t1_render`, `t2_combine`, `t2_e2e`, `t2_wrapper`, `shf1_t2`, `alias_tail`; in the edge-aware crates: `accept`, `esh2_t2`, `esh3_t2`, `esh5_t2`, `esh5b_t2`, `esh5_rsqrt_sweep` and the render crate's `t2`; `t_int32_df` of `fx_eval`). The others run on any machine: the NaN pair and NaN flag tests (`nan_pair`, `nan_render`, `nan_flags`, `nan_x86`), the approximate reciprocal models (`esh2_approx_equiv`, `esh5_approx`), `t3` and `shf1_equiv` of `sharpen`, and `nan_x86_esh`, which checks that the executor leaves the x86 NaN words of `edge_aware_sharpen` as the kernel wrote them.

Device (PoCL or a GPU):

```sh
target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param sharpen_width=6 --param sharpen_amp=1.25 --param sharpen_luma=0.5 --in target/demo/sharpen/in.raw --out target/demo/sharpen/two_cpu.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --fps 25/1 --device gpu --param sharpen_width=6 --param sharpen_amp=1.25 --param sharpen_luma=0.5 --in target/demo/sharpen/in.raw --out target/demo/sharpen/two_gpu.raw
cmp target/demo/sharpen/two_cpu.raw target/demo/sharpen/two_gpu.raw
SAPPHIRE_DEVICE_REPORT=1 SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply sharpen_best_quality_autogen --width 64 --height 32 --depth 32 --fps 25/1 --device gpu --param small_detail_size=4 --param edge_threshold=0.25 --param sharpen_amp=1.25 --in target/demo/sharpen/in.raw --out target/demo/sharpen/best_gpu.raw
cmp target/demo/sharpen/best.raw target/demo/sharpen/best_gpu.raw
SAPPHIRE_REQUIRE_DEVICE=1 cargo test --release --offline -p fx_exec --test device_neighbourhood --test device_pointwise -- --test-threads=1
```

Each exits 0 on PoCL. The Fast report ends `device: + 3, - 3, monochrome 3, postdivide 3, sharpen 6; fallback: none` (the two-pass branch), the Best Quality report `device: edge_aware_sharpen 3, postdivide 3; fallback: none`, and both `cmp` print nothing. The device golden rows run as on the `S_Blur` page, and the NaN-exact mutation check there (`nan_exact_neighbourhood_source_mutants_are_refused`) covers the `sharpen` and `edge_aware_sharpen` kernels. Without a device the `sapphire` device commands exit 3 with `DEVICE-REQUIRED-UNAVAILABLE`.

CI (`.github/workflows/ci.yml`): `monochrome`, `ops_arith` and `libm_powf` are default members (Linux, macOS and Windows); `sharpen` and the six `edge_aware_sharpen` crates are outside the default members and run on Linux in the step `Run the tests of the non-default effect crates that need no stored data`; the PoCL job runs the device tests and the device golden rows.

## What needs the comparison harness

The comparison with the original, the recorded renders and the stored data under `VEC_DIR` are kept outside this repository ([Tests](../effects.md#tests)). Without them the stored-data tests listed above and `crates/neutral-handoff/tests/fx_stage_parity.rs` print `NOT RUN` and pass. The approximate reciprocal bound of the edge-aware core was measured on the port, not against hardware. The full report is identified by its SHA-256 in [Evidence](../effects.md#evidence).

## Verified scope and limits

- Verified: depth 32, Normal opacity, the verified render path, at the sizes and values of the 35 counted recorded renders that use the stages' host arrangement; the verified values are listed in the `S_Sharpen` section of [Driving the effect parameters](../parameters-guide.md). Other sizes and values are INFERRED.
- Host buffer phase: the blur windows of `sharpen` (64 samples or more, a frame 128 pixels or more wide or high) and, on Best Quality, the edge-aware weights on lines of 1,200 samples or more take their order from a buffer address modulo 32 or 64 bytes. The placement column of both rows is the same at every measured size, so the runner uses its default phase; above the thresholds the output equals the original's when the host's phase equals it (INFERRED; [Limits common to the effects](../effects.md#limits-common-to-the-effects)). In the code the weights take their lane split from the phase of each line of 1,200 samples or more, and the render passes phase 0 (`weights::job_1` in `crates/effects/kernels/edge_aware_sharpen_render/src/lib.rs` line 237): every Best Quality render 1,200 pixels wide or more, 1920x1080 among them, rests on that phase 0 (INFERRED). The Fast blur has three passes, so at a `sharpen_width` below about 189 pixels it is independent of the phase (INFERRED).
- Approximate reciprocals: Best Quality within 6 ulp (INFERRED), Fast none (INFERRED). What a CPU's own reciprocal estimates give is not measured (NEEDS-OUTSIDE).
- `ROUTE AVX2-ONLY`; NaN payload and sign bits not verified against the original; depths 8 and 16 not verified.
- The mask of the programs stays the constant 1, and `show` (the detail level display of Best Quality) is not offered.
- Opacity: only the answer 2 renders.

## Device path

| op | device kernel | NaN exact | falls back to the CPU when |
|---|---|---|---|
| `sharpen` | `device_ops_neighbourhood::SHARPEN` (`NEIGHBOURHOOD_BLUR` passes and `sharpen_combine_f32`) | yes (`NAN_EXACT_WRITTEN`) | the tag is outside 1 to 4, the source does not have exactly the tag's channels, the gains or amount are not constants, or the host element count is not given |
| `edge_aware_sharpen` | `EDGE_AWARE_SHARPEN` (`esh_weights_f32`, `esh_cumsum_x_f32`, `esh_cumsum_y_f32`, `esh_filter_x_f32`, `esh_filter_y_f32`, `esh_level_f32`, `esh_add_f32`) | yes (`NAN_EXACT_WRITTEN`) | the tag is outside 1 to 3, the mask is neither a one channel image nor a constant, a parameter operand is not a constant, or an operand is a ripmap |
| `monochrome` | `device_ops_pointwise::MONOCHROME` (a double precision path) | no | a NaN output |
| `+`, `-` | pointwise `ADD`, `SUB` | `+` yes (`NAN_EXACT_COMPILED`), `-` yes (`NAN_EXACT_WRITTEN`) | the kernel declines the shape |
| `postdivide` | `POSTDIVIDE`, copy-alpha arm included | yes (`NAN_EXACT_WRITTEN`) | the kernel declines the shape |

No op has a `CPU_ONLY` entry. MEASURED on PoCL only: `fallback: none` on the plain, threaded and special golden rows of both stages, and the `sharpen` and `edge_aware_sharpen` self-checks report equal NaN payloads (`S_Sharpen` section of [Effects](../effects.md)). PoCL time for one 1920x1080 frame: 1.19 / 1.14 s (Best Quality) and 0.68 / 0.66 s (Fast), against 3.45 / 3.38 s and 0.73 / 0.73 s on the CPU ([Speed](../guide.md#speed)); GPU speed is INFERRED. The `monochrome` kernel's double precision path may run slower on consumer GPUs (INFERRED, [Rendering on a device](../guide.md#rendering-on-a-device)).

## Changing the code safely

- Keep green: `fx_golden.py` (every group, on the CPU and on a device), the crate tests above, `cargo test -p fx_eval -p fx_exec`, the device tests and the NaN-exact mutation check, `test_effect_rules.py`, `gen_docs.py --check` and `check_docs.py`.
- A golden difference means the stage's output bytes changed; the table cannot say whether the original agrees, and `python -I tools/fx_golden.py --write` rewrites `tools/fx_golden.tsv` only after an intended and judged change.
- A change at the approximate sites of the edge-aware crates keeps the sensitivity tests `esh2_approx_equiv` and `esh5_approx` green and restates the bound in the `S_Sharpen` section of [Effects](../effects.md). The edge-aware crates call `powf` through `libm_powf`, never the platform library (`tools/test_effect_rules.py`); two-NaN picks go through `fx_common::nanop` ([Crate layout](../adding-an-effect.md#crate-layout)).
- The `sharpen` op reuses the blur kernel: a change to the blur moves `S_Blur`, `S_DropShadow` and `S_Sharpen` rows together. A CPU change to an op removes it from `ALLOWLIST` unless its device kernel changes in the same patch ([Device](../adding-an-effect.md#device)).
- A change that moves an output updates `tools/fx_golden.tsv`, the `S_Sharpen` sections of [Effects](../effects.md) and [Driving the effect parameters](../parameters-guide.md), `docs/data/effects_status.tsv` when a count changes, and the generated pages.

## Open items

- `cset_opacity` 1 and 3 are refused (`SOURCE-VARIANT-MISMATCH`): no recorded render holds them.
- The approximate reciprocal bound of Best Quality is INFERRED from the port (`S_Sharpen` section of [Effects](../effects.md)); a CPU with other reciprocal estimates is outside the claim.
- The host buffer phase of the edge-aware weights above 1,200 samples a line and of the blur windows above the 64 sample window is the runner's phase 0; the phase a host's allocator gives is not known from the repository (NEEDS-OUTSIDE) ([Limits common to the effects](../effects.md#limits-common-to-the-effects)).
- No golden row reaches the two-pass branch (`monochrome`, `-`, `+`); a golden variant row with `sharpen_luma` other than `sharpen_chroma` would cover it (`tools/fx_golden.py`, `VARIANTS`).
- The executor's `edge_aware_sharpen` entry runs on one thread; the original's pool partition is modelled as one job over the whole range (`crates/effects/kernels/edge_aware_sharpen_render/src/lib.rs`). Whether the original's output depends on that partition (sums restarted per job) is not known from the repository (NEEDS-OUTSIDE).
