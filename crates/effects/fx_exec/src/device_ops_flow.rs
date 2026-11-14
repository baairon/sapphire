//! The `optical_flow` op on the device: the crate's own control code (`optical_flow_entry::run_entry`, `optical_flow::join::run_body`
//! with its level loop, `run_stretch`, `median_pair`) runs on the host through its public traits, and every plane launch is a device
//! kernel on a device resident plane store. The solve body `optical_flow_solve::solve::run_body` is generic over a store that carries data
//! only, so [`dev_solve_body`] is a host transcription of it whose launches are device kernels; the sequence guard test compares it with the
//! crate on recorded launches and plane words.
//!
//! Exactness: every kernel is proven word for word against its crate function by its own piece (`device_ops_flow_solve_a`, `_b`, `_fetch`)
//! and by the compare drivers of this module; the host parts are the crate's own code.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_flow_fetch as fetch;
use crate::device_ops_flow_solve_a as sa;
use crate::device_ops_flow_solve_b as sb;
use crate::types::{Arg, Image};
use fx_common::nanop::NanOps;
use fx_device::ocl::{Arg as ClArg, Buffer};
use fx_device::{DevError, DevImage, Session};
use optical_flow::host::{chunk, FlowServices, HelperName, HostError, ResampleTargets};
use optical_flow::host_types::{Call, Callback as FlowCb, Ctx, Host, Rect24};
use optical_flow_entry::compose::{
    bytes_of, level0_record, pyramid_body_result, served_image, smoothing_scratch, FlowConstants, LAUNCHER, PLANE_SLACK,
};
use optical_flow_entry::wiring::{refusal_text, EntryRefusal};
use optical_flow_entry::{run_entry, EntryHost, EntryPath, EntryPlane, Launch, LaneParams, OutPlane, SCALE_THRESHOLD};
use optical_flow_pyramid::{LevelRecord as Level, LevelTable, PlaneId};
use optical_flow_solve::body::{small_level, Callback, LaunchArgs, Params};
use optical_flow_solve::smooth::Geo;
use optical_flow_solve::solve::{Outcome, RunRefused, EPS_BITS, KS_BITS, MARGIN_BITS, POLAR_CONSTS, SMOOTH_WEIGHT_BITS, WARP_LO_BITS};
use optical_flow_solve::stretch::{run_stretch, Host as StretchHost, HelperCall, PairPlane, Planes, Tile};
use optical_flow_solve::tail::{median_pair, TailHost};
use optical_flow_solve::Shape;
use std::cell::{OnceCell, RefCell};
use std::collections::BTreeMap;
use std::sync::Arc;

pub const OP_CLASS: &str = "optical_flow";

fn refuse(why: &str) -> DevError {
    DevError::Unsupported(format!("optical_flow device: {why}"))
}

// ---------------------------------------------------------------------------------------------------------------------------------
// Kernel launch functions of this module (add_const, normalize_flow_rows, clear, copies, compaction)
// ---------------------------------------------------------------------------------------------------------------------------------

/// `optical_flow::add_const` over the first `n` words of `plane` (the crate's `take(n)`: `n` above the length is clamped by the caller).
pub fn add_const_dev(s: &Arc<Session>, plane: &Buffer<f32>, n: usize, c: f32) -> Result<(), DevError> {
    let n = n.min(plane.len());
    if n == 0 {
        return Ok(());
    }
    if n > u32::MAX as usize {
        return Err(refuse("add_const count above 32 bits"));
    }
    s.launch("flow_d_add_const", n, &[plane.arg(), ClArg::U32(n as u32), ClArg::U32(c.to_bits())])?;
    Ok(())
}

/// `optical_flow::normalize_flow_rows`: rows `row0..row0+rows` of a `width` wide plane pair updated in place from the reference plane `p`.
#[allow(clippy::too_many_arguments)]
pub fn normalize_flow_rows_dev(
    s: &Arc<Session>,
    p: &Buffer<f32>,
    fx: &Buffer<f32>,
    fy: &Buffer<f32>,
    width: i32,
    total_rows: i32,
    stride: i32,
    row0: i32,
    rows: i32,
    num: f32,
    den: f32,
) -> Result<(), DevError> {
    if row0 >= row0.wrapping_add(rows) || width <= 0 {
        return Ok(());
    }
    if rows < 0 || row0 < 0 || stride < width || total_rows < 1 || row0 as i64 + rows as i64 > total_rows as i64 {
        return Err(refuse("normalize_flow_rows outside its model"));
    }
    let need = total_rows as i64 * stride as i64;
    if (p.len() as i64) < need || (fx.len() as i64) < need || (fy.len() as i64) < need {
        return Err(refuse("normalize_flow_rows plane shorter than rows times stride"));
    }
    let total = rows as i64 * width as i64;
    if total > u32::MAX as i64 {
        return Err(refuse("normalize_flow_rows element count above 32 bits"));
    }
    s.launch(
        "flow_d_normalize_flow_rows",
        total as usize,
        &[p.arg(), fx.arg(), fy.arg(), ClArg::I32(width), ClArg::I32(total_rows), ClArg::I32(stride), ClArg::I32(row0), ClArg::U32(num.to_bits()), ClArg::U32(den.to_bits()), ClArg::U32(total as u32)],
    )?;
    Ok(())
}

/// The size guard of `solve::clear` and `solve::copy_rect`: a width and a height inside 1 to 0xffff.
fn rect_ok(l: &Level) -> bool {
    let (cw, rows) = (l.width as u32, l.height as u32);
    cw > 0 && cw < 0x10000 && rows > 0 && rows < 0x10000
}

fn rect_need(l: &Level) -> usize {
    let (cw, rows, st) = (l.width as u32 as usize, l.height as u32 as usize, l.stride as u32 as usize);
    (rows - 1) * st + cw
}

/// `solve::clear`: fills the `width x height` rectangle of the level record at its stride with the word `val`.
pub fn flow_clear_rect(s: &Arc<Session>, plane: &Buffer<f32>, val: u32, l: &Level) -> Result<(), DevError> {
    if !rect_ok(l) {
        return Ok(());
    }
    if rect_need(l) > plane.len() {
        return Err(refuse("clear outside the plane"));
    }
    s.launch(
        "flow_d_clear_rect",
        l.width as usize * l.height as usize,
        &[plane.arg(), ClArg::U32(l.width as u32), ClArg::U32(l.height as u32), ClArg::U32(l.stride as u32), ClArg::U32(val)],
    )?;
    Ok(())
}

/// `solve::copy_rect`: copies the `width x height` rectangle of the level record at its stride from `src` to `dst`.
pub fn flow_copy_rect(s: &Arc<Session>, dst: &Buffer<f32>, src: &Buffer<f32>, l: &Level) -> Result<(), DevError> {
    if !rect_ok(l) {
        return Ok(());
    }
    if rect_need(l) > dst.len() || rect_need(l) > src.len() {
        return Err(refuse("copy outside the planes"));
    }
    s.launch(
        "flow_d_copy_rect",
        l.width as usize * l.height as usize,
        &[dst.arg(), src.arg(), ClArg::U32(l.width as u32), ClArg::U32(l.height as u32), ClArg::U32(l.stride as u32)],
    )?;
    Ok(())
}

/// Copies the first `n` words of `src` to `dst`.
pub fn flow_copy_prefix(s: &Arc<Session>, dst: &Buffer<f32>, src: &Buffer<f32>, n: usize) -> Result<(), DevError> {
    if n == 0 {
        return Ok(());
    }
    if n > dst.len() || n > src.len() || n > u32::MAX as usize {
        return Err(refuse("prefix copy outside the planes"));
    }
    s.launch("flow_d_copy_prefix", n, &[dst.arg(), src.arg(), ClArg::U32(n as u32)])?;
    Ok(())
}

