//! Device ops: pointwise arithmetic and logic, vector shaping and coercions, interpolation and compositing,
//! colour. Each op has an `accepts` that mirrors the operand resolution of the CPU kernel (counts, constants, channel
//! clamps, tags) and refuses everything it does not cover, a `run` that launches the kernels of `fx_device::kernels_pointwise`,
//! and a self-check corpus. The CPU kernel stays the reference: the self-check compares bit for bit.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::types::{Image, KParams};
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

type Acc = fn(&NodeCall, &[ArgShape]) -> Result<(), &'static str>;
type Run = fn(&Arc<Session>, &NodeCall, &[DevArg]) -> Result<DevImage, DevError>;

const ONE_BITS: u32 = 0x3f80_0000;

// ---- shared helpers -------------------------------------------------------------------------------------------

fn unsup(why: impl Into<String>) -> DevError {
    DevError::Unsupported(why.into())
}

fn count_of(s: &ArgShape) -> Result<(usize, bool), &'static str> {
    match s {
        ArgShape::Img { channels } if (1..=4).contains(channels) => Ok((*channels, false)),
        ArgShape::Const { n } if (1..=4).contains(n) => Ok((*n, true)),
        ArgShape::Mip => Err("a ripmap operand"),
        _ => Err("operand channel count outside 1..4"),
    }
}

fn shapes_of(args: &[DevArg]) -> Vec<ArgShape> {
    args.iter()
        .map(|a| match a {
            DevArg::Img(i) => ArgShape::Img { channels: i.channels() },
            DevArg::Const { n, .. } => ArgShape::Const { n: *n },
            DevArg::Mip(_) | DevArg::DevMip(_) => ArgShape::Mip,
        })
        .collect()
}

fn plane<'a>(a: &'a DevArg, c: usize) -> Result<ClArg<'a>, DevError> {
    match a {
        DevArg::Img(i) if c < i.channels() => Ok(i.plane(c).arg()),
        _ => Err(unsup("plane operand missing")),
    }
}

fn lanes(a: &DevArg) -> [u32; 4] {
    match a {
        DevArg::Const { lanes, .. } => *lanes,
        DevArg::Img(_) | DevArg::Mip(_) | DevArg::DevMip(_) => [0; 4],
    }
}

fn is_const(a: &DevArg) -> bool {
    matches!(a, DevArg::Const { .. })
}

fn n_u32(c: &NodeCall) -> Result<u32, DevError> {
    if c.n == 0 || c.n > u32::MAX as usize {
        return Err(unsup("sample count out of range"));
    }
    Ok(c.n as u32)
}

fn check_len(c: &NodeCall, args: &[DevArg]) -> Result<(), DevError> {
    for a in args {
        if let DevArg::Img(i) = a {
            if i.n != c.n {
                return Err(unsup("plane length differs from the frame"));
            }
        }
    }
    Ok(())
}

fn zero(s: &Arc<Session>, out: &DevImage, p: usize, n: u32) -> Result<(), DevError> {
    s.launch("pointwise_fill_f32", n as usize, &[out.plane(p).arg(), ClArg::U32(0), ClArg::U32(n)])?;
    Ok(())
}

fn fill(s: &Arc<Session>, out: &DevImage, p: usize, bits: u32, n: u32) -> Result<(), DevError> {
    s.launch("pointwise_fill_f32", n as usize, &[out.plane(p).arg(), ClArg::U32(bits), ClArg::U32(n)])?;
    Ok(())
}

fn tag_ok(c: &NodeCall) -> Result<usize, &'static str> {
    if (1..=4).contains(&c.tag) {
        Ok(c.tag as usize)
    } else {
        Err("combination tag outside 1..4")
    }
}

fn vec_one_is_one(c: &NodeCall) -> Result<(), &'static str> {
    if c.kp.vec_one.to_bits() != 1.0f32.to_bits() {
        Err("the vector constant is not 1.0")
    } else {
        Ok(())
    }
}

// ---- binary arithmetic: + - * / max min diff -------------------------------------------------------------------

struct BinPlan {
    nch: usize,
    ext_b_side: Option<bool>,
}

fn bin_plan(c: &NodeCall, args: &[ArgShape]) -> Result<BinPlan, &'static str> {
    let tag = tag_ok(c)?;
    vec_one_is_one(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let (ca, a_const) = count_of(&args[0])?;
    let (cb, b_const) = count_of(&args[1])?;
    if a_const && b_const {
        return Err("both operands constant: the CPU kernel reads a plane of a constant");
    }
    let r14 = ca == 3 && cb == 4;
    let r12 = ca == 4 && cb == 3;
    let nch = if r14 || r12 || tag == 3 { 3 } else { tag };
    if nch > tag || ((r14 || r12) && tag < 4) {
        return Err("fewer output planes than the kernel writes");
    }
    Ok(BinPlan { nch, ext_b_side: if r14 { Some(true) } else if r12 { Some(false) } else { None } })
}

fn bin_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], op: u32) -> Result<DevImage, DevError> {
    bin_run_k(s, c, args, op, "pointwise_bin_f32", "pointwise_ext_f32")
}

/// The binary plan with the given main and extension kernels (screen_super has the same operand shapes as `+`).
fn bin_run_k(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], op: u32, kmain: &str, kext: &str) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let plan = bin_plan(c, &shapes).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let (a, b) = (&args[0], &args[1]);
    let (ca, cb) = (count_of(&shapes[0]).map_err(unsup)?.0 as i32, count_of(&shapes[1]).map_err(unsup)?.0 as i32);
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    for ch in 0..plan.nch {
        let ia = (ch as i32).min(ca - 1) as usize;
        let ib = (ch as i32).min(cb - 1) as usize;
        let o = out.plane(ch).arg();
        if is_const(a) {
            let pb = plane(b, ib)?;
            let pb2 = plane(b, ib)?;
            s.launch(kmain, c.n, &[pb, pb2, o, ClArg::U32(n), ClArg::U32(op), ClArg::U32(1), ClArg::U32(lanes(a)[ia]), ClArg::U32(0)])?;
        } else if is_const(b) {
            let pa = plane(a, ia)?;
            let pa2 = plane(a, ia)?;
            s.launch(kmain, c.n, &[pa, pa2, o, ClArg::U32(n), ClArg::U32(op), ClArg::U32(2), ClArg::U32(0), ClArg::U32(lanes(b)[ib])])?;
        } else {
            s.launch(kmain, c.n, &[plane(a, ia)?, plane(b, ib)?, o, ClArg::U32(n), ClArg::U32(op), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0)])?;
        }
    }
    if let Some(b_side) = plan.ext_b_side {
        let src = if b_side { b } else { a };
        let (cbits, cflag) = if is_const(src) { (lanes(src)[3], 1) } else { (0, 0) };
        let sp = if is_const(src) { plane(if b_side { a } else { b }, 0)? } else { plane(src, 3)? };
        s.launch(kext, c.n, &[sp, out.plane(3).arg(), ClArg::U32(n), ClArg::U32(op), ClArg::U32(u32::from(b_side)), ClArg::U32(cflag), ClArg::U32(cbits)])?;
    }
    let written = if plan.ext_b_side.is_some() { 4 } else { plan.nch };
    for p in written..tag {
        zero(s, &out, p, n)?;
    }
    Ok(out)
}

// ---- unary: abs negate -----------------------------------------------------------------------------------------

fn un_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let tag = tag_ok(c)?;
    if args.len() != 1 {
        return Err("one operand expected");
    }
    match args[0] {
        ArgShape::Img { channels } if (1..=4).contains(&channels) => {
            if channels < tag {
                Err("fewer operand planes than the output channels (the CPU kernel loops over the tag)")
            } else {
                Ok(())
            }
        }
        ArgShape::Const { .. } => Err("constant operand at run time (the compiler folds it)"),
        _ => Err("operand is not an image of 1..4 channels"),
    }
}

fn un_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], op: u32) -> Result<DevImage, DevError> {
    un_accepts(c, &shapes_of(args)).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let DevArg::Img(src) = &args[0] else { return Err(unsup("operand is not an image")) };
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    for p in 0..tag {
        s.launch("pointwise_un_f32", c.n, &[src.plane(p).arg(), out.plane(p).arg(), ClArg::U32(n), ClArg::U32(op)])?;
    }
    Ok(out)
}

// ---- if+ and if> -------------------------------------------------------------------------------------------------

struct ChanSel {
    /// condition operand constants are decided on the host
    cond_const: bool,
    two_x: bool,
    two_y: bool,
}

fn sel_k(args: &[ArgShape], kind: u32) -> Result<(), &'static str> {
    let want = if kind == 0 { 3 } else { 4 };
    if args.len() != want {
        return Err("operand count differs from the class");
    }
    for a in args {
        count_of(a)?;
    }
    Ok(())
}

fn sel_accepts(c: &NodeCall, args: &[ArgShape], kind: u32) -> Result<(), &'static str> {
    tag_ok(c)?;
    sel_k(args, kind)
}

