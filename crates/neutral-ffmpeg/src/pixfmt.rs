//! Pixel format table and frame geometry.
//!
//! The table in `pixfmt_table.rs` is generated from the live output of
//! `ffmpeg -hide_banner -pix_fmts` and `ffprobe -show_pixel_formats`, plus
//! layout facts measured on the same binary (see `scripts/gen_pixfmt.py`).
//! Everything here computes the layout that `ffmpeg -f rawvideo` writes:
//! planes back to back, every row tightly packed (alignment 1), the palette of
//! paletted formats at the end.

use std::fmt;

use crate::error::{Error, Result};
use crate::pixfmt_table::{ALIASES, BY_NAME, FORMATS};

/// ffmpeg can convert from this format (`I` in `-pix_fmts`).
pub const F_INPUT: u16 = 1 << 0;
/// ffmpeg can convert to this format (`O` in `-pix_fmts`).
pub const F_OUTPUT: u16 = 1 << 1;
/// Hardware surface format (`H`).
pub const F_HWACCEL: u16 = 1 << 2;
/// Paletted format (`P`).
pub const F_PALETTED: u16 = 1 << 3;
/// Bitstream format, pixels packed below one byte (`B`).
pub const F_BITSTREAM: u16 = 1 << 4;
/// Samples are stored big endian (ffprobe `big_endian`).
pub const F_BIG_ENDIAN: u16 = 1 << 5;
/// More than one plane (ffprobe `planar`).
pub const F_PLANAR: u16 = 1 << 6;
/// RGB family (ffprobe `rgb`).
pub const F_RGB: u16 = 1 << 7;
/// Has an alpha component (ffprobe `alpha`).
pub const F_ALPHA: u16 = 1 << 8;

/// Byte order of multi-byte samples.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Endian {
    /// Samples are single bytes (or the format has no byte order).
    None,
    /// Least significant byte first.
    Little,
    /// Most significant byte first.
    Big,
}

/// How one plane is laid out, as a function of the frame width and height.
///
/// A plane is a grid of pixel groups. `step` is the number of bytes in one
/// group (bits for bitstream formats). A group covers `1 << shift_w` pixels
/// horizontally, and the plane has `ceil(height / (1 << shift_h))` rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneSpec {
    /// Bytes per pixel group (bits per group for bitstream formats).
    pub step: u8,
    /// log2 of the number of pixels one group spans horizontally.
    pub shift_w: u8,
    /// log2 of the vertical subsampling of this plane.
    pub shift_h: u8,
}

/// Const constructor used by the generated table.
pub const fn plane(step: u8, shift_w: u8, shift_h: u8) -> PlaneSpec {
    PlaneSpec { step, shift_w, shift_h }
}

/// Whether ffmpeg emits the format as rawvideo, measured on the generating binary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Emitted and measured at every probe size.
    Yes,
    /// Not emittable; the string says why.
    No(&'static str),
}

/// One row of the format table.
#[derive(Clone, Copy, Debug)]
pub struct FormatInfo {
    /// ffmpeg name.
    pub name: &'static str,
    /// Component count printed by `-pix_fmts`.
    pub nb_components: u8,
    /// Bits per pixel printed by `-pix_fmts` (significant bits, not memory layout).
    pub bits_per_pixel: u16,
    /// Per component bit depth, zero padded.
    pub bit_depths: [u8; 4],
    /// `F_*` bit set.
    pub flags: u16,
    /// log2 of horizontal chroma subsampling, from the layout fit.
    pub chroma_shift_w: u8,
    /// log2 of vertical chroma subsampling, from the layout fit.
    pub chroma_shift_h: u8,
    /// Size in bytes of the word whose byte order matters (1 when it does not).
    pub sample_bytes: u8,
    /// Byte order of that word.
    pub endian: Endian,
    /// Bytes of palette that rawvideo appends after the pixels (1024 or 0).
    pub palette_bytes: u16,
    /// Number of pixel planes (the palette is not counted).
    pub nb_planes: u8,
    /// Layout of each pixel plane; entries past `nb_planes` are zero.
    pub planes: [PlaneSpec; 4],
    /// Rawvideo support.
    pub support: Support,
}

