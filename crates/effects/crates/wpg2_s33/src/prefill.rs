//! warp_prefill (WS4): native port of the AVX2 body (listing addresses, VA = listing for data),
//! the callback that runs per tile (warp slot 33 route, class warp, vtable).
//!
//! The body is a per-pixel "log2 of an approximate gradient magnitude" map of one input plane (MEASURED, listing):
//! for every pixel it forms two directional differences scaled by `v`, offsets them by half of `pre_cell_a_bits` / `pre_cell_b_bits`, takes
//! magnitudes, combines them as `max + 0.5 * min` in double precision and writes `exponent + mantissa` (a bit trick
//! log2) of the result. It is NOT a constant prefill: the WS3 note that the callback "fills with v" was INFERRED and is
//! corrected here. Floats are carried as u32 bit patterns so NaN payloads survive; every operation follows the SSE
//! silicon NaN rule (first source operand wins, quieted; generated NaN is the default QNaN 0xffc00000).
//! std only, no unsafe. ROUTE AVX2-ONLY (the dispatcher and the SSE twin are not ported).

pub const SENTINEL: u32 = 0x7fc0_ffee;
const HALF: u32 = 0x3f00_0000; // CAPTURED
const ONE: u32 = 0x3f80_0000; // CAPTURED
const TWO: u32 = 0x4000_0000; // CAPTURED
const KMIX: u64 = 0x3fe0_0000_0000_0000; // double 0.5, CAPTURED
const ABS: u32 = 0x7fff_ffff; // vandps mask, CAPTURED
const DEF32: u32 = 0xffc0_0000;
const DEF64: u64 = 0xfff8_0000_0000_0000;

/// Inputs the original reads through its eight pointer cells (captured fixups) plus the partition callee result.
#[derive(Clone, Debug)]
pub struct Params {
    pub width: i32,   // the width cell, also the row stride (floats)
    pub height: i32,  // the height cell
    pub flag: u32,    // [a cell] (r14d): nonzero = "min" variant, zero = "sum" variant
    pub pre_cell_a_bits: u32,      // [a cell] f32 bits (WS3: 0.0 on the slot 33 route)
    pub pre_cell_b_bits: u32,      // [a cell] f32 bits (WS3: 0.0)
    pub v: u32,       // [a cell] f32 bits ((hi - lo) * 0.25 on the slot 33 route, WS3)
    pub start: i32,   // partition callee output 1 ([rbp]) CALLEE-MODELLED
    pub count: i32,   // partition callee output 2 ([rbp])
}

#[derive(Debug, PartialEq)]
pub enum BodyError {
    Width,
    Height,
    Range,
    Short,
}