fn sel_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], kind: u32) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    sel_accepts(c, &shapes, kind).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let tag = c.tag as usize;
    let (ia, ib, ix, iy) = if kind == 0 { (0usize, None, 1usize, 2usize) } else { (0, Some(1usize), 2, 3) };
    let cnt = |i: usize| count_of(&shapes[i]).map(|v| v.0 as i32).map_err(unsup);
    let out = DevImage::alloc(s, tag, c.n)?;
    let dummy = args.iter().find(|a| !is_const(a));
    let (cx, cy) = (cnt(ix)?, cnt(iy)?);
    let sel = ChanSel {
        cond_const: is_const(&args[ia]) && ib.map_or(true, |b| is_const(&args[b])),
        two_x: tag > 2 && cx == 2,
        two_y: tag > 2 && cy == 2,
    };
    for ch in 0..tag {
        let chi = ch as i32;
        let (mut xconst, mut yconst) = (is_const(&args[ix]), is_const(&args[iy]));
        let (mut xc, mut yc) = (lanes(&args[ix])[ch], lanes(&args[iy])[ch]);
        if ch == 3 {
            if (cx - 1) & 1 == 0 {
                xc = ONE_BITS;
                xconst = true;
            }
            if (cy - 1) & 1 == 0 {
                yc = ONE_BITS;
                yconst = true;
            }
        }
        let pidx = |k: i32, two: bool| -> usize { if two { (chi - 2).max(0) as usize } else { chi.min(k - 1) as usize } };
        let o = out.plane(ch).arg();
        if sel.cond_const {
            let av = f32::from_bits(lanes(&args[ia])[ch]);
            let cv = match ib {
                None => av > 0.0,
                Some(b) => av > f32::from_bits(lanes(&args[b])[ch]),
            };
            let (aconst, aval, ao, two) = if cv { (xconst, xc, ix, sel.two_x) } else { (yconst, yc, iy, sel.two_y) };
            if aconst {
                fill(s, &out, ch, aval, n)?;
            } else {
                let k = cnt(ao)?;
                s.launch("copy_f32", c.n, &[plane(&args[ao], pidx(k, two))?, o, ClArg::U32(n)])?;
            }
            continue;
        }
        let d = dummy.ok_or_else(|| unsup("no image operand"))?;
        let aidx = |i: usize| -> Result<usize, DevError> { Ok(chi.min(cnt(i)? - 1) as usize) };
        let mut cm = 0u32;
        let (mut wa, mut wb, mut wx, mut wy) = (0u32, 0u32, 0u32, 0u32);
        let pa = if is_const(&args[ia]) {
            cm |= 1;
            wa = lanes(&args[ia])[ch];
            plane(d, 0)?
        } else {
            plane(&args[ia], aidx(ia)?)?
        };
        let pb = match ib {
            Some(b) if is_const(&args[b]) => {
                cm |= 2;
                wb = lanes(&args[b])[ch];
                plane(d, 0)?
            }
            Some(b) => plane(&args[b], aidx(b)?)?,
            None => plane(d, 0)?,
        };
        let px = if xconst {
            cm |= 4;
            wx = xc;
            plane(d, 0)?
        } else {
            plane(&args[ix], pidx(cx, sel.two_x))?
        };
        let py = if yconst {
            cm |= 8;
            wy = yc;
            plane(d, 0)?
        } else {
            plane(&args[iy], pidx(cy, sel.two_y))?
        };
        s.launch("pointwise_sel_f32", c.n, &[pa, pb, px, py, o, ClArg::U32(n), ClArg::U32(kind), ClArg::U32(cm), ClArg::U32(wa), ClArg::U32(wb), ClArg::U32(wx), ClArg::U32(wy)])?;
    }
    Ok(out)
}

// ---- clamp -----------------------------------------------------------------------------------------------------

const CLAMP_ALLONES: u32 = 0xffff_ffff;

fn clamp_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    tag_ok(c)?;
    if args.len() != 3 {
        return Err("three operands expected");
    }
    let mut k = [(0usize, false); 3];
    for (i, a) in args.iter().enumerate() {
        k[i] = count_of(a)?;
    }
    if c.kp.dst3_phase_bytes & 0x1f != 0 {
        return Err("the destination planes are not 32 byte aligned");
    }
    // shape 11x reads the plane of operand C even when it is a constant
    if k[0].1 && k[1].1 && k[2].1 {
        return Err("all three operands constant: the CPU kernel reads a plane of a constant");
    }
    Ok(())
}

fn clamp_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    clamp_accepts(c, &shapes).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let tag = c.tag as usize;
    let (fa, fb, fc) = (is_const(&args[0]), is_const(&args[1]), is_const(&args[2]));
    let shape: u32 = match (fa, fb, fc) {
        (true, false, true) => 0,
        (true, false, false) => 1,
        (true, true, _) => 2,
        (false, true, true) => 3,
        (false, true, false) => 4,
        (false, false, true) => 5,
        (false, false, false) => 6,
    };
    let vend = n - (n & 7);
    let cnt = |i: usize| count_of(&shapes[i]).map(|v| v.0 as i32).map_err(unsup);
    let out = DevImage::alloc(s, tag, c.n)?;
    let dummy = args.iter().find(|a| !is_const(a)).ok_or_else(|| unsup("no image operand"))?;
    for ch in 0..tag {
        let chi = ch as i32;
        let idx = |i: usize| -> Result<usize, DevError> { Ok(chi.min(cnt(i)? - 1) as usize) };
        let cw = |i: usize| lanes(&args[i])[ch];
        // roles per shape: which operand supplies p0, p1, p2 and c0, c1, c2
        let (mut p0, mut p1, mut p2) = (plane(dummy, 0)?, plane(dummy, 0)?, plane(dummy, 0)?);
        let (mut w0, mut w1, mut w2) = (0u32, 0u32, 0u32);
        match shape {
            0 => {
                w0 = cw(0);
                w2 = cw(2);
                p1 = plane(&args[1], idx(1)?)?;
            }
            1 => {
                w0 = cw(0);
                p1 = plane(&args[1], idx(1)?)?;
                p2 = plane(&args[2], idx(2)?)?;
            }
            2 => {
                w0 = cw(0);
                w1 = cw(1);
                if is_const(&args[2]) {
                    return Err(unsup("operand C is a constant in shape 11x"));
                }
                p2 = plane(&args[2], idx(2)?)?;
            }
            3 => {
                w1 = cw(1);
                w2 = cw(2);
                p0 = plane(&args[0], idx(0)?)?;
            }
            4 => {
                w1 = cw(1);
                p0 = plane(&args[0], idx(0)?)?;
                p2 = plane(&args[2], idx(2)?)?;
            }
            5 => {
                w2 = cw(2);
                p0 = plane(&args[0], idx(0)?)?;
                p1 = plane(&args[1], idx(1)?)?;
            }
            _ => {
                p0 = plane(&args[0], idx(0)?)?;
                p1 = plane(&args[1], idx(1)?)?;
                p2 = plane(&args[2], idx(2)?)?;
            }
        }
        s.launch(
            "pointwise_clamp_f32",
            c.n,
            &[p0, p1, p2, out.plane(ch).arg(), ClArg::U32(n), ClArg::U32(shape), ClArg::U32(vend), ClArg::U32(w0), ClArg::U32(w1), ClArg::U32(w2), ClArg::U32(CLAMP_ALLONES)],
        )?;
    }
    Ok(out)
}

// ---- lane routing: the plane service as the CPU kernels call it -----------------------------------------------------

/// Where one output plane comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    /// plane service on argument `arg`, channel `chan` (the channel is clamped to the operand count; a constant fills)
    Chan { arg: usize, chan: i32 },
    /// plane service fill with a constant word
    Fill(u32),
}

/// The plane service's fill value for `n` samples: a pooled fill of 512 or more samples turns a zero (either sign) into +0.0.
fn service_fill_bits(value: u32, n: usize, in_pool: bool) -> u32 {
    if !in_pool && n >= 0x200 && f32::from_bits(value) == 0.0 {
        0
    } else {
        value
    }
}

fn route_check(shapes: &[ArgShape], recipe: &[(usize, Src)]) -> Result<(), &'static str> {
    for (_, src) in recipe {
        if let Src::Chan { arg, chan } = *src {
            let sh = shapes.get(arg).ok_or("operand missing")?;
            if chan < 0 {
                return Err("negative channel");
            }
            let (count, is_c) = match sh {
                ArgShape::Img { channels } => (*channels as i32, false),
                ArgShape::Const { n } => (*n as i32, true),
                ArgShape::Mip => return Err("a ripmap operand"),
            };
            let ch = chan.min(count);
            if is_c && ch >= 4 {
                return Err("constant lane out of range");
            }
            if !is_c && ch >= count {
                return Err("channel beyond the operand's planes");
            }
        }
    }
    Ok(())
}

fn route_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], recipe: &[(usize, Src)], tag: usize, direct_const_fill: bool) -> Result<DevImage, DevError> {
    check_len(c, args)?;
    let n = n_u32(c)?;
    let shapes = shapes_of(args);
    route_check(&shapes, recipe).map_err(unsup)?;
    let out = DevImage::alloc(s, tag, c.n)?;
    let mut written = vec![false; tag];
    for (p, _) in recipe {
        if *p < tag {
            written[*p] = true;
        }
    }
    for p in 0..tag {
        if !written[p] {
            zero(s, &out, p, n)?;
        }
    }
    for (p, src) in recipe {
        if *p >= tag {
            continue;
        }
        match *src {
            Src::Fill(v) => fill(s, &out, *p, service_fill_bits(v, c.n, c.kp.in_pool), n)?,
            Src::Chan { arg, chan } => match &args[arg] {
                DevArg::Img(img) => {
                    let ch = (chan as usize).min(img.channels());
                    s.launch("copy_f32", c.n, &[img.plane(ch).arg(), out.plane(*p).arg(), ClArg::U32(n)])?;
                }
                DevArg::Const { n: cn, lanes } => {
                    let ch = (chan as usize).min(*cn);
                    let v = lanes[ch];
                    let v = if direct_const_fill { v } else { service_fill_bits(v, c.n, c.kp.in_pool) };
                    fill(s, &out, *p, v, n)?;
                }
                DevArg::Mip(_) | DevArg::DevMip(_) => return Err(unsup("a ripmap operand")),
            },
        }
    }
    Ok(out)
}

// v2 v3 v4: output i is argument i, channel 0
fn vec_recipe(k: usize) -> Vec<(usize, Src)> {
    (0..k).map(|i| (i, Src::Chan { arg: i, chan: 0 })).collect()
}

fn vec_accepts(c: &NodeCall, args: &[ArgShape], k: usize) -> Result<(), &'static str> {
    let tag = tag_ok(c)?;
    if args.len() < k {
        return Err("fewer operands than the class takes");
    }
    if tag < k {
        return Err("fewer output planes than the class writes");
    }
    route_check(args, &vec_recipe(k))
}

fn vec_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], k: usize) -> Result<DevImage, DevError> {
    vec_accepts(c, &shapes_of(args), k).map_err(unsup)?;
    route_run(s, c, args, &vec_recipe(k), c.tag as usize, false)
}

// coerce family: member passes of ops_coerce
use ops_coerce::Pass;

fn coerce_recipe(pass: Pass, tag: usize, count: i32) -> Vec<(usize, Src)> {
    let ch = |c: i32| Src::Chan { arg: 0, chan: c };
    let mut r: Vec<(usize, Src)> = Vec::new();
    match pass {
        Pass::C_CELL_A => {
            for c in 0..tag {
                r.push((c, ch(c as i32)));
            }
        }
        Pass::D_CELL_A => {
            for c in 0..tag {
                r.push((c, ch(0)));
            }
        }
        Pass::DE_CELL_A => {
            r.push((0, ch(0)));
            r.push((1, ch(1)));
            r.push((2, Src::Fill(0)));
        }
        Pass::E_CELL_A => {
            if tag > 1 {
                for c in 0..tag - 1 {
                    r.push((c, ch(0)));
                }
            }
            r.push((tag - 1, Src::Fill(ONE_BITS)));
        }
        Pass::EF_CELL_A => {
            for c in 0..3 {
                r.push((c, ch(c as i32)));
            }
            r.push((3, Src::Fill(ONE_BITS)));
        }
        Pass::F_CELL_A => {
            for c in 0..4 {
                r.push((c, ch(0)));
            }
            r.push((3, ch(1)));
        }
        Pass::P_CELL_A => r.push((0, ch(count.wrapping_sub(1)))),
    }
    r
}

