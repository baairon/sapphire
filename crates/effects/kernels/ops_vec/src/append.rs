//! Class `append` (vtable, slot 53 entry): the planes of the first argument followed by the planes of the second,
//! at most four in all.
//!
//! The entry runs the plane service once per destination plane. The first loop takes `first` planes of argument 0
//! (service mode 0, channel c, destination plane c). When fewer than four planes were written, a second loop takes `second`
//! planes of argument 1 (service mode 1, channel j, destination plane first + j) and stops once four planes are written.
//! Constants are broadcast and images copied by the plane service, so the entry itself has no arithmetic.
use crate::*;

/// `first` and `second` are the channel counts of the two arguments (the node's counts), `args` the parameter blocks as the
/// base getter returns them, `outs` the destination planes. Returns the plane service actions in call order.
pub fn render(p: &Params, args: &[Block], first: usize, second: usize, outs: &mut [&mut [f32]]) -> Result<Vec<Action>, VecError> {
    if args.len() < 2 || outs.len() < first {
        return Err(VecError::Shape);
    }
    let mut acts = Vec::new();
    for c in 0..first {
        let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, 0, c as i32, outs[c], &p.roi).map_err(VecError::Plane)?;
        acts.push(a);
    }
    if first < 4 {
        let mut j = 0;
        while j < second {
            if first + j >= outs.len() {
                return Err(VecError::Shape);
            }
            let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, 1, j as i32, outs[first + j], &p.roi).map_err(VecError::Plane)?;
            acts.push(a);
            j += 1;
            if first + j >= 4 {
                break;
            }
        }
    }
    Ok(acts)
}
