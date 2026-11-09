# Using the effects

This page shows how to run the fifteen native effects on a video or on raw frames, how to find their parameters, and what the results are claimed to match. The effects themselves are described in [effects](effects.md); the stage interface they use is specified in [stages](stages.md).

Every command below was run from the repository root on a one second 160x96 test clip, and each ended with exit code 0.

## Setup

The effect stages are linked into `sapphire`; no other executable is needed. The `fx_run` command line is built from the same library and takes `--device cpu|gpu|gpu=N|auto`. It is needed only for `fx_run --describe-params`. Every flag, environment variable, exit code and refusal is listed in the [Command-line reference](cli.md).

```sh
cargo build --release --offline -p neutral-handoff -p fx_exec
```

ffmpeg and ffprobe must be on `PATH` ([finding ffmpeg](ffmpeg.md#finding-ffmpeg)). Linux is the measured platform. CI builds `fx_run` and runs the effect golden hashes (`tools/fx_golden.py`) on Linux, macOS and Windows; the device checks on PoCL run on Linux only, and timing and the recorded-render comparison were measured on Linux only.

A short test clip to try the commands on:

```sh
ffmpeg -v error -f lavfi -i testsrc=size=160x96:rate=24 -t 1 -pix_fmt yuv420p clip.mp4
```

## The stages

Each registered style of an effect is one stage. `sapphire stage list` prints them; the stage id is the form below without the `@`, as `sapphire stage apply` takes it:

| `--stage` form | Effect | Kind |
|---|---|---|
| `@blur_color_autogen`, `@blur_mono_autogen` | `S_Blur` | temporal |
| `@transform_and_blur_autogen`, `@blur_motion_curves_no_transform_autogen` | `BlurMoCurves` | temporal |
| `@diffuse_autogen` | `S_Diffuse` | temporal |
| `stage apply dissolve_shake` only, see [Time and transitions](#time-and-transitions) | `S_DissolveShake` | temporal |
| `@distort_rgb_autogen` | `S_DistortRGB` | temporal |
| `@drop_shadow` | `S_DropShadow` | temporal |
| `@flicker` | `S_Flicker` | temporal |
| `@glow` | `S_Glow` | temporal |
| `@hue_sat_bright_autogen` | `S_HueSatBright` | temporal |
| `@shake_normal_autogen`, `@shake_twitchy_autogen`, `@shake_jumpy_autogen` | `Shake` | temporal |
| `@sharpen_best_quality_autogen`, `@sharpen_fast_autogen` | `S_Sharpen` | temporal |
| `swishpan` (with `stage apply` only, see [Time and transitions](#time-and-transitions)) | `S_SwishPan` | temporal |
| `@time_warp_rgb_autogen` | `S_TimeWarpRGB` | temporal |
| `@trails_autogen` | `S_Trails` | temporal |
| `@warp_perspective_autogen` | `S_WarpPerspective` | temporal |

```sh
target/release/sapphire stage list
```

All 20 stages have kind `temporal`, because the frames/2 convention accepts keyed values only for temporal stages. Only `time_warp_rgb_autogen` and `trails_autogen` read other frames of the clip: the first shifts colour channels in time, and the second reads the whole clip before it writes. The other 18 stages turn each input frame into one output frame and are `temporal` so that they accept keyframes; of them, `swishpan` and the `S_DissolveShake` stage also read the matching frame of a second clip. The kind `frame` belongs to `selftest-invert` and `ffmpeg-filter`.

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

Keys may hold (`h`), ease (`e`) or carry Bezier handles, as the frames/2 convention defines them; [Driving the effect parameters](parameters-guide.md#keyframed-values) has the rules. `trails_autogen` takes no keyed value; every other effect stage does. The output is encoded with libx264; `--lossless` (ffv1) is available at depths 8 and 16 only.

### Large frames and `--stage-timeout 0`

The effect stages render inside the `sapphire` process ([Speed](#speed) lists the time of one 1080p frame for each stage). The pipeline stops a stage that moves no data for 120 seconds, a limit a very large frame or a slow machine can reach. `--stage-timeout 0` turns that limit off:

```sh
target/release/sapphire clip.mp4 --depth 32 --stage-timeout 0 --stage "@transform_and_blur_autogen" -o clip_tb.mp4
```

### Rendering on a device

`--device` picks where the effect kernels run. It is a flag of `sapphire stage apply` and of `fx_run`; the pipeline route takes the device from `SAPPHIRE_DEVICE` (see Using a device from the pipeline, below).

| Value | Meaning |
|---|---|
| `cpu` | The reference path. This is the default; no OpenCL library is loaded. |
| `gpu` | The first usable OpenCL device, a GPU if there is one, else any usable device (a CPU runtime such as PoCL counts). |
| `gpu=N` | Device number N of the probe order. |
| `auto` | The first usable GPU, else the CPU. |

```sh
ffmpeg -v error -i clip.mp4 -f rawvideo -pix_fmt gbrapf32le clip.f32
target/release/sapphire stage apply sharpen_best_quality_autogen --device auto --width 160 --height 96 --depth 32 --fps 24 < clip.f32 > out.f32
```

The OpenCL library is opened at run time, so a machine without one still builds and renders on the CPU. A device is used only after two checks:

- **Probe.** The device must offer double precision floats, correctly rounded single precision divide and square root, denormals and round to nearest, the full OpenCL profile and OpenCL C 1.2. A device that fails is refused with the reasons; `gpu` and `gpu=N` then render on the CPU with a notice (below) and `auto` does so silently. Apple Silicon GPUs have no double precision, so the probe refuses them and a Mac renders on the CPU.
- **Self-check.** Before an operation runs on the device, its kernel is run on a set of adversarial inputs (subnormals, negative zero, infinities, NaN, exact ties, odd frame sizes) and compared bit for bit with the CPU kernel. An operation that differs is not used on the device.

The device output is the same bytes as the CPU output. An operation without a device kernel, one the kernel declines for a given shape or value, and any node whose output holds a NaN all run on the CPU for that node, so a partly supported plan still renders. `SAPPHIRE_DEVICE_REPORT=1` prints how many nodes of each operation ran on the device and how many fell back, with the reason.

**Using a device from the pipeline.** `sapphire INPUT --stage @ID` has no `--device` flag. It runs each effect stage as a `sapphire stage apply` child process that inherits the environment, so `SAPPHIRE_DEVICE` picks the device:

- `SAPPHIRE_DEVICE` takes the values of `--device` (`gpu`, `gpu=N`, `auto`). Unset, empty, `cpu` or a value that does not parse changes nothing and the run is on the CPU.
- The variable applies only when the run has no device installed. A `--device` other than `cpu` on `sapphire stage apply` installs its device first and wins over the variable. `--device cpu` installs nothing, so `SAPPHIRE_DEVICE=gpu` still switches a run that says `--device cpu` or leaves `--device` out. To force the CPU, unset the variable.
- The device is chosen once per rendering thread, by the first effect that executes.
- `SAPPHIRE_DEVICE_REPORT=1` and `SAPPHIRE_REQUIRE_DEVICE=1` work the same through the variable; a pipeline stage's report lines reach the standard error of the pipeline.

```sh
SAPPHIRE_DEVICE=gpu SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire clip.mp4 --depth 32 --stage "@blur_color_autogen" -o clip_dev.mp4
```

**Notices.** An explicit `--device gpu` or `--device gpu=N` that ends on the CPU (no OpenCL library, no usable device, no device number N, or a failed self-check of the whole device) prints one line on standard error, `device: <reason>; rendering on the CPU`, and renders on the CPU. The exit code is unchanged. `auto` prints nothing unless the report is on. `sapphire stage apply` forwards every `device: ` line to its standard error, on success and on failure.

**Confirming that a device was used.** `SAPPHIRE_DEVICE_REPORT=1` prints the device report on standard error, also through `sapphire stage apply`: the device chosen, the nodes that ran on the device by operation and the fallbacks with their reasons. A run that reports no nodes on the device rendered on the CPU.

```sh
SAPPHIRE_DEVICE_REPORT=1 target/release/sapphire stage apply blur_color_autogen --device gpu --width 160 --height 96 --depth 32 --fps 24 < clip.f32 > dev.f32
```

The first line names the device and the result of its self-check. After each frame a line follows with the running count of nodes per operation on the device and of the fallbacks, for example `device: blur 2, postdivide 2; fallback: none`. A stage whose operations have no device kernel, such as `hue_sat_bright_autogen`, reports `device: none`.

**Requiring a device.** `SAPPHIRE_REQUIRE_DEVICE=1` turns a missing device into an error: a `--device` other than `cpu` that installs no device refuses the run instead of rendering on the CPU: `sapphire stage apply` prints a `device: DEVICE-REQUIRED-UNAVAILABLE ...` line and exits with code 3 (`fx_run` exits with code 2). Use it in checks and CI, where a silent fallback would let a device test pass without a device.

**Caches.** The self-check costs about 9 s the first time a device and driver are used. The passes are then kept in the user cache folder (`$XDG_CACHE_HOME/sapphire` or `~/.cache/sapphire` on Linux, `~/Library/Caches/sapphire` on macOS, `%LOCALAPPDATA%\sapphire` on Windows), and a later process skips them (0.07 s). The entry is keyed on the build (a digest of the running executable, which holds the CPU kernels, the device glue and the corpus), the device name, the driver version, the digest of the kernel sources, the build options and the digest of the loaded program binary. A new build, a new driver, a changed kernel or a replaced program binary makes the entry unreadable and the checks run again. Compiled OpenCL programs are cached in `opencl-programs` under the same folder. Deleting the folder is always safe: both caches are rebuilt on the next run. `SAPPHIRE_DEVICE_CHECK_CACHE=0` turns the self-check cache off and `SAPPHIRE_CACHE_DIR` moves the folder.

**Platform notes.**

- Kernels with a double precision path (monochrome, satscale) may run slower on consumer GPUs, whose double precision rate is a fraction of the single precision rate. INFERRED.
- A long kernel can hit the Windows GPU watchdog (TDR), which resets the driver after a few seconds. INFERRED.
- Apple Silicon GPUs have no double precision and are refused by the probe.

A clip of a few frames can cost more on the device than on the CPU, because of the one time setup. Speedups on a real GPU are INFERRED until measured on one; the figures in [Speed](#speed) for a device are PoCL, an OpenCL runtime on the CPU, and show correctness, not GPU speed.

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

## Time and transitions

`--fps` gives each frame its time: frame f is at f / rate seconds ([Host values](cli.md#host-values)). `flicker` changes the brightness with the time, so pass the real rate of the clip; without `--fps` a `flicker` run is refused with `HOST-VALUE-MISSING` (exit 3). The verified renders use whole frames of a 24 frames a second clock.

```sh
target/release/sapphire stage apply flicker --width 160 --height 96 --depth 32 --fps 24 --param amplitude=0.6 --param wave_amp=0.5 --param wave_freq=2.5 < clip.f32 > flicker.f32
```

The three `Shake` stages also read the frame rate: the motion blur and the twitch look one frame ahead and one frame back. Pass the real rate of the clip; without `--fps` a `Shake` run is refused with `FPS-UNGIVEN` (exit 3). The verified renders use whole frames of a 24 frames a second clock as well.

```sh
target/release/sapphire stage apply shake_twitchy_autogen --width 160 --height 96 --depth 32 --fps 24 --param amplitude=1.4 --param stillness=0.35 --param twitch_frequency=4.5 < clip.f32 > shake.f32
```

`swishpan` is a transition from the input clip to a second clip. It takes the second clip as `--layer To=PATH`, raw frames of the same size and depth (one frame, held, or one frame per input frame), and the length of the transition as `--frames N`, the number of input frames: the move starts on the first frame and ends on the last. The layer is optional. The pipeline passes neither input, so `swishpan` runs with `sapphire stage apply` only ([Transition stages](cli.md#transition-stages)):

```sh
ffmpeg -v error -y -f lavfi -i testsrc2=size=160x96:rate=24:duration=1 -f rawvideo -pix_fmt gbrapf32le to.f32
target/release/sapphire stage apply swishpan --width 160 --height 96 --depth 32 --fps 24 --frames 24 --layer To=to.f32 --param direction=1 < clip.f32 > swish.f32
```

The `S_DissolveShake` stage is a transition of the same form: it dissolves from the input clip to the second clip while it shakes the picture, most strongly in the middle of the transition. It takes `--layer To=PATH`, `--frames N` and `--fps`, and runs with `sapphire stage apply` only:

```sh
target/release/sapphire stage apply dissolve_shake --width 160 --height 96 --depth 32 --fps 24 --frames 24 --layer To=to.f32 --param amplitude=1.25 --param frequency=7 < clip.f32 > dshake.f32
```

## Running the stages from twixtor

twixtor's `tools/apply.py` runs any frames/2 stage before its own effect with `--pre-stage CMD`, or after it with `--post-stage CMD` ([running in a wrapper](stages.md#running-in-a-wrapper)). A wrapper fills the placeholders `{width}`, `{height}`, `{depth}` and `{fps}` in the command. From a twixtor checkout, with `SAPPHIRE` set to the absolute path of `target/release/sapphire`:

```sh
python3 -I tools/apply.py clip.mp4 retime --param speed_percent=50 --depth 32 --contract frames/2 -o clip_twixtor.mp4 --pre-stage "$SAPPHIRE stage apply hue_sat_bright_autogen --width {width} --height {height} --depth {depth} --fps {fps} --param hue_shift=0.25"
```

`--contract frames/2` gives the error object and the exit codes of the shared convention. `python3 -I tools/handoff.py stages --json`, run in this repository, prints every stage with a ready command line for a wrapper. The `sapphire` pipeline can also run twixtor's effect itself, between the same stages ([Handoff](../README.md#handoff)).

## Finding a stage's parameters

`sapphire stage list --json` lists every stage with its parameters: the id to pass as `K=V` or `--param K=V`, and the kind (`float`, `int`, `popup`, `checkbox` or `text`). A parameter with several lanes, such as a 2D shift or a colour, takes its values comma separated, for example `shift=0.1,0.2`.

```sh
target/release/sapphire stage list --json
```

`stage list --json` carries the registered default and range of every parameter. The same values for an effect program come from `fx_run --describe-params`:

```sh
target/release/fx_run --describe-params Hue_Sat_Bright_Autogen
```

The argument is the style's program name, the `name` field of the stage in `stage list --json`. The output is JSON: per parameter its name, type, lanes, minimum, maximum and default. These values are part of the effects and are never copied into the documentation; `stage list --json` prints them first, and `fx_run --describe-params` prints the same values from a shell, apart from the default of the `auto_trans` popup of the two transition stages ([Parameter discovery](cli.md#parameter-discovery)).

Two styles of `BlurMoCurves` and `swishpan` read a host curve file, passed as `--param host_curve_values=PATH`; without it they use a constant curve. Only `swishpan` and the `S_DissolveShake` stage take an extra layer, the second clip of their transition; `--layer` on any other effect stage exits 3 with `LAYER-INPUT-UNSUPPORTED` ([effects provider](stages.md#effects-provider)).

Popup parameters take the zero-based index of the item, so the first item is `0`. The opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`) are the exception: they take a host answer, a one-based item number. On the stages of the first ten effects only Normal (`2`) of the opacity choices renders on the stages' host arrangement; All_Opaque and As_Premult are refused with exit 3 (MEASURED), and `cset_output_opacity` of `time_warp_rgb_autogen` renders both of its items. `glow`, `swishpan`, the three `Shake` stages and the `S_DissolveShake` stage render all three answers, each with the host arrangement of its own recorded renders.

See [Driving the effect parameters](parameters-guide.md) for every stage parameter, how to write its value and the values verified against recorded renders.

## Speed

Wall time of one 1920x1080 frame through `sapphire stage apply`, MEASURED on a 12 core server under some shared load (two runs per cell, in seconds, `before` and `after` on the same machine):

| stage | before the in-process route (s) | after, CPU (s) | after, PoCL device (s) |
|---|---|---|---|
| `blur_color_autogen` | 1.27 / 1.22 | 0.72 / 0.76 | 0.74 / 0.73 |
| `blur_mono_autogen` | 1.02 / 1.01 | 0.64 / 0.66 | 0.71 / 0.69 |
| `transform_and_blur_autogen` | 11.26 / 11.87 | 1.01 / 1.17 | 0.72 / 0.74 |
| `blur_motion_curves_no_transform_autogen` | 52.84 / 51.09 | 1.08 / 1.14 | 0.74 / 0.72 |
| `diffuse_autogen` | 3.02 / 2.90 | 1.90 / 2.34 | 0.72 / 0.69 |
| `distort_rgb_autogen` | 8.48 / 7.57 | 1.73 / 1.87 | 2.14 / 1.95 |
| `stage apply drop_shadow` | 1.84 / 1.74 | 1.52 / 1.73 | 0.72 / 0.71 |
| `hue_sat_bright_autogen` | 0.84 / 0.84 | 0.47 / 0.51 | 0.54 / 0.59 |
| `sharpen_best_quality_autogen` | 2.79 / 2.71 | 2.40 / 2.38 | 0.86 / 0.88 |
| `sharpen_fast_autogen` | 1.18 / 1.10 | 0.66 / 0.73 | 0.64 / 0.69 |
| `time_warp_rgb_autogen` | 1.44 / 1.31 | 0.78 / 0.73 | 0.88 / 0.81 |
| `trails_autogen` | 1.26 / 1.16 | 0.85 / 0.75 | 0.85 / 0.88 |
| `warp_perspective_autogen` | 3.87 / 3.49 | 1.37 / 1.31 | 1.44 / 1.46 |

- `before the in-process route` is the previous build: one `fx_run` process per frame and a full ripmap plane copy per row in the warp kernels.
- `after, CPU` is the default `--device cpu`, with the warp copies removed, the render in process and the parallel-safe kernels on all cores (`SAPPHIRE_THREADS=1` runs them on one).
- `after, PoCL device` runs with the device path on PoCL, an OpenCL runtime on the same CPU. It shows that the device path is byte identical and what the path costs on this machine; it is CPU OpenCL and is never GPU speed. Speedups on a real GPU are INFERRED until one is measured.

Most of the remaining time of the light stages is moving the frame in and out of the process, not the effect. Start with a short, small clip.

## What is claimed

The fifteen effects are rebuilt so that, on every case the parity record compares, they give the original's output bit for bit, apart from four counted comparisons of `Shake` and `S_DissolveShake` whose recorded renders were made before three faults were corrected, two in the native code and one in a value handed to the runner ([what rebuilt means](effects.md#what-rebuilt-means)). For the stages, in plain words:

- **MEASURED.** At depth 32, with the frame sizes and parameter values of the recorded renders that use each stage's host arrangement, the stage output is byte identical to the recorded native render, checked by full SHA-256: 382 of 382 on the stages of the first ten effects, 24 of 24 on `flicker`, 36 of 36 on `glow`, 32 of 32 on `swishpan`, 54 of 54 on the three `Shake` stages and 27 of 27 on the `S_DissolveShake` stage ([Effects and stages](../README.md#effects-and-stages)). The other 4 counted `swishpan` renders and 9 counted renders of each of `Shake` and `S_DissolveShake` were recorded at another phase of the default xy buffer, and the three channel renders of `Shake` at 50x7 with their three buffers not all at one phase; they are verified through `fx_run`.
- **INFERRED.** Other frame sizes and parameter values, a real clip among them, run the same code path and are expected to match, but were not measured. The same holds for buffer placement at unmeasured sizes, `S_Trails` clips longer than the measured histories, `S_TimeWarpRGB` shifts beyond the measured ones, host curves other than the identity curve and the recorded `swishpan` Wipe Percent curves, frame times at rates other than 24 frames a second, and transitions of other lengths.
- **Not claimed.** Depths 8 and 16 add a conversion to and from float that is not part of the record. Recorded renders with a different host arrangement are not claimed for the stage.
- **Platform.** Every comparison was judged by an emulator that follows Intel's specification, not by real CPU silicon, and the native code reproduces the original's AVX2 and FMA code tier only.
- **Host.** The stages render the effect, not a full host application. The original host's keyframe interpolation, masks, real host frame fetches and the original's GPU renderer are outside the claim ([limits common to the effects](effects.md#limits-common-to-the-effects)). Sapphire's own values between keys are INFERRED, and its device path is MEASURED on PoCL only.

The limits of each effect are in [effects](effects.md), and the stage limits in [effects provider](stages.md#effects-provider).
