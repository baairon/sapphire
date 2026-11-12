//! The composed host: every typed Call of the Glint three-plane worker mapped to a ported crate, called and never copied.
//! Mapped here: Blur (flag 0) to blur_core_phase, Fill (flag 0) to the zero fill, BrightPass to the
//! glow_layer bright pass, the abort polls, the scratch allocator and the region record. The flag shape of the blur is refused by name (GLINT-BLUR-FLAG-REFUSED) before any output.
use crate::mapped::{blur_flag0, bright_inplace, zero_fill, BLUR_ALIGN_DEFAULT};
use diagonal_box_sum::{box_sum_diagonals, Diagonal, Params as BoxParams};
use glint_diag_box::{run_body, Band, Params as DiagParams, RowPhases};
use glint_pass_cols::pass_b::{run_columns, Alignment as ColAlign, Config as ColCfg};
use glint_pass_rows::{run as run_rows, Alignment as RowAlign, Config as RowCfg};
use glint_worker::{Abort, BlockId, Call, CallbackArgs, Host, Operand, Pass, PassOp, PlaneId, Scratch};
use scaled_screen_blend::blend;
use glow_layer::PlaneMut;
use std::collections::HashMap;

/// Host-side byte phases of the allocations the passes depend on. Every field is a kernel parameter with default 0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Align {
    /// RADII-ALIGN: byte phase (mod 32) of the radii array of the row pass callback.
    pub radii_rows: u8,
    /// RADII-ALIGN: the same for the column pass callback.
    pub radii_cols: u8,
    /// BOXSUM-ALIGN: byte phase (mod 32) of the scratch row ring; the box-sum and diagonal row phases derive from it.
    pub boxsum: usize,
    /// BLUR-ALIGN: base phase of the blur passes.
    pub blur: usize,
}

pub struct ComposedHost {
    pub planes: HashMap<PlaneId, Vec<f32>>,
    /// The three plane handles the bright pass works on in place (the worker's `cells` array).
    pub cells: [PlaneId; 3],
    /// Thresholds: the float[3] the worker receives as its first stack argument.
    pub th: [f32; 3],
    /// The aspect the blur driver reads from the object record, None when its field is off.
    pub aspect: Option<f32>,
    pub phase: usize,
    pub refused: Option<&'static str>,
    pub calls: usize,
    /// The first error a callee crate returned (a named refusal or a crate error), if any.
    pub err: Option<String>,
    pub last_alloc: (u64, u32),
    /// Byte phase (mod 32) of the scratch block's first row (a host value set by the allocator).
    pub ring_phase: usize,
    /// Byte phase (mod 32) of the radii array the worker keeps in its frame, per pass callback (RADII-ALIGN).
    pub radii_phase_rows: u8,
    pub radii_phase_cols: u8,
    /// Callbacks run, by name.
    pub ran: Vec<&'static str>,
}

impl ComposedHost {
    /// Sets the host phases from `a`.
    pub fn set_align(&mut self, a: &Align) {
        self.radii_phase_rows = a.radii_rows;
        self.radii_phase_cols = a.radii_cols;
        self.ring_phase = a.boxsum;
        self.phase = a.blur;
    }
    pub fn new(cells: [PlaneId; 3], th: [f32; 3], aspect: Option<f32>) -> Self {
        ComposedHost { planes: HashMap::new(), cells, th, aspect, phase: BLUR_ALIGN_DEFAULT, refused: None, calls: 0, err: None, last_alloc: (0, 0), ring_phase: 0, radii_phase_rows: 0, radii_phase_cols: 0, ran: Vec::new() }
    }
}

