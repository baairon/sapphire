# Adding a provider

A provider is a program that offers stages over raw frames; twixtor's `vfx` and `sapphire` are two. The contract is [handoff](handoff.md). In the examples `<provider>`, `<program>` and `<effect>` are placeholders; every other value is real and uses twixtor's `retime` effect and the `selftest-invert` stage of both repositories.

## Steps

| Step | Done when |
|---|---|
| 1. Implement `<program> list --json`, or ship a `<provider>.stage.json` manifest | the manifest validates against `contracts/stage-manifest.schema.json` ([The manifest](#the-manifest)) |
| 2. Implement `<program> apply <effect> ...` | it reads and writes raw frames as below ([The apply command](#the-apply-command)) |
| 3. Vendor `contracts/` and `tools/conformance.py` byte for byte | the files equal those of sapphire and twixtor ([Vendoring](#vendoring)) |
| 4. Pass `python -I tools/conformance.py --stage "<program> apply <effect>"` | it ends `result: pass (exit 0)` ([Conformance](#conformance)) |
| 5. Make it findable | `python -I tools/handoff.py stages --json` lists the stage with no `problems` ([Discovery](#discovery)) |

## The manifest

A manifest is one JSON object with these required keys:

| Key | Holds |
|---|---|
| `contract` | `frames/2` |
| `provider.id` | lower case id of the provider |
| `provider.version` | the provider's version text |
| `provider.command` | the argv prefix that `list --json` and `apply STAGE ...` are appended to |
| `stages[].id` | the stage id that follows `apply` |
| `stages[].kind` | `frame` (one frame out per frame in, streamed) or `temporal` (reads any frame of the clip) |
| `stages[].spool` | whether piped input is first copied whole to a file |
| `stages[].geometry` | `preserve` or `declared` ([geometry](handoff.md#geometry)) |
| `stages[].depths` | the depths it accepts, of 8, 16 and 32 |
| `stages[].params` | its parameters (id, name, kind, default and the kind's fields) |
| `stages[].layers` | the extra input layers it reads, or an empty list |

| Rule | Detail |
|---|---|
| Extra keys | allowed; `sapphire` adds `tool`, `pix_fmts`, and `name`, `family` and `hidden` per stage |
| No `list --json` | ship the same object as a `*.stage.json` file; a relative program path in its `command` is relative to the file's folder |
| Bare array | twixtor's `vfx list --json` prints an array without `contract`, which `tools/handoff.py` wraps through the `vfx-list` adapter; a new provider prints a manifest |

A minimal manifest:

```json
{
  "contract": "frames/2",
  "provider": {"id": "<provider>", "version": "<version>", "command": ["<program>"]},
  "stages": [
    {"id": "<effect>", "kind": "frame", "spool": false, "geometry": "preserve",
     "depths": [8, 16, 32], "params": [], "layers": []}
  ]
}
```

`sapphire stage list --json` prints a real manifest, with the stage `selftest-invert`.

## The apply command

The command line has the shape of twixtor's `vfx apply`:

```
<program> apply <effect> --width W --height H --depth 8|16|32 --fps N/D [--frames N] [--param K=V]... [--layer ID=PATH]...
```

| Rule | Detail |
|---|---|
| Input and output | whole raw frames on standard input, whole raw frames on standard output, no header or padding |
| Formats | depth 8 `rgba`, 16 `rgba64le`, 32 `gbrapf32le` (planar G, B, R, A), straight alpha ([frame contract](handoff.md#frame-contract)) |
| Frame count | a `frame` stage writes one frame per frame read; a temporal stage writes the count its parameters and `--frames` say |
| Partial frame | input that ends inside a frame is bad input, exit 4 |
| Empty input | zero frames in, zero frames out, exit 0 |
| Exits | 0 done, 2 usage (an unknown option, a depth other than 8, 16 or 32), 3 unsupported, 4 bad input; any other non zero status is `stage_failed` ([errors and exit codes](handoff.md#errors-and-exit-codes)) |
| Declared geometry | with `geometry` `declared`, `apply <effect> --describe --width W --height H --depth D --fps N/D [--frames N] [--param K=V]...` reads no input and prints the output width, height, depth, fps and frame count (`contracts/describe.schema.json`); depth and fps stay as asked |
| Placeholders | a wrapper fills `{width}`, `{height}`, `{depth}`, `{fps}`, `{frames}` and `{pix_fmt}` in a stage command, then splits it like a shell would, without a shell |

## Vendoring

| File | Rule |
|---|---|
| `contracts/*.json`, `contracts/conformance/*` | copied byte for byte: the schemas, the error table and the conformance cases |
| `tools/conformance.py` | copied byte for byte; Python 3.12 standard library only, run with `python -I` |
| `tools/test_conformance.py`, `tools/handoff.py`, `tools/test_handoff.py`, `docs/handoff.md` | copied as well by a provider that exchanges packets |

A provider that exchanges packets also adds an entry to `tools` in `contracts/tool.json` and runs `python -I tools/handoff.py sync` ([commands](handoff.md#commands)); one that only offers stages skips the registry.

## Discovery

| Source | Form |
|---|---|
| `--provider FILE` | one `*.stage.json` manifest |
| `HANDOFF_PROVIDERS` | folders, separated like `PATH`, whose `*.stage.json` files are manifests |
| `contracts/tool.json` | a registered tool's `providers` entry: the binary, its `list_args` and the adapter (`native` for a manifest) |

`python -I tools/handoff.py stages --json` lists every stage of every provider with a ready command in its `stage` key, the program as an absolute path, for example `.../sapphire stage apply selftest-invert --width {width} --height {height} --depth {depth} --fps {fps}`. A provider that cannot be listed appears under `problems`.

## Conformance

`tools/conformance.py` reads `contracts/conformance/cases.json` and checks level by level:

| Level | Checks |
|---|---|
| L0 | manifests of every registered provider and every `*.stage.json` |
| L1 | the stage matrix: depths 8, 16 and 32, sizes from 1x1 up, frame counts, opaque, partial and zero alpha, alone and chained in both orders |
| L2 | the wrappers end to end with `--lossless`, both orders, an ffmpeg filter between |
| L3 | failure injection under a deadline: the error exits of every stage under test |
| L4 | packets through `tools/handoff.py` in a scratch folder |
| L5 | frames/2 wrapper reports and error documents |

| Flag | Runs |
|---|---|
| `--quick` | a covering subset of about a minute (the default; used by `doctor --conformance` and CI) |
| `--full` | the whole matrix of every level |
| `--levels L0,L1,...` | only these levels |
| `--stage CMD` | L1 and L3 on one stage command: the command up to and including its stage id, or a ready string of `handoff.py stages --json` |
| `--bench` | the wrappers on chained stages; writes `docs/handoff-bench.tsv` |
| `--json` | the result as JSON |
| `--allow-absent-peer` | exit 0 when every part that did not run waited for an absent peer repository (CI) |

| Exit | Meaning |
|---|---|
| 0 | every selected level ran and passed |
| 1 | a check failed |
| 2 | usage |
| 6 | nothing failed, but a level printed `NOT RUN` |

The stage command must name its program by absolute path: the stage runs from a scratch folder, so a relative path fails every case with `tool_missing`. The ready strings of `handoff.py stages --json` are absolute already. The sapphire stage on Windows:

```sh
python -I tools/conformance.py --stage "C:/path/to/sapphire/target/release/sapphire.exe stage apply selftest-invert"
```

It prints `L1 stage matrix pass` and `L3 failure injection pass`, marks the other levels `not selected` and ends `result: pass (exit 0)`.

## Running in a wrapper

```sh
target/release/sapphire clip.mp4 retime --param speed_percent=50 --frames 95 --depth 8 --stage @selftest-invert --stage vfx
target/release/sapphire clip.mp4 --stage "<program> apply <effect> --width {width} --height {height} --depth {depth} --fps {fps}" --geometry declared
python tools/apply.py clip.mp4 retime --param speed_percent=50 --frames 95 --depth 8 --contract frames/2 --pre-stage "<program> apply <effect> --width {width} --height {height} --depth {depth} --fps {fps}"
```

| Wrapper | Stage placement |
|---|---|
| `sapphire` | `--stage CMD`, repeatable, run in order; `vfx` among them marks where twixtor's effect runs (first by default); `@NAME` runs a stage of this repository without naming its program |
| twixtor `tools/apply.py` | `--pre-stage CMD` before `vfx`, `--post-stage CMD` after it; `--contract frames/2` gives the error object and the exits of the convention |
| both | `--geometry declared` goes right after the stage it describes; a stage that changes the frame rate is refused with `unsupported` |