/// `compose::compact`: the `w x h` rows of a plane of row stride `stride` packed into `dst` (row stride `w`).
pub fn flow_compact(s: &Arc<Session>, dst: &Buffer<f32>, src: &Buffer<f32>, w: i32, h: i32, stride: i32) -> Result<(), DevError> {
    if w <= 0 || h <= 0 {
        return Ok(());
    }
    if stride < w {
        return Err(refuse("compaction with a stride below the width"));
    }
    let n = w as i64 * h as i64;
    if n > u32::MAX as i64 || (n as usize) > dst.len() || ((h as usize - 1) * stride as usize + w as usize) > src.len() {
        return Err(refuse("compaction outside the planes"));
    }
    s.launch("flow_d_compact", n as usize, &[dst.arg(), src.arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::U32(stride as u32)])?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------------------
// The device plane store
// ---------------------------------------------------------------------------------------------------------------------------------

/// One plane: the device buffer, its length in words, the registered phase (the CPU's `Plane.phase`) and a lazy host mirror.
pub struct DevPlane {
    pub buf: Buffer<f32>,
    pub len: usize,
    pub phase: usize,
    mirror: OnceCell<Vec<f32>>,
}

/// The device form of `ComposedHost.planes` and `inputs` for one node.
pub struct DevPlanes {
    pub s: Arc<Session>,
    planes: BTreeMap<u64, DevPlane>,
    pub inputs: BTreeMap<u64, Vec<f32>>,
    /// The CPU's `err` (first error kept).
    pub err: Option<HostError>,
    dev_err: RefCell<Option<DevError>>,
    /// Counters for the transfer log: device to host reads, host to device uploads of plane data.
    pub reads: std::cell::Cell<usize>,
    pub uploads: usize,
    /// Byte phase of a plane by id, from the host placement (planes not listed take phase 0), as `ComposedHost.phase_of`.
    pub phase_of: BTreeMap<u64, usize>,
    /// Byte phase of a served plane by (level, plane ordinal), as `ComposedHost.served_phase`.
    pub served_phase: BTreeMap<(usize, usize), usize>,
}

impl DevPlanes {
    pub fn new(s: &Arc<Session>) -> DevPlanes {
        DevPlanes { s: s.clone(), planes: BTreeMap::new(), inputs: BTreeMap::new(), err: None, dev_err: RefCell::new(None), reads: std::cell::Cell::new(0), uploads: 0, phase_of: BTreeMap::new(), served_phase: BTreeMap::new() }
    }

    pub fn fail(&mut self, e: HostError) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }

    fn set_dev(&self, e: DevError) {
        let mut d = self.dev_err.borrow_mut();
        if d.is_none() {
            *d = Some(e);
        }
    }

    /// Whether a device error is pending (any later method returns at once).
    pub fn broken(&self) -> bool {
        self.err.is_some() || self.dev_err.borrow().is_some()
    }

    pub fn take_dev_err(&self) -> Option<DevError> {
        self.dev_err.borrow_mut().take()
    }

    /// Runs a device step; the first error is kept and `None` returned.
    pub fn dev<T>(&self, r: Result<T, DevError>) -> Option<T> {
        match r {
            Ok(v) => Some(v),
            Err(e) => {
                self.set_dev(e);
                None
            }
        }
    }

    /// Every resident plane id with its registered byte phase.
    pub fn phases(&self) -> BTreeMap<u64, usize> {
        self.planes.iter().map(|(k, p)| (*k, p.phase)).collect()
    }

    pub fn has(&self, id: u64) -> bool {
        self.planes.contains_key(&id)
    }

    pub fn len_of(&self, id: u64) -> Option<usize> {
        self.planes.get(&id).map(|p| p.len)
    }

    pub fn phase(&self, id: u64) -> usize {
        self.planes.get(&id).map(|p| p.phase).unwrap_or(0)
    }

    /// The buffer of a plane, or a refusal for an unregistered handle.
    pub fn buf(&self, id: u64) -> Result<&Buffer<f32>, DevError> {
        self.planes.get(&id).map(|p| &p.buf).ok_or_else(|| refuse(&format!("plane {id} is not resident")))
    }

    /// Drops the host mirror of a plane that a launch is about to write.
    pub fn touch(&mut self, id: u64) {
        if let Some(p) = self.planes.get_mut(&id) {
            p.mirror = OnceCell::new();
        }
    }

    fn make(&mut self, data: &[f32], phase: usize) -> Result<DevPlane, DevError> {
        let zero = data.iter().all(|v| v.to_bits() == 0);
        let buf: Buffer<f32> = if data.is_empty() {
            self.s.input(&[0.0f32])?
        } else if zero {
            let b = self.s.output::<f32>(data.len())?;
            self.s.queue.fill(&b, 0.0f32)?;
            b
        } else {
            self.uploads += 1;
            self.s.input(data)?
        };
        Ok(DevPlane { buf, len: data.len(), phase, mirror: OnceCell::new() })
    }

    /// `register_plane`: replaces the plane (a new buffer of `data.len()` words).
    pub fn register(&mut self, id: u64, data: &[f32], phase: usize) {
        if self.broken() {
            return;
        }
        let r = self.make(data, phase);
        if let Some(p) = self.dev(r) {
            self.planes.insert(id, p);
        }
    }

    /// `ensure_plane`: inserts a zero plane of `len` words with the placement's phase for `id` only when absent.
    pub fn ensure(&mut self, id: u64, len: usize) {
        if self.broken() || self.planes.contains_key(&id) {
            return;
        }
        let phase = self.phase_of.get(&id).copied().unwrap_or(0);
        self.register(id, &vec![0.0f32; len], phase);
    }

    /// `ensure_level_plane`: as `ensure` with the phase the placement assigns to served plane `ordinal` of `level`.
    pub fn ensure_level(&mut self, level: usize, ordinal: usize, id: u64, len: usize) {
        if self.broken() || self.planes.contains_key(&id) {
            return;
        }
        let phase = self.served_phase.get(&(level, ordinal)).copied().unwrap_or(0);
        self.register(id, &vec![0.0f32; len], phase);
    }

    /// The whole plane as host words (one readback, kept until the plane is written again).
    pub fn words(&self, id: u64) -> Option<&[f32]> {
        let p = self.planes.get(&id)?;
        let m = p.mirror.get_or_init(|| {
            self.reads.set(self.reads.get() + 1);
            match self.s.queue.read(&p.buf) {
                Ok(v) => v,
                Err(e) => {
                    self.set_dev(e.into());
                    Vec::new()
                }
            }
        });
        Some(m.as_slice())
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// The pyramid services: `Host` and `FlowServices` over the device plane store
// ---------------------------------------------------------------------------------------------------------------------------------

/// `optical_flow::host::ComposedHost` method for method, with every plane launch a device kernel.
pub struct DevServices<'a> {
    pub p: &'a mut DevPlanes,
    pub targets: ResampleTargets,
}

fn dims(r: &Rect24) -> (i32, i32, i32) {
    (r.lo as u32 as i32, (r.lo >> 32) as u32 as i32, (r.lo >> 64) as u32 as i32)
}

impl DevServices<'_> {
    /// The CPU's `take` of every handle in order: the first missing one is the error.
    fn all_present(&mut self, ids: &[u64]) -> bool {
        for id in ids {
            if !self.p.has(*id) {
                self.p.fail(HostError::UnknownHandle(*id));
                return false;
            }
        }
        true
    }

    fn parallel(&mut self, cb: FlowCb, ctx: Ctx) {
        // the tile launch is the resample pair over the second plane of the level
        let ctx = match ctx {
            Ctx::Tile { prev_width, prev_height, source, cur_width, cur_height, .. } => Ctx::Pair { prev_width, prev_height, source, cur_width, cur_height },
            c => c,
        };
        let s = self.p.s.clone();
        match (cb, ctx) {
            (FlowCb::Smooth, Ctx::Smooth { rect, work, flow_x, flow_y, source_copy, weight }) => {
                let (w, h, pitch) = dims(&rect);
                if !self.all_present(&[work, flow_x, flow_y, source_copy]) {
                    return;
                }
                let (row_start, n) = chunk(h, 1, 0);
                if n <= 0 {
                    return;
                }
                self.p.touch(work);
                let ph = self.p.phase(flow_x) as u32;
                let p = &*self.p;
                let r = (|| fetch::flow_stencil_rows_launch(&s, p.buf(work)?, p.buf(flow_x)?, p.buf(flow_y)?, p.buf(source_copy)?, w, h, pitch, weight, row_start, n, ph))();
                self.p.dev(r);
            }
            (FlowCb::Blend, Ctx::Blend { rect, work, flow_x, flow_y, weight, pair_weight }) => {
                let (w, h, pitch) = dims(&rect);
                if !self.all_present(&[work, flow_x, flow_y]) {
                    return;
                }
                let (row_start, n) = chunk(h, 1, 0);
                self.p.touch(flow_x);
                self.p.touch(flow_y);
                let p = &*self.p;
                let r = (|| normalize_flow_rows_dev(&s, p.buf(work)?, p.buf(flow_x)?, p.buf(flow_y)?, w, h, pitch, row_start, n, f32::from_bits(pair_weight), f32::from_bits(weight)))();
                self.p.dev(r);
            }
            (FlowCb::FillDiff, Ctx::Fill { rect, operand_plane, source_plane, destination, weight }) => {
                let (w, h, pitch) = dims(&rect);
                // the difference may run in place (destination is operand_plane): the minuend is then a copy of the output plane
                let alias = operand_plane == destination;
                let ok = if alias { self.all_present(&[source_plane, destination]) } else { self.all_present(&[operand_plane, source_plane, destination]) };
                if !ok {
                    return;
                }
                let (row_start, n) = chunk(h, 1, 0);
                if n <= 0 {
                    return;
                }
                self.p.touch(destination);
                let p = &*self.p;
                let r = (|| {
                    let out = p.buf(destination)?;
                    let tmp;
                    let b: &Buffer<f32> = if alias {
                        tmp = s.output::<f32>(out.len())?;
                        flow_copy_prefix(&s, &tmp, out, out.len())?;
                        &tmp
                    } else {
                        p.buf(operand_plane)?
                    };
                    fetch::sub_scaled_rows_launch(&s, p.buf(source_plane)?, b, out, w, pitch, row_start, n, weight)
                })();
                self.p.dev(r);
            }
            (FlowCb::ResampleHoriz, Ctx::Pair { prev_width, prev_height, source, cur_width, .. }) => {
                let tg = self.targets;
                if !self.all_present(&[source, tg.h_dst]) {
                    return;
                }
                let (row_start, n) = chunk(prev_height as i32, 1, 0);
                if n <= 0 {
                    return;
                }
                self.p.touch(tg.h_dst);
                let ph = self.p.phase(source) as u32;
                let p = &*self.p;
                let r = (|| fetch::resample_h_launch(&s, p.buf(source)?, p.buf(tg.h_dst)?, prev_width as i32, cur_width as i32, row_start, n, ph))();
                self.p.dev(r);
            }
            (FlowCb::ResampleVert, Ctx::Pair { prev_height, cur_width, cur_height, .. }) => {
                let tg = self.targets;
                if !self.all_present(&[tg.v_src, tg.v_dst]) {
                    return;
                }
                let (col_start, n) = chunk(cur_width as i32, 1, 0);
                if n <= 0 {
                    return;
                }
                self.p.touch(tg.v_dst);
                let p = &*self.p;
                let r = (|| fetch::resample_v_launch(&s, p.buf(tg.v_src)?, p.buf(tg.v_dst)?, cur_width as i32, prev_height as i32, cur_height as i32, col_start, n))();
                self.p.dev(r);
            }
            _ => self.p.fail(HostError::FlagRouteNotHosted(HelperName::ForeignContext)),
        }
    }
}

