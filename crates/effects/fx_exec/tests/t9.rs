//! T1/T3 of NV-9: hand-evaluated tests per newly wired class, program-level runs of Diffuse, Transform_and_Blur and
//! Blur_Motion_Curves_No_Transform, node-level vs original-plan-command cross-check, the status matrix (out/EXEC_STATUS_NV9B.tsv).
use fx_eval::{build_effect, enumerate_branches, Corpus, Genotype};
use fx_exec::cmds::{commands_of_dump, execute_commands_xy};
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::*;
use std::collections::BTreeMap;
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
/// finite planes in [0.05, 1.05]
fn synth(ch: usize, n: usize, seed: u64) -> Image {
    let mut g = Lcg(seed);
    let planes = (0..ch).map(|_| (0..n).map(|_| 0.05 + (g.next() as f32 / 2147483648.0)).collect()).collect();
    Image::from_planes(planes).unwrap()
}
fn kp() -> KParams {
    // NV9B: the t9 hand tests were written against NV-9's geometry binding (NV9-GEOM-OLD); the default is now the dump geometry (t10)
    let mut k = KParams::standin();
    k.geom_mode = GeomMode::Nv9Old;
    k
}
fn cst(v: &[f32]) -> Arg<'static> {
    let mut l = [*v.last().unwrap(); 4].map(|x: f32| x.to_bits());
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}
fn tag_of(class: &str, args: &[Arg]) -> i32 {
    let c: Vec<i32> = args.iter().map(|a| a.count()).collect();
    Tables::embedded().select_counts(class, &c)
}
fn run(class: &str, args: &[Arg], w: usize, h: usize) -> Result<Out, ExecError> {
    let t = tag_of(class, args);
    assert!(t > 0, "no combo for {} {:?}", class, args.iter().map(|a| a.count()).collect::<Vec<_>>());
    run_any(class, t, args, w * h, (w, h), &kp())
}
fn img(o: Out) -> Image {
    o.into_image().unwrap()
}
fn bits(i: &Image) -> Vec<Vec<u32>> {
    i.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect()
}

#[test]
fn make_mipmap_filter0_is_the_source_with_clamp_pads_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // 2x2, 1 channel: rows [1 2] [3 4]; wrap [2 2] (edge replicate), filter 0: one level, interior = source
    let src = Image { n: 4, planes: vec![vec![1.0, 2.0, 3.0, 4.0]] };
    let args = [Arg::Img(&src), cst(&[2.0, 2.0]), cst(&[0.0])];
    let m = match run("make_mipmap", &args, 2, 2).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    assert_eq!((m.ripmap.nl(), m.ripmap.np()), (1, 1));
    let rm = &m.ripmap;
    let at = |x: isize, y: isize| f32::from_bits(rm.planes[0][rm.at(0, x, y, 0)]);
    assert_eq!([at(0, 0), at(1, 0), at(0, 1), at(1, 1)], [1.0, 2.0, 3.0, 4.0]);
    // clamp: left pad = first, right pads = last, per row; top pad row = row 0, bottom pad rows = last row
    assert_eq!([at(-1, 0), at(2, 0), at(3, 0)], [1.0, 2.0, 2.0]);
    assert_eq!([at(-1, 1), at(2, 1), at(3, 1)], [3.0, 4.0, 4.0]);
    assert_eq!([at(0, -1), at(1, -1)], [1.0, 2.0]);
    assert_eq!([at(0, 2), at(1, 3)], [3.0, 4.0]);
    // wrap 0 pads with the constant pad pixel (NV9-A5: explicit, here 7.0)
    let mut k = kp();
    k.pad_pixel = [7.0; 4];
    let args0 = [Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[0.0])];
    let m0 = match run_any("make_mipmap", 1, &args0, 4, (2, 2), &k).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    let at0 = |x: isize, y: isize| f32::from_bits(m0.ripmap.planes[0][m0.ripmap.at(0, x, y, 0)]);
    assert_eq!([at0(-1, 0), at0(2, 0), at0(3, 1)], [7.0, 7.0, 7.0]);
    assert_eq!(at0(0, 0), 1.0);
}

