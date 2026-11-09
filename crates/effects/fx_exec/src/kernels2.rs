//! Wiring of the builtin classes of `WIRED`, which `kernels::run_class` leaves as `Unsupported{class}`, each called through the crate's own slot-function entry
//! (or the member pass entry for the coerce family) with the explicit parameters the plan binds. Assumption prefix -.
//!
//! Argument-to-record mapping (NV9-A7, INFERRED from the combo tables' lane counts and the record offsets read by each slot
//! function; the programs' argument order is the plan's order): blur_warp (mip, p1 [2], p2 [2], two cells, modes [2],
//! [flag_cell_a], [unused]); warp_rel (mip, coords [2], modes [2], [flag]); blur (src, amount [2], shape, scale [k], subpixel).

use crate::kernels::{block_of, count_i32, flat_roi, kerr, need, pad_fill, unsupported};
use crate::types::{Arg, ExecError, Geom, Image, KParams, MipImage};
use plane_service as ps;

/// Classes wired here.
pub const WIRED: &[&str] = &[
    "make_mipmap", "blur_warp", "warp_rel", "static2", "monochrome", "coerce_up", "coerce_up_a1", "coerce_to", "coerce_to_alpha", "rotate", "max_element",
    "min_element", "blur", "dissolve", "coerce_to_v3", "coerce_to_v4", "coerce_up_to_v3", "v3_to_v4", "sharpen", "monochrome_a",
    "remove_alpha", "edge_aware_sharpen", "round", "scale_rgb", "add_rgb", "masked_blur", "comp_premult", "copy", "copy_alpha", "grad", "perspective", "perspective_alpha", "warp", "postdivide", "glow", "alpha_from_rgb_max", "append", "v3_to_v4_if4_a", "get_alpha",
];

/// Result of one run-primitive: a plain planar image or the ripmap object of make_mipmap.
#[derive(Clone, Debug)]
pub enum Out {
    Img(Image),
    Mip(MipImage),
}

impl Out {
    pub fn channels(&self) -> usize {
        match self {
            Out::Img(i) => i.channels(),
            Out::Mip(m) => m.channels,
        }
    }
    pub fn into_image(self) -> Result<Image, ExecError> {
        match self {
            Out::Img(i) => Ok(i),
            Out::Mip(_) => Err(ExecError::Shape("a ripmap object cannot be a destination or a plain image".into())),
        }
    }
}

pub fn geom_of(kp: &KParams, w: usize, h: usize) -> Geom {
    kp.geom.unwrap_or_else(|| match kp.geom_mode {
        crate::types::GeomMode::Dump => Geom::dump(w, h, kp.host_aspect),
        crate::types::GeomMode::Nv9Old => Geom::full_frame(w, h),
    })
}

/// The default `xy` plane when the caller binds none (NV9B-A4): an explicit geometry keeps the `xy_grid` of it; otherwise Dump
/// binds the pixel-centre identity plane and Nv9Old the `xy_grid` of the full-frame geometry (NV9-GEOM-OLD).
pub fn default_xy(kp: &KParams, w: usize, h: usize) -> Image {
    if kp.xy_base >= 0 {
        // row 0 head = (32 - base) / 4 mod 8; rows follow base + row * W * 4
        return crate::types::xy_ramp_mode(w, h, ((32 - kp.xy_base as usize % 32) % 32) / 4 % 8, kp.xy_head_mode);
    }
    match (kp.geom, kp.geom_mode) {
        (None, crate::types::GeomMode::Dump) if kp.xy_head >= 0 => crate::types::xy_ramp_mode(w, h, kp.xy_head as usize, kp.xy_head_mode),
        (None, crate::types::GeomMode::Dump) => crate::types::xy_pixel_centre(w, h),
        _ => crate::types::xy_grid(w, h, &geom_of(kp, w, h), 0.0),
    }
}

fn const_lane(a: &Arg, i: usize, class: &str, what: &str) -> Result<f32, ExecError> {
    if !a.is_const() {
        return Err(unsupported(class, &format!("{} must be a constant (the record cell is read as a scalar)", what)));
    }
    Ok(a.f32s()[i.min(3)])
}

/// warp_rel's argument 3 is integer-typed; the footprint path is taken when cvttss2si(flag) != 0 (a fractional flag
/// such as 0.5 truncates to 0 and does not take it; NaN gives 0x80000000 and does). The earlier test was the float compare `flag != 0.0`.
pub fn footprint_taken(flag_arg: f32, const_coords: bool) -> bool {
    cvt_i32(flag_arg) != 0 && !const_coords
}

pub fn cvt_i32(x: f32) -> i32 {
    // cvttss2si semantics: truncation, 0x80000000 on overflow or NaN
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

fn image_of<'a>(a: &Arg<'a>, class: &str) -> Result<&'a Image, ExecError> {
    match a {
        Arg::Img(i) => Ok(i),
        _ => Err(unsupported(class, "the image operand is not a plain planar image")),
    }
}

/// NV9-A8: a constant operand of a class that reads planes is materialised as a broadcast image (the original's `constant` command
/// allocates an image-kind object holding the lanes); a ripmap object is an error.
fn image_or_const(a: &Arg, n: usize, class: &str) -> Result<Image, ExecError> {
    match a {
        Arg::Img(i) => Ok((*i).clone()),
        Arg::Const { n: ln, lanes } => Ok(Image::from_const(lanes, (*ln).max(1), n)),
        Arg::Mip(_) => Err(unsupported(class, "a ripmap object is not a plain planar image")),
    }
}

fn mip_of<'a>(a: &Arg<'a>, class: &str, w: usize, h: usize) -> Result<&'a MipImage, ExecError> {
    match a {
        Arg::Mip(m) => {
            if m.width != w || m.height != h {
                return Err(ExecError::Shape(format!("{}: ripmap built for {}x{}, frame is {}x{}", class, m.width, m.height, w, h)));
            }
            Ok(m)
        }
        _ => Err(unsupported(class, "the first argument must be the make_mipmap result")),
    }
}

/// make_mipmap: slot 33 (alloc, ripmap), slot 32 once per base row, slot 34 (pyramid) on one object (R-PARITY sequence, MM2-A1).
fn make_mipmap_k(args: &[Arg], w: usize, h: usize, kp: &KParams) -> Result<MipImage, ExecError> {
    need(args, 3, "make_mipmap")?;
    // the operand may be an image-kind object filled by `constant` (g06: `(constant [ 0 ] :ref88.2)` then make_mipmap on it,
    // NV9-A8 broadcast); a plain image is used as is, a ripmap object is still refused.
    let src_owned = image_or_const(&args[0], w * h, "make_mipmap")?;
    let src = &src_owned;
    let c = src.channels();
    if !(1..=4).contains(&c) {
        return Err(ExecError::Shape(format!("make_mipmap: {} channels (contract 1..4)", c)));
    }
    if src.n != w * h {
        return Err(ExecError::Shape("make_mipmap: source size differs from the frame".into()));
    }
    let wx = cvt_i32(const_lane(&args[1], 0, "make_mipmap", "wrap")?);
    let wy = cvt_i32(const_lane(&args[1], 1, "make_mipmap", "wrap")?);
    let filter = cvt_i32(const_lane(&args[2], 0, "make_mipmap", "filter")?);
    let o = make_mipmap::mm2::ObjParams { w: w as i32, h: h as i32, c, d0: 1 };
    let (alloc, mut rm) = make_mipmap::mm2::slot33(&o, filter, 0, make_mipmap::mm2::Mut2::None).map_err(|e| kerr("make_mipmap", e))?;
    let pad: Vec<u32> = (0..c).map(|k| kp.pad_pixel[k].to_bits()).collect();
    for y in 0..h {
        let planes: Vec<Vec<u32>> = (0..c).map(|k| src.planes[k][y * w..(y + 1) * w].iter().map(|v| v.to_bits()).collect()).collect();
        let sr = make_mipmap::mm2::SrcRow { format: 2, c, planes };
        let rr = make_mipmap::mm2::RecRow { roi_x: 0, x_cell_a: 0, y_cell_a: y as i32, y_cell_b: 0 };
        make_mipmap::mm2::base_fill(&mut rm, &sr, &rr, wx, &pad, make_mipmap::mm2::Mut2::None).map_err(|e| kerr("make_mipmap", e))?;
    }
    make_mipmap::mm2::slot34(&mut rm, wx, wy, filter, &pad).map_err(|e| kerr("make_mipmap", e))?;
    Ok(MipImage { width: w, height: h, channels: c, filter, wrap: [wx, wy], ripmap: rm, scale: alloc.scale.clone(), base: src.clone() })
}

fn endpoint(a: &Arg, class: &str) -> Result<(bool, Vec<f32>, Vec<f32>), ExecError> {
    match a {
        Arg::Const { .. } => {
            let f = a.f32s();
            Ok((true, vec![f[0]], vec![f[1]]))
        }
        Arg::Img(i) if i.channels() == 2 => Ok((false, i.planes[0].clone(), i.planes[1].clone())),
        _ => Err(unsupported(class, "endpoint operand must be a 2 lane constant or a 2 channel image")),
    }
}

/// blur_warp: slot 52 with flag 0 on the whole frame as one call (count = w * h: the bodies read x and y from the
/// endpoint arrays only, no per-call index enters S2/S3, so the chunking cannot change a result; INFERRED from the S2/S3 formulas).
fn blur_warp_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 8, "blur_warp")?;
    let mip = mip_of(&args[0], "blur_warp", w, h)?;
    let g = geom_of(kp, w, h);
    let (p1c, p1x, p1y) = endpoint(&args[1], "blur_warp")?;
    let (p2c, p2x, p2y) = endpoint(&args[2], "blur_warp")?;
    let f_cc = const_lane(&args[3], 0, "blur_warp", "argument 3")?;
    let assumption_a = const_lane(&args[3], 1, "blur_warp", "argument 3")?;
    let assumption_b = const_lane(&args[4], 0, "blur_warp", "scale_result")?;
    let mode_x = cvt_i32(const_lane(&args[5], 0, "blur_warp", "wrap")?);
    let mode_y = cvt_i32(const_lane(&args[5], 1, "blur_warp", "wrap")?);
    let flag_cell_a = cvt_i32(const_lane(&args[6], 0, "blur_warp", "subpixel")?);
    let _blur_res = const_lane(&args[7], 0, "blur_warp", "blur_res")?;
    let c = mip.channels;
    if c as i32 != tag {
        return Err(ExecError::Shape(format!("blur_warp: tag {} but the ripmap has {} channels", tag, c)));
    }
    let n = w * h;
    let plane00: Vec<f32> = mip.ripmap.planes[0].iter().map(|b| f32::from_bits(*b)).collect();
    let img = blur_warp::ImageObj { width: w as i32, height: h as i32, n: c as i32, w0: mip.ripmap.widths[0] as i32, h0: mip.ripmap.heights[0] as i32, plane00: std::sync::Arc::new(plane00) };
    // the original worker (E1 route, band cell 1) calls the body once per line with count = W. kp.bw_whole_frame (env
    // NV10_BW_WHOLE=1) keeps one call with count = W * H; the vector head/tail split then falls at other columns.
    let whole = std::env::var("NV10_BW_WHOLE").map(|v| v == "1").unwrap_or(false);
    let mut out: Vec<Vec<f32>> = vec![vec![0.0; n]; c];
    let (calls, cnt) = if whole { (1usize, n) } else { (h, w) };
    for r in 0..calls {
        let lo = r * cnt;
        let sl = |v: &Vec<f32>, k: bool| if k { v.clone() } else { v[lo..lo + cnt].to_vec() };
        let this = blur_warp::ThisObj {
            p1_const: p1c as u8,
            p1x: sl(&p1x, p1c),
            p1y: sl(&p1y, p1c),
            p2_const: p2c as u8,
            p2x: sl(&p2x, p2c),
            p2y: sl(&p2y, p2c),
            f_cc,
            assumption_a,
            assumption_b,
            mode_x,
            mode_y,
            flag_cell_a,
        };
        let rec = blur_warp::Rec { count: cnt as i32, aspect_block_d: g.aspect, assumption_e: g.assumption_e, assumption_c: g.roi_x, assumption_d: g.roi_y, assumption_f: g.left, assumption_g: g.right, assumption_h: g.assumption_h };
        let mut o: Vec<Vec<f32>> = vec![vec![0.0; cnt]; c];
        match blur_warp::render(&this, &rec, &img, 0, &mut o) {
            Ok(blur_warp::Outcome::Body { .. }) => {}
            Ok(x) => return Err(kerr("blur_warp", format!("{:?}", x))),
            Err(e) => return Err(kerr("blur_warp", e)),
        }
        for k in 0..c {
            out[k][lo..lo + cnt].copy_from_slice(&o[k]);
        }
    }
    Ok(Image { n, planes: out })
}

