# Effects

Ten of the plugin's effects are rebuilt in this repository as native Rust code. The effect crates are under `crates/effects`; the command line tool `fx_run` renders an effect from its program name, a size and the input planes, with no external data ([Build and run](#build-and-run)). Each registered style of the ten effects is also offered as a stage of `sapphire stage apply`, which reads and writes raw video frames ([Adding a provider](stages.md#effects-provider)). The evidence that each effect matches the original plugin is kept outside the repository (see [Evidence](#evidence)).

Effects rebuilt: <!--f:effects_rebuilt-->10<!--/f--> of <!--f:effects_listed-->10<!--/f--> listed here, the first milestone of the effect rebuild. The other effects of the plugin are not rebuilt; their names are strings only ([names](../README.md#names)). A new effect follows the conventions of [Adding an effect](adding-an-effect.md).

## What rebuilt means

An effect counts as rebuilt when every one of its registered styles passes seven checks on the final binary: identity of the program text, the builtin classes it calls, the data it reads, its literals, the compiled plan, the combination tables and the execution itself. Every comparison was made against the original plugin's code running in an emulator that follows Intel's specification, not on silicon. The outputs of the final binary are recorded and checked by full SHA-256. No result of native execution of the original code is used.

<!--gen:effects_status-->
| effect | registered styles | verified comparisons | recorded renders | re-rendered units |
|---|---|---|---|---|
| `S_Blur` | Blur_Color_Autogen, Blur_Mono_Autogen | 7 | 52 | 52 |
| `BlurMoCurves` | Transform_and_Blur_Autogen, Blur_Motion_Curves_No_Transform_Autogen | 41 | 69 | 69 |
| `S_Diffuse` | Diffuse_Autogen | 31 | 54 | 54 |
| `S_DistortRGB` | Distort_RGB_Autogen | 30 | 30 | 30 |
| `S_DropShadow` | registered without a wrapper program | 24 | 26 | 26 |
| `S_HueSatBright` | Hue_Sat_Bright_Autogen | 130 | 196 | 196 |
| `S_Sharpen` | Sharpen_Best_Quality_Autogen, Sharpen_Fast_Autogen | 43 | 114 | 114 |
| `S_TimeWarpRGB` | Time_Warp_RGB_Autogen | 22 | 22 | 22 |
| `S_Trails` | Trails_Autogen | 78 | 170 | 220 |
| `S_WarpPerspective` | Warp_Perspective_Autogen | 19 | 256 | 256 |
<!--/gen:effects_status-->

*Verified comparisons* are the comparisons between the original's emulator runs and the native code that count toward the effect: <!--f:effects_counted_rows-->425<!--/f--> in all. *Recorded renders* are the outputs of the final binary, recorded as the expected result of each case: <!--f:effects_frozen_members-->989<!--/f--> in all. A few of them are controls, recorded and not counted; the rest are the *counted* recorded renders. *Re-rendered units* are the same cases rendered again with the binary built from this repository, with a second render for the cases that render twice: <!--f:effects_rerun_units-->1,039<!--/f--> units, every one byte identical to its recorded render by full SHA-256 in the three output files (`ae32`, `dest` and `src0`). `BlurMoCurves` is listed by the name of its host record; that string is not in the [name bank](glossary.md#name-bank)'s list of effect names.

The *verified render path* of an effect is the host arrangement its verified comparisons used: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state, 32-bit float images and the effect's own host settings. The stages of `sapphire stage apply` run each style on that path (see [Adding a provider](stages.md#effects-provider)).

## Using the effects

Every effect below is run the same way: build `sapphire` and `fx_run`, write some raw frames at depth 32 and pipe them through the stage. The commands are run from the repository root.

```sh
cargo build --release --offline -p fx_exec -p neutral-handoff
ffmpeg -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le in.raw
```

`sapphire` finds `fx_run` beside itself in `target/release`. Each section gives one worked command that reads `in.raw` and writes `out.raw`. The parameter values in the commands are values the verified comparisons used.

The parameter tables list each stage's parameters as `sapphire stage list --json` prints them: the option that sets it, the stage type and the number of lanes of the value `V`. A parameter with more than one lane is passed as comma separated numbers, for example `--param blur_rel=0.5,1.25`. A parameter that is not passed keeps its registered default. The defaults and valid ranges are printed by `fx_run --describe-params PROGRAM`, for example `target/release/fx_run --describe-params Trails_Autogen`; the stage refuses a value outside the range with exit 3.

Parameters whose names contain `mocha` belong to the mask controls of the original. They are offered because verified comparisons set them, but mask tracking data is not part of the rebuild ([Limits common to the ten effects](#limits-common-to-the-ten-effects)).

Popup parameters take the zero-based index of the item, so the first item is `0`. The opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`) are the exception: they take a host answer, a one-based item number. Of the opacity choices only Normal (`2`) renders on the stages' host arrangement; All_Opaque and As_Premult are refused with exit 3 (MEASURED). `cset_output_opacity` of `time_warp_rgb_autogen` renders both of its items.

See [Driving the effect parameters](parameters-guide.md) for every stage parameter, how to write its value and the values verified against recorded renders.

### `S_Blur`

Blurs the image. The amount sets the blur radius, and a relative amount per axis makes the blur wider in one direction than in the other. Brightness, an offset for dark values, an opacity scale and a mix with the source shape the result after the blur. The two styles are the two choices of the effect's style popup: a colour blur and a mono blur.

| style | stage |
|---|---|
| Blur_Color_Autogen | `stage apply blur_color_autogen` |
| Blur_Mono_Autogen | `stage apply blur_mono_autogen` |

Both stages offer the same parameters:

| parameter | type | lanes |
|---|---|---|
| `--param blur_amount=V` | float | 1 |
| `--param shift_mocha_mask=V` | text | 2 |
| `--param blur_rel=V` | text | 2 |
| `--param brightness=V` | float | 1 |
| `--param scale_opacity=V` | float | 1 |
| `--param mix_with_source=V` | float | 1 |
| `--param subpixel=V` | float | 1 |
| `--param offset_darks=V` | float | 1 |
| `--param filter=V` | float | 1 |
| `--param combine_masks=V` | float | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param invert_mocha_mask=V` | float | 1 |
| `--param resize_mocha_mask=V` | float | 1 |
| `--param resize_mocha_rel_x=V` | float | 1 |
| `--param resize_mocha_rel_y=V` | float | 1 |
| `--param bypass_mocha_mask=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param blur_amount=4.2183990478515625 --param blur_rel=0.573104858,1.25630188 --param brightness=1.36999512 --param mix_with_source=0.337097168 < in.raw > out.raw
```

Parity: 7 verified comparisons, 52 recorded renders and 52 re-rendered units, all equal. No approximate reciprocal instruction is used on the verified render path (MEASURED). Through each stage, 18 of the style's 24 counted recorded renders are re-checked: the ones that match the stage's host arrangement.

### `BlurMoCurves`

Motion blur along a path. The host supplies a motion curve per frame (rotation, shift, depth distance and centre), and the effect averages the image over the shutter interval along that motion. Transform_and_Blur also applies the transform of the curve to the image; Blur_Only blurs along the motion and leaves the image in place. Shutter duration and shift set the interval, exposure bias weights it, and the blur resolution sets how many samples are taken.

| style | stage |
|---|---|
| Transform_and_Blur_Autogen | `stage apply transform_and_blur_autogen` |
| Blur_Motion_Curves_No_Transform_Autogen | `stage apply blur_motion_curves_no_transform_autogen` |

Both stages offer the same parameters:

| parameter | type | lanes |
|---|---|---|
| `--param shutter_duration=V` | float | 1 |
| `--param scale_result=V` | float | 1 |
| `--param subpixel=V` | checkbox | 1 |
| `--param cset_opacity=V` | int | 1 |
| `--param exposure_bias=V` | float | 1 |
| `--param shutter_shift=V` | float | 1 |
| `--param blur_res=V` | int | 1 |
| `--param wrap=V` | text | 2 |
| `--param host_curve_values=V` | text | a file path |

`host_curve_values` names a curve file in the recorded format. Without it the stage writes an identity curve (no rotation, no shift, depth distance 1, centred) and adds every sample `fx_run` asks for across the whole shutter interval, so any `shutter_duration` runs (MEASURED). A sample that neither a given curve file nor the identity curve can serve ends exit 3 with `HOST-CURVE-SAMPLE-UNSERVED` (MEASURED).

```sh
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param scale_result=0.765625 < in.raw > out.raw
```

Parity: 41 verified comparisons (24 for Transform_and_Blur and 17 for Blur_Only), 69 recorded renders and 69 re-rendered units, all equal. The entry code of the route's builtins uses no approximate reciprocal instruction (MEASURED); the kernel bodies behind it were not listed, so whether one is reached is not measured (INFERRED to be none). The curve values come from a stand-in for the host. Of the counted recorded renders, 13 of 35 (Transform_and_Blur) and 13 of 32 (Blur_Only) match the stage's host arrangement.

### `S_Diffuse`

Scatters the pixels of the image: each output pixel takes its value from a nearby position chosen by a fixed pseudo random pattern, so edges break up into grain. The amount sets how far pixels move, a relative amount per axis stretches the scatter, and the wrap setting chooses what happens at the image border.

| style | stage |
|---|---|
| Diffuse_Autogen | `stage apply diffuse_autogen` |

| parameter | type | lanes |
|---|---|---|
| `--param diffuse_amount=V` | float | 1 |
| `--param blur_mocha_mask=V` | float | 1 |
| `--param shift_mocha_mask=V` | text | 2 |
| `--param rel_amount=V` | text | 2 |
| `--param wrap=V` | text | 2 |
| `--param diffuse_amount_host=V` | float | 1 |
| `--param blur_mocha_mask_host=V` | float | 1 |
| `--param bypass_mocha_mask=V` | float | 1 |
| `--param combine_masks=V` | float | 1 |
| `--param invert_mocha_mask=V` | float | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param resize_mocha_mask=V` | float | 1 |
| `--param resize_mocha_rel_x=V` | float | 1 |
| `--param resize_mocha_rel_y=V` | float | 1 |
| `--param show_mocha_mask_only=V` | float | 1 |

`diffuse_amount` and `blur_mocha_mask` are the values as the host passes them; `diffuse_amount_host` and `blur_mocha_mask_host` set the same parameters in the effect's own units.

```sh
target/release/sapphire stage apply diffuse_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param diffuse_amount=48 --param rel_amount=1,1 --param wrap=2,2 < in.raw > out.raw
```

Parity: 31 verified comparisons, 54 recorded renders and 54 re-rendered units, all equal. One approximate reciprocal instruction is reached, inside a logarithm; its effect on the output is 0 ulp under the emulator's bound (MEASURED). 31 of 42 counted recorded renders match the stage's host arrangement.

### `S_DistortRGB`

Bends the image through a distortion field, with a different strength for the red, green and blue channels, so colours split apart at the distorted edges. The field comes from a blurred lens image: the optional `Lens` layer, or the source itself. Amount and its relative amount per axis set the strength, the per-channel warp values set each channel's share, and the warp direction can be rotated.

| style | stage |
|---|---|
| Distort_RGB_Autogen | `stage apply distort_rgb_autogen` |

| parameter | type | lanes |
|---|---|---|
| `--param amount=V` | float | 1 |
| `--param amount_rel=V` | text | 2 |
| `--param warp_red=V` | float | 1 |
| `--param warp_green=V` | float | 1 |
| `--param warp_blue=V` | float | 1 |
| `--param blur_lens=V` | float | 1 |
| `--param wrap=V` | text | 2 |
| `--param filter=V` | float | 1 |
| `--param rotate_warp_dir=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param amount=1.3125 --param amount_rel=1.1875,0.8125 --param warp_red=0.9375 --param warp_green=0.4375 --param warp_blue=0.1875 --param blur_lens=44 < in.raw > out.raw
```

Parity: 30 verified comparisons, 30 recorded renders and 30 re-rendered units, all equal. 28 are byte identical; in 2 the output holds NaN values, which are equal as NaN values while their payload bits are not compared. No approximate reciprocal instruction is used on the verified render path (MEASURED). 26 of 30 counted recorded renders match the stage's host arrangement. No verified comparison used the `Lens` layer.

### `S_DropShadow`

Casts a shadow of the image. The shadow is made from the image's alpha, blurred, shifted, coloured and given its own opacity, then placed under the foreground, whose opacity can be set as well. The optional `Background` layer is composited under the result and the optional `Matte` layer limits it. `comp_premult` chooses how the layers are combined with respect to premultiplied alpha.

| style | stage |
|---|---|
| Drop_Shadow | `stage apply drop_shadow` |

This effect has no wrapper program; the stage runs its registered program, Drop_Shadow.

| parameter | type | lanes |
|---|---|---|
| `--param shadow_blur=V` | float | 1 |
| `--param color=V` | text | 3 |
| `--param shadow_opacity=V` | float | 1 |
| `--param shift=V` | text | 2 |
| `--param fg_opacity=V` | float | 1 |
| `--param comp_premult=V` | checkbox | 1 |
| `--param combine_masks=V` | int | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param shift_host=V` | text | 2 |
| `--param cset_opacity=V` | int | 1 |

`shift` is the offset in the effect's own units; `shift_host` passes it as the host does.

```sh
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --fps 25/1 --param shadow_blur=0.291702271 --param color=0.356862754,0.713725507,0.184313729 --param shadow_opacity=0.679092407 --param shift=0.00299692154,-0.00117826462 --param fg_opacity=0.736099243 --param comp_premult=1 < in.raw > out.raw
```

Parity: 24 verified comparisons, 26 recorded renders and 26 re-rendered units, all equal in both renders. None of the approximate reciprocal instructions in the effect's code is reached on the verified render path (MEASURED for 11 sites; the logarithm sites INFERRED from the plan constants). 20 of 24 counted recorded renders match the stage's host arrangement. No verified comparison used a layer.

### `S_HueSatBright`

Adjusts colour: rotates the hue, scales the saturation, scales the brightness and offsets the dark values. With `preserve_luma` the hue rotation keeps each pixel's luma. `scale_colors` scales the red, green and blue channels separately.

| style | stage |
|---|---|
| Hue_Sat_Bright_Autogen | `stage apply hue_sat_bright_autogen` |

| parameter | type | lanes |
|---|---|---|
| `--param brightness=V` | float | 1 |
| `--param bypass_mocha_mask=V` | float | 1 |
| `--param hue_shift=V` | float | 1 |
| `--param offset_darks=V` | float | 1 |
| `--param preserve_luma=V` | checkbox | 1 |
| `--param saturation=V` | float | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param scale_colors=V` | text | 3 |

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param hue_shift=0.419998169 --param saturation=1.1499939 --param brightness=0.800003052 --param preserve_luma=1 < in.raw > out.raw
```

Parity: 130 verified comparisons, 196 recorded renders and 196 re-rendered units, all equal. The hue rotation with preserved luma uses an approximate reciprocal instruction, so it may differ on real CPUs; the largest change measured stays within Intel's stated bound, 25,244 ulp. The source has three planes and the output alpha is taken from the source. A brightness outside the registered range is not verified. 118 of 130 counted recorded renders match the stage's host arrangement.

### `S_Sharpen`

Sharpens the image. Best Quality separates the image into detail bands (tiny, small, medium and large), scales each band and sharpens along edges, leaving areas below the edge threshold alone. Fast uses a single sharpening width. In both, the amount can be set overall, for luma and chroma, and per channel.

| style | stage |
|---|---|
| Sharpen_Best_Quality_Autogen | `stage apply sharpen_best_quality_autogen` |
| Sharpen_Fast_Autogen | `stage apply sharpen_fast_autogen` |

`sharpen_best_quality_autogen`:

| parameter | type | lanes |
|---|---|---|
| `--param small_detail_size=V` | float | 1 |
| `--param edge_threshold=V` | float | 1 |
| `--param scale_tiny_details=V` | float | 1 |
| `--param scale_small_details=V` | float | 1 |
| `--param scale_medium_details=V` | float | 1 |
| `--param scale_large_details=V` | float | 1 |
| `--param sharpen_amp=V` | float | 1 |
| `--param sharpen_luma=V` | float | 1 |
| `--param sharpen_chroma=V` | float | 1 |
| `--param sharpen_red=V` | float | 1 |
| `--param sharpen_green=V` | float | 1 |
| `--param sharpen_blue=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

`sharpen_fast_autogen`:

| parameter | type | lanes |
|---|---|---|
| `--param sharpen_width=V` | float | 1 |
| `--param sharpen_amp=V` | float | 1 |
| `--param sharpen_luma=V` | float | 1 |
| `--param sharpen_chroma=V` | float | 1 |
| `--param sharpen_red=V` | float | 1 |
| `--param sharpen_green=V` | float | 1 |
| `--param sharpen_blue=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param sharpen_width=100 --param sharpen_amp=0 --param sharpen_luma=0.5 --param sharpen_chroma=1.5 < in.raw > out.raw
```

Parity: 43 verified comparisons, 114 recorded renders and 114 re-rendered units, all equal in both renders. The edge-aware core of Best Quality is sensitive to approximate reciprocals, with a bound of 6 ulp (INFERRED); Fast uses none (INFERRED). Of the counted recorded renders, 19 of 42 (Best Quality) and 16 of 36 (Fast) match the stage's host arrangement.

### `S_TimeWarpRGB`

Shifts the red, green and blue channels in time: each channel is taken from a frame a set number of frames earlier or later, so moving objects get coloured fringes. The chroma clamp (`--param clamp_chroma=V`) limits the colour difference the shift may create.

| style | stage |
|---|---|
| Time_Warp_RGB_Autogen | `stage apply time_warp_rgb_autogen` |

The stage is temporal: it reads frames before and after the current one.

| parameter | type | lanes |
|---|---|---|
| `--param clamp_chroma=V` | float | 1 |
| `--param shift_frames_blue=V` | int | 1 |
| `--param shift_frames_green=V` | int | 1 |
| `--param shift_frames_red=V` | int | 1 |
| `--param cset_output_opacity=V` | int | 1 |
| `--param cset_input_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param shift_frames_red=2 --param shift_frames_green=-2 --param shift_frames_blue=1 --param clamp_chroma=0.5 < in.raw > out.raw
```

Parity: 22 verified comparisons, 22 recorded renders and 22 re-rendered units, all equal. No approximate reciprocal instruction is used on the verified render path (MEASURED). One effect instance runs per process, with a fresh frame cache per render. 10 of 22 counted recorded renders match the stage's host arrangement.

### `S_Trails`

Leaves trails behind moving content: each output frame blends the current frame with the previous output, which fades toward a colour and can be blurred and hue shifted on every step. The new frame's colour and opacity, the fade, the fade colour and the way the two are combined set the look.

| style | stage |
|---|---|
| Trails_Autogen | `stage apply trails_autogen` |

The stage is temporal: it reads the whole clip before it writes.

| parameter | type | lanes |
|---|---|---|
| `--param blur_amount=V` | float | 1 |
| `--param invert_mocha_mask=V` | float | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |
| `--param blur_rel=V` | text | 2 |
| `--param combine=V` | float | 1 |
| `--param fade=V` | float | 1 |
| `--param fade_color=V` | text | 3 |
| `--param new_color=V` | text | 3 |
| `--param new_opacity=V` | float | 1 |
| `--param prev_hue_shift=V` | float | 1 |

```sh
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param blur_amount=1.5625 --param mocha_mask_opacity=0.5 < in.raw > out.raw
```

Parity: 78 verified comparisons, 170 recorded renders and 220 re-rendered units, all equal in both renders. No approximate reciprocal instruction is used on the verified render path (MEASURED). 70 of 120 counted recorded renders match the stage's host arrangement; trails longer than the recorded histories are INFERRED.

### `S_WarpPerspective`

Places the image on a plane seen in perspective. Swing, roll and latitude turn the plane, the depth distance and the lens width set the camera, and the wrap settings choose what fills the area outside the image and above the horizon.

| style | stage |
|---|---|
| Warp_Perspective_Autogen | `stage apply warp_perspective_autogen` |

| parameter | type | lanes |
|---|---|---|
| `--param filter=V` | float | 1 |
| `--param z_dist=V` | float | 1 |
| `--param tele_lens_width=V` | float | 1 |
| `--param swing=V` | float | 1 |
| `--param roll=V` | float | 1 |
| `--param latitude=V` | float | 1 |
| `--param wrap_above_horizon=V` | float | 1 |
| `--param wrap=V` | text | 2 |
| `--param cset_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param z_dist=2.5 --param latitude=80 --param tele_lens_width=1.6000061 --param filter=1 < in.raw > out.raw
```

Parity: 19 verified comparisons, 256 recorded renders and 256 re-rendered units, all equal in both renders. The perspective uses approximate reciprocals, so it may differ on real CPUs: by up to 6 ulp in its first output and 12,972 ulp in its second, where values near zero cancel. The verified sizes are 64x32 and 50x7. 10 of 224 counted recorded renders match the stage's host arrangement.

## Build and run

From the repository root, offline, with a stable Rust toolchain:

```sh
cargo build --release --offline -p fx_exec
```

This builds `target/release/fx_run`. The call is

```sh
fx_run PROGRAM --size WxH --out PREFIX [--synth SEED] [options]
```

- No program table is passed: the effect graphs are built into the tool (see [No external data](#no-external-data)). A programs table in the first place is an optional override for tests, read from the stored test data.
- `PROGRAM` is the registered program, for example the wrapper program of a style (see the table above).
- Planes are raw little endian `f32`, planar. `--in Name=path` binds an input plane, `--synth SEED` fills every unbound input with a fixed pseudo random plane, `--set name=v1,v2` sets a parameter. The output is `PREFIX.dest.f32` with `PREFIX.dest.meta.txt`, which lists the run and every label the run carries. The full option list is the header of `crates/effects/fx_exec/src/bin/fx_run.rs`.
- `fx_run --describe-params PROGRAM` prints the parameters of a program as JSON: name, type, lanes, valid range and registered default.
- The host context is part of the call: options such as `--ctx-time`, `--ctx-word2` and `--bound-mask` are required and have no default, and a call that omits one is refused with a message that names it. The stages of `sapphire stage apply` fill them in for the verified render path.
- The tool refuses what it does not reproduce and says so by name, for example an input class or a parameter value outside the recorded cases.

### No external data

The effects need no external data. Nothing is read from the original plugin or from a copied file at run time: `fx_run` takes no table argument and reads only the input planes and the output path it is given.

- **Effect graphs.** The programs of the thirteen styles are built by Rust constructor code in `fx_eval` (`programs_native`). That code is a tool-made translation of the plugin's program rows into Rust: a development tool produced it from the parsed program structure. It holds no program text and reads no table.
- **Parameter and class facts.** The registered parameters, ranges, input indices, class flags and combination tables that the thirteen programs need are Rust definitions in the crate `effects_rtdata`. A definition is kept only if it is keyed by one of the 25 programs of the styles, by a class they name, or by a builtin they call. A lookup that has no row misses, as it did with the tables.
- **Sin and cos tables.** The sin and cos routines of the effects need four tables. None is stored; all are computed from their mathematical definition at first use, with a small fixed-point big number (320 bit, written for this repository, no crate):

- The sincosf table has 64 entries: the correctly rounded (sin, cos) of k pi/32, exact 0 and 1 at the axes.
- The double sincos table has 64 entries of eight words: the correctly rounded sin and cos, their rounding tails rounded to 24 bits, cos and -sin rounded to one bit, and the exact remainders of cos and -sin against those.
- The pair region of the scalar sin and cos routine (SVML style): pair j of 0 to 64 is cos(j pi/128) h^3/6 and sin(j pi/128) h^2/2 with h = pi/128; the cos table reads the same pairs from the other end. Two constants are 2/h^2 and 6/h^2.
- The 15 chunks of 1/pi of that routine: frac(2^p/pi)/2^(p-7) with p = 7 + 8(m-2), cut into 29 bits, up to 42 bits, and the rest.

Known limit: none for these tables. Bytes of the original 2304 byte tables that no reader touches are not reproduced.

Tests that compare the definitions or the computed tables with copies of the plugin's tables, or sweep whole tables, read those copies from the stored test data under `VEC_DIR` (`effects/rtdata/` and `effects/native_tables/`) and print a `NOT RUN` line when it is absent. No copy is part of the repository.

## Tests

- `cargo test --release` runs the effect crates whose tests need no stored data, with the other default members.
- `cargo test --release -p fx_eval -p fx_exec` runs the rest of the evaluator and the executor. A test that needs stored data from the original (recorded plans, emulator rows, the program corpus) prints a `NOT RUN` line and passes, the way the lift crates' vector tests stay out of a plain `cargo test`.
- `VEC_DIR` names a folder that holds an `effects` folder with the stored data, laid out with the relative paths the tests print in their `NOT RUN` line; with it every test runs. Tests write their files to the system temporary folder, never into the source tree.

## Limits common to the ten effects

- Every result is judged by the emulator, which follows Intel's specification (SDM), not by real CPU silicon.
- `ROUTE AVX2-ONLY`: the original selects one of several code tiers from the CPU's features. The native code reproduces the tier that runs on CPUs with AVX2 and FMA; on CPUs without them the original runs other tiers with different rounding, and those results are not reproduced.
- Where an operation is sensitive to approximate reciprocals, the effect's section states its bound in ulps.
- The host side is modelled, not rebuilt: the verified render path described above. In the test harness, three of the plugin's licence-family routines were answered with fixed values instead of being run, and the host's licence client found no licence server.
- Not verified: a real host application (its parameter units, keyframed animation, multi band and multi threaded rendering, real frame fetches), GPU rendering, 8 and 16 bit formats, trial or unlicensed rendering, mocha with tracking data, and NaN payload and sign bits.
- Licence, activation, trial and machine identity code of the original is not rebuilt.
- Cases outside an effect's verified comparisons, such as other sizes and parameter values, run the same code and are INFERRED.

## Evidence

The native code was written by reading the original plugin's machine code and data. For publication and licensing, see [Legal](../README.md#legal).

The full report, with a paragraph per effect, its exclusion tables and the runs behind it, is kept outside this repository. Its SHA-256 lets a copy be checked:

The full report: `b7ee202da80faf7ce31f95ea5cced46badf3f1b7dacdbec8dc1ea8b865e2910c`
