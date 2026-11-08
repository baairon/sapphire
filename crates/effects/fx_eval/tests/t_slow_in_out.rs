//! slow_in_out constant path against recorded results of the class body. The case file is read through SLOW_IN_OUT_CASES
//! (lines: t in out flag result, f32 bits in hex); without it the test prints NOT RUN.
use fx_eval::fold::slow_in_out;

#[test]
fn matches_recorded_body() {
    let Ok(p) = std::env::var("SLOW_IN_OUT_CASES") else {
        println!("NOT RUN (SLOW_IN_OUT_CASES unset)");
        return;
    };
    let h = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
    let mut n = 0;
    let mut bad = Vec::new();
    for l in std::fs::read_to_string(p).unwrap().lines() {
        let f: Vec<&str> = l.split_whitespace().collect();
        if f.len() != 5 {
            continue;
        }
        let r = slow_in_out(h(f[0]), h(f[1]), h(f[2]), f[3] != "0");
        let want = u32::from_str_radix(f[4], 16).unwrap();
        n += 1;
        if r.to_bits() != want {
            bad.push(format!("{} -> got {:08x} want {:08x}", l, r.to_bits(), want));
        }
    }
    println!("cases {} mismatches {}", n, bad.len());
    assert!(n > 0, "SLOW_IN_OUT_CASES holds no 5-field case line");
    for b in bad.iter().take(10) {
        println!("{}", b);
    }
    assert!(bad.is_empty());
}

#[test]
fn easing_endpoints() {
    assert_eq!(slow_in_out(0.0, 0.5, 0.5, false), 0.0);
    assert_eq!(slow_in_out(1.0, 0.5, 0.5, false), 1.0);
    assert_eq!(slow_in_out(-1.0, 0.5, 0.5, true), 1.0);
    assert_eq!(slow_in_out(-1.0, 0.5, 0.5, false), 0.0);
}

#[test]
fn host_getters_need_explicit_values() {
    use fx_eval::fold::{fold_kind, fold_lanes, FoldKind};
    for c in ["get_percent_time", "get_total_frames", "get_aspect_ratio"] {
        assert_eq!(fold_kind(c).map(|k| k.0), Some(FoldKind::Host));
        assert!(fold_lanes(c, &[], 1, None).is_none());
        let r = fold_lanes(c, &[], 1, Some(&[2.5])).unwrap();
        assert_eq!(r[0], 2.5f32.to_bits());
    }
}