fn isn(b: u32) -> bool {
    (b & ABS) > 0x7f80_0000
}
fn isn64(b: u64) -> bool {
    (b & 0x7fff_ffff_ffff_ffff) > 0x7ff0_0000_0000_0000
}
fn f(b: u32) -> f32 {
    f32::from_bits(b)
}
fn fix(r: f32) -> u32 {
    if r.is_nan() {
        DEF32
    } else {
        r.to_bits()
    }
}
fn fix64(r: f64) -> u64 {
    if r.is_nan() {
        DEF64
    } else {
        r.to_bits()
    }
}
fn first(ops: [u32; 3]) -> Option<u32> {
    ops.iter().find(|&&x| isn(x)).map(|&x| x | 0x0040_0000)
}
fn first64(ops: [u64; 3]) -> Option<u64> {
    ops.iter().find(|&&x| isn64(x)).map(|&x| x | 0x0008_0000_0000_0000)
}
pub fn add_ss(a: u32, b: u32) -> u32 {
    first([a, b, 0]).unwrap_or_else(|| fix(f(a) + f(b)))
}
pub fn sub_ss(a: u32, b: u32) -> u32 {
    first([a, b, 0]).unwrap_or_else(|| fix(f(a) - f(b)))
}
pub fn mul_ss(a: u32, b: u32) -> u32 {
    first([a, b, 0]).unwrap_or_else(|| fix(f(a) * f(b)))
}
/// vminss dst, a, b: a < b ? a : b (NaN or equal returns b).
pub fn min_ss(a: u32, b: u32) -> u32 {
    if f(a) < f(b) {
        a
    } else {
        b
    }
}
fn abs_ss(a: u32) -> u32 {
    a & ABS
}
/// vfmadd213ss / vfmsub213ss dst, s2, s3: s2 * dst (+|-) s3, NaN priority (dst, s2, s3) (INFERRED order), one rounding.
pub fn fma213(dst: u32, s2: u32, s3: u32, sub: bool) -> u32 {
    if let Some(x) = first([dst, s2, s3]) {
        return x;
    }
    let c = if sub { -f(s3) } else { f(s3) };
    fix(f(s2).mul_add(f(dst), c))
}
/// vfmsub231ss dst, s2, s3: s2 * s3 - dst (the only 231 single form in the body).
pub fn fmsub231(dst: u32, s2: u32, s3: u32) -> u32 {
    if let Some(x) = first([dst, s2, s3]) {
        return x;
    }
    fix(f(s2).mul_add(f(s3), -f(dst)))
}
fn cvtss2sd(b: u32) -> u64 {
    if isn(b) {
        return ((b as u64 & 0x8000_0000) << 32) | 0x7ff8_0000_0000_0000 | ((b as u64 & 0x007f_ffff) << 29);
    }
    (f(b) as f64).to_bits()
}
fn cvtsd2ss(b: u64) -> u32 {
    if isn64(b) {
        return (((b >> 63) as u32) << 31) | 0x7fc0_0000 | (((b >> 29) as u32) & 0x003f_ffff);
    }
    fix(f64::from_bits(b) as f32)
}
/// vfmadd231sd dst, s2, s3 = s2 * s3 + dst
fn fma231_sd(dst: u64, s2: u64, s3: u64) -> u64 {
    first64([dst, s2, s3]).unwrap_or_else(|| fix64(f64::from_bits(s2).mul_add(f64::from_bits(s3), f64::from_bits(dst))))
}
/// vfmadd213sd dst, s2, s3 = s2 * dst + s3
fn fma213_sd(dst: u64, s2: u64, s3: u64) -> u64 {
    first64([dst, s2, s3]).unwrap_or_else(|| fix64(f64::from_bits(s2).mul_add(f64::from_bits(dst), f64::from_bits(s3))))
}

/// Shared tail of every pixel (and its copies): compare xa with d in double, combine, round to f32, pseudo log2.
fn finish(xa: u64, d: u64) -> u32 {
    let r = if f64::from_bits(xa) > f64::from_bits(d) {
        fma231_sd(xa, d, KMIX) // d * k + xa
    } else {
        fma213_sd(xa, KMIX, d) // k * xa + d
    };
    let m32 = cvtsd2ss(r);
    let pos = f(m32) > 0.0;
    if !pos {
        return 0;
    }
    let e = ((m32 >> 23) & 0xff) as i32 - 128;
    let mant = (m32 & 0x807f_ffff).wrapping_add(0x3f80_0000);
    add_ss(fix(e as f32), mant)
}

fn avec(i: i32, row: &[u32]) -> u32 {
    row[i as usize]
}

