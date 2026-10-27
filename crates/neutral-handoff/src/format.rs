//! [`Depth`] and [`FrameSpec`]: the raw frame format shared by ffmpeg, `vfx` and the stages.
//!
//! The contract is twixtor's raw frame format: whole frames back to back with no header or
//! padding, rows from the top, alpha straight.
//!
//! | Depth | ffmpeg `-pix_fmt` | Bytes a pixel | Layout |
//! |---|---|---|---|
//! | 8 | `rgba` | 4 | packed R, G, B, A, one byte each |
//! | 16 | `rgba64le` | 8 | packed R, G, B, A, unsigned 16-bit little endian |
//! | 32 | `gbrapf32le` | 16 | planar: the whole G plane, then B, then R, then A, 32-bit float little endian |

use neutral_ffmpeg::PixFmt;

/// Bits per channel of the raw frames between the tools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Depth {
    /// 8 bits, `rgba`.
    U8,
    /// 16 bits, `rgba64le`.
    U16,
    /// 32 bit float, `gbrapf32le`.
    F32,
}

impl Depth {
    /// All depths, ascending.
    pub const ALL: [Depth; 3] = [Depth::U8, Depth::U16, Depth::F32];

    /// From the `--depth` number.
    pub fn from_bits(bits: u32) -> Option<Depth> {
        match bits {
            8 => Some(Depth::U8),
            16 => Some(Depth::U16),
            32 => Some(Depth::F32),
            _ => None,
        }
    }

    /// The `--depth` number.
    pub fn bits(self) -> u32 {
        match self {
            Depth::U8 => 8,
            Depth::U16 => 16,
            Depth::F32 => 32,
        }
    }

    /// The ffmpeg pixel format name of the raw frames.
    pub fn pix_fmt_name(self) -> &'static str {
        match self {
            Depth::U8 => "rgba",
            Depth::U16 => "rgba64le",
            Depth::F32 => "gbrapf32le",
        }
    }

    /// The same format as a `neutral_ffmpeg` constant.
    pub fn pix_fmt(self) -> PixFmt {
        match self {
            Depth::U8 => PixFmt::RGBA,
            Depth::U16 => PixFmt::RGBA64LE,
            Depth::F32 => PixFmt::GBRAPF32LE,
        }
    }

    /// Bytes of one pixel.
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            Depth::U8 => 4,
            Depth::U16 => 8,
            Depth::F32 => 16,
        }
    }

    /// The pixel format asked of ffv1 for a lossless output: `bgra` at 8 bits, `rgba64le` at 16,
    /// none at 32 (ffv1 has no float format).
    pub fn lossless_pix_fmt(self) -> Option<&'static str> {
        match self {
            Depth::U8 => Some("bgra"),
            Depth::U16 => Some("rgba64le"),
            Depth::F32 => None,
        }
    }
}

/// The size and depth of the frames in one pipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameSpec {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Depth of the samples.
    pub depth: Depth,
}

impl FrameSpec {
    /// Bytes of one frame.
    pub fn frame_bytes(&self) -> u64 {
        self.width as u64 * self.height as u64 * self.depth.bytes_per_pixel() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_matches_the_contract() {
        let rows: Vec<(u32, &str, usize)> =
            Depth::ALL.iter().map(|d| (d.bits(), d.pix_fmt_name(), d.bytes_per_pixel())).collect();
        assert_eq!(rows, [(8, "rgba", 4), (16, "rgba64le", 8), (32, "gbrapf32le", 16)]);
    }

    #[test]
    fn pix_fmt_constants_agree_with_the_names() {
        for d in Depth::ALL {
            assert_eq!(d.pix_fmt().name(), d.pix_fmt_name());
            assert!(d.pix_fmt().is_supported());
            assert_eq!(d.pix_fmt().frame_size(160, 96).unwrap(), 160 * 96 * d.bytes_per_pixel());
        }
    }

    #[test]
    fn from_bits_round_trips() {
        for d in Depth::ALL {
            assert_eq!(Depth::from_bits(d.bits()), Some(d));
        }
        assert_eq!(Depth::from_bits(12), None);
        assert_eq!(Depth::from_bits(0), None);
    }

    #[test]
    fn frame_bytes_for_the_smoke_clip() {
        let spec = FrameSpec { width: 160, height: 96, depth: Depth::U16 };
        assert_eq!(spec.frame_bytes(), 122_880);
        let big = FrameSpec { width: 65535, height: 65535, depth: Depth::F32 };
        assert!(big.frame_bytes() > u32::MAX as u64);
    }

    #[test]
    fn lossless_formats() {
        assert_eq!(Depth::U8.lossless_pix_fmt(), Some("bgra"));
        assert_eq!(Depth::U16.lossless_pix_fmt(), Some("rgba64le"));
        assert_eq!(Depth::F32.lossless_pix_fmt(), None);
    }
}
