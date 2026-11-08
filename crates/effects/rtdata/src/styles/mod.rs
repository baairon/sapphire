// Generated from the recorded host data of the styles; plain definitions, no file is read.
// Rows are the ones the effects read; each field is a value the code uses. Do not edit by hand.
//! The styles and their recorded host data.
use crate::types::Style;

mod blur_color;
mod blur_mono;
mod blur_motion_curves_no_transform;
mod diffuse;
mod distort_rgb;
mod drop_shadow;
mod hue_sat_bright;
mod sharpen_best_quality;
mod sharpen_fast;
mod time_warp_rgb;
mod trails;
mod transform_and_blur;
mod warp_perspective;
mod flicker;
mod swishpan;
mod glow;

/// Every style, in the order of the effect list.
pub static STYLES: &[&Style] = &[
    &blur_color::STYLE,
    &blur_mono::STYLE,
    &blur_motion_curves_no_transform::STYLE,
    &diffuse::STYLE,
    &distort_rgb::STYLE,
    &drop_shadow::STYLE,
    &hue_sat_bright::STYLE,
    &sharpen_best_quality::STYLE,
    &sharpen_fast::STYLE,
    &time_warp_rgb::STYLE,
    &trails::STYLE,
    &transform_and_blur::STYLE,
    &warp_perspective::STYLE,
    &flicker::STYLE,
    &swishpan::STYLE,
    &glow::STYLE,
];
