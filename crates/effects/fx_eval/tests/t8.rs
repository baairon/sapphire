//! T8 (NEG-2): the evaluator owns the parameter range table and the measured clamp rule (NEG-1 runs D, D2, D3).
use fx_eval::ranges::*;
use fx_eval::*;

// INT18 (W414 (c) step 7): test-local copies of the two functions removed from fx_eval::ranges (byte copies but for `pub`).
/// The measured clamp of ONE lane against a record (min, max). See the module comment for the rule.
fn clamp_lane(mut x: f32, min: f32, max: f32) -> f32 {
    if max != UNENFORCED_BOUND && x > max {
        x = max;
    }
    if min != -UNENFORCED_BOUND && x < min {
        x = min;
    }
    x
}

/// Lane-wise clamp of a bound value; returns true when any lane changed bits.
fn clamp_lanes(v: &mut [f32], min: f32, max: f32) -> bool {
    let mut changed = false;
    for x in v.iter_mut() {
        let y = clamp_lane(*x, min, max);
        if y.to_bits() != x.to_bits() {
            changed = true;
        }
        *x = y;
    }
    changed
}

use std::collections::BTreeMap;
use std::path::PathBuf;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

fn gen(text: &str, name: &str) -> Genotype {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", text).unwrap();
    let ei = c.find_effect(name, None).unwrap();
    build_effect(&c, ei).unwrap()
}

fn bound(prog_text: &str, name: &str, sets: &[(&str, &[f32])], off: bool) -> BTreeMap<String, Vec<f32>> {
    let g = gen(prog_text, name);
    let mut ov = BTreeMap::new();
    for (k, v) in sets {
        ov.insert(k.to_string(), v.to_vec());
    }
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov, ranges_off: off, no_fold: true, ..Default::default() }).unwrap();
    cp.params.iter().map(|(n, l)| (n.clone(), l.iter().map(|b| f32::from_bits(*b)).collect())).collect()
}

const HSB: &str = "(def_effect Hue_Sat_Bright_Autogen ( S ) ( ) ( saturation = 1 hue_shift = 0 offset_darks = 0 ) ( + ( * S saturation ) ( + hue_shift offset_darks ) ))";

fn sat(v: f32) -> f32 {
    bound(HSB, "Hue_Sat_Bright_Autogen", &[("saturation", &[v])], false)["saturation"][0]
}

fn b(x: f32) -> u32 {
    x.to_bits()
}

