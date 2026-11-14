//! hsv_to_rgb: the AVX2 body 0x5540f0 to 0x554b1f of the dispatcher 0x5540a0.
//!
//! Two forms are selected by the flag byte [obj+0x44]: nonzero reads h, s and v from the object (cells 0x4c, 0x50, 0x54)
//! and fills the planes with one value (the constant form); zero reads the three input planes. The flag byte
//! [obj+0x74] selects code that is instruction for instruction the same in both settings, so it is not a parameter.
//! Plane count checks, the cl flag path and the alpha plane copy belong to the executor.
use fx_common::nanop::{fnma213, NanOps};
use fx_common::x86::{cvttss2si32, flip_sign};

/// 6.0 (cell 0x15e9650), 1.0 (0x15e850c) and 0.5 (0x15e8514), read from the emulated image.
const SIX: f32 = 6.0;
const ONE: f32 = 1.0;
const HALF: f32 = 0.5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsvError {
    /// The three input planes differ in length.
    LengthMismatch,
}

/// `vroundss imm 1` (floor); a NaN input returns the quieted NaN.
#[inline]
fn floor_ss(x: f32) -> f32 {
    if x.is_nan() { f32::from_bits(x.to_bits() | 0x0040_0000) } else { x.floor() }
}

/// The sextant switch shared by every loop. `p` is the chroma product, `vb` the raw bits of v.
/// `konst` selects the operand order of the case 2 fused site (the constant loops use (1-f, p), the plane loops (p, 1-f)).
#[inline]
fn sextant(hh: f32, p: f32, v: f32, vb: u32, konst: bool) -> [f32; 3] {
    let frac = hh.fsub(floor_ss(hh));
    let mut t = SIX.fmul(frac);
    if t >= SIX {
        t = 0.0;
    }
    let idx = cvttss2si32(t);
    let f = t.fsub(idx as f32);
    let vbf = f32::from_bits(vb);
    let q = v.fsub(p);
    if idx < 3 {
        if idx == 0 {
            let w = ONE.fsub(f);
            [vbf, fnma213(w, p, v), q]
        } else if idx == 1 {
            [fnma213(f, p, v), vbf, q]
        } else {
            let w = ONE.fsub(f);
            let b = if konst { fnma213(w, p, v) } else { fnma213(p, w, v) };
            [q, vbf, b]
        }
    } else if idx == 3 {
        [q, fnma213(if konst { f } else { p }, if konst { p } else { f }, v), vbf]
    } else if idx == 4 {
        let w = ONE.fsub(f);
        [fnma213(w, p, v), q, vbf]
    } else {
        [vbf, q, fnma213(if konst { f } else { p }, if konst { p } else { f }, v)]
    }
}

/// One element of the plane form (flag byte [obj+0x44] zero).
pub fn pixel(h: f32, s: f32, v: f32) -> [f32; 3] {
    let vb = v.to_bits();
    if s == 0.0 {
        let x = f32::from_bits(vb);
        return [x, x, x];
    }
    if 0.0 > s {
        let neg_s = flip_sign(s);
        let hh = HALF.fadd(h);
        let p = v.fmul(neg_s);
        sextant(hh, p, v, vb, false)
    } else {
        let p = v.fmul(s);
        sextant(h, p, v, vb, false)
    }
}

/// One element of the constant form (flag byte [obj+0x44] nonzero).
pub fn pixel_const(h: f32, s: f32, v: f32) -> [f32; 3] {
    let vb = v.to_bits();
    if s == 0.0 {
        let x = f32::from_bits(vb);
        return [x, x, x];
    }
    if 0.0 > s {
        let hh = h.fadd(HALF);
        let p = flip_sign(v.fmul(s));
        sextant(hh, p, v, vb, true)
    } else {
        let p = v.fmul(s);
        sextant(h, p, v, vb, true)
    }
}

/// Plane form over `h`, `s`, `v`: three output planes of the same length.
pub fn hsv_to_rgb(h: &[f32], s: &[f32], v: &[f32]) -> Result<[Vec<f32>; 3], HsvError> {
    if h.len() != s.len() || h.len() != v.len() {
        return Err(HsvError::LengthMismatch);
    }
    let n = h.len();
    let mut o = [Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n)];
    for i in 0..n {
        let r = pixel(h[i], s[i], v[i]);
        for k in 0..3 {
            o[k].push(r[k]);
        }
    }
    Ok(o)
}

/// Constant form: `n` elements (a count of zero or below gives empty planes).
pub fn hsv_to_rgb_const(h: f32, s: f32, v: f32, n: i32) -> [Vec<f32>; 3] {
    let n = if n > 0 { n as usize } else { 0 };
    let r = pixel_const(h, s, v);
    [vec![r[0]; n], vec![r[1]; n], vec![r[2]; n]]
}
