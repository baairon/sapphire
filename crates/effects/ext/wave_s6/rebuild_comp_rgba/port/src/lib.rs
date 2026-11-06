//! comp_rgba: native port of builtin class comp_rgba, vtable, slot 53, AVX2 body and its tail-called
//! bodies (listing offsets, VA = offset for data).
//!
//! The class composites an operand A (count `ca` at this) under an operand B (count `cb` at this, premultiplied) per pixel:
//!   T = alpha of B (channel cb-1 when cb is even, channel 0 when cb is odd),
//!   out_j = B_j                  when T >= 1.0
//!         = A_j + B_j            when 0.0 >= T
//!         = fma(A_j, 1 - T, B_j) otherwise (NaN T lands here).
//! Operand sources: flag byte this != 0: A is the constant array at this and B comes from planes (record this);
//! otherwise A comes from planes (record this) and B is the constant array at this when the byte this != 0, else planes.
//! Channel j of an operand with count c reads channel min(j, c-1) (broadcast of the last channel).
//! Std only, no unsafe. Every host fact is an explicit input; no defaults.

pub mod assume;
pub use assume::{assumptions, Assumption};

use std::sync::atomic::{AtomicU32, Ordering};

static MUT: AtomicU32 = AtomicU32::new(0);

/// Select a mutant for the T2 mutation run (0 = none). The port itself never calls it.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}

#[inline]
fn m(id: u32) -> bool {
    MUT.load(Ordering::Relaxed) == id
}

/// Fields of the class object the body reads. `n` is the pixel count per plane (info).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub ca: i32,
    pub cb: i32,
    pub flag_a: bool,
    pub flag_b: bool,
    pub a_vals: [f32; 4],
    pub b_vals: [f32; 4],
    pub n: i32,
}

/// Host services reached on the error path: the class virtual at [vtable] (returns true when it handled the combo itself).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Host {
    pub vcall_f0_handled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    ///  local code, (ca, cb) = (1, 1): one output channel
    Local11,
    ///  local code, (3, 2): three output channels
    Local32,
    ///  via : (1, 4) and (3, 4): four outputs, alpha plane written 1.0
    BodyCellA,
    ///  via : (4, 4) and (4, 1): four outputs with the blended alpha
    BodyCellB,
    ///  via : (4, 2): same shape as 
    BodyCellC,
    ///  via : (3, 1): three outputs
    BodyCellD,
    /// (4, 3): host plane service three times and the fill (not an arithmetic route)
    PadService,
    /// every other pair: virtual [vtable], then the log "CompRGBA: %d,%d not yet implemented."
    Unimplemented,
}

