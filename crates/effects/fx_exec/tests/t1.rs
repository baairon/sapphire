//! T1 (NV-5): selector parity against CHOICES.tsv, hand-evaluated commands, determinism, malformed plans,
//! node-level vs original-plan-command cross-check (Hue_Sat_Bright, Composite), native Composite vs comp_rgba's own render,
//! and the status matrix of the five milestone programs (written to out/EXEC_STATUS_NV5.tsv).
use fx_eval::{build_effect, enumerate_branches, Corpus, Genotype};
use fx_exec::cmds::{commands_of_dump, execute_commands};
use fx_exec::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn genotype(name: &str) -> Genotype {
    let p = effects_vecdir::root().join("record_extract").join("programs").join("programs.tsv");
    let t = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {}", p.display(), e));
    let c = Corpus::from_programs_tsv(&t);
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

/// Dense synthetic planes: random values in [-0.3, 1.3] with exact 0, 1, -0 and 0.5 sprinkled in.
fn synth(ch: usize, n: usize, seed: u64, alpha01: bool) -> Image {
    let mut g = Lcg(seed);
    let mut planes = Vec::new();
    for c in 0..ch {
        let mut p = Vec::with_capacity(n);
        for i in 0..n {
            let r = g.next();
            let mut v = (r as f32 / 2147483648.0) * 1.6 - 0.3;
            if alpha01 && c == 3 {
                v = (r as f32 / 2147483648.0).min(1.0);
            }
            match i % 13 {
                3 => v = 0.0,
                7 => v = 1.0,
                11 if c != 3 => v = 0.5,
                _ => {}
            }
            if i % 29 == 5 && c == 0 {
                v = -0.0;
            }
            p.push(v);
        }
        planes.push(p);
    }
    Image::from_planes(planes).unwrap()
}

fn bits_eq(a: &Image, b: &Image) -> bool {
    a.n == b.n && a.planes.len() == b.planes.len() && a.planes.iter().zip(&b.planes).all(|(x, y)| x.iter().zip(y).all(|(p, q)| p.to_bits() == q.to_bits()))
}

fn parse_sets(s: &str) -> BTreeMap<String, Vec<f32>> {
    let mut m = BTreeMap::new();
    for kv in s.split(';') {
        if let Some((k, v)) = kv.split_once('=') {
            m.insert(k.trim().to_string(), v.split(',').map(|x| x.trim().parse::<f32>().unwrap()).collect());
        }
    }
    m
}

fn dump_sets(dump: &str) -> BTreeMap<String, Vec<f32>> {
    let h = dump.lines().next().unwrap();
    let sets = h.split('\t').find(|f| f.starts_with("sets=")).unwrap_or("sets=");
    parse_sets(&sets[5..])
}

fn binding_with(ov: BTreeMap<String, Vec<f32>>) -> Binding {
    let mut b = Binding::defaults();
    b.overrides = ov;
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    b
}

fn inputs_of(names: &[(&str, Image)], w: usize, h: usize) -> Inputs {
    let mut images = BTreeMap::new();
    for (n, i) in names {
        images.insert(n.to_string(), i.clone());
    }
    Inputs { width: w, height: h, images }
}

// ---------------------------------------------------------------- selector

#[test]
fn selector_reproduces_every_choices_row() {
    if !effects_rtdata::have_tables() || !effects_vecdir::have_all(&["fx_exec/data/CHOICES.tsv"]) {
        return;
    }
    let t = Tables::from_tsv(effects_rtdata::text("fx_exec/data/COMBOS_EXEC.tsv"));
    let text = std::fs::read_to_string(effects_vecdir::path("fx_exec/data/CHOICES.tsv")).unwrap();
    let (mut rows, mut equal, mut diffs) = (0, 0, Vec::new());
    for (i, line) in text.lines().enumerate() {
        let f: Vec<&str> = line.split('\t').collect();
        if i == 0 || f.len() < 8 {
            continue;
        }
        let ops: Vec<Option<i32>> = if f[2].trim().is_empty() { vec![] } else { f[2].split(',').map(|x| Some(x.trim().parse::<i32>().unwrap())).collect() };
        let want: i32 = f[4].trim().parse().unwrap();
        let got = t.select(f[0], &ops);
        rows += 1;
        if got == want {
            equal += 1;
        } else {
            diffs.push(format!("{} {} want {} got {}", f[0], f[2], want, got));
        }
    }
    let _ = std::fs::write(effects_vecdir::out_dir().join("SELECTOR_PARITY_NV5.txt"), format!("rows {}\nequal {}\ndifferent {}\n{}\n", rows, equal, diffs.len(), diffs.join("\n")));
    assert_eq!(rows, 639, "CHOICES.tsv data rows");
    assert!(diffs.is_empty(), "selector differences: {:?}", &diffs[..diffs.len().min(10)]);
}

#[test]
fn selector_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let t = Tables::embedded();
    assert_eq!(t.select_counts("*", &[4, 1]), 4);
    assert_eq!(t.select_counts("*", &[2, 3]), -1); // not in the table
    assert_eq!(t.select_counts("+", &[3, 4]), 4);
    assert_eq!(t.select_counts("comp_rgba", &[3, 3]), 3);
    assert_eq!(t.select_counts("comp_rgba", &[4, 3]), 4);
    assert_eq!(t.select_counts("hue_shift", &[2, 1]), -1);
    assert_eq!(t.select_counts("sat_scale", &[2, 1]), 2);
    assert_eq!(t.select_counts("v3_to_v4_if4", &[3, 1, 4]), 4);
    assert_eq!(t.select_counts("if_neq_const", &[1]), -1); // no table
    assert_eq!(t.select("*", &[Some(1), None]), -1); // unbound
    assert_eq!(t.select_counts("*", &[1]), 1); // prefix match: n < len (spec section 2)
    assert_eq!(t.select_counts("nope", &[1, 1]), -1);
    assert_eq!(t.select_counts("static2", &[]), 2);
}

