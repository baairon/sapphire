//! The executor gives the x86 NaN words on every processor: a ripmap built from a source with infinite samples holds the
//! default NaN 0xffc00000 (never 0x7fc00000) wherever a filter tap is infinity minus infinity.
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::types::{Arg, Image, KParams};

#[test]
fn ripmap_of_an_infinite_source_holds_only_the_x86_default_nan() {
    let (w, h) = (6usize, 4usize);
    let n = w * h;
    let plane: Vec<f32> = (0..n).map(|i| if i % 3 == 0 { f32::INFINITY } else if i % 3 == 1 { f32::NEG_INFINITY } else { 0.5 }).collect();
    let img = Image { n, planes: vec![plane] };
    let wrap = Arg::Const { n: 2, lanes: [1.0f32.to_bits(), 1.0f32.to_bits(), 0, 0] };
    let filter = Arg::Const { n: 1, lanes: [1.0f32.to_bits(), 0, 0, 0] };
    let args = [Arg::Img(&img), wrap, filter];
    let out = run_any("make_mipmap", 1, &args, n, (w, h), &KParams::standin()).unwrap();
    let Out::Mip(m) = out else { panic!("not a ripmap") };
    let words: Vec<u32> = m.ripmap.planes.iter().flatten().copied().collect();
    let nans: Vec<u32> = words.iter().copied().filter(|b| (b & 0x7fff_ffff) > 0x7f80_0000).collect();
    assert!(!nans.is_empty(), "the source must produce NaN words");
    assert!(nans.iter().all(|b| *b == 0xffc0_0000), "{:08x?}", nans.iter().filter(|b| **b != 0xffc0_0000).collect::<Vec<_>>());
}
