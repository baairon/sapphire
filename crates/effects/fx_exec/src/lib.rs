//! fx_exec: native f32 executor of the compiled effect plan.
//!
//! Label: EVALUATOR-UNCHECKED until E-PARITY (the compiler `fx_eval` is EVALUATOR-PLAN-PARITY on 24 dumps).
//! Std only apart from the read-only path dependencies (the builtin crates and `fx_eval`).
//!
//! Entry point: `execute(program, binding, inputs, host_frames) -> ExecOutput` (module `exec`), or
//! `execute_compiled` on a plan you compiled yourself; `cmds::execute_commands` runs a plan in the text form of the original's compiled plans.

pub mod aspect;
/// the execution backend seam (CPU reference, OpenCL device with per node fallback)
pub mod backend;
/// the per op device table: kernels, allowlist, cpu_only reasons and the self-check
pub mod device_ops;
/// device ops of the pointwise family
pub mod device_ops_pointwise;
/// device ops of the sampling and warp family
pub mod device_ops_warp;
/// the warp footprint prefill on the device (slot 33 log2 of gradient maps)
pub mod device_ops_footprint;
/// device ops of the ripmap family (make_mipmap, blur_warp)
pub mod device_ops_ripmap;
/// device ops of blur, masked_blur, grad, sharpen, edge_aware_sharpen and static2
pub mod device_ops_neighbourhood;
/// device ops of plane assembly (copy_alpha, get_alpha, v3_to_v4_if4_a, append)
pub mod device_ops_plane;
/// device ops of screen, overlay_super and alpha_from_rgb_max
pub mod device_ops_glow_ops;
/// device ops of glow (bright pass, then the neighbourhood blur passes)
pub mod device_ops_glow;
pub mod fxrun;
pub mod cmds;
pub mod nanrule;
pub mod downsampled;
pub mod exec;
pub mod twframe;
pub mod hostconv;
/// the 'ae32' source route (readers), TABLE-NULL-ON-ROUTE, destination refused until
pub mod hostconv14;
/// class postdivide, slot 53, copy_alpha < 0 route (T2 against the emulated original)
pub mod postdiv;
/// owner arm with the Opacity loop
pub mod owner_loop;
/// render history (KEEP or RESTART per render), RENDER-HISTORY-PORTED
pub mod history;
/// Opacity conversion (host checkout integer to the loop value), OPACITY-CONVERT-PORTED
pub mod opacity_conv;
pub mod mode_select;
pub mod srcindep;

/// The stored frame rate shared by the rate getter and the stage's `get_time` divisor (defined with the fold).
pub use fx_eval::fold::stored_frame_rate;
pub mod kernels;
/// OPS_SCALAR_PAIRS census rows, the OPS-ARITH-SCALAR-SCALAR-UNJUDGED tripwire, program-text node paths
pub mod ops_census;
pub mod kernels2;
pub mod par;
pub mod input_class;
pub mod selector;
/// STYLE-SELECT 
pub mod style;
pub mod types;
pub mod judge_keys;

pub use hostconv::{dest_for_host, host_to_image, image_to_host, HostConvert};
pub use exec::{execute_ae32, execute_ae32_const, Ae32ExecOutput, execute_host, HostExecOutput, compile_for, execute, execute_compiled, unsupported_classes, Binding, ExecOutput, HostFrames, Inputs, RunRec};
pub use selector::Tables;
pub use types::{xy_grid, xy_pixel_centre, Arg, ExecError, Geom, GeomMode, Image, KParams, MipImage, Val};
