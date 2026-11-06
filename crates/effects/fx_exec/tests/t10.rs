//! T10: the dump's host geometry as the default binding, the NV9-GEOM-OLD switch, the geometry delta
//! on Diffuse and Blur_Motion_Curves_No_Transform, and node-level execution against the native plan text of BOTH plan modes
//! (GpuFirst r8 = 1 and CpuDriver r8 = 0) of the compiler. Outputs: out/GEOM_DELTA_NV9B.tsv, out/MODES_NV9B.tsv.
use fx_eval::cpuplan::{plan_text_mode, PlanMode};
use fx_eval::{build_effect, Corpus, Genotype};
use fx_exec::cmds::{commands_of_dump, execute_commands_xy};
use fx_exec::kernels::run_any;
use fx_exec::kernels2::{default_xy, Out};
use fx_exec::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn corpus() -> Corpus {
    let p = effects_vecdir::root().join("record_extract").join("programs").join("programs.tsv");
    Corpus::from_programs_tsv(&std::fs::read_to_string(&p).unwrap())
}
fn genotype(name: &str) -> Genotype {
    let c = corpus();
    let ei = c.find_effect(name, None).unwrap_or_else(|| panic!("effect {} not found", name));
    build_effect(&c, ei).unwrap()
}
fn synth(ch: usize, n: usize, seed: u64) -> Image {
    let mut s = seed;
    let planes = (0..ch)
        .map(|_| {
            (0..n)
                .map(|_| {
                    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    0.05 + ((s >> 33) as u32 as f32 / 2147483648.0)
                })
                .collect()
        })
        .collect();
    Image::from_planes(planes).unwrap()
}
fn bits(i: &Image) -> Vec<Vec<u32>> {
    i.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect()
}
fn cst(v: &[f32]) -> Arg<'static> {
    let mut l = [*v.last().unwrap(); 4].map(|x: f32| x.to_bits());
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}
fn run_with(class: &str, args: &[Arg], w: usize, h: usize, k: &KParams) -> Out {
    let c: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let t = fx_exec::selector::Tables::embedded().select_counts(class, &c);
    assert!(t > 0);
    run_any(class, t, args, w * h, (w, h), k).unwrap()
}
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
fn binding(ov: &[(&str, Vec<f32>)], hc: bool, mask_const: bool, old: bool) -> Binding {
    let mut b = Binding::defaults();
    for (k, v) in ov {
        b.overrides.insert(k.to_string(), v.clone());
    }
    if mask_const {
        b.const_inputs.insert("Mask".into(), vec![1.0]);
    }
    b.host_curves_unkeyed = hc;
    if old {
        b.kp.geom_mode = GeomMode::Nv9Old;
    }
    b
}
fn host_frames(n: usize) -> HostFrames {
    HostFrames { prev_dest: Some(synth(4, n, 9)), frames: [(-1, synth(4, n, 2)), (0, synth(4, n, 3)), (1, synth(4, n, 4))].into_iter().collect(), frame_round_old: true, ..HostFrames::default() }
}