impl Host for ComposedHost {
    fn call(&mut self, c: &Call, _args: &CallbackArgs) {
        self.calls += 1;
        if self.refused.is_some() {
            return;
        }
        match c {
            Call::Blur { blurred, source, blur_x, roi, flagged, .. } => {
                if *flagged {
                    self.refused = Some(crate::refusal::BLUR_FLAG);
                    return;
                }
                let (w, h) = (roi.w as usize, roi.h as usize);
                let src = self.planes.get(&source.expect("blur source plane id")).cloned().expect("blur source plane");
                let dst = self.planes.get_mut(blurred).expect("blur destination plane");
                blur_flag0(&src, dst, w, h, f32::from_bits(*blur_x), self.aspect, self.phase).expect("blur_core");
            }
            Call::Fill { output, flag, roi } => {
                if *flag != 0 {
                    self.refused = Some(crate::refusal::BLUR_FLAG);
                    return;
                }
                let d = self.planes.get_mut(output).expect("fill plane");
                zero_fill(d, roi.c as usize, roi.h as usize).expect("zero_fill");
            }
            Call::BrightPass { flag, roi } => {
                if *flag != 0 {
                    self.refused = Some(crate::refusal::BLUR_FLAG);
                    return;
                }
                let (w, h) = (roi.w as usize, roi.h as usize);
                let mut taken: Vec<Vec<f32>> = self.cells.iter().map(|k| self.planes.remove(k).expect("bright plane")).collect();
                {
                    let mut it = taken.iter_mut();
                    let mut pl: Vec<PlaneMut> = (0..3).map(|_| PlaneMut { data: it.next().unwrap(), width: w, height: h, stride: w }).collect();
                    bright_inplace(self.th, &mut pl).expect("bright pass");
                }
                for (k, v) in self.cells.iter().zip(taken) {
                    self.planes.insert(*k, v);
                }
            }
            Call::CheckAbort(_) | Call::Free(_) | Call::Alloc { .. } => {}
            Call::CopyPlane { layer, plane, bytes } => {
                let n = (*bytes / 4) as usize;
                let src = self.planes.get(plane).cloned().expect("copy source");
                let dst = self.planes.get_mut(layer).expect("copy destination");
                dst[..n].copy_from_slice(&src[..n]);
            }
            Call::Parallel { op, radii } => self.parallel(*op, *radii, _args),
        }
    }
    fn alloc(&mut self, _ring: Scratch, bytes: u64, slots: u32) -> Option<BlockId> {
        self.last_alloc = (bytes, slots);
        None
    }
    fn abort(&mut self, _which: Abort) -> i32 {
        0
    }
}

fn plane_id(r: Operand) -> PlaneId {
    match r {
        Operand::Plane(p) => p,
        _ => panic!("operand is not a plane"),
    }
}