fn coerce_class(class: &str) -> Option<ops_coerce::Class> {
    Some(match class {
        "coerce_up" => ops_coerce::Class::CoerceUp,
        "coerce_up_a1" => ops_coerce::Class::CoerceUpA1,
        "coerce_to" => ops_coerce::Class::CoerceTo,
        "coerce_to_alpha" => ops_coerce::Class::CoerceToAlpha,
        _ => return None,
    })
}

const TO_V3: [(i32, i32, Pass); 4] = [(1, 3, Pass::D_CELL_A), (2, 3, Pass::DE_CELL_A), (3, 3, Pass::C_CELL_A), (4, 3, Pass::C_CELL_A)];
const TO_V4: [(i32, i32, Pass); 3] = [(1, 4, Pass::E_CELL_A), (3, 4, Pass::EF_CELL_A), (4, 4, Pass::C_CELL_A)];
const UP_TO_V3: [(i32, i32, Pass); 4] = [(1, 3, Pass::D_CELL_A), (2, 3, Pass::DE_CELL_A), (3, 3, Pass::C_CELL_A), (4, 4, Pass::C_CELL_A)];
const REMOVE_ALPHA: [(i32, i32, Pass); 4] = [(1, 1, Pass::C_CELL_A), (2, 1, Pass::C_CELL_A), (3, 3, Pass::C_CELL_A), (4, 3, Pass::C_CELL_A)];

/// The recipe of a coerce class for these operands and tag, or why the CPU kernel does not run it as a plain routing.
fn coerce_plan(class: &str, tag: i32, args: &[ArgShape]) -> Result<Vec<(usize, Src)>, &'static str> {
    if tag < 1 {
        return Err("combination tag outside 1..4");
    }
    if args.is_empty() {
        return Err("operand missing");
    }
    let mut counts = Vec::new();
    for a in args {
        counts.push(match a {
            ArgShape::Img { channels } => *channels as i32,
            ArgShape::Const { n } => *n as i32,
            ArgShape::Mip => return Err("a ripmap operand"),
        });
    }
    let (pass, rtag) = if let Some(cl) = coerce_class(class) {
        let combo = ops_coerce::select(cl, &counts).ok_or("no combination row for these operand counts")?;
        (combo.pass, combo.tag as i32)
    } else {
        let table: &[(i32, i32, Pass)] = match class {
            "coerce_to_v3" => &TO_V3,
            "coerce_to_v4" => &TO_V4,
            "remove_alpha" => &REMOVE_ALPHA,
            _ => &UP_TO_V3,
        };
        if counts.len() != 1 {
            return Err("one operand expected");
        }
        let row = table.iter().find(|r| r.0 == counts[0]).ok_or("no combination row for this operand count")?;
        (row.2, row.1)
    };
    if rtag != tag {
        return Err("selector tag differs from the class row tag");
    }
    if tag > 4 {
        return Err("tag above 4");
    }
    Ok(coerce_recipe(pass, tag as usize, counts[0]))
}

fn coerce_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let r = coerce_plan(c.class, c.tag, args)?;
    route_check(args, &r)
}

fn coerce_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let r = coerce_plan(c.class, c.tag, &shapes_of(args)).map_err(unsup)?;
    route_run(s, c, args, &r, c.tag as usize, false)
}

// v3_to_v4 and v3_to_v4_if4: member passes of the v3_to_v4_if4 crate
fn v34_recipe(combo: usize, count0: i32) -> Result<(Vec<(usize, Src)>, usize), &'static str> {
    use v3_to_v4_if4::Pass as P;
    let cb = v3_to_v4_if4::COMBOS.get(combo).ok_or("no combination row")?;
    let pass = v3_to_v4_if4::combo_pass(combo).ok_or("no combination row")?;
    let mut r = Vec::new();
    match pass {
        P::A => {
            let mx = count0.wrapping_sub(1);
            for c in 0..3 {
                let chan = if c >= mx { mx } else { c };
                r.push((c as usize, Src::Chan { arg: 0, chan }));
            }
            r.push((3, Src::Chan { arg: 1, chan: 0 }));
        }
        P::B => {
            for c in 0..cb.out {
                r.push((c, Src::Chan { arg: 0, chan: c as i32 }));
            }
        }
        P::C => {
            for c in 0..cb.out {
                r.push((c, Src::Chan { arg: 0, chan: 0 }));
            }
        }
        P::D => {
            r.push((0, Src::Chan { arg: 0, chan: 0 }));
            r.push((1, Src::Chan { arg: 1, chan: 0 }));
        }
    }
    Ok((r, cb.out))
}

fn v34_plan(class: &str, tag: i32, args: &[ArgShape]) -> Result<Vec<(usize, Src)>, &'static str> {
    let cnt = |i: usize| -> Result<i32, &'static str> {
        match args.get(i) {
            Some(ArgShape::Img { channels }) => Ok(*channels as i32),
            Some(ArgShape::Const { n }) => Ok(*n as i32),
            _ => Err("operand missing or a ripmap"),
        }
    };
    if tag < 1 {
        return Err("combination tag outside 1..4");
    }
    let combo = if class == "v3_to_v4_if4" {
        if args.len() != 3 {
            return Err("three operands expected");
        }
        v3_to_v4_if4::select_combo([cnt(0)?, cnt(1)?, cnt(2)?]).ok_or("no combination row")?
    } else {
        if args.len() != 2 {
            return Err("two operands expected");
        }
        const ROWS: [[i32; 2]; 6] = [[3, 1], [3, 3], [4, 1], [4, 3], [1, 1], [1, 3]];
        if !ROWS.contains(&[cnt(0)?, cnt(1)?]) {
            return Err("no combination row");
        }
        if tag != 4 {
            return Err("selector tag differs from the class row tag 4");
        }
        0
    };
    let (r, out) = v34_recipe(combo, cnt(0)?)?;
    if (tag as usize) < out {
        return Err("fewer output planes than the combination writes");
    }
    Ok(r)
}

fn v34_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let r = v34_plan(c.class, c.tag, args)?;
    route_check(args, &r)
}

fn v34_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let r = v34_plan(c.class, c.tag, &shapes_of(args)).map_err(unsup)?;
    route_run(s, c, args, &r, c.tag as usize, false)
}

// element_of: the index operand is a constant, so every channel is a routing decided on the host
fn rem3_raw(i: i32) -> i32 {
    let p = (i as i64) * 0x5555_5556i64;
    let hi = (p >> 32) as i32;
    let q = hi.wrapping_sub(i >> 31);
    i.wrapping_sub(q.wrapping_mul(3))
}

fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

fn element_fold(count: i32, raw: i32) -> i32 {
    match count {
        2 => raw & 1,
        3 => {
            let r = rem3_raw(raw);
            if r < 0 {
                r + 3
            } else {
                r
            }
        }
        _ => raw & 3,
    }
}

fn element_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    tag_ok(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    count_of(&args[0])?;
    if !matches!(args[1], ArgShape::Const { .. }) {
        return Err("the index operand is read from constant lanes only");
    }
    Ok(())
}

fn element_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    element_accepts(c, &shapes).map_err(unsup)?;
    let count = count_of(&shapes[0]).map_err(unsup)?.0 as i32;
    let idx = lanes(&args[1]);
    let tag = c.tag as usize;
    let mut r: Vec<(usize, Src)> = Vec::new();
    if count == 1 {
        r.push((0, Src::Chan { arg: 0, chan: 0 }));
        return route_run(s, c, args, &r, tag, false);
    }
    for ch in 0..tag {
        let k = element_fold(count, cvtt(f32::from_bits(idx[ch])));
        r.push((ch, Src::Chan { arg: 0, chan: k }));
    }
    // a constant source fills directly (no pooled zero rule) for count >= 2
    route_run(s, c, args, &r, tag, true)
}

// max_element and min_element
fn reduce_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    tag_ok(c)?;
    if args.len() != 1 {
        return Err("one operand expected");
    }
    let (count, is_c) = count_of(&args[0])?;
    if count >= 2 && is_c {
        return Err("a constant with two or more lanes has no planes for the reduction");
    }
    Ok(())
}

fn reduce_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], op: u32) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    reduce_accepts(c, &shapes).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let count = count_of(&shapes[0]).map_err(unsup)?.0;
    if count == 1 {
        return route_run(s, c, args, &[(0, Src::Chan { arg: 0, chan: 0 })], 1, false);
    }
    let DevArg::Img(img) = &args[0] else { return Err(unsup("operand is not an image")) };
    let out = DevImage::alloc(s, 1, c.n)?;
    let pl = |k: usize| img.plane(k.min(count - 1)).arg();
    s.launch("pointwise_reduce_f32", c.n, &[pl(0), pl(1), pl(2), pl(3), out.plane(0).arg(), ClArg::U32(n), ClArg::U32(count as u32), ClArg::U32(op)])?;
    Ok(out)
}


// ---- per channel operand sources shared by the three operand ops ------------------------------------------------------

/// One input of a per channel kernel: a plane of an operand or a constant word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum In {
    Plane { arg: usize, idx: usize },
    Word(u32),
}

fn dummy_holder(s: &Arc<Session>, args: &[DevArg]) -> Result<Option<DevImage>, DevError> {
    if args.iter().any(|a| !is_const(a)) {
        Ok(None)
    } else {
        Ok(Some(DevImage::alloc(s, 1, 1)?))
    }
}

fn dummy_arg<'a>(args: &'a [DevArg], holder: &'a Option<DevImage>) -> Result<ClArg<'a>, DevError> {
    match holder {
        Some(h) => Ok(h.plane(0).arg()),
        None => {
            let a = args.iter().find(|a| !is_const(a)).ok_or_else(|| unsup("no image operand"))?;
            plane(a, 0)
        }
    }
}

/// Binds three inputs to (pa, pb, pt) pointers and the constant mask and words.
fn bind3<'a>(args: &'a [DevArg], dummy: ClArg<'a>, ins: [In; 3]) -> Result<([ClArg<'a>; 3], u32, [u32; 3]), DevError> {
    let mut cm = 0u32;
    let mut w = [0u32; 3];
    let mut ptr: Vec<ClArg<'a>> = Vec::with_capacity(3);
    for (k, i) in ins.iter().enumerate() {
        match *i {
            In::Plane { arg, idx } => ptr.push(plane(&args[arg], idx)?),
            In::Word(v) => {
                cm |= 1 << k;
                w[k] = v;
                ptr.push(dummy);
            }
        }
    }
    let t = ptr.pop().ok_or_else(|| unsup("bind"))?;
    let b = ptr.pop().ok_or_else(|| unsup("bind"))?;
    let a = ptr.pop().ok_or_else(|| unsup("bind"))?;
    Ok(([a, b, t], cm, w))
}

