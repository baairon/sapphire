# Roadmap

The roadmap lists the twenty effects the rebuild covers. Nineteen are done: each matches the original bit for bit on every compared case, apart from four counted comparisons of `Shake` and `S_DissolveShake` whose recorded renders were made before three faults were corrected, two in the native code and one in a value handed to the runner, and 34 of `S_GlowEdges` that were run with slider values passed as decimals, not as the 16.16 fixed-point values a host passes ([what that covers](effects.md#what-rebuilt-means)). `S_Retime` is not yet done. The styles, parameters and parity of each done effect are in [effects](effects.md), and a new effect follows the conventions of [Adding an effect](adding-an-effect.md).

Effect names are the plugin's own name strings from the [name bank](glossary.md#name-bank). `Shake` and `BlurMoCurves` are host effects whose names are not in the bank, so they are written without the bank's prefix.

## Effects

| Effect | Use | Status |
|---|---|---|
| `S_Blur` | Colour and mono blurs | done |
| `BlurMoCurves` | Smooth motion blur for fast, speed-ramped edits | done |
| `S_BlurMotion` | Motion blur on fast movement | done |
| `S_Diffuse` | Scattered pixels and grain | done |
| `S_DissolveShake` | Shaky transitions between clips | done |
| `S_DistortRGB` | RGB splits and chromatic distortion | done |
| `S_DropShadow` | A blurred, shifted shadow cast from the alpha | done |
| `S_Flicker` | Flashes and intensity in time with a beat | done |
| `S_Glint` | Sparkles on bright objects | done |
| `S_Glow` | Bright highlights and a cinematic glow | done |
| `S_GlowEdges` | Glowing outlines and edge highlights | done |
| `S_HueSatBright` | Hue, saturation and brightness adjustments | done |
| `Shake` | Impact shakes, hits and dramatic moments | done |
| `S_Sharpen` | Crisp, detailed footage | done |
| `S_SwishPan` | Fast directional transitions | done |
| `S_TimeWarpRGB` | Colour channels shifted in time | done |
| `S_Trails` | Fading trails behind moving content | done |
| `S_WarpPerspective` | The image on a plane seen in perspective | done |
| `S_WarpTransform` | Zooms, rotations and dynamic framing | done |
| `S_Retime` | Slow motion and speed changes | |

## Common combinations

- **Fast impact edits:** `Shake`, `S_Glow`, `BlurMoCurves`, `S_DistortRGB` and `S_Flicker`, for hard hits with glowing highlights, colour separation and smooth movement.
- **Cinematic and story edits:** `S_Glow`, `S_GlowEdges`, `S_WarpTransform` and `BlurMoCurves`, for soft highlights, glowing outlines, slow zooms and smooth movement.

Speed ramps, punch zooms, impact frames, titles and camera moves are usually made with the editor's own tools rather than with an effect.

## Platform

The work that every effect shares, finished or in progress:

- **CPU speed.** The effect stages render in process and the kernels use all cores. `blur_motion_curves_no_transform_autogen` went from 52 s to 1.1 s per 1080p frame (MEASURED, [Speed](guide.md#speed)).
- **Keyframes.** Every effect stage but `trails_autogen` takes keyed values ([keyframes](parameters-guide.md#keyframed-values)). Values between keys are INFERRED.
- **Device path.** OpenCL kernels for the heavy operations, byte identical to the CPU path, MEASURED on PoCL; speed on a real GPU is not yet measured ([device](guide.md#rendering-on-a-device)).
- **Time and transitions.** The stages give each frame its time and the frame rate from `--fps`, and a transition stage takes a second clip and the length of the transition ([transition stages](cli.md#transition-stages)). `S_SwishPan` and `S_DissolveShake` are the transitions.
- **Opacity answers.** A stage can run each opacity answer with the host arrangement of that answer's own recorded renders ([effects provider](stages.md#effects-provider)); `S_Glow`, `S_SwishPan`, `Shake`, `S_DissolveShake`, `S_BlurMotion`, `S_WarpTransform` and `S_GlowEdges` render all three, and `S_Glint` renders the answers 2 and 3.
- **Determinism across processors.** The effect crates avoid the platform math library, and the golden hashes of `tools/fx_golden.py`, whose rows include infinities, both default NaN patterns and invalid operations, are checked in CI on Linux, Windows and macOS on ARM. For input without NaN, infinite and overflowing values included, and for input whose NaN words are all `0xffc00000`, every kernel gives the same output bytes on x86_64 and on 64-bit ARM, except `blur_warp`, the motion blur sampler of `BlurMoCurves`, `Shake`, `S_DissolveShake` and `S_BlurMotion`, whose NaN words can still differ for such input (MEASURED kernel by kernel under emulation and through every effect stage on constructed frames; INFERRED for other renders). Input that holds another NaN pattern, or two NaN values with different bits that meet in one operation, can still give NaN words whose sign or payload bits differ between processors, but no number differs (both MEASURED kernel by kernel); for two such NaN values the bits can also differ between compilers (INFERRED).
