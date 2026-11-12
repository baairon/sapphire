//! tilt_alpha: native port of the slot 52 AVX2 and FMA body (the dispatcher selects it for feature word 0x9d97ff).
//! Sections (listing addresses, MEASURED from the listing):
//!  S1 0x63e5ed to 0x63e644: inv = 1 / amount; m = sqrt(fma(tilt, tilt, swivel * swivel)); angle = m * K_DEG
//!  S2 0x63e644: sincosf(angle) (crate libm_sincos) gives (sin_ang, cos_ang)
//!  S3 0x63e649 to 0x63e685: q = f32(1.0 / max(f64(m), K_FLOOR)); cos_over_amount = f64(cos_ang * inv); knt = q * -tilt; ksw = q * swivel
//!  S4 0x63e67d to 0x63e6db: cos_over_amount strictly between K_LO and K_HI takes the host branch (ROI helper and host dispatch; not ported)
//!  S5 0x63e6e0 to 0x63e712: l = f32((f64(w) * K_SCALE) * d / f64(h)); lx = l * inv; L = max(K_FOUR, lx) (leaf 0x6670e0)
//!     sinpa = sin_ang * amount; Ls = (cos_ang * inv > 0) ? -L : L
//!  S6 0x63e773: flag != 0 takes the offload serializer path (not ported, reported as Route::Flag)
//!  S7 0x63e77c to 0x63e8de: n <= 0 return; n < 8 all scalar; n < 1200 vector from 0 in blocks of 8; else peel to 32 byte alignment of dst
//!  S8 scalar element: t = ksw * plane_a; t = fma(knt, plane_b, -t); t = fma(-(sinpa * Ls), t, -(cos_ang * Ls)); below zero gives +0, else min(1.0, t)
//!  S9 vector element (8 wide): t = knt * plane_b; t = fma(-ksw, plane_a, t); t = fma(-sinpa, t, -cos_ang); t = Ls * t; below zero gives +0, else min(max, t)
//! FMA sites: 0x63e617, 0x63e7fa, 0x63e800, 0x63e866, 0x63e86c, 0x63e8b6, 0x63e8bc. Conversion sites: 0x63e652, 0x63e670, 0x63e679, 0x63e6f6,
//! 0x63e6fb, 0x63e712. No approximate instruction in the body. The emulator follows the SDM, not silicon.

use fx_common::nanop::{fma213, fma231, fms213, fms231, fnms213, NanOps};
use fx_common::x86::{flip_sign, maxss, sqrtss};
pub use libm_sincos::sincosf_x;

/// Degrees to radians scale of the angle (cell read from the image by the emulator, 0x3c8efa35).
pub const K_DEG: f32 = f32::from_bits(0x3c8e_fa35);
/// Divisor floor of the angle magnitude (f64 1e-5).
pub const K_FLOOR: f64 = f64::from_bits(0x3ee4_f8b5_88e3_68f1);
/// Numerator of the divisor ratio (f64 1.0).
pub const K_ONE_D: f64 = 1.0;
/// Host branch window: cos_over_amount > K_LO and cos_over_amount < K_HI (f64 -1e-4 and 1e-4).
pub const K_LO: f64 = f64::from_bits(0xbf1a_36e2_eb1c_432d);
pub const K_HI: f64 = f64::from_bits(0x3f1a_36e2_eb1c_432d);
/// (double)0.36f
pub const K_SCALE: f64 = f64::from_bits(0x3fd7_0a3d_8000_0000);
/// Lower bound of the host scale (f32 4.0).
pub const K_FOUR: f32 = 4.0;
/// Scalar clamp maximum (f32 1.0); the vector clamp maximum is eight equal lanes of the same value.
pub const K_ONE: f32 = 1.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// state +0x6c
    pub swivel: f32,
    /// state +0x70
    pub tilt: f32,
    /// state +0x88
    pub amount: f32,
}

