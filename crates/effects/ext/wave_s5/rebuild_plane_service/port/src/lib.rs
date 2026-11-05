//! plane_service: native port of the host plane service (listing offsets, VA = offset for data).
//!
//! (this = rdi, mode = esi, chan = edx, dst = rcx, roi = r8) picks a parameter block, then either
//!   * fills `dst` with the block constant (builds the descriptor, AVX2 route fills), or
//!   * copies the block's source plane into `dst` (flat: memcpy of n*4 bytes; rect: +).
//! Rect fill (roi.mode != 0) and rect copy are NOT REBUILT: the port returns the call they make
//! (value, descriptor words) as an `Action` and leaves `dst` untouched (label REC for that half).
//! Std only, no unsafe. Every host state the original reads is an explicit input (see `assumptions()`).

pub mod assume;
pub use assume::{assumptions, Assumption};

use std::sync::atomic::{AtomicU32, Ordering};

static MUT: AtomicU32 = AtomicU32::new(0);

/// Select a mutant (feature `mutants` only; 0 = none). Used by the T2 mutation run, never by the port itself.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}

#[inline]
fn m(id: u32) -> bool {
    cfg!(feature = "mutants") && MUT.load(Ordering::Relaxed) == id
}

/// ROI / info record handed in r8: the mode field (0 flat, else rect), count, raw 32 bit words (rect only).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Roi {
    pub mode: i32,
    pub n: i32,
    pub port_blk_a: u32,
    pub port_blk_b: u32,
    pub port_blk_c: u32,
}

/// One 0x60 byte parameter block. `values` is the array behind (raw 32 bit words, ints or floats), `is_int` the byte at,
/// `enable` the byte behind, `maxch` the int behind, `img_valid` the byte at of the image object behind,
/// `planes` the channel planes of that image object (one pointer per channel c).
#[derive(Clone, Copy, Debug)]
pub struct Block<'a> {
    pub is_int: bool,
    pub values: &'a [u32],
    pub enable: bool,
    pub maxch: i32,
    pub img_valid: bool,
    pub planes: &'a [&'a [f32]],
}

/// How the block is found. The original compares `[this] - 0x10` with the captured cell (=, the symbol
/// of the 4 slot vtable): equal`blocks[mode]` (this array, a fixed stride); else the class virtual
/// `[vtable](this, mode)` returns the block (CALLEE-MODELLED: the caller supplies the block it returns).
#[derive(Clone, Copy, Debug)]
pub enum Sel<'a> {
    Direct { blocks: &'a [Block<'a>] },
    Virtual { block: Block<'a> },
}

