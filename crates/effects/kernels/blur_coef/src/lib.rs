//! blur_coef: driver arithmetic and small helpers of the shared blur core (class `blur`).
//! Source: neutral_1 dump, driver body (AVX2 route, feature word 0x9d97ff), helpers .
//! Std only, no unsafe. Render flag 0, single worker. See README.md for the assumptions table.
pub mod coef;
pub mod driver;
pub mod helpers;
pub mod x86;

pub use coef::{coeffs, final_scale, radii, Radii};
pub use driver::{drive, plan, DriverParams, PassPlan, Plan};
pub use helpers::{clear, scale_or_copy, HelperError};
