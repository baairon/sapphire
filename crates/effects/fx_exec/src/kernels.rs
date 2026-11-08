//! The run-primitive call frame into the native builtin crates: one wrapper per class. Each wrapper receives the
//! combo tag the native selector chose (the value slot 31 returns, which is also the output channel count and the
//! plane-count getter [vtable] of the hue/sat bodies), the operands as `Arg`and returns the output planes.
//! Every other class returns `ExecError::Unsupported` naming the class.

use crate::types::{Arg, ExecError, Image, KParams};
use plane_service as ps;

/// Classes with a kernel wrapper here (names as in the plan's run-primitive).
pub const SUPPORTED: &[&str] = &[
    "hue_shift", "hue_shift_preserve_luma", "sat_scale", "v3_to_v4_if4", "+", "-", "*", "/", "max", "min", "abs", "negate", "if+", "if>", "one_minus", "comp_rgba",
    "screen_super", "element_of", "v2", "v3", "v4", "diff", "clamp", "interp",
    "overlay_super", "screen",
];

/// Classes the executor itself serves from HOST-FRAMES (no builtin crate involved).
pub const HOST_CLASSES: [&str; 3] = ["get_prev_dest", "save_prev_dest", "get_frame"];

pub fn normalize_class(class: &str) -> &str {
    // NV3-A6: the plan prints `screen_super_if_needed` as `screen_super`
    match class {
        "screen_super_if_needed" => "screen_super",
        "overlay_super_if_needed" => "overlay_super",
        c => c,
    }
}

pub fn is_supported(class: &str) -> bool {
    let c = normalize_class(class);
    SUPPORTED.contains(&c) || HOST_CLASSES.contains(&c) || crate::kernels2::WIRED.contains(&c)
}

pub(crate) fn kerr(class: &str, e: impl std::fmt::Debug) -> ExecError {
    ExecError::Kernel { class: class.to_string(), msg: format!("{:?}", e) }
}

pub(crate) fn unsupported(class: &str, reason: &str) -> ExecError {
    ExecError::Unsupported { class: class.to_string(), reason: reason.to_string() }
}

/// Text of the FRAME-SIZE-OVER-I32 refusal: `what` of `class` is `value`, and the kernel counts in 32 bits.
pub fn frame_size_label(class: &str, what: &str, value: usize) -> String {
    format!("FRAME-SIZE-OVER-I32 ({}: {} is {}, the kernel counts in 32 bits and takes at most {})", class, what, value, i32::MAX)
}

/// `value` as the i32 the kernel crates take. A value above i32::MAX is refused by name (FRAME-SIZE-OVER-I32), not wrapped.
pub(crate) fn count_i32(class: &str, what: &str, value: usize) -> Result<i32, ExecError> {
    i32::try_from(value).map_err(|_| ExecError::Shape(frame_size_label(class, what, value)))
}

/// The block the class virtual [vtable] returns for an operand, as modelled (NV5-A1, PAD-COPY): an image
/// operand is copied plane by plane, a constant operand fills its lane. CALLEE-MODELLED in plane_service.
pub(crate) fn block_of<'a>(a: &Arg<'a>, planes: &'a [&'a [f32]], vals: &'a [u32]) -> ps::Block<'a> {
    ps::Block { is_int: false, values: vals, enable: a.is_const(), maxch: a.count(), img_valid: true, planes }
}

pub(crate) fn flat_roi(n: usize) -> ps::Roi {
    ps::Roi { mode: 0, n: n as i32, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 }
}

pub(crate) fn pad_fill(kp: &KParams, src: &Arg, chan: i32, dst: &mut [f32], n: usize) -> Result<(), ExecError> {
    let planes = src.planes();
    let vals: Vec<u32> = src.bits().to_vec();
    let blk = block_of(src, &planes, &vals);
    let host = ps::Host { in_pool: kp.in_pool };
    ps::plane_service(&host, ps::Sel::Virtual { block: blk }, 0, chan, dst, &flat_roi(n)).map_err(|e| kerr("plane_service", e))?;
    Ok(())
}

fn finish(tag: i32, o: [Vec<f32>; 4]) -> Image {
    let t = (tag.max(0) as usize).min(4);
    let n = o[0].len();
    let planes: Vec<Vec<f32>> = o.into_iter().take(t).collect();
    Image { n, planes }
}

