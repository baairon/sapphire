//! End-to-end executor of the optical flow solve body 0x483ab0 (flag 0 and the counted route): the level loop of `body`,
//! every launch run as a call of the ported callback on the level records' planes, the stretch between levels and the tail blocks.
//! Ordinary typed Rust over planes addressed by handle through a `PlaneStore` supplied by the host. Host services modelled here (labels in `assumptions()`):
//! the row splitter (all rows in one call), the abort poll, the float32 reduction (reset, add, get), the rectangle clear and
//! the strided copy.
use crate::body::{small_level, Callback, LaunchArgs, Level, Params};
use crate::fetch::{deriv_x, deriv_y, polar, warp_fetch, FetchConsts, PolarConsts, StencilConsts};
use crate::median::median_rows;
use crate::relax::{relax_callback, Ctx, NestIn};
use crate::smooth::{smooth_normalise, Geo, Pass};
use crate::stretch::{run_stretch, Direct, HelperCall, Host as StretchHost, PairPlane, Planes, Tile};
use crate::tail::{median_pair, TailHost};
use crate::{diff_sum, flow_update, fma_mix, DiffPlanes, FlowIn, MixIn, Shape};
use fx_common::nanop::NanOps;

/// Cell constants of the AVX2 route as read from the original at load: DET-EPS 0x15f1240, the warp margin 0x15f1244,
/// the derivative scale 0x15f1248 and its vector twin 0x15f0a20, the weights 0x15e8544 and 0x15f0a40, the warp lower bound 0x15e852c.
pub const EPS_BITS: u32 = 0x2edb_e6ff;
pub const MARGIN_BITS: u32 = 0x3f80_0347;
pub const KS_BITS: u32 = 0xbdaa_aaab;
pub const WARP_LO_BITS: u32 = 0x38d1_b717;
pub const SMOOTH_WEIGHT_BITS: u32 = 0x3e7a_e148;

/// Polar cell 0x15f0c40. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_ARG2: f32 = f32::from_bits(0x3f666666);
/// Polar cell 0x15f1250. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_MUL_S: f32 = f32::from_bits(0xc0a00000);
/// Polar cell 0x15f0c60. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_MUL_V: f32 = f32::from_bits(0xc0a00000);
/// Polar cell 0x15f0c20. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_ABS_MASK: u32 = 0x7fffffff;
/// Polar cell 0x15f0c80. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_SUB: f32 = f32::from_bits(0x40400000);
/// Polar cell 0x15f0ca0. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_FIN: f32 = f32::from_bits(0xbf000000);
/// Polar threshold: the smallest positive normal f32. MEASURED-EMULATOR-READ (E2E); CELLS-E2E.
pub const POLAR_THR: f32 = f32::MIN_POSITIVE;

/// Cells of the polar launch of the alternate-launch block, read from the original's loaded image (MEASURED-EMULATOR-READ (E2E); CELLS-E2E).
pub const POLAR_CONSTS: PolarConsts = PolarConsts { arg2: POLAR_ARG2, mul_s: POLAR_MUL_S, mul_v: POLAR_MUL_V, abs_mask: POLAR_ABS_MASK, sub: POLAR_SUB, fin: POLAR_FIN, thr: POLAR_THR };

pub fn assumptions() -> Vec<&'static str> {
    vec![
        "HOST-ROW-SPLIT: the row splitter hands every callback all rows (start 0, count rows) in one call",
        "HOST-REDUCE: the reduction service adds the callback's float32 result to a float32 accumulator in call order; reset sets it to +0",
        "HOST-CLEAR: the rectangle clear (dst, flag, val, rect) fills width x height words of value val at the record's stride from the first element of dst",
        "COPY-DIRECTION: the strided copy (dst, src, flag, rect) copies width x height words at the record's stride, destination first (as the clear)",
        "ALT-BLOCK: when the alternate-launch word is nonzero, each level poll is followed by DerivX(image_a to grad_x), DerivY(image_a to grad_y) and Polar(grad_x, grad_y to copied_plane) before the gather; planes and order measured in the original's launch records",
        "CELLS-E2E: the cell constants of this module, the polar cells included, are the words read from the original at load",
    ]
}

/// How a run ends when it is not refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Done,
    Aborted,
}