// ---------------------------------------------------------------- commands by hand (2x2 images)

fn img(planes: Vec<Vec<f32>>) -> Image {
    Image::from_planes(planes).unwrap()
}

fn run_cmds(cmds: &[&str], inputs: &[Image]) -> Result<Image, ExecError> {
    let v: Vec<String> = cmds.iter().map(|s| s.to_string()).collect();
    let o = execute_commands(&v, inputs, 2, 2, &HostFrames::default(), &KParams::standin())?;
    o.dest.ok_or(ExecError::BadPlan("no convert-to-dest".into()))
}

#[test]
fn identity_convert_roundtrip() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let a = img(vec![vec![1.0, 2.0, 3.0, 4.0]]);
    let d = run_cmds(&["(allocate-images :ref1.2g 1 2 gpu)", "(convert-from-source 0 :ref1.2g)", "(convert-to-dest :ref1.2g)", "(free-images :ref1.2g)"], &[a.clone()]).unwrap();
    assert!(bits_eq(&d, &a));
}

#[test]
fn mul_by_constant_and_add_and_sub_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let a = img(vec![vec![1.0, 2.0, 3.0, 4.0]]);
    let b = img(vec![vec![0.5, 0.25, -1.0, 8.0]]);
    let d = run_cmds(
        &[
            "(allocate-images :ref1.2g 1 2 gpu)", "(convert-from-source 0 :ref1.2g)", "(allocate-images :ref2.2g 1 2 gpu)", "(convert-from-source 1 :ref2.2g)",
            "(allocate-images :ref3.2g 1 2 gpu)", "(run-primitive 1 -1 :ref3.2g * :ref1.2g [ 0.5 ] )", "(run-primitive 1 -1 :ref3.2g + :ref3.2g :ref2.2g )",
            "(run-primitive 1 -1 :ref3.2g - :ref3.2g [ 1 ] )", "(free-images :ref1.2g)", "(free-images :ref2.2g)", "(convert-to-dest :ref3.2g)", "(free-images :ref3.2g)",
        ],
        &[a, b],
    )
    .unwrap();
    // (x*0.5 + y) - 1
    assert_eq!(d.planes[0], vec![0.5 + 0.5 - 1.0, 1.0 + 0.25 - 1.0, 1.5 - 1.0 - 1.0, 2.0 + 8.0 - 1.0]);
}

