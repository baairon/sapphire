//! Native map generator: the work planes live in one struct of word buffers, and the five tile kernels run in process on one
//! chunk covering every row (one worker). The result is the four output planes.
//! Assumptions (labels in `native_assumptions()`): ONE-CHUNK (the splitter gives the whole frame to one worker),
//! X8 (the additive term of the second and fourth outputs is supplied by the caller), EXTENT-ONE-REFUSED.
use crate::kernels::*;
use crate::*;

pub struct NativeAssumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn native_assumptions() -> Vec<NativeAssumption> {
    vec![
        NativeAssumption { id: "ONE-CHUNK", label: "STAND-IN", basis: "the original splits the rows over workers; results of the splat depend on the split; the native builder uses one chunk of all rows" },
        NativeAssumption { id: "X8", label: "STAND-IN", basis: "the additive term of the second and fourth outputs (read from the job object in the original) is a caller value; the default is 0.0" },
        NativeAssumption { id: "OUT4", label: "STAND-IN", basis: "the four outputs are the three work planes the interpolation kernel overwrites and one separate plane" },
    ]
}

#[derive(Debug, PartialEq, Eq)]
pub enum NativeError {
    ExtentOneRefused,
    Shape(String),
}

/// Upper bound of the fill iterations of one sequence.
const MAX_FILL_ITERATIONS: u8 = 25;

/// The work planes of one build. All planes hold `w * h` words (row stride `w`).
struct WorkPlanes {
    prev_planes: [Vec<u32>; 3],
    next_planes: [Vec<u32>; 3],
    a: Vec<u32>,
    b: Vec<u32>,
    target_flow_x: Vec<u32>,
    target_flow_y: Vec<u32>,
    /// Two value planes of the fill passes; which one is read alternates between passes.
    value_a: Vec<u32>,
    value_b: Vec<u32>,
    /// First output plane; best cost during the splat, then a validity plane of the fill passes.
    out_a: Vec<u32>,
    /// Second output plane; the covered flags, kept by the fill passes.
    out_b: Vec<u32>,
    /// Third output plane; the source-done flags, then a validity plane of the fill passes.
    out_c: Vec<u32>,
    /// Fourth output plane.
    out_d: Vec<u32>,
}

/// Pick the (read, written) pair of two planes whose roles alternate.
fn roles<'a>(first: &'a mut Vec<u32>, second: &'a mut Vec<u32>, swapped: bool) -> (&'a mut Vec<u32>, &'a mut Vec<u32>) {
    if swapped {
        (second, first)
    } else {
        (first, second)
    }
}

impl WorkPlanes {
    /// Start a fill sequence: the read value plane takes `source`, the read validity plane takes the covered flags, both
    /// written planes are cleared.
    fn start_sequence(&mut self, source_is_x: bool, swapped: bool) {
        let source: &Vec<u32> = if source_is_x { &self.target_flow_x } else { &self.target_flow_y };
        let (value_in, value_out) = roles(&mut self.value_a, &mut self.value_b, swapped);
        value_in.copy_from_slice(source);
        value_out.fill(0);
        let (valid_in, valid_out) = roles(&mut self.out_a, &mut self.out_c, swapped);
        valid_in.copy_from_slice(&self.out_b);
        valid_out.fill(0);
    }

    /// Repeat: count the zero validity words; stop at zero; otherwise fill and swap the roles. Returns the swap state.
    fn fill_passes(&mut self, w: i32, h: i32, mut swapped: bool) -> bool {
        let mut iterations: u8 = 0;
        loop {
            let (valid_in, _) = roles(&mut self.out_a, &mut self.out_c, swapped);
            let zeros = k4_count_zero(w, w, valid_in, 0, h);
            if zeros == 0.0 {
                break;
            }
            {
                let (value_in, value_out) = roles(&mut self.value_a, &mut self.value_b, swapped);
                let (valid_in, valid_out) = roles(&mut self.out_a, &mut self.out_c, swapped);
                let mut k = K2 { w, h, stride: w, covered: &self.out_b, value_in, value_out, valid_in, valid_out };
                k2_fill(&mut k, 0, h);
            }
            iterations += 1;
            swapped = !swapped;
            if iterations >= MAX_FILL_ITERATIONS {
                break;
            }
        }
        swapped
    }
}

