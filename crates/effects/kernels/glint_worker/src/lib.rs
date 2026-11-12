//! Glint worker (AVX2 route): the orchestration of the box-blur glint pass.
//!
//! The routine computes nothing on pixels itself. For three planes it asks the host to blur and fill, runs the bright
//! pass, and then, for each channel whose gate is above zero, up to four optional passes (horizontal, vertical and two
//! two-dimensional ones). Each pass turns a sigma into three box radii and a callback-argument record, and asks the host
//! to run a pixel operation over the plane. Every such request is a typed [`Call`] handed to a [`Host`] together with the
//! callback arguments at that moment; the host runs the pixel operations (the ported pass crates) and the services
//! (allocator, copies, abort checks). A request with the blur flag set is refused by name before any host call.
//!
//! Float steps go through `fx_common::nanop`; a fresh NaN is 0xffc00000.
use fx_common::nanop::NanOps;

pub const HALF_F: f32 = 0.5;
pub const HALF_D: f64 = 0.5;
pub const ONE_D: f64 = 1.0;
/// The double scale applied to the two-dimensional pass strengths (bit pattern 0x3ff553f7ced91687).
pub const K_D: f64 = f64::from_bits(0x3ff5_53f7_ced9_1687);

/// The 24 byte region record: four dwords and a qword.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Roi {
    pub w: i32,
    pub h: i32,
    pub c: i32,
    pub d: i32,
    pub e: i64,
}

/// Identity of one plane the host holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlaneId(pub u64);

/// Identity of one scratch block the host allocated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub u64);

/// The source record the worker reads (the dispatcher's source argument).
#[derive(Clone, Copy, Debug)]
pub struct SourceRecord {
    /// The record's word at +0x8 (region-shape selector). Only zero is served: the region is { width, height, width, d, e }; any other value is refused by name.
    pub selector: i32,
    /// Byte size of one plane copy.
    pub plane_bytes: i32,
    pub count: i32,
    pub width: i32,
    pub height: i32,
    /// Nonzero selects the diagonal-body route of the two-dimensional passes.
    pub diagonal_mode: i32,
}

/// The four optional passes of a channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    Horizontal,
    Vertical,
    MainDiagonal,
    AntiDiagonal,
}

/// Scratch storage a pass hands to the pixel operation: the row-ring block and the radii record, one pair per pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scratch {
    RowRing(Pass),
    Radii(Pass),
}

/// A pixel operation requested of the host, one per parallel start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassOp {
    Scale,
    Horizontal,
    Vertical,
    /// The diagonal body of the main or the anti diagonal pass (the diagonal-mode route).
    DiagonalBody(Pass),
    /// The copy followed by the diagonal box sum of the main or the anti diagonal pass.
    DiagonalBox(Pass),
}

/// An operand of the callback arguments: unset, a plane or a scratch record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    Unset,
    Plane(PlaneId),
    Scratch(Scratch),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abort {
    Check,
}

/// The callback arguments written for the parallel pixel operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackArgs {
    pub ring: Operand,
    pub dest: Operand,
    pub source: Operand,
    pub scale_bits: u32,
    pub element_count: u32,
    pub width: u32,
    pub height: u32,
    pub radii: Operand,
    pub scale_dest: Operand,
    pub scale_source: Operand,
    pub edge_mode: u32,
    pub shift: u32,
    pub pass_count: u32,
}

impl CallbackArgs {
    pub const EMPTY: CallbackArgs = CallbackArgs {
        ring: Operand::Unset,
        dest: Operand::Unset,
        source: Operand::Unset,
        scale_bits: 0,
        element_count: 0,
        width: 0,
        height: 0,
        radii: Operand::Unset,
        scale_dest: Operand::Unset,
        scale_source: Operand::Unset,
        edge_mode: 0,
        shift: 0,
        pass_count: 0,
    };
}

/// One request to the host, in call order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    Blur { blurred: PlaneId, source: Option<PlaneId>, blur_x: u32, roi: Roi, flagged: bool, saved: Option<PlaneId> },
    Fill { output: PlaneId, flag: i32, roi: Roi },
    BrightPass { flag: i32, roi: Roi },
    CheckAbort(Abort),
    Alloc { ring: Scratch, bytes: u64, slots: u32 },
    Free(BlockId),
    Parallel { op: PassOp, radii: Option<[i32; 3]> },
    CopyPlane { layer: PlaneId, plane: PlaneId, bytes: u64 },
}