#[test]
fn abs_negate_max_min_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let a = img(vec![vec![-1.5, 2.0, -0.25, 0.0]]);
    let d = run_cmds(
        &["(allocate-images :ref1.2g 1 2 gpu)", "(convert-from-source 0 :ref1.2g)", "(run-primitive 1 -1 :ref2.2g abs :ref1.2g )"],
        &[a.clone()],
    );
    assert!(d.is_err()); // :ref2.2g never allocated
    let mk = |op: &str, extra: &str| -> Image {
        run_cmds(
            &[
                "(allocate-images :ref1.2g 1 2 gpu)", "(convert-from-source 0 :ref1.2g)", "(allocate-images :ref2.2g 1 2 gpu)",
                &format!("(run-primitive 1 -1 :ref2.2g {} :ref1.2g {})", op, extra), "(convert-to-dest :ref2.2g)",
            ],
            &[a.clone()],
        )
        .unwrap()
    };
    assert_eq!(mk("abs", "").planes[0], vec![1.5, 2.0, 0.25, 0.0]);
    assert_eq!(mk("negate", "").planes[0], vec![1.5, -2.0, 0.25, -0.0]);
    assert_eq!(mk("max", "[ 0 ]").planes[0], vec![0.0, 2.0, 0.0, 0.0]);
    assert_eq!(mk("min", "[ 0 ]").planes[0], vec![-1.5, 0.0, -0.25, 0.0]);
    assert_eq!(mk("one_minus", "").planes[0], vec![2.5, -1.0, 1.25, 1.0]);
}

#[test]
fn copy_free_constant_and_exec_linewise() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let a = img(vec![vec![1.0, 2.0, 3.0, 4.0]]);
    // constant used as an argument; copy-images then free the source; wrapper commands run inside exec-linewise
    let d = run_cmds(
        &[
            "(allocate-images :ref1.2g 1 2 gpu)",
            "(exec-linewise 0 0\\n  (   )\\n  ( (convert-from-source 0 :ref1.2g)\\n  )\\n  (   )\\n)",
            "(constant [ 2 ] :ref9.2g)",
            "(allocate-images :ref2.2g 1 2 gpu)",
            "(run-primitive 1 -1 :ref2.2g * :ref1.2g :ref9.2g )",
            "(allocate-images :ref3.2 1 2 cpu)",
            "(copy-images :ref3.2 :ref2.2g)",
            "(free-images :ref2.2g)",
            "(exec-linewise 1 1\\n  (   )\\n  ( (convert-to-dest :ref3.2)\\n  )\\n  (   )\\n)",
        ],
        &[a],
    )
    .unwrap();
    assert_eq!(d.planes[0], vec![2.0, 4.0, 6.0, 8.0]);
    // use after free is an error
    let e = run_cmds(&["(allocate-images :ref1.2g 1 2 gpu)", "(free-images :ref1.2g)", "(convert-to-dest :ref1.2g)"], &[]);
    assert!(matches!(e, Err(ExecError::BadPlan(_))));
}