/// The combo dispatch of .
pub fn route(ca: i32, cb: i32) -> Route {
    match (ca, cb) {
        (4, 4) | (4, 1) => Route::BodyCellB,
        (4, 2) => Route::BodyCellC,
        (4, 3) => Route::PadService,
        (3, 4) | (1, 4) => Route::BodyCellA,
        (3, 1) => Route::BodyCellD,
        (3, 2) => Route::Local32,
        (1, 1) => Route::Local11,
        _ => Route::Unimplemented,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Alpha {
    None,
    One,
    Blend,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PadCall {
    /// (this, mode, chan, outs[out], info)
    PlaneService { mode: i32, chan: i32, out: usize },
    /// (this, outs[out], info, value in xmm0)
    FillAlpha { value_bits: u32, out: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Rendered { outs: usize },
    NotImplemented { ca: i32, cb: i32, logged: bool },
    PadService { calls: Vec<PadCall> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompError {
    /// fewer operand or output planes than the combo reads or writes
    PlaneCount,
    /// a plane shorter than n
    ShortPlane,
}

/// Channel index of an operand with count `cnt` for output channel `j` (the clamp the body builds with cmov).
fn sel(cnt: i32, j: usize) -> usize {
    let last = (cnt - 1).max(0) as usize;
    j.min(last)
}

/// Alpha channel of B: cb even -> cb - 1, cb odd -> 0 (test cb,1; cmovne).
fn alpha_idx(cb: i32) -> usize {
    if m(8) {
        0
    } else if cb & 1 != 0 {
        0
    } else {
        (cb - 1) as usize
    }
}

#[inline]
fn mix(a: f32, b: f32, t: f32, omt: f32) -> f32 {
    let ge1 = if m(1) {
        t > 1.0
    } else if m(12) {
        !(t < 1.0)
    } else {
        t >= 1.0
    };
    if ge1 {
        b
    } else if if m(2) { 0.0 > t } else { 0.0 >= t } {
        if m(4) {
            a
        } else {
            a + b
        }
    } else if m(3) {
        a * omt + b
    } else {
        a.mul_add(omt, b)
    }
}

/// Render one comp_rgba call. `a` and `b` are the plane arrays of the operands that are planes (ignored for a constant operand),
/// `outs` the destination planes (at least 4 for a four output route). Nothing is written beyond n or beyond the route's output count.
pub fn render(p: &Params, host: &Host, a: &[&[f32]], b: &[&[f32]], outs: &mut [&mut [f32]]) -> Result<Outcome, CompError> {
    let r = route(p.ca, p.cb);
    let (oc, alpha) = match r {
        Route::Local11 => (1usize, Alpha::None),
        Route::BodyCellD if m(15) => (4, Alpha::One),
        Route::Local32 | Route::BodyCellD => (3, Alpha::None),
        Route::BodyCellA => (4, Alpha::One),
        Route::BodyCellB | Route::BodyCellC => (4, Alpha::Blend),
        Route::PadService => {
            if outs.len() < 4 {
                return Err(CompError::PlaneCount);
            }
            let calls = if m(17) {
                vec![
                    PadCall::PlaneService { mode: 1, chan: 2, out: 2 },
                    PadCall::PlaneService { mode: 1, chan: 1, out: 1 },
                    PadCall::PlaneService { mode: 1, chan: 0, out: 0 },
                    PadCall::FillAlpha { value_bits: 1.0f32.to_bits(), out: 3 },
                ]
            } else {
                vec![
                    PadCall::PlaneService { mode: 1, chan: 0, out: 0 },
                    PadCall::PlaneService { mode: 1, chan: 1, out: 1 },
                    PadCall::PlaneService { mode: 1, chan: 2, out: 2 },
                    PadCall::FillAlpha { value_bits: if m(19) { 0.5f32.to_bits() } else { 1.0f32.to_bits() }, out: 3 },
                ]
            };
            return Ok(Outcome::PadService { calls });
        }
        Route::Unimplemented => {
            return Ok(Outcome::NotImplemented { ca: p.ca, cb: p.cb, logged: if m(16) { host.vcall_f0_handled } else { !host.vcall_f0_handled } });
        }
    };
    let a_const = p.flag_a;
    let b_const = if m(10) { p.flag_b } else { !p.flag_a && p.flag_b };
    let n = if p.n > 0 { p.n as usize } else { 0 };
    if outs.len() < oc {
        return Err(CompError::PlaneCount);
    }
    if n == 0 {
        return Ok(Outcome::Rendered { outs: oc });
    }
    if !a_const && a.len() < p.ca as usize {
        return Err(CompError::PlaneCount);
    }
    if !b_const && b.len() < p.cb as usize {
        return Err(CompError::PlaneCount);
    }
    if (!a_const && a.iter().take(p.ca as usize).any(|s| s.len() < n)) || (!b_const && b.iter().take(p.cb as usize).any(|s| s.len() < n)) || outs.iter().take(oc).any(|s| s.len() < n) {
        return Err(CompError::ShortPlane);
    }
    let tc = alpha_idx(p.cb);
    // the const-B routes of build 1 - T in double and compare in double (vcvtss2sd, vsubsd, vcomisd, vcvtsd2ss)
    let dbl = b_const && matches!(r, Route::Local11 | Route::Local32 | Route::BodyCellD);
    let n_eff = if m(13) { n - 1 } else { n };
    for i in 0..n_eff {
        let t = if b_const { p.b_vals[tc] } else { b[tc][i] };
        let omt = if dbl { (1.0f64 - t as f64) as f32 } else { 1.0f32 - t };
        let ge1 = if dbl { if m(18) { (t as f64) > 1.0f64 } else { (t as f64) >= 1.0f64 } } else { false };
        let le0 = if dbl { 0.0f64 >= (t as f64) } else { false };
        let mut res = [0f32; 4];
        let colour = if alpha == Alpha::None { oc } else { 3 };
        for j in 0..colour {
            let ja = if m(11) { 0 } else { j };
            let jb = if m(9) { j.min((p.cb - 2).max(0) as usize) } else { j };
            let av = if a_const { p.a_vals[sel(p.ca, ja)] } else { a[sel(p.ca, ja)][i] };
            let bv = if b_const { p.b_vals[sel(p.cb, jb)] } else { b[sel(p.cb, jb)][i] };
            res[j] = if dbl {
                if ge1 {
                    bv
                } else if le0 {
                    av + bv
                } else {
                    av.mul_add(omt, bv)
                }
            } else {
                mix(av, bv, t, omt)
            };
        }
        match alpha {
            Alpha::None => {}
            Alpha::One => res[3] = if m(7) { 0.0 } else { 1.0 },
            Alpha::Blend => {
                let a3 = if a_const { p.a_vals[sel(p.ca, 3)] } else { a[sel(p.ca, 3)][i] };
                let b3 = if b_const { p.b_vals[sel(p.cb, 3)] } else { b[sel(p.cb, 3)][i] };
                res[3] = if t >= 1.0 {
                    if m(5) {
                        b3
                    } else {
                        1.0
                    }
                } else if 0.0 >= t {
                    if m(6) {
                        a3 + b3
                    } else {
                        a3
                    }
                } else {
                    a3.mul_add(omt, b3)
                };
            }
        }
        if m(14) && oc >= 3 {
            res.swap(1, 2);
        }
        for j in 0..oc {
            outs[j][i] = res[j];
        }
    }
    Ok(Outcome::Rendered { outs: oc })
}

/// What the recorded pad calls do through the finished plane service crate (, flat modes). The block the class virtual returns is an
/// explicit input (CALLEE-MODELLED); the alpha fill with value 1.0 is re-expressed as a constant fill of n floats (flat ROI).
/// PAD-UNVERIFIED: the T2 compares the call record only.
#[derive(Debug, PartialEq)]
pub enum PadDone {
    Service(plane_service::Action),
    Filled(usize),
    FillRect,
}

pub fn apply_pad(
    calls: &[PadCall],
    host: &plane_service::Host,
    block: plane_service::Block,
    roi: &plane_service::Roi,
    outs: &mut [&mut [f32]],
) -> Result<Vec<PadDone>, String> {
    let mut done = Vec::new();
    for c in calls {
        match *c {
            PadCall::PlaneService { mode, chan, out } => {
                let a = plane_service::plane_service(host, plane_service::Sel::Virtual { block }, mode, chan, outs[out], roi).map_err(|e| format!("{:?}", e))?;
                done.push(PadDone::Service(a));
            }
            PadCall::FillAlpha { value_bits, out } => {
                if roi.mode != 0 {
                    done.push(PadDone::FillRect);
                } else {
                    let n = if roi.n > 0 { roi.n as usize } else { 0 };
                    if outs[out].len() < n {
                        return Err("DstShort".into());
                    }
                    for x in outs[out].iter_mut().take(n) {
                        *x = f32::from_bits(value_bits);
                    }
                    done.push(PadDone::Filled(n));
                }
            }
        }
    }
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pr(ca: i32, cb: i32, fa: bool, fb: bool, av: [f32; 4], bv: [f32; 4], n: i32) -> Params {
        Params { ca, cb, flag_a: fa, flag_b: fb, a_vals: av, b_vals: bv, n }
    }
    const H: Host = Host { vcall_f0_handled: false };
    fn run(p: &Params, a: &[Vec<f32>], b: &[Vec<f32>], noutp: usize, n: usize) -> (Result<Outcome, CompError>, Vec<Vec<f32>>) {
        let mut o: Vec<Vec<f32>> = (0..noutp).map(|_| vec![f32::from_bits(0x7fc0ffee); n + 2]).collect();
        let ar: Vec<&[f32]> = a.iter().map(|v| &v[..]).collect();
        let br: Vec<&[f32]> = b.iter().map(|v| &v[..]).collect();
        let r = {
            let mut orf: Vec<&mut [f32]> = o.iter_mut().map(|v| &mut v[..]).collect();
            render(p, &H, &ar, &br, &mut orf)
        };
        (r, o)
    }

    #[test]
    fn t1_one_channel_three_branches() {
        // (1,1) planes: T = B itself. B = 0.25 -> fma(0.5, 0.75, 0.25) = 0.625; B = 1.0 -> 1.0; B = 0.0 -> A + 0 ; B = -0.5 -> A - 0.5
        let a = vec![vec![0.5f32; 4]];
        let b = vec![vec![0.25f32, 1.0, 0.0, -0.5]];
        let (r, o) = run(&pr(1, 1, false, false, [0.; 4], [0.; 4], 4), &a, &b, 1, 4);
        assert_eq!(r, Ok(Outcome::Rendered { outs: 1 }));
        assert_eq!(&o[0][..4], &[0.625, 1.0, 0.5, 0.0]);
        assert_eq!(o[0][4].to_bits(), 0x7fc0ffee);
    }

    #[test]
    fn t1_rgb_over_rgba_const_b_alpha_one() {
        // (3,4) A planes, B const (0.1, 0.2, 0.3, 0.5): T = 0.5, out_j = A_j * 0.5 + B_j, out3 = 1.0
        let a = vec![vec![1.0f32], vec![2.0], vec![4.0]];
        let (r, o) = run(&pr(3, 4, false, true, [0.; 4], [0.1, 0.2, 0.3, 0.5], 1), &a, &[], 4, 1);
        assert_eq!(r, Ok(Outcome::Rendered { outs: 4 }));
        assert_eq!(o[0][0], 1.0f32.mul_add(0.5, 0.1));
        assert_eq!(o[1][0], 2.0f32.mul_add(0.5, 0.2));
        assert_eq!(o[2][0], 4.0f32.mul_add(0.5, 0.3));
        assert_eq!(o[3][0], 1.0);
    }

    #[test]
    fn t1_rgba_alpha_blend() {
        // (4,4) planes: T from B3. T = 1 -> rgb = B, alpha 1.0; T = 0 -> rgb = A + B, alpha = A3; T = 0.5 -> fma for all four
        let a = vec![vec![0.5f32; 3], vec![0.5; 3], vec![0.5; 3], vec![0.8; 3]];
        let b = vec![vec![0.1f32; 3], vec![0.2; 3], vec![0.3; 3], vec![1.0, 0.0, 0.5]];
        let (_, o) = run(&pr(4, 4, false, false, [0.; 4], [0.; 4], 3), &a, &b, 4, 3);
        assert_eq!([o[0][0], o[1][0], o[2][0], o[3][0]], [0.1, 0.2, 0.3, 1.0]);
        assert_eq!([o[0][1], o[1][1], o[2][1], o[3][1]], [0.6, 0.7, 0.8, 0.8]);
        assert_eq!(o[3][2], 0.8f32.mul_add(0.5, 0.5));
        assert_eq!(o[0][2], 0.5f32.mul_add(0.5, 0.1));
    }

    #[test]
    fn t1_broadcast_and_alpha_index() {
        // (4,1): B has one channel, B_j = B0 for all j, T = B0
        let a = vec![vec![1.0f32], vec![2.0], vec![3.0], vec![4.0]];
        let b = vec![vec![0.5f32]];
        let (r, o) = run(&pr(4, 1, false, false, [0.; 4], [0.; 4], 1), &a, &b, 4, 1);
        assert_eq!(r, Ok(Outcome::Rendered { outs: 4 }));
        for j in 0..4 {
            assert_eq!(o[j][0], a[j][0].mul_add(0.5, 0.5));
        }
        // (3,2): B channels [0, 1, 1], T = B1
        let a = vec![vec![1.0f32], vec![1.0], vec![1.0]];
        let b = vec![vec![0.25f32], vec![0.5]];
        let (_, o) = run(&pr(3, 2, false, false, [0.; 4], [0.; 4], 1), &a, &b, 3, 1);
        assert_eq!(o[0][0], 1.0f32.mul_add(0.5, 0.25));
        assert_eq!(o[1][0], 1.0f32.mul_add(0.5, 0.5));
        assert_eq!(o[2][0], 1.0f32.mul_add(0.5, 0.5));
        assert_eq!(alpha_idx(1), 0);
        assert_eq!(alpha_idx(2), 1);
        assert_eq!(alpha_idx(3), 0);
        assert_eq!(alpha_idx(4), 3);
    }

    #[test]
    fn t1_flags_select_operand_sources() {
        // flag_a set: A const, B planes (flag_b ignored). (3,1): T = B0.
        let b = vec![vec![0.5f32]];
        for fb in [false, true] {
            let (_, o) = run(&pr(3, 1, true, fb, [1.0, 2.0, 3.0, 9.0], [7.0; 4], 1), &[], &b, 3, 1);
            assert_eq!(o[0][0], 1.0f32.mul_add(0.5, 0.5));
            assert_eq!(o[2][0], 3.0f32.mul_add(0.5, 0.5));
        }
        // flag_a clear, flag_b set: B const (7.0 alpha > 1 -> out = B)
        let a = vec![vec![1.0f32], vec![2.0], vec![3.0]];
        let (_, o) = run(&pr(3, 1, false, true, [0.; 4], [7.0, 0., 0., 0.], 1), &a, &[], 3, 1);
        assert_eq!([o[0][0], o[1][0], o[2][0]], [7.0, 7.0, 7.0]);
    }

    #[test]
    fn t3_special_values() {
        let a = vec![vec![2.0f32; 6]];
        let nan = f32::NAN;
        let b = vec![vec![nan, f32::INFINITY, f32::NEG_INFINITY, -0.0, 1e-45, 1.0 - f32::EPSILON]];
        let (_, o) = run(&pr(1, 1, false, false, [0.; 4], [0.; 4], 6), &a, &b, 1, 6);
        assert!(o[0][0].is_nan()); // NaN T takes the fma path
        assert_eq!(o[0][1], f32::INFINITY); // T = inf >= 1: out = B
        assert_eq!(o[0][2], f32::NEG_INFINITY); // T = -inf: A + B
        assert_eq!(o[0][3], 2.0 + -0.0); // T = -0: A + B
        assert_eq!(o[0][4], 2.0f32.mul_add(1.0 - 1e-45, 1e-45)); // tiny T: fma
        assert_eq!(o[0][5], 2.0f32.mul_add(1.0 - (1.0 - f32::EPSILON), 1.0 - f32::EPSILON));
        // determinism
        let (_, o2) = run(&pr(1, 1, false, false, [0.; 4], [0.; 4], 6), &a, &b, 1, 6);
        for i in 0..6 {
            assert_eq!(o[0][i].to_bits(), o2[0][i].to_bits());
        }
    }

    #[test]
    fn t3_degenerate_and_identity() {
        let a = vec![vec![0.3f32; 2]];
        let b = vec![vec![0.0f32; 2]];
        // n <= 0 writes nothing
        for n in [0, -5] {
            let (r, o) = run(&pr(1, 1, false, false, [0.; 4], [0.; 4], n), &a, &b, 1, 2);
            assert_eq!(r, Ok(Outcome::Rendered { outs: 1 }));
            assert_eq!(o[0][0].to_bits(), 0x7fc0ffee);
        }
        // identity: B = 0 everywhere gives A
        let (_, o) = run(&pr(1, 1, false, false, [0.; 4], [0.; 4], 2), &a, &b, 1, 2);
        assert_eq!(&o[0][..2], &[0.3, 0.3]);
        // short plane and missing planes are errors, not panics
        let (r, _) = run(&pr(1, 1, false, false, [0.; 4], [0.; 4], 9), &a, &b, 1, 2);
        assert_eq!(r, Err(CompError::ShortPlane));
        let (r, _) = run(&pr(3, 1, false, false, [0.; 4], [0.; 4], 1), &a, &b, 3, 2);
        assert_eq!(r, Err(CompError::PlaneCount));
    }

    #[test]
    fn t1_dispatch_table_and_error_path() {
        let all = [(4, 4), (4, 1), (4, 2), (4, 3), (3, 4), (1, 4), (3, 1), (3, 2), (1, 1)];
        let mut impl_ = 0;
        for ca in -1..7 {
            for cb in -1..7 {
                if all.contains(&(ca, cb)) {
                    assert_ne!(route(ca, cb), Route::Unimplemented);
                    impl_ += 1;
                } else {
                    assert_eq!(route(ca, cb), Route::Unimplemented);
                }
            }
        }
        assert_eq!(impl_, 9);
        let p = pr(2, 2, false, false, [0.; 4], [0.; 4], 4);
        let mut o: Vec<&mut [f32]> = vec![];
        assert_eq!(render(&p, &Host { vcall_f0_handled: false }, &[], &[], &mut o), Ok(Outcome::NotImplemented { ca: 2, cb: 2, logged: true }));
        assert_eq!(render(&p, &Host { vcall_f0_handled: true }, &[], &[], &mut o), Ok(Outcome::NotImplemented { ca: 2, cb: 2, logged: false }));
    }

    #[test]
    fn t1_pad_route_through_plane_service() {
        let p = pr(4, 3, false, false, [0.; 4], [0.; 4], 5);
        let mut o: Vec<Vec<f32>> = (0..4).map(|_| vec![9.0f32; 5]).collect();
        let r = {
            let mut orf: Vec<&mut [f32]> = o.iter_mut().map(|v| &mut v[..]).collect();
            render(&p, &H, &[], &[], &mut orf).unwrap()
        };
        let calls = match r {
            Outcome::PadService { calls } => calls,
            other => panic!("{:?}", other),
        };
        assert_eq!(calls.len(), 4);
        assert_eq!(o[0], vec![9.0; 5]); // render itself never touches the planes on this route
        let vals = [0.25f32.to_bits(), 0.5f32.to_bits(), 0.75f32.to_bits()];
        let blk = plane_service::Block { is_int: false, values: &vals, enable: true, maxch: 2, img_valid: true, planes: &[] };
        let roi = plane_service::Roi { mode: 0, n: 5, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 };
        let mut orf: Vec<&mut [f32]> = o.iter_mut().map(|v| &mut v[..]).collect();
        let d = apply_pad(&calls, &plane_service::Host { in_pool: false }, blk, &roi, &mut orf).unwrap();
        assert_eq!(d.len(), 4);
        assert_eq!(o[0], vec![0.25; 5]);
        assert_eq!(o[2], vec![0.75; 5]);
        assert_eq!(o[3], vec![1.0; 5]);
    }
}
