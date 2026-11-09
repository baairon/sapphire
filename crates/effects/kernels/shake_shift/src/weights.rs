//! Weight-table builders 0x445ae0 (AVX, fused) and 0x445bf0 (SSE), the rounding helper 0x66da50 and the scale cell of 0x45ab90.

pub const WEIGHT_LEN: usize = 0x1200;
const W_DIVISOR: f32 = 4608.0;
const W_SPLIT: f32 = 0.5;
const W_TWO: f32 = 2.0;
const W_ONE: f32 = 1.0;
const W_UNIT: f32 = 1.0 / 4096.0;
const W_TWO64: f64 = 2.0;
const W_ONE64: f64 = 1.0;

/// x86 `cvttsd2si` (32-bit): truncation; NaN and out-of-range values give 0x80000000.
pub fn cvttsd2si32(x: f64) -> i32 {
    if x.is_nan() {
        return i32::MIN;
    }
    let t = x.trunc();
    if t >= -2147483648.0 && t <= 2147483647.0 {
        t as i32
    } else {
        i32::MIN
    }
}

/// 0x66da50: `cvttsd2si(copysign(0.5, x) + (f64)(x * 4096.0f32))`; the sign is that of the argument.
pub fn round_q12(x: f32) -> i32 {
    let scaled = (x * 4096.0f32) as f64;
    let half = f64::from_bits(0.5f64.to_bits() | ((x as f64).to_bits() & 0x8000_0000_0000_0000));
    cvttsd2si32(half + scaled)
}

/// Kernel of entry `i`, SSE twin 0x445bf0 (separate roundings).
pub fn kernel_sse(i: u32) -> f32 {
    let s = ((i as i32) as f32 / W_DIVISOR).sqrt();
    if W_SPLIT > s {
        let t = (s * s) as f64;
        (W_ONE64 - t * W_TWO64) as f32
    } else {
        let d = W_ONE - s;
        (W_TWO * d) * d
    }
}

/// Kernel of entry `i`, AVX twin 0x445ae0: vfnmadd213sd, vfmsub213ss and vfmsub231ss are fused (3 sites).
pub fn kernel_avx(i: u32) -> f32 {
    let s = ((i as i32) as f32 / W_DIVISOR).sqrt();
    if W_SPLIT > s {
        let t = (s * s) as f64;
        (-W_TWO64).mul_add(t, W_ONE64) as f32
    } else {
        let a = s.mul_add(W_TWO, -W_TWO);
        s.mul_add(a, -a)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WeightTables {
    /// Integer table: `round_q12(kernel(i))`.
    pub int: Vec<i32>,
    /// Float table: `(f32)int[i] * 2^-12`.
    pub float: Vec<f32>,
    /// Twice the first index whose integer entry is zero; None when no entry is zero.
    pub cutoff: Option<i32>,
}

fn build(kernel: fn(u32) -> f32) -> WeightTables {
    let mut int = Vec::with_capacity(WEIGHT_LEN);
    let mut float = Vec::with_capacity(WEIGHT_LEN);
    let mut cutoff = None;
    for i in 0..WEIGHT_LEN as u32 {
        let e = round_q12(kernel(i));
        int.push(e);
        float.push((e as f32) * W_UNIT);
        if cutoff.is_none() && e == 0 {
            cutoff = Some((i as i32).wrapping_add(i as i32));
        }
    }
    WeightTables { int, float, cutoff }
}

pub fn build_sse() -> WeightTables {
    build(kernel_sse)
}
pub fn build_avx() -> WeightTables {
    build(kernel_avx)
}

/// 0x45ab90: `(f32)cutoff * 2^-12`.
pub fn scale_from_cutoff(cutoff: i32) -> f32 {
    (cutoff as f32) * W_UNIT
}
