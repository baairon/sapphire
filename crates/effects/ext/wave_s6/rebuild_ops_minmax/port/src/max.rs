//! Class `max` (vtable, slot 53, AVX2 body). Shape shared with `min` (common.rs).
use crate::common::{run_bin, BKind, Mutant, OpError, Operand, Params};

pub const KIND: BKind = BKind::Max;

/// S3-style entry: renders the output planes from explicit parameters (nout, n, vec_one, dst3 phase) and the two operands.
pub fn render(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]]) -> Result<(), OpError> {
    run_bin(KIND, p, a, b, out, Mutant::None)
}

pub fn render_mutant(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    run_bin(KIND, p, a, b, out, m)
}