#[test]
fn dump_geometry_by_hand() {
    // 64 x 32, a = 1: assumption_f = -0.5, assumption_g = 63.5, t = (32 - 1)/2 = 15.5, u = (63.5 + 0.5)/2 = 32 => assumption_h = -16.5, geom33_a = 47.5, aspect_block_d = 1
    let g = Geom::dump(64, 32, 1.0);
    assert_eq!((g.left, g.right, g.assumption_e, g.assumption_h, g.aspect, g.roi_x, g.roi_y), (-0.5, 63.5, 0.0, -16.5, 1.0, 0, 0));
    assert_eq!(Geom::dump_f_cell_a(64, 32, 1.0), 47.5);
    // a = 2: u = 64/4 = 16 => assumption_h = -0.5, geom33_a = 31.5, aspect_block_d = 0.5
    let g2 = Geom::dump(64, 32, 2.0);
    assert_eq!((g2.assumption_h, g2.aspect), (-0.5, 0.5));
    assert_eq!(Geom::dump_f_cell_a(64, 32, 2.0), 31.5);
    // odd size 50 x 7: t = 3, u = 50/2 = 25 => assumption_h = -22
    let g3 = Geom::dump(50, 7, 1.0);
    assert_eq!((g3.right, g3.assumption_h), (49.5, -22.0));
    // the old binding for contrast (NV9-GEOM-OLD): left 0, right W, assumption_h 0, assumption_e = (H - W)/2
    let o = Geom::full_frame(64, 32);
    assert_eq!((o.left, o.right, o.assumption_h, o.assumption_e), (0.0, 64.0, 0.0, -16.0));
    // KParams default is the dump binding
    assert_eq!(KParams::standin().geom_mode, GeomMode::Dump);
    assert_eq!(kernels2::geom_of(&KParams::standin(), 64, 32), g);
}

#[test]
fn pixel_centre_plane_by_hand_and_equals_the_grid_of_the_dump_geometry() {
    let xy = xy_pixel_centre(8, 4);
    // pixel (0,0): x = 1/8 - 1 = -0.875, y = 3/8; pixel (7,3): x = 15/8 - 1 = 0.875, y = (3 - 6)/8 = -0.375
    assert_eq!([xy.planes[0][0], xy.planes[1][0]], [-0.875, 0.375]);
    assert_eq!([xy.planes[0][31], xy.planes[1][31]], [0.875, -0.375]);
    for (w, h) in [(8usize, 4usize), (50, 7), (64, 32)] {
        let a = xy_pixel_centre(w, h);
        let b = xy_grid(w, h, &Geom::dump(w, h, 1.0), 0.0);
        let mut m = 0.0f32;
        for k in 0..2 {
            for i in 0..w * h {
                m = m.max((a.planes[k][i] - b.planes[k][i]).abs());
            }
        }
        assert!(m < 2e-6, "{}x{}: pixel-centre plane vs xy_grid of the dump geometry differ by {}", w, h, m);
    }
    // the full-frame plane (NV9-GEOM-OLD) under the dump geometry lands at (px - 0.5, py - 0.5): not the identity
    let old = xy_grid(8, 4, &Geom::full_frame(8, 4), 0.0);
    assert_eq!([old.planes[0][0], old.planes[1][0]], [-1.0, 0.5]);
}

#[test]
fn blur_warp_identity_on_the_dump_geometry_and_half_pixel_shift_of_the_old_plane() {
    if !effects_rtdata::have_tables() {
        return;
    }
    for (w, h) in [(8usize, 4usize), (16, 8)] {
        let src = synth(4, w * h, 21);
        let k = KParams::standin();
        let mip = match run_with("make_mipmap", &[Arg::Img(&src), cst(&[0.0, 0.0]), cst(&[0.0])], w, h, &k) {
            Out::Mip(m) => m,
            _ => panic!(),
        };
        let xy = xy_pixel_centre(w, h);
        let args = [Arg::Mip(&mip), Arg::Img(&xy), Arg::Img(&xy), cst(&[0.5, 0.5]), cst(&[1.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
        let out = run_with("blur_warp", &args, w, h, &k).into_image().unwrap();
        assert_eq!(bits(&out), bits(&src), "{}x{}: pixel-centre plane on the dump geometry is the identity", w, h);
        // NV9-GEOM-OLD: the old plane with the old geometry is the identity too ...
        let mut ko = KParams::standin();
        ko.geom_mode = GeomMode::Nv9Old;
        let xo = xy_grid(w, h, &Geom::full_frame(w, h), 0.0);
        let argso = [Arg::Mip(&mip), Arg::Img(&xo), Arg::Img(&xo), cst(&[0.5, 0.5]), cst(&[1.0]), cst(&[0.0, 0.0]), cst(&[1.0]), cst(&[0.0])];
        let outo = run_with("blur_warp", &argso, w, h, &ko).into_image().unwrap();
        assert_eq!(bits(&outo), bits(&src));
        // ... but the old plane under the dump geometry is shifted by (-0.5, -0.5) and blends neighbours (HIGH-STAKES 1)
        let shifted = run_with("blur_warp", &argso, w, h, &k).into_image().unwrap();
        assert_ne!(bits(&shifted), bits(&src));
    }
}

#[test]
fn programs_default_to_the_dump_geometry_and_the_old_switch_still_runs() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let (w, h) = (16usize, 8usize);
        for old in [false, true] {
            let g = genotype("Transform_and_Blur");
            let b = binding(&[], true, true, old);
            let cp = compile_for(&g, &b).unwrap();
            let inp = inputs_for(&cp, w, h, 7);
            let o = execute_compiled(&cp, &b, &inp, &HostFrames::default()).unwrap();
            assert_eq!(bits(&o.dest), bits(&inp.images["Source"]), "old={}: default Transform_and_Blur is the identity on its own xy", old);
        }
    });
}

