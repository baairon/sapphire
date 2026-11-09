//! The executor leaves the output NaN words of edge_aware_sharpen as the kernel wrote them: the kernel gives the x86 words by itself, so
//! the pass that turns an ARM `0x7fc00000` into `0xffc00000` must not run on this class. The operands are infinite constants and finite
//! planes with magnitudes up to about 1e29 (no NaN input); one output word is a fresh NaN that the kernel negates, `0x7fc00000` on x86.
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::types::{Arg, Image, KParams};

/// A deterministic plane of finite values of mixed magnitude.
fn plane(seed: &mut u64, n: usize) -> Vec<f32> {
    let mut nx = || {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    };
    (0..n)
        .map(|_| match nx() % 8 {
            0 => (nx() % 2001) as f32 / 64.0 - 15.6,
            1 => (nx() % 1001) as f32 / 1000.0,
            2 => (nx() as i32 as f32) * 1e-9,
            3 => (nx() % 5) as f32,
            4 => (nx() % 2001) as f32 / 8.0 - 125.0,
            5 => (nx() >> 11) as f32 / (1u64 << 53) as f32 * 3.0 - 0.5,
            6 => (nx() as i32 as f32) * 1e20,
            _ => (nx() % 200001) as f32 * 16.0 - 1.6e6,
        })
        .collect()
}

fn run(seed: u64) -> Vec<u32> {
    let (w, h) = (7usize, 5usize);
    let n = w * h;
    let mut s = 0x9e37_79b9_7f4a_7c15u64 ^ seed.wrapping_mul(0x2545_f491_4f6c_dd1d);
    let src = Image { n, planes: vec![plane(&mut s, n), plane(&mut s, n)] };
    let mask = Image { n, planes: vec![plane(&mut s, n)] };
    let k = |c: usize, l: [u32; 4]| Arg::Const { n: c, lanes: l };
    let args = [
        Arg::Img(&src),
        Arg::Img(&mask),
        k(1, [0x4080_0000; 4]),
        k(2, [0xff80_0000, 0, 0xff80_0000, 0xff80_0000]),
        k(1, [0x4000_0000; 4]),
        k(1, [0x3f80_0000; 4]),
        k(1, [0x3f80_0000; 4]),
        k(4, [0xbf05_94bb, 0x41de_0000, 0xc06f_0000, 0x4000_0000]),
        k(3, [0xbdab_1aa4, 0xc0a4_0000, 0xef92_db27, 0xbdab_1aa4]),
        k(1, [0; 4]),
    ];
    let mut kp = KParams::standin();
    kp.sharpen_count = Some(n);
    let Out::Img(out) = run_any("edge_aware_sharpen", 2, &args, n, (w, h), &kp).unwrap() else { panic!("not an image") };
    out.planes.iter().flat_map(|p| p.iter().map(|x| x.to_bits())).collect()
}

#[test]
fn edge_aware_sharpen_output_nan_word_is_not_rewritten() {
    let words = run(87);
    assert_eq!(words.len(), 70);
    let nans: Vec<(usize, u32)> = words.iter().enumerate().filter(|(_, b)| (**b & 0x7fff_ffff) > 0x7f80_0000).map(|(i, b)| (i, *b)).collect();
    assert_eq!(nans, vec![(36, 0x7fc0_0000)], "{nans:08x?}");
}