/// The reasons a run is refused before any call or write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunRefused {
    SmallLevel,
    FirstIndex,
}

/// The host's planes, reached by handle. `read` returns a copy of the whole plane, `write` stores a copy of the given words
/// from the first element of the plane, `align_bytes` is the byte address of the plane's first element (the kernels use its low bits).
pub trait PlaneStore {
    fn read(&self, id: u64) -> Vec<f32>;
    fn write(&mut self, id: u64, v: &[f32]);
    fn align_bytes(&self, id: u64) -> u64;
    /// Called with each launch in order; the host may record it.
    fn launched(&mut self, _c: Callback) {}
}

/// The planes of the stretch: copies of the words of the finer record `l` (destination) and the coarser record `lp` (source),
/// written back to the store when the stretch ends.
struct StretchMem<'a, S: PlaneStore> {
    store: &'a mut S,
    l: &'a Level,
    lp: &'a Level,
    src_buf: Vec<Vec<f32>>,
    dst_buf: Vec<Vec<f32>>,
    /// output of the latest horizontal tile pass, written to the median_out plane when the stretch ends
    first_pass_out: Option<Vec<f32>>,
    /// whether the horizontal pass output is kept in the median_out plane: measured with the alternate-launch word nonzero only
    keep_first_pass: bool,
}

const PAIR_PLANES: [PairPlane; 6] = [PairPlane::FlowU, PairPlane::FlowV, PairPlane::Nest1T, PairPlane::Nest1U, PairPlane::Nest2T, PairPlane::Nest2U];

fn pair_index(j: PairPlane) -> usize {
    PAIR_PLANES.iter().position(|q| *q == j).unwrap()
}

impl<'a, S: PlaneStore> StretchMem<'a, S> {
    fn new(store: &'a mut S, l: &'a Level, lp: &'a Level, keep_first_pass: bool) -> Self {
        let src_buf = PAIR_PLANES.iter().map(|j| store.read(j.of(lp))).collect();
        let dst_buf = PAIR_PLANES.iter().map(|j| store.read(j.of(l))).collect();
        StretchMem { store, l, lp, src_buf, dst_buf, first_pass_out: None, keep_first_pass }
    }
    fn finish(self) {
        if let Some(sc) = &self.first_pass_out {
            let mut m = self.store.read(self.l.median_out);
            let n = sc.len().min(m.len());
            m[..n].copy_from_slice(&sc[..n]);
            self.store.write(self.l.median_out, &m);
        }
        for (i, j) in PAIR_PLANES.iter().enumerate() {
            self.store.write(j.of(self.lp), &self.src_buf[i]);
            self.store.write(j.of(self.l), &self.dst_buf[i]);
        }
    }
}

impl<'a, S: PlaneStore> Planes for StretchMem<'a, S> {
    fn src(&self, j: PairPlane) -> (&[f32], u32) {
        (&self.src_buf[pair_index(j)], (self.store.align_bytes(j.of(self.lp)) & 15) as u32)
    }
    fn src_mut(&mut self, j: PairPlane) -> &mut [f32] {
        &mut self.src_buf[pair_index(j)]
    }
    fn dst(&mut self, j: PairPlane) -> &mut [f32] {
        &mut self.dst_buf[pair_index(j)]
    }
    fn first_pass(&mut self, scratch: &[f32]) {
        if self.keep_first_pass {
            self.first_pass_out = Some(scratch.to_vec());
        }
    }
}

/// The stretch's host: the shipped `Direct` calls, with the launch order recorded.
struct Launching<'a>(&'a mut Vec<Callback>);

impl<'a> StretchHost for Launching<'a> {
    fn helper(&mut self, c: HelperCall) {
        Direct.helper(c)
    }
    fn tile_pair<P: Planes>(&mut self, mem: &mut P, t: &Tile) {
        self.0.extend([Callback::TilePairFirst, Callback::TilePairSecond]);
        Direct.tile_pair(mem, t)
    }
    fn scale<P: Planes>(&mut self, mem: &mut P, j: PairPlane, l: &Level, ratio: f32) {
        self.0.push(Callback::Scale);
        Direct.scale(mem, j, l, ratio)
    }
    fn border<P: Planes>(&mut self, mem: &mut P, j: PairPlane, lp: &Level) {
        self.0.push(Callback::Border);
        Direct.border(mem, j, lp)
    }
}

