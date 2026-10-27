# sapphire

> [!IMPORTANT]
> The effects do not run yet. This repository holds the rewritten functions, their parity record and the frame handoff pipeline; no effect renders. Running the effects is the next phase.

An independent open source reimplementation of Sapphire behaviour, function by function. It is not affiliated with or endorsed by any other company, product or library.

The plugin's functions, written in Rust and paired with ffmpeg and twixtor. `tools/fn_info.py` looks up any function, `sapphire` runs an ffmpeg decode, twixtor's `vfx`, the stages of any tool and an ffmpeg encode as one pipe, `sapphire stage apply` runs this repository's stages on raw frames, and `tools/handoff.py` and `tools/conformance.py` hand tasks to twixtor and test a stage against the convention.

Sources of truth, for agents and scripts:

| Question | Ask |
|---|---|
| One function: bank name, offset, size, origin, tier, parity, evidence, name check, confirmation, crate file, callers, callees | `python -I tools/fn_info.py ID\|NAME\|OFFSET [--json]`: the function by id, bank name or offset. |
| Every function at once | `docs/index/functions.jsonl`: one JSON object a function, every field. |
| Any figure quoted in a document | `docs/index/facts.json`: the value, text and source of every figure. |
| The stages this repository provides | `target/release/sapphire stage list --json`: the stage manifest. |
| Every stage of sapphire and twixtor, with a ready stage command | `python -I tools/handoff.py stages --json`: each stage with the command line a wrapper takes. |
| What a pipeline run will do | `target/release/sapphire INPUT [EFFECT] ... --dry-run`: the plan and every command, nothing written. |

## Contents