fn out4(n: usize) -> [Vec<f32>; 4] {
    [vec![0.0; n], vec![0.0; n], vec![0.0; n], vec![0.0; n]]
}

fn src3<'a>(a: &Arg<'a>, planes: &'a [&'a [f32]]) -> Option<[&'a [f32]; 3]> {
    if a.is_const() || planes.len() < 3 {
        None
    } else {
        Some([planes[0], planes[1], planes[2]])
    }
}

fn colour_of(a: &Arg) -> [f32; 3] {
    if a.is_const() {
        let f = a.f32s();
        [f[0], f[1], f[2]]
    } else {
        [0.0; 3]
    }
}

pub(crate) fn need(args: &[Arg], k: usize, class: &str) -> Result<(), ExecError> {
    if args.len() != k {
        return Err(ExecError::BadPlan(format!("{} takes {} arguments, plan has {}", class, k, args.len())));
    }
    Ok(())
}

fn hue_shift_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "hue_shift")?;
    let (s, a) = (&args[0], &args[1]);
    let sp = s.planes();
    let ap = a.planes();
    let p = hue_shift::Params { const_colour: s.is_const(), scalar_shift: a.is_const(), colour: colour_of(s), shift: a.f32s()[0], flag: false };
    let inp = hue_shift::Inputs { src: src3(s, &sp), mask: if a.is_const() { None } else { ap.first().copied() } };
    let mut o = out4(n);
    let calls = {
        let [o0, o1, o2, _] = &mut o;
        let mut outs: [&mut [f32]; 3] = [o0.as_mut_slice(), o1.as_mut_slice(), o2.as_mut_slice()];
        hue_shift::run(&p, tag, n as i32, &inp, &mut outs).map_err(|e| kerr("hue_shift", e))?
    };
    for c in calls {
        match c {
            hue_shift::Call::Fill { index, plane } => pad_fill(kp, s, index, &mut o[plane.min(3)], n)?,
            hue_shift::Call::Generic => return Err(unsupported("hue_shift", "generic flag path ")),
        }
    }
    Ok(finish(tag, o))
}

fn sat_scale_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "sat_scale")?;
    let (s, a) = (&args[0], &args[1]);
    let sp = s.planes();
    let ap = a.planes();
    let p = sat_scale::Params { const_colour: s.is_const(), scalar_scale: a.is_const(), colour: colour_of(s), scale: a.f32s()[0], flag: false };
    let inp = sat_scale::Inputs { src: src3(s, &sp), mask: if a.is_const() { None } else { ap.first().copied() } };
    let mut o = out4(n);
    let calls = {
        let [o0, o1, o2, _] = &mut o;
        let mut outs: [&mut [f32]; 3] = [o0.as_mut_slice(), o1.as_mut_slice(), o2.as_mut_slice()];
        sat_scale::run(&p, tag, n as i32, &inp, &mut outs).map_err(|e| kerr("sat_scale", e))?
    };
    for c in calls {
        match c {
            sat_scale::Call::Fill { index, plane } => pad_fill(kp, s, index, &mut o[plane.min(3)], n)?,
            sat_scale::Call::Generic => return Err(unsupported("sat_scale", "generic flag path ")),
        }
    }
    Ok(finish(tag, o))
}

fn hsp_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "hue_shift_preserve_luma")?;
    let (s, a) = (&args[0], &args[1]);
    let sp = s.planes();
    let ap = a.planes();
    let p = hue_shift_preserve_luma::Params {
        plane_count: tag,
        flag: false,
        const_colour: s.is_const(),
        colour: colour_of(s),
        scalar_shift: a.is_const(),
        shift: a.f32s()[0],
        n: n as i32,
    };
    let mask = if a.is_const() { None } else { ap.first().copied() };
    let mut outs3: [Vec<f32>; 3] = [vec![0.0; n], vec![0.0; n], vec![0.0; n]];
    let calls = hue_shift_preserve_luma::render(&p, src3(s, &sp), mask, &mut outs3).map_err(|e| kerr("hue_shift_preserve_luma", e))?;
    let [a0, a1, a2] = outs3;
    let mut o = [a0, a1, a2, vec![0.0; n]];
    for c in calls {
        match c {
            hue_shift_preserve_luma::Call::Fill(plane) => pad_fill(kp, s, plane, &mut o[(plane.max(0) as usize).min(3)], n)?,
            hue_shift_preserve_luma::Call::Generic => return Err(unsupported("hue_shift_preserve_luma", "generic flag path ")),
        }
    }
    Ok(finish(tag, o))
}

