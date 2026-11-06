# Using the effects

This page shows how to run the ten native effects on a video or on raw frames, how to find their parameters, and what the results are claimed to match. The effects themselves are described in [effects](effects.md); the stage interface they use is specified in [stages](stages.md).

Every command below was run from the repository root on a one second 160x96 test clip, and each ended with exit code 0.

## Setup

The effect stages are linked into `sapphire`; no other executable is needed. The `fx_run` command line is built from the same library and also takes `--device cpu|gpu|auto`.

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
```

ffmpeg and ffprobe must be on `PATH` ([finding ffmpeg](ffmpeg.md#finding-ffmpeg)). Linux is the measured platform: `fx_run` is built and tested in CI on Linux only, and its results on macOS and Windows have not been measured.

A short test clip to try the commands on:

```sh
ffmpeg -v error -f lavfi -i testsrc=size=160x96:rate=24 -t 1 -pix_fmt yuv420p clip.mp4
```

## The stages

Each registered style of an effect is one stage. `sapphire stage list` prints them; the stage id is the form below without the `@`, as `sapphire stage apply` takes it:

| `--stage` form | Effect | Kind |
|---|---|---|
| `@blur_color_autogen`, `@blur_mono_autogen` | `S_Blur` | frame |
| `@transform_and_blur_autogen`, `@blur_motion_curves_no_transform_autogen` | `BlurMoCurves` | frame |
| `@diffuse_autogen` | `S_Diffuse` | frame |
| `@distort_rgb_autogen` | `S_DistortRGB` | frame |
| `@drop_shadow` | `S_DropShadow` | frame |
| `@hue_sat_bright_autogen` | `S_HueSatBright` | frame |
| `@sharpen_best_quality_autogen`, `@sharpen_fast_autogen` | `S_Sharpen` | frame |
| `@time_warp_rgb_autogen` | `S_TimeWarpRGB` | temporal |
| `@trails_autogen` | `S_Trails` | temporal |
| `@warp_perspective_autogen` | `S_WarpPerspective` | frame |

```sh
target/release/sapphire stage list
```

A frame stage turns each input frame into one output frame. A temporal stage also reads other frames of the clip: `time_warp_rgb_autogen` shifts colour channels in time, and `trails_autogen` reads the whole clip before it writes.

## Running a stage on a clip

The `sapphire` pipeline decodes the clip with ffmpeg, passes the frames through the stages, encodes the result and verifies it before it moves it to the output path. `--stage "@ID K=V ..."` runs a stage of this repository by its id, with parameters as `K=V` pairs:

```sh
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen hue_shift=0.25" -o clip_hue.mp4
```

- `--depth 32` passes 32-bit float frames to the stage. It is the render path the parity record covers ([What is claimed](#what-is-claimed)).
- `-o` names the output. Without it the output is named after the input; `--force` replaces an existing file once the new output verifies.
- `--dry-run` prints the plan and every command and writes nothing:

```sh
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen hue_shift=0.25 saturation=1.5" --dry-run
```

A parameter left out takes its default. A parameter can also change over the clip with keyframes, `VALUE@FRAME;VALUE@FRAME`, frames counted from 0 on the output; quote the stage string, because `;` ends a shell command:

```sh
target/release/sapphire clip.mp4 --depth 32 --stage "@hue_sat_bright_autogen saturation=0@0;2@24e" -o clip_sat.mp4
```

Keys may hold (`h`), ease (`e`) or carry Bezier handles, as the frames/2 convention defines them; [Driving the effect parameters](parameters-guide.md#keyframed-values) has the rules. `trails_autogen` takes no keyed value. The output is encoded with libx264; `--lossless` (ffv1) is available at depths 8 and 16 only.

### Slow stages and `--stage-timeout 0`

Each frame of an effect stage is one `fx_run` process, so the stages are slow at large sizes ([Speed](#speed)). The pipeline stops a process that moves no data for 120 seconds, which a slow stage can reach on a single large frame. `--stage-timeout 0` turns that limit off:

```sh
target/release/sapphire clip.mp4 --depth 32 --stage-timeout 0 --stage "@transform_and_blur_autogen" -o clip_tb.mp4
```

## Chaining stages

Repeat `--stage` to run several stages in order. Effect stages mix freely with the other stages of this repository, such as the ffmpeg filter stage:

```sh
target/release/sapphire clip.mp4 --depth 32 --stage-timeout 0 --stage "@hue_sat_bright_autogen hue_shift=0.25" --stage "@sharpen_fast_autogen" -o clip_chain.mp4
target/release/sapphire clip.mp4 --depth 32 --stage-timeout 0 --stage "@diffuse_autogen" --stage "@ffmpeg-filter graph=hflip" -o clip_mixed.mp4
```

The claim of each effect covers that stage alone; a chain is the stages run in sequence on each other's output.

## Running a stage on raw frames

`sapphire stage apply ID` reads whole raw frames on standard input and writes the same number of frames, of the same size and format, on standard output. At depth 32 the format is ffmpeg's `gbrapf32le` ([raw frame format](../README.md#raw-frame-format)), so ffmpeg reads and writes it directly:

```sh
ffmpeg -v error -i clip.mp4 -f rawvideo -pix_fmt gbrapf32le clip.f32
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --fps 24 --param hue_shift=0.25 < clip.f32 > hue.f32
```

Parameters are `--param K=V`, repeatable. Stages chain with a shell pipe, and ffmpeg encodes the result:

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 160 --height 96 --depth 32 --fps 24 --param hue_shift=0.25 < clip.f32 | target/release/sapphire stage apply sharpen_fast_autogen --width 160 --height 96 --depth 32 --fps 24 > chain.f32
ffmpeg -v error -f rawvideo -pix_fmt gbrapf32le -video_size 160x96 -framerate 24 -i chain.f32 -c:v libx264 -pix_fmt yuv420p clip_raw.mp4
```

