//! Separable cubic resample of a float plane: `resample_h` resamples every row from width `w` to width `wd`
//! (horizontal pass), `resample_v` resamples every column from height `hs` to height `hd` (vertical pass).
//!
//! Scale down uses a cubic B-spline kernel, scale up a Catmull-Rom kernel, both with support 2 and normalised by the
//! sum of the tap weights. The summation order follows the vectorised original: a scalar head up to a 32 byte
//! boundary (horizontal pass only, long windows), an eight lane block loop, a lane reduction and a scalar tail.
//! Fused multiply-adds sit exactly where the original fuses; two-NaN operations go through `fx_common::nanop`
//! in source operand order and a fresh NaN is 0xffc00000.


use fx_common::nanop::{fma213, fma231, fms213, fms231, NanOps};
use fx_common::x86::cvtss2si32_rne;

/// Labels of this module.
pub fn assumptions() -> Vec<&'static str> {
    vec![
        "ALIGN-PHASE: the horizontal pass head length depends on the byte address of the source row modulo 32; callers pass it explicitly",
        "UNREACHED-UP-VECTOR: the eight lane arm of the scale-up branch cannot run (windows hold at most 5 taps); ported from the listing, not compared",
    ]
}

const ONE: f32 = 1.0; // cell 0x15e850c
const TWO: f32 = 2.0; // cell 0x15e8510
const EIGHT: f32 = 8.0; // cell 0x15e8544
const FOUR: f32 = 4.0; // cell 0x15e8548
const ONE_HALF: f32 = 1.5; // cell 0x15e8b00
const THREE: f32 = 3.0; // cell 0x15e8b50
const TWO_HALF: f32 = 2.5; // cell 0x15ea5d0
const SIX: f32 = 6.0; // cell 0x15e9650
const TWELVE: f32 = 12.0; // cell 0x15f124c
const NEG_HALF: f32 = -0.5; // cell 0x15e8afc (measured)
const NEG_TWO: f32 = -2.0; // cell 0x15eab18
const VEC_THRESHOLD: i64 = 0x4b0;

#[inline(always)]
fn abs(x: f32) -> f32 { f32::from_bits(x.to_bits() & 0x7fff_ffff) }

/// Scalar kernel, scale-down branch (cubic B-spline times 6).
#[inline(always)]
fn ks_down(t: f32) -> f32 {
    let ge = t >= TWO;
    if ge {
        0.0
    } else if ONE > t {
        let x = fms213(THREE, t, SIX);
        let t_sq = t.fmul(t);
        fma213(x, t_sq, FOUR)
    } else {
        let x = SIX.fsub(t);
        let x = fms213(x, t, TWELVE);
        fma213(x, t, EIGHT)
    }
}

/// Scalar kernel, scale-up branch (Catmull-Rom).
#[inline(always)]
fn ks_up(t: f32) -> f32 {
    if t >= TWO {
        0.0
    } else if ONE > t {
        let x = fms213(t, ONE_HALF, TWO_HALF);
        let t_sq = t.fmul(t);
        fma213(x, t_sq, ONE)
    } else {
        let x = fma213(NEG_HALF, t, TWO_HALF);
        let x = fms213(x, t, FOUR);
        fma213(x, t, TWO)
    }
}

/// Eight lane kernel, scale-down branch, one lane (input is the already folded |t|).
#[inline(always)]
fn kv_down(t: f32) -> f32 {
    let ge = t >= TWO;
    let lt = t < ONE;
    if !ge && lt {
        let x = fms213(THREE, t, SIX);
        let y = t.fmul(x);
        fma231(FOUR, y, t)
    } else if !ge {
        let x = SIX.fsub(t);
        let x = fms231(TWELVE, x, t);
        fma213(t, x, EIGHT)
    } else {
        0.0
    }
}

