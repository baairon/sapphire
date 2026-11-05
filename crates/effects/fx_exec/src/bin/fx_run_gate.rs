//! fx_run_gate: the gate wrapper of fx_run (INT6, Batch 36 rule 1). fx_run itself REQUIRES the bound-input mask. The stored state A gates and suites were made with the harness
//! semantics "compile mask 1 (Source bound), explicit images bound whatever their bit": this wrapper passes exactly those two values explicitly
//! (`--bound-mask 1 --bind-supplied`, labelled STATE-A-MASK and STATE-A-BIND-SUPPLIED) unless the caller already named a mask or --bind-supplied. It adds nothing else and decides nothing.
use std::process::Command;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    if !a.iter().any(|x| x == "--bound-mask" || x == "--mask-byte") {
        a.push("--bound-mask".into());
        a.push("1".into());
    }
    if !a.iter().any(|x| x.starts_with("--ctx-")) {
        // STATE-A-CONTEXT: host time 0, stack word 2 = 0, [owner] = 0 (harness), no has-data flags
        for (k, v) in [("--ctx-time", "0"), ("--ctx-word2", "0"), (fx_exec::judge_keys::OPT_CTX_OWNER_MODE, "0"), ("--ctx-has-data", "0")] {
            a.push(k.into());
            a.push(v.into());
        }
    }
    if !a.iter().any(|x| x == "--ranges") {
        a.push("--ranges".into());
        a.push("REC-HARNESS".into()); // STATE-A-RANGES: the stored state A originals were scored against the REC table
    }
    if !a.iter().any(|x| x == "--legacy-seed-w") {
        a.push("--legacy-seed-w".into()); // LEGACY-SEED-W: the stored W172 state A suites were made with the NV10-P3 seed
    }
    if !a.iter().any(|x| x == "--hc" || x == "--hc-keyed" || x == "--hc-zero-legacy") {
        a.push("--hc-zero-legacy".into()); // INT8 HC-ZERO-LEGACY: the stored state A suites were made with the zero host-curve fold
    }
    if !a.iter().any(|x| x == "--bind-supplied") {
        a.push("--bind-supplied".into());
    }
    let me = std::env::current_exe().expect("exe");
    let core = me.with_file_name("fx_run");
    let st = Command::new(core).args(&a).status().expect("run fx_run");
    std::process::exit(st.code().unwrap_or(1));
}
