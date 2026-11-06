//! monochrome_a (IN4A): native port of the CPU render entry of builtin class `monochrome_a` (vtable, first slot, slot 53 =,
//! AVX2 body; AVX tier, SSE tier; slot 54 GPU twin not rebuilt). Listing addresses (code VA = listing address;
//! data VA = listing offset). Assumption prefix IN4A-MA.
//!
//! The class is `monochrome` plus an alpha plane: the node turns a 3-or-more channel image into one luma plane (out0) exactly like `monochrome`
//! (`out = f32(fma(b, w2, fma(r, w0, w1 * g)))` in f64, the same section split n < 8 / n < 0x31d / peel from the destination phase / vector / scalar tail),
//! and then, when the channel count is 4 or 2, copies the LAST channel (count - 1) into out1 through the host plane service (tail call, mode 0).
//! A channel count below 3 first calls the service with channel 0 for out0 (the pad plane) and then, for count 2, the same alpha copy.
//! Sections: S0 dispatcher (word 0x9d97ff); S1 count c = [this], c < 3; S2 planes at [this]/28/30
//! (null object -> null planes); S3 n = [info], n <= 0 skips the compute; S4 split; S5 loops; S6 alpha tail: c == 4 or c == 2 -> svc(0, c - 1, outs[1]).
//! FMA sites: scalar peel, vector, scalar tail . Conversions: vcvtss2sd 
//! vcvtps2pd, vcvtpd2ps, vcvtsd2ss . No rcp/rsqrt, no x87.
//! Weights (MEASURED-DYLIB, gate\dylib_out.txt): scalar cells = 0.3125 (plane 0), = 0.5 (plane 1), = 0.1875 (plane 2);
//! vector cells = the same values in all four lanes. std only, no unsafe, no Default.

use std::sync::atomic::{AtomicU32, Ordering};

static MUT: AtomicU32 = AtomicU32::new(0);
/// Mutation switch for the T2 mutation run (0 = none). Used by the `t2` helper only.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}
#[inline]
fn m(id: u32) -> bool {
    MUT.load(Ordering::Relaxed) == id
}

/// Scalar weights of planes 0, 1, 2 (f64): cells .
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weights {
    pub p0: f64,
    pub p1: f64,
    pub p2: f64,
}

/// Explicit parameters. `channels` = [this], `n` = [info], `dst_phase` = (address of out0) & 0x1f (ALIGN-DEPENDENT, only n >= 0x31d).
/// `scalar` = the three scalar weight cells; `vec_w[p][lane]` = the 32 byte vector cells of plane p, four f64 lanes.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub channels: i32,
    pub n: i32,
    pub dst_phase: u32,
    pub scalar: Weights,
    pub vec_w: [[f64; 4]; 3],
}

/// One plane service call (mode 0): copy source channel `chan` of argument 0 into output plane `out` (0 or 1). For `chan` 0 into out 0 with
/// fewer than 3 channels this is the pad-plane fill of `monochrome`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Call {
    pub chan: i32,
    pub out: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonoError {
    ShortPlane,
    MissingSource,
}

/// Section split : (peel, vector end).
pub fn split(n: i32, dst_phase: u32) -> (usize, usize) {
    let t8 = if m(3) { 9 } else { 8 };
    let t797 = if m(4) { 800 } else { 0x31d };
    if n < t8 {
        return (0, 0);
    }
    if n < t797 {
        return (0, (n & !7) as usize);
    }
    let r8 = dst_phase & 0x1f;
    let mut peel: u32 = 0;
    if !m(12) && r8 != 0 {
        if r8 & 3 != 0 {
            return (0, 0);
        }
        peel = (32 - r8) >> 2;
        if m(5) {
            peel += 1;
        }
    }
    if (n as i64) < peel as i64 + 8 {
        return (0, 0);
    }
    let rem = ((n as u32).wrapping_sub(peel)) & if m(6) { 3 } else { 7 };
    (peel as usize, (n as u32 - rem) as usize)
}

fn to_f32_trunc(x: f64) -> f32 {
    let r = x as f32;
    if r.is_nan() || r.is_infinite() || (r as f64).abs() <= x.abs() {
        return r;
    }
    f32::from_bits(r.to_bits().wrapping_sub(1))
}

