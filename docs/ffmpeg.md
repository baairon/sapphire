# ffmpeg

`ffmpeg` and `ffprobe` run as child processes through the `neutral-ffmpeg` crate ([modules](../crates/neutral-ffmpeg/README.md#modules)). The frame contract, stage chaining and exit codes are in [handoff](handoff.md#frame-contract).

## Process boundary

| Fact | Detail |
|---|---|
| Linking | none: raw frames move over the standard streams of the two programs, and nothing links against the ffmpeg libraries |
| Upstream | ffmpeg is used as installed and never modified |
| Version | 9.0.2 is the measured version; the `ffmpeg-filter` stage runs `ffmpeg -version` and reports another version as unverified on standard error |
| Commands of a run | `sapphire INPUT [EFFECT] ... --dry-run` prints every ffmpeg command as a `[cmd]` line and writes nothing |

## Finding ffmpeg

| Caller | Lookup, in order |
|---|---|
| `sapphire INPUT ...` (`crates/neutral-handoff/src/tools.rs`) and its tests | `--ffmpeg-path`, `MEDIA_FFMPEG_PATH`, `FFMPEG`, `PATH`, then on Windows `%LOCALAPPDATA%\Microsoft\WinGet\Links`, `%ProgramFiles%\ffmpeg\bin`, `%ProgramFiles(x86)%\ffmpeg\bin`; ffprobe the same with `--ffprobe-path`, `MEDIA_FFPROBE_PATH`, `FFPROBE` |
| `@ffmpeg-filter` in the pipeline | the pipeline's ffmpeg, passed on as `--ffmpeg-path` |
| `sapphire stage apply ffmpeg-filter` | `--ffmpeg-path`, `FFMPEG`, `PATH` |
| `neutral_ffmpeg::Binaries` (`crates/neutral-ffmpeg/src/locate.rs`) and its tests | the path set on `Binaries`, `FFMPEG` (`FFPROBE`), `PATH` |
| `tools/handoff.py` | `MEDIA_FFMPEG_PATH`, `FFMPEG`, `PATH`, then the Windows folders of the first row |
| `tools/conformance.py` | `MEDIA_FFMPEG_PATH`, `FFMPEG`, `PATH`; it passes the paths it found to the wrappers it runs |

| Rule | Detail |
|---|---|
| Set but unusable | a flag or variable that is set but names no usable program fails the lookup (`tool_missing`) instead of falling through, so a typo never runs a different ffmpeg |
| Bare name | a value without a folder is searched on `PATH`; a value with a folder is checked where it is |
| `PATH` search | the pipeline and `tools/handoff.py` search like Python's `shutil.which` (on Windows the current folder first, with the `PATHEXT` extensions); `Binaries` appends `.exe` on Windows |
| Nothing found | the pipeline stops with `tool_missing`, `this operation requires ffmpeg and ffprobe.` and one install command per platform |

## Raw frames

Whole frames back to back, no header, straight alpha, in the formats of ffmpeg's `rawvideo` demuxer and muxer (ffmpeg-formats.html, section "rawvideo"):

| Depth | `-pix_fmt` | Bytes per pixel | Layout |
|---|---|---|---|
| 8 | `rgba` | 4 | packed R, G, B, A |
| 16 | `rgba64le` | 8 | packed R, G, B, A, little endian |
| 32 | `gbrapf32le` | 16 | planar: G, B, R, then A |

`ffmpeg -pix_fmts` lists the pixel formats a build supports. The frame size limit and the partial last frame: [frame contract](handoff.md#frame-contract).

## Colour, rate and length

| Topic | Rule |
|---|---|
| Tagged source | decodes and encodes with the matrix of its tag (`bt470bg` as `bt601`, `bt2020nc` as `bt2020`), per `docs/data/contract_colour.tsv` |
| Untagged source, or another tag | `bt709` when max(width, height) >= 1280 or height >= 720, else `smpte170m`, on the display size after rotation; the rule of twixtor's `tools/apply.py` |
| HDR | a `smpte2084` or `arib-std-b67` transfer is refused |
| Plain `Decoder` | has no matrix option, so ffmpeg picks the matrix ("If not specified, the color space type depends on the pixel format", ffmpeg-filters.html, filter `scale`): BT.601 for untagged HD. `filter("[fps=R,]scale=in_color_matrix=M")` applies the rule |
| Probe gaps | `Probe` reads no colour space, rotation or attached picture, so the pipeline runs ffprobe itself for those |
| Frame rate | exact fractions (`30000/1001`, never `29.97`) |
| Truncated input | ffmpeg exits 0 on a truncated file (`docs/data/contract_exit.tsv`), so the pipeline counts frames and bytes and treats a short or partial stream as an error |

## Tests that need ffmpeg

The integration tests in `crates/neutral-ffmpeg/tests/` (`errors.rs`, `filter_live.rs`, `locate.rs`, `loopback.rs`, `parity.rs`, `pixfmt_live.rs`, `probe_live.rs`, `safety.rs`) and the pipeline tests in `crates/neutral-handoff/tests/` run the installed ffmpeg and ffprobe. Without them each test prints `SKIPPED <test>: <reason>` and returns instead of failing ([neutral-ffmpeg tests](../crates/neutral-ffmpeg/README.md#tests)).
