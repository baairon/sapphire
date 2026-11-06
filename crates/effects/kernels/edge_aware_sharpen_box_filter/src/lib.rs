//! Std-only port of the pass jobs of the shared blur core of edge_aware_sharpen (route: AVX2 tier, callback pair).
//!
//! Original code (listing addresses of neutral_1.asm):
//!   * pass job x (dispatcher) -> AVX2 body : one partitioned row pass; each row is the variable window box filter `filter3` over the source row
//!     `[ctx]`driven by the guide row `[ctx]`written to `[ctx]`. The filter is inlined in the job when the callback global equals .
//!   * pass job y -> AVX2 body : the same filter along columns; columns are processed in blocks of 64 through 128 scratch slices
//!     (64 input columns, 64 output columns), the next block's input being loaded while the current block's output is written back; the guide is column major.
//!   * callback (global): the three filter parameters from the radius with the libm floor (roundss tier).
//!   * callback (global, body): the filter itself (judged standalone as `filter3`).
//!   * exp2 helper (AVX tier, no FMA) and (FMA tier), 8 lanes, per-lane slow path when |x| > 126.0 or NaN.
//! Memory is one f32 arena; pointers are element indices, so every aliasing the original allows (source = destination, guide = destination) behaves as there.
//! NOT ported (refused): the filter variant with a fourth parameter (job path taken when !=), the SSE tiers.

pub mod exp2tab;
use exp2tab::*;

/// libm floorf on the roundss tier (feature word & ==): `roundss xmm0, xmm0, 1` (round down); a NaN comes back quieted.
pub fn floorf_roundss(x: f32) -> f32 {
    if x.is_nan() {
        f32::from_bits(x.to_bits() | 0x0040_0000)
    } else {
        x.floor()
    }
}

/// Callback (rdi = output record, xmm0 = radius): {1.0 + floor(r), 3.0, r - floor(r)}.
pub fn cb_cell_a(radius: f32) -> [f32; 3] {
    let f = floorf_roundss(radius);
    [1.0f32 + f, f32::from_bits(0x4040_0000), radius - f]
}

/// What the original's calls are recorded as (the test compares them with the emulator's hook records).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rec {
    /// number of assert-fail calls : source line equal to destination line
    pub asserts: u32,
    /// first argument of the partition service (rows for the x job, columns for the y job)
    pub part_total: i32,
    /// number of scratch slice requests (128 for the y job, 0 for the x job)
    pub slice_calls: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Job {
    pub w: i32,
    pub h: i32,
    pub radius: f32,
    /// arena element indices of ctx
    pub src: usize,
    pub dst: usize,
    pub aux: usize,
}

/// The variable window box filter of (and of the inlined copies in both jobs). `a` values, `b` guide, `out` result, `n` elements.
pub fn filter3(mem: &mut [f32], a: usize, out: usize, b: usize, n: i64, p: [f32; 3], rec: &mut Rec) {
    if a == out {
        rec.asserts += 1;
    }
    if n <= 0 {
        return;
    }
    let one = 1.0f32;
    let two = 2.0f32;
    let (r, p1, p2) = (p[0], p[1], p[2]);
    let rm1 = r - one;
    let mut lo: i64 = 0;
    let mut hi: i64 = 1;
    let mut cnt: i32 = 1;
    let mut sum = mem[a];
    for i in 0..n {
        let g = mem[b + i as usize];
        let lo_thr = g - r;
        let hi_thr = r + g;
        while lo < i && lo_thr > mem[b + lo as usize] {
            cnt = cnt.wrapping_sub(1);
            sum -= mem[a + lo as usize];
            lo += 1;
        }
        let mut h = hi;
        while h < n && hi_thr >= mem[b + h as usize] {
            cnt = cnt.wrapping_add(1);
            sum += mem[a + h as usize];
            h += 1;
        }
        hi = h;
        let v9;
        if p1 > one {
            let mut v = mem[a + i as usize];
            if r > two {
                let mut s8 = sum;
                let mut c8 = cnt;
                let t = g - rm1;
                if t > mem[b + lo as usize] {
                    s8 = sum - mem[a + lo as usize];
                    c8 = cnt.wrapping_sub(1);
                }
                let t2 = mem[b + (h - 1) as usize];
                let u = rm1 + g;
                if t2 > u {
                    c8 = c8.wrapping_sub(1);
                    s8 -= mem[a + (h - 1) as usize];
                }
                if c8 > 0 {
                    v = s8 / (c8 as f32);
                }
            }
            let mean = sum / (cnt as f32);
            let d = mean - v;
            v9 = d.mul_add(p2, v);
        } else {
            v9 = sum / (cnt as f32);
        }
        mem[out + i as usize] = v9;
    }
}

