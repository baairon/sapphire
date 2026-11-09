# Effects

Fifteen of the plugin's effects are rebuilt in this repository as native Rust code. The effect crates are under `crates/effects`; the command line tool `fx_run` renders an effect from its program name, a size and the input planes, with no external data ([Build and run](#build-and-run)). Each registered style of the fifteen effects is also offered as a stage of `sapphire stage apply`, which reads and writes raw video frames ([Adding a provider](stages.md#effects-provider)). The evidence that each effect matches the original plugin is kept outside the repository (see [Evidence](#evidence)). The effects use no code of the [function record](glossary.md#function-record) ([how the two parts relate](../README.md#effects-and-stages)).

Effects rebuilt: <!--f:effects_rebuilt-->15<!--/f--> of <!--f:effects_listed-->15<!--/f--> listed here: the ten of the first milestone and five of Tier 1 of [the roadmap](roadmap.md). The other effects of the plugin are not rebuilt; their names are strings only ([names](../README.md#names)). A new effect follows the conventions of [Adding an effect](adding-an-effect.md).

## What rebuilt means

An effect counts as rebuilt when every one of its registered styles passes seven checks on the final binary: identity of the program text, the builtin classes it calls, the data it reads, its literals, the compiled plan, the combination tables and the execution itself. Every comparison was made against the original plugin's code running in an emulator that follows Intel's specification, not on silicon. The outputs of the final binary are recorded and checked by full SHA-256. No result of native execution of the original code is used.

<!--gen:effects_status-->
| effect | registered styles | verified comparisons | recorded renders | re-rendered units |
|---|---|---|---|---|
| `S_Blur` | Blur_Color_Autogen, Blur_Mono_Autogen | 7 | 52 | 52 |
| `BlurMoCurves` | Transform_and_Blur_Autogen, Blur_Motion_Curves_No_Transform_Autogen | 41 | 69 | 69 |
| `S_Diffuse` | Diffuse_Autogen | 31 | 54 | 54 |
| `S_DissolveShake` | registered without a wrapper program | 38 | 304 | 304 |
| `S_DistortRGB` | Distort_RGB_Autogen | 30 | 30 | 30 |
| `S_DropShadow` | registered without a wrapper program | 24 | 26 | 26 |
| `S_Flicker` | registered without a wrapper program | 24 | 24 | 24 |
| `S_Glow` | registered without a wrapper program | 36 | 36 | 36 |
| `S_HueSatBright` | Hue_Sat_Bright_Autogen | 130 | 196 | 196 |
| `Shake` | Shake_Normal_Autogen, Shake_Twitchy_Autogen, Shake_Jumpy_Autogen | 68 | 2,056 | 2,056 |
| `S_Sharpen` | Sharpen_Best_Quality_Autogen, Sharpen_Fast_Autogen | 43 | 114 | 114 |
| `S_SwishPan` | registered without a wrapper program | 36 | 288 | 288 |
| `S_TimeWarpRGB` | Time_Warp_RGB_Autogen | 22 | 22 | 22 |
| `S_Trails` | Trails_Autogen | 78 | 170 | 220 |
| `S_WarpPerspective` | Warp_Perspective_Autogen | 19 | 256 | 256 |
<!--/gen:effects_status-->

*Verified comparisons* are the comparisons between the original's emulator runs and the native code that count toward the effect: <!--f:effects_counted_rows-->627<!--/f--> in all. Four of them, two of `Shake` and two of `S_DissolveShake`, differ from the original (see those effects). *Recorded renders* are the outputs of the final binary, recorded as the expected result of each case: <!--f:effects_frozen_members-->3,697<!--/f--> in all. Some of them are controls, recorded and not counted; the rest are the *counted* recorded renders. *Re-rendered units* are the same cases rendered again with the binary built from this repository, with a second render for the cases that render twice: <!--f:effects_rerun_units-->3,747<!--/f--> units. Every unit of the first ten effects, `S_Flicker`, `S_Glow` and `S_SwishPan` is byte identical to its recorded render by full SHA-256 in the three output files (`ae32`, `dest` and `src0`), except 24 control units of `S_WarpPerspective`, which changed with the default-xy head form (see that effect). Every unit of `Shake` and `S_DissolveShake` is byte identical to its recorded render by full SHA-256 in `ae32` and `dest`, the two output files recorded for them, except 16 of `Shake`, which changed with the default-xy head form, and 16 of `S_DissolveShake`, which changed with the sine scale of the shake offset; 4 of those 32 are counted units, one for each of the four comparisons that differ from the original, and the other 28 are control units (see those effects). `BlurMoCurves` and `Shake` are listed by the names of their host effects; neither string is in the [name bank](glossary.md#name-bank)'s list of effect names.

The *verified render path* of an effect is the host arrangement its verified comparisons used: one band, full-frame geometry, parameters in the compiler's units and constant over time, one thread, the licensed state, 32-bit float images and the effect's own host settings. The stages of `sapphire stage apply` run each style on that path (see [Adding a provider](stages.md#effects-provider)).

## Using the effects

Every effect below is run the same way: build `sapphire`, write some raw frames at depth 32 and pipe them through the stage. The commands are run from the repository root.

```sh
cargo build --release --offline -p fx_exec -p neutral-handoff
ffmpeg -f lavfi -i testsrc=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le in.raw
```

The effect stages are linked into `sapphire`, so no `fx_run` executable is needed; `fx_run` is built by the same command and is used for `fx_run --describe-params` and for rendering an effect program directly. Each section gives one worked command that reads `in.raw` and writes `out.raw`. The parameter values in the commands are values the verified comparisons used.

The parameter tables list each stage's parameters as `sapphire stage list --json` prints them: the option that sets it, the stage type and the number of lanes of the value `V`. A parameter with more than one lane is passed as comma separated numbers, for example `--param blur_rel=0.5,1.25`. A parameter that is not passed keeps its registered default. The defaults and valid ranges are printed by `fx_run --describe-params PROGRAM`, for example `target/release/fx_run --describe-params Trails_Autogen`; the stage refuses a value outside the range with exit 3.

Parameters whose names contain `mocha` belong to the mask controls of the original. They are offered because verified comparisons set them, but mask tracking data is not part of the rebuild ([Limits common to the effects](#limits-common-to-the-effects)).

Popup parameters take the zero-based index of the item, so the first item is `0`. The opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`) are the exception: they take a host answer, a one-based item number. On the stages of the first ten effects only Normal (`2`) of the opacity choices renders on the stages' host arrangement; All_Opaque and As_Premult are refused with exit 3 (MEASURED), and `cset_output_opacity` of `time_warp_rgb_autogen` renders both of its items. `glow`, `swishpan`, the three `Shake` stages and the `S_DissolveShake` stage render all three answers: each answer runs the host arrangement of its own recorded renders ([Host variants](stages.md#effects-provider)).

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
| `--param subpixel=V` | checkbox | 1 |
| `--param offset_darks=V` | float | 1 |
| `--param filter=V` | int | 1 |
| `--param combine_masks=V` | int | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param invert_mocha_mask=V` | checkbox | 1 |
| `--param resize_mocha_mask=V` | float | 1 |
| `--param resize_mocha_rel_x=V` | float | 1 |
| `--param resize_mocha_rel_y=V` | float | 1 |
| `--param bypass_mocha_mask=V` | checkbox | 1 |
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
| `--param bypass_mocha_mask=V` | checkbox | 1 |
| `--param combine_masks=V` | int | 1 |
| `--param invert_mocha_mask=V` | checkbox | 1 |
| `--param mocha_mask_opacity=V` | float | 1 |
| `--param resize_mocha_mask=V` | float | 1 |
| `--param resize_mocha_rel_x=V` | float | 1 |
| `--param resize_mocha_rel_y=V` | float | 1 |
| `--param show_mocha_mask_only=V` | checkbox | 1 |

`diffuse_amount` and `blur_mocha_mask` are the values as the host passes them; `diffuse_amount_host` and `blur_mocha_mask_host` set the same parameters in the effect's own units.

```sh
target/release/sapphire stage apply diffuse_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param diffuse_amount=48 --param rel_amount=1,1 --param wrap=2,2 < in.raw > out.raw
```

Parity: 31 verified comparisons, 54 recorded renders and 54 re-rendered units, all equal. One approximate reciprocal instruction is reached, inside a logarithm; its effect on the output is 0 ulp under the emulator's bound (MEASURED). 31 of 42 counted recorded renders match the stage's host arrangement.

### `S_DissolveShake`

A transition between two clips that dissolves from the first clip to the second while it shakes the picture. One progress value drives both parts, from 0 at the start of the transition to 1 at its end: the dissolve mixes the two clips by the progress, made steeper around the middle by `dissolve_speed`, and the shake grows from nothing at the start to its full size at the middle and falls back to nothing at the end. With `auto_trans` 1 the progress runs over the whole clip given to the stage, from its first frame to its last; with `auto_trans` 0 the progress is `dissolve_amt`. `transition_dir` swaps which clip leaves and which arrives. The shake is the Normal style of `Shake`, applied to the mixed picture, with the same amplitude, frequency, axis, phase, motion blur, wrap and channel parameters; its motion blur length is `mo_blur_length`.

| style | stage |
|---|---|
| Dissolve_Shake | `stage apply dissolve_shake` |

This effect has no wrapper program; the stage runs its registered program, Dissolve_Shake. It is a transition stage ([Transition stages](cli.md#transition-stages)): the second clip is `--layer To=PATH`, and `--frames N` gives the length of the transition, which spans the clip from its first frame to its last. The layer is optional; without it the first clip dissolves into transparent black. Where the dissolve shows only one of the two clips, before it starts or after it ends, the compiled plan reads only that clip, and the stage renders the frame without the other one ([Transition stages](stages.md#effects-provider)). The shake reads the frame time and the frame rate, so the stage also needs `--fps`; without it the run is refused with `FPS-UNGIVEN` (exit 3) and no frame is written. As a pipeline stage (`--stage @dissolve_shake`) it is refused with `FRAMES-REQUIRED`, because the pipeline passes no `--frames`. For `auto_trans`, `stage list` prints the registered default, 1, and `fx_run --describe-params Dissolve_Shake` prints the program text default, 0; an unset `auto_trans` takes the registered default (MEASURED: an unset run equals `auto_trans=1` and differs from `auto_trans=0`; [Parameter discovery](cli.md#parameter-discovery)).

| parameter | type | lanes |
|---|---|---|
| `--param transition_dir=V` | int | 1 |
| `--param auto_trans=V` | int | 1 |
| `--param dissolve_amt=V` | float | 1 |
| `--param dissolve_speed=V` | float | 1 |
| `--param amplitude=V` | float | 1 |
| `--param frequency=V` | float | 1 |
| `--param motion_blur=V` | checkbox | 1 |
| `--param mo_blur_length=V` | float | 1 |
| `--param seed=V` | float | 1 |
| `--param wrap=V` | text | 2 |
| `--param x_rand_amp=V` | float | 1 |
| `--param x_rand_freq=V` | float | 1 |
| `--param x_wave_amp=V` | float | 1 |
| `--param x_wave_freq=V` | float | 1 |
| `--param x_phase=V` | float | 1 |
| `--param y_rand_amp=V` | float | 1 |
| `--param y_rand_freq=V` | float | 1 |
| `--param y_wave_amp=V` | float | 1 |
| `--param y_wave_freq=V` | float | 1 |
| `--param y_phase=V` | float | 1 |
| `--param z_rand_amp=V` | float | 1 |
| `--param z_rand_freq=V` | float | 1 |
| `--param z_wave_amp=V` | float | 1 |
| `--param z_wave_freq=V` | float | 1 |
| `--param tilt_rand_amp=V` | float | 1 |
| `--param tilt_rand_freq=V` | float | 1 |
| `--param tilt_wave_amp=V` | float | 1 |
| `--param tilt_wave_freq=V` | float | 1 |
| `--param tilt_phase=V` | float | 1 |
| `--param red_amplitude=V` | float | 1 |
| `--param green_amplitude=V` | float | 1 |
| `--param blue_amplitude=V` | float | 1 |
| `--param blue_phase=V` | float | 1 |
| `--param RGB_randomness=V` | float | 1 |
| `--param RGB_frequency=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

`cset_opacity` takes all three answers on this stage: each answer runs the host arrangement of its own recorded renders ([Host variants](stages.md#effects-provider)). The x, y and z amplitudes (`x_rand_amp`, `x_wave_amp`, `y_rand_amp`, `y_wave_amp`, `z_rand_amp` and `z_wave_amp`) take host values, which the runner converts with the frame width on both axes; the tilt amplitudes are record values ([Effects provider](stages.md#effects-provider)).

```sh
ffmpeg -f lavfi -i testsrc2=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le to.raw
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer To=to.raw --param auto_trans=0 --param transition_dir=0 --param dissolve_amt=0.3700000047683716 --param amplitude=1.6999969482421875 --param frequency=5.5 --param x_phase=-0.350006103515625 --param seed=5.0 < in.raw > out.raw
```

Parity: 38 verified comparisons (19 at 64x32 and 19 at 50x7), 304 recorded renders and 304 re-rendered units. Each of the 38 counted parameter sets is rendered and recorded at the eight base phases 0, 4, ..., 28 of the default xy buffer, hence the 304 recorded renders. No approximate reciprocal instruction runs on the verified render path: in the code that renders the verified plans, the original holds only four, inside a logarithm that the mipmap enters only with a nonzero filter or on the GPU path, and every verified render uses filter 0 on the CPU (MEASURED by a static scan of the original's code; that the route reaches none of the four is INFERRED). Two of the verified comparisons differ from the original, one at each size, in every word of the output (8192 of 8192 at 64x32, 1400 of 1400 at 50x7). Their recorded renders were made with two faults: the vertical random amplitude was handed to the runner converted by the frame height, where the original and the runner convert every axis by the frame width, and the sine scale of the shake offset was pi, where the original's is 2 pi. This code has the original's sine scale, and with the amplitude converted by the width the same parameter sets give the original's output byte for byte (MEASURED after the original's output was known, so both stay counted as different). Two more parameter sets, one at each size, are not counted and are not among the recorded renders: their vertical wave amplitude was handed over converted by the frame height too, with the sine scale pi, so their compiled plans differ from the original's in four folded constants; their output is a constant image, which does not show either fault. The Opacity answers 1, 2 and 3 are each verified at 64x32 and 50x7, with input alpha other than 1; answer 3 is verified only without a second clip (an all-zero `To` layer), so its output with a second clip is INFERRED. At All_Opaque the source has no alpha; with a channel parameter away from its default the compile then drops the channel alpha copies and the final alpha assembly, as the original's compile does, which a verified render at 50x7 covers; at 64x32 no verified render covers it (INFERRED to behave the same, since the drop depends on element counts only). No verified render sets a channel parameter at Opacity answer 3: the verified channel renders use answers 1 and 2 and the verified answer 3 renders keep the channel parameters at their defaults, so the output of the channel shake at answer 3 is INFERRED. Each verified render is one frame of a transition of 240 frames at 24 frames a second, at frames 0 to 190, with the second clip absent or present; the time values of other clip lengths and rates are INFERRED. With `auto_trans` 1, after the dissolve ends the effect renders the second clip alone through the shake; that branch is verified only with no second clip (an all-zero `To` layer), so its output for other second clips is INFERRED. At 50x7 the output depends on the phase of the default xy buffer, an input of each run, while the stage uses phase 0 at every size (`--xy-base 0`). 9 verified renders were recorded at a phase other than 0; they are verified through `fx_run` at their recorded phase, and through the stage each is checked against the runner's render at phase 0 (MEASURED). The other 27 of the 36 verified renders that equal the original are byte identical to it through the stage (MEASURED), among them the 8 whose plan reads only one of the two clips ([Transition stages](stages.md#effects-provider)).

### `S_DistortRGB`

Bends the image through a distortion field, with a different strength for the red, green and blue channels, so colours split apart at the distorted edges. The field comes from a blurred lens image, which is the source itself (the stage takes no extra layer). Amount and its relative amount per axis set the strength, the per-channel warp values set each channel's share, and the warp direction can be rotated.

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
| `--param filter=V` | checkbox | 1 |
| `--param rotate_warp_dir=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param amount=1.3125 --param amount_rel=1.1875,0.8125 --param warp_red=0.9375 --param warp_green=0.4375 --param warp_blue=0.1875 --param blur_lens=44 < in.raw > out.raw
```

Parity: 30 verified comparisons, 30 recorded renders and 30 re-rendered units, all equal. 28 are byte identical; in 2 the output holds NaN values, which are equal as NaN values while their payload bits are not compared. No approximate reciprocal instruction is used on the verified render path (MEASURED). 26 of 30 counted recorded renders match the stage's host arrangement.

### `S_DropShadow`

Casts a shadow of the image. The shadow is made from the image's alpha, blurred, shifted, coloured and given its own opacity, then placed under the foreground, whose opacity can be set as well. The shadow is limited by the input's own alpha and composited over nothing else, because the stage takes no extra layer. `comp_premult` chooses how the parts are combined with respect to premultiplied alpha.

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

Parity: 24 verified comparisons, 26 recorded renders and 26 re-rendered units, all equal in both renders. None of the approximate reciprocal instructions in the effect's code is reached on the verified render path (MEASURED for 11 sites; the logarithm sites INFERRED from the plan constants). 20 of 24 counted recorded renders match the stage's host arrangement.

### `S_Flicker`

Changes the brightness of the image from frame to frame. Every pixel of a frame is multiplied, per colour channel, by one factor: `brightness` plus `amplitude` times the sum of a random part and a wave part. The random part is a noise of the frame time at the random frequency and seed, with one value for all channels (scaled by `rand_luma_amp`) and one value per channel (scaled by `rand_color_amp`). The wave part is a sine of the frame time at the wave frequency, with a phase per channel, scaled by `wave_amp`. `red_amp`, `green_amp` and `blue_amp` weight the factor per channel. The alpha of the input is kept.

| style | stage |
|---|---|
| Flicker | `stage apply flicker` |

This effect has no wrapper program; the stage runs its registered program, Flicker. The factor depends on the frame time, so the stage needs `--fps`: frame f is rendered at time f / rate in seconds ([Host values](cli.md#host-values)). Without `--fps` the run is refused with `HOST-VALUE-MISSING` (exit 3) and no frame is written.

| parameter | type | lanes |
|---|---|---|
| `--param amplitude=V` | float | 1 |
| `--param rand_luma_amp=V` | float | 1 |
| `--param rand_color_amp=V` | float | 1 |
| `--param rand_freq=V` | float | 1 |
| `--param wave_amp=V` | float | 1 |
| `--param wave_freq=V` | float | 1 |
| `--param wave_r_phase=V` | float | 1 |
| `--param wave_g_phase=V` | float | 1 |
| `--param wave_b_phase=V` | float | 1 |
| `--param red_amp=V` | float | 1 |
| `--param green_amp=V` | float | 1 |
| `--param blue_amp=V` | float | 1 |
| `--param brightness=V` | float | 1 |
| `--param seed=V` | float | 1 |

```sh
target/release/sapphire stage apply flicker --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=0.330001831 --param rand_luma_amp=0.5 --param rand_color_amp=0.800003052 --param rand_freq=12 --param seed=0.770004272 --param blue_amp=1.69999695 --param brightness=1.19999695 --param wave_g_phase=-0.899993896 --param wave_amp=0.25 < in.raw > out.raw
```

Parity: 24 verified comparisons (12 at 64x32 and 12 at 50x7), 24 recorded renders and 24 re-rendered units, all equal. No approximate reciprocal instruction is used on the verified render path (MEASURED). The verified times are whole frames from 0 to 29 at 24 frames a second; other rates and times between frames run the same code and are INFERRED. The sine of the wave part follows the original's AVX2 and FMA route on every CPU; on a CPU without AVX2 and FMA the original takes another sine routine, which differs from it on 10.7 percent of argument bit patterns by at most 2 ulp (MEASURED on the original's two routines), and those results are not reproduced. The output alpha is the input alpha, verified with input alpha other than 1 at both sizes. 24 of 24 counted recorded renders match the stage's host arrangement.

### `S_Glow`

Adds light around the bright parts of the image. The parts above a threshold are spread out by a wide blur, whose size is set by `width` and scaled per axis (`width_x`, `width_y`) and per colour channel (`width_red`, `width_green`, `width_blue`); the spread light is scaled by the brightness and combined with the source by one of five blend modes (`combine`). `glow_under_source`, `scale_source`, `scale_background`, `light_bg` and `affect_alpha` set how much of the source, the background and the alpha goes into the mix, `show` outputs either the result or the thresholded image the light is made from, and `edge_mode` chooses how the spread treats the border of the frame. Atmosphere is not rendered. The glow stage has no `atmosphere_amp`, and `fx_run` refuses a Glow plan with `atmosphere_amp` other than 0 (`SIMPLEX-GRADIENT-TABLE-ABSENT`): the original's atmosphere noise reads gradient tables that no documented formula reproduces. Its registered default leaves Atmosphere off, so the stage renders without it, and the four parameters that shape the noise are not offered either.

| style | stage |
|---|---|
| Glow | `stage apply glow` |

This effect has no wrapper program; the stage runs its registered program, Glow. The effect's background, matte and tracker mask inputs are not connected: the stage takes no extra layer, and `--layer` is refused with `LAYER-INPUT-UNSUPPORTED`.

| parameter | type | lanes |
|---|---|---|
| `--param width=V` | float | 1 |
| `--param width_x=V` | float | 1 |
| `--param width_y=V` | float | 1 |
| `--param width_red=V` | float | 1 |
| `--param width_green=V` | float | 1 |
| `--param width_blue=V` | float | 1 |
| `--param brightness=V` | float | 1 |
| `--param threshold=V` | float | 1 |
| `--param combine=V` | int | 1 |
| `--param show=V` | int | 1 |
| `--param edge_mode=V` | int | 1 |
| `--param glow_under_source=V` | float | 1 |
| `--param scale_source=V` | float | 1 |
| `--param scale_background=V` | float | 1 |
| `--param light_bg=V` | float | 1 |
| `--param affect_alpha=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |

`cset_opacity` takes all three answers on this stage: each answer runs the host arrangement of its own recorded renders ([Host variants](stages.md#effects-provider)).

```sh
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --fps 24/1 --param width=12.5 --param brightness=1.3500061 --param threshold=0.300003052 --param width_x=1.5 --param width_red=0.800003052 --param edge_mode=0 < in.raw > out.raw
```

Parity: 36 verified comparisons (18 at 64x32 and 18 at 50x7), 36 recorded renders and 36 re-rendered units, all equal. No approximate reciprocal instruction runs on the verified render path: the blur's code holds twelve, six that it does not reach at the one to three planes Glow gives it and six in code tiers that do not run, and no other operation of the route holds one (MEASURED by a static scan of the original's code; that this route reaches none of the twelve is INFERRED). The Opacity answers 1, 2 and 3 are each verified at 64x32 and 50x7, with input alpha other than 1. The verified renders have no mask, no matte and no background: a bound mask or matte, the tracker mask, mocha with tracking data and a background layer are not verified, and the stage does not offer `combine_masks`, which has no effect without a mask. Atmosphere is not rendered (above), and no verified render uses it. 36 of 36 counted recorded renders match the stage's host arrangement.

### `S_HueSatBright`

Adjusts colour: rotates the hue, scales the saturation, scales the brightness and offsets the dark values. With `preserve_luma` the hue rotation keeps each pixel's luma. `scale_colors` scales the red, green and blue channels separately.

| style | stage |
|---|---|
| Hue_Sat_Bright_Autogen | `stage apply hue_sat_bright_autogen` |

| parameter | type | lanes |
|---|---|---|
| `--param brightness=V` | float | 1 |
| `--param bypass_mocha_mask=V` | checkbox | 1 |
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

### `Shake`

Shakes the picture, as a camera does on an impact or in the hand. On every frame the image is moved in x and y, scaled about its centre (the depth axis z) and turned (tilt). The offset of each axis is the sum of a random part, a noise of the frame time, and a wave part, a sine of the frame time. `amplitude` scales every offset and `frequency` sets how fast they change; per axis, a random and a wave amplitude (`x_rand_amp`, `x_wave_amp` and the like), their frequencies and a phase shape the motion, and `phase` shifts the time of all axes. `z_dist` is the scale that the depth offset is added to. With `motion_blur` on, each frame is blurred along the move toward the offset of the next frame, over a length that `motion_blur_length` scales. `wrap` chooses, per axis, what fills the area the move uncovers: nothing, the tiled image or the reflected image. The channel parameters (`red_amplitude`, `green_amplitude` and `blue_amplitude`, the three channel phases, `RGB_randomness` and `RGB_frequency`) shake the red, green and blue channels apart, which splits the colours.

The three styles are the three choices of the effect's style popup. Normal is the continuous shake above. Twitchy runs the shake ten times faster and holds it still between short twitches: a random value of the frame time, taken at `twitch_frequency`, switches the shake on where it is above `stillness`, and the switch is averaged over the frame and its two neighbours. Jumpy moves each offset in jumps between random positions instead of smoothly, and `drift` and `center_bias` shape the jumps.

| style | stage |
|---|---|
| Shake_Normal_Autogen | `stage apply shake_normal_autogen` |
| Shake_Twitchy_Autogen | `stage apply shake_twitchy_autogen` |
| Shake_Jumpy_Autogen | `stage apply shake_jumpy_autogen` |

Each style reads the frame time and the frame rate, so its stage needs `--fps`: frame f is rendered at time f / rate in seconds, and the motion blur and the twitch also read the time one frame later and one frame earlier ([Host values](cli.md#host-values)). Without `--fps` the run is refused with `FPS-UNGIVEN` (exit 3) and no frame is written. The effect's mask input is not connected: the stages fix the three parameters of that input, `mask_use`, `blur_mask` and `invert_mask`, at the values the original holds for an absent mask input ([Mask input](stages.md#effects-provider)).

| parameter | type | lanes | `shake_normal_autogen` | `shake_twitchy_autogen` | `shake_jumpy_autogen` |
|---|---|---|---|---|---|
| `--param amplitude=V` | float | 1 | yes | yes | yes |
| `--param frequency=V` | float | 1 | yes | yes | yes |
| `--param phase=V` | float | 1 | yes | yes | yes |
| `--param stillness=V` | float | 1 | | yes | |
| `--param twitch_frequency=V` | float | 1 | | yes | |
| `--param drift=V` | float | 1 | | | yes |
| `--param center_bias=V` | float | 1 | | | yes |
| `--param z_dist=V` | float | 1 | yes | yes | |
| `--param motion_blur=V` | checkbox | 1 | yes | yes | yes |
| `--param motion_blur_length=V` | float | 1 | yes | yes | yes |
| `--param seed=V` | float | 1 | yes | yes | yes |
| `--param wrap=V` | text | 2 | yes | yes | yes |
| `--param x_rand_amp=V` | float | 1 | yes | yes | yes |
| `--param x_wave_amp=V` | float | 1 | yes | yes | yes |
| `--param x_phase=V` | float | 1 | yes | | |
| `--param y_rand_amp=V` | float | 1 | yes | yes | yes |
| `--param y_rand_freq=V` | float | 1 | yes | | |
| `--param y_wave_amp=V` | float | 1 | yes | yes | yes |
| `--param z_rand_amp=V` | float | 1 | | yes | yes |
| `--param tilt_rand_amp=V` | float | 1 | yes | yes | |
| `--param tilt_wave_amp=V` | float | 1 | yes | | |
| `--param red_amplitude=V` | float | 1 | yes | yes | yes |
| `--param green_amplitude=V` | float | 1 | yes | yes | yes |
| `--param blue_amplitude=V` | float | 1 | yes | yes | yes |
| `--param red_phase=V` | float | 1 | yes | yes | yes |
| `--param green_phase=V` | float | 1 | yes | yes | yes |
| `--param blue_phase=V` | float | 1 | yes | yes | yes |
| `--param RGB_randomness=V` | float | 1 | yes | yes | yes |
| `--param RGB_frequency=V` | float | 1 | yes | yes | yes |
| `--param cset_opacity=V` | int | 1 | yes | yes | yes |

`cset_opacity` takes all three answers on these stages: each answer runs the host arrangement of its own recorded renders ([Host variants](stages.md#effects-provider)). The x, y and z amplitudes take host values, which the runner converts with the frame width on both axes; the tilt amplitudes are record values ([Effects provider](stages.md#effects-provider)).

```sh
target/release/sapphire stage apply shake_normal_autogen --width 64 --height 32 --depth 32 --fps 24/1 --param amplitude=1.6999969482421875 --param frequency=5.5 --param phase=-0.350006103515625 --param x_rand_amp=0.4499969482421875 --param y_rand_freq=2.5 --param z_dist=2.5 < in.raw > out.raw
```

Parity: 68 verified comparisons over the three styles (37 at 64x32 and 31 at 50x7), 2,056 recorded renders and 2,056 re-rendered units. 65 counted parameter sets are rendered and recorded at the eight base phases 0, 4, ..., 28 of the default xy buffer, and three more at 50x7, whose plans hold three default-xy writers, at the 512 combinations of the three writers' phases, hence the 2,056 recorded renders. No approximate reciprocal instruction runs on the verified render path: in the code that renders the verified plans, the original holds only four, inside a logarithm that the mipmap enters only with a nonzero filter or on the GPU path, and every verified render uses filter 0 on the CPU; the mask code of the styles, which holds more, has no node in any verified plan (MEASURED by a static scan of the original's code, the shake offset included; that the route reaches none of the four is INFERRED). The verified times are whole frames from 0 to 139 at 24 frames a second; other rates and times between frames run the same code and are INFERRED. The Opacity answers 1, 2 and 3 are each verified at 64x32 and 50x7, with input alpha other than 1.

Two of the verified comparisons differ from the original: two Twitchy renders at 50x7 at still frames, in 178 and 302 of 1400 words. At such a frame the shake offset is exactly zero, so the compiled plan writes the default xy as a plane that a later exec block reads, and the original gives each row of that plane its own buffer phase. Their recorded renders were made with the runner's former rule, which wrote the default xy as one row buffer for every plan without a perspective node. With the head-form rule of this change the same commands give the original's output byte for byte (MEASURED after the original's output was known, so both stay counted as different). The two forms give the same output when the frame width is a multiple of 8, so the still-frame renders at 64x32 are equal either way. No verified render at a frame width that is not a multiple of 8 judges the plane form.

The channel shake runs three default-xy writers, and the host places the buffer of each at its own phase. The nine channel renders recorded first at 50x7 placed them at different phases, while the runner's renders made for them before the original ran put all three at one phase, so they are not compared and are not among the recorded renders; the same commands with the recorded phases give the original's output in 9 of 9 (MEASURED after the original's output was known). Three more channel renders at 50x7, one per style, were recorded at all 512 combinations of the three phases before the original ran, and each equals the original at the combination its run used (MEASURED), so the channel shake is verified at 50x7 by those three. At 64x32 the phases do not change the output, and the channel renders there count.

At Opacity All_Opaque (`cset_opacity` 1) the source has no alpha; with a channel parameter away from its default the compile then drops the three channel alpha copies and the final alpha assembly, as the original's compile does. No verified `Shake` render covers that case; the drop depends on element counts only (INFERRED). At Opacity answer 3 (`cset_opacity` 3) no verified `Shake` render sets a channel parameter away from its default: the verified channel renders use answer 2 and the verified answer 3 renders keep the channel parameters at their defaults, so the output of the channel shake at answer 3 is INFERRED.

No verified render sets `z_wave_amp` of any style or `z_rand_amp` of Normal away from its default, so the stages do not offer them. Each style hands both to the same depth offset code, which verified renders reach with other values (`z_rand_amp` of Twitchy and Jumpy, and `z_wave_amp` through `S_DissolveShake`), so the output at other values of those four parameters is INFERRED.

The original keeps the three parameters of the unconnected mask input at their text values at every frame size. On the direct `fx_run` route an unbound `blur_mask` is converted from host units instead; the node that reads it is not reached from the output, so the value changes no plan constant and no output (MEASURED on the verified renders). The stages fix all three at their text values.

Where the table search for a position (the phase plus the seed's offset) ends past the last of the 1,024 rows, which happens only for a NaN or positive infinite position, a position of 2^31 or more, or a position less than 0.0004 below a multiple of 1024 of magnitude at most 4096, the original reads the 12 bytes that follow the table. No instruction of the original writes them and they read zero in a recorded Shake render (MEASURED), so the native reads a zero row there. The shake_shift fold of Shake's Jumpy route searches the same table and reads the same row.

At 50x7 the output depends on the phases of the default xy buffers, an input of each run, while the stages use phase 0 for every buffer at every size (`--xy-base 0`). 9 verified renders were recorded at a phase other than 0, and the three channel renders at 50x7 with their three buffers not all at one phase; these 12 are verified through `fx_run` at their recorded phases, and through the stages each is checked against the runner's render at phase 0 (MEASURED). The other 54 of the 66 verified renders that equal the original are byte identical to it through the stages (17 of 22 Normal, 18 of 22 Twitchy and 19 of 22 Jumpy, MEASURED).

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

### `S_SwishPan`

A transition between two clips: the first clip moves out of the frame and the second clip follows it in, to the left, right, up or down (`direction`), with a motion blur along the move. `transition_dir` swaps which clip leaves and which arrives. With `auto_trans` on, the move runs over the whole clip given to the stage, from its first frame to its last, eased at the start and the end by `slow_in` and `slow_out`; with it off, the progress is read from a curve of the host (`host_curve_values`). `overlap` changes how far the second clip trails the first, and `blur_amount` scales the blur, which follows the speed of the move.

| style | stage |
|---|---|
| the registered program, without a wrapper | `stage apply swishpan` |

This effect has no wrapper program; the stage runs its registered program (`fx_run --describe-params SwishPan` prints its parameters; for `auto_trans` it prints the program text default, while an unset `auto_trans` takes the registered default that `stage list` prints, see [Parameter discovery](cli.md#parameter-discovery)). It is one of the two transition stages of this build, with the `S_DissolveShake` stage ([Transition stages](cli.md#transition-stages)): the second clip is `--layer To=PATH`, and `--frames N` gives the length of the transition, which spans the clip from its first frame to its last. The layer is optional; without it the effect runs with no second clip, as some verified renders did. As a pipeline stage (`--stage @swishpan`) it is refused with `FRAMES-REQUIRED`, because the pipeline passes no `--frames`.

| parameter | type | lanes |
|---|---|---|
| `--param transition_dir=V` | int | 1 |
| `--param auto_trans=V` | int | 1 |
| `--param direction=V` | int | 1 |
| `--param blur_amount=V` | float | 1 |
| `--param overlap=V` | float | 1 |
| `--param slow_in=V` | float | 1 |
| `--param slow_out=V` | float | 1 |
| `--param swish_auto_trans=V` | float | 1 |
| `--param cset_opacity=V` | int | 1 |
| `--param host_curve_values=V` | text | a file path |

`cset_opacity` takes all three answers on this stage: each answer runs the host arrangement of its own recorded renders ([Host variants](stages.md#effects-provider)). `host_curve_values` names a curve file in the recorded format; without it the stage writes a constant curve file, as for `BlurMoCurves`. With `auto_trans` 0 the effect reads its progress, the original's Wipe Percent, from that file; the constant file holds a Wipe Percent of 0 on every frame. The verified renders with `auto_trans` 0 run through the stage with a curve file that holds their recorded Wipe Percent ([Host curves](stages.md#effects-provider)).

```sh
ffmpeg -f lavfi -i testsrc2=size=64x32:rate=25 -frames:v 3 -f rawvideo -pix_fmt gbrapf32le to.raw
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24/1 --frames 3 --layer To=to.raw --param transition_dir=0 --param auto_trans=1 --param direction=2 --param blur_amount=300.0 --param overlap=0.1999969482421875 --param slow_in=0.3000030517578125 --param slow_out=0.8000030517578125 --param swish_auto_trans=0.0 < in.raw > out.raw
```

Parity: 36 verified comparisons (18 at 64x32 and 18 at 50x7), 288 recorded renders and 288 re-rendered units, all equal. One approximate reciprocal instruction runs on the verified render path, inside the logarithm that sizes the mipmap levels; its effect on the output is 0 ulp (MEASURED on that logarithm; that the route enters it, and that the reciprocal estimates of the blur are not reached, is INFERRED). The Opacity answers 1, 2 and 3 are each verified at 64x32 and 50x7. Each verified render is one frame of a transition of 100 to 480 frames at 24 frames a second, at frames 0 to 120, with the second clip absent or present; the time values of other clip lengths and rates are INFERRED. The effect reads the default xy from a one-row host buffer whose base phase is an input of each run (`fx_run --xy-base`): each verified render is rendered and recorded at the eight base phases 0, 4, ..., 28, hence the 288 recorded renders, and the original's output is compared with the one at the phase its run used. At 64x32 the eight phases give the same bytes. At 50x7 four verified renders depend on the phase and were recorded at phase 16; the stage uses phase 0 at every size (`--xy-base 0`), so these four are verified through `fx_run` at phase 16, and through the stage each is checked against the native render at phase 0. 32 of 36 counted recorded renders match the stage's host arrangement.

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
| `--param invert_mocha_mask=V` | checkbox | 1 |
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
| `--param filter=V` | checkbox | 1 |
| `--param z_dist=V` | float | 1 |
| `--param tele_lens_width=V` | float | 1 |
| `--param swing=V` | float | 1 |
| `--param roll=V` | float | 1 |
| `--param latitude=V` | float | 1 |
| `--param wrap_above_horizon=V` | checkbox | 1 |
| `--param wrap=V` | text | 2 |
| `--param cset_opacity=V` | int | 1 |

```sh
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --fps 25/1 --param z_dist=2.5 --param latitude=80 --param tele_lens_width=1.6000061 --param filter=1 < in.raw > out.raw
```

Parity: 19 verified comparisons, 256 recorded renders and 256 re-rendered units. 232 units are byte identical in both renders, every counted one among them. The other 24 are control renders at 50x7 of three bindings whose plan writes the default xy inside the first exec block (Swing and Roll set, or wrap_above_horizon 1). Their recorded renders come from the former rule, which wrote the default xy as a per-row plane for every plan with a perspective node; the runner now writes it as one row buffer, as the original's recorded runs of these bindings show, and each changed unit's `ae32` equals the original's render or the counted recorded render of the same binding at the same buffer phases. The perspective uses approximate reciprocals, so it may differ on real CPUs: by up to 6 ulp in its first output and 12,972 ulp in its second, where values near zero cancel. The verified sizes are 64x32 and 50x7. 10 of 224 counted recorded renders match the stage's host arrangement.

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

- **Effect graphs.** The programs of the twenty styles are built by Rust constructor code in `fx_eval` (`programs_native`). That code is a tool-made translation of the plugin's program rows into Rust: a development tool produced it from the parsed program structure. It holds no program text and reads no table.
- **Parameter and class facts.** The registered parameters, ranges, input indices, class flags and combination tables that the twenty styles need are Rust definitions in the crate `effects_rtdata`. A definition is kept only if it is keyed by one of the 35 programs of the styles, by a class they name, or by a builtin they call. A lookup that has no row misses, as it did with the tables.
- **Sin and cos tables.** The sin and cos routines of the effects need four tables. None is stored; all are computed from their mathematical definition at first use, with a small fixed-point big number (320 bit, written for this repository, no crate):

- The sincosf table has 64 entries: the correctly rounded (sin, cos) of k pi/32, exact 0 and 1 at the axes.
- The double sincos table has 64 entries of eight words: the correctly rounded sin and cos, their rounding tails rounded to 24 bits, cos and -sin rounded to one bit, and the exact remainders of cos and -sin against those.
- The pair region of the scalar sin and cos routine (SVML style): pair j of 0 to 64 is cos(j pi/128) h^3/6 and sin(j pi/128) h^2/2 with h = pi/128; the cos table reads the same pairs from the other end. Two constants are 2/h^2 and 6/h^2.
- The 15 chunks of 1/pi of that routine: frac(2^p/pi)/2^(p-7) with p = 7 + 8(m-2), cut into 29 bits, up to 42 bits, and the rest.

Known limit: none for these tables. Bytes of the original 2304 byte tables that no reader touches are not reproduced.

The `sin` of `S_Flicker` follows the 8 lane AVX2 and FMA sine routine of the original. Its slow route, for large arguments, needs two more tables, computed from their definition in the same way at first use:

- The window table has 256 entries of three words: the 96 bits of floor(2^(e-54) / (2 pi)) modulo 2^96 for each biased exponent e.
- The lookup table has 256 entries of four words for the angle 2 pi k / 256: the cosine rounded to a power of two, the rest of the cosine against it, the sine rounded to f32 and the rest of the sine against that.

- **Noise tables.** The noise of `S_Flicker` reads a hash table and a weight table. Neither is stored; both are computed from their definition at first use:

- The hash table has 256 entries of 32 bits. Entry k is (y1 >> 2) + (y2 << 8) + (y3 << 18) modulo 2^32, where y1, y2 and y3 are three successive values of the generator x -> (4096 x + 150889) mod 714025, which starts at 0 and carries from one entry to the next.
- The weight table has 4608 entries. Entry i is round(4096 w(s)) / 4096 with s = sqrt(i / 4608), where w(s) = 1 - 2 s^2 for s below 0.5 and 2 (1 - s)^2 otherwise, evaluated with the fused operations of the original's AVX2 and FMA route and rounded half away from zero.

- **Shake tables.** The shake offset of `Shake` and `S_DissolveShake` reads a hash table, a weight table, the table of a random sawtooth and the tables of a sine routine. None is stored; all are computed from their definition at first use:

- The hash table and the weight table have the definitions of the two noise tables of `S_Flicker` above, and are computed again in the crate `shake_shift`.
- The random sawtooth table has 1,024 rows, each a hash value e_i, a weight a_i and the running sum c_i of the weights before it. With h(v) four rounds of v -> (v >> 8) + T[v mod 256] over the hash table T, e_0 = h(h(7070) + 7070) and e_i = h(e_(i-1) + 7070). Before normalisation a_i = (m_i x 2^-30)^2, with m_i the low 30 bits of e_i; each weight is then divided by the sum of all 1,024 weights times 2^-10, so the weights average 1. Every operation is in single precision, in row order. A row of zeros follows the last row; a search that ends past the table reads it (see `Shake`).
- The sine routine of the shake offset reads 65 pairs and 15 chunks with the definitions of the pair region and the 15 chunks of 1/pi of the scalar sin and cos routine above, computed again in the crate `shake_shift`, and one constant, 1 - 2^-30.

The atmosphere noise of `S_Glow` needs gradient tables that no documented formula reproduces. It is not rendered, and no table of it is stored.

Tests that compare the definitions or the computed tables with copies of the plugin's tables, or sweep whole tables, read those copies from the stored test data under `VEC_DIR` (`effects/rtdata/` and `effects/native_tables/`) and print a `NOT RUN` line when it is absent. No copy is part of the repository.

## Tests

- `cargo test --release` runs the effect crates whose tests need no stored data, with the other default members.
- `cargo test --release -p fx_eval -p fx_exec` runs the rest of the evaluator and the executor. A test that needs stored data from the original (recorded plans, emulator rows, the program corpus) prints a `NOT RUN` line and passes, the way the lift crates' vector tests stay out of a plain `cargo test`.
- `VEC_DIR` names a folder that holds an `effects` folder with the stored data, laid out with the relative paths the tests print in their `NOT RUN` line; with it every test runs. Tests write their files to the system temporary folder, never into the source tree.

## Limits common to the effects

- Every result is judged by the emulator, which follows Intel's specification (SDM), not by real CPU silicon.
- `ROUTE AVX2-ONLY`: the original selects one of several code tiers from the CPU's features. The native code reproduces the tier that runs on CPUs with AVX2 and FMA; on CPUs without them the original runs other tiers with different rounding, and those results are not reproduced.
- Where an operation is sensitive to approximate reciprocals, the effect's section states its bound in ulps.
- The host side is modelled, not rebuilt: the verified render path described above. In the test harness, three of the plugin's licence-family routines were answered with fixed values instead of being run, and the host's licence client found no licence server.
- Not verified: a real host application (its parameter units, its keyframe interpolation, multi band and multi threaded rendering, masks, real frame fetches), the original's GPU renderer, 8 and 16 bit formats, trial or unlicensed rendering, mocha with tracking data, and NaN payload and sign bits.
- Sapphire's own keyframed values between keys are INFERRED (each frame is a plain run at the value the keys give, MEASURED), and its device path is MEASURED on PoCL only.
- Licence, activation, trial and machine identity code of the original is not rebuilt.
- Cases outside an effect's verified comparisons, such as other sizes and parameter values, run the same code and are INFERRED.
- `S_Flicker`, which reads the frame time, was verified with host times at whole frames of a 24 frames a second clock; `Shake` and `S_DissolveShake`, which read the frame time and the frame rate, at whole frames of a 24 frames a second clock as well; `S_SwishPan`, which reads the transition time, at transitions of 100 to 480 frames, and `S_DissolveShake` at transitions of 240 frames. The time values the stages give at other rates and lengths are INFERRED. At a rate whose reduced denominator is not 1 the frame rate and the time follow the original's arithmetic for a host with time scale num and local time step den, and equality with a host's values is INFERRED ([Host values](cli.md#host-values)).
- The Atmosphere of `S_Glow` is not rendered: its noise needs gradient tables that no documented formula reproduces, so the glow stage has no `atmosphere_amp`, and `fx_run` refuses a Glow plan with `atmosphere_amp` other than 0 (`SIMPLEX-GRADIENT-TABLE-ABSENT`). No effect crate ports simplex_noise: `fx_exec` only finds it on the route of a plan in order to refuse that plan.

## Evidence

The native code was written by reading the original plugin's machine code and data. For publication and licensing, see [Legal](../README.md#legal).

The full report, with a paragraph per effect, its exclusion tables and the runs behind it, is kept outside this repository. Its SHA-256 lets a copy be checked:

The full report: `b7ee202da80faf7ce31f95ea5cced46badf3f1b7dacdbec8dc1ea8b865e2910c`
