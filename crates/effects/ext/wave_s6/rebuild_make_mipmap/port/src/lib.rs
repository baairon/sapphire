//! make_mipmap (MM1): native port of the pyramid builder of the class make_mipmap (vtable).
//!
//! Scope (SPEC.md): slot 34 callee -> AVX2 driver body and its two launched kernels (AVX2 tiers).
//! The image object is a RIPMAP: a grid of NL x NP planes. Plane (l, j) has width widths[l] (halved along x with l) and height
//! heights[j] (halved along y with j), C interleaved f32 channels per pixel, and a padded layout: row stride (widths[l] + 3) * C floats,
//! 1 top pad row, heights[j] interior rows, 2 bottom pad rows (one more spare row is allocated and never written), 1 left pad pixel and
//! 2 right pad pixels per row. Std only, no unsafe, explicit parameters (no Default).
//!
//! Floats are held as u32 bit patterns so copies are bit exact; NaN payloads follow the host arithmetic (compared as a class).

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Channels,
    Dims(String),
    Pad,
    Planes(String),
    Contract(String),
}

/// Rows allocated per plane beyond the interior: 1 top pad + 2 bottom pad + 1 spare that is never written.
pub const EXTRA_ROWS: usize = 4;

/// Explicit parameters (this wrap_x, this wrap_y, this filter, rec pad_pixel). No Default.
#[derive(Debug, Clone)]
pub struct Params {
    /// column pad mode: 1 = tile, 2 = clamp, any other value = constant pad pixel (kernels branch on 2, then 1, else constant)
    pub wrap_x: i32,
    /// row pad mode: 0 = constant pad pixel, 1 = tile, 2 = clamp, any other value = no row padding (driver branches 0, 1, 2, else skip)
    pub wrap_y: i32,
    /// nonzero builds the pyramid levels, zero only pads the base level rows
    pub filter: i32,
    /// C bit patterns: the constant pad pixel (rec row)
    pub pad_pixel: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct RipMap {
    pub c: usize,
    pub widths: Vec<usize>,
    pub heights: Vec<usize>,
    /// planes[l * heights.len() + j], bit patterns, padded layout
    pub planes: Vec<Vec<u32>>,
}

impl RipMap {
    pub fn new(c: usize, widths: Vec<usize>, heights: Vec<usize>, fill: u32) -> Result<RipMap, Error> {
        if c == 0 {
            return Err(Error::Channels);
        }
        if widths.is_empty() || heights.is_empty() || widths.iter().chain(heights.iter()).any(|&v| v == 0) {
            return Err(Error::Dims("empty or zero extent".into()));
        }
        let mut planes = Vec::new();
        for &w in &widths {
            for &h in &heights {
                planes.push(vec![fill; (h + EXTRA_ROWS) * (w + 3) * c]);
            }
        }
        Ok(RipMap { c, widths, heights, planes })
    }
    pub fn np(&self) -> usize {
        self.heights.len()
    }
    pub fn nl(&self) -> usize {
        self.widths.len()
    }
    pub fn stride(&self, l: usize) -> usize {
        (self.widths[l] + 3) * self.c
    }
    pub fn idx(&self, l: usize, j: usize) -> usize {
        l * self.heights.len() + j
    }
    pub fn plane_len(&self, l: usize, j: usize) -> usize {
        (self.heights[j] + EXTRA_ROWS) * self.stride(l)
    }
    /// float index of pixel (x, y) channel ch, interior coordinates (0-based); x = -1, w, w+1 and y = -1, h, h+1 address the pads
    pub fn at(&self, l: usize, x: isize, y: isize, ch: usize) -> usize {
        ((1 + y) as usize) * self.stride(l) + ((1 + x) as usize) * self.c + ch
    }
}

/// Deliberate bugs for the mutation test (T2). `None` is the port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mut {
    None,
    TileTopRow1,
    TileBottom2Row2,
    ClampTopRowH,
    ClampBottomRowHm1,
    ZeroTestBitsNonzero,
    ColTileRight2Col2,
    ColClampLeftLast,
    ConstColsZero,
    HalfBecomesQuarter,
    KernelAColShift,
    KernelBRowShift,
    SkipBasePad,
    SkipPhase1Pad,
    FilterIgnored,
    WrapYOtherAsZero,
    ConstRowsTiledAlways,
    SkipPhase2Pad,
    Phase2OnlyLevel0,
    SwapA,
    SwapB,
    SwapAB,
}

pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("MM1-A1", "CAPTURED 0.5f scalar and 8 x 0.5f ymm constant (32 B), both read from the captured image; the vector path is output-identical to the scalar path"),
        ("MM1-A2", "ROUTE AVX2-ONLY: feature word 0x9d97ff served through the cell (captured pointer); AVX (.) and SSE tiers are not ported"),
        ("MM1-A3", "HOST-FP-ASSUMED: MXCSR round to nearest, no FTZ, no DAZ; the VEX engine follows the SDM, not silicon"),
        ("MM1-A4", "the image object (grid NL x NP, widths at, heights at, plane tables at, channel count at) is built by the host allocator (slot 33, vtable calls); here it is an explicit input, NOT REBUILT"),
        ("MM1-A5", "CALLEE-MODELLED / HOST-HOOKED: the launcher runs the kernel once on one thread (serial_for) and the tile partition returns (0, total); memcpy and memset are modelled; accessors are real emulated code"),
        ("MM1-A6", "CONTRACT-LIMIT: level extents obey 2 * widths[l] <= widths[l-1] + 2 and 2 * heights[j] <= heights[j-1] + 2 (kernel reads stay inside the padded planes); the host allocator's rounding rule (INFERRED ceil of half) is not part of this port"),
        ("MM1-A7", "the base level (0,0) interior and its left/right pad columns are written by slot 32 (SSE2 body), which is NOT REBUILT in this piece; they are explicit inputs here"),
        ("MM1-A8", "ALIGN-DEPENDENT: none on outputs; the alignment peels of the driver and kernels (channel count >= 16) select scalar or vector forms of the same exact add/mul, so outputs do not depend on buffer addresses (MEASURED by C = 16..33 cases at phase 0 only)"),
    ]
}

fn is_nan_bits(a: u32) -> bool {
    (a & 0x7f80_0000) == 0x7f80_0000 && (a & 0x007f_ffff) != 0
}

fn is_snan_bits(a: u32) -> bool {
    is_nan_bits(a) && (a & 0x0040_0000) == 0
}

/// NaN rule measured against the VEX engine (T2 diag): one SNaN and one QNaN gives the QNaN; two QNaN or two SNaN give the first operand
/// (quieted); one NaN gives that NaN (quieted); otherwise the IEEE sum (inf - inf gives the host default NaN)
/// NaN pair rule switch (MM2): false = the port rule above (one SNaN and one QNaN gives the QNaN, otherwise the first operand), true = the rule of the
/// emulator's float core (Unicorn/QEMU softfloat, x87 style: QNaN beats SNaN, then the larger significand, then the positive sign), measured to explain the
/// NaN payload residue of MM1 (avx/notes.md: "Unicorn selects the NaN by the x87 rule, SDM and hardware use the first source operand")
static NAN_X87: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_nan_rule_x87(on: bool) {
    NAN_X87.store(on, std::sync::atomic::Ordering::Relaxed);
}

