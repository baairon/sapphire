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

## Effects provider

`sapphire stage list --json` also lists one stage per registered style of the ten landed effects (13 stages). Each stage runs `fx_run` as a child process on the recorded host route, the route the frozen predictions of the parity record used, so the proven binary carries the parity and the stage code has no build coupling to the effect crates. A later effect joins by adding rows to `crates/neutral-handoff/data/fx_stages.tsv` and `fx_params.tsv`.

**Requirements.** The stages need `fx_run` (beside the `sapphire` executable, or named by `SAPPHIRE_FX_RUN`); the effects need no external data. Without `fx_run`, `stage list` still prints the stages, with `"hidden": true`, and `stage apply` exits 3 and names what is missing. A run writes a scratch folder per frame and removes it on every exit path.

**Per frame.** The input frame is written as an 'ae32' word file (A, R, G, B words per pixel, f32 bits, straight alpha). At depth 32 this is a pure byte permutation of the frames/2 planes G, B, R, A. Depths 8 and 16 convert through f32 (`v/255`, `v/65535`, back with round half to even and a clamp) and are not claimed. The `fx_run` output words are relaid back the same way.

The stages as `sapphire stage list` prints them (the literal output of that command; the last 13 lines are the effect stages):

```text
id               name             family    kind   depths    params  layers
selftest-invert  Selftest Invert  selftest  frame  8,16,32   1       0
ffmpeg-filter    ffmpeg Filter    ffmpeg    frame  8,16,32   3       0
blur_color_autogen Blur_Color_Autogen S_Blur    frame  8,16,32   17      0
blur_mono_autogen Blur_Mono_Autogen S_Blur    frame  8,16,32   17      0
transform_and_blur_autogen Transform_and_Blur_Autogen BlurMoCurves frame  8,16,32   9       0
blur_motion_curves_no_transform_autogen Blur_Motion_Curves_No_Transform_Autogen BlurMoCurves frame  8,16,32   9       0
diffuse_autogen  Diffuse_Autogen  S_Diffuse frame  8,16,32   15      0
distort_rgb_autogen Distort_RGB_Autogen S_DistortRGB frame  8,16,32   10      1
drop_shadow      Drop_Shadow      S_DropShadow frame  8,16,32   10      2
hue_sat_bright_autogen Hue_Sat_Bright_Autogen S_HueSatBright frame  8,16,32   8       0
sharpen_best_quality_autogen Sharpen_Best_Quality_Autogen S_Sharpen frame  8,16,32   13      0
sharpen_fast_autogen Sharpen_Fast_Autogen S_Sharpen frame  8,16,32   8       0
time_warp_rgb_autogen Time_Warp_RGB_Autogen S_TimeWarpRGB temporal 8,16,32   6       0
trails_autogen   Trails_Autogen   S_Trails  temporal 8,16,32   11      0
warp_perspective_autogen Warp_Perspective_Autogen S_WarpPerspective frame  8,16,32   9       0
```

Counted members of the parity record that are in profile for each stage (MEASURED, `PROFILE_REPORT.tsv` of the adapter run):

- `Blur_Color_Autogen`: 18 of 24 counted members.
- `Blur_Mono_Autogen`: 18 of 24 counted members.
- `Transform_and_Blur_Autogen`: 13 of 35 counted members.
- `Blur_Motion_Curves_No_Transform_Autogen`: 13 of 32 counted members.
- `Diffuse_Autogen`: 31 of 42 counted members.
- `Distort_RGB_Autogen`: 26 of 30 counted members.
- `S_DropShadow`: 20 of 24 counted members.
- `Hue_Sat_Bright_Autogen`: 118 of 130 counted members.
- `Sharpen_Best_Quality_Autogen`: 19 of 42 counted members.
- `Sharpen_Fast_Autogen`: 16 of 36 counted members.
- `Time_Warp_RGB_Autogen`: 10 of 22 counted members.
- `Trails_Autogen`: 70 of 120 counted members.
- `Warp_Perspective_Autogen`: 10 of 224 counted members.

