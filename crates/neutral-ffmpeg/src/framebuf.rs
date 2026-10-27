//! [`FrameBuf`]: one frame of raw video in the layout `ffmpeg -f rawvideo` uses.
//!
//! The bytes are tightly packed: planes follow each other, rows have no padding
//! (stride equals the row length). Helpers convert to and from buffers whose
//! rows are padded to an alignment, which is what most host applications and
//! SIMD code want.

use std::fmt;

use crate::error::{Error, Result};
use crate::pixfmt::{align_up, FrameLayout, PixFmt, PlaneKind, PlaneLayout};

/// One tightly packed frame together with its format and size.
#[derive(Clone, PartialEq, Eq)]
pub struct FrameBuf {
    layout: FrameLayout,
    data: Vec<u8>,
}

impl fmt::Debug for FrameBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FrameBuf {{ {} {}x{}, {} bytes }}",
            self.layout.fmt(),
            self.layout.width(),
            self.layout.height(),
            self.data.len()
        )
    }
}

impl FrameBuf {
    /// A zero filled frame.
    pub fn new(fmt: PixFmt, width: u32, height: u32) -> Result<FrameBuf> {
        let layout = fmt.plane_layout(width, height)?;
        Ok(FrameBuf { data: vec![0; layout.size()], layout })
    }

    /// Take ownership of `data`, which must be exactly one frame long.
    pub fn from_vec(fmt: PixFmt, width: u32, height: u32, data: Vec<u8>) -> Result<FrameBuf> {
        let layout = fmt.plane_layout(width, height)?;
        if data.len() != layout.size() {
            return Err(Error::BufferSize { expected: layout.size(), actual: data.len() });
        }
        Ok(FrameBuf { layout, data })
    }

    /// Copy `data`, which must be exactly one frame long.
    pub fn from_slice(fmt: PixFmt, width: u32, height: u32, data: &[u8]) -> Result<FrameBuf> {
        FrameBuf::from_vec(fmt, width, height, data.to_vec())
    }

    /// Pixel format.
    pub fn fmt(&self) -> PixFmt {
        self.layout.fmt()
    }

    /// Width in pixels.
    pub fn width(&self) -> u32 {
        self.layout.width()
    }

    /// Height in pixels.
    pub fn height(&self) -> u32 {
        self.layout.height()
    }

    /// Plane geometry of this frame.
    pub fn layout(&self) -> &FrameLayout {
        &self.layout
    }

    /// All planes (the palette of paletted formats is the last entry).
    pub fn planes(&self) -> &[PlaneLayout] {
        self.layout.planes()
    }

    /// Length of the frame in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Always false: a frame has at least one pixel.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// The whole frame, exactly the bytes ffmpeg reads or writes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Mutable access to the whole frame.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Give the bytes back.
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    fn plane_layout(&self, plane: usize) -> Option<&PlaneLayout> {
        self.layout.planes().get(plane)
    }

    /// Bytes of plane `plane`.
    pub fn plane(&self, plane: usize) -> Option<&[u8]> {
        let p = self.plane_layout(plane)?;
        Some(&self.data[p.offset..p.offset + p.size])
    }

    /// Mutable bytes of plane `plane`.
    pub fn plane_mut(&mut self, plane: usize) -> Option<&mut [u8]> {
        let p = *self.plane_layout(plane)?;
        Some(&mut self.data[p.offset..p.offset + p.size])
    }

    /// Row `y` of plane `plane`.
    pub fn row(&self, plane: usize, y: usize) -> Option<&[u8]> {
        let p = self.plane_layout(plane)?;
        if y >= p.rows {
            return None;
        }
        let start = p.offset + y * p.stride;
        Some(&self.data[start..start + p.stride])
    }

    /// Mutable row `y` of plane `plane`.
    pub fn row_mut(&mut self, plane: usize, y: usize) -> Option<&mut [u8]> {
        let p = *self.plane_layout(plane)?;
        if y >= p.rows {
            return None;
        }
        let start = p.offset + y * p.stride;
        Some(&mut self.data[start..start + p.stride])
    }

    /// Tight stride of plane `plane` (bytes per row).
    pub fn stride(&self, plane: usize) -> Option<usize> {
        self.plane_layout(plane).map(|p| p.stride)
    }

    /// Stride of plane `plane` rounded up to a multiple of `align`.
    pub fn aligned_stride(&self, plane: usize, align: usize) -> Option<usize> {
        self.plane_layout(plane).map(|p| align_up(p.stride, align))
    }

    /// Length of [`FrameBuf::to_aligned`] output for `align`.
    pub fn aligned_len(&self, align: usize) -> usize {
        self.layout.aligned_size(align)
    }

