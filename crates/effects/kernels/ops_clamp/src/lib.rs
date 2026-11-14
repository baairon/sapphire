//! clamp: native port of the three-operand builtin `clamp` (vtable, first slot, getter/factory, slot 49,
//! slot 53 dispatcher, AVX2 body, AVX tier, SSE tier, slot 54 GPU twin, slot 60 combo builder).
//! Listing offsets; VA = offset. The node returns the MEDIAN of its three operands (value between the other two, whichever order they come in),
//! with a different compare ladder per operand shape (which operands are constants), so the port keeps the 7 shapes the body has:
//! flags (A,B,C constant) = 101 100 11x 011 010 001 000 (11x: A and B constant, C plane read even if flagged constant).
//! std only, no unsafe, explicit parameters (no Default), S3-style `render`. Assumption prefix IN2C-.
pub use ops_arith::{OpError, Operand};

/// Explicit parameters. `nout` = result of virtual slot 31 (count getter, hooked in R-PARITY); called again after every channel.
/// `n` = [info]. `allones` = the all-ones vector constant (8 lanes, MEASURED-DYLIB 0xffffffff), the xor constant that inverts
/// the vector compare masks; no default. `dst_phase_bytes[c]` = (address of output plane c) & 0x1f (ALIGN-DEPENDENT: only n >= 0x4b0 / 0x45d peels).
pub struct Params {
    pub nout: i32,
    pub n: i32,
    pub allones: u32,
    pub dst_phase_bytes: [u32; 4],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    ClampA,
    ClampB,
    ClampC,
    ConstClamped,
    Swap101,
    Swap100,
    Swap11,
    Swap011,
    Swap010,
    Swap001,
    Swap000,
    ThrM1,
    Thr011Plain,
    NoPeel,
    PeelPlus1,
    EndAll,
    NoutCap3,
    LanesScalar,
    KIgnored,
    Width16,
}

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "clamp_a" => ClampA,
            "clamp_b" => ClampB,
            "clamp_c" => ClampC,
            "const_clamped" => ConstClamped,
            "swap101" => Swap101,
            "swap100" => Swap100,
            "swap11" => Swap11,
            "swap011" => Swap011,
            "swap010" => Swap010,
            "swap001" => Swap001,
            "swap000" => Swap000,
            "thr_m1" => ThrM1,
            "thr011_plain" => Thr011Plain,
            "no_peel" => NoPeel,
            "peel_plus1" => PeelPlus1,
            "end_all" => EndAll,
            "nout_cap3" => NoutCap3,
            "lanes_scalar" => LanesScalar,
            "k_ignored" => KIgnored,
            "width16" => Width16,
            _ => return Option::None,
        })
    }
}

// x86 scalar/vector primitives (SDM semantics): vminss/vminps(a,b) = a < b ? a : b ; vcomiss+ja = ordered greater ; vcmpltps = all ones when a < b ;
// vblendvps(a,b,m) = sign bit of m ? b : a ; vandps / vxorps are bit operations.
fn minss(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
fn lt(a: f32, b: f32) -> u32 {
    if a < b {
        !0
    } else {
        0
    }
}
fn andm(a: f32, m: u32) -> f32 {
    f32::from_bits(a.to_bits() & m)
}
fn blend(a: f32, b: f32, m: u32) -> f32 {
    if m >> 31 != 0 {
        b
    } else {
        a
    }
}

/// The seven shapes. Each has a scalar ladder (head and tail of the loop) and a lane function (the 8 wide vector core).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape {
    S101,
    S100,
    S11,
    S011,
    S010,
    S001,
    S000,
}

pub fn shape_of(fa: bool, fb: bool, fc: bool) -> Shape {
    match (fa, fb, fc) {
        (true, false, true) => Shape::S101,
        (true, false, false) => Shape::S100,
        (true, true, _) => Shape::S11,
        (false, true, true) => Shape::S011,
        (false, true, false) => Shape::S010,
        (false, false, true) => Shape::S001,
        (false, false, false) => Shape::S000,
    }
}