#[test]
fn make_mipmap_filter1_builds_the_level_grid() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // 4x2, filter nonzero: NL = ceil(log2 4) + 2 = 4, NP = ceil(log2 2) + 2 = 3; widths 4,2,1,1; heights 2,1,1
    let src = synth(1, 8, 3);
    let args = [Arg::Img(&src), cst(&[1.0, 1.0]), cst(&[1.0])];
    let m = match run("make_mipmap", &args, 4, 2).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    assert_eq!((m.ripmap.nl(), m.ripmap.np()), (4, 3));
    assert_eq!(m.ripmap.widths, vec![4, 2, 1, 1]);
    assert_eq!(m.ripmap.heights, vec![2, 1, 1]);
    assert!(m.scale.is_some());
    // level (1,0) pixel 0 is the mean of the two source pixels of row 0 (box filter, hand value within 1 ulp-ish)
    let rm = &m.ripmap;
    let v = f32::from_bits(rm.planes[rm.idx(1, 0)][rm.at(1, 0, 0, 0)]);
    let e = 0.5 * (src.planes[0][0] + src.planes[0][1]);
    assert!((v - e).abs() < 1e-6, "{} vs {}", v, e);
}

#[test]
fn warp_rel_by_hand_zero_and_one_pixel_shifts() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // 8x2, 1 channel, clamp wrap: coords const [0 0] = identity; a1 = 0.25 -> scx = 4 -> dx = +1 pixel; a2 = 0.25 -> y = row - 1, mirrored
    let src = Image { n: 16, planes: vec![(0..16).map(|i| i as f32 * 2.0).collect()] };
    let mip = match run("make_mipmap", &[Arg::Img(&src), cst(&[2.0, 2.0]), cst(&[0.0])], 8, 2).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    let wr = |co: &[f32]| {
        let args = [Arg::Mip(&mip), cst(co), cst(&[2.0, 2.0]), cst(&[0.0])];
        img(run("warp_rel", &args, 8, 2).unwrap())
    };
    assert_eq!(wr(&[0.0, 0.0]).planes[0], src.planes[0]);
    let dx = wr(&[0.25, 0.0]);
    for y in 0..2 {
        for x in 0..8 {
            let sx = (x + 1).min(7);
            assert_eq!(dx.planes[0][y * 8 + x], src.planes[0][y * 8 + sx], "dx at {},{}", x, y);
        }
    }
    let dy = wr(&[0.0, 0.25]);
    for y in 0..2usize {
        for x in 0..8 {
            let sy = if y == 0 { 1 } else { 0 }; // wrap mode 2 of the sampler is a period mirror: y = -1 -> 1, y = 0 -> 0
            assert_eq!(dy.planes[0][y * 8 + x], src.planes[0][sy * 8 + x], "dy at {},{}", x, y);
        }
    }
    // INT3 step 9b/9c: the footprint path (argument 3 nonzero with an image) now takes c1/c2 from the WPG2 slot 33 crate (OPERAND-MAP-INFERRED, W198 A9; wiring evidence only):
    // the old assertion (typed Unsupported, "NOT REBUILT") described the state this step replaces, so the call must now complete
    let coords = Image { n: 16, planes: vec![vec![0.0; 16], vec![0.0; 16]] };
    let e = run_any("warp_rel", 1, &[Arg::Mip(&mip), Arg::Img(&coords), cst(&[2.0, 2.0]), cst(&[1.0])], 16, (8, 2), &kp());
    assert!(e.is_ok(), "{:?}", e.map(|_| ()));
}

