//! DS3 mutants and the order test. One test function: the switches DS3_MUT and XWP2_MUT are process wide.
mod t_ds3_common;
use t_ds3_common::*;

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn mutants_and_orders() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let alpha_var = |i: usize| 0.2 + 0.8 * (((i * 5) % 11) as f32 / 10.0).min(1.0);
    let b = binding(2.0, true);
    // new order
    let new_one = run(BO, &b, 50, 7, &|_| 1.0);
    let new_var = run(BO, &b, 50, 7, &alpha_var);
    let plan_new = plan(BO, &b);
    let plan_new_tb = plan(TB, &b);
    // mutant DS3_MUT=inbody: the old order and the old plan
    std::env::set_var("DS3_MUT", "inbody");
    let old_one = run(BO, &b, 50, 7, &|_| 1.0);
    let old_var = run(BO, &b, 50, 7, &alpha_var);
    let plan_old = plan(BO, &b);
    let plan_old_tb = plan(TB, &b);
    let plan_old_nopd = plan(BO, &binding(2.0, false));
    std::env::remove_var("DS3_MUT");
    let plan_nopd = plan(BO, &binding(2.0, false));
    assert_eq!(canon(&plan_old_nopd), canon(&plan_nopd), "do_postdiv false: the mutant changes nothing");
    assert_ne!(canon(&plan_new), canon(&plan_old));
    assert_ne!(canon(&plan_new_tb), canon(&plan_old_tb));
    let in_cmd = |p: &str| canon(p).iter().find(|l| l.contains("exec-downsampled")).unwrap().contains("postdivide");
    assert!(!in_cmd(&plan_new) && in_cmd(&plan_old), "the mutant restores the postdivide inside the command (the s32_g3 text)");
    // alpha-1 plane: both orders agree
    let maxrel = |a: &[f32], b: &[f32]| a.iter().zip(b).map(|(x, y)| (x - y).abs() / x.abs().max(y.abs()).max(1e-6)).fold(0.0f32, f32::max);
    let (r1, rv) = (maxrel(&new_one, &old_one), maxrel(&new_var, &old_var));
    eprintln!("DS3 orders: alpha 1 max relative difference {:e} ({} words differ in bits), alpha varying {:e} ({} words differ)", r1, new_one.iter().zip(&old_one).filter(|(a, b)| a.to_bits() != b.to_bits()).count(), rv, new_var.iter().zip(&old_var).filter(|(a, b)| a.to_bits() != b.to_bits()).count());
    assert!(r1 < 1e-3, "alpha 1: the orders agree to the last bits (the up_plane output of an alpha that is 1 up to rounding); got {}", r1);
    // alpha-varying plane: the orders differ
    assert!(rv > 1e-2 && rv > 50.0 * r1, "alpha varying: the orders differ; got {}", rv);
    assert_ne!(bits(&new_var), bits(&old_var));
    assert!(new_var.iter().all(|x| x.is_finite()));
    // XWP2_MUT=dsone drops the second writer
    let ct = fx_exec::exec::combo_table();
    std::env::set_var("XWP2_MUT", "dsone");
    let (_, one) = fx_eval::xyplace::compile_placed(&program(BO), &ct, &b.compile_options()).unwrap();
    std::env::remove_var("XWP2_MUT");
    assert_eq!(one.len(), 1);
}