A stage refuses a frame size it cannot render, such as 1x1, with exit code 3 (`unsupported`) and a message; the exit codes are in [Exit codes](../README.md#exit-codes).

## Running the stages from twixtor

twixtor's `tools/apply.py` runs any frames/2 stage before its own effect with `--pre-stage CMD`, or after it with `--post-stage CMD` ([running in a wrapper](stages.md#running-in-a-wrapper)). A wrapper fills the placeholders `{width}`, `{height}`, `{depth}` and `{fps}` in the command. From a twixtor checkout, with `SAPPHIRE` set to the absolute path of `target/release/sapphire`:

```sh
python3 -I tools/apply.py clip.mp4 retime --param speed_percent=50 --depth 32 --contract frames/2 -o clip_twixtor.mp4 --pre-stage "$SAPPHIRE stage apply hue_sat_bright_autogen --width {width} --height {height} --depth {depth} --fps {fps} --param hue_shift=0.25"
```

`--contract frames/2` gives the error object and the exit codes of the shared convention. `python3 -I tools/handoff.py stages --json`, run in this repository, prints every stage with a ready command line for a wrapper. The `sapphire` pipeline can also run twixtor's effect itself, between the same stages ([Handoff](../README.md#handoff)).

## Finding a stage's parameters

`sapphire stage list --json` lists every stage with its parameters: the id to pass as `K=V` or `--param K=V`, and the kind (`float`, `checkbox` or `text`). A parameter with several lanes, such as a 2D shift or a colour, takes its values comma separated, for example `shift=0.1,0.2`.

```sh
target/release/sapphire stage list --json
```

The registered default and range of every parameter of an effect program come from `fx_run` itself:

```sh
target/release/fx_run --describe-params Hue_Sat_Bright_Autogen
```

The argument is the style's program name, the `name` field of the stage in `stage list --json`. The output is JSON: per parameter its name, type, lanes, minimum, maximum and default. These values are part of the effects and are never copied into the documentation; ask `fx_run` for them.

Two styles of `BlurMoCurves` read a host curve file, passed as `--param host_curve_values=PATH`; without it they use an identity curve. The `S_DistortRGB` and `S_DropShadow` stages take optional extra layers with `--layer ID=PATH` ([effects provider](stages.md#effects-provider)).

Popup parameters take the zero-based index of the item, so the first item is `0`. The opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`) are the exception: they take a host answer, a one-based item number. Of the opacity choices only Normal (`2`) renders on the stages' host arrangement; All_Opaque and As_Premult are refused with exit 3 (MEASURED). `cset_output_opacity` of `time_warp_rgb_autogen` renders both of its items.

See [Driving the effect parameters](parameters-guide.md) for every stage parameter, how to write its value and the values verified against recorded renders.

## Speed

The stages render in process. Wall time of one frame, MEASURED under shared load on a 12 core server, best of two runs ([effects provider](stages.md#effects-provider)):

| Frame size | Time per frame |
|---|---|
| 1920x1080, seven stages | 0.6 to 1.8 s |
| 1920x1080, five stages (including both BlurMoCurves stages) | 3 to 4.4 s |
| 1920x1080, `distort_rgb_autogen` | about 8.7 s |

Start with a short, small clip.

## What is claimed

The ten effects are rebuilt so that, on every case the parity record compares, they give the original's output bit for bit ([what rebuilt means](effects.md#what-rebuilt-means)). For the stages, in plain words:

- **MEASURED.** At depth 32, with the frame sizes and parameter values of the recorded renders that use each stage's host arrangement, the stage output is byte identical to the recorded native render: 382 of 382, checked by full SHA-256 ([Effects and stages](../README.md#effects-and-stages)).
- **INFERRED.** Other frame sizes and parameter values, a real clip among them, run the same code path and are expected to match, but were not measured. The same holds for buffer placement at unmeasured sizes, `S_Trails` clips longer than the measured histories, `S_TimeWarpRGB` shifts beyond the measured ones, extra layers, and host curves other than the identity curve.
- **Not claimed.** Depths 8 and 16 add a conversion to and from float that is not part of the record. Recorded renders with a different host arrangement are not claimed for the stage.
- **Platform.** Every comparison was judged by an emulator that follows Intel's specification, not by real CPU silicon, and the native code reproduces the original's AVX2 and FMA code tier only.
- **Host.** The stages render the effect, not a full host application: values between keyframes, masks, real host frame fetches and GPU rendering are outside the claim ([limits common to the ten effects](effects.md#limits-common-to-the-ten-effects)).

The limits of each effect are in [effects](effects.md), and the stage limits in [effects provider](stages.md#effects-provider).
