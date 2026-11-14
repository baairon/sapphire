//! Composed host of the flow-build body: maps every `glue::Call` of the flag-0 route to the plane callbacks of this crate over a typed plane
//! arena. Ordinary Rust: no program counter, no register or flag state, no address-keyed memory. The row splitter 0x6701b0 is
//! modelled by `chunk`, the shared partition text (`split.rs`, label SPLIT-MODEL). Workers run one after the other (SERIAL-WORKERS).
use crate::host_types::*;
use crate::resample::{resample_h, resample_v};
use crate::{add_const, affine_rows, flow_stencil_rows, normalize_flow_rows, sub_scaled_rows, sum_rows};
use std::collections::BTreeMap;

pub use crate::split::split as chunk;

/// A float plane with the byte phase (address modulo 32) of its first element.
#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    pub data: Vec<f32>,
    pub phase: usize,
}

/// Which unhosted helper (or malformed launch) a host error names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperName {
    Fill,
    SmoothStep,
    BlendStep,
    ForeignContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostError {
    UnknownHandle(u64),
    /// the flag-word route (the image handle helpers) is not hosted
    FlagRouteNotHosted(HelperName),
    /// a plane copy with no registered host source
    NoSource(u64),
    /// the resample launch has no registered destination planes
    NoResampleTargets,
    Short(u64),
}

/// Planes the resample launches read and write (set by the caller of the level loop; the glue's argument block carries only the sizes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResampleTargets {
    pub h_dst: u64,
    pub v_src: u64,
    pub v_dst: u64,
}

#[derive(Clone)]
pub struct ComposedHost {
    pub planes: BTreeMap<u64, Plane>,
    /// host input images for 0x673100 (by destination handle)
    pub inputs: BTreeMap<u64, Vec<f32>>,
    pub threads: i32,
    pub targets: ResampleTargets,
    pub err: Option<HostError>,
    /// calls seen, in order
    pub log: Vec<Call>,
    /// byte phase of a plane by id, given by the host's placement (planes not listed take phase 0)
    pub phase_of: BTreeMap<u64, usize>,
    /// byte phase of a served plane by (level, plane ordinal), given by the host's placement
    pub served_phase: BTreeMap<(usize, usize), usize>,
}

impl ComposedHost {
    pub fn new(threads: i32) -> Self {
        ComposedHost {
            planes: BTreeMap::new(),
            inputs: BTreeMap::new(),
            threads,
            targets: ResampleTargets::default(),
            err: None,
            log: Vec::new(),
            phase_of: BTreeMap::new(),
            served_phase: BTreeMap::new(),
        }
    }
    pub fn add_plane(&mut self, h: u64, data: Vec<f32>, phase: usize) {
        self.planes.insert(h, Plane { data, phase });
    }
    fn fail(&mut self, e: HostError) {
        if self.err.is_none() {
            self.err = Some(e);
        }
    }
    fn take(&mut self, h: u64) -> Option<Plane> {
        match self.planes.remove(&h) {
            Some(p) => Some(p),
            None => {
                self.fail(HostError::UnknownHandle(h));
                None
            }
        }
    }
    fn dims(r: &Rect24) -> (i32, i32, i32, u64) {
        (r.lo as u32 as i32, (r.lo >> 32) as u32 as i32, (r.lo >> 64) as u32 as i32, r.hi)
    }

