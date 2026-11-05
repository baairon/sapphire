//! Class `dissolve`: native port of the AVX body (reached from the slot 53 middle entry when the
//! feature word has all bits of 0x9d97ff). Addresses are listing offsets unless written as VA (= listing).
//!
//! The body is a clamped lerp per channel, `out = if t <= 0 { A } else if t >= 1 { B } else { fma(t, B - A, A) }`
//! (NaN t takes the lerp branch), with each of the three operands A, B, T either a per-channel scalar or a plane,
//! plus the fills and plain copies that the scalar-T modes reduce to. Operand roles (MEASURED):
//! A = object flag, scalars, plane array; B = flag, scalars, array;
//! T (the matte) = flag, scalars, array. Flag nonzero means scalar, zero means plane.
//! Std only, no unsafe.
use fx_common::plane::{FxError, Plane, PlaneMut};

pub mod consts;
pub use consts::assumptions;

/// Most channels the body can address with scalars: the scalar block of an operand holds four floats before the
/// plane-array pointer (A:, then). More channels would read the pointer as a float.
pub const MAX_CHANNELS: usize = 4;

/// One operand of the lerp.
#[derive(Clone, Copy)]
pub enum Operand<'a> {
    /// Per-channel scalars (flag byte nonzero). Channel c reads element c; object offset of element 0 is (A), (B), (T).
    Scalar([f32; 4]),
    /// One plane per object plane-array slot (flag byte zero); channel c reads slot `min(c, count - 1)` (or the rules below).
    Planes(&'a [Plane<'a>]),
}

/// Object fields the body reads (explicit, no Default). Offsets are object offsets.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// ROI record (signed dword in the original): floats written per channel per call. One call is one row; the API
    /// repeats it for every row of the planes (ASSUMPTION DS-A3).
    pub len: usize,
    /// this: plane count of A (also drives the alpha and 2-plane rules).
    pub a_count: i32,
    /// this: plane count of B.
    pub b_count: i32,
    /// this: plane count of T.
    pub t_count: i32,
    /// this (A flag): true = A is scalar. Used by `renderrender_ops` takes the variant of its `Operand`.
    pub a_scalar: bool,
    /// this (B flag).
    pub b_scalar: bool,
    /// this (T flag).
    pub t_scalar: bool,
    /// this (A scalars, channels 0 to 3). Used by `render`.
    pub a_values: [f32; 4],
    /// this (B scalars).
    pub b_values: [f32; 4],
    /// this (T scalars).
    pub t_values: [f32; 4],
}

/// The per-element rule of every mode (MEASURED, vector core and scalar loops such as).
/// `t <= 0` and `t >= 1` are ordered compares, so NaN falls to the lerp. The fused multiply add is `fma(t, B - A, A)`.
#[inline]
pub fn blend(t: f32, a: f32, b: f32) -> f32 {
    if t <= 0.0 {
        a
    } else if t >= 1.0 {
        b
    } else {
        if cfg!(feature = "mutant_nofma") {
            t * (b - a) + a
        } else {
            t.mul_add(b - a, a)
        }
    }
}

/// Plane index a channel reads for A or B: (A) and (B).
/// `two_plane_rule` (only when the channel count is above 2 and the operand count is exactly 2): max(c - 2, 0); else min(c, count - 1).
fn plane_index(c: usize, count: i32, two_plane_rule: bool) -> usize {
    if two_plane_rule {
        c.saturating_sub(2)
    } else {
        (c as i32).min(count - 1) as usize
    }
}

/// Listing : the fourth channel (r12d == 4) forces A (resp. B) to the scalar 1.0 with the flag set when
/// (count - 1) & 1 == 0, i.e. when the operand has an odd count.
fn alpha_forced(count: i32) -> bool {
    (count - 1) & 1 == 0
}

fn row_slice<'a>(p: &Plane<'a>, y: usize, len: usize) -> Result<&'a [f32], FxError> {
    if p.width < len {
        return Err(FxError::Geometry);
    }
    let start = y * p.stride;
    p.data.get(start..start + len).ok_or(FxError::Stride)
}

fn check_plane(p: &Plane, w: usize, h: usize) -> Result<(), FxError> {
    if p.width != w || p.height != h {
        return Err(FxError::Geometry);
    }
    if p.stride < p.width || (h > 0 && p.data.len() < (h - 1) * p.stride + p.width) {
        return Err(FxError::Stride);
    }
    Ok(())
}

/// What a channel reads for A or B: a scalar value or a plane row.
enum Src<'a> {
    Val(f32),
    Row(&'a [f32]),
}

impl Src<'_> {
    #[inline]
    fn at(&self, i: usize) -> f32 {
        match self {
            Src::Val(v) => *v,
            Src::Row(r) => r[i],
        }
    }
}