/// Rectangle clear of the body's first step.
fn clear<S: PlaneStore>(store: &mut S, dst: u64, val: u32, l: &Level) {
    let (cw, rows_h, s) = (l.width as u32, l.height as u32, l.stride as u32 as usize);
    if cw > 0 && cw < 0x10000 && rows_h > 0 && rows_h < 0x10000 {
        let mut d = store.read(dst);
        for y in 0..rows_h as usize {
            for x in 0..cw as usize {
                d[y * s + x] = f32::from_bits(val);
            }
        }
        store.write(dst, &d);
    }
}

fn copy_rect<S: PlaneStore>(store: &mut S, dst: u64, src: u64, l: &Level) {
    let (cw, rows_h, s) = (l.width as u32, l.height as u32, l.stride as u32 as usize);
    if cw > 0 && cw < 0x10000 && rows_h > 0 && rows_h < 0x10000 {
        let from = store.read(src);
        let mut d = store.read(dst);
        for y in 0..rows_h as usize {
            for x in 0..cw as usize {
                d[y * s + x] = from[y * s + x];
            }
        }
        store.write(dst, &d);
    }
}

/// The tail's host: the median launch and the copy services run on the store.
struct TailWorld<'a, S: PlaneStore> {
    store: &'a mut S,
    l: Level,
}

impl<'a, S: PlaneStore> TailHost for TailWorld<'a, S> {
    fn launch(&mut self, args: LaunchArgs) {
        self.store.launched(args.callback());
        let LaunchArgs::Median(m) = args else { panic!("the tail launches only the median") };
        let (src, dst) = (m.plane, m.median_out);
        let (cw, rows, pitch) = (self.l.width, self.l.height, self.l.stride);
        let s: Vec<u32> = self.store.read(src).iter().map(|f| f.to_bits()).collect();
        let mut d: Vec<u32> = self.store.read(dst).iter().map(|f| f.to_bits()).collect();
        let _ = median_rows(&s, &mut d, cw, rows, pitch, 0, rows);
        let out: Vec<f32> = d.iter().map(|b| f32::from_bits(*b)).collect();
        self.store.write(dst, &out);
    }
    fn copy_back(&mut self, plane: u64, median_out: u64, _pitch: u64, _head: [u64; 3]) {
        let l = self.l;
        copy_rect(self.store, plane, median_out, &l);
    }
}

/// Runs the body from level `idx` down to 0. `poll` is the abort poll: true stops the run.
pub fn run_body<S: PlaneStore>(levels: &[Level], idx: i32, p: &Params, store: &mut S, poll: &mut dyn FnMut() -> bool) -> Result<Outcome, RunRefused> {
    if small_level(levels, idx) {
        return Err(RunRefused::SmallLevel);
    }
    if idx < 0 || idx as usize >= levels.len() {
        return Err(RunRefused::FirstIndex);
    }
    let mut acc: f32;
    if poll() {
        return Ok(Outcome::Aborted);
    }
    let level_rec = &levels[idx as usize];
    for plane in [level_rec.flow_u, level_rec.flow_v, level_rec.nest1_t, level_rec.nest1_u, level_rec.nest2_t, level_rec.nest2_u] {
        clear(store, plane, 0, level_rec);
    }
    let mut k = idx;
    while k >= 0 {
        let l = &levels[k as usize];
        if poll() {
            return Ok(Outcome::Aborted);
        }
        if p.alt_launches != 0 {
            launch_alt(store, l);
        }
        launch_gather(store, l);
        if p.outer_iters > 0 {
            let mut outer_iter = 0i32;
            loop {
                launch_warps(store, l);
                launch_mix(store, l);
                let mut stop = false;
                if p.mid_iters > 0 {
                    let mut mid_iter = 0i32;
                    'mid: loop {
                        launch_flow(store, l, p);
                        if p.inner_iters > 0 {
                            let mut j = 0i32;
                            loop {
                                launch_relax(store, l, p);
                                launch_smooth(store, l, p);
                                acc = 0.0;
                                launch_diff(store, l, &mut acc);
                                let cnt = l.width.wrapping_mul(l.height);
                                let ratio = acc.fdiv(cnt as f32);
                                if p.tol > ratio {
                                    stop = true;
                                    break 'mid;
                                }
                                j = j.wrapping_add(1);
                                if j >= p.inner_iters {
                                    break;
                                }
                            }
                        }
                        mid_iter = mid_iter.wrapping_add(1);
                        if mid_iter >= p.mid_iters {
                            break;
                        }
                    }
                }
                let _ = stop;
                if p.counted_route != 0 {
                    let mut th = TailWorld { store: &mut *store, l: *l };
                    median_pair(l, &mut th);
                }
                outer_iter = outer_iter.wrapping_add(1);
                if outer_iter >= p.outer_iters {
                    break;
                }
            }
        }
        if k > 0 {
            let (lf, lc) = (&levels[(k - 1) as usize], l);
            let pair_state = 0u64;
            let mut launches: Vec<Callback> = Vec::new();
            {
                let mut mem = StretchMem::new(&mut *store, lf, lc, p.alt_launches != 0);
                run_stretch(lf, lc, pair_state, 0, &mut mem, &mut Launching(&mut launches));
                mem.finish();
            }
            for c in launches {
                store.launched(c);
            }
        }
        k -= 1;
    }
    Ok(Outcome::Done)
}

