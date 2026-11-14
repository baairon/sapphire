//! The composed host of slot 52's entry: one shipped type implementing `EntryHost` over the pyramid body's run path, the solve body's executor and the
//! entry's own launches, with one level table and one plane store between the two bodies (no byte block and no word index).
//!
//! The pyramid body (`optical_flow::join::run_body`) builds the pyramid in the composed host's planes; the same typed `LevelTable` is rebuilt by
//! `build_to_cut` over scratch copies (the build is deterministic in its inputs and the image service) and handed to the solve body's
//! `solve::run_body` with the same planes. The entry's four derivative launches are the solve body's `fetch::deriv_x` and `fetch::deriv_y`.

use crate::placement::{flow_level_dims, placement, FlowPlacement, HOST_PHASE_DEFAULT};
use crate::wiring::{run_solve_body, EntryRefusal, WiringOutcome};
use crate::{
    final_warp_rows, run_entry, scale_copy_model, EntryHost, EntryPath, EntryPlane, Launch, LaneParams, OutPlane, WarpPlanes,
    SCALE_THRESHOLD,
};
use optical_flow::body::{BodyResult, Refusal};
use optical_flow::host::{ComposedHost, FlowServices};
use optical_flow::iface::{Scratch, StageEnv, StageParams};
use optical_flow::join::{run_body, BodyEnd};
use optical_flow::{build_to_cut, BodyIn, ImgCall, Rect, SERVED_PLANES};
use optical_flow_pyramid::{LevelRecord, LevelTable, PlaneId};
use optical_flow_solve::fetch::{deriv_x, deriv_y, StencilConsts};
use optical_flow_solve::solve::{PlaneStore, KS_BITS};
use optical_flow_solve::Params;

/// The seven constants of the program call, in program order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowConstants {
    pub max_warps: i32,
    pub max_outer: i32,
    pub max_inner: i32,
    pub lambda: f32,
    /// The use_median operand, converted to an int.
    pub use_median: i32,
    pub texture_scale: f32,
    pub std_smooth: f32,
}

impl FlowConstants {
    /// The operator object's fields (INFERRED-OBJECT-LAYOUT): every operand has a field.
    pub fn obj_fields(&self) -> crate::ObjFields {
        crate::ObjFields {
            max_warps: self.max_warps,
            max_outer: self.max_outer,
            max_inner: self.max_inner,
            use_median: self.use_median,
            lambda_bits: self.lambda.to_bits() as i32,
            texture_scale: self.texture_scale,
            std_smooth_bits: self.std_smooth.to_bits() as i32,
        }
    }
}

/// The flow planes of a finished run, `width * height` words each, rows packed.
#[derive(Clone, Debug, PartialEq)]
pub struct FlowPlanes {
    pub width: usize,
    pub height: usize,
    pub flow_u: Vec<f32>,
    pub flow_v: Vec<f32>,
    pub confidence: Vec<f32>,
}

/// Why a run gives no planes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowStop {
    Refused(EntryRefusal),
    /// The pyramid body returned non-zero (fewer than two usable pyramid levels): the entry returns at once.
    PyramidStopped,
    /// The abort query stopped the run: no planes were read.
    Aborted,
    NoLevels,
    /// The host planes reported an error, or the inputs are shorter than the extent.
    Host(String),
}

/// Ids of the level 0 planes and the scratch planes (arena identifiers chosen by the host).
fn level0_record() -> LevelRecord {
    LevelRecord {
        image_a: 1,
        image_b: 2,
        flow_u: 3,
        flow_v: 4,
        nest1_t: 5,
        nest1_u: 6,
        nest2_t: 7,
        nest2_u: 8,
        warped_image: 9,
        warped_grad_x: 10,
        warped_grad_y: 11,
        coeff: 12,
        flow_u_next: 13,
        flow_v_next: 14,
        relax_first: 15,
        relax_second: 16,
        median_out: 17,
        grad_x: 18,
        grad_y: 19,
        copied_plane: 20,
        ..Default::default()
    }
}

/// The smoothing stages' scratch planes are level 0's own working planes (the original's stages use the shared planes of the level
/// table: warped_image as the result, warped_grad_x, warped_grad_y and coeff as the stencil planes, flow_u_next as the source copy), so
/// their content after the stages is what the later passes of both bodies start from.
fn smoothing_scratch() -> Scratch {
    let l = level0_record();
    Scratch { result: l.warped_image, flow_x: l.warped_grad_x, flow_y: l.warped_grad_y, work: l.coeff, source_copy: l.flow_u_next }
}
const LAUNCHER: u64 = 7;
const PLANE_SLACK: usize = 64;

