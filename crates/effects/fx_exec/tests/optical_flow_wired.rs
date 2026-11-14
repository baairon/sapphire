//! optical_flow through the executor dispatch: three output planes at 64x32 and 50x7, and the named refusal at a side below 2.
use fx_exec::kernels::run_any;
use fx_exec::kernels2::{Out, WIRED};
use fx_exec::*;

fn cst(v: f32) -> Arg<'static> {
    Arg::Const { n: 1, lanes: [v.to_bits(); 4] }
}

fn img(w: usize, h: usize, seed: f32) -> Image {
    Image::from_planes(vec![(0..w * h).map(|i| (((i % w) as f32) * 0.37 + ((i / w) as f32) * 0.21 + seed).sin()).collect()]).unwrap()
}

fn run(w: usize, h: usize) -> Result<Image, ExecError> {
    let (a, b) = (img(w, h, 0.0), img(w, h, 0.4));
    let args = [Arg::Img(&a), Arg::Img(&b), cst(1.0), cst(1.0), cst(1.0), cst(0.1), cst(1.0), cst(0.5), cst(0.2)];
    run_any("optical_flow", 3, &args, w * h, (w, h), &KParams::standin()).and_then(|o: Out| o.into_image())
}

#[test]
fn the_class_is_wired() {
    assert!(WIRED.contains(&"optical_flow"));
}

#[test]
fn three_planes_at_64x32_and_50x7() {
    for (w, h) in [(64usize, 32usize), (50, 7)] {
        let r = run(w, h).expect("planes");
        assert_eq!(r.channels(), 3);
        assert!(r.planes.iter().all(|p| p.len() == w * h));
        let again = run(w, h).unwrap();
        for (x, y) in r.planes.iter().zip(&again.planes) {
            assert_eq!(x.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), y.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        }
    }
}

#[test]
fn a_side_below_two_is_refused_by_name() {
    match run(1, 8) {
        Err(ExecError::Kernel { class, msg }) => {
            assert_eq!(class, "optical_flow");
            assert!(msg.starts_with("OPTICAL-FLOW-SMALL-LEVEL-REFUSED"), "{msg}");
        }
        other => panic!("expected a kernel refusal, got {:?}", other.map(|i| i.channels())),
    }
}

#[test]
fn a_wrong_tag_is_a_shape_error() {
    let (a, b) = (img(8, 8, 0.0), img(8, 8, 0.4));
    let args = [Arg::Img(&a), Arg::Img(&b), cst(1.0), cst(1.0), cst(1.0), cst(0.1), cst(1.0), cst(0.5), cst(0.2)];
    assert!(matches!(run_any("optical_flow", 1, &args, 64, (8, 8), &KParams::standin()), Err(ExecError::Shape(_))));
}

#[test]
fn a_width_of_one_is_refused_by_name_too() {
    for (w, h) in [(1usize, 8usize), (8, 1), (1, 1)] {
        match run(w, h) {
            Err(ExecError::Kernel { class, msg }) => {
                assert_eq!(class, "optical_flow");
                assert!(msg.starts_with("OPTICAL-FLOW-SMALL-LEVEL-REFUSED"), "{w}x{h}: {msg}");
            }
            other => panic!("{w}x{h}: expected a kernel refusal, got {:?}", other.map(|i| i.channels())),
        }
    }
}

#[test]
fn the_nine_operands_are_taken_by_index() {
    use fx_exec::kernels2::flow_constants;
    let (a, b) = (img(4, 4, 0.0), img(4, 4, 0.4));
    // Distinct values per operand; integer operands carry a fraction that truncates.
    let args = [Arg::Img(&a), Arg::Img(&b), cst(3.5), cst(5.5), cst(7.5), cst(0.25), cst(1.5), cst(0.75), cst(0.125)];
    let c = flow_constants(&args).unwrap();
    assert_eq!(c.max_warps, 3);
    assert_eq!(c.max_outer, 5);
    assert_eq!(c.max_inner, 7);
    assert_eq!(c.lambda, 0.25);
    assert_eq!(c.use_median, 1);
    assert_eq!(c.texture_scale, 0.75);
    assert_eq!(c.std_smooth, 0.125);
}

#[test]
fn a_bad_host_phase_is_refused_and_valid_ones_run() {
    let (a, b) = (img(64, 32, 0.0), img(64, 32, 0.4));
    let args = [Arg::Img(&a), Arg::Img(&b), cst(1.0), cst(1.0), cst(1.0), cst(0.1), cst(1.0), cst(0.5), cst(0.2)];
    for bad in [2u32, 64, 65] {
        let mut kp = KParams::standin();
        kp.flow_host_phase = bad;
        match run_any("optical_flow", 3, &args, 64 * 32, (64, 32), &kp) {
            Err(ExecError::Kernel { msg, .. }) => assert!(msg.starts_with("OPTICAL-FLOW-BAD-HOST-VALUE-REFUSED"), "{msg}"),
            _ => panic!("expected the bad host value refusal for {bad}"),
        }
    }
    for ok in [0u32, 16, 28, 32, 60] {
        let mut kp = KParams::standin();
        kp.flow_host_phase = ok;
        assert!(run_any("optical_flow", 3, &args, 64 * 32, (64, 32), &kp).is_ok(), "{ok}");
    }
}
