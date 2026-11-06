//! the exec-downsampled boundary follows the plan (tail leaves the command), the two default-xy writers of the path. No env switch is set here
//! (the mutants are in t_ds3_mut.rs, one test function, because the switches are process wide).
mod t_ds3_common;
use t_ds3_common::*;
use fx_exec::exec::compile_for;

#[test]
fn plan_puts_the_tail_after_the_command_on_a_fresh_image() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    for name in [BO, TB] {
        let text = plan(name, &binding(2.0, true));
        let c = canon(&text);
        let di = c.iter().position(|l| l.contains("exec-downsampled")).unwrap();
        assert!(!c[di].contains("postdivide"), "{}: the tail is outside the command", name);
        let tl = &c[di + 1];
        assert!(tl.starts_with("CMD 6 exec-linewise (exec-linewise 1 1") || tl.contains("(exec-linewise 1 1"), "{}: {}", name, tl);
        assert!(tl.contains("allocate-images") && tl.contains("postdivide") && tl.contains("convert-to-dest") && tl.contains("free-images"), "{}: {}", name, tl);
        // fresh image: the tail output is not the command destination
        let dest = c[di].split_whitespace().nth(5).unwrap().to_string();
        assert!(c[di + 2].contains("free-images") && c[di + 2].contains(&dest), "{}: the command destination is freed after the tail", name);
        let pd = tl.split("postdivide").next().unwrap().split_whitespace().rev().nth(0).unwrap();
        assert_ne!(pd, dest, "{}: postdivide output is not in place on the command destination", name);
    }
}

#[test]
fn without_postdiv_the_tail_is_empty_and_the_plan_is_unchanged() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    for name in [BO, TB] {
        let cp = compile_for(&program(name), &binding(2.0, false)).unwrap();
        assert!(fx_eval::cpuplan::ds_tail(&cp).is_empty(), "{}", name);
        let cp1 = compile_for(&program(name), &binding(0.0, true)).unwrap();
        assert!(fx_eval::cpuplan::ds_tail(&cp1).is_empty(), "{}: no exec-downsampled command", name);
        let cp2 = compile_for(&program(name), &binding(2.0, true)).unwrap();
        assert_eq!(fx_eval::cpuplan::ds_tail(&cp2).len(), 1, "{}", name);
    }
}

#[test]
fn xy_writers_of_the_downsampled_path_bo_two_tb_one() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let ct = fx_exec::exec::combo_table();
    let (_, bo) = fx_eval::xyplace::compile_placed(&program(BO), &ct, &binding(2.0, true).compile_options()).unwrap();
    assert_eq!(bo.len(), 2, "BO at blur_res 2: the hoisted full size writer and the body writer");
    assert!(!bo[0].consumers.is_empty() && !bo[1].consumers.is_empty());
    assert!(bo[0].consumers.iter().all(|c| !bo[1].consumers.contains(c)));
    let (_, tb) = fx_eval::xyplace::compile_placed(&program(TB), &ct, &binding(2.0, true).compile_options()).unwrap();
    assert_eq!(tb.len(), 1, "TB at blur_res 2");
}

#[test]
fn alpha_one_plane_orders_agree_by_construction_of_the_tail_only() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    // alpha 1 everywhere: the tail run on the upsampled body root equals itself under a second run (determinism) and is finite
    let a = run(BO, &binding(2.0, true), 50, 7, &|_| 1.0);
    let b = run(BO, &binding(2.0, true), 50, 7, &|_| 1.0);
    assert_eq!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), b.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    assert!(a.iter().all(|x| x.is_finite()));
}