/// Host image service: the plane id of `c.plane` at level `c.level` (never 0 for a served level, which is at least 1).
pub fn served_image(c: &ImgCall) -> u64 {
    ((c.level as u64) << 16) | c.plane.ordinal() as u64
}

fn refusal_of(r: Refusal) -> EntryRefusal {
    match r {
        Refusal::FlagRoute => EntryRefusal::FlagRoute,
        Refusal::SmallLevel => EntryRefusal::SmallLevel,
        Refusal::LevelTableFull => EntryRefusal::LevelTableFull,
        Refusal::NullImage => EntryRefusal::NullImage,
    }
}

/// Maps how the pyramid body ended to the entry's return value and, for a refusal, its row.
pub fn pyramid_body_result(end: BodyEnd) -> (i32, Option<EntryRefusal>) {
    match end {
        BodyEnd::ReturnedOne => (1, None),
        BodyEnd::ReturnedZero => (0, None),
        BodyEnd::Refused(r) | BodyEnd::Levels(BodyResult::Refused(r)) => (1, Some(refusal_of(r))),
        BodyEnd::Levels(BodyResult::Returned(v)) => (v, None),
    }
}

fn bytes_of(v: &[f32], len: usize) -> Vec<u8> {
    let mut b: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
    b.resize(len, 0);
    b
}

fn compact(data: &[f32], w: usize, h: usize, stride: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(w * h);
    for y in 0..h {
        out.extend_from_slice(&data[y * stride..y * stride + w]);
    }
    out
}

/// The solve body's view of the composed host's planes.
struct HostStore<'a> {
    host: &'a mut ComposedHost,
}

impl PlaneStore for HostStore<'_> {
    fn read(&self, id: u64) -> Vec<f32> {
        self.host.planes.get(&id).map(|p| p.data.clone()).unwrap_or_default()
    }
    fn write(&mut self, id: u64, v: &[f32]) {
        if let Some(p) = self.host.planes.get_mut(&id) {
            let n = v.len().min(p.data.len());
            p.data[..n].copy_from_slice(&v[..n]);
        }
    }
    fn align_bytes(&self, id: u64) -> u64 {
        self.host.planes.get(&id).map(|p| p.phase as u64).unwrap_or(0)
    }
}

/// The composed host: both bodies over one plane store.
pub struct ComposedFlowHost<'a, A: FnMut() -> bool> {
    prev: &'a [f32],
    next: &'a [f32],
    width: i32,
    height: i32,
    host_flag: i32,
    /// ALIGN-PHASE-HOST: byte phase of the render's first image allocation; the op's plane phases follow from it (`placement`).
    host_phase: u32,
    /// Every plane at this one byte phase in place of the placement rule (a host serving planes from aligned slots).
    uniform_phase: Option<u32>,
    img: &'a mut dyn FnMut(&ImgCall) -> u64,
    abort: A,
    host: ComposedHost,
    table: LevelTable,
    refusal: Option<EntryRefusal>,
    solve_stop: Option<FlowStop>,
    solve_aborted: bool,
    warp_scale: f32,
    outputs: [Option<Vec<f32>>; 3],
    output_reads: usize,
}

impl<'a, A: FnMut() -> bool> ComposedFlowHost<'a, A> {
    pub fn new(prev: &'a [f32], next: &'a [f32], width: i32, height: i32, host_flag: i32, img: &'a mut dyn FnMut(&ImgCall) -> u64, abort: A) -> Self {
        ComposedFlowHost {
            prev,
            next,
            width,
            height,
            host_flag,
            host_phase: HOST_PHASE_DEFAULT,
            uniform_phase: None,
            img,
            abort,
            host: ComposedHost::new(1),
            table: LevelTable::default(),
            refusal: None,
            solve_stop: None,
            solve_aborted: false,
            warp_scale: 0.0,
            outputs: [None, None, None],
            output_reads: 0,
        }
    }

    /// The same host with the ALIGN-PHASE-HOST value `phase` (byte phase of the render's first image allocation).
    pub fn with_host_phase(mut self, phase: u32) -> Self {
        self.host_phase = phase;
        self
    }

