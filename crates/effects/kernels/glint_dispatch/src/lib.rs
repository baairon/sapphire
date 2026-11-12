//! Glint render wrapper (the wide-vector route).
//!
//! ROUTE AVX2-ONLY: the wide-vector body runs unconditionally; the narrower vector arm, the scalar arm and the one-time initialisation
//! are not rebuilt.
//!
//! [`wrapper`] reads the layer object (plane array, glint arguments, a record of scratch planes), asks the host
//! which plane layout is in use, computes the scale factors, builds the region record, optionally copies the source
//! plane into two scratch planes, and then hands one request to the three-plane worker. The one-plane layout is refused by
//! name ([`NAME_LAYOUT`]) before the region query and the copy detour. Every host service is a method of [`Host`]; the worker
//! request is a typed value ([`ThreeArgs`]), so the pixel work and its callbacks live in the callee crates behind the host.
//!
//! Float steps go through `fx_common::nanop` (a fresh NaN is 0xffc00000).
use fx_common::nanop::NanOps;
pub use glint_worker::{PlaneId, Refusal, SourceRecord};

/// The refusal of the one-plane layout.
pub const NAME_LAYOUT: &str = "GLINT-LAYOUT-REFUSED (glint: only the three-plane layout is rendered, the one-plane layout is outside the op)";
use glint_worker::{refusal, Words};

/// 0x160d00c: scale applied to the strength of the three-plane layout (2^-5).
pub const SCALE_STRENGTH: f32 = 0.03125;
/// 0x15e8d94: scale applied to the blur widths (2^-3).
pub const SCALE_WIDTH: f32 = 0.125;
/// 0x15e8548: gate scale (4.0).
pub const SCALE_GATE: f32 = 4.0;

/// The 24 byte region record: four dwords and a qword.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Roi {
    pub w: i32,
    pub h: i32,
    pub c: i32,
    pub d: i32,
    pub e: i64,
}

/// The per-layer record the wrapper reads from the object.
#[derive(Clone, Debug)]
pub struct Scratch {
    /// Record kind (1 selects the three-plane copy detour).
    pub kind: i32,
    /// Three source planes.
    pub src: [PlaneId; 3],
}

/// The glint arguments, filled by plan argument index.
#[derive(Clone, Debug)]
pub struct GlintArgs {
    pub strength: f32,
    /// Blur widths per channel.
    pub a: [f32; 3],
    /// Gates per channel.
    pub b: [f32; 3],
    /// Per-pass radius scales, forwarded to the worker.
    pub pass_scale: [f32; 4],
    /// Per-pass gains, forwarded to the worker.
    pub pass_gain: [f32; 4],
}

/// The layer fields the wrapper reads.
#[derive(Clone, Debug)]
pub struct Layer {
    /// Plane array (output plane first).
    pub planes: Vec<PlaneId>,
    pub scratch: Scratch,
    pub args: GlintArgs,
}

/// Request to the three-plane worker.
#[derive(Clone, Debug, PartialEq)]
pub struct ThreeArgs {
    pub sources: [PlaneId; 3],
    pub blurred: [PlaneId; 3],
    pub layer: PlaneId,
    /// Index into the plane array of the optional saved plane.
    pub saved_index: Option<i64>,
    pub scale: f32,
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub c: [f32; 3],
    pub pass_scale: [f32; 4],
    pub pass_gain: [f32; 4],
    pub roi: Roi,
    pub flag: u8,
}

/// Services the wrapper needs.
pub trait Host {
    /// The object's virtual layout query ([obj+0xf8]); the wrapper asks three times.
    fn layout(&mut self) -> i32;
    /// The region record of the source.
    fn region(&mut self, src: &SourceRecord) -> Roi;
    /// Copy of the source plane into a scratch plane (handles, row dword, region record with element size).
    fn copy_plane(&mut self, dst: PlaneId, src: PlaneId, row: i32, roi: &Roi);
    fn worker_three(&mut self, a: &ThreeArgs);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// The original reads memory at address 0 (the three-plane copy detour without a scratch pair).
    NullRead,
    /// A plane index outside the array.
    PlaneRange(i64),
    /// A named refusal decided from the request alone, before any host call other than the layout query.
    Refused(Refusal),
    /// The one-plane layout: refused by [`NAME_LAYOUT`] before any host call other than the layout query.
    LayoutRefused,
}

