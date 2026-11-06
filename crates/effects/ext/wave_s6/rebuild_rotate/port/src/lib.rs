//! rotate (RT1): native port of class `rotate`slot 53 =, AVX2 body (listing offsets of neutral_1.asm,
//! captured VA = offset). Std only, no unsafe. Scratch, not part of the repository.
//!
//! The class rotates a 2-vector per sample about a centre: `(rotate v angle center)`, angle in turns (scaled by 2*pi, the captured f32
//! ), v and center are 2-lane operands, angle is a 1-lane operand; each operand is either an image (two planes, or one for the
//! angle) or a constant. Output: two planes (x', y'). With (dx, dy) = v - center and (s, c) = sincos(2*pi*angle):
//!   x' = center_x + dx*c - dy*s,  y' = center_y + dx*s + dy*c
//! evaluated with the original's exact fused-multiply-add chains (seven code paths, one per operand-kind combination, each with its own
//! operation order; the chains are NOT interchangeable bit for bit). sincos is the finished crate `libm_sincos` (SC1).

use libm_sincos::sincosf_x;

/// captured f32, bytes db0fc940 = 6.2831855f (CAPTURED, lookup_rt.tsv)
pub const TWO_PI: f32 = f32::from_bits(0x40c9_0fdb);

#[derive(Clone, Copy, Debug)]
pub enum Vec2<'a> {
    /// constant lanes (original: byte at arg nonzero, lanes at arg and arg)
    Const(f32, f32),
    /// two planes (original: record at arg, planes at record and record)
    Planes(&'a [f32], &'a [f32]),
}

#[derive(Clone, Copy, Debug)]
pub enum Angle<'a> {
    Const(f32),
    Plane(&'a [f32]),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RotateError {
    /// v, angle and center all constant with n > 0: the original reads null plane pointers (CONTRACT-LIMIT, RT1-A7)
    AllConstant,
    ShortPlane,
}

