//! Self-check of the glint device op against the CPU kernel, bit for bit: the full corpus, the driver proof with a stub `run` (the CPU
//! result copied through the device passes, a stub that zeroes one plane is refused), the calls the op declines, the NaN check of the
//! CPU output, and single-edit mutants of the blend kernel source, each of which the self-check must refuse. Prints NOT RUN without an
//! OpenCL device (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DevImage, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend, NodeCall};
use fx_exec::device_ops::{ArgShape, DevArg, DeviceOp};
use fx_exec::device_ops_glint::{glint_corpus, ALLOW, GLINT};
use fx_exec::types::{Arg, Image, KParams};
use std::sync::Arc;

fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}

fn device() -> Option<Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            None
        }
    }
}

#[test]
fn the_glint_op_passes_the_self_check_against_the_cpu() {
    let Some(ctx) = device() else { return };
    let corpus = glint_corpus();
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &GLINT).expect("device equals the CPU");
    println!("GLINT cases {} outputs {} payloads {}", corpus.len(), tally.outputs, tally);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "NaN payloads differ: {}", tally);
    let b = DeviceBackend::with_allowlist(ctx, ALLOW);
    let rep = b.report();
    assert!(rep.refused.is_empty(), "{:?}", rep.refused);
    assert_eq!(b.admitted(), vec!["glint"]);
}

/// The CPU kernel of a corpus case.
fn cpu_of(case: &fx_exec::device_ops::SelfCase) -> Result<Image, String> {
    let kp = case.kp.unwrap_or_else(KParams::standin);
    let n = case.dims.0 * case.dims.1;
    let mut imgs = case.args.iter();
    let total = case.args.len() + case.consts.len();
    let mut args: Vec<Arg> = Vec::new();
    for pos in 0..total {
        match case.consts.iter().find(|c| c.0 == pos) {
            Some(c) => args.push(Arg::Const { n: c.1, lanes: c.2 }),
            None => args.push(Arg::Img(imgs.next().unwrap())),
        }
    }
    match fx_exec::kernels::run_any("glint", case.tag, &args, n, case.dims, &kp) {
        Ok(fx_exec::kernels2::Out::Img(i)) => Ok(i),
        Ok(_) => Err("ripmap".into()),
        Err(e) => Err(format!("{e}")),
    }
}

#[test]
fn the_cpu_output_of_every_case_holds_no_nan() {
    let corpus = glint_corpus();
    let (mut planes, mut with_nan_inputs) = (0usize, 0usize);
    for case in &corpus {
        if case.args.iter().any(|a| a.planes.iter().any(|p| p.iter().any(|v| v.is_nan()))) {
            with_nan_inputs += 1;
        }
        let out = cpu_of(case).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        for (k, p) in out.planes.iter().enumerate() {
            planes += 1;
            assert!(p.iter().all(|v| !v.is_nan()), "{} plane {k}: the CPU output holds a NaN", case.name);
        }
    }
    println!("CPU NaN check: {} cases, {} with NaN in the source, {} output planes, 0 NaN words", corpus.len(), with_nan_inputs, planes);
    assert!(with_nan_inputs > 0);
}