impl ComposedHost {
    /// Byte phase (mod 32) of ring row `i` of the last scratch block: the allocator hands out base + i * stride, stride = bytes rounded up to 16.
    fn row_phase(&self, i: usize) -> u8 {
        let stride = ((self.last_alloc.0 as usize) + 15) & !15;
        ((self.ring_phase + i * stride) % 32) as u8
    }
    fn fail(&mut self, what: String) {
        if self.err.is_none() {
            self.err = Some(what);
        }
    }
    fn roles(&self, ctx: &CallbackArgs) -> (PlaneId, PlaneId) {
        // (source, destination): the plane cell is read and the layer plane is written
        (plane_id(ctx.source), plane_id(ctx.dest))
    }
    fn parallel(&mut self, op: PassOp, radii: Option<[i32; 3]>, ctx: &CallbackArgs) {
        let (w, h) = (ctx.width as usize, ctx.height as usize);
        match op {
            PassOp::Scale => {
                self.ran.push("scale");
                let (dest_id, source_id) = (plane_id(ctx.scale_dest), plane_id(ctx.scale_source));
                let n = ctx.element_count as usize;
                let src = self.planes.get(&source_id).cloned().expect("scale source");
                let dst = self.planes.get_mut(&dest_id).expect("scale destination");
                blend(&mut dst[..n], &src[..n], f32::from_bits(ctx.scale_bits));
            }
            PassOp::Horizontal | PassOp::Vertical | PassOp::DiagonalBody(_) | PassOp::DiagonalBox(_) if crate::refusal::shift_out_of_range(ctx.shift as i32) => {
                self.fail(crate::refusal::SHIFT.into());
            }
            PassOp::Horizontal | PassOp::Vertical => {
                let r = radii.expect("pass radii");
                let (source_id, dest_id) = self.roles(ctx);
                let src = self.planes.get(&source_id).cloned().expect("pass source");
                let mut dst = self.planes.remove(&dest_id).expect("pass destination");
                let cfg_edge = ctx.edge_mode as i32;
                let shift = ctx.shift as i32;
                let scratch = [self.row_phase(0), self.row_phase(1)];
                let res = if op == PassOp::Horizontal {
                    self.ran.push("pass0");
                    let phases = RowAlign { scratch, radii: self.radii_phase_rows };
                    run_rows(&RowCfg { edge: cfg_edge, radii: &r, shift }, &phases, &src, &mut dst, w, h).map_err(|e| format!("pass0 {e:?}"))
                } else {
                    self.ran.push("pass1");
                    let phases = ColAlign { scratch, radii: self.radii_phase_cols };
                    run_columns(&ColCfg { edge: cfg_edge, radii: &r, shift }, &phases, &src, &mut dst, w, h).map_err(|e| format!("pass1 {e:?}"))
                };
                self.planes.insert(dest_id, dst);
                if let Err(e) = res {
                    self.fail(e);
                }
            }
            PassOp::DiagonalBody(which) => {
                let r = radii.expect("diagonal radii");
                let (source_id, dest_id) = self.roles(ctx);
                if crate::refusal::extent_refused(w as i32, h as i32) {
                    self.fail(crate::refusal::EXTENT.into());
                    return;
                }
                let src = self.planes.get(&source_id).cloned().expect("diag source");
                let mut dst = self.planes.remove(&dest_id).expect("diag destination");
                let (body, name) = if which == Pass::MainDiagonal { (false, "diagA") } else { (true, "diagB") };
                self.ran.push(name);
                let ph = RowPhases([self.row_phase(0) as u32, self.row_phase(1) as u32, self.row_phase(2) as u32]);
                let n = glint_worker::partition_count(w as i32, h as i32);
                let band = Band { start: 0, count: n };
                if !crate::refusal::band_ok(band.start, band.count, n) {
                    self.planes.insert(dest_id, dst);
                    self.fail(crate::refusal::BAND.into());
                    return;
                }
                let mut out = dst.clone();
                out.extend_from_slice(&[0.0; 8]);
                let res = run_body(body, &DiagParams { radii: r, d: ctx.shift as i32 }, &ph, band, &src, &mut out, w, h).map_err(|e| format!("{name} {e:?}"));
                let k = dst.len();
                dst.copy_from_slice(&out[..k]);
                self.planes.insert(dest_id, dst);
                if let Err(e) = res {
                    self.fail(e);
                }
            }
            PassOp::DiagonalBox(which) => {
                let r = radii.expect("box radii");
                let dest_id = plane_id(ctx.dest);
                let mut img = self.planes.remove(&dest_id).expect("box image");
                let (kind, name) = if which == Pass::MainDiagonal { (Diagonal::Main, "boxMain") } else { (Diagonal::Anti, "boxAnti") };
                self.ran.push(name);
                let mut phases = [0u8; 5];
                for (i, p) in phases.iter_mut().enumerate() {
                    *p = self.row_phase(i);
                }
                let p = BoxParams { w: w as i32, h: h as i32, radii: r, shift: ctx.shift as i32, phases, start: 0, count: w as i32 + h as i32 - 1, kind };
                let n = w * h;
                let res = box_sum_diagonals(&mut img[..n], &p).map_err(|e| format!("{name} {e:?}"));
                self.planes.insert(dest_id, img);
                if let Err(e) = res {
                    self.fail(e);
                }
            }
        }
    }
}