fn delta_row(prog: &str, set: &str, ov: &[(&str, Vec<f32>)], hc: bool, mask_const: bool, w: usize, h: usize, rows: &mut Vec<String>) {
    let g = genotype(prog);
    let bn = binding(ov, hc, mask_const, false);
    let bo = binding(ov, hc, mask_const, true);
    let cp = compile_for(&g, &bn).unwrap();
    let inp = inputs_for(&cp, w, h, 55);
    let hf = host_frames(w * h);
    let a = execute_compiled(&cp, &bn, &inp, &hf).unwrap();
    let b = execute_compiled(&cp, &bo, &inp, &hf).unwrap();
    let run: Vec<String> = a.trace.iter().map(|r| r.class.clone()).collect();
    let (mut nd, mut tot, mut maxd, mut sum2, mut sumd, mut mx) = (0usize, 0usize, 0.0f32, 0.0f64, 0.0f64, 0.0f32);
    for (p, q) in a.dest.planes.iter().zip(b.dest.planes.iter()) {
        for (x, y) in p.iter().zip(q.iter()) {
            tot += 1;
            mx = mx.max(x.abs());
            if x.to_bits() != y.to_bits() {
                nd += 1;
                let d = (x - y).abs();
                maxd = maxd.max(d);
                sum2 += (d as f64) * (d as f64);
                sumd += d as f64;
            }
        }
    }
    let rms = if tot > 0 { (sum2 / tot as f64).sqrt() } else { 0.0 };
    rows.push(format!("{}\t{}\t{}x{}\t{}\t{}\t{}\t{}\t{:.3e}\t{:.3e}\t{:.3e}\t{:.3}", prog, set, w, h, run.join(" "), tot, nd, if nd == 0 { "BIT-EQUAL".to_string() } else { "DIFF".to_string() }, maxd, rms, if nd > 0 { sumd / nd as f64 } else { 0.0 }, mx));
}

