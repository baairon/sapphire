//! scratch std-only port of the shared blur core, AVX2 region, for the route of edge_aware_sharpen only:
//! switch mode 4 (table entry 4), plane-count argument n (stack slot [rbx]) in 0..=5, flag [rbx] == 0 (pool path).
//! Everything else the original does in this region (modes 0..3, the vector radius loops for n >= 8, the job makers for flag != 0) is NOT ported
//! and is refused with an error.
//!
//! Callees are explicit: `Host::powf` (libm), the pool service with the pass jobs (not ported by any
//! piece: recorded), the scratch service and the plane copy . The exp2 helper is ported here (FMA variant
//! cpu indicator 9..11, INFERRED) with constants read from the approved file (exp2tab.rs).

pub mod exp2tab;
use exp2tab::*;

/// Listing addresses of the two pass jobs (dispatchers) started through the pool service.
pub mod judge_keys;
pub use judge_keys::{G0, G1, JOB_X, JOB_Y};
/// Cell (captured, captured bytes = after the loader bias : callback) seen by the scratch service as global 0 and the
/// `lea r10,` value stored in global 1.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreError {
    /// Switch value other than 4 (arms 0..3 and the error path above 4 are not ported).
    ModeNotPorted(i32),
    /// Flag non-zero: the job-maker path is not ported.
    FlagNotPorted(u32),
    /// n above 5 (n >= 8 takes the vector loops, 6 and 7 overlap frame variables).
    NScope(i32),
    /// exp2 argument beyond the fast path limit (the original calls the scalar handler per lane).
    Exp2SlowPath(u32),
}

/// The arm sees these inputs. Pointers are opaque ids (equal ids model aliasing).
#[derive(Debug, Clone, Copy)]
pub struct Args {
    pub this: u64,
    pub mode: i32,
    pub p_in: u64,  // rdx (r14)
    pub p_b: u64,   // rcx (r12)
    pub p_c: u64,   // r8 (r13)
    pub p_d: u64,   // r9, saved on the stack
    pub p_e: u64,   // stack arg 7 [rbx]
    pub roi: [i32; 4], // [rbx): dword 0 is r8d for the scratch service, dword 1 is sign extended into rdx
    pub p_f: u64,   // [rbx]
    pub n: i32,     // [rbx]
    pub flag: u32,  // [rbx] (compared as a dword)
    pub sx: f32,    // xmm0 (x14)
    pub sy: f32,    // xmm1 (x15)
}

pub trait Host {
    fn powf(&mut self, x: f32, y: f32) -> f32;
    /// returns the handle written to [rdi]. `hxhy` are the written frame values (for the record only).
    fn scratch_init(&mut self, this: u64, bytes: i64, ecx: u32, r8d: u32, g0: u64, g1: u64, hx: &[f32], hy: &[f32]) -> u64;
    ///  with job pointer `job`struct fields radius (bits), source, destination, aux.
    fn pool_run(&mut self, this: u64, job: u32, radius: u32, src: u64, dst: u64, aux: u64);
    /// (dst, src, edx = 0, ROI qword 0, ROI qword 1, p_f).
    fn copy(&mut self, dst: u64, src: u64, edx: u32, w0: u64, w1: u64, w2: u64);
    /// (handle) after the two globals were cleared.
    fn scratch_free(&mut self, handle: u64, g0: u64, g1: u64);
}

/// vrsqrtss under APPROX-MODEL: the emulator (Unicorn/QEMU legacy rsqrtss) computes 1 / sqrt(x) with both operations rounded to f32. MEASURED
/// equal on the T2 values; the SDM bound (1.5 * 2^-12) is exercised by `approx_bound` in tests.
pub fn rsqrt_ss(x: f32) -> f32 {
    1.0f32 / x.sqrt()
}

/// exp2 helper (lane 0; all lanes carry the same value in the callers): shifter trick plus a degree 6 polynomial on the fractional part
/// with fused multiply-adds, exponent added as an integer.
pub fn exp2_lane(x: f32) -> Result<f32, CoreError> {
    let shifter = f32::from_bits(SHIFTER);
    let t = x + shifter;
    let k = t - shifter;
    let r = x - k;
    let mut p = f32::from_bits(C6);
    for c in [C5, C4, C3, C2, C1, C0] {
        p = p.mul_add(r, f32::from_bits(c));
    }
    let ab = x.to_bits() & ABSMASK;
    if (ab as i32) > (LIMIT as i32) {
        return Err(CoreError::Exp2SlowPath(x.to_bits()));
    }
    Ok(f32::from_bits(p.to_bits().wrapping_add(t.to_bits() << 23)))
}

/// Radii of the n passes (frame layout of the original: hx and hy in the frame, one value per pass i; both inside one 10 float window).
pub fn radii(sx: f32, sy: f32, pw: f32, n: i32) -> Result<(Vec<f32>, Vec<f32>), CoreError> {
    let x13 = pw - 1.0f32;
    let r = rsqrt_ss(x13);
    let x2 = x13 * r;
    let x5 = r * -0.5f32;
    let x3 = x2 * r;
    let x6 = x3 - 3.0f32;
    let x10 = x5 * x6;
    let cx = (3.0f32 * sx) * x10;
    let cy = (3.0f32 * sy) * x10;
    let mut hx = Vec::new();
    let mut hy = Vec::new();
    for i in 0..n.max(0) {
        let e = exp2_lane((n - 1 - i) as f32)?;
        hy.push(cy * e);
        hx.push(cx * e);
    }
    Ok((hx, hy))
}

pub fn core_mode4<H: Host>(a: &Args, h: &mut H) -> Result<(), CoreError> {
    if a.mode != 4 {
        return Err(CoreError::ModeNotPorted(a.mode));
    }
    if a.flag != 0 {
        return Err(CoreError::FlagNotPorted(a.flag));
    }
    if a.n > 5 {
        return Err(CoreError::NScope(a.n));
    }
    // powf(4.0, (float) n), then the Newton refined 1/sqrt(pw - 1)
    let pw = h.powf(4.0f32, a.n as f32);
    let (hx, hy) = radii(a.sx, a.sy, pw, a.n)?;
    // scratch service, global stores, then the pass loop
    let handle = h.scratch_init(a.this, (a.roi[1] as i64) * 4, 0x80, a.roi[0] as u32, G0, G1, &hx, &hy);
    let mut s = a.p_in; // [rbp], starts as r14
    let mut d = a.p_b; // [rbp], starts as r12
    for i in 0..a.n.max(0) as usize {
        for (rad, job, aux) in [(hx[i], JOB_X, a.p_d), (hy[i], JOB_Y, a.p_e)] {
            if rad > 1.0f32 {
                h.pool_run(a.this, job, rad.to_bits(), s, d, aux);
                // cmp rax, r14; cmovne r9, rax; cmove r8, r12 ; r9 defaults to r13, r8 to the old destination
                let (ns, nd) = if s == a.p_in { (a.p_b, a.p_c) } else { (d, s) };
                s = ns;
                d = nd;
            }
        }
    }
    if a.p_c != s {
        let w0 = (a.roi[0] as u32 as u64) | ((a.roi[1] as u32 as u64) << 32);
        let w1 = (a.roi[2] as u32 as u64) | ((a.roi[3] as u32 as u64) << 32);
        h.copy(a.p_c, s, 0, w0, w1, a.p_f);
    }
    if handle != 0 {
        h.scratch_free(handle, 0, 0);
    }
    Ok(())
}
