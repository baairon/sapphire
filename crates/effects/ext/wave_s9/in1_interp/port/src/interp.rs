//! interp slot 53 AVX2 body . See lib.rs for the summary. Every listing address below is a listing offset.

/// CAPTURED : f32 1.0, bytes 0000803f (gate/lookup_out.tsv). Read .
pub const ONE: f32 = 1.0;
/// CAPTURED (and equal to the dylib bytes): the four f64 window bounds of the copy shortcut, 
/// . test A, test B (vcomisd, NaN fails every test).
pub const RNG_A_HI: f64 = f64::from_bits(0x3ddb7cdfd9d7bdbb); // +1.0e-10
pub const RNG_A_LO: f64 = f64::from_bits(0xbddb7cdfd9d7bdbb); // -1.0e-10
pub const RNG_B_HI: f64 = f64::from_bits(0x3ff000000006df38); // 1 + 1.0e-10
pub const RNG_B_LO: f64 = f64::from_bits(0x3feffffffff24190); // 1 - 1.0e-10

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    ClampA,
    ClampB,
    ClampT,
    AlphaAOff,
    AlphaBOff,
    AlphaZero,
    RangeOff,
    RangeWide,
    RangeBWide,
    CopyShort,
    SwapSub,
    ConstIdx0,
    NoutMinus,
    NoutPlus,
    UnfusedCcc,
    UnfusedCcp,
    UnfusedCpc,
    UnfusedCpp,
    UnfusedPcc,
    UnfusedPcp,
    UnfusedPpc,
    UnfusedPpp,
}

pub const MUTANT_NAMES: [&str; 22] = [
    "clamp_a", "clamp_b", "clamp_t", "alpha_a_off", "alpha_b_off", "alpha_zero", "range_off", "range_wide", "range_b_wide", "copy_short", "swap_sub", "const_idx0",
    "nout_minus", "nout_plus", "unfused_ccc", "unfused_ccp", "unfused_cpc", "unfused_cpp", "unfused_pcc", "unfused_pcp", "unfused_ppc", "unfused_ppp",
];

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "clamp_a" => ClampA,
            "clamp_b" => ClampB,
            "clamp_t" => ClampT,
            "alpha_a_off" => AlphaAOff,
            "alpha_b_off" => AlphaBOff,
            "alpha_zero" => AlphaZero,
            "range_off" => RangeOff,
            "range_wide" => RangeWide,
            "range_b_wide" => RangeBWide,
            "copy_short" => CopyShort,
            "swap_sub" => SwapSub,
            "const_idx0" => ConstIdx0,
            "nout_minus" => NoutMinus,
            "nout_plus" => NoutPlus,
            "unfused_ccc" => UnfusedCcc,
            "unfused_ccp" => UnfusedCcp,
            "unfused_cpc" => UnfusedCpc,
            "unfused_cpp" => UnfusedCpp,
            "unfused_pcc" => UnfusedPcc,
            "unfused_pcp" => UnfusedPcp,
            "unfused_ppc" => UnfusedPpc,
            "unfused_ppp" => UnfusedPpp,
            _ => return Option::None,
        })
    }
    fn unfused_variant(self) -> Option<(bool, bool, bool)> {
        use Mutant::*;
        // (a is a plane, b is a plane, t is a plane)
        match self {
            UnfusedCcc => Some((false, false, false)),
            UnfusedCcp => Some((false, false, true)),
            UnfusedCpc => Some((false, true, false)),
            UnfusedCpp => Some((false, true, true)),
            UnfusedPcc => Some((true, false, false)),
            UnfusedPcp => Some((true, false, true)),
            UnfusedPpc => Some((true, true, false)),
            UnfusedPpp => Some((true, true, true)),
            _ => Option::None,
        }
    }
}

/// One of the three operands. A = this (count), (constant flag), (constants), record; B =,
/// record; T =, record. Plane c of a record is at the record's per-channel pointer. A constant operand may have no planes.
pub struct Operand<'a> {
    pub count: i32,
    pub is_const: bool,
    pub consts: [f32; 4],
    pub planes: &'a [&'a [f32]],
}

