//! edge_detect through the executor dispatch: the named refusals (combo tags 2 and 3, and a frame of width 1) are asserted with their
//! text, and the one-element row (tag 1) runs.
use fx_exec::kernels;
use fx_exec::types::{Arg, ExecError, Image, KParams};

fn cst(v: f32) -> Arg<'static> {
    Arg::Const { n: 1, lanes: [v.to_bits(), 0, 0, 0] }
}

fn plane(n: usize) -> Image {
    Image { n, planes: vec![(0..n).map(|i| (i as f32) * 0.125).collect()] }
}

fn unsupported_reason(r: Result<fx_exec::kernels2::Out, ExecError>) -> String {
    match r {
        Err(ExecError::Unsupported { class, reason }) => {
            assert_eq!(class, "edge_detect");
            reason
        }
        Err(e) => panic!("expected an unsupported refusal, got {e}"),
        Ok(_) => panic!("expected a refusal, got a result"),
    }
}

#[test]
fn tags_two_and_three_are_refused_by_name() {
    let img = plane(8);
    let args = [Arg::Img(&img), cst(1.0), cst(1.0), cst(0.0)];
    for tag in [2, 3] {
        let why = unsupported_reason(kernels::run_any("edge_detect", tag, &args, 8, (4, 2), &KParams::standin()));
        assert_eq!(why, "source of 2 or 3 elements (combo rows 1 and 2): only the one-element row is ported");
    }
}

#[test]
fn width_one_is_refused_by_name() {
    let img = plane(4);
    let args = [Arg::Img(&img), cst(1.0), cst(1.0), cst(0.0)];
    let why = unsupported_reason(kernels::run_any("edge_detect", 1, &args, 4, (1, 4), &KParams::standin()));
    assert_eq!(why, "width 1: the body reads one float past the plane (host memory)");
}

#[test]
fn the_one_element_row_runs() {
    let img = plane(8);
    let args = [Arg::Img(&img), cst(1.0), cst(1.0), cst(0.0)];
    let out = kernels::run_any("edge_detect", 1, &args, 8, (4, 2), &KParams::standin()).unwrap().into_image().unwrap();
    assert_eq!(out.planes.len(), 1);
    assert_eq!(out.planes[0].len(), 8);
}