/// The slot 33 geometry record of `warp` and `warp_rel` (gate 0) for a `w` by `h` frame, defined only at host aspect 1.0.
pub(crate) fn slot33_rec(w: usize, h: usize) -> warp_log_gradient::Rec33 {
    let (assumption_f, assumption_g, assumption_h) = (-0.5f32, w as f32 - 0.5, (h as f32 - w as f32) / 2.0 - 0.5);
    let geom33_a = assumption_h + w as f32;
    warp_log_gradient::Rec33 { gate: 0, w: w as i32, h: h as i32, assumption_f: assumption_f.to_bits(), assumption_g: assumption_g.to_bits(), assumption_h: assumption_h.to_bits(), geom33_a: geom33_a.to_bits() }
}

/// The per row record of the `warp` driver for row `y` of a `w` wide frame.
pub(crate) fn warp_row_rec(g: &Geom, w: usize, y: usize) -> warp_render::WarpRec {
    warp_render::WarpRec { count: w as i32, aspect_block_d: g.aspect, assumption_e: g.assumption_e, idx: (y * w) as i32, assumption_c: g.roi_x, assumption_d: g.roi_y, assumption_f: g.left, assumption_g: g.right, assumption_h: g.assumption_h }
}

/// The per row frame of the `warp_rel` driver for row `y` of a `w` wide frame with `c` channels.
pub(crate) fn warp_rel_row(g: &Geom, w: usize, y: usize, c: usize) -> warp_rel::Frame {
    let obj = warp_rel::Obj {
        count: w as i32,
        aspect_block_d: g.aspect,
        sub_x: 0,
        sub_y: 0,
        port_obj_a: y as i32 + g.roi_y,
        port_obj_b: g.roi_x,
        assumption_c: g.roi_x,
        assumption_d: g.roi_y,
        assumption_f: g.left,
        assumption_g: g.right,
        fill: vec![0.0; c],
    };
    warp_rel::Frame { obj, idx: (y * w) as i32 }
}

/// warp_rel: slot 53 (wrapper over F2) once per row, flag 0; the footprint path takes c1/c2 from slot 33 (warp_log_gradient crate).
fn warp_rel_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 4, "warp_rel")?;
    let mip = mip_of(&args[0], "warp_rel", w, h)?;
    let g = geom_of(kp, w, h);
    let mx = cvt_i32(const_lane(&args[2], 0, "warp_rel", "wrap")?);
    let my = cvt_i32(const_lane(&args[2], 1, "warp_rel", "wrap")?);
    let flag_arg = const_lane(&args[3], 0, "warp_rel", "argument 3")?;
    let (const_coords, ca1, ca2, a1, a2) = match &args[1] {
        Arg::Const { .. } => {
            let f = args[1].f32s();
            (true, f[0], f[1], Vec::new(), Vec::new())
        }
        Arg::Img(i) if i.channels() == 2 => (false, 0.0, 0.0, i.planes[0].clone(), i.planes[1].clone()),
        _ => return Err(unsupported("warp_rel", "coordinate operand must be a 2 lane constant or a 2 channel image")),
    };
    // warp_rel's vtable slot 33 is the same function as class warp's (vtables/class_vtable_pairs.tsv), so the recorded slot33_warp serves
    // the footprint path here too (argument 3 nonzero with image coordinates). Same labels as warp_k: ([P] = the flag lane, INFERRED), (ctx STAND-IN).
    let n = w * h;
    let mut c1_arr = vec![0.0f32; n];
    let mut c2_arr = vec![0.0f32; n];
    let mut flag_byte = 0u8;
    if footprint_taken(flag_arg, const_coords) {
        let g33 = geom_of(kp, w, h);
        let bits = |v: &Vec<f32>| v.iter().map(|x| x.to_bits()).collect::<Vec<u32>>();
        let (ub, vb) = (bits(&a1), bits(&a2));
        let mut c1b = vec![warp_log_gradient::SENT; n];
        let mut c2b = vec![warp_log_gradient::SENT; n];
        let prim33 = warp_log_gradient::Prim33 { n: cvt_i32(flag_arg), cc: 0 };
        let rec33 = slot33_rec(w, h); if kp.host_aspect != 1.0 { return Err(unsupported("warp_rel", "slot 33 geometry cells ctx measured only at host aspect 1.0")); } // A10: the measured cells replace the withdrawn stand-in
        let mut pool = warp_log_gradient::Pool::default();
        let th = warp_log_gradient::Threads { nthr: 1, nthr2: 1, tid: 0 };
        let mut ev = Vec::new();
        let o = warp_log_gradient::slot33_warp(warp_log_gradient::FW_AVX2, &prim33, Some(&rec33), None, (&ub, &vb), &mut c1b, &mut c2b, &mut pool, &th, &mut ev).map_err(|e| kerr("warp_rel slot 33", e))?;
        flag_byte = o.flag_byte;
        c1_arr = c1b.iter().map(|b| f32::from_bits(*b)).collect();
        c2_arr = c2b.iter().map(|b| f32::from_bits(*b)).collect();
    }
    let c = mip.channels;
    if c as i32 != tag {
        return Err(ExecError::Shape(format!("warp_rel: tag {} but the ripmap has {} channels", tag, c)));
    }
    let prim = warp_rel::Prim { const_coords, const_a1: ca1, const_a2: ca2, a1_arr: a1, a2_arr: a2, c1_arr, c2_arr, flag: flag_byte, mx, my };
    let nl = mip.ripmap.nl();
    let np = mip.ripmap.np();
    let f1 = warp_sampler::Params {
        n: c as i32,
        width_tab: mip.ripmap.widths.iter().map(|&v| v as i32).collect(),
        height_tab: mip.ripmap.heights.iter().map(|&v| v as i32).collect(),
        t: mip.scale.as_ref().map(|s| s.iter().map(|b| f32::from_bits(*b)).collect()).unwrap_or_default(),
        flag_x: 0,
        flag_y: 0,
    };
    let (nl_i, np_i) = (nl as i32, np as i32);
    let (w_i, h_i) = (w as i32, h as i32);
    let mk_f1 = || warp_sampler::Params { n: f1.n, width_tab: f1.width_tab.clone(), height_tab: f1.height_tab.clone(), t: f1.t.clone(), flag_x: 0, flag_y: 0 };
    let lat = warp_sampler::Lattice {
        bufs: (0..nl).map(|l| (0..np).map(|j| mip.ripmap.planes[mip.ripmap.idx(l, j)].iter().map(|b| f32::from_bits(*b)).collect()).collect()).collect(),
    };
    let nt = crate::par::threads_for(h, n);
    let parts = crate::par::row_bands(h, nt, |lo, hi| {
        let mut wimg = warp_rel::Image { f1: mk_f1(), w: w_i, h: h_i, d_cell_d: nl_i, d_cell_e: np_i, host: warp_rel::Host::Mode0 };
        let mut band: Vec<Vec<f32>> = vec![vec![0.0; (hi - lo) * w]; c];
        for y in lo..hi {
            let fr = warp_rel_row(&g, w, y, c);
            let mut row: Vec<Vec<f32>> = vec![vec![0.0; w]; c];
            warp_rel::render(&prim, &fr, &mut wimg, &lat, &mut row).map_err(|e| kerr("warp_rel", e))?;
            for k in 0..c {
                band[k][(y - lo) * w..(y - lo + 1) * w].copy_from_slice(&row[k]);
            }
        }
        Ok(band)
    })?;
    Ok(Image { n, planes: crate::par::join_planes(parts, c, n) })
}

fn static2_k(args: &[Arg], n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 0, "static2")?;
    let t = static2::table();
    let (a, b) = static2::render_planes(&t, &static2::Params { n: n as i32, seed_a: kp.static2_seed_a, seed_b: 0 });
    Ok(Image { n, planes: vec![a, b] })
}

fn monochrome_k(args: &[Arg], n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 1, "monochrome")?;
    let src_owned = image_or_const(&args[0], n, "monochrome")?;
    let src = &src_owned;
    let c = src.channels();
    let p = monochrome::Params { channels: c as i32, n: n as i32, dst_phase: 0 };
    let s3 = if c >= 3 { Some([src.planes[0].as_slice(), src.planes[1].as_slice(), src.planes[2].as_slice()]) } else { None };
    let mut out = vec![0.0f32; n];
    let calls = monochrome::render(&p, s3, &mut out).map_err(|e| kerr("monochrome", e))?;
    for cl in calls {
        match cl {
            monochrome::Call::Fill => pad_fill(kp, &args[0], 0, &mut out, n)?,
        }
    }
    Ok(Image { n, planes: vec![out] })
}

fn coerce_k(class: &str, args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    let cl = match class {
        "coerce_up" => ops_coerce::Class::CoerceUp,
        "coerce_up_a1" => ops_coerce::Class::CoerceUpA1,
        "coerce_to" => ops_coerce::Class::CoerceTo,
        _ => ops_coerce::Class::CoerceToAlpha,
    };
    if args.is_empty() {
        return Err(ExecError::BadPlan(format!("{} without an argument", class)));
    }
    let counts: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let combo = ops_coerce::select(cl, &counts).ok_or(ExecError::NoCombo { class: class.into(), operands: counts.clone() })?;
    if combo.tag as i32 != tag {
        return Err(ExecError::Kernel { class: class.into(), msg: format!("selector tag {} differs from the crate's combo tag {}", tag, combo.tag) });
    }
    let planes = args[0].planes();
    let vals = args[0].bits().to_vec();
    let blk = [block_of(&args[0], &planes, &vals)];
    let p = ops_coerce::Params { pass: combo.pass, tag: combo.tag, host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; (tag.max(0) as usize).max(4)];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        ops_coerce::render(&p, &blk, &mut outs).map_err(|e| kerr(class, e))?;
    }
    outv.truncate(tag.max(0) as usize);
    Ok(Image { n, planes: outv })
}