fn fadd(a: u32, b: u32) -> u32 {
    let (na, nb) = (is_nan_bits(a), is_nan_bits(b));
    if na && nb {
        if NAN_X87.load(std::sync::atomic::Ordering::Relaxed) {
            let (sa, sb) = (is_snan_bits(a), is_snan_bits(b));
            let pick = if sa && !sb {
                b
            } else if sb && !sa {
                a
            } else {
                let (fa, fb) = (a & 0x007f_ffff, b & 0x007f_ffff);
                if fa > fb || (fa == fb && (a >> 31) < (b >> 31)) {
                    a
                } else {
                    b
                }
            };
            return pick | 0x0040_0000;
        }
        if is_snan_bits(a) && !is_snan_bits(b) {
            return b;
        }
        if !is_snan_bits(a) && is_snan_bits(b) {
            return a;
        }
        return a | 0x0040_0000;
    }
    if na {
        return a | 0x0040_0000;
    }
    if nb {
        return b | 0x0040_0000;
    }
    (f32::from_bits(a) + f32::from_bits(b)).to_bits()
}
fn avg(a: u32, b: u32, m: Mut) -> u32 {
    let s = fadd(a, b);
    if is_nan_bits(s) {
        return s | 0x0040_0000;
    }
    let k = if m == Mut::HalfBecomesQuarter { 0.25f32 } else { 0.5f32 };
    (k * f32::from_bits(s)).to_bits()
}

fn pad_is_zero(p: &Params, m: Mut) -> bool {
    if m == Mut::ZeroTestBitsNonzero {
        p.pad_pixel.iter().all(|&v| v == 0)
    } else {
        p.pad_pixel.iter().all(|&v| f32::from_bits(v) == 0.0)
    }
}

/// driver: the three pad rows of plane (l, j) (rows 0, h+1, h+2 of the padded layout)
fn pad_rows(rm: &mut RipMap, l: usize, j: usize, p: &Params, m: Mut) {
    let s = rm.stride(l);
    let h = rm.heights[j];
    let c = rm.c;
    let id = rm.idx(l, j);
    let zero = pad_is_zero(p, m);
    let wy = if m == Mut::WrapYOtherAsZero && !(0..=2).contains(&p.wrap_y) { 0 } else { p.wrap_y };
    let pl = &mut rm.planes[id];
    let row = |r: usize| r * s..(r + 1) * s;
    match wy {
        0 => {
            for r in [0, h + 1, h + 2] {
                for k in row(r) {
                    pl[k] = if zero && m != Mut::ConstRowsTiledAlways { 0 } else { p.pad_pixel[k % c] };
                }
            }
        }
        1 => {
            let top_src = if m == Mut::TileTopRow1 { 1 } else { h };
            let b2_src = if m == Mut::TileBottom2Row2 { 2.min(h) } else { 1 };
            let t: Vec<u32> = pl[row(top_src)].to_vec();
            let b: Vec<u32> = pl[row(1)].to_vec();
            let b2: Vec<u32> = pl[row(b2_src)].to_vec();
            pl[row(0)].copy_from_slice(&t);
            pl[row(h + 1)].copy_from_slice(&b);
            pl[row(h + 2)].copy_from_slice(&b2);
        }
        2 => {
            let top_src = if m == Mut::ClampTopRowH { h } else { 1 };
            let bot_src = if m == Mut::ClampBottomRowHm1 { h.saturating_sub(1).max(1) } else { h };
            let t: Vec<u32> = pl[row(top_src)].to_vec();
            let b: Vec<u32> = pl[row(bot_src)].to_vec();
            pl[row(0)].copy_from_slice(&t);
            pl[row(h + 1)].copy_from_slice(&b);
            pl[row(h + 2)].copy_from_slice(&b);
        }
        _ => {}
    }
}