/// Services the body needs from its caller.
pub trait Host {
    fn call(&mut self, c: &Call, args: &CallbackArgs);
    /// Scratch block allocation: the block (None when none is held).
    fn alloc(&mut self, ring: Scratch, bytes: u64, slots: u32) -> Option<BlockId>;
    fn abort(&mut self, which: Abort) -> i32;
}

/// Everything the worker takes.
#[derive(Clone, Debug)]
pub struct Input {
    pub source: SourceRecord,
    pub blur_x: u32,
    pub blurred: [PlaneId; 3],
    pub sources: [PlaneId; 3],
    pub outputs: [PlaneId; 3],
    pub layer: PlaneId,
    pub saved: Option<PlaneId>,
    pub a: [u32; 3],
    pub b: [u32; 3],
    pub c: [u32; 3],
    pub pass_scale: [u32; 4],
    pub pass_gain: [u32; 4],
    pub roi: Roi,
    pub flag: i32,
}

/// Mantissa field of a binary32.
const MANTISSA_MASK: u32 = (1u32 << 23) - 1;

fn f(b: u32) -> f32 {
    f32::from_bits(b)
}

/// cvttss2si: NaN or out of range gives i32::MIN.
pub fn cvtt32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
/// Quiet bit of a binary64 NaN.
const QUIET_D: u64 = 1 << 51;
/// The negative quiet NaN a multiply of two non-NaN operands produces.
const FRESH_NAN_D: u64 = 0xfff8_0000_0000_0000;

/// Double multiply with the silicon rule: the first NaN operand, quieted; when neither operand is NaN and the product is
/// NaN, the negative quiet class, which narrows to 0xffc00000 on every host.
fn mul_d(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        f64::from_bits(a.to_bits() | QUIET_D)
    } else if b.is_nan() {
        f64::from_bits(b.to_bits() | QUIET_D)
    } else {
        let p = a * b;
        if p.is_nan() {
            f64::from_bits(FRESH_NAN_D)
        } else {
            p
        }
    }
}
/// cvtsd2ss with the NaN payload kept (truncated) and quieted, a fresh NaN canonical.
fn narrow(x: f64) -> f32 {
    if x.is_nan() {
        f32::from_bits(0x7fc0_0000 | (((x.to_bits() >> 29) as u32) & MANTISSA_MASK) | if x.is_sign_negative() { 0x8000_0000 } else { 0 })
    } else {
        x as f32
    }
}
fn widen(x: f32) -> f64 {
    if x.is_nan() {
        let b = x.to_bits();
        f64::from_bits((((b >> 31) as u64) << 63) | 0x7ff8_0000_0000_0000 | (((b & MANTISSA_MASK) as u64) << 29))
    } else {
        x as f64
    }
}

fn mulhi(v: i32) -> i32 {
    ((v as i64 * 0x5555_5556i64) >> 32) as i32
}

/// The three box radii from v >= 0 and lim = (dim >> 1) - 1: a third (high multiply by 0x55555556) clamped, then half of the
/// remainder clamped, then the rest clamped.
pub fn radii(v: i32, lim: i32) -> [i32; 3] {
    let mut first = mulhi(v).wrapping_sub(v >> 31);
    if first >= lim {
        first = lim;
    }
    let rest = v.wrapping_sub(first);
    let mut second = rest.wrapping_add(((rest as u32) >> 31) as i32) >> 1;
    if second >= lim {
        second = lim;
    }
    let used = first.wrapping_add(second);
    let third = v.wrapping_sub(used);
    let third = if third < lim { third } else { lim };
    [first, second, third]
}

/// Radii of a dimension (all zero for a dimension of 1 or less).
fn tri(dim: i32, v: i32) -> [i32; 3] {
    if dim > 1 {
        radii(v.max(0), (dim >> 1) - 1)
    } else {
        [0, 0, 0]
    }
}

/// The refusals decided from a request's own words, before any host call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    BlurFlag,
    RegionSelector,
    DiagExtent,
    Shift,
}

pub const NAME_BLUR_FLAG: &str = "GLINT-BLUR-FLAG-REFUSED (glint: the flagged blur shape is outside the rendered route, so the frame is not rendered)";
pub const NAME_REGION_SELECTOR: &str = "GLINT-REGION-SELECTOR-REFUSED (glint: a nonzero region selector in the source record is outside the rendered route, so the frame is not rendered)";
pub const NAME_DIAG_EXTENT: &str = "GLINT-DIAG-EXTENT-REFUSED (glint: the diagonal passes at a frame width of 1, or an empty frame, take steps outside the diagonal row that no input determines, so the frame is not rendered)";
pub const NAME_SHIFT: &str = "GLINT-SHIFT-REFUSED (glint: a shift of 2^30 pixels or more, or a shift whose product is out of the integer range, is outside the box passes' range, so the frame is not rendered)";

