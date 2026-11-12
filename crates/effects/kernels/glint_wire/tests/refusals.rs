use glint_worker::{Call, CallbackArgs, Host, Input, Operand, PassOp, PlaneId, Pass, Roi, SourceRecord};
use glint_wire::host::ComposedHost;
use glint_wire::refusal::*;
use glint_wire::render;

fn host_for(w: usize, h: usize) -> ComposedHost {
    let mut ho = ComposedHost::new([1, 2, 3].map(PlaneId), [0.5; 3], None);
    for k in [1u64, 2, 3, 31, 21] {
        ho.planes.insert(PlaneId(k), (0..w * h).map(|i| (i % 7) as f32 * 0.25).collect());
    }
    ho.last_alloc = (((w.max(h)) * 4) as u64, 3);
    ho
}
fn ctx(w: usize, h: usize, d: i32) -> CallbackArgs {
    let mut c = CallbackArgs::EMPTY;
    c.dest = Operand::Plane(PlaneId(31));
    c.source = Operand::Plane(PlaneId(1));
    c.width = w as u32;
    c.height = h as u32;
    c.shift = d as u32;
    c
}
fn run_mode(w: usize, h: usize, d: i32, op: PassOp) -> (ComposedHost, Vec<f32>) {
    let mut ho = host_for(w, h);
    let before = ho.planes[&PlaneId(31)].clone();
    ho.call(&Call::Parallel { op, radii: Some([1, 1, 1]) }, &ctx(w, h, d));
    (ho, before)
}

#[test]
fn diagonal_extent_refused_without_output() {
    for (w, h) in [(1, 5), (1, 3), (0, 4), (4, 0)] {
        for op in [PassOp::DiagonalBody(Pass::MainDiagonal), PassOp::DiagonalBody(Pass::AntiDiagonal)] {
            let (ho, before) = run_mode(w, h, 0, op);
            assert_eq!(ho.err.as_deref(), Some(EXTENT), "{w}x{h}");
            assert_eq!(ho.planes[&PlaneId(31)][..], before[..]);
        }
    }
}

#[test]
fn diagonal_bodies_called_on_ordinary_regions() {
    for (w, h) in [(2, 5), (3, 3), (5, 4), (64, 32)] {
        let (ho, _) = run_mode(w, h, 0, PassOp::DiagonalBody(Pass::MainDiagonal));
        assert!(ho.err.is_none(), "{w}x{h}: {:?}", ho.err);
        assert_eq!(ho.ran, vec!["diagA"]);
    }
}

#[test]
fn band_predicate_bounds() {
    let n = ((5 - 1) >> 1) + 4 - 2;
    for (start, count) in [(-1, 2), (0, n + 1), (n, 1), (1, n), (0, -1)] {
        assert!(!band_ok(start, count, n), "{start} {count}");
    }
    assert!(band_ok(0, n, n));
    assert!(band_ok(n, 0, n));
    assert!(!band_ok(0, 1, -1));
}

#[test]
fn width_one_not_refused_without_the_diagonal_route() {
    // the box-sum copy route (diagonal mode clear) is not refused at width 1
    let mut ho = host_for(1, 5);
    ho.call(&Call::Parallel { op: PassOp::DiagonalBox(Pass::AntiDiagonal), radii: Some([0, 0, 0]) }, &ctx(1, 5, 0));
    assert_ne!(ho.err.as_deref(), Some(EXTENT));
}

fn precheck(i: &Input) -> Result<(), &'static str> {
    glint_worker::refusal_three(i).map_or(Ok(()), |r| Err(r.name()))
}

fn input(w: i32, h: i32, diagonal_mode: i32, pass_scale: [f32; 4], c: f32) -> Input {
    let u = |v: f32| v.to_bits();
    Input {
        source: SourceRecord { selector: 0, plane_bytes: w * h * 4, count: 0, width: 0, height: 0, diagonal_mode }, blur_x: 0, blurred: [1, 2, 3].map(PlaneId), sources: [11, 12, 13].map(PlaneId), outputs: [21, 22, 23].map(PlaneId), layer: PlaneId(31), saved: None,
        a: [u(2.0); 3], b: [u(1.0); 3], c: [u(c); 3], pass_scale: pass_scale.map(u), pass_gain: [u(1.0); 4], roi: Roi { w, h, c: w, d: 0, e: 4 }, flag: 0,
    }
}

#[test]
fn shift_predicate_boundaries() {
    assert!(!shift_out_of_range(i32::MAX / 2));
    assert!(!shift_out_of_range(-(i32::MAX / 2)));
    assert!(shift_out_of_range(0x4000_0000));
    assert!(shift_out_of_range(i32::MIN));
}

#[test]
fn op_refuses_shift_products() {
    for c in [2147483648.0f32, f32::NAN, -2147483904.0, 1073741824.0] {
        let inp = input(8, 8, 0, [0.0, 0.0, 1.0, 0.0], c);
        assert_eq!(precheck(&inp), Err(SHIFT), "c={c}");
    }
    // 0x3fffffff is not representable in f32: the largest f32 below 2^30 passes
    let ok = f32::from_bits(0x4e7f_ffff);
    assert!(precheck(&input(8, 8, 0, [0.0, 0.0, 1.0, 0.0], ok)).is_ok());
    assert!(precheck(&input(8, 8, 0, [2.0, 2.0, 2.0, 2.0], 0.0)).is_ok());
    // a pass that does not run (strength zero) does not refuse
    assert!(precheck(&input(8, 8, 0, [0.0; 4], f32::NAN)).is_ok());
}

#[test]
fn op_refuses_diagonal_extent_only_on_the_diagonal_route() {
    let t = [0.0, 0.0, 1.0, 0.0];
    assert_eq!(precheck(&input(1, 5, 1, t, 0.0)), Err(EXTENT));
    assert_eq!(precheck(&input(4, 0, 1, t, 0.0)), Err(EXTENT));
    assert!(precheck(&input(2, 5, 1, t, 0.0)).is_ok());
    assert!(precheck(&input(1, 5, 0, t, 0.0)).is_ok());
    // a diagonal pass that does not run does not trigger it
    assert!(precheck(&input(1, 5, 1, [2.0, 0.0, 0.0, 0.0], 0.0)).is_ok());
}

#[test]
fn render_writes_no_plane_on_refusal() {
    let mut ho = host_for(1, 5);
    let inp = input(1, 5, 1, [0.0, 0.0, 1.0, 0.0], 0.0);
    let snap = ho.planes.clone();
    assert_eq!(render(&inp, &mut ho), Err(EXTENT.to_string()));
    assert_eq!(ho.planes, snap);
    assert_eq!(ho.calls, 0);
}
