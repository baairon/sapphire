# sapphire

An independent open source reimplementation of Sapphire behaviour, function by function. It is not affiliated with or endorsed by any other company, product or library.

sapphire has two parts with different jobs. The **effects** are <!--f:effects_rebuilt-->19<!--/f--> of the plugin's video effects, rebuilt in Rust: `fx_run` renders one effect from its input planes, and the `sapphire` command decodes a clip with ffmpeg, passes its frames through one or more effect stages (and, if wanted, twixtor's `vfx`), encodes the result and checks it before writing the output. The **function record** is a reverse-engineering reference of one small code section of a separate code image of a build of the plugin, by its function names mostly bundled video hardware library code and std: for each of its <!--f:fn_total-->2,702<!--/f--> functions it holds the exact name, the call edges, a Rust port and the evidence that the port matches the original on every compared input, and [docs/parity.md](docs/parity.md) says exactly what is covered. The effects are rebuilt separately and do not call the function record ([Effects and stages](#effects-and-stages)).

## Status

- **Effects.** Nineteen effects run as native Rust and match the original bit for bit on every compared case, apart from four counted comparisons of `S_Shake` and `S_DissolveShake` whose recorded renders were made before three faults were corrected, two in the native code and one in a value handed to the runner, and 34 of `S_GlowEdges` that were run with slider values passed as decimals, not as the 16.16 fixed-point values a host passes ([effects](docs/effects.md)). They render through `fx_run` and as `sapphire stage` commands, from code in this repository alone, with no external data. The [roadmap](docs/roadmap.md) lists the effects the rebuild covers: the nineteen that are done and `S_Retime`, which is not yet done.
- **Speed.** The CPU path renders one 1920x1080 frame of the slowest stage, `blur_motion_curves_no_transform_autogen`, in about 1.1 s where the earlier one process per frame route took about 52 s (MEASURED; [Speed](docs/guide.md#speed)).
- **Keyframes.** Every effect stage accepts keyed parameter values (`VALUE@FRAME;VALUE@FRAME`) except `trails_autogen`; each frame is a plain run at the value the keys give for it (MEASURED), and the values between keys are INFERRED ([guide](docs/guide.md#running-a-stage-on-a-clip)).
- **Device path.** `--device gpu|gpu=N|auto` and `SAPPHIRE_DEVICE` run the kernels through OpenCL, byte identical to the CPU path. It is MEASURED on PoCL, an OpenCL runtime on the CPU, only; speed on a real GPU is INFERRED ([guide](docs/guide.md#rendering-on-a-device)).
- **Function record.** A reverse-engineering reference of one small code section of a separate code image; by their bank names its functions are mostly the bundled AJA video hardware SDK and std, plus display and video output classes of its own ([Names](#names)), and no effect uses them. <!--f:strict_fns-->2,702<!--/f--> of <!--f:fn_total-->2,702<!--/f--> functions reach [strict parity](docs/glossary.md#strict) ([Parity](#parity)).
- **Usage.** [docs/guide.md](docs/guide.md) shows how to run the effects on a video; the [Command-line reference](docs/cli.md) lists every flag, environment variable, exit code and refusal.

## Contents

- [Status](#status)
- [Build](#build)
- [Quick start](#quick-start)
- [Documentation](#documentation)
- [Parity](#parity)
- [Effects and stages](#effects-and-stages)
  - [Names](#names)
  - [Crate map](#crate-map)
- [Interface](#interface)
  - [Pipeline](#pipeline)
  - [Stage commands](#stage-commands)
  - [Raw frame format](#raw-frame-format)
  - [Exit codes](#exit-codes)
  - [Python tools](#python-tools)
- [Handoff](#handoff)
- [Tests](#tests)
  - [Smoke test](#smoke-test)
- [Limits](#limits)
- [Legal](#legal)
- [License](#license)

## Build

From the repository root, with a stable Rust toolchain, offline:

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
```

On a Windows machine without the MSVC linker the GNU toolchain builds the same code: `cargo +stable-x86_64-pc-windows-gnu build --release --offline -p neutral-handoff`. CI builds `fx_run` and runs the effect golden hashes (`tools/fx_golden.py`) on Linux, macOS and Windows; the device checks on PoCL and the tests of the effect crates outside the default members run on Linux only.

| Binary | Path (`.exe` added on Windows) | Used by |
|---|---|---|
| `sapphire` | `target/release/sapphire` | the pipeline, the stage provider of `contracts/tool.json`, `tools/handoff.py doctor`, `tools/conformance.py` |
| `fx_run` | `target/release/fx_run` | the command line of the effect runner; the 20 effect stages use the same library inside `sapphire` and need no `fx_run` process. The executable is needed only to run `fx_run --describe-params` from a shell; `stage list --json` prints the ranges and defaults without it |
| `fake-vfx` | `target/release/fake-vfx` | the pipeline tests, as a stand in for twixtor's `vfx` |

`CARGO_TARGET_DIR` moves the binaries; `SAPPHIRE_PATH` points the Python tools at another build. The pipeline and the `neutral-ffmpeg` tests need ffmpeg and ffprobe ([finding ffmpeg](docs/ffmpeg.md#finding-ffmpeg)).

## Quick start

After [Build](#build), with ffmpeg on `PATH`, from the repository root. Make a one second test clip, list the stages, then run one effect stage over the clip:

```sh
ffmpeg -v error -f lavfi -i testsrc=size=160x96:rate=24 -t 1 -pix_fmt yuv420p clip.mp4
target/release/sapphire stage list
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen hue_shift=0.25" -o clip_hue.mp4
```

The last command decodes `clip.mp4`, shifts its hue with the `S_HueSatBright` stage, encodes `clip_hue.mp4` and verifies it. Depth 32 is the render path the [parity record](docs/glossary.md#parity-record) covers. Stage ids, parameters, chaining and twixtor are in the [guide](docs/guide.md).

## Documentation

- [Command-line reference](docs/cli.md): every command, flag, environment variable, exit code and refusal of `sapphire`, `sapphire stage` and `fx_run`.
- [docs/guide.md](docs/guide.md): using the effects on a clip or on raw frames, chaining, parameters, speed and what is claimed.
- [docs/parameters-guide.md](docs/parameters-guide.md): every stage parameter, how to write its value, and the values verified against recorded renders.
- [docs/effects.md](docs/effects.md): the nineteen native effects, their status, how to build and run `fx_run`, and their shared limits.
- [docs/stages.md](docs/stages.md): the stage manifest and apply command, how a new provider joins, and the effects provider.
- [docs/roadmap.md](docs/roadmap.md): the effects the rebuild covers, the nineteen done and the one not yet done, and the work every effect shares.
- [docs/adding-an-effect.md](docs/adding-an-effect.md): how to add an effect.
- [docs/glossary.md](docs/glossary.md): the technical terms of the function record.
- [docs/parity.md](docs/parity.md): what parity means for the function record, the tiers, strict and strictest, the caveats.
- [docs/harness.md](docs/harness.md): the oracle, worlds, the `Env` trait, vector sets, comparison and replay.
- [docs/lifting.md](docs/lifting.md): what the function record documents, mechanical lift and hand port, the crate layout, the x86 rules the ports reproduce.
- [docs/evidence.md](docs/evidence.md): the evidence classes and the record behind each function's status.
- [docs/assumptions.md](docs/assumptions.md): every assumption the parity figures rest on.
- [docs/missing-data.md](docs/missing-data.md): references the dump does not contain, and their placeholders.
- [docs/reproduce.md](docs/reproduce.md): what reruns from this repository alone and what needs the archive.
- [docs/names.md](docs/names.md): the name bank, origins, the name check and confirmation levels.
- [docs/ffmpeg.md](docs/ffmpeg.md): how sapphire finds and runs ffmpeg.
- [docs/handoff.md](docs/handoff.md): how sapphire, twixtor and ffmpeg pass raw frames to each other.
- [docs/generated/index.md](docs/generated/index.md): the generated reference of functions, classes, stubs, effects, parameters and crates.
- [docs/handoff-bench.tsv](docs/handoff-bench.tsv): the wrappers timed on chained stages, written by `tools/conformance.py --bench`.

`docs/index/` holds the machine readable index: `facts.json` (every figure with its source), `functions.jsonl` (one row a function) and `pages.json` (every page with its title and size). `docs/data/` holds the checked in evidence, among it the function map `docs/data/FUNCTION_MAP.tsv`; `docs/generated/`, `docs/index/` and the figures in every page are written from it by `python -I tools/gen_docs.py`, so change the data, then regenerate.

For scripts, the quickest answers are: `python -I tools/fn_info.py ID|NAME|OFFSET [--json]` for one function, `docs/index/functions.jsonl` for every function, `docs/index/facts.json` for any figure, `target/release/sapphire stage list --json` for this repository's stages, `python -I tools/handoff.py stages --json` for every stage of sapphire and twixtor with a ready command, and `target/release/sapphire INPUT [EFFECT] ... --dry-run` for what a run will do.

## Parity

The figures in this section are those of the function record: the <!--f:fn_total-->2,702<!--/f--> functions of the [dump](docs/glossary.md#dump), one small code section of a separate code image; by their bank names they are mostly bundled video hardware library code and std, plus display and video output classes of its own ([Names](#names)). The <!--f:effects_rebuilt-->19<!--/f--> effects are checked separately ([what rebuilt means](docs/effects.md#what-rebuilt-means)).

1:1 parity with the original code comes before speed, convenience and a cleaner design: when they conflict, parity wins.

- **Output.** A port gives what its original function gives on every compared input: bit for bit on oracle vectors ([tier P1](docs/glossary.md#p1)), and the same returns, memory writes and calls in every [compared world](docs/glossary.md#compared-world) ([tier P2v](docs/glossary.md#p2v)) ([tiers](docs/parity.md#tiers)).
- **Interface.** Each port keeps its function's offset in its symbol, `f_XXXXXXXX`, and the name bank gives that function's exact name.
- **No near misses.** A port that differs on any compared input is in no parity figure ([unproven](docs/parity.md#unproven)).
- **Measured, not assumed.** Each function and its port run on the same generated inputs and every result is compared ([harness](docs/harness.md)); what the figures rest on is in [assumptions](docs/assumptions.md).

| Standard | Functions | Share of code bytes |
|---|---|---|
| Strict parity: matched on every compared input | <!--f:strict_fns-->2,702<!--/f--> of <!--f:fn_total-->2,702<!--/f-->, of which <!--f:close_labelled_rows-->1<!--/f--> rests on a stated assumption | <!--f:strict_pct-->100.0<!--/f-->% (<!--f:close_labelled_pct-->1.3<!--/f-->% of the code rests on stated assumptions) |
| Strictest, standard revised on 2026-11-05: strict, with every [branch outcome](docs/glossary.md#branch-outcome) seen in a compared world or explained under the [harness model](docs/glossary.md#harness-model), including the rules HR-P2 and HR-FP | <!--f:strictest_rev_fns-->2,701<!--/f--> of <!--f:fn_total-->2,702<!--/f--> | <!--f:strictest_rev_pct-->98.7<!--/f-->% |
| Strictest, earlier definition (compared worlds and the original harness model only, evidence class `audit3` excluded) | <!--f:strictest_fns-->2,697<!--/f--> | <!--f:strictest_pct-->97.9<!--/f-->% |
| Unproven: never matched | <!--f:unproven_fns-->0<!--/f--> | <!--f:unproven_pct-->0.0<!--/f-->% |

The <!--f:close_labelled_rows-->1<!--/f--> function that rests on a stated assumption is [strict](docs/glossary.md#strict), not [strictest](docs/glossary.md#strictest): `f_0001b438`, under assumption A1 (the sorted range is a whole number of 248 byte records, <!--f:close_rows_A1-->1<!--/f--> row). The earlier assumptions F (an outcome that only faulting executions take, <!--f:close_rows_F-->0<!--/f--> rows) and P2 (stores through the function's own frame never hit the record array, <!--f:close_rows_P2-->0<!--/f-->) became the harness rules HR-FP and HR-P2, and PN (<!--f:close_rows_PN-->0<!--/f-->) is retired. Each is defined in [assumptions](docs/assumptions.md#summary) and listed in [evidence](docs/evidence.md#the-last-seven-rows).

The strictest standard was revised on 2026-11-05: the harness model gained two named rules, HR-P2 (frame disjointness) and HR-FP (fault parity) ([harness](docs/harness.md#harness-rules-hr-p2-and-hr-fp)). A function that reaches strictest only through them has the [evidence class](docs/glossary.md#evidence-class) `audit3`, which the second strictest row includes and the third row excludes, so both figures stay visible. A figure rises only with its measurement and an independent audit: <!--f:ev_audit3_fns-->4<!--/f--> functions have `audit3` at this date (`f_00031780` and `f_000651c2` under HR-FP, `f_0002405b` and `f_00024a39` under HR-P2), each proof re-derived independently. PN is retired (`f_00056fa6` now has its 0x57936 outcome seen without a pin). A1 remains for one function: `f_0001b438` keeps it because its outcomes 0x1d3bc, 0x1ddc9 and 0x1e3ba still need the caller precondition A1 (a valid sort range), which is not a harness fact. No world of any kind takes them, and the standard does not make a caller precondition part of the harness model; doing so would be a further change of the standard.

Known port divergences outside the harness model are listed in [parity](docs/parity.md#known-port-divergences-outside-the-harness-model): worlds that put a buffer argument inside the function's own frame (they violate HR-P2) expose differences in `f_0001b438` (32 of 1609 stack-resident worlds) and in `f_0002405b` and `f_00024a39` (11 stack-resident worlds). They are open port fixes and change no figure.

Shares are of the <!--f:code_bytes-->964,659<!--/f--> code bytes of the <!--f:fn_total-->2,702<!--/f--> functions, most of them bundled library code.

## Effects and stages

The two parts of the repository share a method and its tools, but their crates share no code: each part judges Rust code against the original running in the Unicorn [emulator](docs/glossary.md#emulator). The <!--f:effects_rebuilt-->19<!--/f--> effects are rebuilt from the plugin's effect programs and the builtin classes they call ([what rebuilt means](docs/effects.md#what-rebuilt-means)); the effect crates cite the listing of a separate code section of a build of the plugin, much larger than the one the function record covers, and none cites the record's section. Per the name bank, the function record's section holds bundled library code, std, component code (by its class names, display and video output classes) and functions of undetermined origin, and the bank maps no effect to any of them ([Names](#names)).

The effects' tools build on the function record's. Their kernel comparisons run in a harness that uses the register and error tables of the record's oracle (`archive:tools/oracle.py`); the disassembly listing that the effect crates cite was made by the record's indexing tool (`archive:tools/index_dump.py`); and an adapted copy of the record's lifter has lifted and compared functions of the larger section, none of which is in this repository. The effect crates use no code of the function record: their x86 rules are their own (`crates/effects/fx_common/src/x86.rs` and `crates/effects/fx_common/src/nanop.rs`), and no effect crate and no command depends on a lift crate, `neutral-world` or `neutral-parity` ([Crate map](#crate-map)).

The nineteen native effects, their status and what each claim covers are in [docs/effects.md](docs/effects.md); the effects the rebuild covers are listed in [docs/roadmap.md](docs/roadmap.md). A stage is a program that passes raw frames through the pipeline; a new provider joins as in [docs/stages.md](docs/stages.md).

Each registered style of the nineteen effects is also a frames/2 stage, so twixtor and any other wrapper can run it as `sapphire stage apply STAGE` (25 stages). They are linked into `sapphire`, are always listed and need no `fx_run` process; `fx_run` is needed only for `--describe-params`. The [guide](docs/guide.md) shows how to run them.

| Effect | Stages | Kind | Byte identical through the stage |
|---|---|---|---|
| `S_Blur` | 2 | temporal | 36 of 48 recorded renders |
| `S_BlurMoCurves` styles | 2 | temporal | 26 of 67 |
| `S_BlurMotion` | 2 | temporal | 232 of 352 |
| `S_Diffuse` | 1 | temporal | 31 of 42 |
| `S_DissolveShake` | 1 | temporal | 27 of 36 |
| `S_DistortRGB` | 1 | temporal | 26 of 30 |
| `S_DropShadow` | 1 | temporal | 20 of 24 |
| `S_Flicker` | 1 | temporal | 24 of 24 |
| `S_Glint` | 1 | temporal | 22 of 23 |
| `S_Glow` | 1 | temporal | 36 of 36 |
| `S_GlowEdges` | 1 | temporal | 34 of 34 |
| `S_HueSatBright` | 1 | temporal | 118 of 130 |
| `S_Shake` | 3 | temporal | 54 of 66 |
| `S_Sharpen` | 2 | temporal | 35 of 78 |
| `S_SwishPan` | 1 | temporal | 32 of 36 |
| `S_TimeWarpRGB` | 1 | temporal | 10 of 22 |
| `S_Trails` | 1 | temporal | 70 of 120 |
| `S_WarpPerspective` | 1 | temporal | 10 of 224 |
| `S_WarpTransform` | 1 | temporal | 61 of 78 |

All 25 stages have kind `temporal`, because the frames/2 convention accepts keyed values only for temporal stages. Only `time_warp_rgb_autogen` and `trails_autogen` read frames other than the current one; the rest are `temporal` so that they accept keyframes. `swishpan` and the `S_DissolveShake` stage are transitions: they also read a second clip, given with `--layer`, and the clip length, given with `--frames` ([transition stages](docs/cli.md#transition-stages)).

The first number counts the recorded renders of the parity record that use the stage's fixed host arrangement; each of them is byte identical through `sapphire stage apply` at depth 32 (382 of 382 on the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective`, 24 of 24 on `flicker`, 36 of 36 on `glow`, 32 of 32 on `swishpan`, 54 of 54 on the three `S_Shake` stages and 27 of 27 on the `S_DissolveShake` stage). On `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage the arrangement follows the Opacity answer (host variants). The other recorded renders use a different host arrangement and are not claimed for the stage. For `S_Shake` and `S_DissolveShake` the second number counts the counted renders that equal the original ([effects](docs/effects.md)). Each stage's host, parameters, size refusals, timing and claim limits are in [docs/stages.md](docs/stages.md#effects-provider).

### Names

The name bank is the source of truth for every name; the code keeps its `f_XXXXXXXX` symbols ([names](docs/names.md)).

| Table | Rows | Holds |
|---|---|---|
| `docs/data/names.tsv` | <!--f:names_total-->2,702<!--/f--> | per function: exact C++ name, Rust label, kind, origin, mangled name |
| `docs/data/stub_names.tsv` | <!--f:stubs_total-->458<!--/f--> | named import stubs |
| `docs/data/classes_names.tsv` | <!--f:classes_total-->16<!--/f--> | local classes with their base classes |
| `docs/data/effect_names.tsv` | <!--f:effects_total-->272<!--/f--> | effect names |
| `docs/data/param_candidates.tsv` | <!--f:params_total-->1,040<!--/f--> | parameter candidate strings, bound to no function |

**Effect names.** The <!--f:effects_total-->272<!--/f--> effect names are strings only: none is mapped to functions, classes, parameters or behaviour, and the nineteen effects of [docs/effects.md](docs/effects.md) are rebuilt separately as native code. They are listed verbatim in [docs/generated/effects.md](docs/generated/effects.md), the parameter candidates in `docs/generated/parameters.md`; a candidate is not a confirmed parameter.

**Origin.** The bank classes <!--f:origin_bundled-->1,983<!--/f--> functions as bundled library code (the class `CNTV2Card` alone has <!--f:cntv2card_fns-->906<!--/f-->), <!--f:origin_std-->514<!--/f--> as std, <!--f:origin_undetermined-->136<!--/f--> as undetermined and <!--f:origin_component-->69<!--/f--> as component, by their class names the section's own display and video output classes ([names: origin](docs/names.md#origin)).

**Confirmation.** Each function name has one level in `docs/data/name_confirmation.tsv`:

| Level | Functions | Meaning |
|---|---|---|
| `confirmed` | <!--f:conf_confirmed-->1,085<!--/f--> | at least two independent positive pieces of evidence |
| `consistent` | <!--f:conf_consistent-->875<!--/f--> | one positive piece of evidence, nothing against |
| `twin` | <!--f:conf_twin-->432<!--/f--> | flagged by the name check as interchangeable with a differently named function (`twin_interchangeable`); the code cannot decide |
| `stub` | <!--f:conf_stub-->68<!--/f--> | a constant return or empty body; the code cannot decide |
| `weak` | <!--f:conf_weak-->241<!--/f--> | no positive evidence |
| `mismatch` | <!--f:conf_mismatch-->1<!--/f--> | evidence against the name (`f_0006590c`) |

A name is the bank's assignment for its offset. As a test of the mapping as a whole, argument counts fit the bank signatures <!--f:arity_fit_pct-->97.2<!--/f-->% of the time against about <!--f:arity_shift_pct-->68<!--/f-->% with every name shifted by one function, and all <!--f:class_hier_match-->16<!--/f--> class hierarchies match the structure inferred from the code.

### Crate map

Workspace members of `Cargo.toml`: every folder under `crates/`, version 0.1.0, edition 2021, MIT, unpublished. External crates: none. A plain `cargo test` covers the default members: the four `neutral-*` crates and the 61 effect crates whose tests need no stored data.

| Member | Builds | Owns | Key modules (`src/`) | Tests (`#[test]` lines, scan) |
|---|---|---|---|---|
| `crates/neutral-world` | library | The Rust side of the world harness of the function record: the seeded model of everything outside the dump, the `Env` trait every lifted function is written against, world replay and comparison. Only the lift crates depend on it; no effect crate and no command does | `lib.rs` (`Env`, `World`, `check`, `Report`) | <!--f:tests_scan_neutral_world-->17<!--/f-->; `tests/synthetic.rs`, `tests/pin.rs` |
| `crates/neutral-parity` | library | Replay of oracle vectors against the ported functions of the function record. Only the tests of the lift crates depend on it; no effect crate and no command does | `lib.rs` (`parse`, `check`, `Report`) | <!--f:tests_scan_neutral_parity-->4<!--/f--> |
| `crates/neutral-ffmpeg` | library | ffmpeg and ffprobe as child processes: decoder, encoder, filter graph, probe, pixel format table | `decoder.rs`, `encoder.rs`, `filter.rs`, `probe.rs`, `pixfmt.rs` with `pixfmt_table.rs`, `locate.rs`, `process.rs` | <!--f:tests_scan_neutral_ffmpeg-->130<!--/f-->; `tests/*_live.rs` need ffmpeg |
| `crates/neutral-handoff` | library, binaries `sapphire` and `fake-vfx` | The pipeline (decode, stages, encode, verify), the stage provider, the frames/2 contract ([handoff](docs/handoff.md)) | `main.rs`, `plan.rs`, `pipeline.rs`, `stage.rs`, `ffstage.rs`, `geometry.rs`, `verify.rs`, `error.rs`, `bin/fake_vfx.rs` | <!--f:tests_scan_neutral_handoff-->273<!--/f-->; `tests/cross_twixtor.rs` runs against twixtor |
| `crates/lift-a`, `crates/lift-b`, `crates/lift-1` to `crates/lift-8` | library | The function record: the ported functions of its code section, a reverse-engineering reference that no effect crate and no command depends on. The <!--f:lift_crates-->10<!--/f--> lift crates hold <!--f:lift_defs_total-->5,248<!--/f--> `fn f_` definitions of the <!--f:fn_total-->2,702<!--/f--> ids, and <!--f:lift_ids_multi-->1,605<!--/f--> ids are defined more than once; the per-crate detail is in [lifting](docs/lifting.md#crate-layout) | `p1/`, `p2v/`, `missing.rs` (placeholders), `dup.rs`, `x86rt.rs` (`lift_rt.rs` in `lift-a`); `liftcheck/` in `lift-a` (feature `liftcheck`) | per crate in `docs/generated/crates.md`; `#[ignore]` without the stored vector sets |

The effect crates are under `crates/effects` (the evaluator `fx_eval`, the executor and `fx_run` in `fx_exec`, the kernel crates they call in `crates/effects/kernels`, and the test helpers `effects_vecdir` and `effects_rtdata`); they are workspace members, and the ones whose tests need no stored data are default members ([effects](docs/effects.md)).

Each `neutral-*` crate has a readme in its folder. Outside the workspace: `contracts/` (JSON schemas of the handoff convention and `contracts/conformance/cases.json`), `tools/` (Python 3.12 standard library: `handoff.py`, `conformance.py`, `gen_docs.py` with `docgen/`, `check_docs.py`, `fn_info.py`, `confirm_names.py` and their tests) and `docs/`.

## Interface

Each program's `--help` lists its flags; the contract in full is `docs/handoff.md`.

### Pipeline

`sapphire INPUT [EFFECT] [flags]` probes the input with ffprobe, decodes it with ffmpeg to the raw format of `--depth`, runs twixtor's `vfx apply EFFECT` and the `--stage` commands in order, encodes with ffmpeg, verifies the result against the input and only then moves it to the output path. The plan, the commands, the checks and the output naming are those of twixtor's `tools/apply.py`: on <!--f:pipeline_parity_equal-->43<!--/f--> of <!--f:pipeline_parity_rows-->43<!--/f--> rows of `docs/data/pipeline_parity.tsv` both give equal frames. `sapphire` defaults to `--contract frames/2`, `tools/apply.py` to frames/1 ([contract versions](docs/handoff.md#contract-versions)). Reports name each step by `role` and `index`. `--geometry declared` asks each stage for its output geometry with `--describe`; `--describe` on the pipeline prints the output geometry of the run and stops.

### Stage commands

`sapphire stage list --json` prints the stage manifest of this repository (`contracts/stage-manifest.schema.json`). `sapphire stage apply STAGE --width W --height H --depth 8|16|32 [--fps N[/D]] [--frames N] [--param K=V]... [--layer ID=PATH]... [--device cpu|gpu|gpu=N|auto] [--in FILE] [--out FILE]` reads whole frames on standard input and writes the same number of frames of the same size and format on standard output.

| Stage | Kind, geometry | Parameters |
|---|---|---|
| `selftest-invert` | frame, preserve | `invert_alpha` (checkbox, default off) |
| `ffmpeg-filter` | frame, declared | `graph` (an ffmpeg filter graph, default `null`), `expect` (`preserve` or `declared`), `unsafe` (`1` runs filters outside the safe list) ([the ffmpeg filter stage](docs/handoff.md#the-ffmpeg-filter-stage)) |
| 20 effect stages | temporal, preserve | per stage, see [Effects provider](docs/stages.md#effects-provider) |

In a pipeline, `--stage @selftest-invert` runs that stage in process and `--stage "@ffmpeg-filter graph=G"` or an effect stage such as `--stage @hue_sat_bright_autogen` runs `sapphire stage apply ...` as a child process of the pipeline (`SAPPHIRE_PATH` or the running executable). That child renders in process through the `fx_exec` library, so no `fx_run` process is started. The pipeline has no `--device` flag; the child inherits the environment, so `SAPPHIRE_DEVICE=gpu` runs a pipeline stage on a device ([guide](docs/guide.md#rendering-on-a-device)). Every flag and variable is in the [Command-line reference](docs/cli.md). The syntax of a filter graph is ffmpeg's own (ffmpeg documentation: ffmpeg-filters.html).

### Raw frame format

ffmpeg's own raw formats, so `ffmpeg -f rawvideo -pix_fmt <fmt>` reads and writes them directly ([frame contract](docs/handoff.md#frame-contract)).

| `--depth` | ffmpeg `-pix_fmt` | Bytes a pixel | Layout |
|---|---|---|---|
| 8 | `rgba` | 4 | packed R, G, B, A, one byte each |
| 16 | `rgba64le` | 8 | packed R, G, B, A, unsigned 16-bit little endian |
| 32 | `gbrapf32le` | 16 | planar: the whole G plane, then B, R and A; 32-bit IEEE float little endian |

Whole frames back to back, no header or padding, straight alpha, at most 4 GiB a frame; a clip that ends in part of a frame is an error.

### Exit codes

Under frames/2, for the pipeline and `tools/handoff.py` (`contracts/error.schema.json`); a stage command exits 2, 3 or 4 for `usage`, `unsupported` or `bad_input`:

| Code | Error code |
|---|---|
| 0 | done |
| 1 | `stage_failed` |
| 2 | `usage` |
| 3 | `unsupported` |
| 4 | `bad_input`, `broken_pipe`, `partial_frame` |
| 5 | `stage_timeout` |
| 6 | `tool_missing` |
| 130 | `interrupted` (`tools/handoff.py`; `sapphire` installs no Ctrl+C handler) |

A `--json` failure is the error object `{"ok": false, "contract", "error": {"code", "stage", "exit_code", "message", "stderr_tail"}}`. Under `--contract frames/1` the pipeline exits 0, 1 for any failure, or 2 for a command line the parser rejects ([errors and exit codes](docs/handoff.md#errors-and-exit-codes)).

### Python tools

| Tool | Does |
|---|---|
| `tools/fn_info.py ID\|NAME\|OFFSET` | one function's row from the map, the bank and the confirmation table, with callers, callees and stubs |
| `tools/gen_docs.py` | regenerates `docs/generated/`, `docs/index/` and every marker; `--check` reports drift without writing |
| `tools/check_docs.py` | regeneration drift, links, anchors, inline paths, ids, flag and exit code tables, ASCII, dashes, secrets, first person, claims, ffmpeg citations, archive citations, effect names, process terms (`tools/process_terms.py`) |

The handoff tools, `tools/handoff.py` and `tools/conformance.py`, are in [Handoff](#handoff).

## Handoff

sapphire runs twixtor's `vfx` and the stages of any tool inside its own pipeline and hands tasks to twixtor as packets, under the convention both repositories share ([handoff](docs/handoff.md)). It finds twixtor by twixtor's marker files: `--peer-dir twixtor=DIR`, else `HANDOFF_TWIXTOR_DIR`, else the one sibling folder that holds them.

| Command | Does |
|---|---|
| `python -I tools/handoff.py stages --json` | Lists the stages of sapphire and twixtor, each with the command line a wrapper takes. |
| `target/release/sapphire INPUT EFFECT --stage CMD` | Runs twixtor's `vfx` and a listed stage between an ffmpeg decode and encode; take CMD from `stages --json`. |
| `python -I tools/handoff.py new --to twixtor --task TEXT --input PATH` | Writes a task packet into twixtor's `handoff/` folder; twixtor runs `accept ID` and `done ID --output PATH --run SPEC`, and `verify ID --other` checks the result from here. |
| `python -I tools/handoff.py doctor` | Checks twixtor, ffmpeg, ffprobe, the `sapphire` binary, the contracts and the shared files. |
| `python -I tools/conformance.py --quick` | Runs the covering conformance subset in about a minute; `--full` runs the whole matrix and `--stage CMD` one stage. |
| `python -I tools/conformance.py --bench` | Times both wrappers on chained stages and rewrites `docs/handoff-bench.tsv`; run `sync` after it. |
| `python -I tools/handoff.py sync --check` | Compares the shared files with twixtor's by SHA-256; `sync` copies sapphire's versions over. |

The `shared` list of `contracts/tool.json` is byte equal in both repositories: the schemas in `contracts/`, `contracts/conformance/cases.json`, `docs/handoff.md`, `docs/handoff-bench.tsv`, `tools/handoff.py`, `tools/conformance.py` and their tests. Change one here, then run `sync`.

sapphire's own stages are `selftest-invert`, `ffmpeg-filter` and the 20 effect stages ([Stage commands](#stage-commands), [Effects provider](docs/stages.md#effects-provider)); a new tool joins as in [stages](docs/stages.md).

## Tests

From the repository root:

```sh
cargo test --release --offline -- --test-threads=2
python -I tools/fx_golden.py
python -I tools/test_handoff.py
python -I tools/test_conformance.py
python -I tools/test_docgen.py
python -I tools/gen_docs.py --check
python -I tools/check_docs.py
python -I tools/test_effect_rules.py
python -I tools/conformance.py --quick
python -I tools/handoff.py doctor --conformance
```

- **Default members.** `cargo test` runs `neutral-world`, `neutral-parity`, `neutral-ffmpeg`, `neutral-handoff` and the 61 effect crates whose tests need no stored data. The `lift-*` tests replay stored vector sets that are archived, not in the repository, and are marked `#[ignore]`; running them with the archive: [reproduce](docs/reproduce.md#re-creating-the-evidence).
- **Effects.** The effect tests are described in [effects](docs/effects.md#tests). `tools/fx_golden.py` renders the 25 stages on synthetic clips and compares the SHA-256 of every output with `tools/fx_golden.tsv`; with `SAPPHIRE_DEVICE=gpu SAPPHIRE_REQUIRE_DEVICE=1` (PoCL or a GPU) it checks the device path against the same hashes.
- **ffmpeg.** The live tests of `neutral-ffmpeg` and the pipeline tests need ffmpeg and ffprobe and print a `SKIPPED` line without them ([tests that need ffmpeg](docs/ffmpeg.md#tests-that-need-ffmpeg)). One `neutral-ffmpeg` test always prints `SKIPPED`, because the pixel format table generator is not in the repository.
- **twixtor.** `crates/neutral-handoff/tests/cross_twixtor.rs` runs twixtor's `vfx` as a stage of this pipeline and this crate's `stage apply` inside twixtor's `tools/apply.py`, in every order; it skips with a printed reason when ffmpeg, `vfx`, Python or `apply.py` is missing.
- **Conformance.** `tools/conformance.py --quick` is the covering subset of about a minute; the other modes are in [Handoff](#handoff).

**Continuous integration.** `.github/workflows/ci.yml` (Linux, macOS and Windows, Python 3.12) and `.forgejo/workflows/ci.yml` (Linux in the `node:24-bookworm` container, with Rust from rustup) install ffmpeg (a static 9.0 build on Linux), run `cargo test --release`, build `sapphire` and `fx_run`, run `tools/fx_golden.py` (on all three systems), compile `tools/`, then run `test_handoff.py`, `test_conformance.py`, `conformance.py --quick --allow-absent-peer`, `test_docgen.py`, `gen_docs.py --check`, `check_docs.py` and `test_effect_rules.py`, on every push to `main` and every pull request, with a 60 minute limit. On Linux, both workflows install PoCL and run `fx_golden.py` through the device backend; the GitHub workflow also runs the device tests and the tests of the effect crates outside the default members. Both can also be started by hand. A new push to a pull request cancels its run in flight; each push to `main` keeps a run of its own. twixtor is not checked out in CI, so every part that needs it prints a `NOT RUN` line, and `--allow-absent-peer` lets only those parts through.

### Smoke test

Confirms a build end to end. Run from the repository root after [Build](#build), with ffmpeg on `PATH`; scratch files go to the git ignored `target/demo/`.

```sh
mkdir -p target/demo
ffmpeg -v error -f lavfi -i testsrc2=size=160x96:rate=24 -t 2 -c:v libx264 -crf 10 -pix_fmt yuv420p target/demo/clip.mp4
target/release/sapphire stage list --json
ffmpeg -v error -i target/demo/clip.mp4 -f rawvideo -pix_fmt rgba - > target/demo/clip.rgba
target/release/sapphire stage apply selftest-invert --width 160 --height 96 --depth 8 < target/demo/clip.rgba | target/release/sapphire stage apply selftest-invert --width 160 --height 96 --depth 8 > target/demo/twice.rgba
cmp target/demo/clip.rgba target/demo/twice.rgba
target/release/sapphire target/demo/clip.mp4 --stage @selftest-invert --depth 8 --dry-run
python -I tools/fn_info.py f_0006590c
python -I tools/handoff.py doctor
```

| Step | Expected |
|---|---|
| `stage list --json` | a manifest with `"contract": "frames/2"`, provider id `sapphire` and the stages `selftest-invert`, `ffmpeg-filter` and the 20 effect stages |
| raw pipe and `cmp` | exit 0 and no output from `cmp`: inverting twice returns the input |
| `--dry-run` | `[plan] stages on clip.mp4: 160x96, 24.000 fps, 8 bpc (rgba), decode \| pre1 \| encode -> ...`, three `[cmd]` lines, then `[dry-run] nothing was written` |
| `fn_info.py` | the bank name first with the id beside it, then offset, kind, origin, tier, name check `mismatch` and confirmation `mismatch` |
| `doctor` | with twixtor checked out beside this repository, one `[ok]` line per check and `doctor: all required checks passed` |

## Limits

| Not covered | What it means |
|---|---|
| The <!--f:close_labelled_rows-->1<!--/f--> function on a stated assumption (`f_0001b438`) | strict only under A1 ([assumptions](docs/assumptions.md#summary)); a world that breaks the assumption was not compared |
| Inputs outside the seeded worlds | parity is measured on seeded inputs, not proved for every input ([caveats](docs/parity.md#caveats)) |
| Callees outside the dump | modelled as writing nothing and returning a seeded word |
| Stub argument values | <!--f:stub_arity_sites-->48<!--/f--> call sites passed fewer arguments than the bank signatures; those values were never compared |
| Missing references | data, imports and constants not in the dump; <!--f:missing_refs-->19,563<!--/f--> references are named placeholders |
| Archived evidence | the dumps, the full disassembly listing and the stored vector sets are archived, and without them the `lift-*` tests stay ignored ([reproduce](docs/reproduce.md#what-needs-the-archive)) |
| Effect behaviour | only the nineteen effects of [docs/effects.md](docs/effects.md) are rebuilt and rendered, as native code outside the function record; no effect name is mapped to a function, and the other effect names are strings only |
| Name certainty | a name is the bank's assignment for its offset; the `twin` and `stub` levels cannot be decided by the code |
| ffmpeg versions | 9.0.2 is the measured version; other versions run conformance live and are reported as unverified |
| POSIX | the process group branches have not run yet; the first Linux or macOS CI run covers them ([known gaps](docs/handoff.md#known-gaps)) |
| Interrupt | `sapphire` installs no Ctrl+C handler and never exits 130 |
| Windows launchers | a launcher in front of ffmpeg, such as the Chocolatey shim, is ended on a timeout but the ffmpeg it started is not; point `FFMPEG` at the real `ffmpeg.exe` |
| Pixel format table | its generator is not in the repository, so the table cannot be regenerated here |

## Legal

- **Independent work.** This project is written independently and contains no binaries of the software it reimplements.
- **No affiliation.** It is not affiliated with, endorsed by or sponsored by any other company, product or library.
- **Trademarks.** Product and library names belong to their owners and appear here only to identify them.
- **Interoperability.** Function, class, effect and parameter names follow the build it replaces, so that the code can be matched with it.
- **No warranty.** Everything here is provided as is under the MIT License, without warranty of any kind and without liability.
- **Your responsibility.** You need the rights to the media and the software you process, and you stay bound by the licences of any software you use alongside this project, ffmpeg included.
- **Rights holders.** If you believe something here infringes your rights, email [hi@bairon.dev](mailto:hi@bairon.dev) or [open an issue](https://github.com/baairon/sapphire/issues). Every concern gets a prompt answer in good faith.

## License

MIT, see [LICENSE](LICENSE).
