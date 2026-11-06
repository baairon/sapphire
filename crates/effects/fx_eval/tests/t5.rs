//! Rules read from the original's listing (slot 38 bodies, number function, class flags).
use fx_eval::compile::ArgView2;
use fx_eval::fold::collapse_arm;
use fx_eval::layout::{class_flags, flags_of};
use fx_eval::parse::orig_atof;

fn c(l: [f32; 4], n: usize, el: i32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(el) }
}
fn img(el: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: el as usize, elnum: Some(el) }
}
fn s(x: f32) -> ArgView2 {
    c([x, x, x, x], 1, 1)
}

#[test]
fn number_function_0x6688a0() {
    assert_eq!(orig_atof("0.01", false).unwrap().to_bits(), 0x3c23d70b);
    assert_eq!(orig_atof("0.05", false).unwrap().to_bits(), 0x3d4cccce);
    assert_eq!(orig_atof("-1.5", false).unwrap(), -1.5);
    assert_eq!(orig_atof("  +12", false).unwrap(), 12.0);
    assert_eq!(orig_atof("2e2", false).unwrap(), 200.0);
    assert_eq!(orig_atof("1.5x", false).unwrap(), 1.5);
    assert_eq!(orig_atof("1.2.3", false).unwrap(), orig_atof("1.2", false).unwrap());
    assert!(orig_atof("abc", false).is_none());
    assert!(orig_atof("", false).is_none());
    // the AVX2 twin (0x668650) differs only in the integer part (fused multiply add): equal for small integers
    assert_eq!(orig_atof("123.25", true), orig_atof("123.25", false));
}

#[test]
fn tilt_does_not_test_arg7() {
    let z = s(0.0);
    let o = s(1.0);
    let a = vec![img(4), z.clone(), s(0.3), z.clone(), z.clone(), o.clone(), z.clone(), s(0.7)];
    assert_eq!(collapse_arm("tilt", &a, Some(4)), Some(0));
    let mut b = a.clone();
    b[5] = s(2.0);
    assert_eq!(collapse_arm("tilt", &b, Some(4)), None);
    let mut d = a.clone();
    d[6] = s(0.1);
    assert_eq!(collapse_arm("tilt", &d, Some(4)), None);
}

#[test]
fn interp_arms() {
    let (z, o) = (s(0.0), s(1.0));
    assert_eq!(collapse_arm("interp", &[img(4), img(4), z.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("interp", &[img(4), img(4), o.clone()], Some(4)), Some(1));
    assert_eq!(collapse_arm("interp", &[s(0.25), s(0.25), s(0.5)], Some(1)), Some(0));
    assert_eq!(collapse_arm("interp", &[z.clone(), o.clone(), img(1)], Some(1)), Some(2));
    assert_eq!(collapse_arm("interp", &[img(1), o.clone(), img(1)], Some(1)), None);
}

#[test]
fn cmult_arms() {
    let one2 = c([1.0, 0.0, 0.0, 0.0], 2, 2);
    let zero2 = c([0.0; 4], 2, 2);
    assert_eq!(collapse_arm("cmult", &[img(2), one2.clone()], Some(2)), Some(0));
    assert_eq!(collapse_arm("cmult", &[one2.clone(), img(2)], Some(2)), Some(1));
    assert_eq!(collapse_arm("cmult", &[zero2.clone(), img(2)], Some(2)), Some(0));
    assert_eq!(collapse_arm("cmult", &[img(2), zero2.clone()], Some(2)), Some(1));
    // arg 0 must have 2 channels for the arg 1 one-arm
    assert_eq!(collapse_arm("cmult", &[img(1), one2.clone()], Some(2)), None);
    // (1, 0.5) is not complex one
    assert_eq!(collapse_arm("cmult", &[img(2), c([1.0, 0.5, 0.0, 0.0], 2, 2)], Some(2)), None);
}

#[test]
fn scale_rgb_and_add_rgb_and_screen_and_eab() {
    assert_eq!(collapse_arm("scale_rgb", &[img(3), s(1.0)], Some(3)), Some(0));
    assert_eq!(collapse_arm("scale_rgb", &[s(1.0), img(3)], Some(3)), Some(1));
    // odd channel count: arg 1 zero forwards arg 1
    assert_eq!(collapse_arm("scale_rgb", &[img(3), s(0.0)], Some(3)), Some(1));
    // even and unequal counts skip the zero arms
    assert_eq!(collapse_arm("scale_rgb", &[img(4), c([0.0; 4], 1, 1)], Some(4)), None);
    assert_eq!(collapse_arm("add_rgb", &[img(3), s(0.0)], Some(3)), Some(0));
    assert_eq!(collapse_arm("add_rgb", &[s(0.0), img(3)], Some(3)), Some(1));
    assert_eq!(collapse_arm("screen", &[img(3), s(1.0)], Some(3)), Some(1));
    assert_eq!(collapse_arm("screen", &[s(1.0), img(3)], Some(3)), Some(0));
    let mut a = vec![img(4), img(4), s(1.0), s(3.0), img(4), img(4), img(4)];
    assert_eq!(collapse_arm("edge_aware_blur", &a, Some(4)), None);
    a[5] = s(0.0);
    assert_eq!(collapse_arm("edge_aware_blur", &a, Some(4)), Some(0));
    a[5] = img(4);
    a[6] = s(0.0);
    assert_eq!(collapse_arm("edge_aware_blur", &a, Some(4)), Some(0));
}

#[test]
fn unconditional_and_noncollapsing_classes() {
    assert_eq!(collapse_arm("motion_blur", &[img(4), s(0.7)], Some(4)), Some(0));
    assert_eq!(collapse_arm("force_no_cache", &[img(4)], Some(4)), Some(0));
    assert_eq!(collapse_arm("copy", &[img(4)], Some(4)), Some(0));
    // slot 38 is the default (-1) body 0x4c9580
    assert_eq!(collapse_arm("get_alpha", &[img(4)], Some(1)), None);
    assert_eq!(collapse_arm("log_for_mipmap_rotate", &[s(1.0), s(0.0)], Some(1)), None);
}

#[test]
fn class_flag_words_slot49() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // the full stored table (test data): the flag words of all classes; the product holds the classes the 13 styles use and agrees on them
    let mut full = std::collections::HashMap::new();
    for l in effects_rtdata::text("fx_eval/data/CLASS_FLAGS.tsv").lines().skip(1) {
        let f: Vec<&str> = l.split('\t').collect();
        full.entry(f[0].to_string()).or_insert(u32::from_str_radix(f[1].trim_start_matches("0x"), 16).unwrap());
    }
    assert!(full.len() >= 300);
    let ff = |p: &str| full[p.strip_suffix("_if_needed").unwrap_or(p)];
    assert_eq!(ff("*"), 0x34);
    assert_eq!(ff("comp_rgba"), 0x30);
    assert_eq!(ff("blur") & 0x2000, 0x2000);
    assert_eq!(ff("blur") & 0x4, 0x4);
    assert_eq!(ff("make_mipmap") & 0x4, 0);
    assert_eq!(ff("screen_super_if_needed") & 0x4, 0x4);
    for (name, f) in class_flags() {
        assert_eq!(full[name], *f, "{}", name);
        assert_eq!(flags_of(name), *f, "{}", name);
    }
    assert_eq!(flags_of("*"), 0x34);
    assert_eq!(flags_of("blur") & 0x2000, 0x2000);
}
