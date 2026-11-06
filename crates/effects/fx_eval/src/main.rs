//! fx_eval_cli: run the reader over the captured program texts.
//!   fx_eval_cli all <programs.tsv> <PROGRAM_FORMS.tsv> <outdir>
//!   fx_eval_cli one <programs.tsv> <effect name> [va]

use fx_eval::{build_effect, genotype_tsv, Corpus};
use std::collections::{BTreeMap, HashMap};
use std::fs;

fn main() {
    // deep programs: run on a thread with a large stack
    let h = std::thread::Builder::new().stack_size(256 << 20).spawn(real_main).expect("thread");
    let code = h.join().unwrap_or(2);
    std::process::exit(code);
}

fn real_main() -> i32 {
    let a: Vec<String> = std::env::args().collect();
    if let Some(r) = fx_eval::ranges::range_set_from_args(&a) {
        match r {
            Ok(s) => {
                let _ = fx_eval::ranges::set_range_set(s);
            }
            Err(e) => {
                eprintln!("{}", e);
                return 2;
            }
        }
    }
    if a.len() < 3 {
        eprintln!("usage: fx_eval_cli all <programs.tsv> <forms.tsv> <outdir> | one <programs.tsv> <name> [va]");
        return 2;
    }
    match a[1].as_str() {
        "all" if a.len() >= 5 => all(&a[2], &a[3], &a[4]),
        "one" if a.len() >= 4 => one(&a[2], &a[3], a.get(4).map(|s| s.as_str()).filter(|s| !s.starts_with("--"))),
        "compile" if a.len() >= 5 => compile_one(&a[2], &a[3], &a[4], a.get(5).map(|s| s.as_str()).filter(|s| !s.starts_with("--"))),
        "eln" if a.len() >= 5 => eln(&a[2], &a[3], &a[4]),
        "plancmp" if a.len() >= 6 => plancmp(&a[2], &a[3], &a[4], &a[5], a.iter().any(|x| x == "--hc-unkeyed"), a.iter().position(|x| x == "--hc-keyed").and_then(|i| a.get(i + 1)).map(|s| s.as_str()), if a.iter().any(|x| x == "cpu") { fx_eval::cpuplan::PlanMode::CpuDriver } else { fx_eval::cpuplan::PlanMode::GpuFirst }),
        "lgtest" if a.len() >= 4 => lgtest(&a[2], &a[3]),
        "mask-table" if a.len() >= 3 => mask_table(&a[2]),
        "diff-plans" if a.len() >= 4 => diff_plans(&a[2], &a[3]),
        "xyplan" if a.len() >= 5 => xyplan(&a[2], &a[3], &a[4], a.get(5).map(|s| s.as_str()).unwrap_or("")),
        "plan" if a.len() >= 5 => plan_one(&a[2], &a[3], &a[4], a.get(5).map(|s| s.as_str()).unwrap_or("")),
        "compile-all" if a.len() >= 6 => compile_all(&a[2], &a[3], &a[4], &a[5]),
        _ => {
            eprintln!("bad arguments");
            2
        }
    }
}

/// `--presence-mask HEX` (INPUT-PRESENCE-MASK, one native input), `--input-chan NAME=N` (repeatable; the channel count of an input
/// whose presence bit is set, a host input with no default) and the named option `--mixed-mask` (attribution only).
fn presence_args() -> Result<(Option<u8>, BTreeMap<String, i32>, bool), String> {
    let a: Vec<String> = std::env::args().collect();
    let pm = match a.iter().position(|x| x == "--presence-mask") {
        Some(p) => Some(u8::from_str_radix(a.get(p + 1).ok_or("--presence-mask needs a value")?.trim_start_matches("0x"), 16).map_err(|_| "--presence-mask must be one hex byte".to_string())?),
        None => None,
    };
    let mut ch = BTreeMap::new();
    for (i, x) in a.iter().enumerate() {
        if x == "--input-chan" {
            let v = a.get(i + 1).ok_or("--input-chan needs NAME=N")?;
            let (n, c) = v.split_once('=').ok_or("--input-chan needs NAME=N")?;
            ch.insert(n.to_string(), c.parse::<i32>().map_err(|_| "--input-chan count must be an integer".to_string())?);
        }
    }
    Ok((pm, ch, a.iter().any(|x| x == "--mixed-mask")))
}