/// IN4A-A1: combo rows of coerce_to_v3, coerce_to_v4 and coerce_up_to_v3 (audit_evt COMBOS.tsv, MEASURED): (operand channel count,
/// result count, member pass). The passes are the finished passes (R-PARITY at the pass entry), the q0 pointers of the rows are
///  D_CELL_A, DE_CELL_A, C_CELL_A, E_CELL_A, EF_CELL_A (image base 0x10000000).
const TO_V3: [(i32, i32, ops_coerce::Pass); 4] =
    [(1, 3, ops_coerce::Pass::D_CELL_A), (2, 3, ops_coerce::Pass::DE_CELL_A), (3, 3, ops_coerce::Pass::C_CELL_A), (4, 3, ops_coerce::Pass::C_CELL_A)];
const TO_V4: [(i32, i32, ops_coerce::Pass); 3] = [(1, 4, ops_coerce::Pass::E_CELL_A), (3, 4, ops_coerce::Pass::EF_CELL_A), (4, 4, ops_coerce::Pass::C_CELL_A)];
const UP_TO_V3: [(i32, i32, ops_coerce::Pass); 4] =
    [(1, 3, ops_coerce::Pass::D_CELL_A), (2, 3, ops_coerce::Pass::DE_CELL_A), (3, 3, ops_coerce::Pass::C_CELL_A), (4, 4, ops_coerce::Pass::C_CELL_A)];

/// IN7-A1: combo rows of remove_alpha (COMBOS.tsv, MEASURED): operand channel counts 1 2 3 4 give result counts 1 1 3 3, and all four rows
/// carry q0 = = pass C_CELL_A of (out[c] = argument 0 channel c for c < tag).
const REMOVE_ALPHA: [(i32, i32, ops_coerce::Pass); 4] =
    [(1, 1, ops_coerce::Pass::C_CELL_A), (2, 1, ops_coerce::Pass::C_CELL_A), (3, 3, ops_coerce::Pass::C_CELL_A), (4, 3, ops_coerce::Pass::C_CELL_A)];

/// IN4A-W1: coerce_to_v3 / coerce_to_v4 / coerce_up_to_v3 (and remove_alpha) through the member passes with the classes' own combo rows.
fn coerce_ext_k(class: &str, args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    if args.is_empty() {
        return Err(ExecError::BadPlan(format!("{} without an argument", class)));
    }
    let table: &[(i32, i32, ops_coerce::Pass)] = match class {
        "coerce_to_v3" => &TO_V3,
        "coerce_to_v4" => &TO_V4,
        "remove_alpha" => &REMOVE_ALPHA,
        _ => &UP_TO_V3,
    };
    let counts: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let row = if counts.len() == 1 { table.iter().find(|r| r.0 == counts[0]) } else { None };
    let (_, rtag, pass) = *row.ok_or(ExecError::NoCombo { class: class.into(), operands: counts.clone() })?;
    if rtag != tag {
        return Err(ExecError::Kernel { class: class.into(), msg: format!("selector tag {} differs from the class row tag {}", tag, rtag) });
    }
    let planes = args[0].planes();
    let vals = args[0].bits().to_vec();
    let blk = [block_of(&args[0], &planes, &vals)];
    let p = ops_coerce::Params { pass, tag: rtag as usize, host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; (tag.max(0) as usize).max(4)];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        ops_coerce::render(&p, &blk, &mut outs).map_err(|e| kerr(class, e))?;
    }
    outv.truncate(tag.max(0) as usize);
    Ok(Image { n, planes: outv })
}

/// IN4A-W2: v3_to_v4(value, alpha). All six combo rows (COMBOS.tsv: (3,1) (3,3) (4,1) (4,3) (1,1) (1,3), result 4) carry the member
/// pass, the same function as pass A of v3_to_v4_if4 (, R-PARITY at the pass entry): planes 0..2 = argument 0 channel
/// min(c, count - 1), plane 3 = argument 1 channel 0. Combo index 0 of the v3_to_v4_if4 crate is pass A with 4 results.
fn v3_to_v4_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "v3_to_v4")?;
    let counts = [args[0].count(), args[1].count()];
    const ROWS: [[i32; 2]; 6] = [[3, 1], [3, 3], [4, 1], [4, 3], [1, 1], [1, 3]];
    if !ROWS.contains(&counts) {
        return Err(ExecError::NoCombo { class: "v3_to_v4".into(), operands: counts.to_vec() });
    }
    if tag != 4 {
        return Err(ExecError::Kernel { class: "v3_to_v4".into(), msg: format!("selector tag {} differs from the class row tag 4", tag) });
    }
    let (p0, p1) = (args[0].planes(), args[1].planes());
    let (v0, v1) = (args[0].bits().to_vec(), args[1].bits().to_vec());
    let blocks = [block_of(&args[0], &p0, &v0), block_of(&args[1], &p1, &v1)];
    let p = v3_to_v4_if4::Params { combo: 0, host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; 4];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        v3_to_v4_if4::render(&p, &blocks, &mut outs).map_err(|e| kerr("v3_to_v4", e))?;
    }
    Ok(Image { n, planes: outv })
}

/// IN4A-W3: sharpen(src, gains, amount) through the crate `sharpen` (render flag 0). Argument mapping INFERRED from the
/// record offsets the driver reads (source record this, gains this, amount this) and the plan text
/// `sharpen :src [ 2 2 2 ] [ 0.112 ]`; combos (1|2|3|4,1,1) and (3,3,1) all have result count = source channels. The radius uses the
/// frame width as ctx[0x20] (the same host field the blur wiring binds).
fn sharpen_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 3, "sharpen")?;
    let src_owned = image_or_const(&args[0], w * h, "sharpen")?;
    let src = &src_owned;
    let c = src.channels();
    if c != tag.max(0) as usize || src.n != w * h {
        return Err(ExecError::Shape(format!("sharpen: source has {} channels, combo tag {} (the crate needs equal counts)", c, tag)));
    }
    if !args[1].is_const() || !args[2].is_const() {
        return Err(unsupported("sharpen", "gains and amount must be constants (record cells read as scalars)"));
    }
    let sp = sharpen::Params {
        amount: args[2].f32s()[0],
        gains: args[1].f32s().to_vec(),
        in_count: c,
        ctx_width: if kp.blur_render_scale > 0 { kp.blur_render_scale } else { w as i32 },
        aspect: None,
        count: Some(kp.sharpen_count.ok_or_else(|| ExecError::Shape("sharpen: the host element count (ctx) must be given (HOST-COUNT, no default)".into()))?),
    };
    let planes: Vec<fx_common::plane::Plane> = (0..c).map(|k| fx_common::plane::Plane::new(&src.planes[k], w, h, w)).collect::<Result<_, _>>().map_err(|e| kerr("sharpen", e))?;
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; w * h]; c];
    {
        let mut dp: Vec<fx_common::plane::PlaneMut> = Vec::new();
        for v in outv.iter_mut() {
            dp.push(fx_common::plane::PlaneMut::new(v.as_mut_slice(), w, h, w).map_err(|e| kerr("sharpen", e))?);
        }
        sharpen::render_alias(&sp, &planes, &mut dp, kp.sharpen_alias).map_err(|e| kerr("sharpen", e))?;
    }
    Ok(Image { n: w * h, planes: outv })
}


/// edge_aware_sharpen slot 52 (flag 0 route) through `edge_aware_sharpen::run`. Argument order:
/// 0 Source, 1 Mask (image = plane, constant = scalar), 2 blur_type, 3 deriv_io (2 lanes), 4 levels, 5 tiny_detail_size,
/// 6 edge_threshold, 7 sigmoid_scale (4 lanes), 8 rel_chan (3 lanes), 9 show_1 (unused by the slot driver). No default fills a missing
/// cell. Compile context: tag = combo tag = nch; flag 0 (the verified render path, ESM1-A1); ctx[0x20] = frame width (as the blur wiring);
/// the ctx width and height cells = w, h; the ctx aspect cell = host aspect cell (kp.host_aspect, 1.0 INFERRED, ESM1-A2). The sigmoid lanes 4..6 are the stale
/// entry cells of the record (4.2e-45, 1.4e-45, 0.0), read only by iteration counts above 4 (ESM1-A3).
fn edge_aware_sharpen_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    use edge_aware_sharpen as e;
    need(args, 10, "edge_aware_sharpen")?;
    let src = image_or_const(&args[0], w * h, "edge_aware_sharpen")?;
    let c = "edge_aware_sharpen";
    let lane = |i: usize, l: usize, what: &str| const_lane(&args[i], l, c, what);
    let mask_img: Option<Vec<f32>>;
    let mask = match &args[1] {
        Arg::Img(im) => {
            if im.channels() != 1 {
                return Err(ExecError::Shape(format!("edge_aware_sharpen: mask has {} channels", im.channels())));
            }
            mask_img = Some(im.planes[0].clone());
            e::MaskKind::Plane
        }
        Arg::Const { .. } => {
            mask_img = None;
            e::MaskKind::Scalar(lane(1, 0, "mask")?)
        }
        Arg::Mip(_) => return Err(unsupported(c, "a ripmap object is not a mask")),
    };
    let ops = e::Operands {
        mask,
        blur_type: lane(2, 0, "blur_type")?,
        deriv_io: [lane(3, 0, "deriv_io")?, lane(3, 1, "deriv_io")?],
        levels: lane(4, 0, "levels")?,
        tiny_detail_size: lane(5, 0, "tiny_detail_size")?,
        edge_threshold: lane(6, 0, "edge_threshold")?,
        sigmoid_scale: [lane(7, 0, "sigmoid_scale")?, lane(7, 1, "sigmoid_scale")?, lane(7, 2, "sigmoid_scale")?, lane(7, 3, "sigmoid_scale")?, f32::from_bits(3), f32::from_bits(1), 0.0],
        rel_chan: [lane(8, 0, "rel_chan")?, lane(8, 1, "rel_chan")?, lane(8, 2, "rel_chan")?],
    };
    let nch = src.channels() as i32;
    if nch != tag {
        return Err(ExecError::Shape(format!("edge_aware_sharpen: source has {} channels, combo tag {}", nch, tag)));
    }
    let cx = e::CompileCtx { tag, flag: 0, nch, cw: if kp.blur_render_scale > 0 { kp.blur_render_scale } else { w as i32 }, w: w as i32, h: h as i32, kcell: kp.host_aspect };
    let srefs: Vec<&[f32]> = src.planes.iter().map(|p| p.as_slice()).collect();
    let inp = e::Inputs { source: &srefs, mask: mask_img.as_deref() };
    match e::run(&cx, &ops, &inp, e::Scratch::Zero) {
        Ok(o) => Ok(Image { n: w * h, planes: o.planes }),
        Err(er) => {
            let name = match &er {
                e::ExecErr::MaskUnbound => "MASK-UNBOUND",
                e::ExecErr::FlagNonzero(_) => "ERROR-PATH FLAG-NONZERO",
                e::ExecErr::TagNegative(_) => "ERROR-PATH TAG-NEGATIVE",
                e::ExecErr::NchRange(_) => "NCH-RANGE",
                e::ExecErr::ItersAbove7(_) => "ERROR-PATH ITERS-ABOVE-7",
                e::ExecErr::VariantNch1 => "PORT-LIMIT VARIANT-NCH1",
                _ => "EXEC",
            };
            Err(unsupported(c, &format!("{}: {:?}", name, er)))
        }
    }
}