/// Pass job x . `startcount`: the partition service's answer for `rows = job.h`.
pub fn job_x(mem: &mut [f32], job: &Job, start: i32, count: i32, rec: &mut Rec) {
    rec.part_total = job.h;
    let p = cb_cell_a(job.radius);
    if count <= 0 {
        return;
    }
    let off = job.w.wrapping_mul(start) as i64;
    let w = job.w as i64;
    for b in 0..count as i64 {
        let o = (off + b * w) as usize;
        filter3(mem, job.src + o, job.dst + o, job.aux + o, w, p, rec);
    }
}

/// Pass job y . `startcount`: the partition service's answer for `columns = job.w`. The arena is extended by the 128 scratch slices
/// during the call and truncated afterwards.
pub fn job_y(mem: &mut Vec<f32>, job: &Job, start: i32, count: i32, rec: &mut Rec) {
    rec.part_total = job.w;
    rec.slice_calls = 128;
    let p = cb_cell_a(job.radius);
    if count <= 0 || job.h <= 0 {
        return;
    }
    let h = job.h as usize;
    let w = job.w as i64;
    let sbase = mem.len();
    mem.resize(sbase + 128 * h, 0.0);
    let ins = |c: usize| sbase + c * h; // input slice c (0..64)
    let outs = |c: usize| sbase + (64 + c) * h; // output slice c
    let aux_base = job.aux as i64 + (job.h.wrapping_mul(start)) as i64;
    let src_base = job.src as i64 + start as i64;
    let dst_base = job.dst as i64 + start as i64;
    let mut k: i64 = 0;
    loop {
        let ecx = (count as i64 - 64 * k).min(64);
        let edx = (count as i64 - 64 * (k + 1)).min(64);
        if k == 0 {
            for row in 0..h as i64 {
                for c in 0..ecx {
                    mem[ins(c as usize) + row as usize] = mem[(src_base + row * w + c) as usize];
                }
            }
        }
        for c in 0..ecx {
            let bptr = (aux_base + (64 * k + c) * job.h as i64) as usize;
            filter3(mem, ins(c as usize), outs(c as usize), bptr, job.h as i64, p, rec);
        }
        for row in 0..h as i64 {
            for c in 0..ecx {
                let v = mem[outs(c as usize) + row as usize];
                mem[(dst_base + 64 * k + c + row * w) as usize] = v;
            }
            for c in 0..edx.max(0) {
                let v = mem[(src_base + 64 * k + 64 + c + row * w) as usize];
                mem[ins(c as usize) + row as usize] = v;
            }
        }
        if (count as i64) > 64 * (k + 1) {
            k += 1;
        } else {
            break;
        }
    }
    mem.truncate(sbase);
}

/// exp2 helper, one lane. `fma`: variant (vfmadd213ps Horner) or (vmulps + vaddps). Err(bits): the lane takes the slow path (|x| > 126.0 or NaN,
/// compared as a signed integer), where the original calls .
pub fn exp2_lane(x: f32, fma: bool) -> Result<f32, u32> {
    let shifter = f32::from_bits(SHIFTER);
    let t = x + shifter;
    let k = t - shifter;
    let r = x - k;
    let mut p = f32::from_bits(C6);
    for c in [C5, C4, C3, C2, C1, C0] {
        p = if fma { p.mul_add(r, f32::from_bits(c)) } else { r * p + f32::from_bits(c) };
    }
    let ab = x.to_bits() & ABSMASK;
    if (ab as i32) > (LIMIT as i32) {
        return Err(x.to_bits());
    }
    Ok(f32::from_bits(p.to_bits().wrapping_add(t.to_bits() << 23)))
}
