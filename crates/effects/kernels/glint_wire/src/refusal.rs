//! Named refusals of the glint op. The four decided from a request's own words (blur flag, region selector, diagonal extent, shift)
//! are named and decided by the worker's one predicate ([`glint_worker::refusal`]); this module holds the names the host guards use and
//! the band guard, an invariant of the host's own partition.
pub use glint_worker::{extent_refused, shift_out_of_range, NAME_BLUR_FLAG as BLUR_FLAG, NAME_DIAG_EXTENT as EXTENT, NAME_REGION_SELECTOR as REGION_SELECTOR, NAME_SHIFT as SHIFT};

pub const BAND: &str = "GLINT-DIAG-BAND-REFUSED (glint: a band outside the partition of the diagonal rows)";
pub use glint_dispatch::NAME_LAYOUT as LAYOUT;

/// The band of the diagonal bodies must lie in [0, n) where n = ((W - 1) >> 1) + H - 2.
pub fn band_ok(start: i32, count: i32, n: i32) -> bool {
    start >= 0 && count >= 0 && start as i64 + count as i64 <= n.max(0) as i64
}