/// the bit-order table of the INPUT-PRESENCE-MASK for every effect of the corpus (program, position, input, default, bind-pass node index, bind bit, range bit).
fn mask_table(programs: &str) -> i32 {
    let text = match fs::read_to_string(programs) {
        Ok(t) => t,
        Err(_) => return 2,
    };
    let c = Corpus::from_programs_tsv(&text);
    println!("program\tva\tposition\tinput\tdefault\tbind_node_index_INFERRED\tbind_pass_bit_\trange_pass_bit_");
    for (ei, e) in c.effects.iter().enumerate() {
        let Ok(g) = build_effect(&c, ei) else {
            println!("{}\t{}\tBUILD_FAIL\t\t\t\t\t", e.name, e.va);
            continue;
        };
        for (j, n, d, k, b, r) in fx_eval::compile::presence_bit_rows(&g) {
            let f = |x: Option<usize>| x.map(|v| v.to_string()).unwrap_or_else(|| "none".into());
            println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", e.name, e.va, j, n, d, k, f(b), f(r));
        }
    }
    0
}

/// rule 1: the bound-input mask of the CLI, `--bound-mask HEX` (STATE-A-MASK when a gate passes it); None when missing or unparsable.
fn mask_arg() -> Option<u32> {
    let a: Vec<String> = std::env::args().collect();
    let p = a.iter().position(|x| x == "--bound-mask")?;
    u32::from_str_radix(a.get(p + 1)?.trim_start_matches("0x"), 16).ok()
}

/// (step L2): `--layout-merge` builds the REGISTERED program (the layout row merged as fx_run does: layout order, `:extra`, `:hardcode`, the
/// layout input group) for a program that has a layout row; without it the def path is unchanged (the existing plan sets were recorded from the def text).
fn layout_merge_flag() -> bool {
    std::env::args().any(|x| x == "--layout-merge")
}

fn build_cli(c: &Corpus, ei: usize, name: &str) -> Result<fx_eval::Genotype, String> {
    if layout_merge_flag() {
        fx_eval::layoutmerge::build_registered(c, ei, name).map(|x| x.0)
    } else {
        build_effect(c, ei).map_err(|e| e.to_string())
    }
}

fn one(programs: &str, name: &str, va: Option<&str>) -> i32 {
    let text = match fs::read_to_string(programs) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {}: {}", programs, e);
            return 2;
        }
    };
    let c = Corpus::from_programs_tsv(&text);
    let ei = match c.find_effect(name, va) {
        Some(i) => i,
        None => {
            eprintln!("effect {} not found (parse failures: {})", name, c.failures.len());
            return 1;
        }
    };
    match build_cli(&c, ei, name) {
        Ok(g) => {
            print!("{}", genotype_tsv(&g));
            if layout_merge_flag() {
                // both names of every record and input
                print!("{}", fx_eval::layoutmerge::names_tsv(&g));
            }
            0
        }
        Err(e) => {
            eprintln!("{}: {}", name, e);
            1
        }
    }
}

