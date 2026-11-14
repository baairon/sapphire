//! Tail of the counted route: the interior median filter of the two flow planes of a level. Ordinary typed Rust: the level record is
//! the argument; the median launch and the copy back are calls of `TailHost`, in program order.
use crate::body::{Level, LaunchArgs, MedianArgs, TileShape};

pub trait TailHost {
    /// parallel-for launch with its typed arguments
    fn launch(&mut self, args: LaunchArgs);
    /// strided copy of the median output back into a flow plane: destination plane, source plane, level pitch word, level head words
    fn copy_back(&mut self, plane: u64, median_out: u64, pitch: u64, head: [u64; 3]);
}

fn cols(l: &Level) -> i32 {
    l.width
}
fn rows(l: &Level) -> i32 {
    l.height
}
/// Median of each flow plane of the level (words 12 and 13) into the plane at word 22, then the copy back, for levels with at least
/// three columns and three rows.
pub fn median_pair<H: TailHost>(l: &Level, host: &mut H) {
    let median_out = l.median_out;
    for plane in [l.flow_u, l.flow_v] {
        if cols(l) >= 3 && rows(l) >= 3 {
            host.launch(LaunchArgs::Median(MedianArgs { shape: TileShape::of(l), plane, median_out }));
            {
                let shape = TileShape::of(l);
                host.copy_back(plane, median_out, l.elem_bytes, [shape.size, shape.stride, shape.elem_bytes]);
            }
        }
    }
}