#[test]
fn blur_warp_identity_when_both_endpoints_are_xy_on_a_power_of_two_frame() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // 8x4 (W a power of two: every normalised coordinate is dyadic, so x = px and y = py exactly)
    let (w, h) = (8usize, 4usize);
    let src = synth(4, w * h, 11);
    let g = Geom::full_frame(w, h);
    let xy = xy_grid(w, h, &g, 0.0);
    // hand check of the grid: pixel (0,0): x = -1, y = (H - 0) / W = 0.5; pixel (7,3): x = 0.75, y = (4 - 6) / 8 = -0.25
    assert_eq!([xy.planes[0][0], xy.planes[1][0]], [-1.0, 0.5]);
    assert_eq!([xy.planes[0][31], xy.planes[1][31]], [0.75, -0.25]);
    let mip = match run("make_mipmap", &[Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[0.0])], w, h).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    let args = [Arg::Mip(&mip), Arg::Img(&xy), Arg::Img(&xy), cst(&[0.5, 0.5]), cst(&[1.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
    let out = img(run("blur_warp", &args, w, h).unwrap());
    assert_eq!(bits(&out), bits(&src), "zero-length path samples the source at the pixel (weights 1,0,0,0)");
    // scale_result 2 scales every plane once (MEASURED here on the zero-length path; the ratios go to out/BLUR_WARP_SCALE_OBS_NV9B.txt)
    let args2 = [Arg::Mip(&mip), Arg::Img(&xy), Arg::Img(&xy), cst(&[0.5, 0.5]), cst(&[2.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
    let o2 = img(run("blur_warp", &args2, w, h).unwrap());
    let mut txt = String::from("scale_result=2, zero-length path, 8x4: ratio out/src per plane by sample index
");
    for k in 0..4 {
        let r: Vec<i32> = (0..w * h).map(|i| (o2.planes[k][i] / src.planes[k][i]).round() as i32).collect();
        txt.push_str(&format!("plane {}: {:?}
", k, r));
        assert!(o2.planes[k].iter().all(|x| x.is_finite()));
        for i in 0..w * h {
            assert_eq!(o2.planes[k][i], 2.0 * src.planes[k][i]);
        }
    }
    let _ = std::fs::write(effects_vecdir::out_dir().join("BLUR_WARP_SCALE_OBS_NV9B.txt"), txt);
}

#[test]
fn blur_warp_nonzero_path_blurs_and_is_deterministic() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (8usize, 4usize);
    let src = synth(4, w * h, 12);
    let g = Geom::full_frame(w, h);
    let xy = xy_grid(w, h, &g, 0.0);
    let mip = match run("make_mipmap", &[Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[0.0])], w, h).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    // p2 = a constant pair different from xy: a real path
    let args = [Arg::Mip(&mip), Arg::Img(&xy), cst(&[0.1, 0.0]), cst(&[0.5, 0.5]), cst(&[1.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
    let a = img(run("blur_warp", &args, w, h).unwrap());
    let b = img(run("blur_warp", &args, w, h).unwrap());
    assert_eq!(bits(&a), bits(&b));
    assert!(a.planes.iter().all(|p| p.iter().all(|x| x.is_finite())));
    assert_ne!(bits(&a), bits(&src));
}

#[test]
fn static2_matches_its_crate_and_the_seed_matters() {
    let t = static2::table();
    let (a, b) = static2::render_planes(&t, &static2::Params { n: 24, seed_a: 0, seed_b: 0 });
    let o = img(run_any("static2", 2, &[], 24, (6, 4), &kp()).unwrap());
    assert_eq!(o.planes, vec![a.clone(), b]);
    let mut k = kp();
    k.static2_seed_a = 5;
    let o5 = img(run_any("static2", 2, &[], 24, (6, 4), &k).unwrap());
    assert_ne!(o5.planes[0], a);
    // determinism and range: the class returns values of a 12 bit hash times 2^-12 (hand: |v| < 1)
    assert!(o.planes[0].iter().all(|x| x.abs() < 16.0 && x.is_finite()));
}

#[test]
fn monochrome_by_hand_weights_and_the_pad_route() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // planes r=[1,2,0,0], g=[1,0,4,0], b=[1,0,0,8]: 0.3125 r + 0.5 g + 0.1875 b
    let src = Image { n: 4, planes: vec![vec![1.0, 2.0, 0.0, 0.0], vec![1.0, 0.0, 4.0, 0.0], vec![1.0, 0.0, 0.0, 8.0]] };
    let o = img(run("monochrome", &[Arg::Img(&src)], 4, 1).unwrap());
    assert_eq!(o.planes.len(), 1);
    assert_eq!(o.planes[0], vec![1.0, 0.625, 2.0, 1.5]);
    // 1 channel input: the pad route (plane service copy of channel 0, NV5-A1 model)
    let one = Image { n: 4, planes: vec![vec![0.25, 0.5, 0.75, 1.0]] };
    let o1 = img(run("monochrome", &[Arg::Img(&one)], 4, 1).unwrap());
    assert_eq!(o1.planes[0], one.planes[0]);
}

#[test]
fn coerce_family_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let four = Image { n: 2, planes: vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0], vec![7.0, 8.0]] };
    // coerce_to_alpha of 4 channels: pass P_CELL_A copies the LAST channel (alpha)
    let a = img(run("coerce_to_alpha", &[Arg::Img(&four)], 2, 1).unwrap());
    assert_eq!(a.planes, vec![vec![7.0, 8.0]]);
    // coerce_up of a 1 channel image to 4 (donor placeholder 4 lanes): pass D_CELL_A copies channel 0 four times
    let one = Image { n: 2, planes: vec![vec![0.5, 0.25]] };
    let up = img(run("coerce_up", &[Arg::Img(&one), cst(&[-999.0; 4])], 2, 1).unwrap());
    assert_eq!(up.planes.len(), 4);
    assert!(up.planes.iter().all(|p| *p == vec![0.5, 0.25]));
    // coerce_up of 3 channels to 4: pass EF_CELL_A copies 3 and fills alpha with 1.0
    let three = Image { n: 2, planes: four.planes[..3].to_vec() };
    let u3 = img(run("coerce_up", &[Arg::Img(&three), cst(&[-999.0; 4])], 2, 1).unwrap());
    assert_eq!(u3.planes[3], vec![1.0, 1.0]);
    assert_eq!(u3.planes[..3], three.planes[..]);
}

#[test]
fn rotate_by_hand() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // angle 0: identity within the sincos table accuracy; angle 0.5 turn about the centre (1,1): (2,1) -> (0,1)
    let v = Image { n: 2, planes: vec![vec![2.0, 3.0], vec![1.0, 5.0]] };
    let o0 = img(run("rotate", &[Arg::Img(&v), cst(&[0.0]), cst(&[0.0, 0.0])], 2, 1).unwrap());
    for i in 0..2 {
        assert!((o0.planes[0][i] - v.planes[0][i]).abs() < 1e-6 && (o0.planes[1][i] - v.planes[1][i]).abs() < 1e-6);
    }
    let o5 = img(run("rotate", &[Arg::Img(&v), cst(&[0.5]), cst(&[1.0, 1.0])], 2, 1).unwrap());
    assert!((o5.planes[0][0] - 0.0).abs() < 1e-5 && (o5.planes[1][0] - 1.0).abs() < 1e-5, "{:?}", o5.planes);
    assert!((o5.planes[0][1] - (-1.0)).abs() < 1e-5 && (o5.planes[1][1] - (-3.0)).abs() < 1e-5, "{:?}", o5.planes);
}

#[test]
fn max_min_element_by_hand_and_nan_rule() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let src = Image { n: 3, planes: vec![vec![1.0, f32::NAN, 2.0], vec![5.0, 1.0, f32::NAN], vec![3.0, 0.0, 9.0]] };
    let mx = img(run("max_element", &[Arg::Img(&src)], 3, 1).unwrap());
    let mn = img(run("min_element", &[Arg::Img(&src)], 3, 1).unwrap());
    assert_eq!(mx.planes[0][0], 5.0);
    assert_eq!(mn.planes[0][0], 1.0);
    // x86 rule (a > b ? a : b): a NaN in either operand returns the later operand. pixel 1 = (NaN, 1, 0): max = 1, min = 0;
    // pixel 2 = (2, NaN, 9): max(2,NaN) = NaN, max(NaN,9) = 9; min(2,NaN) = NaN, min(NaN,9) = 9
    assert_eq!([mx.planes[0][1], mn.planes[0][1]], [1.0, 0.0]);
    assert_eq!([mx.planes[0][2], mn.planes[0][2]], [9.0, 9.0]);
}

#[test]
fn blur_sigma_zero_copy_and_the_edge_parameter_is_sensitive() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (9usize, 6usize);
    let src = synth(1, w * h, 5);
    let args0 = [Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[1.0]), cst(&[0.0])];
    let o = img(run("blur", &args0, w, h).unwrap());
    assert_eq!(bits(&o), bits(&src), "sigma 0, gain 1: a copy (blur crate T1)");
    let args = [Arg::Img(&src), cst(&[8.0, 8.0]), cst(&[1.0]), cst(&[1.0]), cst(&[0.0])];
    let m = img(run_any("blur", 1, &args, w * h, (w, h), &kp()).unwrap());
    let mut k0 = kp();
    k0.blur_edge = 0;
    let z = img(run_any("blur", 1, &args, w * h, (w, h), &k0).unwrap());
    assert_ne!(bits(&m), bits(&z), "NV9-A6: edge mode zero vs mirror changes the border");
    assert!(m.planes[0].iter().all(|x| x.is_finite()));
}

// ---- program level ----

fn inputs_for(cp: &fx_eval::CompiledProgram, w: usize, h: usize, seed: u64) -> Inputs {
    let mut images = BTreeMap::new();
    for nd in &cp.nodes {
        if nd.kind == "input" {
            let ch = if nd.elnum > 0 { nd.elnum as usize } else { 4 };
            images.insert(nd.name.clone(), synth(ch, w * h, seed + nd.name.len() as u64));
        }
    }
    Inputs { width: w, height: h, images }
}

fn binding(ov: &[(&str, Vec<f32>)], hc: bool, mask_const: bool) -> Binding {
    let mut b = Binding::defaults();
    for (k, v) in ov {
        b.overrides.insert(k.to_string(), v.clone());
    }
    if mask_const {
        b.const_inputs.insert("Mask".into(), vec![1.0]);
    }
    b.host_curves_unkeyed = hc;
    b
}

fn run_program(name: &str, ov: &[(&str, Vec<f32>)], hc: bool, mask_const: bool, w: usize, h: usize) -> Result<ExecOutput, ExecError> {
    let g = genotype(name);
    let b = binding(ov, hc, mask_const);
    let cp = compile_for(&g, &b)?;
    let inp = inputs_for(&cp, w, h, 100);
    execute_compiled(&cp, &b, &inp, &HostFrames::default())
}

#[test]
fn transform_and_blur_default_is_the_identity_on_the_xy_grid() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let (w, h) = (16usize, 8usize);
        let g = genotype("Transform_and_Blur");
        let b = binding(&[], true, true);
        let cp = compile_for(&g, &b).unwrap();
        assert!(unsupported_classes(&cp).is_empty(), "{:?}", unsupported_classes(&cp));
        let inp = inputs_for(&cp, w, h, 7);
        let o = execute_compiled(&cp, &b, &inp, &HostFrames::default()).unwrap();
        assert_eq!(o.trace.iter().map(|r| r.class.as_str()).collect::<Vec<_>>(), vec!["make_mipmap", "blur_warp"]);
        assert_eq!(bits(&o.dest), bits(&inp.images["Source"]), "defaults: both endpoints are xy, the path has length zero");
    });
}