/// Mutation / test switches. `Cfg::none()` is the real behaviour (the only one `render` uses).
#[derive(Clone, Copy, Debug)]
pub struct Cfg {
    /// bit k set: FMA site k computed unfused (a*b rounded, then + c rounded)
    pub unfuse: u64,
    pub logic: Logic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Logic {
    None,
    SwapSinCos,
    NoScale,
    NegSin,
    NegCos,
    StdSinCos,
    P3AsP2,
    P7Unfolded,
    NShort,
    OutSwap,
    DeltaReversed,
}

impl Cfg {
    pub const fn none() -> Cfg {
        Cfg { unfuse: 0, logic: Logic::None }
    }
    #[inline]
    fn f(&self, site: u32, a: f32, b: f32, c: f32) -> f32 {
        if (self.unfuse >> site) & 1 == 1 {
            a * b + c
        } else {
            a.mul_add(b, c)
        }
    }
    #[inline]
    fn sc(&self, arg: f32) -> (f32, f32) {
        let x = if self.logic == Logic::NoScale { arg } else { TWO_PI * arg };
        let (mut s, mut c) = if self.logic == Logic::StdSinCos { x.sin_cos() } else { sincosf_x(x) };
        match self.logic {
            Logic::SwapSinCos => core::mem::swap(&mut s, &mut c),
            Logic::NegSin => s = -s,
            Logic::NegCos => c = -c,
            _ => {}
        }
        (s, c)
    }
}

pub const FMA_SITES: u32 = 32;

/// S3 entry: renders the two output planes from explicit operands. `n <= 0` writes nothing and succeeds.
pub fn render(v: Vec2, angle: Angle, center: Vec2, n: i32, ox: &mut [f32], oy: &mut [f32]) -> Result<(), RotateError> {
    render_cfg(Cfg::none(), v, angle, center, n, ox, oy)
}

#[inline]
fn ld(p: &[f32], i: usize) -> f32 {
    p[i]
}

pub fn render_cfg(g: Cfg, v: Vec2, angle: Angle, center: Vec2, n: i32, ox: &mut [f32], oy: &mut [f32]) -> Result<(), RotateError> {
    if n <= 0 {
        return Ok(());
    }
    let mut nn = n as usize;
    let (ox, oy) = if g.logic == Logic::OutSwap { (oy, ox) } else { (ox, oy) };
    if ox.len() < nn || oy.len() < nn {
        return Err(RotateError::ShortPlane);
    }
    let chk = |p: &[f32]| if p.len() < nn { Err(RotateError::ShortPlane) } else { Ok(()) };
    if let Vec2::Planes(a, b) = v {
        chk(a)?;
        chk(b)?;
    }
    if let Vec2::Planes(a, b) = center {
        chk(a)?;
        chk(b)?;
    }
    if let Angle::Plane(a) = angle {
        chk(a)?;
    }
    if g.logic == Logic::NShort {
        nn -= 1;
    }
    let rev = g.logic == Logic::DeltaReversed;
    // delta helper: v - center (original) or center - v (mutant)
    let dl = |a: f32, b: f32| if rev { b - a } else { a - b };
    match angle {
        Angle::Plane(ang) => match (v, center) {
            // P1:, v and center constant
            (Vec2::Const(a, b), Vec2::Const(cc, d)) => {
                let dx = a - cc;
                let dy = b - d;
                for i in 0..nn {
                    let (s, c) = g.sc(ld(ang, i));
                    let t1 = g.f(0, -dy, s, cc);
                    let t2 = g.f(1, s, dx, d);
                    ox[i] = g.f(2, dx, c, t1);
                    oy[i] = g.f(3, c, dy, t2);
                }
            }
            // P2:, v constant, centre planes
            (Vec2::Const(a, b), Vec2::Planes(cx, cy)) => {
                for i in 0..nn {
                    let (s, c) = g.sc(ld(ang, i));
                    let (cxi, cyi) = (ld(cx, i), ld(cy, i));
                    let dx = dl(a, cxi);
                    let dy = dl(b, cyi);
                    let t1 = g.f(4, dx, c, cxi);
                    ox[i] = g.f(5, -dy, s, t1);
                    let t2 = g.f(6, dy, c, cyi);
                    oy[i] = g.f(7, s, dx, t2);
                }
            }
            // P3: split: v planes, centre constant
            (Vec2::Planes(vx, vy), Vec2::Const(cc, d)) => {
                for i in 0..nn {
                    let (s, c) = g.sc(ld(ang, i));
                    let (vxi, vyi) = (ld(vx, i), ld(vy, i));
                    let dx = dl(vxi, cc);
                    let dy = dl(vyi, d);
                    if g.logic == Logic::P3AsP2 {
                        let t1 = g.f(8, dx, c, cc);
                        ox[i] = g.f(10, -dy, s, t1);
                        let t2 = g.f(9, dy, c, d);
                        oy[i] = g.f(11, s, dx, t2);
                    } else {
                        let t1 = g.f(8, dx, c, cc);
                        let t2 = g.f(9, dx, s, d);
                        ox[i] = g.f(10, -dy, s, t1);
                        oy[i] = g.f(11, c, dy, t2);
                    }
                }
            }
            // P4:, all planes
            (Vec2::Planes(vx, vy), Vec2::Planes(cx, cy)) => {
                for i in 0..nn {
                    let (s, c) = g.sc(ld(ang, i));
                    let (cxi, cyi) = (ld(cx, i), ld(cy, i));
                    let dx = dl(ld(vx, i), cxi);
                    let dy = dl(ld(vy, i), cyi);
                    let t1 = g.f(12, dx, c, cxi);
                    ox[i] = g.f(13, -dy, s, t1);
                    let t2 = g.f(14, dy, c, cyi);
                    oy[i] = g.f(15, s, dx, t2);
                }
            }
        },
        Angle::Const(a0) => {
            // sincos once, before the n test of the original 
            let (s, c) = g.sc(a0);
            match (v, center) {
                // P5:, v constant, centre planes
                (Vec2::Const(a, b), Vec2::Planes(cx, cy)) => {
                    for i in 0..nn {
                        let (cxi, cyi) = (ld(cx, i), ld(cy, i));
                        let dx = dl(a, cxi);
                        let dy = dl(b, cyi);
                        let t1 = g.f(16, dx, c, cxi);
                        ox[i] = g.f(17, -dy, s, t1);
                        let t2 = g.f(18, c, dy, cyi);
                        oy[i] = g.f(19, dx, s, t2);
                    }
                }
                // P6:, v planes, centre planes
                (Vec2::Planes(vx, vy), Vec2::Planes(cx, cy)) => {
                    for i in 0..nn {
                        let (cxi, cyi) = (ld(cx, i), ld(cy, i));
                        let dx = dl(ld(vx, i), cxi);
                        let dy = dl(ld(vy, i), cyi);
                        let t1 = g.f(20, dx, c, cxi);
                        ox[i] = g.f(21, -dy, s, t1);
                        let t2 = g.f(22, c, dy, cyi);
                        oy[i] = g.f(23, dx, s, t2);
                    }
                }
                // P7:, centre constant (v planes; v constant is outside the contract)
                (Vec2::Planes(vx, vy), Vec2::Const(cc, d)) => {
                    if g.logic == Logic::P7Unfolded {
                        for i in 0..nn {
                            let dx = dl(ld(vx, i), cc);
                            let dy = dl(ld(vy, i), d);
                            let t1 = g.f(28, dx, c, cc);
                            ox[i] = g.f(30, -dy, s, t1);
                            let t2 = g.f(29, c, dy, d);
                            oy[i] = g.f(31, dx, s, t2);
                        }
                    } else {
                        let k1a = g.f(24, s, d, cc);
                        let k1 = g.f(25, -cc, c, k1a);
                        let k2a = g.f(26, -s, cc, d);
                        let k2 = g.f(27, -d, c, k2a);
                        for i in 0..nn {
                            let (vxi, vyi) = (ld(vx, i), ld(vy, i));
                            let t1 = g.f(28, c, vxi, k1);
                            let t2 = g.f(29, vxi, s, k2);
                            ox[i] = g.f(30, -vyi, s, t1);
                            oy[i] = g.f(31, vyi, c, t2);
                        }
                    }
                }
                (Vec2::Const(_, _), Vec2::Const(_, _)) => return Err(RotateError::AllConstant),
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub tier: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "RT1-A1", tier: "CAPTURED", text: "TWO_PI = f32 0x40c90fdb, five sites in the AVX2 body" },
        Assumption { id: "RT1-A2", tier: "ASSUMED-SEMANTIC 2 (inherited SC1-A1, SC1-A2)", text: "sincos callee is the finished crate libm_sincos; its uncaptured table cells are stand-ins on both sides of T2" },
        Assumption { id: "RT1-A3", tier: "ROUTE AVX2-ONLY", text: "feature word served as 0x9d97ff; the SSE tier is not ported (it is the third tier; no AVX tier exists in this class)" },
        Assumption { id: "RT1-A4", tier: "HOST-FP-ASSUMED", text: "MXCSR round to nearest even, no FTZ, no DAZ; the emulator follows the SDM, not silicon (vfmadd NaN rules)" },
        Assumption { id: "RT1-A5", tier: "INFERRED", text: "operand layout: argument i at this*(i+1): const byte, lanes, record (planes at record); arg0 = v, arg1 = angle (turns), arg2 = centre; outputs outs[0], outs[1]" },
        Assumption { id: "RT1-A6", tier: "INFERRED", text: "single combo (2, 1, 2) tag 2 (audit_evt COMBOS.tsv); constness is not part of the combo, so all operand-kind combinations run this one body" },
        Assumption { id: "RT1-A7", tier: "CONTRACT-LIMIT", text: "v, angle and centre all constant with n > 0 dereferences null planes in the original (P7 reads v planes unconditionally); the port returns AllConstant; whether the evaluator folds such nodes is UNKNOWN (W95)" },
        Assumption { id: "RT1-A8", tier: "ALIGN-INDEPENDENT", text: "scalar loops only (no peel, no vector core): results do not depend on plane alignment or on n" },
        Assumption { id: "RT1-A9", tier: "CONTRACT-LIMIT", text: "short planes (shorter than n) and overlapping in/out planes are outside the contract" },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(v: Vec2, a: Angle, c: Vec2, n: usize) -> (Vec<f32>, Vec<f32>) {
        let mut ox = vec![f32::from_bits(0x7fc0ffee); n.max(1)];
        let mut oy = ox.clone();
        render(v, a, c, n as i32, &mut ox, &mut oy).unwrap();
        (ox, oy)
    }

    #[test]
    fn two_pi_is_captured_value() {
        assert_eq!(TWO_PI, 6.2831855f32);
    }

    #[test]
    fn hand_quarter_turn_about_origin() {
        // (1, 0) turned a quarter turn (angle 0.25 turns): x' = c*1 ~ -4.37e-8, y' = s*1 = 1
        let vx = [1.0f32];
        let vy = [0.0f32];
        let (ox, oy) = run(Vec2::Planes(&vx, &vy), Angle::Const(0.25), Vec2::Const(0.0, 0.0), 1);
        assert!(ox[0].abs() < 1e-7 && (oy[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn identity_at_angle_zero_centre_zero_exact() {
        let vx = [0.3f32, -2.5, 1e-20, 7.0];
        let vy = [-0.1f32, 4.0, 3.0, -0.0];
        let (ox, oy) = run(Vec2::Planes(&vx, &vy), Angle::Const(0.0), Vec2::Const(0.0, 0.0), 4);
        for i in 0..4 {
            assert_eq!(ox[i].to_bits(), vx[i].to_bits(), "x {}", i);
            assert!((oy[i] - vy[i]).abs() == 0.0);
        }
    }

    #[test]
    fn half_turn_about_centre() {
        // half turn about (1, 1): (3, 1) -> (-1, 1) up to the sin(pi) residue
        let (ox, oy) = run(Vec2::Const(3.0, 1.0), Angle::Plane(&[0.5]), Vec2::Const(1.0, 1.0), 1);
        assert!((ox[0] + 1.0).abs() < 1e-5 && (oy[0] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn all_seven_paths_agree_within_rounding() {
        let n = 5;
        let ang: Vec<f32> = (0..n).map(|i| 0.07 * i as f32 - 0.1).collect();
        let vx: Vec<f32> = (0..n).map(|i| 0.5 + i as f32).collect();
        let vy: Vec<f32> = (0..n).map(|i| 2.0 - 0.3 * i as f32).collect();
        let cx: Vec<f32> = vec![0.25; n];
        let cy: Vec<f32> = vec![-0.5; n];
        let reference = |v: (f32, f32), a: f32, c: (f32, f32)| {
            let (s, co) = ((6.2831853f64 * a as f64).sin(), (6.2831853f64 * a as f64).cos());
            let (dx, dy) = ((v.0 - c.0) as f64, (v.1 - c.1) as f64);
            ((c.0 as f64 + dx * co - dy * s) as f32, (c.1 as f64 + dx * s + dy * co) as f32)
        };
        let vcs = [Vec2::Planes(&vx, &vy)];
        for (vi, v) in vcs.iter().enumerate() {
            for cen in [Vec2::Const(0.25, -0.5), Vec2::Planes(&cx, &cy)] {
                for a in [Angle::Plane(&ang), Angle::Const(0.13)] {
                    let (ox, oy) = run(*v, a, cen, n);
                    for i in 0..n {
                        let ai = match a { Angle::Plane(p) => p[i], Angle::Const(c) => c };
                        let (rx, ry) = reference((vx[i], vy[i]), ai, (0.25, -0.5));
                        assert!((ox[i] - rx).abs() < 1e-4 && (oy[i] - ry).abs() < 1e-4, "v{} i{} {} {} vs {} {}", vi, i, ox[i], oy[i], rx, ry);
                    }
                }
            }
        }
        // constant v paths
        for cen in [Vec2::Const(0.25, -0.5), Vec2::Planes(&cx, &cy)] {
            for a in [Angle::Plane(&ang), Angle::Const(0.13)] {
                if let (Vec2::Const(..), Angle::Const(_)) = (cen, a) { continue; }
                let (ox, oy) = run(Vec2::Const(1.5, -2.0), a, cen, n);
                for i in 0..n {
                    let ai = match a { Angle::Plane(p) => p[i], Angle::Const(c) => c };
                    let (rx, ry) = reference((1.5, -2.0), ai, (0.25, -0.5));
                    assert!((ox[i] - rx).abs() < 1e-4 && (oy[i] - ry).abs() < 1e-4);
                }
            }
        }
    }

    #[test]
    fn degenerate_sizes_and_all_const() {
        let mut ox = [1.0f32; 4];
        let mut oy = [2.0f32; 4];
        assert!(render(Vec2::Const(1.0, 2.0), Angle::Const(0.1), Vec2::Const(0.0, 0.0), 0, &mut ox, &mut oy).is_ok());
        assert!(render(Vec2::Const(1.0, 2.0), Angle::Const(0.1), Vec2::Const(0.0, 0.0), -5, &mut ox, &mut oy).is_ok());
        assert_eq!(ox, [1.0; 4]);
        assert_eq!(render(Vec2::Const(1.0, 2.0), Angle::Const(0.1), Vec2::Const(0.0, 0.0), 2, &mut ox, &mut oy), Err(RotateError::AllConstant));
        assert_eq!(render(Vec2::Const(1.0, 2.0), Angle::Plane(&[0.1]), Vec2::Const(0.0, 0.0), 2, &mut ox, &mut oy), Err(RotateError::ShortPlane));
    }

    #[test]
    fn nan_inf_and_determinism() {
        let vx = [f32::NAN, f32::INFINITY, -f32::INFINITY, 1.0];
        let vy = [0.0f32, 1.0, f32::NAN, 2.0];
        let (a1, b1) = run(Vec2::Planes(&vx, &vy), Angle::Const(0.3), Vec2::Const(0.0, 0.0), 4);
        let (a2, b2) = run(Vec2::Planes(&vx, &vy), Angle::Const(0.3), Vec2::Const(0.0, 0.0), 4);
        for i in 0..4 {
            assert_eq!(a1[i].to_bits(), a2[i].to_bits());
            assert_eq!(b1[i].to_bits(), b2[i].to_bits());
        }
        assert!(a1[0].is_nan() && b1[2].is_nan());
        // infinite angle gives NaN planes
        let (c, d) = run(Vec2::Const(1.0, 1.0), Angle::Plane(&[f32::INFINITY]), Vec2::Planes(&[0.0], &[0.0]), 1);
        assert!(c[0].is_nan() && d[0].is_nan());
    }

    #[test]
    fn fused_chain_differs_from_unfused_somewhere() {
        // each FMA site must be observable (T1 mirror of the mutation test)
        let n = 64;
        let ang: Vec<f32> = (0..n).map(|i| 0.0137 * i as f32 + 0.011).collect();
        let vx: Vec<f32> = (0..n).map(|i| 1.0 / (1.0 + i as f32)).collect();
        let vy: Vec<f32> = (0..n).map(|i| 0.7 - 0.01 * i as f32).collect();
        let mut hit = 0u64;
        for t in 0..40usize {
        let tf = 0.173 * t as f32 + 0.0291;
        let cx: Vec<f32> = (0..n).map(|i| 0.1137 * ((i * 7 + t) % 13) as f32 + tf).collect();
        let cy: Vec<f32> = (0..n).map(|i| -0.0313 * ((i * 5 + t) % 11) as f32 - tf * 0.7).collect();
        let base = |v: Vec2, a: Angle, c: Vec2, g: Cfg| {
            let mut ox = vec![0.0f32; n];
            let mut oy = vec![0.0f32; n];
            render_cfg(g, v, a, c, n as i32, &mut ox, &mut oy).unwrap();
            (ox, oy)
        };
        let kinds: Vec<(Vec2, Angle, Vec2)> = vec![
            (Vec2::Const(0.37 + tf, -0.91), Angle::Plane(&ang), Vec2::Const(0.2 + tf, 0.3 - tf)),
            (Vec2::Const(0.37 + tf, -0.91), Angle::Plane(&ang), Vec2::Planes(&cx, &cy)),
            (Vec2::Planes(&vx, &vy), Angle::Plane(&ang), Vec2::Const(0.2 + tf, 0.3 - tf)),
            (Vec2::Planes(&vx, &vy), Angle::Plane(&ang), Vec2::Planes(&cx, &cy)),
            (Vec2::Const(0.37 + tf, -0.91), Angle::Const(0.123 + tf * 0.5), Vec2::Planes(&cx, &cy)),
            (Vec2::Planes(&vx, &vy), Angle::Const(0.123 + tf * 0.5), Vec2::Planes(&cx, &cy)),
            (Vec2::Planes(&vx, &vy), Angle::Const(0.123 + tf * 0.5), Vec2::Const(0.2 + tf, 0.3 - tf)),
        ];
        for (v, a, c) in &kinds {
            let r = base(*v, *a, *c, Cfg::none());
            for site in 0..FMA_SITES {
                let m = base(*v, *a, *c, Cfg { unfuse: 1 << site, logic: Logic::None });
                if m.0.iter().zip(&r.0).any(|(p, q)| p.to_bits() != q.to_bits()) || m.1.iter().zip(&r.1).any(|(p, q)| p.to_bits() != q.to_bits()) {
                    hit |= 1 << site;
                }
            }
        }
        }
        assert_eq!(hit, (1u64 << FMA_SITES) - 1, "sites not observable: {:032b}", !hit & ((1u64 << FMA_SITES) - 1));
    }
}
