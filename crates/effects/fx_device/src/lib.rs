#![deny(unsafe_code)]
//! fx_device: the OpenCL device foundation of the effect executor.
//!
//! The crate has no dependency and no build-time requirement on OpenCL: the ICD loader is opened
//! at run time ([`cl`]), so a machine without a runtime only gets a [`Backend::Refused`] or
//! [`Backend::Cpu`] value from [`select`]. A device is used only when it passes the probe
//! ([`probe::requirements`]) and the base self-check ([`selfcheck::check_base`]); each op kernel
//! is then admitted by the executor's own self-check against the CPU kernel.
//!
//! Contract of every kernel: `FP_CONTRACT OFF`, `-cl-fp32-correctly-rounded-divide-sqrt`, no `mad`
//! (and `fma` only in a kernel that mirrors a CPU `mul_add`), hexadecimal float literals and
//! bounded loops ([`lint`]). A device never changes the bytes a render produces: an op that is not
//! allowlisted, an error, a memory miss or a NaN in a float output sends that node to the CPU.
//!
//! `unsafe` code is confined to [`cl`]; `tests/no_unsafe.rs` scans every other file for it.

#[allow(unsafe_code)]
pub mod cl;
pub mod checkcache;
pub mod device;
pub mod devcheck;
pub mod image;
pub mod kernels;
pub mod lint;
pub mod prelude;
pub mod probe;
pub mod progcache;
pub mod selfcheck;
pub mod sim;

pub use device::{select, Backend, DeviceCtx, DeviceKind};
pub use image::{DevError, DevImage, Session};