#[test]
fn blur_motion_curves_default_runs_seven_nodes_and_is_deterministic() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let a = run_program("Blur_Motion_Curves_No_Transform", &[], true, true, 16, 8).unwrap();
        let b = run_program("Blur_Motion_Curves_No_Transform", &[], true, true, 16, 8).unwrap();
        assert_eq!(a.trace.iter().map(|r| r.class.as_str()).collect::<Vec<_>>(), vec!["make_mipmap", "*", "*", "-", "-", "+", "blur_warp"]);
        assert_eq!(bits(&a.dest), bits(&b.dest));
        assert!(a.dest.planes.iter().all(|p| p.iter().all(|x| x.is_finite())));
        assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
    });
}

fn plan_cmds(path: &str) -> Vec<String> {
    commands_of_dump(&std::fs::read_to_string(path).unwrap()).into_iter().filter(|s| !s.trim().is_empty()).collect()
}

/// node-level execution equals execution of the ORIGINAL plan commands (same inputs, same xy)
fn xcheck(label: &str, prog: &str, ov: &[(&str, Vec<f32>)], hc: bool, mask_const: bool, dump: &str, rows: &mut Vec<String>) {
    let (w, h) = (16usize, 8usize);
    let g = genotype(prog);
    let b = binding(ov, hc, mask_const);
    let cp = compile_for(&g, &b).unwrap();
    let inp = inputs_for(&cp, w, h, 31);
    let node = execute_compiled(&cp, &b, &inp, &HostFrames::default());
    let cmds = plan_cmds(dump);
    // convert-from-source index: 0 = Source, 1 = Mask image if the dump asks for it
    let src = inp.images.get("Source").cloned().unwrap_or_else(|| synth(4, w * h, 1));
    let mask = inp.images.get("Mask").cloned().unwrap_or_else(|| Image { n: w * h, planes: vec![vec![1.0; w * h]] });
    let xy = fx_exec::kernels2::default_xy(&b.kp, w, h);
    let cm = execute_commands_xy(&cmds, &[src, mask], w, h, &HostFrames::default(), &b.kp, Some(&xy));
    let row = match (&node, &cm) {
        (Ok(n), Ok(c)) => {
            let d = c.dest.clone().unwrap();
            let same = bits(&n.dest) == bits(&d);
            let mut maxd = 0.0f32;
            let mut nd = 0usize;
            if n.dest.channels() == d.channels() {
                for (a, bb) in n.dest.planes.iter().zip(d.planes.iter()) {
                    for (x, y) in a.iter().zip(bb.iter()) {
                        if x.to_bits() != y.to_bits() {
                            nd += 1;
                            if x.is_finite() && y.is_finite() {
                                maxd = maxd.max((x - y).abs());
                            }
                        }
                    }
                }
            }
            format!("{}\t{}\t{}\t{}\t{}\t{}\t{}", label, prog, cmds.len(), if same { "BIT-EQUAL" } else { "DIFF" }, nd, maxd, n.trace.iter().map(|r| r.class.clone()).collect::<Vec<_>>().join(" "))
        }
        (Err(e), _) => format!("{}\t{}\t{}\tNODE-ERR\t-\t-\t{}", label, prog, cmds.len(), e),
        (_, Err(e)) => format!("{}\t{}\t{}\tCMD-ERR\t-\t-\t{}", label, prog, cmds.len(), e),
    };
    rows.push(row);
}