/// Explicit parameters, no Default. `nout` = result of base virtual slot 31 (: the word of the chosen combo entry; called at
///  and again after every channel; hooked in R-PARITY, W69). `n` = [info] (i32, sign extended).
pub struct Params {
    pub nout: i32,
    pub n: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpError {
    MissingPlane,
    ShortPlane,
    ShortOut,
    MissingOut,
    ConstIndex,
    NegativeIndex,
}

/// The element function of all eight variants: fma(b - a, t, a) (vsubss/vsubps, then one vfmadd*ss/ps; the product is not rounded).
fn lerp(a: f32, b: f32, t: f32, plane_bits: (bool, bool, bool), m: Mutant) -> f32 {
    let d = if m == Mutant::SwapSub { a - b } else { b - a };
    if m.unfused_variant() == Some(plane_bits) {
        d * t + a
    } else {
        d.mul_add(t, a)
    }
}

fn plane_at<'a>(o: &Operand<'a>, idx: i32, n: usize) -> Result<&'a [f32], InterpError> {
    if idx < 0 {
        return Err(InterpError::NegativeIndex);
    }
    let p = o.planes.get(idx as usize).copied().ok_or(InterpError::MissingPlane)?;
    if p.len() < n {
        return Err(InterpError::ShortPlane);
    }
    Ok(p)
}

/// The window test of (a, b converted to f64 by vcvtss2sd, four ordered compares).
fn in_window(xa: f32, xb: f32, m: Mutant) -> bool {
    let (a, b) = (xa as f64, xb as f64);
    let (ahi, alo) = if m == Mutant::RangeWide { (1e-9, -1e-9) } else { (RNG_A_HI, RNG_A_LO) };
    let (bhi, blo) = if m == Mutant::RangeBWide { (1.000001, 0.999999) } else { (RNG_B_HI, RNG_B_LO) };
    a < ahi && a > alo && b < bhi && b > blo
}

/// Render `nout` channels of n elements each. Mirrors the body: channel loop, alpha rule, the eight
/// variants (A,B const; T plane), (all const), (the other combinations).
pub fn run(p: &Params, a: &Operand, b: &Operand, t: &Operand, outs: &mut [&mut [f32]], m: Mutant) -> Result<(), InterpError> {
    let mut nout = p.nout;
    if m == Mutant::NoutMinus {
        nout -= 1;
    }
    if m == Mutant::NoutPlus {
        nout += 1;
    }
    if nout <= 0 {
        return Ok(()); // test eax,eax; jle 
    }
    let (ca1, cb1, ct1) = (a.count.wrapping_sub(1), b.count.wrapping_sub(1), t.count.wrapping_sub(1));
    // flag bytes live on the stack ([rbp], [rbp]) and the alpha rule rewrites them for every later channel
    let (mut fa, mut fb, ft) = (a.is_const, b.is_const, t.is_const);
    let nn = if p.n > 0 { p.n as usize } else { 0 };
    let mut ch: i32 = 0;
    loop {
        let r12 = ch + 1;
        let cix = if m == Mutant::ConstIdx0 { 0 } else { ch };
        let konst = |o: &Operand, v: i32| -> Result<f32, InterpError> {
            if (0..4).contains(&v) {
                Ok(o.consts[v as usize])
            } else {
                Err(InterpError::ConstIndex)
            }
        };
        // constants are loaded for every channel; an index beyond 3 is outside the contract (reads other fields)
        let mut xa = konst(a, cix).unwrap_or(0.0);
        let mut xb = konst(b, cix).unwrap_or(0.0);
        let xt = konst(t, cix).unwrap_or(0.0);
        if r12 == 4 {
            // operands with fewer than 4 channels read as the constant ONE at channel 3
            let k1 = if m == Mutant::AlphaZero { 0.0 } else { ONE };
            if ca1 < 3 && m != Mutant::AlphaAOff {
                xa = k1;
                fa = true;
            }
            if cb1 < 3 && m != Mutant::AlphaBOff {
                xb = k1;
                fb = true;
            }
        }
        if nn > 0 {
            if cix > 3 {
                return Err(InterpError::ConstIndex);
            }
            let out_idx = ch as usize;
            if out_idx >= outs.len() {
                return Err(InterpError::MissingOut);
            }
            if outs[out_idx].len() < nn {
                return Err(InterpError::ShortOut);
            }
            let ia = if m == Mutant::ClampA { ch } else { ch.min(ca1) };
            let ib = if m == Mutant::ClampB { ch } else { ch.min(cb1) };
            let it = if m == Mutant::ClampT { ch } else { ch.min(ct1) };
            let pa = if fa { None } else { Some(plane_at(a, ia, nn)?) };
            let pb = if fb { None } else { Some(plane_at(b, ib, nn)?) };
            let pt = if ft { None } else { Some(plane_at(t, it, nn)?) };
            let bits = (pa.is_some(), pb.is_some(), pt.is_some());
            if fa && fb && !ft && m != Mutant::RangeOff && in_window(xa, xb, m) {
                // copy t's plane (memcpy for n > 0x18, an 8 wide copy loop otherwise: pure copy)
                let src = pt.unwrap();
                let cnt = if m == Mutant::CopyShort { nn - 1 } else { nn };
                outs[out_idx][..cnt].copy_from_slice(&src[..cnt]);
            } else {
                let o = &mut outs[out_idx];
                for i in 0..nn {
                    let va = pa.map_or(xa, |s| s[i]);
                    let vb = pb.map_or(xb, |s| s[i]);
                    let vt = pt.map_or(xt, |s| s[i]);
                    o[i] = lerp(va, vb, vt, bits, m);
                }
            }
        }
        // call [vtable] again; loop while r12d < result (the hook returns the same nout)
        if r12 < nout {
            ch += 1;
        } else {
            break;
        }
    }
    Ok(())
}

