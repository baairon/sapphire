//! copy_alpha: the render entry (virtual slot 52, flag 0). Colour channels come from the first input,
//! the alpha channel from the second input. Every output channel is one plane of `n` floats.
//!
//! Each channel is taken from an input node by one of two routes:
//! * the node is a scalar node (a flag byte is set): the output plane is filled with one value; the value is read
//!   as a 32-bit integer and converted to f32 (round to nearest even) when the node's integer byte is set,
//!   otherwise its bits are used as they are;
//! * the node holds planes: the output plane is a copy of the plane of that channel.
//!
//! The channel index is clamped to the node's last channel (`last`) before the lookup. A scalar fill of
//! [`POOL_MIN`] elements or more is handed to the worker pool when the pool is idle; the pool worker clears the
//! range with zero bytes when the value compares equal to zero, so -0.0 is stored as +0.0 on that route.
//! The entry works on the dense element range only (see [`assumptions`]).

/// One input node of the render entry.
#[derive(Clone, Debug)]
pub enum Source<'a> {
    /// Scalar node: `values[ch]` is a 32-bit word; `is_int` selects integer-to-float conversion.
    Scalar { values: &'a [u32], is_int: bool, last: i32 },
    /// Plane node: `planes[ch]` holds at least `n` floats.
    Planes { planes: &'a [&'a [f32]], last: i32 },
}

/// Entry parameters.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Channel mode of the operator (the alpha path runs when it is 2 or 4).
    pub mode: i32,
    /// Output channel count.
    pub nchan: i32,
    /// Elements per plane.
    pub n: usize,
    /// True when the worker pool is idle (the pool state word is zero). Scalar fills of [`POOL_MIN`] elements or more
    /// then run through the pool, whose worker stores +0.0 for any value that compares equal to zero.
    pub pool_idle: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params { mode: 0, nchan: 0, n: 0, pool_idle: true }
    }
}

/// Fill count from which a scalar fill is handed to the pool.
pub const POOL_MIN: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Fewer output planes than channels, or an output plane shorter than `n`.
    OutputShape,
    /// The alpha path needs at least one output channel.
    NoChannels,
    /// A channel index (after clamping) is negative or outside the node's data.
    ChannelRange,
    /// A plane shorter than `n`.
    PlaneShort,
}

/// Stand-ins of this port, one label per line.
pub fn assumptions() -> Vec<&'static str> {
    vec![
        "CA-A1 DENSE-ROI: the element-range record is dense (record word +8 is 0); the ranged copy and ranged fill routes are not rebuilt",
        "CA-A2 FILL-COUNT: the scalar fill writes n elements; the factor read from the constant record cell is taken as 1",
        "CA-A3 POOL-ROUTE: fills of 512 elements or more run through the pool when its state word is zero; the pool is modelled as one call of the worker over the whole range",
        "CA-A4 MEMCPY-HOST: the plane copy is the host copy routine (dense, n*4 bytes)",
        "CA-A5 NODE-FAST-PATH: the input nodes come from the direct node table, not from the virtual accessor",
        "CA-A6 LAST-CHANNEL: the clamp bound is the pointee of the node's channel-count cell, used as the largest valid channel index",
    ]
}

fn int_to_f32(bits: u32) -> f32 {
    (bits as i32) as f32
}

fn take_channel(src: &Source, ch: i32, out: &mut [f32], n: usize, pool_idle: bool) -> Result<(), Error> {
    let last = match src {
        Source::Scalar { last, .. } | Source::Planes { last, .. } => *last,
    };
    let c = if last < ch { last } else { ch };
    if c < 0 {
        return Err(Error::ChannelRange);
    }
    let c = c as usize;
    match src {
        Source::Scalar { values, is_int, .. } => {
            let w = *values.get(c).ok_or(Error::ChannelRange)?;
            let mut v = if *is_int { int_to_f32(w) } else { f32::from_bits(w) };
            if pool_idle && n >= POOL_MIN && v == 0.0 {
                v = 0.0; // the pool worker clears the range with zero bytes
            }
            for o in out.iter_mut().take(n) {
                *o = v;
            }
        }
        Source::Planes { planes, .. } => {
            let p = planes.get(c).ok_or(Error::ChannelRange)?;
            if p.len() < n {
                return Err(Error::PlaneShort);
            }
            out[..n].copy_from_slice(&p[..n]);
        }
    }
    Ok(())
}

/// Run the entry: `out[i]` for the colour channels from `a`, the alpha output channel from `b`.
pub fn copy_alpha(p: &Params, a: &Source, b: &Source, out: &mut [&mut [f32]]) -> Result<(), Error> {
    let flag = p.mode == 2 || p.mode == 4;
    if p.nchan < 0 || out.len() < p.nchan as usize || out.iter().take(p.nchan as usize).any(|o| o.len() < p.n) {
        return Err(Error::OutputShape);
    }
    if flag && p.nchan < 1 {
        return Err(Error::NoChannels);
    }
    let cnt = if flag { p.nchan - 1 } else { p.nchan };
    for i in 0..cnt {
        take_channel(a, i, &mut *out[i as usize], p.n, p.pool_idle)?;
    }
    if flag {
        take_channel(b, p.mode - 1, &mut *out[(p.nchan - 1) as usize], p.n, p.pool_idle)?;
    }
    Ok(())
}