fn dims(l: &Level) -> (i32, i32, i32) {
    (l.width, l.height, l.stride)
}

/// The alternate-launch block (original 0x486b26, returning to 0x483d21): the derivative pair on image_a into the gradient planes,
/// then the polar magnitude of the two gradients into the descriptor plane.
fn launch_alt<S: PlaneStore>(store: &mut S, l: &Level) {
    let (cw, rows_h, st) = dims(l);
    let sc = StencilConsts { k_s: f32::from_bits(KS_BITS), k_v: f32::from_bits(KS_BITS), c_s: 8.0, c_v: 8.0 };
    store.launched(Callback::DerivX);
    let inp = store.read(l.image_a);
    let mut gx = store.read(l.grad_x);
    deriv_x(&inp, &mut gx, cw, st, 0, rows_h, (store.align_bytes(l.grad_x) & 31) as u32, &sc, None);
    store.write(l.grad_x, &gx);
    store.launched(Callback::DerivY);
    let mut gy = store.read(l.grad_y);
    deriv_y(&inp, &mut gy, rows_h, st, 0, cw, &sc, None);
    store.write(l.grad_y, &gy);
    store.launched(Callback::Polar);
    let (gx, gy) = (store.read(l.grad_x), store.read(l.grad_y));
    let mut out = store.read(l.copied_plane);
    // The original's task block binds the y gradient to the P operand (+0x20) and the x gradient to the Q operand (+0x18) of the polar callback (0x4793e0);
    // the row peel phase is taken from the P plane's slot.
    polar(&gy, &gx, &mut out, cw, st, 0, rows_h, (store.align_bytes(l.grad_y) & 31) as u32, &POLAR_CONSTS);
    store.write(l.copied_plane, &out);
}

fn launch_gather<S: PlaneStore>(store: &mut S, l: &Level) {
    let (cw, rows_h, st) = dims(l);
    let sc = StencilConsts { k_s: f32::from_bits(KS_BITS), k_v: f32::from_bits(KS_BITS), c_s: 8.0, c_v: 8.0 };
    store.launched(Callback::DerivX);
    let inp = store.read(l.image_b);
    let mut o = store.read(l.grad_x);
    deriv_x(&inp, &mut o, cw, st, 0, rows_h, (store.align_bytes(l.grad_x) & 31) as u32, &sc, None);
    store.write(l.grad_x, &o);
    store.launched(Callback::DerivY);
    let mut o = store.read(l.grad_y);
    deriv_y(&inp, &mut o, rows_h, st, 0, cw, &sc, None);
    store.write(l.grad_y, &o);
}

