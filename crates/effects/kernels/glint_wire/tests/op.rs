use glint_wire::host::{Align, ComposedHost};
use glint_wire::op::{render, source_record, Request, SERVED_DIAGONAL_MODE};
use glint_wire::refusal;
use glint_worker::{run, Input, PlaneId, Roi};

fn frame(w: usize, h: usize) -> [Vec<f32>; 3] {
    let mk = |k: usize| (0..w * h).map(|i| ((i * (3 + k) + k) % 11) as f32 * 0.125 + if i % 7 == 0 { 1.5 } else { 0.0 }).collect::<Vec<f32>>();
    [mk(0), mk(1), mk(2)]
}

fn plan_request<'a>(w: usize, h: usize, planes: &'a [Vec<f32>; 3]) -> Request<'a> {
    Request {
        width: w,
        height: h,
        count: w as i32,
        sources: [&planes[0], &planes[1], &planes[2]],
        thresholds: [0.699997; 3],
        strength: 2.688,
        a: [30.0, 60.0, 90.0],
        b: [1.0; 3],
        pass_scale: [1.0, 1.0, 0.75, 0.75],
        pass_gain: [1.0; 4],
        align: Align::default(),
        flag: 0,
    }
}

#[test]
fn align_defaults_to_zero_and_is_forwarded() {
    assert_eq!(Align::default(), Align { radii_rows: 0, radii_cols: 0, boxsum: 0, blur: 0 });
    let mut h = ComposedHost::new([1, 2, 3].map(PlaneId), [0.0; 3], None);
    assert_eq!((h.radii_phase_rows, h.radii_phase_cols, h.ring_phase, h.phase), (0, 0, 0, 0));
    h.set_align(&Align { radii_rows: 20, radii_cols: 4, boxsum: 8, blur: 12 });
    assert_eq!((h.radii_phase_rows, h.radii_phase_cols, h.ring_phase, h.phase), (20, 4, 8, 12));
}

#[test]
fn flagged_request_is_refused_before_any_output() {
    let p = frame(8, 5);
    let mut r = plan_request(8, 5, &p);
    r.flag = 1;
    let e = render(&r).unwrap_err();
    assert!(e.starts_with("GLINT-BLUR-FLAG-REFUSED"), "{e}");
    assert_eq!(e, refusal::BLUR_FLAG);
}

#[test]
fn served_frames_never_set_the_diagonal_word() {
    for w in [1usize, 2, 3, 5, 64, 4096] {
        for h in [1usize, 2, 5, 32, 2160] {
            for count in [0, 1, w as i32, 7680] {
                let r = source_record(w, h, count).unwrap();
                assert_eq!(r.diagonal_mode, SERVED_DIAGONAL_MODE);
                assert_eq!(r.diagonal_mode, 0);
                assert_eq!((r.width, r.height, r.count, r.selector), (w as i32, h as i32, count, 0));
            }
        }
    }
    assert!(source_record(1 << 20, 1 << 20, 1).is_err());
}

#[test]
fn render_equals_the_worker_run_on_hand_built_inputs() {
    for (w, h) in [(2usize, 5usize), (1, 5), (8, 5), (64, 32)] {
        let p = frame(w, h);
        let r = plan_request(w, h, &p);
        let got = render(&r).unwrap();
        let count = w as f32;
        let (cells, arr, dsts, layer_plane) = ([1u64, 2, 3].map(PlaneId), [11u64, 12, 13].map(PlaneId), [21u64, 22, 23].map(PlaneId), PlaneId(31));
        let mut host = ComposedHost::new(cells, r.thresholds, None);
        for k in 0..3 {
            host.planes.insert(arr[k], p[k].clone());
            host.planes.insert(cells[k], vec![0.0; w * h]);
            host.planes.insert(dsts[k], vec![0.0; w * h]);
        }
        host.planes.insert(layer_plane, vec![0.0; w * h]);
        let wd = count * 0.125;
        let a = [wd * 30.0, wd * 60.0, wd * 90.0];
        let inp = Input {
            source: source_record(w, h, w as i32).unwrap(),
            blur_x: (0.03125f32 * 2.688 * count).to_bits(),
            blurred: cells,
            sources: arr,
            outputs: dsts,
            layer: layer_plane,
            saved: None,
            a: a.map(f32::to_bits),
            b: [4.0f32.to_bits(); 3],
            c: [0; 3],
            pass_scale: [1.0f32, 1.0, 0.75, 0.75].map(f32::to_bits),
            pass_gain: [1.0f32; 4].map(f32::to_bits),
            roi: Roi { w: w as i32, h: h as i32, c: w as i32, d: 0, e: 4 },
            flag: 0,
        };
        run(&inp, &mut host);
        assert!(host.refused.is_none() && host.err.is_none(), "{w}x{h}");
        for k in 0..3 {
            let want = &host.planes[&dsts[k]];
            assert!(got[k].iter().zip(want).all(|(x, y)| x.to_bits() == y.to_bits()), "{w}x{h} plane {k}");
        }
    }
}

#[test]
fn render_is_live_and_deterministic() {
    let p = frame(64, 32);
    let r = plan_request(64, 32, &p);
    let a = render(&r).unwrap();
    let b = render(&r).unwrap();
    assert_eq!(a, b);
    assert!(a.iter().any(|pl| pl.iter().any(|v| *v != 0.0)));
    let mut shifted = plan_request(64, 32, &p);
    shifted.thresholds = [0.1; 3];
    assert_ne!(render(&shifted).unwrap(), a);
}

#[test]
fn bad_shapes_are_refused() {
    let p = frame(4, 4);
    let mut r = plan_request(4, 4, &p);
    r.height = 3;
    assert!(render(&r).is_err());
    r.height = 0;
    r.width = 0;
    assert!(render(&r).is_err());
}
