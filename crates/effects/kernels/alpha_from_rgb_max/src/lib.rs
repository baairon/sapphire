//! alpha_from_rgb_max: CPU render of the builtin.
//!
//! The builtin writes up to four output channels from an input image object:
//! * mode != 4: output channel `i` (for `i` in `0..nout`) is a copy of input channel `min(i, nch-1)`.
//! * mode == 4: channels `0..ncopy` are copied the same way (`ncopy` = 3 when `nout > 2`, else 1), then channel 3 is
//!   `scale * max(max(c0, c1), c2)` over the freshly written channels. Special values of `scale`:
//!   `scale == 0` fills channel 3 with 0.0; for a single-channel input (`nch == 1`) `scale == 1` fills channel 3 with 1.0 and any other
//!   scale gives `scale * c2`; with `nch != 1` and `scale == 1` the product is skipped (plain maximum).
//! A constant input (no pixel planes, one value per channel) fills the copied channels with that value.
//!
//! Arithmetic follows the vector kernels: `max(a, b)` returns the second operand when either is NaN or both are equal (Intel order),
//! and the product follows the silicon NaN rule (first NaN operand, quieted; see `imul`) with a fresh NaN canonicalized to 0xffc00000.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The input has no channel.
    NoChannels,
    /// Fewer output planes than the mode needs.
    OutputPlanes,
    /// A plane is shorter than the pixel count.
    ShortPlane,
    /// A constant input has fewer values than channels.
    ShortConst,
}

/// Input image object of the builtin.
pub enum Source<'a> {
    /// One float plane per channel.
    Planes(&'a [&'a [f32]]),
    /// One value per channel. `is_int` selects 32-bit integer values converted to float (round to nearest even).
    Const { bits: &'a [u32], is_int: bool },
}

#[derive(Clone, Debug)]
pub struct Params {
    /// Mode word of the builtin (4 selects the alpha derivation).
    pub mode: i32,
    /// Input channel count.
    pub nch: i32,
    /// Scale factor.
    pub scale: f32,
    /// Output channel count reported by the object.
    pub nout: i32,
    /// Pixels per plane.
    pub n: usize,
}

/// Stand-ins and contract limits of this port, one label per line.
pub fn assumptions() -> Vec<&'static str> {
    vec![
        "AFRM-A1 input descriptor channel count equals the object channel count (the original clamps to the descriptor count as well)",
        "AFRM-A2 dense region: no region of interest record (the fill and copy cover exactly n floats from the plane start)",
        "AFRM-A3 vector path alignment and width have no effect on results (all variants are element-wise with the same operand order)",
        "AFRM-A4 nout in 1..=4 and, in mode 4, four output planes; output channel 2 keeps its previous content when nout <= 2 and nch == 1 reads it",
        "AFRM-A5 single tile: the fill and copy host services run as one serial pass",
    ]
}

/// Intel `maxss`/`maxps` operand order: NaN or equal returns the second operand.
#[inline]
pub fn imax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn quiet(x: f32) -> f32 {
    f32::from_bits(x.to_bits() | 0x0040_0000)
}

/// `mulss`/`mulps` with the silicon NaN rule: when either operand is NaN the first NaN operand is returned, quieted (for two NaN
/// operands that is the first operand). A fresh NaN (no NaN operand) is 0xffc00000.
#[inline]
pub fn imul(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        return quiet(if a.is_nan() { a } else { b });
    }
    let r = a * b;
    if r.is_nan() {
        f32::from_bits(0xffc0_0000)
    } else {
        r
    }
}

fn channel_value<'a>(src: &'a Source<'a>, ch: usize, n: usize) -> Result<ChanSrc<'a>, Error> {
    match src {
        Source::Planes(p) => {
            let pl = p.get(ch).ok_or(Error::NoChannels)?;
            if pl.len() < n {
                return Err(Error::ShortPlane);
            }
            Ok(ChanSrc::Plane(pl))
        }
        Source::Const { bits, is_int } => {
            let b = *bits.get(ch).ok_or(Error::ShortConst)?;
            Ok(ChanSrc::Value(if *is_int { (b as i32) as f32 } else { f32::from_bits(b) }))
        }
    }
}