    /// The same host with every plane at the byte phase `phase` (a host that serves planes from aligned slots, as the entry
    /// harness does) in place of the placement rule.
    pub fn with_uniform_phase(mut self, phase: u32) -> Self {
        self.uniform_phase = Some(phase & 31);
        self
    }

    /// Records the placement's phases in the plane store: level 0 planes by id, the served planes by level and plane.
    fn apply_placement(&mut self, pl: &FlowPlacement) {
        let l = level0_record();
        let words = [
            l.warped_image, l.warped_grad_x, l.warped_grad_y, l.coeff, l.flow_u_next, l.flow_v_next, l.relax_first, l.relax_second, l.median_out,
            l.image_a, l.image_b, l.grad_x, l.grad_y, l.flow_u, l.flow_v, l.nest1_t, l.copied_plane,
        ];
        for (id, ph) in words.into_iter().zip(pl.descriptor) {
            self.host.phase_of.insert(id, ph as usize);
        }
        for (id, ph) in [l.nest1_u, l.nest2_t, l.nest2_u].into_iter().zip(pl.outputs) {
            self.host.phase_of.insert(id, ph as usize);
        }
        for (j, row) in pl.served.iter().enumerate() {
            for (plane, ph) in SERVED_PLANES.iter().zip(row) {
                self.host.served_phase.insert((j + 1, plane.ordinal() as usize), *ph as usize);
            }
        }
    }

    fn phase_of(&self, id: PlaneId) -> usize {
        self.host.phase_of.get(&id).copied().unwrap_or(0)
    }

    /// Number of plane reads made by the entry's own launches and copies (zero on every path that gives no planes).
    pub fn output_reads(&self) -> usize {
        self.output_reads
    }

    fn id_of(&self, sel: EntryPlane) -> PlaneId {
        let l = &self.table.levels[0];
        match sel {
            EntryPlane::FlowU => l.flow_u,
            EntryPlane::FlowV => l.flow_v,
            EntryPlane::FlowUNext => l.flow_u_next,
            EntryPlane::FlowVNext => l.flow_v_next,
            EntryPlane::Nest1T => l.nest1_t,
            EntryPlane::Nest1U => l.nest1_u,
            EntryPlane::Nest2T => l.nest2_t,
            EntryPlane::MedianOut => l.median_out,
        }
    }

    fn read_plane(&mut self, id: PlaneId) -> Vec<f32> {
        self.output_reads += 1;
        self.host.planes.get(&id).map(|p| p.data.clone()).unwrap_or_default()
    }

    fn write_plane(&mut self, id: PlaneId, v: &[f32]) {
        if let Some(p) = self.host.planes.get_mut(&id) {
            let n = v.len().min(p.data.len());
            p.data[..n].copy_from_slice(&v[..n]);
        }
    }

    fn compact_level0(&self, data: &[f32]) -> Vec<f32> {
        let l = &self.table.levels[0];
        compact(data, l.width as usize, l.height as usize, l.stride as usize)
    }

    fn slot(o: OutPlane) -> usize {
        match o {
            OutPlane::FlowU => 0,
            OutPlane::FlowV => 1,
            OutPlane::Confidence => 2,
        }
    }

    /// Registers the smoothing stages' planes with every word +0 and the zero copy-in images (the contract of cleared buffers).
    fn register_cleared_scratch(&mut self, n: usize) {
        let scratch = smoothing_scratch();
        for id in [scratch.result, scratch.flow_x, scratch.flow_y, scratch.work, scratch.source_copy] {
            let phase = self.phase_of(id);
            self.host.register_plane(id, vec![0.0; n + PLANE_SLACK], phase);
        }
        for id in [scratch.flow_x, scratch.flow_y, scratch.work] {
            self.host.inputs.insert(id, vec![0.0; n + PLANE_SLACK]);
        }
    }

