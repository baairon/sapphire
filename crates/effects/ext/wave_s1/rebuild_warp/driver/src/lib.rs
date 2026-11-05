//! Port of the warp pixel driver, AVX2 body of (listing addresses).
//! Source of truth: neutral_1.asm (listing), see SPEC.md. Silicon NaN rule: plain Rust arithmetic, `mul_add`
//! exactly where the listing fuses. Engine statement: the emulator follows the SDM, not silicon.
//! Not ported: the AVX and SSE twins, the plane-fill kernel (recorded only), the guard 
//! (recorded only), the context constructor in slot 53.

use bwb_libm::fmod_x;
use warp_f1::{sample, Call, F1Error, Lattice, Params as F1Params};

pub const ONE: f32 = 1.0; //  (captured)
pub const HALF: f32 = 0.5; // 
pub const NEG_ONE: f32 = -1.0; //  (captured, bytes 000080bf)
pub const DIM_EPS: f64 = 1.01; //  (captured, bytes 295c8fc2f528f03f)
/// Head count of the scalar prologue of the aligned ymm loop: (32 - ((rbp) & 31)) / 4 with rbp = 8 mod 32.
pub const HEAD: usize = 6;
/// Largest plane count whose temp row (rbp upward) stays below the saved frame words at [rbp].
pub const MAX_N_BLEND: i32 = 10;

/// Call records, in order.
#[derive(Clone, Debug, PartialEq)]
pub enum Rec {
    ///  guard (line), no-op stand-in A2
    AssertCellA,
    /// (xmm0 = x f64 bits, xmm1 = y f64 bits)
    Fmod { x: u64, y: u64 },
    /// (r15, 0, 0)
    AccCellA,
    /// (r15, 0)
    AccCellB,
    /// rdi = plane array, esi = col, rdx = r15, ecx = mx, r8d = my, r9d = i, stack = j, xmm0..3
    F1 { col: i32, mx: i32, my: i32, i: i32, j: i32, x: u32, y: u32, a: u32, b: u32 },
    /// empty image path: then for plane k
    Fill { k: i32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriverError {
    /// idiv fault (mirror mode with a zero or overflowing dimension)
    IdivFault,
    /// a read outside the supplied lattice buffer or coordinate arrays
    Oob,
    /// plane count above MAX_N_BLEND on the blend path: the original overwrites its own frame
    StackSmash,
    F1(F1Error),
}

/// Warp object (rdi) fields.
#[derive(Clone, Copy, Debug)]
pub struct Obj {
    pub count: i32, //
    pub aspect_block_d: f32,   //
    pub assumption_e: f32,   //
    pub assumption_c: i32,
    pub assumption_d: i32,
    pub assumption_f: f32,
    pub assumption_g: f32,
    pub assumption_h: f32,
}

/// Image context (rdx = r15): F1 parameters (n at, tables at) plus the driver's own fields.
pub struct Image {
    pub f1: F1Params,
    pub w: i32,   //
    pub h: i32,   //
    pub d_cell_d: i32, //
    pub d_cell_e: i32, //
}

/// Entry arguments other than the object and the image.
pub struct Args<'a> {
    pub a1: &'a [f32],   // rcx
    pub a2: &'a [f32],   // r8
    pub const_coords: bool, // r9d != 0: coordinates only computed for pixel 1
    pub c1: &'a [f32],   // [rbx]
    pub c2: &'a [f32],   // [rbx]
    pub flag: i32,       // [rbx]
    pub mx: i32,         // [rbx]
    pub my: i32,         // [rbx]
}

pub struct Assumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "A1", basis: "F1 channel count n is the image plane count [r15], passed implicitly through rdx = r15; the driver never passes a separate count", label: "MEASURED (rdx = r15)" },
        Assumption { id: "A2", basis: " guard (line) is a recorded no-op", label: "ASSUMPTION" },
        Assumption { id: "A3", basis: " in host mode 0 (byte == 0): data = bufs[0][0] + (n + n*(wt[0]+3)) floats, stride = (wt[0]+3)*n ", label: "MEASURED listing" },
        Assumption { id: "A4", basis: " (16 B uncovered) is only the assert expression string used; no stand-in is read on the AVX2 route", label: "INFERRED" },
        Assumption { id: "A5", basis: "temp row rbp is 8 mod 32 (rsp is and-ed to 32 before two pushes), so the ymm prologue head is 6 floats for n >= 29", label: "INFERRED from the prologue " },
        Assumption { id: "A6", basis: "n > 10 on the blend path overwrites the saved frame words at [rbp] (temp row spans rbp upward); the port returns StackSmash", label: "INFERRED" },
        Assumption { id: "A7", basis: "idiv by zero or INT_MIN / -1 in mirror mode faults in the original; the port returns IdivFault", label: "MEASURED " },
        Assumption { id: "A8", basis: "FMA sites in the AVX2 body: vfmadd132ss, vfmsub213ss, vfmadd231ss, vfmadd231ps; no rcp/rsqrt/x87", label: "MEASURED listing" },
        Assumption { id: "A9", basis: "fmod modelled by bwb_libm::fmod_x; the emulator hook uses Python math.fmod semantics (as BW5A)", label: "ASSUMPTION" },
    ]
}

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