**Canonical host.** A stage runs one fixed host arrangement per style, chosen from the counted members (the joint tuple with the most members, every measured size covered, four planes where members have them, ties to the three plane arrangement). The stage derives the rest per frame: `--size` and `--geom` from the frame (`-0.5,W-0.5,1,0,(H-1-W)/2,0,0`), `--host-frame` and `--ctx-time` from the frame index (0 based), and the buffer placement per frame size (a table at the measured sizes, the 64x32 placement elsewhere, INFERRED). `--palpha-cells` of Warp_Perspective is `W,1,1.0`. The prior buffer is a constant fill word. The options of each style:

- `Blur_Color_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prior-fill 00000000`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set invert_mask=0`, `--set mask_use=0`.
- `Blur_Mono_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 2,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--effect-answer 2`, `--effect-count 2`, `--prior-fill 00000000`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set invert_mask=0`, `--set mask_use=0`.
- `Transform_and_Blur_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prior-fill aaaaaaaa`. Placement by size: 50x7: --ds-phase 0|--xy-base 0; 64x32: --ds-phase 0|--xy-base 0. Fixed parameters: `--host-param blur_mocha_mask=0`, `--set shift_mocha_mask=0,0`, `--set blur_mask=0.05`, `--set invert_mask=0`, `--set mask_use=0`.
- `Blur_Motion_Curves_No_Transform_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--effect-answer 2`, `--effect-count 2`, `--prior-fill aaaaaaaa`. Placement by size: 50x7: --ds-phase 0|--xy-base-list 0,0|--xy-render 1; 64x32: --ds-phase 0|--xy-base 0. Fixed parameters: `--host-param blur_mocha_mask=0`, `--set shift_mocha_mask=0,0`, `--set blur_mask=0.05`, `--set invert_mask=0`, `--set mask_use=0`.
- `Diffuse_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7ea840`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prior-fill 00000000`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--set invert_mask=0`, `--set mask_use=0`, `--set blur_mask=0`.
- `Distort_RGB_Autogen`: host `--src-planes 3`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7e9cc0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 3,2`, `--ae32-copier 0`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prerender-late 0,0,0`, `--mode-inputs 1,2`, `--class-cells 02,80,-`, `--class-record Source=3`, `--class3-record 0,1,3,03`, `--prior-fill aaaaaaaa`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--set blur_mask=0`, `--set invert_mask=0`, `--set mask_use=0`, `--host-param blur_mocha_mask=0`, `--set shift_mocha_mask=0,-0`.
- `S_DropShadow`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data f8`, `--ranges REAL`, `--src-chan 4`, `--prior-fill 00000000`. Placement by size: 49x9: ; 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set matte_use=1`, `--set invert_matte=0`, `--set show_mocha_mask_only=0`, `--set resize_mocha_mask=1`, `--set shift_mocha_mask=0,-0`, `--set resize_mocha_rel_x=1`, `--set resize_mocha_rel_y=1`, `--set invert_mocha_mask=0`, `--set bypass_mocha_mask=0`.
- `Hue_Sat_Bright_Autogen`: host `--src-planes 3`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7e91c0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 3,2`, `--ae32-copier 0`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prerender-late 0,0,0`, `--mode-inputs 1,2`, `--class-cells 02,80,-`, `--class-record Source=3`, `--class3-record 0,1,3,02`, `--prior-fill aaaaaaaa`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--set blur_mask=0.05`, `--set invert_mask=0`, `--set mask_use=0`, `--set shift_mocha_mask=0,-0`, `--host-param blur_mocha_mask=0`.
- `Sharpen_Best_Quality_Autogen`: host `--src-planes 3`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7e9cc0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 3,2`, `--ae32-copier 0`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data f8`, `--ranges REAL`, `--src-chan 3`, `--prerender-late 0,0,0`, `--mode-inputs 1,2`, `--host-inplace 0`, `--class-cells 00,0a,-`, `--class-record Source=3`, `--class3-record 0,1,3,02`, `--prior-fill 00000000`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set invert_mask=0`, `--set mask_use=0`, `--set blur_mask=0.05`, `--set bypass_mocha_mask=0`, `--set combine_masks=0`, `--set invert_mocha_mask=0`, `--set mocha_mask_opacity=1`, `--set resize_mocha_mask=1`, `--set resize_mocha_rel_x=1`, `--set resize_mocha_rel_y=1`, `--set shift_mocha_mask=0,-0`, `--set show_mocha_mask_only=0`.
- `Sharpen_Fast_Autogen`: host `--src-planes 3`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7e9cc0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 3,2`, `--ae32-copier 0`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data f8`, `--ranges REAL`, `--src-chan 3`, `--prerender-late 0,0,0`, `--mode-inputs 1,2`, `--host-inplace 0`, `--class-cells 00,0a,-`, `--class-record Source=3`, `--class3-record 0,1,3,02`, `--prior-fill 00000000`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set invert_mask=0`, `--set mask_use=0`, `--set blur_mask=0.05`, `--set bypass_mocha_mask=0`, `--set combine_masks=0`, `--set invert_mocha_mask=0`, `--set mocha_mask_opacity=1`, `--set resize_mocha_mask=1`, `--set resize_mocha_rel_x=1`, `--set resize_mocha_rel_y=1`, `--set shift_mocha_mask=0,-0`, `--set show_mocha_mask_only=0`.
- `Time_Warp_RGB_Autogen`: host `--src-planes 3`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7e9cc0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 3,3`, `--ae32-copier none`, `--host-inplace 1`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prerender-late 0,0,0`, `--class-cells 02,00,-`, `--class-record Source=3`, `--class3-record 0,1,3,02`, `--prior-fill aaaaaaaa`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set blur_mask=0.05`, `--set invert_mask=0`, `--set mask_use=0`, `--set shift_mocha_mask=0,-0`, `--tw-route 240,0,1,1,0,0,1`.
- `Trails_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data 7f8`, `--ranges REAL`, `--prior-fill 00000000`. Placement by size: 50x7: ; 64x32: . Fixed parameters: `--host-param blur_mocha_mask=0`, `--set blur_mask=0.05`, `--set invert_mask=0`, `--set mask_use=0`, `--set shift_mocha_mask=0,0`, `--prog-flag-59e 13`, `--host-depth 4`, `--host-alloc 1`, `--host-id 1`, `--host-step 1`.
- `Warp_Perspective_Autogen`: host `--src-planes 4`, `--owner-arm 0,0,0`, `--src-row58 0`, `--src-head 0x7eb3d0`, `--aspect-triple 0,3f800000,3f800000`, `--ae32-dest 4,1`, `--ae32-copier none`, `--dest-stack-word 0`, `--units-ratio 1,1`, `--bound-mask 1`, `--ctx-word2 0`, `--ctx-1ae 4`, `--ctx-has-data f8`, `--ranges REAL`, `--src-chan 4`, `--palpha-cells W,1,1.0`, `--prior-fill 00000000`. Placement by size: 50x7: --persp-base 16|--palpha-base 16|--xy-base 16; 64x32: --persp-base 16|--palpha-base 16|--xy-base 16. Fixed parameters: `--host-param blur_mask=0`, `--host-param blur_mocha_mask=0`, `--set shift_mocha_mask=0,-0`.

**Parameters.** A parameter is offered when a counted member set it through the route. Every other parameter of the style stays at its registered default, which `fx_run` applies, and a stage passes on only the parameters it was given. A value is passed through unchanged (a vector is `a,b` or `r,g,b`). `fx_params.tsv` holds only the id, the `fx_run` option form and the name of each offered parameter, taken from the manifests; the type, lanes, range and registered default are asked from `fx_run --describe-params`, which prints them from the native definitions, when the stage builds its list.

**Two inputs that `fx_run` reveals by refusing or defaults.** `--do-postdiv` is a required cross-check on the 'ae32' route, and `fx_run` derives its value from the owner arm and the opacity parameters. The stage sends 0, and when `fx_run` answers with the line `POSTDIV-MISMATCH: the owner arm ... derives ctx = N, the recorded cell is M` it reads N (an exact pattern, covered by a unit test), repeats the frame and keeps N for every later frame of the clip. A line it cannot read is `stage_failed` carrying that line. `--owner-opacity` is omitted: `fx_run` derives it. The stage passes `--ns-default-unbound` on every run: an unbound normalize_scale parameter then takes its registered default from the native definitions instead of the `PARAM-UNITS` refusal (without the flag `fx_run` is unchanged).

**Layers.** Distort_RGB offers an optional `Lens` layer, Drop_Shadow optional `Background` and `Matte` layers. A layer file holds one frame (held for the whole clip) or at least as many frames as the input; any other length is exit 4. No counted member used a layer input, so layers are INFERRED.

**Temporal stages.** `time_warp_rgb_autogen` keeps a bounded window of the largest positive plus the largest negative channel shift plus 1 frames. Before frame 0 the route clamps (`max(0, ...)`); past the last input frame the stage holds the last frame (a stage choice with no evidence, INFERRED). `trails_autogen` is `spool`: it buffers the clip and runs the measured history route once, with `--host-times 1..N` and the render index as the frame (render k is clip frame k-1 at time k). That time base starting at 1 is a per style host choice, measured for two frame histories and INFERRED beyond them.

**Host curves.** `transform_and_blur_autogen` and `blur_motion_curves_no_transform_autogen` read a host curve file. Pass its path as `--param host_curve_values=PATH`. Without it the stage writes a constant identity curve (rotate 0, shift 0, z_dist 1, centre 50 by 50) in the recorded format, with the samples the frame needs. MEASURED: 2 of the 13 in profile members of each style have a recorded curve file equal to that text byte for byte at frame 0; other curve values and later frames are INFERRED.

**Exits.** An unknown or out of range parameter is exit 3. A refusal of `fx_run` (its exit 2, for example `PARAM-UNITS-NOT-JUDGED`) is exit 3 carrying the refusal; an `fx_run` exit 1 is `stage_failed`. A frame narrower than 2 pixels is exit 3 (the host grid needs two columns). Each stage therefore refuses the two 1x1 cases of `tools/conformance.py` with exit 3; these are documented size refusals (never padded), and every other case of L1 and L3 passes for all 13 stages. A partial input frame is exit 4 and empty input is exit 0.

**The claim.**

| Case | Claim |
|---|---|
| Depth 32, the sizes and parameter values the in profile counted members used | byte identical to the frozen predictions (MEASURED by `tests/fx_stage_parity.rs`, full SHA-256) |
| Other sizes and values | INFERRED (the same code path) |
| Buffer placement at sizes other than the measured ones | INFERRED |
| Trails clips longer than the measured histories, Time_Warp windows beyond the measured shifts | INFERRED |
| Layers, host curves other than the identity curve | INFERRED |
| Depths 8 and 16 | new conversion, not claimed |

**Limits.** Each frame is one `fx_run` process, so the stages are slow. MEASURED (wall time per frame including process start, under shared load on a 12 core server, `out/TIMING.tsv`): 0.2 to 0.35 s at 160x96 for every stage; at 1920x1080 0.9 to 6.8 s per frame for most stages, `Blur_Motion_Curves_No_Transform_Autogen` about 12 s and `Transform_and_Blur_Autogen` about 54 s per frame. An in process route is a later optimisation and needs its own parity run.

Members that are not in profile (a different host arrangement, a fractional frame, a `--prev` file given outside the history chain) are counted in the parity record and are not claimed for the stage.


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
