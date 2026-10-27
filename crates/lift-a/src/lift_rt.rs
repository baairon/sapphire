//! Runtime support for machine-generated ports (scripts\lift.py): a local model of the function's own
//! stack frame (same addresses as the oracle) plus x86 floating point conversion helpers.
//! Memory outside the frame goes through `neutral_world::Env` (P2v) or `crate::Mem` (P1).
use crate::Mem;
use neutral_world::Env;

/// Stack pointer the oracle gives a function at its first instruction (return address slot at rsp).
pub const RSP_ENTRY: u64 = 0x7fff_eff8;
const FRAME_LO: u64 = 0x7fff_f000 - 0x7000;
const FRAME_LEN: usize = 0x8000;

pub struct Stack {
    b: Vec<u8>,
}

impl Stack {
    pub fn new() -> Stack {
        let mut s = Stack { b: vec![0u8; FRAME_LEN] };
        s.put(RSP_ENTRY, 8, 0x0dea_d000);
        s
    }
    /// Stack argument slot k (at [rsp+8+8k] on entry).
    pub fn put_arg(&mut self, slot: u64, v: u64) {
        self.put(RSP_ENTRY + 8 + 8 * slot, 8, v);
    }
    #[inline]
    fn has(&self, a: u64, w: u32) -> bool {
        a >= FRAME_LO && a + (w as u64) <= FRAME_LO + FRAME_LEN as u64
    }
    fn put(&mut self, a: u64, w: u32, v: u64) {
        let o = (a - FRAME_LO) as usize;
        self.b[o..o + w as usize].copy_from_slice(&v.to_le_bytes()[..w as usize]);
    }
    fn get(&self, a: u64, w: u32) -> u64 {
        let o = (a - FRAME_LO) as usize;
        let mut t = [0u8; 8];
        t[..w as usize].copy_from_slice(&self.b[o..o + w as usize]);
        u64::from_le_bytes(t)
    }
}

#[inline]
pub fn ld<E: Env>(env: &mut E, s: &Stack, a: u64, w: u32) -> u64 {
    if s.has(a, w) {
        return s.get(a, w);
    }
    match w {
        1 => env.r8(a) as u64,
        2 => env.r16(a) as u64,
        4 => env.r32(a) as u64,
        _ => env.r64(a),
    }
}

#[inline]
pub fn st<E: Env>(env: &mut E, s: &mut Stack, a: u64, w: u32, v: u64) {
    if s.has(a, w) {
        s.put(a, w, v);
        return;
    }
    match w {
        1 => env.w8(a, v as u8),
        2 => env.w16(a, v as u16),
        4 => env.w32(a, v as u32),
        _ => env.w64(a, v),
    }
}

pub fn ld128<E: Env>(env: &mut E, s: &Stack, a: u64) -> u128 {
    let lo = ld(env, s, a, 8) as u128;
    let hi = ld(env, s, a.wrapping_add(8), 8) as u128;
    lo | (hi << 64)
}

pub fn st128<E: Env>(env: &mut E, s: &mut Stack, a: u64, v: u128) {
    st(env, s, a, 8, v as u64);
    st(env, s, a.wrapping_add(8), 8, (v >> 64) as u64);
}

#[inline]
pub fn ldm(m: &mut dyn Mem, s: &Stack, a: u64, w: u32) -> u64 {
    if s.has(a, w) {
        return s.get(a, w);
    }
    match w {
        1 => m.rd8(a) as u64,
        2 => m.rd16(a) as u64,
        4 => m.rd32(a) as u64,
        _ => m.rd64(a),
    }
}

#[inline]
pub fn stm(m: &mut dyn Mem, s: &mut Stack, a: u64, w: u32, v: u64) {
    if s.has(a, w) {
        s.put(a, w, v);
        return;
    }
    match w {
        1 => m.wr8(a, v as u8),
        2 => m.wr16(a, v as u16),
        4 => m.wr32(a, v as u32),
        _ => m.wr64(a, v),
    }
}

pub fn ld128m(m: &mut dyn Mem, s: &Stack, a: u64) -> u128 {
    let lo = ldm(m, s, a, 8) as u128;
    let hi = ldm(m, s, a.wrapping_add(8), 8) as u128;
    lo | (hi << 64)
}

pub fn st128m(m: &mut dyn Mem, s: &mut Stack, a: u64, v: u128) {
    stm(m, s, a, 8, v as u64);
    stm(m, s, a.wrapping_add(8), 8, (v >> 64) as u64);
}

// ---- floating point helpers (x86 semantics) -------------------------------------------------------
pub fn f32_of(x: u128) -> f32 {
    f32::from_bits(x as u32)
}
pub fn f64_of(x: u128) -> f64 {
    f64::from_bits(x as u64)
}
pub fn with_lo32(x: u128, v: f32) -> u128 {
    (x & !0xffff_ffffu128) | v.to_bits() as u128
}
pub fn with_lo64(x: u128, v: f64) -> u128 {
    (x & !0xffff_ffff_ffff_ffffu128) | v.to_bits() as u128
}

/// cvttss2si / cvttsd2si: truncate; NaN and out of range give the integer indefinite value.
pub fn cvtt32(x: f64) -> u32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x8000_0000
    } else {
        (x as i32) as u32
    }
}
pub fn cvtt64(x: f64) -> u64 {
    if x.is_nan() || x >= 9223372036854775808.0 || x < -9223372036854775808.0 {
        0x8000_0000_0000_0000
    } else {
        (x as i64) as u64
    }
}
fn round_even(x: f64) -> f64 {
    x.round_ties_even()
}
/// cvtss2si / cvtsd2si with the default rounding mode (nearest even).
pub fn cvt32(x: f64) -> u32 {
    let r = round_even(x);
    if x.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
        0x8000_0000
    } else {
        (r as i32) as u32
    }
}
pub fn cvt64(x: f64) -> u64 {
    let r = round_even(x);
    if x.is_nan() || r >= 9223372036854775808.0 || r < -9223372036854775808.0 {
        0x8000_0000_0000_0000
    } else {
        (r as i64) as u64
    }
}
/// minss / minsd: the second operand when either is NaN or both are zero.
pub fn min_f(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn max_f(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}
pub fn min_s(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn max_s(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
/// ucomiss / ucomisd result as (zf, pf, cf).
pub fn ucomi(a: f64, b: f64) -> (bool, bool, bool) {
    if a.is_nan() || b.is_nan() {
        (true, true, true)
    } else if a > b {
        (false, false, false)
    } else if a < b {
        (false, false, true)
    } else {
        (true, false, false)
    }
}
#[inline]
pub fn parity(v: u64) -> bool {
    (v as u8).count_ones() % 2 == 0
}