/// S3-style render entry from explicit parameters (the CPU render entry's pixel work, flag 0, AVX2 route).
pub fn render(p: &Params, a: &Operand, b: &Operand, t: &Operand, outs: &mut [&mut [f32]]) -> Result<(), InterpError> {
    run(p, a, b, t, outs, Mutant::None)
}

pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("IN1-A1", "CAPTURED", "ONE 1.0f; the four f64 window bounds (captured_values.tsv rows 'mix C06 bound'); the dylib holds the same bytes (MEASURED-DYLIB, gate/dylib_out.txt)"),
        ("IN1-A2", "ROUTE", "AVX2 route only: feature word 0x9d97ff (cell -> pointer -> word) selects; AVX tier and SSE tier not ported or compared; GPU twin slot 54 NOT REBUILT"),
        ("IN1-A3", "HOST-FP-ASSUMED", "MXCSR round to nearest, no FTZ, no DAZ; the emulator follows the SDM, not silicon; f32::mul_add is fused (software where the CPU lacks FMA)"),
        ("IN1-A4", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 is an explicit parameter; it is the word of the chosen combo entry (1..4 for the 20 measured combos in audit_evt/COMBOS.tsv); the combo choice itself is COMBO-PARITY (evaluator lane)"),
        ("IN1-A5", "LIBC-MODEL", "memcpy thunk (copy shortcut, n > 0x18) is a functional model in the emulator (kind memcpy) and a slice copy in Rust; arguments (dst, src, 4n) are exactly the plane pointers and 4n"),
        ("IN1-A6", "INFERRED", "this are the operand channel counts, their 'is constant' flags, the constants, records with plane c at the record's per-channel pointer: measured from the body's loads, host fill not run"),
        ("IN1-A7", "ALIGN-INDEPENDENT (INFERRED + T2)", "head, ymm core and tail of every variant compute the same element function, so the output does not depend on the destination address; T2 varies the destination phase over 15 values; no phase parameter exists"),
        ("IN1-A8", "CONTRACT", "counts < 1 or > 4 with n > 0 when a plane is read, a missing or short plane, nout > 4, and dst aliasing an input are outside the contract (errors / untested); n <= 0 or nout <= 0 writes nothing"),
    ]
}