impl Refusal {
    pub fn name(self) -> &'static str {
        match self {
            Refusal::BlurFlag => NAME_BLUR_FLAG,
            Refusal::RegionSelector => NAME_REGION_SELECTOR,
            Refusal::DiagExtent => NAME_DIAG_EXTENT,
            Refusal::Shift => NAME_SHIFT,
        }
    }
}

/// The shift predicate: the conversion is i32::MIN, or the magnitude exceeds 0x3fffffff.
pub fn shift_out_of_range(d: i32) -> bool {
    d == i32::MIN || d.unsigned_abs() > 0x3fff_ffff
}

/// The band count of the diagonal bodies for a region of width `w` and height `h`: ((w - 1) >> 1) + h - 2. The host's band guard
/// and the extent predicate both use this one function.
pub fn partition_count(w: i32, h: i32) -> i32 {
    ((w.wrapping_sub(1)) >> 1).wrapping_add(h).wrapping_sub(2)
}

/// A diagonal pass that runs when the region width is below 2, the height is below 1, or the partition count is negative. The 2x1
/// partition has a negative row count and is refused.
pub fn extent_refused(w: i32, h: i32) -> bool {
    w < 2 || h < 1 || partition_count(w, h) < 0
}

/// The strengths of the four optional passes of one gate (one text: the body and the predicate both use it).
pub struct Strengths {
    pub h: f32,
    pub v_scale: f32,
    pub v: f32,
    pub main: f32,
    pub anti: f32,
}

pub(crate) fn three_strengths(scale: &[u32; 4], gain: &[u32; 4], gate: f32, diagonal_mode: bool) -> Strengths {
    let h = f(scale[0]).fmul(f(gain[0])).fmul(gate);
    let half_or_one = if diagonal_mode { HALF_D } else { ONE_D };
    let v_scale = narrow(mul_d(widen(f(scale[1])), half_or_one));
    let v = gate.fmul(f(gain[1])).fmul(v_scale);
    let main = narrow(mul_d(mul_d(widen(f(scale[2])), K_D), widen(gate.fmul(f(gain[2])))));
    let anti = narrow(mul_d(mul_d(widen(f(scale[3])), K_D), widen(gate.fmul(f(gain[3])))));
    Strengths { h, v_scale, v, main, anti }
}

/// The words of a request the predicate reads.
pub struct Words {
    pub a: [u32; 3],
    pub b: [u32; 3],
    pub c: [u32; 3],
    pub pass_scale: [u32; 4],
    pub pass_gain: [u32; 4],
}

/// The one refusal predicate: decides the four named refusals from the request alone, in a fixed order (flag, selector, then the
/// passes), before any host call. `w` and `h` are the region's width and height.
pub fn refusal(source: &SourceRecord, flag: i32, w: i32, h: i32, words: &Words) -> Option<Refusal> {
    if flag != 0 {
        return Some(Refusal::BlurFlag);
    }
    if source.selector != 0 {
        return Some(Refusal::RegionSelector);
    }
    let diag = source.diagonal_mode != 0;
    let Words { a, b, c, pass_scale, pass_gain } = words;
    // the order of the deep build: per channel, the passes h, v, main, anti; per pass the shift guard (glint_wire host.rs
    // parallel, shift arm) and then, for a diagonal body, the extent guard (host.rs diagonal arm)
    for channel in 0..3usize {
        let gate = f(b[channel]);
        if !(gate > 0.0) {
            continue;
        }
        let (cc, aa) = (f(c[channel]), f(a[channel]));
        let st = three_strengths(pass_scale, pass_gain, gate, diag);
        if st.h > 0.0 {
            let sc = f(pass_scale[0]);
            let (ra, g) = (sc.fmul(aa), sc.fmul(cc));
            if (ra > 0.0 || g != 0.0 || g.is_nan()) && shift_out_of_range(cvtt32(g)) {
                return Some(Refusal::Shift);
            }
        }
        if st.v > 0.0 {
            let (ra, g) = (st.v_scale.fmul(aa), st.v_scale.fmul(cc));
            if (ra > 0.0 || g != 0.0 || g.is_nan()) && shift_out_of_range(cvtt32(g)) {
                return Some(Refusal::Shift);
            }
        }
        for (k, s) in [(2usize, st.main), (3, st.anti)] {
            if s > 0.0 {
                if shift_out_of_range(cvtt32(f(pass_scale[k]).fmul(cc))) {
                    return Some(Refusal::Shift);
                }
                if diag && extent_refused(w, h) {
                    return Some(Refusal::DiagExtent);
                }
            }
        }
    }
    None
}

