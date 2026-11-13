# Using the effects

This page shows how to run the nineteen native effects on a video or on raw frames, how to find their parameters, and what the results are claimed to match. The effects themselves are described in [effects](effects.md); the stage interface they use is specified in [stages](stages.md).

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
| `@transform_and_blur_autogen`, `@blur_motion_curves_no_transform_autogen` | `S_BlurMoCurves` | temporal |
| `@blur_motion_autogen`, `@blur_motion_mono_autogen` | `S_BlurMotion` | temporal |
| `@diffuse_autogen` | `S_Diffuse` | temporal |
| `stage apply dissolve_shake` only, see [Time and transitions](#time-and-transitions) | `S_DissolveShake` | temporal |
| `@distort_rgb_autogen` | `S_DistortRGB` | temporal |
| `@drop_shadow` | `S_DropShadow` | temporal |
| `@flicker` | `S_Flicker` | temporal |
| `@glint` | `S_Glint` | temporal |
| `@glow` | `S_Glow` | temporal |
| `@glow_edges` | `S_GlowEdges` | temporal |
| `@hue_sat_bright_autogen` | `S_HueSatBright` | temporal |
| `@shake_normal_autogen`, `@shake_twitchy_autogen`, `@shake_jumpy_autogen` | `S_Shake` | temporal |
| `@sharpen_best_quality_autogen`, `@sharpen_fast_autogen` | `S_Sharpen` | temporal |
| `swishpan` (with `stage apply` only, see [Time and transitions](#time-and-transitions)) | `S_SwishPan` | temporal |
| `@time_warp_rgb_autogen` | `S_TimeWarpRGB` | temporal |
| `@trails_autogen` | `S_Trails` | temporal |
| `@warp_perspective_autogen` | `S_WarpPerspective` | temporal |
| `@warp_transform_autogen` | `S_WarpTransform` | temporal |

```sh
target/release/sapphire stage list
```

All 25 stages have kind `temporal`, because the frames/2 convention accepts keyed values only for temporal stages. Only `time_warp_rgb_autogen` and `trails_autogen` read other frames of the clip: the first shifts colour channels in time, and the second reads the whole clip before it writes. The other 23 stages turn each input frame into one output frame and are `temporal` so that they accept keyframes (INFERRED for the four stages of `S_BlurMotion`, `S_WarpTransform` and `S_GlowEdges` and for the `glint` stage: their stage rows request no other frame); of them, `swishpan` and the `S_DissolveShake` stage also read the matching frame of a second clip. The kind `frame` belongs to `selftest-invert` and `ffmpeg-filter`.

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

The device output is the same bytes as the CPU output. A node the kernel declines for a given shape or value, and a node whose output holds a NaN that the device may not keep (below), run on the CPU for that node, so a partly supported plan still renders. `SAPPHIRE_DEVICE_REPORT=1` prints how many nodes of each operation ran on the device and how many fell back, with the reason.

**Coverage.** Every operation that the plans of the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails`, `S_WarpPerspective`, `S_BlurMotion`, `S_WarpTransform`, `S_GlowEdges` and `S_Glint` dispatch has a device kernel. An operation with a `CPU_ONLY` entry in `crates/effects/fx_exec/src/device_ops.rs` has no kernel and runs on the CPU (`no_kernel`); no operation of these stages has one. MEASURED on PoCL on a 12 core server: all 173 golden rows of these stages give device output equal to the CPU output by full SHA-256, at the default thread count and at `SAPPHIRE_THREADS` 1, 3 and 12, and every node of 164 of them runs on the device (`fallback: none`). The 210 counted recorded renders of `S_Flicker`, `S_Glow`, `S_SwishPan`, `S_Shake` and `S_DissolveShake` and the 487 of `S_BlurMotion` (352), `S_WarpTransform` (78), `S_GlowEdges` (34) and `S_Glint` (23) give device output equal to the CPU output with `fallback: none` on every one. The other 9 golden rows are special rows (size `40x12s`: infinities, NaN and subnormal values): 8 rerun NaN outputs on the CPU (below) and `distort_rgb_autogen` refuses its special row on both paths with `F2(Oob)`. At its default parameters the plan of `hue_sat_bright_autogen` folds to the source frame and holds no node, so it reports `device: none`; with a colour parameter set (the variant rows `+hs` and `+pl`) its nodes run on the device.

**NaN outputs.** The CPU kernels keep the x86 NaN rules: which NaN of two survives, its payload and its sign. A node whose device output holds a NaN keeps that output only when its operation is NaN exact on the device in use; otherwise the node is rerun on the CPU and the CPU's words are used. An operation is NaN exact when both hold:

- it is on the `NAN_EXACT` list of `device_ops.rs`: its kernel computes every NaN by explicit tests in the CPU's operand order (a quieted operand, or the fresh NaN 0xffc00000), so the hardware's own NaN propagation never reaches an output word;
- its self-check on the device in use, which adds NaN of both signs and several payloads to every operand, reports equal NaN payloads and signs.

The list has 40 operations in two classes. `NAN_EXACT_WRITTEN` (35) holds the operations whose NaN words follow from the CPU source on any compiler: word copies and fills, compares and selects (`max`, `min`), the first operand of a subtraction or a division (`-`, `/`, `diff`), `postdivide`, `alpha_from_rgb_max`, and nine operations whose CPU kernels write each two-NaN pick through `fx_common::nanop`: `blur`, `masked_blur`, `sharpen`, `edge_aware_sharpen`, `tilt`, `tilt_alpha`, `edge_detect`, `blur_channels` and `glint`. `NAN_EXACT_COMPILED` (5: `+`, `*`, `screen_super`, `dissolve`, `make_mipmap`) holds the operations whose NaN words are those of the compiled CPU instruction (a commutative operation whose operands the compiler may swap, a `mul_add` through the platform `fmaf`, a fresh NaN of a plain operation); their kernels copy the pick of the build the self-check compares with. The admission is therefore per device and per build: MEASURED on PoCL on that server with this build, where all 40 report equal payloads; INFERRED on every other device and build, where the self-check is the guard.

The other operations keep the CPU rerun of NaN outputs: `glow` is not on the list (its blur passes use the NaN-exact `blur` kernel, but the operation as a whole is not admitted, so its self-check adds no NaN cases); `blur_warp` reaches the CPU's two-NaN picks through the compiled instruction at each site of five kernel bodies, among them a fresh NaN meeting its own negation, which its kernel does not copy (first differing word: CPU 0x7fc00000, device 0xffc00000); `warp` and `warp_rel` sample with the hardware NaN rule, as the CPU sampler does, and have no NaN-exact kernel; `screen`, `overlay_super` and the remaining operations are not on the list. On the golden rows only the special rows reach this rerun: `glow` (glow), `swishpan` (warp), `warp_perspective_autogen` (warp), `transform_and_blur_autogen`, `blur_motion_curves_no_transform_autogen`, `blur_motion_autogen`, `blur_motion_mono_autogen` and the `S_DissolveShake` stage (blur_warp). Each of these rows is byte identical to the CPU through the rerun (MEASURED).

**On another device.** A GPU follows the same two guards: the self-check refuses any operation whose kernel differs from the CPU on the corpus, and a NaN output of an operation that the device did not admit as NaN exact is rerun on the CPU. A device whose NaN propagation or rounding differs from the CPU's therefore costs reruns or CPU nodes, not bytes, within what the corpus covers. Bit identity on a real GPU beyond the corpus, the number of operations a given GPU admits and its speed are INFERRED: nothing here was measured on a GPU.

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

The first line names the device and the result of its self-check. After each frame a line follows with the running count of nodes per operation on the device and of the fallbacks, for example `device: blur 2, postdivide 2; fallback: none`. A fallback names the operation, its count and the reason, for example `fallback: blur_warp 3 (nan_output 3)` on the special golden row of `transform_and_blur_autogen`. A plan that holds no node reports `device: none`, for example `hue_sat_bright_autogen` at its default parameters, whose plan folds to the source frame.

**Requiring a device.** `SAPPHIRE_REQUIRE_DEVICE=1` turns a missing device into an error: a `--device` other than `cpu` that installs no device refuses the run instead of rendering on the CPU: `sapphire stage apply` prints a `device: DEVICE-REQUIRED-UNAVAILABLE ...` line and exits with code 3 (`fx_run` exits with code 2). Use it in checks and CI, where a silent fallback would let a device test pass without a device.

**Caches.** The self-check is paid on PoCL the first time a build meets a device and driver (MEASURED on a 12 core server: 67 to 69 s at a one-minute load of 3.3 to 5.3 for 24 frames of 160x96 of `blur_color_autogen` with an empty cache folder, nearly all of it the self-check; the time grows with the machine's load). Only passes are kept, in the user cache folder (`$XDG_CACHE_HOME/sapphire` or `~/.cache/sapphire` on Linux, `~/Library/Caches/sapphire` on macOS, `%LOCALAPPDATA%\sapphire` on Windows), and a later process skips them: the same run then takes 0.38 s, against 0.15 s on the CPU. A failed check is never kept, so it runs again in every process. The entry is keyed on the build (a digest of the running executable, which holds the CPU kernels, the device glue and the corpus), the device name, the driver version, the digest of the kernel sources, the build options and the digest of the loaded program binary. A new build, a new driver, a changed kernel or a replaced program binary makes the entry unreadable and the checks run again. Compiled OpenCL programs are cached in `opencl-programs` under the same folder. Deleting the folder is always safe: both caches are rebuilt on the next run. `SAPPHIRE_DEVICE_CHECK_CACHE=0` turns the self-check cache off and `SAPPHIRE_CACHE_DIR` moves the folder.

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

The three `S_Shake` stages also read the frame rate: the motion blur and the twitch look one frame ahead and one frame back. Pass the real rate of the clip; without `--fps` a `S_Shake` run is refused with `FPS-UNGIVEN` (exit 3). The verified renders use whole frames of a 24 frames a second clock as well.

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

Two styles of `S_BlurMoCurves` and `swishpan` read a host curve file, passed as `--param host_curve_values=PATH`; without it they use a constant curve. Only `swishpan` and the `S_DissolveShake` stage take an extra layer, the second clip of their transition; `--layer` on any other effect stage exits 3 with `LAYER-INPUT-UNSUPPORTED` ([effects provider](stages.md#effects-provider)).

Popup parameters take the zero-based index of the item, so the first item is `0`. The opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`) are the exception: they take a host answer, a one-based item number. On the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective` only Normal (`2`) of the opacity choices renders on the stages' host arrangement; All_Opaque and As_Premult are refused with exit 3 (MEASURED), and `cset_output_opacity` of `time_warp_rgb_autogen` renders both of its items. `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage render all three answers, each with the host arrangement of its own recorded renders.

See [Driving the effect parameters](parameters-guide.md) for every stage parameter, how to write its value and the values verified against recorded renders.

## Speed

Wall time of one 1920x1080 frame through `sapphire stage apply`, MEASURED on a 12 core server under shared load (one-minute load average 5.3 to 9.2; two runs per cell after one warm-up run, in seconds, on the same machine; the two `after` columns in one session with CPU and device runs interleaved, except the rows of `blur_motion_autogen`, `blur_motion_mono_autogen`, `warp_transform_autogen`, `stage apply glow_edges` and `glint`, which come from a later session (below), the `before` column in an earlier session at another load). `swishpan` and the `S_DissolveShake` stage render a 3 frame clip (`--frames 3`), and their cells are its wall time divided by 3. The twelve stages added after the previous build have no `before` cell (`-`):

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

- `before the in-process route` is the previous build: one `fx_run` process per frame and a full ripmap plane copy per row in the warp kernels.
- `after, CPU` is the default `--device cpu`, with the warp copies removed, the render in process and the parallel-safe kernels on all cores (`SAPPHIRE_THREADS=1` runs them on one). The CPU cells use the default thread count (all 12 cores) and are sensitive to machine load. `distort_rgb_autogen` shows it most: at a one-minute load of 6 to 9 this build and the earlier in-process builds all measure 4.4 to 4.9 s for its frame, most of it system time in the threaded path, and 1.7 to 2.0 s with `SAPPHIRE_THREADS=1`, so its CPU cell is a contention figure and not a slower kernel. `sharpen_best_quality_autogen`'s `after, CPU` cell lies above its `before` cell for the same reason: the build before this table, timed interleaved with it under the same load, measures 2.5 to 3.6 s against 2.8 to 3.3 s, with equal output bytes.
- `after, PoCL device` runs with the device path on PoCL, an OpenCL runtime on the same CPU, with `SAPPHIRE_REQUIRE_DEVICE=1`. Every run gave the CPU's output bytes and `fallback: none`. It shows that the device path is byte identical and what the path costs on this machine; it is CPU OpenCL and is never GPU speed. Speedups on a real GPU are INFERRED until one is measured.

Most of the remaining time of the light stages is moving the frame in and out of the process, not the effect. Start with a short, small clip.

## What is claimed

The nineteen effects are rebuilt so that, on every case the parity record compares, they give the original's output bit for bit, apart from four counted comparisons of `S_Shake` and `S_DissolveShake` whose recorded renders were made before three faults were corrected, two in the native code and one in a value handed to the runner, and 34 of `S_GlowEdges` that were run with slider values passed as decimals, not as the 16.16 fixed-point values a host passes ([what rebuilt means](effects.md#what-rebuilt-means)). For the stages, in plain words:

- **MEASURED.** At depth 32, with the frame sizes and parameter values of the recorded renders that use each stage's host arrangement, the stage output is byte identical to the recorded native render, checked by full SHA-256: 382 of 382 on the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective`, 24 of 24 on `flicker`, 36 of 36 on `glow`, 32 of 32 on `swishpan`, 54 of 54 on the three `S_Shake` stages and 27 of 27 on the `S_DissolveShake` stage ([Effects and stages](../README.md#effects-and-stages)). The other 4 counted `swishpan` renders and 9 counted renders of each of `S_Shake` and `S_DissolveShake` were recorded at another phase of the default xy buffer, and the three channel renders of `S_Shake` at 50x7 with their three buffers not all at one phase; they are verified through `fx_run`.
- **INFERRED.** Other frame sizes and parameter values, a real clip among them, run the same code path and are expected to match, but were not measured. The same holds for buffer placement at unmeasured sizes, `S_Trails` clips longer than the measured histories, `S_TimeWarpRGB` shifts beyond the measured ones, host curves other than the identity curve and the recorded `swishpan` Wipe Percent curves, frame times at rates other than 24 frames a second, and transitions of other lengths.
- **Not claimed.** Depths 8 and 16 add a conversion to and from float that is not part of the record. Recorded renders with a different host arrangement are not claimed for the stage.
- **Platform.** Every comparison was judged by an emulator that follows Intel's specification, not by real CPU silicon, and the native code reproduces the original's AVX2 and FMA code tier only.
- **Host.** The stages render the effect, not a full host application. The original host's keyframe interpolation, masks, real host frame fetches and the original's GPU renderer are outside the claim ([limits common to the effects](effects.md#limits-common-to-the-effects)). Sapphire's own values between keys are INFERRED, and its device path is MEASURED on PoCL only.

The limits of each effect are in [effects](effects.md), and the stage limits in [effects provider](stages.md#effects-provider).
