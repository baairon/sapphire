# sapphire

The pipeline from ffmpeg through twixtor's `vfx` and back to ffmpeg, plus the stage commands of this repository. Standard library only, no unsafe code. The plan, the ffmpeg and `vfx` command lines, the checks and the output naming are those of twixtor's `tools/apply.py`, so the same input, effect and flags give the same output frames: equal on <!--f:pipeline_parity_equal-->43<!--/f--> of <!--f:pipeline_parity_rows-->43<!--/f--> rows of `docs/data/pipeline_parity.tsv`. The convention it implements is [handoff](../../docs/handoff.md).

```text
input file --ffprobe--> plan (size, rate, colour matrix, stream)
   |
   ffmpeg decode   (neutral_ffmpeg::Decoder, raw frames of the --depth format)
   |
   stages, in order: vfx apply EFFECT, --stage commands, stages of this crate (@NAME)
   |
   ffmpeg encode   (neutral_ffmpeg::Encoder, same format in, file out)
   |
output file, verified against the input, then moved into place
```

## Commands

| Command | Does |
|---|---|
| `sapphire INPUT [EFFECT] [flags]` | the pipeline: decode, `vfx apply EFFECT` and the `--stage` commands, encode, verify |
| `sapphire stage list [--json]` | the stage manifest of this repository (`contracts/stage-manifest.schema.json`) |
| `sapphire stage apply STAGE --width W --height H --depth 8\|16\|32 [--fps N[/D]] [--frames N] [--param K=V]...` | one stage over raw frames on standard input and output; `--describe` prints the output geometry without reading input |
| `sapphire --version` | `sapphire 0.1.0` |

`sapphire --help` and `sapphire stage --help` list every flag. The binary is built with `cargo build --release --offline -p neutral-handoff` and is the native provider of `contracts/tool.json` (`SAPPHIRE_PATH` overrides its location).

## Contract

