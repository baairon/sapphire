//! Class `v3` (vtable, slot 53 entry): out[i] = argument i for i in 0..3 (MEASURED).
use crate::*;

pub const ENTRY: u32 = 0;
/// EV-T COMBOS.tsv: one row, args (1, 1, 1), result 3 channels, no member pass pointer.
pub const COMBO: Combo = Combo { args: &[1, 1, 1], out: 3 };

/// `args` are the node's parameter blocks as the base getter returns them (blocks[mode]); `outs` the destination planes.
pub fn render(p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Vec<Action>, VecError> {
    assemble(3, p, args, outs)
}
