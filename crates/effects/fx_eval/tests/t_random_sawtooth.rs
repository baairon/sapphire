use fx_eval::fold::fold_lanes;
use fx_eval::sawtooth::{assumptions, cvt_i32, random_sawtooth_body as body, random_sawtooth_traced, seed_of};
use fx_eval::ArgView2;

fn s(x: f32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: [x, x, x, x], n: 1, elnum: None }
}

fn call(seed: f32, ph: f32, p: [f32; 5]) -> Option<[u32; 4]> {
    fold_lanes("random_sawtooth", &[s(seed), s(ph), s(p[0]), s(p[1]), s(p[2]), s(p[3]), s(p[4])], 1, None)
}

#[test]
fn deterministic_and_in_range_for_the_shake_call_shape() {
    for k in 0..200 {
        let ph = k as f32 * 0.37 - 20.0;
        let a = body(7, ph, 0.0, 1.0, 0.0, 0.0, 0.0);
        let b = body(7, ph, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(a.to_bits(), b.to_bits());
        assert!((0.0..1.0).contains(&a), "phase {} gave {}", ph, a);
    }
}

#[test]
fn a_phase_inside_one_table_row_is_constant_per_row() {
    // the value depends on the row picked by phase, so two phases in the same 1024-block and row differ only through the blend term, which is zero here
    let a = body(3, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let b = body(3, 10.0000001, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(a.to_bits(), b.to_bits());
}

#[test]
fn fold_matches_the_body_and_the_seed_cell() {
    let r = call(3.0, 0.5, [0.0, 1.0, 0.0, 0.0, 0.0]).unwrap();
    assert_eq!(r[0], body(seed_of(3.0), 0.5, 0.0, 1.0, 0.0, 0.0, 0.0).to_bits());
    assert_eq!(seed_of(3.0), 12288);
    assert_eq!(seed_of(f32::NAN), i32::MIN);
    assert_eq!(cvt_i32(3.0e9), i32::MIN);
    assert_eq!(cvt_i32(-2147483648.0), i32::MIN);
    assert_eq!(cvt_i32(-2.5), -2);
    assert_eq!(cvt_i32(2147483648.0), i32::MIN);
    assert_eq!(cvt_i32(2147483520.0), 2147483520);
}

#[test]
fn fewer_than_seven_arguments_do_not_fold() {
    assert!(fold_lanes("random_sawtooth", &[s(3.0), s(0.0)], 1, None).is_none());
}

#[test]
fn special_phases_return_without_panic() {
    for bits in [0x7fc0_0000u32, 0xffc0_0000, 0x7f80_0000, 0xff80_0000, 0, 0x8000_0000, 1, 0x7f7f_ffff, 0xff7f_ffff] {
        let v = body(5, f32::from_bits(bits), 0.0, 1.0, 0.0, 0.0, 0.0);
        let _ = v.to_bits();
    }
    assert_eq!(assumptions().len(), 1);
}

#[test]
fn widths_one_to_four_use_each_lane() {
    let ph = ArgView2 { is_const: true, lanes: [0.5, 1.5, 2.5, 3.5], n: 4, elnum: None };
    let r = fold_lanes("random_sawtooth", &[s(1.0), ph, s(0.0), s(1.0), s(0.0), s(0.0), s(0.0)], 4, None).unwrap();
    for i in 0..4 {
        assert_eq!(r[i], body(4096, [0.5, 1.5, 2.5, 3.5][i], 0.0, 1.0, 0.0, 0.0, 0.0).to_bits());
    }
}

/// Recorded cases of the reference body: VEC_DIR/saw_cases.tsv, rows `edi p0 p1 p2 p3 p4 p5 result` (hex words).
#[test]
fn recorded_cases() {
    let dir = match std::env::var("VEC_DIR") {
        Ok(d) => d,
        Err(_) => {
            println!("NOT RUN: VEC_DIR is not set");
            return;
        }
    };
    let path = std::path::Path::new(&dir).join("saw_cases.tsv");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => {
            println!("NOT RUN: {} is absent", path.display());
            return;
        }
    };
    let mut n = 0;
    let mut rows = String::new();
    let mut bad: Vec<String> = Vec::new();
    for l in text.lines().filter(|l| !l.trim().is_empty()) {
        let w: Vec<u32> = l.split_whitespace().map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
        let f = |i: usize| f32::from_bits(w[i]);
        let (g, pair) = random_sawtooth_traced(w[0] as i32, f(1), f(2), f(3), f(4), f(5), f(6));
        let got = g.to_bits();
        // both operands NaN in the sign division: the reference run returns the second operand, the silicon rule the first; equal as a class
        let class_eq = pair && f32::from_bits(got).is_nan() && f32::from_bits(w[7]).is_nan() && (got & 0x7fff_ffff) == (w[7] & 0x7fff_ffff);
        rows.push_str(&format!("{}\t{:08x}\t{:08x}\t{}\n", l.trim().rsplit_once(' ').unwrap().0.replace(' ', ","), w[7], got, if got == w[7] { "EQ" } else if class_eq { "ENGINE-NAN-PAIR" } else { "DIFF" }));
        if got != w[7] && !class_eq {
            bad.push(l.to_string());
        }
        n += 1;
    }
    if let Ok(o) = std::env::var("RUST_CASES_OUT") {
        std::fs::write(o, rows).unwrap();
    }
    println!("{} recorded cases, {} differ", n, bad.len());
    assert!(bad.is_empty(), "first differing case: {}", bad[0]);
}