/// kernel pad columns of interior row y of plane (l, j): left pad pixel and two right pad pixels
fn pad_cols(rm: &mut RipMap, l: usize, j: usize, y: usize, p: &Params, m: Mut) {
    let c = rm.c;
    let w = rm.widths[l];
    let s = rm.stride(l);
    let id = rm.idx(l, j);
    let base = (1 + y) * s;
    let pl = &mut rm.planes[id];
    for ch in 0..c {
        let first = pl[base + c + ch];
        let last = pl[base + w * c + ch];
        let (left, r1, r2) = match p.wrap_x {
            1 => (last, first, if m == Mut::ColTileRight2Col2 { pl[base + (2.min(w)) * c + ch] } else { first }),
            2 => (if m == Mut::ColClampLeftLast { last } else { first }, last, last),
            _ => {
                let v = if m == Mut::ConstColsZero { 0 } else { p.pad_pixel[ch] };
                (v, v, v)
            }
        };
        pl[base + ch] = left;
        pl[base + (w + 1) * c + ch] = r1;
        pl[base + (w + 2) * c + ch] = r2;
    }
}

/// kernel : horizontal 2:1 for level l of row-level 0
fn kernel_a(rm: &mut RipMap, l: usize, p: &Params, m: Mut) {
    let c = rm.c;
    let h0 = rm.heights[0];
    let wl = rm.widths[l];
    let sp = rm.stride(l - 1);
    let sd = rm.stride(l);
    let src = rm.planes[rm.idx(l - 1, 0)].clone();
    let did = rm.idx(l, 0);
    let shift = if m == Mut::KernelAColShift { 1 } else { 0 };
    for y in 0..h0 {
        for x in 0..wl {
            for ch in 0..c {
                let a = src[(1 + y) * sp + (1 + 2 * x + shift) * c + ch];
                let b = src[(1 + y) * sp + (2 + 2 * x + shift) * c + ch];
                rm.planes[did][(1 + y) * sd + (1 + x) * c + ch] = avg_order(a, b, m, true);
            }
        }
        pad_cols(rm, l, 0, y, p, m);
    }
}

/// kernel : vertical 2:1 for row-level j, all x levels
fn kernel_b(rm: &mut RipMap, l: usize, j: usize, p: &Params, m: Mut) {
    let c = rm.c;
    let wl = rm.widths[l];
    let hj = rm.heights[j];
    let s = rm.stride(l);
    let src = rm.planes[rm.idx(l, j - 1)].clone();
    let did = rm.idx(l, j);
    let shift = if m == Mut::KernelBRowShift { 1 } else { 0 };
    for y in 0..hj {
        for x in 0..wl {
            for ch in 0..c {
                let k = (1 + x) * c + ch;
                let a = src[(1 + 2 * y + shift) * s + k];
                let b = src[(2 + 2 * y + shift) * s + k];
                rm.planes[did][(1 + y) * s + k] = avg_order(a, b, m, false);
            }
        }
        pad_cols(rm, l, j, y, p, m);
    }
}

fn validate(p: &Params, rm: &RipMap) -> Result<(), Error> {
    if rm.c == 0 {
        return Err(Error::Channels);
    }
    if p.pad_pixel.len() != rm.c {
        return Err(Error::Pad);
    }
    if rm.widths.is_empty() || rm.heights.is_empty() || rm.widths.iter().chain(rm.heights.iter()).any(|&v| v == 0) {
        return Err(Error::Dims("empty or zero extent".into()));
    }
    if rm.planes.len() != rm.nl() * rm.np() {
        return Err(Error::Planes("plane count".into()));
    }
    for l in 0..rm.nl() {
        for j in 0..rm.np() {
            if rm.planes[rm.idx(l, j)].len() != rm.plane_len(l, j) {
                return Err(Error::Planes(format!("plane ({},{}) length", l, j)));
            }
        }
    }
    Ok(())
}

fn contract(rm: &RipMap) -> Result<(), Error> {
    for l in 1..rm.nl() {
        if 2 * rm.widths[l] > rm.widths[l - 1] + 2 {
            return Err(Error::Contract(format!("2 * widths[{}] > widths[{}] + 2", l, l - 1)));
        }
    }
    for j in 1..rm.np() {
        if 2 * rm.heights[j] > rm.heights[j - 1] + 2 {
            return Err(Error::Contract(format!("2 * heights[{}] > heights[{}] + 2", j, j - 1)));
        }
    }
    Ok(())
}