impl Host for DevServices<'_> {
    fn call(&mut self, c: Call) {
        if self.p.broken() {
            return;
        }
        let s = self.p.s.clone();
        match c {
            Call::ParallelFor { cb, ctx, .. } => self.parallel(cb, ctx),
            Call::CopyIn { dst, rect, .. } => {
                let (_, h, pitch) = dims(&rect);
                let n = (h as i64 * pitch as i64).max(0) as usize;
                match self.p.inputs.get(&dst).cloned() {
                    None => self.p.fail(HostError::NoSource(dst)),
                    Some(src) => match self.p.len_of(dst) {
                        Some(len) if len >= n && src.len() >= n => {
                            if n > 0 {
                                self.p.touch(dst);
                                let r = (|| {
                                    let tmp = s.input(&src[..n])?;
                                    flow_copy_prefix(&s, self.p.buf(dst)?, &tmp, n)
                                })();
                                self.p.dev(r);
                            }
                        }
                        Some(_) => self.p.fail(HostError::Short(dst)),
                        None => self.p.fail(HostError::UnknownHandle(dst)),
                    },
                }
            }
            Call::CopyPlane { dst, src, rect, .. } => {
                let (_, h, pitch) = dims(&rect);
                let n = (h as i64 * pitch as i64).max(0) as usize;
                let Some(slen) = self.p.len_of(src) else { return self.p.fail(HostError::UnknownHandle(src)) };
                match self.p.len_of(dst) {
                    Some(len) if len >= n && slen >= n => {
                        self.p.touch(dst);
                        let p = &*self.p;
                        let r = (|| flow_copy_prefix(&s, p.buf(dst)?, p.buf(src)?, n))();
                        self.p.dev(r);
                    }
                    Some(_) => self.p.fail(HostError::Short(dst)),
                    None => self.p.fail(HostError::UnknownHandle(dst)),
                }
            }
            Call::Fill { .. } => self.p.fail(HostError::FlagRouteNotHosted(HelperName::Fill)),
            Call::SmoothStep { .. } => self.p.fail(HostError::FlagRouteNotHosted(HelperName::SmoothStep)),
            Call::BlendStep { .. } => self.p.fail(HostError::FlagRouteNotHosted(HelperName::BlendStep)),
        }
    }
}

impl FlowServices for DevServices<'_> {
    fn register_plane(&mut self, id: PlaneId, data: Vec<f32>, phase: usize) {
        self.p.register(id, &data, phase);
    }
    fn ensure_plane(&mut self, id: PlaneId, len: usize) {
        self.p.ensure(id, len);
    }
    fn ensure_level_plane(&mut self, level: usize, ordinal: usize, id: PlaneId, len: usize) {
        self.p.ensure_level(level, ordinal, id, len);
    }
    fn plane_words(&self, id: PlaneId) -> Option<&[f32]> {
        self.p.words(id)
    }
    fn set_resample_targets(&mut self, targets: ResampleTargets) {
        self.targets = targets;
    }
    fn affine_plane(&mut self, h: PlaneId, w: i32, rows: i32, stride: i32, scale: f32, offset: f32) {
        if self.p.broken() || !self.all_present(&[h]) {
            return;
        }
        let (row_start, n) = chunk(rows, 1, 0);
        if n <= 0 {
            return;
        }
        let s = self.p.s.clone();
        self.p.touch(h);
        let p = &*self.p;
        let r = (|| fetch::affine_rows_launch(&s, p.buf(h)?, w, stride, row_start, n, scale.to_bits(), offset.to_bits()))();
        self.p.dev(r);
    }
    fn sum_slots(&self, h: PlaneId, w: i32, rows: i32, stride: i32) -> Option<[f32; 128]> {
        if !self.p.has(h) {
            return None;
        }
        let mut slots = [0.0f32; 128];
        if self.p.broken() {
            return Some(slots);
        }
        let r = self.p.buf(h).and_then(|b| fetch::sum_rows_value(&self.p.s, b, w, stride, 0, rows, self.p.phase(h) as u32));
        if let Some(v) = self.p.dev(r) {
            slots[0] = v;
        }
        Some(slots)
    }
    fn add_const_plane(&mut self, h: PlaneId, n: i64, c: f32) {
        if self.p.broken() || !self.all_present(&[h]) {
            return;
        }
        let (start, k) = chunk(n as i32, 1, 0);
        let _ = start;
        let s = self.p.s.clone();
        self.p.touch(h);
        let p = &*self.p;
        let r = (|| add_const_dev(&s, p.buf(h)?, k.max(0) as usize, c))();
        self.p.dev(r);
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// The solve: a host transcription of `optical_flow_solve::solve::run_body` whose launches are device kernels
// ---------------------------------------------------------------------------------------------------------------------------------

fn stencil_bits() -> fetch::StencilBits {
    fetch::StencilBits { k_s: KS_BITS, c_s: 8.0f32.to_bits(), k_v: KS_BITS, c_v: 8.0f32.to_bits() }
}

/// `Planes` of the stretch with no data: the device host never reads it (the planes are in the device store).
pub struct NoPlanes;

impl Planes for NoPlanes {
    fn src(&self, _j: PairPlane) -> (&[f32], u32) {
        (&[], 0)
    }
    fn src_mut(&mut self, _j: PairPlane) -> &mut [f32] {
        &mut []
    }
    fn dst(&mut self, _j: PairPlane) -> &mut [f32] {
        &mut []
    }
}

/// The stretch's host: the shipped `Direct` passes as device launches, with the launch order recorded.
pub struct DevStretchHost<'a> {
    pl: &'a mut DevPlanes,
    keep_first_pass: bool,
    /// The latest horizontal pass output (a `dst_w x sh` scratch) and its length.
    last: Option<(Buffer<f32>, usize)>,
    pub launches: Vec<Callback>,
}