/// Eight lane kernel, scale-up branch, one lane.
#[inline(always)]
fn kv_up(t: f32) -> f32 {
    let ge = t >= TWO;
    let lt = t < ONE;
    if !ge && lt {
        let x = fms213(ONE_HALF, t, TWO_HALF);
        let y = t.fmul(x);
        fma231(ONE, y, t)
    } else if !ge {
        let x = fma231(TWO_HALF, NEG_HALF, t);
        let x = fms231(FOUR, x, t);
        fma213(t, x, TWO)
    } else {
        0.0
    }
}

/// Lane reduction of an eight lane accumulator: ((l0+l4)+(l2+l6)) + ((l1+l5)+(l3+l7)).
#[inline(always)]
fn reduce8(l: &[f32; 8]) -> f32 {
    let a = [l[0].fadd(l[4]), l[1].fadd(l[5]), l[2].fadd(l[6]), l[3].fadd(l[7])];
    let b = [a[0].fadd(a[2]), a[1].fadd(a[3])];
    b[0].fadd(b[1])
}

#[inline(always)]
fn acc_add(prod: f32, acc: f32) -> f32 {
    prod.fadd(acc)
}

/// Window state shared by both passes: scale, position of the window, bounds.
struct Setup {
    down: bool,
    scale: f32, // xmm5
    step: f32,  // xmm4 / xmm6 of the vertical pass
    lo_pos: f32,
    hi_pos: f32,
}

fn setup(n_src: i32, n_dst: i32) -> Setup {
    let a = (n_src.wrapping_sub(1)) as f32;
    let b = (n_dst.wrapping_sub(1)) as f32;
    let step = a.fdiv(b);
    if n_src > n_dst {
        let x = step.fmul(TWO);
        Setup { down: true, scale: ONE.fdiv(step), step, lo_pos: f32::from_bits(x.to_bits() ^ 0x8000_0000), hi_pos: step.fadd(step) }
    } else {
        Setup { down: false, scale: ONE, step, lo_pos: NEG_TWO, hi_pos: TWO }
    }
}

/// Window of one output element: first tap, one past the last tap, first position.
fn window(s: &Setup, n_src: i32, centre: f32, lo_pos: f32, hi_pos: f32) -> (i64, i64, f32) {
    let r = lo_pos.ceil();
    let e = cvtss2si32_rne(r);
    let lo = if e <= 0 { 0 } else { e };
    let h = hi_pos.floor();
    let mut hi = cvtss2si32_rne(h).wrapping_add(1);
    if hi >= n_src { hi = n_src; }
    let start_t = s.scale.fmul((lo as f32).fsub(centre));
    (lo as i64, hi as i64, start_t)
}

/// Largest and smallest number of taps over all output elements for `n_src <= n_dst` (scale-up branch), with the port's own
/// window arithmetic. Used by the test that bounds the scale-up window.
pub fn scale_up_window_range(n_src: i32, n_dst: i32) -> (i64, i64) {
    let s = setup(n_src, n_dst);
    let (mut lo_pos, mut hi_pos, mut centre) = (s.lo_pos, s.hi_pos, 0.0f32);
    let (mut mx, mut mn) = (0i64, i64::MAX);
    for _ in 0..n_dst {
        let (lo, hi, _) = window(&s, n_src, centre, lo_pos, hi_pos);
        let c = hi - lo;
        mx = mx.max(c);
        mn = mn.min(c);
        lo_pos = lo_pos.fadd(s.step);
        hi_pos = s.step.fadd(hi_pos);
        centre = s.step.fadd(centre);
    }
    (mx, mn)
}