#[test]
fn geometry_delta_dump_vs_old_on_diffuse_and_blur_motion_curves() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let mut rows = vec!["program\tset\tsize\trun_list\tfloats\tdiffering\tresult\tmax_abs_diff\trms_over_all\tmean_abs_over_differing\tmax_abs_value".to_string()];
        let sets_d: Vec<(&str, Vec<(&str, Vec<f32>)>)> = vec![
            ("default", vec![]),
            ("blur_mask=1", vec![("blur_mask", vec![1.0])]),
            ("wrap=0,0", vec![("wrap", vec![0.0, 0.0])]),
            ("wrap=1,1", vec![("wrap", vec![1.0, 1.0])]),
            ("diffuse_amount=0", vec![("diffuse_amount", vec![0.0])]),
            ("rel_amount=2,0.5", vec![("rel_amount", vec![2.0, 0.5])]),
        ];
        let sets_b: Vec<(&str, Vec<(&str, Vec<f32>)>)> = vec![
            ("default", vec![]),
            ("rotate=30 z_dist=1.2 shift=0.1,0.2 shutter 0.5/0.25", vec![("rotate", vec![30.0]), ("z_dist", vec![1.2]), ("shift", vec![0.1, 0.2]), ("shutter_duration", vec![0.5]), ("shutter_shift", vec![0.25])]),
            ("shift=0.3,-0.2 shutter_duration=1", vec![("shift", vec![0.3, -0.2]), ("shutter_duration", vec![1.0])]),
        ];
        for (w, h) in [(16usize, 8usize), (50, 7), (64, 32)] {
            for (s, ov) in &sets_d {
                delta_row("Diffuse", s, ov, false, true, w, h, &mut rows);
            }
            for (s, ov) in &sets_b {
                delta_row("Blur_Motion_Curves_No_Transform", s, ov, true, true, w, h, &mut rows);
            }
            delta_row("Transform_and_Blur", "default", &[], true, true, w, h, &mut rows);
            delta_row("Transform_and_Blur", "rotate=30 z_dist=1.2 shift=0.1,0.2 shutter 0.5/0.25", &[("rotate", vec![30.0]), ("z_dist", vec![1.2]), ("shift", vec![0.1, 0.2]), ("shutter_duration", vec![0.5]), ("shutter_shift", vec![0.25])], true, true, w, h, &mut rows);
        }
        let rep = rows.join("\n") + "\n";
        let _ = std::fs::write(effects_vecdir::out_dir().join("GEOM_DELTA_NV9B.tsv"), &rep);
        eprintln!("{}", rep);
    });
}

/// commands of a native plan text (the `CMD\t<i>\t<kind>\t<text>\t...` lines), empty and non-command lines dropped
fn native_cmds(text: &str) -> Vec<String> {
    commands_of_dump(text).into_iter().filter(|s| s.trim_start().starts_with('(')).collect()
}

fn mode_row(label: &str, prog: &str, ov: &[(&str, Vec<f32>)], hc: bool, mask_const: bool, rows: &mut Vec<String>) -> bool {
    let (w, h) = (16usize, 8usize);
    let g = genotype(prog);
    let b = binding(ov, hc, mask_const, false);
    let cp = compile_for(&g, &b).unwrap();
    let inp = inputs_for(&cp, w, h, 31);
    let hf = host_frames(w * h);
    let node = execute_compiled(&cp, &b, &inp, &hf);
    let src = inp.images.get("Source").cloned().unwrap_or_else(|| synth(4, w * h, 1));
    let mask = inp.images.get("Mask").cloned().unwrap_or_else(|| Image { n: w * h, planes: vec![vec![1.0; w * h]] });
    let xy = default_xy(&b.kp, w, h);
    let mut res: Vec<Result<Image, String>> = Vec::new();
    let mut ncmd = Vec::new();
    for mode in [PlanMode::GpuFirst, PlanMode::CpuDriver] {
        let cmds = native_cmds(&plan_text_mode(&cp, mode));
        ncmd.push(cmds.len());
        res.push(match execute_commands_xy(&cmds, &[src.clone(), mask.clone()], w, h, &hf, &b.kp, Some(&xy)) {
            Ok(c) => c.dest.ok_or_else(|| "no dest".to_string()),
            Err(e) => Err(format!("{}", e)),
        });
    }
    let cmp = |a: &Image, d: &Image| -> (usize, f32) {
        let mut nd = 0usize;
        let mut mx = 0.0f32;
        if a.channels() != d.channels() {
            return (usize::MAX, f32::NAN);
        }
        for (p, q) in a.planes.iter().zip(d.planes.iter()) {
            for (x, y) in p.iter().zip(q.iter()) {
                if x.to_bits() != y.to_bits() {
                    nd += 1;
                    if x.is_finite() && y.is_finite() {
                        mx = mx.max((x - y).abs());
                    }
                }
            }
        }
        (nd, mx)
    };
    let mut ok = true;
    let run: Vec<String> = node.as_ref().map(|o| o.trace.iter().map(|r| r.class.clone()).collect()).unwrap_or_default();
    let cell = |r: &Result<Image, String>, ok: &mut bool| -> String {
        match (&node, r) {
            (Ok(n), Ok(d)) => {
                let (nd, mx) = cmp(&n.dest, d);
                if nd != 0 && !(mx <= 1e-5) {
                    *ok = false;
                }
                if nd == 0 { "BIT-EQUAL".to_string() } else { format!("DIFF {} floats max {:.2e}", nd, mx) }
            }
            (Err(e), _) => {
                *ok = false;
                format!("NODE-ERR {}", e)
            }
            (_, Err(e)) => format!("CMD-ERR {}", e.chars().take(80).collect::<String>()),
        }
    };
    let c0 = cell(&res[0], &mut ok);
    let c1 = cell(&res[1], &mut ok);
    let m2m = match (&res[0], &res[1]) {
        (Ok(a), Ok(b)) => if bits(a) == bits(b) { "BIT-EQUAL".to_string() } else { "DIFF".to_string() },
        _ => "-".to_string(),
    };
    rows.push(format!("{}\t{}\t{}\t{}/{}\t{}\t{}\t{}", label, prog, run.join(" "), ncmd[0], ncmd[1], c0, c1, m2m));
    ok
}