fn all(programs: &str, forms: &str, outdir: &str) -> i32 {
    let text = match fs::read_to_string(programs) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {}: {}", programs, e);
            return 2;
        }
    };
    let ftext = match fs::read_to_string(forms) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {}: {}", forms, e);
            return 2;
        }
    };
    let c = Corpus::from_programs_tsv(&text);
    if let Err(e) = fs::create_dir_all(outdir) {
        eprintln!("cannot create {}: {}", outdir, e);
        return 2;
    }
    // PROGRAM_FORMS rows: name va target30 ...
    let mut listed: Vec<(String, String, bool)> = Vec::new();
    for (i, line) in ftext.lines().enumerate() {
        if i == 0 || line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 3 {
            continue;
        }
        listed.push((f[0].to_string(), f[1].to_string(), f[2] == "1"));
    }
    let mut main_tsv = String::new();
    let mut other_tsv = String::new();
    let mut status = String::from("name\tva\ttarget30\tstatus\tnodes\tfunctions\tinputs\tparams\terror\n");
    let mut heads: BTreeMap<String, usize> = BTreeMap::new();
    let (mut ok, mut bad, mut tok, mut tbad) = (0usize, 0usize, 0usize, 0usize);
    let mut wd = 0usize;
    let mut listed_set = std::collections::HashSet::new();
    for (name, va, target) in &listed {
        listed_set.insert((name.clone(), va.clone()));
        let ei = c.find_effect(name, Some(va));
        match ei.map(|i| build_effect(&c, i)) {
            Some(Ok(g)) => {
                ok += 1;
                wd += g.word_defaults.len();
                if *target {
                    tok += 1;
                }
                for n in &g.nodes {
                    if n.kind == fx_eval::NodeKind::Function {
                        *heads.entry(n.name.clone()).or_insert(0) += 1;
                    }
                }
                status.push_str(&format!(
                    "{}\t{}\t{}\tOK\t{}\t{}\t{}\t{}\t\n",
                    name,
                    va,
                    *target as u8,
                    g.nodes.len(),
                    g.function_names().len(),
                    g.inputs.len(),
                    g.params.len()
                ));
                main_tsv.push_str(&genotype_tsv(&g));
            }
            Some(Err(e)) => {
                bad += 1;
                if *target {
                    tbad += 1;
                }
                status.push_str(&format!("{}\t{}\t{}\tBUILD_FAIL\t\t\t\t\t{}\n", name, va, *target as u8, e));
            }
            None => {
                bad += 1;
                if *target {
                    tbad += 1;
                }
                let why = c
                    .failures
                    .iter()
                    .find(|f| f.name == *name && f.va == *va)
                    .map(|f| f.msg.clone())
                    .unwrap_or_else(|| "effect not in corpus".into());
                status.push_str(&format!("{}\t{}\t{}\tPARSE_FAIL\t\t\t\t\t{}\n", name, va, *target as u8, why));
            }
        }
    }
    // the other def_effect rows (Autogen wrappers and unnamed rows): informational
    let (mut ook, mut obad) = (0usize, 0usize);
    let mut other_status = String::from("name\tva\tstatus\tnodes\terror\n");
    for (i, e) in c.effects.iter().enumerate() {
        if listed_set.contains(&(e.name.clone(), e.va.clone())) {
            continue;
        }
        match build_effect(&c, i) {
            Ok(g) => {
                ook += 1;
                other_status.push_str(&format!("{}\t{}\tOK\t{}\t\n", e.name, e.va, g.nodes.len()));
                other_tsv.push_str(&genotype_tsv(&g));
            }
            Err(er) => {
                obad += 1;
                other_status.push_str(&format!("{}\t{}\tBUILD_FAIL\t\t{}\n", e.name, e.va, er));
            }
        }
    }
    let mut hs = String::from("head\tcount\n");
    for (h, n) in &heads {
        hs.push_str(&format!("{}\t{}\n", h, n));
    }
    let mut pf = String::from("kind\tname\tva\tmsg\n");
    for f in &c.failures {
        pf.push_str(&format!("{}\t{}\t{}\t{}\n", f.kind, f.name, f.va, f.msg));
    }
    let w = |n: &str, s: &str| {
        let _ = fs::write(format!("{}/{}", outdir, n), s);
    };
    w("GENOTYPE_NV1.tsv", &main_tsv);
    w("GENOTYPE_NV1_OTHER.tsv", &other_tsv);
    w("STATUS_NV1.tsv", &status);
    w("STATUS_NV1_OTHER.tsv", &other_status);
    w("HEADS_NV1.tsv", &hs);
    w("PARSE_FAILURES_NV1.tsv", &pf);
    println!(
        "corpus: effects parsed {}, defuns parsed {}, layout rows {}, parse failures {}, dup effect names {:?}, dup defun names {:?}",
        c.effects.len(),
        c.defuns.len(),
        c.layout_rows,
        c.failures.len(),
        c.dup_effects,
        c.dup_defuns
    );
    println!("listed programs: {} built ok, {} failed (targets: {} ok, {} failed)", ok, bad, tok, tbad);
    println!("other def_effect rows: {} built ok, {} failed", ook, obad);
    println!("bare-word parameter defaults in the listed programs: {}", wd);
    println!("distinct function heads in the listed programs: {}", heads.len());
    0
}

fn load(programs: &str, combos: &str) -> Option<(Corpus, fx_eval::ComboTable)> {
    let text = fs::read_to_string(programs).ok()?;
    let ct = fs::read_to_string(combos).ok()?;
    Some((Corpus::from_programs_tsv(&text), fx_eval::ComboTable::from_tsv(&ct)))
}