    /// Copy the frame into one buffer whose rows are padded with zeros to a multiple
    /// of `align` bytes (planes still follow each other, as in ffmpeg's
    /// `av_image_copy_to_buffer`). The palette, if any, is appended unpadded.
    pub fn to_aligned(&self, align: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.aligned_len(align));
        for p in self.planes() {
            match p.kind {
                PlaneKind::Palette => out.extend_from_slice(&self.data[p.offset..p.offset + p.size]),
                PlaneKind::Samples => {
                    let padded = align_up(p.stride, align);
                    for y in 0..p.rows {
                        let start = p.offset + y * p.stride;
                        out.extend_from_slice(&self.data[start..start + p.stride]);
                        out.resize(out.len() + (padded - p.stride), 0);
                    }
                }
            }
        }
        out
    }

    /// Inverse of [`FrameBuf::to_aligned`]: strip the row padding.
    pub fn from_aligned(fmt: PixFmt, width: u32, height: u32, align: usize, bytes: &[u8]) -> Result<FrameBuf> {
        let mut frame = FrameBuf::new(fmt, width, height)?;
        let expected = frame.aligned_len(align);
        if bytes.len() != expected {
            return Err(Error::BufferSize { expected, actual: bytes.len() });
        }
        let planes: Vec<PlaneLayout> = frame.planes().to_vec();
        let mut at = 0usize;
        for p in planes {
            match p.kind {
                PlaneKind::Palette => {
                    frame.data[p.offset..p.offset + p.size].copy_from_slice(&bytes[at..at + p.size]);
                    at += p.size;
                }
                PlaneKind::Samples => {
                    let padded = align_up(p.stride, align);
                    for y in 0..p.rows {
                        let dst = p.offset + y * p.stride;
                        frame.data[dst..dst + p.stride].copy_from_slice(&bytes[at..at + p.stride]);
                        at += padded;
                    }
                }
            }
        }
        Ok(frame)
    }

    /// Copy plane `plane` into `dst`, whose rows are `dst_stride` bytes apart.
    pub fn copy_plane_to(&self, plane: usize, dst: &mut [u8], dst_stride: usize) -> Result<()> {
        let p = *self
            .plane_layout(plane)
            .ok_or_else(|| Error::InvalidArgument(format!("{} has no plane {}", self.fmt(), plane)))?;
        let needed = strided_len(p.rows, p.stride, dst_stride)?;
        if dst.len() < needed {
            return Err(Error::BufferSize { expected: needed, actual: dst.len() });
        }
        for y in 0..p.rows {
            let src = p.offset + y * p.stride;
            dst[y * dst_stride..y * dst_stride + p.stride].copy_from_slice(&self.data[src..src + p.stride]);
        }
        Ok(())
    }

    /// Fill plane `plane` from `src`, whose rows are `src_stride` bytes apart.
    pub fn copy_plane_from(&mut self, plane: usize, src: &[u8], src_stride: usize) -> Result<()> {
        let p = *self
            .plane_layout(plane)
            .ok_or_else(|| Error::InvalidArgument(format!("{} has no plane {}", self.fmt(), plane)))?;
        let needed = strided_len(p.rows, p.stride, src_stride)?;
        if src.len() < needed {
            return Err(Error::BufferSize { expected: needed, actual: src.len() });
        }
        for y in 0..p.rows {
            let dst = p.offset + y * p.stride;
            self.data[dst..dst + p.stride].copy_from_slice(&src[y * src_stride..y * src_stride + p.stride]);
        }
        Ok(())
    }
}

