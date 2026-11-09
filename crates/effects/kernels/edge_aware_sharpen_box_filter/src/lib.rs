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
use fx_common::nanop::{fma231, has_nan, NanOps};
use fx_common::x86::{nx_add, nx_div, nx_sub};

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
    [1.0f32 + f, f32::from_bits(0x4040_0000), nx_sub(radius, f)]
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

/// One arithmetic step of the filter: the plain operation, or (`RULE`) the operation with the NaN rule of the original instruction.
#[inline(always)]
fn add_r<const RULE: bool>(x: f32, y: f32) -> f32 {
    if RULE { x.fadd(y) } else { x + y }
}

#[inline(always)]
fn sub_r<const RULE: bool>(x: f32, y: f32) -> f32 {
    if RULE { x.fsub(y) } else { x - y }
}

/// `d * p2 + v` fused (`vfmadd231ss` form, destination `v`).
#[inline(always)]
fn fma_r<const RULE: bool>(v: f32, d: f32, p2: f32) -> f32 {
    if RULE { fma231(v, d, p2) } else { d.mul_add(p2, v) }
}

/// The filter over one line of `n` elements held in three distinct slices: values `av`, guide `bv`, result `ov`. With `RULE` false every step is the
/// plain operation, which is bit exact unless a NaN is produced; with `RULE` true every step follows the NaN rule of its instruction.
#[inline(never)]
fn filter_slices<const RULE: bool>(av: &[f32], bv: &[f32], ov: &mut [f32], p: [f32; 3]) {
    let n = av.len();
    let bv = &bv[..n];
    let ov = &mut ov[..n];
    let one = 1.0f32;
    let two = 2.0f32;
    let (r, p1, p2) = (p[0], p[1], p[2]);
    let rm1 = r - one;
    let mut lo: usize = 0;
    let mut hi: usize = 1;
    let mut cnt: i32 = 1;
    let mut sum = av[0];
    for i in 0..n {
        let g = bv[i];
        let lo_thr = nx_sub(g, r);
        let hi_thr = nx_add(r, g);
        while lo < i && lo_thr > bv[lo] {
            cnt = cnt.wrapping_sub(1);
            sum = sub_r::<RULE>(sum, av[lo]);
            lo += 1;
        }
        let mut h = hi;
        while h < n && hi_thr >= bv[h] {
            cnt = cnt.wrapping_add(1);
            sum = add_r::<RULE>(sum, av[h]);
            h += 1;
        }
        hi = h;
        let v9;
        if p1 > one {
            let mut v = av[i];
            if r > two {
                let mut s8 = sum;
                let mut c8 = cnt;
                let t = nx_sub(g, rm1);
                if t > bv[lo] {
                    s8 = sub_r::<RULE>(sum, av[lo]);
                    c8 = cnt.wrapping_sub(1);
                }
                let t2 = bv[h - 1];
                let u = nx_add(rm1, g);
                if t2 > u {
                    c8 = c8.wrapping_sub(1);
                    s8 = sub_r::<RULE>(s8, av[h - 1]);
                }
                if c8 > 0 {
                    v = s8 / (c8 as f32);
                }
            }
            let mean = nx_div(sum, cnt as f32);
            let d = sub_r::<RULE>(mean, v);
            v9 = fma_r::<RULE>(v, d, p2);
        } else {
            v9 = nx_div(sum, cnt as f32);
        }
        ov[i] = v9;
    }
}

/// The line in place on the arena with the NaN rule at every step, for a result range that overlaps the values or the guide: the original's
/// access order, element by element.
#[cold]
#[inline(never)]
fn filter_aliased(mem: &mut [f32], a: usize, out: usize, b: usize, n: i64, p: [f32; 3]) {
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
        let lo_thr = nx_sub(g, r);
        let hi_thr = nx_add(r, g);
        while lo < i && lo_thr > mem[b + lo as usize] {
            cnt = cnt.wrapping_sub(1);
            sum = sum.fsub(mem[a + lo as usize]);
            lo += 1;
        }
        let mut h = hi;
        while h < n && hi_thr >= mem[b + h as usize] {
            cnt = cnt.wrapping_add(1);
            sum = sum.fadd(mem[a + h as usize]);
            h += 1;
        }
        hi = h;
        let v9;
        if p1 > one {
            let mut v = mem[a + i as usize];
            if r > two {
                let mut s8 = sum;
                let mut c8 = cnt;
                let t = nx_sub(g, rm1);
                if t > mem[b + lo as usize] {
                    s8 = sum.fsub(mem[a + lo as usize]);
                    c8 = cnt.wrapping_sub(1);
                }
                let t2 = mem[b + (h - 1) as usize];
                let u = nx_add(rm1, g);
                if t2 > u {
                    c8 = c8.wrapping_sub(1);
                    s8 = s8.fsub(mem[a + (h - 1) as usize]);
                }
                if c8 > 0 {
                    v = s8 / (c8 as f32);
                }
            }
            let mean = nx_div(sum, cnt as f32);
            let d = mean.fsub(v);
            v9 = fma231(v, d, p2);
        } else {
            v9 = nx_div(sum, cnt as f32);
        }
        mem[out + i as usize] = v9;
    }
}

