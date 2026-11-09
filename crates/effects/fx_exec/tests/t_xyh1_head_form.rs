//! The head form of a default-xy writer follows its placement in the plan on every route: a writer allocated outside every exec block is a plane whose
//! rows take their own address phase (PerRow), a writer allocated in the line list of its block is one row buffer (OneHead). With `--xy-base B` and no
//! list the run renders as `--xy-base-list B,...,B`, perspective destinations included. The writer statics are process wide, so the tests that
//! compile in this process hold LOCK.
use fx_eval::{build_effect, Corpus};
use fx_exec::exec::{compile_for, Binding, PERSP_ROWBUF, XY_LIST, XY_WRITERS};
use fx_exec::kernels2::default_xy;
use fx_exec::types::{xy_ramp_mode, Image, XyHeadMode};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

fn text_prog(text: &str) -> fx_eval::Genotype {
    let mut c = Corpus::new();
    c.add_text("def_effect", "Probe", "0x0", text).expect("parse");
    let ei = c.find_effect("Probe", None).expect("find");
    build_effect(&c, ei).expect("build")
}

fn forms() -> Vec<XyHeadMode> {
    XY_WRITERS.lock().unwrap().iter().map(|w| w.2).collect()
}

/// The default-xy images the run builds for the writers of the placement model on `w` x `h` with the base `base`: (forms, images), one per writer.
fn writer_xy(g: &fx_eval::Genotype, w: usize, h: usize, base: i32) -> (Vec<XyHeadMode>, Vec<Image>) {
    let mut b = Binding::defaults();
    b.input_elnum.insert("Source".into(), 3);
    b.kp.xy_base = base;
    compile_for(g, &b).expect("compile");
    let ws = XY_WRITERS.lock().unwrap().clone();
    let imgs = ws
        .iter()
        .map(|(_, base, mode, _)| {
            let mut kp = b.kp;
            kp.xy_base = *base;
            kp.xy_head_mode = *mode;
            default_xy(&kp, w, h)
        })
        .collect();
    (ws.iter().map(|x| x.2).collect(), imgs)
}

#[test]
fn head_form_follows_the_placement_on_every_route() {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // a plane writer: `variable 2` is written in the first block and read by warp in the next one
    let plane = text_prog("(def_effect Probe (Source) (rgba_ok) () (warp (make_mipmap Source [2 2] 0) xy [2 2] 0))");
    // a block-local writer: the offset sum is formed in the block that writes xy
    let local = text_prog("(def_effect Probe (Source) (rgba_ok) () (warp (make_mipmap Source [2 2] 0) (+ xy [0.01 0.02]) [2 2] 0))");
    // base 16: head (32 - 16) / 4 = 4
    let head = 4usize;
    for (w, h) in [(50usize, 7usize), (64, 32)] {
        let per_row = xy_ramp_mode(w, h, head, XyHeadMode::PerRow);
        let one_row = xy_ramp_mode(w, h, head, XyHeadMode::OneHead);
        let (f, im) = writer_xy(&plane, w, h, 16);
        assert_eq!(f, vec![XyHeadMode::PerRow], "plane writer form at {}x{}", w, h);
        if w == 50 {
            assert_ne!(per_row.planes[0][w..2 * w], per_row.planes[0][..w], "row 1 of the per-row plane differs from row 0 at 50x7");
            assert_eq!(im[0].planes, per_row.planes, "plane writer at 50x7: per-row heads");
            assert_ne!(im[0].planes, one_row.planes);
        } else {
            assert_eq!(per_row.planes, one_row.planes, "64x32: the two forms are the same plane");
            assert_eq!(im[0].planes, one_row.planes, "plane writer at 64x32: byte identical to the one-row form");
        }
        // a block-local writer keeps the one-row form
        let (f, im) = writer_xy(&local, w, h, 16);
        assert_eq!(f, vec![XyHeadMode::OneHead], "block-local writer form at {}x{}", w, h);
        assert_eq!(im[0].planes, one_row.planes, "block-local writer at {}x{}: one-row form", w, h);
    }
    // the base route is the list route with a uniform list: the same writers
    let base_ws = {
        let _ = writer_xy(&plane, 50, 7, 16);
        XY_WRITERS.lock().unwrap().clone()
    };
    *XY_LIST.lock().unwrap() = Some((vec![(16, false)], 1));
    let _ = writer_xy(&plane, 50, 7, 16);
    let list_ws = XY_WRITERS.lock().unwrap().clone();
    *XY_LIST.lock().unwrap() = None;
    assert_eq!(base_ws, list_ws, "--xy-base 16 gives the writers of --xy-base-list 16");
    // a plan with no default-xy writer is unchanged by a base
    let nowriter = text_prog("(def_effect Probe (Source) (rgba_ok) () (make_mipmap Source [2 2] 0))");
    let _ = writer_xy(&nowriter, 50, 7, 16);
    assert!(forms().is_empty() && PERSP_ROWBUF.lock().unwrap().is_empty());
}

const W: usize = 50;
const H: usize = 7;