    fn ensure_solve_planes(&mut self, dst_a: &[u8], dst_b: &[u8]) {
        let first_level = self.table.levels[0];
        for (id, bytes) in [(first_level.image_a, dst_a), (first_level.image_b, dst_b)] {
            if self.host.plane_words(id).is_none() {
                let words: Vec<f32> = bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
                let phase = self.phase_of(id);
                self.host.register_plane(id, words, phase);
            }
        }
        for (j, r) in self.table.levels.clone().into_iter().enumerate() {
            let n = (r.stride.max(0) as usize) * (r.height.max(0) as usize) + PLANE_SLACK;
            if j >= 1 {
                for plane in SERVED_PLANES {
                    let mut rec = r;
                    let id = *rec.plane_mut(plane);
                    self.host.ensure_level_plane(j, plane.ordinal() as usize, id, n);
                }
            }
            for id in [
                r.image_a, r.image_b, r.grad_x, r.grad_y, r.warped_image, r.warped_grad_x, r.warped_grad_y, r.coeff, r.flow_u, r.flow_v, r.flow_u_next,
                r.flow_v_next, r.nest1_t, r.nest1_u, r.nest2_t, r.nest2_u, r.relax_first, r.relax_second, r.median_out, r.copied_plane,
            ] {
                self.host.ensure_plane(id, n);
            }
        }
    }
}

impl<A: FnMut() -> bool> EntryHost for ComposedFlowHost<'_, A> {
    fn pyramid_body(&mut self, p: &LaneParams) -> i32 {
        let (w, h) = (self.width, self.height);
        let n = (w as usize) * (h as usize);
        let len = n * 4 + 256;
        let (src_a, src_b) = (bytes_of(self.prev, len), bytes_of(self.next, len));
        let (mut dst_a, mut dst_b) = (vec![0u8; len], vec![0u8; len]);
        let inp = BodyIn {
            rect: Rect { w, h, pitch: w, reserved: 0, q: 4 },
            levels: p.four,
            scale: p.diag_weight,
            flag: self.host_flag,
            smoothing_word: p.scale_gt,
            planes: level0_record(),
            src_a: &src_a,
            src_b: &src_b,
        };
        let scratch = smoothing_scratch();
        let env = StageEnv {
            launcher: LAUNCHER,
            params: StageParams { blend_weight: p.scale_bits, smooth_weight: p.std_smooth_bits as u32, flag_route: self.host_flag != 0, flag_cell_bits: 0 },
            scratch,
            table_word: 1,
        };
        // every plane's byte phase from the host placement (ALIGN-PHASE-HOST); the row sums key their vector order on it
        let dims = flow_level_dims(w, h, p.diag_weight, p.four);
        let placed = match self.uniform_phase {
            Some(u) => FlowPlacement { descriptor: [u; 17], outputs: [u; 3], served: vec![[u; 8]; dims.len().saturating_sub(1)] },
            None => placement(w, h, &dims, self.host_phase),
        };
        self.apply_placement(&placed);
        let (align_a, align_b) = (placed.descriptor[9] as usize, placed.descriptor[10] as usize);
        // the smoothing stages run in level 0's working planes; the three copy-in planes are cleared to zero by the stage (the original's
        // copy-in clears them), which the zero input images reproduce
        self.register_cleared_scratch(n);
        // the final warp's scale operand is the lambda word
        self.warp_scale = f32::from_bits(p.lambda_bits as u32);
        let end = run_body(&inp, &mut dst_a, &mut dst_b, &mut *self.img, align_a, align_b, &mut self.host, env);
        let (ret, refusal) = pyramid_body_result(end);
        if refusal.is_some() {
            self.refusal = refusal;
            return ret;
        }
        let (mut sa, mut sb) = (dst_a.clone(), dst_b.clone());
        let (_, table) = build_to_cut(&inp, &mut sa, &mut sb, &mut *self.img);
        self.table = table;
        if ret == 0 && !self.table.levels.is_empty() {
            self.ensure_solve_planes(&dst_a, &dst_b);
        }
        ret
    }

    fn solve_body(&mut self, p: &LaneParams) {
        let params = Params {
            outer_iters: p.max_warps,
            mid_iters: p.max_outer,
            inner_iters: p.max_inner,
            counted_route: p.use_median,
            flow_gain: p.lambda_bits as u32,
            relax_scale: p.smooth_weight.to_bits(),
            tol: p.tiny_epsilon,
            smooth_bias: p.small_weight.to_bits(),
            alt_launches: p.one,
        };
        let first = self.table.first();
        let levels = self.table.levels.clone();
        let ComposedFlowHost { host, abort, .. } = self;
        let mut store = HostStore { host };
        let mut poll = || abort();
        match run_solve_body(&levels, first, &params, &mut store, &mut poll) {
            WiringOutcome::Done => {}
            WiringOutcome::NoFrame => self.solve_aborted = true,
            WiringOutcome::Refused(r) => self.refusal = Some(r),
            WiringOutcome::NoLevels => self.solve_stop = Some(FlowStop::NoLevels),
        }
    }

    fn poll_abort(&mut self) -> bool {
        if self.solve_aborted || self.refusal.is_some() || self.solve_stop.is_some() {
            return true;
        }
        (self.abort)()
    }

    fn launch(&mut self, l: Launch) {
        let rec = self.table.levels[0];
        let (w, h, st) = (rec.width, rec.height, rec.stride);
        let sc = StencilConsts { k_s: f32::from_bits(KS_BITS), k_v: f32::from_bits(KS_BITS), c_s: 8.0, c_v: 8.0 };
        match l {
            Launch::DerivX { src, dst } => {
                let (src_id, dst_id) = (self.id_of(src), self.id_of(dst));
                let inp = self.read_plane(src_id);
                let mut out = self.read_plane(dst_id);
                let phase = (self.host.planes.get(&dst_id).map(|p| p.phase).unwrap_or(0) & 31) as u32;
                deriv_x(&inp, &mut out, w, st, 0, h, phase, &sc, None);
                self.write_plane(dst_id, &out);
            }
            Launch::DerivY { src, dst } => {
                let (src_id, dst_id) = (self.id_of(src), self.id_of(dst));
                let inp = self.read_plane(src_id);
                let mut out = self.read_plane(dst_id);
                deriv_y(&inp, &mut out, h, st, 0, w, &sc, None);
                self.write_plane(dst_id, &out);
            }
            Launch::FinalWarp => {
                let ids = [rec.image_a, rec.image_b, rec.flow_u, rec.flow_v, rec.flow_u_next, rec.flow_v_next, rec.nest1_t, rec.nest2_t];
                let v: Vec<Vec<f32>> = ids.iter().map(|id| self.read_plane(*id)).collect();
                let mut out = self.read_plane(rec.median_out);
                let planes = WarpPlanes {
                    width: w,
                    height: h,
                    stride: st,
                    target_plane: &v[0],
                    img: &v[1],
                    u: &v[2],
                    v: &v[3],
                    grad_a_x: &v[4],
                    grad_a_y: &v[5],
                    grad_b_x: &v[6],
                    grad_b_y: &v[7],
                    scale: self.warp_scale,
                };
                final_warp_rows(&planes, &mut out, 0, h);
                self.write_plane(rec.median_out, &out);
            }
        }
    }

    fn copy_plane(&mut self, src: EntryPlane, out: OutPlane) {
        let id = self.id_of(src);
        let data = self.read_plane(id);
        self.outputs[Self::slot(out)] = Some(self.compact_level0(&data));
    }

    fn scale_copy(&mut self, src: EntryPlane, out: OutPlane, k: f32) {
        let id = self.id_of(src);
        let data = self.read_plane(id);
        let packed = self.compact_level0(&data);
        let mut dst = vec![0.0f32; packed.len()];
        scale_copy_model(&mut dst, &packed, k);
        self.outputs[Self::slot(out)] = Some(dst);
    }
}