#[test]
fn table_is_embedded_and_complete() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // the copy of the full table (test data); the defined rows are compared with it in t_native_defs.rs
    let t = RangeTable::parse(effects_rtdata::text("fx_eval/data/PARAM_RANGES.tsv"));
    assert_eq!(t.rows.len(), 437, "RNG1 236 rows (REC values for three Warp_Perspective_Autogen layout rows) plus SHE2 74 Sharpen rows");
    assert_eq!(t.programs(), vec!["Hue_Sat_Bright_Autogen", "Diffuse_Autogen", "Trails_Autogen", "Time_Warp_RGB_Autogen", "Transform_and_Blur_Autogen", "Blur_Motion_Curves_No_Transform_Autogen", "Blur_Color_Autogen", "Blur_Mono_Autogen", "Distort_RGB_Autogen", "Warp_Perspective_Autogen", "Drop_Shadow", "Hue_Sat_Bright", "Diffuse", "Trails", "Time_Warp_RGB", "Transform_and_Blur", "Blur_Motion_Curves_No_Transform", "Sharpen_Fast_Autogen", "Sharpen_Fast", "Sharpen_Best_Quality", "Sharpen_Best_Quality_Autogen"]);
    // INT6 (Batch 36 rule 5): the default table is REAL: Drop_Shadow shift is unbounded (min * in the layout text)
    let r = t.lookup("Drop_Shadow", "shift").unwrap();
    assert_eq!((r.lanes, r.min, r.max), (2, -1.0e6, 1.0e6));
    let r = t.lookup("Drop_Shadow", "fg_opacity").unwrap();
    assert_eq!((r.min, r.max), (0.0, 1.0));
    // DSW2: Drop_Shadow rows (REC-HARNESS table = the harness record ranges, MEASURED from the PREC rows of the original plan dumps)
    let t = RangeTable::parse(effects_rtdata::text("fx_eval/data/PARAM_RANGES_REC.tsv"));
    assert_eq!(fx_eval::ranges::row_label("Diffuse").as_deref(), Some("RANGE-ROWS-CORE-COPY"));
    assert_eq!(fx_eval::ranges::row_label("Sharpen_Fast").as_deref(), Some("RANGE-ROWS-CORE-BY-TEXT"));
    let r = t.lookup("Drop_Shadow", "shift").unwrap();
    assert_eq!((r.lanes, r.min, r.max), (2, 0.0, 1.0e6));
    let r = t.lookup("Drop_Shadow", "matte_use").unwrap();
    assert_eq!((r.min, r.max), (0.0, 1.0));
    let r = t.lookup("Drop_Shadow", "color").unwrap();
    assert_eq!((r.lanes, r.min, r.max), (3, -1.0e6, 1.0e6));
    let r = t.lookup("Hue_Sat_Bright_Autogen", "saturation").unwrap();
    assert_eq!((r.min, r.max, r.default.as_str()), (-2.0, 8.0, "1"));
    let r = t.lookup("Hue_Sat_Bright_Autogen", "hue_shift").unwrap();
    assert_eq!((r.min, r.max), (-1.0, 1.0));
    let r = t.lookup("Hue_Sat_Bright_Autogen", "offset_darks").unwrap();
    assert_eq!((r.min, r.max), (-8.0, 2.0));
    let r = t.lookup("Trails_Autogen", "new_opacity").unwrap();
    assert_eq!((r.min, r.max), (0.0, 10.0));
    // z_dist min is the f32 of 0.001 (record 0.0010000000474974513)
    let r = t.lookup("Transform_and_Blur_Autogen", "z_dist").unwrap();
    assert_eq!(r.min as f64, 0.0010000000474974513);
    assert!(t.lookup("Hue_Sat_Bright_Autogen", "saturation").is_some());
    assert!(t.lookup("Hue_Sat_Bright_Autogen", "nope").is_none());
    // every default lies inside its range (lane 0 of the text default)
    for r in &t.rows {
        if r.ptype == "boolean" || r.ptype == "popup" || r.ptype == "int" || r.ptype == "color" || r.ptype == "vector" || r.ptype == "angle" {
            continue;
        }
        let d: f32 = r.default.split_whitespace().next().unwrap().parse::<f64>().unwrap() as f32;
        assert!(d >= r.min && d <= r.max, "{} {} default {} outside [{}, {}]", r.program, r.parameter, d, r.min, r.max);
    }
}

#[test]
fn rule_nan_passes_through_with_its_bits() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    let nan = f32::from_bits(0x7fc00000);
    assert_eq!(b(clamp_lane(nan, -2.0, 8.0)), 0x7fc00000);
    let nan2 = f32::from_bits(0xffc00001);
    assert_eq!(b(clamp_lane(nan2, -2.0, 8.0)), 0xffc00001);
    assert_eq!(b(sat(nan)), 0x7fc00000);
}

#[test]
fn rule_negative_zero_is_kept() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    assert_eq!(b(clamp_lane(-0.0, -2.0, 8.0)), 0x80000000);
    assert_eq!(b(clamp_lane(-0.0, 0.0, 1.0)), 0x80000000);
    assert_eq!(b(sat(-0.0)), 0x80000000);
    assert_eq!(b(sat(0.0)), 0);
}

#[test]
fn rule_infinities_clamp_to_the_bounds() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    assert_eq!(sat(f32::INFINITY), 8.0);
    assert_eq!(sat(f32::NEG_INFINITY), -2.0);
}

#[test]
fn rule_bounds_are_exact_and_one_ulp_outside_is_clamped() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    assert_eq!(sat(-2.0), -2.0);
    assert_eq!(sat(8.0), 8.0);
    assert_eq!(sat(f32::from_bits((-2.0f32).to_bits() + 1)), -2.0); // -2 - 1 ulp
    assert_eq!(sat(f32::from_bits(8.0f32.to_bits() + 1)), 8.0); // 8 + 1 ulp
    assert_eq!(sat(f32::from_bits((-2.0f32).to_bits() - 1)), f32::from_bits((-2.0f32).to_bits() - 1)); // -2 + 1 ulp is inside
    assert_eq!(sat(-3.0), -2.0);
    assert_eq!(sat(9.0), 8.0);
    assert_eq!(sat(1.5), 1.5);
}