/// The three ranges of a line whose result range overlaps neither the values nor the guide: `(values, guide, result)`.
fn split_line(mem: &mut [f32], a: usize, b: usize, out: usize, n: usize) -> (&[f32], &[f32], &mut [f32]) {
    let (left, rest) = mem.split_at_mut(out);
    let (ov, right) = rest.split_at_mut(n);
    let (left, right): (&[f32], &[f32]) = (left, right);
    let pick = |x: usize| if x + n <= out { &left[x..x + n] } else { &right[x - out - n..x - out] };
    (pick(a), pick(b), ov)
}

/// The variable window box filter of (and of the inlined copies in both jobs). `a` values, `b` guide, `out` result, `n` elements.
///
/// The line is first computed with plain arithmetic. The plain and the rule form of a step agree on whether the result is NaN and on its value
/// when it is not, and no branch of the line reads a sum, so a line whose plain results hold no NaN is bit identical under the NaN rule. A line
/// with a NaN result is recomputed with the rule. That redo needs the inputs intact, so a line whose result range overlaps the values or the
/// guide runs in place with the rule at once.
pub fn filter3(mem: &mut [f32], a: usize, out: usize, b: usize, n: i64, p: [f32; 3], rec: &mut Rec) {
    if a == out {
        rec.asserts += 1;
    }
    if n <= 0 {
        return;
    }
    let nu = n as usize;
    let disjoint = |x: usize| x + nu <= out || out + nu <= x;
    if disjoint(a) && disjoint(b) {
        let (av, bv, ov) = split_line(mem, a, b, out, nu);
        filter_slices::<false>(av, bv, ov, p);
        if has_nan(ov) {
            filter_slices::<true>(av, bv, ov, p);
        }
    } else {
        filter_aliased(mem, a, out, b, n, p);
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

#[cfg(test)]
mod rule_tests {
    use super::*;

    /// The whole line under the NaN rule, in place on the arena, the reference for [`filter3`].
    fn reference(mem: &mut [f32], a: usize, out: usize, b: usize, n: i64, p: [f32; 3]) {
        filter_aliased(mem, a, out, b, n, p);
    }

    #[test]
    fn filter3_equals_the_rule_line() {
        let pool = [
            f32::from_bits(0x7f80_0001),
            f32::from_bits(0xffc0_0004),
            f32::from_bits(0x7fc0_0000),
            f32::INFINITY,
            f32::NEG_INFINITY,
            0.0,
            -0.0,
            1e-40,
            3.0e38,
        ];
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let params = [[1.0f32, 1.0, 0.25], [2.0, 3.0, 0.5], [5.0, 3.0, 0.5], [9.0, 3.0, 0.125], [4.0, 0.5, 0.75]];
        let mut redone = 0usize;
        for round in 0..1500usize {
            let n = [1usize, 2, 5, 17, 64, 300][round % 6];
            let p = params[(round / 6) % params.len()];
            // layout: 0 disjoint, 1 out equals the values, 2 out overlaps the guide, 3 guide equals the values
            let layout = round % 4;
            let (a, b, out) = match layout {
                0 => (0, n + 3, 2 * n + 9),
                1 => (0, n + 3, 0),
                2 => (0, n + 3, n + 3 + n / 2),
                _ => (n + 1, n + 1, 3 * n + 4),
            };
            let len = 4 * n + 16;
            let density = [0u64, 400, 40, 5][(round / 24) % 4];
            let mem: Vec<f32> = (0..len)
                .map(|_| {
                    let r = next();
                    if density != 0 && r % density == 0 { pool[(r >> 8) as usize % pool.len()] } else { ((r >> 16) % 1001) as f32 / 250.0 - 2.0 }
                })
                .collect();
            let (mut x, mut y) = (mem.clone(), mem.clone());
            let mut rec = Rec::default();
            filter3(&mut x, a, out, b, n as i64, p, &mut rec);
            reference(&mut y, a, out, b, n as i64, p);
            if layout == 0 && y[out..out + n].iter().any(|v| v.is_nan()) {
                redone += 1;
            }
            for i in 0..len {
                assert_eq!(x[i].to_bits(), y[i].to_bits(), "round {} layout {} i {}", round, layout, i);
            }
            assert_eq!(rec.asserts, (a == out) as u32);
        }
        assert!(redone > 50, "the NaN redo was exercised {} times", redone);
    }

    #[test]
    fn finite_line_equals_the_plain_line() {
        let n = 40usize;
        let mem: Vec<f32> = (0..3 * n).map(|i| (i % 7) as f32 * 0.5).collect();
        let (mut x, mut y) = (mem.clone(), mem.clone());
        let mut rec = Rec::default();
        let p = cb_cell_a(3.5);
        filter3(&mut x, 0, 2 * n, n, n as i64, p, &mut rec);
        let (av, bv, ov) = split_line(&mut y, 0, n, 2 * n, n);
        filter_slices::<false>(av, bv, ov, p);
        assert_eq!(x, y);
    }
}