/// Values of one element: p0, p1, p2 = A, B, C plane element (unused ones are 0), c0, c1, c2 = A, B, C constants of the channel.
#[derive(Clone, Copy)]
struct El {
    p0: f32,
    src_ch1: f32,
    p2: f32,
    clamp_ch0: f32,
    c1: f32,
    c2: f32,
}

fn scalar(s: Shape, e: El, m: Mutant) -> f32 {
    match s {
        // x = B plane, c2 = C const (xmm8), c0 = A const (xmm0)
        Shape::S101 => {
            let (x, c2, clamp_ch0) = if m == Mutant::Swap101 { (e.src_ch1, e.clamp_ch0, e.c2) } else { (e.src_ch1, e.c2, e.clamp_ch0) };
            if c2 > x {
                if x > clamp_ch0 {
                    x
                } else {
                    minss(c2, clamp_ch0)
                }
            } else if c2 > clamp_ch0 {
                c2
            } else {
                minss(x, clamp_ch0)
            }
        }
        // x = B plane, y = C plane, c0
        Shape::S100 => {
            let (x, y) = if m == Mutant::Swap100 { (e.p2, e.src_ch1) } else { (e.src_ch1, e.p2) };
            if y > x {
                if x > e.clamp_ch0 {
                    x
                } else {
                    minss(y, e.clamp_ch0)
                }
            } else if y > e.clamp_ch0 {
                y
            } else {
                minss(x, e.clamp_ch0)
            }
        }
        // x = C plane, c1 (xmm9), c0 (xmm0)
        Shape::S11 => {
            let (x, c1, clamp_ch0) = (e.p2, if m == Mutant::Swap11 { e.clamp_ch0 } else { e.c1 }, if m == Mutant::Swap11 { e.c1 } else { e.clamp_ch0 });
            if x > c1 {
                if c1 > clamp_ch0 {
                    c1
                } else {
                    minss(x, clamp_ch0)
                }
            } else if x > clamp_ch0 {
                x
            } else {
                minss(c1, clamp_ch0)
            }
        }
        //  (c2 > c1) and (else): x = A plane, c1, c2
        Shape::S011 => {
            let x = e.p0;
            if e.c2 > e.c1 || m == Mutant::Swap011 {
                if e.c1 > x {
                    e.c1
                } else {
                    minss(e.c2, x)
                }
            } else if e.c2 > x {
                e.c2
            } else {
                minss(e.c1, x)
            }
        }
        // y = C plane (xmm1), x = A plane (xmm0), c1
        Shape::S010 => {
            let (x, y) = if m == Mutant::Swap010 { (e.p2, e.p0) } else { (e.p0, e.p2) };
            if y > e.c1 {
                if e.c1 > x {
                    e.c1
                } else {
                    minss(y, x)
                }
            } else if y > x {
                y
            } else {
                minss(e.c1, x)
            }
        }
        // b = B plane (xmm1), a = A plane (xmm0), c2
        Shape::S001 => {
            let (a, b) = if m == Mutant::Swap001 { (e.src_ch1, e.p0) } else { (e.p0, e.src_ch1) };
            if e.c2 > b {
                if b > a {
                    b
                } else {
                    minss(e.c2, a)
                }
            } else if e.c2 > a {
                e.c2
            } else {
                minss(b, a)
            }
        }
        // b = B plane (xmm2), c = C plane (xmm0), a = A plane (xmm1)
        Shape::S000 => {
            let (b, c, a) = if m == Mutant::Swap000 { (e.p2, e.src_ch1, e.p0) } else { (e.src_ch1, e.p2, e.p0) };
            if c > b {
                if b > a {
                    b
                } else {
                    minss(c, a)
                }
            } else if c > a {
                c
            } else {
                minss(b, a)
            }
        }
    }
}