enum ChanSrc<'a> {
    Plane(&'a [f32]),
    Value(f32),
}

fn copy_channel(src: &Source, nch: i32, i: i32, n: usize, dst: &mut [f32]) -> Result<(), Error> {
    // the caller clamps to nch - 1; the helper clamps to the descriptor count (equal, AFRM-A1)
    let ch = i.min(nch - 1).max(-1);
    if ch < 0 {
        return Err(Error::NoChannels);
    }
    match channel_value(src, ch as usize, n)? {
        ChanSrc::Plane(p) => dst[..n].copy_from_slice(&p[..n]),
        ChanSrc::Value(v) => dst[..n].iter_mut().for_each(|d| *d = v),
    }
    Ok(())
}

/// Runs the builtin. `out` holds the output planes (each at least `p.n` floats); planes not written keep their content.
pub fn run(p: &Params, src: &Source, out: &mut [&mut [f32]]) -> Result<(), Error> {
    run_traced(p, src, out, None)
}

/// A product whose two operands are both NaN: output plane 3 element index, first operand, second operand, result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NanPair {
    pub index: usize,
    pub a: u32,
    pub b: u32,
    pub result: u32,
}

/// `run` that also records every product with two NaN operands.
pub fn run_traced(p: &Params, src: &Source, out: &mut [&mut [f32]], mut trace: Option<&mut Vec<NanPair>>) -> Result<(), Error> {
    let mut rec = |index: usize, a: f32, b: f32| {
        if let Some(t) = trace.as_deref_mut() {
            if a.is_nan() && b.is_nan() {
                t.push(NanPair { index, a: a.to_bits(), b: b.to_bits(), result: imul(a, b).to_bits() });
            }
        }
    };
    let n = p.n;
    if p.nch < 1 {
        return Err(Error::NoChannels);
    }
    if out.iter().any(|o| o.len() < n) {
        return Err(Error::ShortPlane);
    }
    if p.mode != 4 {
        if (p.nout.max(0) as usize) > out.len() {
            return Err(Error::OutputPlanes);
        }
        for i in 0..p.nout.max(0) {
            copy_channel(src, p.nch, i, n, out[i as usize])?;
        }
        return Ok(());
    }
    if out.len() < 4 {
        return Err(Error::OutputPlanes);
    }
    let big = p.nout > 2;
    let ncopy = if big { 3 } else { 1 };
    for i in 0..ncopy {
        copy_channel(src, p.nch, i, n, out[i as usize])?;
    }
    let s = p.scale;
    if s == 0.0 {
        out[3][..n].iter_mut().for_each(|d| *d = 0.0);
        return Ok(());
    }
    let (a, b) = out.split_at_mut(3);
    let (o3, a) = (&mut b[0][..n], &*a);
    if p.nch == 1 {
        if s == 1.0 {
            o3.iter_mut().for_each(|d| *d = 1.0);
        } else {
            let c2 = &a[2][..n];
            for (i, (d, &x)) in o3.iter_mut().zip(c2).enumerate() {
                rec(i, s, x);
                *d = imul(s, x);
            }
        }
        return Ok(());
    }
    let (c0, c1, c2) = (&a[0][..n], &a[1][..n], &a[2][..n]);
    if s == 1.0 {
        for i in 0..n {
            o3[i] = imax(imax(c0[i], c1[i]), c2[i]);
        }
    } else {
        for i in 0..n {
            let m = imax(imax(c0[i], c1[i]), c2[i]);
            rec(i, s, m);
            o3[i] = imul(s, m);
        }
    }
    Ok(())
}