/// Host state read by the fill and copy wrappers: the pool flag behind cells (captured), nonzero inside a pool worker.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Host {
    pub in_pool: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// flat fill with n <= 0 elements: nothing written
    FillNone,
    /// inline AVX2 fill of `count` elements with `value` bits
    FillInline { value: u32, count: usize },
    /// pooled fill (n >= 0x200 and not in a pool worker), value == 0.0 (also -0.0): memset 0
    FillPooledZero { count: usize },
    /// pooled fill, any other value (NaN included): fills `value` bits
    FillPooledValue { value: u32, count: usize },
    /// rect fill: call (dst, value, desc on the stack) NOT REBUILT, dst untouched
    FillRect { value: u32, desc: [u64; 3] },
    /// flat copy of `count` elements (memcpy)
    CopyFlat { count: usize },
    /// rect copy: assert first when the image object byte is 0, then (dst, src plane, 1, desc[2], img, desc on stack) NOT REBUILT
    CopyRect { chan: usize, desc: [u64; 3], assert_called: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlError {
    ModeOutOfRange,
    NegativeChannel,
    ValuesShort,
    PlaneMissing,
    DstShort,
    SrcShort,
    NegativeCount,
}

/// 24 byte descriptor from the ROI. Constants CAPTURED: = 0x0000000100000000, = 0,
///  = 4, lane 1 = 0, = 4.
pub fn descriptor(roi: &Roi) -> [u64; 3] {
    if roi.mode == 0 {
        let hi0: u64 = if m(15) { 2 } else { 1 };
        [(hi0 << 32) | roi.n as u32 as u64, (0u64 << 32) | roi.n as u32 as u64, 4]
    } else if m(16) {
        [roi.port_blk_b as u64 | (roi.port_blk_a as u64) << 32, roi.port_blk_c as u64, 4]
    } else {
        [roi.port_blk_a as u64 | (roi.port_blk_b as u64) << 32, roi.port_blk_c as u64 | 0u64 << 32, 4]
    }
}

/// The service. `chan` is the signed channel (edx), clamped to the block's `maxch` (cmovl).
pub fn plane_service(host: &Host, sel: Sel, mode: i32, chan: i32, dst: &mut [f32], roi: &Roi) -> Result<Action, PlError> {
    let blk: Block = match sel {
        Sel::Direct { blocks } => {
            let idx = if m(11) { mode + 1 } else { mode };
            if idx < 0 {
                return Err(PlError::ModeOutOfRange);
            }
            *blocks.get(idx as usize).ok_or(PlError::ModeOutOfRange)?
        }
        Sel::Virtual { block } => block,
    };
    // r12d = min(chan, *maxch) (cmp edx,r12d; cmovl r12d,edx)
    let ch = if m(1) {
        chan.max(blk.maxch)
    } else if m(2) {
        chan
    } else if blk.maxch < chan {
        blk.maxch
    } else {
        chan
    };
    if ch < 0 {
        return Err(PlError::NegativeChannel);
    }
    let ch = ch as usize;
    let enable = if m(4) { !blk.enable } else { blk.enable };
    if enable {
        let raw = *blk.values.get(ch).ok_or(PlError::ValuesShort)?;
        // cvtsi2ss when the byte at is set, plain movss otherwise
        let value: u32 = if blk.is_int && !m(3) { (raw as i32 as f32).to_bits() } else { raw };
        let desc = descriptor(roi);
        let rect = if m(5) { roi.mode > 0 } else { roi.mode != 0 };
        if rect {
            return Ok(Action::FillRect { value, desc });
        }
        // AVX2 body : n = desc[1].lo * desc[0].hi (imul eax), test eax,eax; jle -> done
        let n = (desc[1] as u32 as i32).wrapping_mul((desc[0] >> 32) as u32 as i32);
        if n <= 0 {
            return Ok(Action::FillNone);
        }
        let mut count = n as usize;
        if m(10) {
            count -= 1;
        }
        if dst.len() < count {
            return Err(PlError::DstShort);
        }
        let big = if m(6) { n > 0x200 } else { n >= 0x200 };
        let pooled = if m(8) { host.in_pool && big } else { !host.in_pool && big };
        if pooled {
            let zero = if m(7) {
                value == 0
            } else if m(14) {
                f32::from_bits(value) == 0.0 || f32::from_bits(value).is_nan()
            } else {
                f32::from_bits(value) == 0.0
            };
            let bits = if zero { 0 } else { value };
            for d in dst[..count].iter_mut() {
                *d = f32::from_bits(bits);
            }
            return Ok(if zero { Action::FillPooledZero { count } } else { Action::FillPooledValue { value, count } });
        }
        for d in dst[..count].iter_mut() {
            *d = f32::from_bits(value);
        }
        return Ok(Action::FillInline { value, count });
    }
    // copy path
    let src_plane = *blk.planes.get(ch).ok_or(PlError::PlaneMissing)?;
    if roi.mode != 0 {
        let assert_called = !blk.img_valid && !m(17);
        return Ok(Action::CopyRect { chan: ch, desc: descriptor(roi), assert_called });
    }
    if roi.n < 0 {
        return Err(PlError::NegativeCount);
    }
    // r14 = n << 2 (64 bit), rdx = low 32 bits sign extended
    let size = ((roi.n as i64) << 2) as i32 as i64;
    if size < 0 {
        return Err(PlError::NegativeCount);
    }
    let mut count = (size / 4) as usize;
    if m(9) && count > 0 {
        count -= 1;
    }
    if m(13) {
        // mutant: wrong source plane
        let other = blk.planes.get(0).copied().unwrap_or(src_plane);
        if other.len() < count {
            return Err(PlError::SrcShort);
        }
        if dst.len() < count {
            return Err(PlError::DstShort);
        }
        dst[..count].copy_from_slice(&other[..count]);
        return Ok(Action::CopyFlat { count });
    }
    if src_plane.len() < count {
        return Err(PlError::SrcShort);
    }
    if dst.len() < count {
        return Err(PlError::DstShort);
    }
    dst[..count].copy_from_slice(&src_plane[..count]);
    Ok(Action::CopyFlat { count })
}
