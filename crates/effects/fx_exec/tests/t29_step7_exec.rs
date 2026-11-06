//! param_int at -2147483648.0f32, warp_rel's flag through cvttss2si, and
//! integer-typed set_arg_const cases (812 in-rule cases recorded in the emulator, cst3_setargconst_t2/out/cst3_t2_out.jsonl) as tests
//! of the native x86 truncation used by warp_rel (kernels2::cvt_i32) and by element_of (ops_element::common::cvtt). Asserts written before
//! the first run.
use fx_exec::kernels2::{cvt_i32, footprint_taken};
use fx_exec::owner_loop::param_int;

#[test]
fn param_int_accepts_int_min() {
    // value + 0.5f = -2147483648.0f32 (the 0.5 is below half an ulp), in range for cvttss2si
    assert_eq!(param_int(-2147483648.0f32), Some(i32::MIN));
    assert_eq!(param_int(-2147483904.0f32), None);
    assert_eq!(param_int(2147483648.0f32), None);
    assert_eq!(param_int(f32::NAN), None);
    assert_eq!(param_int(0.6f32), Some(1));
    assert_eq!(param_int(0.4f32), Some(0));
}

#[test]
fn warp_rel_fractional_flag() {
    assert!(!footprint_taken(0.5, false));
    assert!(!footprint_taken(-0.75, false));
    assert!(!footprint_taken(0.0, false));
    assert!(!footprint_taken(-0.0, false));
    assert!(footprint_taken(1.0, false));
    assert!(footprint_taken(1.5, false));
    assert!(footprint_taken(f32::NAN, false));
    assert!(!footprint_taken(1.0, true));
}

#[test]
fn cst3_integer_typed_cases() {
    if !effects_vecdir::have_all(&["fx_exec/tests/data/CST3_INT_CASES.tsv"]) {
        return;
    }
    let t = &std::fs::read_to_string(effects_vecdir::path("fx_exec/tests/data/CST3_INT_CASES.tsv")).unwrap_or_default();
    let mut n_cases = 0;
    for l in t.lines().filter(|l| !l.is_empty()) {
        let f: Vec<&str> = l.split('\t').collect();
        assert_eq!(f.len(), 13, "row {}", l);
        let n: usize = f[2].parse().unwrap();
        let s: Vec<u32> = f[3..9].iter().map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
        let lanes: Vec<u32> = f[9..13].iter().map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
        for j in 0..4 {
            let src = f32::from_bits(s[j.min(n - 1)]);
            assert_eq!(cvt_i32(src) as u32, lanes[j], "case {} lane {} (warp_rel cvt_i32)", f[0], j);
            assert_eq!(ops_element::common::cvtt(src) as u32, lanes[j], "case {} lane {} (element_of cvtt)", f[0], j);
        }
        n_cases += 1;
    }
    assert_eq!(n_cases, 812);
}