    fn parallel(&mut self, cb: Callback, ctx: Ctx) {
        let t = self.threads;
        // the tile launch is the resample pair over the second plane of the level
        let ctx = match ctx {
            Ctx::Tile { prev_width, prev_height, source, cur_width, cur_height, .. } => Ctx::Pair { prev_width, prev_height, source, cur_width, cur_height },
            c => c,
        };
        match (cb, ctx) {
            (Callback::Smooth, Ctx::Smooth { rect, work, flow_x, flow_y, source_copy, weight }) => {
                let (w, h, pitch, _) = Self::dims(&rect);
                let (Some(mut out), Some(b), Some(c), Some(d)) = (self.take(work), self.take(flow_x), self.take(flow_y), self.take(source_copy)) else { return };
                for tid in 0..t {
                    let (row_start, n) = chunk(h, t, tid);
                    flow_stencil_rows(&mut out.data, &b.data, &c.data, &d.data, w, h, pitch, f32::from_bits(weight), row_start, n, b.phase);
                }
                self.planes.insert(work, out);
                self.planes.insert(flow_x, b);
                self.planes.insert(flow_y, c);
                self.planes.insert(source_copy, d);
            }
            (Callback::Blend, Ctx::Blend { rect, work, flow_x, flow_y, weight, pair_weight }) => {
                let (w, h, pitch, _) = Self::dims(&rect);
                let (Some(p), Some(mut fx), Some(mut fy)) = (self.take(work), self.take(flow_x), self.take(flow_y)) else { return };
                for tid in 0..t {
                    let (row_start, n) = chunk(h, t, tid);
                    normalize_flow_rows(&p.data, &mut fx.data, &mut fy.data, w, h, pitch, row_start, n, f32::from_bits(pair_weight), f32::from_bits(weight));
                }
                self.planes.insert(work, p);
                self.planes.insert(flow_x, fx);
                self.planes.insert(flow_y, fy);
            }
            (Callback::FillDiff, Ctx::Fill { rect, operand_plane, source_plane, destination, weight }) => {
                let (w, h, pitch, _) = Self::dims(&rect);
                // the difference may run in place (destination is operand_plane): the minuend is then a copy of the output plane
                let alias = operand_plane == destination;
                let minuend_taken = if alias { None } else { self.take(operand_plane) };
                let (Some(a), Some(mut out)) = (self.take(source_plane), self.take(destination)) else { return };
                let b = if alias { out.clone() } else if let Some(b) = minuend_taken { b } else { return };
                for tid in 0..t {
                    let (row_start, n) = chunk(h, t, tid);
                    sub_scaled_rows(&a.data, &b.data, &mut out.data, w, pitch, row_start, n, f32::from_bits(weight), a.phase);
                }
                if !alias {
                    self.planes.insert(operand_plane, b);
                }
                self.planes.insert(source_plane, a);
                self.planes.insert(destination, out);
            }
            (Callback::ResampleHoriz, Ctx::Pair { prev_width, prev_height, source, cur_width, .. }) => {
                let tg = self.targets;
                let (Some(src), Some(mut dst)) = (self.take(source), self.take(tg.h_dst)) else {
                    self.fail(HostError::NoResampleTargets);
                    return;
                };
                for tid in 0..t {
                    let (row_start, n) = chunk(prev_height as i32, t, tid);
                    resample_h(&src.data, prev_width as i32, prev_height as i32, cur_width as i32, &mut dst.data, src.phase, row_start, n);
                }
                self.planes.insert(source, src);
                self.planes.insert(tg.h_dst, dst);
            }
            (Callback::ResampleVert, Ctx::Pair { prev_height, cur_width, cur_height, .. }) => {
                let tg = self.targets;
                let (Some(src), Some(mut dst)) = (self.take(tg.v_src), self.take(tg.v_dst)) else {
                    self.fail(HostError::NoResampleTargets);
                    return;
                };
                for tid in 0..t {
                    let (col_start, n) = chunk(cur_width as i32, t, tid);
                    resample_v(&src.data, cur_width as i32, prev_height as i32, cur_height as i32, &mut dst.data, col_start, n);
                }
                self.planes.insert(tg.v_src, src);
                self.planes.insert(tg.v_dst, dst);
            }
            _ => self.fail(HostError::FlagRouteNotHosted(HelperName::ForeignContext)),
        }
    }

    /// Partial sums of a plane launched like 0x46f230. ONE-CHUNK: the measured renders run at thread count 1 (all splitter
    /// calls (0, total)), so one sequential pass covers all rows and its partial goes to slot 0, whatever `threads` is.
    pub fn sum_slots(&self, h: u64, w: i32, rows: i32, stride: i32) -> Option<[f32; 128]> {
        let p = self.planes.get(&h)?;
        let mut slots = [0.0f32; 128];
        slots[0] = sum_rows(&p.data, w, stride, 0, rows, p.phase);
        Some(slots)
    }

    /// Zero fill / constant add over `n` elements (0x675d00 behind 0x676140).
    pub fn add_const_plane(&mut self, h: u64, n: i64, c: f32) {
        let Some(mut p) = self.take(h) else { return };
        for tid in 0..self.threads {
            let (s, k) = chunk(n as i32, self.threads, tid);
            add_const(&mut p.data[s as usize..], k as i64, c);
        }
        self.planes.insert(h, p);
    }

    /// In place affine map of a plane (0x46fa00).
    pub fn affine_plane(&mut self, h: u64, w: i32, rows: i32, stride: i32, scale: f32, offset: f32) {
        let Some(mut p) = self.take(h) else { return };
        for tid in 0..self.threads {
            let (row_start, n) = chunk(rows, self.threads, tid);
            affine_rows(&mut p.data, w, stride, row_start, n, scale, offset, p.phase);
        }
        self.planes.insert(h, p);
    }
}

