//! T25: the owner-arm function and the native postdivide insertion test against 8 original plans
//! (fixtures/pdv1/PLAN_ORIG_*: the original compiler 0x13370 run in the emulator at ctx+0x774 = 0 and 1, plain and SMART stacks).
use fx_eval::cpuplan::{plan_text_mode, PlanMode};
use fx_eval::{build_effect, compile, CompileOptions, Corpus};
use fx_exec::hostconv14::*;
use std::path::PathBuf;

fn root() -> PathBuf {
    effects_vecdir::root()
}

/// command texts with reference names and trailing blanks normalised; the host-side cmd-0x1aa0500 entries of the dump are not plan commands
fn norm(cmds: &[String]) -> Vec<String> {
    cmds.iter()
        .map(|c| {
            let t: Vec<String> = c
                .split_whitespace()
                .map(|w| if w.starts_with(":ref") { format!(":ref{}", &w[w.find(')').unwrap_or(w.len())..]) } else { w.to_string() })
                .collect();
            t.join(" ").replace(" )", ")")
        })
        .collect()
}

fn orig_cmds(file: &str) -> Vec<String> {
    let t = std::fs::read_to_string(root().join("fixtures").join("pdv1").join(file)).unwrap();
    t.lines().filter(|l| l.starts_with("CMD\t")).map(|l| l.split('\t').nth(3).unwrap_or("").to_string()).filter(|c| c.starts_with('(')).collect()
}

fn native_cmds(prog: &str, do_postdiv: bool) -> Vec<String> {
    let p = root().join("record_extract").join("programs").join("programs.tsv");
    let c = Corpus::from_programs_tsv(&std::fs::read_to_string(p).unwrap());
    let g = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
    let mut o = CompileOptions::default();
    o.const_inputs.insert("Mask".into(), vec![1.0]);
    o.do_postdiv = Some(do_postdiv);
    let cp = compile(&g, &fx_exec::exec::combo_table(), &o).unwrap();
    plan_text_mode(&cp, PlanMode::GpuFirst)
        .lines()
        .filter(|l| l.starts_with("CMD\t"))
        .map(|l| l.split('\t').nth(3).unwrap_or("").to_string())
        .filter(|c| c.starts_with('('))
        .collect()
}

#[test]
fn t25_owner_arm_cells() {
    let a = |o, b, x| OwnerArm { owner_arm_nonzero: o, owner_arm_bit2: b, mode_byte_exit: x, opacity_bits: None, output_opacity_bits: None };
    // v = 1 iff [owner+0x13e0] != 0 and bit 2 clear: (v, not v, v)
    assert_eq!(owner_arm(&a(true, false, false)), ArmCells { ctx_gate_a: 1, ctx_gate_b: 0, ctx_postdiv_cell: 1 });
    assert_eq!(owner_arm(&a(true, true, false)), ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 0 });
    assert_eq!(owner_arm(&a(false, false, false)), ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 0 });
    assert_eq!(owner_arm(&a(false, true, false)), ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 0 });
    // the mode-byte-2 exit: constructor defaults (0, 1, 1)
    // the exit is withdrawn: refused by name; the recorded no-loop form keeps its old answer only for the comparison test
    for (o, b) in [(false, false), (true, false), (false, true), (true, true)] {
        assert!(fx_exec::hostconv14::try_owner_arm(&a(o, b, true)).is_err());
    }
    // variant B iff v = 0 (outside the exit), postdivide iff v = 1; the owner arm never gives the refused (0, 0) gate state
    assert_eq!(derived_head(4, &SourceGate::from_arm(&a(true, false, false), 0)).unwrap(), R4_HEAD);
    assert!(!derived_do_postdiv(&a(false, false, false)));
    assert_eq!(derived_head(4, &SourceGate::from_arm(&a(false, false, false), 0)).unwrap(), VB4_HEAD);
    assert!(derived_do_postdiv(&a(true, false, false)));
    for o in [false, true] {
        for b in [false, true] {
            for x in [false] {
                let c = owner_arm(&a(o, b, x));
                assert!(!(c.ctx_gate_a == 0 && c.ctx_gate_b == 0));
            }
        }
    }
}

