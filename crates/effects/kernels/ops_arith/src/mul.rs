//! Class `*` (slot 53 dispatcher, AVX2 body in lib.rs docs). One module per class; the body shape is shared (common.rs).
use crate::common::{run, Kind, Mutant, OpError, Operand, Params};

pub const KIND: Kind = Kind::Mul;

pub fn render(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]]) -> Result<(), OpError> {
    run(KIND, p, a, b, out, Mutant::None)
}

pub fn render_mutant(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    run(KIND, p, a, b, out, m)
}