#[test]
fn node_level_equals_native_plan_text_in_both_plan_modes() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    big(|| {
        let mut rows = vec!["label\tprogram\trun_list\tcommands_gpu/cpu\tnode_vs_gpufirst_cmds\tnode_vs_cpudriver_cmds\tgpufirst_vs_cpudriver_cmds".to_string()];
        let mut core_ok = true;
        let mut core = |label: &str, prog: &str, ov: &[(&str, Vec<f32>)], hc: bool, mc: bool, rows: &mut Vec<String>| {
            core_ok &= mode_row(label, prog, ov, hc, mc, rows);
        };
        core("Diffuse default", "Diffuse", &[], false, true, &mut rows);
        core("Diffuse invert", "Diffuse", &[("invert_mask", vec![1.0])], false, true, &mut rows);
        core("Diffuse mask_use", "Diffuse", &[("mask_use", vec![1.0])], false, true, &mut rows);
        core("Diffuse blur_mask", "Diffuse", &[("blur_mask", vec![1.0])], false, true, &mut rows);
        core("Diffuse wrap0", "Diffuse", &[("wrap", vec![0.0, 0.0])], false, true, &mut rows);
        core("Diffuse da0", "Diffuse", &[("diffuse_amount", vec![0.0])], false, true, &mut rows);
        core("TaB hooked", "Transform_and_Blur", &[], true, true, &mut rows);
        core("BMC hooked", "Blur_Motion_Curves_No_Transform", &[], true, true, &mut rows);
        core("BMC hooked set", "Blur_Motion_Curves_No_Transform", &[("rotate", vec![30.0]), ("z_dist", vec![1.2]), ("shift", vec![0.1, 0.2]), ("shutter_duration", vec![0.5]), ("shutter_shift", vec![0.25])], true, true, &mut rows);
        core("Hue_Sat_Bright default", "Hue_Sat_Bright", &[], false, true, &mut rows);
        core("Hue_Sat_Bright set", "Hue_Sat_Bright", &[("hue_shift", vec![0.25]), ("saturation", vec![1.5]), ("brightness", vec![0.8])], false, true, &mut rows);
        core("Trails default", "Trails", &[], false, true, &mut rows);
        let rep = rows.join("\n") + "\n";
        let _ = std::fs::write(effects_vecdir::out_dir().join("MODES_NV9B.tsv"), &rep);
        eprintln!("{}", rep);
        assert!(core_ok, "node-level and native plan text must agree within 1e-5 in both modes");
    });
}