/// One lane of the vector core, transcribed instruction by instruction (k = the xor constant `allones`).
fn lane(s: Shape, e: El, k: u32) -> f32 {
    match s {
        // 
        Shape::S101 => {
            let (x, clamp_ch0, c2) = (e.src_ch1, e.clamp_ch0, e.c2);
            let less_2 = lt(clamp_ch0, c2);
            let v3 = minss(c2, clamp_ch0);
            let bits_5 = less_2 ^ k;
            let less_14 = lt(x, c2);
            let less_10 = lt(clamp_ch0, x);
            let bits_9 = less_14 & less_10;
            let bits_11 = k ^ less_10;
            let masked_12 = andm(x, bits_9);
            let bits_13 = less_14 & bits_11;
            let bits_10_b = k ^ less_14;
            let blend_15 = blend(masked_12, v3, bits_13);
            let min_1 = minss(x, clamp_ch0);
            let bits_9_b = bits_10_b & less_2;
            let blend_11 = blend(blend_15, c2, bits_9_b);
            let bits_12 = bits_10_b & bits_5;
            blend(blend_11, min_1, bits_12)
        }
        // 
        Shape::S100 => {
            let (x, y, clamp_ch0) = (e.src_ch1, e.p2, e.clamp_ch0);
            let bits_5 = lt(clamp_ch0, x);
            let less_10 = lt(x, y);
            let min_8 = minss(y, clamp_ch0);
            let less_14 = lt(clamp_ch0, y);
            let bits_4 = less_10 & bits_5;
            let bits_6 = k ^ bits_5;
            let masked_7 = andm(x, bits_4);
            let bits_9 = less_10 & bits_6;
            let bits_15 = k ^ less_10;
            let blend_11 = blend(masked_7, min_8, bits_9);
            let min_1 = minss(x, clamp_ch0);
            let bits_13 = bits_15 & less_14;
            let bits_14_b = k ^ less_14;
            let v4 = blend(blend_11, y, bits_13);
            let bits_5_b = bits_15 & bits_14_b;
            blend(v4, min_1, bits_5_b)
        }
        // 
        Shape::S11 => {
            let (x, c1, clamp_ch0) = (e.p2, e.c1, e.clamp_ch0);
            let less_3 = lt(clamp_ch0, c1);
            let min_6 = minss(c1, clamp_ch0);
            let less_2 = less_3 ^ k;
            let bits_13 = lt(c1, x);
            let blend_11 = minss(x, clamp_ch0);
            let bits_8 = bits_13 & less_3;
            let masked_10 = andm(c1, bits_8);
            let less_8_b = lt(clamp_ch0, x);
            let bits_12 = bits_13 & less_2;
            let blend_14 = blend(masked_10, blend_11, bits_12);
            let less_10 = k ^ bits_13;
            let bits_15 = less_10 & less_8_b;
            let blend_11_b = blend(blend_14, x, bits_15);
            let bits_1 = k ^ less_8_b;
            let bits_12_b = less_10 & bits_1;
            blend(blend_11_b, min_6, bits_12_b)
        }
        //  (c2 > c1) and (else)
        Shape::S011 => {
            let x = e.p0;
            if e.c2 > e.c1 {
                let bits_4 = lt(x, e.c1);
                let v3 = minss(e.c2, x);
                blend(v3, e.c1, bits_4)
            } else {
                let bits_4 = lt(x, e.c2);
                let v3 = minss(e.c1, x);
                blend(v3, e.c2, bits_4)
            }
        }
        // 
        Shape::S010 => {
            let (x, y, c1) = (e.p0, e.p2, e.c1);
            let less_10 = lt(c1, y);
            let bits_4 = lt(x, c1);
            let masked_7 = minss(y, x);
            let less_14 = lt(x, y);
            let min_0 = minss(c1, x);
            let less_3 = less_10 & bits_4;
            let bits_5 = k ^ bits_4;
            let min_6 = andm(c1, less_3);
            let bits_8 = less_10 & bits_5;
            let bits_15 = k ^ less_10;
            let blend_11 = blend(min_6, masked_7, bits_8);
            let bits_13 = bits_15 & less_14;
            let bits_14_b = k ^ less_14;
            let v3 = blend(blend_11, y, bits_13);
            let bits_4_b = bits_15 & bits_14_b;
            blend(v3, min_0, bits_4_b)
        }
        // 
        Shape::S001 => {
            let (a, b, c2) = (e.p0, e.src_ch1, e.c2);
            let bits_11 = lt(b, c2);
            let bits_5 = lt(a, b);
            let min_9 = minss(c2, a);
            let less_14 = lt(a, c2);
            let min_0 = minss(b, a);
            let bits_4 = bits_11 & bits_5;
            let bits_6 = k ^ bits_5;
            let masked_7 = andm(b, bits_4);
            let less_10 = bits_11 & bits_6;
            let bits_15 = k ^ bits_11;
            let masked_12 = blend(masked_7, min_9, less_10);
            let bits_13 = bits_15 & less_14;
            let bits_14_b = k ^ less_14;
            let v4 = blend(masked_12, c2, bits_13);
            let bits_1 = bits_15 & bits_14_b;
            blend(v4, min_0, bits_1)
        }
        // 
        Shape::S000 => {
            let (b, c, a) = (e.src_ch1, e.p2, e.p0);
            let less_7 = lt(b, c);
            let less_2 = lt(a, b);
            let min_5 = minss(c, a);
            let bits_13 = lt(a, c);
            let bits_1 = less_7 & less_2;
            let less_3 = k ^ less_2;
            let v4 = andm(b, bits_1);
            let bits_6 = less_7 & less_3;
            let less_14 = k ^ less_7;
            let min_8 = blend(v4, min_5, bits_6);
            let v2 = minss(b, a);
            let less_10 = less_14 & bits_13;
            let bits_15 = k ^ bits_13;
            let min_1 = blend(min_8, c, less_10);
            let bits_3_b = less_14 & bits_15;
            blend(min_1, v2, bits_3_b)
        }
    }
}