fn idiv_q(a: i32, d: i32) -> Result<i32, DriverError> {
    if d == 0 || (a == i32::MIN && d == -1) {
        return Err(DriverError::IdivFault);
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

/// S3/S4 edge wrap of one coordinate (for x, for y)
pub fn wrap(recs: &mut Vec<Rec>, v: f32, dim: i32, mode: i32) -> Result<f32, DriverError> {
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

/// S2 scalars: (scx = [rbp], scy = [rbp])
pub fn scales(o: &Obj) -> (f32, f32) {
    let x13 = o.assumption_g - o.assumption_f;
    (HALF * x13, (HALF * o.aspect_block_d) * x13)
}

/// S3: x coordinate before the wrap
pub fn x_raw(o: &Obj, scx: f32, a1: f32) -> f32 {
    let t = a1 + ONE;
    let base = o.assumption_f - (o.assumption_c as f32);
    t.mul_add(scx, base) // vfmadd132ss 
}

/// S4: y coordinate before the wrap
pub fn y_raw(o: &Obj, scy: f32, a2: f32) -> f32 {
    let t = a2 - ONE;
    let m = scy.mul_add(t, -o.assumption_h); // vfmsub213ss : t*scy - assumption_h
    let top = o.assumption_e - (o.assumption_d as f32);
    top - m
}

fn clamp_dir(c: f32, dim: i32) -> f32 {
    let d = (dim as f64) - DIM_EPS;
    let cd = c as f64;
    let m = if d < cd { d } else { cd }; // vminsd: src1 < src2 ? src1 : src2
    m as f32
}

/// Blend weights : (w0 for [r14], wn for +n, ws for +stride, wd for the diagonal)
pub fn weights(fx: f32, fy: f32) -> (f32, f32, f32, f32) {
    let t = fx * fy;
    let w0 = ((ONE - fx) - fy) + t;
    let wn = fx - t;
    let ws = fy - t;
    (w0, wn, ws, t)
}

/// One tap set blend for channel j; `vec` selects the packed formula (vfmadd231ps order), otherwise the scalar one.
fn blend1(w: (f32, f32, f32, f32), r0: f32, rn: f32, rs: f32, rd: f32, vec: bool) -> f32 {
    let (w0, wn, ws, wd) = w;
    if vec {
        let v = wn * rn;
        let v = w0.mul_add(r0, v);
        let v = wd.mul_add(rd, v);
        ws.mul_add(rs, v)
    } else {
        let s = wd * rd;
        let t = ws * rs;
        let s = wn.mul_add(rn, s);
        let t = w0.mul_add(r0, t);
        s + t
    }
}

/// Which formula channel j uses for plane count n: true = packed (ymm or xmm), false = scalar.
pub fn packed_flags(n: usize) -> Vec<bool> {
    let mut f = vec![false; n];
    if n < 16 {
        return f;
    }
    let head = if n < 29 { 0 } else { HEAD };
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

pub fn run(o: &Obj, img: &mut Image, lat: &Lattice, a: &Args, out: &mut [Vec<f32>]) -> Result<Vec<Rec>, DriverError> {
    let mut recs = Vec::new();
    let n = img.f1.n;
    let wh = img.w.wrapping_mul(img.h);
    if wh == 0 {
        for k in 0..n.max(0) {
            recs.push(Rec::Fill { k });
        }
        return Ok(recs);
    }
    if a.flag != 0 && !(img.d_cell_d > 1 && img.d_cell_e > 1) {
        recs.push(Rec::AssertCellA);
    }
    img.f1.flag_x = a.mx;
    img.f1.flag_y = a.my;
    let (scx, scy) = scales(o);
    let (mut x, mut y) = (0.0f32, 0.0f32); // [rbp], [rbp]
    let at = |v: &[f32], i: usize| v.get(i).copied().ok_or(DriverError::Oob);
    let packed = packed_flags(n.max(0) as usize);
    for r12 in 1..=(o.count as i64) {
        let pix = (r12 - 1) as usize;
        if !(a.const_coords && r12 != 1) {
            let xr = x_raw(o, scx, at(a.a1, pix)?);
            x = wrap(&mut recs, xr, img.w, a.mx)?;
            let yr = y_raw(o, scy, at(a.a2, pix)?);
            y = wrap(&mut recs, yr, img.h, a.my)?;
        }
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
                sample(&img.f1, lat, &call, &mut refs).map_err(DriverError::F1)?;
                continue;
            }
        }
        // normal path 
        let wt0 = img.f1.width_tab.first().copied().ok_or(DriverError::Oob)?;
        let ht0 = img.f1.height_tab.first().copied().ok_or(DriverError::Oob)?;
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
        recs.push(Rec::AccCellB);
        let stride = (wt0.wrapping_add(3)).wrapping_mul(n);
        let off0 = (n as i64) + (n.wrapping_mul(wt0.wrapping_add(3)) as i64);
        let ecx = (ix.wrapping_sub(1)).wrapping_mul(n);
        let r14d = (iy.wrapping_sub(1)).wrapping_mul(stride);
        let base = off0 + (ecx as i64) + (r14d as i64);
        if n <= 0 {
            continue;
        }
        if n > MAX_N_BLEND {
            return Err(DriverError::StackSmash);
        }
        let buf = lat.bufs.first().and_then(|b| b.first()).ok_or(DriverError::Oob)?;
        let w = weights(fx, fy);
        let get = |k: i64| -> Result<f32, DriverError> {
            if k < 0 {
                return Err(DriverError::Oob);
            }
            buf.get(k as usize).copied().ok_or(DriverError::Oob)
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
            let pl = out.get_mut(c).ok_or(DriverError::Oob)?;
            *pl.get_mut(pix).ok_or(DriverError::Oob)? = *t;
        }
    }
    Ok(recs)
}
