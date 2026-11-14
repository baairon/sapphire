//! Plane callbacks of the optical flow class (AVX2 and FMA arm) and the flow-build body 0x47a380 as ordinary Rust (package optical_flow).
mod callbacks;
mod flowbuild;
mod flowstage;
pub mod body;
mod level;
mod gluefn;
pub mod host;
pub mod refusal;
pub mod split;
pub mod host_types;
pub mod iface;
pub mod join;
mod tailfn;
/// The launch targets of the resample pair are the callbacks of the resample crate.
pub use optical_flow_resample as resample;
pub use callbacks::*;
pub use flowbuild::*;
pub use flowstage::*;
