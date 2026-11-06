//! executor class for edge_aware_sharpen, slot 52, render flag 0 route.
//!
//! Interface contract (MAPPING.md):
//! - NO DEFAULTS: `Operands`, `CompileCtx` and `Inputs` have no `Default` impl and no constructor that fills a field; the caller states every field.
//! - Input 0 is the Source (nch planes), input 1 is the Mask (MASK-UNBOUND when the mask kind is a plane and input 1 is not bound).
//! - The fourth input plane at nch 3 is ABSENT (the original's table entry 3 is NULL, MEASURED). The arena ids of absent planes are
//!   `NO_PLANE`; any access is a `Tripwire` error and is counted in `Output::plane3_accesses` (the count is zero on a successful return).
//! - Refusals (named): flag != 0 (FLAG-NONZERO), tag < 0 (TAG-NEGATIVE), nch outside 1..=3 (NCH-RANGE), iterations above 7 (ITERS-ABOVE-7).
//! - PORT-LIMIT VARIANT-NCH1: deriv_io lane 0 != 0 at nch 1 reads input plane 1, whose table entry the record does not give; refused (the program literal is 0).
//! - Scratch planes are a HOST stand-in (the original's scratch allocator is not real here); the caller states the fill.
use esh6_driver as d;

/// Mask operand kind: byte [this] == 0 selects the plane (input 1), otherwise the scalar float [this].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaskKind {
    Plane,
    Scalar(f32),
}

/// The ten operand cells of the class (MAPPING.md section 1), all stated by the caller.
#[derive(Debug, Clone, Copy)]
pub struct Operands {
    pub mask: MaskKind,
    pub blur_type: f32,           // [this]
    pub deriv_io: [f32; 2],       // [this], [this]
    pub levels: f32,              // [this]
    pub tiny_detail_size: f32,    // [this]
    pub edge_threshold: f32,      // [this]
    /// [this + 4k], k = 0..6: lanes 0..3 are the operand, lanes 4..6 are the stale entry cells (4.2e-45, 1.4e-45, 0.0 in the record); the caller states all seven.
    pub sigmoid_scale: [f32; 7],
    pub rel_chan: [f32; 3],       // [this + 4k], per-channel gains for nch <= 3
}

/// Compile-context inputs.
#[derive(Debug, Clone, Copy)]
pub struct CompileCtx {
    pub tag: i32,
    pub flag: u8,
    pub nch: i32,
    pub cw: i32,    // ctx[0x20]
    pub w: i32,     // ctx[0x28]
    pub h: i32,     // ctx[0x2c]
    pub kcell: f32, // ctx[0x34]
}

pub struct Inputs<'a> {
    /// input 0: the Source planes, w*h floats each
    pub source: &'a [&'a [f32]],
    /// input 1: the Mask plane, if bound
    pub mask: Option<&'a [f32]>,
}

/// HOST stand-in fill of the scratch planes.
#[derive(Debug, Clone, Copy)]
pub enum Scratch {
    Zero,
    Poison(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecErr {
    FlagNonzero(u8),
    TagNegative(i32),
    NchRange(i32),
    ItersAbove7(i32),
    VariantNch1,
    MaskUnbound,
    Shape(String),
    Tripwire(usize),
    Driver(String),
}

#[derive(Debug)]
pub struct Output {
    pub planes: Vec<Vec<f32>>,
    pub calls: d::Calls,
    /// accesses to index 3 (or any NO_PLANE id); the successful return implies 0
    pub plane3_accesses: u32,
}

pub fn run(cx: &CompileCtx, ops: &Operands, inp: &Inputs, scratch: Scratch) -> Result<Output, ExecErr> {
    if cx.flag != 0 {
        return Err(ExecErr::FlagNonzero(cx.flag));
    }
    if cx.tag < 0 {
        return Err(ExecErr::TagNegative(cx.tag));
    }
    if !(1..=3).contains(&cx.nch) {
        return Err(ExecErr::NchRange(cx.nch));
    }
    let iters = esh3_jobs::cvtt32(ops.levels);
    if iters > 7 {
        return Err(ExecErr::ItersAbove7(iters));
    }
    let nu = cx.nch as usize;
    if cx.w < 1 || cx.h < 1 || cx.cw < 0 {
        return Err(ExecErr::Shape(format!("w {} h {} cw {}", cx.w, cx.h, cx.cw)));
    }
    let wh = (cx.w as usize) * (cx.h as usize);
    if inp.source.len() != nu || inp.source.iter().any(|p| p.len() != wh) {
        return Err(ExecErr::Shape(format!("source planes {} (nch {}), each w*h = {}", inp.source.len(), nu, wh)));
    }
    let variant = esh3_jobs::cvtt32(ops.deriv_io[0]);
    if variant != 0 && nu == 1 {
        return Err(ExecErr::VariantNch1);
    }
    let sig_byte = match ops.mask {
        MaskKind::Scalar(_) => true,
        MaskKind::Plane => {
            match inp.mask {
                None => return Err(ExecErr::MaskUnbound),
                Some(m) if m.len() != wh => return Err(ExecErr::Shape(format!("mask plane len {} != {}", m.len(), wh))),
                Some(_) => {}
            }
            false
        }
    };
    let sigma0 = if let MaskKind::Scalar(s) = ops.mask { s } else { 0.0 };
    // arena: nu sources, then 3n+5 scratch planes (the last is the curve plane = the mask), then nu outputs
    let nt = 3 * nu + 5;
    let mut mem: Vec<f32> = Vec::with_capacity((nu + nt + nu) * wh);
    for p in inp.source {
        mem.extend_from_slice(p);
    }
    let fill = match scratch {
        Scratch::Zero => 0.0f32,
        Scratch::Poison(b) => f32::from_bits(b),
    };
    mem.resize((nu + nt + nu) * wh, fill);
    let curve_off = (nu + 3 * nu + 4) * wh;
    match (ops.mask, inp.mask) {
        (MaskKind::Plane, Some(m)) => mem[curve_off..curve_off + wh].copy_from_slice(m),
        _ => mem[curve_off..curve_off + wh].iter_mut().for_each(|x| *x = 0.0),
    }
    let mut inp_ids = [d::NO_PLANE; 4];
    for (k, id) in inp_ids.iter_mut().enumerate().take(nu) {
        *id = k * wh;
    }
    let ids = d::Ids { inp: inp_ids, t: (0..nt).map(|k| (nu + k) * wh).collect(), out: (0..nu).map(|k| (nu + nt + k) * wh).collect() };
    let mut arr = [0f32; 11];
    arr[..7].copy_from_slice(&ops.sigmoid_scale);
    arr[7..10].copy_from_slice(&ops.rel_chan);
    let th = d::This { n: cx.nch, sig_byte, sigma0, mode: ops.blur_type, variant: ops.deriv_io[0], bcflag: ops.deriv_io[1], iters: ops.levels, scale: ops.tiny_detail_size, output_a: ops.edge_threshold, arr };
    let c = d::Cx { cw: cx.cw, w: cx.w, h: cx.h, kcell: cx.kcell };
    match d::slot52(&th, &c, &ids, &mut mem, cx.flag) {
        Ok(calls) => {
            let planes = (0..nu).map(|k| mem[ids.out[k]..ids.out[k] + wh].to_vec()).collect();
            Ok(Output { planes, calls, plane3_accesses: 0 })
        }
        Err(d::DErr::Tripwire(k)) => Err(ExecErr::Tripwire(k)),
        Err(e) => Err(ExecErr::Driver(format!("{:?}", e))),
    }
}
