//! sat_scale: native port of the AVX2 body.
//!
//! Source of truth: neutral_1.asm (listing offsets; captured VA = offset).
//! The body mixes each pixel (or one constant colour) with its luma:
//!   out = s * colour + (1 - s) * luma(colour)
//! with a per pixel scale from a mask plane or one scalar. Class identity (sat_scale of
//! S_HueSatBright) is a GUESS from names; this is a body-level port.
//!
//! Sections (S = section of the listing):
//!   S0 prologue, plane count test (vcall [vtable]): count < 3 -> fill loop (S6)
//!   S1 flag byte (cl) != 0 -> generic callee (S5), no arithmetic here
//!   S2 const colour + scalar scale (this != 0, this != 0)
//!   S3 const colour + mask plane (this != 0, this == 0)
//!   S4 planes + scalar scale (this == 0, this != 0) and
//!       planes + mask plane (this == 0, this == 0)
//!   S5 call (this, info, outs)
//!   S6 tail: plane count == 4 -> tail call (this, 0, 3, outs[3], info)
//!
//! FMA sites (vex, listing addresses): not listed here.

/// Luma weights (doubles), captured VA in parens: A = 0.3125,
/// B = 0.5, C = 0.1875, ONE .
pub const LUMA_A: f64 = 0.3125;
pub const LUMA_B: f64 = 0.5;
pub const LUMA_C: f64 = 0.1875;
pub const ONE: f32 = 1.0;

/// Fields of the 0xa0 object the body reads.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// this (byte): colour comes from the three floats below instead of the source planes.
    pub const_colour: bool,
    /// this (byte): scale is the scalar below instead of a mask plane.
    pub scalar_scale: bool,
    /// this (r, g, b of the constant colour).
    pub colour: [f32; 3],
    /// this.
    pub scale: f32,
    /// the cl argument (nonzero: generic path through).
    pub flag: bool,
}

/// Source planes (this object, r, g, b) and mask plane (this object).
#[derive(Clone, Copy)]
pub struct Inputs<'a> {
    pub src: Option<[&'a [f32]; 3]>,
    pub mask: Option<&'a [f32]>,
}

