//! The input index of each PARAM_INPUTS row is the position in the registered program's input list (the layout order).
//! (i) every row of every registered program of the native corpus names an input inside that list;
//! (ii) for the three Warp_Perspective_Autogen rows, with only the named input's mask bit set and an override outside the
//! row's range, the value is clamped; with the mask clear it is not.
use fx_eval::*;
use std::collections::BTreeMap;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

#[test]
fn every_row_index_is_inside_the_registered_input_list() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) || !effects_rtdata::have_tables() {
        return;
    }
    let c = Corpus::native();
    let (mut programs, mut rows, mut inside) = (0usize, 0usize, 0usize);
    for prog in programs_native::registered_names() {
        let ei = match c.find_effect(&prog, None) {
            Some(e) => e,
            None => continue,
        };
        let (g, _) = match layoutmerge::build_registered(&c, ei, &prog) {
            Ok(x) => x,
            Err(_) => continue,
        };
        programs += 1;
        for p in &g.params {
            if let Some(k) = ranges::param_input_index(&g.name, &p.name) {
                rows += 1;
                if k < g.inputs.len() {
                    inside += 1;
                }
            }
        }
    }
    println!("programs {} rows {} inside {}", programs, rows, inside);
    assert!(rows > 0);
    assert_eq!(rows, inside);
}

fn compiled_lanes(mask: u32, param: &str, val: Vec<f32>) -> Vec<f32> {
    let c = Corpus::native();
    let ei = c.find_effect("Warp_Perspective_Autogen", None).expect("program");
    let (g, _) = layoutmerge::build_registered(&c, ei, "Warp_Perspective_Autogen").expect("build");
    let mut overrides = BTreeMap::new();
    overrides.insert(param.to_string(), val);
    let o = CompileOptions { minimal_context: true, bound_input_mask: mask, overrides, ..Default::default() };
    let cp = compile(&g, &combos(), &o).expect("compile");
    cp.params.iter().find(|(n, _)| n == param).map(|(_, b)| b.iter().map(|x| f32::from_bits(*x)).collect()).expect("param")
}

#[test]
fn warp_perspective_rows_clamp_only_with_the_input_bound() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) || !effects_rtdata::have_tables() {
        return;
    }
    let c = Corpus::native();
    let ei = c.find_effect("Warp_Perspective_Autogen", None).expect("program");
    let (g, _) = layoutmerge::build_registered(&c, ei, "Warp_Perspective_Autogen").expect("build");
    let table = ranges::RangeTable::builtin();
    let (mut checked, mut clamped, mut unclamped) = (0usize, 0usize, 0usize);
    for p in &g.params {
        let k = match ranges::param_input_index(&g.name, &p.name) {
            Some(k) => k,
            None => continue,
        };
        let row = match table.lookup(&g.name, &p.name) {
            Some(r) => r.clone(),
            None => continue,
        };
        let outside = if row.max_enforced {
            row.max + 1.0
        } else if row.min_enforced {
            row.min - 1.0
        } else {
            continue;
        };
        let inlist = vec![outside; row.lanes.max(1)];
        checked += 1;
        let bound = compiled_lanes(1u32 << k, &p.name, inlist.clone());
        let mut expect = inlist.clone();
        ranges::clamp_param(&g.name, &p.name, &mut expect);
        if bound.iter().zip(expect.iter()).all(|(a, b)| a.to_bits() == b.to_bits()) && bound != inlist {
            clamped += 1;
        }
        let clear = compiled_lanes(0, &p.name, inlist.clone());
        if clear.iter().zip(inlist.iter()).all(|(a, b)| a.to_bits() == b.to_bits()) {
            unclamped += 1;
        }
    }
    println!("rows checked {} clamped_when_bound {} unclamped_when_clear {}", checked, clamped, unclamped);
    assert_eq!(checked, 3);
    assert_eq!(clamped, 3);
    assert_eq!(unclamped, 3);
}