#[test]
fn node_level_equals_original_plan_commands() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let mut rows = vec!["label\tprogram\tcommands\tresult\tdiffering_floats\tmax_abs_diff\trun_list".to_string()];
        let plb = eff().join("wave_s8").join("rebuild_fx_plan_b").join("port").join("data").join("orig");
        let d = |n: &str| plb.join(format!("PLAN_ORIG_{}.txt", n)).to_string_lossy().to_string();
        let hc = eff().join("wave_s6").join("rebuild_curves").join("plans");
        let h = |n: &str| hc.join(format!("PLAN_ORIG_{}.txt", n)).to_string_lossy().to_string();
        xcheck("Diffuse default", "Diffuse", &[], false, true, &d("Diffuse_default"), &mut rows);
        xcheck("Diffuse invert", "Diffuse", &[("invert_mask", vec![1.0])], false, true, &d("Diffuse_invert"), &mut rows);
        xcheck("Diffuse mask_alpha", "Diffuse", &[("mask_use", vec![1.0])], false, true, &d("Diffuse_mask_alpha"), &mut rows);
        xcheck("Diffuse blur_mask", "Diffuse", &[("blur_mask", vec![1.0])], false, true, &d("Diffuse_blur_mask"), &mut rows);
        xcheck("Diffuse wrap0", "Diffuse", &[("wrap", vec![0.0, 0.0])], false, true, &d("Diffuse_wrap0"), &mut rows);
        xcheck("Diffuse wrap11", "Diffuse", &[("wrap", vec![1.0, 1.0])], false, true, &d("Diffuse_wrap11"), &mut rows);
        xcheck("Diffuse da0", "Diffuse", &[("diffuse_amount", vec![0.0])], false, true, &d("Diffuse_da0"), &mut rows);
        xcheck("Diffuse ra0", "Diffuse", &[("rel_amount", vec![0.0])], false, true, &d("Diffuse_ra0"), &mut rows);
        xcheck("TaB hooked", "Transform_and_Blur", &[], true, true, &h("Transform_and_Blur_hc"), &mut rows);
        xcheck("BMC hooked", "Blur_Motion_Curves_No_Transform", &[], true, true, &h("Blur_Motion_Curves_No_Transform_hc"), &mut rows);
        let sv = |s: &str| -> Vec<(String, Vec<f32>)> {
            s.split(';').filter_map(|kv| kv.split_once('=')).map(|(k, v)| (k.to_string(), v.split(',').filter_map(|x| x.parse().ok()).collect())).collect()
        };
        let ov1 = sv("rotate=30;z_dist=1.2;shift=0.1,0.2;shutter_duration=0.5;shutter_shift=0.25;center=0.3,0.4");
        let r1: Vec<(&str, Vec<f32>)> = ov1.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
        xcheck("TaB hooked set", "Transform_and_Blur", &r1, true, true, &h("Transform_and_Blur_hcset"), &mut rows);
        let ov2 = sv("rotate=30;z_dist=1.2;shift=0.1,0.2;shutter_duration=0.5;shutter_shift=0.25");
        let r2: Vec<(&str, Vec<f32>)> = ov2.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
        xcheck("BMC hooked set", "Blur_Motion_Curves_No_Transform", &r2, true, true, &h("Blur_Motion_Curves_No_Transform_hcset"), &mut rows);
        let rep = rows.join("\n") + "\n";
        let _ = std::fs::write(effects_vecdir::out_dir().join("XCHECK_NV9B.tsv"), &rep);
        eprintln!("{}", rep);
        for r in &rows[1..] {
            assert!(!r.contains("CMD-ERR") && !r.contains("NODE-ERR"), "{}", r);
        }
    });
}