fn launch3(s: &Arc<Session>, c: &NodeCall, kernel: &str, args: &[DevArg], holder: ClArg, ins: [In; 3], out: ClArg, extra: &[u32]) -> Result<(), DevError> {
    let (p, cm, w) = bind3(args, holder, ins)?;
    let n = n_u32(c)?;
    let mut a: Vec<ClArg> = vec![p[0], p[1], p[2], out, ClArg::U32(n), ClArg::U32(cm), ClArg::U32(w[0]), ClArg::U32(w[1]), ClArg::U32(w[2])];
    for e in extra {
        a.push(ClArg::U32(*e));
    }
    s.launch(kernel, c.n, &a)?;
    Ok(())
}

fn lane_in(args: &[DevArg], arg: usize, lane: usize) -> In {
    In::Word(lanes(&args[arg])[lane])
}

fn src_in(args: &[DevArg], shapes: &[ArgShape], arg: usize, ch: usize) -> Result<In, DevError> {
    let cnt = count_of(&shapes[arg]).map_err(unsup)?.0;
    Ok(if is_const(&args[arg]) { In::Plane { arg, idx: 0 } } else { In::Plane { arg, idx: ch.min(cnt - 1) } })
}

// ---- interp ------------------------------------------------------------------------------------------------------------

fn three_ops(c: &NodeCall, args: &[ArgShape], need_image: bool) -> Result<[(usize, bool); 3], &'static str> {
    tag_ok(c)?;
    if args.len() != 3 {
        return Err("three operands expected");
    }
    let k = [count_of(&args[0])?, count_of(&args[1])?, count_of(&args[2])?];
    if need_image && k.iter().all(|x| x.1) {
        return Err("all operands constant (the compiler folds the node)");
    }
    Ok(k)
}

fn interp_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    three_ops(c, args, true).map(|_| ())
}

const RNG_A_HI: f64 = f64::from_bits(0x3ddb7cdfd9d7bdbb);
const RNG_A_LO: f64 = f64::from_bits(0xbddb7cdfd9d7bdbb);
const RNG_B_HI: f64 = f64::from_bits(0x3ff000000006df38);
const RNG_B_LO: f64 = f64::from_bits(0x3feffffffff24190);

fn interp_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let k = three_ops(c, &shapes, true).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    let holder = dummy_holder(s, args)?;
    let dh = dummy_arg(args, &holder)?;
    for ch in 0..tag {
        let alpha = ch == 3;
        let ins_ab = |arg: usize| -> Result<(In, bool, f32), DevError> {
            let forced = alpha && k[arg].0 - 1 < 3;
            if forced {
                Ok((In::Word(ONE_BITS), true, 1.0))
            } else if k[arg].1 {
                let w = lanes(&args[arg])[ch];
                Ok((In::Word(w), true, f32::from_bits(w)))
            } else {
                Ok((src_in(args, &shapes, arg, ch)?, false, 0.0))
            }
        };
        let (ia, fa, xa) = ins_ab(0)?;
        let (ib, fb, xb) = ins_ab(1)?;
        let it = if k[2].1 { lane_in(args, 2, ch) } else { src_in(args, &shapes, 2, ch)? };
        let ft = k[2].1;
        let in_window = {
            let (a, b) = (xa as f64, xb as f64);
            a < RNG_A_HI && a > RNG_A_LO && b < RNG_B_HI && b > RNG_B_LO
        };
        if fa && fb && !ft && in_window {
            let In::Plane { arg, idx } = it else { return Err(unsup("matte plane")) };
            s.launch("copy_f32", c.n, &[plane(&args[arg], idx)?, out.plane(ch).arg(), ClArg::U32(n)])?;
        } else {
            launch3(s, c, "pointwise_interp_f32", args, dh, [ia, ib, it], out.plane(ch).arg(), &[])?;
        }
    }
    Ok(out)
}

// ---- dissolve ----------------------------------------------------------------------------------------------------------

fn dissolve_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    three_ops(c, args, true).map(|_| ())
}

fn dissolve_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let k = three_ops(c, &shapes, true).map_err(unsup)?;
    check_len(c, args)?;
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    let holder = dummy_holder(s, args)?;
    let dh = dummy_arg(args, &holder)?;
    for ch in 0..tag {
        let pick = |arg: usize| -> In {
            let (count, is_c) = k[arg];
            if ch == 3 && (count - 1) & 1 == 0 {
                return In::Word(ONE_BITS);
            }
            if is_c {
                return In::Word(lanes(&args[arg])[ch]);
            }
            let rule = tag > 2 && count == 2;
            let idx = if rule { ch.saturating_sub(2) } else { ch.min(count - 1) };
            In::Plane { arg, idx }
        };
        let it = if k[2].1 { lane_in(args, 2, ch) } else { In::Plane { arg: 2, idx: ch.min(k[2].0 - 1) } };
        launch3(s, c, "pointwise_dissolve_f32", args, dh, [pick(0), pick(1), it], out.plane(ch).arg(), &[])?;
    }
    Ok(out)
}

// ---- comp_premult ------------------------------------------------------------------------------------------------------

enum PremChan {
    FillOne,
    Compute { ins: [In; 3], c3: bool },
}

fn prem_plan(c: &NodeCall, args: &[ArgShape], lane: &dyn Fn(usize, usize) -> u32) -> Result<Vec<PremChan>, &'static str> {
    let k = three_ops(c, args, false)?;
    let tag = c.tag as usize;
    let mut r = Vec::new();
    for ch in 0..tag {
        let (ca, a_const) = k[0];
        let (cb, b_const0) = k[1];
        let (ct, t_const) = k[2];
        let mut b_const = b_const0;
        let mut b_from_t = false;
        if ch == 3 {
            if ca - 1 == 2 {
                r.push(PremChan::FillOne);
                return Ok(r);
            }
            if cb - 1 == 2 {
                b_from_t = true;
                b_const = t_const;
            }
        }
        let t_idx = ch.min(ct - 1);
        let a_in = if a_const { In::Word(lane(0, ch)) } else { In::Plane { arg: 0, idx: ch.min(ca - 1) } };
        let t_in = if t_const { In::Word(lane(2, ch)) } else { In::Plane { arg: 2, idx: t_idx } };
        let b_in = if b_const {
            In::Word(lane(1, ch))
        } else if b_from_t {
            In::Plane { arg: 2, idx: t_idx }
        } else {
            In::Plane { arg: 1, idx: ch.min(cb - 1) }
        };
        if a_const && b_const && t_const {
            return Err("every input of a channel is constant (the CPU rounds through an integer)");
        }
        r.push(PremChan::Compute { ins: [a_in, b_in, t_in], c3: a_const && !b_const && t_const });
    }
    Ok(r)
}

fn prem_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    prem_plan(c, args, &|_, _| 0).map(|_| ())
}

fn prem_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let plan = prem_plan(c, &shapes, &|a, l| lanes(&args[a])[l]).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    let holder = dummy_holder(s, args)?;
    let dh = dummy_arg(args, &holder)?;
    let mut written = vec![false; tag];
    for (ch, p) in plan.iter().enumerate() {
        written[ch] = true;
        match p {
            PremChan::FillOne => fill(s, &out, ch, ONE_BITS, n)?,
            PremChan::Compute { ins, c3 } => launch3(s, c, "pointwise_prem_f32", args, dh, *ins, out.plane(ch).arg(), &[u32::from(*c3)])?,
        }
    }
    for p in 0..tag {
        if !written[p] {
            zero(s, &out, p, n)?;
        }
    }
    Ok(out)
}

// ---- comp_rgba ---------------------------------------------------------------------------------------------------------

enum CompChan {
    FillOne,
    Compute { ins: [In; 3], mode: u32 },
}

enum CompPlan {
    Chans(Vec<CompChan>),
    Pad(Vec<(usize, Src)>),
}

fn comp_plan(c: &NodeCall, args: &[ArgShape], lane: &dyn Fn(usize, usize) -> u32) -> Result<CompPlan, &'static str> {
    let tag = tag_ok(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let (ca, a_const) = count_of(&args[0])?;
    let (cb, b_flag) = count_of(&args[1])?;
    if a_const && b_flag {
        return Err("both operands constant: the CPU kernel reads a plane of a constant");
    }
    let b_const = b_flag;
    #[derive(PartialEq, Clone, Copy)]
    enum R {
        L11,
        L32,
        A,
        B,
        C,
        D,
        Pad,
    }
    let r = match (ca, cb) {
        (4, 4) | (4, 1) => R::B,
        (4, 2) => R::C,
        (4, 3) => R::Pad,
        (3, 4) | (1, 4) => R::A,
        (3, 1) => R::D,
        (3, 2) => R::L32,
        (1, 1) => R::L11,
        _ => return Err("pair is a table entry the original reports as not yet implemented"),
    };
    if r == R::Pad {
        if tag < 4 {
            return Err("fewer output planes than the kernel writes");
        }
        return Ok(CompPlan::Pad(vec![(0, Src::Chan { arg: 1, chan: 0 }), (1, Src::Chan { arg: 1, chan: 1 }), (2, Src::Chan { arg: 1, chan: 2 }), (3, Src::Fill(ONE_BITS))]));
    }
    let (oc, alpha_one, alpha_blend) = match r {
        R::L11 => (1usize, false, false),
        R::L32 | R::D => (3, false, false),
        R::A => (4, true, false),
        _ => (4, false, true),
    };
    if tag < oc {
        return Err("fewer output planes than the kernel writes");
    }
    let tc = if cb & 1 != 0 { 0 } else { cb - 1 };
    let dbl = b_const && matches!(r, R::L11 | R::L32 | R::D);
    let sel = |cnt: usize, j: usize| j.min(cnt.saturating_sub(1));
    let t_in = if b_const { In::Word(lane(1, tc)) } else { In::Plane { arg: 1, idx: tc } };
    let ab = |j: usize| -> (In, In) {
        let a = if a_const { In::Word(lane(0, sel(ca, j))) } else { In::Plane { arg: 0, idx: sel(ca, j) } };
        let b = if b_const { In::Word(lane(1, sel(cb, j))) } else { In::Plane { arg: 1, idx: sel(cb, j) } };
        (a, b)
    };
    let colour = if alpha_one || alpha_blend { 3 } else { oc };
    let mut v = Vec::new();
    for j in 0..colour {
        let (a, b) = ab(j);
        v.push(CompChan::Compute { ins: [a, b, t_in], mode: u32::from(dbl) });
    }
    if alpha_one {
        v.push(CompChan::FillOne);
    }
    if alpha_blend {
        let (a, b) = ab(3);
        v.push(CompChan::Compute { ins: [a, b, t_in], mode: 2 });
    }
    Ok(CompPlan::Chans(v))
}