fn launch_warps<S: PlaneStore>(store: &mut S, l: &Level) {
    let (cw, rows_h, st) = dims(l);
    let fc = FetchConsts { lo: f32::from_bits(WARP_LO_BITS), margin: f32::from_bits(MARGIN_BITS) };
    for (s, o) in [(l.image_b, l.warped_image), (l.grad_x, l.warped_grad_x), (l.grad_y, l.warped_grad_y)] {
        store.launched(Callback::Warp);
        let (flow_x, dy, src) = (store.read(l.flow_u), store.read(l.flow_v), store.read(s));
        let mut out = store.read(o);
        warp_fetch(&flow_x, &dy, &src, &mut out, cw, rows_h, st, 0, rows_h, &fc);
        store.write(o, &out);
    }
}

fn launch_mix<S: PlaneStore>(store: &mut S, l: &Level) {
    let (cw, rows_h, st) = dims(l);
    store.launched(Callback::Mix);
    let (a, b, c, d, e, f) = (store.read(l.image_a), store.read(l.warped_image), store.read(l.warped_grad_x), store.read(l.warped_grad_y), store.read(l.flow_u), store.read(l.flow_v));
    let mut o = store.read(l.coeff);
    let sh = Shape { width: cw, stride: st, phase_bytes: (store.align_bytes(l.coeff) & 31) as u32 };
    fma_mix(&MixIn { a: &a, b: &b, c: &c, d: &d, e: &e, f: &f }, &mut o, &sh, 0, rows_h);
    store.write(l.coeff, &o);
}

fn launch_flow<S: PlaneStore>(store: &mut S, l: &Level, p: &Params) {
    let (cw, rows_h, st) = dims(l);
    store.launched(Callback::Flow);
    let (u, v, a, b, c) = (store.read(l.flow_u), store.read(l.flow_v), store.read(l.warped_grad_x), store.read(l.warped_grad_y), store.read(l.coeff));
    let (mut flow_u_out, mut flow_v_out) = (store.read(l.flow_u_next), store.read(l.flow_v_next));
    flow_update(&FlowIn { u: &u, v: &v, a: &a, b: &b, c: &c }, &mut flow_u_out, &mut flow_v_out, cw, st, 0, rows_h, f32::from_bits(p.relax_scale), f32::from_bits(p.flow_gain), f32::from_bits(EPS_BITS));
    store.write(l.flow_u_next, &flow_u_out);
    store.write(l.flow_v_next, &flow_v_out);
}

fn launch_relax<S: PlaneStore>(store: &mut S, l: &Level, p: &Params) {
    let (cw, rows_h, st) = dims(l);
    store.launched(Callback::Relax);
    let (nest1_t, nest1_u, nest1_acc) = (store.read(l.nest1_t), store.read(l.nest1_u), store.read(l.flow_u_next));
    let (nest2_t, nest2_u, nest2_acc) = (store.read(l.nest2_t), store.read(l.nest2_u), store.read(l.flow_v_next));
    let (mut relax_out_first, mut relax_out_second) = (store.read(l.relax_first), store.read(l.relax_second));
    // At the alternate-launch word the original's relax callback (0x460fd0) reads the polar output plane as its scale plane, the same plane the
    // smooth launch takes as its third argument; with the word zero the scale plane is absent. The stored launch registers do not show the pointer,
    // so the plane is identified by the words the launches write.
    let scale_plane = if p.alt_launches != 0 { Some(store.read(l.copied_plane)) } else { None };
    let ctx = Ctx {
        width: cw,
        height: rows_h,
        stride: st,
        scale: f32::from_bits(p.relax_scale),
        v: scale_plane.as_deref(),
        nests: [NestIn { t: &nest1_t, u: &nest1_u, acc: &nest1_acc }, NestIn { t: &nest2_t, u: &nest2_u, acc: &nest2_acc }],
        phase_bytes: (store.align_bytes(l.nest1_t) & 63) as u32,
    };
    relax_callback(&ctx, [&mut relax_out_first, &mut relax_out_second], 0, rows_h);
    store.write(l.relax_first, &relax_out_first);
    store.write(l.relax_second, &relax_out_second);
}