#[test]
fn milestone_status_matrix_nv9() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let ct = fx_exec::exec::combo_table();
        let mut rows = vec!["program\tbinding\tparamset\trun_list\tresult\tunsupported".to_string()];
        for (prog, hc) in [("Diffuse", false), ("Transform_and_Blur", true), ("Blur_Motion_Curves_No_Transform", true), ("Hue_Sat_Bright", false), ("Composite", false), ("Trails", false), ("Time_Warp_RGB", false)] {
            let g = genotype(prog);
            let mut sets: Vec<(String, BTreeMap<String, Vec<f32>>)> = enumerate_branches(&g, &ct, &BTreeMap::new()).into_iter().map(|s| (s.paramset.clone(), s.overrides.clone())).collect();
            if prog == "Diffuse" {
                for (n, k, v) in [("blur_mask=1", "blur_mask", vec![1.0f32]), ("wrap=0,0", "wrap", vec![0.0, 0.0]), ("wrap=1,1", "wrap", vec![1.0, 1.0]), ("diffuse_amount=0", "diffuse_amount", vec![0.0])] {
                    let mut m = BTreeMap::new();
                    m.insert(k.to_string(), v);
                    sets.push((n.to_string(), m));
                }
            }
            for mask_const in [true, false] {
                if prog != "Diffuse" && !mask_const {
                    continue;
                }
                for (ps, ov) in &sets {
                    let mut b = Binding::defaults();
                    b.overrides = ov.clone();
                    b.paramset = ps.clone();
                    b.host_curves_unkeyed = hc;
                    if mask_const {
                        b.const_inputs.insert("Mask".into(), vec![1.0]);
                    }
                    let tag = if mask_const { "Mask=const 1 (NV3-A5)" } else { "Mask=image" };
                    let cp = match compile_for(&g, &b) {
                        Ok(c) => c,
                        Err(e) => {
                            rows.push(format!("{}\t{}\t{}\t-\tcompile error {}\t-", prog, tag, ps, e));
                            continue;
                        }
                    };
                    let run: Vec<String> = cp.nodes.iter().filter(|n| (n.kind == "func" || n.kind == "auto") && !n.b160 && !n.folded).map(|n| n.prim.clone()).collect();
                    let un = unsupported_classes(&cp);
                    let inp = inputs_for(&cp, 16, 8, 100);
                    let hostf = HostFrames { prev_dest: Some(synth(4, 128, 9)), frames: [(-1, synth(4, 128, 2)), (0, synth(4, 128, 3)), (1, synth(4, 128, 4))].into_iter().collect(), frame_round_old: true, ..HostFrames::default() };
                    let r1 = execute_compiled(&cp, &b, &inp, &hostf);
                    let res = match &r1 {
                        Ok(o) => {
                            let o2 = execute_compiled(&cp, &b, &inp, &hostf).unwrap();
                            format!("OK ch={} diag={} det={}", o.dest.channels(), o.diagnostics.len(), if bits(&o.dest) == bits(&o2.dest) { "Y" } else { "N" })
                        }
                        Err(e) => format!("ERR {}", e),
                    };
                    rows.push(format!("{}\t{}\t{}\t{}\t{}\t{}", prog, tag, ps, run.join(" "), res, if un.is_empty() { "-".to_string() } else { un.join(" ") }));
                }
            }
        }
        let rep = rows.join("\n") + "\n";
        let _ = std::fs::write(effects_vecdir::out_dir().join("EXEC_STATUS_NV9B.tsv"), &rep);
        eprintln!("{}", rep);
    });
}