#[test]
fn rule_order_upper_bound_first_lower_last() {
    // run D2: degenerate record min 5 max 1: 3, -7, 9 all give 5
    for v in [3.0f32, -7.0, 9.0] {
        assert_eq!(clamp_lane(v, 5.0, 1.0), 5.0, "v = {}", v);
    }
    // the same value with the order reversed would give 1 for 9 and 5 only via min-first; pin the measured one
    assert_eq!(clamp_lane(0.0, 5.0, 1.0), 5.0);
    // ordinary range, order is irrelevant
    assert_eq!(clamp_lane(9.0, -2.0, 8.0), 8.0);
}

#[test]
fn rule_record_bound_of_plus_minus_1e6_is_not_enforced() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    // run D3: brightness [0,1e6]: 2e6 passes, -5 gives 0
    assert_eq!(clamp_lane(2.0e6, 0.0, 1.0e6), 2.0e6);
    assert_eq!(clamp_lane(-5.0, 0.0, 1.0e6), 0.0);
    // [-1e6,1e6]: scale_colors -2e6 and 3e6, Trails_Autogen prev_hue_shift -1e7 and 1e7 pass
    for v in [-2.0e6f32, 3.0e6, -1.0e7, 1.0e7] {
        assert_eq!(clamp_lane(v, -1.0e6, 1.0e6), v);
    }
    let br = bound(
        "(def_effect Hue_Sat_Bright_Autogen ( S ) ( ) ( brightness = 1 ) ( * S brightness ))",
        "Hue_Sat_Bright_Autogen",
        &[("brightness", &[2.0e6])],
        false,
    );
    assert_eq!(br["brightness"][0], 2.0e6);
    let br = bound(
        "(def_effect Hue_Sat_Bright_Autogen ( S ) ( ) ( brightness = 1 ) ( * S brightness ))",
        "Hue_Sat_Bright_Autogen",
        &[("brightness", &[-5.0])],
        false,
    );
    assert_eq!(br["brightness"][0], 0.0);
    // a bound that is NOT +-1e6 is enforced, 1e6 + 1ulp as a record bound included
    assert_eq!(clamp_lane(2.0e6, 0.0, f32::from_bits(1.0e6f32.to_bits() + 1)), f32::from_bits(1.0e6f32.to_bits() + 1));
}

#[test]
fn clamp_is_lane_wise_on_vectors() {
    let mut v = [-9.0f32, 0.5, 9.0, f32::NAN];
    let ch = clamp_lanes(&mut v, -1.0, 1.0);
    assert!(ch);
    assert_eq!(v[0], -1.0);
    assert_eq!(v[1], 0.5);
    assert_eq!(v[2], 1.0);
    assert!(v[3].is_nan());
    let mut w = [0.25f32, 0.5];
    assert!(!clamp_lanes(&mut w, -1.0, 1.0));
}

#[test]
fn compile_clamps_every_bound_scalar_before_the_plan() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    let m = bound(HSB, "Hue_Sat_Bright_Autogen", &[("saturation", &[-3.0]), ("hue_shift", &[1.5]), ("offset_darks", &[-9.0])], false);
    assert_eq!(m["saturation"][0], -2.0);
    assert_eq!(m["hue_shift"][0], 1.0);
    assert_eq!(m["offset_darks"][0], -8.0);
    let m = bound(HSB, "Hue_Sat_Bright_Autogen", &[("saturation", &[9.0]), ("hue_shift", &[-1.5]), ("offset_darks", &[3.0])], false);
    assert_eq!(m["saturation"][0], 8.0);
    assert_eq!(m["hue_shift"][0], -1.0);
    assert_eq!(m["offset_darks"][0], 2.0);
    // unbound parameters keep the program default
    let m = bound(HSB, "Hue_Sat_Bright_Autogen", &[], false);
    assert_eq!((m["saturation"][0], m["hue_shift"][0], m["offset_darks"][0]), (1.0, 0.0, 0.0));
}