/// Vector split of one channel (identical in all seven shapes except the middle threshold): n < 8 all scalar; n < thr no peel, vector end n & ~7;
/// else peel from the destination phase (0: none; not a multiple of 4: all scalar; else (32 - ph) >> 2), all scalar when n < peel + 8, vector end n - ((n - peel) & 7).
fn split(n: usize, phase: u32, thr: usize, m: Mutant) -> (usize, usize) {
    let w = if m == Mutant::Width16 { 16 } else { 8 };
    let thr = if m == Mutant::ThrM1 { thr - 1 } else { thr };
    if n < w {
        return (0, 0);
    }
    if n < thr {
        return (0, n & !(w - 1));
    }
    let ph = (phase & 0x1f) as usize;
    if ph != 0 && (ph & 3) != 0 {
        return (0, 0);
    }
    let mut peel = if ph == 0 { 0 } else { (32 - ph) >> 2 };
    if m == Mutant::NoPeel {
        peel = 0;
    }
    if m == Mutant::PeelPlus1 {
        peel += 1;
    }
    if n < peel + w {
        return (0, 0);
    }
    if m == Mutant::EndAll {
        return (peel, n & !(w - 1));
    }
    (peel, n - ((n - peel) & (w - 1)))
}

fn plane<'a>(o: &Operand<'a>, idx: i32, n: usize) -> Result<&'a [f32], OpError> {
    if idx < 0 {
        return Err(OpError::ConstIndex);
    }
    let p = o.planes.get(idx as usize).copied().ok_or(OpError::MissingPlane)?;
    if p.len() < n {
        return Err(OpError::ShortPlane);
    }
    Ok(p)
}

pub fn render(p: &Params, a: &Operand, b: &Operand, c: &Operand, out: &mut [&mut [f32]]) -> Result<(), OpError> {
    run(p, a, b, c, out, Mutant::None)
}

pub fn render_mutant(p: &Params, a: &Operand, b: &Operand, c: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    run(p, a, b, c, out, m)
}