fn launch_smooth<S: PlaneStore>(store: &mut S, l: &Level, p: &Params) {
    let (cw, rows_h, st) = dims(l);
    store.launched(Callback::Smooth);
    let (pa, pd) = (store.read(l.relax_first), store.read(l.relax_second));
    // The original's smooth launch (callback 0x46b9d0) passes the polar output plane as its third argument (register rdx) when the alternate-launch
    // word is nonzero and null otherwise.
    let scale_plane = if p.alt_launches != 0 { Some(store.read(l.copied_plane)) } else { None };
    let (mut pb, mut pc, mut pe, mut pf) = (store.read(l.nest1_t), store.read(l.nest1_u), store.read(l.nest2_t), store.read(l.nest2_u));
    let geo = Geo { width: cw, height: rows_h, stride: st, relax_scale: f32::from_bits(p.relax_scale), smooth_weight: f32::from_bits(SMOOTH_WEIGHT_BITS), c: f32::from_bits(p.smooth_bias) };
    {
        let mut pass_first = Pass { src: &pa, p: &mut pb, q: &mut pc };
        let mut pass_second = Pass { src: &pd, p: &mut pe, q: &mut pf };
        smooth_normalise(&geo, scale_plane.as_deref(), &mut pass_first, &mut pass_second, 0, rows_h as u32);
    }
    store.write(l.nest1_t, &pb);
    store.write(l.nest1_u, &pc);
    store.write(l.nest2_t, &pe);
    store.write(l.nest2_u, &pf);
}

fn launch_diff<S: PlaneStore>(store: &mut S, l: &Level, acc: &mut f32) {
    let (cw, rows_h, st) = dims(l);
    store.launched(Callback::Diff);
    let (pp, pq) = (store.read(l.relax_second), store.read(l.relax_first));
    let (mut x, mut y) = (store.read(l.flow_v), store.read(l.flow_u));
    let sh = Shape { width: cw, stride: st, phase_bytes: (store.align_bytes(l.flow_v) & 31) as u32 };
    let r = diff_sum(&DiffPlanes { p: &pp, q: &pq }, &mut x, &mut y, &sh, 0, rows_h);
    store.write(l.flow_v, &x);
    store.write(l.flow_u, &y);
    *acc = acc.fadd(r);
}

#[cfg(test)]
mod mix_launch_tests {
    use super::*;
    use std::collections::HashMap;

    struct Planes(HashMap<u64, Vec<f32>>);

    impl PlaneStore for Planes {
        fn read(&self, id: u64) -> Vec<f32> {
            self.0[&id].clone()
        }
        fn write(&mut self, id: u64, v: &[f32]) {
            self.0.get_mut(&id).unwrap()[..v.len()].copy_from_slice(v);
        }
        fn align_bytes(&self, id: u64) -> u64 {
            id * 64
        }
    }

    fn level(side: i32) -> Level {
        let p = |i: u64| 0x1000 + i;
        Level {
            width: side,
            height: side,
            stride: side,
            rect_word3: 0,
            elem_bytes: 4,
            index: 0,
            image_a: p(0), image_b: p(1), grad_x: p(2), grad_y: p(3),
            warped_image: p(4), warped_grad_x: p(5), warped_grad_y: p(6), coeff: p(7),
            flow_u: p(8), flow_v: p(9), flow_u_next: p(10), flow_v_next: p(11),
            nest1_t: p(12), nest1_u: p(13), nest2_t: p(14), nest2_u: p(15),
            relax_first: p(16), relax_second: p(17), median_out: p(18),
            copied_plane: p(19),
        }
    }

    /// The mix launch reads the warped x gradient as the third and the warped y gradient as the fourth operand of
    /// out = -(u*gx - warped_image) - (v*gy + image_a). Distinct constant planes make a swapped pair change every word.
    #[test]
    fn the_mix_launch_binds_the_warped_gradients_in_order() {
        for side in [5, 40] {
            let l = level(side);
            let n = (side * side) as usize;
            let mut m = HashMap::new();
            for (id, v) in [
                (l.image_a, 1.0f32), (l.warped_image, 2.0), (l.warped_grad_x, 3.0), (l.warped_grad_y, 5.0), (l.flow_u, 7.0), (l.flow_v, 11.0), (l.coeff, 0.0),
            ] {
                m.insert(id, vec![v; n]);
            }
            let mut store = Planes(m);
            launch_mix(&mut store, &l);
            // -(7*3 - 2) - (11*5 + 1) = -75; with the gradients swapped it would be -67
            assert!(store.0[&l.coeff].iter().all(|w| w.to_bits() == (-75.0f32).to_bits()), "side {}", side);
        }
    }
}
