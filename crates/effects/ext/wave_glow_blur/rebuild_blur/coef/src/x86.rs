//! COPY of the three conversion helpers of fx_common::x86 (S5), kept local so this crate stays std only.
//! Intel semantics: NaN or out of range gives i32::MIN. MXCSR (round to nearest even) assumed.

/// `cvttss2si r32, xmm`: truncate toward zero; NaN or out of range gives i32::MIN.
#[inline]
pub fn cvttss2si32(x: f32) -> i32 {
    let t = x.trunc();
    if t >= -2147483648.0 && t < 2147483648.0 { t as i32 } else { i32::MIN }
}

/// `cvttsd2si r32, xmm`: truncate toward zero; NaN or out of range gives i32::MIN.
#[inline]
pub fn cvttsd2si32(x: f64) -> i32 {
    let t = x.trunc();
    if t >= -2147483648.0 && t < 2147483648.0 { t as i32 } else { i32::MIN }
}

/// `cvtss2si r32, xmm` under round to nearest even; NaN or out of range gives i32::MIN.
#[inline]
pub fn cvtss2si32_rne(x: f32) -> i32 {
    let r = x.round_ties_even();
    if r >= -2147483648.0 && r < 2147483648.0 { r as i32 } else { i32::MIN }
}