fn v3_to_v4_if4_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 3, "v3_to_v4_if4")?;
    let counts = [args[0].count(), args[1].count(), args[2].count()];
    let combo = v3_to_v4_if4::select_combo(counts).ok_or(ExecError::NoCombo { class: "v3_to_v4_if4".into(), operands: counts.to_vec() })?;
    let (p0, p1) = (args[0].planes(), args[1].planes());
    let (v0, v1) = (args[0].bits().to_vec(), args[1].bits().to_vec());
    let blocks = [block_of(&args[0], &p0, &v0), block_of(&args[1], &p1, &v1)];
    let p = v3_to_v4_if4::Params { combo, host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        v3_to_v4_if4::render(&p, &blocks, &mut outs).map_err(|e| kerr("v3_to_v4_if4", e))?;
    }
    Ok(Image { n, planes: outv })
}

fn arith_operand<'a>(a: &Arg<'a>, planes: &'a [&'a [f32]]) -> ops_arith::Operand<'a> {
    ops_arith::Operand { count: a.count(), is_const: a.is_const(), consts: a.f32s(), planes }
}

fn binary_k(class: &str, args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, class)?;
    let (pa, pb) = (args[0].planes(), args[1].planes());
    let (oa, ob) = (arith_operand(&args[0], &pa), arith_operand(&args[1], &pb));
    let p = ops_arith::Params { nout: tag, n: n as i32, vec_one: kp.vec_one, dst3_phase_bytes: kp.dst3_phase_bytes };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        let r = match class {
            "+" => ops_arith::add::render(&p, &oa, &ob, &mut outs),
            "-" => ops_arith::sub::render(&p, &oa, &ob, &mut outs),
            "*" => ops_arith::mul::render(&p, &oa, &ob, &mut outs),
            "/" => ops_arith::div::render(&p, &oa, &ob, &mut outs),
            "diff" => {
                let dp = ops_diff::Params { nout: tag, n: n as i32, vec_one: kp.vec_one, vec_mask: DIFF_VEC_MASK, dst3_phase_bytes: kp.dst3_phase_bytes };
                ops_diff::render(&dp, &oa, &ob, &mut outs)
            }
            "max" => ops_minmax::max::render(&p, &oa, &ob, &mut outs),
            _ => ops_minmax::min::render(&p, &oa, &ob, &mut outs),
        };
        r.map_err(|e| kerr(class, e))?;
    }
    Ok(Image { n, planes: outv })
}

fn screen_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "screen_super")?;
    let (pa, pb) = (args[0].planes(), args[1].planes());
    let oa = screen_super::Operand { count: args[0].count(), is_const: args[0].is_const(), consts: args[0].f32s(), planes: &pa };
    let ob = screen_super::Operand { count: args[1].count(), is_const: args[1].is_const(), consts: args[1].f32s(), planes: &pb };
    let p = screen_super::Params { nout: tag, n: n as i32, vec_one: kp.vec_one, dst3_phase_bytes: kp.dst3_phase_bytes };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        screen_super::render(&p, &oa, &ob, &mut outs).map_err(|e| kerr("screen_super", e))?;
    }
    Ok(Image { n, planes: outv })
}

fn overlay_k(args: &[Arg], tag: i32, n: usize, _kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "overlay_super")?;
    let n32 = count_i32("overlay_super", "the plane length", n)?;
    let (pa, pb) = (args[0].planes(), args[1].planes());
    let oa = overlay_super::Operand { count: args[0].count(), is_const: args[0].is_const(), consts: args[0].f32s(), planes: &pa };
    let ob = overlay_super::Operand { count: args[1].count(), is_const: args[1].is_const(), consts: args[1].f32s(), planes: &pb };
    let p = overlay_super::Params { nout: tag, n: n32 };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        overlay_super::render(&p, &oa, &ob, &mut outs).map_err(|e| kerr("overlay_super", e))?;
    }
    Ok(Image { n, planes: outv })
}