impl StretchHost for DevStretchHost<'_> {
    fn helper(&mut self, _c: HelperCall) {}

    fn tile_pair<P: Planes>(&mut self, _mem: &mut P, t: &Tile) {
        self.launches.extend([Callback::TilePairFirst, Callback::TilePairSecond]);
        if self.pl.broken() {
            return;
        }
        let s = self.pl.s.clone();
        let (sw, sh) = (t.lp.width, t.lp.height);
        let (dst_w, dst_h) = (t.l.width, t.l.height);
        let (src_id, dst_id) = (t.j.of(t.lp), t.j.of(t.l));
        let ph = (self.pl.phase(src_id) & 15) as u32;
        self.pl.touch(dst_id);
        let keep = self.keep_first_pass;
        let pl = &*self.pl;
        let r = (|| -> Result<(Buffer<f32>, usize), DevError> {
            let len = dst_w.max(0) as usize * sh.max(0) as usize;
            let scratch = s.output::<f32>(len.max(1))?;
            s.queue.fill(&scratch, 0.0f32)?;
            let src = pl.buf(src_id)?;
            if (sw.max(0) as usize) * (sh.max(0) as usize) > src.len() {
                return Err(refuse("stretch source shorter than its extent"));
            }
            fetch::resample_h_launch(&s, src, &scratch, sw, dst_w, 0, sh, ph)?;
            fetch::resample_v_launch(&s, &scratch, pl.buf(dst_id)?, dst_w, sh, dst_h, 0, dst_w)?;
            Ok((scratch, len))
        })();
        if let Some(v) = self.pl.dev(r) {
            if keep {
                self.last = Some(v);
            }
        }
    }

    fn scale<P: Planes>(&mut self, _mem: &mut P, j: PairPlane, l: &Level, ratio: f32) {
        self.launches.push(Callback::Scale);
        let (w, h) = (l.width, l.height);
        if self.pl.broken() || w <= 0 || h <= 0 {
            return;
        }
        let s = self.pl.s.clone();
        let id = j.of(l);
        self.pl.touch(id);
        let pl = &*self.pl;
        let r = (|| sa::scale_plane_dev(&s, pl.buf(id)?, w, l.stride, ratio, 0, h as u32))();
        self.pl.dev(r);
    }

    fn border<P: Planes>(&mut self, _mem: &mut P, j: PairPlane, lp: &Level) {
        self.launches.push(Callback::Border);
        let (w, h) = (lp.width, lp.height);
        if self.pl.broken() || w <= 0 || h <= 0 {
            return;
        }
        let s = self.pl.s.clone();
        let id = j.of(lp);
        self.pl.touch(id);
        let pl = &*self.pl;
        let r = (|| sa::zero_border_dev(&s, pl.buf(id)?, w, h, lp.stride, 0, h as u32))();
        self.pl.dev(r);
    }
}

/// The tail's host: the median launch and the copy back as device kernels.
pub struct DevTailHost<'a> {
    pl: &'a mut DevPlanes,
    l: Level,
    rec: &'a mut Vec<Callback>,
}

impl TailHost for DevTailHost<'_> {
    fn launch(&mut self, args: LaunchArgs) {
        self.rec.push(args.callback());
        if self.pl.broken() {
            return;
        }
        let LaunchArgs::Median(m) = args else {
            self.pl.set_dev(refuse("the tail launches only the median"));
            return;
        };
        let s = self.pl.s.clone();
        let l = self.l;
        self.pl.touch(m.median_out);
        let pl = &*self.pl;
        let r = (|| sb::median_rows_dev(&s, pl.buf(m.plane)?, pl.buf(m.median_out)?, l.width, l.height, l.stride, 0, l.height))();
        self.pl.dev(r);
    }

    fn copy_back(&mut self, plane: u64, median_out: u64, _pitch: u64, _head: [u64; 3]) {
        if self.pl.broken() {
            return;
        }
        let s = self.pl.s.clone();
        let l = self.l;
        self.pl.touch(plane);
        let pl = &*self.pl;
        let r = (|| flow_copy_rect(&s, pl.buf(plane)?, pl.buf(median_out)?, &l))();
        self.pl.dev(r);
    }
}

/// The polar magnitude of the alternate launch block (`fetch::polar` on the gradient planes into the descriptor plane).
fn launch_polar(pl: &DevPlanes, l: &Level) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    // the y gradient is the P operand and the x gradient the Q operand; the row peel phase is taken from the P plane
    crate::device_ops_flow_polar::polar_launch(&pl.s, pl.buf(l.grad_y)?, pl.buf(l.grad_x)?, pl.buf(l.copied_plane)?, cw, st, 0, rows, (pl.phase(l.grad_y) & 31) as u32, &POLAR_CONSTS)
}


fn launch_alt(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::DerivX);
    pl.touch(l.grad_x);
    fetch::deriv_x_launch(&pl.s, pl.buf(l.image_a)?, pl.buf(l.grad_x)?, cw, st, 0, rows, (pl.phase(l.grad_x) & 31) as u32, &stencil_bits())?;
    rec.push(Callback::DerivY);
    pl.touch(l.grad_y);
    fetch::deriv_y_launch(&pl.s, pl.buf(l.image_a)?, pl.buf(l.grad_y)?, rows, st, 0, cw, &stencil_bits())?;
    rec.push(Callback::Polar);
    pl.touch(l.copied_plane);
    launch_polar(pl, l)
}

fn launch_gather(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::DerivX);
    pl.touch(l.grad_x);
    fetch::deriv_x_launch(&pl.s, pl.buf(l.image_b)?, pl.buf(l.grad_x)?, cw, st, 0, rows, (pl.phase(l.grad_x) & 31) as u32, &stencil_bits())?;
    rec.push(Callback::DerivY);
    pl.touch(l.grad_y);
    fetch::deriv_y_launch(&pl.s, pl.buf(l.image_b)?, pl.buf(l.grad_y)?, rows, st, 0, cw, &stencil_bits())
}

fn launch_warps(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    for (src, out) in [(l.image_b, l.warped_image), (l.grad_x, l.warped_grad_x), (l.grad_y, l.warped_grad_y)] {
        rec.push(Callback::Warp);
        pl.touch(out);
        fetch::warp_fetch_launch(&pl.s, pl.buf(l.flow_u)?, pl.buf(l.flow_v)?, pl.buf(src)?, pl.buf(out)?, cw, rows, st, 0, rows, WARP_LO_BITS, MARGIN_BITS)?;
    }
    Ok(())
}

fn launch_mix(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::Mix);
    pl.touch(l.coeff);
    let sh = Shape { width: cw, stride: st, phase_bytes: (pl.phase(l.coeff) & 31) as u32 };
    let planes = [pl.buf(l.image_a)?, pl.buf(l.warped_image)?, pl.buf(l.warped_grad_x)?, pl.buf(l.warped_grad_y)?, pl.buf(l.flow_u)?, pl.buf(l.flow_v)?];
    sa::fma_mix_dev(&pl.s, planes, pl.buf(l.coeff)?, &sh, 0, rows)
}

fn launch_flow(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level, p: &Params) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::Flow);
    pl.touch(l.flow_u_next);
    pl.touch(l.flow_v_next);
    let inp = sa::FlowBufs { u: pl.buf(l.flow_u)?, v: pl.buf(l.flow_v)?, a: pl.buf(l.warped_grad_x)?, b: pl.buf(l.warped_grad_y)?, c: pl.buf(l.coeff)? };
    sa::flow_update_dev(&pl.s, &inp, pl.buf(l.flow_u_next)?, pl.buf(l.flow_v_next)?, cw, st, 0, rows, f32::from_bits(p.relax_scale), f32::from_bits(p.flow_gain), f32::from_bits(EPS_BITS))
}

fn launch_relax(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level, p: &Params) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::Relax);
    pl.touch(l.relax_first);
    pl.touch(l.relax_second);
    let nest_one = sa::RelaxNest { t: pl.buf(l.nest1_t)?, u: pl.buf(l.nest1_u)?, acc: pl.buf(l.flow_u_next)?, out: pl.buf(l.relax_first)? };
    let nest_two = sa::RelaxNest { t: pl.buf(l.nest2_t)?, u: pl.buf(l.nest2_u)?, acc: pl.buf(l.flow_v_next)?, out: pl.buf(l.relax_second)? };
    let v = if p.alt_launches != 0 { Some(pl.buf(l.copied_plane)?) } else { None };
    sa::relax_dev(&pl.s, cw, rows, st, f32::from_bits(p.relax_scale), v, [&nest_one, &nest_two], (pl.phase(l.nest1_t) & 63) as u32, 0, rows)
}

fn launch_smooth(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level, p: &Params) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::Smooth);
    for id in [l.nest1_t, l.nest1_u, l.nest2_t, l.nest2_u] {
        pl.touch(id);
    }
    let scale = if p.alt_launches != 0 { Some(pl.buf(l.copied_plane)?) } else { None };
    let geo = Geo { width: cw, height: rows, stride: st, relax_scale: f32::from_bits(p.relax_scale), smooth_weight: f32::from_bits(SMOOTH_WEIGHT_BITS), c: f32::from_bits(p.smooth_bias) };
    let first = sb::SmoothPass { src: pl.buf(l.relax_first)?, p: pl.buf(l.nest1_t)?, q: pl.buf(l.nest1_u)? };
    let second = sb::SmoothPass { src: pl.buf(l.relax_second)?, p: pl.buf(l.nest2_t)?, q: pl.buf(l.nest2_u)? };
    sb::smooth_normalise_dev(&pl.s, &geo, scale, &first, &second, 0, rows as u32)
}

