//! Plane types (S3), the error type, seeded synthetic generators and raw little-endian f32 file helpers.
//! Generators here are for quick property tests; they are NOT bit-identical to the harness's numpy image set `std_v1`.
//! T2 tests must read the harness files through `oracle_io` instead of regenerating inputs.

use std::fmt;
use std::path::Path;

/// Read-only f32 plane: `stride` is the row pitch in f32 units, rows are `width` samples long.
#[derive(Clone, Copy, Debug)]
pub struct Plane<'a> {
    pub data: &'a [f32],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

/// Writable f32 plane.
#[derive(Debug)]
pub struct PlaneMut<'a> {
    pub data: &'a mut [f32],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

/// Errors of the common entry point `render(p, src, dst)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FxError {
    /// Wrong number of source or destination planes.
    PlaneCount,
    /// Output geometry differs from the input geometry.
    Geometry,
    /// A stride is smaller than the width, or a slice is too short for height rows.
    Stride,
}

impl fmt::Display for FxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FxError::PlaneCount => "wrong plane count",
            FxError::Geometry => "output geometry differs from input",
            FxError::Stride => "stride smaller than width or slice too short",
        };
        f.write_str(s)
    }
}

impl std::error::Error for FxError {}

impl<'a> Plane<'a> {
    pub fn new(data: &'a [f32], width: usize, height: usize, stride: usize) -> Result<Plane<'a>, FxError> {
        check_geometry(data.len(), width, height, stride)?;
        Ok(Plane { data, width, height, stride })
    }

    /// Row `y` as a slice of `width` samples.
    pub fn row(&self, y: usize) -> &'a [f32] {
        &self.data[y * self.stride..y * self.stride + self.width]
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[y * self.stride + x]
    }
}

impl<'a> PlaneMut<'a> {
    pub fn new(data: &'a mut [f32], width: usize, height: usize, stride: usize) -> Result<PlaneMut<'a>, FxError> {
        check_geometry(data.len(), width, height, stride)?;
        Ok(PlaneMut { data, width, height, stride })
    }

    pub fn row_mut(&mut self, y: usize) -> &mut [f32] {
        let (w, s) = (self.width, self.stride);
        &mut self.data[y * s..y * s + w]
    }

    pub fn set(&mut self, x: usize, y: usize, v: f32) {
        self.data[y * self.stride + x] = v;
    }
}

fn check_geometry(len: usize, width: usize, height: usize, stride: usize) -> Result<(), FxError> {
    if stride < width {
        return Err(FxError::Stride);
    }
    if height > 0 && len < (height - 1) * stride + width {
        return Err(FxError::Stride);
    }
    Ok(())
}

/// Check that every plane in `src` and `dst` has the same width and height (the S3 rule "output geometry equals input").
pub fn check_same_geometry(src: &[Plane], dst: &[PlaneMut]) -> Result<(), FxError> {
    let (w, h) = match src.first() {
        Some(p) => (p.width, p.height),
        None => return Err(FxError::PlaneCount),
    };
    if src.iter().any(|p| p.width != w || p.height != h) || dst.iter().any(|p| p.width != w || p.height != h) {
        return Err(FxError::Geometry);
    }
    Ok(())
}

/// Small seeded generator (xorshift64*), deterministic across platforms.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    /// Uniform in [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / 16777216.0
    }
}

/// Horizontal ramp 0..1 (value x / (width - 1)), `height` rows, stride = width.
pub fn gen_grad(width: usize, height: usize) -> Vec<f32> {
    let d = (width.max(2) - 1) as f32;
    (0..width * height).map(|i| (i % width) as f32 / d).collect()
}

/// Seeded noise in [0, 1).
pub fn gen_noise(width: usize, height: usize, seed: u64) -> Vec<f32> {
    let mut r = Rng::new(seed);
    (0..width * height).map(|_| r.next_f32()).collect()
}

/// Checkerboard of 0.0 and 1.0 with `cell` pixel squares.
pub fn gen_checker(width: usize, height: usize, cell: usize) -> Vec<f32> {
    (0..width * height).map(|i| if ((i % width) / cell + (i / width) / cell) % 2 == 0 { 0.0 } else { 1.0 }).collect()
}

/// Constant plane.
pub fn gen_flat(width: usize, height: usize, v: f32) -> Vec<f32> {
    vec![v; width * height]
}

/// The edge values of the S6 `edge_16x4` case, in order: NaN, +inf, -inf, denormal, -denormal, +0, -0, 1e38, -1e38, then 1, -1, 0.5, 2, 1e-30, -1e-30, 65504.
pub fn edge_values() -> [f32; 16] {
    [
        f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::from_bits(0x0000_0400), f32::from_bits(0x8000_0400), 0.0, -0.0, 1e38, -1e38,
        1.0, -1.0, 0.5, 2.0, 1e-30, -1e-30, 65504.0,
    ]
}

/// Read a raw little-endian f32 file (length must be a multiple of 4).
pub fn read_f32_le(path: &Path) -> Result<Vec<f32>, String> {
    let b = std::fs::read(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    if b.len() % 4 != 0 {
        return Err(format!("{}: length {} is not a multiple of 4", path.display(), b.len()));
    }
    Ok(b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
}

/// Write a raw little-endian f32 file.
pub fn write_f32_le(path: &Path, v: &[f32]) -> Result<(), String> {
    let mut b = Vec::with_capacity(v.len() * 4);
    for x in v {
        b.extend_from_slice(&x.to_le_bytes());
    }
    std::fs::write(path, b).map_err(|e| format!("{}: {}", path.display(), e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_checks() {
        let d = vec![0.0f32; 10];
        assert!(Plane::new(&d, 4, 2, 5).is_ok()); // 5 + 4 = 9 <= 10
        assert_eq!(Plane::new(&d, 4, 3, 5).unwrap_err(), FxError::Stride);
        assert_eq!(Plane::new(&d, 6, 1, 4).unwrap_err(), FxError::Stride);
        let a = Plane::new(&d, 4, 2, 5).unwrap();
        let mut o = vec![0.0f32; 10];
        let m = PlaneMut::new(&mut o, 3, 2, 5).unwrap();
        assert_eq!(check_same_geometry(&[a], &[m]).unwrap_err(), FxError::Geometry);
        assert_eq!(check_same_geometry(&[], &[]).unwrap_err(), FxError::PlaneCount);
    }

    #[test]
    fn generators_are_deterministic() {
        assert_eq!(gen_noise(8, 4, 7), gen_noise(8, 4, 7));
        assert_ne!(gen_noise(8, 4, 7), gen_noise(8, 4, 8));
        let g = gen_grad(5, 2);
        assert_eq!(g[0], 0.0);
        assert_eq!(g[4], 1.0);
        let c = gen_checker(4, 4, 2);
        assert_eq!((c[0], c[2], c[8]), (0.0, 1.0, 1.0));
        assert!(gen_noise(64, 64, 1).iter().all(|v| (0.0..1.0).contains(v)));
    }

    #[test]
    fn raw_f32_round_trip() {
        let p = std::env::temp_dir().join("fx_common_plane_rt.f32");
        let v = vec![1.5f32, -0.0, f32::NAN, 1e-40];
        write_f32_le(&p, &v).unwrap();
        let r = read_f32_le(&p).unwrap();
        assert_eq!(r.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), v.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn rows() {
        let d: Vec<f32> = (0..10).map(|i| i as f32).collect();
        let p = Plane::new(&d, 3, 2, 5).unwrap();
        assert_eq!(p.row(1), &[5.0, 6.0, 7.0]);
        assert_eq!(p.get(2, 1), 7.0);
    }
}
