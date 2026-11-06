//! Port of the second warp sampler F2, AVX2 body of dispatcher (listing addresses).
//! Source of truth: neutral_1.asm (listing), see SPEC.md. Silicon NaN rule: plain Rust arithmetic, `mul_add`
//! exactly where the listing fuses. Engine statement: the emulator follows the SDM, not silicon.
//! ROUTE AVX2-ONLY. HOST-FP-ASSUMED (round to nearest, no FTZ, no DAZ).
//! Reused natives (read-only path dependencies, no private copies): warp_f1 (F1), bwb_libm (fmod).
//! Labelled copies of the pure helpers of the finished slot 53 driver (warp_driver, same arithmetic, same listing shapes):
//! `cvtt`, `weights`, the wrap/mirror rule and the blend formulas.

use bwb_libm::fmod_x;
use warp_f1::{sample, Call, F1Error, Lattice, Params as F1Params};

pub const ONE: f32 = 1.0; //  (captured)
pub const HALF: f32 = 0.5; //  (captured)
pub const NEG_ONE: f32 = -1.0; //  (captured)
pub const DIM_EPS: f64 = 1.01; //  (captured, bytes 295c8fc2f528f03f)
/// Largest plane count of the blend path that does not destroy the return address (temp row at rbp upward,
/// frame words above it; INFERRED for the entry alignment where rbx = A+16). Above 8 the saved rbp at [rbp] is overwritten.
pub const MAX_N_SAFE: i32 = 8;
pub const MAX_N_BLEND: i32 = 18;
/// Phase input: bytes of the temp row address modulo 32. The temp row is at rbp and rbp = A-16 with A 32-aligned
/// by the body's own `and rsp,-32`so the phase is 16, not a caller buffer address (INFERRED from).
pub const TEMP_PHASE_BYTES: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub enum Rec {
    ///  (assert helper) recorded no-op
    AssertCellA,
    /// (xmm0 = x f64 bits, xmm1 = y f64 bits)
    Fmod { x: u64, y: u64 },
    /// (r15, 0, 0)
    AccCellA,
    /// (r15, 0)
    AccCellB,
    /// esi = col, ecx = mx, r8d = my, r9d = i, stack = j, xmm0..3 bits
    F1 { col: i32, mx: i32, my: i32, i: i32, j: i32, x: u32, y: u32, a: u32, b: u32 },
    /// empty image path: (rdi = frame desc, rsi = this) for plane k
    DescCellA { k: i32 },
    /// empty image path: (rdi = planes[k], esi = 0, desc by value, xmm0 = this+4k)
    Fill { k: i32, value: u32 },
    /// blend path with 9 <= n <= 18: the temp row overwrites the saved rbp at [rbp] (outside the original's intended contract)
    FrameSmash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F2Error {
    IdivFault,
    Oob,
    /// plane count above MAX_N_BLEND on the blend path: the original overwrites its return address
    StackSmash,
    F1(F1Error),
}

/// Warp object (rdi, r14) fields read by F2.
#[derive(Clone, Debug)]
pub struct Obj {
    pub count: i32,  //
    pub aspect_block_d: f32,    //
    pub sub_x: i32,  // [[this] field]
    pub sub_y: i32,  // [[this]]
    pub port_obj_a: i32,
    pub port_obj_b: i32,
    pub assumption_c: i32,
    pub assumption_d: i32,
    pub assumption_f: f32,
    pub assumption_g: f32,
    pub fill: Vec<f32>, // this+4k, used by the empty image path
}

/// Where the getters find the buffers (byte [S]).
pub enum Host {
    /// byte == 0: pointer = lattice.bufs[i][j] + (n + n*(wt[i]+3)) floats
    Mode0,
    /// byte != 0: pointer = [S] + 4*(([S]*e8[j] + e0[i]) * [S]); also calls the assert helper
    Mode1 { buf: Vec<f32>, c0: i32, d0: i32, e0: Vec<i32>, e8: Vec<i32> },
}

/// Image context (rdx = r15).
pub struct Image {
    pub f1: F1Params,
    pub w: i32,
    pub h: i32,
    pub d_cell_d: i32,
    pub d_cell_e: i32,
    pub host: Host,
}

pub struct Args<'a> {
    pub a1: &'a [f32],
    pub a2: &'a [f32],
    pub const_coords: bool, // r9d != 0
    pub c1: &'a [f32],      // [rbx]
    pub c2: &'a [f32],      // [rbx]
    pub flag: i32,          // [rbx]
    pub mx: i32,            // [rbx]
    pub my: i32,            // [rbx]
}