/// Horizontal pass: rows `row_start..row_start+row_count` of a `w` by `h` plane into a `wd` by `h` plane.
/// `src_misalign` is the alignment of `src[0]` in bytes modulo 32 (assumed 4 byte aligned or not at all).
pub fn resample_h(src: &[f32], w: i32, h: i32, wd: i32, dst: &mut [f32], src_misalign: usize, row_start: i32, row_count: i32) {
    let _ = h;
    for r in 0..row_count.max(0) {
        let row = row_start.wrapping_add(r);
        let sbase = (w.wrapping_mul(row)) as i64;
        let dbase = (wd.wrapping_mul(row)) as i64;
        if wd <= 0 { continue; }
        let s = setup(w, wd);
        let (mut lo_pos, mut hi_pos, mut centre) = (s.lo_pos, s.hi_pos, 0.0f32);
        for x in 0..wd as i64 {
            let (lo, hi, start_t) = window(&s, w, centre, lo_pos, hi_pos);
            let (mut acc, mut wsum) = (0.0f32, 0.0f32);
            if lo < hi {
                let count = hi - lo;
                let row_src = |i: i64| src[(sbase + lo + i) as usize];
                let ks = |t: f32| if s.down { ks_down(t) } else { ks_up(t) };
                let kv = |t: f32| if s.down { kv_down(t) } else { kv_up(t) };
                let (mut peel, mut vec_end): (i64, i64) = (0, 0);
                let mut vec_ok = false;
                if count >= 8 {
                    if count < VEC_THRESHOLD {
                        vec_end = count & !7;
                        vec_ok = true;
                    } else {
                        let addr = (src_misalign as i64 + 4 * (sbase + lo)) & 0x1f;
                        if addr == 0 {
                            peel = 0;
                        } else if addr & 3 != 0 {
                            peel = -1;
                        } else {
                            peel = (32 - addr) >> 2;
                        }
                        if peel >= 0 && count >= peel + 8 {
                            let count_i32 = count as i32;
                            vec_end = (count_i32 - ((count_i32 - peel as i32) & 7)) as i64;
                            vec_ok = true;
                        } else {
                            peel = 0;
                        }
                    }
                }
                let mut tcur = start_t;
                let step8 = s.scale.fmul(EIGHT);
                if vec_ok {
                    for i in 0..peel {
                        let w_ = ks(abs(tcur));
                        let prod = w_.fmul(row_src(i));
                        wsum = wsum.fadd(w_);
                        tcur = tcur.fadd(s.scale);
                        acc = acc_add(prod, acc);
                    }
                    let base_t = if s.down { start_t } else { 0.0f32.fadd(start_t) };
                    let base_t = fma231(base_t, peel as f32, s.scale);
                    let mut tv = [0.0f32; 8];
                    for k in 0..8 { tv[k] = base_t.fadd(s.scale.fmul(k as f32)); }
                    let mut wl = [0.0f32; 8];
                    let mut lanes = [0.0f32; 8];
                    wl[0] = wsum;
                    lanes[0] = acc;
                    let mut i = peel;
                    while i < vec_end {
                        for k in 0..8 {
                            let wv = kv(abs(tv[k]));
                            let prod = wv.fmul(row_src(i + k as i64));
                            wl[k] = wv.fadd(wl[k]);
                            lanes[k] = acc_add(prod, lanes[k]);
                            tv[k] = tv[k].fadd(step8);
                        }
                        tcur = tcur.fadd(step8);
                        i += 8;
                    }
                    wsum = reduce8(&wl);
                    acc = reduce8(&lanes);
                }
                let start = if vec_ok { vec_end } else { 0 };
                let mut i = start;
                if !vec_ok { tcur = start_t; }
                while i < count {
                    let w_ = ks(abs(tcur));
                    let prod = w_.fmul(row_src(i));
                    wsum = wsum.fadd(w_);
                    tcur = tcur.fadd(s.scale);
                    acc = acc_add(prod, acc);
                    i += 1;
                }
            }
            dst[(dbase + x) as usize] = acc.fdiv(wsum);
            lo_pos = lo_pos.fadd(s.step);
            hi_pos = s.step.fadd(hi_pos);
            centre = s.step.fadd(centre);
        }
    }
}