/// IN4A-W4: monochrome_a (slot 53, AVX2 body, crate `monochrome_a`K-PARITY and R-PARITY of IN4A): luma plane out0 for 3 or
/// more channels, the pad-plane fill of channel 0 for fewer, and the alpha plane out1 = channel (count - 1) for 2 and 4 channels. Every plane
/// service call the port reports (channel, output plane) is run through the finished `plane_service` crate. Weight cells MEASURED-DYLIB.
fn monochrome_a_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 1, "monochrome_a")?;
    let src_owned = image_or_const(&args[0], n, "monochrome_a")?;
    let src = &src_owned;
    let c = src.channels();
    let want = monochrome_a::COMBOS.iter().find(|r| r.0 == c as i32).map(|r| r.1);
    if want != Some(tag) {
        return Err(ExecError::Kernel { class: "monochrome_a".into(), msg: format!("selector tag {} differs from the class row for {} channels ({:?})", tag, c, want) });
    }
    let p = monochrome_a::Params {
        channels: c as i32,
        n: n as i32,
        dst_phase: 0,
        scalar: monochrome_a::Weights { p0: 0.3125, p1: 0.5, p2: 0.1875 },
        vec_w: [[0.3125; 4], [0.5; 4], [0.1875; 4]],
    };
    let s3 = if c >= 3 { Some([src.planes[0].as_slice(), src.planes[1].as_slice(), src.planes[2].as_slice()]) } else { None };
    let mut out0 = vec![0.0f32; n];
    let mut out1 = vec![0.0f32; n];
    let calls = monochrome_a::render(&p, s3, &mut out0).map_err(|e| kerr("monochrome_a", e))?;
    for cl in calls {
        let dst = if cl.out == 0 { &mut out0 } else { &mut out1 };
        pad_fill(kp, &args[0], cl.chan, dst, n)?;
    }
    let planes = if tag >= 2 { vec![out0, out1] } else { vec![out0] };
    Ok(Image { n, planes })
}

/// class `grad` (slot 52 flag 0, crate `grad` of, R-PARITY at floor). Operands (program order, INFERRED from the
/// Distort_RGB text `(grad (monochrome Lens) blur_lens scale 1)` and field map): argument 0 = the source plane (one channel, combo row
/// (1,1,1,1)), argument 1 = this (amount, parameter 0), argument 2 = this (gain multiplier, parameter 1), argument 3 = this
/// (subpixel, parameter 2). Output channel 0 = dx, channel 1 = dy (selector tag 2; INFERRED reads the binding in the README). The blurred
/// plane (aux) is scratch and dropped. A frame of width 1 is refused (the gradient needs a neighbouring column).
fn grad_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    // MUTANT-SWITCH: `mu(id)` is a constant false unless the build has the feature `drw2_mut`; then COV_MUT=id selects one edit at run time.
    #[cfg(feature = "drw2_mut")]
    let mu = |id: u32| grad::mu(id);
    #[cfg(not(feature = "drw2_mut"))]
    let mu = |_id: u32| false;
    need(args, 4, "grad")?;
    if tag != 2 && !mu(23) {
        return Err(ExecError::Kernel { class: "grad".into(), msg: format!("selector tag {} differs from the class row (1,1,1,1) -> 2", tag) });
    }
    if w < (if mu(24) { 1 } else { 2 }) {
        return Err(unsupported("grad", "width 1: the original reads one float past the plane (GR-A3, CONTRACT-LIMIT w >= 2)"));
    }
    let src_owned = image_or_const(&args[0], w * h, "grad")?;
    if src_owned.n != w * h || src_owned.channels() < 1 {
        return Err(ExecError::Shape("grad: source smaller than the frame".into()));
    }
    // the aspect block is the port of set_aspect (aspect.rs, T2-checked against the emulated original); a value outside the
    // callers' range (double a > 1e-4 and a < 1e4, NaN included) never reaches set_aspect in the original (the callers substitute 1.0): class S, error.
    // HOST-ASPECT-FALLBACK. The final aspect value a goes through the tail of the caller block : outside (1e-4 as double, 1e4 as float) or NaN
    // the original calls set_aspect(1.0f). T2 582 cases 0 differing (t2/emu_caller.tsv, tests/t18_caller.rs). The refusal HOST-ASPECT-RANGE is withdrawn.
    let a_eff = if crate::aspect::in_caller_range(kp.host_aspect) { kp.host_aspect } else { crate::aspect::FALLBACK_A };
    let asp_blk = crate::aspect::set_aspect(a_eff, 0.0).expect("in-range aspect is nonzero");
    let (ia, ig) = if mu(21) { (2, 1) } else { (1, 2) };
    let amount = const_lane(&args[ia], if mu(34) { 1 } else { 0 }, "grad", "amount")?;
    let gain_param = const_lane(&args[ig], 0, "grad", "gain")?;
    let subpixel = if mu(22) { 0 } else { cvt_i32(const_lane(&args[3], 0, "grad", "subpixel")?) };
    let gp = grad::GradParams {
        amount,
        gain_param,
        subpixel,
        edge_mode: if mu(27) { 1 - kp.blur_edge } else { kp.blur_edge },
        render_scale: if mu(28) {
            h as i32
        } else if mu(29) {
            1
        } else if kp.blur_render_scale > 0 {
            kp.blur_render_scale
        } else {
            w as i32
        },
        // at host_aspect 1.0 the values are aspect flag 0 and info = 0 (MEASURED in the E1 runs, README). The derivation for other aspects
        // (set_aspect : = (a != 1.0), = 1/a, = ((double)a > 1.6)) is INFERRED: the E1 write-watch did not see set_aspect write this info object.
        aspect: if asp_blk.aspect_block_b != 0 || mu(32) { Some(f32::from_bits(asp_blk.aspect_block_d)) } else { None },
        // one source, the plugin's derivation (set_aspect aspect_block_e); KParams.kparams_a is a checked test interface
        info_aspect_set: (asp_blk.aspect_block_e != 0) != mu(31),
        flag_1: if mu(30) { 0 } else { 1 },
        past_end: if mu(35) { 1.0 } else { 0.0 },
    };
    if let Some(v) = kp.kparams_a {
        if v != (asp_blk.aspect_block_e != 0) {
            return Err(kerr("grad", format!("GRAD-INFO3C-MISMATCH: test interface info_aspect {} differs from the set_aspect derivation {}", v, asp_blk.aspect_block_e != 0)));
        }
    }
    let n = w * h;
    let mut aux = vec![0.0f32; n];
    let mut dx = vec![0.0f32; n];
    let mut dy = vec![0.0f32; n];
    let sp = if mu(33) { src_owned.planes.len() - 1 } else { 0 };
    grad::render(&gp, &src_owned.planes[sp], w, h, &mut aux, &mut dx, &mut dy).map_err(|e| kerr("grad", e))?;
    if mu(25) {
        return Ok(Image { n, planes: vec![dy, dx] });
    }
    if mu(26) {
        return Ok(Image { n, planes: vec![dx.clone(), dx] });
    }
    Ok(Image { n, planes: vec![dx, dy] })
}

/// `round` (slot 53, AVX2 body, `scale_rgb::round_snap`K-PARITY at floor) and `scale_rgb` (slot 53
/// `scale_rgb::scale_rgb`K-PARITY). Both read the same two-record object (this: operand channel count, constant flag,
/// constant lanes, plane array), so the operand mapping is shared: argument 0 = record 0 (n_in, flag_a, in_p, src planes), argument 1 =
/// record 1 (n_out, flag_b, out_p, aux planes). The channel-count getter [vtable] is the selector tag (as for hue/sat, INFERRED, IN7-A2).
/// A constant on both operands makes the original read the aux plane of a constant (a null pointer slot): refused as unsupported (the
/// compiler folds such a node in the original; `fx_eval` has no fold for these classes). scale_rgb's host pad call (mode 0 = argument
/// 0 block, mode 1 = argument 1 block, channel idx, destination = output plane idx) runs through the finished `plane_service` crate (IN7-A3).
fn round_k(class: &str, args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, class)?;
    if args[0].is_const() && args[1].is_const() {
        return Err(unsupported(class, "both operands constant: the body reads the plane array of a constant operand (the original folds the node)"));
    }
    let planes_of = |a: &Arg| -> Option<Vec<Vec<f32>>> { if a.is_const() { None } else { Some(a.planes().iter().map(|p| p.to_vec()).collect()) } };
    let (ca, cb) = (args[0].f32s(), args[1].f32s());
    let o = scale_rgb::Obj {
        n_in: args[0].count(),
        n_out: args[1].count(),
        flag_a: args[0].is_const(),
        flag_b: args[1].is_const(),
        in_p: ca,
        out_p: cb,
        src: planes_of(&args[0]),
        aux: planes_of(&args[1]),
    };
    let mut out: Vec<Vec<f32>> = vec![vec![0.0; n]; 4];
    if class == "round" {
        scale_rgb::round_snap(&o, n as i32, tag, &mut out).map_err(|e| kerr("round", e))?;
    } else {
        let (rec, _written) = scale_rgb::scale_rgb(&o, n as i32, tag, &mut out).map_err(|e| kerr("scale_rgb", e))?;
        if let Some(pc) = rec {
            let (pa, pb) = (args[0].planes(), args[1].planes());
            let (va, vb) = (args[0].bits().to_vec(), args[1].bits().to_vec());
            let blocks = [block_of(&args[0], &pa, &va), block_of(&args[1], &pb, &vb)];
            let host = ps::Host { in_pool: kp.in_pool };
            let dst = out.get_mut(pc.idx as usize).ok_or(ExecError::Shape("scale_rgb pad plane".into()))?;
            ps::plane_service(&host, ps::Sel::Direct { blocks: &blocks }, pc.mode as i32, pc.idx, dst, &flat_roi(n)).map_err(|e| kerr("plane_service", e))?;
        }
    }
    out.truncate((tag.max(0) as usize).min(4));
    Ok(Image { n, planes: out })
}