pub struct Assumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "FX2-A1", basis: "ROUTE AVX2-ONLY: dispatcher tests mask 0x9d97ff (and r10,0x9d97ff; cmp; jne) and jumps to; mask 0x117ff goes to the SSE body, bit 0 to; neither twin is ported", label: "MEASURED listing" },
        Assumption { id: "FX2-A2", basis: " (assert helper) is a recorded no-op, as in warp_driver; reached when flag != 0 and not (d_cell_d > 1 and d_cell_e > 1), and from when the host byte is nonzero", label: "ASSUMED, CALLEE-MODELLED" },
        Assumption { id: "FX2-A3", basis: " (16 B uncovered, rdi of the assert call) is only an assert argument; the port never reads it; zero stand-in on both sides would be ARBITRARY and changes no output (no bit of F2's result depends on it)", label: "ARBITRARY, not read" },
        Assumption { id: "FX2-A4", basis: "temp row phase: rbp is 16 mod 32 (frame self-aligned), so the scalar head of the aligned ymm loop is 4 floats for n >= 29; n >= 29 is unreachable without destroying the return address", label: "INFERRED, ALIGN-DEPENDENT (body's own frame)" },
        Assumption { id: "FX2-A5", basis: "blend path n in 9..=18 overwrites the saved rbp (and for n >= 19 the return address); the port reports FrameSmash for 9..=18 and Err(StackSmash) above", label: "INFERRED" },
        Assumption { id: "FX2-A6", basis: "idiv by zero or INT_MIN/-1 in mirror mode faults in the original; the port returns IdivFault", label: "MEASURED listing" },
        Assumption { id: "FX2-A7", basis: "FMA sites: vfmadd231ss, vfnmsub213ss, vfmadd231ss, (8) and vfmadd231ps; no rcp/rsqrt/x87 anywhere in ", label: "MEASURED listing (mnemonic count)" },
        Assumption { id: "FX2-A8", basis: "fmod is the finished bwb_libm native; the emulator hook uses Python math.fmod semantics (as warp_driver)", label: "HOST-HOOKED" },
        Assumption { id: "FX2-A9", basis: "F1 is the finished warp_f1 native (contract n <= 4, host mode 0); the empty image fill and the assert helper are recorded only (no native exists)", label: "CALLEE-MODELLED (fills not rebuilt)" },
        Assumption { id: "FX2-A10", basis: "getters ported for both host modes from the; mode 0 buffers come from the explicit Lattice", label: "MEASURED listing" },
    ]
}

/// cvttss2si: NaN and out of range give 0x80000000.
pub fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