/// The predicate over the three-plane worker's input.
pub fn refusal_three(i: &Input) -> Option<Refusal> {
    refusal(&i.source, i.flag, i.roi.w, i.roi.h, &Words { a: i.a, b: i.b, c: i.c, pass_scale: i.pass_scale, pass_gain: i.pass_gain })
}

struct Run<'a, H: Host> {
    i: &'a Input,
    h: &'a mut H,
    c: CallbackArgs,
}

impl<'a, H: Host> Run<'a, H> {
    fn emit(&mut self, call: Call) {
        self.h.call(&call, &self.c);
    }
    fn poll_check(&mut self) -> i32 {
        self.emit(Call::CheckAbort(Abort::Check));
        self.h.abort(Abort::Check)
    }
    fn poll_check_stop(&mut self) -> Result<(), ()> {
        if self.poll_check() != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
    fn free_if(&mut self, block: Option<BlockId>) {
        if let Some(b) = block {
            self.emit(Call::Free(b));
        }
    }
    /// Tail shared by the passes: the final scale of the plane by `v` when `v != 0` (a NaN counts as non-zero).
    fn tail(&mut self, channel: usize, v: f32) -> Result<(), ()> {
        self.poll_check_stop()?;
        let i = self.i;
        if v != 0.0 || v.is_nan() {
            self.c.scale_dest = Operand::Plane(i.outputs[channel]);
            self.c.scale_bits = v.to_bits();
            self.c.scale_source = Operand::Plane(i.layer);
            self.c.element_count = i.roi.c.wrapping_mul(i.roi.h) as u32;
            self.emit(Call::Parallel { op: PassOp::Scale, radii: None });
        }
        Ok(())
    }
    /// Horizontal and vertical passes: `scale` multiplies the A and C inputs, `v` is the pass strength.
    fn horizontal_vertical(&mut self, channel: usize, scale: f32, v: f32, vertical: bool) -> Result<(), ()> {
        self.poll_check_stop()?;
        let i = self.i;
        let plane = i.blurred[channel];
        let (w, h) = (i.roi.w, i.roi.h);
        let ra = scale.fmul(f(i.a[channel]));
        let gain = scale.fmul(f(i.c[channel]));
        let dim = if vertical { h } else { w };
        if ra > 0.0 || gain != 0.0 || gain.is_nan() {
            let r = tri(dim, cvtt32(ra.fadd(HALF_F)));
            let pass = if vertical { Pass::Vertical } else { Pass::Horizontal };
            let bytes = (dim as i64 as u64) << 2;
            let block = self.h.alloc(Scratch::RowRing(pass), bytes, 2);
            self.emit(Call::Alloc { ring: Scratch::RowRing(pass), bytes, slots: 2 });
            self.c.ring = Operand::Scratch(Scratch::RowRing(pass));
            self.c.dest = Operand::Plane(i.layer);
            self.c.source = Operand::Plane(plane);
            self.c.width = w as u32;
            self.c.height = h as u32;
            self.c.radii = Operand::Scratch(Scratch::Radii(pass));
            self.c.shift = cvtt32(gain) as u32;
            self.c.edge_mode = 0;
            self.c.pass_count = 3;
            self.emit(Call::Parallel { op: if vertical { PassOp::Vertical } else { PassOp::Horizontal }, radii: Some(r) });
            self.free_if(block);
        } else {
            self.emit(Call::CopyPlane { layer: i.layer, plane, bytes: ((w.wrapping_mul(h) as i64) << 2) as u64 });
        }
        self.tail(channel, v)
    }
    /// The main diagonal pass (`anti` false) and the anti diagonal pass (`anti` true): radii from the smaller dimension.
    fn diagonal(&mut self, channel: usize, anti: bool, v: f32) -> Result<(), ()> {
        let i = self.i;
        let plane = i.blurred[channel];
        let (w, h) = (i.roi.w, i.roi.h);
        let pass = if anti { Pass::AntiDiagonal } else { Pass::MainDiagonal };
        let scale = f(i.pass_scale[if anti { 3 } else { 2 }]);
        let ra = scale.fmul(f(i.a[channel]));
        let gain = scale.fmul(f(i.c[channel]));
        let mn = w.min(h);
        let mx = w.max(h);
        let r = tri(mn, cvtt32(ra.fadd(HALF_F)));
        self.c.dest = Operand::Plane(i.layer);
        self.c.height = h as u32;
        self.c.source = Operand::Plane(plane);
        self.c.width = w as u32;
        self.c.radii = Operand::Scratch(Scratch::Radii(pass));
        self.c.shift = cvtt32(gain) as u32;
        let diagonal_mode = i.source.diagonal_mode != 0;
        let bytes = (mx as i64 as u64) << 2;
        let slots = if diagonal_mode { 3 } else { 5 };
        let block = self.h.alloc(Scratch::RowRing(pass), bytes, slots);
        self.emit(Call::Alloc { ring: Scratch::RowRing(pass), bytes, slots });
        self.c.ring = Operand::Scratch(Scratch::RowRing(pass));
        if diagonal_mode {
            self.emit(Call::Parallel { op: PassOp::DiagonalBody(pass), radii: Some(r) });
        } else {
            self.emit(Call::CopyPlane { layer: i.layer, plane, bytes: i.source.plane_bytes as i64 as u64 });
            if self.poll_check() != 0 {
                self.free_if(block);
                return self.tail(channel, v);
            }
            self.emit(Call::Parallel { op: PassOp::DiagonalBox(pass), radii: Some(r) });
        }
        self.free_if(block);
        self.tail(channel, v)
    }
    fn body(&mut self) -> Result<(), ()> {
        let i = self.i;
        let flagged = i.flag != 0;
        for k in 0..3usize {
            self.emit(Call::Blur { blurred: i.blurred[k], source: Some(i.sources[k]), blur_x: i.blur_x, roi: i.roi, flagged, saved: if flagged { i.saved } else { None } });
            self.emit(Call::Fill { output: i.outputs[k], flag: i.flag, roi: i.roi });
        }
        self.emit(Call::BrightPass { flag: i.flag, roi: i.roi });
        for channel in 0..3usize {
            let gate = f(i.b[channel]);
            if !(gate > 0.0) {
                continue;
            }
            let st = three_strengths(&i.pass_scale, &i.pass_gain, gate, i.source.diagonal_mode != 0);
            if st.h > 0.0 {
                self.horizontal_vertical(channel, f(i.pass_scale[0]), st.h, false)?;
            }
            if st.v > 0.0 {
                self.horizontal_vertical(channel, st.v_scale, st.v, true)?;
            }
            if st.main > 0.0 {
                self.diagonal(channel, false, st.main)?;
            }
            if st.anti > 0.0 {
                self.diagonal(channel, true, st.anti)?;
            }
        }
        Ok(())
    }
}

/// Run the worker body against a host. A named refusal returns before the first host call.
pub fn run<H: Host>(inp: &Input, host: &mut H) -> Result<(), Refusal> {
    if let Some(r) = refusal_three(inp) {
        return Err(r);
    }
    let mut r = Run { i: inp, h: host, c: CallbackArgs::EMPTY };
    let _ = r.body();
    Ok(())
}

/// Labelled stand-ins.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "ROUTE AVX2-ONLY: the AVX2 body runs unconditionally; the feature-word dispatch and the AVX and SSE arms are not rebuilt",
        "HOST-HOOKED: parallel start, scratch allocator, abort checks and copies are host services",
        "HOST-FP-ASSUMED: round to nearest, no flush to zero",
    ]
}

#[cfg(test)]
mod nan_pick_tests {
    use super::*;

    #[test]
    fn fresh_nan_product_narrows_to_negative_quiet() {
        let p = mul_d(f64::INFINITY, 0.0);
        assert_eq!(narrow(p).to_bits(), 0xffc0_0000);
        let q = mul_d(0.0, f64::NEG_INFINITY);
        assert_eq!(narrow(q).to_bits(), 0xffc0_0000);
    }

    #[test]
    fn first_nan_operand_is_kept_and_quieted() {
        let a = f64::from_bits(0x7ff0_0000_0000_0001);
        assert_eq!(mul_d(a, f64::NAN).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(mul_d(2.0, a).to_bits(), 0x7ff8_0000_0000_0001);
    }
}