fn comp_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    match comp_plan(c, args, &|_, _| 0)? {
        CompPlan::Pad(r) => route_check(args, &r),
        CompPlan::Chans(_) => Ok(()),
    }
}

fn comp_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let plan = comp_plan(c, &shapes, &|a, l| lanes(&args[a])[l]).map_err(unsup)?;
    let tag = c.tag as usize;
    match plan {
        CompPlan::Pad(r) => route_run(s, c, args, &r, tag, true),
        CompPlan::Chans(v) => {
            check_len(c, args)?;
            let n = n_u32(c)?;
            let out = DevImage::alloc(s, tag, c.n)?;
            let holder = dummy_holder(s, args)?;
            let dh = dummy_arg(args, &holder)?;
            for (ch, p) in v.iter().enumerate() {
                match p {
                    CompChan::FillOne => fill(s, &out, ch, ONE_BITS, n)?,
                    CompChan::Compute { ins, mode } => launch3(s, c, "pointwise_comp_f32", args, dh, *ins, out.plane(ch).arg(), &[*mode])?,
                }
            }
            for p in v.len()..tag {
                zero(s, &out, p, n)?;
            }
            Ok(out)
        }
    }
}

// ---- postdivide --------------------------------------------------------------------------------------------------------

fn postdiv_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let tag = tag_ok(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let (e, is_c) = count_of(&args[0])?;
    if is_c {
        return Err("constant source");
    }
    if !matches!(args[1], ArgShape::Const { .. }) {
        return Err("copy_alpha is not a constant");
    }
    if e == 1 || e == 3 {
        return Err("the compiler folds postdivide on one and three channels");
    }
    if e < tag.min(3) {
        return Err("fewer source planes than the colour outputs");
    }
    Ok(())
}

fn postdiv_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    postdiv_accepts(c, &shapes).map_err(unsup)?;
    check_len(c, args)?;
    let copy_alpha = cvtt(f32::from_bits(lanes(&args[1])[0]));
    if copy_alpha >= 0 {
        return Err(unsup("copy_alpha at or above zero (the alpha from another image)"));
    }
    let n = n_u32(c)?;
    let DevArg::Img(src) = &args[0] else { return Err(unsup("source is not an image")) };
    let e = src.channels();
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    let pa = src.plane(e - 1);
    let colour_mode = if tag <= 2 { 0u32 } else { 1 };
    let nc = tag.min(3);
    let ncol = if tag <= 2 { 1 } else { nc };
    for k in 0..ncol {
        s.launch("pointwise_postdiv_f32", c.n, &[src.plane(k).arg(), pa.arg(), out.plane(k).arg(), ClArg::U32(n), ClArg::U32(colour_mode)])?;
    }
    if tag == 2 || tag == 4 {
        let p = tag - 1;
        s.launch("pointwise_postdiv_f32", c.n, &[src.plane(0).arg(), pa.arg(), out.plane(p).arg(), ClArg::U32(n), ClArg::U32(2)])?;
    }
    Ok(out)
}

// ---- round, scale_rgb, add_rgb -----------------------------------------------------------------------------------------

fn two_ops(c: &NodeCall, args: &[ArgShape]) -> Result<(usize, usize, bool, bool), &'static str> {
    tag_ok(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let (ni, fa) = count_of(&args[0])?;
    let (no, fb) = count_of(&args[1])?;
    if fa && fb {
        return Err("both operands constant: the CPU kernel reads a plane of a constant");
    }
    Ok((ni, no, fa, fb))
}

fn rgb_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    two_ops(c, args).map(|_| ())
}

/// The pad call and the loop channel count of scale_rgb and add_rgb: (mode, plane index) and nch.
fn rgb_pad(ni: usize, no: usize, tag: usize) -> (Option<(usize, usize)>, usize) {
    let (ni, no) = (ni as i32, no as i32);
    let call = if ni == 4 {
        if no < 4 {
            Some(0u32)
        } else {
            None
        }
    } else if ni == 2 {
        if no < 2 {
            Some(0)
        } else if no == 4 {
            Some(1)
        } else {
            None
        }
    } else if no == 4 {
        if ni < 4 {
            Some(1)
        } else {
            None
        }
    } else if no == 2 && ni < 2 {
        Some(1)
    } else {
        None
    };
    match call {
        Some(0) => (Some((0, (ni - 1) as usize)), (ni - 1) as usize),
        Some(_) => (Some((1, (no - 1) as usize)), (no - 1) as usize),
        None => (None, tag),
    }
}

fn rgb_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], op: u32) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let (ni, no, fa, fb) = two_ops(c, &shapes).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let tag = c.tag as usize;
    let (pad, nch) = rgb_pad(ni, no, tag);
    if nch > 4 {
        return Err(unsup("channel index beyond the output planes"));
    }
    if let Some((mode, idx)) = pad {
        route_check(&shapes, &[(idx, Src::Chan { arg: mode, chan: idx as i32 })]).map_err(unsup)?;
    }
    let out = DevImage::alloc(s, tag, c.n)?;
    let mut written = vec![false; tag];
    let (a, b) = (&args[0], &args[1]);
    for ch in 0..nch {
        if ch >= tag {
            continue;
        }
        written[ch] = true;
        let kin = ch.min(ni - 1);
        let kout = ch.min(no - 1);
        let o = out.plane(ch).arg();
        if fa {
            let pb = plane(b, kout)?;
            s.launch("pointwise_bin_f32", c.n, &[plane(b, kout)?, pb, o, ClArg::U32(n), ClArg::U32(op), ClArg::U32(1), ClArg::U32(lanes(a)[kin]), ClArg::U32(0)])?;
        } else if fb {
            let pa = plane(a, kin)?;
            s.launch("pointwise_bin_f32", c.n, &[plane(a, kin)?, pa, o, ClArg::U32(n), ClArg::U32(op), ClArg::U32(2), ClArg::U32(0), ClArg::U32(lanes(b)[kout])])?;
        } else {
            s.launch("pointwise_bin_f32", c.n, &[plane(a, kin)?, plane(b, kout)?, o, ClArg::U32(n), ClArg::U32(op), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0)])?;
        }
    }
    if let Some((mode, idx)) = pad {
        if idx < tag {
            written[idx] = true;
            let r = [(idx, Src::Chan { arg: mode, chan: idx as i32 })];
            pad_into(s, c, args, &out, &r)?;
        }
    }
    for p in 0..tag {
        if !written[p] {
            zero(s, &out, p, n)?;
        }
    }
    Ok(out)
}

/// Runs plane service recipes into planes of an existing image.
fn pad_into(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], out: &DevImage, recipe: &[(usize, Src)]) -> Result<(), DevError> {
    let n = n_u32(c)?;
    for (p, src) in recipe {
        match *src {
            Src::Fill(v) => fill(s, out, *p, service_fill_bits(v, c.n, c.kp.in_pool), n)?,
            Src::Chan { arg, chan } => match &args[arg] {
                DevArg::Img(img) => {
                    let ch = (chan as usize).min(img.channels());
                    s.launch("copy_f32", c.n, &[img.plane(ch).arg(), out.plane(*p).arg(), ClArg::U32(n)])?;
                }
                DevArg::Const { n: cn, lanes } => {
                    let ch = (chan as usize).min(*cn);
                    fill(s, out, *p, service_fill_bits(lanes[ch], c.n, c.kp.in_pool), n)?;
                }
                DevArg::Mip(_) | DevArg::DevMip(_) => return Err(unsup("a ripmap operand")),
            },
        }
    }
    Ok(())
}

fn rgb_accepts2(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let (ni, no, _, _) = two_ops(c, args)?;
    let (pad, nch) = rgb_pad(ni, no, c.tag as usize);
    if nch > 4 {
        return Err("channel index beyond the output planes");
    }
    if let Some((mode, idx)) = pad {
        route_check(args, &[(idx, Src::Chan { arg: mode, chan: idx as i32 })])?;
    }
    Ok(())
}

fn round_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    two_ops(c, args).map(|_| ())
}

fn round_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let (ni, no, fa, fb) = two_ops(c, &shapes).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let tag = c.tag as usize;
    let r12 = ni == 3 && no == 4;
    let r14 = ni == 4 && no == 3;
    let eff = if r12 || r14 || tag == 3 { 3 } else { tag };
    let out = DevImage::alloc(s, tag, c.n)?;
    let (a, b) = (&args[0], &args[1]);
    let mut written = vec![false; tag];
    for ch in 0..eff {
        let kin = ch.min(ni - 1);
        let kout = ch.min(no - 1);
        if ch >= tag {
            continue;
        }
        written[ch] = true;
        let o = out.plane(ch).arg();
        if fa {
            s.launch("pointwise_round_f32", c.n, &[plane(b, kout)?, plane(b, kout)?, o, ClArg::U32(n), ClArg::U32(0), ClArg::U32(1), ClArg::U32(lanes(a)[kin]), ClArg::U32(0)])?;
        } else if fb {
            s.launch("pointwise_round_f32", c.n, &[plane(a, kin)?, plane(a, kin)?, o, ClArg::U32(n), ClArg::U32(0), ClArg::U32(2), ClArg::U32(0), ClArg::U32(lanes(b)[kout])])?;
        } else {
            s.launch("pointwise_round_f32", c.n, &[plane(a, kin)?, plane(b, kout)?, o, ClArg::U32(n), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0)])?;
        }
    }
    if tag >= 4 {
        if r12 {
            written[3] = true;
            let o = out.plane(3).arg();
            if fb {
                s.launch("pointwise_round_f32", c.n, &[plane(a, 0)?, plane(a, 0)?, o, ClArg::U32(n), ClArg::U32(1), ClArg::U32(2), ClArg::U32(0), ClArg::U32(lanes(b)[3])])?;
            } else {
                s.launch("pointwise_round_f32", c.n, &[plane(b, 3)?, plane(b, 3)?, o, ClArg::U32(n), ClArg::U32(1), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0)])?;
            }
        }
        if r14 {
            written[3] = true;
            let o = out.plane(3).arg();
            if fa {
                s.launch("pointwise_round_f32", c.n, &[plane(b, 0)?, plane(b, 0)?, o, ClArg::U32(n), ClArg::U32(2), ClArg::U32(1), ClArg::U32(lanes(a)[3]), ClArg::U32(0)])?;
            } else {
                s.launch("pointwise_round_f32", c.n, &[plane(a, 3)?, plane(a, 3)?, o, ClArg::U32(n), ClArg::U32(2), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0)])?;
            }
        }
    }
    for p in 0..tag {
        if !written[p] {
            zero(s, &out, p, n)?;
        }
    }
    Ok(out)
}

// ---- colour: sat_scale, hue_shift, hue_shift_preserve_luma ---------------------------------------------------------------

