//! ESH5 FM2 population for the vrsqrtss model: find x (with pw = x + 1 exact, so x13 = pw - 1 = x) where the two-rounding model 1/sqrt(x) in f32 and the
//! single-rounding model differ. Writes the first values to t2/rsqrt_diff.txt (bits of pw). Sweeps bit patterns of x in [1, 2^23) and a sample of larger ones.
use std::io::Write;
#[test]
fn sweep() {
    let mut found: Vec<u32> = Vec::new();
    let (mut tested, mut differ) = (0u64, 0u64);
    let mut b = 0x3f800000u32;
    while b < 0x4b000000 {
        let x = f32::from_bits(b);
        let pw = x + 1.0f32;
        if pw - 1.0f32 == x {
            tested += 1;
            let a = 1.0f32 / x.sqrt();
            let s = (1.0f64 / (x as f64).sqrt()) as f32;
            if a.to_bits() != s.to_bits() {
                differ += 1;
                if differ % 50_000 == 1 && found.len() < 40 {
                    found.push(pw.to_bits());
                }
            }
        }
        b += 1;
    }
    println!("rsqrt sweep: tested {} values of x in [1, 2^23) with exact pw, two-rounding vs single-rounding differ at {}", tested, differ);
    let mut f = std::fs::File::create(effects_vecdir::out_dir().join("rsqrt_diff.txt")).unwrap();
    for p in &found {
        writeln!(f, "{}", p).unwrap();
    }
    writeln!(f, "# tested {} differ {}", tested, differ).unwrap();
}
