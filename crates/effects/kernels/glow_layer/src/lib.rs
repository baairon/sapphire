//! glow: bright-pass layer (what the glow pass hands to the blur core). std only, no unsafe.
//! Follows the AVX2 route body at 0x674000 (listing address space), reached from the dispatcher head 0x673fa0.
//! The blur core and the full glow layer live in `full`; the add-back onto the picture is not part of this crate.
pub mod bright;
pub mod copy;
pub mod full;
pub mod plane;
pub mod x86;

pub use bright::{assumptions, bright_pass, bright_pass_flat, bright_pass_in_place, bright_pass_roi, Assumption, BrightParams};
pub use copy::{copy_plane, CopyReport, RoiRecord};
pub use full::{full_assumptions, glow_pass, glow_sigmas, pass_args, render_full, GlowParams, PassArgs, GLOW_PASSES};
pub use plane::{FxError, Plane, PlaneMut};

/// Alias of [`bright_pass`] with the same arguments and behaviour.
pub fn render(p: &BrightParams, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    bright_pass(p, src, dst)
}
