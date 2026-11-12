//! The glint class takes seven operands: source, thresholds, strength, A, B, T0, T1; the op renders the three-plane layout only.
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::types::{Arg, ExecError, Image, KParams};
use glint_wire::host::Align;

fn c(v: &[f32]) -> Arg<'static> {
    let mut l = [0u32; 4];
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}

fn img(w: usize, h: usize) -> Image {
    Image { n: w * h, planes: (0..3).map(|k| (0..w * h).map(|i| ((i * (3 + k) + k) % 11) as f32 * 0.125 + if i % 7 == 0 { 1.5 } else { 0.0 }).collect()).collect() }
}

fn run(w: usize, h: usize, tag: i32, kp: &KParams, mutate: impl Fn(&mut Vec<Arg>)) -> Result<Out, ExecError> {
    let im = img(w, h);
    let mut a: Vec<Arg> = vec![Arg::Img(&im), c(&[0.699997; 3]), c(&[2.688]), c(&[30.0, 60.0, 90.0]), c(&[1.0, 1.0, 1.0]), c(&[1.0, 1.0, 0.75, 0.75]), c(&[1.0, 1.0, 1.0, 1.0])];
    mutate(&mut a);
    run_any("glint", tag, &a, w * h, (w, h), kp)
}

fn planes(o: Out) -> Vec<Vec<f32>> {
    match o {
        Out::Img(i) => i.planes,
        _ => panic!("not an image"),
    }
}

#[test]
fn glint_is_a_wired_class_with_a_combo_row() {
    assert!(fx_exec::kernels::is_supported("glint"));
}

#[test]
fn plan_arguments_render_three_planes() {
    for (w, h) in [(64, 32), (2, 5), (1, 5)] {
        let p = planes(run(w, h, 3, &KParams::standin(), |_| {}).unwrap());
        assert_eq!(p.len(), 3);
        assert!(p.iter().all(|pl| pl.len() == w * h));
    }
    let p = planes(run(64, 32, 3, &KParams::standin(), |_| {}).unwrap());
    assert!(p.iter().any(|pl| pl.iter().any(|v| *v != 0.0)));
}

#[test]
fn arguments_are_taken_by_index() {
    let base = planes(run(64, 32, 3, &KParams::standin(), |_| {}).unwrap());
    for k in 1..7usize {
        let other = planes(run(64, 32, 3, &KParams::standin(), |a| {
            let lanes = a[k].count() as usize;
            let v: Vec<f32> = a[k].f32s()[..lanes].iter().map(|x| *x * 0.1 + 0.01).collect();
            a[k] = c(&v);
        })
        .unwrap());
        assert_ne!(base, other, "argument {k} has no effect");
    }
}

#[test]
fn other_layouts_and_shapes_are_refused_by_name() {
    let e = run(8, 8, 1, &KParams::standin(), |_| {}).unwrap_err().to_string();
    assert!(e.contains("GLINT-LAYOUT-REFUSED"), "{e}");
    let e = run(8, 8, 3, &KParams::standin(), |a| a.truncate(6)).unwrap_err().to_string();
    assert!(e.contains("takes 7 arguments"), "{e}");
}

#[test]
fn align_parameters_default_to_zero() {
    assert_eq!(KParams::standin().glint_align, Align::default());
    let a = planes(run(64, 32, 3, &KParams::standin(), |_| {}).unwrap());
    let mut kp = KParams::standin();
    kp.glint_align = Align { radii_rows: 0, radii_cols: 0, boxsum: 0, blur: 0 };
    assert_eq!(a, planes(run(64, 32, 3, &kp, |_| {}).unwrap()));
}

#[test]
fn plan_shapes_select_a_row() {
    let t = fx_exec::selector::Tables::embedded();
    assert!(t.has("glint"));
    for src in [1, 3, 4] {
        let tag = t.select_counts("glint", &[src, 3, 1, 3, 3, 4, 4]);
        assert_eq!(tag, 3, "source of {src} channels");
    }
    assert_eq!(t.select_counts("glint", &[3, 3, 1, 3, 3, 4, 4, 1]), -1);
}

#[test]
fn refused_requests_name_their_refusal() {
    // The kernel boundary: a request with the flag set, and a request in the one-plane layout, each end in an error that carries the
    // refusal name before any plane is returned. The entry takes no output path and returns no plane on a refusal.
    let (w, h) = (8usize, 8usize);
    let im = img(w, h);
    fn args(im: &Image) -> Vec<Arg<'_>> {
        vec![Arg::Img(im), c(&[0.699997; 3]), c(&[2.688]), c(&[30.0, 60.0, 90.0]), c(&[1.0, 1.0, 1.0]), c(&[1.0, 1.0, 0.75, 0.75]), c(&[1.0, 1.0, 1.0, 1.0])]
    }
    let e = run_any("glint", 1, &args(&im), w * h, (w, h), &KParams::standin()).unwrap_err();
    assert!(matches!(e, ExecError::Kernel { .. }), "{e:?}");
    assert!(e.to_string().contains("GLINT-LAYOUT-REFUSED"), "{e}");
    let g = glint_wire::op::Request {
        width: w,
        height: h,
        count: w as i32,
        sources: [&im.planes[0], &im.planes[1], &im.planes[2]],
        thresholds: [0.699997; 3],
        strength: 2.688,
        a: [30.0, 60.0, 90.0],
        b: [1.0, 1.0, 1.0],
        pass_scale: [1.0, 1.0, 0.75, 0.75],
        pass_gain: [1.0, 1.0, 1.0, 1.0],
        align: KParams::standin().glint_align,
        flag: 1,
    };
    let msg = glint_wire::op::render(&g).unwrap_err();
    assert!(msg.contains("GLINT-BLUR-FLAG-REFUSED"), "{msg}");
}

#[test]
fn lanes_of_arguments_five_and_six_are_the_pass_scale_and_the_pass_gain() {
    let im = img(8, 8);
    let a: Vec<Arg> = vec![
        Arg::Img(&im),
        c(&[0.1, 0.2, 0.3]),
        c(&[0.4]),
        c(&[0.5, 0.6, 0.7]),
        c(&[0.8, 0.9, 1.0]),
        c(&[1.25, 1.5, 1.75, 2.0]),
        c(&[3.25, 3.5, 3.75, 4.0]),
    ];
    let g = fx_exec::kernels2::glint_lanes(&a).unwrap();
    assert_eq!(g.pass_scale, [1.25, 1.5, 1.75, 2.0]);
    assert_eq!(g.pass_gain, [3.25, 3.5, 3.75, 4.0]);
    assert_eq!(g.thresholds, [0.1, 0.2, 0.3]);
    assert_eq!(g.strength, 0.4);
    assert_eq!(g.a, [0.5, 0.6, 0.7]);
    assert_eq!(g.b, [0.8, 0.9, 1.0]);
}