| Topic | Behaviour |
|---|---|
| Version | frames/2 by default; `--contract frames/1` speaks the exit codes and report keys of frames/1 ([contract versions](../../docs/handoff.md#contract-versions)) |
| Exits under frames/2 | 0 done, 1 `stage_failed`, 2 `usage`, 3 `unsupported`, 4 `bad_input`, `broken_pipe` or `partial_frame`, 5 `stage_timeout`, 6 `tool_missing`; a `--json` failure is the error object of `contracts/error.schema.json` ([errors and exit codes](../../docs/handoff.md#errors-and-exit-codes)) |
| Exits under frames/1 | 0 done, 1 any failure, 2 a command line the parser rejects |
| Stage exits | a `--stage` command that exits 2, 3 or 4 is `usage`, `unsupported` or `bad_input`, as `vfx` is |
| Reports | `--json` steps carry `role` (decode, pre, effect, post, encode, verify) and `index`; `name` is a label, `pre1` and `post1` counted from the `vfx` marker ([reports](../../docs/handoff.md#reports)) |
| Stage order | `vfx` among the `--stage` commands marks where the effect runs; without it the effect runs first |
| Watchdog | `--stage-timeout S` (default 120, 0 disables) kills a decode, stage command or encode that moved no data for S seconds; `vfx` is never judged |
| Geometry | `--geometry declared` after a `--stage` asks that stage `--describe`; `--describe` on the wrapper prints the geometry of the run and stops ([geometry](../../docs/handoff.md#geometry)) |

## Stages

`stage list --json` prints a manifest with `"contract": "frames/2"`, provider id `sapphire` and two stages:

| Stage | Kind, geometry | Parameters | Module |
|---|---|---|---|
| `selftest-invert` | frame, preserve | `invert_alpha` (checkbox, default off) | `src/stage.rs`: the conformance stage; inverting twice returns the input |
| `ffmpeg-filter` | frame, declared | `graph` (one input, one output, default `null`), `expect` (`preserve` or `declared`), `unsafe` (`1` runs filters outside the safe list) | `src/ffstage.rs` on `neutral_ffmpeg::Filter` |

| `ffmpeg-filter` rule | Detail |
|---|---|
| `expect=preserve` | bytes out are the same count as bytes in, frame for frame; a graph that changes the size, rate or frame count is `unsupported` (exit 3) |
| `expect=declared` | `--describe` measures the output geometry with a bounded dry run (at most 2 seconds and 60 frames) |
| Safe filters | without `unsafe=1` a graph may only use the frame filters of `SAFE_FILTERS` in `src/ffstage.rs`; any other filter is refused ([the ffmpeg filter stage](../../docs/handoff.md#the-ffmpeg-filter-stage)) |
| ffmpeg | `--ffmpeg-path`, else the lookup of `neutral_ffmpeg::Binaries` |

In the pipeline, `--stage @selftest-invert` runs that stage in process; `--stage "@ffmpeg-filter graph=G"` runs `sapphire stage apply ffmpeg-filter` as a child process (`SAPPHIRE_PATH` or the running executable) with the pipeline's ffmpeg. How another provider offers stages: [stages](../../docs/stages.md).

## Modules

| Module (`src/`) | Owns |
|---|---|
| `main.rs` | the binary: the pipeline wrapper and the `stage` subcommand |
| `error.rs` | `Error`, its codes and exit codes |
| `format.rs` | `Depth` and `FrameSpec`, the raw frame format |
| `rate.rs` | `Rate`, an exact frame rate |
| `media.rs` | `Info`, what `ffprobe` says a file is |
| `tools.rs` | finding ffmpeg, ffprobe, `vfx` and stage programs ([finding ffmpeg](../../docs/ffmpeg.md#finding-ffmpeg)) |
| `vfx.rs` | `vfx list`, `vfx apply --check` and the contract version check |
| `plan.rs` | `Request`, `prepare`, the commands and the `--dry-run` text |
| `stage.rs` | the stage registry, `selftest-invert`, the manifest and the frame loop of `stage apply` |
| `ffstage.rs` | the `ffmpeg-filter` stage |
| `geometry.rs` | declared geometry, `--describe` answers and their chain |
| `pipeline.rs` | threads, queues, the watchdog and the root cause of a failure |
| `verify.rs` | the checks of a finished output |
| `run.rs` | scratch folder, layer clips, the run itself, the output text and JSON |
| `cli.rs`, `args.rs` | the command line |
| `paths.rs`, `pytext.rs`, `pyjson.rs`, `sys.rs` | output naming and the Python compatible text, JSON and process helpers |
| `bin/fake_vfx.rs` | `fake-vfx`, a stand in for twixtor's `vfx` in the tests |

## Tests

`cargo test --release --offline -p neutral-handoff -- --test-threads=2` from the repository root; <!--f:tests_scan_neutral_handoff-->199<!--/f--> `#[test]` lines by text scan ([crates](../../docs/generated/crates.md)).

| File | Checks | Needs |
|---|---|---|
| unit tests in `src/` | plan, rates, formats, errors, arguments, paths, geometry, stages | see each module |
| `tests/stage_cli.rs` | `stage list --json` and `stage apply selftest-invert` as a process | nothing |
| `tests/pipeline_fake.rs` | stage order, the in-process stage, the error object for every way a stage can misbehave | ffmpeg |
| `tests/frames2.rs` | the manifest, `--describe`, `ffmpeg-filter`, geometry through the wrapper, `role` and `index`, the exits | ffmpeg |
| `tests/parity_apply_py.rs` | the same plan, command lines, refusals and frames as twixtor's `tools/apply.py` | ffmpeg, Python, twixtor |
| `tests/cross_twixtor.rs` | each tool as a stage of the other, inverting twice in every order | ffmpeg, Python, twixtor's `vfx` and `tools/apply.py` |

A test whose tools are missing prints a `SKIPPED` line with its reason and returns; under `HANDOFF_REQUIRE_PEERS=1` a missing twixtor fails the test instead. The cross repository tests find twixtor's `vfx` through the lookup of `src/tools.rs` (`VFX_PATH`, the build folders, `HANDOFF_TWIXTOR_DIR` or a sibling checkout found by its markers, `PATH`), and its `tools/apply.py` through `HANDOFF_TWIXTOR_DIR`, `APPLY_PY` or that sibling checkout.

## Limits

| Limit | Effect |
|---|---|
| No Ctrl+C handler (standard library only, no unsafe code) | the binary never exits 130 |
| POSIX process groups (`src/sys.rs`) | the POSIX branches have not run yet; the first Linux or macOS CI run covers them ([platforms](../../docs/handoff.md#platforms)) |
