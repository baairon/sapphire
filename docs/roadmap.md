# Roadmap

The goal is 20 effects in native Rust: the ten that run today and the ten below. The other effect names of the plugin are not planned. The status of each finished effect is in [effects](effects.md).

## Platform

The work that every effect shares, finished or in progress:

- **CPU speed.** The effect stages render in process and the kernels use all cores. `blur_motion_curves_no_transform_autogen` went from 52 s to 1.1 s per 1080p frame (MEASURED, [Speed](guide.md#speed)).
- **Keyframes.** Every effect stage but `trails_autogen` takes keyed values ([keyframes](parameters-guide.md#keyframed-values)). Values between keys are INFERRED.
- **Device path.** OpenCL kernels for the heavy operations, byte identical to the CPU path, MEASURED on PoCL; speed on a real GPU is not yet measured ([device](guide.md#rendering-on-a-device)).
- **Determinism across processors.** The golden hashes of `tools/fx_golden.py` are checked in CI on Linux, macOS and Windows, so the output bytes are held equal across processors; the effect crates avoid the platform math library for the same reason.

## Done

Each of these matches the original bit for bit on every compared case ([what that covers](effects.md#what-rebuilt-means)). Per-effect status is in the table of [effects](effects.md).

- `S_Blur`
- `BlurMoCurves`
- `S_Diffuse`
- `S_DistortRGB`
- `S_DropShadow`
- `S_HueSatBright`
- `S_Sharpen`
- `S_TimeWarpRGB`
- `S_Trails`
- `S_WarpPerspective`

## Next ten

The ten below are Tier 1 and in progress. The order follows how much code already ported each effect can reuse, cheapest first, with the two heaviest last. The choice of effects is by how often edits use them (INFERRED: no public usage statistics exist), taking one or two per kind of look: glow and light, motion, beat, colour split, glitch and transition.

| Order | Effect | Use |
|---|---|---|
| 1 | `S_BlurDirectional` | Blur along a chosen direction, for speed and whip looks. |
| 2 | `S_WarpChroma` | Warp that splits colour channels, for colour-split transitions. |
| 3 | `S_Flicker` | Brightness flicker, for beat and strobe looks. |
| 4 | `S_Glint` | Sparkle highlights on bright points. |
| 5 | `S_SwishPan` | Fast swish between shots, for pan transitions. |
| 6 | `Shake` | Camera shake in three styles (Normal, Twitchy, Jumpy), for impacts and beats. |
| 7 | `S_Glow` | Soft glow around bright areas. |
| 8 | `S_Rays` | Light rays streaming from bright areas. |
| 9 | `S_DigitalDamage` | Digital glitch and corruption. |
| 10 | `S_LensFlare` | Lens flare, the plugin's best-known light effect. |

`Shake`, like `BlurMoCurves`, is a host effect of the plugin whose name string is not in the [name bank](glossary.md#name-bank), so both are written here without the bank's prefix.