/// The whole slot-53 AVX2 body: for ch in 0..nout (the count getter is called again after each channel), n <= 0 skips the channel.
pub fn run(p: &Params, a: &Operand, b: &Operand, c: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    let mut nch = p.nout.max(0);
    if m == Mutant::NoutCap3 {
        nch = nch.min(3);
    }
    let k = if m == Mutant::KIgnored { !0u32 } else { p.allones };
    let s = shape_of(a.is_const, b.is_const, c.is_const);
    let nn = p.n;
    for chan in 0..nch {
        // cmp edx,esi ; cmovl esi,edx : idx = min(ch, count - 1), signed
        let mut ia = chan.min(a.count - 1);
        let mut ib = chan.min(b.count - 1);
        let mut ic = chan.min(c.count - 1);
        if m == Mutant::ClampA {
            ia = chan.min(a.count);
        }
        if m == Mutant::ClampB {
            ib = chan.min(b.count);
        }
        if m == Mutant::ClampC {
            ic = chan.min(c.count);
        }
        if nn <= 0 {
            continue;
        }
        let n = nn as usize;
        let d = out.get_mut(chan as usize).ok_or(OpError::MissingOut)?;
        if d.len() < n {
            return Err(OpError::MissingOut);
        }
        // constants are indexed by the channel itself (this+4ch, 0x7c+4ch, 0xac+4ch), not by the clamped index
        let centre_x = |o: &Operand, i: i32| -> Result<f32, OpError> {
            let j = if m == Mutant::ConstClamped { i } else { chan };
            if (0..=3).contains(&j) {
                Ok(o.consts[j as usize])
            } else {
                Err(OpError::ConstIndex)
            }
        };
        let (mut clamp_ch0, mut c1, mut c2) = (0.0f32, 0.0f32, 0.0f32);
        let (mut p0, mut src_ch1, mut p2): (&[f32], &[f32], &[f32]) = (&[], &[], &[]);
        match s {
            Shape::S101 => {
                clamp_ch0 = centre_x(a, ia)?;
                c2 = centre_x(c, ic)?;
                src_ch1 = plane(b, ib, n)?;
            }
            Shape::S100 => {
                clamp_ch0 = centre_x(a, ia)?;
                src_ch1 = plane(b, ib, n)?;
                p2 = plane(c, ic, n)?;
            }
            Shape::S11 => {
                clamp_ch0 = centre_x(a, ia)?;
                c1 = centre_x(b, ib)?;
                p2 = plane(c, ic, n)?;
            }
            Shape::S011 => {
                c1 = centre_x(b, ib)?;
                c2 = centre_x(c, ic)?;
                p0 = plane(a, ia, n)?;
            }
            Shape::S010 => {
                c1 = centre_x(b, ib)?;
                p0 = plane(a, ia, n)?;
                p2 = plane(c, ic, n)?;
            }
            Shape::S001 => {
                c2 = centre_x(c, ic)?;
                p0 = plane(a, ia, n)?;
                src_ch1 = plane(b, ib, n)?;
            }
            Shape::S000 => {
                p0 = plane(a, ia, n)?;
                src_ch1 = plane(b, ib, n)?;
                p2 = plane(c, ic, n)?;
            }
        }
        let el = |i: usize| El {
            p0: p0.get(i).copied().unwrap_or(0.0),
            src_ch1: src_ch1.get(i).copied().unwrap_or(0.0),
            p2: p2.get(i).copied().unwrap_or(0.0),
            clamp_ch0,
            c1,
            c2,
        };
        let thr = if s == Shape::S011 && m != Mutant::Thr011Plain { 0x45d } else { 0x4b0 };
        let (peel, e) = split(n, p.dst_phase_bytes[(chan as usize) & 3], thr, m);
        if e == 0 {
            for i in 0..n {
                d[i] = scalar(s, el(i), m);
            }
            continue;
        }
        for i in 0..peel {
            d[i] = scalar(s, el(i), m);
        }
        for i in peel..e {
            d[i] = if m == Mutant::LanesScalar { scalar(s, el(i), m) } else { lane(s, el(i), k) };
        }
        for i in e..n {
            d[i] = scalar(s, el(i), m);
        }
    }
    Ok(())
}