/// `add_rgb` (slot 53, crate `add_rgb`additive twin of scale_rgb). Same operand mapping as `round_k` (BLW1-A1, INFERRED
/// from the identical this layout read by the listing and from the 12 combo rows being identical to `*`): argument 0 = record 0
/// (n_in, flag_a, in_p, src planes), argument 1 = record 1 (n_out, flag_b, out_p, aux planes); channel-count getter = selector tag. The
/// pad call runs through the finished plane_service crate as for scale_rgb. Both operands constant: refused (the compiler folds it).
fn add_rgb_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "add_rgb")?;
    if args[0].is_const() && args[1].is_const() {
        return Err(unsupported("add_rgb", "both operands constant: the body reads the plane array of a constant operand (the original folds the node)"));
    }
    let planes_of = |a: &Arg| -> Option<Vec<Vec<f32>>> { if a.is_const() { None } else { Some(a.planes().iter().map(|p| p.to_vec()).collect()) } };
    let o = add_rgb::Obj {
        n_in: args[0].count(),
        n_out: args[1].count(),
        flag_a: args[0].is_const(),
        flag_b: args[1].is_const(),
        in_p: args[0].f32s(),
        out_p: args[1].f32s(),
        src: planes_of(&args[0]),
        aux: planes_of(&args[1]),
    };
    let mut out: Vec<Vec<f32>> = vec![vec![0.0; n]; 4];
    let (rec, _written) = add_rgb::add_rgb(&o, n as i32, tag, &mut out).map_err(|e| kerr("add_rgb", e))?;
    if let Some(pc) = rec {
        let (pa, pb) = (args[0].planes(), args[1].planes());
        let (va, vb) = (args[0].bits().to_vec(), args[1].bits().to_vec());
        let blocks = [block_of(&args[0], &pa, &va), block_of(&args[1], &pb, &vb)];
        let host = ps::Host { in_pool: kp.in_pool };
        let dst = out.get_mut(pc.idx as usize).ok_or(ExecError::Shape("add_rgb pad plane".into()))?;
        ps::plane_service(&host, ps::Sel::Direct { blocks: &blocks }, pc.mode as i32, pc.idx, dst, &flat_roi(n)).map_err(|e| kerr("plane_service", e))?;
    }
    out.truncate((tag.max(0) as usize).min(4));
    Ok(Image { n, planes: out })
}

/// `masked_blur` (slot 52, crate `masked_blur` after, threshold `MASK_THRESHOLD` = 2^-13 MEASURED-DYLIB). Arguments in
/// the program's order (source, mask, amount [1 or 2 lanes], filter_shape, subpixel); the combo rows are (src channels 1..4, 1, 1 or 2, 1, 1).
/// The selector tag is the plane count (vfunc 31). render_scale and aspect as in `blur_k` (BLW1-A2: the same host values are assumed for the
/// masked wrapper, INFERRED from the crate's sigma rule: sigma = (amount * k) * 0.125 with k = src). Edge fixed to zero by the crate.
fn masked_blur_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 5, "masked_blur")?;
    let src_owned = image_or_const(&args[0], w * h, "masked_blur")?;
    let mask_owned = image_or_const(&args[1], w * h, "masked_blur")?;
    let c = tag.max(0) as usize;
    if src_owned.channels() < c || src_owned.n != w * h || mask_owned.channels() < 1 || mask_owned.n != w * h {
        return Err(ExecError::Shape("masked_blur: source or mask smaller than the combo tag or than the frame".into()));
    }
    let amount = [const_lane(&args[2], 0, "masked_blur", "amount")?, const_lane(&args[2], 1, "masked_blur", "amount")?];
    let shape = cvt_i32(const_lane(&args[3], 0, "masked_blur", "filter_shape")?);
    let subpixel = cvt_i32(const_lane(&args[4], 0, "masked_blur", "subpixel")?);
    let bp = masked_blur::MaskedBlurParams {
        amount,
        filter_shape: shape,
        subpixel,
        plane_count: c,
        render_scale: if kp.blur_render_scale > 0 { kp.blur_render_scale } else { w as i32 },
        aspect: None,
        mask_threshold: masked_blur::MASK_THRESHOLD,
    };
    let sp: Vec<fx_common::plane::Plane> = (0..c).map(|k| fx_common::plane::Plane::new(&src_owned.planes[k], w, h, w)).collect::<Result<_, _>>().map_err(|e| kerr("masked_blur", e))?;
    let mp = fx_common::plane::Plane::new(&mask_owned.planes[0], w, h, w).map_err(|e| kerr("masked_blur", e))?;
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; w * h]; c];
    {
        let mut dp: Vec<fx_common::plane::PlaneMut> = Vec::new();
        for v in outv.iter_mut() {
            dp.push(fx_common::plane::PlaneMut::new(v.as_mut_slice(), w, h, w).map_err(|e| kerr("masked_blur", e))?);
        }
        masked_blur::render(&bp, &sp, Some(&mp), &mut dp).map_err(|e| kerr("masked_blur", e))?;
    }
    Ok(Image { n: w * h, planes: outv })
}

/// `comp_premult` (slot 53, AVX2 body, crate `comp_premult` from K-PARITY at floor, R-PARITY for
/// the 10 measured combo rows, ROUTE AVX2-ONLY). Argument 0 = A (this), argument 1 = B (0x70 .), argument 2 =
/// T, the alpha (0xa0 ...). `tag` (the combo row's output tag) is the class count getter (virtual slot 31) = number of output channels
/// (DSW1-A1, INFERRED from SPEC section 7 and the rows (4,4,1)->4 ... (3,4,1)->4, as for round_k). The crate fills a constant-only
/// call through the int32 round trip itself. nout above 4 and aliasing are refused by the crate / not possible here (fresh outputs).
fn comp_premult_k(args: &[Arg], tag: i32, n: usize) -> Result<Image, ExecError> {
    need(args, 3, "comp_premult")?;
    let (pa, pb, pt) = (args[0].planes(), args[1].planes(), args[2].planes());
    // rep1 is removed; constant lanes arrive filled by exec.rs const_val (the original's set_arg_const binding,
    // R-PARITY); comp_premult reads them raw as the original does .
    let rep1 = |a: &Arg| -> [f32; 4] { a.f32s() };
    let oa = comp_premult::Operand { count: args[0].count(), is_const: args[0].is_const(), consts: rep1(&args[0]), planes: &pa };
    let ob = comp_premult::Operand { count: args[1].count(), is_const: args[1].is_const(), consts: rep1(&args[1]), planes: &pb };
    let ot = comp_premult::Operand { count: args[2].count(), is_const: args[2].is_const(), consts: rep1(&args[2]), planes: &pt };
    let nout = tag.clamp(0, 4) as usize;
    if tag > 4 {
        return Err(unsupported("comp_premult", "the combo row asks for more than 4 output channels"));
    }
    let p = comp_premult::Params { nout: tag, n: n as i32 };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; nout];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        comp_premult::run(&p, &oa, &ob, &ot, &mut outs, comp_premult::Mutant::None).map_err(|e| kerr("comp_premult", e))?;
    }
    Ok(Image { n, planes: outv })
}

/// `copy` (slot 52, a 128 byte driver loop; crate `copy_planes`R-PARITY of the slot function against the emulator with the plane
/// service hooked as record_args). The loop calls the host plane service once per output channel ch < tag as service(this, mode 0,
/// channel ch, destination outs[ch], info): argument 0's channel min(ch, count - 1) is copied, or a constant operand fills its lane
/// (`plane_service` crate, flat modes K-PARITY; CALLEE-MODELLED block choice as everywhere, NV5-A1). The class count getter is the selector tag.
fn copy_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 1, "copy")?;
    let nout = tag.clamp(0, 4) as usize;
    let mut out: Vec<Vec<f32>> = vec![vec![0.0; n]; nout];
    for c in copy_planes::slot52_calls(tag) {
        if c.mode != 0 {
            return Err(unsupported("copy", "the slot function made a plane service call with a mode other than 0"));
        }
        let dst = out.get_mut(c.out as usize).ok_or(ExecError::Shape("copy destination plane".into()))?;
        pad_fill(kp, &args[0], c.chan, dst, n)?;
    }
    Ok(Image { n, planes: out })
}

/// `copy_alpha` (slot 52; crate `copy_alpha`): output channels 0 to tag-2 are channels of argument 0, the last output channel is channel
/// tag-1 of argument 1 when the combo has 2 or 4 channels; any other tag copies all channels from argument 0. The operator's mode word
/// is `fx_eval::fold::copy_alpha_mode` (the combo tag, except that a 2 channel output fed by a 4 channel second operand takes its alpha from that
/// operand's channel 3: the mode word is then 4 and the alpha channel index mode-1 addresses the last channel of the second operand); the compile's
/// collapse test reads the same function. A constant operand
/// fills its lanes; the pool of a scalar fill is idle outside a pool worker.
fn copy_alpha_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "copy_alpha")?;
    let nout = tag.clamp(0, 4) as usize;
    let planes: Vec<Vec<&[f32]>> = args.iter().map(|a| a.planes()).collect();
    let lanes: Vec<Vec<u32>> = args.iter().map(|a| a.bits()[..a.count().clamp(0, 4) as usize].to_vec()).collect();
    let src = |i: usize| -> copy_alpha::Source {
        if args[i].is_const() {
            copy_alpha::Source::Scalar { values: &lanes[i], is_int: false, last: args[i].count() - 1 }
        } else {
            copy_alpha::Source::Planes { planes: &planes[i], last: args[i].count() - 1 }
        }
    };
    let mut out: Vec<Vec<f32>> = vec![vec![0.0; n]; nout];
    {
        let mut refs: Vec<&mut [f32]> = out.iter_mut().map(|v| v.as_mut_slice()).collect();
        let mode = fx_eval::fold::copy_alpha_mode(tag, args[1].count());
        let p = copy_alpha::Params { mode, nchan: tag, n, pool_idle: !kp.in_pool };
        copy_alpha::copy_alpha(&p, &src(0), &src(1), &mut refs).map_err(|e| kerr("copy_alpha", e))?;
    }
    Ok(Image { n, planes: out })
}

fn rotate_k(args: &[Arg], n: usize) -> Result<Image, ExecError> {
    need(args, 3, "rotate")?;
    let (vp, cp, ap);
    let v = match &args[0] {
        Arg::Const { .. } => rotate2d::Vec2::Const(args[0].f32s()[0], args[0].f32s()[1]),
        Arg::Img(i) if i.channels() == 2 => {
            vp = (i.planes[0].as_slice(), i.planes[1].as_slice());
            rotate2d::Vec2::Planes(vp.0, vp.1)
        }
        _ => return Err(unsupported("rotate", "vector operand must be a 2 lane constant or a 2 channel image")),
    };
    let angle = match &args[1] {
        Arg::Const { .. } => rotate2d::Angle::Const(args[1].f32s()[0]),
        Arg::Img(i) if i.channels() == 1 => {
            ap = i.planes[0].as_slice();
            rotate2d::Angle::Plane(ap)
        }
        _ => return Err(unsupported("rotate", "angle operand must be a constant or a 1 channel image")),
    };
    let center = match &args[2] {
        Arg::Const { .. } => rotate2d::Vec2::Const(args[2].f32s()[0], args[2].f32s()[1]),
        Arg::Img(i) if i.channels() == 2 => {
            cp = (i.planes[0].as_slice(), i.planes[1].as_slice());
            rotate2d::Vec2::Planes(cp.0, cp.1)
        }
        _ => return Err(unsupported("rotate", "centre operand must be a 2 lane constant or a 2 channel image")),
    };
    let mut ox = vec![0.0f32; n];
    let mut oy = vec![0.0f32; n];
    rotate2d::render(v, angle, center, n as i32, &mut ox, &mut oy).map_err(|e| kerr("rotate", e))?;
    Ok(Image { n, planes: vec![ox, oy] })
}

