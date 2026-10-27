# neutral-ffmpeg

Runs `ffmpeg` and `ffprobe` as child processes and moves `rawvideo` frames over their standard streams; a failure carries ffmpeg's own error text. Nothing links against libav, and any installed ffmpeg build works. How the rest of the repository uses it: [ffmpeg](../../docs/ffmpeg.md).

## Modules

| Module (`src/`) | Type | Role |
|---|---|---|
| `decoder.rs` | `Decoder` | ffmpeg decodes a file; the crate reads raw frames from its standard output |
| `encoder.rs` | `Encoder` | the crate writes raw frames to ffmpeg's standard input; ffmpeg encodes and muxes |
| `filter.rs` | `Filter` | raw frames through a `-vf` filter graph; a thread always drains the output, so a graph that holds frames back cannot deadlock the pipes (the base of the `ffmpeg-filter` stage of `sapphire`) |
| `probe.rs`, `json.rs` | `Probe`, `probe_file` | ffprobe's JSON output, per stream `codec_type`, `codec_name`, `width`, `height`, `pix_fmt`, `r_frame_rate`, `avg_frame_rate`, `duration`, `nb_frames`, `sample_rate`, `channels`, and the container's `format_name` and `duration`; `json.rs` is the strict parser it reads with |
| `pixfmt.rs`, `pixfmt_table.rs` | `PixFmt` | every ffmpeg pixel format with the exact frame geometry `ffmpeg -f rawvideo` writes: planes back to back, rows tightly packed, palette at the end ([Pixel format table](#pixel-format-table)) |
| `framebuf.rs` | `FrameBuf` | one tightly packed frame, with helpers for padded strides |
| `sizeprobe.rs` | | the frame size an ffmpeg command will produce, read from a one frame dry run with the `showinfo` filter appended |
| `locate.rs` | `Binaries` | the path set on `Binaries`, then `FFMPEG` (`FFPROBE`), then `PATH` with `.exe` appended on Windows; a set but unusable path is `Error::ToolNotFound`, listing every place tried ([finding ffmpeg](../../docs/ffmpeg.md#finding-ffmpeg)) |
| `process.rs`, `exec.rs` | | child processes with standard error drained on a thread, killed when dropped; short jobs under a hard deadline that ends in `Error::Timeout` |
| `error.rs` | `Error` | the error type of every module |
| `lib.rs` | | the crate root and its re-exports |

## Pixel format table

`src/pixfmt_table.rs` is generated from `ffmpeg -pix_fmts` and `ffprobe -show_pixel_formats` of ffmpeg 9.0.2 (`GENERATED_FROM`), with layout facts measured on the same binary. `tests/pixfmt_live.rs` compares it with the installed ffmpeg and ffprobe, so a build whose format list differs fails there.

## Tests

`cargo test --release --offline -p neutral-ffmpeg` from the repository root; <!--f:tests_scan_neutral_ffmpeg-->130<!--/f--> `#[test]` lines by text scan ([crates](../../docs/generated/crates.md)).

| File | Checks |
|---|---|
| unit tests in `src/` | parsing, frame geometry, buffers, process plumbing |
| `tests/errors.rs` | what the caller sees when ffmpeg or the input is wrong |
| `tests/filter_live.rs` | filter graphs: identity, pixel format, size and frame count changes, no deadlock |
| `tests/locate.rs` | the resolution order of ffmpeg and ffprobe, on real files |
| `tests/loopback.rs` | `Decoder` and `Encoder`: lavfi frames encoded losslessly, decoded again and compared byte for byte |
| `tests/parity.rs` | the `Decoder`'s frames against the same `ffmpeg ... -f rawvideo -` command run directly, for every format the table can emit |
| `tests/pixfmt_live.rs` | the pixel format table against the installed ffmpeg and ffprobe |
| `tests/probe_live.rs` | the ffprobe reader on files generated from lavfi sources |
| `tests/safety.rs` | runaway jobs: finite dry run inputs, hard deadlines with kill, an error when the size cannot be measured |

Every live test skips without ffmpeg and ffprobe ([tests that need ffmpeg](../../docs/ffmpeg.md#tests-that-need-ffmpeg)). With them present, one line remains: `SKIPPED generated_file_is_what_the_script_produces_now: generator or probe data not found`.

## Limits

| Limit | Effect |
|---|---|
| `Probe` reads no colour space, rotation or attached picture | the `sapphire` pipeline runs ffprobe itself for those |
| ffmpeg exits 0 on a truncated input file (`docs/data/contract_exit.tsv`) | `Decoder` returns the whole frames it got without an error; callers count frames and bytes ([known gaps](../../docs/handoff.md#known-gaps)) |
| Pixel format table | the generator is not in the repository, so the table cannot be regenerated here |
