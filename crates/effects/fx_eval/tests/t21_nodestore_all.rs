//! T21: the node store 0x11b978 runs on every record the range pass visits (skip set and row-less popups included).
//! (1) a skip-path parameter compiled with history -0 and then at its default +0 keeps -0; (2) a row-less popup bound to -0 in a fresh instance with
//! a +0 default keeps +0. Test (3) (312 cases unchanged) is the examples/sw_native run recorded in the folder.
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

// Drop_Shadow-style text (as t19): invert_matte and matte_use have record input 2 (Matte, PARAM_INPUTS.tsv); bound mask 0 puts them on the skip path.
/// The Drop_Shadow test program: stored test data (`VEC_DIR/effects/fx_eval/data/TEST_DROP_SHADOW.txt`), not part of the repository.
fn ds() -> String {
    std::fs::read_to_string(effects_vecdir::path("fx_eval/data/TEST_DROP_SHADOW.txt")).expect("TEST_DROP_SHADOW.txt")
}
// a program with no range rows at all; `mode` is a popup without a row (the popup range rule path)
const PP: &str = "(def_effect T21_Popup ( Source ) ( ) ( mode = 0 ( popup A B ) ) ( * Source mode ))";

fn run(text: &str, prog: &str, hist: Option<BTreeMap<usize, Vec<u32>>>, name: &str, v: Option<f32>) -> (u32, BTreeMap<usize, Vec<u32>>) {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", text).unwrap();
    let g = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
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
fn skip_path_history_minus_zero_then_default_plus_zero_keeps_minus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    // locate invert_matte's cell by a unique value, then plant -0 as the previous compile's cell
    let (b0, h0) = run(&ds(), "Drop_Shadow", None, "invert_matte", Some(0.75));
    assert_eq!(b0, 0.75f32.to_bits());
    let key = *h0.iter().find(|(_, v)| v.first() == Some(&0.75f32.to_bits())).map(|(k, _)| k).unwrap();
    let mut h = h0.clone();
    h.insert(key, vec![NZ]);
    let (b, _) = run(&ds(), "Drop_Shadow", Some(h), "invert_matte", None);
    assert_eq!(b, NZ, "skip-path record: +0 == -0 as floats, the old cell -0 is kept");
}

#[test]
fn rowless_popup_minus_zero_fresh_instance_keeps_plus_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let (b, _) = run(PP, "T21_Popup", None, "mode", Some(-0.0));
    assert_eq!(b, PZ, "row-less popup: fresh instance old cell is the default +0, -0 does not replace it");
}