#[test]
fn comp_rgba_by_hand_and_no_combo_and_unsupported() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // A = Back (all 0.5), B = Front premultiplied: alpha 1 -> B; alpha 0 -> A + B; alpha 0.5 -> fma(A, 0.5, B)
    let back = img(vec![vec![0.5; 4], vec![0.5; 4], vec![0.5; 4], vec![0.5; 4]]);
    let front = img(vec![vec![0.25; 4], vec![0.25; 4], vec![0.25; 4], vec![1.0, 0.0, 0.5, 1.0]]);
    let cmds = [
        "(allocate-images :ref9.2g 4 2 gpu)", "(convert-from-source 1 :ref9.2g)", "(allocate-images :ref10.2g 4 2 gpu)", "(convert-from-source 0 :ref10.2g)",
        "(allocate-images :ref0.2g 4 2 gpu)", "(run-primitive 1 -1 :ref0.2g comp_rgba :ref9.2g :ref10.2g )", "(convert-to-dest :ref0.2g)",
    ];
    let d = run_cmds(&cmds, &[front, back]).unwrap();
    assert_eq!(d.planes[0], vec![0.25, 0.5 + 0.25, 0.5f32.mul_add(0.5, 0.25), 0.25]);
    assert_eq!(d.planes[3], vec![1.0, 0.5 + 0.0, 0.5f32.mul_add(0.5, 0.5), 1.0]);
    // (2,3) operands of `*` have no table entry: the selector's -1 is an error
    let e = run_cmds(
        &["(allocate-images :ref1.2g 2 2 gpu)", "(allocate-images :ref2.2g 3 2 gpu)", "(allocate-images :ref3.2g 3 2 gpu)", "(run-primitive 1 -1 :ref3.2g * :ref1.2g :ref2.2g )"],
        &[],
    );
    assert!(matches!(e, Err(ExecError::NoCombo { .. })), "{:?}", e);
    let e = run_cmds(&["(allocate-images :ref1.2g 4 2 gpu)", "(run-primitive 1 -1 :ref1.2g gamma :ref1.2g [ 2 ] )"], &[]);
    assert!(matches!(e, Err(ExecError::Unsupported { ref class, .. }) if class == "gamma"), "{:?}", e);
}

#[test]
fn malformed_plans_never_panic() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let junk = ["", "(", ")", "((((", "(run-primitive", "(run-primitive 1 -1)", "(run-primitive 1 -1 :ref1 *)", "(allocate-images)", "(allocate-images :ref1 x 2 gpu)",
        "(convert-from-source)", "(convert-from-source 9 :ref1)", "(copy-images :a :b)", "(free-images :ref1)", "(constant [ ] :ref1)", "(constant [ a b ] :ref1)", "(exec-linewise 1 1 (", "[ 1 2 ]", "atom", "(frobnicate 1)",
        "(constant [ 1 2 3 4 5 ] :ref1)", "(allocate-images :ref1 4 2 gpu) (run-primitive 1 -1 :ref1 + :ref1 :ref1)", "(run-primitive 1 -1 :ref1 hue_shift :ref1 [ 1 ] [ 2 ] [ 3 ])"];
    for j in junk {
        let _ = run_cmds(&[j], &[]);
    }
    // every prefix of the Composite plan commands
    let dump = std::fs::read_to_string(effects_vecdir::root().join("wave_s7").join("rebuild_fx_plan").join("port").join("data").join("orig").join("PLAN_ORIG_Composite_default.txt")).unwrap();
    for c in commands_of_dump(&dump) {
        for k in 0..c.len() {
            if c.is_char_boundary(k) {
                let _ = run_cmds(&["(allocate-images :ref9.2g 4 2 gpu)", &c[..k]], &[synth(4, 4, 1, true), synth(4, 4, 2, true)]);
            }
        }
    }
    // malformed node-level input: missing and mis-shaped images
    big(|| {
        let g = genotype("Composite");
        let b = binding_with(BTreeMap::new());
        let e = execute(&g, &b, &inputs_of(&[], 2, 2), &HostFrames::default());
        assert!(matches!(e, Err(ExecError::MissingInput(_))), "{:?}", e);
        let e = execute(&g, &b, &inputs_of(&[("Front", synth(3, 4, 1, true)), ("Back", synth(4, 4, 2, true))], 2, 2), &HostFrames::default());
        assert!(matches!(e, Err(ExecError::Shape(_))), "{:?}", e);
        let e = execute(&g, &b, &inputs_of(&[("Front", synth(4, 5, 1, true)), ("Back", synth(4, 5, 2, true))], 2, 2), &HostFrames::default());
        assert!(matches!(e, Err(ExecError::Shape(_))), "{:?}", e);
    });
}

// ---------------------------------------------------------------- programs