/// The driver (AVX2 body) with its kernels: pads the base level rows, then (filter != 0) builds the x levels of row-level 0
/// with the horizontal kernel and pads each, then for each row-level j >= 1 builds all x levels with the vertical kernel and pads them.
/// The base level (0, 0) interior and its left/right pad columns must be present in `rm` (written by slot 32, NOT REBUILT here).
pub fn build(p: &Params, rm: &mut RipMap) -> Result<(), Error> {
    build_m(p, rm, Mut::None)
}

pub fn build_m(p: &Params, rm: &mut RipMap, m: Mut) -> Result<(), Error> {
    validate(p, rm)?;
    if m != Mut::SkipBasePad {
        pad_rows(rm, 0, 0, p, m);
    }
    if p.filter == 0 && m != Mut::FilterIgnored {
        return Ok(());
    }
    contract(rm)?;
    let (nl, np) = (rm.nl(), rm.np());
    if nl > 1 {
        for l in 1..nl {
            kernel_a(rm, l, p, m);
            if m != Mut::SkipPhase1Pad {
                pad_rows(rm, l, 0, p, m);
            }
        }
    }
    if np > 1 {
        for j in 1..np {
            let lmax = if m == Mut::Phase2OnlyLevel0 { 1 } else { nl };
            for l in 0..lmax {
                kernel_b(rm, l, j, p, m);
            }
            if m != Mut::SkipPhase2Pad {
                for l in 0..lmax {
                    pad_rows(rm, l, j, p, m);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fb(x: f32) -> u32 {
        x.to_bits()
    }
    fn gf(rm: &RipMap, l: usize, j: usize, x: isize, y: isize, ch: usize) -> f32 {
        f32::from_bits(rm.planes[rm.idx(l, j)][rm.at(l, x, y, ch)])
    }
    /// base 5 x 4, C = 1: interior 100 y + 10 x + 1, left pad 5000 + 10 y + 1, right pads 6000 / 7000 + 10 y + 1 (as in the probe)
    fn base(wx: i32, wy: i32, filter: i32, pad: f32) -> (Params, RipMap) {
        let mut rm = RipMap::new(1, vec![5, 3], vec![4, 2], fb(f32::from_bits(0x7fc0ffee))).unwrap();
        for y in 0..4isize {
            let i = rm.at(0, -1, y, 0);
            rm.planes[0][i] = fb(5000.0 + 10.0 * y as f32 + 1.0);
            for x in 0..5isize {
                let i = rm.at(0, x, y, 0);
                rm.planes[0][i] = fb(100.0 * y as f32 + 10.0 * x as f32 + 1.0);
            }
            let i = rm.at(0, 5, y, 0);
            rm.planes[0][i] = fb(6000.0 + 10.0 * y as f32 + 1.0);
            let i = rm.at(0, 6, y, 0);
            rm.planes[0][i] = fb(7000.0 + 10.0 * y as f32 + 1.0);
        }
        (Params { wrap_x: wx, wrap_y: wy, filter, pad_pixel: vec![fb(pad)] }, rm)
    }

    #[test]
    fn hand_values_constant_pad() {
        // out = 0.5 * (a + b); level (1,0) x = 0 is (1 + 11) / 2 = 6, x = 1 is (21 + 31) / 2 = 26, x = 2 is (41 + 6001) / 2
        let (p, mut rm) = base(0, 0, 1, -1.0);
        build(&p, &mut rm).unwrap();
        assert_eq!(gf(&rm, 1, 0, 0, 0, 0), 6.0);
        assert_eq!(gf(&rm, 1, 0, 1, 0, 0), 26.0);
        assert_eq!(gf(&rm, 1, 0, 2, 0, 0), 3021.0);
        // vertical kernel : (0,1) y = 0 is (1 + 101) / 2 = 51, y = 1 is (201 + 301) / 2 = 251
        assert_eq!(gf(&rm, 0, 1, 0, 0, 0), 51.0);
        assert_eq!(gf(&rm, 0, 1, 0, 1, 0), 251.0);
        assert_eq!(gf(&rm, 1, 1, 0, 0, 0), 56.0);
        // constant pad everywhere: base top/bottom rows, level left/right columns
        assert_eq!(gf(&rm, 0, 0, 2, -1, 0), -1.0);
        assert_eq!(gf(&rm, 0, 0, 2, 4, 0), -1.0);
        assert_eq!(gf(&rm, 0, 0, 2, 5, 0), -1.0);
        assert_eq!(gf(&rm, 1, 0, -1, 2, 0), -1.0);
        assert_eq!(gf(&rm, 1, 0, 3, 2, 0), -1.0);
        assert_eq!(gf(&rm, 1, 0, 4, 2, 0), -1.0);
        // base left/right pad columns are left alone
        assert_eq!(gf(&rm, 0, 0, -1, 1, 0), 5011.0);
    }

    #[test]
    fn tile_and_clamp_rules() {
        let (p, mut rm) = base(1, 1, 1, -1.0);
        build(&p, &mut rm).unwrap();
        // rows: top <- row h, both bottoms <- row 1 (whole padded rows)
        assert_eq!(gf(&rm, 0, 0, 2, -1, 0), gf(&rm, 0, 0, 2, 3, 0));
        assert_eq!(gf(&rm, 0, 0, 2, 4, 0), 21.0);
        assert_eq!(gf(&rm, 0, 0, 2, 5, 0), 21.0);
        // columns: left <- last, right1 and right2 <- first
        assert_eq!(gf(&rm, 1, 0, -1, 0, 0), gf(&rm, 1, 0, 2, 0, 0));
        assert_eq!(gf(&rm, 1, 0, 3, 0, 0), 6.0);
        assert_eq!(gf(&rm, 1, 0, 4, 0, 0), 6.0);
        let (p, mut rm) = base(2, 2, 1, -1.0);
        build(&p, &mut rm).unwrap();
        assert_eq!(gf(&rm, 0, 0, 2, -1, 0), 21.0);
        assert_eq!(gf(&rm, 0, 0, 2, 4, 0), 321.0);
        assert_eq!(gf(&rm, 0, 0, 2, 5, 0), 321.0);
        assert_eq!(gf(&rm, 1, 0, -1, 0, 0), 6.0);
        assert_eq!(gf(&rm, 1, 0, 3, 0, 0), 3021.0);
        assert_eq!(gf(&rm, 1, 0, 4, 0, 0), 3021.0);
    }

    #[test]
    fn filter_zero_pads_base_rows_only() {
        let (p, mut rm) = base(0, 0, 0, -1.0);
        let before = rm.planes[1].clone();
        build(&p, &mut rm).unwrap();
        assert_eq!(rm.planes[1], before);
        assert_eq!(gf(&rm, 0, 0, 2, -1, 0), -1.0);
    }

    #[test]
    fn other_row_mode_pads_nothing() {
        let (p, mut rm) = base(0, 3, 1, -1.0);
        build(&p, &mut rm).unwrap();
        assert_eq!(rm.planes[0][rm.at(0, 2, -1, 0)], 0x7fc0ffee);
    }

    #[test]
    fn zero_pad_pixel_writes_positive_zero_rows_but_copies_columns() {
        let (p, mut rm) = base(0, 0, 1, -0.0);
        build(&p, &mut rm).unwrap();
        assert_eq!(rm.planes[0][rm.at(0, 2, -1, 0)], 0); // memset path: +0
        assert_eq!(rm.planes[rm.idx(1, 0)][rm.at(1, -1, 0, 0)], 0x8000_0000); // column pad copies the pad pixel: -0
        let (p, mut rm) = base(0, 0, 1, 0.5);
        build(&p, &mut rm).unwrap();
        assert_eq!(gf(&rm, 0, 0, 2, -1, 0), 0.5);
    }

    #[test]
    fn nan_inf_and_overflow() {
        let (p, mut rm) = base(0, 0, 1, f32::NAN);
        let i = rm.at(0, 0, 0, 0);
        rm.planes[0][i] = fb(f32::INFINITY);
        let i = rm.at(0, 1, 0, 0);
        rm.planes[0][i] = fb(f32::NEG_INFINITY);
        let i = rm.at(0, 2, 0, 0);
        rm.planes[0][i] = fb(3.0e38);
        let i = rm.at(0, 3, 0, 0);
        rm.planes[0][i] = fb(3.0e38);
        build(&p, &mut rm).unwrap();
        assert!(gf(&rm, 1, 0, 0, 0, 0).is_nan()); // inf + -inf
        assert_eq!(gf(&rm, 1, 0, 1, 0, 0), f32::INFINITY); // 3e38 + 3e38 overflows before the halving
        assert!(gf(&rm, 0, 0, 2, -1, 0).is_nan());
    }

    #[test]
    fn deterministic_degenerate_and_errors() {
        let (p, mut a) = base(1, 2, 1, 0.25);
        let mut b = a.clone();
        build(&p, &mut a).unwrap();
        build(&p, &mut b).unwrap();
        assert_eq!(a.planes, b.planes);
        // 1 x 1 base, one level: only the row pads
        let mut rm = RipMap::new(2, vec![1], vec![1], 0).unwrap();
        let p = Params { wrap_x: 0, wrap_y: 1, filter: 1, pad_pixel: vec![1, 2] };
        build(&p, &mut rm).unwrap();
        assert!(RipMap::new(0, vec![1], vec![1], 0).is_err());
        assert!(RipMap::new(1, vec![], vec![1], 0).is_err());
        assert!(RipMap::new(1, vec![0], vec![1], 0).is_err());
        let mut rm = RipMap::new(1, vec![4, 4], vec![1], 0).unwrap();
        let p = Params { wrap_x: 0, wrap_y: 0, filter: 1, pad_pixel: vec![0] };
        assert!(matches!(build(&p, &mut rm), Err(Error::Contract(_))));
        let p = Params { wrap_x: 0, wrap_y: 0, filter: 1, pad_pixel: vec![] };
        assert!(matches!(build(&p, &mut rm), Err(Error::Pad)));
        assert_eq!(assumptions().len(), 8);
    }

    #[test]
    fn every_mutant_changes_something_on_the_probe_case() {
        let mut changed = 0;
        let muts = [Mut::TileTopRow1, Mut::KernelAColShift, Mut::HalfBecomesQuarter, Mut::SkipBasePad, Mut::FilterIgnored];
        for m in muts {
            let (p, mut a) = base(1, 1, if m == Mut::FilterIgnored { 0 } else { 1 }, -1.0);
            let (_, mut b) = base(1, 1, if m == Mut::FilterIgnored { 0 } else { 1 }, -1.0);
            build_m(&p, &mut a, Mut::None).unwrap();
            build_m(&p, &mut b, m).unwrap();
            if a.planes != b.planes {
                changed += 1;
            }
        }
        assert_eq!(changed, muts.len());
    }
}

/// NaN operand order experiment (T2 measures which order the emulated kernels use): kernel A passes `is_a = true`, kernel B `false`.
fn avg_order(a: u32, b: u32, m: Mut, is_a: bool) -> u32 {
    let swap = match m {
        Mut::SwapAB => true,
        Mut::SwapA => is_a,
        Mut::SwapB => !is_a,
        _ => false,
    };
    if swap {
        avg(a, b, m)
    } else {
        avg(b, a, m)
    }
}

pub mod mm2;