fn compile_one(programs: &str, combos: &str, name: &str, va: Option<&str>) -> i32 {
    let (c, ct) = match load(programs, combos) {
        Some(x) => x,
        None => {
            eprintln!("cannot read inputs");
            return 2;
        }
    };
    let ei = match c.find_effect(name, va) {
        Some(i) => i,
        None => {
            eprintln!("effect {} not found", name);
            return 1;
        }
    };
    let g = match build_cli(&c, ei, name) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}: {}", name, e);
            return 1;
        }
    };
    let sets = fx_eval::enumerate_branches_opt(&g, &ct, &BTreeMap::new(), std::env::args().any(|x| x == "--hc-zero-legacy"));
    println!("# {} branch sets: {}", name, sets.len());
    for s in &sets {
        let opt = fx_eval::CompileOptions { overrides: s.overrides.clone(), input_elnum: BTreeMap::new(), strict: false, paramset: s.paramset.clone(), hc_zero_legacy: std::env::args().any(|x| x == "--hc-zero-legacy"), ..Default::default() };
        if let Ok(cp) = fx_eval::compile(&g, &ct, &opt) {
            print!("{}", cp.plan_tsv());
            for d in &cp.diagnostics {
                println!("DIAG\t{}", d);
            }
        }
    }
    0
}

fn compile_all(programs: &str, forms: &str, combos: &str, outdir: &str) -> i32 {
    let (c, ct) = match load(programs, combos) {
        Some(x) => x,
        None => {
            eprintln!("cannot read inputs");
            return 2;
        }
    };
    let ftext = fs::read_to_string(forms).unwrap_or_default();
    let _ = fs::create_dir_all(outdir);
    let mut plans = String::new();
    let mut summary = String::from("name\tva\tstatus\tbranch_params\tbranch_sets\tdefault_nodes\tdefault_funcs\tdefault_pruned\tdiag_default\tunknown_classes\n");
    for (i, line) in ftext.lines().enumerate() {
        if i == 0 || line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 3 || f[2] != "1" {
            continue;
        }
        let (name, va) = (f[0], f[1]);
        let ei = match c.find_effect(name, Some(va)) {
            Some(i) => i,
            None => {
                summary.push_str(&format!("{}\t{}\tNOT_FOUND\t\t\t\t\t\t\t\n", name, va));
                continue;
            }
        };
        let g = match build_cli(&c, ei, name) {
            Ok(g) => g,
            Err(e) => {
                summary.push_str(&format!("{}\t{}\tBUILD_FAIL {}\t\t\t\t\t\t\t\n", name, va, e));
                continue;
            }
        };
        let (bp, _) = fx_eval::branch_params(&g);
        let sets = fx_eval::enumerate_branches_opt(&g, &ct, &BTreeMap::new(), std::env::args().any(|x| x == "--hc-zero-legacy"));
        let opt = fx_eval::CompileOptions { paramset: "default".into(), hc_zero_legacy: std::env::args().any(|x| x == "--hc-zero-legacy"), ..Default::default() };
        match fx_eval::compile(&g, &ct, &opt) {
            Ok(cp) => {
                let mut uk: Vec<String> = cp.diagnostics.iter().filter(|d| d.starts_with("UNKNOWN-CLASS")).map(|d| d.split(' ').nth(1).unwrap_or("").to_string()).collect();
                uk.sort();
                uk.dedup();
                summary.push_str(&format!("{}\t{}\tOK\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", name, va, bp.join(","), sets.len(), cp.nodes.len(), cp.count("func"), cp.pruned.len(), cp.diagnostics.len(), uk.join(",")));
            }
            Err(e) => summary.push_str(&format!("{}\t{}\tCOMPILE_FAIL {}\t\t\t\t\t\t\t\n", name, va, e)),
        }
        for s in &sets {
            let o = fx_eval::CompileOptions { overrides: s.overrides.clone(), input_elnum: BTreeMap::new(), strict: false, paramset: s.paramset.clone(), hc_zero_legacy: std::env::args().any(|x| x == "--hc-zero-legacy"), ..Default::default() };
            if let Ok(cp) = fx_eval::compile(&g, &ct, &o) {
                plans.push_str(&cp.plan_tsv());
            }
        }
    }
    let _ = fs::write(format!("{}/PLAN_NV2.tsv", outdir), plans);
    let _ = fs::write(format!("{}/COMPILE_SUMMARY_NV2.tsv", outdir), summary);
    0
}