impl FormatInfo {
    /// True when the format can be emitted and sized.
    pub fn is_supported(&self) -> bool {
        matches!(self.support, Support::Yes)
    }

    /// Why the format is unsupported.
    pub fn unsupported_reason(&self) -> Option<&'static str> {
        match self.support {
            Support::Yes => None,
            Support::No(reason) => Some(reason),
        }
    }

    /// True when `flag` (one of the `F_*` constants) is set.
    pub fn has(&self, flag: u16) -> bool {
        self.flags & flag != 0
    }

    /// The pixel planes that are described for this format.
    pub fn plane_specs(&self) -> &[PlaneSpec] {
        &self.planes[..self.nb_planes as usize]
    }
}

/// Handle to one row of the format table. Cheap to copy and compare.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PixFmt(pub(crate) u16);

impl fmt::Debug for PixFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PixFmt({})", self.name())
    }
}

impl fmt::Display for PixFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Whether a plane holds pixels or the palette of a paletted format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneKind {
    /// Pixel samples.
    Samples,
    /// 256 four byte palette entries, placed right after the pixels.
    Palette,
}

/// Position and shape of one plane inside a tightly packed frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneLayout {
    /// Pixels or palette.
    pub kind: PlaneKind,
    /// Byte offset of the first row inside the frame.
    pub offset: usize,
    /// Bytes per row (ffmpeg `av_image_get_linesize`, alignment 1).
    pub stride: usize,
    /// Number of rows.
    pub rows: usize,
    /// Bytes in the plane (`stride * rows`; the whole palette for `Palette`).
    pub size: usize,
    /// Pixel groups per row.
    pub cols: usize,
    /// Pixels covered by one group (`1 << shift_w`).
    pub group_width: u32,
    /// Bytes per group, or bits per group when `bit_packed` is set.
    pub group_bytes: usize,
    /// log2 of the horizontal subsampling of the plane.
    pub shift_w: u8,
    /// log2 of the vertical subsampling of the plane.
    pub shift_h: u8,
    /// Groups are counted in bits and rows are padded to whole bytes.
    pub bit_packed: bool,
}

const NO_PLANE: PlaneLayout = PlaneLayout {
    kind: PlaneKind::Samples,
    offset: 0,
    stride: 0,
    rows: 0,
    size: 0,
    cols: 0,
    group_width: 1,
    group_bytes: 0,
    shift_w: 0,
    shift_h: 0,
    bit_packed: false,
};

/// Where every plane of a frame of one format and size lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameLayout {
    fmt: PixFmt,
    width: u32,
    height: u32,
    planes: [PlaneLayout; 5],
    count: usize,
    size: usize,
}

impl FrameLayout {
    /// The format this layout belongs to.
    pub fn fmt(&self) -> PixFmt {
        self.fmt
    }

    /// Frame width in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Frame height in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// All planes, pixel planes first, the palette (if any) last.
    pub fn planes(&self) -> &[PlaneLayout] {
        &self.planes[..self.count]
    }

    /// Number of pixel planes (without the palette).
    pub fn pixel_planes(&self) -> usize {
        self.planes().iter().filter(|p| p.kind == PlaneKind::Samples).count()
    }

    /// Total bytes of a tightly packed frame, equal to the rawvideo frame length.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Bytes of the same frame when every row is padded to a multiple of `align`
    /// (ffmpeg `av_image_get_buffer_size(.., align)`).
    pub fn aligned_size(&self, align: usize) -> usize {
        self.planes()
            .iter()
            .map(|p| match p.kind {
                PlaneKind::Samples => align_up(p.stride, align) * p.rows,
                PlaneKind::Palette => p.size,
            })
            .sum()
    }
}

