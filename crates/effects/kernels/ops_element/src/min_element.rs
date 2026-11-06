//! min_element: slot 53 dispatcher, AVX2 body (max_element's body with vminss/vminps), GPU twin slot 54 NOT REBUILT.
use crate::common::*;
use crate::reduce::{render_m, Kind, RMutant, RParams};

pub fn render(p: &RParams, planes: &[&[f32]], svc: &Svc, dst: &mut [f32]) -> Result<Trace, ElError> {
    render_m(Kind::Min, p, planes, svc, dst, RMutant::None)
}
pub fn render_mutant(p: &RParams, planes: &[&[f32]], svc: &Svc, dst: &mut [f32], m: RMutant) -> Result<Trace, ElError> {
    render_m(Kind::Min, p, planes, svc, dst, m)
}
