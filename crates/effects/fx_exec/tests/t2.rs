//! T2: hand-evaluated programs on the node-level path: Time_Warp_RGB (HOST-FRAMES get_frame), Trails
//! (get_prev_dest, save_prev_dest), Hue_Sat_Bright single-parameter sets, errors for missing host frames.
use fx_eval::{build_effect, Corpus, Genotype};
use fx_exec::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
}

fn genotype(name: &str) -> Genotype {
    let p = effects_vecdir::root().join("record_extract").join("programs").join("programs.tsv");
    let t = std::fs::read_to_string(&p).unwrap();
    let c = Corpus::from_programs_tsv(&t);
    build_effect(&c, c.find_effect(name, None).unwrap()).unwrap()
}

fn ramp(ch: usize, n: usize, base: f32) -> Image {
    Image::from_planes((0..ch).map(|c| (0..n).map(|i| base + c as f32 * 0.125 + i as f32 * 0.01).collect()).collect()).unwrap()
}

fn binding(ov: &[(&str, Vec<f32>)]) -> Binding {
    let mut b = Binding::defaults();
    for (k, v) in ov {
        b.overrides.insert(k.to_string(), v.clone());
    }
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    b
}

#[test]
fn time_warp_rgb_default_is_a_channel_gather_of_three_host_frames() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data"]) {
        return;
    }
    big(|| {
        let g = genotype("Time_Warp_RGB");
        let (w, h) = (6, 3);
        let n = w * h;
        let mut hf = HostFrames::default();
        hf.frame_round_old = true; // NV5-FRAME-ROUND-OLD: this test encodes the frame rule round(t), keys -1, 0, 1
        hf.frames.insert(1, ramp(4, n, 1.0));
        hf.frames.insert(0, ramp(4, n, 2.0));
        hf.frames.insert(-1, ramp(4, n, 3.0));
        let mut images = BTreeMap::new();
        images.insert("Source".to_string(), ramp(4, n, 9.0));
        let inp = Inputs { width: w, height: h, images };
        let o = execute(&g, &binding(&[]), &inp, &hf).unwrap();
        assert_eq!(o.dest.channels(), 3);
        // (get_frame [1]) element 0, (get_frame [0]) element 1, (get_frame [-1]) element 2 (plan text of the original's plan dump)
        assert_eq!(o.dest.planes[0], hf.frames[&1].planes[0]);
        assert_eq!(o.dest.planes[1], hf.frames[&0].planes[1]);
        assert_eq!(o.dest.planes[2], hf.frames[&-1].planes[2]);
        // a missing host frame is a named error, not a guess
        hf.frames.remove(&-1);
        let e = execute(&g, &binding(&[]), &inp, &hf);
        assert!(matches!(e, Err(ExecError::MissingHostFrame(_))), "{:?}", e);
    });
}

#[test]
fn trails_default_blends_prev_dest_and_source_and_saves_the_result() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data"]) {
        return;
    }
    big(|| {
        let g = genotype("Trails");
        let (w, h) = (5, 3);
        let n = w * h;
        let src = ramp(4, n, 0.25);
        let prev = ramp(4, n, 0.5);
        let mut images = BTreeMap::new();
        images.insert("Source".to_string(), src.clone());
        let inp = Inputs { width: w, height: h, images };
        let hf = HostFrames { prev_dest: Some(prev.clone()), frames: BTreeMap::new(), frame_round_old: false, ..HostFrames::default() };
        let o = execute(&g, &binding(&[]), &inp, &hf).unwrap();
        // the two `*` constants as the compiler folded them (the 0.2 is 1 - 0.8 in f32, not the literal 0.2)
        let cp = compile_for(&g, &binding(&[])).unwrap();
        let ks: Vec<f32> = cp
            .nodes
            .iter()
            .filter(|nd| nd.prim == "*" && !nd.b160 && !nd.folded)
            .map(|nd| nd.args.iter().find_map(|a| cp.nodes.iter().find(|m| m.idx == *a && m.b160).map(|m| f32::from_bits(m.lanes[0]))).unwrap())
            .collect();
        assert_eq!(ks.len(), 2);
        let (k1, k2) = (ks[0], ks[1]);
        let classes: Vec<&str> = o.trace.iter().map(|r| r.class.as_str()).collect();
        assert_eq!(classes, vec!["get_prev_dest", "*", "*", "+", "save_prev_dest"]);
        for c in 0..4 {
            for i in 0..n {
                let want = prev.planes[c][i] * k1 + src.planes[c][i] * k2;
                assert_eq!(o.dest.planes[c][i].to_bits(), want.to_bits(), "c{} i{}", c, i);
            }
        }
        assert_eq!(o.saved_prev_dest.as_ref().unwrap(), &o.dest);
        // first frame: no previous destination, get_prev_dest returns its argument (the Source in the default branch; INFERRED)
        let o0 = execute(&g, &binding(&[]), &inp, &HostFrames::default()).unwrap();
        for c in 0..4 {
            for i in 0..n {
                assert_eq!(o0.dest.planes[c][i].to_bits(), (src.planes[c][i] * k1 + src.planes[c][i] * k2).to_bits());
            }
        }
    });
}