/// Shared plan of the three colour ops: argument 0 is the colour (planes or a constant), argument 1 the scale or shift (a
/// constant or a mask plane). Below three output planes the colour is routed; from three up the kernel computes planes 0..2 and
/// plane 3 is routed.
fn colour_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let tag = tag_ok(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let (cs, s_const) = count_of(&args[0])?;
    let (_, a_const) = count_of(&args[1])?;
    if tag < 3 {
        let r: Vec<(usize, Src)> = (0..tag).map(|i| (i, Src::Chan { arg: 0, chan: i as i32 })).collect();
        return route_check(args, &r);
    }
    if !s_const && cs < 3 {
        return Err("fewer than three source planes");
    }
    if !a_const && !matches!(args[1], ArgShape::Img { channels } if channels >= 1) {
        return Err("mask is not an image");
    }
    if tag == 4 {
        return route_check(args, &[(3, Src::Chan { arg: 0, chan: 3 })]);
    }
    Ok(())
}

fn colour_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], kernel: &str, which: u32) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    colour_accepts(c, &shapes).map_err(unsup)?;
    let tag = c.tag as usize;
    if tag < 3 {
        let r: Vec<(usize, Src)> = (0..tag).map(|i| (i, Src::Chan { arg: 0, chan: i as i32 })).collect();
        return route_run(s, c, args, &r, tag, false);
    }
    check_len(c, args)?;
    let n = n_u32(c)?;
    let out = DevImage::alloc(s, tag, c.n)?;
    let (s_const, a_const) = (is_const(&args[0]), is_const(&args[1]));
    let lc = lanes(&args[0]);
    let shift_w = lanes(&args[1])[0];
    if which == 2 && s_const && a_const {
        // constant colour with a scalar shift: three regions with the vector, quad and scalar bodies of the CPU kernel
        let colour = [f32::from_bits(lc[0]), f32::from_bits(lc[1]), f32::from_bits(lc[2])];
        let shift = f32::from_bits(shift_w);
        let p = hue_shift_preserve_luma::Params { plane_count: 3, flag: false, const_colour: true, colour, scalar_shift: true, shift, n: 15 };
        let mut o = [vec![0.0f32; 15], vec![0.0f32; 15], vec![0.0f32; 15]];
        hue_shift_preserve_luma::render(&p, None, None, &mut o).map_err(|e| unsup(format!("{e:?}")))?;
        let nn = c.n;
        let m8 = if nn >= 8 { nn & !7 } else { 0 };
        let mut regions: Vec<(usize, usize, usize)> = Vec::new();
        if m8 > 0 {
            regions.push((0, m8, 0));
        }
        let mut idx = m8;
        if nn > m8 {
            let rest = nn - m8;
            if rest >= 4 {
                let m4 = rest & !3;
                regions.push((idx, idx + m4, 8));
                idx += m4;
            }
            if idx < nn {
                regions.push((idx, nn, 12));
            }
        }
        for (lo, hi, at) in regions {
            for k in 0..3 {
                s.launch("pointwise_fillr_f32", nn, &[out.plane(k).arg(), ClArg::U32(o[k][at].to_bits()), ClArg::U32(lo as u32), ClArg::U32(hi as u32)])?;
            }
        }
    } else {
        let holder = dummy_holder(s, args)?;
        let dummy = dummy_arg(args, &holder)?;
        let dummy2 = dummy_arg(args, &holder)?;
        let dummy3 = dummy_arg(args, &holder)?;
        let dummy4 = dummy_arg(args, &holder)?;
        let (pr, pg, pb) = if s_const { (dummy, dummy2, dummy3) } else { (plane(&args[0], 0)?, plane(&args[0], 1)?, plane(&args[0], 2)?) };
        let pm = if a_const { dummy4 } else { plane(&args[1], 0)? };
        let flags = u32::from(s_const) | (u32::from(a_const) << 1);
        let mut a: Vec<ClArg> = vec![pr, pg, pb, pm, out.plane(0).arg(), out.plane(1).arg(), out.plane(2).arg(), ClArg::U32(n), ClArg::U32(flags), ClArg::U32(lc[0]), ClArg::U32(lc[1]), ClArg::U32(lc[2]), ClArg::U32(shift_w)];
        if which == 0 {
            // sat_scale: the luma of a constant colour, from the CPU crate
            let l = sat_scale::luma_const([f32::from_bits(lc[0]), f32::from_bits(lc[1]), f32::from_bits(lc[2])]);
            a.push(ClArg::U32(l.to_bits()));
        }
        s.launch(kernel, c.n, &a)?;
    }
    if tag == 4 {
        pad_into(s, c, args, &out, &[(3, Src::Chan { arg: 0, chan: 3 })])?;
    }
    Ok(out)
}

// ---- monochrome and monochrome_a ---------------------------------------------------------------------------------------

fn mono_accepts(c: &NodeCall, args: &[ArgShape], alpha: bool) -> Result<(), &'static str> {
    tag_ok(c)?;
    if args.len() != 1 {
        return Err("one operand expected");
    }
    let (ch, is_c) = count_of(&args[0])?;
    if is_c {
        return Err("constant operand");
    }
    if alpha {
        let want = match ch {
            1 => 1,
            2 => 2,
            3 => 1,
            _ => 2,
        };
        if c.tag != want {
            return Err("selector tag differs from the class row");
        }
    }
    Ok(())
}

fn mono_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], alpha: bool) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    mono_accepts(c, &shapes, alpha).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let ch = count_of(&shapes[0]).map_err(unsup)?.0;
    let DevArg::Img(src) = &args[0] else { return Err(unsup("operand is not an image")) };
    let planes = if alpha && c.tag >= 2 { 2 } else { 1 };
    let out = DevImage::alloc(s, planes, c.n)?;
    if ch <= 2 {
        s.launch("copy_f32", c.n, &[src.plane(0).arg(), out.plane(0).arg(), ClArg::U32(n)])?;
    } else {
        s.launch("pointwise_luma_f32", c.n, &[src.plane(0).arg(), src.plane(1).arg(), src.plane(2).arg(), out.plane(0).arg(), ClArg::U32(n)])?;
    }
    if alpha && (ch == 2 || ch == 4) {
        s.launch("copy_f32", c.n, &[src.plane(ch - 1).arg(), out.plane(1).arg(), ClArg::U32(n)])?;
    }
    Ok(out)
}

// ---- corpus -------------------------------------------------------------------------------------------------------

/// An operand of a self-check case: an image of that many channels or a constant of that many lanes.
#[derive(Clone, Copy, Debug)]
pub enum Od {
    Img(usize),
    Const(usize),
}

const TINY: [u32; 10] = [0x0000_0000, 0x8000_0000, 0x3f80_0000, 0xbf80_0000, 0x4000_0000, 0x3f00_0000, 0x7f80_0000, 0xff80_0000, 0x4040_0000, 0xc000_0000];

/// Values of the unit range and just outside it, where blends, alpha tests and clamps change branch.
const UNIT: [u32; 16] = [
    0x0000_0000, 0x8000_0000, 0x3727_c5ac, 0x3900_0000, 0x3900_0001, 0x38ff_ffff, 0x3e80_0000, 0x3f00_0000, 0x3f40_0000, 0x3f7f_ffff, 0x3f80_0000, 0x3f80_0001, 0x3fc0_0000,
    0x4000_0000, 0xbf00_0000, 0xb727_c5ac,
];

/// Sixteenths from -1/16 to 1.25: colours with many ties between channels.
fn sixteenth(i: usize) -> f32 {
    ((i % 22) as f32 - 1.0) / 16.0
}

fn pick(vals: &[f32], flavor: usize, i: usize) -> f32 {
    match flavor {
        1 => f32::from_bits(TINY[(i ^ (i >> 3)) % TINY.len()]),
        2 => f32::from_bits(UNIT[(i ^ (i >> 4)) % UNIT.len()]),
        3 => sixteenth(i ^ (i >> 2)),
        _ => vals[i % vals.len()],
    }
}

/// A constant operand is encoded as an image with `n == 0`: planes 0..3 hold the four lane words, plane 4 the lane count.
pub fn const_image(n_lanes: usize, lanes: [u32; 4]) -> Image {
    let mut planes: Vec<Vec<f32>> = lanes.iter().map(|b| vec![f32::from_bits(*b)]).collect();
    planes.push(vec![n_lanes as f32]);
    Image { n: 0, planes }
}

fn case_args(ods: &[Od], n: usize, flavor: usize, salt: usize, vals: &[f32]) -> Vec<Image> {
    ods.iter()
        .enumerate()
        .map(|(k, od)| match *od {
            Od::Img(ch) => {
                let planes = (0..ch).map(|p| (0..n).map(|i| pick(vals, flavor, i * 7 + (k * 4 + p) * 13 + n * 3 + salt)).collect()).collect();
                Image { n, planes }
            }
            Od::Const(nl) => {
                let mut l = [0u32; 4];
                for (j, w) in l.iter_mut().enumerate() {
                    *w = pick(vals, flavor, salt * 3 + k * 5 + j * 11 + 1).to_bits();
                }
                const_image(nl, l)
            }
        })
        .collect()
}

/// Cases for one class: every template, tag and frame shape, in a wide-value and a tie-rich flavour, kept only where `accepts`
/// takes them, then thinned evenly to at most `cap` cases. `dims` picks the frame shapes.
pub fn gen_cases(class: &'static str, accepts: Acc, templates: &[Vec<Od>], tags: &[i32], dims: &[(usize, usize)], cap: usize) -> Vec<SelfCase> {
    let vals = adversarial_f32(1500, 77);
    let kp = KParams::standin();
    let mut keys: Vec<(usize, i32, (usize, usize), usize)> = Vec::new();
    for (ti, t) in templates.iter().enumerate() {
        let shapes: Vec<ArgShape> = t.iter().map(|o| match *o { Od::Img(c) => ArgShape::Img { channels: c }, Od::Const(c) => ArgShape::Const { n: c } }).collect();
        for &tag in tags {
            for &(w, h) in dims {
                let call = NodeCall { idx: 0, class, tag, n: w * h, dims: (w, h), kp: &kp };
                if accepts(&call, &shapes).is_err() {
                    continue;
                }
                for flavor in 0..4usize {
                    keys.push((ti, tag, (w, h), flavor));
                }
            }
        }
    }
    let total = keys.len();
    let take: Vec<usize> = if total <= cap { (0..total).collect() } else { (0..cap).map(|k| k * total / cap).collect() };
    take.into_iter()
        .enumerate()
        .map(|(salt, k)| {
            let (ti, tag, (w, h), flavor) = keys[k];
            let t = &templates[ti];
            SelfCase { name: format!("{class} {w}x{h} tag {tag} {t:?} f{flavor}"), dims: (w, h), tag, args: case_args(t, w * h, flavor, salt + 2, &vals), consts: vec![], kp: None }
        })
        .collect()
}