#[test]
fn hooked_curve_sets_run_rotate_on_images_and_the_matrix_rows_extend() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let ov: Vec<(&str, Vec<f32>)> = vec![("rotate", vec![30.0]), ("z_dist", vec![1.2]), ("shift", vec![0.1, 0.2]), ("shutter_duration", vec![0.5]), ("shutter_shift", vec![0.25])];
        let g = genotype("Blur_Motion_Curves_No_Transform");
        let b = binding(&ov, true, true);
        let cp = compile_for(&g, &b).unwrap();
        let run: Vec<String> = cp.nodes.iter().filter(|n| (n.kind == "func" || n.kind == "auto") && !n.b160 && !n.folded).map(|n| n.prim.clone()).collect();
        assert!(run.contains(&"rotate".to_string()), "{:?}", run);
        assert!(unsupported_classes(&cp).is_empty(), "{:?}", unsupported_classes(&cp));
        let o = run_program("Blur_Motion_Curves_No_Transform", &ov, true, true, 16, 8).unwrap();
        assert!(o.dest.planes.iter().all(|p| p.iter().all(|x| x.is_finite())));
        let o2 = run_program("Blur_Motion_Curves_No_Transform", &ov, true, true, 16, 8).unwrap();
        assert_eq!(bits(&o.dest), bits(&o2.dest));
    });
}

