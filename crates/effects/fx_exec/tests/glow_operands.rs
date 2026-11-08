//! The glow class takes eight operands: source, thresholds, stretch, widths, brightness, subpixel, edge mode, show.
use fx_exec::kernels2::{run_wired, Out};
use fx_exec::types::{Arg, ExecError, Image, KParams};

fn c(v: &[f32]) -> Arg<'static> {
    let mut l = [0u32; 4];
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}

fn img(w: usize, h: usize) -> Image {
    Image { n: w * h, planes: vec![(0..w * h).map(|i| ((i * 37 % 101) as f32) / 100.0).collect()] }
}

fn run(w: usize, h: usize, edge: f32, show: f32, count: usize) -> Result<Out, ExecError> {
    let im = img(w, h);
    let mut a: Vec<Arg> = vec![Arg::Img(&im), c(&[0.5, 0.5, 0.5]), c(&[1.0, 1.0]), c(&[12.0, 14.4, 16.7999]), c(&[2.0, 2.0, 2.0]), c(&[1.0]), c(&[edge]), c(&[show])];
    a.truncate(count);
    run_wired("glow", 1, &a, w * h, (w, h), &KParams::standin())
}

fn plane(o: Out) -> Vec<f32> {
    match o {
        Out::Img(i) => i.planes[0].clone(),
        _ => panic!("not an image"),
    }
}

#[test]
fn eight_operands_render_and_edge_is_the_plan_operand() {
    let a = plane(run(64, 32, 1.0, 0.0, 8).unwrap());
    let b = plane(run(64, 32, 0.0, 0.0, 8).unwrap());
    assert_eq!(a.len(), 64 * 32);
    assert_ne!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), b.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), "edge mode 0 and 1 must differ");
    let s = plane(run(64, 32, 1.0, 1.0, 8).unwrap());
    assert_ne!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), s.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), "show operand must change the output");
}

#[test]
fn seven_operands_are_refused() {
    assert!(matches!(run(64, 32, 1.0, 0.0, 7), Err(ExecError::BadPlan(_))));
}

#[test]
fn edge_mode_outside_zero_and_one_is_refused_and_zero_sizes_are_noops() {
    assert!(run(64, 32, 2.0, 0.0, 8).is_err());
    for (w, h) in [(64usize, 0usize), (0, 32), (0, 0)] {
        // a zero-size frame is a no-op: the call succeeds and returns an image with no pixels
        let r = run(w, h, 1.0, 0.0, 8);
        match r {
            Ok(Out::Img(i)) => assert!(i.n == 0 && i.planes.iter().all(|p| p.is_empty()), "{}x{}: n {}", w, h, i.n),
            Ok(_) => panic!("{}x{}: not an image", w, h),
            Err(e) => panic!("{}x{}: expected a no-op, got Err({:?})", w, h, e),
        }
    }
}