/// The body for one tile. `input` and `output` are h*w planes (stride = width); only rows start..start+count of `output` are written.
pub fn run_body(p: &Params, input: &[u32], output: &mut [u32]) -> Result<(), BodyError> {
    if p.count <= 0 {
        return Ok(()); //  jle: nothing written
    }
    if p.width < 2 {
        return Err(BodyError::Width);
    }
    if p.height < 1 {
        return Err(BodyError::Height);
    }
    if p.start < 0 || (p.start as i64 + p.count as i64) > p.height as i64 {
        return Err(BodyError::Range);
    }
    let w = p.width as usize;
    let h = p.height as usize;
    if input.len() < w * h || output.len() < w * h {
        return Err(BodyError::Short);
    }
    let hb0 = mul_ss(HALF, p.pre_cell_a_bits); //  chain: xmm2 = 0.5 * pre_cell_a_bits, spilled to [rbp]
    let hb8 = mul_ss(HALF, p.pre_cell_b_bits); // xmm4 = 0.5 * pre_cell_b_bits, spilled to [rbp]
    let two_v = mul_ss(TWO, p.v); // xmm12 
    let flag = p.flag != 0;
    let wi = w as i32;
    for r in 0..p.count {
        let y = p.start + r;
        let yu = y as usize;
        let cur = &input[yu * w..yu * w + w];
        let prev = if y == 0 { cur } else { &input[(yu - 1) * w..(yu - 1) * w + w] };
        let next = if y == p.height - 1 {
            cur
        } else if yu + 1 < h {
            &input[(yu + 1) * w..(yu + 1) * w + w]
        } else {
            return Err(BodyError::Range);
        };
        let s = if y == 0 || y == p.height - 1 { TWO } else { ONE }; // xmm8 via xmm6 / xmm7
        let sy = mul_ss(p.v, s); // 
        let o = &mut output[yu * w..yu * w + w];
        // ---- x = 0 
        let c0 = avec(0, cur);
        let c1 = avec(1, cur);
        let a = sub_ss(c0, avec(0, prev));
        let d1 = sub_ss(c1, c0);
        let up = fmsub231(hb8, a, sy);
        let vx0 = fma213(two_v, d1, p.pre_cell_a_bits, false);
        let dn0 = sub_ss(avec(0, next), c0);
        let down = fmsub231(hb8, dn0, sy);
        let xa = cvtss2sd(abs_ss(vx0));
        let ua = abs_ss(up);
        let da = abs_ss(down);
        let d = if flag {
            let mn = min_ss(ua, da);
            cvtss2sd(add_ss(mn, mn))
        } else {
            cvtss2sd(add_ss(da, ua))
        };
        o[0] = finish(xa, d);
        // ---- middle x = 1..w-2
        let mut dr_last = d1;
        let mut cn_last = c1;
        for x in 1..(wi - 1) {
            let cx = avec(x, cur);
            let cp = avec(x - 1, cur);
            let cn = avec(x + 1, cur);
            let dl = sub_ss(cx, cp);
            let dr = sub_ss(cn, cx);
            let dn = sub_ss(avec(x, next), cx);
            let dp = sub_ss(cx, avec(x, prev));
            let lv = abs_ss(fma213(dl, p.v, hb0, false));
            let rv = abs_ss(fma213(p.v, dr, hb0, false));
            let nv = abs_ss(fma213(dn, sy, hb8, true));
            let pv = abs_ss(fma213(dp, sy, hb8, true));
            let (xa, d) = if flag {
                let mx = min_ss(lv, rv);
                let my = min_ss(pv, nv);
                (cvtss2sd(add_ss(mx, mx)), cvtss2sd(add_ss(my, my)))
            } else {
                (cvtss2sd(add_ss(rv, lv)), cvtss2sd(add_ss(nv, pv)))
            };
            o[x as usize] = finish(xa, d);
            dr_last = dr;
            cn_last = cn;
        }
        // ---- x = w-1 
        let vxl = abs_ss(fma213(dr_last, two_v, p.pre_cell_a_bits, false));
        let xa = cvtss2sd(vxl);
        let cl = cn_last;
        let dn = sub_ss(avec(wi - 1, next), cl);
        let dp = sub_ss(cl, avec(wi - 1, prev));
        let pv = abs_ss(fma213(dp, sy, hb8, true));
        let nv = abs_ss(fma213(sy, dn, hb8, true));
        let d = if flag {
            let mn = min_ss(pv, nv);
            cvtss2sd(add_ss(mn, mn))
        } else {
            cvtss2sd(add_ss(nv, pv))
        };
        o[(wi - 1) as usize] = finish(xa, d);
    }
    Ok(())
}