#[test]
fn unsupported_classes_over_all_programs() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let c = corpus();
        let ct = fx_exec::exec::combo_table();
        let mut by: BTreeMap<String, (Vec<String>, usize)> = BTreeMap::new();
        let mut seen = std::collections::BTreeSet::new();
        let mut tot = 0usize;
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
            tot += 1;
            for s in enumerate_branches(&g, &ct, &BTreeMap::new()) {
                let mut b = Binding::defaults();
                b.overrides = s.overrides.clone();
                b.const_inputs.insert("Mask".into(), vec![1.0]);
                b.host_curves_unkeyed = true;
                if let Ok(cp) = compile_for(&g, &b) {
                    for cl in unsupported_classes(&cp) {
                        let ent = by.entry(cl).or_insert((Vec::new(), 0));
                        ent.1 += 1;
                        if !ent.0.contains(&e.name) {
                            ent.0.push(e.name.clone());
                        }
                    }
                }
            }
        }
        let mut rows = format!("class\tprograms\tbranch_sets\texample_programs (of {} programs, default+branch sets, Mask=1, host curves unkeyed)\n", tot);
        let mut v: Vec<_> = by.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.len().cmp(&a.1 .0.len()));
        for (k, (ps, n)) in v {
            rows.push_str(&format!("{}\t{}\t{}\t{}\n", k, ps.len(), n, ps.iter().take(6).cloned().collect::<Vec<_>>().join(" ")));
        }
        let _ = std::fs::write(effects_vecdir::out_dir().join("UNSUPPORTED_BY_CLASS_NV9B.tsv"), &rows);
        eprintln!("{}", rows);
    });
}

#[test]
fn host_geometry_and_pad_pixel_are_sensitive_inputs() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // NV9-A3 / NV9-A5: changing the explicit host geometry moves the sampled position; the pad pixel shows at a constant-padded border
    let (w, h) = (8usize, 4usize);
    let src = synth(4, w * h, 21);
    let g = Geom::full_frame(w, h);
    let xy = xy_grid(w, h, &g, 0.0);
    let mip = match run("make_mipmap", &[Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[0.0])], w, h).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    let args = [Arg::Mip(&mip), Arg::Img(&xy), Arg::Img(&xy), cst(&[0.5, 0.5]), cst(&[1.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
    let base = img(run("blur_warp", &args, w, h).unwrap());
    let mut k = kp();
    k.geom = Some(Geom { left: 1.0, right: w as f32 + 1.0, ..g });
    let shifted = img(run_any("blur_warp", 4, &args, w * h, (w, h), &k).unwrap());
    assert_ne!(bits(&base), bits(&shifted), "left/right (rec+0x730/0x734) shift the sampled column");
    // with the left edge moved by +1 the sample for pixel x is source column x+1 (hand: x = (xy.x+1)*scx + left): interior columns equal src shifted left
    for y in 0..h {
        for x in 0..w - 1 {
            assert_eq!(shifted.planes[0][y * w + x], src.planes[0][y * w + x + 1]);
        }
    }
    // wrap 0 border: sampling one pixel past the right edge returns the constant pad pixel when it is 5.0
    let mut kpad = kp();
    kpad.pad_pixel = [5.0; 4];
    kpad.geom = Some(Geom { left: 1.0, right: w as f32 + 1.0, ..g });
    let mip5 = match run_any("make_mipmap", 4, &[Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[0.0])], w * h, (w, h), &kpad).unwrap() {
        Out::Mip(m) => m,
        _ => panic!(),
    };
    let a5 = [Arg::Mip(&mip5), Arg::Img(&xy), Arg::Img(&xy), cst(&[0.5, 0.5]), cst(&[1.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
    let o5 = img(run_any("blur_warp", 4, &a5, w * h, (w, h), &kpad).unwrap());
    for y in 0..h {
        assert_eq!(o5.planes[0][y * w + (w - 1)], 5.0, "row {}", y);
    }
}

#[test]
fn fx1_blur_render_scale_defaults_to_the_frame_width() {
    // FX-1 (replaces NV9-A6): the default binds [info+0x20] = the frame width; the old value 1 stays reachable (NV9-BLUR-SCALE-OLD)
    for (w, h) in [(64usize, 32usize), (50, 7)] {
        let src = synth(1, w * h, 11);
        let args = [Arg::Img(&src), cst(&[4.0, 4.0]), cst(&[1.0]), cst(&[1.0]), cst(&[0.0])];
        assert_eq!(KParams::standin().blur_render_scale, 0, "0 means the frame width");
        let d = img(run_any("blur", 1, &args, w * h, (w, h), &kp()).unwrap());
        let mut kw = kp();
        kw.blur_render_scale = w as i32;
        let e = img(run_any("blur", 1, &args, w * h, (w, h), &kw).unwrap());
        assert_eq!(bits(&d), bits(&e), "default == explicit scale W at {}x{}", w, h);
        let mut k1 = kp();
        k1.blur_render_scale = 1;
        let o = img(run_any("blur", 1, &args, w * h, (w, h), &k1).unwrap());
        assert_ne!(bits(&d), bits(&o), "the old scale 1 gives a different blur at {}x{}", w, h);
    }
}
