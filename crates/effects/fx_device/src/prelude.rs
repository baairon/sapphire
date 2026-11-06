//! The OpenCL C prelude every device program starts with, and its Rust transcription ([`rs`]).
// Provenance: adapted from twixtor plugins/retime-engine/src/device/kernels/prelude.rs (8087d06).
//!
//! Precision contract. `FP_CONTRACT OFF`, so no expression is fused into a multiply add; `fma()` is
//! written explicitly, and only where the CPU kernel calls `mul_add`. `cl_khr_fp64` for the double
//! precision steps. Every float to integer conversion goes through a range test first, because a C
//! cast of an out of range value is undefined: inside the range the cast truncates toward zero,
//! outside it and for NaN the result is the minimum of the target type, which is what x86 `cvtt`
//! gives. Float literals are written in hexadecimal only, so no decimal to binary rounding is
//! involved.

/// The prelude source. It must stay the first text of every program ([`crate::kernels::program`]).
pub const PRELUDE: &str = r#"#pragma OPENCL FP_CONTRACT OFF
#pragma OPENCL EXTENSION cl_khr_fp64 : enable

/* Truncation toward zero; outside [-2^31, 2^31) and NaN give INT_MIN (x86 cvttss2si). */
int cvtt_i32_f(float x) {
    if (x >= -0x1p31f && x < 0x1p31f) {
        return (int)x;
    }
    return INT_MIN;
}

/* Truncation toward zero; outside [-2^63, 2^63) and NaN give LONG_MIN. */
long cvtt_i64_f(float x) {
    if (x >= -0x1p63f && x < 0x1p63f) {
        return (long)x;
    }
    return LONG_MIN;
}

/* Truncation toward zero of a double; outside (-2^31 - 1, 2^31) and NaN give INT_MIN. */
int cvtt_i32_d(double x) {
    if (x > -0x1.00000002p31 && x < 0x1p31) {
        return (int)x;
    }
    return INT_MIN;
}

/* Truncation toward zero of a double; outside [-2^63, 2^63) and NaN give LONG_MIN. */
long cvtt_i64_d(double x) {
    if (x >= -0x1p63 && x < 0x1p63) {
        return (long)x;
    }
    return LONG_MIN;
}
"#;

/// Rust transcriptions of the prelude, one function per OpenCL C function, statement by statement.
pub mod rs {
    pub fn cvtt_i32_f(x: f32) -> i32 {
        if x >= -2147483648.0f32 && x < 2147483648.0f32 {
            return x as i32;
        }
        i32::MIN
    }

    pub fn cvtt_i64_f(x: f32) -> i64 {
        if x >= -9223372036854775808.0f32 && x < 9223372036854775808.0f32 {
            return x as i64;
        }
        i64::MIN
    }

    pub fn cvtt_i32_d(x: f64) -> i32 {
        if x > -2147483649.0f64 && x < 2147483648.0f64 {
            return x as i32;
        }
        i32::MIN
    }

    pub fn cvtt_i64_d(x: f64) -> i64 {
        if x >= -9223372036854775808.0f64 && x < 9223372036854775808.0f64 {
            return x as i64;
        }
        i64::MIN
    }
}
