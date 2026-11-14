use optical_flow::body::{BodyResult, Refusal};
use optical_flow::join::BodyEnd;
use optical_flow::ImgCall;
use optical_flow_entry::compose::*;
use optical_flow_entry::wiring::EntryRefusal;
use optical_flow_entry::{run_entry, EntryPath, ObjFields};

fn consts() -> FlowConstants {
    FlowConstants { max_warps: 1, max_outer: 1, max_inner: 1, lambda: 0.1, use_median: 1, texture_scale: 0.5, std_smooth: 0.2 }
}

fn image(w: usize, h: usize, seed: f32) -> Vec<f32> {
    (0..w * h).map(|i| (((i % w) as f32) * 0.37 + ((i / w) as f32) * 0.21 + seed).sin()).collect()
}

fn bits(p: &FlowPlanes) -> Vec<u32> {
    p.flow_u.iter().chain(&p.flow_v).chain(&p.confidence).map(|x| x.to_bits()).collect()
}

#[test]
fn a_done_run_gives_the_planes() {
    for (w, h) in [(64usize, 32usize), (50, 7)] {
        let (a, b) = (image(w, h, 0.0), image(w, h, 0.4));
        let p = run_flow(&a, &b, w as i32, h as i32, &consts(), || false).expect("planes");
        assert_eq!((p.width, p.height), (w, h));
        assert_eq!((p.flow_u.len(), p.flow_v.len(), p.confidence.len()), (w * h, w * h, w * h));
        assert!(p.confidence.iter().any(|x| *x != 0.0), "confidence plane is all zero");
    }
}

#[test]
fn two_runs_are_equal() {
    let (w, h) = (40usize, 24usize);
    let (a, b) = (image(w, h, 0.0), image(w, h, 0.3));
    let flow_out = run_flow(&a, &b, w as i32, h as i32, &consts(), || false).unwrap();
    let r2 = run_flow(&a, &b, w as i32, h as i32, &consts(), || false).unwrap();
    assert_eq!(bits(&flow_out), bits(&r2));
}

#[test]
fn a_side_below_two_refuses_before_any_output() {
    let a = vec![0.5f32; 64];
    for (w, h) in [(1, 8), (8, 1), (0, 8), (-3, 8)] {
        assert_eq!(run_flow(&a, &a, w, h, &consts(), || false), Err(FlowStop::Refused(EntryRefusal::SmallLevel)));
    }
}

#[test]
fn flag_route_refuses_before_any_output() {
    let (w, h) = (32usize, 16usize);
    let a = image(w, h, 0.0);
    let mut img = |c: &ImgCall| served_image(c);
    let r = run_flow_with(&a, &a, w as i32, h as i32, &consts(), 1, &mut img, || false);
    assert_eq!(r, Err(FlowStop::Refused(EntryRefusal::FlagRoute)));
}

#[test]
fn null_image_refuses_before_any_output() {
    let (w, h) = (32usize, 16usize);
    let a = image(w, h, 0.0);
    let mut img = |_: &ImgCall| 0u64;
    let r = run_flow_with(&a, &a, w as i32, h as i32, &consts(), 0, &mut img, || false);
    assert_eq!(r, Err(FlowStop::Refused(EntryRefusal::NullImage)));
}

#[test]
fn pyramid_body_ends_map_to_the_four_rows() {
    assert_eq!(pyramid_body_result(BodyEnd::Refused(Refusal::LevelTableFull)), (1, Some(EntryRefusal::LevelTableFull)));
    assert_eq!(pyramid_body_result(BodyEnd::Levels(BodyResult::Refused(Refusal::SmallLevel))), (1, Some(EntryRefusal::SmallLevel)));
    assert_eq!(pyramid_body_result(BodyEnd::Refused(Refusal::NullImage)), (1, Some(EntryRefusal::NullImage)));
    assert_eq!(pyramid_body_result(BodyEnd::Refused(Refusal::FlagRoute)), (1, Some(EntryRefusal::FlagRoute)));
    assert_eq!(pyramid_body_result(BodyEnd::ReturnedOne), (1, None));
    assert_eq!(pyramid_body_result(BodyEnd::Levels(BodyResult::Returned(0))), (0, None));
}

#[test]
fn a_tiny_image_stops_in_pyramid_body() {
    let a = vec![0.5f32; 9];
    assert_eq!(run_flow(&a, &a, 3, 3, &consts(), || false), Err(FlowStop::PyramidStopped));
}

#[test]
fn abort_gives_no_planes_and_no_plane_read() {
    let (w, h) = (32usize, 16usize);
    let (a, b) = (image(w, h, 0.0), image(w, h, 0.4));
    // abort from the first poll, and abort only at the entry's own poll after lane B
    for k in [0usize, 1, 2, 5, 1000] {
        let mut n = 0usize;
        let mut img = |c: &ImgCall| served_image(c);
        let mut host = ComposedFlowHost::new(&a, &b, w as i32, h as i32, 0, &mut img, move || {
            n += 1;
            n > k
        });
        let path = run_entry(&mut host, &consts().obj_fields(), optical_flow_entry::SCALE_THRESHOLD, false);
        if path != EntryPath::InLine {
            assert_eq!(path, EntryPath::Aborted, "k {k}");
            assert_eq!(host.output_reads(), 0, "k {k}");
        } else {
            assert!(host.output_reads() > 0);
        }
    }
}

#[test]
fn every_object_field_has_an_operand() {
    let o: ObjFields = consts().obj_fields();
    assert_eq!((o.max_warps, o.max_outer, o.max_inner), (1, 1, 1));
    assert_eq!(o.use_median, 1);
    assert_eq!(o.lambda_bits, 0.1f32.to_bits() as i32);
    assert_eq!(o.texture_scale, 0.5);
    assert_eq!(o.std_smooth_bits, 0.2f32.to_bits() as i32);
}

/// UNWRITTEN-KEEP: when lane A returns early (a refusal or a stop in its body) the entry returns the failure status, makes no
/// plane read or output copy, and hands back no output plane.
#[test]
fn pyramid_body_early_return_returns_failure_and_writes_no_output() {
    let cases: [(usize, usize, i32, bool); 4] = [(32, 16, 1, false), (32, 16, 0, true), (3, 3, 0, false), (8, 8, 1, true)];
    for (w, h, flag, null_image) in cases {
        let a = image(w, h, 0.0);
        let mut img = |c: &ImgCall| if null_image { 0u64 } else { served_image(c) };
        let mut host = ComposedFlowHost::new(&a, &a, w as i32, h as i32, flag, &mut img, || false);
        let path = run_entry(&mut host, &consts().obj_fields(), optical_flow_entry::SCALE_THRESHOLD, false);
        assert_eq!(path, EntryPath::PyramidStopped, "case {w}x{h} flag {flag} null {null_image}");
        assert_eq!(host.output_reads(), 0, "case {w}x{h}");
        let mut img2 = |c: &ImgCall| if null_image { 0u64 } else { served_image(c) };
        assert!(run_flow_with(&a, &a, w as i32, h as i32, &consts(), flag, &mut img2, || false).is_err(), "case {w}x{h}");
    }
}
