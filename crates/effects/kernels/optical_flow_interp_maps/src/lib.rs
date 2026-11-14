//! Optical flow interpolation maps, AVX2 and FMA route.
//!
//! `native::interp_maps` builds the four output maps from two colour images and a flow field. It prepares the work planes,
//! runs the five tile kernels of `kernels` in a fixed order on one chunk covering every row, and runs one scalar scan
//! itself (`splat_scan`): every pixel whose mask word is zero carries a flow vector that is applied to the pixel position;
//! when the target is inside the frame, a matching error between the two image planes at the displaced position is
//! computed over up to three channels and the target keeps the smallest error.
//! Assumption labels: see `assumptions()`.
use fx_common::nanop::{fma213, fma231, fms213, fnms213, NanOps};
use fx_common::x86::{cvtss2si32_rne, cvttss2si32};

pub struct Assumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

/// Value of the constants the scan reads. Both are splat as scalars.
pub const UNIT: f32 = 1.0;
pub const HALF: f32 = 0.5;

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption {
            id: "FRAC-ONE",
            label: "MEASURED-EMULATOR-READ",
            basis: "scalar f32 1.0 (bits 0000803f): fraction of a clamped sample position; read in counted renders",
        },
        Assumption {
            id: "HALF-CELL",
            label: "WINDOW-OF-ADJACENT-READ",
            basis: "scalar f32 0.5 (bits 0000003f): the pixel centre offset; taken from the window read next to the unit constant, not read on its own",
        },
        Assumption {
            id: "NEG-ONE",
            label: "COMPUTED",
            basis: "the f32 bits 0xbf800000 used as a multiplier; a constant of the code",
        },
    ]
}

/// Split of one source coordinate into the lower and upper sample index and the fraction between them.
/// `c < 0` clamps to samples 0 and 1; `c >= n - 1` clamps to samples n-2 and n-1 with fraction 1; otherwise floor.
#[inline]
pub fn split_coord(c: f32, n: i32) -> (i32, i32, f32) {
    if 0.0f32 > c {
        (0, 1, 0.0)
    } else if c >= (n.wrapping_sub(1)) as f32 {
        (n.wrapping_sub(2), n.wrapping_sub(1), UNIT)
    } else {
        let i = cvtss2si32_rne(c.floor());
        (i, i.wrapping_add(1), c.fsub(i as f32))
    }
}

/// Squared matching error of one channel at one pixel:
/// e = -(((sample_br - sample_lower_left - (sample_tr - sample_tl)) * fx - (sample_tl - sample_lower_left)) * fy) - ((sample_tr - sample_tl) * fx - p + sample_tl), squared.
#[inline]
pub fn channel_error(sample_tl: f32, sample_tr: f32, sample_lower_left: f32, sample_br: f32, fx: f32, fy: f32, p: f32) -> f32 {
    let dxa = sample_tr.fsub(sample_tl);
    let delta_bottom = sample_br.fsub(sample_lower_left);
    let dyl = sample_tl.fsub(sample_lower_left);
    let t = delta_bottom.fsub(dxa);
    let a = fms213(dxa, fx, p);
    let b = fms213(t, fx, dyl);
    let c = a.fadd(sample_tl);
    let e = fnms213(b, fy, c);
    e.fmul(e)
}

/// Planes of one scan: the colour planes and the flow field are read; the target flow, the best cost and the covered flag
/// are written; the source-done flags are read.
pub struct ScanPlanes<'a> {
    pub prev_planes: [&'a [u32]; 3],
    pub next_planes: [&'a [u32]; 3],
    pub a: &'a [u32],
    pub b: &'a [u32],
    pub target_flow_x: &'a mut [u32],
    pub target_flow_y: &'a mut [u32],
    pub best_cost: &'a mut [u32],
    pub covered: &'a mut [u32],
    pub source_done: &'a [u32],
}

#[inline]
fn rd(plane: &[u32], i: i64) -> u32 {
    plane[usize::try_from(i).expect("negative buffer index")]
}
#[inline]
fn rdf(plane: &[u32], i: i64) -> f32 {
    f32::from_bits(rd(plane, i))
}
#[inline]
fn wr(plane: &mut [u32], i: i64, v: u32) {
    plane[usize::try_from(i).expect("negative buffer index")] = v;
}

/// The scan: for each row and column whose mask word is zero, move the pixel by `scale` times its flow vector, and keep
/// the smallest matching error per target pixel (target flow words, error and covered flag are written).
pub fn splat_scan(planes: &mut ScanPlanes, w: i32, h: i32, stride: i32, nch: i32, scale_bits: u32) {
    if h <= 0 {
        return;
    }
    let scale = f32::from_bits(scale_bits);
    let nc = nch.min(3);
    let mut row_start: i64 = 0;
    for row in 0..h {
        if w > 0 {
            let yf = row as f32;
            let ysum = HALF.fadd(yf);
            for x in 0..w as i64 {
                if rd(planes.source_done, row_start + x) != 0 {
                    continue;
                }
                let a_bits = rd(planes.a, row_start + x);
                let b_bits = rd(planes.b, row_start + x);
                let av = f32::from_bits(a_bits);
                let bv = f32::from_bits(b_bits);
                let xf = (x as i32) as f32;
                let sb = scale.fmul(bv);
                let tx = HALF.fadd(xf);
                let ty = fma213(sb, -1.0f32, ysum);
                let tx = fma231(tx, scale, av);
                let ix = cvttss2si32(tx);
                let iy = cvttss2si32(ty);
                if ix < 0 || ix >= w || iy < 0 || iy >= h {
                    continue;
                }
                let mut cost = 0.0f32;
                if nc > 0 {
                    let src_x = av.fadd(xf);
                    let src_y = bv.fadd(yf);
                    let (x_lo, x_hi, fx) = split_coord(src_x, w);
                    let (y_lo, y_hi, fy) = split_coord(src_y, h);
                    let at = |yy: i32, xx: i32| -> i64 { yy.wrapping_mul(stride) as i64 + xx as i64 };
                    let prev_index = row.wrapping_mul(stride) as i64 + x;
                    for c in 0..nc as usize {
                        let pl = planes.next_planes[c];
                        let sample_tl = rdf(pl, at(y_lo, x_lo));
                        let sample_tr = rdf(pl, at(y_lo, x_hi));
                        let sample_lower_left = rdf(pl, at(y_hi, x_lo));
                        let sample_br = rdf(pl, at(y_hi, x_hi));
                        let p = rdf(planes.prev_planes[c], prev_index);
                        let e = channel_error(sample_tl, sample_tr, sample_lower_left, sample_br, fx, fy, p);
                        cost = if c == 0 { e } else { e.fadd(cost) };
                    }
                }
                let d = ix.wrapping_add(iy.wrapping_mul(stride)) as i64;
                if rd(planes.covered, d) != 0 {
                    let old = rdf(planes.best_cost, d);
                    if old > cost {
                        wr(planes.target_flow_x, d, a_bits);
                        wr(planes.target_flow_y, d, b_bits);
                        wr(planes.best_cost, d, cost.to_bits());
                    }
                } else {
                    wr(planes.target_flow_x, d, a_bits);
                    wr(planes.target_flow_y, d, b_bits);
                    wr(planes.best_cost, d, cost.to_bits());
                    wr(planes.covered, d, 1);
                }
            }
        }
        row_start += stride as i64;
    }
}

pub mod kernels;
pub mod native;
