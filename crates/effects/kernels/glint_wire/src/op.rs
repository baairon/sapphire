//! The glint op entry: the plan's source planes and six float arguments through the wide-vector wrapper into the three-plane
//! worker and the composed host.
//!
//! The arguments are taken in plan order and filled by meaning: thresholds (3 lanes), strength (1), A (3), B (3), T0 (4) and T1 (4).
//! The op builds the source record the worker reads from the served frame ([`source_record`]); the layer's scratch planes are
//! allocated here, so the op always runs the layout without the copy detour.
use crate::host::{Align, ComposedHost};
use crate::refusal;
use glint_dispatch::{refusal_of, wrapper, Error as DispatchError, GlintArgs, Host as DispatchHost, Layer, Roi as WrapRoi, Scratch as WrapScratch, ThreeArgs};
use glint_worker::{Input, PlaneId, Roi, SourceRecord};

/// The layout answer of the op: three planes per frame.
pub const THREE_PLANE_LAYOUT: i32 = 3;
/// The value the op stores in the source record's diagonal-mode field (the record's word at +0x3c). No input the stage serves
/// sets it, so the diagonal route is never taken by a rendered frame (label DIAG-ROUTE-UNREACHED-BY-STAGE).
pub const SERVED_DIAGONAL_MODE: i32 = 0;

/// One call of the op.
#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub width: usize,
    pub height: usize,
    /// The render scale the wrapper multiplies into the strength and the blur widths (the frame width unless the host sets it).
    pub count: i32,
    pub sources: [&'a [f32]; 3],
    pub thresholds: [f32; 3],
    pub strength: f32,
    pub a: [f32; 3],
    pub b: [f32; 3],
    /// Per-pass radius scales (plan argument T0).
    pub pass_scale: [f32; 4],
    /// Per-pass gains (plan argument T1).
    pub pass_gain: [f32; 4],
    pub align: Align,
    /// The low byte of the wrapper's fourth argument; the op passes 0, any other value is refused.
    pub flag: u8,
}

/// The source record the op builds from the served frame. The diagonal-mode field is fixed to [`SERVED_DIAGONAL_MODE`] because
/// the frame carries no input that sets it.
pub fn source_record(width: usize, height: usize, count: i32) -> Result<SourceRecord, String> {
    let (w, h) = (i32::try_from(width).map_err(|_| "frame width over i32")?, i32::try_from(height).map_err(|_| "frame height over i32")?);
    let bytes = width.checked_mul(height).and_then(|n| n.checked_mul(4)).and_then(|n| i32::try_from(n).ok()).ok_or("frame size over i32")?;
    Ok(SourceRecord { selector: 0, plane_bytes: bytes, count, width: w, height: h, diagonal_mode: SERVED_DIAGONAL_MODE })
}

struct Wire {
    record: SourceRecord,
    host: ComposedHost,
    dsts: [PlaneId; 3],
    err: Option<String>,
}

fn bits3(v: [f32; 3]) -> [u32; 3] {
    v.map(f32::to_bits)
}
fn bits4(v: [f32; 4]) -> [u32; 4] {
    v.map(f32::to_bits)
}

impl DispatchHost for Wire {
    fn layout(&mut self) -> i32 {
        THREE_PLANE_LAYOUT
    }
    fn region(&mut self, s: &SourceRecord) -> WrapRoi {
        WrapRoi { w: s.width, h: s.height, c: s.width, d: 0, e: 4 }
    }
    fn copy_plane(&mut self, _dst: PlaneId, _src: PlaneId, _row: i32, _roi: &WrapRoi) {
        self.err.get_or_insert_with(|| "glint: the plane copy detour is not taken by the op".to_string());
    }
    fn worker_three(&mut self, a: &ThreeArgs) {
        if a.saved_index.is_some() {
            self.err.get_or_insert_with(|| "glint: a saved plane is outside the op".to_string());
            return;
        }
        let inp = Input {
            source: self.record,
            blur_x: a.scale.to_bits(),
            blurred: a.blurred,
            sources: a.sources,
            outputs: self.dsts,
            layer: a.layer,
            saved: None,
            a: bits3(a.a),
            b: bits3(a.b),
            c: bits3(a.c),
            pass_scale: bits4(a.pass_scale),
            pass_gain: bits4(a.pass_gain),
            roi: Roi { w: a.roi.w, h: a.roi.h, c: a.roi.c, d: a.roi.d, e: a.roi.e },
            flag: a.flag as i32,
        };
        if let Err(e) = crate::render(&inp, &mut self.host) {
            self.err.get_or_insert(e);
        }
    }
}

/// Renders one frame; the three output planes. Every refusal returns before any plane is produced.
pub fn render(req: &Request) -> Result<[Vec<f32>; 3], String> {
    let n = req.width.checked_mul(req.height).ok_or("frame size overflow")?;
    let record = source_record(req.width, req.height, req.count)?;
    let args = GlintArgs { strength: req.strength, a: req.a, b: req.b, pass_scale: req.pass_scale, pass_gain: req.pass_gain };
    // the refusal predicate decides from the request alone, before any plane is built
    if let Some(r) = refusal_of(&record, &args, req.flag) {
        return Err(r.name().to_string());
    }
    if n == 0 || req.sources.iter().any(|p| p.len() != n) {
        return Err(format!("glint: the source planes must hold {}x{} samples", req.width, req.height));
    }
    let src_ids = [PlaneId(1), PlaneId(2), PlaneId(3)];
    let planes = vec![PlaneId(10), PlaneId(11), PlaneId(12), PlaneId(13)];
    let dsts = [PlaneId(21), PlaneId(22), PlaneId(23)];
    let mut host = ComposedHost::new([planes[1], planes[2], planes[3]], req.thresholds, None);
    host.set_align(&req.align);
    for k in 0..3 {
        host.planes.insert(src_ids[k], req.sources[k].to_vec());
        host.planes.insert(planes[1 + k], vec![0.0; n]);
        host.planes.insert(dsts[k], vec![0.0; n]);
    }
    host.planes.insert(planes[0], vec![0.0; n]);
    let layer = Layer {
        planes,
        scratch: WrapScratch { kind: 0, src: src_ids },
        args,
    };
    let mut wire = Wire { record, host, dsts, err: None };
    wrapper(&mut wire, &layer, &record, req.flag).map_err(|e| match e {
        DispatchError::Refused(r) => r.name().to_string(),
        DispatchError::LayoutRefused => refusal::LAYOUT.to_string(),
        e => format!("glint: {:?}", e),
    })?;
    if let Some(e) = wire.err {
        return Err(e);
    }
    let take = |h: &mut ComposedHost, id: PlaneId| h.planes.remove(&id).ok_or_else(|| "glint: an output plane is missing".to_string());
    Ok([take(&mut wire.host, dsts[0])?, take(&mut wire.host, dsts[1])?, take(&mut wire.host, dsts[2])?])
}

/// Stand-in labels of the op entry.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "DIAG-ROUTE-UNREACHED-BY-STAGE: the source record's diagonal-mode word is 0 for every served frame",
        "COUNT-IS-RENDER-SCALE: the wrapper's count is the render scale, the frame width unless the host sets it",
        "RADII-ALIGN, BOXSUM-ALIGN, BLUR-ALIGN: host byte phases, default 0",
        "ROUTE AVX2-ONLY: the wide-vector body runs unconditionally",
    ]
}