fn launch_diff(pl: &mut DevPlanes, rec: &mut Vec<Callback>, l: &Level, acc: &mut f32) -> Result<(), DevError> {
    let (cw, rows, st) = (l.width, l.height, l.stride);
    rec.push(Callback::Diff);
    pl.touch(l.flow_v);
    pl.touch(l.flow_u);
    let sh = Shape { width: cw, stride: st, phase_bytes: (pl.phase(l.flow_v) & 31) as u32 };
    let r = sa::diff_sum_dev(&pl.s, pl.buf(l.relax_second)?, pl.buf(l.relax_first)?, pl.buf(l.flow_v)?, pl.buf(l.flow_u)?, &sh, 0, rows)?;
    *acc = acc.fadd(r);
    Ok(())
}

/// Runs the solve body from level `idx` down to 0 on the device plane store: `optical_flow_solve::solve::run_body` with every launch a device
/// kernel. `rec` receives every launch in the order the crate reports them to `PlaneStore::launched`. A device error is kept in `pl` and the
/// run ends as aborted; the caller reads `pl` before using the result.
pub fn dev_solve_body(levels: &[Level], idx: i32, p: &Params, pl: &mut DevPlanes, poll: &mut dyn FnMut() -> bool, rec: &mut Vec<Callback>) -> Result<Outcome, RunRefused> {
    if small_level(levels, idx) {
        return Err(RunRefused::SmallLevel);
    }
    if idx < 0 || idx as usize >= levels.len() {
        return Err(RunRefused::FirstIndex);
    }
    macro_rules! step {
        ($e:expr) => {
            if pl.broken() {
                return Ok(Outcome::Aborted);
            } else if let Err(e) = $e {
                pl.set_dev(e);
                return Ok(Outcome::Aborted);
            }
        };
    }
    let mut acc: f32;
    if poll() {
        return Ok(Outcome::Aborted);
    }
    let level_rec = &levels[idx as usize];
    for plane in [level_rec.flow_u, level_rec.flow_v, level_rec.nest1_t, level_rec.nest1_u, level_rec.nest2_t, level_rec.nest2_u] {
        pl.touch(plane);
        let s = pl.s.clone();
        step!(pl.buf(plane).and_then(|b| flow_clear_rect(&s, b, 0, level_rec)));
    }
    let mut k = idx;
    while k >= 0 {
        let l = &levels[k as usize];
        if poll() {
            return Ok(Outcome::Aborted);
        }
        if p.alt_launches != 0 {
            step!(launch_alt(pl, rec, l));
        }
        step!(launch_gather(pl, rec, l));
        if p.outer_iters > 0 {
            let mut outer_iter = 0i32;
            loop {
                step!(launch_warps(pl, rec, l));
                step!(launch_mix(pl, rec, l));
                if p.mid_iters > 0 {
                    let mut mid_iter = 0i32;
                    'mid: loop {
                        step!(launch_flow(pl, rec, l, p));
                        if p.inner_iters > 0 {
                            let mut j = 0i32;
                            loop {
                                step!(launch_relax(pl, rec, l, p));
                                step!(launch_smooth(pl, rec, l, p));
                                acc = 0.0;
                                step!(launch_diff(pl, rec, l, &mut acc));
                                let cnt = l.width.wrapping_mul(l.height);
                                let ratio = acc.fdiv(cnt as f32);
                                if p.tol > ratio {
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
                if p.counted_route != 0 {
                    let mut th = DevTailHost { pl: &mut *pl, l: *l, rec: &mut *rec };
                    median_pair(l, &mut th);
                    step!(Ok::<(), DevError>(()));
                }
                outer_iter = outer_iter.wrapping_add(1);
                if outer_iter >= p.outer_iters {
                    break;
                }
            }
        }
        if k > 0 {
            let (lf, lc) = (&levels[(k - 1) as usize], l);
            let launches = {
                let mut host = DevStretchHost { pl: &mut *pl, keep_first_pass: p.alt_launches != 0, last: None, launches: Vec::new() };
                run_stretch(lf, lc, 0, 0, &mut NoPlanes, &mut host);
                // the first pass output of the last tile pair stays in the finer level's median_out plane (the crate's `StretchMem::finish`)
                if let Some((scratch, len)) = host.last.take() {
                    if !host.pl.broken() {
                        let s = host.pl.s.clone();
                        host.pl.touch(lf.median_out);
                        let pl2 = &*host.pl;
                        let r = (|| {
                            let m = pl2.buf(lf.median_out)?;
                            flow_copy_prefix(&s, m, &scratch, len.min(m.len()))
                        })();
                        host.pl.dev(r);
                    }
                }
                host.launches
            };
            rec.extend(launches);
            step!(Ok::<(), DevError>(()));
        }
        k -= 1;
    }
    Ok(Outcome::Done)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// The entry host: `optical_flow_entry::compose::ComposedFlowHost` with the device plane store
// ---------------------------------------------------------------------------------------------------------------------------------

use optical_flow::{build_to_cut, BodyIn, Cut, ImgCall, Rect, SERVED_PLANES};
use optical_flow_entry::placement::{flow_level_dims, placement, FlowPlacement, HOST_PHASE_DEFAULT};
use optical_flow::iface::{StageEnv, StageParams};
use optical_flow::join::run_body as run_pyramid_body;
use optical_flow_entry::compose::FlowStop;
use optical_flow_entry::{lane_params, ObjFields};

/// Counts of one node, for the transfer log.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FlowStats {
    /// Device to host plane reads (`plane_words` mirrors), not counting the operand and one word reductions.
    pub plane_reads: usize,
    /// Host to device plane uploads (non zero planes).
    pub plane_uploads: usize,
    /// The launches the solve reported, in order.
    pub solve_launches: Vec<Callback>,
}

pub struct DevFlowHost<'a> {
    pub pl: DevPlanes,
    prev: &'a [f32],
    next: &'a [f32],
    width: i32,
    height: i32,
    img: &'a mut dyn FnMut(&ImgCall) -> u64,
    table: LevelTable,
    refusal: Option<EntryRefusal>,
    solve_stop: Option<FlowStop>,
    solve_aborted: bool,
    warp_scale: f32,
    out: Option<DevImage>,
    produced: [bool; 3],
    pub seq: Vec<Callback>,
    /// ALIGN-PHASE-HOST: byte phase of the render's first image allocation (the placement's input).
    host_phase: u32,
    /// Every plane at this one byte phase in place of the placement rule, as `ComposedFlowHost::with_uniform_phase`.
    uniform_phase: Option<u32>,
}

impl<'a> DevFlowHost<'a> {
    pub fn new(s: &Arc<Session>, prev: &'a [f32], next: &'a [f32], width: i32, height: i32, img: &'a mut dyn FnMut(&ImgCall) -> u64) -> Self {
        DevFlowHost {
            pl: DevPlanes::new(s),
            prev,
            next,
            width,
            height,
            img,
            table: LevelTable::default(),
            refusal: None,
            solve_stop: None,
            solve_aborted: false,
            warp_scale: 0.0,
            out: None,
            produced: [false; 3],
            seq: Vec::new(),
            host_phase: HOST_PHASE_DEFAULT,
            uniform_phase: None,
        }
    }

    /// The same host with the ALIGN-PHASE-HOST value `phase`.
    pub fn with_host_phase(mut self, phase: u32) -> Self {
        self.host_phase = phase;
        self
    }

    /// The same host with every plane at the byte phase `phase`.
    pub fn with_uniform_phase(mut self, phase: u32) -> Self {
        self.uniform_phase = Some(phase & 31);
        self
    }

    /// Records the placement's phases in the plane store, as `ComposedFlowHost::apply_placement`.
    fn apply_placement(&mut self, pl: &FlowPlacement) {
        let l = level0_record();
        let words = [
            l.warped_image, l.warped_grad_x, l.warped_grad_y, l.coeff, l.flow_u_next, l.flow_v_next, l.relax_first, l.relax_second, l.median_out,
            l.image_a, l.image_b, l.grad_x, l.grad_y, l.flow_u, l.flow_v, l.nest1_t, l.copied_plane,
        ];
        for (id, ph) in words.into_iter().zip(pl.descriptor) {
            self.pl.phase_of.insert(id, ph as usize);
        }
        for (id, ph) in [l.nest1_u, l.nest2_t, l.nest2_u].into_iter().zip(pl.outputs) {
            self.pl.phase_of.insert(id, ph as usize);
        }
        for (j, row) in pl.served.iter().enumerate() {
            for (plane, ph) in SERVED_PLANES.iter().zip(row) {
                self.pl.served_phase.insert((j + 1, plane.ordinal() as usize), *ph as usize);
            }
        }
    }

    fn phase_of(&self, id: PlaneId) -> usize {
        self.pl.phase_of.get(&id).copied().unwrap_or(0)
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

    fn slot(o: OutPlane) -> usize {
        match o {
            OutPlane::FlowU => 0,
            OutPlane::FlowV => 1,
            OutPlane::Confidence => 2,
        }
    }

    fn register_cleared_scratch(&mut self, n: usize) {
        let scratch = smoothing_scratch();
        for id in [scratch.result, scratch.flow_x, scratch.flow_y, scratch.work, scratch.source_copy] {
            let phase = self.phase_of(id);
            self.pl.register(id, &vec![0.0; n + PLANE_SLACK], phase);
        }
        for id in [scratch.flow_x, scratch.flow_y, scratch.work] {
            self.pl.inputs.insert(id, vec![0.0; n + PLANE_SLACK]);
        }
    }

    fn ensure_solve_planes(&mut self, dst_a: &[u8], dst_b: &[u8]) {
        let first_level = self.table.levels[0];
        for (id, bytes) in [(first_level.image_a, dst_a), (first_level.image_b, dst_b)] {
            if !self.pl.has(id) {
                let words: Vec<f32> = bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
                let phase = self.phase_of(id);
                self.pl.register(id, &words, phase);
            }
        }
        for (j, r) in self.table.levels.clone().into_iter().enumerate() {
            let n = (r.stride.max(0) as usize) * (r.height.max(0) as usize) + PLANE_SLACK;
            if j >= 1 {
                for plane in SERVED_PLANES {
                    let mut rec = r;
                    let id = *rec.plane_mut(plane);
                    self.pl.ensure_level(j, plane.ordinal() as usize, id, n);
                }
            }
            for id in [
                r.image_a, r.image_b, r.grad_x, r.grad_y, r.warped_image, r.warped_grad_x, r.warped_grad_y, r.coeff, r.flow_u, r.flow_v, r.flow_u_next,
                r.flow_v_next, r.nest1_t, r.nest1_u, r.nest2_t, r.nest2_u, r.relax_first, r.relax_second, r.median_out, r.copied_plane,
            ] {
                self.pl.ensure(id, n);
            }
        }
    }

    /// The output image (three planes of level 0's `w x h`), allocated on first use.
    fn out_plane(&mut self, o: OutPlane) -> Result<(Arc<Session>, usize), DevError> {
        let rec = self.table.levels.first().ok_or_else(|| refuse("no level table"))?;
        let n = rec.width.max(0) as usize * rec.height.max(0) as usize;
        if n != self.width as usize * self.height as usize {
            return Err(refuse("level 0 is not the frame size"));
        }
        let s = self.pl.s.clone();
        if self.out.is_none() {
            let d = DevImage::alloc(&s, 3, n)?;
            for c in 0..3 {
                s.queue.fill(d.plane(c), 0.0f32)?;
            }
            self.out = Some(d);
        }
        Ok((s, Self::slot(o)))
    }
}

impl EntryHost for DevFlowHost<'_> {
    fn pyramid_body(&mut self, p: &LaneParams) -> i32 {
        use optical_flow::body::{BodyResult, Refusal};
        let _ = (BodyResult::Returned(0), Refusal::FlagRoute);
        let (w, h) = (self.width, self.height);
        let n = (w as usize) * (h as usize);
        let len = n * 4 + 256;
        let (src_a, src_b) = (bytes_of(self.prev, len), bytes_of(self.next, len));
        let (mut dst_a, mut dst_b) = (vec![0u8; len], vec![0u8; len]);
        let inp = BodyIn {
            rect: Rect { w, h, pitch: w, reserved: 0, q: 4 },
            levels: p.four,
            scale: p.diag_weight,
            flag: 0,
            smoothing_word: p.scale_gt,
            planes: level0_record(),
            src_a: &src_a,
            src_b: &src_b,
        };
        let env = StageEnv {
            launcher: LAUNCHER,
            params: StageParams { blend_weight: p.scale_bits, smooth_weight: p.std_smooth_bits as u32, flag_route: false, flag_cell_bits: 0 },
            scratch: smoothing_scratch(),
            table_word: 1,
        };
        let dims = flow_level_dims(w, h, p.diag_weight, p.four);
        let placed = match self.uniform_phase {
            Some(u) => FlowPlacement { descriptor: [u; 17], outputs: [u; 3], served: vec![[u; 8]; dims.len().saturating_sub(1)] },
            None => placement(w, h, &dims, self.host_phase),
        };
        self.apply_placement(&placed);
        let (align_a, align_b) = (placed.descriptor[9] as usize, placed.descriptor[10] as usize);
        self.register_cleared_scratch(n);
        self.warp_scale = f32::from_bits(p.lambda_bits as u32);
        let end = {
            let mut svc = DevServices { p: &mut self.pl, targets: ResampleTargets::default() };
            run_pyramid_body(&inp, &mut dst_a, &mut dst_b, &mut *self.img, align_a, align_b, &mut svc, env)
        };
        let (ret, refusal) = pyramid_body_result(end);
        if refusal.is_some() {
            self.refusal = refusal;
            return ret;
        }
        let (mut sa_, mut sb_) = (dst_a.clone(), dst_b.clone());
        let (_, table) = build_to_cut(&inp, &mut sa_, &mut sb_, &mut *self.img);
        self.table = table;
        if ret == 0 && !self.table.levels.is_empty() && !self.pl.broken() {
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
        let mut poll = || false;
        match dev_solve_body(&levels, first, &params, &mut self.pl, &mut poll, &mut self.seq) {
            Ok(Outcome::Done) => {}
            Ok(Outcome::Aborted) => self.solve_aborted = true,
            Err(RunRefused::SmallLevel) => self.refusal = Some(EntryRefusal::SmallLevel),
            Err(RunRefused::FirstIndex) => self.solve_stop = Some(FlowStop::NoLevels),
        }
    }

    fn poll_abort(&mut self) -> bool {
        self.solve_aborted || self.refusal.is_some() || self.solve_stop.is_some() || self.pl.broken()
    }

    fn launch(&mut self, l: Launch) {
        if self.pl.broken() {
            return;
        }
        let Some(rec) = self.table.levels.first().copied() else {
            self.pl.set_dev(refuse("no level table"));
            return;
        };
        let (w, h, st) = (rec.width, rec.height, rec.stride);
        let s = self.pl.s.clone();
        match l {
            Launch::DerivX { src, dst } => {
                let (src_id, dst_id) = (self.id_of(src), self.id_of(dst));
                self.pl.touch(dst_id);
                let pl = &self.pl;
                let r = (|| fetch::deriv_x_launch(&s, pl.buf(src_id)?, pl.buf(dst_id)?, w, st, 0, h, (pl.phase(dst_id) & 31) as u32, &stencil_bits()))();
                self.pl.dev(r);
            }
            Launch::DerivY { src, dst } => {
                let (src_id, dst_id) = (self.id_of(src), self.id_of(dst));
                self.pl.touch(dst_id);
                let pl = &self.pl;
                let r = (|| fetch::deriv_y_launch(&s, pl.buf(src_id)?, pl.buf(dst_id)?, h, st, 0, w, &stencil_bits()))();
                self.pl.dev(r);
            }
            Launch::FinalWarp => {
                self.pl.touch(rec.median_out);
                let scale_bits = self.warp_scale.to_bits();
                let pl = &self.pl;
                let r = (|| {
                    let p = fetch::FinalWarpPlanes {
                        target: pl.buf(rec.image_a)?,
                        img: pl.buf(rec.image_b)?,
                        u: pl.buf(rec.flow_u)?,
                        v: pl.buf(rec.flow_v)?,
                        grad_a_x: pl.buf(rec.flow_u_next)?,
                        grad_a_y: pl.buf(rec.flow_v_next)?,
                        grad_b_x: pl.buf(rec.nest1_t)?,
                        grad_b_y: pl.buf(rec.nest2_t)?,
                    };
                    fetch::final_warp_launch(&s, &p, pl.buf(rec.median_out)?, w, h, st, 0, h, scale_bits)
                })();
                self.pl.dev(r);
            }
        }
    }

    fn copy_plane(&mut self, src: EntryPlane, out: OutPlane) {
        if self.pl.broken() {
            return;
        }
        let id = self.id_of(src);
        let rec = self.table.levels[0];
        let r = self.out_plane(out).and_then(|(s, c)| {
            let d = self.out.as_ref().expect("allocated");
            flow_compact(&s, d.plane(c), self.pl.buf(id)?, rec.width, rec.height, rec.stride)
        });
        self.produced[Self::slot(out)] = r.is_ok();
        self.pl.dev(r);
    }

    fn scale_copy(&mut self, src: EntryPlane, out: OutPlane, k: f32) {
        if self.pl.broken() {
            return;
        }
        let id = self.id_of(src);
        let rec = self.table.levels[0];
        let r = self.out_plane(out).and_then(|(s, c)| {
            let d = self.out.as_ref().expect("allocated");
            let n = rec.width.max(0) as usize * rec.height.max(0) as usize;
            let tmp = s.output::<f32>(n.max(1))?;
            flow_compact(&s, &tmp, self.pl.buf(id)?, rec.width, rec.height, rec.stride)?;
            fetch::scale_copy_launch(&s, &tmp, d.plane(c), k.to_bits(), n)
        });
        self.produced[Self::slot(out)] = r.is_ok();
        self.pl.dev(r);
    }
}

/// Runs one node on the device: the crate's `run_entry` over [`DevFlowHost`]. Every outcome the CPU node reports as an error (a refusal, a stopped
/// pyramid, an abort) is returned as `DevError::Unsupported`, so the executor reruns the node on the CPU and the CPU raises its own error.
pub fn run_dev_flow(s: &Arc<Session>, prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants) -> Result<(DevImage, FlowStats), DevError> {
    run_dev_flow_at(s, prev, next, width, height, c, HOST_PHASE_DEFAULT)
}

/// `run_dev_flow` for the ALIGN-PHASE-HOST value `host_phase`, the value the CPU node takes from `KParams::flow_host_phase`.
pub fn run_dev_flow_at(s: &Arc<Session>, prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants, host_phase: u32) -> Result<(DevImage, FlowStats), DevError> {
    run_dev_flow_core(s, prev, next, width, height, c, host_phase, None)
}

/// `run_dev_flow` with every plane at the byte phase `phase`, as `run_flow_uniform` on the CPU.
pub fn run_dev_flow_uniform(s: &Arc<Session>, prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants, phase: u32) -> Result<(DevImage, FlowStats), DevError> {
    run_dev_flow_core(s, prev, next, width, height, c, HOST_PHASE_DEFAULT, Some(phase))
}

/// Runs the entry on the device host for the host value `host_phase` and checks every plane's registered phase against the placement
/// table the CPU host applies (level 0 planes by descriptor word and output order, served planes by level and plane). Returns the number
/// of planes compared, or the first difference.
pub fn check_device_placement(s: &Arc<Session>, w: i32, h: i32, c: &FlowConstants, host_phase: u32) -> Result<usize, String> {
    let n = (w as usize) * (h as usize);
    let (prev, next) = (vec![0.5f32; n], vec![0.25f32; n]);
    let mut img = served_image;
    let mut host = DevFlowHost::new(s, &prev, &next, w, h, &mut img).with_host_phase(host_phase);
    let _ = run_entry(&mut host, &c.obj_fields(), SCALE_THRESHOLD, false);
    let p = lane_params(&c.obj_fields(), SCALE_THRESHOLD);
    let dims = flow_level_dims(w, h, p.diag_weight, p.four);
    let want = placement(w, h, &dims, host_phase);
    let got = host.pl.phases();
    let l = level0_record();
    let mut seen = 0usize;
    let mut check = |name: String, id: u64, ph: u8| -> Result<(), String> {
        match got.get(&id) {
            Some(g) if *g == ph as usize => {
                seen += 1;
                Ok(())
            }
            other => Err(format!("{w}x{h} host phase {host_phase}: {name} device {other:?} placement {ph}")),
        }
    };
    let words = [
        l.warped_image, l.warped_grad_x, l.warped_grad_y, l.coeff, l.flow_u_next, l.flow_v_next, l.relax_first, l.relax_second, l.median_out,
        l.image_a, l.image_b, l.grad_x, l.grad_y, l.flow_u, l.flow_v, l.nest1_t, l.copied_plane,
    ];
    for (i, (id, ph)) in words.into_iter().zip(want.descriptor).enumerate() {
        check(format!("descriptor word {i}"), id, ph as u8)?;
    }
    for (i, (id, ph)) in [l.nest1_u, l.nest2_t, l.nest2_u].into_iter().zip(want.outputs).enumerate() {
        check(format!("output {i}"), id, ph as u8)?;
    }
    for (j, row) in want.served.iter().enumerate() {
        let mut rec = host.table.levels.get(j + 1).copied().ok_or_else(|| format!("{w}x{h}: level {} missing", j + 1))?;
        for (plane, ph) in SERVED_PLANES.iter().zip(row) {
            let id = *rec.plane_mut(*plane);
            check(format!("level {} plane {}", j + 1, plane.ordinal()), id, *ph as u8)?;
        }
    }
    Ok(seen)
}

#[allow(clippy::too_many_arguments)]
fn run_dev_flow_core(s: &Arc<Session>, prev: &[f32], next: &[f32], width: i32, height: i32, c: &FlowConstants, host_phase: u32, uniform: Option<u32>) -> Result<(DevImage, FlowStats), DevError> {
    if width < 2 || height < 2 {
        return Err(refuse(refusal_text(EntryRefusal::SmallLevel)));
    }
    let n = (width as usize) * (height as usize);
    if prev.len() < n || next.len() < n {
        return Err(refuse("input image shorter than width times height"));
    }
    let mut img = served_image;
    let mut h = DevFlowHost::new(s, prev, next, width, height, &mut img).with_host_phase(host_phase);
    if let Some(u) = uniform {
        h = h.with_uniform_phase(u);
    }
    let path = run_entry(&mut h, &c.obj_fields(), SCALE_THRESHOLD, false);
    if let Some(e) = h.pl.take_dev_err() {
        return Err(e);
    }
    if let Some(r) = h.refusal {
        return Err(refuse(refusal_text(r)));
    }
    if let Some(e) = h.solve_stop.take() {
        return Err(refuse(&format!("late outcome {e:?}")));
    }
    if let Some(e) = &h.pl.err {
        return Err(refuse(&format!("OPTICAL-FLOW-HOST ({e:?})")));
    }
    match path {
        EntryPath::InLine => {}
        other => return Err(refuse(&format!("entry path {other:?}"))),
    }
    let stats = FlowStats { plane_reads: h.pl.reads.get(), plane_uploads: h.pl.uploads, solve_launches: std::mem::take(&mut h.seq) };
    match (h.out.take(), h.produced) {
        (Some(d), [true, true, true]) => Ok((d, stats)),
        _ => Err(refuse("an output plane was not produced")),
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// accepts, run, corpus
// ---------------------------------------------------------------------------------------------------------------------------------

/// The frame refusals the CPU node raises, decided from the frame size alone (the level build decides them before it reads a pixel):
/// `Some(reason)` when the CPU node errors, `None` when the node runs.
pub fn flow_shape_refusal(w: usize, h: usize) -> Option<&'static str> {
    use std::collections::HashMap;
    use std::sync::Mutex;
    static CACHE: Mutex<Option<HashMap<(usize, usize), Option<&'static str>>>> = Mutex::new(None);
    if let Some(v) = CACHE.lock().ok().and_then(|g| g.as_ref().and_then(|m| m.get(&(w, h)).copied())) {
        return v;
    }
    let v = shape_refusal_uncached(w, h);
    if let Ok(mut g) = CACHE.lock() {
        g.get_or_insert_with(HashMap::new).insert((w, h), v);
    }
    v
}

fn shape_refusal_uncached(w: usize, h: usize) -> Option<&'static str> {
    if w < 2 || h < 2 || w > i32::MAX as usize / 8 || h > i32::MAX as usize / 8 || w.saturating_mul(h) > (1usize << 28) {
        return Some(refusal_text(EntryRefusal::SmallLevel));
    }
    let p = lane_params(&ObjFields::default(), SCALE_THRESHOLD);
    let len = w * h * 4 + 256;
    let (src_a, src_b) = (vec![0u8; len], vec![0u8; len]);
    let (mut da, mut db) = (vec![0u8; len], vec![0u8; len]);
    let inp = BodyIn {
        rect: Rect { w: w as i32, h: h as i32, pitch: w as i32, reserved: 0, q: 4 },
        levels: p.four,
        scale: p.diag_weight,
        flag: 0,
        smoothing_word: 0,
        planes: level0_record(),
        src_a: &src_a,
        src_b: &src_b,
    };
    let mut img = served_image;
    let (first, table) = build_to_cut(&inp, &mut da, &mut db, &mut img);
    match first.cut {
        Cut::TooFewLevels => return Some("OPTICAL-FLOW-TOO-FEW-LEVELS (optical_flow: the frame is too small for a pyramid of two levels)"),
        Cut::TableFull => return Some(refusal_text(EntryRefusal::LevelTableFull)),
        Cut::NullImage => return Some(refusal_text(EntryRefusal::NullImage)),
        Cut::Helper => return Some(refusal_text(EntryRefusal::FlagRoute)),
        Cut::Alt20 | Cut::Launch | Cut::Degenerate => {}
    }
    if table.levels.iter().any(|l| l.width < 2 || l.height < 2) {
        return Some(refusal_text(EntryRefusal::SmallLevel));
    }
    if table.first() < 0 {
        return Some("OPTICAL-FLOW-NO-LEVELS (optical_flow: the level table is empty)");
    }
    None
}

pub fn optical_flow_accepts(call: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    if shapes.len() != 9 {
        return Err("optical_flow takes nine operands");
    }
    if call.tag != 3 {
        return Err("optical_flow: the combo row is tag 3");
    }
    for s in &shapes[..2] {
        match s {
            ArgShape::Mip => return Err("optical_flow: a ripmap object is not a plain planar image"),
            ArgShape::Img { channels: 0 } => return Err("optical_flow: an operand image has no plane"),
            _ => {}
        }
    }
    for s in &shapes[2..] {
        if !matches!(s, ArgShape::Const { n } if *n >= 1) {
            return Err("optical_flow: the parameter operands must be constants");
        }
    }
    if call.dims.0.checked_mul(call.dims.1) != Some(call.n) {
        return Err("optical_flow: frame size and plane length differ");
    }
    if !optical_flow_entry::placement::host_phase_valid(call.kp.flow_host_phase) {
        return Err("optical_flow: the host phase value is not a multiple of 4 from 0 to 28");
    }
    match flow_shape_refusal(call.dims.0, call.dims.1) {
        Some(r) => Err(r),
        None => Ok(()),
    }
}

fn operand_plane(a: &DevArg, n: usize) -> Result<Vec<f32>, DevError> {
    match a {
        DevArg::Img(d) => {
            if d.channels() == 0 || d.plane(0).len() < n {
                return Err(refuse("operand image shorter than the frame"));
            }
            Ok(d.download()?.swap_remove(0))
        }
        DevArg::Const { n: ln, lanes } => Ok(Image::from_const(lanes, (*ln).max(1), n).planes.swap_remove(0)),
        _ => Err(refuse("a ripmap object is not a plain planar image")),
    }
}

pub fn flow_constants_of(args: &[DevArg]) -> Result<FlowConstants, DevError> {
    let mut a: Vec<Arg> = Vec::with_capacity(9);
    for d in args {
        a.push(match d {
            DevArg::Const { n, lanes } => Arg::Const { n: *n, lanes: *lanes },
            _ => Arg::Const { n: 1, lanes: [0; 4] },
        });
    }
    crate::kernels2::flow_constants(&a).map_err(|e| refuse(&format!("constants: {e}")))
}

pub fn optical_flow_run(s: &Arc<Session>, call: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 9 || call.tag != 3 {
        return Err(refuse("operand count or tag"));
    }
    let (w, h) = call.dims;
    let n = w * h;
    let c = flow_constants_of(args)?;
    let prev = operand_plane(&args[0], n)?;
    let next = operand_plane(&args[1], n)?;
    run_dev_flow_at(s, &prev, &next, w as i32, h as i32, &c, call.kp.flow_host_phase).map(|(d, _)| d)
}

/// One whole-run case: frame, the data kind, the iteration settings and the five scalar parameters.
#[derive(Clone, Copy, Debug)]
pub struct FlowCase {
    pub w: usize,
    pub h: usize,
    pub kind: u8,
    pub iters: (f32, f32, f32),
    pub lambda: f32,
    pub unit: f32,
    pub texture: f32,
    pub smooth: f32,
}

/// The frame pair of a case: `kind` 0 a smooth ramp and its shifted copy, 1 random, 2 a constant frame, 3 NaN of both signs and several
/// payloads, zeros of both signs, subnormals and infinities in both frames.
pub fn case_frames(c: &FlowCase) -> (Vec<f32>, Vec<f32>) {
    let n = c.w * c.h;
    let mut seed = 0x2545_f491_4f6c_dd1du64 ^ (n as u64 * 7919 + c.kind as u64);
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let ramp = |x_shift: f32, i: usize| {
        let (x, y) = ((i % c.w) as f32, (i / c.w) as f32);
        0.25 + 0.02 * (x - x_shift) + 0.015 * y + 0.2 * fx_common::shake_sin::sinf_1((x - x_shift) * 0.7)
    };
    match c.kind {
        0 => ((0..n).map(|i| ramp(0.0, i)).collect(), (0..n).map(|i| ramp(1.0, i)).collect()),
        1 => ((0..n).map(|_| (next() % 10_000) as f32 / 10_000.0).collect(), (0..n).map(|_| (next() % 10_000) as f32 / 10_000.0).collect()),
        2 => (vec![0.5; n], vec![0.5; n]),
        // Steep ramps: the gradient norm is above one at every pixel, the bottom right one included.
        4 => {
            let steep = |x_shift: f32, i: usize| {
                let (x, y) = ((i % c.w) as f32, (i / c.w) as f32);
                0.25 + 1.5 * (x - x_shift) + 1.25 * y + 0.5 * fx_common::shake_sin::sinf_1((x - x_shift) * 0.7)
            };
            ((0..n).map(|i| steep(0.0, i)).collect(), (0..n).map(|i| steep(1.0, i)).collect())
        }
        _ => {
            let specials = [
                f32::from_bits(0x7fc0_0000),
                f32::from_bits(0xffc0_0000),
                f32::from_bits(0x7fc0_1234),
                f32::from_bits(0xffc0_4321),
                f32::from_bits(0x7f80_0001),
                0.0,
                -0.0,
                f32::from_bits(1),
                f32::from_bits(0x8000_0007),
                f32::INFINITY,
                f32::NEG_INFINITY,
            ];
            let mut mk = |x_shift: f32| -> Vec<f32> {
                (0..n).map(|i| if next() % 5 == 0 { specials[(next() % specials.len() as u64) as usize] } else { ramp(x_shift, i) }).collect()
            };
            (mk(0.0), mk(1.0))
        }
    }
}

fn lane(v: f32) -> [u32; 4] {
    [v.to_bits(), 0, 0, 0]
}

pub fn case_consts(c: &FlowCase) -> Vec<(usize, usize, [u32; 4])> {
    vec![
        (2, 1, lane(c.iters.0)),
        (3, 1, lane(c.iters.1)),
        (4, 1, lane(c.iters.2)),
        (5, 1, lane(c.lambda)),
        (6, 1, lane(c.unit)),
        (7, 1, lane(c.texture)),
        (8, 1, lane(c.smooth)),
    ]
}

/// The self-check corpus cases: whole runs on small frames.
pub fn flow_cases() -> Vec<FlowCase> {
    let c = |w, h, kind, iters, lambda, unit, texture, smooth| FlowCase { w, h, kind, iters, lambda, unit, texture, smooth };
    vec![
        c(16, 16, 0, (1.0, 1.0, 1.0), 1.0, 0.0, 0.0, 0.0),
        c(16, 16, 0, (2.0, 2.0, 3.0), 1.0, 1.0, 0.5, 0.5),
        c(16, 16, 1, (2.0, 2.0, 3.0), 0.0, 1.0, 0.0, 0.5),
        c(9, 7, 0, (1.0, 2.0, 40.0), 1.0, 0.0, 0.0, 0.0),
        c(9, 7, 1, (2.0, 1.0, 2.0), 2.0, 1.0, 0.5, 0.0),
        c(8, 8, 0, (1.0, 1.0, 1.0), 1.0, 1.0, 0.5, 0.5),
        c(8, 8, 2, (1.0, 1.0, 1.0), 1.0, 1.0, 0.0, 0.0),
        c(16, 16, 3, (1.0, 1.0, 2.0), 1.0, 1.0, 0.0, 0.5),
        c(9, 7, 3, (2.0, 2.0, 2.0), 1.0, 0.0, 0.5, 0.0),
        c(16, 16, 0, (1.0, 1.0, 300.0), 1.0, 1.0, 0.0, 0.0),
        // A wide frame, past the 1,200 pixel row route boundary.
        c(1232, 8, 0, (2.0, 2.0, 1.0), 1.0, 1.0, 0.25, 0.2),
        // Gradient norms above one, the bottom right pixel included.
        c(12, 10, 4, (2.0, 2.0, 2.0), 1.0, 1.0, 0.5, 0.5),
        c(9, 7, 4, (1.0, 2.0, 3.0), 1.0, 0.0, 0.0, 0.5),
    ]
}

fn flow_selfcase(c: &FlowCase) -> SelfCase {
    let (a, b) = case_frames(c);
    let n = c.w * c.h;
    SelfCase {
        name: format!("{}x{} kind {} iters {:?} lambda {} unit {} texture {} smooth {}", c.w, c.h, c.kind, c.iters, c.lambda, c.unit, c.texture, c.smooth),
        dims: (c.w, c.h),
        tag: 3,
        args: vec![Image { n, planes: vec![a] }, Image { n, planes: vec![b] }],
        consts: case_consts(c),
        kp: None,
    }
}

pub fn optical_flow_corpus() -> Vec<SelfCase> {
    flow_cases().iter().filter(|c| flow_shape_refusal(c.w, c.h).is_none()).map(flow_selfcase).collect()
}

pub const OPTICAL_FLOW: DeviceOp = DeviceOp { class: OP_CLASS, accepts: optical_flow_accepts, run: optical_flow_run, corpus: optical_flow_corpus };
