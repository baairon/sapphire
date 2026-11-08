# Command-line reference

This page is the exact interface of the `sapphire` executable and of `fx_run --describe-params`, for a program or an AI agent that drives them. Every flag, environment variable, exit code and refusal label below is read from the code; every command in a `sh` block was run on a small clip and exited with the code the block states. For a walk through the effects see the [guide](guide.md); for what each stage parameter means see [Driving the effect parameters](parameters-guide.md).

Commands are written from the repository root after `cargo build --release -p neutral-handoff -p fx_exec`, which builds `target/release/sapphire` and `target/release/fx_run`. `ffmpeg` and `ffprobe` must be on `PATH` (see [Environment variables](#environment-variables)). A line that ends in `# rc N` exits with code N; a line without it exits with 0.

## Entry points

| Command | Reads | Writes | Use |
|---|---|---|---|
| `sapphire INPUT [EFFECT] [flags]` | a video file | a video file, a plan or a JSON result on standard output | decode with ffmpeg, run the stages in order, encode and verify |
| `sapphire stage list [--json]` | nothing | the stage table or the stage manifest on standard output | discover stage ids, kinds and parameters |
| `sapphire stage apply STAGE [flags]` | raw frames on standard input or `--in` | the same number of raw frames on standard output or `--out` | run one stage over raw frames |
| `fx_run --describe-params PROGRAM...` | nothing | one JSON object on standard output | the type and lanes of every parameter of an effect program |

`sapphire --help`, `sapphire stage --help` and `sapphire stage apply --help` print the usage text and exit 0. `sapphire --version` prints `sapphire` and the crate version.

## Quick start

The first block makes a 160x96, 24 frame test clip and the same clip as raw 32 bit float frames; the other blocks of this page use these files.

```sh
ffmpeg -v error -y -f lavfi -i testsrc=size=160x96:rate=24:duration=1 -pix_fmt yuv420p clip.mp4
ffmpeg -v error -y -i clip.mp4 -f rawvideo -pix_fmt gbrapf32le clip.f32
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen hue_shift=0.25" -o clip_hue.mp4 --force
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --fps 24 --param hue_shift=0.25 --in clip.f32 --out hue.f32
```

The third command renders the effect stage inside a pipeline and writes `clip_hue.mp4`. The fourth renders the same stage over raw frames. Both run in the `sapphire` process; no `fx_run` executable is needed to render.

## `sapphire INPUT [EFFECT_ID] [flags]`

The pipeline decodes `INPUT` to raw frames, runs the chain, and encodes. The chain is, in order, the optional `EFFECT_ID` (run by twixtor's `vfx apply`) and every `--stage`. Without an `EFFECT_ID` the chain is the stages alone; with neither the command is a usage error (`give an EFFECT or at least one --stage`, exit 2).

| Argument | Default | Meaning |
|---|---|---|
| `INPUT` | required | the video file; a missing file is exit 4 |
| `EFFECT_ID` | none | the `vfx` effect id; run first unless a `--stage` equal to `vfx` marks where it goes |
| `--stage CMD` | none | one stage, repeatable, run in the order given. `CMD` is split like a shell command and the placeholders `{width}`, `{height}`, `{depth}`, `{fps}`, `{frames}` and `{pix_fmt}` are replaced. `@ID [K=V ...]` runs the stage `ID` of this tool; any other word is a program run as a raw frame filter |
| `--geometry G` | `preserve` | after a `--stage` (frames/2 only): `preserve` or `declared` (the stage answers `--describe` with its output size); any other value is exit 3 |
| `--stage-timeout S` | 120 | kill a decode, stage or encode process that moved no data for S seconds; `0` turns the limit off. `vfx` is not judged. A kill is exit 5 |
| `--depth 8\|16\|32` | 16 | bits per channel between ffmpeg and the stages: `rgba`, `rgba64le` or `gbrapf32le`. Depth 32 is the depth of the parity record |
| `--lossless` | off | encode with ffv1 in RGB (`.mkv`) instead of libx264 CRF 18; depth 8 or 16 only (depth 32 is exit 3) |
| `-o`, `--output PATH` | `<input>_<effect>.mp4`, or `<input>_stages.mp4` without an effect (`.mkv` with `--lossless`) | the output path; an existing path is exit 4 unless `--force`; the input path is exit 4 |
| `--force` | off | replace an existing `-o` path once the new output verifies |
| `--dry-run` | off | print the plan and the commands, write nothing |
| `--json` | off | print the result, or the error object, as JSON on standard output |
| `--describe` | off | frames/2 only: print the output geometry of the whole chain as one JSON line and stop |
| `--contract frames/1\|frames/2` | `frames/2` | the exit codes and report keys spoken (see [Exit codes](#exit-codes)) |
| `--param K=V` | none | a parameter of the `EFFECT_ID` (repeatable). It does not reach a `@ID` stage: write those inside the `--stage` string |
| `--layer ID=VIDEO` | none | an extra input clip of a temporal `EFFECT_ID` (repeatable) |
| `--frames N` | none | output length in frames of a temporal `EFFECT_ID` |
| `--vfx PATH` | found by [`VFX_PATH`](#environment-variables) and the build folders | the `vfx` executable |
| `--ffmpeg-path PATH`, `--ffprobe-path PATH` | found by [environment](#environment-variables) | the ffmpeg and ffprobe executables; a path that does not exist ends the run with 6 |
| `-V`, `--version` | | print the version |
| `-h`, `--help` | | print the usage text |

`--flag value` and `--flag=value` are both read. A stage string holding `;` (keyframes) or spaces must be quoted for the shell.

An `@ID` stage with an effect id runs as a child process, `sapphire stage apply ID --width W --height H --depth D --fps N/D --param ...`, of the program named by [`SAPPHIRE_PATH`](#environment-variables) (default: the running executable). The child inherits the environment, and the pipeline has no `--device` flag: choose the device with `SAPPHIRE_DEVICE` ([Rendering on a device](#rendering-on-a-device)). The child's standard error is not shown on a successful run; with `--json` its last 5 lines are in `stages[].stderr_tail`, and on a failure they are in the error object. The child gets no `--frames` and no `--layer`, so a [transition stage](#transition-stages) given as `@ID` (`@swishpan`) refuses with `FRAMES-REQUIRED` (exit 3); run it with `stage apply`.

```sh
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen hue_shift=0.25" -o v1.mp4 --dry-run
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen" --describe
target/release/sapphire clip.mp4 --depth 32 --lossless --stage "@hue_sat_bright_autogen" -o v2.mkv --force # rc 3
target/release/sapphire clip.mp4 --depth 32 --stage "@blur_color_autogen cset_opacity=4" -o v3.mp4 --force --json # rc 3
target/release/sapphire clip.mp4 --depth 32 --stage "@blur_color_autogen cset_opacity=4" -o v3.mp4 --force --contract frames/1 # rc 1
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen" -o clip.mp4 # rc 4
target/release/sapphire clip.mp4 # rc 2
```

## `sapphire stage list`

```text
sapphire stage list [--json] [--all] [--contract frames/1|frames/2]
```

Without `--json` it prints a table with one row per stage and the columns `id`, `name`, `family`, `kind`, `depths`, `params` (a count) and `layers` (a count). With `--json` it prints the stage manifest (`contracts/stage-manifest.schema.json`): `contract`, `provider`, `tool`, `version`, `depths`, `pix_fmts` and `stages`. `--all` is read and has no effect (every stage is always listed). An unknown option is exit 2. `--contract` changes only the `contract` field.

Each stage has `id`, `name`, `family`, `kind`, `spool`, `geometry`, `hidden`, `depths`, `params` and `layers`. The `id` is what `stage apply` and `@ID` take. `name` is the program name that `fx_run --describe-params` takes. `hidden` is always `false`: the 18 stages (`selftest-invert`, `ffmpeg-filter` and the 16 effect stages) are linked into the executable.

```sh
target/release/sapphire stage list
target/release/sapphire stage list --json
target/release/sapphire stage list --bogus # rc 2
```

## `sapphire stage apply`

```text
sapphire stage apply STAGE --width W --height H --depth 8|16|32 [--fps N[/D]] [--frames N]
                     [--param K=V]... [--layer ID=PATH]... [--in FILE] [--out FILE]
                     [--check] [--describe] [--ffmpeg-path PATH] [--json]
                     [--contract frames/1|frames/2] [--device cpu|gpu|gpu=N|auto]
```

The stage reads whole raw frames and writes the same number of frames of the same size and format. The format is set by `--depth`: 8 is `rgba` (4 bytes a pixel), 16 is `rgba64le` (8 bytes) and 32 is `gbrapf32le` (planar G, B, R, A as 32 bit floats, 16 bytes a pixel). Alpha is straight. A frame is `width x height x bytes per pixel` bytes, and a frame of more than 4 GiB is exit 2.

| Argument | Default | Meaning |
|---|---|---|
| `STAGE` | required | a stage id from `stage list`; a missing id is exit 2, an unknown id exit 3 |
| `--width W`, `--height H` | required | the frame size, positive integers; the effect stages need a width of 2 or more |
| `--depth 8\|16\|32` | required | the frame format. Depths 8 and 16 convert through float and are not in the parity record |
| `--fps N[/D]` | none | the frame rate, such as `24`, `29.97` or `30000/1001`. Required when any `--param` is keyed (exit 2 otherwise); `--describe` reports `25/1` when it is absent. An effect stage also derives the `get_time` value of each frame from it ([Host values](#host-values)); `flicker` reads that value, and without `--fps` it is refused with `HOST-VALUE-MISSING` (exit 3) |
| `--frames N` | none | the clip length. Every stage checks the frame count: a different count is exit 4 after the frames were written. A [transition stage](#transition-stages) requires it, with N of 2 or more, and derives its time values from it. `0` or a word is exit 2 |
| `--param K=V` | none | a stage parameter (repeatable); an id given twice is exit 2 |
| `--layer ID=PATH` | none | the second clip of a [transition stage](#transition-stages): `To=PATH`, or the layer id the stage declares. Refused on every other effect stage with `LAYER-INPUT-UNSUPPORTED` (exit 3); on a `frame` stage it is a usage error (exit 2) |
| `--in FILE` | standard input | read the frames from a file; `-` is standard input. A terminal on standard input is exit 2 |
| `--out FILE` | standard output | write the frames to a file; `-` is standard output. A terminal on standard output is exit 2; the same file as `--in` is exit 2 |
| `--check` | off | check the arguments and exit 0 without reading frames |
| `--describe` | off | read no input and print the output geometry as one JSON line (frames/2 only) |
| `--ffmpeg-path PATH` | found by [environment](#environment-variables) | used only by `ffmpeg-filter` |
| `--device D` | `cpu` | `cpu`, `gpu`, `gpu=N` (device N of the probe order, decimal digits only) or `auto`; see [Rendering on a device](#rendering-on-a-device) |
| `--json` | off | print a failure as the JSON error object on standard error (standard output carries frames) |
| `--contract frames/1\|frames/2` | `frames/2` | `frames/2` for the stage exit codes and the `--describe` answer. Both contracts exit with the stage codes of [Exit codes](#exit-codes) |
| `-h`, `--help` | | print the usage text |

On a failure the stage prints one line `error: <code>: <message>` on standard error, or with `--json` one JSON object. The message of a frame failure starts with `frame N:`. Output frames already written stay written; there is no rollback.

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out o1.f32 --check
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --describe
target/release/sapphire stage apply nope --width 160 --height 96 --depth 32 --in clip.f32 --out o2.f32 --json # rc 3
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --frames 5 --in clip.f32 --out o3.f32 # rc 4
target/release/sapphire stage apply hue_sat_bright_autogen --width 1 --height 96 --depth 32 --in clip.f32 --out o4.f32 # rc 3
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 12 --in clip.f32 --out o5.f32 # rc 2
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --in clip.f32 --out clip.f32 # rc 2
```

## `fx_run --describe-params`

```text
fx_run --describe-params PROGRAM...
```

Prints one JSON object with a member per program, `{"PROGRAM":[{"name":..,"type":..,"lanes":..,"min":..,"max":..,"default":..}]}`, and exits 0. The `type` is `scalar`, `boolean`, `popup`, `int`, `color`, `vector` or `angle`; `default` is a text with one value per lane separated by spaces. An unknown program gives an empty list (`"NAME":[]`) and exit 0. Arguments that start with `--` or equal `-` are skipped. The program name is the `name` of the stage in `stage list`. Building `fx_run` is optional: rendering needs only `sapphire`, and `stage list --json` already prints the defaults and ranges.

```sh
target/release/fx_run --describe-params Hue_Sat_Bright_Autogen
target/release/fx_run --describe-params Hue_Sat_Bright_Autogen Blur_Color_Autogen
target/release/fx_run --describe-params No_Such_Program
```

Any other `fx_run` argument list is the internal runner interface that `sapphire` drives itself; an incomplete one prints a usage line and exits 2. `fx_run` also takes `--device cpu|gpu|gpu=N|auto`; any other value prints `--device takes cpu, gpu or auto` and exits 2.

`fx_run --host-value NAME=VALUE` (repeatable, once per NAME, with a finite VALUE) gives the renderer one host value: NAME is `get_percent_time`, `get_total_frames`, `get_aspect_ratio` or `get_time`, and VALUE is a decimal number or `0x` followed by the f32 bits in hex. Any other name, or a value that does not parse, prints `--host-value takes get_percent_time|get_total_frames|get_aspect_ratio|get_time=VALUE` and exits 2. A NAME given a second time prints `--host-value: refused: NAME is given twice` and exits 2. A VALUE that is not finite (`nan`, `inf`, `-inf`, a decimal above the f32 range such as `1e40`, which reads as infinity, or the bits of an f32 NaN or infinity such as `0x7fc00000`) prints `--host-value: refused: NAME is not finite (nan, inf and -inf are refused; a decimal above the f32 range reads as inf)` and exits 2. Both refusals come while the options are read, and no output file is written. The renderer takes these four values only from `--host-value`: a program that reads one of them is given no default in its place. `sapphire stage apply` passes them itself, as f32 bits ([Host values](#host-values)).

`fx_run --plan-text PATH` writes the plan the run executes to the file PATH: the compiled command list in the plan print form, then one tab separated `NODE` line per compiled node (index, kind, name, primitive, element count, arguments, constant flag, folded flag and the four lane words in hex). It changes no output. The plan is written once the run has succeeded, to a temporary file in the folder of PATH that is then renamed to PATH, so a run that is refused or fails writes no plan and leaves a file already at PATH unchanged. Before it renders anything the run checks that the folder takes a new file, by creating and removing a probe file next to PATH; that check neither creates nor truncates PATH. `--plan-text` with no path (last on the line, or followed by another option) prints `--plan-text: refused: the option needs a file path` and exits 2, and a folder or path that cannot be written prints `--plan-text: refused: cannot write PATH: REASON` and exits 2, at the check or after the run. PATH is not compared with the input paths, so give a path that is not an input.

`fx_run --xy-base N` gives the base phase N (0 to 31) of the buffer that the writer of the default xy fills, a host input of each run; `--xy-base-list B,B,...` gives one base per default-xy writer of the placement model, and its first entry stands for `--xy-base` when that is absent. On the 'ae32' route a run needs the base only when the compiled plan runs a reader of the default xy: the `xy` special node or a `perspective` primitive that is not folded or constant and is reached from the plan root through the arguments of nodes that run. Such a run without a base refuses with `XY-BASE-UNGIVEN` and exit 2, naming the plan node. The check follows the plan compile, so the argument checks before the compile (such as `PERSP-BASE-UNGIVEN` and `UNKNOWN-PARAM-NAME`) and a compile failure (exit 1) come first, and a plan whose xy reader is pruned at compile time or not reached runs without a base. `PERSP-BASE-UNGIVEN` still looks at every node of the program before the compile: a `perspective` or `perspective_alpha` node anywhere in it needs `--persp-base` or `--palpha-base`. `sapphire stage apply` passes the base from the placement column of each stage whose plan reads the default xy ([Effects provider](stages.md#effects-provider)).

## Built-in stages

Two stages are not effects. `selftest-invert` inverts the three colour samples of every pixel (`max - v`; `1.0 - v` at depth 32) and the alpha too with `--param invert_alpha=1`. `ffmpeg-filter` runs an ffmpeg filter graph over the frames: `--param graph=G` (default `null`), `--param expect=preserve|declared` (whether the graph leaves the frame size alone or the stage declares a new one through `--describe`) and `--param unsafe=1` for graphs that open files. Both are `frame` stages at depths 8, 16 and 32.

```sh
target/release/sapphire stage apply selftest-invert --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out inv.f32
target/release/sapphire stage apply ffmpeg-filter --width 160 --height 96 --depth 32 --fps 24 --param graph=hflip --in clip.f32 --out flip.f32
target/release/sapphire clip.mp4 --depth 32 --stage "@diffuse_autogen" --stage "@ffmpeg-filter graph=hflip" -o chain.mp4 --force
```

## Stage kinds, spool and keyframes

`stage list` gives every stage a `kind`:

| Kind | Stages | Meaning |
|---|---|---|
| `frame` | `selftest-invert`, `ffmpeg-filter` | output frame N depends on input frame N only; a parameter cannot be keyed |
| `temporal` | the 16 effect stages | the frames/2 convention allows a keyed parameter; output frame N still depends on input frame N (on `swishpan` also on frame N of its second clip, the clip length and the rate), except for `time_warp_rgb_autogen` (it reads the frames its channel shifts name) and `trails_autogen` (it reads the frames before it) |

`spool: true` (only `trails_autogen`) means the stage reads the whole input before it writes the first frame. Its memory grows with the clip: MEASURED about 33 bytes a pixel for every frame (about 492 KiB a frame at 160x96, from the peak memory of runs of 6, 12 and 24 frames), so a 1080p clip of 100 frames needs about 6.8 GB (INFERRED from that rate). `trails_autogen` renders the clip in one run, so a keyed value is exit 3.

A keyed value is `V@F;V@F;...` on the output frame number F counted from 0 (a fraction is allowed), with `--fps` as its time base:

- `V@Fh` holds the value until the next key and `V@Fe` eases it; not both.
- `V@F` may carry Bezier handles `iS/I` and `oS/I` (speed in units a second, influence 0.1 to 100 percent) before the `h` or `e`.
- Before the first key the value is the first key's, after the last key the last one's. Keys may come in any order; two keys on one frame are exit 3.
- Floats, angles and two-lane points interpolate; integers, popups, checkboxes, colours and text hold until the next key. A host value (the ids that take a frame-width unit, see [Preview](#recipe-a-low-resolution-preview)) is put on the 1/65536 grid, rounded half away from zero.
- Keyed values between keys are INFERRED to match the original host; the stage is MEASURED to give the same bytes at frame N as a plain run at the value the keys give for frame N.

```sh
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --param "blur_amount=1@0;2@10e" --in clip.f32 --out key.f32
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --param "blur_amount=1@0;2@10" --in clip.f32 --out key2.f32 # rc 2
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --param "blur_amount=1@0;2@0" --in clip.f32 --out key3.f32 # rc 3
target/release/sapphire stage apply trails_autogen --width 160 --height 96 --depth 32 --fps 24 --param "blur_amount=1@0;2@10" --in clip.f32 --out key4.f32 # rc 3
target/release/sapphire stage apply selftest-invert --width 160 --height 96 --depth 32 --fps 24 --param "invert_alpha=1@0" --in clip.f32 --out key5.f32 # rc 3
```

## Transition stages

A transition stage renders a change from the input clip to a second clip over the length of the clip. It is an effect stage whose `layers` in `stage list --json` is not empty (a `layers` count above 0 in `stage list`); each layer has an `id` and an `optional` flag. `swishpan` is the transition stage of this build (`stage list` shows 1 layer for it and 0 for the other effect stages); its layer `To` is optional. The refusals of this section are also checked by unit tests on a stand-in transition profile.

A transition stage takes two inputs besides the frames:

- `--layer To=PATH`: the second clip. `To` names the first layer the stage declares, and that layer's own `id` is accepted as well. The file holds raw frames in the format of `--depth` at the `--width` and `--height` of the input: either one frame, held for every input frame, or one frame per input frame, layer frame f going with input frame f. A layer of more than one frame that ends before the input is exit 4 at the first frame it lacks, after the earlier frames were written. PATH must be a regular file: the stage takes the layer's size from the file system and reads each frame at its offset, so a named pipe or a process substitution such as `<(ffmpeg ...)` reports 0 bytes and is refused with `LAYER-SIZE-MISMATCH`. A layer whose `optional` is `true` may be left out.
- `--frames N`: the clip length, N of 2 or more. It sets the time values of every frame ([Host values](#host-values)), and the input must hold N frames: a different count is exit 4 after the frames were written.

Output frame f depends on input frame f, the layer frame that goes with it, the parameters, N and the `--fps` rate. The stage checks `--frames` before the layer, and both before any frame is read: a refusal writes no frame. `--check` and `--describe` check neither input. The refusals are exit 3: `FRAMES-REQUIRED` and `FRAMES-TOO-SMALL` for the frame count, and `LAYER-ID-UNKNOWN`, `LAYER-DUPLICATE`, `LAYER-FILE-MISSING`, `LAYER-SIZE-MISMATCH` and `LAYER-INPUT-MISSING` for the layer ([Refusal labels](#refusal-labels)). `--layer` on an effect stage that is not a transition stage is `LAYER-INPUT-UNSUPPORTED`.

```text
ffmpeg -v error -y -f lavfi -i testsrc2=size=160x96:rate=24:duration=1 -f rawvideo -pix_fmt gbrapf32le to.f32
target/release/sapphire stage apply swishpan --width 160 --height 96 --depth 32 --fps 24 --frames 24 --layer To=to.f32 --in clip.f32 --out mix.f32
target/release/sapphire stage apply swishpan --width 160 --height 96 --depth 32 --fps 24 --layer To=to.f32 --in clip.f32 --out mix.f32 # rc 3
target/release/sapphire stage apply swishpan --width 160 --height 96 --depth 32 --fps 24 --frames 1 --layer To=to.f32 --in clip.f32 --out mix.f32 # rc 3
target/release/sapphire stage apply swishpan --width 160 --height 96 --depth 32 --fps 24 --frames 24 --in clip.f32 --out solo.f32
```

The first line writes a second clip of 24 frames at 160x96 and depth 32 (5,898,240 bytes). The second renders the transition from `clip.f32` of the [Quick start](#quick-start) to it over 24 frames: frame f is rendered with `get_percent_time` f32(f) / f32(23), from 0 on the first frame to 1 on the last. The third leaves out `--frames` (`FRAMES-REQUIRED`), the fourth gives a clip length of 1 (`FRAMES-TOO-SMALL`), and the fifth leaves out the layer, which `swishpan` allows because its layer is optional: the effect then runs with no second clip. A stage whose layer is not optional refuses that line with `LAYER-INPUT-MISSING`.

### Host values

`sapphire stage apply` renders each frame of an effect stage with these host values, passed to the renderer as `--host-value NAME=0x...` with the f32 bits. Here f is the output frame number counted from 0, N the `--frames` value and num/den the `--fps` rate as a reduced fraction (`24` is 24/1, `29.97` is 2997/100).

| Host value | Passed | Value at frame f |
|---|---|---|
| `get_time` | when `--fps` is given | the time in seconds as the original computes it for a host with time scale num and local time step den: F = f32(f x den) / f32(den), rate = f32(num) / f32(den), get_time = F / rate, each an f32 division (a rate below 0.0001 is replaced by 30). At `--fps 24` it is f32(f) / f32(24) |
| `get_percent_time` | on a transition stage | f32(f) / f32(N - 1), one correctly rounded f32 division: 0 on the first frame, 1 on the last |
| `get_total_frames` | on a transition stage | f32(N) |
| `get_aspect_ratio` | on a transition stage | f32(H) / f32(W), the frame height over its width, one correctly rounded f32 division: 0.5 at 64x32 |

`trails_autogen` renders its clip in one run of its history route and passes none of these values. The stages of the first ten effects render the same bytes with `get_time` as without it (MEASURED on their stage parity renders, see [effects provider](stages.md#effects-provider)). `flicker` reads `get_time` on every frame; `swishpan` reads `get_percent_time`, `get_total_frames` and `get_aspect_ratio`. Without `--fps` no `get_time` is passed, and a stage whose program reads it is refused with `HOST-VALUE-MISSING` (exit 3) before any frame is written. A rate or a frame whose time fields do not fit the host's 32-bit fields is refused with `HOST-TIME-RANGE` (exit 3). The rate is checked before any frame is read; the tick f x den of frame f is checked when frame f is rendered, so the frames before it are already written when it is refused. The time values describe a transition that spans exactly the clip the stage is given, from its first frame to its last; a second clip that starts at another time, or a clip cut from a longer transition, is not modelled.

## Parameter discovery

Ask `stage list --json` first. It prints every stage parameter with its `id`, `name`, `kind`, `default` and, for numbers, `min` and `max`; a `popup` also has `choices`. The kinds:

| `kind` | Value to pass | Checked |
|---|---|---|
| `float` | a decimal number | finite and inside `min` to `max`, else exit 3 |
| `int` | a whole number | inside `min` to `max`, else exit 3. A popup given as a record value is an `int` (zero-based item index). A `cset_*` id is a host answer (one-based): its `default` is the answer the style registers, its `min` is 1 and its `max` is the item count of the popup, and an answer outside them is exit 3 before any frame is read |
| `checkbox` | `1`, `0`, `true`, `false`, `on` or `off` | any other word is exit 3 |
| `text` | free text, or lane values separated by commas with no spaces (`0.5,1.5`) | the lane count of a vector is checked, else exit 3 |
| `popup` | one of `choices` | only `ffmpeg-filter` has one |

The stage offers only the parameters that a recorded render set; every other parameter of the program stays at its registered default. `fx_run --describe-params` adds what `stage list` does not print: the `type` (scalar, boolean, popup, int, color, vector or angle) and the `lanes` of a text parameter. It does not describe a `cset_*` id: the default and range of an opacity answer come from the style's native parameter definitions, and `stage list --json` prints them. Defaults and ranges are read from these commands and are not copied into the documentation.

The two commands give the same default values with one exception: for a one-lane popup, `stage list` prints the default an unset run takes, the item the style registers as a zero-based index, and `fx_run --describe-params` prints the program text default. Of the offered parameters only `auto_trans` of `swishpan` has two different values (MEASURED over every stage): `fx_run --describe-params SwishPan` prints `0`, `stage list` prints `1`, and an unset `auto_trans` renders the same bytes as `auto_trans=1`, not as `auto_trans=0` (MEASURED). For an id whose route is `--host-param` (the host-unit ids of [Preview](#recipe-a-low-resolution-preview)), both commands print the program's record value as the `default`, not a value in the host units that `--param` takes: an unset parameter renders at the host value its style registers, converted at the frame width, so the printed `default` passed as a value gives another render (MEASURED on `width` of `glow` and `blur_amount` of `swishpan`). Leave such a parameter unset to render its default.

```sh
target/release/sapphire stage list --json | python3 -I -c "import json,sys; d=json.load(sys.stdin); s=[x for x in d['stages'] if x['id']=='hue_sat_bright_autogen'][0]; print(s['kind'], [(p['id'], p['kind']) for p in s['params']])"
target/release/fx_run --describe-params Hue_Sat_Bright_Autogen
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --param nope=1 --in clip.f32 --out p1.f32 # rc 3
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --param hue_shift=abc --in clip.f32 --out p2.f32 # rc 3
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --param hue_shift=9 --in clip.f32 --out p3.f32 # rc 3
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --param hue_shift=0.1 --param hue_shift=0.2 --in clip.f32 --out p4.f32 # rc 2
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --param cset_opacity=9 --in clip.f32 --out p5.f32 # rc 3
```

## Environment variables

The renderer reads these variables. A value is compared as written: `1` means the one character `1`, and a variable that is set but not a recognised value is ignored without a message unless the row says otherwise.

| Variable | Read by | Value | Effect and precedence |
|---|---|---|---|
| `SAPPHIRE_DEVICE` | the effect renderer | `cpu`, `gpu`, `gpu=N`, `auto` | picks the device when no device was chosen on the command line: unset, `cpu`, or an unparsable value render on the CPU. `--device cpu` does not override it; `--device gpu`, `gpu=N` and `auto` do. The pipeline has no `--device`, so this is how a pipeline stage uses a device |
| `SAPPHIRE_DEVICE_REPORT` | the effect renderer | `1` | print `device: ` lines on standard error: the device opened with the result of its self-check, then after every frame the nodes run on the device and the fallbacks; also names the reason when `auto` ends on the CPU |
| `SAPPHIRE_REQUIRE_DEVICE` | the effect renderer | `1` | a request for a device (from `--device` or `SAPPHIRE_DEVICE`) that installs none refuses the render with `DEVICE-REQUIRED-UNAVAILABLE` instead of falling back to the CPU. A run that asks for `cpu` or nothing is not affected |
| `SAPPHIRE_THREADS` | the effect renderer | a positive integer | worker threads of the parallel kernels; `1` is serial. `0`, a negative number or text means all available cores. The output bytes do not depend on it |
| `SAPPHIRE_PAR_MIN` | the effect renderer | a non-negative integer | the smallest frame, in samples, that uses more than one thread; default 65536. `0` makes every frame with 2 rows or more parallel. The output bytes do not depend on it |
| `SAPPHIRE_CACHE_DIR` | the device path | a folder | replaces the folder of the device caches. Default: `$XDG_CACHE_HOME/sapphire` (an absolute path only) or `~/.cache/sapphire` on Linux, `~/Library/Caches/sapphire` on macOS, `%LOCALAPPDATA%\sapphire` on Windows, else the temporary folder plus `sapphire`. The value is used as given (no `sapphire` is added) |
| `SAPPHIRE_DEVICE_CHECK_CACHE` | the device path | `0` | `0` turns the self-check cache off, so every self-check runs in every process; anything else leaves it on |
| `SAPPHIRE_DEVICE_PROGRAM_CACHE` | the device path | `0` | `0` turns the compiled-program cache (`opencl-programs` under the cache folder) off, so each process builds the OpenCL programs from source; anything else leaves it on |
| `SAPPHIRE_PATH` | the pipeline | a program path | the program that runs `@ID` stages as children; default: the running executable. An empty value means the default |
| `MEDIA_FFMPEG_PATH`, `FFMPEG` | the pipeline | a path or program name | the ffmpeg to use: `--ffmpeg-path`, then `MEDIA_FFMPEG_PATH`, then `FFMPEG` (the first non-empty one decides alone and a missing file is exit 6), then `PATH`, then on Windows the WinGet link and `ProgramFiles` folders. `stage apply` of `ffmpeg-filter` reads `--ffmpeg-path`, then `FFMPEG`, then `PATH` |
| `MEDIA_FFPROBE_PATH`, `FFPROBE` | the pipeline | a path or program name | the same order for ffprobe |
| `VFX_PATH` | the pipeline | a file | the `vfx` executable when `--vfx` is absent; a missing file is exit 6. Then `$CARGO_TARGET_DIR/release`, the folder of `sapphire`, the `target/release` folders of twixtor checkouts beside this one, then `PATH` |
| `HANDOFF_TWIXTOR_DIR` | the pipeline | a folder | the twixtor checkout searched for `vfx`, instead of the siblings of this repository; ignored when `--vfx` or `VFX_PATH` is given |
| `CARGO_TARGET_DIR` | the pipeline | a folder | its `release` subfolder is searched for `vfx` |
| `PATH`, `PATHEXT` | the pipeline | the usual | program lookup for ffmpeg, ffprobe, `vfx` and stage programs (`PATHEXT` on Windows) |
| `LOCALAPPDATA`, `ProgramFiles`, `ProgramFiles(x86)` | the pipeline, Windows | folders | the places searched for ffmpeg and ffprobe when nothing else names them; `LOCALAPPDATA` is also the cache folder |
| `XDG_CACHE_HOME`, `HOME` | the device path | folders (`XDG_CACHE_HOME` only when absolute) | the Linux and macOS cache folder, see `SAPPHIRE_CACHE_DIR` |

### Fenced switches

The effect renderer refuses to run when any of 40 diagnostic switches is set in the environment, so a stray variable cannot change a render silently. The refusal is `ENV-SWITCH-REFUSED: NAME is set; the 'ae32' route runs with no environment switch` (exit 3 through `stage apply`). Unset the named variable. The names are:

`DSW5_CLONE`, `DSW5_CLONE_SCOPE`, `NV10_BW_WHOLE`, `DS1_OFF`, `NV9_BLUR_SCALE_OLD`, `CV_MUT`, `COV_MUT`, `DS2_PHASE`, `DS_MUT`, `FX_SETS_ORIG_NUMBER`, `LG_TRACE`, `NV3B_HC`, `NV3D_MODE`, `TW2_LOGFRAMES`, `DS2_XY`, `FX1_COERCE_FOLD_OLD`, `FX1_COERCE_EXTRA_OFF`, `SHE4_OFF`, `DSW3_TRACE`, `WPG1_PERSP_WHOLE`, `INT1_CONVERT`, `FX1_TRACE`, `CV_MUTANT`, `WPG1_PBASE`, `WPG1_ABASE`, `WPG1_S8`, `WPG1_S48`, `WPG1_S30`, `WPG12_OLDFMT`, `DSW3_UNBOUND_OFF`, `FLD1_NO_COPIES`, `FLD1_NO_INTTYPE`, `NEG2_RANGES_OFF`, `LYM2_REPORT`, `NV3E_DONOR`, `NV3E_DS`, `NV3E_FLAG`, `NV3E_INPLACE`, `NV3E_LASTC`, `NV3E_SLOT`.

`SAPPHIRE_FX_RUN` is named by a constant in the code and read only by a test; it has no effect on `sapphire`.

```sh
SAPPHIRE_THREADS=1 target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out t1.f32
SAPPHIRE_THREADS=4 SAPPHIRE_PAR_MIN=0 target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out t4.f32
cmp t1.f32 t4.f32
WPG1_PBASE=1 target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out t5.f32 # rc 3
```

## Rendering on a device

`--device` is a `stage apply` flag. The pipeline route has none, so a pipeline stage takes its device from `SAPPHIRE_DEVICE`, which its `stage apply` child inherits. The choice is made once per process, in this order:

1. `--device gpu`, `--device gpu=N` or `--device auto` on the command line decides, even when the device is then refused (the render is on the CPU and one `device: <reason>; rendering on the CPU` line is printed). `SAPPHIRE_DEVICE` is not read.
2. With no `--device`, or with `--device cpu`, `SAPPHIRE_DEVICE` decides: `gpu`, `gpu=N` or `auto` selects that device; `cpu`, unset or an unparsable value changes nothing.

| Value | Meaning |
|---|---|
| `cpu` | the reference path, no OpenCL library loaded |
| `gpu` | the first usable OpenCL device of any type (a CPU runtime such as PoCL counts) |
| `gpu=N` | device number N of the probe order |
| `auto` | the first usable GPU-type device, else the CPU (a CPU runtime does not count, so `auto` renders on the CPU on a machine that has only PoCL) |

A device is used only after the probe and a self-check; a node or a run that the device cannot serve exactly runs on the CPU, and the bytes are the same either way. The first run of a build on a device pays the self-check (MEASURED 12.4 s for 24 frames of 160x96, against 0.14 s on the CPU); later processes read it from the cache. Whether a device was used is confirmed with `SAPPHIRE_DEVICE_REPORT=1`: the first `device: ` line names the device, and each later line counts the nodes run on it, for example `device: blur 2, postdivide 2; fallback: none`. A stage with no device kernel reports `device: none`.

```sh
SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --device gpu --in clip.f32 --out dev.f32
SAPPHIRE_DEVICE=gpu SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --device cpu --in clip.f32 --out dev2.f32
cmp dev.f32 dev2.f32
SAPPHIRE_DEVICE=gpu SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire clip.mp4 --depth 32 --stage "@blur_color_autogen" -o dev.mp4 --force --json | python3 -I -c "import json,sys; d=json.load(sys.stdin); print([s['stderr_tail'][-1] for s in d['stages'] if s['name']=='pre1'])"
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --device gpu=99 --in clip.f32 --out dev3.f32
OCL_ICD_VENDORS=/nonexistent SAPPHIRE_REQUIRE_DEVICE=1 target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --device gpu --in clip.f32 --out dev4.f32 # rc 3
```

In a pipeline the report is in the result: the last line of the `pre1` step's `stderr_tail` is the running count of the last frame. In a check or CI job set `SAPPHIRE_REQUIRE_DEVICE=1`, so a missing device ends the run instead of passing on the CPU. The last command shows the refusal; `OCL_ICD_VENDORS` is the OpenCL loader's own variable and here hides every device.

## Exit codes

The exit code depends on the entry point.

`sapphire INPUT ...` under `--contract frames/2` (the default):

| Exit | Name | Meaning |
|---|---|---|
| 0 | | done |
| 1 | `stage_failed` | a stage exited with another non-zero code, returned more frames than it was given, or the output did not verify |
| 2 | `usage` | a flag or argument the parser rejects |
| 3 | `unsupported` | an unknown effect or stage, a bad parameter value, HDR input, `--lossless` at depth 32, an odd side on the lossy path, or a stage that exits 3 |
| 4 | `bad_input` | no such input, no video stream, an output path that exists (without `--force`) or equals the input, or a stage that exits 4 |
| 4 | `broken_pipe` | a stage closed its input or output before the frames were through |
| 4 | `partial_frame` | a stream ended in the middle of a frame |
| 5 | `stage_timeout` | a process moved no data for `--stage-timeout` seconds and was killed |
| 6 | `tool_missing` | ffmpeg, ffprobe, `vfx` or a stage program was not found or would not start |

The exit of a failing `@ID` stage child is passed through its code: 2 is `usage`, 3 `unsupported`, 4 `bad_input`, anything else `stage_failed` (exit 1). Under `--contract frames/1` every failure is exit 1, except a command line the parser rejects, which is exit 2.

Other entry points:

| Entry point | Exits |
|---|---|
| `sapphire stage apply` | 0 done; 1 `stage_failed` (a renderer failure, or a stage process that fails); 2 `usage` (also a keyed value without `--fps`, a terminal on the frame streams, a missing `--width`, `--height`, `--depth` or stage id, `--frames 0`, `--layer` on a `frame` stage); 3 `unsupported` (an unknown stage or parameter, `--layer` on an effect stage that is not a transition stage, a transition stage without a usable `--frames` or layer, a bad value, a width under 2, a stage table row whose `host_variants` column is refused, a refusal of the renderer); 4 `bad_input` and `partial_frame` and `broken_pipe` (a missing or short file, `--frames` that does not match, a layer shorter than the input); 6 `tool_missing` (`ffmpeg-filter` without ffmpeg). The same codes under both contracts. 5 is not produced |
| `sapphire stage list` | 0; 2 for an unknown option or a bad `--contract` value |
| `fx_run --describe-params` | 0 |
| `fx_run` (other arguments) | 0 done; 1 the execution failed; 2 a usage error or a refusal (a label on standard error). It never exits 3: `sapphire` turns the renderer's exit 2 into exit 3 |

A refusal of the renderer is exit 3 through `sapphire` and exit 2 through `fx_run`; a keyed value without `--fps` is exit 2 through `sapphire` and never reaches `fx_run`. The error line is `error: <code>: <message>` on standard error. With `--json` the pipeline prints `{"contract","ok":false,"error":{"code","stage","exit_code","message","stderr_tail"}}` on standard output, where `exit_code` is the exit of the failing child and `stderr_tail` holds at most 5 lines of its standard error (`contracts/error.schema.json`); `stage apply` prints the same object on standard error.

## Refusal labels

A label is an upper case word of two or more parts joined by hyphens that names one refusal. It appears in the message (`frame N: fx_run refused: <label> (...)`), in the child's `stderr_tail` and in the JSON `message`. A stage refusal is exit 3 through `sapphire` and exit 2 through `fx_run`. Status MEASURED means the row was reproduced by a command on this build; CODE means it is defined in the code and was not reproduced.

| Label | Entry point | Exit | Status | Meaning | What to change |
|---|---|---|---|---|---|
| `DEVICE-REQUIRED-UNAVAILABLE` | `stage apply`, `fx_run`, pipeline | 3 (`fx_run` 2) | MEASURED | `SAPPHIRE_REQUIRE_DEVICE=1` and a requested device was not installed; a `device: ` line gives the reason | install or select a device, or unset `SAPPHIRE_REQUIRE_DEVICE` |
| `ENV-SWITCH-REFUSED` | `stage apply`, `fx_run`, pipeline | 3 (`fx_run` 2) | MEASURED | one of the 40 fenced switches is set in the environment | unset the variable the message names |
| `HOST-VALUE-OFF-GRID` | `stage apply`, `fx_run` | 3 (`fx_run` 2) | MEASURED | a host-unit value is not a multiple of 1/65536 on a parameter whose host slider holds a 16.16 value: every `--host-param` id but `width` of `glow`, `blur_mocha_mask` of `diffuse_autogen` and `shift_host` of the `S_DropShadow` stage, which take an off-grid value as given (MEASURED with `4.00001`). On `trails_autogen` the message reads `render 1 failed with rc 2` and the label is in the standard error tail | round it to the grid (see [Preview](#recipe-a-low-resolution-preview)) |
| `POPUP-ANSWER-OUT-OF-RANGE` | `fx_run` | 2 | MEASURED | an opacity answer (`--cset DISPLAY=N`) is outside 1 to the item count of its popup. `stage apply` and the pipeline check the same range first and refuse the value with `bad value for` (exit 3) before any frame is read, so a stage run does not reach this label | pass `1` to the count; only `2` renders on most stages |
| `SOURCE-VARIANT-MISMATCH` | `stage apply`, `fx_run` | 3 (`fx_run` 2) | MEASURED | `cset_opacity=1` or `3` on a stage whose `host_variants` column has no entry for that answer (every effect stage but `glow` and `swishpan`, which have entries for 1 and 3): the host arrangement of the stage does not cover that opacity choice | use `cset_opacity=2` or leave it unset |
| `LAYER-INPUT-UNSUPPORTED` | `stage apply` | 3 | MEASURED | `--layer` was given to an effect stage that is not a [transition stage](#transition-stages) (any id, any path). The refusal comes before any frame is read, with empty standard output | drop `--layer`; only transition stages take a second layer |
| `HOST-VALUE-MISSING` | `stage apply`, `fx_run` | 3 (`fx_run` 2) | MEASURED | the compiled plan runs a host getter (`get_time`, `get_percent_time`, `get_total_frames` or `get_aspect_ratio`) that no value was given for: `stage apply` without `--fps` on `flicker`, or `fx_run` without `--host-value NAME=VALUE`. The message names the program, the plan node and the getter. The pipeline always passes `--fps` | pass `--fps` with the rate of the clip, or `--host-value NAME=VALUE` to `fx_run` |
| `FRAMES-REQUIRED` | `stage apply` | 3 | CODE | a transition stage was run without `--frames N`; its time values need the clip length | pass the clip length as `--frames N` |
| `FRAMES-TOO-SMALL` | `stage apply` | 3 | CODE | `--frames 1` on a transition stage: the time runs from the first frame to the last, which needs N of 2 or more (`--frames 0` is exit 2 from the argument parser) | give a clip of 2 frames or more and its length |
| `LAYER-ID-UNKNOWN` | `stage apply` | 3 | CODE | the id before `=` in `--layer` is neither `To` nor a layer the transition stage declares | use `To`, or an `id` from the stage's `layers` |
| `HOST-TIME-RANGE` | `stage apply` | 3 | MEASURED | the time scale num or the local step den does not fit the host's 32-bit time fields, checked before any frame is read, or the tick f x den of frame f does not, checked when frame f is rendered: the frames before f are then already written (`--fps 1/2147483647` writes 2 frames of a 3 frame clip, then exits 3 at frame 2) | use a rate with a smaller numerator or denominator, or a shorter clip |
| `SIMPLEX-GRADIENT-TABLE-ABSENT` | `fx_run` | 2 | CODE | the compiled plan runs simplex_noise (Glow with atmosphere_amp other than 0), whose gradient tables no documented formula reproduces | render Glow with atmosphere_amp 0 |
| `FRAME-SIZE-OVER-I32` | `fx_run` | 2 | CODE | the plane length W x H of `--size WxH` is above 2147483647, the largest sample count the kernels take in 32 bits; refused for any program before any input file is read, and no output file is written. The kernels of `overlay_super` and `screen` (plane length) and `glow` (frame width) refuse the same counts with this label when the library is called directly. Through `stage apply` such a frame would be an `fx_run` refusal, exit 3 (INFERRED: a frame of that size was not run) | render at a frame size of at most 2147483647 pixels |
| `LAYER-DUPLICATE` | `stage apply` | 3 | CODE | one layer was given twice, also as `To` and as its declared id | give each layer once |
| `LAYER-FILE-MISSING` | `stage apply` | 3 | CODE | the layer file cannot be opened; the message carries the system error | correct the path |
| `LAYER-SIZE-MISMATCH` | `stage apply` | 3 | MEASURED | the layer file is empty or not a whole number of frames of the input's `--width`, `--height` and `--depth`; the message gives the file size and the frame size. A named pipe, or any path that is not a regular file, reports 0 bytes and is refused here | write the layer at the size and depth of the input, to a regular file |
| `LAYER-INPUT-MISSING` | `stage apply` | 3 | CODE | a transition stage whose layer is not optional was run without `--layer` | add `--layer To=PATH` |
| `TICKMAP-POINT-HOST-INPUT-UNSOURCED` | `stage apply`, `fx_run` | 3 (`fx_run` 2) | MEASURED | the file given as `host_curve_values` is not a complete host curve file (an empty file gives it, with `TICKMAP-HOST-FIELD-UNSOURCED` inside) | omit `host_curve_values` to use the identity curve, or pass a full curve file |
| `HOST-CURVE-SAMPLE-UNSERVED` | `stage apply`, pipeline | 3 | CODE | a host curve sample the render reads is not in the given curve file and the identity curve cannot serve it (it serves `rotate`, `shift` and `z_dist` on the two `BlurMoCurves` stages and `wipe_amt` on `swishpan`, and never a sample of a parameter the stage offers) | add the sample to the file, or use the identity curve |
| `HOST-CURVE-VALUE-MISSING` | `fx_run` | 1 or 2 | CODE | `fx_run` names one missing curve sample; `sapphire` adds an identity sample and repeats the frame (a bounded number of times), and ends with `HOST-CURVE-SAMPLE-UNSERVED` if it cannot | none when `sapphire` is the caller |
| `XY-BASE-LIST-MISMATCH` | `fx_run` | 1 | CODE | the buffer placement model has N writers, the list gives M; `sapphire` reads N from the message and repeats the frame | none when `sapphire` is the caller |
| `XY-BASE-LIST-UNSERVED` | `stage apply`, pipeline | 3 | CODE | the placement `fx_run` asks for cannot be served: the measured list holds unequal base phases or an `r1` entry, so one phase cannot be given to N writers, or the count could not be read | render at a frame size of the recorded sizes. Not reproduced on 91 runs of 13 stages at 7 sizes from 2x2 to 321x181 |
| `XY-BASE-UNGIVEN` | `fx_run` | 2 | CODE | the compiled plan runs a reader of the default xy (the `xy` special node or a `perspective` primitive, not folded and reached from the plan root) and neither `--xy-base` nor `--xy-base-list` was given; the message names the plan node. The check follows the plan compile: a plan whose reader is pruned or not reached needs no base ([`fx_run`](#fx_run---describe-params)) | pass `--xy-base` with the base phase of the run; the stages pass it from their placement column |
| `HOST-VARIANT-MALFORMED` | `stage apply`, pipeline | 3 | CODE | the stage's `host_variants` column does not parse: an entry has no `:` after `PARAM_ID=VALUE`, a key that is not `PARAM_ID=VALUE` or has an empty id or value, no option, an item that does not start with `--`, or an option with no value (the form is `PARAM_ID=VALUE:--opt value\|--opt value`, entries separated by `;`) | correct the row of `crates/neutral-handoff/data/fx_stages.tsv` |
| `HOST-VARIANT-PARAMS` | `stage apply`, pipeline | 3 | CODE | the entries of one row name more than one selecting parameter | key every entry of the row by the same parameter |
| `HOST-VARIANT-PARAM-UNKNOWN` | `stage apply`, pipeline | 3 | CODE | the selecting parameter is not an offered parameter of the stage (it has no row in `fx_params.tsv`) | use a parameter id the stage offers |
| `HOST-VARIANT-DUPLICATE` | `stage apply`, pipeline | 3 | CODE | two entries of one row have the same value, compared as numbers when both are numbers (`1` and `1.0` are one value) | keep one entry per value |
| `HOST-VARIANT-AT-DEFAULT` | `stage apply`, pipeline | 3 | CODE | an entry is keyed by the registered default of its parameter: the default `fx_run --describe-params` gives or, for an opacity answer (`cset_*`), the host answer its style registers as the default (`2` for `cset_opacity` and `cset_input_opacity`, `1` for `cset_output_opacity`); the `host` column serves the default | remove the entry |
| `HOST-VARIANT-OPTION` | `stage apply`, pipeline | 3 | CODE | an entry carries an option the runner derives (`--do-postdiv`, `--owner-opacity`) or the stage sets per run (`--size`, `--out`, `--ae32-src`, `--ae32-prior`, `--ae32-out`, `--geom`, `--host-frame`, `--ctx-time`, `--host-times`, `--host-value`, `--ns-default-unbound`, `--frame-words`), one a parameter gives (`--cset`, `--set`, `--host-param`, `--host-curve-values`), one fixed per style (`--prior-fill`), or an option of the row's `fixed` column | remove the option from the entry |
| `HOST-VARIANT-PLACEMENT` | `stage apply`, pipeline | 3 | CODE | an entry carries an option that the row's `placement` column sets at any measured size | keep the option in the placement column |
| `POSTDIV-MISMATCH` | `fx_run` | 2 | CODE | the cross-check value `do_postdiv` differs from the one the opacity arm derives; `sapphire` reads the derived value from the message and repeats the frame. If it cannot read it, the run is `stage_failed` (exit 1) | none when `sapphire` is the caller |
| `PARAM-UNITS-NOT-JUDGED` | `fx_run` | 2 | CODE | a host-unit parameter has a normalize scale the renderer has not been judged for | give the parameter as a record value, or leave it at its default |

The `HOST-VARIANT-*` labels refuse a fault of the built-in stage table, before any frame is read; the `host_variants` column and its rules are in [Effects provider](stages.md#effects-provider). No row of this build is refused, and each label is checked by a unit test.

Other `fx_run` refusals use the same shape (an upper case label, then a bracket with the detail) and come from the checks of the recorded render route; the stages always pass the values those checks need, so one appears only for an unusual combination. Read it as `error: unsupported: frame N: fx_run refused: LABEL (...)`: exit 3, the stage cannot render this combination, and the fix is to change the parameter or the frame size named in the detail or to leave that parameter at its default. A refusal that ends with a message instead of a label is listed below.

| Message starts with | Exit | Meaning | What to change |
|---|---|---|---|
| `unknown stage` | 3 | the id is not in `stage list` | use a listed id |
| `unknown param` | 3 | the id is not offered by this stage | use an id of `stage list --json` |
| `bad value for` | 3 | a value of the wrong kind, outside its range (an opacity answer outside 1 to the item count of its popup too), a wrong lane count, or a bad keyframe | see [Parameter discovery](#parameter-discovery) |
| `given more than once` | 2 | the same id twice | give each id once |
| `given both as --host-param and --set` | 3 | two ids reach one program parameter | give only one of the pair |
| `keyframes need --fps` | 2 | a keyed value and no `--fps` | add `--fps N[/D]` |
| `renders the whole clip in one fx_run run` | 3 | a keyed value for `trails_autogen` | give a plain value |
| `keyframes need a temporal effect` | 3 | a keyed value for a `frame` stage | give a plain value |
| `needs a width of 2 or more` | 3 | the effect stage needs two columns | use a width of 2 or more |
| `is a frame effect, --layer only apply to temporal effects` | 2 | `--layer` on a `frame` stage (`selftest-invert`, `ffmpeg-filter`) | drop `--layer` |
| `refusing to read raw frames from a terminal` | 2 | no `--in` and standard input is a terminal | pipe frames in or pass `--in` |
| `refusing to write raw frames to a terminal` | 2 | no `--out` and standard output is a terminal | pipe the frames out or pass `--out` |
| `are the same file` | 2 | `--out` is `--in` | use another output path |
| `input ends with a partial frame` | 4 | the input is not a whole number of frames | check `--width`, `--height` and `--depth` |
| `the input holds` | 4 | `--frames` differs from the input length | correct `--frames` |
| `expects a positive integer` | 2 | `--width`, `--height` or `--frames` is 0 or not a whole number | pass a positive integer |
| `frames but the input has more` | 4 | the layer of a transition stage holds more than one frame but fewer than the input | give one frame, or one frame per input frame |
| `missing --width` | 2 | a required flag is absent | pass it |
| `expects cpu, gpu, gpu=N or auto` | 2 | a bad `--device` value | use one of the four |

Layers: MEASURED on this build, `--layer` is refused for `Matte` and `Background` (`stage apply drop_shadow`) and `Lens` and `To` (`distort_rgb_autogen`) with `LAYER-INPUT-UNSUPPORTED` (exit 3). The one layer that renders is the second clip of `swishpan`, the transition stage of this build ([Transition stages](#transition-stages)).

```sh
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --param blur_amount=4.00001 --in clip.f32 --out l1.f32 # rc 3
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --param cset_opacity=4 --in clip.f32 --out l2.f32 # rc 3
target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --param cset_opacity=1 --in clip.f32 --out l3.f32 # rc 3
target/release/sapphire stage apply drop_shadow --width 160 --height 96 --depth 32 --fps 24 --layer Matte=clip.f32 --in clip.f32 --out l4.f32 # rc 3
target/release/sapphire stage apply drop_shadow --width 160 --height 96 --depth 32 --fps 24 --layer Background=clip.f32 --in clip.f32 --out l5.f32 # rc 3
target/release/sapphire stage apply transform_and_blur_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out l6.f32
printf '' > empty_curve.txt
target/release/sapphire stage apply transform_and_blur_autogen --width 160 --height 96 --depth 32 --fps 24 --param host_curve_values=empty_curve.txt --in clip.f32 --out l7.f32 # rc 3
```

## Recipe: masking and compositing around a stage

The effect stages have no mask input (the one layer a stage takes is the second clip of a [transition stage](#transition-stages)), so a mask is applied outside: render the full frame with the stage, then blend it over the original with a mask clip. `maskedmerge` takes the base, the overlay and the mask, and shows the overlay where the mask is white. The mask clip must have the same size, rate and frame count as the clip, and all three inputs are converted to the same planar format first (a YUV mask would also blend the chroma planes against grey).

```sh
target/release/sapphire clip.mp4 --depth 32 --stage "@blur_color_autogen blur_amount=4" -o fx.mp4 --force
ffmpeg -v error -y -f lavfi -i "color=c=black:s=160x96:r=24:d=1,drawbox=x=40:y=24:w=80:h=48:color=white:t=fill,gblur=sigma=2,format=yuv420p" -c:v libx264 -crf 12 mask.mp4
ffmpeg -v error -y -i clip.mp4 -i fx.mp4 -i mask.mp4 -filter_complex "[0:v]format=gbrp[a];[1:v]format=gbrp[b];[2:v]format=gbrp[m];[a][b][m]maskedmerge,format=yuv420p" -c:v libx264 -crf 18 masked.mp4
```

The result equals the original where the mask is black and the stage output where it is white (MEASURED on the test clip: PSNR 44.8 dB against the original and 26.1 dB against the stage output in a corner outside the box; 46.7 dB against the stage output and 21.9 dB against the original in the middle of the box). It is INFERRED to differ from a mask applied inside the effect at the edge of the mask for spatial effects (blur, glow, diffuse): the inside-the-effect mask would limit what is read as well as what is written, while here the blur has already spread colour across the mask edge in the full render. A mask that changes over time is a mask clip with the same frame count. To use a stage on part of the frame only, crop the clip, render the crop and overlay it back with ffmpeg in the same way; the stage then sees only the crop, which changes a spatial effect at the crop edge.

## Recipe: a low-resolution preview

Trim and scale the clip with ffmpeg, scale every host-unit value by preview width over full width, and keep the result on the 1/65536 grid. A host-unit value is one that scales with the frame width: the ids whose route is `--host-param` in `crates/neutral-handoff/data/fx_params.tsv` (`blur_amount` of the blur, trails and `swishpan` stages, `diffuse_amount`, `blur_mocha_mask`, `blur_lens`, `shadow_blur`, `shift_host`, `small_detail_size`, `sharpen_width` and `width` of `glow`). The renderer converts such a value as `record = host x (2 / W) x s`, so the same record value at width W' needs `host x W' / W`. Other parameters (ratios, angles, counts, checkboxes) are not scaled. An off-grid value is refused (`HOST-VALUE-OFF-GRID`, exit 3), except on `width` of `glow`, `blur_mocha_mask` and `shift_host`, which take it as given ([Refusal labels](#refusal-labels)).

```sh
ffmpeg -v error -y -f lavfi -i testsrc2=size=1920x1080:rate=24:duration=1 -pix_fmt yuv420p full.mp4
ffmpeg -v error -y -i full.mp4 -t 0.5 -vf scale=160:-2 preview.mp4
python3 -I -c "print(int(10*160/1920*65536+0.5)/65536)"
target/release/sapphire preview.mp4 --depth 32 --stage "@blur_color_autogen blur_amount=0.8333282470703125" -o preview_fx.mp4 --force
target/release/sapphire preview.mp4 --depth 32 --stage "@blur_color_autogen blur_amount=0.8333" -o preview_bad.mp4 --force # rc 3
target/release/sapphire full.mp4 --depth 32 --stage "@blur_color_autogen blur_amount=10" -o full_fx.mp4 --force
```

The python line prints `0.8333282470703125`, the value to pass at a preview width of 160 for `blur_amount=10` on a 1920 wide clip (`int(x * 65536 + 0.5)` rounds half up; values are positive). The preview approximates the downscaled full render; they are not byte equal (INFERRED). `ffmpeg -t` trims the preview to the first half second; a frame size of 160x90 or any even size works.

## Recipe: render time of a clip

Time is frames times seconds a frame at 1080p, scaled by pixels. The table is MEASURED on a 12 core server under some load (two runs per stage, from [Speed](guide.md#speed)): seconds for one 1920x1080 frame through `sapphire stage apply` on the CPU, and with `--device gpu` on PoCL (an OpenCL runtime on the same CPU, not GPU speed).

| Stage | CPU (s) | PoCL (s) |
|---|---|---|
| `blur_color_autogen` | 0.72 / 0.76 | 0.74 / 0.73 |
| `blur_mono_autogen` | 0.64 / 0.66 | 0.71 / 0.69 |
| `transform_and_blur_autogen` | 1.01 / 1.17 | 0.72 / 0.74 |
| `blur_motion_curves_no_transform_autogen` | 1.08 / 1.14 | 0.74 / 0.72 |
| `diffuse_autogen` | 1.90 / 2.34 | 0.72 / 0.69 |
| `distort_rgb_autogen` | 1.73 / 1.87 | 2.14 / 1.95 |
| `stage apply drop_shadow` | 1.52 / 1.73 | 0.72 / 0.71 |
| `hue_sat_bright_autogen` | 0.47 / 0.51 | 0.54 / 0.59 |
| `sharpen_best_quality_autogen` | 2.40 / 2.38 | 0.86 / 0.88 |
| `sharpen_fast_autogen` | 0.66 / 0.73 | 0.64 / 0.69 |
| `time_warp_rgb_autogen` | 0.78 / 0.73 | 0.88 / 0.81 |
| `trails_autogen` | 0.85 / 0.75 | 0.85 / 0.88 |
| `warp_perspective_autogen` | 1.37 / 1.31 | 1.44 / 1.46 |

Estimate `frames x seconds x (width x height / 2073600)`. A 10 second 1080p clip at 24 frames a second through `blur_color_autogen` is 240 frames x 0.76 s, about 3 minutes. The scaling by pixels is INFERRED; it was checked at one point: 24 frames of 160x96 predict 0.13 s for `blur_color_autogen` and 0.09 s for `hue_sat_bright_autogen`, and the measured walls were 0.14 s and 0.08 s. A pipeline adds the ffmpeg decode and encode, which run alongside the stage. The device adds the one-time self-check on its first use (see above). `trails_autogen` also holds the clip in memory (see [Stage kinds](#stage-kinds-spool-and-keyframes)): about 33 bytes a pixel for every frame, so budget memory for long 1080p clips before the time. Start with a short, small clip.

```sh
/usr/bin/time -f "wall=%e s maxrss=%M KB" target/release/sapphire stage apply blur_color_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out time1.f32
/usr/bin/time -f "wall=%e s maxrss=%M KB" target/release/sapphire stage apply trails_autogen --width 160 --height 96 --depth 32 --fps 24 --in clip.f32 --out time2.f32
```