fn minss(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn idiv_q(a: i32, d: i32) -> Result<i32, F2Error> {
    if d == 0 || (a == i32::MIN && d == -1) {
        return Err(F2Error::IdivFault);
    }
    Ok(a / d)
}

fn fm(recs: &mut Vec<Rec>, v: f32, dim: i32) -> f32 {
    let wf = dim as f32;
    let neg = 0.0f32 > v;
    recs.push(Rec::Fmod { x: (v as f64).to_bits(), y: (wf as f64).to_bits() });
    let r = fmod_x(v as f64, wf as f64);
    let r = if neg { (wf.abs() as f64) + r } else { r };
    r as f32
}

/// Edge wrap of one coordinate (x:; y:). mode 1 wrap, mode 2 mirror, else unchanged.
pub fn wrap(recs: &mut Vec<Rec>, v: f32, dim: i32, mode: i32) -> Result<f32, F2Error> {
    match mode {
        1 => {
            if dim == 0 {
                return Ok(0.0);
            }
            Ok(fm(recs, v, dim))
        }
        2 => {
            let q = idiv_q(cvtt(v), dim)?;
            let par = q & 1;
            let negb = !(v >= 0.0);
            let refl = if negb { par == 0 } else { par != 0 };
            let wf = dim as f32;
            if refl {
                let r = fm(recs, v, dim);
                Ok(wf - r)
            } else {
                Ok(fm(recs, v, dim))
            }
        }
        _ => Ok(v),
    }
}

/// Scales: (scx = [rbp], scy = [rbp]) from .
pub fn scales(o: &Obj) -> (f32, f32) {
    let x13 = o.assumption_g - o.assumption_f;
    (HALF * x13, (HALF * o.aspect_block_d) * x13)
}

/// X before the wrap : fma(a1, scx, (f32)(i - 1 + (port_obj_b - ox)) - (f32)(assumption_c - ox)). `i` is the 1-based pixel.
pub fn x_raw(o: &Obj, scx: f32, a1: f32, i: i32) -> f32 {
    let edx = o.port_obj_b.wrapping_sub(o.sub_x);
    let edi = o.assumption_c.wrapping_sub(o.sub_x);
    let r8 = i.wrapping_add(edx).wrapping_sub(1);
    let base = (r8 as f32) - (edi as f32);
    a1.mul_add(scx, base) // vfmadd231ss 
}

/// Y before the wrap : fnmsub(scy, a2, (f32)(assumption_d - oy)) + (f32)(port_obj_a - oy).
pub fn y_raw(o: &Obj, scy: f32, a2: f32) -> f32 {
    let cy = o.assumption_d.wrapping_sub(o.sub_y) as f32;
    let dy = o.port_obj_a.wrapping_sub(o.sub_y) as f32;
    let t = (-scy).mul_add(a2, -cy); // vfnmsub213ss : -(scy*a2) - cy
    t + dy
}

fn clamp_dir(c: f32, dim: i32) -> f32 {
    let d = (dim as f64) - DIM_EPS;
    let cd = c as f64;
    let m = if d < cd { d } else { cd }; // vminsd: src1 < src2 ? src1 : src2
    m as f32
}

/// Blend weights : (w0, wn, ws, wd)
pub fn weights(fx: f32, fy: f32) -> (f32, f32, f32, f32) {
    let t = fx * fy;
    let w0 = ((ONE - fx) - fy) + t;
    let wn = fx - t;
    let ws = fy - t;
    (w0, wn, ws, t)
}

fn blend1(w: (f32, f32, f32, f32), r0: f32, rn: f32, rs: f32, rd: f32, vec: bool) -> f32 {
    let (w0, wn, ws, wd) = w;
    if vec {
        // ymm, xmm 
        let v = wn * rn;
        let v = w0.mul_add(r0, v);
        let v = wd.mul_add(rd, v);
        ws.mul_add(rs, v)
    } else {
        // scalar 
        let s = wd * rd;
        let t = ws * rs;
        let s = wn.mul_add(rn, s);
        let t = w0.mul_add(r0, t);
        s + t
    }
}

/// Which formula channel j uses for plane count n: true = packed. `head` = scalar head count for n >= 29 (default 4, FX2-A4).
pub fn packed_flags(n: usize, head: usize) -> Vec<bool> {
    let mut f = vec![false; n];
    if n < 16 {
        return f;
    }
    let head = if n < 29 { 0 } else { head };
    if n < head + 16 {
        return f;
    }
    let nblk = (n - head) / 16;
    let edx = head + nblk * 16;
    for j in head..edx {
        f[j] = true;
    }
    let m = n - edx;
    if m >= 4 {
        for j in edx..edx + (m & !3) {
            f[j] = true;
        }
    }
    f
}

/// Head count from the temp row phase (bytes mod 32): (32 - phase) / 4 when phase != 0, else 0.
pub fn head_from_phase(phase_bytes: usize) -> usize {
    let p = phase_bytes & 31;
    if p == 0 {
        0
    } else {
        (32 - p) / 4
    }
}

/// Getter (S, i, j): (buffer index source, offset in floats). Records nothing (the caller records).
fn acquire<'a>(img: &'a Image, lat: &'a Lattice, i: i32, j: i32) -> Result<(&'a [f32], i64), F2Error> {
    match &img.host {
        Host::Mode0 => {
            let w = *img.f1.width_tab.get(usize::try_from(i).map_err(|_| F2Error::Oob)?).ok_or(F2Error::Oob)?;
            let base = (img.f1.n.wrapping_add(w.wrapping_add(3).wrapping_mul(img.f1.n))) as i64;
            let row = lat.bufs.get(usize::try_from(i).map_err(|_| F2Error::Oob)?).ok_or(F2Error::Oob)?;
            let buf = row.get(usize::try_from(j).map_err(|_| F2Error::Oob)?).ok_or(F2Error::Oob)?;
            Ok((buf.as_slice(), base))
        }
        Host::Mode1 { buf, c0, d0, e0, e8 } => {
            let ej = *e8.get(usize::try_from(j).map_err(|_| F2Error::Oob)?).ok_or(F2Error::Oob)?;
            let ei = *e0.get(usize::try_from(i).map_err(|_| F2Error::Oob)?).ok_or(F2Error::Oob)?;
            let r8 = c0.wrapping_mul(ej).wrapping_add(ei).wrapping_mul(*d0);
            Ok((buf.as_slice(), r8 as i64))
        }
    }
}