/// Runs slot 52's entry over two images of `width` by `height` words and the seven constants. `host_flag` is the host image handle flag
/// word (non zero is the refused flag route), `img` the host image service, `abort` the abort query. Planes are returned only for a run
/// that finished; every other path returns a stop and reads no plane. The planes take the reference host's placement
/// (`HOST_PHASE_DEFAULT`).
pub fn run_flow_with<A: FnMut() -> bool>(
    prev: &[f32],
    next: &[f32],
    width: i32,
    height: i32,
    c: &FlowConstants,
    host_flag: i32,
    img: &mut dyn FnMut(&ImgCall) -> u64,
    abort: A,
) -> Result<FlowPlanes, FlowStop> {
    run_flow_placed(prev, next, width, height, c, host_flag, HOST_PHASE_DEFAULT, img, abort)
}

/// `run_flow_with` for the ALIGN-PHASE-HOST value `host_phase` (byte phase of the render's first image allocation).
#[allow(clippy::too_many_arguments)]
pub fn run_flow_placed<A: FnMut() -> bool>(
    prev: &[f32],
    next: &[f32],
    width: i32,
    height: i32,
    c: &FlowConstants,
    host_flag: i32,
    host_phase: u32,
    img: &mut dyn FnMut(&ImgCall) -> u64,
    abort: A,
) -> Result<FlowPlanes, FlowStop> {
    run_flow_core(prev, next, width, height, c, host_flag, Some(host_phase), None, img, abort)
}