fn eln(programs: &str, combos: &str, name: &str) -> i32 {
    let (c, ct) = match load(programs, combos) {
        Some(x) => x,
        None => return 2,
    };
    let ei = match c.find_effect(name, None) {
        Some(i) => i,
        None => return 1,
    };
    match build_cli(&c, ei, name) {
        Ok(g) => match fx_eval::compile(&g, &ct, &fx_eval::CompileOptions::default()) {
            Ok(cp) => {
                print!("{}", cp.elnum_rows());
                0
            }
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}

fn parse_sets(sets: &str) -> BTreeMap<String, Vec<f32>> {
    let mut m = BTreeMap::new();
    for kv in sets.split(';') {
        if let Some((k, v)) = kv.split_once('=') {
            let orig = std::env::var("FX_SETS_ORIG_NUMBER").is_ok();
            let vals: Vec<f32> = v.split(',').filter_map(|x| if orig { fx_eval::parse::orig_atof(x.trim(), false) } else { x.trim().parse::<f32>().ok() }).collect();
            if !k.is_empty() && !vals.is_empty() {
                m.insert(k.to_string(), vals);
            }
        }
    }
    m
}

fn plan_one(programs: &str, combos: &str, name: &str, sets: &str) -> i32 {
    let (c, ct) = match load(programs, combos) {
        Some(x) => x,
        None => return 2,
    };
    let Some(ei) = c.find_effect(name, None) else { return 1 };
    let Ok(g) = build_cli(&c, ei, name) else { return 1 };
    let opt = fx_eval::CompileOptions { overrides: parse_sets(sets), paramset: if sets.is_empty() { "default".into() } else { sets.into() }, minimal_context: true, host_curves_unkeyed: std::env::var("NV3B_HC").is_ok(), hc_zero_legacy: std::env::args().any(|x| x == "--hc-zero-legacy"), ..Default::default() };
    match fx_eval::compile(&g, &ct, &opt) {
        Ok(cp) => {
            print!("{}", fx_eval::cpuplan::plan_text_mode(&cp, if std::env::var("NV3D_MODE").map(|v| v == "cpu").unwrap_or(false) { fx_eval::cpuplan::PlanMode::CpuDriver } else { fx_eval::cpuplan::PlanMode::GpuFirst }));
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}

/// the default-xy writers of the placement model (stack word 2 = 1 plan), one line per writer, then the plan text
fn xyplan(programs: &str, combos: &str, name: &str, sets: &str) -> i32 {
    let (c, ct) = match load(programs, combos) {
        Some(x) => x,
        None => return 2,
    };
    let Some(ei) = c.find_effect(name, None) else { return 1 };
    let Ok(g) = build_cli(&c, ei, name) else { return 1 };
    let hd = std::env::var("XWP2_HD").ok().and_then(|v| u32::from_str_radix(v.trim_start_matches("0x"), 16).ok()).unwrap_or(0x7f8);
    let opt = fx_eval::CompileOptions { overrides: parse_sets(sets), paramset: if sets.is_empty() { "default".into() } else { sets.into() }, minimal_context: true, host_curves_unkeyed: std::env::var("NV3B_HC").is_ok(), ctx_has_data: hd, ..Default::default() };
    match fx_eval::xyplace::compile_placed(&g, &ct, &opt) {
        Ok((cp, ws)) => {
            for w in &ws {
                println!("WRITER\tref{}\tgid={}\t{:?}\tpreserve={}\tconsumers={:?}", w.node, w.gid, w.place, w.preserve, w.consumers);
            }
            print!("{}", fx_eval::cpuplan::plan_text_mode(&cp, fx_eval::cpuplan::PlanMode::CpuDriver));
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}

fn diff_plans(a: &str, b: &str) -> i32 {
    let (Ok(ta), Ok(tb)) = (fs::read_to_string(a), fs::read_to_string(b)) else {
        eprintln!("cannot read inputs");
        return 2;
    };
    let d = fx_eval::plan::diff(&fx_eval::plan::canon_any(&ta), &fx_eval::plan::canon_any(&tb));
    if d.is_empty() {
        println!("EQUAL");
        0
    } else {
        for l in &d {
            println!("{}", l);
        }
        1
    }
}

/// Compare every PLAN_ORIG_*.txt in `dir` with our plan under the same binding. SOURCE-UNAPPROVED.
fn plancmp(dir: &str, programs: &str, combos: &str, outdir: &str, hc: bool, hck: Option<&str>, mode: fx_eval::cpuplan::PlanMode) -> i32 {
    let (c, ct) = match load(programs, combos) {
        Some(x) => x,
        None => return 2,
    };
    let _ = fs::create_dir_all(outdir);
    let mut files: Vec<_> = match fs::read_dir(dir) {
        Ok(r) => r.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.file_name().and_then(|n| n.to_str()).map(|n| n.starts_with("PLAN_ORIG_") && n.ends_with(".txt")).unwrap_or(false)).collect(),
        Err(_) => return 2,
    };
    files.sort();
    // keyed host curve answers, TSV rows tag, curve name, time bits (hex), lanes (hex words, comma separated); one E1 run's own hostcurve_calls
    let keyed_rows: Option<Vec<Vec<String>>> = hck.map(|p| fs::read_to_string(p).unwrap_or_default().lines().filter(|l| !l.starts_with("tag\t") && !l.is_empty()).map(|l| l.split('\t').map(|x| x.to_string()).collect()).collect());
    let mut summary = String::from("program\ttag\tsets\tstatus\tfirst_difference\n");
    let mut w113 = String::from("program\ttag\truns\tbits\tlayout\tw113\tfirst_layout_difference\n");
    // `--argcol` writes ARGCOL.tsv (report-only; the existing tables are unchanged)
    let argcol_on = std::env::args().any(|x| x == "--argcol");
    let mut argcol_rows = String::from("program\ttag\targcol\trunprims_orig\trunprims_native\tdetail\n");
    for f in files {
        let text = fs::read_to_string(&f).unwrap_or_default();
        let head = text.lines().find(|l| l.starts_with("#EP3")).unwrap_or("");
        let mut prog = String::new();
        let mut tag = String::new();
        let mut sets = String::new();
        for kv in head.split('\t') {
            if let Some(v) = kv.strip_prefix("program=") {
                prog = v.into();
            } else if let Some(v) = kv.strip_prefix("tag=") {
                tag = v.into();
            } else if let Some(v) = kv.strip_prefix("sets=") {
                sets = v.into();
            }
        }
        let Some(ei) = c.find_effect(&prog, None) else {
            summary.push_str(&format!("{}\t{}\t{}\tNOT_FOUND\t\n", prog, tag, sets));
            continue;
        };
        let Ok(g) = build_cli(&c, ei, &prog) else {
            summary.push_str(&format!("{}\t{}\t{}\tBUILD_FAIL\t\n", prog, tag, sets));
            continue;
        };
        let mut bound_elnum = BTreeMap::new();
        if let Some(r) = tag.strip_prefix("mk") {
            if let Some(c) = r.chars().next().and_then(|ch| ch.to_digit(10)) {
                bound_elnum.insert("Mask".to_string(), c as i32);
            }
        }
        let all_args: Vec<String> = std::env::args().collect();
        let ctx = match fx_eval::CompileContext::from_args(&all_args).and_then(|c| c.check_native_preserve().map(|_| c)) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("plancmp: {}", e);
                return 2;
            }
        };
        let (presence_mask, input_chan, mixed_mask) = match presence_args() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("plancmp: {}", e);
                return 2;
            }
        };
        if presence_mask.is_some() {
            for (k, v) in &input_chan {
                bound_elnum.insert(k.clone(), *v);
            }
        }
        let bound_input_mask = match mask_arg() {
            Some(m) => m,
            None if presence_mask.is_some() => 0,
            None => {
                eprintln!("plancmp: --bound-mask HEX is required and must parse; no environment override");
                return 2;
            }
        };
        let host_keyed = keyed_rows.as_ref().map(|rows| {
            let mut m: BTreeMap<(String, u32), Vec<f32>> = BTreeMap::new();
            for r in rows.iter().filter(|r| r.len() >= 4 && r[0] == tag) {
                let tb = u32::from_str_radix(&r[2], 16).unwrap_or(0);
                let l: Vec<f32> = r[3].split(',').filter_map(|x| u32::from_str_radix(x, 16).ok()).map(f32::from_bits).collect();
                m.insert((r[1].clone(), tb), l);
            }
            m
        });
        let opt = fx_eval::CompileOptions { overrides: parse_sets(&sets), paramset: tag.clone(), minimal_context: true, host_curves_unkeyed: hc, host_keyed, bound_elnum, bound_input_mask, hc_zero_legacy: std::env::args().any(|x| x == "--hc-zero-legacy"), presence_mask, mixed_mask, ctx_word2: ctx.stack_word2, ctx_has_data: ctx.has_data, ..Default::default() };
        let cp = match fx_eval::compile(&g, &ct, &opt) {
            Ok(c) => c,
            Err(e) => {
                summary.push_str(&format!("{}\t{}\t{}\tCOMPILE_FAIL\t{}\n", prog, tag, sets, e.msg.replace('\t', " ")));
                continue;
            }
        };
        let _ = fs::write(format!("{}/DIAG_{}_{}.txt", outdir, prog, tag), cp.diagnostics.join("
") + "
");
        if cp.ctx_word2 {
            if let Some(r) = fx_eval::preserve::model_refusal(&cp) {
                eprintln!("plancmp: {} (program {}, tag {})", r, prog, tag);
                return 2;
            }
        }
        let mine = fx_eval::plan::canon_mine_mode(&cp, mode);
        let hexl = |l: &[u32; 4]| l.iter().map(|b| b.to_le_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>()).collect::<String>();
        let mut mine_l: Vec<String> = cp.node_state.iter().filter(|s| s.b160 && s.folded).map(|s| hexl(&s.lanes)).collect();
        for c in cp.clones.iter().filter(|c| c.b160 && c.folded) {
            mine_l.push(hexl(&c.lanes));
        }
        if std::env::var_os("FLD1_NO_COPIES").is_none() {
            mine_l.extend(fx_eval::cpuplan::materialised_copies(&cp).iter().map(|l| hexl(l)));
        }
        let mut tot = 0;
        let mut hit = 0;
        let mut missed: Vec<String> = Vec::new();
        for l in text.lines().filter(|l| l.starts_with("NODE")) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() > 13 && f[3] == "kind=2" && f[13] == format!("{}=1", fx_eval::judge_keys::PLAN_KEY_NODE_FLAG) {
                let lanes = f[8].trim_start_matches("lanes=").to_string();
                tot += 1;
                if let Some(p) = mine_l.iter().position(|x| *x == lanes) {
                    mine_l.remove(p);
                    hit += 1;
                } else {
                    missed.push(format!("{}:{}", f[4].trim_start_matches("prim="), lanes));
                }
            }
        }
        let orig = fx_eval::plan::canon_orig(&text);
        let d = fx_eval::plan::diff(&mine, &orig);
        let st = if d.is_empty() { "EVALUATOR-PLAN-PARITY" } else { "DIFF" };
        summary.push_str(&format!("{}	{}	{}	{}	{}	{}/{}	{}
", prog, tag, sets, st, d.first().cloned().unwrap_or_default(), hit, tot, missed.join(" ")));
        {
            let mut lm = layout_canon(&fx_eval::cpuplan::plan_text_mode(&cp, mode));
            let lo = layout_canon(&text);
            if !lo.iter().any(|x| x.starts_with("cmd-0x")) {
                lm.retain(|x| !x.starts_with("cmd-0x"));
            }
            let lay_eq = lm == lo;
            let first = lm.iter().zip(lo.iter()).position(|(a, b)| a != b).map(|i| format!("cmd {}: mine `{}` vs original `{}`", i, lm[i], lo[i])).unwrap_or_else(|| if lm.len() != lo.len() { format!("command count mine {} vs original {}", lm.len(), lo.len()) } else { String::new() });
            let bits_ok = hit == tot;
            let w = d.is_empty() && bits_ok && lay_eq;
            w113.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\n", prog, tag, if d.is_empty() { "RUNS-EQUAL" } else { "RUNS-DIFF" }, if bits_ok { "BITS-EQUAL" } else { "BITS-DIFF" }, if lay_eq { "LAYOUT-EQUAL" } else { "LAYOUT-DIFF" }, if w { "PLAN-PARITY" } else { "PLAN-MISS" }, first));
        }
        let _ = fs::write(format!("{}/MINE_{}_{}.txt", outdir, prog, tag), fx_eval::cpuplan::plan_text_mode(&cp, mode));
        if argcol_on {
            // report-only per-argument column (original plan text against the native plan text of this compile)
            let (v, no, nn, d) = fx_eval::argcol::compare(&text, &fx_eval::cpuplan::plan_text_mode(&cp, mode));
            argcol_rows.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", prog, tag, v, no, nn, d.replace('\t', " ")));
        }
        let _ = fs::write(format!("{}/CANON_MINE_{}_{}.txt", outdir, prog, tag), mine.to_text());
        let _ = fs::write(format!("{}/CANON_ORIG_{}_{}.txt", outdir, prog, tag), orig.to_text());
        let _ = fs::write(format!("{}/DIFF_{}_{}.txt", outdir, prog, tag), d.join("\n") + "\n");
    }
    let _ = fs::write(format!("{}/PLANCMP_NV3.tsv", outdir), &summary);
    let _ = fs::write(format!("{}/PLAN_PARITY.tsv", outdir), &w113);
    if argcol_on {
        let _ = fs::write(format!("{}/ARGCOL.tsv", outdir), &argcol_rows);
    }
    print!("{}", summary);
    0
}

/// layout tokens: every printed token of every command in order, image ids (`:refN`) canonicalised by
/// first appearance (numbering may differ, in-place writes and id reuse after a free may not), the
/// suffix after the id (format code, g flag) and the gpu/cpu token kept. Commands without printed text
/// (the per-image cleanup records `cmd-0x...`) compare by their opcode name only.
pub fn layout_canon(text: &str) -> Vec<String> {
    let mut ids: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut out = Vec::new();
    for l in text.lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 4 || f[0] != "CMD" {
            continue;
        }
        let cmd = if f[3].trim().is_empty() { f[2].to_string() } else { f[3].replace("\\n", " ") };
        let b: Vec<char> = cmd.chars().collect();
        let mut s = String::new();
        let mut i = 0;
        while i < b.len() {
            if b[i] == ':' && b[i..].iter().take(4).collect::<String>() == ":ref" {
                i += 4;
                let mut id = String::new();
                while i < b.len() && b[i].is_ascii_digit() {
                    id.push(b[i]);
                    i += 1;
                }
                let mut suf = String::new();
                while i < b.len() && (b[i] == '.' || b[i].is_ascii_alphanumeric()) {
                    suf.push(b[i]);
                    i += 1;
                }
                let n = ids.len();
                let k = *ids.entry(id).or_insert(n);
                s.push_str(&format!(":r{}{}", k, suf));
            } else {
                s.push(b[i]);
                i += 1;
            }
        }
        out.push(s.split_whitespace().collect::<Vec<_>>().join(" "));
    }
    out
}

/// NV-3e: feed the r8 = 1 eval order (EVL_r1) through the ported linewise pass and compare order and lg with the original r8 = 0 eval list (EVL_r0).
fn lgtest(r1: &str, r0: &str) -> i32 {
    fn parse(path: &str) -> Vec<(String, Vec<(usize, bool, String, i32, Vec<usize>, i32)>)> {
        let t = fs::read_to_string(path).unwrap_or_default();
        let mut out: Vec<(String, Vec<(usize, bool, String, i32, Vec<usize>, i32)>)> = Vec::new();
        for l in t.lines() {
            if let Some(n) = l.strip_prefix("#### ") {
                out.push((n.trim().to_string(), vec![]));
            } else if let Some(cur) = out.last_mut() {
                let l = l.trim();
                let Some(p) = l.find('(') else { continue };
                let Some(q) = l.find(')') else { continue };
                let id: usize = l[p + 1..q].parse().unwrap_or(0);
                let kv = |k: &str| -> String { l.split_whitespace().find_map(|w| w.strip_prefix(k)).unwrap_or("").to_string() };
                let args: Vec<usize> = kv("args=").split(',').filter(|x| !x.is_empty()).filter_map(|x| x.parse().ok()).collect();
                cur.1.push((id, kv("kind=") == "2", kv("prim="), kv("el=").parse().unwrap_or(1), args, kv("lg=").parse().unwrap_or(-1)));
            }
        }
        out
    }
    let a = parse(r1);
    let b = parse(r0);
    let (mut ok, mut tot) = (0, 0);
    for (name, rows) in &a {
        let Some((_, orows)) = b.iter().find(|(n, _)| *n == format!("{}_r0", name) || n == name) else { continue };
        let nodes: Vec<fx_eval::lgpass::LNode> = rows
            .iter()
            .map(|r| fx_eval::lgpass::LNode { id: r.0, is_func: r.1, prim: r.2.clone(), elnum: r.3, fmt: if r.2 == "make_mipmap" { 3 } else { 2 }, args: r.4.clone(), g: -1 })
            .collect();
        let res = fx_eval::lgpass::run_pass(nodes);
        let mine: Vec<(usize, i32)> = res.iter().map(|x| (x.id, x.g)).collect();
        let orig: Vec<(usize, i32)> = orows.iter().map(|r| (r.0, r.5)).collect();
        let ord_eq = mine.iter().map(|x| x.0).collect::<Vec<_>>() == orig.iter().map(|x| x.0).collect::<Vec<_>>();
        // ids canonical by first appearance among eligible
        let canon = |v: &Vec<(usize, i32)>| -> Vec<i32> {
            let mut m: HashMap<i32, i32> = HashMap::new();
            v.iter().map(|x| if x.1 < 0 { -1 } else { let n = m.len() as i32; *m.entry(x.1).or_insert(n) }).collect()
        };
        let lg_eq = canon(&mine) == canon(&orig);
        tot += 1;
        if ord_eq && lg_eq {
            ok += 1;
        }
        println!("{}\torder {}\tlg {}\tmine {:?}\torig {:?}", name, if ord_eq { "EQ" } else { "DIFF" }, if lg_eq { "EQ" } else { "DIFF" }, mine, orig);
    }
    println!("LGTEST order+lg equal {} of {}", ok, tot);
    0
}