fn screen2_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "screen")?;
    let n32 = count_i32("screen", "the plane length", n)?;
    let (pa, pb) = (args[0].planes(), args[1].planes());
    let oa = screen::Operand { count: args[0].count(), is_const: args[0].is_const(), consts: args[0].f32s(), planes: &pa };
    let ob = screen::Operand { count: args[1].count(), is_const: args[1].is_const(), consts: args[1].f32s(), planes: &pb };
    let p = screen::Params { nout: tag, n: n32, vec_one: kp.vec_one, dst3_phase_bytes: kp.dst3_phase_bytes };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        screen::render(&p, &oa, &ob, &mut outs).map_err(|e| kerr("screen", e))?;
    }
    Ok(Image { n, planes: outv })
}

fn unary_k(class: &str, args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 1, class)?;
    if args[0].is_const() {
        return Err(unsupported(class, "constant operand at run time (the compiler folds it)"));
    }
    let planes = args[0].planes();
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        match class {
            "abs" | "negate" => {
                let mask = if class == "abs" { ops_minmax::common::ABS_SCALAR_MASK } else { ops_minmax::common::NEG_SCALAR_MASK };
                let up = ops_minmax::UParams { n: n as i32, vec_mask: mask, dst_phase_bytes: [0; 4] };
                let r = if class == "abs" { ops_minmax::abs::render(&up, tag, &planes, &mut outs) } else { ops_minmax::negate::render(&up, tag, &planes, &mut outs) };
                r.map_err(|e| kerr(class, e))?;
            }
            _ => {
                let p = one_minus::Params { count: tag, n: n as i32, vec_one: kp.vec_one, dst_phase_bytes: Vec::new() };
                one_minus::render(&p, &planes, &mut outs).map_err(|e| kerr(class, e))?;
            }
        }
    }
    Ok(Image { n, planes: outv })
}

fn ifop_operand(a: &Arg) -> ops_if::Operand {
    ops_if::Operand {
        count: a.count(),
        is_const: a.is_const(),
        consts: a.bits(),
        planes: if a.is_const() { None } else { Some(a.planes().iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect()) },
    }
}

fn if_k(class: &str, args: &[Arg], tag: i32, n: usize) -> Result<Image, ExecError> {
    let (kind, k) = if class == "if+" { (ops_if::Kind::Plus, 3) } else { (ops_if::Kind::Gt, 4) };
    need(args, k, class)?;
    let ops: Vec<ops_if::Operand> = args.iter().map(ifop_operand).collect();
    let p = ops_if::Params { nout: tag, n: n as i32, one: 1.0f32.to_bits(), k_mask: 0x8000_0000, dst_phase: 0, dst_init: 0 };
    let mut out: Vec<Vec<u32>> = vec![vec![0u32; n]; tag.max(0) as usize];
    ops_if::render(kind, &p, &ops, &mut out).map_err(|e| kerr(class, e))?;
    Ok(Image { n, planes: out.into_iter().map(|v| v.into_iter().map(f32::from_bits).collect()).collect() })
}

fn comp_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "comp_rgba")?;
    let (a, b) = (&args[0], &args[1]);
    let (pa, pb) = (a.planes(), b.planes());
    let p = comp_rgba::Params { ca: a.count(), cb: b.count(), flag_a: a.is_const(), flag_b: b.is_const(), a_vals: a.f32s(), b_vals: b.f32s(), n: n as i32 };
    let host = comp_rgba::Host { vcall_f0_handled: false };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    let outcome = {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        comp_rgba::render(&p, &host, &pa, &pb, &mut outs).map_err(|e| kerr("comp_rgba", e))?
    };
    match outcome {
        comp_rgba::Outcome::Rendered { .. } => {}
        comp_rgba::Outcome::NotImplemented { ca, cb, .. } => {
            return Err(unsupported("comp_rgba", &format!("pair ({},{}) is a table entry the original reports as not yet implemented", ca, cb)));
        }
        comp_rgba::Outcome::PadService { calls } => {
            // (4,3): host plane service; mode 0 = operand A block, mode 1 = operand B block (NV5-A5, CALLEE-MODELLED)
            let (va, vb) = (a.bits().to_vec(), b.bits().to_vec());
            let blocks = [block_of(a, &pa, &va), block_of(b, &pb, &vb)];
            let host = ps::Host { in_pool: kp.in_pool };
            for c in calls {
                match c {
                    comp_rgba::PadCall::PlaneService { mode, chan, out } => {
                        let dst = outv.get_mut(out).ok_or(ExecError::Shape("comp_rgba pad plane".into()))?;
                        ps::plane_service(&host, ps::Sel::Direct { blocks: &blocks }, mode, chan, dst, &flat_roi(n)).map_err(|e| kerr("plane_service", e))?;
                    }
                    comp_rgba::PadCall::FillAlpha { value_bits, out } => {
                        let dst = outv.get_mut(out).ok_or(ExecError::Shape("comp_rgba pad plane".into()))?;
                        for x in dst.iter_mut() {
                            *x = f32::from_bits(value_bits);
                        }
                    }
                }
            }
        }
    }
    Ok(Image { n, planes: outv })
}