/// Round `value` up to a multiple of `align` (`align` of 0 or 1 leaves it alone).
pub fn align_up(value: usize, align: usize) -> usize {
    if align <= 1 {
        value
    } else {
        value.div_ceil(align) * align
    }
}

fn ceil_shift(n: u64, shift: u8) -> u64 {
    (n + (1u64 << shift) - 1) >> shift
}

/// Mirror of ffmpeg's `av_image_check_size` plus the `int` limit of
/// `av_image_get_buffer_size`.
fn check_dims(width: u32, height: u32) -> Result<()> {
    if width == 0 || height == 0 {
        return Err(Error::InvalidSize { width, height, reason: "width and height must be at least 1" });
    }
    let area = (width as u64 + 128) * (height as u64 + 128);
    if area >= (i32::MAX as u64) / 8 {
        return Err(Error::InvalidSize { width, height, reason: "frame area is larger than ffmpeg accepts" });
    }
    Ok(())
}

impl PixFmt {
    /// Look a format up by its ffmpeg name. Canonical names and the
    /// little-endian spellings ffmpeg also accepts (`rgb32`, `gray16`,
    /// `yuv420p10`, ...) are recognised. Matching is case sensitive, like ffmpeg.
    pub fn from_name(name: &str) -> Option<PixFmt> {
        if let Ok(i) = BY_NAME.binary_search_by(|(n, _)| (*n).cmp(name)) {
            return Some(PixFmt(BY_NAME[i].1));
        }
        if let Ok(i) = ALIASES.binary_search_by(|(n, _)| (*n).cmp(name)) {
            return Some(PixFmt(ALIASES[i].1));
        }
        None
    }

    /// Like [`PixFmt::from_name`] but returns an error naming the bad input.
    pub fn parse(name: &str) -> Result<PixFmt> {
        PixFmt::from_name(name).ok_or_else(|| Error::UnknownPixFmt(name.to_string()))
    }

    /// Every format in the table, in the order of `ffmpeg -pix_fmts`.
    pub fn all() -> impl Iterator<Item = PixFmt> + Clone {
        (0..FORMATS.len() as u16).map(PixFmt)
    }

    /// Number of formats in the table.
    pub fn count() -> usize {
        FORMATS.len()
    }

