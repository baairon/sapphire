//! Literal path, generic folding (M1), collapse rules (M3 to M8), binding (M2).
use fx_eval::fold::*;
use fx_eval::parse::orig_number;
use fx_eval::*;

fn cv(l: [f32; 4], n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(n as i32) }
}
fn img(n: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: n as usize, elnum: Some(n) }
}
fn bits(x: Option<f64>) -> u32 {
    (x.unwrap() as f32).to_bits()
}

#[test]
fn literal_path_matches_the_originals_lanes() {
    // original lanes read from the PREC rows of the recorded dumps
    assert_eq!(bits(orig_number("0.01")), 0x3c23d70b);
    assert_eq!(bits(orig_number("0.05")), 0x3d4cccce);
    assert_eq!(bits(orig_number("0.112")), 0x3de56041);
    assert_eq!(bits(orig_number("0.9")), 0x3f666667);
    // literals where the old nearest-f32 path and the original agree
    assert_eq!(bits(orig_number("0.15")), 0x3e19999a);
    assert_eq!(bits(orig_number("0.4")), 0x3ecccccd);
    assert_eq!(bits(orig_number("0.5")), 0x3f000000);
    assert_eq!(bits(orig_number("-360")), (-360f32).to_bits());
    assert_eq!(bits(orig_number(".5")), 0x3f000000);
    assert!(orig_number("1e3").is_none());
}

#[test]
fn generic_fold_set_and_exemptions() {
    // every pure class with all-constant arguments folds
    for c in ["atan", "cexp", "cmult", "cos", "log", "mod", "negate", "v3", "v4", "coerce_to", "interp", "min_element", "max_element", "get_time", "v3_to_v4"] {
        assert!(fold_kind(c).is_some(), "{}", c);
    }
    // an image argument blocks the fold of cdiv, clog, clamp, dist, mag
    for c in ["cdiv", "clog", "clamp", "dist", "mag"] {
        assert!(!can_fold(c, &[img(2), cv([1.0; 4], 1), cv([1.0; 4], 1)]), "{}", c);
    }
    // donor of coerce_to may be an image; mocha_render folds at mocha_project 0 whatever the rest is
    assert!(can_fold("coerce_to", &[cv([0.0; 4], 1), img(4)]));
    assert!(can_fold("mocha_render", &[cv([0.0; 4], 1), img(4)]));
}

#[test]
fn fold_values() {
    // cmult of a scalar by I gives (0, alpha): the scalar is (x, 0)
    let r = fold_lanes("cmult", &[cv([6.28, 6.28, 6.28, 6.28], 1), cv([0.0, 1.0, 0.0, 0.0], 2)], 2, None).unwrap();
    assert_eq!(f32::from_bits(r[0]), 0.0);
    assert_eq!(f32::from_bits(r[1]), 6.28);
    // mocha_render at project 0 is the constant 1, get_alpha of a 3 channel value is 1
    assert_eq!(fold_lanes("mocha_render", &[cv([0.0; 4], 1)], 1, None).unwrap()[0], 1f32.to_bits());
    assert_eq!(fold_lanes("get_alpha", &[img(3)], 1, None).unwrap()[0], 1f32.to_bits());
    // v3_to_v4 of two scalars is (1,1,1,1) at 4 elements; lanes at or past the element count are zero
    assert_eq!(fold_lanes("v3_to_v4", &[cv([1.0; 4], 1), cv([1.0; 4], 1)], 4, None).unwrap(), [1f32.to_bits(); 4]);
    assert_eq!(fold_lanes("v3_to_v4_if4", &[cv([1.0; 4], 1), cv([1.0; 4], 1)], 2, None).unwrap()[2], 0);
}

#[test]
fn collapse_rules_m3_to_m8() {
    let z = cv([0.0; 4], 1);
    let o = cv([1.0; 4], 1);
    assert_eq!(collapse_arm("interp", &[img(4), img(4), z.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("interp", &[img(4), img(4), o.clone()], Some(4)), Some(1));
    assert_eq!(collapse_arm("interp", &[img(4), img(4), cv([0.5; 4], 1)], Some(4)), None);
    assert_eq!(collapse_arm("add_rgb", &[img(4), z.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("scale_rgb", &[img(4), o.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("scale_rgb", &[img(4), z.clone()], Some(4)), None);
    assert_eq!(collapse_arm("screen_super_if_needed", &[img(4), z.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("copy", &[img(1)], Some(1)), Some(0));
    assert_eq!(collapse_arm("alpha_from_rgb_max", &[img(3), o.clone(), z.clone()], Some(3)), Some(0));
    assert_eq!(collapse_arm("alpha_from_rgb_max", &[img(4), o.clone(), z.clone()], Some(3)), None);
    assert_eq!(collapse_arm("alpha_from_rgb_max", &[img(4), o.clone(), z.clone()], Some(4)), None);
    assert_eq!(collapse_arm("motion_blur", &[img(4)], Some(4)), Some(0));
}
