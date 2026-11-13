# Driving the effect parameters

How to run each of the 25 effect stages with `sapphire stage apply`, what every offered parameter controls, how to write its value, and how the stage hands the value to `fx_run`.

The stages themselves (requirements, host arrangement, exits, limits) are described in [Effects provider](stages.md#effects-provider). This page is about the parameters.

## Conventions on this page

- **INFERRED** marks a description that rests on reading the effect graph (the native program constructors in `crates/effects/fx_eval/src/programs_native.rs` and the parameter definitions in `crates/effects/rtdata`), not on a measurement.
- **MEASURED** marks a statement checked by running the code. "Verified at these values" lists the parameter values at which the stage output is byte identical to a recorded render of the original (MEASURED at depth 32, full SHA-256: by `crates/neutral-handoff/tests/fx_stage_parity.rs` for 382 recorded renders over the 13 stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective`, and for `flicker`, `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage by rendering through `sapphire stage apply` their counted renders that match the stage's host arrangement, 24, 36, 32, 54 and 27, and for the two `S_BlurMotion` stages, `warp_transform_autogen`, the `S_GlowEdges` stage and the `glint` stage the same way, 112, 120, 61, 34 and 22; all byte identical). Values outside those lists run through the same code path and are INFERRED.
- Every worked command below was run on this tree and ended with exit 0 (MEASURED, 64x32 `testsrc` clip, 3 frames, depth 32; the `S_Glint` command on a 64x32 clip of random values, 3 frames, depth 32, and that its exit does not depend on the clip is INFERRED).

## Running a stage

A stage reads whole raw frames on standard input (or `--in FILE`) and writes the same number of frames of the same size and format. Depth 32 frames are `gbrapf32le`; depths 8 and 16 are `rgba` and `rgba64le` and go through a conversion that is not part of the parity record.

```sh
ffmpeg -loglevel error -f lavfi -i testsrc=size=64x32:rate=24 -frames:v 3 -pix_fmt gbrapf32le -f rawvideo clip.raw
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param blur_amount=4 --in clip.raw --out out.raw
```

Each parameter is one `--param ID=VALUE`. A parameter that is not given stays at its registered default, which `fx_run` applies. Inside a pipeline the same stage runs as `--stage @blur_color_autogen` (see [Running in a wrapper](stages.md#running-in-a-wrapper)).

## Reading defaults and ranges

Defaults and ranges are not printed in the documentation. They come from the native definitions and are printed on demand:

```sh
target/release/fx_run --describe-params Blur_Color_Autogen
target/release/fx_run --describe-params Blur_Color_Autogen Drop_Shadow Trails_Autogen
```

The output is JSON: for each program, one entry per registered parameter with `name`, `type` (scalar, boolean, popup, int, color, vector or angle), `lanes`, `min`, `max` and `default` (one value per lane, separated by spaces). The `name` is the program parameter name, which is the stage parameter id except for the few ids listed under each stage below. `target/release/sapphire stage list --json` prints the same defaults and ranges per stage parameter, read from `fx_run` when the list is built, with one difference: for a one-lane popup it prints the default an unset run takes, the item the style registers. Of the offered parameters only `auto_trans` of `swishpan` differs: `fx_run --describe-params SwishPan` prints the program text default `0`, while `stage list` prints `1` and an unset `auto_trans` renders as `1` (MEASURED). For a `--host-param` parameter (below) both print the program's record value, not a host value. The program of each stage is the second column of `crates/neutral-handoff/data/fx_stages.tsv` and the `name` printed by `sapphire stage list`.

The stage refuses a value outside the printed range with exit 3 before any frame is read, for example `bad value for blur_res: 3 is outside 0 to 2` (MEASURED).

## Writing a value

- **Scalar.** A decimal number: `--param brightness=1.25`.
- **Vector and color.** One number per lane, separated by commas with no spaces: `--param blur_rel=0.5,1.5` (two lanes, x then y), `--param color=0.2,0.5,0.8` (red, green, blue, each 0 to 1 for the visible range). A wrong lane count is exit 3.
- **Checkbox.** `0` (off) or `1` (on): `--param subpixel=1`.
- **Integer.** A whole number: `--param shift_frames_red=2`.
- **Popup given as a record value (`--set`).** The zero-based item index, as the effect graph reads it: the first item is `0`. Example: `filter` of `S_Blur` lists Box, Triangle and Gauss, so `--param filter=1` selects Triangle. A two-lane popup such as `wrap` takes one index per axis: `--param wrap=1,2`. The printed range of such a popup is `0` to the item count minus one (MEASURED for `blur_res`).
- **Popup given as a host answer (`cset_*` ids).** The one-based item number, as a host answers a popup: the first item is `1`. These are the opacity popups described below. `stage list --json` prints such an id with `min` 1, `max` the item count of the popup and `default` the answer the style registers, and the stage refuses an answer outside 1 to the item count with exit 3 before any frame is read, for example `bad value for cset_opacity: 9 is outside 1 to 3` (MEASURED).

## How the stage passes a value to fx_run

Each offered parameter has one route, listed in `crates/neutral-handoff/data/fx_params.tsv` (third column) and in the stage sections below.

- **`--set NAME=VALUE`, the record value.** The value goes to the effect graph unchanged, lane by lane. Positions and sizes given this way are in the graph's own normalized units, not pixels (INFERRED).
- **`--host-param NAME=VALUE`, a host value with a unit conversion.** The value is what a host would show on its slider, in units tied to the frame width. `fx_run` converts it to the record value as `record = host x (2 / W) x s` in single precision, where `W` is the frame width in pixels and `s` is the parameter's registered normalize scale from the native definitions (MEASURED against the original for lane 0; the formula is in `crates/effects/fx_eval/src/units.rs`). The same host value therefore makes a proportionally smaller record value on a wider frame. The host value must lie on the 1/65536 grid where the host slider holds a 16.16 value: a value off the grid, such as `4.00001`, is exit 3 with `HOST-VALUE-OFF-GRID` on every `--host-param` parameter but `width` of `S_Glow`, `blur_mocha_mask` of `S_Diffuse` and `shift_host` of `S_DropShadow`, which take an off-grid value as given (MEASURED: exit 0). The `default` printed for a `--host-param` parameter is its record value, not a host value: an unset parameter renders at the host value the style registers, converted at the frame width, and the printed `default` passed as a host value gives another render (MEASURED on `width` of `S_Glow` and `blur_amount` of `S_SwishPan`). Leave the parameter unset to render its default. The conversion uses the frame width for vertical values too: the y amplitudes of the `S_Shake` and `S_DissolveShake` stages divide by the width, not the height (MEASURED against the original at 64x32 and 50x7). The two-lane form `X,Y` exists only for the `shift_host` parameter of the `S_DropShadow` stage: lane 0 converts as above and lane 1 is negated (the host's y axis points the other way).
- **`--cset DISPLAY=N`, a host popup answer.** Used for the opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`). `fx_run` derives the effect's opacity handling from the answer.
- **`--host-curve-values PATH`.** Used only by `host_curve_values` of the two `S_BlurMoCurves` stages and of `swishpan`; see those sections.

A parameter offered under two ids (for example `diffuse_amount` and `diffuse_amount_host` of `diffuse_autogen`) reaches the same program parameter through the two routes. Give only one of them: both together are exit 3 (`given both as --host-param and --set`, MEASURED for the `shift` pair of the `S_DropShadow` stage).

Parameters that a stage fixes (the mask inputs, for example) are not offered and cannot be changed; they are listed per stage in [Effects provider](stages.md#effects-provider).

## Shared parameters

### The opacity popup

`cset_opacity` (display name `Opacity`) is the host popup that tells the effect how to treat the input alpha. Its items are All_Opaque, Normal and As_Premult, so it takes `1`, `2` or `3` (INFERRED from the program layouts). On the stages of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective` only `2` (Normal) renders; `1` and `3` are refused by `fx_run` with exit 3 because the recorded host arrangement does not cover them (MEASURED on every such stage that offers it). On `glow`, `swishpan`, the three `S_Shake` stages and the `S_DissolveShake` stage all three answers render: the stage runs each answer with the host arrangement of its own recorded renders, from the `host_variants` column of [Effects provider](stages.md#effects-provider) (MEASURED). Leaving it unset is the same as `2` for a stage run (INFERRED). `time_warp_rgb_autogen` has `cset_input_opacity` (display name `Input Opacity`, same items and the same result: only `2` renders, MEASURED) and `cset_output_opacity` (display name `Output Opacity`, items All_Opaque and Copy_From_Input; `1` and `2` both render, MEASURED).

No recorded render of `S_Blur`, `S_BlurMoCurves`, `S_Diffuse`, `S_DistortRGB`, `S_DropShadow`, `S_HueSatBright`, `S_Sharpen`, `S_TimeWarpRGB`, `S_Trails` and `S_WarpPerspective` sets an opacity popup. The recorded renders of `glow`, `swishpan`, `S_Shake` and `S_DissolveShake` set `cset_opacity` to `1`, `2` and `3`.

### The tracker mask group

Several stages offer some of `mocha_mask_opacity`, `invert_mocha_mask`, `resize_mocha_mask`, `resize_mocha_rel_x`, `resize_mocha_rel_y`, `shift_mocha_mask`, `blur_mocha_mask`, `bypass_mocha_mask`, `combine_masks` and `show_mocha_mask_only`. They shape the planar tracker mask of the effect graph (INFERRED):

- `resize_mocha_mask` scales the mask, and `resize_mocha_rel_x` and `resize_mocha_rel_y` scale it further per axis; `shift_mocha_mask` (two lanes) moves it (INFERRED).
- `invert_mocha_mask` (checkbox) inverts it; `mocha_mask_opacity` multiplies it; `blur_mocha_mask` softens it (INFERRED).
- `bypass_mocha_mask` (checkbox) replaces it by a full mask (INFERRED).
- `combine_masks` (popup Union, Intersect, Mocha_Only; record values 0, 1, 2) chooses how the tracker mask joins the effect's own mask: maximum, minimum, or the tracker mask alone. The join is taken only when the effect's mask input is connected; with the stage's unconnected mask the tracker mask is used as it is (INFERRED).
- `show_mocha_mask_only` (checkbox) outputs the tracker mask instead of the effect result (INFERRED).

The stages connect no tracker project, so these parameters act on the mask the graph builds without one. They are offered because recorded renders set them, and the verified values below are byte identical to those renders (MEASURED).

## `S_Blur`

Two stages: `blur_color_autogen` (program `Blur_Color_Autogen`) blurs the colour image; `blur_mono_autogen` (program `Blur_Mono_Autogen`) blurs a monochrome version of it. Both offer the same parameters.

- `blur_amount`, scalar, `--host-param` (host units, converted relative to the frame width). The blur width (INFERRED). Verified at `3.295623779296875`, `4.2183990478515625` and `96`.
- `blur_rel`, two lanes, `--set`. Multiplies the blur width per axis, x then y (INFERRED). Verified at `0.573104858,1.25630188`.
- `brightness`, scalar, `--set`. Scales the light parts of the blurred image (INFERRED). Verified at `1.36999512`.
- `offset_darks`, scalar, `--set`. Is subtracted from the brightness gain and then added back to the colour, lifting or lowering the dark parts (INFERRED). Verified at `0.412704468` and `0.628295898` (`blur_color_autogen`), `0.628295898` (`blur_mono_autogen`).
- `scale_opacity`, scalar, `--set`. Scales the alpha of the blurred image (INFERRED). Verified at `0.731903076`.
- `mix_with_source`, scalar, `--set`. Blends the result back towards the input: 0 keeps the blur, 1 returns the input (INFERRED). Verified at `0.337097168` and `0.581298828` (`blur_color_autogen`), `0.337097168` (`blur_mono_autogen`).
- `filter`, popup Box, Triangle, Gauss, `--set` record value 0 to 2. The blur kernel (INFERRED). Verified at `1`.
- `subpixel`, checkbox, `--set`. Sub-pixel accuracy of the blur (INFERRED). Verified at `1`.
- The tracker mask group: `shift_mocha_mask` (verified at `0,0`, `0.19999980926513672,0.09999990463256836` and `0.19999998807907104,0.09999999403953552`), `combine_masks` (`2`), `mocha_mask_opacity` (`0.5`), `invert_mocha_mask` (`1`), `resize_mocha_mask` (`0.5`), `resize_mocha_rel_x` (`0.75`), `resize_mocha_rel_y` (`1.25`), `bypass_mocha_mask` (`1`).
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param blur_amount=4 --param blur_rel=0.5,1.5 --param filter=1 --param cset_opacity=2 --in clip.raw --out out.raw
target/release/sapphire stage apply blur_mono_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param blur_amount=4 --param brightness=1.25 --param mix_with_source=0.25 --in clip.raw --out out.raw
```

## `S_BlurMoCurves`

Two stages: `transform_and_blur_autogen` (program `Transform_and_Blur_Autogen`) moves the image along a transform and blurs it over the shutter interval; `blur_motion_curves_no_transform_autogen` (program `Blur_Motion_Curves_No_Transform_Autogen`) applies only the motion blur and leaves the image in place (INFERRED). The transform itself (rotation, shift, distance and centre over time) comes from the host curve, not from a parameter.

Wall time of one 1920x1080 frame, MEASURED: `blur_motion_curves_no_transform_autogen` about 1.1 s and `transform_and_blur_autogen` about 1.1 s on the CPU (about 52 s and 11 s before the warp kernels stopped copying whole planes per row); the full table is in [Speed](guide.md#speed).

- `host_curve_values`, a file path, `--host-curve-values`. A host curve file in the recorded text format, giving rotation, shift and distance per time sample. Without it the stage writes a constant identity curve (no rotation, no shift, distance 1) carrying every time sample `fx_run` asks for, so the effect then has no motion: the worked `transform_and_blur_autogen` command below returns its input unchanged (MEASURED). A sample missing from a given file is exit 3 with `HOST-CURVE-SAMPLE-UNSERVED`, naming the missing sample (MEASURED). Verified with 13 recorded curve files per stage.
- `shutter_duration`, scalar, `--set`. The length of the shutter interval in frames: the curve is read at `shutter_shift + shutter_duration / 2` and `shutter_shift - shutter_duration / 2` (INFERRED). Any value in the printed range runs: 0, 0.5, 1, 2 and the largest printed value each end with exit 0 on both stages (MEASURED). Verified at `2`.
- `shutter_shift`, scalar, `--set`. Moves the shutter interval earlier or later relative to the frame (INFERRED). Not in the verified lists.
- `exposure_bias`, scalar, `--set`. Weights the start against the end of the interval: the start gets `1 - exposure_bias`, the end `exposure_bias` (INFERRED). Not in the verified lists.
- `scale_result`, scalar, `--set`. Scales the brightness of the result (INFERRED). Verified at `0.765625` (`blur_motion_curves_no_transform_autogen`).
- `blur_res`, popup Full, Half, Quarter, `--set` record value 0 to 2. The resolution the blur is computed at (INFERRED). Verified at `1` and `2` (`blur_motion_curves_no_transform_autogen`).
- `wrap`, two-lane popup No, Tile, Reflect, `--set` record values 0 to 2 per axis. What the effect samples outside the frame (INFERRED). Verified at `1,2` and `2,1` (`blur_motion_curves_no_transform_autogen`).
- `subpixel`, checkbox, `--set`. Sub-pixel accuracy of the sampling (INFERRED). Not in the verified lists.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply transform_and_blur_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param shutter_duration=2 --param shutter_shift=0.5 --param exposure_bias=0.25 --param wrap=1,2 --in clip.raw --out out.raw
target/release/sapphire stage apply blur_motion_curves_no_transform_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param shutter_duration=2 --param blur_res=1 --param scale_result=0.75 --in clip.raw --out out.raw
```

## `S_BlurMotion`

Two stages: `blur_motion_autogen` (program `Blur_Motion_Autogen`) and `blur_motion_mono_autogen` (program `Blur_Motion_Mono_Autogen`), the colour and the mono choice of the effect's Effect popup. Each averages the image along a camera move from a start to an end position about a centre (INFERRED). The mask inputs are not connected and mocha is off: the stage fixes the mask parameters and `show_mocha_mask_only` off.

- `center`, two lanes, `--set`. The point the move turns and scales about (INFERRED). Verified at `0.5625,-1.0625` and `1.0,-1.8600000143051147`.
- `from_z_dist` and `to_z_dist`, scalar, `--set`. The depth distance at the start and at the end of the move (INFERRED). Verified at `1.5`, `2.5` and `3.0` (`from_z_dist`); `1.25` (`to_z_dist`).
- `from_rotate` and `to_rotate`, scalar, `--set`. The rotation at the start and at the end of the move, in degrees (INFERRED). Verified at `-17.0` and `-25.0`, and `-21.0` on the mono stage (`from_rotate`); `-12.0`, `25.0` and `42.0`, and `13.0` on the mono stage (`to_rotate`).
- `from_shift` and `to_shift`, two lanes, `--host-param` (host units). The shift at the start and at the end of the move (INFERRED). Verified at `0.5,-0.25` (`from_shift`); `-0.375,0.125` (`to_shift`).
- `exposure_bias`, scalar, `--set`. Weights the samples along the path (INFERRED). Verified at `0.0`, `0.25`, `0.375`, `0.625`, `0.75` and `1.0`, and `0.125` on the mono stage.
- `blur_res`, popup, `--set` record value 0 to 2: full, half or quarter resolution (INFERRED). Verified at `1` and `2`.
- `wrap`, two-lane popup, `--set` record values 0 to 2 per axis. What fills the area the move brings in from outside the frame (INFERRED). Verified at `0.0,1.0`, `0.0,2.0`, `1.0,0.0` and `2.0,0.0`.
- `subpixel`, checkbox, `--set`. Sub-pixel sampling (INFERRED). Verified at `0`.
- `cset_opacity`: see the opacity popup. All three answers render on these stages. Verified at `1` and `3`; the renders that leave it unset run the default answer.

`to_z_dist` and `to_shift` are offered because verified comparisons set them. The verified sizes are 64x32 and 50x7, at a pixel aspect of 1.0.

```sh
target/release/sapphire stage apply blur_motion_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param center=0.5625,-1.0625 --param from_rotate=-17.0 --param exposure_bias=0.25 --in clip.raw --out out.raw
target/release/sapphire stage apply blur_motion_mono_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param center=0.5625,-1.0625 --param blur_res=1 --in clip.raw --out out.raw
```

## `S_Diffuse`

One stage, `diffuse_autogen` (program `Diffuse_Autogen`). It displaces each pixel by a random offset, which scatters the image (INFERRED).

- `diffuse_amount`, scalar, `--host-param` (host units relative to the frame width). The size of the random displacement (INFERRED). Verified at `0`, `37.5`, `48` and `96`.
- `diffuse_amount_host`, scalar, `--set` of the program parameter `diffuse_amount`. The same displacement given as a record value. Not in the verified lists. Give either this or `diffuse_amount`.
- `rel_amount`, two lanes, `--set`. Scales the displacement per axis (INFERRED). Verified at `0.600006104,1.69999695` and `1,1`.
- `wrap`, two-lane popup No, Tile, Reflect, `--set` record values 0 to 2 per axis. What the displacement samples beyond the frame edge (INFERRED). Verified at `1,2` and `2,2`.
- `blur_mocha_mask`, scalar, `--host-param`, and `blur_mocha_mask_host`, scalar, `--set` of the same program parameter. Softness of the tracker mask (INFERRED). `blur_mocha_mask` is verified at `0` and `8`.
- The tracker mask group: `shift_mocha_mask` (verified at `0,0`, `0,-0`, `0.125,0.09375` and `0.159999996,0.119999997`), `bypass_mocha_mask` (`0`, `1`), `combine_masks` (`0`, `1`), `invert_mocha_mask` (`0`, `1`), `mocha_mask_opacity` (`0.5`, `1`), `resize_mocha_mask` (`0`, `1`, `1.5`), `resize_mocha_rel_x` (`0`, `1`, `1.19999695`), `resize_mocha_rel_y` (`0`, `0.800003052`, `1`), `show_mocha_mask_only` (`0`).

```sh
target/release/sapphire stage apply diffuse_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param diffuse_amount=8 --param rel_amount=1,0.5 --param wrap=2,2 --in clip.raw --out out.raw
```

## `S_DissolveShake`

One stage, `stage apply dissolve_shake`, which runs the registered program itself (no wrapper). It is a transition: the input clip dissolves into a second clip while the picture shakes, most strongly at the middle of the transition (INFERRED). The second clip is `--layer To=PATH` (optional) and the length of the transition is `--frames N` ([Transition stages](cli.md#transition-stages)). The shake reads the frame time and the frame rate, so the stage also needs `--fps`; without it the run is refused with `FPS-UNGIVEN` (exit 3).

- `auto_trans`, popup, `--set` record value. `1` runs the transition over the whole clip, from 0 on the first frame to 1 on the last; `0` takes the progress from `dissolve_amt` (INFERRED). Verified at `0` and `1`.
- `dissolve_amt`, scalar, `--set`. The progress of the transition when `auto_trans` is 0, from 0 to 1 (INFERRED). Verified at `0.28999999165534973`, `0.3700000047683716`, `0.4099999964237213`, `0.4300000071525574`, `0.46000000834465027`, `0.47999998927116394`, `0.49000000953674316`, `0.5099999904632568`, `0.5199999809265137`, `0.550000011920929`, `0.5799999833106995` and `0.6200000047683716`.
- `dissolve_speed`, scalar, `--set`. How steeply the dissolve crosses the middle of the transition (INFERRED). Verified at `1.5`.
- `transition_dir`, popup, `--set` record value. `0` dissolves from the input clip to the second clip; `1` swaps the two (INFERRED). Verified at `0` and `1`.
- `amplitude` and `frequency`, scalar, `--set`. The size of the shake at the middle of the transition and how fast it changes with the frame time (INFERRED). Verified at `0.3699951171875`, `1.25`, `1.6999969482421875`, `2.1999969482421875`, `2.399993896484375` and `3.3000030517578125` (`amplitude`); `4`, `5.5`, `6.5`, `7` and `12.5` (`frequency`).
- `x_rand_amp`, `x_wave_amp`, `y_rand_amp`, `y_wave_amp`, `z_rand_amp` and `z_wave_amp`, scalar, `--host-param` (host units, converted with the frame width on both axes). The random and the wave part of the horizontal move, the vertical move and the depth offset, which scales the picture (INFERRED). Verified at `0.375` and `0.600006103515625` (`x_rand_amp`); `0.4499969482421875` (`x_wave_amp`); `0.1999969482421875` (`y_rand_amp`); `0.399993896484375` and `0.5500030517578125` (`y_wave_amp`); `0.0625` (`z_rand_amp`); `0.125` (`z_wave_amp`).
- `x_rand_freq`, `x_wave_freq`, `y_rand_freq`, `y_wave_freq`, `z_rand_freq` and `z_wave_freq`, scalar, `--set`. The rates of those parts (INFERRED). Verified at `2.5` (`x_rand_freq`); `1.25` (`x_wave_freq`); `0.649993896484375` and `0.75` (`y_wave_freq`); `1.5` (`z_rand_freq`); `0.9499969482421875` (`z_wave_freq`). `y_rand_freq` is verified through `fx_run` only (below).
- `x_phase`, `y_phase` and `tilt_phase`, scalar, `--set`. Shift the time of one axis, in seconds (INFERRED). Verified at `-0.350006103515625` and `0.3000030517578125` (`x_phase`); `-0.1999969482421875` (`y_phase`); `0.3000030517578125` (`tilt_phase`).
- `tilt_rand_amp`, `tilt_wave_amp`, `tilt_rand_freq` and `tilt_wave_freq`, scalar, `--set`. The random and the wave part of the turn, in degrees, and their rates (INFERRED). Verified at `12` (`tilt_rand_amp`); `9` (`tilt_wave_amp`); `2` (`tilt_rand_freq`); `1.5` (`tilt_wave_freq`).
- `motion_blur`, checkbox, `--set`, and `mo_blur_length`, scalar, `--set`. Blur each frame along its move toward the next frame, over that many frame moves (INFERRED). Verified at `0` (`motion_blur`); `1.75` and `3` (`mo_blur_length`).
- `seed`, scalar, `--set`. Picks another random sequence (INFERRED). Verified at `3` and `5`.
- `wrap`, two-lane popup, `--set` record value, one index per axis: `0` No, `1` Tile, `2` Reflect. What fills the area the move uncovers (INFERRED). Verified at `0,1`.
- `red_amplitude`, `green_amplitude`, `blue_amplitude`, `blue_phase`, `RGB_randomness` and `RGB_frequency`, scalar, `--set`. Shake the colour channels apart (see `S_Shake` below; INFERRED). Verified at `0.5` (red), `2` (green), `1.399993896484375` (blue), `0.1999969482421875` (`blue_phase`), `0.399993896484375` (`RGB_randomness`) and `3` (`RGB_frequency`).
- `cset_opacity`: see the opacity popup. All three answers render on this stage. Verified at `1` and `3`; the renders that leave it unset run the default answer.

The verified transitions are 240 frames long (`--frames 240`), at the frames 0 to 190, at 64x32 and 50x7. Nine more counted renders at 50x7 were recorded at another phase of the default xy buffer and are verified through `fx_run` only, so the values that only they set are not listed here (MEASURED: every listed value is set by a counted render that is byte identical through the stage).

```sh
ffmpeg -loglevel error -f lavfi -i testsrc2=size=64x32:rate=24 -frames:v 3 -pix_fmt gbrapf32le -f rawvideo to.raw
target/release/sapphire stage apply dissolve_shake --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --layer To=to.raw --param amplitude=1.25 --param frequency=7 --in clip.raw --out out.raw
```

## `S_DistortRGB`

One stage, `distort_rgb_autogen` (program `Distort_RGB_Autogen`). It takes the gradient of a blurred monochrome lens image (the input itself; the stage takes no `Lens` layer) and displaces the red, green and blue channels along it by different amounts (INFERRED).

- `amount`, scalar, `--set`. The overall strength of the displacement; a negative value pushes the other way (INFERRED). Verified at 9 values from `-0.9375` to `1.5625`.
- `amount_rel`, two lanes, `--set`. Scales the displacement per axis (INFERRED). Verified at 10 value pairs, each lane between `0.6875` and `1.4375`.
- `warp_red`, `warp_green`, `warp_blue`, scalars, `--set`. The displacement of each channel as a multiple of the common displacement (INFERRED). Verified at 9, 10 and 9 values between `0` and `0.9375`.
- `blur_lens`, scalar, `--host-param` (host units relative to the frame width). How much the lens image is blurred before its gradient is taken; it also scales the strength (INFERRED). Verified at 11 values from `0` to `68`.
- `rotate_warp_dir`, angle in degrees, `--set`. Rotates the direction of the displacement (INFERRED). Verified at `-30` and `45`.
- `wrap`, two-lane popup No, Tile, Reflect, `--set` record values 0 to 2 per axis. Edge handling of the displaced sampling (INFERRED). Verified at `0,0`, `0,2`, `1,1` and `2,1`.
- `filter`, checkbox, `--set`. Smooth (filtered) sampling of the displaced channels (INFERRED). Verified at `0`.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param amount=1.5 --param blur_lens=24 --param warp_red=0.25 --param rotate_warp_dir=45 --in clip.raw --out out.raw
```

## `S_DropShadow`

One stage, `sapphire stage apply drop_shadow` (its program has the same name, capitalised). It takes the input's alpha as a matte, moves it, blurs it, colours it, and composites the input over the shadow (INFERRED). The stage takes no `Matte` or `Background` layer. On an opaque input the shadow is hidden behind the input: the worked command below returns its input unchanged (MEASURED); the shadow shows where the input is transparent.

- `shadow_blur`, scalar, `--host-param` (host units relative to the frame width). The softness of the shadow (INFERRED). Verified at 7 values from `0` to `24`.
- `shift`, two lanes, `--set` (record values). The offset of the shadow from the matte (INFERRED). Verified at 7 value pairs, from `0,-0` to `1.25999975,-1.25999975`.
- `shift_host`, two lanes `X,Y`, `--host-param` of the program parameter `shift` (host units relative to the frame width; lane 1 is negated). The same offset given as a host value. Verified at `0,0`. Give either `shift` or `shift_host`.
- `color`, three lanes (red, green, blue), `--set`. The shadow colour (INFERRED). Verified at 8 colours.
- `shadow_opacity`, scalar, `--set`. The strength of the shadow (INFERRED). Verified at 10 values from `0` to `1`.
- `fg_opacity`, scalar, `--set`. The opacity of the input composited over the shadow (INFERRED). Verified at 9 values from `0.736099243` to `1`.
- `comp_premult`, checkbox, `--set`. Composites the input as premultiplied colour (1) or by a straight blend (0) (INFERRED). Verified at `0` and `1`.
- `combine_masks`, popup Union, Intersect, Mocha_Only, `--set` record value 0 to 2. Here the join is between the matte and the tracker mask, and it is always taken (INFERRED). Verified at `0`, `1` and `2`.
- `mocha_mask_opacity`, scalar, `--set`. See the tracker mask group. Verified at `0.327102661`, `0.5` and `1`.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply drop_shadow --width 64 --height 32 --depth 32 --frames 3 \
  --param shadow_blur=4 --param shift_host=4,4 --param color=0.2,0.5,0.8 --param shadow_opacity=0.9 --in clip.raw --out out.raw
```

## `S_Flicker`

One stage, `flicker`, which runs the registered program itself (no wrapper). Every pixel of a frame is multiplied, per colour channel, by one factor that changes with the frame time (INFERRED). The time comes from `--fps`, so the stage needs it ([Time and transitions](guide.md#time-and-transitions)); without it the run is refused with `HOST-VALUE-MISSING` (exit 3).

- `brightness`, scalar, `--set`. The base of the factor: with the random and wave parts at 0 the image is multiplied by it (INFERRED). Verified at `0.800003052`, `0.899993896`, `1.1000061` and `1.19999695`.
- `amplitude`, scalar, `--set`. Scales the random and wave parts together (INFERRED). Verified at `0`, `0.330001831`, `0.449996948` and `0.600006104`.
- `rand_luma_amp`, scalar, `--set`. The random part common to the three channels (INFERRED). Verified at `0`, `0.5` and `1.5`.
- `rand_color_amp`, scalar, `--set`. A random part per channel, which tints the change (INFERRED). Verified at `0`, `0.600006104` and `0.800003052`.
- `rand_freq`, scalar, `--set`. How fast the random part changes with the frame time (INFERRED). Verified at `0.5`, `12` and `20`.
- `seed`, scalar, `--set`. Picks another random sequence (INFERRED). Verified at `0.309997559` and `0.770004272`.
- `wave_amp` and `wave_freq`, scalar, `--set`. The size of the wave part and its frequency in cycles a second of frame time (INFERRED). Verified at `0`, `0.25`, `0.5`, `0.600006104` and `1.5` (`wave_amp`); `2.5`, `3.5` and `17` (`wave_freq`).
- `wave_r_phase`, `wave_g_phase` and `wave_b_phase`, scalar, `--set`. The phase of the wave per channel, in cycles (INFERRED). Verified at `-0.399993896` and `0.649993896` (red); `-0.899993896` and `-0.199996948` (green); `-0.699996948` and `0.149993896` (blue).
- `red_amp`, `green_amp` and `blue_amp`, scalar, `--set`. Weight the random and wave parts per channel (INFERRED). Verified at `1.30000305` (red); `0.699996948` and `2.19999695` (green); `1.69999695` (blue).

The verified times are the frames 0, 2, 3, 5, 11, 13, 19 and 29 of a 24 frames a second clock, at 64x32 and 50x7.

```sh
target/release/sapphire stage apply flicker --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param amplitude=0.5 --param rand_color_amp=0.6 --param wave_amp=0.5 --param wave_freq=2.5 --in clip.raw --out out.raw
```

## `S_Glint`

One stage, `glint`, which runs the registered program itself (no wrapper). It draws rays of light out of the parts of the image above a threshold and combines them with the source by a screen blend (INFERRED). The background, sparks matte and mask inputs are not connected and mocha is off; Matte Type and Combine Masks are not offered. A frame narrower than 2 pixels is refused.

- `threshold`, scalar, `--set`. The level above which a part of the image gives rays (INFERRED). Verified at `0.25`, `0.3000030517578125` and `0.350006103515625`.
- `size`, scalar, `--host-param` (host units). The length of the rays (INFERRED). Verified at `300`.
- `size_x`, `size_y` and `size_diag1`, scalar, `--set`. Scale the length per direction: along the rows, the columns and a diagonal (INFERRED). Verified at `1.5` (`size_x`); `0.75` (`size_y`); `0.5` (`size_diag1`).
- `size_red` and `size_blue`, scalar, `--set`. Scale the length per colour channel (INFERRED). Verified at `0.5` (red); `2.0` (blue).
- `brightness`, scalar, `--set`. Scales the rays (INFERRED). Verified at `0.8000030517578125`, `1.25` and `1.5`.
- `brightness_x` and `brightness_y`, scalar, `--set`. Scale the rays per direction (INFERRED). Verified at `1.25` (`brightness_x`); `0.5` (`brightness_y`).
- `hue_shift`, scalar, `--set`. Turns the hue of the rays and keeps their brightness (INFERRED). Verified at `-0.5`.
- `glint_from_alpha`, scalar, `--set`. Takes the rays from the alpha instead of the colour (INFERRED). Verified at `1.0`.
- `scale_source` and `scale_background`, scalar, `--set`. Scale the source and the background in the mix (INFERRED). Verified at `0.75` (`scale_source`); `0.5` (`scale_background`).
- `affect_alpha`, scalar, `--set`. How much the rays add to the alpha (INFERRED). Verified at `0.5`.
- `cset_opacity`: see the opacity popup. The answers 2 and 3 render on this stage; the answer 1 is refused with `SOURCE-VARIANT-MISMATCH` (exit 3). Verified at `2` and `3`; the renders that leave it unset run the default answer.

The verified sizes are 64x32 and 50x7, at a pixel aspect of 1.0. At the registered `threshold` the verified test images give no rays, so the default parameters verify only the pass-through (see `S_Glint` in [Effects](effects.md)).

```sh
target/release/sapphire stage apply glint --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param threshold=0.350006103515625 --param size=300 --param size_red=0.5 --param size_blue=2.0 --in clip.raw --out out.raw
```

## `S_Glow`

One stage, `glow`, which runs the registered program itself (no wrapper). It spreads the parts of the image above a threshold into a wide glow and combines the glow with the source (INFERRED). The background, matte and tracker mask inputs of the program are not connected: the stage offers no mask parameter, and `--layer` is refused.

- `threshold`, scalar, `--set`. The level above which a part of the image glows (INFERRED). Verified at `0.300003052`, `0.350006104`, `0.550003052` and `0.699996948`.
- `width`, scalar, `--host-param` (host units, converted relative to the frame width). The size of the glow (INFERRED). Verified at `12.5` and `20.75`.
- `width_x` and `width_y`, scalar, `--set`. Scale the size per axis (INFERRED). Verified at `0.699996948` and `1.5` (`width_x`); `1.30000305` and `2.19999695` (`width_y`).
- `width_red`, `width_green` and `width_blue`, scalar, `--set`. Scale the size per colour channel, so the colours of the glow spread by different amounts (INFERRED). Verified at `0.800003052` (red); `1.6000061` (green); `0.600006104` and `1.8999939` (blue).
- `brightness`, scalar, `--set`. Scales the glow (INFERRED). Verified at `1.1000061`, `1.3500061`, `1.6000061`, `2.6000061` and `3.19999695`.
- `combine`, popup, `--set` record value. How the glow joins the source, one of five blend modes given by its zero-based index (INFERRED). Verified at `0`, `1`, `3` and `4`.
- `show`, popup, `--set` record value. `0` outputs the result and `1` the thresholded image the glow is made from (INFERRED). Verified at `1`.
- `edge_mode`, popup, `--set` record value. How the spread treats the border of the frame (INFERRED). Verified at `0`.
- `glow_under_source`, scalar, `--set`. Moves the glow from over the source to under it (INFERRED). Verified at `0.75`.
- `scale_source` and `scale_background`, scalar, `--set`. Scale the source and the background in the mix; with no background connected the background is empty (INFERRED). Verified at `0.600006104` and `0.800003052` (`scale_source`); `1.3999939` (`scale_background`).
- `light_bg`, scalar, `--set`. How much the glow lights the background (INFERRED). Verified at `0.300003052`.
- `affect_alpha`, scalar, `--set`. How much the glow adds to the alpha (INFERRED). Verified at `0.25` and `0.5`.
- The atmosphere parameters are not offered: Atmosphere is not rendered, and `atmosphere_amp` keeps its registered default, which leaves Atmosphere off (see `S_Glow` in [Effects](effects.md)).
- `cset_opacity`: see the opacity popup. All three answers render on this stage. Verified at `1` and `3`; the renders that leave it unset run the default answer.

The verified sizes are 64x32 and 50x7.

```sh
target/release/sapphire stage apply glow --width 64 --height 32 --depth 32 --frames 3 \
  --param combine=1 --param glow_under_source=0.75 --param cset_opacity=3 --in clip.raw --out out.raw
```

## `S_GlowEdges`

One stage, `stage apply glow_edges`, which runs the registered program itself (no wrapper). It finds the edges of the image, spreads them into a glow and combines the glow with the source (INFERRED). The mask inputs are not connected and mocha is off. A frame narrower than 2 pixels is refused.

- `width`, scalar, `--host-param` (host units). The size of the glow (INFERRED). Unset, it is passed as its registered default. Verified at `20`, `26.57000732421875`, `41.30000305175781`, `62.899993896484375` and `78.39999389648438`.
- `width_x` and `width_y`, scalar, `--set`. Scale the size per axis (INFERRED). Verified at `0.600006103515625`, `0.899993896484375` and `1.5` (`width_x`); `0.75` and `1.600006103515625` (`width_y`).
- `width_red`, `width_green` and `width_blue`, scalar, `--set`. Scale the size per colour channel (INFERRED). Verified at `0.8000030517578125` (red); `1.350006103515625` (green); `2.1999969482421875` (blue).
- `edges_smooth`, scalar, `--host-param` (host units). How much the image is smoothed before its edges are found (INFERRED). Verified at `1.17999267578125`, `1.2100067138671875` and `1.42999267578125`.
- `edges_brightness`, scalar, `--set`. Scales the edges (INFERRED). Verified at `0.67999267578125`.
- `edges_threshold`, scalar, `--set`. The level below which an edge does not glow (INFERRED). Verified at `0.17999267578125` and `0.2899932861328125`.
- `glow_brightness`, scalar, `--set`. Scales the glow (INFERRED). Verified at `0.7100067138671875` and `1.6999969482421875`.
- `color`, three lanes (red, green, blue), `--set`. The colour of the glow (INFERRED). Verified at `0.501960813999176,0.7529411911964417,0.1882352977991104`.
- `combine`, popup, `--set` record value. How the glow joins the source, one of five blend modes given by its zero-based index (INFERRED). Verified at `0`, `1`, `3` and `4`.
- `show`, popup, `--set` record value. `0` outputs the result and `1` an intermediate image (INFERRED). The second item is not counted as verified (see `S_GlowEdges` in [Effects](effects.md)).
- `glow_from_alpha`, scalar, `--set`. Takes the glow from the alpha (INFERRED). Not counted as verified when on (see `S_GlowEdges` in [Effects](effects.md)).
- `glow_under_source`, scalar, `--set`. Moves the glow from over the source to under it (INFERRED). Verified at `0.600006103515625`.
- `scale_source` and `scale_background`, scalar, `--set`. Scale the source and the background in the mix (INFERRED). Verified at `0.350006103515625` and `0.8000030517578125` (`scale_source`); `0.8000030517578125` (`scale_background`).
- `light_bg`, scalar, `--set`. How much the glow lights the background (INFERRED). Verified at `0.6999969482421875`.
- `affect_alpha`, scalar, `--set`. How much the glow adds to the alpha (INFERRED). Verified at `0.2899932861328125`, `0.42999267578125` and `0.850006103515625`.
- `cset_opacity`: see the opacity popup. All three answers render on this stage. Verified at `1` and `3`; the renders that leave it unset run the default answer.

The verified sizes are 64x32 and 50x7, at a pixel aspect of 1.0.

```sh
target/release/sapphire stage apply glow_edges --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param affect_alpha=0.2899932861328125 --param combine=1 --in clip.raw --out out.raw
```

## `S_HueSatBright`

One stage, `hue_sat_bright_autogen` (program `Hue_Sat_Bright_Autogen`). It rotates the hue, scales the saturation, then scales the lights and offsets the darks (INFERRED). It offers no opacity popup.

- `hue_shift`, scalar, `--set`. The hue rotation, as a fraction of a full turn (INFERRED). Verified at 38 values from `-0.789993286` to `0.86000061`.
- `preserve_luma`, checkbox, `--set`. Keeps the luma of each pixel while the hue rotates (INFERRED). Verified at `0` and `1`.
- `saturation`, scalar, `--set`. Scales the saturation: 1 keeps it, 0 removes it, a negative value inverts the colour around the grey (INFERRED). Verified at 32 values from `-0.800003052` to `1.88000488`.
- `brightness`, scalar, `--set`. Scales the light parts (INFERRED). Verified at 22 values from `0` to `1.83000183`.
- `scale_colors`, three lanes (red, green, blue), `--set`. A per-channel factor on top of `brightness` (INFERRED). Verified at 14 colours.
- `offset_darks`, scalar, `--set`. Added to every channel after the scaling, lifting or lowering the dark parts (INFERRED). Verified at 24 values from `-1.3999939` to `1.27000427`.
- `mocha_mask_opacity` (verified at 20 values from `0.0899963379` to `0.910003662`) and `bypass_mocha_mask` (`1`): see the tracker mask group.

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param hue_shift=-0.25 --param saturation=1.5 --param brightness=0.8 --param scale_colors=1,0.9,0.8 \
  --param preserve_luma=1 --in clip.raw --out out.raw
```

## `S_Shake`

Three stages, `shake_normal_autogen`, `shake_twitchy_autogen` and `shake_jumpy_autogen`, one per style; each runs the style's wrapper program. On every frame the picture is moved, scaled and turned by offsets that change with the frame time (INFERRED). The time and the frame rate come from `--fps`, so the stages need it ([Time and transitions](guide.md#time-and-transitions)); without it the run is refused with `FPS-UNGIVEN` (exit 3). The mask input of the programs is not connected, its three parameters are fixed, and no mask parameter is offered. A parameter that only some of the styles offer names them.

- `amplitude`, scalar, `--set`. Scales every offset (INFERRED). Verified at `0.600006103515625` and `1.6999969482421875` (Normal), `1.399993896484375` (Twitchy) and `1.3000030517578125` (Jumpy).
- `frequency`, scalar, `--set`. How fast the offsets change with the frame time; Twitchy runs ten times this rate (INFERRED). Verified at `5.5` (Normal), `6` (Twitchy) and `3.5` (Jumpy).
- `phase`, scalar, `--set`. Shifts the time of every axis, in seconds (INFERRED). Verified at `-0.350006103515625` (Normal), `-0.399993896484375` (Twitchy) and `-0.600006103515625` (Jumpy).
- `x_rand_amp` and `x_wave_amp`, scalar, `--host-param` (host units, converted with the frame width). The random and the wave part of the horizontal move (INFERRED). Verified at `0.350006103515625` and `0.4499969482421875` (Normal), `0.350006103515625` and `0.5` (Twitchy), `0.350006103515625` and `0.399993896484375` (Jumpy) for `x_rand_amp`; `0.149993896484375` and `0.3000030517578125` (Normal), `0.3000030517578125` (Twitchy), `0.25` and `0.3000030517578125` (Jumpy) for `x_wave_amp`.
- `y_rand_amp` (every style; no value of Normal is verified through the stage) and `y_wave_amp`, scalar, `--host-param` (host units, converted with the frame width as well). The vertical move (INFERRED). Verified at `0.350006103515625` (Twitchy) and `0.5` (Jumpy) for `y_rand_amp`; `0.25` and `0.3000030517578125` (Normal) and `0.25` (Twitchy and Jumpy) for `y_wave_amp`.
- `z_rand_amp` (Twitchy and Jumpy), scalar, `--host-param`. The random part of the depth offset, which scales the picture (INFERRED). Verified at `0.149993896484375` (Twitchy) and `0.1999969482421875` (Jumpy).
- `z_dist` (Normal and Twitchy), scalar, `--set`. The scale that the depth offset is added to (INFERRED). Verified at `2.5` (Normal) and `1.5` (Twitchy).
- `x_phase` and `y_rand_freq` (Normal), scalar, `--set`. The time shift of the horizontal offset, in seconds, and the rate of the vertical random part (INFERRED). Verified at `-0.1999969482421875` and `2.5`.
- `tilt_rand_amp` (Normal and Twitchy) and `tilt_wave_amp` (Normal), scalar, `--set`. The random and the wave part of the turn, in degrees (INFERRED). Verified at `4` (Normal) and `6` (Twitchy) for `tilt_rand_amp`; `7` for `tilt_wave_amp`.
- `stillness` and `twitch_frequency` (Twitchy), scalar, `--set`. A random value of the frame time, taken at the twitch frequency, switches the shake on where it is above `stillness`, so a larger `stillness` holds the picture still for longer (INFERRED). Verified at `0.350006103515625` and `0.899993896484375` (`stillness`); `1.25` and `4.5` (`twitch_frequency`).
- `drift` and `center_bias` (Jumpy), scalar, `--set`. Shape the jumps between positions (INFERRED). Verified at `0.100006103515625` and `0.649993896484375` (`drift`); `0.8000030517578125` and `2` (`center_bias`).
- `motion_blur`, checkbox, `--set`, and `motion_blur_length`, scalar, `--set`. Blur each frame along its move toward the next frame, over that many frame moves (INFERRED). Verified at `1` and `3`.
- `seed`, scalar, `--set`. Picks another random sequence (INFERRED). Verified at `13`.
- `wrap`, two-lane popup, `--set` record value, one index per axis: `0` No, `1` Tile, `2` Reflect. What fills the area the move uncovers (INFERRED). Verified at `1,0`.
- `red_amplitude`, `green_amplitude`, `blue_amplitude`, `red_phase`, `green_phase`, `blue_phase`, `RGB_randomness` and `RGB_frequency`, scalar, `--set`. Shake the red, green and blue channels apart: per channel a scale of the offsets and a time shift, and a random part that differs per channel at its own rate (INFERRED). Verified at `1.5`, `2` and `0.6999969482421875` (the three amplitudes), `-0.100006103515625`, `0.0800018310546875` and `0.1999969482421875` (the three phases), `0.399993896484375` and `0.899993896484375` (`RGB_randomness`) and `3` (`RGB_frequency`).
- `cset_opacity`: see the opacity popup. All three answers render on these stages. Verified at `1` and `3`; the renders that leave it unset run the default answer.

The verified times are whole frames of a 24 frames a second clock, from 0 to 47 (Normal), 53 (Twitchy) and 58 (Jumpy), at 64x32 and 50x7.

```sh
target/release/sapphire stage apply shake_twitchy_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param amplitude=1.4 --param stillness=0.35 --param twitch_frequency=4.5 --param x_rand_amp=0.5 --in clip.raw --out out.raw
target/release/sapphire stage apply shake_jumpy_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param amplitude=1.3 --param drift=0.65 --param center_bias=0.8 --param frequency=3.5 --in clip.raw --out out.raw
```

## `S_Sharpen`

Two stages: `sharpen_best_quality_autogen` (program `Sharpen_Best_Quality_Autogen`), an edge-aware sharpen over several detail levels, and `sharpen_fast_autogen` (program `Sharpen_Fast_Autogen`), a single-width sharpen (INFERRED).

Both split the image into luma and chroma. When `sharpen_luma` equals `sharpen_chroma`, the whole image is sharpened in one pass with the per-channel gains `sharpen_red`, `sharpen_green` and `sharpen_blue`; otherwise luma and chroma are sharpened separately and added (INFERRED).

Shared by both stages:

- `sharpen_amp`, scalar, `--set`. The overall strength (INFERRED). Verified at `0`, `1`, `1.25` and `1.75`.
- `sharpen_luma`, scalar, `--set`. The strength on the luma (INFERRED). Verified at `0`, `0.5`, `0.75` and `1`.
- `sharpen_chroma`, scalar, `--set`. The strength on the chroma (INFERRED). Verified at `0.75`, `1`, `1.25` and `1.5`.
- `sharpen_red`, `sharpen_green`, `sharpen_blue`, scalars, `--set`. Per-channel gains (INFERRED). Verified at `-0.5`, `0.75`, `1`, `1.5` (red); `-0.25`, `0.5`, `1`, `1.25` (green); `0.5`, `1` (blue).
- `cset_opacity`: see the opacity popup.

Only `sharpen_best_quality_autogen`:

- `small_detail_size`, scalar, `--host-param` (host units relative to the frame width). The size of the small details; the tiny level is half of it (INFERRED). Verified at `0`, `4` and `6`.
- `edge_threshold`, scalar, `--set`. How strong an edge must be before it is protected from the sharpen (INFERRED). Verified at `0`, `0.25` and `0.350006104`.
- `scale_tiny_details`, `scale_small_details`, `scale_medium_details`, `scale_large_details`, scalars, `--set`. The gain of each detail level (INFERRED). Verified at `1.75`, `2.5`, `3` (tiny); `0.75`, `1.25`, `1.5` (small); `0.5`, `1`, `1.5` (medium); `2`, `2.25`, `3.5` (large).

Only `sharpen_fast_autogen`:

- `sharpen_width`, scalar, `--host-param` (host units relative to the frame width). The width of the sharpen (INFERRED). Verified at `0`, `26.879989624023438`, `60` and `100`.

```sh
target/release/sapphire stage apply sharpen_best_quality_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param small_detail_size=4 --param sharpen_amp=1.25 --param edge_threshold=0.25 --in clip.raw --out out.raw
target/release/sapphire stage apply sharpen_fast_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param sharpen_width=6 --param sharpen_amp=1.25 --param sharpen_luma=0.5 --in clip.raw --out out.raw
```

## `S_SwishPan`

One stage, `swishpan`, which runs the registered program itself (no wrapper). It is a transition: the input clip moves out of the frame and a second clip follows it in, with a motion blur along the move (INFERRED). The second clip is `--layer To=PATH` (optional) and the length of the transition is `--frames N` ([Transition stages](cli.md#transition-stages)).

- `direction`, popup, `--set` record value. The direction of the move: `0` left, `1` right, `2` up, `3` down (INFERRED). Verified at `0`, `1`, `2` and `3`.
- `transition_dir`, popup, `--set` record value. `0` moves the input clip out and brings the second clip in; `1` swaps the two (INFERRED). Verified at `0` and `1`.
- `auto_trans`, popup, `--set` record value. `1` runs the move over the whole clip, from 0 on the first frame to 1 on the last; `0` reads the progress from the host curve (INFERRED). Verified at `0` and `1`.
- `slow_in` and `slow_out`, scalar, `--set`. Ease the start and the end of the move (INFERRED). Verified at `0.3000030517578125`, `0.399993896484375`, `0.5` and `0.6999969482421875` (`slow_in`); `0.149993896484375`, `0.5` and `0.8000030517578125` (`slow_out`).
- `overlap`, scalar, `--set`. Shortens or lengthens the distance between the two clips, so that they overlap or leave a gap (INFERRED). Verified at `-0.3000030517578125`, `0`, `0.1999969482421875` and `0.4499969482421875`.
- `blur_amount`, scalar, `--host-param` (host units, converted relative to the frame width). Scales the motion blur, which follows the speed of the move (INFERRED). Verified at `0`, `300`, `479.9999694824219` and `700`.
- `swish_auto_trans`, scalar, `--set`. A hidden parameter of the program: above 0 the blur follows the eased move, as with `auto_trans` 1 (INFERRED). Verified at `0`, `0.5`, `1.5` and `2`.
- `cset_opacity`: see the opacity popup. All three answers render on this stage. Verified at `1`, `2` and `3`.
- `host_curve_values`, a file path, `--host-curve-values`. The curve file that `auto_trans` 0 reads the progress, the original's Wipe Percent, from. Without it the stage's constant curve file gives a Wipe Percent of 0 (MEASURED: exit 0). Verified with `auto_trans` 0 and curve files that hold a Wipe Percent of 35 and 50 percent (the curve values 22938 and 32768, the fraction times 65536), constant over the render.

The verified transitions are 100, 120, 240 and 480 frames long (`--frames`), at the frames 0, 17, 25, 30, 40, 60, 90 and 120, at 64x32 and 50x7.

```sh
target/release/sapphire stage apply swishpan --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param direction=2 --param slow_in=0.3 --param overlap=0.2 --in clip.raw --out out.raw
```

## `S_TimeWarpRGB`

One stage, `time_warp_rgb_autogen` (program `Time_Warp_RGB_Autogen`). It takes the red, green and blue channels from different frames of the clip (INFERRED).

- `shift_frames_red`, `shift_frames_green`, `shift_frames_blue`, integers, `--set`. The frame offset of each channel: frame `n` takes the channel from frame `n + shift`. Before the first frame the stage clamps to frame 0; past the last frame it holds the last frame (stage behaviour, INFERRED beyond the measured shifts). Verified at `-1`, `2`, `3` (red), `-2`, `1`, `3` (green) and `-3`, `1`, `2` (blue).
- `--param clamp_chroma=V`, scalar, `--set`. Below 1, limits the colour spread (largest minus smallest channel) of the result by reducing its saturation (INFERRED). Verified at `0.25`, `0.5` and `0.75`.
- `cset_input_opacity` and `cset_output_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply time_warp_rgb_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param shift_frames_red=2 --param shift_frames_green=0 --param shift_frames_blue=-1 --param clamp_chroma=0.5 \
  --in clip.raw --out out.raw
```

## `S_Trails`

One stage, `trails_autogen` (program `Trails_Autogen`). Each output frame combines the current frame with the previous output, faded, tinted, blurred and hue shifted, so moving content leaves trails (INFERRED). The stage buffers the whole clip and renders it in one run.

- `new_color`, three lanes, `--set`, and `new_opacity`, scalar, `--set`. The current frame is multiplied by `new_opacity` times `new_color` before it is combined (INFERRED). Verified at 5 colours and at `0.5` and `1.5`.
- `fade`, scalar, `--set`. How much of the previous output carries over (INFERRED). Verified at `0`, `0.5` and `1`.
- `fade_color`, three lanes, `--set`. Tints the carried-over trail (INFERRED). Verified at 5 colours.
- `prev_hue_shift`, scalar, `--set`. Rotates the hue of the trail on every frame, as a fraction of a full turn (INFERRED). Verified at `0.25`.
- `blur_amount`, scalar, `--host-param` (host units relative to the frame width), and `blur_rel`, two lanes, `--set`. Blur the trail on every frame, per axis (INFERRED). Verified at `0`, `1.5625` and `2`, and at `0.5,1.75`.
- `combine`, popup Ave, Max, Screen, Add, Over, Under, Min, `--set` record value 0 to 6. How the current frame and the trail combine (INFERRED). Verified at `2` and `6`.
- `invert_mocha_mask` (verified at `1`) and `mocha_mask_opacity` (`0.5`): see the tracker mask group.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply trails_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param combine=2 --param fade=0.5 --param new_color=0.9,0.4,0.2 --param blur_amount=2 --in clip.raw --out out.raw
```

## `S_WarpPerspective`

One stage, `warp_perspective_autogen` (program `Warp_Perspective_Autogen`). It lays the image on a tilted plane seen in perspective (INFERRED).

- `latitude`, scalar in degrees, `--set`. The tilt of the plane towards the horizon (INFERRED). Verified at `60` and `80`.
- `z_dist`, scalar, `--set`. The distance of the plane from the viewer (INFERRED). Verified at `2.5`.
- `tele_lens_width`, scalar, `--set`. The lens: larger values flatten the perspective (INFERRED). Verified at `1.6000061`.
- `swing`, angle in degrees, `--set`. Rotates the plane after the perspective (INFERRED). Verified at `-12`, `-0` and `40`.
- `roll`, angle in degrees, `--set`. Rotates the image before the perspective (INFERRED). Verified at `-10`, `-0` and `30`.
- `wrap`, two-lane popup No, Tile, Reflect, `--set` record values 0 to 2 per axis. Edge handling on the plane (INFERRED). Verified at `0,2`.
- `wrap_above_horizon`, checkbox, `--set`. Fills the area above the horizon instead of leaving it transparent (INFERRED). Verified at `1`.
- `filter`, checkbox, `--set`. Smooth (filtered) sampling (INFERRED). Verified at `0` and `1`.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param latitude=60 --param swing=-12 --param roll=30 --param z_dist=2.5 --param wrap=0,2 --in clip.raw --out out.raw
```

## `S_WarpTransform`

One stage, `warp_transform_autogen` (program `Warp_Transform_Autogen`). It moves, scales, turns and slants the image in three dimensions (INFERRED). The mask, matte and crop inputs are not connected, mocha is off and the host's motion blur is off.

- `shift`, two lanes, `--set`. The position of the image; `0,0` is the layer centre and renders the image in place. Unset, it is answered with the registered (50, 50) pixels, which moves the image by (50 - W/2, 50 - H/2) pixels and leaves small frames empty (MEASURED, see `S_WarpTransform` in [Effects](effects.md)). Verified at `0.0,0.0`, `0.24,-0.06` and `0.375,0.1875`.
- `scale`, two lanes, `--set`. The size per axis; a negative lane mirrors the image (INFERRED). Verified at `-0.8000030517578125,1.0`, `-1.5,1.0` and `0.6999969482421875,1.3000030517578125`.
- `rotate`, scalar, `--set`. Turns the image in its plane, in degrees (INFERRED). Verified at 10 values from `-25.0` to `20.0`.
- `swivel` and `tilt`, scalar, `--set`. Turn the image about the vertical and the horizontal axis, in degrees (INFERRED). Verified at `3.0`, `4.0`, `6.0` and `12.0` (`swivel`); `-14.0`, `-7.0`, `-6.0` and `-5.0` (`tilt`).
- `z_dist`, scalar, `--set`. The depth distance of the camera (INFERRED). Verified at `0.600006103515625` and `1.600006103515625`.
- `perspective_amount`, scalar, `--set`. How strongly the turned image is foreshortened (INFERRED). Verified at `0.5` and `2.0`.
- `shear`, two lanes, `--set`. Slants the image per axis (INFERRED). Verified at `-0.149993896484375,0.0`, `0.0,0.1199951171875` and `0.25,-0.100006103515625`.
- `wrap`, two-lane popup, `--set` record values 0 to 2 per axis, x then y (the host's values 1, 2 and 3). What fills the area outside the image; `1` and `2` fill it from the image, each in its own way (INFERRED). Verified at `1.0,2.0` and `2.0,1.0`.
- `filter`, checkbox, `--set`. Filtered sampling (INFERRED). Verified at `0`.
- `cset_opacity`: see the opacity popup. All three answers render on this stage. Verified at `1`, `2` and `3`.

The verified sizes are 64x32 and 50x7.

```sh
target/release/sapphire stage apply warp_transform_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param shift=0.0,0.0 --param rotate=-22.0 --param shear=-0.149993896484375,0.0 --in clip.raw --out out.raw
```

## Keyframed values

Every stage but `trails_autogen` accepts a keyed value: `--param 'ID=V@F;V@F;...'`, with `F` an output frame number counted from 0 (fractions allowed) and `--fps` giving the time base. The syntax is the frames/2 convention's: a trailing `h` holds a key until the next one, a trailing `e` eases it, and `iS/I` and `oS/I` before the suffix are Bezier handles (`S` in units a second, `I` the influence in percent, 0.1 to 100). Before the first key the value is the first key's, after the last key the last one's.

- Scalars and angles interpolate: straight between plain keys, eased next to an `e` key, along the handle's curve next to a handle. A scalar that a handle overshoots is held inside its range.
- Two-lane vectors (`shift`, `blur_rel`, `rel_amount` and the like) move on the straight line between their keys.
- Integers, popups (the two-lane `wrap` among them), checkboxes, colours, the opacity answers and `host_curve_values` hold their value until the next key and refuse handles.
- A host value (`--host-param`) between keys is rounded to the 1/65536 grid of a host slider.
- Each key value is checked like a plain value; a bad key, two keys on one frame tick or a missing `--fps` is refused before a frame is read.

```sh
target/release/sapphire stage apply hue_sat_bright_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param 'saturation=0@0;2@2e' --in clip.raw --out out.raw
target/release/sapphire stage apply blur_color_autogen --width 64 --height 32 --depth 32 --fps 24 --frames 3 \
  --param 'blur_amount=2@0o0/50;6@2' --param 'filter=0@0h;2@1' --in clip.raw --out out.raw
```

Each frame of a keyed run equals a plain run at the value the keys give for that frame, byte for byte (MEASURED on five stages). The values between keys are INFERRED.

## Errors while driving parameters

All of these end with exit 3 and an `unsupported` error naming the cause (MEASURED):

- a value outside the range `fx_run --describe-params` prints, or a wrong lane count;
- a popup answer outside 1 to the item count (`POPUP-ANSWER-OUT-OF-RANGE`), or an opacity answer the stage's host arrangement does not cover;
- a host value off the 1/65536 grid (`HOST-VALUE-OFF-GRID`), except on `width` of `glow`, `blur_mocha_mask` and `shift_host`;
- the same program parameter given through both routes;
- a keyed value on `trails_autogen`, a malformed key or two keys on one tick (a keyed value without `--fps` is exit 2);
- a host curve sample that neither the given file nor the stage's identity curve can serve (`HOST-CURVE-SAMPLE-UNSERVED`);
- on `swishpan` and the `S_DissolveShake` stage, a missing or unusable `--frames` or layer (`FRAMES-REQUIRED`, `FRAMES-TOO-SMALL` and the `LAYER-*` refusals of [Refusal labels](cli.md#refusal-labels)).

A run of `flicker` without `--fps` is refused with `HOST-VALUE-MISSING` (exit 3), and a run of the three `S_Shake` stages or the `S_DissolveShake` stage without it with `FPS-UNGIVEN` (exit 3).
