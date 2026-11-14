//! Callback 0x46b9d0, AVX2 and FMA arm: smoothing and normalisation of two plane pairs.
//!
//! The listing is scalar code in both arms (the AVX2 arm only uses the three-operand scalar fused forms). Two passes over
//! the same row range, each on a triple (src, p, q): pass 1 uses (0x28, 0x30, 0x38), pass 2 uses (0x40, 0x48, 0x50).
//! With c = field 0x78 and a damping factor d = c > 0 ? 1 / (c + 4) : f6c / f68, per element
//!   q' = q + d * (s * dy - c * q)      p' = p + d * (s * dx - c * p)
//! where dy is the forward vertical difference of `src` (zero at and below the last row), dx the forward horizontal
//! difference (zero at the last column) and s the scale plane element (absent plane: s = 1, which gives the unscaled
//! fused forms of the listing). The pair (p', q') is divided by its Euclidean length when the squared length exceeds 1.
//! The last row and the last column use their own fused forms; the pass-2 forms differ from the pass-1 forms in the
//! operand order of one fused site per scale-absent variant (that matters only for the NaN operand choice).
//! Row range: (start, count) as written by the host row splitter 0x6701b0; both passes are skipped when start >= start + count.
use fx_common::nanop::{fma231, fms213, fms231, fnma231, fnms213, NanOps};
use fx_common::x86::sqrtss;

/// Cell 0x15e850c: scalar f32 1.0 (bytes read by the original at load, reader 0x470152 and the reads of this callback).
pub const ONE: f32 = 1.0;
/// Cell 0x15e8548: scalar f32 4.0 (bytes read by the original at load, reader 0x47016a).
pub const FOUR: f32 = 4.0;

pub struct Assumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "CELL-ONE", label: "MEASURED-EMULATOR-READ", basis: "scalar f32 1.0, threshold of the length test and numerator of the damping factor (read from the original at load)" },
        Assumption { id: "CELL-FOUR", label: "MEASURED-EMULATOR-READ", basis: "scalar f32 4.0, added to c in the damping factor (read from the original at load)" },
        Assumption { id: "NO-ALIAS", label: "ASSUMED", basis: "the six planes and the scale plane do not overlap each other or the state record" },
        Assumption { id: "NO-OVERFLOW", label: "ASSUMED", basis: "row offsets fit in 32 bits (the listing multiplies row by stride in 32 bits for the first row of a range)" },
        Assumption { id: "SPLITTER", label: "HOST-SERVICE", basis: "the row range (start, count) is an input; the original's row splitter is a hooked host service" },
    ]
}

#[derive(Clone, Copy)]
pub struct Geo {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    /// field 0x68
    pub relax_scale: f32,
    /// field 0x6c
    pub smooth_weight: f32,
    /// field 0x78
    pub c: f32,
}

/// One pass: the read-only source plane and the two planes updated in place.
pub struct Pass<'a> {
    pub src: &'a [f32],
    pub p: &'a mut [f32],
    pub q: &'a mut [f32],
}

/// Damping factor (listing 0x46baa4..0x46baba and the reload at 0x46be17..0x46be2d).
pub fn damping(g: &Geo) -> f32 {
    if g.c > 0.0 {
        ONE.fdiv(g.c.fadd(FOUR))
    } else {
        g.smooth_weight.fdiv(g.relax_scale)
    }
}

