//! The first-set programs under the linewise make_mipmap rewrite and the per-channel Output print: no make_mipmap node of a first-set plan
//! is read only linewise (so the rewrite never applies to it), and the CPU-driver plan text of a v3_to_v4_if4 node whose third operand is
//! the destination (Drop_Shadow) prints that operand as one lane. A computed type donor keeps its per-channel print (Distort_RGB_Autogen).
use fx_eval::{build_effect, Corpus};
use fx_exec::{compile_for, Binding};

/// The programs added after the first set.
const LATER: [&str; 3] = ["Flicker", "Glow", "SwishPan"];

#[test]
fn first_set_plans_are_untouched_by_the_linewise_rewrite() {
    let corpus = Corpus::native();
    let (mut programs, mut mips) = (0, 0);
    for prog in fx_eval::programs_native::registered_names().iter().filter(|n| !LATER.contains(&n.as_str())) {
        let Some(ei) = corpus.find_effect(prog.as_str(), None) else { continue };
        let g = build_effect(&corpus, ei).unwrap();
        let mut b = Binding::defaults();
        b.hc_zero_legacy = true;
        let Ok(cp) = compile_for(&g, &b) else { continue };
        programs += 1;
        for n in cp.nodes.iter().filter(|n| n.prim == "make_mipmap" && !n.folded) {
            mips += 1;
            let f = fx_eval::cpuplan::node_format(&cp, n.idx);
            println!("{} make_mipmap node {} format {} root {}", prog, n.idx, f, n.idx == cp.root);
            assert!(n.idx == cp.root || f != 2, "{} node {}: a linewise make_mipmap would now return its base image", prog, n.idx);
        }
        let text = fx_eval::cpuplan::plan_text_mode(&cp, fx_eval::cpuplan::PlanMode::CpuDriver);
        for l in text.lines().filter(|l| l.contains("[ -999 -999")) {
            println!("MULTI {} {}", prog, l);
        }
        if prog == "Drop_Shadow" {
            for l in text.lines().filter(|l| l.contains("v3_to_v4_if4")) {
                assert!(!l.contains("[ -999 -999"), "{}: a multi-lane Output operand is printed: {}", prog, l);
            }
        }
    }
    println!("first-set programs compiled: {}; make_mipmap nodes: {}", programs, mips);
    assert!(programs >= 10, "only {} first-set programs compiled", programs);
}