- [Parity](#parity)
- [Names](#names)
- [Crate map](#crate-map)
- [Build](#build)
- [Interface](#interface)
  - [Pipeline](#pipeline)
  - [Stage commands](#stage-commands)
  - [Raw frame format](#raw-frame-format)
  - [Exit codes](#exit-codes)
  - [Python tools](#python-tools)
- [Handoff](#handoff)
- [Smoke test](#smoke-test)
- [Tests](#tests)
- [Limits](#limits)
- [Documentation](#documentation)
- [Legal](#legal)
- [License](#license)

## Parity

1:1 parity with the build it replaces is the first priority of this project, ahead of speed, convenience and a cleaner design. When they conflict, parity wins.

- **Output.** A port gives what the function it replaces gives on every compared input: bit for bit on oracle vectors (tier P1), and the same returns, memory writes and calls in every compared world (tier P2v) ([tiers](docs/parity.md#tiers)).
- **Interface.** Each port keeps its function's offset in its symbol, `f_XXXXXXXX`, and the name bank gives that function's exact name.
- **No near misses.** A port that differs on any compared input counts in no parity figure ([unproven](docs/parity.md#unproven)).
- **Measured, not assumed.** Each function and its port run on the same generated inputs and every result is compared ([harness](docs/harness.md)); what the figures rest on is in [assumptions](docs/assumptions.md).

| Standard | Functions | Share of code bytes |
|---|---|---|
| Strict parity: matched on every compared input | <!--f:strict_fns-->2,695<!--/f--> of <!--f:fn_total-->2,702<!--/f--> | <!--f:strict_pct-->97.4<!--/f-->% |
| Strictest: strict, with every branch outcome counted over compared worlds only | <!--f:strictest_fns-->2,692<!--/f--> | <!--f:strictest_pct-->96.9<!--/f-->% |
| Unproven: never matched | <!--f:unproven_fns-->7<!--/f--> | <!--f:unproven_pct-->2.6<!--/f-->% |

Shares are of the <!--f:code_bytes-->964,659<!--/f--> code bytes of the <!--f:fn_total-->2,702<!--/f--> functions, most of them bundled library code.

## Names

The name bank is the source of truth for every name; the code keeps its `f_XXXXXXXX` symbols ([names](docs/names.md)).

| Table | Rows | Holds |
|---|---|---|
| `docs/data/names.tsv` | <!--f:names_total-->2,702<!--/f--> | per function: exact C++ name, Rust label, kind, origin, mangled name |
| `docs/data/stub_names.tsv` | <!--f:stubs_total-->458<!--/f--> | named import stubs |
| `docs/data/classes_names.tsv` | <!--f:classes_total-->16<!--/f--> | local classes with their base classes |
| `docs/data/effect_names.tsv` | <!--f:effects_total-->272<!--/f--> | effect names |
| `docs/data/param_candidates.tsv` | <!--f:params_total-->1,040<!--/f--> | parameter candidate strings, bound to no function |

**Effect names.** The <!--f:effects_total-->272<!--/f--> effect names are strings only: none is mapped to functions, classes, parameters or behaviour, and this repository renders no sapphire effect. They are listed verbatim in [docs/generated/effects.md](docs/generated/effects.md), the parameter candidates in `docs/generated/parameters.md`; a candidate is not a confirmed parameter.

**Origin.** The bank classes <!--f:origin_bundled-->1,983<!--/f--> functions as bundled library code (the class `CNTV2Card` alone has <!--f:cntv2card_fns-->906<!--/f-->), <!--f:origin_std-->514<!--/f--> as std, <!--f:origin_undetermined-->136<!--/f--> as undetermined and <!--f:origin_component-->69<!--/f--> as component.

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

## Crate map

Workspace members of `Cargo.toml`: every folder under `crates/`, version 0.1.0, edition 2021, MIT, unpublished. External crates: none. A plain `cargo test` covers the default members, the four `neutral-*` crates.

| Member | Builds | Owns | Key modules (`src/`) | Tests (`#[test]` lines, scan) |
|---|---|---|---|---|
| `crates/neutral-world` | library | The Rust side of the world harness: the seeded model of everything outside the dump, the `Env` trait every lifted function is written against, world replay and comparison | `lib.rs` (`Env`, `World`, `check`, `Report`) | <!--f:tests_scan_neutral_world-->12<!--/f-->; `tests/synthetic.rs` |
| `crates/neutral-parity` | library | Replay of oracle vectors against ported functions | `lib.rs` (`parse`, `check`, `Report`) | <!--f:tests_scan_neutral_parity-->4<!--/f--> |
| `crates/neutral-ffmpeg` | library | ffmpeg and ffprobe as child processes: decoder, encoder, filter graph, probe, pixel format table | `decoder.rs`, `encoder.rs`, `filter.rs`, `probe.rs`, `pixfmt.rs` with `pixfmt_table.rs`, `locate.rs`, `process.rs` | <!--f:tests_scan_neutral_ffmpeg-->130<!--/f-->; `tests/*_live.rs` need ffmpeg |
| `crates/neutral-handoff` | library, binaries `sapphire` and `fake-vfx` | The pipeline (decode, stages, encode, verify), the stage provider, the frames/2 contract ([handoff](docs/handoff.md)) | `main.rs`, `plan.rs`, `pipeline.rs`, `stage.rs`, `ffstage.rs`, `geometry.rs`, `verify.rs`, `error.rs`, `bin/fake_vfx.rs` | <!--f:tests_scan_neutral_handoff-->199<!--/f-->; `tests/cross_twixtor.rs` runs against twixtor |
| `crates/lift-a`, `crates/lift-b`, `crates/lift-1` to `crates/lift-8` | library | The ported functions. The <!--f:lift_crates-->10<!--/f--> lift crates hold <!--f:lift_defs_total-->5,248<!--/f--> `fn f_` definitions of the <!--f:fn_total-->2,702<!--/f--> ids, and <!--f:lift_ids_multi-->1,605<!--/f--> ids are defined more than once; the per-crate detail is in [lifting](docs/lifting.md#crate-layout) | `p1/`, `p2v/`, `missing.rs` (placeholders), `dup.rs`, `x86rt.rs` (`lift_rt.rs` in `lift-a`); `liftcheck/` in `lift-a` (feature `liftcheck`) | per crate in `docs/generated/crates.md`; `#[ignore]` without the stored vector sets |

Each `neutral-*` crate has a readme in its folder. Outside the workspace: `contracts/` (JSON schemas of the handoff convention and `contracts/conformance/cases.json`), `tools/` (Python 3.12 standard library: `handoff.py`, `conformance.py`, `gen_docs.py` with `docgen/`, `check_docs.py`, `fn_info.py`, `confirm_names.py` and their tests) and `docs/`.

## Build

From the repository root, with a stable Rust toolchain, offline:

```sh
cargo build --release --offline -p neutral-handoff
```

On a Windows machine without the MSVC linker the GNU toolchain builds the same code: `cargo +stable-x86_64-pc-windows-gnu build --release --offline -p neutral-handoff`.

| Binary | Path (`.exe` added on Windows) | Used by |
|---|---|---|
| `sapphire` | `target/release/sapphire` | the pipeline, the stage provider of `contracts/tool.json`, `tools/handoff.py doctor`, `tools/conformance.py` |
| `fake-vfx` | `target/release/fake-vfx` | the pipeline tests, as a stand in for twixtor's `vfx` |

`CARGO_TARGET_DIR` moves both; `SAPPHIRE_PATH` points the Python tools at another build. The pipeline and the `neutral-ffmpeg` tests need ffmpeg and ffprobe ([finding ffmpeg](docs/ffmpeg.md#finding-ffmpeg)).

## Interface

Each program's `--help` lists its flags; the contract in full is `docs/handoff.md`.

### Pipeline

`sapphire INPUT [EFFECT] [flags]` probes the input with ffprobe, decodes it with ffmpeg to the raw format of `--depth`, runs twixtor's `vfx apply EFFECT` and the `--stage` commands in order, encodes with ffmpeg, verifies the result against the input and only then moves it to the output path. The plan, the commands, the checks and the output naming are those of twixtor's `tools/apply.py`: on <!--f:pipeline_parity_equal-->43<!--/f--> of <!--f:pipeline_parity_rows-->43<!--/f--> rows of `docs/data/pipeline_parity.tsv` both give equal frames. `sapphire` defaults to `--contract frames/2`, `tools/apply.py` to frames/1 ([contract versions](docs/handoff.md#contract-versions)). Reports name each step by `role` and `index`. `--geometry declared` asks each stage for its output geometry with `--describe`; `--describe` on the pipeline prints the output geometry of the run and stops.

### Stage commands

`sapphire stage list --json` prints the stage manifest of this repository (`contracts/stage-manifest.schema.json`). `sapphire stage apply STAGE --width W --height H --depth 8|16|32 [--fps N[/D]] [--frames N] [--param K=V]...` reads whole frames on standard input and writes the same number of frames of the same size and format on standard output.

| Stage | Kind, geometry | Parameters |
|---|---|---|
| `selftest-invert` | frame, preserve | `invert_alpha` (checkbox, default off) |
| `ffmpeg-filter` | frame, declared | `graph` (an ffmpeg filter graph, default `null`), `expect` (`preserve` or `declared`), `unsafe` (`1` runs filters outside the safe list) ([the ffmpeg filter stage](docs/handoff.md#the-ffmpeg-filter-stage)) |

In a pipeline, `--stage @selftest-invert` runs that stage in process and `--stage "@ffmpeg-filter graph=G"` runs `sapphire stage apply ffmpeg-filter` as a child process. The syntax of a filter graph is ffmpeg's own (ffmpeg documentation: ffmpeg-filters.html).

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
| `tools/check_docs.py` | regeneration drift, links, anchors, inline paths, ids, flag and exit code tables, ASCII, dashes, secrets, first person, claims, ffmpeg citations, archive citations, effect names |

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

sapphire's own stages are `selftest-invert` and `ffmpeg-filter` ([Stage commands](#stage-commands)); a new tool joins as in [stages](docs/stages.md).

## Smoke test

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
| `stage list --json` | a manifest with `"contract": "frames/2"`, provider id `sapphire` and the stages `selftest-invert` and `ffmpeg-filter` |
| raw pipe and `cmp` | exit 0 and no output from `cmp`: inverting twice returns the input |
| `--dry-run` | `[plan] stages on clip.mp4: 160x96, 24.000 fps, 8 bpc (rgba), decode \| pre1 \| encode -> ...`, three `[cmd]` lines, then `[dry-run] nothing was written` |
| `fn_info.py` | the bank name first with the id beside it, then offset, kind, origin, tier, name check `mismatch` and confirmation `mismatch` |
| `doctor` | with twixtor checked out beside this repository, one `[ok]` line per check and `doctor: all required checks passed` |

## Tests

From the repository root:

```sh
cargo test --release --offline -- --test-threads=2
python -I tools/test_handoff.py
python -I tools/test_conformance.py
python -I tools/test_docgen.py
python -I tools/gen_docs.py --check
python -I tools/check_docs.py
python -I tools/conformance.py --quick
python -I tools/handoff.py doctor --conformance
```

- **Default members.** `cargo test` runs `neutral-world`, `neutral-parity`, `neutral-ffmpeg` and `neutral-handoff`. The `lift-*` tests replay stored vector sets that are archived, not in the repository, and are marked `#[ignore]`; running them with the archive: [reproduce](docs/reproduce.md#re-creating-the-evidence).
- **ffmpeg.** The live tests of `neutral-ffmpeg` and the pipeline tests need ffmpeg and ffprobe and print a `SKIPPED` line without them ([tests that need ffmpeg](docs/ffmpeg.md#tests-that-need-ffmpeg)). One `neutral-ffmpeg` test always prints `SKIPPED`, because the pixel format table generator is not in the repository.
- **twixtor.** `crates/neutral-handoff/tests/cross_twixtor.rs` runs twixtor's `vfx` as a stage of this pipeline and this crate's `stage apply` inside twixtor's `tools/apply.py`, in every order; it skips with a printed reason when ffmpeg, `vfx`, Python or `apply.py` is missing.
- **Conformance.** `tools/conformance.py --quick` is the covering subset of about a minute; the other modes are in [Handoff](#handoff).

**Continuous integration.** `.github/workflows/ci.yml` (Linux, macOS and Windows, Python 3.12) and `.forgejo/workflows/ci.yml` (Linux in the `node:24-bookworm` container, with Rust from rustup) install ffmpeg (a static 9.0 build on Linux), run `cargo test --release`, build `sapphire`, compile `tools/`, then run `test_handoff.py`, `test_conformance.py`, `conformance.py --quick --allow-absent-peer`, `test_docgen.py`, `gen_docs.py --check` and `check_docs.py`, on every push to `main` and every pull request, with a 60 minute limit. Both can also be started by hand. A new push to a pull request cancels its run in flight; each push to `main` keeps a run of its own. twixtor is not checked out in CI, so every part that needs it prints a `NOT RUN` line, and `--allow-absent-peer` lets only those parts through.

## Limits

| Not covered | What it means |
|---|---|
| The <!--f:unproven_fns-->7<!--/f--> unproven functions | never matched; their ports are in the crates but carry no parity claim |
| Inputs outside the seeded worlds | parity is measured on seeded inputs, not proved for every input ([caveats](docs/parity.md#caveats)) |
| Callees outside the dump | modelled as writing nothing and returning a seeded word |
| Stub argument values | <!--f:stub_arity_sites-->48<!--/f--> call sites passed fewer arguments than the bank signatures; those values were never compared |
| Missing references | data, imports and constants not in the dump; <!--f:missing_refs-->19,563<!--/f--> references are named placeholders |
| Archived evidence | the dumps, the full disassembly listing and the stored vector sets are archived, and without them the `lift-*` tests stay ignored ([reproduce](docs/reproduce.md#what-needs-the-archive)) |
| Effect behaviour | no sapphire effect is mapped to functions or rendered; effect names are strings only |
| Name certainty | a name is the bank's assignment for its offset; the `twin` and `stub` levels cannot be decided by the code |
| ffmpeg versions | 9.0.2 is the measured version; other versions run conformance live and are reported as unverified |
| POSIX | the process group branches have not run yet; the first Linux or macOS CI run covers them ([known gaps](docs/handoff.md#known-gaps)) |
| Interrupt | `sapphire` installs no Ctrl+C handler and never exits 130 |
| Windows launchers | a launcher in front of ffmpeg, such as the Chocolatey shim, is ended on a timeout but the ffmpeg it started is not; point `FFMPEG` at the real `ffmpeg.exe` |
| Pixel format table | its generator is not in the repository, so the table cannot be regenerated here |

## Documentation

| File | Specifies |
|---|---|
| [`docs/parity.md`](docs/parity.md) | What parity means: the tiers, strict parity and strictest, the unproven functions, the caveats, the parity terms. |
| [`docs/harness.md`](docs/harness.md) | The oracle, worlds, the `Env` trait, vector sets, comparison, replay, mutation checks, the harness terms. |
| [`docs/lifting.md`](docs/lifting.md) | Mechanical lift and hand port, what the lifter writes, the crate layout with per-crate counts, the x86 rules the ports reproduce. |
| [`docs/evidence.md`](docs/evidence.md) | The evidence classes, the audit, how to read an audit detail, the record behind each function's tier and status. |
| [`docs/assumptions.md`](docs/assumptions.md) | Every assumption the parity figures rest on: callee model, pinned words, no-return declarations, landing pads, hand proofs, stub argument counts. |
| [`docs/missing-data.md`](docs/missing-data.md) | Missing references: what the code needs that the dump does not contain, and their placeholders. |
| [`docs/reproduce.md`](docs/reproduce.md) | What reruns from this repository alone and what needs the archive. |
| [`docs/names.md`](docs/names.md) | The name bank, origins, the name check, confirmation levels and the naming rules. |
| [`docs/ffmpeg.md`](docs/ffmpeg.md) | How sapphire runs ffmpeg: lookup, the crate, raw frames, colour, rate and length, tests. |
| [`docs/handoff.md`](docs/handoff.md) | How sapphire, twixtor and ffmpeg pass raw frames to each other: contract versions, the frame contract, errors and exits, reports, discovery, geometry, the ffmpeg filter stage, packets, commands, platforms, known gaps. |
| [`docs/stages.md`](docs/stages.md) | How a new provider offers stages and passes conformance. |
| [`docs/generated/index.md`](docs/generated/index.md) | The generated reference: functions by offset range, classes, stubs, effects, parameters, tiers, evidence, crates, missing data. |
| [`docs/handoff-bench.tsv`](docs/handoff-bench.tsv) | The bench table of `tools/conformance.py --bench`: the wrappers timed on chained stages. |

`docs/index/` holds the machine readable index: `facts.json` (every figure with its source), `functions.jsonl` (one row a function) and `pages.json` (every page with its title and size). `docs/data/` holds the checked in evidence, among it the function map `docs/data/FUNCTION_MAP.tsv`; `docs/generated/`, `docs/index/` and the figures in every page are written from it by `python -I tools/gen_docs.py`, so change the data, then regenerate.

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