/// Each fused call carries a tag `/*@ADDR:PASS ...*/` with the listing address of the instruction it models in pass 1 and in pass 2.
fn pass(g: &Geo, s: Option<&[f32]>, pl: &mut Pass, start: i32, count: u32, second: bool) {
    let w = g.width as i64;
    if w <= 0 {
        return;
    }
    let (c, d) = (g.c, damping(g));
    let cd = c.fmul(d);
    for k in 0..count {
        let y = start.wrapping_add(k as i32);
        let off = y.wrapping_mul(g.stride) as i64;
        let nxt = off + g.stride as i64;
        let last_row = y >= g.height.wrapping_sub(1);
        for i in 0..w {
            let last_col = i >= w - 1;
            let ix = (off + i) as usize;
            let (cc, bb) = (pl.q[ix], pl.p[ix]);
            let centre = pl.src[ix];
            let (bv, cv, grad_sq) = match (s, last_row, last_col) {
                (Some(sel_plane), false, false) => {
                    let sv = sel_plane[ix];
                    let (an, right) = (pl.src[(nxt + i) as usize], pl.src[ix + 1]);
                    let (down_diff, right_diff) = (an.fsub(centre), right.fsub(centre));
                    let (c_bb, c_cc) = (c.fmul(bb), c.fmul(cc));
                    let u_down = /*@46bc00:1 46bf5e:2*/ fms213(down_diff, sv, c_cc);
                    let u_right = /*@46bc05:1 46bf63:2*/ fms213(right_diff, sv, c_bb);
                    let cv = /*@46bc0a:1 46bf68:2*/ fma231(cc, u_down, d);
                    let bv = /*@46bc0f:1 46bf6d:2*/ fma231(bb, u_right, d);
                    (bv, cv, /*@46bc18:1 46bf76:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (Some(sel_plane), false, true) => {
                    let sv = sel_plane[ix];
                    let an = pl.src[(nxt + i) as usize];
                    let c_cc = c.fmul(cc);
                    let bv = /*@46bc42:1 46bfa1:2*/ fnma231(bb, bb, cd);
                    let u = /*@46bc4c:1 46bfac:2*/ fms213(an.fsub(centre), sv, c_cc);
                    let cv = /*@46bc51:1 46bfb1:2*/ fma231(cc, u, d);
                    (bv, cv, /*@46bc5a:1 46bfba:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (Some(sel_plane), true, false) => {
                    let sv = sel_plane[ix];
                    let right = pl.src[ix + 1];
                    let cv = /*@46bd19:1 46c078:2*/ fnma231(cc, cc, cd);
                    let u = /*@46bd46:1 46c0a6:2*/ fms213(right.fsub(centre), sv, c.fmul(bb));
                    let bv = /*@46bd4b:1 46c0ab:2*/ fma231(bb, u, d);
                    (bv, cv, /*@46bd50:1 46c0b0:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (Some(_), true, true) => {
                    let cv = /*@46bd19:1 46c078:2*/ fnma231(cc, cc, cd);
                    let bv = /*@46bd75:1 46c0d5:2*/ fnma231(bb, bb, cd);
                    (bv, cv, /*@46bd7a:1 46c0da:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (None, false, false) => {
                    let (an, right) = (pl.src[(nxt + i) as usize], pl.src[ix + 1]);
                    let c_cc = c.fmul(cc);
                    let b_term_right = if second { /*@46c1df:2*/ fms213(c, bb, right) } else { /*@46c44a:1*/ fms231(right, bb, c) };
                    let b_term_centre = /*@46c44f:1 46c1e6:2*/ fms231(bb, d, centre);
                    let c_term = /*@46c454:1 46c1eb:2*/ fms231(cc, c_cc, d);
                    let bv = /*@46c459:1 46c1f0:2*/ fnms213(b_term_right, d, b_term_centre);
                    let cv = /*@46c464:1 46c1fb:2*/ fnms213(centre.fsub(an), d, c_term);
                    (bv, cv, /*@46c46d:1 46c204:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (None, false, true) => {
                    let an = pl.src[(nxt + i) as usize];
                    let c_cc = c.fmul(cc);
                    let c_term = /*@46c4a2:1 46c23d:2*/ fms213(c_cc, d, cc);
                    let bv = /*@46c4a7:1 46c242:2*/ fnma231(bb, bb, cd);
                    let cv = /*@46c4b1:1 46c24d:2*/ fms213(an.fsub(centre), d, c_term);
                    (bv, cv, /*@46c4ba:1 46c256:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (None, true, false) => {
                    let right = pl.src[ix + 1];
                    let cv = /*@46c55a:1 46c2fd:2*/ fnma231(cc, cc, cd);
                    let b_term_right = if second { /*@46c31d:2*/ fms213(c, bb, right) } else { /*@46c57c:1*/ fms231(right, bb, c) };
                    let b_term_centre = /*@46c581:1 46c324:2*/ fms213(centre, d, bb);
                    let bv = /*@46c586:1 46c329:2*/ fnms213(b_term_right, d, b_term_centre);
                    (bv, cv, /*@46c58b:1 46c32e:2*/ fma231(cv.fmul(cv), bv, bv))
                }
                (None, true, true) => {
                    let cv = /*@46c55a:1 46c2fd:2*/ fnma231(cc, cc, cd);
                    let bv = /*@46c5be:1 46c361:2*/ fnma231(bb, bb, cd);
                    (bv, cv, /*@46c5c3:1 46c366:2*/ fma231(cv.fmul(cv), bv, bv))
                }
            };
            if grad_sq > ONE {
                let n = sqrtss(grad_sq);
                pl.p[ix] = bv.fdiv(n);
                pl.q[ix] = cv.fdiv(n);
            } else {
                pl.p[ix] = bv;
                pl.q[ix] = cv;
            }
        }
    }
}

/// The whole callback: both passes over the row range. `s` is the scale plane (field 0x70), None when it is null.
pub fn smooth_normalise(g: &Geo, s: Option<&[f32]>, pass1: &mut Pass, pass2: &mut Pass, start: i32, count: u32) {
    if start >= start.wrapping_add(count as i32) {
        return;
    }
    pass(g, s, pass1, start, count, false);
    pass(g, s, pass2, start, count, true);
}

/// Scalar callback 0x474800 (176 B, single path): multiplies the plane at field 0x18 in place by the f32 at field 0x30 over the
/// rows start..start+count (signed row loop as in the listing: rows while y < start + count in 32 bit signed arithmetic).
/// Per element `x = x * scale` (mulss, plane element first).
pub fn scale_plane(plane: &mut [f32], width: i32, stride: i32, scale: f32, start: i32, count: u32) {
    let end = start.wrapping_add(count as i32);
    if start >= end {
        return;
    }
    let mut off = start.wrapping_mul(stride) as i64;
    let mut y = start;
    while y < end {
        for i in 0..width.max(0) as i64 {
            let ix = (off + i) as usize;
            plane[ix] = plane[ix].fmul(scale);
        }
        y = y.wrapping_add(1);
        off += stride as i64;
    }
}
