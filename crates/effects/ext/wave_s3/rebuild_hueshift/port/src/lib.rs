//! hue_shift: native port of the AVX2 body (HU1).
//!
//! Source of truth: neutral_1.asm (listing offsets; captured VA = offset).
//! Gate (MEASURED neutral_1.asm): the dispatcher does `and r8,0x9d97ff; cmp r8,0x9d97ff;
//! jne; jmp `bit 0 alone jumps to the SSE twin (not ported). So is the
//! AVX2 body (scalar VEX with FMA, no ymm).
//! Per pixel: RGB -> (V, S, hue) with a sign-flip trick for negative values, h/6 + shift (fused),
//! wrapped with floor, then HSV -> RGB by sector. Class identity (hue_shift of S_HueSatBright) is a
//! GUESS from names; this is a body-level port. rgb_to_hsv and hsv_to_rgb are NOT called: both
//! conversions are inlined (the only calls in the body are vcall [vtable]).
//!
//! Sections (listing):
//!   S0 prologue, plane count (vcall [vtable]); count < 3 -> S6b
//!   S1 flag byte (cl) != 0 -> S5
//!   S2 const colour (this != 0) + scalar shift (this != 0)
//!   S3 const colour + mask plane (this == 0)
//!   S4a planes + scalar shift; S4b planes + mask plane
//!   S5 call (this, info, outs)
//!   S6a plane count == 4 -> tail call (this,0,3,outs[3],info); S6b count < 3 loop
//!   out-of-line hue branches: (S2), (S3), (S4a), (S4b)
//!
//! FMA sites: 
//!     
//!   (28, matches fn_stats_bodies.tsv fma=28). No rcp/rsqrt, no x87.

/// 4.0, 1/6, 0.5,
/// 1.0, 2.0 : CAPTURED.
/// 6.0 : NOT CAPTURED (lookup_hue.tsv U4), stand-in (HU1-A1).
pub const FOUR: f32 = 4.0;
pub const SIXTH: f32 = f32::from_bits(0x3e2aaaad);
pub const HALF: f32 = 0.5;
pub const ONE: f32 = 1.0;
pub const TWO: f32 = 2.0;
pub const SIX: f32 = 6.0;

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// this (byte): colour comes from the three floats below.
    pub const_colour: bool,
    /// this (byte): hue shift is the scalar below instead of a mask plane.
    pub scalar_shift: bool,
    /// this.
    pub colour: [f32; 3],
    /// this: hue shift in turns.
    pub shift: f32,
    /// the cl argument (nonzero: generic path).
    pub flag: bool,
}

