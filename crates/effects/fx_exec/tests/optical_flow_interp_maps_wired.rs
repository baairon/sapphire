//! optical_flow_interp_maps through the executor dispatch: four output planes at 2x2 and 50x7, and the named refusal at extent 1.
use fx_exec::kernels;
use fx_exec::types::{Arg, Image, KParams};

fn run(w: usize, h: usize) -> Result<Image, fx_exec::types::ExecError> {
    let n = w * h;
    let col = Image { n, planes: (0..4).map(|c| (0..n).map(|i| ((i * 7 + c) % 11) as f32 * 0.1).collect()).collect() };
    let flow = Image { n, planes: vec![vec![0.25; n], vec![-0.25; n], vec![0.0; n]] };
    let scale = Arg::Const { n: 1, lanes: [0.5f32.to_bits(); 4] };
    let args = [Arg::Img(&col), Arg::Img(&col), Arg::Img(&flow), scale];
    kernels::run_any("optical_flow_interp_maps", 4, &args, n, (w, h), &KParams::standin()).and_then(|o| o.into_image())
}

#[test]
fn renders_at_2x2_and_50x7() {
    for (w, h) in [(2usize, 2usize), (50, 7)] {
        let o = run(w, h).unwrap();
        assert_eq!(o.planes.len(), 4);
        assert!(o.planes.iter().all(|p| p.len() == w * h));
        assert_eq!(o, run(w, h).unwrap());
    }
}

#[test]
fn extent_one_is_refused_by_name() {
    for (w, h) in [(1usize, 7usize), (50, 1), (1, 1)] {
        let e = run(w, h).unwrap_err();
        assert!(e.to_string().contains("EXTENT-ONE-REFUSED"), "{e}");
    }
}