/// Calls the body hands to its callees, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    /// (this, info, outs)
    Generic,
    /// (this, 0, index, outs[plane], info)
    Fill { index: i32, plane: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SatError {
    /// a plane is shorter than n
    ShortPlane,
    /// the original dereferences a null source plane object here (outside its contract)
    MissingSource,
    /// the original dereferences a null mask object here (outside its contract)
    MissingMask,
}

use fx_common::x86::{nx_fma, nx_fma64, nx_mul, nx_mul64, nx_sub};

/// luma of the constant colour: (cg*B, fma cb*C, fma cr*A), rounded to f32.
pub fn luma_const(c: [f32; 3]) -> f32 {
    let t = nx_mul64(c[1] as f64, LUMA_B);
    let t = nx_fma64(c[2] as f64, LUMA_C, t);
    nx_fma64(c[0] as f64, LUMA_A, t) as f32
}

/// luma of a pixel: (B*pg, fma pr*A, fma pb*C), rounded to f32.
pub fn luma_px(r: f32, g: f32, b: f32) -> f32 {
    let t = nx_mul64(LUMA_B, g as f64);
    let t = nx_fma64(r as f64, LUMA_A, t);
    nx_fma64(b as f64, LUMA_C, t) as f32
}

fn check_len(a: &[f32], n: usize) -> Result<(), SatError> {
    if a.len() < n { Err(SatError::ShortPlane) } else { Ok(()) }
}

/// S2/S3 shared tail: constant colour with scale s
fn const_px(c: [f32; 3], l: f32, s: f32) -> [f32; 3] {
    if s == 0.0 {
        return [l, l, l];
    }
    let t = nx_fma(-s, l, l);
    [nx_fma(s, c[0], t), nx_fma(s, c[1], t), nx_fma(s, c[2], t)]
}

/// The pixel arithmetic of S2..S4 for `n` pixels (n <= 0 writes nothing).
pub fn pixels(p: &Params, n: i32, inp: &Inputs, outs: &mut [&mut [f32]; 3]) -> Result<(), SatError> {
    if n <= 0 {
        return Ok(());
    }
    let n = n as usize;
    for o in outs.iter() {
        check_len(o, n)?;
    }
    if p.const_colour {
        let l = luma_const(p.colour);
        if p.scalar_scale {
            let v = const_px(p.colour, l, p.scale);
            for i in 0..n {
                outs[0][i] = v[0];
                outs[1][i] = v[1];
                outs[2][i] = v[2];
            }
        } else {
            let m = inp.mask.ok_or(SatError::MissingMask)?;
            check_len(m, n)?;
            for i in 0..n {
                let v = const_px(p.colour, l, m[i]);
                outs[0][i] = v[0];
                outs[1][i] = v[1];
                outs[2][i] = v[2];
            }
        }
        return Ok(());
    }
    let src = inp.src.ok_or(SatError::MissingSource)?;
    for pl in src.iter() {
        check_len(pl, n)?;
    }
    if p.scalar_scale {
        let s = p.scale;
        let w1 = nx_sub(ONE, s); // 
        for i in 0..n {
            let (r, g, b) = (src[0][i], src[1][i], src[2][i]);
            let l = luma_px(r, g, b);
            if s == 0.0 {
                outs[0][i] = l;
                outs[1][i] = l;
                outs[2][i] = l;
            } else {
                let w = nx_mul(w1, l); // 
                outs[0][i] = nx_fma(s, r, w);
                outs[1][i] = nx_fma(s, g, w);
                outs[2][i] = nx_fma(b, s, w);
            }
        }
    } else {
        let m = inp.mask.ok_or(SatError::MissingMask)?;
        check_len(m, n)?;
        for i in 0..n {
            let (r, g, b) = (src[0][i], src[1][i], src[2][i]);
            let s = m[i];
            let l = luma_px(r, g, b);
            if s == 0.0 {
                outs[0][i] = l;
                outs[1][i] = l;
                outs[2][i] = l;
            } else {
                let t = nx_fma(-l, s, l); // 
                outs[0][i] = nx_fma(s, r, t);
                outs[1][i] = nx_fma(s, g, t);
                outs[2][i] = nx_fma(b, s, t);
            }
        }
    }
    Ok(())
}

/// The whole function : `count` is the value of the vcall [vtable] (plane count),
/// `n` the pixel count (info). Returns the calls made to the two callees.
pub fn run(p: &Params, count: i32, n: i32, inp: &Inputs, outs: &mut [&mut [f32]; 3]) -> Result<Vec<Call>, SatError> {
    let mut calls = Vec::new();
    if count < 3 {
        // for i in 0..count (the count is re-read each time): (this, 0, i, outs[i], info)
        let mut i = 0;
        while count > 0 && i < count {
            calls.push(Call::Fill { index: i, plane: i as usize });
            i += 1;
        }
        return Ok(calls);
    }
    if p.flag {
        calls.push(Call::Generic);
    } else {
        pixels(p, n, inp, outs)?;
    }
    if count == 4 {
        calls.push(Call::Fill { index: 3, plane: 3 });
    }
    Ok(calls)
}

/// Assumption list (S3 render shape: explicit).
pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HS1-A1", "ONE = 1.0f CAPTURED (lookup_sat.tsv, 0000803f)"),
        ("HS1-A2", "luma weights A,B,C = 0.3125, 0.5, 0.1875 CAPTURED (lookup_sat.tsv f64 bytes)"),
        ("HS1-A3", "vcall [vtable] (plane count) is a host service: modelled as a fixed integer, hooked ret_const on the emulator side"),
        ("HS1-A4", "callee (plane fill/copy host service) and (flag path) are not ported: arguments recorded, results are stand-ins on both sides"),
        ("HS1-A5", "AVX2 route entered directly; the dispatcher test is a pure branch on the feature word and is not emulated"),
        ("HS1-A6", "null source or mask object when the mode needs it faults the original: outside the contract, reported not compared"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pr(cc: bool, sc: bool, colour: [f32; 3], scale: f32) -> Params {
        Params { const_colour: cc, scalar_scale: sc, colour, scale, flag: false }
    }

    fn go(p: &Params, count: i32, n: i32, inp: &Inputs) -> ([Vec<f32>; 3], Vec<Call>) {
        let mut a = vec![f32::from_bits(0x7fc0ffee); 8];
        let mut b = a.clone();
        let mut c = a.clone();
        let calls = run(p, count, n, inp, &mut [&mut a, &mut b, &mut c]).unwrap();
        ([a, b, c], calls)
    }

    #[test]
    fn luma_values() {
        // 0.3125*1 + 0.5*0 + 0.1875*0 = 0.3125 (hand evaluated)
        assert_eq!(luma_const([1.0, 0.0, 0.0]), 0.3125);
        assert_eq!(luma_const([0.0, 1.0, 0.0]), 0.5);
        assert_eq!(luma_const([0.0, 0.0, 1.0]), 0.1875);
        assert_eq!(luma_px(1.0, 1.0, 1.0), 1.0);
        assert_eq!(luma_px(0.0, 0.0, 0.0), 0.0);
    }

    #[test]
    fn const_colour_scalar() {
        let c = [1.0, 0.0, 0.0];
        // s = 0: all channels luma 0.3125
        let (o, _) = go(&pr(true, true, c, 0.0), 3, 5, &Inputs { src: None, mask: None });
        assert!(o.iter().all(|p| p[..5].iter().all(|&v| v == 0.3125)));
        assert!(o[0][5].is_nan());
        // s = 1: colour itself (t = L - L = 0)
        let (o, _) = go(&pr(true, true, c, 1.0), 3, 4, &Inputs { src: None, mask: None });
        assert_eq!((o[0][0], o[1][0], o[2][0]), (1.0, 0.0, 0.0));
        // s = 2: 2*c + (L - 2L) = 2c - L: r = 2 - 0.3125, g = -0.3125
        let (o, _) = go(&pr(true, true, c, 2.0), 3, 4, &Inputs { src: None, mask: None });
        assert_eq!((o[0][3], o[1][3], o[2][3]), (1.6875, -0.3125, -0.3125));
    }

    #[test]
    fn planes_scalar_and_mask() {
        let (r, g, b) = (vec![1.0f32, 0.5], vec![0.0f32, 0.5], vec![0.0f32, 0.5]);
        let inp = Inputs { src: Some([&r, &g, &b]), mask: None };
        // s = 1 scalar: w = 0, out = s*c + 0
        let (o, _) = go(&pr(false, true, [0.0; 3], 1.0), 3, 2, &inp);
        assert_eq!((o[0][0], o[1][1], o[2][1]), (1.0, 0.5, 0.5));
        // s = 0: luma of pixel 0 = 0.3125, of pixel 1 = 0.5
        let (o, _) = go(&pr(false, true, [0.0; 3], 0.0), 3, 2, &inp);
        assert_eq!((o[0][0], o[1][0], o[0][1]), (0.3125, 0.3125, 0.5));
        // mask path: s = [0.5, 2]; pixel 0: t = 0.3125 - 0.15625 = 0.15625, r = 0.5 + 0.15625
        let m = vec![0.5f32, 2.0];
        let inp = Inputs { src: Some([&r, &g, &b]), mask: Some(&m) };
        let (o, _) = go(&pr(false, false, [0.0; 3], 0.0), 3, 2, &inp);
        assert_eq!((o[0][0], o[1][0], o[2][0]), (0.65625, 0.15625, 0.15625));
        // grey pixel 1 is unchanged for any s: t = L(1-s), s*0.5 + 0.5(1-s) = 0.5
        assert_eq!((o[0][1], o[1][1], o[2][1]), (0.5, 0.5, 0.5));
    }

    #[test]
    fn calls_and_counts() {
        let inp = Inputs { src: None, mask: None };
        let mut p = pr(true, true, [1.0; 3], 1.0);
        assert_eq!(go(&p, 2, 4, &inp).1, vec![Call::Fill { index: 0, plane: 0 }, Call::Fill { index: 1, plane: 1 }]);
        assert!(go(&p, 0, 4, &inp).1.is_empty());
        assert_eq!(go(&p, 4, 4, &inp).1, vec![Call::Fill { index: 3, plane: 3 }]);
        assert!(go(&p, 5, 4, &inp).1.is_empty());
        p.flag = true;
        assert_eq!(go(&p, 4, 4, &inp).1, vec![Call::Generic, Call::Fill { index: 3, plane: 3 }]);
        // flag path leaves the outputs untouched
        assert!(go(&p, 3, 4, &inp).0[0][0].is_nan());
    }

    #[test]
    fn degenerate_and_nan() {
        let inp = Inputs { src: None, mask: None };
        let p = pr(true, true, [1.0; 3], 1.0);
        let (o, _) = go(&p, 3, 0, &inp);
        assert!(o[0][0].is_nan()); // n = 0 writes nothing
        let (o, _) = go(&p, 3, -3, &inp);
        assert!(o[0][0].is_nan());
        // NaN scale takes the general path and gives NaN
        let (o, _) = go(&pr(true, true, [1.0; 3], f32::NAN), 3, 2, &inp);
        assert!(o[0][0].is_nan() && o[2][1].is_nan());
        // errors
        let mut a = vec![0.0f32; 2];
        let mut b = a.clone();
        let mut c = a.clone();
        assert_eq!(run(&pr(true, false, [1.0; 3], 0.0), 3, 2, &inp, &mut [&mut a, &mut b, &mut c]).unwrap_err(), SatError::MissingMask);
        assert_eq!(run(&pr(false, true, [1.0; 3], 0.0), 3, 2, &inp, &mut [&mut a, &mut b, &mut c]).unwrap_err(), SatError::MissingSource);
        assert_eq!(run(&pr(true, true, [1.0; 3], 0.0), 3, 5, &inp, &mut [&mut a, &mut b, &mut c]).unwrap_err(), SatError::ShortPlane);
    }

    #[test]
    fn deterministic_finite() {
        let r: Vec<f32> = (0..8).map(|i| i as f32 * 0.13 - 0.2).collect();
        let g: Vec<f32> = r.iter().map(|v| v * 0.7 + 0.1).collect();
        let b: Vec<f32> = r.iter().map(|v| 1.0 - v).collect();
        let m: Vec<f32> = (0..8).map(|i| i as f32 * 0.4).collect();
        let inp = Inputs { src: Some([&r, &g, &b]), mask: Some(&m) };
        let p = pr(false, false, [0.0; 3], 0.0);
        let x = go(&p, 3, 8, &inp).0;
        let y = go(&p, 3, 8, &inp).0;
        assert_eq!(x, y);
        assert!(x.iter().all(|pl| pl.iter().all(|v| v.is_finite())));
    }
}