#[derive(Clone, Copy)]
pub struct Inputs<'a> {
    pub src: Option<[&'a [f32]; 3]>,
    pub mask: Option<&'a [f32]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    /// (this, info, outs)
    Generic,
    /// (this, 0, index, outs[plane], info)
    Fill { index: i32, plane: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HueError {
    ShortPlane,
    MissingSource,
    MissingMask,
}

/// vminss dst,a,b: a < b ? a : b (second operand on NaN or equal).
pub fn minss(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
/// vmaxss dst,a,b: a > b ? a : b.
pub fn maxss(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// vcvttss2si r32: NaN and out of range give 0x80000000.
pub fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 { i32::MIN } else { x as i32 }
}

pub const T_FLIP: u32 = 1 << 0;
pub const T_HI_ZERO: u32 = 1 << 1;
pub const T_S_ZERO: u32 = 1 << 2;
pub const T_HUE_R: u32 = 1 << 3;
pub const T_HUE_G: u32 = 1 << 4;
pub const T_HUE_B: u32 = 1 << 5;
pub const T_Q_NEG: u32 = 1 << 6;
pub const T_S_NEG: u32 = 1 << 7;
pub const T_T_CLAMP: u32 = 1 << 8;
pub const T_T_NAN: u32 = 1 << 9;
/// sector i (0..=5) sets bit 10 + i; any other integer (NaN gives i32::MIN) sets bit 16.
pub const T_SECTOR0: u32 = 10;
pub const T_SECTOR_OTHER: u32 = 1 << 16;

/// One pixel: (and the three twin loops). `shift` is the scalar or the mask value.
pub fn pixel(r: f32, g: f32, b: f32, shift: f32) -> [f32; 3] {
    pixel_traced(r, g, b, shift).0
}

/// Same, with a branch trace (T_* bits) for coverage reports.
pub fn pixel_traced(mut r: f32, mut g: f32, mut b: f32, shift: f32) -> ([f32; 3], u32) {
    let mut tr = 0u32;
    let mn = minss(minss(r, g), b);
    let mx = maxss(maxss(r, g), b);
    let neg_mn = -mn;
    let (v, hi, lo): (f32, f32, f32);
    if neg_mn > mx {
        //  vcomiss -min,max; jbe skips (NaN skips too)
        tr |= T_FLIP;
        v = mn;
        r = -r;
        g = -g;
        b = -b;
        hi = neg_mn;
        lo = -mx;
    } else {
        v = mx;
        hi = mx;
        lo = mn;
    }
    let delta = hi - lo;
    if hi == 0.0 {
        return ([v, v, v], tr | T_HI_ZERO);
    }
    let mut s = delta / hi;
    if s == 0.0 {
        return ([v, v, v], tr | T_S_ZERO);
    }
    let h = if r == hi {
        tr |= T_HUE_R;
        let q = (g - b) / delta;
        let q6 = q + SIX;
        if 0.0 > q {
            tr |= T_Q_NEG;
            q6
        } else {
            q
        }
    } else if g == hi {
        tr |= T_HUE_G;
        TWO + (b - r) / delta
    } else {
        tr |= T_HUE_B;
        FOUR + (r - g) / delta
    };
    let x = h.mul_add(SIXTH, shift);
    let frac;
    if 0.0 > s {
        tr |= T_S_NEG;
        s = -s;
        let y = HALF + x;
        frac = y - y.floor();
    } else {
        frac = x - x.floor();
    }
    let c = v * s;
    let t6 = SIX * frac;
    //  vcomiss t6,6; jb keeps t6 (unordered sets CF, so NaN is kept), otherwise 0
    let t = if t6 < SIX || t6.is_nan() {
        t6
    } else {
        tr |= T_T_CLAMP;
        0.0
    };
    if t.is_nan() {
        tr |= T_T_NAN;
    }
    let i = cvtt(t);
    let f = t - (i as f32);
    let omf = ONE - f;
    let p = v - c;
    let tq = (-omf).mul_add(c, v); // V - (1-f) C
    let qq = (-f).mul_add(c, v); // V - f C
    tr |= if (0..=5).contains(&i) { 1 << (T_SECTOR0 + i as u32) } else { T_SECTOR_OTHER };
    let out = if i < 3 {
        if i == 0 {
            [v, tq, p]
        } else if i == 1 {
            [qq, v, p]
        } else {
            [p, v, tq]
        }
    } else if i == 3 {
        [p, qq, v]
    } else if i == 4 {
        [tq, p, v]
    } else {
        [v, p, qq]
    };
    (out, tr)
}

fn check_len(a: &[f32], n: usize) -> Result<(), HueError> {
    if a.len() < n { Err(HueError::ShortPlane) } else { Ok(()) }
}

/// Pixel loops S2..S4b for `n` pixels (n <= 0 writes nothing). Calls `visit` with each pixel's trace.
pub fn pixels_traced(
    p: &Params,
    n: i32,
    inp: &Inputs,
    outs: &mut [&mut [f32]; 3],
    visit: &mut dyn FnMut(u32),
) -> Result<(), HueError> {
    if n <= 0 {
        return Ok(());
    }
    let n = n as usize;
    for o in outs.iter() {
        check_len(o, n)?;
    }
    let src = if p.const_colour {
        None
    } else {
        let s = inp.src.ok_or(HueError::MissingSource)?;
        for pl in s.iter() {
            check_len(pl, n)?;
        }
        Some(s)
    };
    let mask = if p.scalar_shift {
        None
    } else {
        let m = inp.mask.ok_or(HueError::MissingMask)?;
        check_len(m, n)?;
        Some(m)
    };
    for i in 0..n {
        let (r, g, b) = match src {
            None => (p.colour[0], p.colour[1], p.colour[2]),
            Some(s) => (s[0][i], s[1][i], s[2][i]),
        };
        let sh = match mask {
            None => p.shift,
            Some(m) => m[i],
        };
        let (o, tr) = pixel_traced(r, g, b, sh);
        visit(tr);
        outs[0][i] = o[0];
        outs[1][i] = o[1];
        outs[2][i] = o[2];
    }
    Ok(())
}

pub fn pixels(p: &Params, n: i32, inp: &Inputs, outs: &mut [&mut [f32]; 3]) -> Result<(), HueError> {
    pixels_traced(p, n, inp, outs, &mut |_| {})
}

/// The whole function (`count` = vcall [vtable], `n` = info), with a trace visitor.
pub fn run_traced(
    p: &Params,
    count: i32,
    n: i32,
    inp: &Inputs,
    outs: &mut [&mut [f32]; 3],
    visit: &mut dyn FnMut(u32),
) -> Result<Vec<Call>, HueError> {
    let mut calls = Vec::new();
    if count < 3 {
        // for i in 0..count (count re-read each time): (this, 0, i, outs[i], info)
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
        pixels_traced(p, n, inp, outs, visit)?;
    }
    if count == 4 {
        calls.push(Call::Fill { index: 3, plane: 3 });
    }
    Ok(calls)
}

pub fn run(p: &Params, count: i32, n: i32, inp: &Inputs, outs: &mut [&mut [f32]; 3]) -> Result<Vec<Call>, HueError> {
    run_traced(p, count, n, inp, outs, &mut |_| {})
}

pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HU1-A1", "6.0 NOT CAPTURED (lookup_hue.tsv U4, also U16 around it): stand-in 6.0f on both sides (the cell is shared with hsv_to_rgb, no re-extraction); the emulator side patches the 4 bytes 0000c040"),
        ("HU1-A2", "sign mask 16 bytes CAPTURED (00000080 then 12 zero bytes), read through the data image"),
        ("HU1-A3", "4.0, 1/6 = 0x3e2aaaad, 0.5, 1.0, 2.0 CAPTURED (lookup_hue.tsv)"),
        ("HU1-A4", "vcall [vtable] (plane count) is a host service: fixed integer, ret_const on the emulator side"),
        ("HU1-A5", "callees not ported: arguments recorded, results are stand-ins on both sides"),
        ("HU1-A6", "AVX2 route entered directly; dispatcher not emulated; SSE twin not ported"),
        ("HU1-A7", "null source or mask object when the mode needs it faults the original: outside the contract, reported not compared"),
        ("HU1-A8", "class identity (hue_shift of S_HueSatBright, a hue rotation by `shift` turns) is a GUESS from names; the port is body-level"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pr(cc: bool, ss: bool, colour: [f32; 3], shift: f32) -> Params {
        Params { const_colour: cc, scalar_shift: ss, colour, shift, flag: false }
    }

    #[test]
    fn hand_hues() {
        // red, shift 0: h = (g-b)/delta = 0 -> sector 0, f = 0, V = 1, C = 1: [1, 1-(1-0)*1, 1-1] = [1,0,0]
        assert_eq!(pixel(1.0, 0.0, 0.0, 0.0), [1.0, 0.0, 0.0]);
        // red, shift 0.25: x = 0.25, t = 1.5, i = 1, f = 0.5: [V - fC, V, V - C] = [0.5, 1, 0]
        assert_eq!(pixel(1.0, 0.0, 0.0, 0.25), [0.5, 1.0, 0.0]);
        // red, shift 0.5: x = 0.5, t = 3, i = 3, f = 0: [V-C, V - 0, V] = [0, 1, 1]
        assert_eq!(pixel(1.0, 0.0, 0.0, 0.5), [0.0, 1.0, 1.0]);
        // red, shift 0.75: t = 4.5, i = 4, f = 0.5: [V-(1-f)C, V-C, V] = [0.5, 0, 1]
        assert_eq!(pixel(1.0, 0.0, 0.0, 0.75), [0.5, 0.0, 1.0]);
        // red, shift 0.875: t = 5.25, i = 5, f = 0.25: [V, V-C, V-fC] = [1, 0, 0.75]
        assert_eq!(pixel(1.0, 0.0, 0.0, 0.875), [1.0, 0.0, 0.75]);
        // green: h = 2 + (b-r)/delta = 2, x = 1/3 (rounded), shift 0 stays green within 1e-6
        let o = pixel(0.0, 1.0, 0.0, 0.0);
        assert!((o[1] - 1.0).abs() < 1e-6 && o[0].abs() < 1e-6 && o[2].abs() < 1e-6, "{:?}", o);
        // blue shifted by one full turn returns blue
        let o = pixel(0.0, 0.0, 1.0, 1.0);
        assert!((o[2] - 1.0).abs() < 1e-6 && o[0].abs() < 1e-6 && o[1].abs() < 1e-6, "{:?}", o);
    }

    #[test]
    fn hue_wrap_negative_q() {
        // magenta (tie r == b, r branch first): q = (0-1)/1 = -1, q + 6 = 5, x = 5/6, t = 5, i = 5, f = 0:
        // [V, V-C, V - 0*C] = [1, 0, 1]
        let (o, tr) = pixel_traced(1.0, 0.0, 1.0, 0.0);
        assert!(tr & T_HUE_R != 0 && tr & T_Q_NEG != 0);
        assert!((o[0] - 1.0).abs() < 1e-5 && o[1].abs() < 1e-5 && (o[2] - 1.0).abs() < 1e-5, "{:?}", o);
        let (_, tr) = pixel_traced(1.0, 0.0, 0.5, 0.0);
        assert!(tr & T_Q_NEG != 0 && tr & T_HUE_R != 0);
        // negative shift wraps: red shift -0.25 = 0.75 turn
        assert_eq!(pixel(1.0, 0.0, 0.0, -0.25), pixel(1.0, 0.0, 0.0, 0.75));
    }

    #[test]
    fn grey_and_zero() {
        assert_eq!(pixel(0.5, 0.5, 0.5, 0.3), [0.5, 0.5, 0.5]);
        assert_eq!(pixel(0.0, 0.0, 0.0, 0.3), [0.0, 0.0, 0.0]);
        // negative grey flips: V = min = -0.5, hi = 0.5, lo = 0.5: S = 0
        let (o, tr) = pixel_traced(-0.5, -0.5, -0.5, 0.3);
        assert_eq!(o, [-0.5, -0.5, -0.5]);
        assert!(tr & T_FLIP != 0 && tr & T_S_ZERO != 0);
        // negative colour: (-1, -0.5, -0.5) flips: V = -1, hi = 1, lo = 0.5, r' = 1 == hi -> q = 0
        let (o, tr) = pixel_traced(-1.0, -0.5, -0.5, 0.0);
        assert!(tr & T_FLIP != 0 && tr & T_HUE_R != 0);
        assert_eq!(o, [-1.0, -0.5, -0.5]);
    }

    #[test]
    fn identity_shift_keeps_primary_and_secondary() {
        for c in [[1.0f32, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.5, 0.25, 0.25]] {
            let o = pixel(c[0], c[1], c[2], 0.0);
            for k in 0..3 {
                assert!((o[k] - c[k]).abs() < 1e-5, "{:?} {:?}", c, o);
            }
        }
    }

    #[test]
    fn nan_inf_and_finite() {
        let o = pixel(f32::NAN, 0.2, 0.3, 0.1);
        let _ = o; // no panic
        let (_, tr) = pixel_traced(0.5, 0.25, 0.1, f32::NAN);
        assert!(tr & T_T_NAN != 0 && tr & T_SECTOR_OTHER != 0);
        for i in 0..64 {
            let r = (i as f32) * 0.037 - 0.3;
            let o = pixel(r, 1.0 - r, r * 0.5, 0.21);
            assert!(o.iter().all(|v| v.is_finite()));
            assert_eq!(o, pixel(r, 1.0 - r, r * 0.5, 0.21));
        }
        // t6 rounds up to 6.0: red with shift -1e-8 gives frac = 1.0 (f32), t = 0 (clamped)
        let (o, tr) = pixel_traced(1.0, 0.0, 0.0, -1e-8);
        assert!(tr & T_T_CLAMP != 0);
        assert_eq!(o, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn modes_and_calls() {
        let r = vec![1.0f32, 0.0];
        let g = vec![0.0f32, 1.0];
        let b = vec![0.0f32, 0.0];
        let m = vec![0.0f32, 1.0 / 3.0];
        let inp = Inputs { src: Some([&r, &g, &b]), mask: Some(&m) };
        let (mut a, mut bb, mut c) = (vec![9.0f32; 4], vec![9.0f32; 4], vec![9.0f32; 4]);
        let calls = run(&pr(false, false, [0.0; 3], 0.0), 4, 2, &inp, &mut [&mut a, &mut bb, &mut c]).unwrap();
        assert_eq!(calls, vec![Call::Fill { index: 3, plane: 3 }]);
        assert_eq!((a[0], bb[0], c[0]), (1.0, 0.0, 0.0));
        assert!(c[1] > 0.99 && a[1] < 0.01 && bb[1] < 0.01); // green pixel rotated by 1/3 turn is blue
        assert_eq!(a[2], 9.0); // beyond n untouched
        let none = Inputs { src: None, mask: None };
        assert_eq!(
            run(&pr(true, true, [1.0, 0.0, 0.0], 0.0), 2, 2, &none, &mut [&mut a, &mut bb, &mut c]).unwrap(),
            vec![Call::Fill { index: 0, plane: 0 }, Call::Fill { index: 1, plane: 1 }]
        );
        let mut p = pr(true, true, [1.0, 0.0, 0.0], 0.0);
        p.flag = true;
        assert_eq!(run(&p, 3, 2, &none, &mut [&mut a, &mut bb, &mut c]).unwrap(), vec![Call::Generic]);
        assert!(run(&pr(true, true, [1.0; 3], 0.0), 3, 0, &none, &mut [&mut a, &mut bb, &mut c]).unwrap().is_empty());
        assert!(run(&pr(true, true, [1.0; 3], 0.0), 0, 2, &none, &mut [&mut a, &mut bb, &mut c]).unwrap().is_empty());
        assert_eq!(run(&pr(true, false, [1.0; 3], 0.0), 3, 2, &none, &mut [&mut a, &mut bb, &mut c]).unwrap_err(), HueError::MissingMask);
        assert_eq!(run(&pr(false, true, [1.0; 3], 0.0), 3, 2, &none, &mut [&mut a, &mut bb, &mut c]).unwrap_err(), HueError::MissingSource);
        assert_eq!(run(&pr(true, true, [1.0; 3], 0.0), 3, 9, &none, &mut [&mut a, &mut bb, &mut c]).unwrap_err(), HueError::ShortPlane);
    }

    #[test]
    fn const_colour_fills_every_pixel_the_same() {
        let none = Inputs { src: None, mask: None };
        let (mut a, mut b, mut c) = (vec![9.0f32; 5], vec![9.0f32; 5], vec![9.0f32; 5]);
        run(&pr(true, true, [1.0, 0.0, 0.0], 0.25), 3, 5, &none, &mut [&mut a, &mut b, &mut c]).unwrap();
        for i in 0..5 {
            assert_eq!((a[i], b[i], c[i]), (0.5, 1.0, 0.0));
        }
    }
}
