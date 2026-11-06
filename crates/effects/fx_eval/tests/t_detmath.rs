//! The deterministic elementary functions against the platform math library of the machine that runs the test.
//! Every difference is printed and counted; a difference larger than one unit in the last place fails. The 13 effect
//! programs call none of these functions in their folded graphs (no atan, log, tan, exp, cexp or clog node), so no value
//! they fold can differ; that is checked by scanning the native program builders.
use fx_eval::detmath as dm;

fn ulps(a: f32, b: f32) -> u32 {
    if a.is_nan() && b.is_nan() {
        return 0;
    }
    if a == b {
        return 0;
    }
    let (x, y) = (a.to_bits() as i64, b.to_bits() as i64);
    let o = |v: i64| if v & 0x8000_0000 != 0 { 0x8000_0000 - v } else { v };
    (o(x) - o(y)).unsigned_abs() as u32
}

fn sweep() -> Vec<f32> {
    let mut v = vec![0.0f32, -0.0, 1.0, -1.0, 0.5, 2.0, 10.0, 100.0, 1e-3, 1e-20, 1e-40, 3.0e38, 0.1, 0.785398, 1.5707964, 3.1415927, 88.0, -87.0, -103.0];
    let mut s: u64 = 20261106;
    for _ in 0..3000 {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let e = ((s >> 33) % 60) as i32 - 30;
        let m = 1.0 + ((s >> 8) & 0xff_ffff) as f32 / 16777216.0;
        let sign = if s & 1 == 1 { -1.0 } else { 1.0 };
        v.push(sign * m * 2f32.powi(e));
    }
    v
}

fn report(name: &str, f: impl Fn(f32) -> (f32, f32)) -> (usize, usize) {
    let (mut n, mut d) = (0, 0);
    for x in sweep() {
        let (ours, plat) = f(x);
        n += 1;
        let u = ulps(ours, plat);
        if u != 0 && !(ours.is_nan() && plat.is_nan()) {
            d += 1;
            println!("{} x={:e} ({:08x}) ours {:08x} platform {:08x} ({} ulp)", name, x, x.to_bits(), ours.to_bits(), plat.to_bits(), u);
        }
        assert!(u <= 1, "{} x={:e}: {} ulp", name, x, u);
    }
    println!("{}: {} inputs, {} differ in the last bit", name, n, d);
    (n, d)
}

#[test]
fn against_the_platform() {
    report("ln", |x| (dm::ln(x), x.ln()));
    report("exp", |x| (dm::exp(x.clamp(-110.0, 95.0)), x.clamp(-110.0, 95.0).exp()));
    report("tan", |x| (dm::tan(x), x.tan()));
    report("sin", |x| (dm::sin(x), x.sin()));
    report("cos", |x| (dm::cos(x), x.cos()));
    report("atan", |x| (dm::atan(x), x.atan()));
    report("atan2", |x| (dm::atan2(x, 0.37 - x), x.atan2(0.37 - x)));
    // the double precision library rounded once to single: agrees with a correctly rounded result except next to a tie
    report("ln/f64", |x| (dm::ln(x), (x as f64).ln() as f32));
    report("tan/f64", |x| (dm::tan(x), (x as f64).tan() as f32));
    report("sin/f64", |x| (dm::sin(x), (x as f64).sin() as f32));
    report("cos/f64", |x| (dm::cos(x), (x as f64).cos() as f32));
    report("atan/f64", |x| (dm::atan(x), (x as f64).atan() as f32));
    report("atan2/f64", |x| (dm::atan2(x, 0.37 - x), (x as f64).atan2((0.37 - x) as f64) as f32));
    let mut d = 0;
    for x in sweep() {
        if x != 0.0 && dm::floor_log10(x) as f32 != x.abs().log10().floor() {
            d += 1;
            println!("floor_log10 x={:e} ({:08x}) ours {} platform {}", x, x.to_bits(), dm::floor_log10(x), x.abs().log10().floor());
        }
    }
    println!("floor_log10: {} inputs, {} differ (the exact value against log10 then floor)", sweep().len(), d);
    let mut d = 0;
    for e in -40..=38 {
        if dm::pow10_f32(e) != 10f32.powi(e) {
            d += 1;
            println!("pow10_f32 {} ours {:e} powi {:e}", e, dm::pow10_f32(e), 10f32.powi(e));
        }
    }
    println!("pow10_f32: 79 exponents, {} differ from powi", d);
    let mut d = 0;
    for e in -320..=308 {
        let (a, b) = (dm::pow10_f64(e), 10f64.powf(e as f64));
        if a != b {
            d += 1;
            println!("pow10_f64 {} ours {:e} powf {:e}", e, a, b);
        }
    }
    println!("pow10_f64: 629 exponents, {} differ from powf", d);
}

#[test]
fn no_folded_program_value_uses_them() {
    let src = include_str!("../src/programs_native.rs");
    for f in ["atan", "log", "tan", "exp", "cexp", "clog"] {
        assert!(!src.contains(&format!("call(\"{}\"", f)), "a native program calls {}", f);
    }
}