fn pick<'a>(op: &Operand<'a>, c: usize, count: i32, rule: bool, y: usize, len: usize) -> Result<Src<'a>, FxError> {
    if c == 3 && alpha_forced(count) {
        return Ok(Src::Val(1.0));
    }
    match op {
        Operand::Scalar(v) => Ok(Src::Val(v[c])),
        Operand::Planes(ps) => Ok(Src::Row(row_slice(&ps[plane_index(c, count, rule)], y, len)?)),
    }
}

/// Channel count of the original (vtable offset 0xf8, slot 31, W29) is `dst.len()` here. Errors: `PlaneCount` for no
/// destination, more than `MAX_CHANNELS`, a plane operand with a count below 1 or above the planes given; `Geometry`
/// and `Stride` as in S3 (every plane has the destination geometry; `p.len` may not exceed the width).
///
/// A, B, T are the operands (which one is which is decided by the object offsets in the module header). The flags and
/// scalars in `p` are ignored here; the variants of the operands play their role. Not modelled (ASSUMPTION DS-A2): the
/// original writes the flag bytes = 1 into the object when it reaches channel 4, so a second call on the
/// same object differs; `flags_after_call` reports the written state.
pub fn render_ops(p: &Params, a: Operand, b: Operand, t: Operand, dst: &mut [PlaneMut]) -> Result<(), FxError> {
    let nch = dst.len();
    if nch == 0 || nch > MAX_CHANNELS {
        return Err(FxError::PlaneCount);
    }
    let (w, h) = (dst[0].width, dst[0].height);
    if dst.iter().any(|d| d.width != w || d.height != h) {
        return Err(FxError::Geometry);
    }
    for d in dst.iter() {
        if d.stride < d.width || (h > 0 && d.data.len() < (h - 1) * d.stride + d.width) {
            return Err(FxError::Stride);
        }
    }
    if p.len > w {
        return Err(FxError::Geometry);
    }
    for (op, count) in [(&a, p.a_count), (&b, p.b_count), (&t, p.t_count)] {
        if let Operand::Planes(ps) = op {
            if count < 1 || (count as usize) > ps.len() {
                return Err(FxError::PlaneCount);
            }
            for pl in ps.iter().take(count as usize) {
                check_plane(pl, w, h)?;
            }
        }
    }
    let len = p.len;
    if len == 0 {
        return Ok(());
    }
    // the two-plane rule exists only when the channel count is above 2
    let rule_a = nch > 2 && p.a_count == 2;
    let rule_b = nch > 2 && p.b_count == 2;
    for y in 0..h {
        for (c, d) in dst.iter_mut().enumerate() {
            let out = &mut d.data[y * d.stride..y * d.stride + len];
            let av = pick(&a, c, p.a_count, rule_a, y, len)?;
            let bv = pick(&b, c, p.b_count, rule_b, y, len)?;
            match &t {
                Operand::Scalar(v) => {
                    let ts = v[c];
                    for (i, o) in out.iter_mut().enumerate() {
                        *o = blend(ts, av.at(i), bv.at(i));
                    }
                }
                Operand::Planes(ps) => {
                    let t_row = row_slice(&ps[(c as i32).min(p.t_count - 1) as usize], y, len)?;
                    for (i, o) in out.iter_mut().enumerate() {
                        *o = blend(t_row[i], av.at(i), bv.at(i));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Object flag bytes after one call with `nch` channels (write 1 when the
/// operand count is odd and the fourth channel is processed). Input: the flags before the call.
pub fn flags_after_call(p: &Params, nch: usize) -> (bool, bool) {
    if nch >= 4 {
        (p.a_scalar || alpha_forced(p.a_count), p.b_scalar || alpha_forced(p.b_count))
    } else {
        (p.a_scalar, p.b_scalar)
    }
}

/// S3 wrapper. `src` = A planes (`a_count` of them unless `a_scalar`), then B planes, then T planes, in that order;
/// scalar operands take their values from `p`.
pub fn render(p: &Params, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    let need = |scalar: bool, count: i32| -> Result<usize, FxError> {
        if scalar {
            Ok(0)
        } else if count < 1 {
            Err(FxError::PlaneCount)
        } else {
            Ok(count as usize)
        }
    };
    let (na, nb, nt) = (need(p.a_scalar, p.a_count)?, need(p.b_scalar, p.b_count)?, need(p.t_scalar, p.t_count)?);
    if src.len() != na + nb + nt {
        return Err(FxError::PlaneCount);
    }
    let (pa, rest) = src.split_at(na);
    let (pb, pt) = rest.split_at(nb);
    let a = if p.a_scalar { Operand::Scalar(p.a_values) } else { Operand::Planes(pa) };
    let b = if p.b_scalar { Operand::Scalar(p.b_values) } else { Operand::Planes(pb) };
    let t = if p.t_scalar { Operand::Scalar(p.t_values) } else { Operand::Planes(pt) };
    render_ops(p, a, b, t, dst)
}
