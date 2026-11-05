//! T19 (INT7 step 2, NZ1 PATCH 3 a to g): the node store with the compile history. Program: a Drop_Shadow-style text with `shadow_opacity` (REAL row [0, 1e6], default 1).
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

/// The Drop_Shadow test program: stored test data (`VEC_DIR/effects/fx_eval/data/TEST_DROP_SHADOW.txt`), not part of the repository.
fn ds() -> String {
    std::fs::read_to_string(effects_vecdir::path("fx_eval/data/TEST_DROP_SHADOW.txt")).expect("TEST_DROP_SHADOW.txt")
}

fn run(hist: Option<BTreeMap<usize, Vec<u32>>>, name: &str, v: Option<f32>) -> (u32, BTreeMap<usize, Vec<u32>>) {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", &ds()).unwrap();
    let ei = c.find_effect("Drop_Shadow", None).unwrap();
    let g = build_effect(&c, ei).unwrap();
    let mut ov = BTreeMap::new();
    if let Some(x) = v {
        ov.insert(name.to_string(), vec![x]);
    }
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov, no_fold: true, minimal_context: true, history: hist, ..Default::default() }).unwrap();
    let b = cp.params.iter().find(|(n, _)| n == name).map(|(_, l)| l[0]).unwrap();
    (b, cp.cells)
}

const PZ: u32 = 0x0000_0000;
const NZ: u32 = 0x8000_0000;

#[test]
fn a_old_plus_zero_new_minus_zero_keeps_plus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (b1, h1) = run(None, "shadow_opacity", Some(0.0));
    assert_eq!(b1, PZ);
    let (b2, _) = run(Some(h1), "shadow_opacity", Some(-0.0));
    assert_eq!(b2, PZ);
}
#[test]
fn b_old_one_new_minus_zero_stores_minus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (b1, h1) = run(None, "shadow_opacity", Some(1.0));
    assert_eq!(b1, 0x3f80_0000);
    let (b2, _) = run(Some(h1), "shadow_opacity", Some(-0.0));
    assert_eq!(b2, NZ);
}
#[test]
fn c_old_minus_zero_new_negative_clamps_to_plus_zero_keeps_minus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (_, h1) = run(None, "shadow_opacity", Some(-0.0));
    let (b2, _) = run(Some(h1), "shadow_opacity", Some(-0.5));
    assert_eq!(b2, NZ);
}
#[test]
fn d_old_minus_zero_new_plus_zero_keeps_minus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (_, h1) = run(None, "shadow_opacity", Some(-0.0));
    let (b2, _) = run(Some(h1), "shadow_opacity", Some(0.0));
    assert_eq!(b2, NZ);
}
#[test]
fn e_nan_is_stored_in_both_directions() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (b1, h1) = run(None, "shadow_opacity", Some(f32::NAN));
    assert!(f32::from_bits(b1).is_nan());
    let (b2, h2) = run(Some(h1), "shadow_opacity", Some(0.0));
    assert_eq!(b2, PZ);
    let (b3, _) = run(Some(h2), "shadow_opacity", Some(f32::NAN));
    assert!(f32::from_bits(b3).is_nan());
}
#[test]
fn f_sentinel_bounds_are_not_enforced() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // Drop_Shadow.color: record [-1e6, 1e6], min_enforced and max_enforced both N: a value beyond the sentinel passes
    let mut r = [3.0e6f32, -3.0e6];
    assert_eq!(ranges::clamp_param("Drop_Shadow", "color", &mut r), Some(false));
    assert_eq!(r, [3.0e6, -3.0e6]);
    // an enforced max (shadow_opacity has max 1e6 not enforced; fg_opacity has max 1 enforced)
    let mut s = [2.0f32];
    assert_eq!(ranges::clamp_param("Drop_Shadow", "fg_opacity", &mut s), Some(true));
    assert_eq!(s, [1.0]);
}
#[test]
fn g_fresh_instance_default_one_and_minus_zero_stores_minus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (b, _) = run(None, "shadow_opacity", Some(-0.0));
    assert_eq!(b, NZ);
}
#[test]
fn node_store_unit() {
    let mut n = [-0.0f32, f32::NAN, 2.0];
    ranges::node_store(&[0.0, 1.0, 2.0], &mut n);
    assert_eq!(n[0].to_bits(), PZ);
    assert!(n[1].is_nan());
    assert_eq!(n[2], 2.0);
}
