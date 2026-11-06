# Driving the effect parameters

How to run each of the 13 effect stages with `sapphire stage apply`, what every offered parameter controls, how to write its value, and how the stage hands the value to `fx_run`.

The stages themselves (requirements, host arrangement, exits, limits) are described in [Effects provider](stages.md#effects-provider). This page is about the parameters.

## Conventions on this page

- **INFERRED** marks a description that rests on reading the effect graph (the native program constructors in `crates/effects/fx_eval/src/programs_native.rs` and the parameter definitions in `crates/effects/rtdata`), not on a measurement.
- **MEASURED** marks a statement checked by running the code. "Verified at these values" lists the parameter values at which the stage output is byte identical to a recorded render of the original (MEASURED by `crates/neutral-handoff/tests/fx_stage_parity.rs`: 382 recorded renders over the 13 stages, all byte identical, depth 32). Values outside those lists run through the same code path and are INFERRED.
- Every worked command below was run on this tree and ended with exit 0 (MEASURED, 64x32 `testsrc` clip, 3 frames, depth 32).

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

The output is JSON: for each program, one entry per registered parameter with `name`, `type` (scalar, boolean, popup, int, color, vector or angle), `lanes`, `min`, `max` and `default` (one value per lane, separated by spaces). The `name` is the program parameter name, which is the stage parameter id except for the few ids listed under each stage below. `target/release/sapphire stage list --json` prints the same defaults and ranges per stage parameter, read from `fx_run` when the list is built. The program of each stage is the second column of `crates/neutral-handoff/data/fx_stages.tsv` and the `name` printed by `sapphire stage list`.

The stage refuses a value outside the printed range with exit 3 before any frame is read, for example `bad value for blur_res: 3 is outside 0 to 2` (MEASURED).

## Writing a value

- **Scalar.** A decimal number: `--param brightness=1.25`.
- **Vector and color.** One number per lane, separated by commas with no spaces: `--param blur_rel=0.5,1.5` (two lanes, x then y), `--param color=0.2,0.5,0.8` (red, green, blue, each 0 to 1 for the visible range). A wrong lane count is exit 3.
- **Checkbox.** `0` (off) or `1` (on): `--param subpixel=1`.
- **Integer.** A whole number: `--param shift_frames_red=2`.
- **Popup given as a record value (`--set`).** The zero-based item index, as the effect graph reads it: the first item is `0`. Example: `filter` of `S_Blur` lists Box, Triangle and Gauss, so `--param filter=1` selects Triangle. A two-lane popup such as `wrap` takes one index per axis: `--param wrap=1,2`. The printed range of such a popup is `0` to the item count minus one (MEASURED for `blur_res`).
- **Popup given as a host answer (`cset_*` ids).** The one-based item number, as a host answers a popup: the first item is `1`. These are the opacity popups described below. An answer outside 1 to the item count is exit 3 with `POPUP-ANSWER-OUT-OF-RANGE` (MEASURED).

## How the stage passes a value to fx_run

Each offered parameter has one route, listed in `crates/neutral-handoff/data/fx_params.tsv` (third column) and in the stage sections below.

- **`--set NAME=VALUE`, the record value.** The value goes to the effect graph unchanged, lane by lane. Positions and sizes given this way are in the graph's own normalized units, not pixels (INFERRED).
- **`--host-param NAME=VALUE`, a host value with a unit conversion.** The value is what a host would show on its slider, in units tied to the frame width. `fx_run` converts it to the record value as `record = host x (2 / W) x s` in single precision, where `W` is the frame width in pixels and `s` is the parameter's registered normalize scale from the native definitions (MEASURED against the original for lane 0; the formula is in `crates/effects/fx_eval/src/units.rs`). The same host value therefore makes a proportionally smaller record value on a wider frame. The host value must lie on the 1/65536 grid of a host slider; a value off the grid, such as `4.00001`, is exit 3 with `HOST-VALUE-OFF-GRID` (MEASURED). The two-lane form `X,Y` exists only for the `shift_host` parameter of the `S_DropShadow` stage: lane 0 converts as above and lane 1 is negated (the host's y axis points the other way).
- **`--cset DISPLAY=N`, a host popup answer.** Used for the opacity popups (`cset_opacity`, `cset_input_opacity`, `cset_output_opacity`). `fx_run` derives the effect's opacity handling from the answer.
- **`--host-curve-values PATH`.** Used only by `host_curve_values` of the two `BlurMoCurves` stages; see that section.

A parameter offered under two ids (for example `diffuse_amount` and `diffuse_amount_host` of `diffuse_autogen`) reaches the same program parameter through the two routes. Give only one of them: both together are exit 3 (`given both as --host-param and --set`, MEASURED for the `shift` pair of the `S_DropShadow` stage).