fn plane(l: &Layer, i: i64) -> Result<PlaneId, Error> {
    if i < 0 || i as usize >= l.planes.len() {
        return Err(Error::PlaneRange(i));
    }
    Ok(l.planes[i as usize])
}

fn three(l: &Layer, i: i64) -> Result<[PlaneId; 3], Error> {
    Ok([plane(l, i)?, plane(l, i + 1)?, plane(l, i + 2)?])
}

/// The scaled words of one call, one text for the predicate and the worker requests.
struct Scaled {
    scale: f32,
    count_f: f32,
}

fn scaled(l: &GlintArgs, count: i32) -> Scaled {
    let count_f = count as f32;
    Scaled { scale: SCALE_STRENGTH.fmul(l.strength).fmul(count_f), count_f }
}

fn three_words(args: &GlintArgs, s: &Scaled) -> ([f32; 3], [f32; 3]) {
    let w = s.count_f.fmul(SCALE_WIDTH);
    ([w.fmul(args.a[0]), w.fmul(args.a[1]), w.fmul(args.a[2])], [SCALE_GATE.fmul(args.b[0]), SCALE_GATE.fmul(args.b[1]), SCALE_GATE.fmul(args.b[2])])
}

/// The refusal predicate over a call of the wrapper, from the request alone. The region is
/// { width, height, width, d, e } of the source record. Used by the wrapper as its first act and by the op entry before any plane is built.
pub fn refusal_of(src: &SourceRecord, args: &GlintArgs, flag: u8) -> Option<Refusal> {
    let s = scaled(args, src.count);
    let (a, b) = three_words(args, &s);
    let words = Words { a: a.map(f32::to_bits), b: b.map(f32::to_bits), c: [0; 3], pass_scale: args.pass_scale.map(f32::to_bits), pass_gain: args.pass_gain.map(f32::to_bits) };
    refusal(src, flag as i32, src.width, src.height, &words)
}

/// The wide-vector wrapper. `src` is the source record (its +0x8 word and its count at +0x20 are read here); `flag` is the low byte of the fourth
/// argument. The layout queries come first, then the layout refusal and the refusal predicate, then the region query and the copy detour.
pub fn wrapper<H: Host>(host: &mut H, l: &Layer, src: &SourceRecord, flag: u8) -> Result<(), Error> {
    let kind_first = host.layout();
    let kind_second = host.layout();
    let kind_third = host.layout();
    if kind_third == 1 {
        return Err(Error::LayoutRefused);
    }
    if let Some(r) = refusal_of(src, &l.args, flag) {
        return Err(Error::Refused(r));
    }
    // pair: index of the scratch pair in the plane array when the layout is 3 and the record kind is 1.
    let mut pair: Option<i64> = None;
    let mut first: i64 = 1;
    if kind_first == 3 && l.scratch.kind == 1 {
        pair = Some(1);
        first = 3;
    }
    let saved_index = if flag != 0 { Some((kind_second.wrapping_add(first as i32)) as i64) } else { None };
    let sc = scaled(&l.args, src.count);
    let scale = sc.scale;
    let layer = plane(l, 0)?;
    let mut sources = l.scratch.src;
    if l.scratch.kind == 1 {
        let roi = host.region(src);
        let p = pair.ok_or(Error::NullRead)?;
        let roi4 = Roi { e: 4, ..roi };
        let copy_a = plane(l, p)?;
        host.copy_plane(copy_a, l.scratch.src[0], src.selector, &roi4);
        let copy_b = plane(l, p + 1)?;
        host.copy_plane(copy_b, l.scratch.src[0], src.selector, &roi4);
        sources = [l.scratch.src[0], copy_a, copy_b];
    }
    let (a, b) = three_words(&l.args, &sc);
    let roi = host.region(src);
    let req = ThreeArgs { sources, blurred: three(l, first)?, layer, saved_index, scale, a, b, c: [0.0; 3], pass_scale: l.args.pass_scale, pass_gain: l.args.pass_gain, roi, flag };
    host.worker_three(&req);
    Ok(())
}

/// Stand-in labels of this port.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "CELLS: the three float cells are the constants SCALE_STRENGTH, SCALE_WIDTH and SCALE_GATE",
        "ROUTE AVX2-ONLY: the wide-vector body runs unconditionally",
        "HOST-SERVICES: the layout query, the region record constructor and the plane copy are host methods",
        "NOT-REBUILT: the mid-vector and scalar arms and the initialise-and-retry route",
    ]
}
