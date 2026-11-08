//! Local copy of the x86 helper semantics (same signatures as fx_common::x86).
//!
//! The second operand passes through `black_box`. With a constant second operand the compiler may otherwise lower the select to a
//! number-semantics minimum or maximum (fminnm and fmaxnm on AArch64), which returns a quiet NaN for a signalling NaN first operand
//! where minss and maxss return the second operand.

use std::hint::black_box;

/// Intel minss(a, b): first source a; NaN or equal returns b.
#[inline]
pub fn minss(a: f32, b: f32) -> f32 {
    let b = black_box(b);
    if a < b { a } else { b }
}

/// Intel maxss(a, b): first source a; NaN or equal returns b.
#[inline]
pub fn maxss(a: f32, b: f32) -> f32 {
    let b = black_box(b);
    if a > b { a } else { b }
}