fn element_reduce_k(class: &str, args: &[Arg], n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 1, class)?;
    let a = &args[0];
    let planes = a.planes();
    let p = ops_element::reduce::RParams { count: a.count(), n: n as i32 };
    let svc = ops_element::Svc { in_pool: kp.in_pool, is_int: false, values: a.bits(), enable: a.is_const(), img_valid: true };
    let mut dst = vec![0.0f32; n];
    if class == "max_element" {
        ops_element::max_element::render(&p, &planes, &svc, &mut dst).map_err(|e| kerr(class, e))?;
    } else {
        ops_element::min_element::render(&p, &planes, &svc, &mut dst).map_err(|e| kerr(class, e))?;
    }
    Ok(Image { n, planes: vec![dst] })
}

fn blur_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 5, "blur")?;
    // at Shift X = Shift Y = 0 warp_rel folds away and the original hands blur the make_mipmap object itself (dzrec plan text)
    let src_owned = match &args[0] {
        Arg::Mip(m) => m.base.clone(),
        a => image_or_const(a, w * h, "blur")?,
    };
    let src = &src_owned;
    let c = tag.max(0) as usize;
    if src.channels() < c || src.n != w * h {
        return Err(ExecError::Shape("blur: source smaller than the combo tag or than the frame".into()));
    }
    let amount = [const_lane(&args[1], 0, "blur", "amount")?, const_lane(&args[1], 1, "blur", "amount")?];
    let shape = cvt_i32(const_lane(&args[2], 0, "blur", "filter_shape")?);
    let sc = if args[3].is_const() { args[3].f32s() } else { return Err(unsupported("blur", "scale must be a constant")) };
    let subpixel = cvt_i32(const_lane(&args[4], 0, "blur", "subpixel")?);
    let bp = blur::BlurParams {
        amount,
        filter_shape: shape,
        scale: sc,
        scale_count: args[3].count(),
        subpixel,
        edge_mode: kp.blur_edge,
        plane_count: c,
        render_scale: if kp.blur_render_scale > 0 { kp.blur_render_scale } else { w as i32 },
        aspect: kp.blur_aspect, //: the host input, no constant
    };
    let sp: Vec<fx_common::plane::Plane> = (0..c).map(|k| fx_common::plane::Plane::new(&src.planes[k], w, h, w)).collect::<Result<_, _>>().map_err(|e| kerr("blur", e))?;
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; w * h]; c];
    {
        let mut dp: Vec<fx_common::plane::PlaneMut> = Vec::new();
        for v in outv.iter_mut() {
            dp.push(fx_common::plane::PlaneMut::new(v.as_mut_slice(), w, h, w).map_err(|e| kerr("blur", e))?);
        }
        blur::render(&bp, &sp, &mut dp).map_err(|e| kerr("blur", e))?;
    }
    Ok(Image { n: w * h, planes: outv })
}

/// IN3-A1 (dissolve, slot 53 -> AVX2 body, crate `dissolve`K-PARITY plus RPARITY.tsv "ACHIEVED (I)"): the plan's
/// arguments are (A = value at t <= 0, B = value at t >= 1, T = matte), the order of slot 38 rule (: argument 2 is
/// the matte, arguments 0 and 1 are compared by). An image operand is one plane per channel (count = channels); a
/// constant operand is the scalar block (count = lanes; the compiler's lanes of a 1 lane constant are replicated, a 2 or 3 lane
/// constant keeps its 4th lane as printed). The output channel count is the selector tag (the combo table's `len`). The object
/// flag bytes the original writes at channel 4 (DS-A2) are not carried from row to row: every call is a fresh object.
fn dissolve_k(args: &[Arg], tag: i32, w: usize, h: usize) -> Result<Image, ExecError> {
    need(args, 3, "dissolve")?;
    let nch = tag.max(0) as usize;
    if nch == 0 || nch > dissolve::MAX_CHANNELS {
        return Err(ExecError::Shape(format!("dissolve: combo tag {} outside 1..4", tag)));
    }
    let mut counts = [0i32; 3];
    let mut scal = [false; 3];
    let mut vals = [[0.0f32; 4]; 3];
    for (k, a) in args.iter().take(3).enumerate() {
        match a {
            Arg::Img(i) => {
                if i.n != w * h {
                    return Err(ExecError::Shape(format!("dissolve: operand {} has {} samples, frame is {}", k, i.n, w * h)));
                }
                counts[k] = i.channels() as i32;
            }
            Arg::Const { n, .. } => {
                counts[k] = *n as i32;
                scal[k] = true;
                vals[k] = a.f32s();
            }
            Arg::Mip(_) => return Err(unsupported("dissolve", "a ripmap object is not a dissolve operand")),
        }
    }
    let mut planes: Vec<Vec<fx_common::plane::Plane>> = Vec::new();
    for a in args.iter().take(3) {
        let mut v = Vec::new();
        if let Arg::Img(i) = a {
            for p in &i.planes {
                v.push(fx_common::plane::Plane::new(p, w, h, w).map_err(|e| kerr("dissolve", e))?);
            }
        }
        planes.push(v);
    }
    let p = dissolve::Params {
        len: w,
        a_count: counts[0],
        b_count: counts[1],
        t_count: counts[2],
        a_scalar: scal[0],
        b_scalar: scal[1],
        t_scalar: scal[2],
        a_values: vals[0],
        b_values: vals[1],
        t_values: vals[2],
    };
    let op = |k: usize| -> dissolve::Operand {
        if scal[k] {
            dissolve::Operand::Scalar(vals[k])
        } else {
            dissolve::Operand::Planes(&planes[k])
        }
    };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; w * h]; nch];
    {
        let mut dp: Vec<fx_common::plane::PlaneMut> = Vec::new();
        for v in outv.iter_mut() {
            dp.push(fx_common::plane::PlaneMut::new(v.as_mut_slice(), w, h, w).map_err(|e| kerr("dissolve", e))?);
        }
        dissolve::render_ops(&p, op(0), op(1), op(2), &mut dp).map_err(|e| kerr("dissolve", e))?;
    }
    Ok(Image { n: w * h, planes: outv })
}

// ---------------------------------------------------------------------------------------------------------------------------------
// perspective, perspective_alpha (slot 52) and warp (slot 53), through the warp_render slot functions.

const WPG1_FW: u64 = warp_render::FW_AVX2;

fn env_f(name: &str) -> Option<f64> {
    std::env::var(name).ok().and_then(|v| v.parse::<f64>().ok())
}

/// The three scalar arguments of perspective and perspective_alpha: (z_dist -> this, lat/90 -> this, 1/tele_width -> this).
/// (INFERRED): the record offsets are the 2nd, 3rd and 4th plan arguments (a fixed stride from the image record at).
fn persp_scalars(args: &[Arg], class: &str) -> Result<(f32, f32, f32), ExecError> {
    need(args, 4, class)?;
    Ok((const_lane(&args[1], 0, class, "argument 1 (z_dist)")?, const_lane(&args[2], 0, class, "argument 2 (latitude / 90)")?, const_lane(&args[3], 0, class, "argument 3 (1 / tele width)")?))
}

fn uv_of<'a>(args: &'a [Arg], class: &str, n: usize) -> Result<(&'a [f32], &'a [f32]), ExecError> {
    match &args[0] {
        Arg::Img(i) if i.channels() == 2 && i.n == n => Ok((&i.planes[0], &i.planes[1])),
        _ => Err(unsupported(class, "argument 0 must be a 2 channel image of the frame size (u and v planes)")),
    }
}

/// Row chunking (INFERRED from blur_warp's measured per line call): one slot 52 call per row with count = w; the row's
/// out2 buffer phase is (row * w * 4) & 0x1f with a 32 byte aligned plane base (ALIGN-ASSUMED). WPG1_PERSP_WHOLE=1 runs one call of w * h.
#[allow(dead_code)]
fn rows_of(w: usize, h: usize) -> Vec<(usize, usize, usize)> {
    rows_of_b(w, h, 0)
}

/// the same with the plane base phase (bytes mod 32) of the output plane as an explicit per-run host input (ALIGN-HOST-PER-RUN, MEASURED
/// 0 or 16 in the E1 runs); `base` is read from env WPG1_PBASE (perspective out2) or WPG1_ABASE (perspective_alpha out1), default 0 = the pre-WPE3 behaviour.
fn rows_of_b(w: usize, h: usize, base: usize) -> Vec<(usize, usize, usize)> {
    rows_of_p(w, h, base, false)
}

/// `rowbuf` = the destination is one row buffer reused for every row (constant phase `base`), else a plane (base + row x W x 4)
fn rows_of_p(w: usize, h: usize, base: usize, rowbuf: bool) -> Vec<(usize, usize, usize)> {
    if std::env::var("WPG1_PERSP_WHOLE").map(|v| v == "1").unwrap_or(false) {
        return vec![(0, w * h, 0)];
    }
    (0..h).map(|y| (y * w, w, if rowbuf { base & 0x1f } else { (base + y * w * 4) & 0x1f })).collect()
}

fn env_base(name: &str) -> usize {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(0)
}

fn perspective_k(args: &[Arg], w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    let (a, gpu_call_a, b) = persp_scalars(args, "perspective")?;
    let n = w * h;
    let (u, v) = uv_of(args, "perspective", n)?;
    let (mut o1, mut o2) = (vec![0.0f32; n], vec![0.0f32; n]);
    for (lo, cnt, ph) in rows_of_p(w, h, if kp.persp_base >= 0 { kp.persp_base as usize } else { env_base("WPG1_PBASE") }, kp.persp_rowbuf) {
        let t = warp_render::PerspThis { a, gpu_call_a, b, inputs: Some((&u[lo..lo + cnt], &v[lo..lo + cnt])) };
        // out2 is written at its row start; the glue takes the phase of that start
        let mut r1 = vec![0.0f32; cnt];
        let mut r2 = vec![0.0f32; cnt];
        warp_render::slot52_perspective(WPG1_FW, &t, false, cnt as i32, ph as u32, &mut r1, &mut r2).map_err(|e| kerr("perspective", e))?;
        o1[lo..lo + cnt].copy_from_slice(&r1);
        o2[lo..lo + cnt].copy_from_slice(&r2);
    }
    Ok(Image { n, planes: vec![o1, o2] })
}

