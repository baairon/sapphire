//! fx_exec_cli coverage <programs.tsv> : for the milestone programs, every branch set, the run list classes and the
//! classes this executor cannot run.
use fx_eval::{build_effect, compile, enumerate_branches, Corpus};
use std::collections::BTreeMap;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 || a[1] != "coverage" {
        eprintln!("usage: fx_exec_cli coverage <programs.tsv> [program ...]");
        std::process::exit(2);
    }
    let path = a[2].clone();
    let names: Vec<String> = if a.len() > 3 { a[3..].to_vec() } else { ["Hue_Sat_Bright", "Composite", "Diffuse", "Trails", "Time_Warp_RGB"].iter().map(|s| s.to_string()).collect() };
    let h = std::thread::Builder::new().stack_size(256 << 20).spawn(move || run(&path, &names)).unwrap();
    std::process::exit(h.join().unwrap());
}

fn run(path: &str, names: &[String]) -> i32 {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {}: {}", path, e);
            return 2;
        }
    };
    let c = Corpus::from_programs_tsv(&text);
    let ct = fx_exec::exec::combo_table();
    println!("program\tparamset\trun_list\tunsupported");
    for name in names {
        let ei = match c.find_effect(name, None) {
            Some(i) => i,
            None => {
                println!("{}\t-\t-\teffect not found", name);
                continue;
            }
        };
        let g = match build_effect(&c, ei) {
            Ok(g) => g,
            Err(e) => {
                println!("{}\t-\t-\tbuild error {}", name, e);
                continue;
            }
        };
        for s in fx_eval::enumerate_branches_opt(&g, &ct, &BTreeMap::new(), std::env::args().any(|x| x == "--hc-zero-legacy")) {
            let o = fx_eval::CompileOptions { overrides: s.overrides.clone(), paramset: s.paramset.clone(), hc_zero_legacy: std::env::args().any(|x| x == "--hc-zero-legacy"), ..Default::default() };
            match compile(&g, &ct, &o) {
                Ok(cp) => {
                    let run: Vec<String> = cp.nodes.iter().filter(|n| (n.kind == "func" || n.kind == "auto") && !n.b160 && !n.folded).map(|n| n.prim.clone()).collect();
                    let un = fx_exec::unsupported_classes(&cp);
                    println!("{}\t{}\t{}\t{}", name, s.paramset, run.join(" "), if un.is_empty() { "-".to_string() } else { un.join(" ") });
                }
                Err(e) => println!("{}\t{}\t-\tcompile error {}", name, s.paramset, e),
            }
        }
    }
    0
}