/// Getter (S, i): (width_tab[i] + 3) * n, 32 bit wrapping.
fn stride(img: &Image, i: i32) -> Result<i32, F2Error> {
    let w = *img.f1.width_tab.get(usize::try_from(i).map_err(|_| F2Error::Oob)?).ok_or(F2Error::Oob)?;
    Ok(w.wrapping_add(3).wrapping_mul(img.f1.n))
}

/// Whole body (after the dispatcher). `phase_bytes` is the temp row phase input (default TEMP_PHASE_BYTES).
pub fn run(o: &Obj, img: &mut Image, lat: &Lattice, a: &Args, out: &mut [Vec<f32>], phase_bytes: usize) -> Result<Vec<Rec>, F2Error> {
    let mut recs = Vec::new();
    let n = img.f1.n;
    let wh = img.w.wrapping_mul(img.h);
    if wh == 0 {
        // for each plane then 
        for k in 0..n.max(0) {
            recs.push(Rec::DescCellA { k });
            let v = o.fill.get(k as usize).copied().ok_or(F2Error::Oob)?;
            recs.push(Rec::Fill { k, value: v.to_bits() });
        }
        return Ok(recs);
    }
    if a.flag != 0 && !(img.d_cell_d > 1 && img.d_cell_e > 1) {
        recs.push(Rec::AssertCellA);
    }
    img.f1.flag_x = a.mx;
    img.f1.flag_y = a.my;
    let (scx, scy) = scales(o);
    let (mut x0, mut y0) = (0.0f32, 0.0f32); // [rbp], [rbp]
    let at = |v: &[f32], i: usize| v.get(i).copied().ok_or(F2Error::Oob);
    let packed = packed_flags(n.max(0) as usize, head_from_phase(phase_bytes));
    let mut smashed = false;
    for r12 in 1..=(o.count as i64) {
        let pix = (r12 - 1) as usize;
        if !a.const_coords || r12 == 1 {
            x0 = at(a.a1, pix)?;
            y0 = at(a.a2, pix)?;
        }
        let xr = x_raw(o, scx, x0, r12 as i32);
        let x = wrap(&mut recs, xr, img.w, a.mx)?;
        let yr = y_raw(o, scy, y0);
        let y = wrap(&mut recs, yr, img.h, a.my)?;
        if a.flag != 0 {
            let c1 = at(a.c1, pix)?;
            let c2 = at(a.c2, pix)?;
            let c1pos = 0.0f32 < c1 || c1.is_nan();
            let c2pos = 0.0f32 < c2 || c2.is_nan();
            if c1pos || c2pos {
                let p = if c1pos {
                    clamp_dir(c1, img.d_cell_d)
                } else if 0.0f32 > c1 {
                    0.0
                } else {
                    clamp_dir(c1, img.d_cell_d)
                };
                let q = if 0.0f32 <= c2 || c2.is_nan() { clamp_dir(c2, img.d_cell_e) } else { 0.0 };
                let ip = cvtt(p);
                let jq = cvtt(q);
                let fa = p - (ip as f32);
                let fb = q - (jq as f32);
                recs.push(Rec::F1 { col: pix as i32, mx: a.mx, my: a.my, i: ip, j: jq, x: x.to_bits(), y: y.to_bits(), a: fa.to_bits(), b: fb.to_bits() });
                let call = Call { x, y, a: fa, b: fb, col: pix as i32, i: ip, j: jq };
                let mut refs: Vec<&mut [f32]> = out.iter_mut().map(|v| v.as_mut_slice()).collect();
                sample(&img.f1, lat, &call, &mut refs).map_err(F2Error::F1)?;
                continue;
            }
        }
        // normal path 
        let wt0 = img.f1.width_tab.first().copied().ok_or(F2Error::Oob)?;
        let ht0 = img.f1.height_tab.first().copied().ok_or(F2Error::Oob)?;
        let xc = if a.mx == 0 {
            if NEG_ONE > x {
                NEG_ONE
            } else {
                minss(wt0 as f32, x)
            }
        } else {
            x
        };
        let yc = if a.my == 0 {
            if NEG_ONE > y {
                NEG_ONE
            } else {
                minss(ht0 as f32, y)
            }
        } else {
            y
        };
        let sx = ONE + xc;
        let sy = ONE + yc;
        let ix = cvtt(sx);
        let iy = cvtt(sy);
        let fx = sx - (ix as f32);
        let fy = sy - (iy as f32);
        recs.push(Rec::AccCellA);
        let (buf, off0) = acquire(img, lat, 0, 0)?;
        recs.push(Rec::AccCellB);
        if !matches!(img.host, Host::Mode0) {
            recs.push(Rec::AssertCellA);
        }
        let stride = stride(img, 0)?;
        let ecx = (ix.wrapping_sub(1)).wrapping_mul(n);
        let r14d = (iy.wrapping_sub(1)).wrapping_mul(stride);
        let base = off0 + (ecx as i64) + (r14d as i64);
        if n <= 0 {
            continue;
        }
        if n > MAX_N_BLEND {
            return Err(F2Error::StackSmash);
        }
        if n > MAX_N_SAFE && !smashed {
            smashed = true;
            recs.push(Rec::FrameSmash);
        }
        let w = weights(fx, fy);
        let get = |k: i64| -> Result<f32, F2Error> {
            if k < 0 {
                return Err(F2Error::Oob);
            }
            buf.get(k as usize).copied().ok_or(F2Error::Oob)
        };
        let mut temp = vec![0.0f32; n as usize];
        for j in 0..(n as usize) {
            let jj = j as i64;
            let r0 = get(base + jj)?;
            let rn = get(base + n as i64 + jj)?;
            let rs = get(base + stride as i64 + jj)?;
            let rd = get(base + n as i64 + stride as i64 + jj)?;
            temp[j] = blend1(w, r0, rn, rs, rd, packed[j]);
        }
        for (c, t) in temp.iter().enumerate() {
            let pl = out.get_mut(c).ok_or(F2Error::Oob)?;
            *pl.get_mut(pix).ok_or(F2Error::Oob)? = *t;
        }
    }
    Ok(recs)
}