/// perspective_alpha needs three host values read from [[rec]] (i32, i32, f64) that no measured source supplies
/// (PARAM-UNCAPTURED 3): no default; WPG1_S8, WPG1_S48 and WPG1_S30 must be set (test use), else the class is refused.
fn perspective_alpha_k(args: &[Arg], w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    let (a, gpu_call_a, b) = persp_scalars(args, "perspective_alpha")?;
    let n = w * h;
    let (_u, v) = uv_of(args, "perspective_alpha", n)?;
    let (s8, port_info_b, port_info_a) = match kp.palpha_cells {
        Some((x, y, z)) => (x, y, z),
        None => match (env_f("WPG1_S8"), env_f("WPG1_S48"), env_f("WPG1_S30")) {
        (Some(x), Some(y), Some(z)) => (x as i32, y as i32, z),
        _ => {
            return Err(unsupported("perspective_alpha", "the host values [[rec]] (i32) and (f64) are not measured (PARAM-UNCAPTURED, no default): set WPG1_S8, WPG1_S48, WPG1_S30"))
        }
        },
    };
    let par = warp_render::AlphaParams { params_b: gpu_call_a, params_a: a, params_c: b };
    let mut out = vec![0.0f32; n];
    for (lo, cnt, ph) in rows_of_p(w, h, if kp.palpha_base >= 0 { kp.palpha_base as usize } else { env_base("WPG1_ABASE") }, kp.palpha_rowbuf) {
        let info = warp_render::AlphaInfo { n: cnt as i32, roi_form: 0, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0, s8, port_info_b, port_info_a };
        let mut dst = vec![0.0f32; cnt];
        let o = warp_render::slot52_perspective_alpha(WPG1_FW, &par, &info, false, &v[lo..lo + cnt], &mut dst, ph).map_err(|e| kerr("perspective_alpha", e))?;
        if let warp_render::AlphaRoute::Fill { .. } = o.route {
            return Err(unsupported("perspective_alpha", "x8 == 0 takes the ROI zero fill whose record fields are host values (not wired)"));
        }
        out[lo..lo + cnt].copy_from_slice(&dst);
    }
    Ok(Image { n, planes: vec![out] })
}

/// warp: slot 53 once per row, flag 0, record geometry from the plan's Geom (the same fields the warp_rel and blur_warp arms feed;
/// the image geometry block [img] is assumed absent). The 4th argument is the plan's `filter`a nonzero filter with image coordinates
/// is the footprint path that needs slot 33's host outputs c1/c2 (callback, NOT REBUILT).
fn warp_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 4, "warp")?;
    let mip = mip_of(&args[0], "warp", w, h)?;
    let g = geom_of(kp, w, h);
    let mx = cvt_i32(const_lane(&args[2], 0, "warp", "wrap")?);
    let my = cvt_i32(const_lane(&args[2], 1, "warp", "wrap")?);
    // slot 53 driver entry mode assertion. The registered popup `wrap` is 0..2 (No, Tile, Reflect); the driver tests 1 and 2 only
    // so another value is outside the contract: refused by name, never run.
    if !(0..=2).contains(&mx) || !(0..=2).contains(&my) {
        return Err(kerr("warp", format!("WARP-MODE-OUT-OF-CONTRACT: mode lanes mx {} my {} outside the registered popup range 0..2 of wrap (OUT-OF-CONTRACT by input value)", mx, my)));
    }
    let filter = const_lane(&args[3], 0, "warp", "argument 3 (filter)")?;
    let n = w * h;
    let (const_coords, ca1, ca2, a1, a2) = match &args[1] {
        Arg::Const { .. } => {
            let f = args[1].f32s();
            (true, f[0], f[1], Vec::new(), Vec::new())
        }
        Arg::Img(i) if i.channels() == 2 => (false, 0.0, 0.0, i.planes[0].clone(), i.planes[1].clone()),
        _ => return Err(unsupported("warp", "coordinate operand must be a 2 lane constant or a 2 channel image")),
    };
    // (last): slot 33 (over, partition as real code, callback) supplies c1/c2 for the footprint path, once per frame
    // (chunking: whole frame for slot 33, rows for slot 53, INFERRED). [P] = the filter lane (INFERRED). ctx has NO measured source: STAND-IN below.
    let mut c1_arr = vec![0.0f32; n];
    let mut c2_arr = vec![0.0f32; n];
    let mut flag_byte = 0u8;
    if filter != 0.0 && !const_coords {
        let g33 = geom_of(kp, w, h);
        let bits = |v: &Vec<f32>| v.iter().map(|x| x.to_bits()).collect::<Vec<u32>>();
        let (ub, vb) = (bits(&a1), bits(&a2));
        let mut c1b = vec![warp_log_gradient::SENT; n];
        let mut c2b = vec![warp_log_gradient::SENT; n];
        let prim33 = warp_log_gradient::Prim33 { n: cvt_i32(filter), cc: 0 };
        let rec33 = slot33_rec(w, h); if kp.host_aspect != 1.0 { return Err(unsupported("warp", "slot 33 geometry cells ctx measured only at host aspect 1.0")); } // A10: the measured cells replace the withdrawn stand-in
        let mut pool = warp_log_gradient::Pool::default(); // captured state: g_cell_b = 0, serial callback
        let th = warp_log_gradient::Threads { nthr: 1, nthr2: 1, tid: 0 };
        let mut ev = Vec::new();
        let o = warp_log_gradient::slot33_warp(warp_log_gradient::FW_AVX2, &prim33, Some(&rec33), None, (&ub, &vb), &mut c1b, &mut c2b, &mut pool, &th, &mut ev).map_err(|e| kerr("warp slot 33", e))?;
        flag_byte = o.flag_byte;
        c1_arr = c1b.iter().map(|b| f32::from_bits(*b)).collect();
        c2_arr = c2b.iter().map(|b| f32::from_bits(*b)).collect();
    }
    let c = mip.channels;
    if c as i32 != tag {
        return Err(ExecError::Shape(format!("warp: tag {} but the ripmap has {} channels", tag, c)));
    }
    let nl = mip.ripmap.nl();
    let np = mip.ripmap.np();
    let f1 = warp_render::F1Params {
        n: c as i32,
        width_tab: mip.ripmap.widths.iter().map(|&v| v as i32).collect(),
        height_tab: mip.ripmap.heights.iter().map(|&v| v as i32).collect(),
        t: mip.scale.as_ref().map(|s| s.iter().map(|b| f32::from_bits(*b)).collect()).unwrap_or_default(),
        flag_x: 0,
        flag_y: 0,
    };
    let mk_img = || warp_render::DriverImage {
        f1: warp_render::F1Params { n: f1.n, width_tab: f1.width_tab.clone(), height_tab: f1.height_tab.clone(), t: f1.t.clone(), flag_x: 0, flag_y: 0 },
        w: w as i32,
        h: h as i32,
        d_cell_d: nl as i32,
        d_cell_e: np as i32,
    };
    let lat = warp_render::Lattice {
        bufs: (0..nl).map(|l| (0..np).map(|j| mip.ripmap.planes[mip.ripmap.idx(l, j)].iter().map(|b| f32::from_bits(*b)).collect()).collect()).collect(),
    };
    let prim = warp_render::WarpPrim { const_coords, const_a1: ca1, const_a2: ca2, a1_arr: a1, a2_arr: a2, c1_arr, c2_arr, flag: flag_byte, mx, my };
    let nt = crate::par::threads_for(h, n);
    let parts = crate::par::row_bands(h, nt, |lo, hi| {
        let mut img = mk_img();
        let mut band: Vec<Vec<f32>> = vec![vec![0.0; (hi - lo) * w]; c];
        for y in lo..hi {
            let rec = warp_row_rec(&g, w, y);
            let mut row: Vec<Vec<f32>> = vec![vec![0.0; w]; c];
            warp_render::slot53_warp(&prim, &rec, None, &mut img, &lat, &mut row).map_err(|e| kerr("warp", e))?;
            for k in 0..c {
                band[k][(y - lo) * w..(y - lo + 1) * w].copy_from_slice(&row[k]);
            }
        }
        Ok(band)
    })?;
    Ok(Image { n, planes: crate::par::join_planes(parts, c, n) })
}

thread_local! {
    ///: the host alpha planes of the 'ae32' sources by source index (the 1-plane source conversion of the
    /// run's own host words), set by execute_ae32 for one run and cleared after it; None on every other route.
    pub(crate) static COPY_ALPHA_PLANES: std::cell::RefCell<Option<Vec<Vec<f32>>>> = std::cell::RefCell::new(None);
}

/// set (Some) or clear (None) the per-run host alpha planes of the copy-alpha arm.
pub fn set_copy_alpha_planes(p: Option<Vec<Vec<f32>>>) {
    COPY_ALPHA_PLANES.with(|c| *c.borrow_mut() = p);
}

/// `postdivide` (slot 53, crate-free port in postdiv.rs; T2 against the emulated original:
/// 234 cases, 0 differing floats, 11 of 12 mutants killed). Argument 0 = the image (elnum = its channels),
/// argument 1 = copy_alpha ([this], an int32 of the constant); the combo tag is the output count (COMBOS_EXEC rows postdivide 1..4).
fn postdivide_k(args: &[Arg], tag: i32, n: usize) -> Result<Image, ExecError> {
    need(args, 2, "postdivide")?;
    let copy_alpha = cvt_i32(const_lane(&args[1], 0, "postdivide", "copy_alpha")?);
    let src = image_or_const(&args[0], n, "postdivide")?;
    let e = src.channels();
    let bits: Vec<Vec<u32>> = src.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect();
    let sref: Vec<&[u32]> = bits.iter().map(|v| v.as_slice()).collect();
    let nout = tag.clamp(0, 4) as usize;
    let mut out = vec![vec![0u32; n]; nout];
    //: the copy-alpha arm on the recorded route only (copy_alpha 0, elnum 3, nout 3), checked before the
    // alpha is fetched; every other copy_alpha >= 0 route reaches postdiv_into below and keeps POSTDIV-COPY-ALPHA-SOURCE in its error text.
    let route = crate::postdiv::CopyAlphaRoute { copy_alpha, elnum: e, nout: tag };
    if copy_alpha == crate::postdiv::ROUTE_COPY_ALPHA && e == crate::postdiv::ROUTE_ELNUM && tag == crate::postdiv::ROUTE_NOUT {
        let alpha: Vec<u32> = COPY_ALPHA_PLANES
            .with(|c| c.borrow().as_ref().and_then(|v| v.get(copy_alpha as usize).cloned()))
            .ok_or_else(|| unsupported("postdivide", &format!("POSTDIV-COPY-ALPHA-SOURCE (copy-alpha arm: no host alpha plane for input {} on this route)", copy_alpha)))?
            .iter()
            .map(|x| x.to_bits())
            .collect();
        if alpha.len() != n {
            return Err(unsupported("postdivide", &format!("POSTDIV-COPY-ALPHA-SOURCE (copy-alpha arm: the host alpha plane has {} samples, the plane has {})", alpha.len(), n)));
        }
        crate::postdiv::postdiv_copy_alpha_into(&sref, &alpha, &route, n as i64, &mut out, crate::postdiv::PdMut::None).map_err(|er| unsupported("postdivide", &er.to_string()))?;
        return Ok(Image { n, planes: out.into_iter().map(|p| p.into_iter().map(f32::from_bits).collect()).collect() });
    }
    crate::postdiv::postdiv_into(&sref, e, copy_alpha, tag, n as i64, &mut out, crate::postdiv::PdMut::None).map_err(|er| unsupported("postdivide", &er.to_string()))?;
    Ok(Image { n, planes: out.into_iter().map(|p| p.into_iter().map(f32::from_bits).collect()).collect() })
}

