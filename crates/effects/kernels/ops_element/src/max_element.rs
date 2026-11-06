//! max_element: slot 52/53 (EG: the class overrides 53; dispatcher, AVX2 body, GPU twin slot 54 NOT REBUILT).
use crate::common::*;
use crate::reduce::{render_m, Kind, RMutant, RParams};

pub fn render(p: &RParams, planes: &[&[f32]], svc: &Svc, dst: &mut [f32]) -> Result<Trace, ElError> {
    render_m(Kind::Max, p, planes, svc, dst, RMutant::None)
}
pub fn render_mutant(p: &RParams, planes: &[&[f32]], svc: &Svc, dst: &mut [f32], m: RMutant) -> Result<Trace, ElError> {
    render_m(Kind::Max, p, planes, svc, dst, m)
}