    /// Canonical ffmpeg name.
    pub fn name(self) -> &'static str {
        self.info().name
    }

    /// Table row.
    pub fn info(self) -> &'static FormatInfo {
        &FORMATS[self.0 as usize]
    }

    /// True when ffmpeg emits this format as rawvideo (the table is measured).
    pub fn is_supported(self) -> bool {
        self.info().is_supported()
    }

    /// Why the format is unsupported, if it is.
    pub fn unsupported_reason(self) -> Option<&'static str> {
        self.info().unsupported_reason()
    }

    /// True when the format has more than one plane.
    pub fn is_planar(self) -> bool {
        self.info().has(F_PLANAR)
    }

    /// True when the format has an alpha component.
    pub fn has_alpha(self) -> bool {
        self.info().has(F_ALPHA)
    }

    /// True for the RGB family.
    pub fn is_rgb(self) -> bool {
        self.info().has(F_RGB)
    }

    /// True for hardware surface formats.
    pub fn is_hwaccel(self) -> bool {
        self.info().has(F_HWACCEL)
    }

    /// True when the format is paletted.
    pub fn is_paletted(self) -> bool {
        self.info().has(F_PALETTED)
    }

    /// True for sub-byte (bitstream) formats.
    pub fn is_bitstream(self) -> bool {
        self.info().has(F_BITSTREAM)
    }

    /// Byte order of multi-byte samples (`Endian::None` for byte samples).
    pub fn endian(self) -> Endian {
        self.info().endian
    }

    /// Size of the word whose byte order matters: 1, 2 or 4 (0 if unsupported).
    pub fn sample_bytes(self) -> u8 {
        self.info().sample_bytes
    }

    /// `(log2 horizontal, log2 vertical)` chroma subsampling.
    pub fn chroma_shift(self) -> (u8, u8) {
        (self.info().chroma_shift_w, self.info().chroma_shift_h)
    }

    /// Number of pixel planes (0 for unsupported formats).
    pub fn nb_planes(self) -> usize {
        self.info().nb_planes as usize
    }

    /// Bytes in one frame of this format, exactly as `ffmpeg -f rawvideo` writes it.
    pub fn frame_size(self, width: u32, height: u32) -> Result<usize> {
        Ok(self.plane_layout(width, height)?.size())
    }

    /// Offsets, strides and row counts of every plane for the given frame size.
    pub fn plane_layout(self, width: u32, height: u32) -> Result<FrameLayout> {
        let info = self.info();
        if let Support::No(reason) = info.support {
            return Err(Error::UnsupportedPixFmt { name: info.name, reason });
        }
        check_dims(width, height)?;
        let bit_packed = info.has(F_BITSTREAM);
        let mut planes = [NO_PLANE; 5];
        let mut count = 0usize;
        let mut offset: u64 = 0;
        for spec in info.plane_specs() {
            let cols = ceil_shift(width as u64, spec.shift_w);
            let rows = ceil_shift(height as u64, spec.shift_h);
            let mut stride = spec.step as u64 * cols;
            if bit_packed {
                stride = (stride + 7) / 8;
            }
            let size = stride * rows;
            planes[count] = PlaneLayout {
                kind: PlaneKind::Samples,
                offset: offset as usize,
                stride: stride as usize,
                rows: rows as usize,
                size: size as usize,
                cols: cols as usize,
                group_width: 1u32 << spec.shift_w,
                group_bytes: spec.step as usize,
                shift_w: spec.shift_w,
                shift_h: spec.shift_h,
                bit_packed,
            };
            count += 1;
            offset += size;
        }
        if info.palette_bytes > 0 {
            planes[count] = PlaneLayout {
                kind: PlaneKind::Palette,
                offset: offset as usize,
                stride: 4,
                rows: info.palette_bytes as usize / 4,
                size: info.palette_bytes as usize,
                cols: 1,
                group_width: 1,
                group_bytes: 4,
                shift_w: 0,
                shift_h: 0,
                bit_packed: false,
            };
            count += 1;
            offset += info.palette_bytes as u64;
        }
        if offset > i32::MAX as u64 {
            return Err(Error::InvalidSize { width, height, reason: "frame is larger than 2 GiB" });
        }
        Ok(FrameLayout { fmt: self, width, height, planes, count, size: offset as usize })
    }

    /// Bytes per row of plane `plane` for a frame `width` pixels wide, tightly packed.
    pub fn stride(self, plane: usize, width: u32) -> Result<usize> {
        let layout = self.plane_layout(width, 1)?;
        layout
            .planes()
            .get(plane)
            .map(|p| p.stride)
            .ok_or_else(|| Error::InvalidArgument(format!("{} has no plane {}", self.name(), plane)))
    }

    /// Row stride of plane `plane` rounded up to a multiple of `align`.
    pub fn aligned_stride(self, plane: usize, width: u32, align: usize) -> Result<usize> {
        Ok(align_up(self.stride(plane, width)?, align))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_sorted_and_consistent() {
        assert!(BY_NAME.windows(2).all(|w| w[0].0 < w[1].0), "BY_NAME must be sorted");
        assert!(ALIASES.windows(2).all(|w| w[0].0 < w[1].0), "ALIASES must be sorted");
        assert_eq!(BY_NAME.len(), FORMATS.len());
        for (name, idx) in BY_NAME.iter() {
            assert_eq!(FORMATS[*idx as usize].name, *name);
        }
        for (alias, idx) in ALIASES.iter() {
            assert!((*idx as usize) < FORMATS.len(), "{}", alias);
            assert!(PixFmt::from_name(alias).is_some());
            assert!(BY_NAME.binary_search_by(|(n, _)| (*n).cmp(alias)).is_err(), "{} shadows a canonical name", alias);
        }
    }

    #[test]
    fn named_constants_point_at_their_rows() {
        assert_eq!(PixFmt::YUV420P.name(), "yuv420p");
        assert_eq!(PixFmt::RGB24.name(), "rgb24");
        assert_eq!(PixFmt::BGRA.name(), "bgra");
        assert_eq!(PixFmt::GRAY.name(), "gray");
        assert_eq!(PixFmt::NV12.name(), "nv12");
        assert_eq!(PixFmt::YUV420P10LE.name(), "yuv420p10le");
        assert_eq!(PixFmt::_0RGB.name(), "0rgb");
        assert_eq!(PixFmt::PAL8.name(), "pal8");
    }

    #[test]
    fn from_name_round_trips_every_format() {
        for fmt in PixFmt::all() {
            assert_eq!(PixFmt::from_name(fmt.name()), Some(fmt));
            assert_eq!(fmt.to_string(), fmt.name());
        }
        assert_eq!(PixFmt::from_name("not-a-format"), None);
        assert_eq!(PixFmt::from_name("YUV420P"), None, "names are case sensitive like ffmpeg");
        assert!(matches!(PixFmt::parse("bogus"), Err(Error::UnknownPixFmt(n)) if n == "bogus"));
    }

    #[test]
    fn aliases_resolve_to_native_little_endian() {
        if let Some(f) = PixFmt::from_name("yuv420p10") {
            assert_eq!(f, PixFmt::YUV420P10LE);
        }
        if let Some(f) = PixFmt::from_name("rgb32") {
            assert_eq!(f, PixFmt::BGRA);
        }
        if let Some(f) = PixFmt::from_name("gray16") {
            assert_eq!(f, PixFmt::GRAY16LE);
        }
    }

    #[test]
    fn known_frame_sizes() {
        // Values taken from `ffmpeg -f lavfi -i color=c=black:s=WxH -frames:v 1 -pix_fmt FMT -f rawvideo -`.
        assert_eq!(PixFmt::YUV420P.frame_size(1, 1).unwrap(), 3);
        assert_eq!(PixFmt::YUV420P.frame_size(33, 17).unwrap(), 867);
        assert_eq!(PixFmt::YUV420P.frame_size(175, 99).unwrap(), 26125);
        assert_eq!(PixFmt::NV12.frame_size(175, 99).unwrap(), 26125);
        assert_eq!(PixFmt::YUYV422.frame_size(33, 17).unwrap(), 1156);
        assert_eq!(PixFmt::RGB24.frame_size(33, 17).unwrap(), 1683);
        assert_eq!(PixFmt::PAL8.frame_size(33, 17).unwrap(), 1585);
        assert_eq!(PixFmt::MONOW.frame_size(33, 17).unwrap(), 85);
        assert_eq!(PixFmt::RGB4.frame_size(33, 17).unwrap(), 289);
        assert_eq!(PixFmt::YUV420P10LE.frame_size(33, 17).unwrap(), 1734);
        assert_eq!(PixFmt::GBRAPF32LE.frame_size(2, 2).unwrap(), 64);
    }

    #[test]
    fn odd_sizes_round_chroma_up() {
        let l = PixFmt::YUV420P.plane_layout(33, 17).unwrap();
        let p = l.planes();
        assert_eq!(p.len(), 3);
        assert_eq!((p[0].cols, p[0].rows, p[0].stride), (33, 17, 33));
        assert_eq!((p[1].cols, p[1].rows, p[1].stride), (17, 9, 17));
        assert_eq!((p[2].cols, p[2].rows, p[2].offset), (17, 9, 33 * 17 + 17 * 9));
        assert_eq!(l.size(), 33 * 17 + 2 * 17 * 9);
    }

    #[test]
    fn palette_follows_the_pixels() {
        let l = PixFmt::PAL8.plane_layout(33, 17).unwrap();
        assert_eq!(l.planes().len(), 2);
        assert_eq!(l.planes()[1].kind, PlaneKind::Palette);
        assert_eq!(l.planes()[1].offset, 33 * 17);
        assert_eq!(l.planes()[1].size, 1024);
        assert_eq!(l.pixel_planes(), 1);
    }

    #[test]
    fn invalid_sizes_are_rejected() {
        assert!(matches!(PixFmt::RGB24.frame_size(0, 4), Err(Error::InvalidSize { .. })));
        assert!(matches!(PixFmt::RGB24.frame_size(4, 0), Err(Error::InvalidSize { .. })));
        assert!(matches!(PixFmt::RGB24.frame_size(u32::MAX, 2), Err(Error::InvalidSize { .. })));
        assert!(matches!(PixFmt::RGB24.frame_size(40000, 40000), Err(Error::InvalidSize { .. })));
    }

    #[test]
    fn unsupported_formats_say_why() {
        for fmt in PixFmt::all().filter(|f| f.is_hwaccel()) {
            match fmt.frame_size(16, 16) {
                Err(Error::UnsupportedPixFmt { name, reason }) => {
                    assert_eq!(name, fmt.name());
                    assert!(!reason.is_empty());
                }
                other => panic!("{}: {:?}", fmt, other),
            }
        }
        assert!(!PixFmt::VAAPI.is_supported());
        assert!(PixFmt::VAAPI.unsupported_reason().is_some());
    }

    #[test]
    fn layout_planes_are_contiguous_for_every_supported_format() {
        for fmt in PixFmt::all().filter(|f| f.is_supported()) {
            for &(w, h) in &[(1u32, 1u32), (2, 2), (33, 17), (64, 48), (175, 99), (255, 1), (1, 255)] {
                let l = fmt.plane_layout(w, h).unwrap();
                let mut expect = 0usize;
                for p in l.planes() {
                    assert_eq!(p.offset, expect, "{} {}x{}", fmt, w, h);
                    assert_eq!(p.size, p.stride * p.rows, "{} {}x{}", fmt, w, h);
                    expect += p.size;
                }
                assert_eq!(l.size(), expect);
                assert_eq!(l.aligned_size(1), l.size());
                assert!(l.aligned_size(32) >= l.size());
            }
        }
    }

    #[test]
    fn alignment_helpers() {
        assert_eq!(align_up(33, 32), 64);
        assert_eq!(align_up(64, 32), 64);
        assert_eq!(align_up(5, 0), 5);
        assert_eq!(align_up(5, 1), 5);
        assert_eq!(align_up(10, 6), 12);
        assert_eq!(PixFmt::YUV420P.stride(0, 33).unwrap(), 33);
        assert_eq!(PixFmt::YUV420P.stride(1, 33).unwrap(), 17);
        assert_eq!(PixFmt::YUV420P.aligned_stride(1, 33, 16).unwrap(), 32);
        assert!(PixFmt::YUV420P.stride(3, 33).is_err());
    }

    #[test]
    fn byte_order_facts() {
        assert_eq!(PixFmt::GRAY16LE.endian(), Endian::Little);
        assert_eq!(PixFmt::GRAY16BE.endian(), Endian::Big);
        assert_eq!(PixFmt::GRAY16LE.sample_bytes(), 2);
        assert_eq!(PixFmt::RGB24.endian(), Endian::None);
        assert_eq!(PixFmt::RGB24.sample_bytes(), 1);
        assert_eq!(PixFmt::GRAYF32LE.sample_bytes(), 4);
        assert_eq!(PixFmt::YUV420P.chroma_shift(), (1, 1));
        assert_eq!(PixFmt::YUV422P.chroma_shift(), (1, 0));
        assert_eq!(PixFmt::YUV444P.chroma_shift(), (0, 0));
    }
}
