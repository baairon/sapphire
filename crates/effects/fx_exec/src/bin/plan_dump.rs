//! plan_dump <programs.tsv> <program> [--hc] [--cpu (r8 = 0 CpuDriver plan text)] [name=v1,v2 ...]: prints the native plan (node table and plan text) of one branch set.
use fx_eval::cpuplan::{plan_text_mode, PlanMode};
use fx_eval::{build_effect, compile, Corpus};
use std::collections::BTreeMap;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let h = std::thread::Builder::new().stack_size(256 << 20).spawn(move || run(a)).unwrap();
    std::process::exit(h.join().unwrap());
}

fn run(a: Vec<String>) -> i32 {
    if a.len() < 3 {
        eprintln!("usage: plan_dump <programs.tsv> <program> [--hc] [--cpu (r8 = 0 CpuDriver plan text)] [name=v1,v2 ...]");
        return 2;
    }
    let text = std::fs::read_to_string(&a[1]).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&text);
    let ei = c.find_effect(&a[2], None).expect("effect");
    let g = build_effect(&c, ei).expect("build");
    // plan_dump has only the state A semantics (Mask = 1 constant, no compile context); it refuses to run unless the named option is passed
    if !a.iter().any(|x| x == "--state-a-defaults") {
        eprintln!("plan_dump: refused: only the named option --state-a-defaults exists (Mask := 1, no compile context); not usable in counted runs");
        return 2;
    }
    eprintln!("plan_dump: STATE-A-DEFAULTS (Mask := 1, no compile context; not a counted run)");
    let mut o = fx_eval::CompileOptions::default();
    let mut mode = PlanMode::GpuFirst;
    o.const_inputs.insert("Mask".into(), vec![1.0]);
    for s in &a[3..] {
        if s == "--cpu" {
            mode = PlanMode::CpuDriver;
        } else if s == "--ranges-off" {
            o.ranges_off = true;
        } else if let Some(v) = s.strip_prefix("--src-chan=") {
            // Source (and Foreground) bound with N channels, as fx_run's --src-chan (compiles at Source 3 channels)
            let k: i32 = v.parse().expect("--src-chan=N");
            o.input_elnum.insert("Source".into(), k);
            o.input_elnum.insert("Foreground".into(), k);
        } else if s == "--gid" {
        } else if s == "--hc" {
            o.host_curves_unkeyed = true;
        } else if s == "--hc-zero-legacy" {
            o.hc_zero_legacy = true;
        } else if let Some((k, v)) = s.split_once('=') {
            let lanes: Vec<f32> = v.split(',').map(|x| x.parse().unwrap()).collect();
            o.overrides.insert(k.to_string(), lanes);
        }
    }
    let ct = fx_exec::exec::combo_table();
    let _ = fx_eval::fold::v3_take();
    let _ = fx_eval::compile::eq_const_scan_take();
    let cp = compile(&g, &ct, &o).expect("compile");
    let (eqc, eqd) = fx_eval::compile::eq_const_scan_take();
    eprintln!("EQCONST\tcalls={}\tdiff={}\tbit4={}", eqc, eqd, fx_eval::layout::program_flag_bit4(&g) as u8);
    println!("root={} nodes={}", cp.root, cp.nodes.len());
    for n in &cp.nodes {
        println!(
            "N{}\t{}\t{}\tprim={}\tel={}\targs={:?}\tconst={}\tfolded={}\tlanes={:08x},{:08x},{:08x},{:08x}\tparam={:?}",
            n.idx, n.kind, n.name, n.prim, n.elnum, n.args, n.b160, n.folded, n.lanes[0], n.lanes[1], n.lanes[2], n.lanes[3], n.param
        );
    }
    println!("{}", plan_text_mode(&cp, mode));
    // with --gid, the genotype id and program-text path of every plan node, the compile's fold list and its v3_to_v4_if4 folds
    if a.iter().any(|x| x == "--gid") {
        for n in &cp.nodes {
            println!("GID	N{}	gid {}	{}", n.idx, n.gid, fx_exec::ops_census::node_path(&cp, n.idx));
        }
        for f in &cp.folds {
            println!("FOLD	{}", f);
        }
        for x in fx_eval::fold::v3_take() {
            println!("V3FOLD	gid {}	n {}	arg0 {}	arg1 {}	result {}	form {}	path {}", x.0, x.1, x.6.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), x.3.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), x.4.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), x.5, fx_exec::ops_census::node_path_fold(&cp, &g, x.0));
        }
    }
    let _ = BTreeMap::<String, String>::new();
    0
}
