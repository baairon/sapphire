//! perspective_alpha: native port of slot 52 AVX2 body (dispatcher, word 0x9d97ff selects).
//! Class `perspective_alpha`vtable . The emulator follows the SDM, not silicon.
//! Sections (listing addresses, MEASURED from neutral_1.asm):
//!  S1 prologue; theta = ((K_CELL_A * this.params_b) * K_CELL_C); call sincos -> (r0, r1)
//!  S2 x8 = r1 * this.params_a; x8 == 0 (ordered) goes to S3
//!  S3 ROI record then plane fill (dst, flag, 0, roi): memset(dst, 0, 4 * (c * b)) for flag 0
//!  S4 edi = trunc(((s8 * K_CELL_B) * port_info_a) / (f32(params_c * f32(port_info_b)))), m = max(edi, 4),
//!     ecx = x8 > 0 ? -m : m; P = params_c * r0
//!  S5 flag != 0: tail call (not ported; its arguments are reported)
//!  S6 section plan: n <= 0 return; n < 16 all scalar; n < 29 vector from 0; else peel to 32 byte alignment of dst
//!  S7 scalar peel loop, S8 16 wide loop (two ymm), S9 scalar tail
//!     every element: out = clamp(f32(ecx) * fma(-P, v, -r1)): below zero gives +0, else min(max, q). Scalar parts clamp at 1.0f
//! vector parts at the 8 lane cell (NOT captured: stand-in, PX2-A1).
//! FMA sites: vfnmsub213ss, and vfmsub213ps (ymm3 = -P), vfnmsub213ss: all out = fma(-P, v, -r1).

pub use libm_sincos::sincosf_x;

/// The one native for (crate libm_sincos) in the shape `perspective_alpha_with` takes. Always `Some`: every f32 is handled
/// (finite |theta| >= 8192 goes through the ported), so `Error::SincosOutOfContract` only comes from a caller supplied closure.
pub fn sincos_native(x: f32) -> Option<(f32, f32)> {
    Some(sincosf_x(x))
}