Parameters that a stage fixes (the mask inputs, for example) are not offered and cannot be changed; they are listed per stage in [Effects provider](stages.md#effects-provider).

## Shared parameters

### The opacity popup

`cset_opacity` (display name `Opacity`) is the host popup that tells the effect how to treat the input alpha. Its items are All_Opaque, Normal and As_Premult, so it takes `1`, `2` or `3` (INFERRED from the program layouts). On the host arrangement the stages run, only `2` (Normal) renders; `1` and `3` are refused by `fx_run` with exit 3 because the recorded host arrangement does not cover them (MEASURED on every stage that offers it). Leaving it unset is the same as `2` for a stage run (INFERRED). `time_warp_rgb_autogen` has `cset_input_opacity` (display name `Input Opacity`, same items and the same result: only `2` renders, MEASURED) and `cset_output_opacity` (display name `Output Opacity`, items All_Opaque and Copy_From_Input; `1` and `2` both render, MEASURED).

No recorded render sets an opacity popup, so its values are not in the verified lists.

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

## `BlurMoCurves`

Two stages: `transform_and_blur_autogen` (program `Transform_and_Blur_Autogen`) moves the image along a transform and blurs it over the shutter interval; `blur_motion_curves_no_transform_autogen` (program `Blur_Motion_Curves_No_Transform_Autogen`) applies only the motion blur and leaves the image in place (INFERRED). The transform itself (rotation, shift, distance and centre over time) comes from the host curve, not from a parameter.

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

## `S_DistortRGB`

One stage, `distort_rgb_autogen` (program `Distort_RGB_Autogen`). It takes the gradient of a blurred monochrome lens image (the optional `Lens` layer, else the input itself) and displaces the red, green and blue channels along it by different amounts (INFERRED).

- `amount`, scalar, `--set`. The overall strength of the displacement; a negative value pushes the other way (INFERRED). Verified at 9 values from `-0.9375` to `1.5625`.
- `amount_rel`, two lanes, `--set`. Scales the displacement per axis (INFERRED). Verified at 10 value pairs, each lane between `0.6875` and `1.4375`.
- `warp_red`, `warp_green`, `warp_blue`, scalars, `--set`. The displacement of each channel as a multiple of the common displacement (INFERRED). Verified at 9, 10 and 9 values between `0` and `0.9375`.
- `blur_lens`, scalar, `--host-param` (host units relative to the frame width). How much the lens image is blurred before its gradient is taken; it also scales the strength (INFERRED). Verified at 11 values from `0` to `68`.
- `rotate_warp_dir`, scalar angle in degrees, `--set`. Rotates the direction of the displacement (INFERRED). Verified at `-30` and `45`.
- `wrap`, two lanes (one popup index per axis, No, Tile, Reflect), `--set`. Edge handling of the displaced sampling (INFERRED). Verified at `0,0`, `0,2`, `1,1` and `2,1`.
- `filter`, checkbox, `--set`. Smooth (filtered) sampling of the displaced channels (INFERRED). Verified at `0`.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply distort_rgb_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param amount=1.5 --param blur_lens=24 --param warp_red=0.25 --param rotate_warp_dir=45 --in clip.raw --out out.raw
```

## `S_DropShadow`

One stage, `sapphire stage apply drop_shadow` (its program has the same name, capitalised). It takes a matte (the optional `Matte` layer, else the input's alpha), moves it, blurs it, colours it, and composites the input over the shadow and the optional `Background` layer (INFERRED). On an opaque input the shadow is hidden behind the input: the worked command below returns its input unchanged (MEASURED); the shadow shows where the input is transparent.

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
- `swing`, scalar angle in degrees, `--set`. Rotates the plane after the perspective (INFERRED). Verified at `-12`, `-0` and `40`.
- `roll`, scalar angle in degrees, `--set`. Rotates the image before the perspective (INFERRED). Verified at `-10`, `-0` and `30`.
- `wrap`, two lanes (one popup index per axis, No, Tile, Reflect), `--set`. Edge handling on the plane (INFERRED). Verified at `0,2`.
- `wrap_above_horizon`, checkbox, `--set`. Fills the area above the horizon instead of leaving it transparent (INFERRED). Verified at `1`.
- `filter`, checkbox, `--set`. Smooth (filtered) sampling (INFERRED). Verified at `0` and `1`.
- `cset_opacity`: see the opacity popup.

```sh
target/release/sapphire stage apply warp_perspective_autogen --width 64 --height 32 --depth 32 --frames 3 \
  --param latitude=60 --param swing=-12 --param roll=30 --param z_dist=2.5 --param wrap=0,2 --in clip.raw --out out.raw
```

## Keyframed values

Every stage but `trails_autogen` accepts a keyed value: `--param 'ID=V@F;V@F;...'`, with `F` an output frame number counted from 0 (fractions allowed) and `--fps` giving the time base. The syntax is the frames/2 convention's: a trailing `h` holds a key until the next one, a trailing `e` eases it, and `iS/I` and `oS/I` before the suffix are Bezier handles (`S` in units a second, `I` the influence in percent, 0.1 to 100). Before the first key the value is the first key's, after the last key the last one's.

- Scalars and angles interpolate: straight between plain keys, eased next to an `e` key, along the handle's curve next to a handle. A scalar that a handle overshoots is held inside its range.
- Two-lane vectors (`shift`, `blur_rel`, `rel_amount` and the like) move on the straight line between their keys.
- Integers, popups, checkboxes, colours, the opacity answers and `host_curve_values` hold their value until the next key and refuse handles.
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
- a host value off the 1/65536 grid (`HOST-VALUE-OFF-GRID`);
- the same program parameter given through both routes;
- a keyed value on `trails_autogen`, a malformed key or two keys on one tick (a keyed value without `--fps` is exit 2);
- a host curve sample that neither the given file nor the stage's identity curve can serve (`HOST-CURVE-SAMPLE-UNSERVED`).