#[test]
fn composite_equals_comp_rgba_render_and_original_plan() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let g = genotype("Composite");
        let (w, h) = (17, 5);
        let n = w * h;
        let front = synth(4, n, 11, true);
        let back = synth(4, n, 12, true);
        let b = binding_with(BTreeMap::new());
        let inp = inputs_of(&[("Front", front.clone()), ("Back", back.clone())], w, h);
        let o1 = execute(&g, &b, &inp, &HostFrames::default()).unwrap();
        let o2 = execute(&g, &b, &inp, &HostFrames::default()).unwrap();
        assert!(bits_eq(&o1.dest, &o2.dest), "determinism");
        assert_eq!(o1.trace.len(), 1);
        assert_eq!(o1.trace[0].class, "comp_rgba");
        assert!(o1.diagnostics.is_empty(), "{:?}", o1.diagnostics);
        // S3 render of comp_rgba directly: A = Back, B = Front, (4,4)
        let p = comp_rgba::Params { ca: 4, cb: 4, flag_a: false, flag_b: false, a_vals: [0.0; 4], b_vals: [0.0; 4], n: n as i32 };
        let pa: Vec<&[f32]> = back.planes.iter().map(|x| x.as_slice()).collect();
        let pb: Vec<&[f32]> = front.planes.iter().map(|x| x.as_slice()).collect();
        let mut outv = vec![vec![0.0f32; n]; 4];
        {
            let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|x| x.as_mut_slice()).collect();
            comp_rgba::render(&p, &comp_rgba::Host { vcall_f0_handled: false }, &pa, &pb, &mut outs).unwrap();
        }
        assert!(bits_eq(&o1.dest, &Image::from_planes(outv).unwrap()), "native Composite differs from comp_rgba's S3 render");
        // the original plan's commands (inputs by index: 0 = Front, 1 = Back)
        let dump = std::fs::read_to_string(effects_vecdir::root().join("wave_s7").join("rebuild_fx_plan").join("port").join("data").join("orig").join("PLAN_ORIG_Composite_default.txt")).unwrap();
        let oc = execute_commands(&commands_of_dump(&dump), &[front, back], w, h, &HostFrames::default(), &KParams::standin()).unwrap();
        assert!(bits_eq(&o1.dest, oc.dest.as_ref().unwrap()), "node-level vs original plan commands");
    });
}

#[test]
fn hue_sat_bright_node_level_equals_original_plan_commands_on_all_six_dumps() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let g = genotype("Hue_Sat_Bright");
        let (w, h) = (19, 7);
        let n = w * h;
        let src = synth(4, n, 21, true);
        let dir = effects_vecdir::root().join("wave_s7").join("rebuild_fx_plan").join("port").join("data").join("orig");
        let mut rep = String::from("dump\truns\tbit_equal_to_original_plan_commands\n");
        for tag in ["default", "pl0", "pl1", "pl0_active", "pl1_active", "probe"] {
            let dump = std::fs::read_to_string(dir.join(format!("PLAN_ORIG_Hue_Sat_Bright_{}.txt", tag))).unwrap();
            let b = binding_with(dump_sets(&dump));
            let inp = inputs_of(&[("Source", src.clone())], w, h);
            let o = execute(&g, &b, &inp, &HostFrames::default()).unwrap_or_else(|e| panic!("{}: {}", tag, e));
            let o2 = execute(&g, &b, &inp, &HostFrames::default()).unwrap();
            assert!(bits_eq(&o.dest, &o2.dest));
            let oc = execute_commands(&commands_of_dump(&dump), &[src.clone()], w, h, &HostFrames::default(), &KParams::standin()).unwrap_or_else(|e| panic!("{} cmds: {}", tag, e));
            let eq = bits_eq(&o.dest, oc.dest.as_ref().unwrap());
            // the plan text prints constants with 6 significant digits, so a computed f32 lane such as 1.1 * 1.3 can differ
            // by one ulp from the parsed text: the bit-exact claim is for the dumps whose constants print exactly
            let mut maxd = 0.0f32;
            for (x, y) in o.dest.planes.iter().zip(&oc.dest.as_ref().unwrap().planes) {
                for (p, q) in x.iter().zip(y) {
                    maxd = maxd.max((p - q).abs());
                }
            }
            rep.push_str(&format!("{}\t{}\t{}\tmax_abs_diff={:e}\n", tag, o.trace.len(), eq, maxd));
            if !eq {
                assert!(tag == "probe" && maxd < 2e-6, "{}: differs (max abs diff {:e})", tag, maxd);
                continue;
            }
            if tag.ends_with("default") || tag == "pl0" || tag == "pl1" {
                assert_eq!(o.trace.len(), 0, "{}: identity expected", tag);
                assert!(bits_eq(&o.dest, &src), "{}: default branch must pass Source through", tag);
            } else {
                assert!(o.trace.len() >= 3, "{}", tag);
            }
            assert!(eq, "{}: node-level result differs from the original plan commands on the same builtins", tag);
        }
        let _ = std::fs::write(effects_vecdir::out_dir().join("HSB_XCHECK_NV5.tsv"), rep);
    });
}