/// v3_to_v4_if4_a(source, alpha, extra): the selected combo row runs its pass through the plane service. Pass F reads the
/// channel count of the second argument (INFERRED to be the value the class holds at this+0x70).
fn v3_to_v4_if4_a_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 3, "v3_to_v4_if4_a")?;
    let counts = [args[0].count(), args[1].count(), args[2].count()];
    let combo = v3_to_v4_if4_a::select_combo(counts).ok_or(ExecError::NoCombo { class: "v3_to_v4_if4_a".into(), operands: counts.to_vec() })?;
    if v3_to_v4_if4_a::COMBOS[combo].out as i32 != tag {
        return Err(ExecError::Kernel { class: "v3_to_v4_if4_a".into(), msg: format!("selector tag {} differs from the row result count {}", tag, v3_to_v4_if4_a::COMBOS[combo].out) });
    }
    let (p0, p1) = (args[0].planes(), args[1].planes());
    let (v0, v1) = (args[0].bits().to_vec(), args[1].bits().to_vec());
    let blocks = [block_of(&args[0], &p0, &v0), block_of(&args[1], &p1, &v1)];
    let p = v3_to_v4_if4_a::Params { combo, host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n), this_nch: args[1].count() };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; tag.max(0) as usize];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        v3_to_v4_if4_a::render(&p, &blocks, &mut outs).map_err(|e| kerr("v3_to_v4_if4_a", e))?;
    }
    Ok(Image { n, planes: outv })
}

/// get_alpha(source): plane 3 of a four-channel source, plane 1 of a two-channel source, else the constant 1.0 plane. One result plane.
fn get_alpha_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 1, "get_alpha")?;
    if tag != 1 {
        return Err(ExecError::Kernel { class: "get_alpha".into(), msg: format!("selector tag {} differs from the single result plane", tag) });
    }
    let p0 = args[0].planes();
    let v0 = args[0].bits().to_vec();
    let blocks = [block_of(&args[0], &p0, &v0)];
    let p = get_alpha::Params { count: args[0].count(), host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; 1];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        get_alpha::render(&p, &blocks, &mut outs).map_err(|e| kerr("get_alpha", e))?;
    }
    Ok(Image { n, planes: outv })
}

/// glow (render body of the glow layer class), called through the crate's `render_full` with the explicit parameters the
/// plan binds. Argument order: source, thresholds [1 or 3], stretch [2], widths [1 or 3], brightness [1 or 3], subpixel, edge mode,
/// show threshold (eight operands, as the program text calls it). The combination tag is the plane count (1 or 3). Render scale and
/// aspect come from the host record fields of `KParams`; the edge mode is the plan's operand.
fn glow_k(args: &[Arg], tag: i32, w: usize, h: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 8, "glow")?;
    let render_scale = if kp.blur_render_scale > 0 { kp.blur_render_scale } else { count_i32("glow", "the frame width", w)? };
    let src = image_or_const(&args[0], w * h, "glow")?;
    let c = tag.max(0) as usize;
    if (c != 1 && c != 3) || src.channels() < c || src.n != w * h {
        return Err(ExecError::Shape("glow: the combination tag must be 1 or 3 and the source must hold that many planes".into()));
    }
    let lanes = |a: &Arg, what: &str| -> Result<[f32; 3], ExecError> {
        let mut v = [0.0f32; 3];
        for (i, x) in v.iter_mut().enumerate().take(if c == 1 { 1 } else { 3 }) {
            *x = const_lane(a, if a.count() == 1 { 0 } else { i }, "glow", what)?;
        }
        Ok(v)
    };
    let gp = glow_layer::GlowParams {
        thresholds: lanes(&args[1], "thresholds")?,
        width_xy: [const_lane(&args[2], 0, "glow", "stretch")?, const_lane(&args[2], if args[2].count() == 1 { 0 } else { 1 }, "glow", "stretch")?],
        width: lanes(&args[3], "widths")?,
        brightness: lanes(&args[4], "brightness")?,
        subpixel: cvt_i32(const_lane(&args[5], 0, "glow", "subpixel")?),
        edge_mode: cvt_i32(const_lane(&args[6], 0, "glow", "edge_mode")?),
        show_threshold: cvt_i32(const_lane(&args[7], 0, "glow", "show_threshold")?),
        plane_count: c,
        render_scale,
        aspect: kp.blur_aspect,
    };
    let sp: Vec<glow_layer::Plane> = (0..c).map(|k| glow_layer::Plane { data: &src.planes[k], width: w, height: h, stride: w }).collect();
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; w * h]; c];
    {
        let mut dp: Vec<glow_layer::PlaneMut> = outv.iter_mut().map(|v| glow_layer::PlaneMut { data: v.as_mut_slice(), width: w, height: h, stride: w }).collect();
        glow_layer::render_full(&gp, &sp, &mut dp).map_err(|e| kerr("glow", e))?;
    }
    Ok(Image { n: w * h, planes: outv })
}

/// alpha_from_rgb_max rgb amount output: mode 4 of the builtin (the mode word of the object is 4 on every recorded plan). The
/// amount is the scale; the output operand only carries the output channel count, which the combo tag gives (4 for the
/// recorded 4 channel input). Output planes 0 to 2 copy the input colour channels, plane 3 is the scaled maximum of the three.
fn alpha_from_rgb_max_k(args: &[Arg], tag: i32, n: usize) -> Result<Image, ExecError> {
    need(args, 3, "alpha_from_rgb_max")?;
    if tag != 4 {
        return Err(unsupported("alpha_from_rgb_max", "only the 4 channel output combo is wired"));
    }
    let scale = const_lane(&args[1], 0, "alpha_from_rgb_max", "amount")?;
    let planes = args[0].planes();
    let bits = args[0].bits();
    let src = if args[0].is_const() {
        alpha_from_rgb_max::Source::Const { bits: &bits[..(args[0].count().clamp(0, 4) as usize)], is_int: false }
    } else {
        alpha_from_rgb_max::Source::Planes(&planes)
    };
    let p = alpha_from_rgb_max::Params { mode: 4, nch: args[0].count(), scale, nout: tag, n };
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; 4];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        alpha_from_rgb_max::run(&p, &src, &mut outs).map_err(|e| kerr("alpha_from_rgb_max", format!("{:?}", e)))?;
    }
    Ok(Image { n, planes: outv })
}

/// `append` (a, b): the planes of `a` followed by the planes of `b`, at most four; constants are broadcast.
fn append_k(args: &[Arg], tag: i32, n: usize, kp: &KParams) -> Result<Image, ExecError> {
    need(args, 2, "append")?;
    let (first, second) = (args[0].count().max(0) as usize, args[1].count().max(0) as usize);
    let nout = tag.max(0) as usize;
    if nout == 0 || nout > 4 {
        return Err(ExecError::Shape(format!("append: combo tag {} outside 1..4", tag)));
    }
    let p = ops_vec::Params { host: ps::Host { in_pool: kp.in_pool }, roi: flat_roi(n) };
    let planes: Vec<Vec<&[f32]>> = args.iter().take(2).map(|a| a.planes()).collect();
    let vals: Vec<Vec<u32>> = args.iter().take(2).map(|a| a.bits().to_vec()).collect();
    let blocks: Vec<ps::Block> = args.iter().take(2).enumerate().map(|(i, a)| block_of(a, &planes[i], &vals[i])).collect();
    let mut outv: Vec<Vec<f32>> = vec![vec![0.0; n]; 4];
    {
        let mut outs: Vec<&mut [f32]> = outv.iter_mut().map(|v| v.as_mut_slice()).collect();
        ops_vec::append::render(&p, &blocks, first, second, &mut outs).map_err(|e| kerr("append", e))?;
    }
    outv.truncate(nout);
    Ok(Image { n, planes: outv })
}

/// Runs one wired class. `tag` is the selector's result; `(w, h)` the frame.
pub fn run_wired(class: &str, tag: i32, args: &[Arg], n: usize, dims: (usize, usize), kp: &KParams) -> Result<Out, ExecError> {
    let (w, h) = dims;
    if w * h != n {
        return Err(ExecError::Shape(format!("{}: frame {}x{} has {} samples, plane length is {}", class, w, h, w * h, n)));
    }
    Ok(match class {
        "make_mipmap" => Out::Mip(make_mipmap_k(args, w, h, kp)?),
        "blur_warp" => Out::Img(blur_warp_k(args, tag, w, h, kp)?),
        "warp_rel" => Out::Img(warp_rel_k(args, tag, w, h, kp)?),
        "static2" => Out::Img(static2_k(args, n, kp)?),
        "monochrome" => Out::Img(monochrome_k(args, n, kp)?),
        "coerce_up" | "coerce_up_a1" | "coerce_to" | "coerce_to_alpha" => Out::Img(coerce_k(class, args, tag, n, kp)?),
        "rotate" => Out::Img(rotate_k(args, n)?),
        "max_element" | "min_element" => Out::Img(element_reduce_k(class, args, n, kp)?),
        "blur" => Out::Img(blur_k(args, tag, w, h, kp)?),
        "dissolve" => Out::Img(dissolve_k(args, tag, w, h)?),
        "coerce_to_v3" | "coerce_to_v4" | "coerce_up_to_v3" | "remove_alpha" => Out::Img(coerce_ext_k(class, args, tag, n, kp)?),
        "round" | "scale_rgb" => Out::Img(round_k(class, args, tag, n, kp)?),
        "add_rgb" => Out::Img(add_rgb_k(args, tag, n, kp)?),
        "masked_blur" => Out::Img(masked_blur_k(args, tag, w, h, kp)?),
        "comp_premult" => Out::Img(comp_premult_k(args, tag, n)?),
        "copy" => Out::Img(copy_k(args, tag, n, kp)?),
        "copy_alpha" => Out::Img(copy_alpha_k(args, tag, n, kp)?),
        "grad" => Out::Img(grad_k(args, tag, w, h, kp)?),
        "v3_to_v4" => Out::Img(v3_to_v4_k(args, tag, n, kp)?),
        "sharpen" => Out::Img(sharpen_k(args, tag, w, h, kp)?),
        "edge_aware_sharpen" => Out::Img(edge_aware_sharpen_k(args, tag, w, h, kp)?),
        "monochrome_a" => Out::Img(monochrome_a_k(args, tag, n, kp)?),
        "perspective" => Out::Img(perspective_k(args, w, h, kp)?),
        "perspective_alpha" => Out::Img(perspective_alpha_k(args, w, h, kp)?),
        "warp" => Out::Img(warp_k(args, tag, w, h, kp)?),
        "postdivide" => Out::Img(postdivide_k(args, tag, n)?),
        "glow" => Out::Img(glow_k(args, tag, w, h, kp)?),
        "alpha_from_rgb_max" => Out::Img(alpha_from_rgb_max_k(args, tag, n)?),
        "append" => Out::Img(append_k(args, tag, n, kp)?),
        "v3_to_v4_if4_a" => Out::Img(v3_to_v4_if4_a_k(args, tag, n, kp)?),
        "get_alpha" => Out::Img(get_alpha_k(args, tag, n, kp)?),
        other => return Err(unsupported(other, "not a wired class")),
    })
}
