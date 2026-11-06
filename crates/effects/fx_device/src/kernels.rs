//! Device kernel sources: OpenCL C 1.2 as Rust string constants, the build options and the registry
//! the source lint walks.
// Provenance: adapted from twixtor plugins/retime-engine/src/device/kernels/mod.rs and starter.rs (8087d06).
//!
//! Every kernel here reproduces the operation order and rounding of the CPU function it ports. Each
//! has a Rust transcription in [`rs`] (one function per work item) that the simulator runs and the
//! tests compare with the CPU code bit for bit.
//!
//! * `copy_f32`: moves 32 bit words, so NaN payloads, -0.0 and subnormals pass unchanged.
//! * `one_minus_f32`: `1 - x` in single precision, one subtraction, no fusion.
//! * `any_nan_f32`: sets a flag when a float plane holds a NaN (the NaN rule check, run on the device so
//!   an output stays resident).
//! * `copy_u32`, `chain_f32`: base self-check kernels (a bit copy and a dependent `v * k + x` loop in
//!   which contraction or flushing would show).
//!
//! To add an op kernel see the API notes of the crate delivery: add the source here, register it in
//! [`SOURCES`], add its transcription to [`rs`] and its self-check case in the executor.

use crate::prelude::PRELUDE;

/// The build options of every program: exactly these, never `-cl-fast-relaxed-math`,
/// `-cl-mad-enable` or any option that relaxes IEEE 754 behaviour.
pub const BUILD_OPTIONS: &str = "-cl-std=CL1.2 -cl-fp32-correctly-rounded-divide-sqrt";

/// Iterations of `chain_f32`; a constant, so every launch is bounded.
pub const CHAIN_STEPS: usize = 64;

pub const COPY_F32: &str = r#"
__kernel void copy_f32(__global const uint* src, __global uint* dst, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    dst[i] = src[i];
}
"#;

pub const ONE_MINUS_F32: &str = r#"
__kernel void one_minus_f32(__global const float* src, __global float* dst, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    dst[i] = 0x1p+0f - src[i];
}
"#;

pub const ANY_NAN_F32: &str = r#"
__kernel void any_nan_f32(__global const float* x, __global uint* flag, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    if (isnan(x[i])) {
        atomic_or(flag, 1u);
    }
}
"#;

pub const BASE: &str = r#"
__kernel void copy_u32(__global const uint* src, __global uint* dst, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    dst[i] = src[i];
}

__kernel void chain_f32(__global const float* x, __global const float* k, __global float* out, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float v = 0x0p+0f;
    for (int s = 0; s < 64; s++) {
        v = v * k[i] + x[i];
    }
    out[i] = v;
}
"#;

/// One registered source: its name, text and whether it mirrors a CPU `mul_add` (and may then call `fma()`).
#[derive(Clone, Copy, Debug)]
pub struct Source {
    pub name: &'static str,
    pub text: &'static str,
    /// True only for a kernel whose CPU function fuses with `mul_add`. Its text must carry the marker
    /// comment `MIRRORS_MUL_ADD:` naming the CPU function.
    pub mirrors_mul_add: bool,
}

/// Every kernel source of the crate. The lint checks each one; a test fails when a `pub const ...: &str` of this
/// file is missing here.
pub const SOURCES: &[Source] = &[
    Source { name: "prelude::PRELUDE", text: PRELUDE, mirrors_mul_add: false },
    Source { name: "kernels::COPY_F32", text: COPY_F32, mirrors_mul_add: false },
    Source { name: "kernels::ONE_MINUS_F32", text: ONE_MINUS_F32, mirrors_mul_add: false },
    Source { name: "kernels::ANY_NAN_F32", text: ANY_NAN_F32, mirrors_mul_add: false },
    Source { name: "kernels::BASE", text: BASE, mirrors_mul_add: false },
];

/// The parts that make up the one device program (every kernel above, after the prelude).
pub const PROGRAM_PARTS: &[&str] = &[COPY_F32, ONE_MINUS_F32, ANY_NAN_F32, BASE];

/// One program text: the prelude, then `parts` in order.
pub fn program(parts: &[&str]) -> String {
    let mut text = String::with_capacity(PRELUDE.len() + parts.iter().map(|p| p.len() + 1).sum::<usize>());
    text.push_str(PRELUDE);
    for p in parts {
        text.push('\n');
        text.push_str(p);
    }
    text
}

/// The program every session builds: the prelude and all registered kernels.
pub fn device_program() -> String {
    program(PROGRAM_PARTS)
}

/// Rust transcriptions of the kernels, one function per work item, statement by statement.
pub mod rs {
    pub fn copy_f32_item(src: &[u32], gid: usize) -> u32 {
        src[gid]
    }

    pub fn one_minus_f32_item(src: &[f32], gid: usize) -> f32 {
        1.0f32 - src[gid]
    }

    /// The flag one work item contributes: 1 when the sample is a NaN.
    pub fn any_nan_f32_item(x: &[f32], gid: usize) -> u32 {
        u32::from(x[gid].is_nan())
    }

    pub fn chain_f32_item(x: f32, k: f32) -> f32 {
        let mut v = 0.0f32;
        for _ in 0..super::CHAIN_STEPS {
            v = v * k + x;
        }
        v
    }
}
