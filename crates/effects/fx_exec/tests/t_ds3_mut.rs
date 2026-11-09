//! The order test of the exec-downsampled tail: the postdivide runs in its own command after the body (not inside it), the plane the run renders is
//! fixed by a hash of its words, and the duplicated default-xy variable has two writers. One test function: the writer statics are process wide.
mod t_ds3_common;
use t_ds3_common::*;

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

fn fnv(v: &[u32]) -> u64 {
    v.iter().fold(0xcbf29ce484222325u64, |h, w| w.to_le_bytes().iter().fold(h, |h, b| (h ^ *b as u64).wrapping_mul(0x100000001b3)))
}

#[test]
fn tail_order_and_writers() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let alpha_var = |i: usize| 0.2 + 0.8 * (((i * 5) % 11) as f32 / 10.0).min(1.0);
    let b = binding(2.0, true);
    let new_one = run(BO, &b, 50, 7, &|_| 1.0);
    let new_var = run(BO, &b, 50, 7, &alpha_var);
    let plan_new = plan(BO, &b);
    let in_cmd = |p: &str| canon(p).iter().find(|l| l.contains("exec-downsampled")).unwrap().contains("postdivide");
    assert!(!in_cmd(&plan_new), "the postdivide is a command of its own after the exec-downsampled command");
    assert!(new_one.iter().chain(new_var.iter()).all(|x| x.is_finite()));
    assert_eq!(fnv(&bits(&new_one)), HASH_ONE, "alpha 1 plane");
    assert_eq!(fnv(&bits(&new_var)), HASH_VAR, "alpha varying plane");
    // the duplicated xy variable has two writers (a full-size one for the hoisted nodes, then the body writer)
    let ct = fx_exec::exec::combo_table();
    let (_, ws) = fx_eval::xyplace::compile_placed(&program(BO), &ct, &b.compile_options()).unwrap();
    assert_eq!(ws.len(), 2);
}
const HASH_ONE: u64 = 0x7851e2c3bf0a2380;
const HASH_VAR: u64 = 0xebe14f1a10cafb86;