fn element_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "element_of")?;
    let (a, ix) = (&args[0], &args[1]);
    if !ix.is_const() {
        return Err(unsupported("element_of", "the index operand is read from constant lanes only (image index not in the builtin)"));
    }
    let p = ops_element::EParams { count: a.count(), nout: tag, n: n as i32, a_const: a.is_const(), a_consts: a.f32s(), idx: ix.f32s() };
    let mut planes: Vec<Vec<f32>> = a.planes().iter().map(|x| x.to_vec()).collect();
    let svc = ops_element::Svc { in_pool: kp.in_pool, is_int: false, values: a.bits(), enable: a.is_const(), img_valid: true };
    let mut outs: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    ops_element::element_of::render(&p, &mut planes, &svc, &mut outs, [-1; 4]).map_err(|e| kerr("element_of", e))?;
    Ok(Image { n, planes: outs })
}

fn vec_k(class: &str, args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    let p = ops_vec::Params { host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let planes: Vec<Vec<&[f32]>> = args.iter().map(|a| a.planes()).collect();
    let vals: Vec<Vec<u32>> = args.iter().map(|a| a.bits().to_vec()).collect();
    let blocks: Vec<ps::Block> = args.iter().enumerate().map(|(i, a)| block_of(a, &planes[i], &vals[i])).collect();
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        let r = match class {
            "v2" => ops_vec::v2::render(&p, &blocks, &mut outs),
            "v3" => ops_vec::v3::render(&p, &blocks, &mut outs),
            _ => ops_vec::v4::render(&p, &blocks, &mut outs),
        };
        r.map_err(|e| kerr(class, e))?;
    }
    Ok(Image { n, planes: outv })
}

/// `interp` (a, b, t) through the native crate; the AVX2 body and its slot function are at K-PARITY / R-PARITY.
fn interp_k(args: &[Arg], tag: i32, n: usize) -> Result<Image, ExecError> {
    need(args, 3, "interp")?;
    let (pa, pb, pt) = (args[0].planes(), args[1].planes(), args[2].planes());
    let oa = interp::Operand { count: args[0].count(), is_const: args[0].is_const(), consts: args[0].f32s(), planes: &pa };
    let ob = interp::Operand { count: args[1].count(), is_const: args[1].is_const(), consts: args[1].f32s(), planes: &pb };
    let ot = interp::Operand { count: args[2].count(), is_const: args[2].is_const(), consts: args[2].f32s(), planes: &pt };
    let p = interp::Params { nout: tag, n: n as i32 };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        interp::render(&p, &oa, &ob, &ot, &mut outs).map_err(|e| kerr("interp", e))?;
    }
    Ok(Image { n, planes: outv })
}