/// Bytes a buffer needs to hold `rows` rows of `row_bytes` at `stride` spacing.
fn strided_len(rows: usize, row_bytes: usize, stride: usize) -> Result<usize> {
    if stride < row_bytes {
        return Err(Error::InvalidArgument(format!("stride {} is smaller than the row length {}", stride, row_bytes)));
    }
    Ok(rows.saturating_sub(1) * stride + row_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counting(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i % 251) as u8).collect()
    }

    #[test]
    fn new_is_zeroed_and_sized() {
        let f = FrameBuf::new(PixFmt::YUV420P, 33, 17).unwrap();
        assert_eq!(f.len(), 867);
        assert!(f.as_bytes().iter().all(|&b| b == 0));
        assert_eq!((f.width(), f.height(), f.fmt()), (33, 17, PixFmt::YUV420P));
        assert_eq!(format!("{:?}", f), "FrameBuf { yuv420p 33x17, 867 bytes }");
    }

    #[test]
    fn from_vec_checks_length() {
        assert!(FrameBuf::from_vec(PixFmt::RGB24, 4, 4, vec![0; 48]).is_ok());
        match FrameBuf::from_vec(PixFmt::RGB24, 4, 4, vec![0; 47]) {
            Err(Error::BufferSize { expected: 48, actual: 47 }) => {}
            other => panic!("{:?}", other),
        }
        assert!(FrameBuf::new(PixFmt::VAAPI, 4, 4).is_err());
    }

    #[test]
    fn planes_rows_and_strides() {
        let mut f = FrameBuf::from_vec(PixFmt::YUV420P, 6, 4, counting(24 + 12)).unwrap();
        assert_eq!(f.planes().len(), 3);
        assert_eq!(f.plane(0).unwrap().len(), 24);
        assert_eq!(f.plane(1).unwrap().len(), 6);
        assert_eq!(f.plane(3), None);
        assert_eq!(f.stride(0), Some(6));
        assert_eq!(f.stride(1), Some(3));
        assert_eq!(f.aligned_stride(1, 16), Some(16));
        assert_eq!(f.row(0, 1).unwrap(), &counting(36)[6..12]);
        assert_eq!(f.row(0, 4), None);
        f.row_mut(1, 1).unwrap().copy_from_slice(&[9, 9, 9]);
        assert_eq!(&f.plane(1).unwrap()[3..6], &[9, 9, 9]);
        f.plane_mut(2).unwrap().fill(7);
        assert!(f.plane(2).unwrap().iter().all(|&b| b == 7));
    }

    #[test]
    fn aligned_round_trip_for_every_supported_format() {
        let sizes = [(1u32, 1u32), (3, 5), (33, 17), (64, 48)];
        for fmt in PixFmt::all().filter(|f| f.is_supported()) {
            for &(w, h) in &sizes {
                let layout = fmt.plane_layout(w, h).unwrap();
                let f = FrameBuf::from_vec(fmt, w, h, counting(layout.size())).unwrap();
                for &align in &[1usize, 4, 32, 64] {
                    let padded = f.to_aligned(align);
                    assert_eq!(padded.len(), f.aligned_len(align), "{} {}x{} align {}", fmt, w, h, align);
                    let back = FrameBuf::from_aligned(fmt, w, h, align, &padded).unwrap();
                    assert_eq!(back, f, "{} {}x{} align {}", fmt, w, h, align);
                }
            }
        }
    }

    #[test]
    fn aligned_padding_is_zero_and_rows_start_on_the_boundary() {
        let f = FrameBuf::from_vec(PixFmt::GRAY, 5, 3, counting(15)).unwrap();
        let padded = f.to_aligned(8);
        assert_eq!(padded.len(), 24);
        for y in 0..3 {
            assert_eq!(&padded[y * 8..y * 8 + 5], &counting(15)[y * 5..y * 5 + 5]);
            assert_eq!(&padded[y * 8 + 5..y * 8 + 8], &[0, 0, 0]);
        }
        assert!(FrameBuf::from_aligned(PixFmt::GRAY, 5, 3, 8, &padded[..23]).is_err());
    }

    #[test]
    fn strided_copies() {
        let f = FrameBuf::from_vec(PixFmt::GRAY, 4, 3, counting(12)).unwrap();
        let mut dst = vec![0xAA; 3 * 6];
        f.copy_plane_to(0, &mut dst, 6).unwrap();
        assert_eq!(&dst[0..4], &[0, 1, 2, 3]);
        assert_eq!(&dst[4..6], &[0xAA, 0xAA]);
        assert_eq!(&dst[12..16], &[8, 9, 10, 11]);
        let mut g = FrameBuf::new(PixFmt::GRAY, 4, 3).unwrap();
        g.copy_plane_from(0, &dst, 6).unwrap();
        assert_eq!(g, f);
        assert!(f.copy_plane_to(0, &mut dst, 3).is_err(), "stride smaller than the row");
        assert!(f.copy_plane_to(0, &mut dst[..10], 6).is_err(), "destination too short");
        assert!(f.copy_plane_to(2, &mut dst, 6).is_err(), "no such plane");
    }

    #[test]
    fn palette_is_kept_by_the_aligned_conversion() {
        let mut data = counting(5 * 2);
        data.extend((0..1024).map(|i| (i % 7) as u8));
        let f = FrameBuf::from_vec(PixFmt::PAL8, 5, 2, data).unwrap();
        let padded = f.to_aligned(8);
        assert_eq!(padded.len(), 16 + 1024);
        assert_eq!(FrameBuf::from_aligned(PixFmt::PAL8, 5, 2, 8, &padded).unwrap(), f);
    }
}