const K_CELL_A: f32 = f32::from_bits(0xbe80_0000); // -0.25 (MEASURED)
const K_CELL_C: f32 = f32::from_bits(0x40c9_0fdb); // 2 pi, 4 B read (MEASURED)
const K_CELL_B: f64 = f64::from_bits(0x3fd7_0a3d_8000_0000); // (double)0.36f (MEASURED)
const ONE: f32 = 1.0; //  (MEASURED)
/// Vector clamp maximum, 8 lanes of (NOT captured): stand-in 1.0f (PX2-A1, same cell as SC2-A1).
pub const RAMP_MAX_STANDIN: f32 = 1.0;
/// ROI element size, qword, MEASURED = 4.
const ROI_ELEM: i64 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    /// this
    pub params_b: f32,
    /// this
    pub params_a: f32,
    /// this
    pub params_c: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Info {
    /// info: element count of the run
    pub n: i32,
    /// info: ROI form selector (0: c = port_blk_a, else c = port_blk_c)
    pub roi_form: i32,
    /// info
    pub port_blk_a: i32,
    pub port_blk_b: i32,
    pub port_blk_c: i32,
    /// [info] and (i32), (f64)
    pub s8: i32,
    pub port_info_b: i32,
    pub port_info_a: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Route {
    /// x8 == 0: ROI zero fill of `count` floats (with flag 0; the byte count is 4 * count) or, flag != 0, only the call is reported
    Fill { count: i32, flag: bool },
    /// flag != 0: tail call (this, info, outs, xmm0 = r1, xmm1 = P); no plane write
    Gpu { r1: f32, p: f32 },
    /// the element loops ran
    Ramp,
}

/// Section split of the element loops: elements [0, peel) scalar, [peel, vend) 16 wide, [vend, n) scalar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plan {
    pub peel: usize,
    pub vend: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Outcome {
    pub route: Route,
    /// argument of 
    pub theta: f32,
    pub r0: f32,
    pub r1: f32,
    pub x8: f32,
    /// edi handed to (esi is 4); only on the non fill routes
    pub edi: Option<i32>,
    pub ecx: Option<i32>,
    pub p: Option<f32>,
    pub plan: Option<Plan>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// the injected sincos returned None (never the default entry: libm_sincos handles every f32 incl. finite |theta| >= 8192)
    SincosOutOfContract,
    ShortBuffer,
}

fn cvttsd2si32(x: f64) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x <= -2147483649.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Section plan of for a destination whose address has `phase` = (address & 0x1f) bytes.
/// `None` for n <= 0 (return). A phase that is not a multiple of 4 gives all scalar.
pub fn plan(n: i32, phase: usize) -> Option<Plan> {
    if n <= 0 {
        return None;
    }
    let n = n as usize;
    let all_scalar = Plan { peel: 0, vend: 0 };
    if n < 16 {
        return Some(all_scalar);
    }
    if n < 29 {
        return Some(Plan { peel: 0, vend: n & !15 });
    }
    let ph = phase & 0x1f;
    let peel = if ph == 0 {
        0
    } else if ph & 3 != 0 {
        return Some(all_scalar);
    } else {
        (32 - ph) / 4
    };
    if n < peel + 16 {
        return Some(all_scalar);
    }
    Some(Plan { peel, vend: n - ((n - peel) & 15) })
}

/// One element: q = f32(ecx) * fma(-P, v, -r1); q < 0 gives +0.0 (the integer 0 store or the cmpltps/andnps mask), else min(max, q)
/// with the minss/minps rule (second operand on NaN or equality).
fn ramp(cf: f32, p: f32, r1: f32, v: f32, max: f32) -> f32 {
    let q = nx_mul(cf, nx_fma(-p, v, -r1));
    if q < 0.0 {
        0.0
    } else if max < q {
        max
    } else {
        q
    }
}

/// The body with an injectable sincos and vector clamp maximum (the call-record check feeds the emulator's stand-in; T2 varies `ramp_max`).
/// `dst_phase` = (address of dst[0] & 0x1f) in bytes.
#[allow(clippy::too_many_arguments)]
pub fn perspective_alpha_with(
    sc: &dyn Fn(f32) -> Option<(f32, f32)>,
    ramp_max: f32,
    par: &Params,
    info: &Info,
    flag: bool,
    src: &[f32],
    dst: &mut [f32],
    dst_phase: usize,
) -> Result<Outcome, Error> {
    let theta = nx_mul(nx_mul(K_CELL_A, par.params_b), K_CELL_C);
    let (r0, r1) = sc(theta).ok_or(Error::SincosOutOfContract)?;
    let x8 = nx_mul(r1, par.params_a);
    let mut o = Outcome { route: Route::Ramp, theta, r0, r1, x8, edi: None, ecx: None, p: None, plan: None };
    if x8 == 0.0 {
        let b = info.port_blk_b;
        let c = if info.roi_form == 0 { info.port_blk_a } else { info.port_blk_c };
        let cnt = c.wrapping_mul(b);
        o.route = Route::Fill { count: cnt, flag };
        if !flag {
            let bytes = (cnt as i64) * ROI_ELEM;
            let n = if bytes > 0 { (bytes / 4) as usize } else { 0 };
            if dst.len() < n {
                return Err(Error::ShortBuffer);
            }
            dst[..n].fill(0.0);
        }
        return Ok(o);
    }
    let p = nx_mul(par.params_c, r0);
    let den = nx_mul(par.params_c, info.port_info_b as f32) as f64;
    let num = ((info.s8 as f64) * K_CELL_B) * info.port_info_a;
    let edi = cvttsd2si32(num / den);
    let m = edi.max(4);
    let ecx = if x8 > 0.0 { m.wrapping_neg() } else { m };
    o.edi = Some(edi);
    o.ecx = Some(ecx);
    o.p = Some(p);
    if flag {
        o.route = Route::Gpu { r1, p };
        return Ok(o);
    }
    let Some(pl) = plan(info.n, dst_phase) else {
        return Ok(o);
    };
    o.plan = Some(pl);
    let n = info.n as usize;
    if src.len() < n || dst.len() < n {
        return Err(Error::ShortBuffer);
    }
    let cf = ecx as f32;
    for i in 0..n {
        let vector = i >= pl.peel && i < pl.vend;
        dst[i] = ramp(cf, p, r1, src[i], if vector { ramp_max } else { ONE });
    }
    Ok(o)
}

/// The body as shipped: native sincos port, vector clamp maximum = stand-in PX2-A1.
pub fn perspective_alpha(par: &Params, info: &Info, flag: bool, src: &[f32], dst: &mut [f32], dst_phase: usize) -> Result<Outcome, Error> {
    perspective_alpha_with(&sincos_native, RAMP_MAX_STANDIN, par, info, flag, src, dst, dst_phase)
}

use fx_common::x86::{nx_fma, nx_mul};

pub struct Assumption {
    pub id: &'static str,
    pub va: &'static str,
    pub value: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "PX2-A1", va: ", 32 B", value: "vector clamp maximum 1.0f x8 (same cell and value as SC2-A1); T2 also varies it (0.75f) on both sides", basis: "NOT captured (lookup.py U32); stand-in on both sides" },
        Assumption { id: "PX2-A2", va: ", 768 B;, 3,840 B", value: "INHERITED from (crate libm_sincos): SC1-A1 sincosf table entries 16..63 and SC1-A2 double table entries 4..63, ASSUMED-SEMANTIC; this crate holds no table (src/sincos.rs retired)", basis: "NOT captured README assumptions 1 and 2; the emulator image of T2 carries the same stand-in data files" },
        Assumption { id: "PX2-A3", va: ", bytes 4..16", value: "only 4 of 16 bytes captured; the read is a 4 byte vmulss, so the value is MEASURED (2pi)", basis: ", lookup.py N4/U12" },
        Assumption { id: "PX2-A4", va: "", value: "finite |theta| >= 8192 (|params_b| > 5215.19; the earlier note said 1303 and forgot the factor 0.25 * 2pi = 1.5708) runs the double sincos and the reducer, ported inside libm_sincos and executed in the emulator: covered by T2 (RERUN_SC1.md)", basis: "" },
        Assumption { id: "PX2-A5", va: "", value: "flag != 0: the tail call (ramp route) and the flag path of (jump to, fill route) are not ported; arguments are compared by the call-record check only", basis: "" },
        Assumption { id: "PX2-A6", va: "", value: "the flag 0 fill ends in (tail jump) or (call, when != 0): both modelled as memset(dst, 0, bytes) in the emulator and as a zero fill in the port (same as HX-A1 in the blur crate)", basis: "; callee bodies not read" },
        Assumption { id: "PX2-A7", va: "", value: "feature word 0x9d97ff selects the AVX2 body (dispatcher not ported)", basis: "" },
        Assumption { id: "PX2-A8", va: " (target of the pointer cell)", value: "the dword read by is zero (pointer cell, captured zero range) so the fill takes the tail jump to; the choice does not change the zero fill", basis: "MEASURED (lookup.py Z8); only the memset model of PX2-A6 is assumed" },
    ]
}
