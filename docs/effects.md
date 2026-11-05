# Effects

Ten of the plugin's effects are rebuilt in this repository as native Rust code. The effect crates are under `crates/effects`; the command line tool `fx_run` renders an effect from its program text, a size and the input planes, once the optional data bundle is in place ([Build and run](#build-and-run)). The evidence that each effect matches the original plugin is kept outside the repository (see [Evidence](#evidence)).

Effects rebuilt: <!--f:effects_rebuilt-->10<!--/f--> of <!--f:effects_listed-->10<!--/f--> listed here, the first milestone of the effect rebuild. The other effects of the plugin are not rebuilt; their names are strings only ([names](../README.md#names)).

## What rebuilt means

An effect counts as rebuilt when every one of its registered styles passes the project's seven checks on the final binary: identity of the program text, the builtin classes it calls, the data it reads, its literals, the compiled plan, the combination tables and the execution itself. Every comparison was made against the original plugin's code running in an emulator that follows Intel's specification, not on silicon. The outputs of the final binary are recorded as predictions and checked by full SHA-256. No result of native execution of the original code is used.

<!--gen:effects_status-->
| effect | registered styles | counted rows | frozen members | re-run units |
|---|---|---|---|---|
| `S_Blur` | Blur_Color_Autogen, Blur_Mono_Autogen | 7 | 52 | 52 |
| `BlurMoCurves` | Transform_and_Blur_Autogen, Blur_Motion_Curves_No_Transform_Autogen | 41 | 69 | 69 |
| `S_Diffuse` | Diffuse_Autogen | 31 | 54 | 54 |
| `S_DistortRGB` | Distort_RGB_Autogen | 30 | 30 | 30 |
| `S_DropShadow` | registered without a wrapper program | 24 | 26 | 26 |
| `S_HueSatBright` | Hue_Sat_Bright_Autogen | 130 | 196 | 196 |
| `S_Sharpen` | Sharpen_Best_Quality_Autogen, Sharpen_Fast_Autogen | 43 | 114 | 114 |
| `S_TimeWarpRGB` | Time_Warp_RGB_Autogen | 22 | 22 | 22 |
| `S_Trails` | Trails_Autogen | 78 | 170 | 220 |
| `S_WarpPerspective` | Warp_Perspective_Autogen | 19 | 256 | 256 |
<!--/gen:effects_status-->

*Counted rows* are the booked comparison rows of the original's emulator runs against the native code that count for the effect: <!--f:effects_counted_rows-->425<!--/f--> in all. *Frozen members* are the recorded native predictions of the final binary: <!--f:effects_frozen_members-->989<!--/f--> in all. *Re-run units* are the same members run again with the binary built from this repository, with a second render for the multi-render rows: <!--f:effects_rerun_units-->1,039<!--/f--> units, every one byte identical to its frozen prediction by full SHA-256 in the three output files (`ae32`, `dest` and `src0`). `BlurMoCurves` is listed by the name of its host record; that string is not in the name bank's list of effect names.

## Report wording

The text each effect is reported with, copied from its paragraph in the recovery workspace.

### `S_Blur`

```text
S_Blur (Blur_Color_Autogen and Blur_Mono_Autogen, the two registered styles selected by the Effect popup) is rebuilt in native Rust from the original plugin's code and data. Every building block of its programs matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 48 counted compare rows END-TO-END EQUAL, plus 4 inert controls (the Blur_Color_BM and Blur_Mono_BM rows under MOCHA-PARAMS-NOT-CLAIMED), also END-TO-END EQUAL: 52 of 52 booked compare rows EQUAL, all 52 booked FINAL (Blur_Color and Blur_Mono at 64x32 and 50x7, including the Opacity answer 1 and 3 rows)(1). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (AUD4 B1: 0 rcp or rsqrt entries among the builtin entries of the route classes). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images, a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. If the harness arrangement named LICENCE-GATES-FORCED is changed by Ale's decision, this result is re-run under the new arrangement before it is reported, or withdrawn. Licence, activation, trial and machine-identity code of the original is not rebuilt and not analysed beyond the judge-side gate answers of W130.
```

### `BlurMoCurves`

```text
S_BlurMoCurves (two registered styles: Transform_and_Blur, run by Transform_and_Blur_Autogen, and Blur_Only, run by Blur_Motion_Curves_No_Transform_Autogen) is rebuilt in native Rust from the original plugin's code and data. Every building block of its programs matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 41 counted rows END-TO-END EQUAL on fx_run_s32_g4 (24 Transform_and_Blur and 17 Blur_Only, at the claim sizes 64x32 and 50x7), with 1 inert control and 10 history rows not counted (BOOKED.tsv). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (APPROX-MODEL, INFERRED for this effect: the BMC route listing file AUDIT_BUILTINS_LISTING_BMC.tsv covers the entry stubs of the 7 route classes, 14 slot entries, and shows 0 rcp or rsqrt instructions in them (MEASURED by AUD10); the kernel bodies behind the stubs have no BMC listing file on record, so whether an approximate instruction is reached on this route is not measured here and no ulp figure is stated; another effect's figure is not used as evidence, W1018 (c)). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units with curve values supplied by the host stand-in HC-KEYED-LINEAR, one thread, the licensed state as set by the test harness (the W130 gate answers), f32 images, and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. Licence, activation, trial and machine-identity code of the original is not rebuilt and not analysed beyond the judge-side gate answers of W130.
```

### `S_Diffuse`

```text
S_Diffuse (Diffuse_Autogen, its only registered style) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 40 of 40 compare rows END-TO-END EQUAL (31 booked FINAL: 20 list runs, mocha 11; 6 inert controls (GB1, REACH and ZRB at both sizes, W792); the 3 IMB rows compared and recorded, not booked as branch sets, Batch 52)(1)(2). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (APPROX-MODEL: the one approximate site reached on this route, rcpps 0xb77c93 in the plugin's log 0xb78140, changes the level counts and the log result bits by 0 under W49's bound, so the output change is 0 ulp; see item 8). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images, a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. If the harness arrangement named LICENCE-GATES-FORCED is changed by Ale's decision, this result is re-run under the new arrangement before it is reported, or withdrawn. Licence, activation, trial and machine-identity code of the original is not rebuilt and not analysed beyond the judge-side gate answers of W130.
```

### `S_DistortRGB`

```text
S_DistortRGB (Distort_RGB, effect Distort_RGB_Autogen on the claim route) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 30 of 30 counted rows END-TO-END EQUAL (28 byte-EQUAL by full SHA-256 and 2 EQUAL BY NAN CLASS with payloads not claimed, ALPHA-SPECIAL-RUN; 30 of 30 BOOKED-FINAL; CMP_COUNTED.tsv and BOOKED.tsv of drc1_distort_e2e). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (no approximate instruction on this route: 0 rcp or rsqrt ops in the 42 listing entries of the 21 route classes, aud3_distort_audit/AUDIT_BUILTINS_LISTING_DISTORT.tsv, MEASURED by AUD3). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state, f32 images, and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed.
```

### `S_DropShadow`

```text
S_DropShadow (Drop_Shadow, registered without an Autogen wrapper) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 24 of 24 booked rows END-TO-END EQUAL in both renders (BOOKED_G2E.tsv, MEASURED; by size 13 at 64x32, 10 at 50x7, 1 at 49x9, plus 3 POSTHOC-SEEN, 3 MISS-50x7-FEED and 3 inert controls named as limits). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (no approximate instruction reached on this route: the 11 approximate sites in the cited ranges are not entered (aud5_dropshadow_audit/APPROX_DS.md line 3; the log sites INFERRED from the plan constants, not hooked)) (W1018 (c)). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images, a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed.
```

### `S_HueSatBright`

```text
S_HueSatBright (Hue_Sat_Bright_Autogen, its only registered style) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator (the six run primitives of the claim route, hue_shift, hue_shift_preserve_luma, sat_scale, `*`, `+` and dissolve, by their cited labels on the pinned tree, W494; blur runs only at a non-default Blur Mocha, which is not claimed, W792, W802), our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen (196 of 196 certificate rows of the booked runs W418-ROW EQUAL, CERT_R.tsv), and the whole effect produces the same pixels as the original CPU render path on test images: 130 of 130 counted runs END-TO-END EQUAL in all four slots (CMP_R.tsv rows of the 130 counted runs by PAR2's own counting rule, w802_restate.py, equal to AUD1M's count; the 66 inert controls are also END-TO-END EQUAL and are named, not counted: listed in HSB_INERT66.json; W786 block rule 6, W802), 196 booked rows, one per binding and size (130 counted and 66 inert controls, W792, W802), every one END-TO-END EQUAL and every frozen prediction PRE-REPREDICT-EQUAL on the claim binary. The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ on real CPUs (APPROX-SENSITIVE: hue_shift_preserve_luma, worst measured change within the SDM bound 25,244 ulp, the flag test at 0x34dcfa NOT ENTERED on the claim route). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images with a three-plane class-3 source and the destination alpha taken from the host source row (DEST-ALPHA-FROM-SOURCE-ROW), an in-place host for the mode-3 branch (MODE3-CLASS3-ROUTED; whether a real After Effects host renders a SmartFX in place is not claimed), a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), and the other named limits below. The host serial number field reads 0 in the claim host model and a real host's value is not claimed (HOST-SERIAL-ZERO). Host values outside a parameter's registered valid range are not claimed; the one counted-pass binding outside it (h2BG5, Brightness -0.25) is kept as information (BINDING-OUT-OF-RANGE). Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. If the harness arrangement named LICENCE-GATES-FORCED is changed by Ale's decision, this result is re-run under the new arrangement before it is reported, or withdrawn.
```

### `S_Sharpen`

```text
S_Sharpen (Sharpen_Best_Quality_Autogen and Sharpen_Fast_Autogen, its two registered styles) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 43 booked rows (35 ordinary and 8 Opacity rows) END-TO-END EQUAL in both renders by full SHA-256, with 51 certificates PLAN-EQUAL (BOOKED.tsv, CERT_RESULTS.tsv); 18 inert controls (the W749 mocha rows) and 8 POSTHOC-SEEN rows are recorded and not counted; the 1018 rows of FEED_NATIVE_JOIN.tsv over the booked rows reconcile with the 975 float-lane comparisons (932 EQUAL plus 43 INERT-UNFETCHED; the 43 OUTPUT-INERT-OR-OWNER-ARM rows are the owner-arm opacity rows, compared through EQUAL-VIA-OWNER-ARM). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (APPROX-SENSITIVE: the edge_aware_sharpen core of the Best style is classed APPROX-SENSITIVE with a 6 ulp bound by W92 (esh5_core/CHECKPOINT.md, INFERRED class), and its heap legs hBN_64x32 and hFN_64x32 give byte identical outputs under --cxxarena 0 (HEAP-LEG-TAG-ONLY, AUD9); the Fast route lists no approximate class, AUD9 ITEMS.tsv W757-A13b, INFERRED). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images, a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), and the other named limits below (MOCHA-PARAMS-NOT-CLAIMED, HOST-INPLACE-NOT-CLAIMED, MASK-NS-PARAMS-AT-DEFAULT, HOST-ASPECT-SQUARE, FRAME-CACHE-FRESH, HOST-FRAMES, HOST-HEAP-PHASE-MODELLED, MODE-BODY-NOT-PORTED, HOST-SERIAL-ZERO, LICENCE-FAMILY-CELLS-LEFT-OUT). Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. If the harness arrangement named LICENCE-GATES-FORCED is changed by Ale's decision, this result is re-run under the new arrangement before it is reported, or withdrawn. Licence, activation, trial and machine-identity code of the original is not rebuilt and not analysed beyond the judge-side gate answers of W130.
```

### `S_TimeWarpRGB`

```text
S_TimeWarpRGB (Time_Warp_RGB, with Time_Warp_RGB_Autogen on the claim route) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 22 of 22 counted rows END-TO-END EQUAL by full SHA-256 of the compared render, 22 of 22 booked on fx_run_s32_g1t with 22 of 22 W418 certificates and 22 of 22 control builds EQUAL (BOOKED.tsv, CERT_TW.tsv, control_g1t/CONTROL.tsv). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (no approximate instruction on this route: 24 listing entries of the route's builtins, 0 rcp or rsqrt, AUD6 A9, aud6_timewarp_audit/README.md and AUDIT_BUILTINS_LISTING_TW.tsv, MEASURED). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images, a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), one effect instance per process (HOST-ONE-INSTANCE), a fresh frame cache per render or fractional parts shared across a sequence (CONTRACT-LIMIT FRAME-CACHE-FRESH), and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. If the harness arrangement named LICENCE-GATES-FORCED is changed by Ale's decision, this result is re-run under the new arrangement before it is reported, or withdrawn.
```

### `S_Trails`

```text
S_Trails (Trails_Autogen, its only registered style) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 78 of 78 booked rows END-TO-END EQUAL in both renders by full SHA-256 (42 base rows, 34 ADD-H KEEP bindings, 2 SZN size-change rows; COUNTS_FINAL.tsv, BOOKED.tsv); 18 base and 8 ADD-H inert controls (mocha family, W749, W756) and the 6 default-binding duplicates are information and not counts. The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (no approximate instruction on this route: 0 rcp or rsqrt ops, MEASURED for all ten route classes of the Trails plans (W1018 (d)): approx_recip_ops is 0 in the 14 entries of min, *, +, blur, hue_shift, dissolve and postdivide at slots 38 and 49, aud2_hsb_audit/AUDIT_BUILTINS_LISTING.tsv rows 2 to 49 (48 of 48 rows 0), and 0 rcp or rsqrt ops in each entry of get_prev_dest, save_prev_dest, screen_super, hue_shift and min by address, aud8_trails_audit/CLOSE_RESIDUES_W9.md section A9). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them, f32 images, and the other named limits (HOST-ONE-INSTANCE, HOST-ASPECT-SQUARE, HOST-SERIAL-ZERO, HOST-CLOCK, DATE-GATE, HOST-COLOUR-ALPHA-FF, LICENCE-FAMILY-CELLS-LEFT-OUT, FRAME-CACHE-FRESH, MASK-NS-PARAMS-AT-DEFAULT, MOCHA-PARAMS-NOT-CLAIMED, DROPPED-MASK-ONLY, ALPHA-MODE-HOST). Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed. Licence, activation, trial and machine-identity code of the original is not rebuilt and not analysed beyond the judge-side gate answers of W130.
```

### `S_WarpPerspective`

```text
S_WarpPerspective (Warp_Perspective, with Warp_Perspective_Autogen on the claim route) is rebuilt in native Rust from the original plugin's code and data. Every building block of its program matches the original code bit for bit when both run on the same inputs in an emulator, our evaluator produces the same execution plan as the original compiler on parameter sets it had never seen, and the whole effect produces the same pixels as the original CPU render path on test images: 19 counted rows (9 at 64x32 and 10 at 50x7, booked on fx_run_s32_g2f) each EQUAL in render 1 and render 2 by full SHA-256 to the frozen member its own record selects, with certificate and control (BOOKED.tsv; the 2 void OP1Z rows are information and not counted). The judge is an emulator that follows Intel's specification, not real silicon, so vector sections that use approximate reciprocals may differ slightly on real CPUs (APPROX-SENSITIVE: perspective out1 6 ulp, out2 12,972 ulp (cancellation near zero, 6.82 x 2^-24 of operand scale), W179 and W92, DECISIONS_20261030.md line 827; CONTRACT-LIMIT for F1 above n = 4 is named (W179, line 827; label CONTRACT-LIMIT); the route's six approximate-reciprocal sites are listed in APPROX_SITES.md, accepted at DECISIONS_20261030.md line 1027). The After Effects host side is modelled, not rebuilt: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state as set by the test harness, which answered three of the plugin's licence-family routines with fixed values instead of running them (LICENCE-GATES-FORCED), f32 images, a disconnected host whose licence client finds no licence server (DISCONNECTED-HOST, HOST-LICENCE-UNREACHABLE), 64x32 and 50x7 as the only claim sizes (CLAIM-SIZES), the word 2 = 0 certificates not claimed at 50x7 (WORD2-XY-SPLIT), the rows lost to named limits (MISS-FEED-SIGN, MISS-50x7-OPEN, MISS-RENDER2-PHASE), the alpha mode derived by the dump's rule from the host fields (ALPHA-MODE-HOST), and the other named limits below. Behaviour inside a real host, keyframed animation, GPU rendering and 8 or 16 bit formats is not claimed.
```

## Build and run

From the repository root, offline, with a stable Rust toolchain:

```sh
cargo build --release --offline -p fx_exec
```

This builds `target/release/fx_run`. The call is

```sh
fx_run PROGRAMS.TSV PROGRAM --size WxH --out PREFIX [--synth SEED] [options]
```

- `PROGRAMS.TSV` is the table of effect program texts of the plugin. It comes with the optional data bundle (the file `programs.tsv` in the bundle's folder) and is not part of the main files of this repository.
- `PROGRAM` is the registered program, for example the wrapper program of a style (see the table above).
- Planes are raw little endian `f32`, planar. `--in Name=path` binds an input plane, `--synth SEED` fills every unbound input with a fixed pseudo random plane, `--set name=v1,v2` sets a parameter. The output is `PREFIX.dest.f32` with `PREFIX.dest.meta.txt`, which lists the run and every label the run carries. The full option list is the header of `crates/effects/fx_exec/src/bin/fx_run.rs`.
- The host context is part of the call: options such as `--ctx-time`, `--ctx-word2` and `--bound-mask` are required and have no default, and a call that omits one is refused with a message that names it. The calls the evidence was made with are recorded per member in the recovery workspace.
- The tool refuses what it does not reproduce and says so by name, for example an input class or a parameter value outside the recorded cases.

### Runtime data

Running an effect needs the optional data bundle: its `programs.tsv` as the programs argument and its tables through `EFFECTS_DATA_DIR`. The bundle holds data taken from the original plugin, the program texts and a set of lookup and class tables, and it comes with a list of their sources. It is kept out of the repository's main files. `EFFECTS_DATA_DIR` names the folder that holds the tables; when it is unset the folder is `runtime_data` beside the effect crates. When a table is absent `fx_run` stops with exit code 2 and a message that names the table and the folder.

Without the bundle the product builds and its tests pass, but no effect renders: the tests that need the tables or the stored data print a `NOT RUN` line and pass. Tables that carry only text written for the project are part of the source.

## Tests

- `cargo test --release` runs the effect crates whose tests need no stored data and no runtime tables, with the other default members.
- `cargo test --release -p fx_eval -p fx_exec` runs the rest of the evaluator and the executor. A test that needs stored data from the original (recorded plans, emulator rows, the program corpus) prints a `NOT RUN` line and passes, the way the lift crates' vector tests stay out of a plain `cargo test`.
- `VEC_DIR` names a folder that holds an `effects` folder with the stored data, laid out with the relative paths the tests print in their `NOT RUN` line; with it and the runtime tables every test runs. Tests write their files to the system temporary folder, never into the source tree.

## Limits common to the ten effects

- Every result is judged by the emulator, which follows Intel's specification (SDM), not by real CPU silicon.
- `ROUTE AVX2-ONLY`: the original selects one of several code tiers from the CPU's features. The native code reproduces the tier that runs on CPUs with AVX2 and FMA; on CPUs without them the original runs other tiers with different rounding, and those results are not reproduced.
- Where an operation has an approximate class (APPROX-SENSITIVE), the effect's paragraph states its bound in ulps.
- Not claimed: a real host application (its parameter units, keyframed animation, multi band and multi threaded rendering, real frame fetches), GPU rendering, 8 and 16 bit formats, trial or unlicensed rendering, mocha with tracking data, and NaN payload and sign bits.
- Licence, activation, trial and machine identity code of the original is not rebuilt.
- Each paragraph adds its own named limits and the rows it does not count.

## Evidence

The native code was written by reading the original plugin's machine code and data. Whether it may be published or distributed, and under what licence, is the owner's decision; see [Legal](../README.md#legal).

The full report, the ten paragraphs with their exclusion tables and the runs behind them are kept in the recovery workspace, not in this repository. This page carries their SHA-256 so that a copy can be checked.

The full report (`FINAL_REPORT_W141_20261105.md`): `b7ee202da80faf7ce31f95ea5cced46badf3f1b7dacdbec8dc1ea8b865e2910c`

<!--gen:effects_paragraph_sha-->
| effect | SHA-256 of its paragraph |
|---|---|
| `S_Blur` | `39687d0d49bc354ecb98dc37ac834b6e4d7b64ce366b1c3f06cdbc1843089149` |
| `BlurMoCurves` | `6be5fcc826a4c3cdae20361637b34a77c05282ea3002f65db671e08771668c5d` |
| `S_Diffuse` | `195b220b283f4b16e5fac72fe44f32e16375f00c0f34ba81d900f190c99c2487` |
| `S_DistortRGB` | `63981f3f11e5d17a82d1946b3b0163f365e2782af76195565a72e07b803e5051` |
| `S_DropShadow` | `f994afc72cba6a0c7c11d4edd30a81d8546dfc63b8559c26cdad81cb993180bb` |
| `S_HueSatBright` | `28493c431fea7a9190bd1fe64d051a8eba4a6680f056a2807d88b6269edb1e43` |
| `S_Sharpen` | `86756d4638788e9dc53ffaa18db5111c9a71ffa153f70832c39d2f3f56694a66` |
| `S_TimeWarpRGB` | `efe10cebfd7407fc2a850037ed9e8c5bda4196e0604a5960d65a623e307fceba` |
| `S_Trails` | `1686f74200f7b49bd08158b98e7f107912f91ca893d369bbed22e1c225044959` |
| `S_WarpPerspective` | `6a4be1cc4983091115367861f832ea0806fd663b687b33ac4fe82f8b2dcc425b` |
<!--/gen:effects_paragraph_sha-->