/// The plane services the level loop takes from its host: plane registration and reads, the affine map, the row sums and the
/// constant add. `ComposedHost` is the one shipped implementation.
pub trait FlowServices: Host {
    /// Registers (or replaces) the plane `id` with its words and byte phase.
    fn register_plane(&mut self, id: PlaneId, data: Vec<f32>, phase: usize);
    /// Registers a zero plane of `len` words under `id` when none is registered.
    fn ensure_plane(&mut self, id: PlaneId, len: usize);
    /// Registers a zero plane of `len` words under `id` for the served plane `ordinal` of `level` (from 1) when none is registered;
    /// a host with a placement gives it the phase the placement assigns to that level and plane.
    fn ensure_level_plane(&mut self, level: usize, ordinal: usize, id: PlaneId, len: usize) {
        let _ = (level, ordinal);
        self.ensure_plane(id, len);
    }
    /// The words of plane `id`, when registered.
    fn plane_words(&self, id: PlaneId) -> Option<&[f32]>;
    /// Points the resample launches at their intermediate and destination planes.
    fn set_resample_targets(&mut self, targets: ResampleTargets);
    fn affine_plane(&mut self, h: PlaneId, w: i32, rows: i32, stride: i32, scale: f32, offset: f32);
    fn sum_slots(&self, h: PlaneId, w: i32, rows: i32, stride: i32) -> Option<[f32; 128]>;
    fn add_const_plane(&mut self, h: PlaneId, n: i64, c: f32);
}

impl FlowServices for ComposedHost {
    fn register_plane(&mut self, id: PlaneId, data: Vec<f32>, phase: usize) {
        self.add_plane(id, data, phase);
    }
    fn ensure_plane(&mut self, id: PlaneId, len: usize) {
        let phase = self.phase_of.get(&id).copied().unwrap_or(0);
        self.planes.entry(id).or_insert(Plane { data: vec![0.0; len], phase });
    }
    fn ensure_level_plane(&mut self, level: usize, ordinal: usize, id: PlaneId, len: usize) {
        let phase = self.served_phase.get(&(level, ordinal)).copied().unwrap_or(0);
        self.planes.entry(id).or_insert(Plane { data: vec![0.0; len], phase });
    }
    fn plane_words(&self, id: PlaneId) -> Option<&[f32]> {
        self.planes.get(&id).map(|p| p.data.as_slice())
    }
    fn set_resample_targets(&mut self, targets: ResampleTargets) {
        self.targets = targets;
    }
    fn affine_plane(&mut self, h: PlaneId, w: i32, rows: i32, stride: i32, scale: f32, offset: f32) {
        ComposedHost::affine_plane(self, h, w, rows, stride, scale, offset)
    }
    fn sum_slots(&self, h: PlaneId, w: i32, rows: i32, stride: i32) -> Option<[f32; 128]> {
        ComposedHost::sum_slots(self, h, w, rows, stride)
    }
    fn add_const_plane(&mut self, h: PlaneId, n: i64, c: f32) {
        ComposedHost::add_const_plane(self, h, n, c)
    }
}

impl Host for ComposedHost {
    fn call(&mut self, c: Call) {
        self.log.push(c.clone());
        match c {
            Call::ParallelFor { cb, ctx, .. } => self.parallel(cb, ctx),
            Call::CopyIn { dst, rect, .. } => {
                let (_, h, pitch, _) = Self::dims(&rect);
                let n = (h as i64 * pitch as i64).max(0) as usize;
                match self.inputs.get(&dst).cloned() {
                    None => self.fail(HostError::NoSource(dst)),
                    Some(src) => match self.planes.get_mut(&dst) {
                        Some(p) if p.data.len() >= n && src.len() >= n => p.data[..n].copy_from_slice(&src[..n]),
                        Some(_) => self.fail(HostError::Short(dst)),
                        None => self.fail(HostError::UnknownHandle(dst)),
                    },
                }
            }
            Call::CopyPlane { dst, src, rect, .. } => {
                let (_, h, pitch, _) = Self::dims(&rect);
                let n = (h as i64 * pitch as i64).max(0) as usize;
                let Some(s) = self.planes.get(&src).map(|p| p.data.clone()) else { return self.fail(HostError::UnknownHandle(src)) };
                match self.planes.get_mut(&dst) {
                    Some(p) if p.data.len() >= n && s.len() >= n => p.data[..n].copy_from_slice(&s[..n]),
                    Some(_) => self.fail(HostError::Short(dst)),
                    None => self.fail(HostError::UnknownHandle(dst)),
                }
            }
            Call::Fill { .. } => self.fail(HostError::FlagRouteNotHosted(HelperName::Fill)),
            Call::SmoothStep { .. } => self.fail(HostError::FlagRouteNotHosted(HelperName::SmoothStep)),
            Call::BlendStep { .. } => self.fail(HostError::FlagRouteNotHosted(HelperName::BlendStep)),
        }
    }
}
