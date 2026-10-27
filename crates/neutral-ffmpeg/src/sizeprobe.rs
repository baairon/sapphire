//! Learning the size of the frames an ffmpeg command is going to produce.
//!
//! The decoder and the filter slice a byte stream into frames, so they must
//! know the frame size before the first byte arrives. When the caller does not
//! say, a dry run of the same graph is made for one frame with the `showinfo`
//! filter appended, and the `s:WxH` field of its log line is read.

use std::ffi::OsString;
use std::path::Path;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::exec::run_capture;
use crate::process::os;

/// Find `s:WxH` in a showinfo log line.
pub(crate) fn parse_showinfo_size(stderr: &str) -> Option<(u32, u32)> {
    for line in stderr.lines() {
        if !line.contains("showinfo") {
            continue;
        }
        let mut rest = line;
        while let Some(pos) = rest.find(" s:") {
            let after = &rest[pos + 3..];
            let token: String = after.chars().take_while(|c| c.is_ascii_digit() || *c == 'x').collect();
            if let Some((w, h)) = token.split_once('x') {
                if let (Ok(w), Ok(h)) = (w.parse::<u32>(), h.parse::<u32>()) {
                    if w > 0 && h > 0 {
                        return Some((w, h));
                    }
                }
            }
            rest = after;
        }
    }
    None
}

/// Hard deadline for one size measurement. The inputs of the dry run are finite,
/// so a healthy run ends in well under a second; anything that runs this long is
/// stuck (a graph that never emits a frame) and is killed.
pub(crate) const DETECT_DEADLINE: Duration = Duration::from_secs(60);

/// Run `ffmpeg -hide_banner -loglevel info -nostdin <args> -frames:v 1 -f null -`
/// and return the size showinfo reports. `args` must contain a FINITE input (the
/// callers bound it with `d=` or an input `-t`) and a video filter chain that ends
/// in `showinfo`. The run is killed after [`DETECT_DEADLINE`].
pub(crate) fn detect_size(ffmpeg: &Path, args: &[OsString], context: &str) -> Result<(u32, u32)> {
    detect_size_within(ffmpeg, args, context, DETECT_DEADLINE)
}

pub(crate) fn detect_size_within(
    ffmpeg: &Path,
    args: &[OsString],
    context: &str,
    deadline: Duration,
) -> Result<(u32, u32)> {
    let mut full: Vec<OsString> = ["-hide_banner", "-loglevel", "info", "-nostdin"].iter().map(os).collect();
    full.extend(args.iter().cloned());
    full.extend(["-frames:v", "1", "-f", "null", "-"].iter().map(os));
    let out = run_capture(ffmpeg, &full, deadline, "ffmpeg", &format!("measuring the frame size for {}", context))?;
    let stderr = out.stderr_text();
    if !out.status.success() {
        return Err(Error::Ffmpeg {
            tool: "ffmpeg",
            context: format!("measure the frame size for {}", context),
            code: out.status.code(),
            stderr,
        });
    }
    parse_showinfo_size(&stderr).ok_or_else(|| Error::SizeUnknown { context: context.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_size_field() {
        let line = "[Parsed_showinfo_1 @ 000001e2c4b6ba40] n:   0 pts:      0 pts_time:0       duration:      1 \
                    duration_time:0.04    fmt:yuv420p sar:1/1 s:320x240 i:P iskey:1 type:I checksum:5A1F7B2C \
                    plane_checksum:[1 2 3] mean:[10 20 30] stdev:[1.0 2.0 3.0]";
        assert_eq!(parse_showinfo_size(line), Some((320, 240)));
    }

    #[test]
    fn ignores_other_lines() {
        let log = "Input #0, lavfi, from 'color=c=black:s=64x48':\n  Stream #0:0: Video: wrapped_avframe, 64x48 s:1x1\n";
        assert_eq!(parse_showinfo_size(log), None);
        assert_eq!(parse_showinfo_size(""), None);
        assert_eq!(parse_showinfo_size("showinfo s:0x5"), None);
        assert_eq!(parse_showinfo_size("showinfo s:abc"), None);
    }
}