/// One element: `t = w1 * g` (vmulsd/vmulpd), `t = fma(r, w0, t)`, `t = fma(b, w2, t)`, one rounding to f32 (vcvtsd2ss/vcvtpd2ps).
fn luma(w: (f64, f64, f64), r: f32, g: f32, b: f32) -> f32 {
    let (w0, w1, w2) = if m(1) {
        (w.2, w.1, w.0)
    } else if m(2) {
        (w.1, w.0, w.2)
    } else {
        w
    };
    if m(8) {
        let t = (w1 as f32) * g;
        let t = r.mul_add(w0 as f32, t);
        return b.mul_add(w2 as f32, t);
    }
    let (r, g, b) = (r as f64, g as f64, b as f64);
    let t = if m(9) {
        let t = r * w0;
        let t = g.mul_add(w1, t);
        b.mul_add(w2, t)
    } else if m(11) {
        let t = w1 * g;
        let t = r * w0 + t;
        b * w2 + t
    } else {
        let t = w1 * g;
        let t = r.mul_add(w0, t);
        b.mul_add(w2, t)
    };
    if m(10) {
        to_f32_trunc(t)
    } else if m(16) {
        let v = t as f32;
        if v != 0.0 && v.abs() < f32::MIN_POSITIVE {
            0.0
        } else {
            v
        }
    } else {
        t as f32
    }
}

/// Runs the node: fills out0 (luma, only for channels >= 3 and n > 0) and returns the plane service calls in program order.
/// `src` = planes 0, 1, 2 of argument 0 ([this] null gives None: only an error if the compute path needs it).
pub fn render(p: &Params, src: Option<[&[f32]; 3]>, out0: &mut [f32]) -> Result<Vec<Call>, MonoError> {
    let c = p.channels;
    let mut calls = Vec::new();
    let lim = if m(7) { 3 } else { 2 };
    if c <= lim {
        if !m(23) {
            calls.push(Call { chan: if m(24) { 1 } else { 0 }, out: 0 });
        }
    } else {
        let n = if m(13) && p.n == 0 { 1 } else { p.n };
        if n > 0 {
            let src = src.ok_or(MonoError::MissingSource)?;
            let nu = n as usize;
            if src.iter().any(|pl| pl.len() < nu) || out0.len() < nu {
                return Err(MonoError::ShortPlane);
            }
            let (lo, hi) = split(n, p.dst_phase);
            for i in 0..nu {
                let w = if i >= lo && i < hi {
                    let l = (i - lo) & 3;
                    (p.vec_w[0][l], p.vec_w[1][l], p.vec_w[2][l])
                } else {
                    (p.scalar.p0, p.scalar.p1, p.scalar.p2)
                };
                out0[i] = luma(w, src[0][i], src[1][i], src[2][i]);
            }
        }
    }
    // edx == 4 or edx == 2 -> svc(0, edx - 1, outs[1])
    let alpha = if m(21) { c == 4 } else if m(22) { c == 4 || c == 2 || c == 3 } else { c == 4 || c == 2 };
    if alpha {
        calls.push(Call { chan: if m(20) { c } else { c - 1 }, out: if m(25) { 0 } else { 1 } });
    }
    Ok(calls)
}

/// The class's combo rows (audit_evt COMBOS.tsv, MEASURED): operand channel count -> result count.
pub const COMBOS: [(i32, i32); 4] = [(1, 1), (2, 2), (3, 1), (4, 2)];

pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("IN4A-MA1", "weight cells: scalar and vector are explicit parameters (no default); MEASURED-DYLIB values 0.3125 / 0.5 / 0.1875 in every lane; T2 runs a second value for each group (vector alt, scalar alt)"),
        ("IN4A-MA2", "ROUTE AVX2-ONLY: feature word cell served as 0x9d97ff in the R-PARITY run; AVX tier, SSE tier, init retry and the GPU twin are not ported or compared"),
        ("IN4A-MA3", "HOST-HOOKED: the callee (host plane service) is a record-only hook in T2 (arguments this, mode 0, channel, destination plane, info compared); the executor wiring runs the finished plane_service crate for every recorded call"),
        ("IN4A-MA4", "ALIGN-DEPENDENT: dst_phase only chooses the peel for n >= 0x31d; HOST-FP-ASSUMED (round to nearest, no FTZ, no DAZ); a null [this] with n > 0 and c >= 3 is outside the contract (error here)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Params = Params {
        channels: 3,
        n: 0,
        dst_phase: 0,
        scalar: Weights { p0: 0.3125, p1: 0.5, p2: 0.1875 },
        vec_w: [[0.3125; 4], [0.5; 4], [0.1875; 4]],
    };
    fn p(ch: i32, n: i32) -> Params {
        Params { channels: ch, n, ..P }
    }

    #[test]
    fn luma_hand_values_and_grey_identity() {
        let (r, g, b) = ([1.0f32, 0.25], [0.0f32, 0.25], [0.0f32, 0.25]);
        let mut o = [9.0f32; 2];
        let c = render(&p(3, 2), Some([&r, &g, &b]), &mut o).unwrap();
        assert!(c.is_empty());
        assert_eq!(o[0], 0.3125);
        assert_eq!(o[1], 0.25);
    }

    #[test]
    fn alpha_call_for_two_and_four_channels_only() {
        let (r, g, b) = ([0.5f32; 9], [0.5f32; 9], [0.5f32; 9]);
        let mut o = [0.0f32; 9];
        assert_eq!(render(&p(4, 9), Some([&r, &g, &b]), &mut o).unwrap(), vec![Call { chan: 3, out: 1 }]);
        assert_eq!(render(&p(3, 9), Some([&r, &g, &b]), &mut o).unwrap(), vec![]);
        assert_eq!(render(&p(5, 9), Some([&r, &g, &b]), &mut o).unwrap(), vec![]);
        assert_eq!(render(&p(2, 9), None, &mut o).unwrap(), vec![Call { chan: 0, out: 0 }, Call { chan: 1, out: 1 }]);
        assert_eq!(render(&p(1, 9), None, &mut o).unwrap(), vec![Call { chan: 0, out: 0 }]);
        // n <= 0 still copies the alpha plane for 4 channels
        assert_eq!(render(&p(4, 0), None, &mut o).unwrap(), vec![Call { chan: 3, out: 1 }]);
        assert_eq!(render(&p(0, 5), None, &mut o).unwrap(), vec![Call { chan: 0, out: 0 }]);
    }

    #[test]
    fn split_rule_and_vector_cells() {
        assert_eq!(split(7, 0), (0, 0));
        assert_eq!(split(8, 0), (0, 8));
        assert_eq!(split(100, 0), (0, 96));
        assert_eq!(split(0x31d, 4), (7, 0x31d - ((0x31d - 7) & 7) as usize));
        assert_eq!(split(900, 2), (0, 0));
        // vector cells matter only in the vector range
        let n = 16usize;
        let src = [vec![0.7f32; n], vec![0.3f32; n], vec![0.9f32; n]];
        let mut a = vec![0.0f32; n];
        let mut b = vec![0.0f32; n];
        let mut q = p(3, n as i32);
        render(&q, Some([&src[0], &src[1], &src[2]]), &mut a).unwrap();
        q.vec_w[1] = [0.6; 4];
        render(&q, Some([&src[0], &src[1], &src[2]]), &mut b).unwrap();
        assert!(a != b);
        let mut q2 = p(3, 7);
        let mut c = vec![0.0f32; 7];
        let mut d = vec![0.0f32; 7];
        render(&q2, Some([&src[0][..7], &src[1][..7], &src[2][..7]]), &mut c).unwrap();
        q2.vec_w[1] = [0.6; 4];
        render(&q2, Some([&src[0][..7], &src[1][..7], &src[2][..7]]), &mut d).unwrap();
        assert_eq!(c, d);
    }

    #[test]
    fn errors_and_determinism() {
        let mut o = [0.0f32; 4];
        assert_eq!(render(&p(3, 4), None, &mut o), Err(MonoError::MissingSource));
        let s = [0.0f32; 2];
        assert_eq!(render(&p(3, 4), Some([&s, &s, &s]), &mut o), Err(MonoError::ShortPlane));
        let t = [f32::NAN, f32::INFINITY, -0.0, 1e-40];
        let mut a = [0.0f32; 4];
        let mut b = [0.0f32; 4];
        render(&p(3, 4), Some([&t, &t, &t]), &mut a).unwrap();
        render(&p(3, 4), Some([&t, &t, &t]), &mut b).unwrap();
        assert_eq!(a.map(f32::to_bits), b.map(f32::to_bits));
    }
}