/// A stub `run` that returns the CPU result copied through the device (the driver path is exercised end to end).
fn stub_cpu_run(s: &Arc<fx_device::Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, fx_device::DevError> {
    let mut hold: Vec<Image> = Vec::new();
    for x in a {
        if let DevArg::Img(d) = x {
            hold.push(Image { n: d.n, planes: d.download()? });
        }
    }
    let mut it = hold.iter();
    let args: Vec<Arg> = a.iter().map(|x| match x {
        DevArg::Img(_) => Arg::Img(it.next().unwrap()),
        DevArg::Const { n, lanes } => Arg::Const { n: *n, lanes: *lanes },
        _ => unreachable!(),
    }).collect();
    match fx_exec::kernels::run_any("glint", c.tag, &args, c.n, c.dims, c.kp) {
        Ok(fx_exec::kernels2::Out::Img(i)) => DevImage::upload(s, &i.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()),
        _ => Err(fx_device::DevError::Unsupported("cpu".into())),
    }
}

/// A stub that zeroes the middle output plane.
fn stub_zero_plane(s: &Arc<fx_device::Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, fx_device::DevError> {
    let d = stub_cpu_run(s, c, a)?;
    let mut planes = d.download()?;
    for v in planes[1].iter_mut() {
        *v = 0.0;
    }
    DevImage::upload(s, &planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>())
}

#[test]
fn the_driver_accepts_a_cpu_copy_stub_and_refuses_a_zeroing_stub() {
    let Some(ctx) = device() else { return };
    let mut op = GLINT;
    op.run = stub_cpu_run;
    let t = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect("the CPU result copied through the device passes");
    println!("HARNESS pass stub: outputs {}", t.outputs);
    op.run = stub_zero_plane;
    let e = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect_err("a zeroed plane must be refused");
    println!("HARNESS zero stub refused: {}", e.lines().next().unwrap_or(""));
}

fn consts(v: &[&[f32]]) -> Vec<(usize, [u32; 4])> {
    v.iter().map(|l| {
        let last = *l.last().unwrap();
        (l.len(), core::array::from_fn(|i| l.get(i).copied().unwrap_or(last).to_bits()))
    }).collect()
}

fn base_consts() -> Vec<(usize, [u32; 4])> {
    consts(&[&[0.5], &[1.0], &[1.0], &[1.0], &[1.0, 1.0, 1.0, 1.0], &[0.25]])
}

#[test]
fn declines_and_refusals() {
    let Some(ctx) = device() else { return };
    let (w, h) = (16usize, 16usize);
    let n = w * h;
    let src = Image { n, planes: (0..3).map(|p| (0..n).map(|i| ((i * 7 + p * 3) % 17) as f32 / 16.0).collect()).collect() };
    let up = DevImage::upload(&ctx.session, &src.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()).unwrap();
    let kp = KParams::standin();
    fn call_of(tag: i32, n: usize, dims: (usize, usize), kp: &KParams) -> NodeCall<'_> {
        NodeCall { idx: 0, class: "glint", tag, n, dims, kp }
    }
    let shapes = |cs: &[(usize, [u32; 4])], first: ArgShape| -> Vec<ArgShape> {
        let mut v = vec![first];
        v.extend(cs.iter().map(|c| ArgShape::Const { n: c.0 }));
        v
    };
    let img3 = ArgShape::Img { channels: 3 };
    let ok = base_consts();
    assert!((GLINT.accepts)(&call_of(3, n, (w, h), &kp), &shapes(&ok, img3)).is_ok());
    // tag 1 (LAYOUT)
    assert!((GLINT.accepts)(&call_of(1, n, (w, h), &kp), &shapes(&ok, img3)).is_err());
    // 6 operands
    assert!((GLINT.accepts)(&call_of(3, n, (w, h), &kp), &shapes(&ok[..5], img3)).is_err());
    // a one lane constant source
    assert!((GLINT.accepts)(&call_of(3, n, (w, h), &kp), &shapes(&ok, ArgShape::Const { n: 1 })).is_err());
    // two planes
    assert!((GLINT.accepts)(&call_of(3, n, (w, h), &kp), &shapes(&ok, ArgShape::Img { channels: 2 })).is_err());
    // a non constant threshold operand
    let mut sh = shapes(&ok, img3);
    sh[1] = ArgShape::Img { channels: 3 };
    assert!((GLINT.accepts)(&call_of(3, n, (w, h), &kp), &sh).is_err());
    // a nonzero blur phase
    let mut kp_blur = KParams::standin();
    kp_blur.glint_align.blur = 4;
    assert!((GLINT.accepts)(&call_of(3, n, (w, h), &kp_blur), &shapes(&ok, img3)).is_err());
    // a frame over i32::MAX / 4 samples (shape only)
    let big = NodeCall { idx: 0, class: "glint", tag: 3, n: (i32::MAX as usize) / 4 + 1, dims: ((i32::MAX as usize) / 4 + 1, 1), kp: &kp };
    assert!((GLINT.accepts)(&big, &shapes(&ok, img3)).is_err());
    println!("ACCEPTS refusals: 8 of 8");

    let run_with = |cs: &[(usize, [u32; 4])], kp: &KParams| -> Result<DevImage, fx_device::DevError> {
        let mut dev: Vec<DevArg> = vec![DevArg::Img(&up)];
        dev.extend(cs.iter().map(|&(n, lanes)| DevArg::Const { n, lanes }));
        (GLINT.run)(&ctx.session, &call_of(3, n, (w, h), kp), &dev)
    };
    let cpu_with = |cs: &[(usize, [u32; 4])]| {
        let mut args: Vec<Arg> = vec![Arg::Img(&src)];
        args.extend(cs.iter().map(|&(n, lanes)| Arg::Const { n, lanes }));
        fx_exec::kernels::run_any("glint", 3, &args, n, (w, h), &kp)
    };
    // an infinite T0 lane with its pass active: the CPU names GLINT-SHIFT-REFUSED, the device declines with the same name
    let inf = consts(&[&[0.5], &[1.0], &[1.0], &[1.0], &[f32::INFINITY, 1.0, 1.0, 1.0], &[0.25]]);
    let cpu_err = cpu_with(&inf).err().expect("the CPU refuses an infinite pass scale").to_string();
    assert!(cpu_err.contains("GLINT-SHIFT-REFUSED"), "{cpu_err}");
    match run_with(&inf, &kp) {
        Err(fx_device::DevError::Unsupported(why)) => assert!(why.contains("GLINT-SHIFT-REFUSED"), "{why}"),
        other => panic!("expected the shift decline, got {other:?}"),
    }
    // a NaN strength: the CPU renders, the blur kernel declines
    let nan = consts(&[&[0.5], &[f32::NAN], &[1.0], &[1.0], &[1.0, 1.0, 1.0, 1.0], &[0.25]]);
    assert!(cpu_with(&nan).is_ok());
    assert!(matches!(run_with(&nan, &kp), Err(fx_device::DevError::Unsupported(_))));
    // a blur phase through run (accepts is not consulted)
    assert!(matches!(run_with(&ok, &kp_blur), Err(fx_device::DevError::Unsupported(_))));
    println!("RUN declines: 3 of 3 (shift refusal by name, NaN strength, blur phase)");
}

/// Single-edit mutants of the blend kernel (name, text in the program, replacement). Each text occurs once in the program. Three
/// edits are not here because they cannot change a result: `1 > t` to `1 >= t`, `1 > s` to `1 >= s` (omt = 0 gives fma(0, d, 1) = 1 for a finite d)
/// and the two `max` selects of the plain route (f >= max(d, s) for d in [0, 1], and no output holds a NaN or leaves [0, 1]).
const MUTANTS: &[(&str, &str, &str)] = &[
    ("NaN form of the s test", "float s = src[i];\n    if (!(s > 0x0p+0f)) {", "float s = src[i];\n    if (s < 0x0p+0f) {"),
    ("fused multiply-add replaced by unfused", "dst[i] = nf_fma213(omt, d, t);", "dst[i] = nf_add(nf_mul(omt, d), t);"),
    ("subtraction operand swap on the plain route", "float omt = nf_sub(0x1p+0f, s);", "float omt = nf_sub(s, 0x1p+0f);"),
    ("fused operand order on the plain route", "float f = nf_fma213(omt, d, s);", "float f = nf_fma213(omt, s, d);"),
    ("route flag inverted", "if (one_route != 0) {", "if (one_route == 0) {"),
    ("clip constant of the scaled route", "} else {\n            dst[i] = 0x1p+0f;\n        }\n    }\n}", "} else {\n            dst[i] = 0x0p+0f;\n        }\n    }\n}"),
    ("scale applied to the wrong operand", "float t = nf_mul(scale, s);", "float t = nf_mul(scale, d);"),
];

#[test]
fn every_blend_kernel_mutant_is_refused_by_the_self_check() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let binary_hash = |s: &fx_device::Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(fx_device::Session::open_source(&dev, &info, &base, opts).unwrap());
    let base_hash = binary_hash(&clean);
    let op: DeviceOp = GLINT;
    let t = fx_exec::device_ops::selfcheck(&clean, &op).expect("the unmutated program passes");
    assert!(matches!(t.nan_payloads, NanPayloads::Same));
    println!("MUTANT base binary {base_hash:016x}");
    let mut prev = base_hash;
    let mut killed = 0usize;
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(fx_device::Session::open_source(&dev, &info, &text, opts).unwrap());
        let h = binary_hash(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        assert_ne!(h, prev, "mutant `{name}` built the previous mutant's binary");
        prev = h;
        let verdict = match fx_exec::device_ops::selfcheck(&s, &op) {
            Err(m) => format!("KILLED (mismatch: {})", m.lines().next().unwrap_or("")),
            Ok(_) => "SURVIVED".to_string(),
        };
        println!("MUTANT {name}: {verdict} binary {h:016x}");
        if verdict.starts_with("KILLED") {
            killed += 1;
        }
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a mutant survived");
}

/// Runs `glint_blend_f32` directly with the scale-1 route on a source of exactly 1.0 and the given destinations; returns the words.
fn blend_s1(s: &Arc<fx_device::Session>, d: &[f32]) -> Vec<u32> {
    use fx_device::cl::Arg as ClArg;
    let n = d.len();
    let dst = DevImage::upload(s, &[d]).unwrap();
    let src = DevImage::upload(s, &[&vec![1.0f32; n]]).unwrap();
    s.launch("glint_blend_f32", n, &[dst.plane(0).arg(), src.plane(0).arg(), ClArg::U32(n as u32), ClArg::F32(1.0), ClArg::I32(1)]).unwrap();
    dst.download().unwrap().remove(0).iter().map(|v| v.to_bits()).collect()
}

/// A source of exactly 1.0 over a destination of 1.5, infinity, NaN or an ordinary value gives exactly 1.0 (the clip of the scale-1 route).
fn blend_boundary_mismatches(s: &Arc<fx_device::Session>) -> usize {
    let d = [1.5f32, f32::INFINITY, f32::NAN, f32::from_bits(0xffc0_0001), 3.0e38, 0.25, 1.0, 0.0];
    blend_s1(s, &d).iter().filter(|&&w| w != 1.0f32.to_bits()).count()
}

#[test]
fn the_blend_clips_a_source_of_one_over_a_destination_above_one() {
    let Some(ctx) = device() else { return };
    assert_eq!(blend_boundary_mismatches(&ctx.session), 0);
}

#[test]
fn the_blend_boundary_mutant_is_refused_by_the_direct_case() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let (old, new) = ("if (0x1p+0f > s) {", "if (0x1p+0f >= s) {");
    assert_eq!(base.matches(old).count(), 1, "the text must occur once");
    let s = Arc::new(fx_device::Session::open_source(&dev, &info, &base.replacen(old, new, 1), opts).unwrap());
    let bad = blend_boundary_mismatches(&s);
    println!("BLEND boundary mutant: {bad} of 8 words differ");
    assert!(bad > 0, "the boundary mutant survived the direct case");
}
