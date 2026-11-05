//! Class `v2` (vtable, slot 53 entry): out[i] = argument i for i in 0..2 (MEASURED).
use crate::*;

pub const ENTRY: u32 = 0;
/// EV-T COMBOS.tsv: one row, args (1, 1), result 2 channels, no member pass pointer.
pub const COMBO: Combo = Combo { args: &[1, 1], out: 2 };

/// `args` are the node's parameter blocks as the base getter returns them (blocks[mode]); `outs` the destination planes.
pub fn render(p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Vec<Action>, VecError> {
    assemble(2, p, args, outs)
}