/// A reduced set of frame shapes for the corpora with many operand templates: one pixel, a row below and above 8, an odd frame, and one above 512
/// samples (the pooled fill rule).
pub const DIMS_SMALL: &[(usize, usize)] = &[(1, 1), (7, 1), (33, 2), (45, 23)];

/// Frame shapes of the pointwise corpora: one pixel, a row with a partial vector tail (9), and an odd frame above 512 samples.
pub const PW_DIMS: &[(usize, usize)] = &[(1, 1), (9, 1), (45, 23)];

fn all_templates(nargs: usize, allow_const: bool) -> Vec<Vec<Od>> {
    let mut out: Vec<Vec<Od>> = vec![vec![]];
    for _ in 0..nargs {
        let mut next = Vec::new();
        for t in &out {
            for ch in 1..=4usize {
                let mut a = t.clone();
                a.push(Od::Img(ch));
                next.push(a);
                if allow_const {
                    let mut b = t.clone();
                    b.push(Od::Const(ch));
                    next.push(b);
                }
            }
        }
        out = next;
    }
    out
}

fn bin_corpus_for(class: &'static str, acc: Acc) -> Vec<SelfCase> {
    gen_cases(class, acc, &all_templates(2, true), &[1, 2, 3, 4], PW_DIMS, 300)
}

fn bin_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    bin_plan(c, a).map(|_| ())
}

fn un_corpus_for(class: &'static str) -> Vec<SelfCase> {
    gen_cases(class, un_accepts, &all_templates(1, true), &[1, 2, 3, 4], PW_DIMS, 120)
}

fn clamp_corpus() -> Vec<SelfCase> {
    gen_cases("clamp", clamp_accepts, &all_templates(3, true), &[1, 2, 3, 4], PW_DIMS, 420)
}

fn sel_corpus_for(class: &'static str, acc: Acc, nargs: usize) -> Vec<SelfCase> {
    gen_cases(class, acc, &all_templates(nargs, true), &[1, 2, 3, 4], PW_DIMS, 360)
}

fn vec_corpus(class: &'static str, acc: Acc, k: usize) -> Vec<SelfCase> {
    let mut t = Vec::new();
    for mask in 0..(1usize << k) {
        t.push((0..k).map(|i| if mask >> i & 1 == 1 { Od::Const(1 + i % 4) } else { Od::Img(1 + (i + mask) % 4) }).collect::<Vec<_>>());
    }
    gen_cases(class, acc, &t, &[k as i32, 4], DIMS_SMALL, 200)
}

fn coerce_corpus_for(class: &'static str, nargs: usize) -> Vec<SelfCase> {
    gen_cases(class, coerce_accepts, &all_templates(nargs, true), &[1, 2, 3, 4], DIMS_SMALL, 240)
}

// ---- wide range corpus for the double precision kernels -------------------------------------------------------------

/// A finite f32 whose exponent field is uniform over 0..=254 (subnormals to the largest binade), with a random sign and mantissa.
fn wide_f32(rng: &mut fx_device::sim::Rng) -> f32 {
    let e = rng.below(255) as u32;
    let m = rng.next_u32() & 0x007f_ffff;
    let sign = (rng.next_u32() & 1) << 31;
    f32::from_bits(sign | (e << 23) | m)
}

/// A value of the unit range with a random sign, as pixels hold.
fn pixel_f32(rng: &mut fx_device::sim::Rng) -> f32 {
    let v = (rng.next_u32() >> 8) as f32 / (1u32 << 24) as f32;
    if rng.next_u32() & 1 == 1 { -v } else { v }
}

/// The luma of a pixel in the CPU's order: `mul_add` steps in double precision, rounded once to f32.
fn luma_in_order(r: f32, g: f32, b: f32) -> f32 {
    let t = 0.5 * g as f64;
    let t = (r as f64).mul_add(0.3125, t);
    (b as f64).mul_add(0.1875, t) as f32
}

/// The same sum with the last two steps swapped.
fn luma_swapped(r: f32, g: f32, b: f32) -> f32 {
    let t = 0.5 * g as f64;
    let t = (b as f64).mul_add(0.1875, t);
    (r as f64).mul_add(0.3125, t) as f32
}

/// Triples whose double precision luma changes by one ulp of f32 when the two `mul_add` steps swap: one wide channel with two pixel
/// values, or one pixel channel with two wide ones. Found by a fixed search, so every build sees the same set.
fn luma_sensitive_triples(want: usize) -> Vec<[f32; 3]> {
    let mut rng = fx_device::sim::Rng::new(0x1d5e_a11c);
    let mut out: Vec<[f32; 3]> = vec![[-0.23911922, -0.11966951, -9.35e15]];
    let mut tries = 0usize;
    while out.len() < want && tries < 6_000_000 {
        tries += 1;
        let mut t = [pixel_f32(&mut rng), pixel_f32(&mut rng), pixel_f32(&mut rng)];
        let k = rng.below(3) as usize;
        t[k] = wide_f32(&mut rng);
        if tries % 3 == 0 {
            t[(k + 1) % 3] = wide_f32(&mut rng);
        }
        if t.iter().any(|v| !v.is_finite()) {
            continue;
        }
        if luma_in_order(t[0], t[1], t[2]).to_bits() != luma_swapped(t[0], t[1], t[2]).to_bits() {
            out.push(t);
        }
    }
    out
}

/// Cases for an op with a double precision path: operand values over the whole finite f32 exponent range (subnormal to the largest
/// binade, both signs), and the luma-sensitive triples in the first three channels of the first image operand. These catch an order of
/// the double steps that agrees on pixel-range input but not on wide input.
pub fn wide_cases(class: &'static str, accepts: Acc, templates: &[Vec<Od>], tags: &[i32]) -> Vec<SelfCase> {
    const W: usize = 64;
    const H: usize = 24;
    let n = W * H;
    let kp = KParams::standin();
    let triples = luma_sensitive_triples(n);
    let mut rng = fx_device::sim::Rng::new(0x71de_c0de ^ class.len() as u64);
    let mut out = Vec::new();
    for (ti, t) in templates.iter().enumerate() {
        let shapes: Vec<ArgShape> = t.iter().map(|o| match *o { Od::Img(c) => ArgShape::Img { channels: c }, Od::Const(c) => ArgShape::Const { n: c } }).collect();
        for &tag in tags {
            let call = NodeCall { idx: 0, class, tag, n, dims: (W, H), kp: &kp };
            if accepts(&call, &shapes).is_err() {
                continue;
            }
            for mode in 0..2usize {
                let args: Vec<Image> = t
                    .iter()
                    .enumerate()
                    .map(|(k, od)| match *od {
                        Od::Img(ch) => {
                            let mut planes: Vec<Vec<f32>> = (0..ch).map(|_| (0..n).map(|_| wide_f32(&mut rng)).collect()).collect();
                            if mode == 1 && k == 0 {
                                for (i, tr) in triples.iter().cycle().take(n).enumerate() {
                                    for (p, v) in tr.iter().enumerate().take(ch) {
                                        planes[p][i] = *v;
                                    }
                                }
                            } else if mode == 1 {
                                for pl in planes.iter_mut() {
                                    pl.iter_mut().for_each(|v| *v = pixel_f32(&mut rng));
                                }
                            }
                            Image { n, planes }
                        }
                        Od::Const(nl) => {
                            let mut l = [0u32; 4];
                            for w in l.iter_mut() {
                                *w = if mode == 1 { pixel_f32(&mut rng) } else { wide_f32(&mut rng) }.to_bits();
                            }
                            const_image(nl, l)
                        }
                    })
                    .collect();
                out.push(SelfCase { name: format!("{class} wide {W}x{H} tag {tag} {t:?} m{mode} #{ti}"), dims: (W, H), tag, args, consts: vec![], kp: None });
            }
        }
    }
    out
}

/// The corpus of an op with a double precision path: the general one plus [`wide_cases`].
fn with_wide(mut base: Vec<SelfCase>, class: &'static str, accepts: Acc, templates: &[Vec<Od>], tags: &[i32]) -> Vec<SelfCase> {
    base.extend(wide_cases(class, accepts, templates, tags));
    base
}

// ---- op table ------------------------------------------------------------------------------------------------------

macro_rules! bin_op {
    ($cname:ident, $class:expr, $code:expr) => {
        pub const $cname: DeviceOp = DeviceOp {
            class: $class,
            accepts: bin_accepts,
            run: {
                fn run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
                    bin_run(s, c, a, $code)
                }
                run
            },
            corpus: {
                fn corpus() -> Vec<SelfCase> {
                    bin_corpus_for($class, bin_accepts)
                }
                corpus
            },
        };
    };
}

bin_op!(ADD, "+", 0);
bin_op!(SUB, "-", 1);
bin_op!(MUL, "*", 2);
bin_op!(DIV, "/", 3);
bin_op!(MAX, "max", 4);
bin_op!(MIN, "min", 5);
bin_op!(DIFF, "diff", 6);

macro_rules! un_op {
    ($cname:ident, $class:expr, $code:expr) => {
        pub const $cname: DeviceOp = DeviceOp {
            class: $class,
            accepts: un_accepts,
            run: {
                fn run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
                    un_run(s, c, a, $code)
                }
                run
            },
            corpus: {
                fn corpus() -> Vec<SelfCase> {
                    un_corpus_for($class)
                }
                corpus
            },
        };
    };
}

un_op!(ABS, "abs", 0);
un_op!(NEGATE, "negate", 1);

fn ifplus_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    sel_accepts(c, a, 0)
}
fn ifgt_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    sel_accepts(c, a, 1)
}

pub const IF_PLUS: DeviceOp = DeviceOp {
    class: "if+",
    accepts: ifplus_accepts,
    run: |s, c, a| sel_run(s, c, a, 0),
    corpus: || sel_corpus_for("if+", ifplus_accepts, 3),
};
pub const IF_GT: DeviceOp = DeviceOp {
    class: "if>",
    accepts: ifgt_accepts,
    run: |s, c, a| sel_run(s, c, a, 1),
    corpus: || sel_corpus_for("if>", ifgt_accepts, 4),
};
pub const CLAMP: DeviceOp = DeviceOp { class: "clamp", accepts: clamp_accepts, run: clamp_run, corpus: clamp_corpus };

fn v2_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    vec_accepts(c, a, 2)
}
fn v3_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    vec_accepts(c, a, 3)
}
fn v4_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    vec_accepts(c, a, 4)
}
pub const V2: DeviceOp = DeviceOp { class: "v2", accepts: v2_accepts, run: |s, c, a| vec_run(s, c, a, 2), corpus: || vec_corpus("v2", v2_accepts, 2) };
pub const V3: DeviceOp = DeviceOp { class: "v3", accepts: v3_accepts, run: |s, c, a| vec_run(s, c, a, 3), corpus: || vec_corpus("v3", v3_accepts, 3) };
pub const V4: DeviceOp = DeviceOp { class: "v4", accepts: v4_accepts, run: |s, c, a| vec_run(s, c, a, 4), corpus: || vec_corpus("v4", v4_accepts, 4) };