/// Every stand-in of this port with its tier.
pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("IN2C-A1", "MEASURED-DYLIB", "vector constant (32 B, uncovered in the image) = 0xffffffff x8, the xor constant of every lane mask inversion (vxorps K,mask); explicit parameter allones; T2 runs all-ones and an alternate value, so a wrong value shows"),
        ("IN2C-A2", "ROUTE", "AVX2 route only: feature word selects; AVX tier and SSE tier are not ported"), // origin: 0x9d97ff
        ("IN2C-A3", "HOST-FP-ASSUMED", "MXCSR round to nearest, no FTZ, no DAZ; compares and min/max only (no arithmetic), the emulator follows the SDM, not silicon"),
        ("IN2C-A4", "ALIGN-DEPENDENT", "dst_phase_bytes[c] = (address of output plane c) & 0x1f is an explicit input; only n >= 0x4b0 (shape 011: 0x45d) peels, below it the vector core starts at element 0"),
        ("IN2C-A5", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 is an explicit parameter (hooked ret_const in R-PARITY); it is called again after every channel and treated as constant; combo choice and the 9 entry combo table of the own builder (words 1,2,2,3,3,3,4,4 and terminator) are not rebuilt (INFERRED: operand channel count triples)"),
        ("IN2C-A6", "INFERRED", "this operand channel counts, constant flags, constants one float per channel indexed by the channel c (not clamped), records at with plane c at the record's per-channel pointer: measured from the body, host fill not run"),
        ("IN2C-A7", "CONTRACT", "constant index > 3 (nout > 4) and missing or short planes return an error; shape 11x reads C's plane even when C is flagged constant; operand roles (which of A,B,C is the median's value) are per shape and exactly as the body"),
        ("IN2C-A8", "INFERRED", "the node is the median of its three operands for finite inputs (MEASURED per shape); NaN and tie behaviour are exactly the compare ladders ported here and are not a claim about the effect language"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    const S: u32 = 0x7fc0ffee;
    fn outs(k: usize, n: usize) -> Vec<Vec<f32>> {
        (0..k).map(|_| vec![f32::from_bits(S); n]).collect()
    }
    fn op<'a>(count: i32, planes: &'a [&'a [f32]]) -> Operand<'a> {
        Operand { count, is_const: false, consts: [0.0; 4], planes }
    }
    fn cop(count: i32, c: [f32; 4]) -> Operand<'static> {
        Operand { count, is_const: true, consts: c, planes: &[] }
    }
    fn p(nout: i32, n: i32) -> Params {
        Params { nout, n, allones: !0, dst_phase_bytes: [0; 4] }
    }
    fn go(pp: &Params, a: &Operand, b: &Operand, c: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), OpError> {
        let mut mo: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        render(pp, a, b, c, &mut mo)
    }
    fn median(a: f32, b: f32, c: f32) -> f32 {
        let mut v = [a, b, c];
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        v[1]
    }

    #[test]
    fn median_all_shapes_finite() {
        // all 8 flag combos over a grid of orderings, n = 37 (vector core 32 + tail), the result is the median in every shape
        let n = 37usize;
        let vals = [-2.0f32, -0.5, 0.0, 0.5, 1.0, 3.0];
        let mk = |sel: usize| (0..n).map(|i| vals[(i * 7 + sel * 3 + i / 6) % vals.len()]).collect::<Vec<f32>>();
        let (da, db, dc) = (mk(0), mk(1), mk(2));
        for flags in 0..8u32 {
            let (fa, fb, fc) = (flags & 1 != 0, flags & 2 != 0, flags & 4 != 0);
            let cv = [0.5f32, 0.5, 0.5, 0.5];
            let pa = [da.as_slice()];
            let pb = [db.as_slice()];
            let pc = [dc.as_slice()];
            let a =if fa { cop(1, cv) } else { op(1, &pa) };
            let b = if fb { cop(1, [-0.5; 4]) } else { op(1, &pb) };
            let c = if fc { cop(1, [1.0; 4]) } else { op(1, &pc) };
            let mut o = outs(1, n + 4);
            // shape 11x reads C's plane even when flagged constant: give it one in that case
            let c = if fa && fb { op(1, &pc) } else { c };
            go(&p(1, n as i32), &a, &b, &c, &mut o).unwrap();
            for i in 0..n {
                let x = if fa { 0.5 } else { da[i] };
                let y = if fb { -0.5 } else { db[i] };
                let z = if fc && !(fa && fb) { 1.0 } else { dc[i] };
                assert_eq!(o[0][i], median(x, y, z), "flags {} i {}", flags, i);
            }
            assert_eq!(o[0][n].to_bits(), S);
        }
    }

    #[test]
    fn channels_and_constant_index() {
        // A has 1 channel (clamped plane index), B constant with per-channel constants indexed by the channel itself
        let n = 10usize;
        let a0: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let pa = [a0.as_slice()];
        let a = op(1, &pa);
        let b = cop(3, [1.0, 2.0, 3.0, 4.0]);
        let c = cop(3, [6.0, 6.0, 6.0, 6.0]);
        let mut o = outs(3, n);
        // shape 011: A plane, B const, C const: median(x, c1[ch], 6)
        go(&p(3, n as i32), &a, &b, &c, &mut o).unwrap();
        assert_eq!(o[0][0], 1.0); // median(0, 1, 6)
        assert_eq!(o[1][0], 2.0);
        assert_eq!(o[2][9], 6.0); // median(9, 3, 6) = 6
        assert_eq!(o[2][4], 4.0); // median(4, 3, 6) = 4
    }

    #[test]
    fn nan_and_ties_follow_the_ladder() {
        // shape 000 scalar ladder: b=NaN: c > b false -> c > a ? c : minss(b, a) = a (b<a false -> a)
        let (a, b, c) = ([1.0f32], [f32::NAN], [2.0f32]);
        let (pa, pb, pc) = ([&a[..]], [&b[..]], [&c[..]]);
        let mut o = outs(1, 1);
        go(&p(1, 1), &op(1, &pa), &op(1, &pb), &op(1, &pc), &mut o).unwrap();
        assert_eq!(o[0][0], 2.0); // c > a -> c
        let (a, b, c) = ([5.0f32], [f32::NAN], [2.0f32]);
        let (pa, pb, pc) = ([&a[..]], [&b[..]], [&c[..]]);
        go(&p(1, 1), &op(1, &pa), &op(1, &pb), &op(1, &pc), &mut o).unwrap();
        assert_eq!(o[0][0], 5.0); // not c > a: minss(NaN, 5) = 5
    }

    #[test]
    fn vector_core_matches_scalar_ladder_for_finite() {
        let n = 200usize;
        let vals = [-3.0f32, -1.0, 0.0, 0.25, 1.0, 2.5, 7.0];
        let mk = |sel: usize| (0..n).map(|i| vals[(i * 5 + sel * 2 + i / 7) % vals.len()]).collect::<Vec<f32>>();
        let (da, db, dc) = (mk(0), mk(1), mk(2));
        let (pa, pb, pc) = ([da.as_slice()], [db.as_slice()], [dc.as_slice()]);
        let mut o = outs(1, n);
        go(&p(1, n as i32), &op(1, &pa), &op(1, &pb), &op(1, &pc), &mut o).unwrap();
        for i in 0..n {
            assert_eq!(o[0][i], median(da[i], db[i], dc[i]), "i {}", i);
        }
    }

    #[test]
    fn peel_and_degenerate() {
        let n = 1300usize;
        let d: Vec<f32> = (0..n).map(|i| (i % 11) as f32 - 5.0).collect();
        let pa = [d.as_slice()];
        let mut o = outs(1, n);
        let pp = Params { nout: 1, n: n as i32, allones: !0, dst_phase_bytes: [8, 0, 0, 0] };
        let (a, b, c) = (op(1, &pa), cop(1, [-1.0; 4]), cop(1, [2.0; 4]));
        let mut mo: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        render(&pp, &a, &b, &c, &mut mo).unwrap();
        assert_eq!(o[0][0], -1.0);
        assert_eq!(o[0][7], 2.0); // d = 2 -> median(2,-1,2) = 2
        let mut o = outs(1, 4);
        assert!(go(&p(0, 4), &a, &b, &c, &mut o).is_ok());
        assert_eq!(o[0][0].to_bits(), S);
        assert!(go(&p(1, -5), &a, &b, &c, &mut o).is_ok());
        assert_eq!(go(&p(5, 4), &a, &b, &c, &mut outs(5, 4)), Err(OpError::ConstIndex));
        assert_eq!(assumptions().len(), 8);
    }
}