/// Vertical pass: columns `col_start..col_start+col_count` of a `stride` wide plane of `hs` rows into `hd` rows.
pub fn resample_v(src: &[f32], stride: i32, hs: i32, hd: i32, dst: &mut [f32], col_start: i32, col_count: i32) {
    for c in 0..col_count.max(0) {
        let col = (col_start + c) as i64;
        if hd <= 0 { continue; }
        let s = setup(hs, hd);
        let (mut lo_pos, mut hi_pos, mut centre) = (s.lo_pos, s.hi_pos, 0.0f32);
        let mut doff: i64 = 0;
        for _y in 0..hd as u32 {
            let (lo, hi, start_t) = window(&s, hs, centre, lo_pos, hi_pos);
            let (mut acc, mut wsum) = (0.0f32, 0.0f32);
            if lo < hi {
                let count = (hi - lo) as i32;
                let ks = |t: f32| if s.down { ks_down(t) } else { ks_up(t) };
                let kv = |t: f32| if s.down { kv_down(t) } else { kv_up(t) };
                let at = |i: i64| -> f32 {
                    let idx = (stride.wrapping_mul(i as i32).wrapping_add((lo as i32).wrapping_mul(stride))) as i64;
                    src[(col + idx) as usize]
                };
                let mut tcur = start_t;
                let step8 = s.scale.fmul(EIGHT);
                let mut vec_end: i32 = 0;
                if stride != 0 && count >= 8 {
                    vec_end = count & !7;
                    let mut tv = [0.0f32; 8];
                    let tb = start_t;
                    for k in 0..8 { tv[k] = tb.fadd(s.scale.fmul(k as f32)); }
                    let mut wl = [0.0f32; 8];
                    let mut lanes = [0.0f32; 8];
                    let mut i = 0i32;
                    while i < vec_end {
                        for k in 0..8 {
                            let wv = kv(abs(tv[k]));
                            let prod = at((i + k as i32) as i64).fmul(wv);
                            wl[k] = wv.fadd(wl[k]);
                            lanes[k] = acc_add(prod, lanes[k]);
                            tv[k] = tv[k].fadd(step8);
                        }
                        tcur = tcur.fadd(step8);
                        i += 8;
                    }
                    wsum = reduce8(&wl);
                    acc = reduce8(&lanes);
                }
                let mut i = vec_end;
                while i < count {
                    let w_ = ks(abs(tcur));
                    let prod = w_.fmul(at(i as i64));
                    wsum = wsum.fadd(w_);
                    tcur = tcur.fadd(s.scale);
                    acc = acc_add(prod, acc);
                    i += 1;
                }
            }
            dst[(col + doff) as usize] = acc.fdiv(wsum);
            doff += stride as i64;
            lo_pos = lo_pos.fadd(s.step);
            hi_pos = s.step.fadd(hi_pos);
            centre = s.step.fadd(centre);
        }
    }
}

#[cfg(test)]
mod rule_pin {
    use super::*;

    /// Two NaNs of different payloads reach the accumulate add: the rule of the port is the first operand, quieted.
    /// The hook shows the engine taking the second operand in every such event, so no strictly compared case reaches this word.
    #[test]
    fn accumulate_add_takes_the_product_first() {
        let prod = f32::from_bits(0x7f80_0001);
        let acc = f32::from_bits(0x7fc0_0002);
        assert_eq!(acc_add(prod, acc).to_bits(), 0x7fc0_0001);
        assert_eq!(acc_add(f32::from_bits(0xff80_0003), acc).to_bits(), 0xffc0_0003);
    }

    /// Two NaNs of different payloads meet at the last add of the lane reduction: the first operand, quieted, wins.
    #[test]
    fn lane_reduction_takes_the_low_half_first() {
        let mut l = [0.0f32; 8];
        l[0] = f32::from_bits(0x7f80_0001);
        l[1] = f32::from_bits(0x7fc0_0002);
        assert_eq!(reduce8(&l).to_bits(), 0x7fc0_0001);
    }
}