fn floats(n: usize, seed: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for k in 0..n {
        let x = 0.05 + ((k * 7 + seed * 13) % 31) as f32 / 40.0;
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}

/// Warp_Perspective_Autogen with Swing and Roll set: the plan allocates its default-xy writer inside the first exec block (a rotate in the plan),
/// and its perspective destination is a row buffer. Returns (rc, ae32 words, meta text).
fn warp_run(tag: &str, w: usize, h: usize, xy: &[&str]) -> (Option<i32>, Vec<u8>, String) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("xyh1_{}_{}_{}x{}", std::process::id(), tag, w, h));
    std::fs::create_dir_all(&d).unwrap();
    let src = d.join("src.bin");
    std::fs::write(&src, floats(4 * w * h, 3)).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0xaau8; 16 * w * h]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let geom = format!("-0.5,{},1,0,{},0,0", w as f32 - 0.5, (h as f32 - w as f32) / 2.0 - 0.5);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args(["Warp_Perspective_Autogen", "--size", &format!("{}x{}", w, h), "--out", &os]);
    cmd.args(["--ae32-src", &format!("Source={}", src.display()), "--src-planes", "4", "--owner-arm", "0,0,0", "--owner-opacity", "1.0", "--src-row58", "0"]);
    cmd.args(["--src-head", "0x7eb3d0", "--aspect-triple", "0,3f800000,3f800000", "--geom", &geom, "--ae32-dest", "4,1", "--ae32-prior", prior.to_str().unwrap()]);
    cmd.args(["--ae32-copier", "none", "--dest-stack-word", "0", "--ae32-out", &format!("{}.ae32.u32", os), "--units-ratio", "1,1", "--do-postdiv", "1"]);
    cmd.args(["--bound-mask", "1", "--host-frame", "0", "--ctx-time", "0", "--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "f8", "--ranges", "REAL"]);
    cmd.args(["--src-chan", "4", "--host-param", "blur_mask=0", "--host-param", "blur_mocha_mask=0", "--set", "shift_mocha_mask=0,-0"]);
    cmd.args(["--set", "swing=40", "--set", "roll=-10", "--persp-base", "16", "--palpha-base", "16", "--palpha-cells", &format!("{},1,1.0", w)]);
    cmd.args(xy);
    let p = cmd.output().expect("fx_run");
    let words = std::fs::read(format!("{}.ae32.u32", os)).unwrap_or_default();
    let meta = std::fs::read_to_string(format!("{}.dest.meta.txt", os)).unwrap_or_default();
    if p.status.code() != Some(0) {
        eprintln!("{}", String::from_utf8_lossy(&p.stderr));
    }
    (p.status.code(), words, meta)
}

fn label(meta: &str) -> String {
    meta.lines().find(|l| l.starts_with("label XY-HEAD-MODE")).unwrap_or("").to_string()
}

#[test]
fn a_perspective_plan_with_the_writer_in_the_first_block_renders_one_row_on_the_base_route() {
    for (w, h) in [(W, H), (64usize, 32usize)] {
        let (rc_b, base, meta_b) = warp_run("base", w, h, &["--xy-base", "16"]);
        let (rc_l, list, _) = warp_run("list", w, h, &["--xy-base-list", "16"]);
        assert_eq!((rc_b, rc_l), (Some(0), Some(0)));
        assert!(!base.is_empty());
        assert_eq!(base, list, "{}x{}: the base route equals the one-entry list", w, h);
        assert!(label(&meta_b).contains(": OneHead;"), "{}", label(&meta_b));
        assert!(label(&meta_b).contains("derived from the placement of the default-xy writers in the plan"), "{}", label(&meta_b));
    }
}

#[test]
fn head_form_scope_follows_the_placement_of_the_native_programs() {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // Warp_Perspective_Autogen at its defaults: the writer is a plane (PerRow, as the primitive set's rule also said); Transform_and_Blur_Autogen at its
    // defaults has no default-xy writer; Blur_Motion_Curves_No_Transform_Autogen: a row buffer
    let c = Corpus::native();
    for (name, want) in [("Warp_Perspective_Autogen", vec![XyHeadMode::PerRow]), ("Transform_and_Blur_Autogen", vec![]), ("Blur_Motion_Curves_No_Transform_Autogen", vec![XyHeadMode::OneHead])] {
        let g = build_effect(&c, c.find_effect(name, None).unwrap()).unwrap();
        let mut b = Binding::defaults();
        b.input_elnum.insert("Source".into(), 4);
        b.kp.xy_base = 16;
        b.hc_zero_legacy = true;
        for (k, v) in [("blur_mask", vec![0.0]), ("blur_mocha_mask", vec![0.0]), ("shift_mocha_mask", vec![0.0, -0.0])] {
            b.overrides.insert(k.to_string(), v);
        }
        let a: Vec<String> = ["--ctx-time", "0", "--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "f8"].iter().map(|x| x.to_string()).collect();
        b.ctx = Some(fx_eval::CompileContext::from_args(&a).unwrap());
        compile_for(&g, &b).expect("compile");
        assert_eq!(forms(), want, "{}", name);
    }
}
