//! (wave_s9 in3_wire_misc): dissolve wiring. T1 hand tests of the wired class, node-level vs ORIGINAL plan commands on the
//! Trails_Autogen x1 dump (mocha_mask_opacity 0.5), and the corpus sweep that records how dissolve, gamma, postdivide and if_eq are reached
//! (operand kinds and constant values) so the three classes without a wiring can be judged from evidence (out/IN3_CLASS_USE.tsv).
use fx_eval::{build_effect, enumerate_branches, Corpus, Genotype};
use fx_exec::cmds::{commands_of_dump, execute_commands_xy};
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn eff() -> PathBuf {
    effects_vecdir::root()
}
fn corpus() -> Corpus {
    let p = eff().join("record_extract").join("programs").join("programs.tsv");
    Corpus::from_programs_tsv(&std::fs::read_to_string(&p).unwrap())
}
fn genotype(name: &str) -> Genotype {
    let c = corpus();
    let ei = c.find_effect(name, None).unwrap_or_else(|| panic!("effect {} not found", name));
    build_effect(&c, ei).unwrap()
}
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) as u32
    }
}
fn synth(ch: usize, n: usize, seed: u64) -> Image {
    let mut g = Lcg(seed);
    let planes = (0..ch).map(|_| (0..n).map(|_| 0.05 + (g.next() as f32 / 2147483648.0)).collect()).collect();
    Image::from_planes(planes).unwrap()
}
fn cst(v: &[f32]) -> Arg<'static> {
    let mut l = [*v.last().unwrap(); 4].map(|x: f32| x.to_bits());
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}
fn run(args: &[Arg], w: usize, h: usize) -> Image {
    let c: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let t = Tables::embedded().select_counts("dissolve", &c);
    assert!(t > 0, "no combo for dissolve {:?}", c);
    match run_any("dissolve", t, args, w * h, (w, h), &KParams::standin()).unwrap() {
        Out::Img(i) => i,
        _ => panic!(),
    }
}
fn bits(i: &Image) -> Vec<Vec<u32>> {
    i.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect()
}

#[test]
fn dissolve_scalar_matte_hand_values() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // A = 2, B = 6 (4 channels, 4 samples), T = 0.5 -> 4; T = 0.25 -> 3; T <= 0 -> A; T >= 1 -> B; NaN T takes the lerp (NaN)
    let a = Image::from_const(&[2.0f32.to_bits(); 4], 4, 4);
    let b = Image::from_const(&[6.0f32.to_bits(); 4], 4, 4);
    for (t, want) in [(0.5f32, 4.0f32), (0.25, 3.0), (0.0, 2.0), (-1.0, 2.0), (1.0, 6.0), (7.0, 6.0)] {
        let o = run(&[Arg::Img(&a), Arg::Img(&b), cst(&[t])], 2, 2);
        assert_eq!(o.channels(), 4);
        assert!(o.planes.iter().all(|p| p.iter().all(|x| *x == want)), "t {}: {:?}", t, o.planes[0]);
    }
    let o = run(&[Arg::Img(&a), Arg::Img(&b), cst(&[f32::NAN])], 2, 2);
    assert!(o.planes.iter().all(|p| p.iter().all(|x| x.is_nan())));
}

#[test]
fn dissolve_plane_matte_and_scalar_endpoints_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // (1,3,3): A a 1 lane constant, B and T 3 channel images: out = lerp per channel
    let t = Image { n: 4, planes: vec![vec![0.0, 0.5, 1.0, 0.25]; 3] };
    let bb = Image { n: 4, planes: vec![vec![5.0; 4]; 3] };
    let o = run(&[cst(&[1.0]), Arg::Img(&bb), Arg::Img(&t)], 2, 2);
    assert_eq!(o.channels(), 3);
    for p in &o.planes {
        assert_eq!(p.clone(), vec![1.0, 3.0, 5.0, 2.0]);
    }
    // (3,4,1) with T = 0.5: A has an odd count so channel 3 reads A as 1.0 (listing 0x512a13); B channel 3 is the plane
    let a = Image { n: 4, planes: vec![vec![0.0; 4], vec![2.0; 4], vec![4.0; 4]] };
    let b = Image { n: 4, planes: vec![vec![8.0; 4], vec![8.0; 4], vec![8.0; 4], vec![3.0; 4]] };
    let o = run(&[Arg::Img(&a), Arg::Img(&b), cst(&[0.5])], 2, 2);
    assert_eq!(o.channels(), 4);
    assert_eq!(o.planes[0][0], 4.0);
    assert_eq!(o.planes[1][0], 5.0);
    assert_eq!(o.planes[2][0], 6.0);
    assert_eq!(o.planes[3][0], 2.0);
}

