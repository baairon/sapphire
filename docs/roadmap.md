# Roadmap

The planned effects are ranked by how useful they are in YouTube edits: montages, highlights, music videos and gaming videos. The ranking is a practical judgment of each effect's use, not a measured statistic (INFERRED: no public usage figures exist). Tier 1 is built first, then each tier in rank order. The status of each finished effect is in [effects](effects.md).

Effect names are the plugin's own name strings from the [name bank](glossary.md#name-bank). `Shake` and `BlurMoCurves` are host effects whose names are not in the bank, so they are written without the bank's prefix.

## Platform

The work that every effect shares, finished or in progress:

- **CPU speed.** The effect stages render in process and the kernels use all cores. `blur_motion_curves_no_transform_autogen` went from 52 s to 1.1 s per 1080p frame (MEASURED, [Speed](guide.md#speed)).
- **Keyframes.** Every effect stage but `trails_autogen` takes keyed values ([keyframes](parameters-guide.md#keyframed-values)). Values between keys are INFERRED.
- **Device path.** OpenCL kernels for the heavy operations, byte identical to the CPU path, MEASURED on PoCL; speed on a real GPU is not yet measured ([device](guide.md#rendering-on-a-device)).
- **Time and transitions.** The stages give each frame its time and the frame rate from `--fps`, and a transition stage takes a second clip and the length of the transition ([transition stages](cli.md#transition-stages)). `S_SwishPan` and `S_DissolveShake` are the transitions.
- **Opacity answers.** A stage can run each opacity answer with the host arrangement of that answer's own recorded renders ([effects provider](stages.md#effects-provider)); `S_Glow`, `S_SwishPan`, `Shake` and `S_DissolveShake` render all three.
- **Determinism across processors.** The effect crates avoid the platform math library, and the golden hashes of `tools/fx_golden.py`, whose rows include infinities, both default NaN patterns and invalid operations, are checked in CI on Linux, Windows and macOS on ARM. For input without NaN, infinite and overflowing values included, and for input whose NaN words are all `0xffc00000`, every kernel gives the same output bytes on x86_64 and on 64-bit ARM, except `blur_warp`, the motion blur sampler of `BlurMoCurves`, `Shake` and `S_DissolveShake`, whose NaN words can still differ for such input (MEASURED kernel by kernel under emulation and through every effect stage on constructed frames; INFERRED for other renders). Input that holds another NaN pattern, or two NaN values with different bits that meet in one operation, can still give NaN words whose sign or payload bits differ between processors, but no number differs (both MEASURED kernel by kernel); for two such NaN values the bits can also differ between compilers (INFERRED).

## Done

Each of these matches the original bit for bit on every compared case, apart from four counted comparisons of `Shake` and `S_DissolveShake` whose recorded renders were made before three faults were corrected, two in the native code and one in a value handed to the runner ([what that covers](effects.md#what-rebuilt-means)). Per-effect status is in the table of [effects](effects.md).

- `S_Blur`
- `BlurMoCurves`
- `S_Diffuse`
- `S_DissolveShake`
- `S_DistortRGB`
- `S_DropShadow`
- `S_Flicker`
- `S_Glow`
- `S_HueSatBright`
- `Shake`
- `S_Sharpen`
- `S_SwishPan`
- `S_TimeWarpRGB`
- `S_Trails`
- `S_WarpPerspective`

## Tier 1: essential

Ranks 1 to 8. The effects most edits reach for first. All eight are done.

| Rank | Effect | Use | Status |
|---|---|---|---|
| 1 | `Shake` | Impact shakes, hits and dramatic moments | done |
| 2 | `S_Glow` | Bright highlights and a cinematic glow | done |
| 3 | `BlurMoCurves` | Smooth motion blur for fast, speed-ramped edits | done |
| 4 | `S_DistortRGB` | RGB splits and chromatic distortion | done |
| 5 | `S_Flicker` | Flashes and intensity in time with a beat | done |
| 6 | `S_Sharpen` | Crisp, detailed footage | done |
| 7 | `S_DissolveShake` | Shaky transitions between clips | done |
| 8 | `S_SwishPan` | Fast directional transitions | done |

## Tier 2: very useful

Ranks 9 to 16. Polish for montages and cinematic edits.

| Rank | Effect | Use |
|---|---|---|
| 9 | `S_BlurMotion` | Motion blur on fast movement |
| 10 | `S_WarpTransform` | Zooms, rotations and dynamic framing |
| 11 | `S_GlowEdges` | Glowing outlines and edge highlights |
| 12 | `S_Retime` | Slow motion and speed changes |
| 13 | `S_DigitalDamage` | Glitches and digital distortion |
| 14 | `S_Glint` | Sparkles on bright objects |

Ranks 15 and 16 are push-in and pull-out transitions (Hyper Push and Hyper Pull). Neither is in the name bank or the host records of the plugin version this project rebuilds (MEASURED), so neither has code to rebuild.

## Tier 3: situational

Ranks 17 to 24. Atmosphere and scene transitions for slower, story-driven edits.

| Rank | Effect | Use |
|---|---|---|
| 17 | `S_DissolveGlow` | Glowing scene transitions |
| 18 | `S_FilmDamage` | Dark, vintage film looks |
| 19 | `S_TVDamage` | Distorted screen effects |
| 20 | `S_Rays` | Dramatic light beams |
| 21 | `S_EdgeRays` | Rays extending from object edges |
| 22 | `S_Vignette` | Darkened corners for focus |
| 23 | `S_LightLeak` | Warm flashes and atmosphere |
| 24 | `S_LensFlare` | Bright cinematic lighting |

## Tier 4: optional

Ranks 25 to 30. Creative looks that fewer edits need.

| Rank | Effect | Use |
|---|---|---|
| 27 | `S_WarpVortex` | Spinning vortex distortions |
| 28 | `S_JpegDamage` | Compression artifacts and glitch looks |
| 29 | `S_PixelSort` | Stylized pixel distortion |
| 30 | `S_KaleidoRadial` | Kaleidoscope-style transitions |

Ranks 25 and 26 are a prism lens distortion (Prism Lens) and an extreme glow (Ultra Glow). Neither is in the name bank or the host records of the plugin version this project rebuilds (MEASURED), so neither has code to rebuild.

## Common combinations

- **Fast impact edits:** `Shake`, `S_Glow`, `BlurMoCurves`, `S_DistortRGB` and `S_Flicker`, for hard hits with glowing highlights, colour separation and smooth movement.
- **Cinematic and story edits:** `S_Glow`, `S_Vignette`, `S_Rays`, `S_FilmDamage` and `BlurMoCurves`.

Speed ramps, punch zooms, impact frames, titles and camera moves are usually made with the editor's own tools rather than with an effect.
