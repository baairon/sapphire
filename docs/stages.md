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

An effect stage refuses some cases by design, and the tool counts a refused case as a failed check: it prints a `FAIL` line with the exit code and the message, and the run ends `result: fail (exit 1)`. The refusals of the effect stages that are not transition stages are the 1x1 cases (Exits under [Effects provider](#effects-provider)).

A transition stage, `swishpan` or the `S_DissolveShake` stage, also needs `--frames N` equal to the number of input frames, with N of 2 or more ([Transition stages](cli.md#transition-stages)). The tool appends `--width`, `--height`, `--depth` and `--fps` to a stage command and never `--frames`, so the command carries a fixed clip length. 12 is the longest clip of L1 and of L3:

```sh
python -I tools/conformance.py --stage "C:/path/to/sapphire/target/release/sapphire.exe stage apply swishpan --frames 12"
```

Every L1 case of 12 frames then passes but 1x1, every L3 case passes but the empty input, and the `FAIL` lines are these refusals:

| Case | Exit | Message |
|---|---|---|
| L1 at 1x1, any frame count | 3 | `swishpan needs a width of 2 or more` |
| L1 at 1 or 2 frames, 2 pixels wide or more | 4 | `the input holds N frames, but --frames says 12` |
| L3 `empty input` | 4 | `the input holds 0 frames, but --frames says 12` |

MEASURED: the default mode prints L1 `4 passed, 7 failed` and L3 `32 passed, 1 failed`, and `--full` prints L1 `72 passed, 99 failed` and L3 `32 passed, 1 failed` (a case that passes is two checks, its run and a repeat that must give the same bytes). A command without `--frames` is refused with `FRAMES-REQUIRED` (exit 3), and one with `--frames 1` with `FRAMES-TOO-SMALL` (exit 3), before any frame is read: every L1 case fails, and so do the L3 checks of a partial and of an empty input (MEASURED in the default mode: L1 `0 passed, 9 failed`, L3 `31 passed, 2 failed`). A clip of one frame is therefore not checked on a transition stage. With `stage apply dissolve_shake --frames 12` as the stage command the refusals are the same (the 1x1 message names the stage), and `--full` prints the same figures, L1 `72 passed, 99 failed` and L3 `32 passed, 1 failed` (MEASURED).

## Effects provider

`sapphire stage list --json` also lists one stage per registered style of the nineteen landed effects (25 stages). Each stage renders in process through the `fx_exec` library, the same code as `fx_run`, on the verified render path, the host arrangement the recorded renders of the parity record used. A later effect joins by adding rows to `crates/neutral-handoff/data/fx_stages.tsv` and `fx_params.tsv`.

**Requirements.** The stages are linked into `sapphire`; the effects need no external data, and no `fx_run` executable is needed. A run writes no scratch files and no debug side files (only the Trails history route chains its renders through a scratch folder, removed on every exit path). `--device cpu|gpu|gpu=N|auto` picks the backend (default `cpu`); the device path is described in [Rendering on a device](guide.md#rendering-on-a-device) and falls back to the CPU for any node, or the whole run, that a device cannot serve exactly.

**Per frame.** The input frame is handed to the runner in memory as 'ae32' words (A, R, G, B words per pixel, f32 bits, straight alpha); no file is written. At depth 32 this is a pure byte permutation of the frames/2 planes G, B, R, A. Depths 8 and 16 convert through f32 (`v/255`, `v/65535`, back with round half to even and a clamp) and are not verified. The runner's output words are relaid back the same way.

The stages as `sapphire stage list` prints them (the literal output of that command; the last 25 lines are the effect stages):

```text
id               name             family    kind   depths    params  layers
selftest-invert  Selftest Invert  selftest  frame  8,16,32   1       0
ffmpeg-filter    ffmpeg Filter    ffmpeg    frame  8,16,32   3       0
blur_color_autogen Blur_Color_Autogen S_Blur    temporal 8,16,32   17      0
blur_mono_autogen Blur_Mono_Autogen S_Blur    temporal 8,16,32   17      0
transform_and_blur_autogen Transform_and_Blur_Autogen BlurMoCurves temporal 8,16,32   9       0
blur_motion_curves_no_transform_autogen Blur_Motion_Curves_No_Transform_Autogen BlurMoCurves temporal 8,16,32   9       0
diffuse_autogen  Diffuse_Autogen  S_Diffuse temporal 8,16,32   15      0
distort_rgb_autogen Distort_RGB_Autogen S_DistortRGB temporal 8,16,32   10      0
drop_shadow      Drop_Shadow      S_DropShadow temporal 8,16,32   10      0
hue_sat_bright_autogen Hue_Sat_Bright_Autogen S_HueSatBright temporal 8,16,32   8       0
sharpen_best_quality_autogen Sharpen_Best_Quality_Autogen S_Sharpen temporal 8,16,32   13      0
sharpen_fast_autogen Sharpen_Fast_Autogen S_Sharpen temporal 8,16,32   8       0
time_warp_rgb_autogen Time_Warp_RGB_Autogen S_TimeWarpRGB temporal 8,16,32   6       0
trails_autogen   Trails_Autogen   S_Trails  temporal 8,16,32   11      0
warp_perspective_autogen Warp_Perspective_Autogen S_WarpPerspective temporal 8,16,32   9       0
flicker          Flicker          S_Flicker temporal 8,16,32   14      0
glow             Glow             S_Glow    temporal 8,16,32   17      0
swishpan         SwishPan         S_SwishPan temporal 8,16,32   10      1
shake_normal_autogen Shake_Normal_Autogen S_Shake   temporal 8,16,32   25      0
shake_twitchy_autogen Shake_Twitchy_Autogen S_Shake   temporal 8,16,32   25      0
shake_jumpy_autogen Shake_Jumpy_Autogen S_Shake   temporal 8,16,32   23      0
dissolve_shake   Dissolve_Shake   S_DissolveShake temporal 8,16,32   36      1
blur_motion_autogen Blur_Motion_Autogen S_BlurMotion temporal 8,16,32   12      0
blur_motion_mono_autogen Blur_Motion_Mono_Autogen S_BlurMotion temporal 8,16,32   12      0
warp_transform_autogen Warp_Transform_Autogen S_WarpTransform temporal 8,16,32   11      0
glow_edges       Glow_Edges       S_GlowEdges temporal 8,16,32   20      0
glint            Glint            S_Glint   temporal 8,16,32   16      0
```

Through each stage, the counted recorded renders of the parity record that match the stage's host arrangement are rendered again and compared by full SHA-256 (MEASURED). For the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective` this is the stage provider's parity run, `tests/fx_stage_parity.rs`: 382 of 382 byte identical. The counted renders of `flicker`, `glow`, `swishpan`, the three `S_Shake` stages, the `S_DissolveShake` stage, the two `S_BlurMotion` stages, `warp_transform_autogen`, the `S_GlowEdges` stage and the `glint` stage are not part of that test; each was rendered through `sapphire stage apply` and compared with the original's output. For the three `S_Shake` stages and the `S_DissolveShake` stage the second number counts the counted renders that equal the original; the four that differ are described in [Effects](effects.md):

| style | recorded renders matching the stage's host arrangement |
|---|---|
| Blur_Color_Autogen | 18 of 24 |
| Blur_Mono_Autogen | 18 of 24 |
| Transform_and_Blur_Autogen | 13 of 35 |
| Blur_Motion_Curves_No_Transform_Autogen | 13 of 32 |
| Blur_Motion_Autogen | 112 of 168 |
| Blur_Motion_Mono_Autogen | 120 of 184 |
| Diffuse_Autogen | 31 of 42 |
| Dissolve_Shake | 27 of 36 |
| Distort_RGB_Autogen | 26 of 30 |
| Drop_Shadow | 20 of 24 |
| Flicker | 24 of 24 |
| Glint | 22 of 23 |
| Glow | 36 of 36 |
| Glow_Edges | 34 of 34 |
| Hue_Sat_Bright_Autogen | 118 of 130 |
| Shake_Normal_Autogen | 17 of 22 |
| Shake_Twitchy_Autogen | 18 of 22 |
| Shake_Jumpy_Autogen | 19 of 22 |
| Sharpen_Best_Quality_Autogen | 19 of 42 |
| Sharpen_Fast_Autogen | 16 of 36 |
| `swishpan` (its registered program) | 32 of 36 |
| Time_Warp_RGB_Autogen | 10 of 22 |
| Trails_Autogen | 70 of 120 |
| Warp_Perspective_Autogen | 10 of 224 |
| Warp_Transform_Autogen | 61 of 78 |

**Host arrangement.** Each style runs the host arrangement of its recorded runs, listed in `crates/neutral-handoff/data/fx_stages.tsv`: the options of the `host` column, then the buffer placement of the `placement` column for the frame size, then the options of the `fixed` column, in that order on the runner's command line. Where the recorded runs at another value of one offered parameter used other host options, the `host_variants` column holds them (below). The stage derives the rest per frame: `--size` and `--geom` from the frame (`-0.5,W-0.5,1,0,(H-1-W)/2,0,0`), `--host-frame` and `--ctx-time` from the frame index (0 based), and the buffer placement per frame size (a table at the measured sizes, the 64x32 placement elsewhere, INFERRED).

**Writer phase lists.** A placement entry `--xy-base-list COUNT:B,B;COUNT:B` gives one phase list per default-xy writer count, and the stage passes the list for the writer count of the compiled plan (`S_BlurMotion` runs one writer at Blur Res Half and Quarter and two otherwise; a `tilt_alpha` plan of `S_WarpTransform` runs two). A plan with no default-xy writer takes no list. A writer count that the placement does not list is refused with `XY-BASE-LIST-UNSERVED`.

**Host variants.** `host_variants` is the last column of `fx_stages.tsv`. It is `-` on 14 of the 25 effect stages. `glow`, `swishpan`, the three `S_Shake` stages, the `S_DissolveShake` stage, the two `S_BlurMotion` stages, `warp_transform_autogen` and the `S_GlowEdges` stage carry entries keyed by `cset_opacity` for the Opacity answers 1 and 3, taken from the recorded runs at those answers. `glint` carries one entry, for the Opacity answer 3: its recorded runs hold no answer 1, so that answer meets `SOURCE-VARIANT-MISMATCH` (exit 3, MEASURED). A row carries entries only when the recorded runs of its style at some value of one offered parameter (another opacity answer, for example) used a host option other than the `host` column's, such as another source plane count or reader head. The entries are separated by `;`, and each is `PARAM_ID=VALUE:` followed by options in the form of the `host` column, `--opt value|--opt value`. The unit tests use this text on a copy of the `blur_color_autogen` row (`--extra-opt` stands for an option the `host` column lacks):

```text
cset_opacity=1:--src-planes 3|--src-head 0x7e91c0|--extra-opt 9;cset_opacity=3:--src-head 0x7ea840
```

- **One selecting parameter.** Every entry of a row names the same PARAM_ID, an offered parameter of the stage (a row of `fx_params.tsv`), and each VALUE appears once. No entry is keyed at the parameter's registered default (for an opacity answer, the answer its style registers as the default): the `host` column serves the default.
- **Selection per frame.** The stage reads the parameter's value at the frame, after keys (an opacity answer, popup or integer holds until its next key, so a keyed selecting parameter changes entry at its keys). It starts from the `host` column; each option of the entry whose VALUE equals that value replaces the option of the same name where it stands, or is added after the host options when the column lacks it. Options the entry does not name keep their `host` values, and the placement and fixed options follow as before. VALUE and the frame's value compare as numbers when both read as numbers (`1` equals `1.0`), else as text.
- **No entry.** A frame whose value has no entry, and a run that does not give the parameter, use the `host` column unchanged. A value whose host arrangement no recorded run shows therefore meets the runner's own checks (for example `SOURCE-VARIANT-MISMATCH`, exit 3) instead of a guessed arrangement.
- **What an entry holds.** The options of that value's own recorded runs, including those the runner also derives and checks against the given ones (the reader head `--src-head`, checked by `SOURCE-VARIANT-MISMATCH`; the destination planes and mode `--ae32-dest PLANES,MODE`, checked by `MODE-BYTE-MISMATCH`), so each value is checked against its own records. An entry never carries a parameter value, an input the runner derives or a placement option.
- **The postdivide value.** The `--do-postdiv` value learned from `POSTDIV-MISMATCH` (below) is learned again whenever a later frame's run refuses with another value, so a keyed selecting parameter that changes entry within the clip needs nothing more.

The column is checked when `stage apply` starts to render, after `LAYER-INPUT-UNSUPPORTED` and the `--frames` checks of a transition stage and before the checks of the frame width, the parameters and the layer file, so a refusal writes no frame. `--check` and `--describe` do not check it. Each fault is refused by name with exit 3 ([refusal labels](cli.md#refusal-labels)):

| Label | Refused when |
|---|---|
| `HOST-VARIANT-MALFORMED` | the column does not parse: an entry has no `:` after `PARAM_ID=VALUE`, its key is not `PARAM_ID=VALUE` or has an empty id or value, it holds no option, one of its items does not start with `--`, or an option has no value |
| `HOST-VARIANT-PARAMS` | the entries of the row name more than one PARAM_ID |
| `HOST-VARIANT-PARAM-UNKNOWN` | PARAM_ID is not an offered parameter of the stage |
| `HOST-VARIANT-DUPLICATE` | two entries have the same VALUE (compared as above, so `1` and `1.0` are the same) |
| `HOST-VARIANT-AT-DEFAULT` | VALUE is the registered default of the parameter, compared as above (`2` and `2.0` are the same). For a parameter that `fx_run --describe-params` describes, that is the default it gives. An opacity answer (`cset_*`) is not described there: its registered default is the one-based host answer that the style's native parameter definitions register for the popup, `2` for every `cset_opacity` and for `cset_input_opacity`, `1` for `cset_output_opacity` (both of `time_warp_rgb_autogen`) |
| `HOST-VARIANT-OPTION` | an entry carries `--do-postdiv` or `--owner-opacity` (derived by the runner), `--size`, `--out`, `--ae32-src`, `--ae32-prior`, `--ae32-out`, `--geom`, `--host-frame`, `--ctx-time`, `--host-times`, `--host-value`, `--ns-default-unbound` or `--frame-words` (set by the stage on every run), `--cset`, `--set`, `--host-param` or `--host-curve-values` (given by a parameter), `--prior-fill` (fixed per style), or an option of the row's `fixed` column |
| `HOST-VARIANT-PLACEMENT` | an entry carries an option that the `placement` column sets at any measured size; placement stays per size |

MEASURED by unit tests: the column parses (and each malformed form is refused); every row of this build passes the checks; an unbound parameter, an unlisted value and the default run the `host` column; an entry replaces and adds options and leaves the others; a keyed selecting parameter (`1@0;3@2;2@4`) gives each frame the host options of a plain run at that frame's value; each refusal carries its name and exit 3; a `cset_opacity` entry keyed at its registered default `2` (also written `2.0`) is refused with `HOST-VARIANT-AT-DEFAULT`, and entries keyed at `1` and `3` pass; every opacity answer that a stage offers has a registered default answer; the entry's options reach the runner's command line. Through the entries of `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage, the counted recorded renders at Opacity answers 1 and 3 that match the stage's host arrangement render byte identical to the original's output (12 on `glow`, 4 on `swishpan`, 8 on the three `S_Shake` stages and 6 on the `S_DissolveShake` stage, MEASURED as for the table of stage renders above); a run at answer 1 or 3 on the `host` column alone is refused by name (MEASURED).

**Parameters.** A parameter is offered when a counted recorded render set it. Every other parameter of the style stays at its registered default, which `fx_run` applies, and a stage passes on only the parameters it was given. A value is passed through unchanged (a vector is `a,b` or `r,g,b`; a checkbox given as `true`, `false`, `on` or `off` is passed as `1` or `0`), or read per frame when it is keyed (below). `fx_params.tsv` holds only the id, the `fx_run` option form and the name of each offered parameter, taken from the manifests; the type, lanes, range and registered default are asked from `fx_run --describe-params`, which prints them from the native definitions, when the stage builds its list. An opacity answer (`cset_*`) is not described there: the stage reads the answer the style registers and the item count of the popup from the style's native parameter definitions, lists them as its `default` and as its range 1 to the item count, checks host variant entries against that default (`HOST-VARIANT-AT-DEFAULT` above), and refuses an answer outside the range with exit 3 before any frame is read. A one-lane popup lists the default an unset run takes, the item the style registers as a zero-based index, in place of the program text default; of the offered parameters this changes only `auto_trans` of `swishpan`, listed as `1` where `--describe-params` prints `0` ([Parameter discovery](cli.md#parameter-discovery)). The listed default of a `--host-param` id is the program's record value, not a host value.

**Two inputs that the runner reveals by refusing or defaults.** `--do-postdiv` is a required cross-check on the 'ae32' route, and the runner derives its value from the host's opacity arm (`--owner-arm`) and the opacity parameters. The stage sends 0, and when the runner refuses with `POSTDIV-MISMATCH: the owner arm ... derives ctx = N, the recorded cell is M` it takes N from the structured field `io.postdiv` of the run's result (no text is parsed), repeats the frame and keeps N for the later frames of the clip, until a later frame's run refuses with another value. A refusal that carries no value is `stage_failed` carrying that line. `--owner-opacity` is omitted: the runner derives it. The stage passes `--ns-default-unbound` on every run: an unbound normalize_scale parameter then takes its registered default from the native definitions instead of the `PARAM-UNITS` refusal (without the flag the runner is unchanged).

**Keyframes.** The 20 effect stages are `kind: temporal`, because the frames/2 convention accepts keyed values (`V@F;V@F;...`) only for temporal stages; their output frame N still depends only on input frame N (and, for Time_Warp and Trails, on the frames they always read). A keyed value follows the convention's syntax and interpolation exactly: keys at output frame numbers counted from 0, `h` to hold and `e` to ease a key, Bezier handles `iS/I` and `oS/I`, the value of the first key before it and of the last key after it. Scalars and angles interpolate, two-lane vectors move on the straight line between their keys, and integers, popups, checkboxes, colours, opacity answers and file paths hold until the next key. A float that a handle overshoots is held inside its range. Keys are stored in ticks of the rate given by `--fps` (100 a frame at an integer rate, D a frame at a rate N/D), so a keyed value without `--fps` is exit 2. The stage reads each keyed parameter at the frame and runs the in-process runner with that value; a host value (`--host-param`) between keys is put on the 1/65536 grid of a host slider, round half away from zero. `trails_autogen` renders the clip in one run and refuses a keyed value with exit 3. MEASURED: frame N of a keyed run is byte identical to frame N of a plain run at the value the keys give for frame N (linear, eased, Bezier, held and two-lane cases on five stages). Values between keys are not recorded renders of the original, so they are INFERRED.

**Mask input.** The programs' `Mask` input is not offered as a layer. On the stages' host arrangements `fx_run` refuses a connected mask (`INPUT-CLASS-CELL-UNRECORDED` on the four-plane styles, a plane-count refusal on the three-plane styles; MEASURED on seven stages), because the recorded host runs never connected one; the mask stays the constant 1. The three `S_Shake` stages fix the three parameters of that input in their `fixed` column: `--set mask_use=0`, `--set blur_mask=0.05` and `--set invert_mask=0`. These are the values the original holds for the parameters of an absent input at every frame size (MEASURED in the recorded runs at 64x32 and 50x7); left unbound, `blur_mask` would take the runner's default for a host-unit parameter, converted with the frame width. The node that reads it is not reached from the output (MEASURED in the compiled plans of the recorded runs), so neither value changes the output. `S_DissolveShake` has no mask input.

**Layers.** Only a transition stage offers a layer, and `swishpan` and the `S_DissolveShake` stage are the two of the 20 effect stages: `stage list --json` shows `"layers": [{"id": "To", "optional": true}]` for each of them and `"layers": []` for the other 18, and `--layer` on any of those exits 3 with `LAYER-INPUT-UNSUPPORTED` before any frame is read (MEASURED). The recorded host runs of those styles never bound a layer input; `glow` keeps its background and matte inputs unconnected, as in its verified renders. The `layers` field stays in the manifest for transition stages and for other providers.

**Transition stages.** A style whose row of `fx_stages.tsv` names a layer in its `layers` column is a transition stage (`-` names none; a trailing `?` marks the layer optional). Its manifest entry lists the layer as `{"id": ..., "optional": ...}`, and the stage takes two inputs that the other effect stages refuse or only check:

- The second clip, `--layer To=PATH`. `To` binds the first declared layer, and the declared id binds it as well. The stage opens the file before the first frame and reads it as raw frames of the input's depth and size: one frame, held for the whole clip, or one frame per input frame. Each layer frame is handed to the runner in memory as 'ae32' words, like the input frame, under the layer's id (`--ae32-src ID=layer_ID.u32`). The layer is required unless it is marked optional.
- The clip length, `--frames N`, with N of 2 or more. Frame f is rendered with `get_percent_time` f32(f) / f32(N - 1), `get_total_frames` f32(N) and `get_aspect_ratio` f32(H) / f32(W), the frame height over its width, handed to the runner as `--host-value NAME=0x...` with the f32 bits. The transition therefore spans exactly the clip the stage is given, from 0 on its first frame to 1 on its last. A second clip that starts at another time, or a clip cut from a longer transition, is not modelled (a choice of the stage, INFERRED).

A transition stage gives a frame's plan only the clips it reads: where the compiled plan of a frame does not read one of the two clips (for the `S_DissolveShake` stage, before its dissolve starts or after it ends), the stage renders that frame without the clip. `fx_run` itself keeps refusing a given clip that the compiled plan does not read.

A missing or unusable input is refused by name with exit 3 before any frame is read ([refusal labels](cli.md#refusal-labels)); the example and the order of the checks are in [Transition stages](cli.md#transition-stages). MEASURED by unit tests on a stand-in transition profile: the layer words and the three host values reach the runner's arguments, `get_percent_time` is f32(f) / f32(N - 1) at f = 0, 12 and N - 1 for N = 25, `get_aspect_ratio` is 7/50 at 50x7, and each refusal carries its name and exit 3. `swishpan` is registered: 32 of its 36 counted recorded renders, each one frame of a recorded transition, render through the stage byte identical to the original's output (MEASURED as for the table of stage renders above, with `--frames` set to the recorded transition length, and for the renders with `auto_trans` 0 with `host_curve_values` set to a curve file that holds the recorded Wipe Percent). The other four are 50x7 renders recorded with the default xy buffer at base phase 16, while the stage places it at phase 0 (`--xy-base 0`) at every size: they are verified through `fx_run` at phase 16, so all 36 are byte identical through `fx_run`, and through the stage each of the four is checked against the native render at phase 0.

The `S_DissolveShake` stage is registered as well. Its plan does not always read both clips: where the dissolve shows only one of them, before it starts or after it ends, the compiled plan reads only that clip. The runner then reports the other clip as a source that is not an input of the compiled plan, and the stage renders the frame again without it, as a host hands a plan only the inputs it reads; with the Opacity answer 1 entry, the count of three-plane source images in the host options drops by one with that clip. A unit test checks the removal (MEASURED). Of the 36 counted renders of the effect that equal the original, the plan of 8 reads only one clip (6 only the first, 2 only the second). Through the stage all 36 are byte identical to the runner's render at phase 0 of the default xy buffer, and 27 to the original, these 8 among them, at their recorded transition length of 240 frames (MEASURED as for the table of stage renders above). The other 9 are 50x7 renders recorded at phase 16 and are verified through `fx_run`, as for `swishpan`.

**Frame time.** Every effect stage run with `--fps` also hands the runner `get_time` for frame f at the reduced rate num/den, the time in seconds as the original computes it for a host with time scale num and local time step den: F = f32(f x den) / f32(den), rate = f32(num) / f32(den), get_time = F / rate, each an f32 division (a rate below 0.0001 is replaced by 30). At 24/1 it is f32(f) / f32(24). It also hands the runner `get_frames_per_second` f32(num) / f32(den), the frame rate the original computes for a host with time scale num and local time step den (a rate below 0.0001 is replaced by 30). A time scale num above 4294967295, a local step den above 2147483647 or a frame whose tick f x den is above 2147483647 does not fit the host's time fields and is refused with `HOST-TIME-RANGE` (exit 3); the rate is checked before any frame is read. The Trails history route passes no host value. With `get_time` passed, the 382 recorded renders of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective` in the table above stay byte identical through the stages (MEASURED). `flicker` reads it on every frame; its recorded renders, at whole frames of a 24 frames a second clock, render through the stage at `--fps 24` (24 of 24 above). Without `--fps` such a stage is refused with `HOST-VALUE-MISSING` (exit 3) before any frame is written. The three `S_Shake` stages and the `S_DissolveShake` stage read both `get_time` and `get_frames_per_second`; without `--fps` they are refused with `FPS-UNGIVEN` (exit 3) before any frame is written (MEASURED on the four stages by a test of the command line). Their recorded renders are at whole frames of a 24 frames a second clock and render through the stages at `--fps 24` (the table above).

**Temporal stages.** `time_warp_rgb_autogen` keeps a bounded window of the largest positive plus the largest negative channel shift plus 1 frames. Before frame 0 the route clamps (`max(0, ...)`); past the last input frame the stage holds the last frame (a choice of the stage, not verified, INFERRED). `trails_autogen` is `spool`: it buffers the clip and runs the measured history route once, with `--host-times 1..N` and the render index as the frame (render k is clip frame k-1 at time k). That time base starting at 1 is a per style host choice, measured for two frame histories and INFERRED beyond them.

**Host curves.** `transform_and_blur_autogen`, `blur_motion_curves_no_transform_autogen` and `swishpan` read a host curve file. Pass its path as `--param host_curve_values=PATH`. Without it the stage writes a constant identity curve (rotate 0, shift 0, z_dist 1, centre 50 by 50) in the recorded format, with the samples the frame needs. MEASURED: 2 of the 13 recorded renders of each `S_BlurMoCurves` style that match the stage's host arrangement have a recorded curve file equal to that text byte for byte at frame 0; other curve values and later frames are INFERRED. The identity curve gains every sample `fx_run` asks for across the whole shutter interval, so any `shutter_duration` runs (MEASURED). A sample that neither a given curve file nor the identity curve can serve ends exit 3 `unsupported` with `HOST-CURVE-SAMPLE-UNSERVED` (MEASURED). For `swishpan` the curve file carries the wipe progress, the original's Wipe Percent (curve index 6), that the effect reads when `auto_trans` is 0. When none is given the stage writes the same constant curve file with a Wipe Percent of 0, so an `auto_trans` 0 run reads a progress of 0 on every frame (MEASURED: exit 0); the counted renders with `auto_trans` 0 are verified with a curve file that holds their recorded Wipe Percent, 35 or 50 percent, constant over the render. The identity curve serves only samples of the stage's own program (index 6 on `swishpan`, 22 to 25 on the two `S_BlurMoCurves` stages) and never one of a parameter the stage offers; such a sample ends with `HOST-CURVE-SAMPLE-UNSERVED`.

**Host-unit parameters.** A host-unit parameter with one host lane is converted by the runner as the host value times 2 over the frame width times its normalize scale (1, 2 or 4), on both axes: a vertical amplitude such as `y_rand_amp` uses the width too, so a frame of 50x7 divides by 50. A normalize scale of 1 with one host lane is accepted for one-lane parameters only; a two-lane parameter takes both host lanes, and any other scale is refused with `PARAM-UNITS-NOT-JUDGED`.

**Exits.** An unknown or out of range parameter is exit 3. A refusal of the runner (its code 2, for example `PARAM-UNITS-NOT-JUDGED`) is exit 3 carrying the refusal; a runner failure (its code 1) is `stage_failed`. A frame narrower than 2 pixels is exit 3 (the host grid needs two columns). Each stage therefore refuses the two 1x1 cases of `tools/conformance.py` with exit 3; these are documented size refusals (never padded), and every other case of L1 and L3 passes for the 18 effect stages that are not transitions (MEASURED on each of the 18; with `--full` on `flicker`, `glow` and the three `S_Shake` stages, L1 refuses its 27 cases at 1x1 and passes the other 216). `swishpan` and the `S_DissolveShake` stage also need `--frames` equal to the number of input frames ([Transition stages](cli.md#transition-stages)). With a stage command that sets `--frames 12`, its L1 cases of 12 frames pass at every size but 1x1, a case with another frame count ends with exit 4 (the input does not hold the frames `--frames` gives), and L3 passes every case but the empty input (MEASURED; the command and the refusals are in [Conformance](#conformance)). A partial input frame is exit 4 and empty input is exit 0, except on `swishpan` and the `S_DissolveShake` stage, where an empty input does not hold the frames its `--frames` gives (exit 4).

**What is verified.**

| Case | Status |
|---|---|
| Depth 32, the sizes and parameter values of the counted recorded renders that match the stage's host arrangement | byte identical to the recorded renders, full SHA-256 (MEASURED by `tests/fx_stage_parity.rs` for the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective`, and for `flicker`, `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage by rendering each counted render through `sapphire stage apply`) |
| Other sizes and values | INFERRED (the same code path) |
| Buffer placement and host buffer phases at sizes other than the measured ones | INFERRED; above the thresholds in [limits common to the effects](effects.md#limits-common-to-the-effects) the original's own output depends on the phase |
| Trails clips longer than the measured histories, Time_Warp windows beyond the measured shifts | INFERRED |
| Host curves other than the identity curve | INFERRED, except the recorded Wipe Percent curves of the counted `swishpan` renders with `auto_trans` 0, which are byte identical through the stage |
| Keyed values between keys | INFERRED (each frame is a plain run at its value, MEASURED) |
| The layer and time values of a transition stage | the arguments handed to the runner, MEASURED by unit tests on a stand-in transition profile; the counted renders of `swishpan` and the `S_DissolveShake` stage that match their host arrangement are byte identical through the stage, at their recorded transition lengths; other lengths and rates are INFERRED |
| Host variants | the parse, the per frame selection and the refusals, MEASURED by unit tests on a copy of the `blur_color_autogen` row; the counted renders of `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage at Opacity answers 1 and 3 that match the stage's host arrangement are byte identical through the entries |
| The `get_time` value | unit tested at 24/1, 30/1, 24000/1001, 30000/1001, 60000/1001 and 2997/100 against the original's f32 read order; equality with a host's value at a rate whose denominator is not 1 is INFERRED, because the time scale and local step a host passes at such a rate are not recorded |
| The `get_frames_per_second` value | at 24/1 and 2997/1 equal to the rate in the recorded Shake and Dissolve_Shake renders, whose host passes a local time step of 1 (MEASURED); at a rate whose denominator is not 1 it follows the original's f32 arithmetic for a host with time scale num and local time step den, and equality with a host's value is INFERRED, because the time scale and local time step a host passes at such a rate are not recorded |
| Depths 8 and 16 | new conversion, not verified |

**Limits.** The stages render in process, one runner thread per clip, and the parallel-safe kernels use all cores (`SAPPHIRE_THREADS=1` for one). MEASURED wall time of one 1920x1080 frame through `sapphire stage apply` on a 12 core server under shared load (one-minute load average 5.3 to 9.2), two runs per cell after one warm-up run, in seconds. The frame holds random finite values in [0, 1). `swishpan` and the `S_DissolveShake` stage need a clip of at least 3 frames: their cells are the wall time of a 3 frame clip (`--frames 3`) divided by 3. The two `after` columns were measured in one session, CPU and device runs interleaved, except the rows of `blur_motion_autogen`, `blur_motion_mono_autogen`, `warp_transform_autogen`, `stage apply glow_edges` and `glint`, which come from a later session (below). The `before` column is the previous build, measured in an earlier session at another load, and was not measured for the twelve stages added after it (`-`):

| stage | before the in-process route (s) | after, CPU (s) | after, PoCL device (s) |
|---|---|---|---|
| `blur_color_autogen` | 1.27 / 1.22 | 0.63 / 0.73 | 0.79 / 0.73 |
| `blur_mono_autogen` | 1.02 / 1.01 | 0.59 / 0.57 | 0.64 / 0.70 |
| `transform_and_blur_autogen` | 11.26 / 11.87 | 1.03 / 1.01 | 0.73 / 0.69 |
| `blur_motion_curves_no_transform_autogen` | 52.84 / 51.09 | 1.09 / 1.10 | 0.74 / 0.73 |
| `diffuse_autogen` | 3.02 / 2.90 | 2.13 / 1.90 | 0.71 / 0.62 |
| `distort_rgb_autogen` | 8.48 / 7.57 | 4.80 / 4.45 | 1.02 / 1.00 |
| `stage apply drop_shadow` | 1.84 / 1.74 | 1.63 / 1.50 | 0.63 / 0.64 |
| `hue_sat_bright_autogen` | 0.84 / 0.84 | 0.41 / 0.44 | 0.50 / 0.50 |
| `sharpen_best_quality_autogen` | 2.79 / 2.71 | 3.45 / 3.38 | 1.19 / 1.14 |
| `sharpen_fast_autogen` | 1.18 / 1.10 | 0.73 / 0.73 | 0.68 / 0.66 |
| `time_warp_rgb_autogen` | 1.44 / 1.31 | 0.74 / 0.79 | 0.89 / 0.80 |
| `trails_autogen` | 1.26 / 1.16 | 0.86 / 0.82 | 0.94 / 0.93 |
| `warp_perspective_autogen` | 3.87 / 3.49 | 1.41 / 1.39 | 0.93 / 0.93 |
| `flicker` | - | 0.49 / 0.45 | 0.54 / 0.57 |
| `glow` | - | 0.92 / 0.92 | 0.78 / 0.80 |
| `swishpan` | - | 2.32 / 2.55 | 1.16 / 0.90 |
| `shake_normal_autogen` | - | 1.62 / 2.04 | 0.68 / 0.66 |
| `shake_twitchy_autogen` | - | 1.78 / 1.80 | 0.92 / 0.92 |
| `shake_jumpy_autogen` | - | 2.03 / 2.17 | 0.67 / 0.76 |
| `stage apply dissolve_shake` | - | 1.26 / 1.23 | 0.53 / 0.52 |
| `blur_motion_autogen` | - | 6.35 / 6.93 | 1.34 / 1.43 |
| `blur_motion_mono_autogen` | - | 3.95 / 3.93 | 1.05 / 1.25 |
| `warp_transform_autogen` | - | 0.91 / 0.88 | 1.27 / 1.11 |
| `stage apply glow_edges` | - | 0.89 / 0.91 | 0.77 / 0.79 |
| `glint` | - | 1.40 / 1.46 | 1.01 / 1.05 |

The rows of `blur_motion_autogen`, `blur_motion_mono_autogen`, `warp_transform_autogen`, `stage apply glow_edges` and `glint` were measured in a later session at a one-minute load of 6.5 to 7.4, CPU and device runs interleaved. In that session `transform_and_blur_autogen` measures 0.99 / 1.02 s on the CPU and 0.77 / 0.77 s on PoCL, and `blur_motion_curves_no_transform_autogen` 1.31 / 1.10 s and 0.79 / 0.83 s: within 0.10 s of their cells above, except the first CPU run of `blur_motion_curves_no_transform_autogen` (1.31 s against 1.09 s). At its default parameters, which the timing uses, the plan of `warp_transform_autogen` holds only `make_mipmap` and `postdivide`: with `rotate` and `shear` set (golden `+tl`) it adds `tilt` and `warp`.

The CPU cells use the default thread count (all 12 cores) and are sensitive to machine load. `distort_rgb_autogen` shows it most: at a one-minute load of 6 to 9 this build and the earlier in-process builds all measure 4.4 to 4.9 s for its frame, most of it system time in the threaded path, and 1.7 to 2.0 s with `SAPPHIRE_THREADS=1`, so its CPU cell is a contention figure and not a slower kernel. `sharpen_best_quality_autogen`'s `after, CPU` cell lies above its `before` cell for the same reason: the build before this table, timed interleaved with it under the same load, measures 2.5 to 3.6 s against 2.8 to 3.3 s, with equal output bytes.

The default path (`--device cpu`) is byte identical to the former one process per frame route on the parity record. The `PoCL device` column is `--device gpu` on PoCL, an OpenCL runtime on the CPU, with `SAPPHIRE_REQUIRE_DEVICE=1`: in every run the output equals the CPU output by SHA-256 and the report line ends `fallback: none` (`hue_sat_bright_autogen` reports `device: none`: at its default parameters, which the timing uses, its plan folds to the source frame and holds no node). It is not GPU speed, which is INFERRED until measured on a GPU. See [Rendering on a device](guide.md#rendering-on-a-device).

Recorded renders that do not match the stage's host arrangement (a different host arrangement, a fractional frame, a `--prev` file given outside the history chain) count in the parity record and are not verified through the stage.


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
