# Roadmap

The roadmap lists the twenty effects the rebuild covers. All twenty are done: each matches the original bit for bit on every compared case, apart from four counted comparisons of `S_Shake` and `S_DissolveShake` whose recorded renders were made before three faults were corrected, two in the native code and one in a value handed to the runner, and 34 of `S_GlowEdges` that were run with slider values passed as decimals, not as the 16.16 fixed-point values a host passes ([what that covers](effects.md#what-rebuilt-means)). The styles, parameters and parity of each done effect are in [effects](effects.md), and a new effect follows the conventions of [Adding an effect](adding-an-effect.md).

Effect names are the plugin's own name strings from the [name bank](glossary.md#name-bank). `S_Shake` and `S_BlurMoCurves` are host effects; the bank holds no name string for either, so they are written as the plugin's documentation names them.

## Effects

| Effect | Function | Status |
|---|---|---|
| `S_Blur` | Colour and mono blurs | done |
| `S_BlurMoCurves` | Motion blur along a per-frame motion curve supplied by the host | done |
| `S_BlurMotion` | Motion blur along a move between a start and an end rotation, depth distance and shift | done |
| `S_Diffuse` | Pixels displaced by a fixed pseudo random pattern | done |
| `S_DissolveShake` | Transition that mixes two clips by a progress value and offsets the mix per frame | done |
| `S_DistortRGB` | Displacement through a distortion field, with a separate strength per colour channel | done |
| `S_DropShadow` | A blurred, shifted shadow cast from the alpha | done |
| `S_Flicker` | Per-frame brightness factor from noise and sine terms of the frame time | done |
| `S_Glint` | Streaks along rows, columns and diagonals from the pixels above a threshold | done |
| `S_Glow` | Light spread around the pixels above a brightness threshold | done |
| `S_GlowEdges` | Light spread around the detected edges of the image | done |
| `S_HueSatBright` | Hue, saturation and brightness adjustments | done |
| `S_Retime` | Speed change of a clip, with frames interpolated along the optical flow | done |
| `S_Shake` | Per-frame offset, scale and rotation from noise and sine terms | done |
| `S_Sharpen` | Sharpening by detail bands or by a single width | done |
| `S_SwishPan` | Transition that moves one clip out of the frame and the other in, with motion blur along the move | done |
| `S_TimeWarpRGB` | Colour channels shifted in time | done |
| `S_Trails` | Blend of each frame with the previous output, which fades toward a colour | done |
| `S_WarpPerspective` | The image on a plane seen in perspective | done |
| `S_WarpTransform` | Scale, shift, rotation, shear and turn about the horizontal and vertical axes | done |

## Platform

The work that every effect shares, finished or in progress:

- **CPU speed.** The effect stages render in process and the kernels use all cores. `blur_motion_curves_no_transform_autogen` went from 52 s to 1.1 s per 1080p frame (MEASURED, [Speed](guide.md#speed)).
- **Keyframes.** Every effect stage but `trails_autogen` takes keyed values ([keyframes](parameters-guide.md#keyframed-values)). Values between keys are INFERRED.
- **Device path.** OpenCL kernels for every op of the executor's dispatch (the CPU-only list of `fx_exec` is empty), byte identical to the CPU path, MEASURED on PoCL; speed on a real GPU is not yet measured ([device](guide.md#rendering-on-a-device)).
- **Time and transitions.** The stages give each frame its time and the frame rate from `--fps`, and a transition stage takes a second clip and the length of the transition ([transition stages](cli.md#transition-stages)). `S_SwishPan` and `S_DissolveShake` are the transitions.
- **Opacity answers.** A stage can run each opacity answer with the host arrangement of that answer's own recorded renders ([effects provider](stages.md#effects-provider)); `S_Glow`, `S_SwishPan`, `S_Shake`, `S_DissolveShake`, `S_BlurMotion`, `S_WarpTransform` and `S_GlowEdges` render all three, and `S_Glint` renders the answers 2 and 3.
- **Determinism across processors.** The effect crates avoid the platform math library, and the golden hashes of `tools/fx_golden.py`, whose rows include infinities, both default NaN patterns and invalid operations, are checked in CI on Linux, Windows and macOS on ARM. For input without NaN, infinite and overflowing values included, and for input whose NaN words are all `0xffc00000`, every kernel gives the same output bytes on x86_64 and on 64-bit ARM, except `blur_warp`, the motion blur sampler of `S_BlurMoCurves`, `S_Shake`, `S_DissolveShake` and `S_BlurMotion`, whose NaN words can still differ for such input (MEASURED kernel by kernel under emulation and through every effect stage on constructed frames; INFERRED for other renders). Input that holds another NaN pattern, or two NaN values with different bits that meet in one operation, can still give NaN words whose sign or payload bits differ between processors, but no number differs (both MEASURED kernel by kernel); for two such NaN values the bits can also differ between compilers (INFERRED).
