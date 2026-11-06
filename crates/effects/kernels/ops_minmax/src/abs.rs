//! Class `abs` (vtable, slot 53, AVX2 body, own slot 60 builder : 4 combos, counts 1..4).
use crate::common::{run_un, Mutant, OpError, UKind, UParams};

pub const KIND: UKind = UKind::Abs;

/// S3-style entry: `count` channels of `n` floats from `planes`, written to `out`; `p` carries n, the vector mask and the destination phases.
pub fn render(p: &UParams, count: i32, planes: &[&[f32]], out: &mut [&mut [f32]]) -> Result<(), OpError> {
    run_un(KIND, p, count, planes, out, Mutant::None)
}

pub fn render_mutant(p: &UParams, count: i32, planes: &[&[f32]], out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    run_un(KIND, p, count, planes, out, m)
}