/// Runs one builtin class. `tag` is the selector's result (output channel count).
pub fn run_class(class: &str, tag: i32, args: &[Arg], n: usize, kp: &KParams) -> Result<Image, ExecError> {
    let c = normalize_class(class);
    if tag < 1 || tag > 4 {
        return Err(ExecError::Shape(format!("{}: combo tag {} outside 1..4", c, tag)));
    }
    match c {
        "hue_shift" => hue_shift_k(args, tag, n, kp),
        "sat_scale" => sat_scale_k(args, tag, n, kp),
        "hue_shift_preserve_luma" => hsp_k(args, tag, n, kp),
        "v3_to_v4_if4" => v3_to_v4_if4_k(args, tag, n, kp),
        "+" | "-" | "*" | "/" | "max" | "min" | "diff" => binary_k(c, args, tag, n, kp),
        "clamp" => clamp_k(args, tag, n, kp),
        "abs" | "negate" | "one_minus" => unary_k(c, args, tag, n, kp),
        "if+" | "if>" => if_k(c, args, tag, n),
        "comp_rgba" => comp_k(args, tag, n, kp),
        "screen_super" => screen_k(args, tag, n, kp),
        "overlay_super" => overlay_k(args, tag, n, kp),
        "screen" => screen2_k(args, tag, n, kp),
        "element_of" => element_k(args, tag, n, kp),
        "v2" | "v3" | "v4" => vec_k(c, args, tag, n, kp),
        "interp" => interp_k(args, tag, n),
        other => Err(unsupported(other, "no kernel wrapper in fx_exec (builtin not wired or not rebuilt)")),
    }
}

/// Dispatch: the classes of `kernels2::WIRED` through `kernels2::run_wired` (the ripmap result of make_mipmap is not a
/// plain image), every other class through `run_class`.
pub fn run_any(class: &str, tag: i32, args: &[Arg], n: usize, dims: (usize, usize), kp: &KParams) -> Result<crate::kernels2::Out, ExecError> {
    let c = normalize_class(class);
    if crate::kernels2::WIRED.contains(&c) {
        if tag < 1 || tag > 4 {
            return Err(ExecError::Shape(format!("{}: combo tag {} outside 1..4", c, tag)));
        }
        let mut out = crate::kernels2::run_wired(c, tag, args, n, dims, kp)?;
        if let (true, crate::kernels2::Out::Img(img)) = (crate::nanrule::ACTIVE && !crate::nanrule::NO_OUTPUT_REWRITE.contains(&c), &mut out) {
            let arm = args.iter().any(|a| crate::nanrule::has_arm_nan(&a.f32s()) || a.planes().iter().any(|p| crate::nanrule::has_arm_nan(p)));
            crate::nanrule::canon_planes(&mut img.planes, arm);
        }
        return Ok(out);
    }
    let mut img = run_class(c, tag, args, n, kp)?;
    if crate::nanrule::ACTIVE && !crate::nanrule::NO_OUTPUT_REWRITE.contains(&c) {
        let arm = args.iter().any(|a| crate::nanrule::has_arm_nan(&a.f32s()) || a.planes().iter().any(|p| crate::nanrule::has_arm_nan(p)));
        crate::nanrule::canon_planes(&mut img.planes, arm);
    }
    Ok(crate::kernels2::Out::Img(img))
}

/// diff vec_mask = OP2-A3, MEASURED-DYLIB 0x7fffffff per lane.
pub const DIFF_VEC_MASK: u32 = 0x7fff_ffff;
/// clamp allones: MEASURED-DYLIB 0xffffffff per lane.
pub const CLAMP_ALLONES: u32 = 0xffff_ffff;

/// clamp(a, b, c), three operands, native median ladders per operand shape (crate `clamp`). Output plane phases all take
/// `kp.dst3_phase_bytes` (a 32 byte aligned allocation gives 0; ALIGN-DEPENDENT only for n >=).
fn clamp_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 3, "clamp")?;
    let (pa, pb, pc) = (args[0].planes(), args[1].planes(), args[2].planes());
    let oa = ops_arith::Operand { count: args[0].count(), is_const: args[0].is_const(), consts: args[0].f32s(), planes: &pa };
    let ob = ops_arith::Operand { count: args[1].count(), is_const: args[1].is_const(), consts: args[1].f32s(), planes: &pb };
    let oc = ops_arith::Operand { count: args[2].count(), is_const: args[2].is_const(), consts: args[2].f32s(), planes: &pc };
    let p = ops_clamp::Params { nout: tag, n: n as i32, allones: CLAMP_ALLONES, dst_phase_bytes: [kp.dst3_phase_bytes; 4] };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        ops_clamp::render(&p, &oa, &ob, &oc, &mut outs).map_err(|e| kerr("clamp", e))?;
    }
    Ok(Image { n, planes: outv })
}