macro_rules! coerce_op {
    ($cname:ident, $class:expr, $nargs:expr) => {
        pub const $cname: DeviceOp = DeviceOp { class: $class, accepts: coerce_accepts, run: coerce_run, corpus: || coerce_corpus_for($class, $nargs) };
    };
}
coerce_op!(COERCE_UP, "coerce_up", 2);
coerce_op!(COERCE_UP_A1, "coerce_up_a1", 2);
coerce_op!(COERCE_TO, "coerce_to", 2);
coerce_op!(COERCE_TO_ALPHA, "coerce_to_alpha", 1);
coerce_op!(COERCE_TO_V3, "coerce_to_v3", 1);
coerce_op!(COERCE_TO_V4, "coerce_to_v4", 1);
coerce_op!(COERCE_UP_TO_V3, "coerce_up_to_v3", 1);
coerce_op!(REMOVE_ALPHA_OP, "remove_alpha", 1);

pub const V3_TO_V4: DeviceOp = DeviceOp { class: "v3_to_v4", accepts: v34_accepts, run: v34_run, corpus: || gen_cases("v3_to_v4", v34_accepts, &all_templates(2, true), &[4], DIMS_SMALL, 200) };
pub const V3_TO_V4_IF4: DeviceOp = DeviceOp {
    class: "v3_to_v4_if4",
    accepts: v34_accepts,
    run: v34_run,
    corpus: || gen_cases("v3_to_v4_if4", v34_accepts, &all_templates(3, true), &[1, 2, 3, 4], DIMS_SMALL, 240),
};

pub const ELEMENT_OF: DeviceOp = DeviceOp {
    class: "element_of",
    accepts: element_accepts,
    run: element_run,
    corpus: || {
        let mut t = Vec::new();
        for ch in 1..=4usize {
            t.push(vec![Od::Img(ch), Od::Const(4)]);
            t.push(vec![Od::Const(ch), Od::Const(4)]);
            t.push(vec![Od::Img(ch), Od::Const(1)]);
        }
        gen_cases("element_of", element_accepts, &t, &[1, 2, 3, 4], DIMS_SMALL, 240)
    },
};

pub const MAX_ELEMENT: DeviceOp = DeviceOp {
    class: "max_element",
    accepts: reduce_accepts,
    run: |s, c, a| reduce_run(s, c, a, 0),
    corpus: || gen_cases("max_element", reduce_accepts, &all_templates(1, true), &[1], DIMS_SMALL, 80),
};
pub const MIN_ELEMENT: DeviceOp = DeviceOp {
    class: "min_element",
    accepts: reduce_accepts,
    run: |s, c, a| reduce_run(s, c, a, 1),
    corpus: || gen_cases("min_element", reduce_accepts, &all_templates(1, true), &[1], DIMS_SMALL, 80),
};


fn screen_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    bin_plan(c, a).map(|_| ())
}

pub const SCREEN_SUPER: DeviceOp = DeviceOp {
    class: "screen_super",
    accepts: screen_accepts,
    run: |s, c, a| bin_run_k(s, c, a, 0, "pointwise_screen_f32", "pointwise_screen_ext_f32"),
    corpus: || bin_corpus_for("screen_super", screen_accepts),
};

pub const INTERP: DeviceOp = DeviceOp {
    class: "interp",
    accepts: interp_accepts,
    run: interp_run,
    corpus: || gen_cases("interp", interp_accepts, &all_templates(3, true), &[1, 2, 3, 4], PW_DIMS, 400),
};

pub const DISSOLVE: DeviceOp = DeviceOp {
    class: "dissolve",
    accepts: dissolve_accepts,
    run: dissolve_run,
    corpus: || gen_cases("dissolve", dissolve_accepts, &all_templates(3, true), &[1, 2, 3, 4], PW_DIMS, 400),
};

pub const COMP_PREMULT: DeviceOp = DeviceOp {
    class: "comp_premult",
    accepts: prem_accepts,
    run: prem_run,
    corpus: || gen_cases("comp_premult", prem_accepts, &all_templates(3, true), &[1, 2, 3, 4], PW_DIMS, 400),
};

pub const COMP_RGBA: DeviceOp = DeviceOp {
    class: "comp_rgba",
    accepts: comp_accepts,
    run: comp_run,
    corpus: || with_wide(gen_cases("comp_rgba", comp_accepts, &all_templates(2, true), &[1, 2, 3, 4], PW_DIMS, 400), "comp_rgba", comp_accepts, &all_templates(2, true), &[1, 2, 3, 4]),
};

pub const POSTDIVIDE: DeviceOp = DeviceOp {
    class: "postdivide",
    accepts: postdiv_accepts,
    run: postdiv_run,
    corpus: || {
        let t: Vec<Vec<Od>> = (1..=4usize).map(|e| vec![Od::Img(e), Od::Const(1)]).collect();
        let mut cases = gen_cases("postdivide", postdiv_accepts, &t, &[1, 2, 3, 4], PW_DIMS, 200);
        for c in cases.iter_mut() {
            c.args[1] = const_image(1, [(-1.0f32).to_bits(), 0, 0, 0]);
        }
        cases
    },
};

pub const ROUND: DeviceOp = DeviceOp {
    class: "round",
    accepts: round_accepts,
    run: round_run,
    corpus: || with_wide(gen_cases("round", round_accepts, &all_templates(2, true), &[1, 2, 3, 4], PW_DIMS, 300), "round", round_accepts, &all_templates(2, true), &[1, 2, 3, 4]),
};

pub const SCALE_RGB: DeviceOp = DeviceOp {
    class: "scale_rgb",
    accepts: rgb_accepts2,
    run: |s, c, a| rgb_run(s, c, a, 2),
    corpus: || gen_cases("scale_rgb", rgb_accepts2, &all_templates(2, true), &[1, 2, 3, 4], PW_DIMS, 300),
};

pub const ADD_RGB: DeviceOp = DeviceOp {
    class: "add_rgb",
    accepts: rgb_accepts2,
    run: |s, c, a| rgb_run(s, c, a, 0),
    corpus: || gen_cases("add_rgb", rgb_accepts2, &all_templates(2, true), &[1, 2, 3, 4], PW_DIMS, 300),
};

fn colour_templates() -> Vec<Vec<Od>> {
    let mut t = Vec::new();
    for ch in 1..=4usize {
        for s in [Od::Img(ch), Od::Const(ch)] {
            t.push(vec![s, Od::Const(1)]);
            t.push(vec![s, Od::Img(1)]);
        }
    }
    t
}

/// Frame shapes for the colour ops: sizes on both sides of the 8 and 4 sample bodies of the constant case, and one above 512.
pub const COLOUR_DIMS: &[(usize, usize)] = &[(1, 1), (7, 1), (9, 1), (12, 1), (15, 1), (33, 2), (45, 23)];

pub const SAT_SCALE: DeviceOp = DeviceOp {
    class: "sat_scale",
    accepts: colour_accepts,
    run: |s, c, a| colour_run(s, c, a, "pointwise_sat_f32", 0),
    corpus: || with_wide(gen_cases("sat_scale", colour_accepts, &colour_templates(), &[1, 2, 3, 4], COLOUR_DIMS, 400), "sat_scale", colour_accepts, &colour_templates(), &[1, 2, 3, 4]),
};
pub const HUE_SHIFT: DeviceOp = DeviceOp {
    class: "hue_shift",
    accepts: colour_accepts,
    run: |s, c, a| colour_run(s, c, a, "pointwise_hue_f32", 1),
    corpus: || with_wide(gen_cases("hue_shift", colour_accepts, &colour_templates(), &[1, 2, 3, 4], COLOUR_DIMS, 400), "hue_shift", colour_accepts, &colour_templates(), &[1, 2, 3, 4]),
};
pub const HUE_SHIFT_PRESERVE_LUMA: DeviceOp = DeviceOp {
    class: "hue_shift_preserve_luma",
    accepts: colour_accepts,
    run: |s, c, a| colour_run(s, c, a, "pointwise_hsp_f32", 2),
    corpus: || with_wide(gen_cases("hue_shift_preserve_luma", colour_accepts, &colour_templates(), &[1, 2, 3, 4], COLOUR_DIMS, 400), "hue_shift_preserve_luma", colour_accepts, &colour_templates(), &[1, 2, 3, 4]),
};

fn mono_accepts_plain(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    mono_accepts(c, a, false)
}
fn mono_accepts_alpha(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    mono_accepts(c, a, true)
}
pub const MONOCHROME: DeviceOp = DeviceOp {
    class: "monochrome",
    accepts: mono_accepts_plain,
    run: |s, c, a| mono_run(s, c, a, false),
    corpus: || with_wide(gen_cases("monochrome", mono_accepts_plain, &all_templates(1, false), &[1], DIMS_SMALL, 120), "monochrome", mono_accepts_plain, &all_templates(1, false), &[1]),
};
pub const MONOCHROME_A: DeviceOp = DeviceOp {
    class: "monochrome_a",
    accepts: mono_accepts_alpha,
    run: |s, c, a| mono_run(s, c, a, true),
    corpus: || with_wide(gen_cases("monochrome_a", mono_accepts_alpha, &all_templates(1, false), &[1, 2], DIMS_SMALL, 120), "monochrome_a", mono_accepts_alpha, &all_templates(1, false), &[1, 2]),
};

/// The ops of this module (the registration lines of `device_ops.rs` name them one by one).
pub fn ops() -> Vec<&'static DeviceOp> {
    vec![
        &ADD, &SUB, &MUL, &DIV, &MAX, &MIN, &DIFF, &ABS, &NEGATE, &IF_PLUS, &IF_GT, &CLAMP, &V2, &V3, &V4, &COERCE_UP, &COERCE_UP_A1, &COERCE_TO, &COERCE_TO_ALPHA,
        &COERCE_TO_V3, &COERCE_TO_V4, &COERCE_UP_TO_V3, &REMOVE_ALPHA_OP, &V3_TO_V4, &V3_TO_V4_IF4, &ELEMENT_OF, &MAX_ELEMENT, &MIN_ELEMENT,
        &SCREEN_SUPER, &INTERP, &DISSOLVE, &COMP_PREMULT, &COMP_RGBA, &POSTDIVIDE, &ROUND, &SCALE_RGB, &ADD_RGB, &SAT_SCALE, &HUE_SHIFT, &HUE_SHIFT_PRESERVE_LUMA, &MONOCHROME, &MONOCHROME_A,
    ]
}