#[test]
fn hue_sat_bright_single_parameters_by_hand() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data"]) {
        return;
    }
    big(|| {
        let g = genotype("Hue_Sat_Bright");
        let (w, h) = (7, 3);
        let n = w * h;
        let src = ramp(4, n, 0.125);
        let mut images = BTreeMap::new();
        images.insert("Source".to_string(), src.clone());
        let inp = Inputs { width: w, height: h, images };
        let hf = HostFrames::default();
        let run = |ov: &[(&str, Vec<f32>)]| execute(&g, &binding(ov), &inp, &hf).unwrap();
        // brightness 1.5 only: rgb * 1.5, alpha * 1
        let o = run(&[("brightness", vec![1.5])]);
        for i in 0..n {
            for c in 0..3 {
                assert_eq!(o.dest.planes[c][i].to_bits(), (src.planes[c][i] * 1.5f32).to_bits());
            }
            assert_eq!(o.dest.planes[3][i].to_bits(), (src.planes[3][i] * 1.0f32).to_bits());
        }
        // offset_darks 0.1 only: rgb + 0.1 (the plan subtracts a folded constant, see the dump compare), alpha unchanged
        let o = run(&[("offset_darks", vec![0.1])]);
        assert_eq!(o.trace.iter().map(|r| r.class.as_str()).collect::<Vec<_>>(), vec!["*", "+"]);
        // every single non-default set runs (no unsupported class on the Hue_Sat_Bright path)
        let sets: Vec<Vec<(&str, Vec<f32>)>> = vec![
            vec![("saturation", vec![1.0])],
            vec![("saturation", vec![0.0])],
            vec![("saturation", vec![2.0])],
            vec![("hue_shift", vec![0.25])],
            vec![("scale_colors", vec![2.0, 2.0, 2.0])],
            vec![("preserve_luma", vec![1.0]), ("hue_shift", vec![0.25])],
            vec![("preserve_luma", vec![0.0]), ("hue_shift", vec![0.25])],
        ];
        let mut rep = String::from("set\truns\n");
        for s in &sets {
            let o = run(s);
            let o2 = run(s);
            assert_eq!(o.dest, o2.dest);
            assert!(o.dest.planes.iter().all(|p| p.iter().all(|x| x.is_finite())), "{:?}", s);
            rep.push_str(&format!("{:?}\t{}\n", s, o.trace.iter().map(|r| r.class.clone()).collect::<Vec<_>>().join(" ")));
        }
        let _ = std::fs::write(effects_vecdir::out_dir().join("HSB_SETS_NV5.tsv"), rep);
    });
}

#[test]
fn alpha_pad_plane_is_the_source_alpha_for_hue_sat_ops() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data"]) {
        return;
    }
    big(|| {
        // NV5-A1 (PAD-COPY): the alpha plane the hue/sat bodies hand to the plane service is a copy of the source alpha
        let g = genotype("Hue_Sat_Bright");
        let (w, h) = (9, 2);
        let n = w * h;
        let src = ramp(4, n, 0.3);
        let mut images = BTreeMap::new();
        images.insert("Source".to_string(), src.clone());
        let inp = Inputs { width: w, height: h, images };
        for ov in [vec![("hue_shift", vec![0.25f32])], vec![("saturation", vec![0.5])], vec![("preserve_luma", vec![1.0]), ("hue_shift", vec![0.25])]] {
            let o = execute(&g, &binding(&ov), &inp, &HostFrames::default()).unwrap();
            assert_eq!(o.trace.len(), 1);
            assert_eq!(o.dest.planes[3], src.planes[3], "{:?}", ov);
            assert_ne!(o.dest.planes[0], src.planes[0], "{:?}: colour must change", ov);
        }
    });
}