#[test]
fn t25_postdivide_insertion_matches_pdv1_plans() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let mut checked = 0;
    for prog in ["Blur_Color", "Diffuse"] {
        for pd in [false, true] {
            let nat = norm(&native_cmds(prog, pd));
            for stack in ["plain", "smart"] {
                let f = format!("PLAN_ORIG_{}_pd{}_{}.txt", prog, pd as u8, stack);
                let org = norm(&orig_cmds(&f));
                assert_eq!(nat, org, "{}: native plan differs from the original's", f);
                let k = org.iter().position(|c| c.contains(" postdivide ")).map(|k| k as i64).unwrap_or(-1);
                let d = org.iter().position(|c| c.starts_with("(convert-to-dest")).unwrap() as i64;
                if pd {
                    assert_eq!(k, d - 1, "{}: the postdivide primitive sits right before convert-to-dest", f);
                    assert_eq!(org[k as usize], "(run-primitive 1 -1 :ref postdivide :ref [ -1 ])");
                } else {
                    assert_eq!(k, -1);
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 8);
}

/// the Opacity loop. Blur_Color records rcD 64x32 and 50x7 (and pdBC):
/// [owner+0x13e0] = 0, [owner+0x1260] = 18 (bit 2 clear), Opacity record 1.0 (popup answer 2, the default), ctx+0x774 = 1 and owner cells
/// (1354, 1358, 1398, 139c) = (1, 0, 1, 1). The arm with the loop must give the recorded ctx cell; the recorded arm without it must not.
#[test]
fn t25_opacity_loop_bcj2_blur_color() {
    let rec = OwnerArm { owner_arm_nonzero: false, owner_arm_bit2: (18u32 & 4) != 0, mode_byte_exit: false, opacity_bits: Some(1.0f32.to_bits()), output_opacity_bits: None };
    let with = owner_arm(&rec);
    assert_eq!(with.ctx_postdiv_cell, 1, "with the loop: recorded ctx+0x774 = 1");
    assert_eq!(with, ArmCells { ctx_gate_a: 1, ctx_gate_b: 0, ctx_postdiv_cell: 1 });
    let without = fx_exec::hostconv14::owner_arm_no_loop(&rec);
    assert_ne!(without.ctx_postdiv_cell, 1, "without the loop the arm must fail the recorded cell");
    // the owner cells themselves (setup_arm) equal record
    let o = fx_exec::owner_loop::setup_arm(false, false, &[fx_exec::owner_loop::ParamView { name: "opacity".into(), kind: 4, value: 1.0 }]).unwrap();
    assert_eq!((o.ae32_ready_c, o.ae32_ready_d, o.late_out_c, o.late_out_d), (1, 0, 1, 1));
    // the value classes of rule 4: m = 0 (answer 1) gives 1358 = 3 and v = 0; m = 2 (answer 3) v = 0; Diffuse (no opacity parameter) v = 0
    let m0 = owner_arm(&OwnerArm { opacity_bits: Some(0.0f32.to_bits()), ..rec });
    assert_eq!(m0, ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 0 });
    let m2 = owner_arm(&OwnerArm { opacity_bits: Some(2.0f32.to_bits()), ..rec });
    assert_eq!(m2, ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 0 });
    assert_eq!(owner_arm(&OwnerArm { opacity_bits: None, ..rec }), ArmCells { ctx_gate_a: 0, ctx_gate_b: 1, ctx_postdiv_cell: 0 });
    // bit 2 set: the loop leaves v = 0
    assert_eq!(owner_arm(&OwnerArm { owner_arm_bit2: true, ..rec }).ctx_postdiv_cell, 0);
    // NaN refused by name
    assert!(fx_exec::hostconv14::try_owner_arm(&OwnerArm { opacity_bits: Some(f32::NAN.to_bits()), ..rec }).is_err());
    // never the refused gate state (0, 0)
    for v in [-1.0f32, 0.0, 0.49, 0.5, 1.0, 1.49, 2.0, 3.0] {
        for b in [false, true] {
            let c = owner_arm(&OwnerArm { owner_arm_bit2: b, opacity_bits: Some(v.to_bits()), ..rec });
            assert!(!(c.ctx_gate_a == 0 && c.ctx_gate_b == 0));
        }
    }
}