#[test]
fn dissolve_is_deterministic_and_sensitive_to_the_matte() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let a = synth(4, 64, 1);
    let b = synth(4, 64, 2);
    let r1 = run(&[Arg::Img(&a), Arg::Img(&b), cst(&[0.5])], 8, 8);
    let r2 = run(&[Arg::Img(&a), Arg::Img(&b), cst(&[0.5])], 8, 8);
    let r3 = run(&[Arg::Img(&a), Arg::Img(&b), cst(&[0.6])], 8, 8);
    assert_eq!(bits(&r1), bits(&r2));
    assert_ne!(bits(&r1), bits(&r3));
}

fn plan_cmds(path: &str) -> Vec<String> {
    commands_of_dump(&std::fs::read_to_string(path).unwrap()).into_iter().filter(|s| !s.trim().is_empty()).collect()
}

fn sv(s: &str) -> Vec<(String, Vec<f32>)> {
    s.split(';').filter_map(|kv| kv.split_once('=')).map(|(k, v)| (k.to_string(), v.split(',').filter_map(|x| x.parse().ok()).collect())).collect()
}

#[test]
fn trails_autogen_x1_node_level_equals_original_plan_commands() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let (w, h) = (16usize, 8usize);
        let n = w * h;
        let g = genotype("Trails_Autogen");
        let mut b = Binding::defaults();
        for (k, v) in sv("combine_masks=1;mocha_mask_opacity=0.5;blur_amount=0.2;combine=1") {
            b.overrides.insert(k, v);
        }
        b.const_inputs.insert("Mask".into(), vec![1.0]);
        let cp = compile_for(&g, &b).unwrap();
        let mut images = BTreeMap::new();
        for nd in &cp.nodes {
            if nd.kind == "input" {
                images.insert(nd.name.clone(), synth(if nd.elnum > 0 { nd.elnum as usize } else { 4 }, n, 31 + nd.name.len() as u64));
            }
        }
        let inp = Inputs { width: w, height: h, images };
        let prev = synth(4, n, 77);
        let host = HostFrames { prev_dest: Some(prev), ..HostFrames::default() };
        let node = execute_compiled(&cp, &b, &inp, &host).expect("node level");
        assert!(node.trace.iter().any(|r| r.class == "dissolve"), "dissolve did not run: {:?}", node.trace.iter().map(|r| r.class.clone()).collect::<Vec<_>>());
        let dump = eff().join("wave_s8").join("ag1_autogen").join("plans_fresh").join("PLAN_ORIG_Trails_Autogen_x1_r0.txt");
        let cmds = plan_cmds(&dump.to_string_lossy());
        let src = inp.images.get("Source").cloned().unwrap();
        let mask = Image { n, planes: vec![vec![1.0; n]] };
        let xy = fx_exec::kernels2::default_xy(&b.kp, w, h);
        let cm = execute_commands_xy(&cmds, &[src, mask], w, h, &host, &b.kp, Some(&xy)).expect("command level");
        let d = cm.dest.clone().unwrap();
        let same = bits(&node.dest) == bits(&d);
        let line = format!(
            "Trails_Autogen x1\tcommands {}\t{}\trun list: {}\tcommand trace: {}\n",
            cmds.len(),
            if same { "BIT-EQUAL" } else { "DIFF" },
            node.trace.iter().map(|r| format!("{}({:?}->{})", r.class, r.operands, r.out_channels)).collect::<Vec<_>>().join(" "),
            cm.trace.join(" ; ")
        );
        let _ = std::fs::create_dir_all(effects_vecdir::out_dir());
        let _ = std::fs::write(effects_vecdir::out_dir().join("XCHECK_IN3.tsv"), &line);
        eprintln!("{}", line);
        assert!(same);
        // negative control: a different matte must differ from the plan commands' result
        let mut b2 = b.clone();
        b2.overrides.insert("mocha_mask_opacity".into(), vec![0.75]);
        let cp2 = compile_for(&g, &b2).unwrap();
        let node2 = execute_compiled(&cp2, &b2, &inp, &host).unwrap();
        assert_ne!(bits(&node2.dest), bits(&d));
    });
}