fn try_program(name: &str, tag: &str, ov: BTreeMap<String, Vec<f32>>, g: &Genotype) -> String {
    let b = binding_with(ov);
    let cp = match compile_for(g, &b) {
        Ok(c) => c,
        Err(e) => return format!("{}\t{}\t-\t-\tcompile error: {}", name, tag, e),
    };
    let (w, h) = (16, 4);
    let mut images = BTreeMap::new();
    for nd in &cp.nodes {
        if nd.kind == "input" {
            images.insert(nd.name.clone(), synth(if nd.elnum > 0 { nd.elnum as usize } else { 4 }, w * h, 77, true));
        }
    }
    let mut hf = HostFrames::default();
    hf.frame_round_old = true; // NV5-FRAME-ROUND-OLD: the t1 dumps were taken with the NV-5 rule
    hf.prev_dest = Some(synth(4, w * h, 5, true));
    for off in -4..=4 {
        hf.frames.insert(off, synth(4, w * h, 100 + (off + 10) as u64, true));
    }
    let inp = Inputs { width: w, height: h, images };
    let run: Vec<String> = cp.nodes.iter().filter(|n| (n.kind == "func" || n.kind == "auto") && !n.b160 && !n.folded).map(|n| n.prim.clone()).collect();
    let un = unsupported_classes(&cp);
    match execute_compiled(&cp, &b, &inp, &hf) {
        Ok(o) => {
            let again = execute_compiled(&cp, &b, &inp, &hf).unwrap();
            assert!(bits_eq(&o.dest, &again.dest), "determinism {} {}", name, tag);
            format!("{}\t{}\t{}\tOK ch={} diag={}\t{}", name, tag, run.join(" "), o.dest.channels(), o.diagnostics.len(), if un.is_empty() { "-".into() } else { un.join(" ") })
        }
        Err(e) => format!("{}\t{}\t{}\tERR {}\t{}", name, tag, run.join(" "), e, if un.is_empty() { "-".into() } else { un.join(" ") }),
    }
}

#[test]
fn milestone_status_matrix() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let p = effects_vecdir::root().join("record_extract").join("programs").join("programs.tsv");
        let t = std::fs::read_to_string(&p).unwrap();
        let c = Corpus::from_programs_tsv(&t);
        let ct = exec::combo_table();
        let mut rows = String::from("program\tparamset\trun_list\tresult\tunsupported\n");
        for name in ["Hue_Sat_Bright", "Composite", "Diffuse", "Trails", "Time_Warp_RGB"] {
            let g = build_effect(&c, c.find_effect(name, None).unwrap()).unwrap();
            for s in enumerate_branches(&g, &ct, &BTreeMap::new()) {
                rows.push_str(&try_program(name, &s.paramset, s.overrides.clone(), &g));
                rows.push('\n');
            }
        }
        std::fs::write(effects_vecdir::out_dir().join("EXEC_STATUS_NV5.tsv"), &rows).unwrap();
        assert!(rows.lines().any(|l| l.starts_with("Hue_Sat_Bright") && l.contains("\tOK ")));
        assert!(rows.lines().any(|l| l.starts_with("Composite") && l.contains("\tOK ")));
    });
}