/// Host record fields read on the main route: ctx+0x520 points at a record holding an i32 at +8, a f64 at +0x30 and an i32 at +0x48.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Host {
    pub w: i32,
    pub d: f64,
    pub h: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Route {
    /// host branch taken (ROI helper and host dispatch; no plane write by this body)
    HostBranch,
    /// flag != 0: offload serializer (not ported); the plane is not written
    Flag,
    /// n <= 0: nothing to do
    Empty,
    /// the element loops ran
    Ramp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plan {
    pub peel: usize,
    pub vend: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Outcome {
    pub route: Route,
    pub sin_ang: f32,
    pub cos_ang: f32,
    pub cos_over_amount: f64,
    /// signed host scale, knt and ksw (the operands of the element loops); 0.0 until computed
    pub ls: f32,
    pub knt: f32,
    pub ksw: f32,
    pub plan: Option<Plan>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ShortBuffer,
}

/// Double precision multiply and divide with the hardware NaN rule (first operand, quieted; a fresh NaN is the default NaN).
fn pick64(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        f64::from_bits(a.to_bits() | 0x0008_0000_0000_0000)
    } else if b.is_nan() {
        f64::from_bits(b.to_bits() | 0x0008_0000_0000_0000)
    } else {
        f64::from_bits(0xfff8_0000_0000_0000)
    }
}
fn mulsd(a: f64, b: f64) -> f64 {
    let r = a * b;
    if r.is_nan() { pick64(a, b) } else { r }
}
fn divsd(a: f64, b: f64) -> f64 {
    let r = a / b;
    if r.is_nan() { pick64(a, b) } else { r }
}
/// `vcvtsd2ss`: a NaN keeps its sign and the top of its payload and is quieted.
fn cvtsd2ss(x: f64) -> f32 {
    if x.is_nan() {
        let b = x.to_bits();
        f32::from_bits((((b >> 63) as u32) << 31) | 0x7fc0_0000 | (((b >> 29) as u32) & 0x003f_ffff))
    } else {
        x as f32
    }
}
/// `vcvtss2sd`: a NaN keeps its sign and payload and is quieted.
fn cvtss2sd(x: f32) -> f64 {
    if x.is_nan() {
        let b = x.to_bits();
        f64::from_bits((((b >> 31) as u64) << 63) | 0x7ff8_0000_0000_0000 | (((b & 0x003f_ffff) as u64) << 29))
    } else {
        x as f64
    }
}
/// `vmaxsd a, b`: a > b ? a : b (NaN or equal gives b).
fn maxsd(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// Section plan of the element loops for a destination whose start has the byte phase `phase` = (start & 0x1f) bytes; `None` for n <= 0.
pub fn plan(n: i32, phase: usize) -> Option<Plan> {
    if n <= 0 {
        return None;
    }
    let n = n as usize;
    let all_scalar = Plan { peel: 0, vend: 0 };
    if n < 8 {
        return Some(all_scalar);
    }
    if n < 0x4b0 {
        return Some(Plan { peel: 0, vend: n & !7 });
    }
    let ph = phase & 0x1f;
    let peel = if ph == 0 {
        0
    } else if ph & 3 != 0 {
        return Some(all_scalar);
    } else {
        (32 - ph) / 4
    };
    if n < peel + 8 {
        return Some(all_scalar);
    }
    Some(Plan { peel, vend: n - ((n - peel) & 7) })
}

/// Clamp shared by every element: a strictly negative value gives +0, else min(max, t) with the minss rule (NaN or equal gives t).
#[inline(always)]
fn clamp(t: f32, max: f32) -> f32 {
    if 0.0 > t { 0.0 } else if max < t { max } else { t }
}

/// The body with an injectable sincos and vector clamp maximum. `dst_phase` = (start of dst & 0x1f) in bytes, the byte phase of the destination start.
#[allow(clippy::too_many_arguments)]
pub fn tilt_alpha_with(
    sc: &dyn Fn(f32) -> (f32, f32),
    vmax: f32,
    st: &State,
    host: &Host,
    flag: bool,
    n: i32,
    plane_a: &[f32],
    plane_b: &[f32],
    dst: &mut [f32],
    dst_phase: usize,
) -> Result<Outcome, Error> {
    let inv = 1.0f32.fdiv(st.amount);
    let sw2 = st.swivel.fmul(st.swivel);
    let ntl = flip_sign(st.tilt);
    let s = fma213(st.tilt, st.tilt, sw2);
    let m = sqrtss(s);
    let ang = m.fmul(K_DEG);
    let (sin_ang, cos_ang) = sc(ang);
    let d = maxsd(cvtss2sd(m), K_FLOOR);
    let cinv = cos_ang.fmul(inv);
    let q = cvtsd2ss(divsd(K_ONE_D, d));
    let cos_over_amount = cvtss2sd(cinv);
    let knt = q.fmul(ntl);
    let ksw = q.fmul(st.swivel);
    let mut o = Outcome { route: Route::Ramp, sin_ang, cos_ang, cos_over_amount, ls: 0.0, knt, ksw, plan: None };
    if cos_over_amount > K_LO && K_HI > cos_over_amount {
        o.route = Route::HostBranch;
        return Ok(o);
    }
    let width_scaled = mulsd(host.w as f64, K_SCALE);
    let width_depth = mulsd(width_scaled, host.d);
    let width_ratio = divsd(width_depth, host.h as f64);
    let lx = cvtsd2ss(width_ratio).fmul(inv);
    let big_l = maxss(K_FOUR, lx);
    let sinpa = sin_ang.fmul(st.amount);
    let ls = if cinv > 0.0 { flip_sign(big_l) } else { big_l };
    o.ls = ls;
    if flag {
        o.route = Route::Flag;
        return Ok(o);
    }
    let Some(pl) = plan(n, dst_phase) else {
        o.route = Route::Empty;
        return Ok(o);
    };
    o.plan = Some(pl);
    let n = n as usize;
    if plane_a.len() < n || plane_b.len() < n || dst.len() < n {
        return Err(Error::ShortBuffer);
    }
    let a = sinpa.fmul(ls);
    let b = cos_ang.fmul(ls);
    let nksw = flip_sign(ksw);
    let nsin = flip_sign(sinpa);
    for i in 0..n {
        dst[i] = if i >= pl.peel && i < pl.vend {
            let t = knt.fmul(plane_b[i]);
            let t = fma231(t, nksw, plane_a[i]);
            let t = fms213(t, nsin, cos_ang);
            clamp(ls.fmul(t), vmax)
        } else {
            let t = ksw.fmul(plane_a[i]);
            let t = fms231(t, knt, plane_b[i]);
            clamp(fnms213(t, a, b), K_ONE)
        };
    }
    Ok(o)
}

/// The body as shipped: native sincos port, vector clamp maximum equal to the scalar one.
pub fn tilt_alpha(st: &State, host: &Host, flag: bool, n: i32, plane_a: &[f32], plane_b: &[f32], dst: &mut [f32], dst_phase: usize) -> Result<Outcome, Error> {
    tilt_alpha_with(&sincosf_x, K_ONE, st, host, flag, n, plane_a, plane_b, dst, dst_phase)
}

pub struct Assumption {
    pub id: &'static str,
    pub value: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "TA-A1", value: "feature word 0x9d97ff selects the AVX2 and FMA body (dispatcher not ported)", basis: "harness feature word" },
        Assumption { id: "TA-A2", value: "flag != 0 (offload serializer 0x363d40) is refused by name, not rendered", basis: "host offload plumbing, not ported" },
        // TA-A3 ROI helper: 0x5b9bd0; host dispatch: 0x673100.
        Assumption { id: "TA-A3", value: "host branch (ROI helper and host dispatch) is refused by name when |cos(angle) / amount| < 1e-4", basis: "host services, not ported" },
        Assumption { id: "TA-A4", value: "host record fields (i32 at +8, f64 at +0x30, i32 at +0x48 of the record at ctx+0x520) are caller supplied; measured 64, 1.0 and 1 for the swivel 15, tilt 20 binding only", basis: "MEASURED-EMULATOR-READ" },
        Assumption { id: "TA-A5", value: "sincosf and double tables inherited from libm_sincos, which computes every entry; measured equal to the emulator tables", basis: "libm_sincos assumptions" },
        // TA-A6 sign mask cell: 0x160e6a0.
        Assumption { id: "TA-A6", value: "scalar constants read once from the image by a hook: 1.0, 1.0 f64, 4.0, pi/180, 1e-5, 1e-4, -1e-4, (double)0.36f; the sign mask cell is 0x80000000 in lane 0; the 32 byte clamp cell is eight lanes of 1.0", basis: "MEASURED-EMULATOR-READ" },
    ]
}