/// `run_flow` with every plane at the byte phase `phase` (a host serving planes from aligned slots, as the entry harness does).
pub fn run_flow_uniform<A: FnMut() -> bool>(prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants, phase: u32, abort: A) -> Result<FlowPlanes, FlowStop> {
    run_flow_core(prev, next, width, height, c, 0, None, Some(phase), &mut served_image, abort)
}

#[allow(clippy::too_many_arguments)]
fn run_flow_core<A: FnMut() -> bool>(
    prev: &[f32],
    next: &[f32],
    width: i32,
    height: i32,
    c: &FlowConstants,
    host_flag: i32,
    host_phase: Option<u32>,
    uniform_phase: Option<u32>,
    img: &mut dyn FnMut(&ImgCall) -> u64,
    abort: A,
) -> Result<FlowPlanes, FlowStop> {
    if width < 2 || height < 2 {
        return Err(FlowStop::Refused(EntryRefusal::SmallLevel));
    }
    let n = (width as usize) * (height as usize);
    if prev.len() < n || next.len() < n {
        return Err(FlowStop::Host("input image shorter than width times height".into()));
    }
    let mut h = ComposedFlowHost::new(prev, next, width, height, host_flag, img, abort).with_host_phase(host_phase.unwrap_or(HOST_PHASE_DEFAULT));
    if let Some(u) = uniform_phase {
        h = h.with_uniform_phase(u);
    }
    let path = run_entry(&mut h, &c.obj_fields(), SCALE_THRESHOLD, false);
    if let Some(r) = h.refusal {
        return Err(FlowStop::Refused(r));
    }
    if let Some(e) = h.solve_stop.take() {
        return Err(e);
    }
    if let Some(e) = &h.host.err {
        return Err(FlowStop::Host(format!("{e:?}")));
    }
    match path {
        EntryPath::PyramidStopped => Err(FlowStop::PyramidStopped),
        EntryPath::Aborted | EntryPath::Deferred => Err(FlowStop::Aborted),
        EntryPath::InLine => {
            let [u, v, conf] = std::mem::take(&mut h.outputs);
            match (u, v, conf) {
                (Some(flow_u), Some(flow_v), Some(confidence)) => Ok(FlowPlanes { width: width as usize, height: height as usize, flow_u, flow_v, confidence }),
                _ => Err(FlowStop::Host("an output plane was not produced".into())),
            }
        }
    }
}

/// `run_flow_with` over the served image service and no flag route.
pub fn run_flow<A: FnMut() -> bool>(prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants, abort: A) -> Result<FlowPlanes, FlowStop> {
    run_flow_with(prev, next, width, height, c, 0, &mut served_image, abort)
}

/// `run_flow` for the ALIGN-PHASE-HOST value `host_phase`.
pub fn run_flow_at<A: FnMut() -> bool>(prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants, host_phase: u32, abort: A) -> Result<FlowPlanes, FlowStop> {
    run_flow_placed(prev, next, width, height, c, 0, host_phase, &mut served_image, abort)
}


#[cfg(test)]
mod cleared_start {
    use super::*;

    /// CLEARED-START: at the start of a run the host holds no output and every working plane of level 0 that the stages use is +0 bits.
    #[test]
    fn working_planes_and_outputs_start_at_zero_bits() {
        let (a, b) = (vec![0.0f32; 12], vec![0.0f32; 12]);
        let mut img = served_image;
        let mut h = ComposedFlowHost::new(&a, &b, 4, 3, 0, &mut img, || false);
        assert!(h.outputs.iter().all(|o| o.is_none()));
        assert!(h.host.planes.is_empty());
        h.register_cleared_scratch(12);
        let l = level0_record();
        for id in [l.warped_image, l.warped_grad_x, l.warped_grad_y, l.coeff, l.flow_u_next] {
            let p = h.host.planes.get(&id).expect("registered");
            assert!(p.data.iter().all(|w| w.to_bits() == 0), "plane {id} not cleared");
        }
    }

    /// The smoothing planes are level 0's working planes, not separate planes.
    #[test]
    fn smoothing_scratch_is_level_zero_working_planes() {
        let (s, l) = (smoothing_scratch(), level0_record());
        assert_eq!((s.result, s.flow_x, s.flow_y, s.work, s.source_copy), (l.warped_image, l.warped_grad_x, l.warped_grad_y, l.coeff, l.flow_u_next));
    }
}