#[test]
fn clamp_reaches_the_fold() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    // a constant product folds AFTER the clamp: saturation -3 clamps to -2, (-2 * 3) = -6 in the folded constant
    let g = gen("(def_effect Hue_Sat_Bright_Autogen ( S ) ( ) ( saturation = 1 ) ( * S ( * saturation 3 ) ))", "Hue_Sat_Bright_Autogen");
    let mut ov = BTreeMap::new();
    ov.insert("saturation".to_string(), vec![-3.0f32]);
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov.clone(), ..Default::default() }).unwrap();
    let c = cp.nodes.iter().find(|n| n.kind == "const" && n.folded).unwrap();
    assert_eq!(f32::from_bits(c.lanes[0]), -6.0);
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov, ranges_off: true, ..Default::default() }).unwrap();
    let c = cp.nodes.iter().find(|n| n.kind == "const" && n.folded).unwrap();
    assert_eq!(f32::from_bits(c.lanes[0]), -9.0);
}

#[test]
fn ranges_off_switch_keeps_the_raw_value() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    let m = bound(HSB, "Hue_Sat_Bright_Autogen", &[("saturation", &[-3.0]), ("hue_shift", &[1.5])], true);
    assert_eq!(m["saturation"][0], -3.0);
    assert_eq!(m["hue_shift"][0], 1.5);
}

#[test]
fn other_programs_and_unknown_parameters_are_untouched() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    // a program without rows: nothing changes
    let m = bound("(def_effect Foo ( S ) ( ) ( saturation = 1 ) ( * S saturation ))", "Foo", &[("saturation", &[-3.0])], false);
    assert_eq!(m["saturation"][0], -3.0);
    // a parameter without a row in a program with rows
    // INT4 (W224 a): a claimed program with a parameter that has no row is an ERROR, never a guess
    let g = gen("(def_effect Hue_Sat_Bright_Autogen ( S ) ( ) ( zzz = 1 ) ( * S zzz ))", "Hue_Sat_Bright_Autogen");
    let mut ov = BTreeMap::new();
    ov.insert("zzz".to_string(), vec![-3.0f32]);
    let e = compile(&g, &combos(), &CompileOptions { overrides: ov, no_fold: true, ..Default::default() }).err().expect("must be an error");
    assert!(e.msg.starts_with("RANGE-ROW-MISSING"), "{}", e.msg);
}

#[test]
fn clamp_table_values_of_the_three_new_programs() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // the expectations DF1 NEXT lists (clamp_chroma, z_dist, exposure_bias, wrap, blur_res, center)
    let p = |prog: &str, k: &str, v: f32| {
        let mut x = [v];
        clamp_param(prog, k, &mut x);
        x[0]
    };
    assert_eq!(p("Time_Warp_RGB_Autogen", "clamp_chroma", -0.5), 0.0);
    assert_eq!(p("Time_Warp_RGB_Autogen", "clamp_chroma", 1.5), 1.0);
    assert_eq!(p("Time_Warp_RGB_Autogen", "shift_frames_red", -2.0), -2.0);
    assert_eq!(p("Time_Warp_RGB_Autogen", "shift_frames_red", 31.0), 31.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "z_dist", 0.0005), 0.0010000000474974513f64 as f32);
    assert_eq!(p("Transform_and_Blur_Autogen", "z_dist", 0.0), 0.0010000000474974513f64 as f32);
    assert_eq!(p("Transform_and_Blur_Autogen", "exposure_bias", -0.2), 0.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "exposure_bias", 1.5), 1.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "wrap", 3.0), 2.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "blur_res", 3.0), 2.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "center", -0.25), 0.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "rotate", -45.0), -45.0);
    assert_eq!(p("Transform_and_Blur_Autogen", "shift", -0.5), -0.5);
    assert_eq!(p("Transform_and_Blur_Autogen", "shutter_shift", -0.5), -0.5);
    assert_eq!(p("Blur_Motion_Curves_No_Transform_Autogen", "z_dist", 0.0), 0.0010000000474974513f64 as f32);
    assert_eq!(p("Blur_Motion_Curves_No_Transform_Autogen", "center", -0.25), 0.0);
}
