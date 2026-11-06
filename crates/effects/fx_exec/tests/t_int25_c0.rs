//! the c0 control on fx_run's 'ae32' compile path. c0 text (Blend, the unchanged node
//! `( if_neq_const Matte 1 m0 default_matte )`) with Matte unbound: fx_run binds an unbound input whose text default is a number as that
//! constant (Binding.const_inputs, fx_run.rs, the rule 1 loop), then compiles through fx_exec::compile_for. The original ran this state
//! with flags 1 and 1, counts 1 and 1 and arm 3 (PROBE_RESULTS.tsv row c0, MEASURED there). A different arm is a finding.
use fx_eval::{build_effect, Corpus};
use fx_exec::{compile_for, Binding};

// the trace is process global and the test harness runs tests on parallel threads: one lock serialises the compiles of this file
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn blend_c0() -> fx_eval::Genotype {
    let text = std::fs::read_to_string(effects_vecdir::path("fx_eval/tests/eqc1_probe/progs_c0.tsv")).unwrap();
    let c = Corpus::from_programs_tsv(&text);
    build_effect(&c, c.find_effect("Blend", None).unwrap()).unwrap()
}

/// fx_run's binding of the declared inputs: bit k set binds an image (4 planes on the 'ae32' R4 route); bit k clear with a numeric text
/// default binds the constant.
fn claim_binding(g: &fx_eval::Genotype, mask: u32) -> Binding {
    let mut b = Binding::defaults();
    b.bound_input_mask = mask;
    for (k, inp) in g.inputs.iter().enumerate() {
        if (mask >> k) & 1 == 1 {
            b.input_elnum.insert(inp.name.clone(), 4);
            continue;
        }
        if let Some(v) = inp.default.as_deref().and_then(fx_eval::orig_number_pub) {
            b.const_inputs.insert(inp.name.clone(), vec![v as f32]);
        }
    }
    b
}

#[test]
fn c0_matte_unbound_flags_1_1_counts_1_1_arm_3() {
    if !effects_vecdir::have_all(&["fx_eval/tests/eqc1_probe/progs_c0.tsv"]) {
        return;
    }
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let g = blend_c0();
    let names: Vec<&str> = g.inputs.iter().map(|i| i.name.as_str()).collect();
    assert_eq!(names, vec!["SourceA", "SourceB", "Matte"]);
    let b = claim_binding(&g, 0b011);
    assert_eq!(b.const_inputs.get("Matte"), Some(&vec![1.0f32]));
    let _ = fx_eval::compile::ifneq_trace_take();
    let cp = compile_for(&g, &b).expect("compile");
    let tr = fx_eval::compile::ifneq_trace_take();
    assert_eq!(tr.len(), 1, "{:?}", tr);
    let (_, f0, f1, n0, n1, arm) = tr[0];
    assert_eq!((f0 as u8, f1 as u8, n0, n1, arm), (1, 1, 1, 1, Some(3)), "{:?}", tr);
    assert!(cp.plan_tsv().contains("ifneqc\t3"), "{}", cp.plan_tsv());
    println!("C0-CONTROL\tflags {} {}\tcounts {} {}\tarm {:?}", f0 as u8, f1 as u8, n0, n1, arm);
}

#[test]
fn c0_matte_bound_as_image_takes_arm_2() {
    if !effects_vecdir::have_all(&["fx_eval/tests/eqc1_probe/progs_c0.tsv"]) {
        return;
    }
    // discrimination half: Matte bound as a 4-plane image is not constant, so the flag gate gives arm 2 (c6 shape)
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let g = blend_c0();
    let b = claim_binding(&g, 0b111);
    let _ = fx_eval::compile::ifneq_trace_take();
    let cp = compile_for(&g, &b).expect("compile");
    let tr = fx_eval::compile::ifneq_trace_take();
    assert_eq!(tr.len(), 1, "{:?}", tr);
    assert!(!tr[0].1, "{:?}", tr);
    assert_eq!(tr[0].5, Some(2), "{:?}", tr);
    assert!(cp.plan_tsv().contains("ifneqc\t2"), "{}", cp.plan_tsv());
}