/// Run the whole map generator: `prev` and `next` are the colour planes (up to three are read), `flow_x` and `flow_y` the flow planes
/// (w*h each); returns the four output planes.
pub fn interp_maps(w: usize, h: usize, prev: &[Vec<f32>], next: &[Vec<f32>], flow_x: &[f32], flow_y: &[f32], scale: f32, flow_offset: f32) -> Result<[Vec<f32>; 4], NativeError> {
    if w == 1 || h == 1 {
        return Err(NativeError::ExtentOneRefused);
    }
    let n = w * h;
    if prev.len() != next.len() || prev.is_empty() || flow_x.len() != n || flow_y.len() != n || prev.iter().chain(next.iter()).any(|p| p.len() != n) {
        return Err(NativeError::Shape("interp_maps: plane sizes".into()));
    }
    let nch = prev.len().min(3) as i32;
    let bits = |src: &[f32]| -> Vec<u32> { src.iter().map(|v| v.to_bits()).collect() };
    let zeros = || vec![0u32; n];
    let colour = |planes: &[Vec<f32>]| -> [Vec<u32>; 3] { std::array::from_fn(|k| if k < planes.len() { bits(&planes[k]) } else { zeros() }) };
    let mut p = WorkPlanes {
        prev_planes: colour(prev),
        next_planes: colour(next),
        a: bits(flow_x),
        b: bits(flow_y),
        target_flow_x: zeros(),
        target_flow_y: zeros(),
        value_a: zeros(),
        value_b: zeros(),
        out_a: zeros(),
        out_b: zeros(),
        out_c: zeros(),
        out_d: zeros(),
    };
    let (wi, hi) = (w as i32, h as i32);

    // forward splat; all work planes start at zero
    {
        let mut k = K1 {
            w: wi, h: hi, stride: wi, nch, scale, neg1: -1.0f32,
            prev_planes: [&p.prev_planes[0], &p.prev_planes[1], &p.prev_planes[2]],
            next_planes: [&p.next_planes[0], &p.next_planes[1], &p.next_planes[2]],
            a: &p.a, b: &p.b,
            target_flow_x: &mut p.target_flow_x, target_flow_y: &mut p.target_flow_y,
            best_cost: &mut p.out_a, covered: &mut p.out_b, source_done: &mut p.out_c,
        };
        if k1_splat(&mut k, 0, hi).is_err() {
            return Err(NativeError::ExtentOneRefused);
        }
    }
    // scalar scan
    {
        let mut planes = ScanPlanes {
            prev_planes: [&p.prev_planes[0], &p.prev_planes[1], &p.prev_planes[2]],
            next_planes: [&p.next_planes[0], &p.next_planes[1], &p.next_planes[2]],
            a: &p.a, b: &p.b,
            target_flow_x: &mut p.target_flow_x, target_flow_y: &mut p.target_flow_y,
            best_cost: &mut p.out_a, covered: &mut p.out_b, source_done: &p.out_c,
        };
        splat_scan(&mut planes, wi, hi, wi, nch, scale.to_bits());
    }
    // first sequence: fill from the first target flow plane, then keep the filled values where the flag is zero
    p.start_sequence(true, false);
    let swapped = p.fill_passes(wi, hi, false);
    {
        let (value_in, _) = roles(&mut p.value_a, &mut p.value_b, swapped);
        k3_masked_copy(wi, wi, &p.out_b, value_in, &mut p.target_flow_x, 0, hi);
    }
    // second sequence: the same with the second target flow plane
    p.start_sequence(false, swapped);
    let swapped = p.fill_passes(wi, hi, swapped);
    {
        let (value_in, _) = roles(&mut p.value_a, &mut p.value_b, swapped);
        k3_masked_copy(wi, wi, &p.out_b, value_in, &mut p.target_flow_y, 0, hi);
    }
    // interpolation of the output maps
    {
        let mut k = K5 {
            w: wi, stride: wi, scale, flow_offset, a: &p.target_flow_x, b: &p.target_flow_y, flow_b_origin: 0,
            out: [&mut p.out_a, &mut p.out_b, &mut p.out_c, &mut p.out_d],
        };
        k5_interp(&mut k, 0, hi);
    }
    let cv = |v: &[u32]| v.iter().map(|b| f32::from_bits(*b)).collect::<Vec<f32>>();
    Ok([cv(&p.out_a), cv(&p.out_b), cv(&p.out_c), cv(&p.out_d)])
}