fn read_f32s(p: &std::path::Path) -> Vec<f32> {
    let b = std::fs::read(p).unwrap();
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// Copies of the first original planes (audit_ep4 runA, runC: Composite, 64x32, A = Front 4 planes, B = Back 4 planes, OUT 4 planes).
/// Compares the native Composite byte for byte (bit patterns, NaN included) with the original's output planes.
#[test]
fn composite_against_the_original_planes_of_ep4() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data"]) {
        return;
    }
    big(|| {
        let g = genotype("Composite");
        let mut rep = String::from("run\tcase\tsamples\tbit_equal\tboth_nan_different_payload\tdifferent\tfirst_difference\n");
        let mut total_diff = 0usize;
        for run in ["ep4_runA", "ep4_runC"] {
            for case in 1..=2 {
                let p = effects_vecdir::path("fx_exec/data").join(run).join(format!("ORIG_PLANES_Composite_case{}_64x32.bin", case));
                let v = read_f32s(&p);
                let n = 64 * 32;
                assert_eq!(v.len(), 12 * n);
                let mk = |o: usize| Image::from_planes((0..4).map(|c| v[(o + c) * n..(o + c + 1) * n].to_vec()).collect()).unwrap();
                let (front, back, orig) = (mk(0), mk(4), mk(8));
                let mut images = BTreeMap::new();
                images.insert("Front".to_string(), front);
                images.insert("Back".to_string(), back);
                let o = execute(&g, &binding(&[]), &Inputs { width: 64, height: 32, images }, &HostFrames::default()).unwrap();
                let (mut eq, mut nan_pay, mut diff, mut first) = (0, 0, 0, String::new());
                for c in 0..4 {
                    for i in 0..n {
                        let (a, b) = (o.dest.planes[c][i], orig.planes[c][i]);
                        if a.to_bits() == b.to_bits() {
                            eq += 1;
                        } else if a.is_nan() && b.is_nan() {
                            nan_pay += 1;
                        } else {
                            diff += 1;
                            if first.is_empty() {
                                first = format!("c{} i{} native {:08x} original {:08x}", c, i, a.to_bits(), b.to_bits());
                            }
                        }
                    }
                }
                if run == "ep4_runC" {
                    total_diff += diff; // runA is an aborted run (meta: vex_steps 21, output plane never written): reported, not asserted
                }
                rep.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\n", run, case, 4 * n, eq, nan_pay, diff, first));
            }
        }
        std::fs::write(effects_vecdir::out_dir().join("EP4_COMPOSITE_COMPARE_NV5.tsv"), rep).unwrap();
        assert_eq!(total_diff, 0);
    });
}

#[test]
fn nv8_frame_rule_is_floor_of_max_zero_t() {
    use fx_exec::exec::frame_key;
    assert_eq!(frame_key(-1.0), 0);
    assert_eq!(frame_key(-0.0), 0);
    assert_eq!(frame_key(0.5), 0);
    assert_eq!(frame_key(0.75), 0);
    assert_eq!(frame_key(1.0), 1);
    assert_eq!(frame_key(1.25), 1);
    assert_eq!(frame_key(2.5), 2);
    // / M1 (MEASURED: the original instructions 0x60a4a2..0x60a4b8 run in Unicorn on 24 inputs, the native text equals them 48 of 48): NaN and values >= 2^31 give the integer indefinite 0x80000000; the old assertion (NaN reads frame 0) was an INFERRED rule
    assert_eq!(frame_key(f32::NAN), i32::MIN);
    assert_eq!(frame_key(2147483648.0), i32::MIN);
    assert_eq!(frame_key(3.0e9), i32::MIN);
    assert_eq!(frame_key(f32::INFINITY), i32::MIN);
    assert_eq!(frame_key(2147483520.0), 2147483520);
}