fn arg_descr(cp: &fx_eval::CompiledProgram, a: usize) -> String {
    let nd = match cp.nodes.iter().find(|n| n.idx == a) {
        Some(n) => n,
        None => return "?".into(),
    };
    let n = (nd.elnum.max(1) as usize).min(4);
    if nd.kind == "const" || nd.b160 || nd.folded {
        let l: Vec<String> = (0..n).map(|i| format!("{}", f32::from_bits(nd.lanes[i]))).collect();
        format!("c{}[{}]", n, l.join(","))
    } else if nd.kind == "input" {
        format!("img{}:{}", nd.elnum, nd.name)
    } else {
        format!("img{}:{}", nd.elnum, nd.prim)
    }
}

#[test]
fn class_use_over_the_corpus() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let c = corpus();
        let ct = fx_exec::exec::combo_table();
        let want = ["dissolve", "gamma", "postdivide", "if_eq"];
        // (class, operand descriptor with constants, out channels) -> (programs, sets, example)
        let mut by: BTreeMap<(String, String), (BTreeSet<String>, usize, String)> = BTreeMap::new();
        let mut shapes: BTreeMap<(String, String), (BTreeSet<String>, usize)> = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for e in &c.effects {
            if !seen.insert(e.name.clone()) {
                continue;
            }
            let ei = match c.find_effect(&e.name, None) {
                Some(i) => i,
                None => continue,
            };
            let g = match build_effect(&c, ei) {
                Ok(g) => g,
                Err(_) => continue,
            };
            for s in enumerate_branches(&g, &ct, &BTreeMap::new()) {
                let mut b = Binding::defaults();
                b.overrides = s.overrides.clone();
                b.const_inputs.insert("Mask".into(), vec![1.0]);
                b.host_curves_unkeyed = true;
                let cp = match compile_for(&g, &b) {
                    Ok(cp) => cp,
                    Err(_) => continue,
                };
                for nd in &cp.nodes {
                    if (nd.kind == "func" || nd.kind == "auto") && !nd.b160 && !nd.folded && want.contains(&nd.prim.as_str()) {
                        let d: Vec<String> = nd.args.iter().map(|a| arg_descr(&cp, *a)).collect();
                        let shape: Vec<String> = d.iter().map(|x| x.split(|ch| ch == ':' || ch == '[').next().unwrap().to_string()).collect();
                        let key = (nd.prim.clone(), format!("{} -> {}ch", d.join(" "), nd.elnum));
                        let ent = by.entry(key).or_insert((BTreeSet::new(), 0, s.paramset.clone()));
                        ent.0.insert(e.name.clone());
                        ent.1 += 1;
                        let sh = shapes.entry((nd.prim.clone(), format!("{} -> {}ch", shape.join(" "), nd.elnum))).or_insert((BTreeSet::new(), 0));
                        sh.0.insert(e.name.clone());
                        sh.1 += 1;
                    }
                }
            }
        }
        let mut out = String::from("class\toperands (cN = constant of N lanes with values, imgN = image of N channels : source) -> output channels\tprograms\tbranch_sets\texample_set\texample_programs\n");
        for ((cl, d), (ps, n, ex)) in &by {
            out.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", cl, d, ps.len(), n, ex, ps.iter().take(5).cloned().collect::<Vec<_>>().join(" ")));
        }
        let mut sh = String::from("class\toperand shape -> output channels\tprograms\tbranch_sets\texample_programs\n");
        for ((cl, d), (ps, n)) in &shapes {
            sh.push_str(&format!("{}\t{}\t{}\t{}\t{}\n", cl, d, ps.len(), n, ps.iter().take(8).cloned().collect::<Vec<_>>().join(" ")));
        }
        let _ = std::fs::create_dir_all(effects_vecdir::out_dir());
        let _ = std::fs::write(effects_vecdir::out_dir().join("IN3_CLASS_USE.tsv"), &out);
        let _ = std::fs::write(effects_vecdir::out_dir().join("IN3_CLASS_SHAPES.tsv"), &sh);
        eprintln!("{}", sh);
    });
}

/// Every branch set of every program whose run list has a dissolve and no other unsupported class: executed on a 16x8 frame with synthetic
/// inputs (Mask = constant 1, host curves unkeyed); counts the sets that run and groups the errors (out/IN3_DISSOLVE_SETS.tsv).
#[test]
fn dissolve_bearing_sets_run_end_to_end() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let c = corpus();
        let ct = fx_exec::exec::combo_table();
        let (w, h) = (16usize, 8usize);
        let mut seen = BTreeSet::new();
        let mut rows = String::from("program\tbranch_sets_with_dissolve_run\tsets_other_unsupported\tsets_ok\tsets_err\tfirst_error\tdissolve_operand_counts_seen\n");
        let (mut tot_dis, mut tot_ok, mut tot_other, mut tot_err) = (0usize, 0usize, 0usize, 0usize);
        let mut progs_ok = BTreeSet::new();
        let mut progs_dis = BTreeSet::new();
        for e in &c.effects {
            if !seen.insert(e.name.clone()) {
                continue;
            }
            let ei = match c.find_effect(&e.name, None) {
                Some(i) => i,
                None => continue,
            };
            let g = match build_effect(&c, ei) {
                Ok(g) => g,
                Err(_) => continue,
            };
            let (mut nd, mut nok, mut nother, mut nerr) = (0usize, 0usize, 0usize, 0usize);
            let mut first = String::new();
            let mut ops: BTreeSet<String> = BTreeSet::new();
            for s in enumerate_branches(&g, &ct, &BTreeMap::new()) {
                let mut b = Binding::defaults();
                b.overrides = s.overrides.clone();
                b.const_inputs.insert("Mask".into(), vec![1.0]);
                b.host_curves_unkeyed = true;
                let cp = match compile_for(&g, &b) {
                    Ok(cp) => cp,
                    Err(_) => continue,
                };
                if !cp.nodes.iter().any(|n| (n.kind == "func" || n.kind == "auto") && !n.b160 && !n.folded && n.prim == "dissolve") {
                    continue;
                }
                nd += 1;
                if !unsupported_classes(&cp).is_empty() {
                    nother += 1;
                    continue;
                }
                let mut images = BTreeMap::new();
                for n in &cp.nodes {
                    if n.kind == "input" {
                        images.insert(n.name.clone(), synth(if n.elnum > 0 { n.elnum as usize } else { 4 }, w * h, 5 + n.name.len() as u64));
                    }
                }
                let inp = Inputs { width: w, height: h, images };
                let mut frames = HostFrames::default();
                frames.prev_dest = Some(synth(4, w * h, 9));
                for k in -8..=8 {
                    frames.frames.insert(k, synth(4, w * h, 20 + (k + 8) as u64));
                }
                match execute_compiled(&cp, &b, &inp, &frames) {
                    Ok(o) => {
                        nok += 1;
                        for r in o.trace.iter().filter(|r| r.class == "dissolve") {
                            ops.insert(format!("{:?}->{}", r.operands, r.out_channels));
                        }
                    }
                    Err(er) => {
                        nerr += 1;
                        if first.is_empty() {
                            first = format!("{}", er).chars().take(120).collect();
                        }
                    }
                }
            }
            if nd > 0 {
                progs_dis.insert(e.name.clone());
                if nok > 0 {
                    progs_ok.insert(e.name.clone());
                }
                tot_dis += nd;
                tot_ok += nok;
                tot_other += nother;
                tot_err += nerr;
                rows.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\n", e.name, nd, nother, nok, nerr, first, ops.iter().cloned().collect::<Vec<_>>().join(" ")));
            }
        }
        rows.push_str(&format!("TOTAL\t{}\t{}\t{}\t{}\tprograms with a dissolve run: {}, with at least one set running end to end: {}\t\n", tot_dis, tot_other, tot_ok, tot_err, progs_dis.len(), progs_ok.len()));
        let _ = std::fs::create_dir_all(effects_vecdir::out_dir());
        let _ = std::fs::write(effects_vecdir::out_dir().join("IN3_DISSOLVE_SETS.tsv"), &rows);
        eprintln!("{}", rows.lines().last().unwrap());
    });
}
