//! The warp footprint prefill on the device: the slot 33 route of `warp` and `warp_rel` (`warp_log_gradient::slot33_warp`),
//! which turns the two coordinate planes into the log2 of gradient maps c1 and c2 that the footprint branch of the driver reads.
//!
//! The kernel is `fp_prefill_u32` in `fx_device::kernels_footprint`. The host part ([`slot33_plan`]) takes the record cells, the
//! plane scales `v1`, `v2` and the min or sum variant of each plane from the same expressions and the same public helpers of
//! the CPU crate as `kernels2::warp_k`; [`slot33_run`] launches the kernel once per plane. [`fp_lattice`] and
//! [`fp_warp_launch`] then run the driver's footprint branch with the F1 lattice sampler over every ripmap plane (`fp_warp_f32`),
//! for `warp` (`warp_driver::run`) and `warp_rel` (`warp_rel_sampler::run`, whose footprint branch is the same statements).
//!
//! Self-checks, each against the CPU crate on this device:
//! * [`prefill_selfcheck`]: `fp_prefill_u32` against `warp_log_gradient::prefill::run_body` on frames from 2x1 to 64x32 with
//!   ramps, flat planes, subnormal steps, overflowing steps, NaN of both signs with payloads, infinities and signed zeros, both
//!   variants and adversarial `v`, `pre_a`, `pre_b`; width 1 frames must be declined (the CPU body refuses them).
//! * [`ops_selfcheck`]: `fp33_ops_u32` against the public helpers `add_ss`, `sub_ss`, `mul_ss`, `min_ss`, `fma213` and
//!   `fmsub231`, NaN payloads compared.
//! * [`route_selfcheck`]: [`slot33_run`] against `slot33_warp` for every filter class, frame and host aspect.

use crate::kernels2::cvt_i32;
use fx_device::cl::{Arg as ClArg, Buffer};
use fx_device::selfcheck::adversarial_f32;
use fx_device::sim::Rng;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;
use warp_log_gradient::prefill::{self, add_ss, fma213, fmsub231, min_ss, mul_ss, sub_ss, BodyError, Params};
use warp_log_gradient::{Prim33, Rec33, QUARTER};

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(m.into())
}

/// The host scalars of slot 33 for one frame and one filter word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot33Plan {
    /// the flag byte the driver reads: 1 when the prefill runs (filter word nonzero), else 0 (c1 and c2 are then not read)
    pub flag_byte: u8,
    /// plane scales (f32 bits): v1 for the u plane (c1), v2 for the v plane (c2)
    pub v1: u32,
    pub v2: u32,
    /// min variant (1) or sum variant (0) of each plane
    pub min1: u32,
    pub min2: u32,
}

/// The slot 33 record of `warp_k` and `warp_rel_k` for a w x h frame at host aspect 1.0 (`kernels2::slot33_rec`, the one both CPU
/// kernels build).
pub fn slot33_record(w: usize, h: usize) -> Rec33 {
    crate::kernels2::slot33_rec(w, h)
}

/// The scalars `slot33_warp` derives for `prim` and `rec` (gate 0): flag byte, plane scales and variants. `filter` is the lane the
/// executor passes (warp: the filter lane; warp_rel: argument 3); the prim word is `cvt_i32` of it, as in the CPU route.
pub fn slot33_plan(w: usize, h: usize, filter: f32, host_aspect: f32) -> Result<Slot33Plan, DevError> {
    if host_aspect != 1.0 {
        return Err(unsup("slot 33 geometry cells are defined only at host aspect 1.0 (the CPU kernel reports it)"));
    }
    let prim = Prim33 { n: cvt_i32(filter), cc: 0 };
    let rec = slot33_record(w, h);
    let flag = prim.n != 0 && prim.cc == 0;
    let r8 = if prim.n >= 3 {
        2
    } else if prim.n >= 2 {
        1
    } else {
        0
    };
    let v1 = mul_ss(sub_ss(rec.assumption_g, rec.assumption_f), QUARTER);
    let v2 = mul_ss(sub_ss(rec.geom33_a, rec.assumption_h), QUARTER);
    Ok(Slot33Plan { flag_byte: flag as u8, v1, v2, min1: (r8 > 0) as u32, min2: (r8 > 1) as u32 })
}

fn frame_ok(w: usize, h: usize) -> Result<usize, DevError> {
    if w < 2 {
        return Err(unsup("slot 33 needs a frame at least 2 pixels wide (the CPU body reports it)"));
    }
    if h == 0 || w >= (1 << 24) || h >= (1 << 24) || w * h >= i32::MAX as usize {
        return Err(unsup("frame outside the prefill kernel's index range"));
    }
    Ok(w * h)
}

/// One launch of `fp_prefill_u32`: `dst` = the log2 of gradient map of `src` (both w x h, row stride w).
#[allow(clippy::too_many_arguments)]
pub fn prefill_launch<T: fx_device::cl::Pod, U: fx_device::cl::Pod>(s: &Session, src: &Buffer<T>, dst: &Buffer<U>, w: usize, h: usize, v: u32, pre_a: u32, pre_b: u32, min: u32) -> Result<(), DevError> {
    let n = frame_ok(w, h)?;
    if src.len() < n || dst.len() < n {
        return Err(unsup("prefill plane shorter than the frame"));
    }
    s.launch("fp_prefill_u32", n, &[src.arg(), dst.arg(), ClArg::I32(w as i32), ClArg::I32(h as i32), ClArg::U32(v), ClArg::U32(pre_a), ClArg::U32(pre_b), ClArg::I32(min as i32)])?;
    Ok(())
}

/// What the device slot 33 route leaves for the footprint branch.
pub struct Slot33Dev {
    pub plan: Slot33Plan,
    /// plane 0 = c1 (from u), plane 1 = c2 (from v); None when the flag byte is 0 (the CPU leaves its sentinel there, never read)
    pub c: Option<DevImage>,
}

/// The slot 33 route on the device for coordinate planes `u` and `v` of a w x h frame. Declines what the CPU route reports as
/// an error (host aspect other than 1.0, a frame narrower than 2 with the flag set), so the CPU runs such a node.
pub fn slot33_run(s: &Arc<Session>, w: usize, h: usize, filter: f32, host_aspect: f32, u: &Buffer<f32>, v: &Buffer<f32>) -> Result<Slot33Dev, DevError> {
    let plan = slot33_plan(w, h, filter, host_aspect)?;
    if plan.flag_byte == 0 {
        return Ok(Slot33Dev { plan, c: None });
    }
    let n = frame_ok(w, h)?;
    let c = DevImage::alloc(s, 2, n)?;
    prefill_launch(s, u, c.plane(0), w, h, plan.v1, 0, 0, plan.min1)?;
    prefill_launch(s, v, c.plane(1), w, h, plan.v2, 0, 0, plan.min2)?;
    Ok(Slot33Dev { plan, c: Some(c) })
}

// ---- footprint branch and F1 lattice sampler ------------------------------------------------------------------------

/// The ripmap as `fp_warp_f32` reads it: every plane in one buffer, plane (l, j) at float `offs[l * np + j]` with `plen` floats,
/// the level widths and heights and the scale table (f32 bits; empty when the ripmap has none, so every lookup misses).
pub(crate) struct FpLattice<'a> {
    owned: Option<DevImage>,
    borrowed: Option<&'a Buffer<f32>>,
    pub offs: Vec<u32>,
    pub plen: Vec<u32>,
    pub wtab: Vec<i32>,
    pub htab: Vec<i32>,
    pub t: Vec<u32>,
}

impl FpLattice<'_> {
    fn buf(&self) -> &Buffer<f32> {
        match (&self.owned, self.borrowed) {
            (Some(d), _) => d.plane(0),
            (None, Some(b)) => b,
            (None, None) => unreachable!("a lattice holds a buffer"),
        }
    }
}

fn level_tab(v: &[usize], what: &str) -> Result<Vec<i32>, DevError> {
    if v.is_empty() || v.iter().any(|&x| x >= (1 << 24)) {
        return Err(unsup(&format!("ripmap {what} table outside the footprint kernel's range")));
    }
    Ok(v.iter().map(|&x| x as i32).collect())
}

/// The whole lattice of a host ripmap (uploaded, every plane) or of a device resident one (its own buffer and offsets).
pub(crate) fn fp_lattice<'a>(s: &Arc<Session>, m: &crate::device_ops_warp::MipRef<'a>) -> Result<FpLattice<'a>, DevError> {
    use crate::device_ops_warp::MipRef;
    let (widths, heights, scale) = match m {
        MipRef::Host(h) => (&h.ripmap.widths, &h.ripmap.heights, &h.scale),
        MipRef::Dev(d) => (&d.widths, &d.heights, &d.scale),
    };
    let wtab = level_tab(widths, "width")?;
    let htab = level_tab(heights, "height")?;
    let (nl, np) = (wtab.len(), htab.len());
    let t = scale.clone().unwrap_or_default();
    let mut offs = Vec::with_capacity(nl * np);
    let mut plen = Vec::with_capacity(nl * np);
    let lat = match m {
        MipRef::Host(h) => {
            let rp = &h.ripmap;
            if rp.planes.len() != nl * np {
                return Err(unsup("ripmap plane count differs from its level tables"));
            }
            let mut flat: Vec<f32> = Vec::new();
            for l in 0..nl {
                for j in 0..np {
                    let pl = &rp.planes[rp.idx(l, j)];
                    offs.push(flat.len());
                    plen.push(pl.len());
                    flat.extend(pl.iter().map(|b| f32::from_bits(*b)));
                }
            }
            if flat.is_empty() || flat.len() >= u32::MAX as usize {
                return Err(unsup("ripmap outside the footprint kernel's index range"));
            }
            FpLattice { owned: Some(DevImage::upload(s, &[flat.as_slice()])?), borrowed: None, offs: Vec::new(), plen: Vec::new(), wtab, htab, t }
        }
        MipRef::Dev(d) => {
            if d.offsets.len() != nl * np || d.total() == 0 || d.total() >= u32::MAX as usize || d.total() > d.dev.plane(0).len() {
                return Err(unsup("resident ripmap outside the footprint kernel's index range"));
            }
            for l in 0..nl {
                for j in 0..np {
                    offs.push(d.offsets[l * np + j]);
                    plen.push(d.plane_len(l, j));
                }
            }
            FpLattice { owned: None, borrowed: Some(d.dev.plane(0)), offs: Vec::new(), plen: Vec::new(), wtab, htab, t }
        }
    };
    if plen.iter().any(|&p| p >= i32::MAX as usize) {
        return Err(unsup("ripmap plane outside the footprint kernel's index range"));
    }
    Ok(FpLattice { offs: offs.iter().map(|&o| o as u32).collect(), plen: plen.iter().map(|&p| p as u32).collect(), ..lat })
}

/// One launch of `fp_warp_f32`: the footprint branch with the F1 sampler where c1 or c2 is positive or NaN, the normal path on
/// plane (0, 0) elsewhere. `coords` = the two coordinate planes, `pre` = the prefill maps (plane 0 = c1, plane 1 = c2), `out` the
/// result (ch planes). A tap or index outside what the CPU reads declines (the CPU kernel reports its error).
#[allow(clippy::too_many_arguments)]
pub(crate) fn fp_warp_launch(s: &Arc<Session>, rel: bool, w: usize, h: usize, ch: usize, lat: &FpLattice, coords: &DevImage, pre: &DevImage, mx: i32, my: i32, g: &crate::types::Geom, out: &DevImage) -> Result<(), DevError> {
    let n = w * h;
    if !(1..=4).contains(&ch) || w == 0 || h == 0 || w >= (1 << 24) || h >= (1 << 24) || n >= i32::MAX as usize {
        return Err(unsup("frame outside the footprint kernel's range"));
    }
    if coords.n != n || pre.n != n || out.n != n || coords.channels() != 2 || pre.channels() != 2 || out.channels() != ch {
        return Err(unsup("footprint operands of another size"));
    }
    let ctx = &s.context;
    let ro = fx_device::cl::Access::ReadOnly;
    let nz = |v: &[u32]| if v.is_empty() { vec![0u32] } else { v.to_vec() };
    let offs = Buffer::from_slice(ctx, &lat.offs, ro)?;
    let plen = Buffer::from_slice(ctx, &lat.plen, ro)?;
    let wtab = Buffer::from_slice(ctx, &lat.wtab, ro)?;
    let htab = Buffer::from_slice(ctx, &lat.htab, ro)?;
    let ttab = Buffer::from_slice(ctx, &nz(&lat.t), ro)?;
    let err = Buffer::from_slice(ctx, &[0u32], fx_device::cl::Access::ReadWrite)?;
    let o = |k: usize| out.plane(k.min(ch - 1));
    s.launch(
        "fp_warp_f32",
        n,
        &[
            lat.buf().arg(),
            offs.arg(),
            plen.arg(),
            wtab.arg(),
            htab.arg(),
            ttab.arg(),
            ClArg::I32(lat.wtab.len() as i32),
            ClArg::I32(lat.htab.len() as i32),
            ClArg::I32(lat.t.len().min(i32::MAX as usize) as i32),
            coords.plane(0).arg(),
            coords.plane(1).arg(),
            pre.plane(0).arg(),
            pre.plane(1).arg(),
            o(0).arg(),
            o(1).arg(),
            o(2).arg(),
            o(3).arg(),
            ClArg::I32(ch as i32),
            ClArg::I32(w as i32),
            ClArg::I32(h as i32),
            ClArg::I32(mx),
            ClArg::I32(my),
            ClArg::F32(g.aspect),
            ClArg::F32(g.left),
            ClArg::F32(g.right),
            ClArg::F32(g.assumption_e),
            ClArg::F32(g.assumption_h),
            ClArg::I32(g.roi_x),
            ClArg::I32(g.roi_y),
            ClArg::I32(rel as i32),
            err.arg(),
        ],
    )?;
    if s.queue.read(&err)?[0] != 0 {
        return Err(unsup("a footprint tap or level index outside the ripmap (the CPU kernel reports it)"));
    }
    Ok(())
}

// ---- self-checks --------------------------------------------------------------------------------------------------

/// How a self-check went.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrefillTally {
    /// cases compared on the device
    pub compared: usize,
    /// output words compared bit for bit
    pub words: usize,
    /// cases the CPU refuses and the device declined
    pub cpu_error_declined: usize,
}

impl std::fmt::Display for PrefillTally {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} cases, {} words bit for bit, {} CPU errors declined", self.compared, self.words, self.cpu_error_declined)
    }
}

/// Frames of the prefill corpus: two column frames, single rows, odd sizes, the recorded golden sizes, width 1 (declined).
pub const PREFILL_FRAMES: &[(usize, usize)] = &[(2, 1), (2, 2), (2, 3), (3, 1), (3, 5), (7, 2), (16, 9), (33, 2), (45, 23), (64, 11), (64, 32), (50, 7), (1, 1), (1, 3)];

const NAN_WORDS: &[u32] = &[0x7fc0_0000, 0xffc0_0000, 0x7f80_0001, 0xff80_0001, 0x7fa5_a5a5, 0xffd3_1337, 0x7fff_ffff, 0xffbf_ffff, 0x7fc0_ffee];
const SPECIAL_WORDS: &[u32] = &[0x0000_0000, 0x8000_0000, 0x7f80_0000, 0xff80_0000, 0x0000_0001, 0x8000_0001, 0x007f_ffff, 0x8080_0000, 0x7f7f_ffff, 0xff7f_ffff];

fn unit(r: &mut Rng) -> f32 {
    (r.next_u32() >> 8) as f32 / 16777216.0
}

/// One input plane of kind `kind` for a w x h frame.
fn plane_kind(r: &mut Rng, kind: usize, w: usize, h: usize) -> Vec<u32> {
    let n = w * h;
    let adv: Vec<u32> = adversarial_f32(64, r.next_u64()).iter().map(|v| v.to_bits()).collect();
    let sx = [0.001f32, 0.37, 3.0, -2.5, 1.0e-3, 117.0][r.below(6) as usize];
    let sy = [0.0f32, 0.25, -1.75, 9.5, 1.0e-2][r.below(5) as usize];
    (0..n)
        .map(|i| {
            let (x, y) = ((i % w) as f32, (i / w) as f32);
            match kind {
                // smooth ramp with a little noise
                0 => (sx * x + sy * y + 0.01 * unit(r)).to_bits(),
                // flat plane: zero gradient everywhere (a few signed zeros)
                1 => {
                    if r.below(4) == 0 {
                        0x8000_0000
                    } else if r.below(3) == 0 {
                        0
                    } else {
                        0.625f32.to_bits()
                    }
                }
                // subnormal steps: differences and magnitudes in the subnormal range
                2 => {
                    let k = (i as u32).wrapping_mul(3) % 41 + r.below(5) as u32;
                    if r.below(2) == 0 {
                        k
                    } else {
                        k | 0x8000_0000
                    }
                }
                // overflowing steps: neighbours of opposite sign near the largest finite value
                3 => {
                    let big = f32::MAX * (0.5 + 0.5 * unit(r));
                    (if (i + (i / w)) % 2 == 0 { big } else { -big }).to_bits()
                }
                // adversarial: NaN of both signs with payloads, infinities, subnormals, signed zeros in a ramp
                4 => match r.below(6) {
                    0 => NAN_WORDS[r.below(NAN_WORDS.len() as u64) as usize],
                    1 => SPECIAL_WORDS[r.below(SPECIAL_WORDS.len() as u64) as usize],
                    2 => adv[r.below(adv.len() as u64) as usize],
                    _ => (sx * x - sy * y).to_bits(),
                },
                // a sparse sprinkle of specials in a steep ramp, so most neighbourhoods stay finite
                _ => {
                    if r.below(23) == 0 {
                        if r.below(2) == 0 {
                            NAN_WORDS[r.below(NAN_WORDS.len() as u64) as usize]
                        } else {
                            SPECIAL_WORDS[r.below(SPECIAL_WORDS.len() as u64) as usize]
                        }
                    } else {
                        (x * x * 0.03125 + y * 7.0 + unit(r)).to_bits()
                    }
                }
            }
        })
        .collect()
}

/// (v, pre_a, pre_b) triples: the route's own (w / 4, 0, 0) first, then adversarial scales and offsets.
fn param_sets(w: usize, h: usize) -> Vec<(u32, u32, u32)> {
    let p = slot33_plan(w, h, 1.0, 1.0).expect("host aspect 1.0");
    vec![
        (p.v1, 0, 0),
        (p.v2, 0, 0),
        (0.25f32.to_bits(), 0, 0),
        (0x0000_0003, 0, 0),
        (0x7f00_0000, 0, 0),
        (0, 0, 0),
        (0x8000_0000, 0, 0),
        ((-1.5f32).to_bits(), 0.75f32.to_bits(), (-3.0f32).to_bits()),
        (0.5f32.to_bits(), 0x0000_0010, 0x8000_0010),
        (0x7fa0_0001, 0, 0),
        (0.5f32.to_bits(), 0xffc1_2345, 0),
        (0.5f32.to_bits(), 0, 0x7f81_0000),
        (0x7f80_0000, 0, 0),
        (2.0f32.to_bits(), 0x7f80_0000, 0xff80_0000),
    ]
}

fn first_diff(want: &[u32], got: &[u32]) -> Option<usize> {
    want.iter().zip(got).position(|(a, b)| a != b).or(if want.len() != got.len() { Some(want.len().min(got.len())) } else { None })
}

/// `fp_prefill_u32` against `prefill::run_body`, every output word bit for bit.
pub fn prefill_selfcheck(s: &Arc<Session>) -> Result<PrefillTally, String> {
    let mut r = Rng::new(0x5eed_0033);
    let mut t = PrefillTally::default();
    for &(w, h) in PREFILL_FRAMES {
        let n = w * h;
        let params: Vec<(u32, u32, u32)> = if w >= 2 { param_sets(w, h) } else { vec![(0.25f32.to_bits(), 0, 0)] };
        for kind in 0..6usize {
            for (pi, &(v, pre_a, pre_b)) in params.iter().enumerate() {
                // every kind meets the route's own scales; the adversarial scales go with a subset of the kinds
                if pi >= 2 && (kind + pi) % 3 != 0 {
                    continue;
                }
                for flag in 0..2u32 {
                    let input = plane_kind(&mut r, kind, w, h);
                    let name = format!("prefill {w}x{h} kind {kind} v 0x{v:08x} pre 0x{pre_a:08x} 0x{pre_b:08x} flag {flag}");
                    let p = Params { width: w as i32, height: h as i32, flag, pre_cell_a_bits: pre_a, pre_cell_b_bits: pre_b, v, start: 0, count: h as i32 };
                    let mut want = vec![prefill::SENTINEL; n];
                    let cpu = prefill::run_body(&p, &input, &mut want);
                    let src = s.input(&input).map_err(|e| format!("{name}: upload: {e}"))?;
                    let dst = s.output::<u32>(n).map_err(|e| format!("{name}: alloc: {e}"))?;
                    let dev = prefill_launch(s, &src, &dst, w, h, v, pre_a, pre_b, flag);
                    match (cpu, dev) {
                        (Ok(()), Ok(())) => {
                            let got = s.queue.read(&dst).map_err(|e| format!("{name}: read: {e}"))?;
                            if let Some(i) = first_diff(&want, &got) {
                                let at = |k: Option<usize>| k.and_then(|k| input.get(k)).map(|b| format!("0x{b:08x}")).unwrap_or_else(|| "-".into());
                                let (x, y) = (i % w, i / w);
                                return Err(format!(
                                    "{name}: word {i} (x {x}, y {y}) cpu 0x{:08x} device 0x{:08x}; input here {} left {} right {} up {} down {}",
                                    want[i],
                                    got.get(i).copied().unwrap_or(0),
                                    at(Some(i)),
                                    at(i.checked_sub(1)),
                                    at(Some(i + 1)),
                                    at(i.checked_sub(w)),
                                    at(Some(i + w))
                                ));
                            }
                            t.compared += 1;
                            t.words += n;
                        }
                        (Err(BodyError::Width), Err(DevError::Unsupported(_))) => t.cpu_error_declined += 1,
                        (Ok(()), Err(e)) => return Err(format!("{name}: the device declined a frame the CPU runs: {e}")),
                        (Err(e), Ok(())) => return Err(format!("{name}: the CPU body fails ({e:?}) but the device produced an output")),
                        (Err(e), Err(d)) => return Err(format!("{name}: CPU error {e:?}, device error {d}")),
                    }
                }
            }
        }
    }
    if t.compared == 0 {
        return Err("prefill: no case ran on the device".into());
    }
    Ok(t)
}

/// `fp33_ops_u32` against the public helpers of the CPU port (seven words per operand triple, NaN payloads included).
pub fn ops_selfcheck(s: &Arc<Session>) -> Result<PrefillTally, String> {
    let mut r = Rng::new(0x5eed_0034);
    let adv: Vec<u32> = adversarial_f32(400, 0x5eed_0035).iter().map(|v| v.to_bits()).collect();
    let mut pool: Vec<u32> = adv;
    pool.extend_from_slice(NAN_WORDS);
    pool.extend_from_slice(SPECIAL_WORDS);
    let pick = |r: &mut Rng, pool: &[u32]| -> u32 {
        match r.below(4) {
            0 => NAN_WORDS[r.below(NAN_WORDS.len() as u64) as usize] ^ ((r.below(2) as u32) << 31),
            _ => pool[r.below(pool.len() as u64) as usize],
        }
    };
    let n = 6000usize;
    let mut a: Vec<u32> = (0..n).map(|_| pick(&mut r, &pool)).collect();
    let mut b: Vec<u32> = (0..n).map(|_| pick(&mut r, &pool)).collect();
    let mut c: Vec<u32> = (0..n).map(|_| pick(&mut r, &pool)).collect();
    // exhaustive pairs and triples of the NaN words, so every priority position is met by two and three NaN operands
    for &x in NAN_WORDS {
        for &y in NAN_WORDS {
            for &z in &[0x3f80_0000u32, 0x7fc0_0001, 0xffa0_0002] {
                a.push(x);
                b.push(y);
                c.push(z);
            }
        }
    }
    // every pair of special words (signed zeros, infinities, extremes), so the equal operand arm of the min is met
    for &x in SPECIAL_WORDS {
        for &y in SPECIAL_WORDS {
            a.push(x);
            b.push(y);
            c.push(0x8000_0000);
        }
    }
    // inf * 0 + x and inf - inf: NaN from non NaN operands
    for &(x, y, z) in &[(0x7f80_0000u32, 0u32, 0x3f80_0000u32), (0, 0xff80_0000, 0x7f80_0000), (0x7f80_0000, 0x3f80_0000, 0x7f80_0000), (0x7f80_0000, 0x7f80_0000, 0x7f80_0000)] {
        a.push(x);
        b.push(y);
        c.push(z);
    }
    let n = a.len();
    let ab = s.input(&a).map_err(|e| e.to_string())?;
    let bb = s.input(&b).map_err(|e| e.to_string())?;
    let cb = s.input(&c).map_err(|e| e.to_string())?;
    let ob = s.output::<u32>(n * 7).map_err(|e| e.to_string())?;
    s.launch("fp33_ops_u32", n, &[ab.arg(), bb.arg(), cb.arg(), ob.arg(), ClArg::U32(n as u32)]).map_err(|e| format!("ops: {e}"))?;
    let got = s.queue.read(&ob).map_err(|e| e.to_string())?;
    const NAMES: [&str; 7] = ["add_ss", "sub_ss", "mul_ss", "min_ss", "fma213", "fma213 sub", "fmsub231"];
    for i in 0..n {
        let (x, y, z) = (a[i], b[i], c[i]);
        let want = [add_ss(x, y), sub_ss(x, y), mul_ss(x, y), min_ss(x, y), fma213(x, y, z, false), fma213(x, y, z, true), fmsub231(x, y, z)];
        for k in 0..7 {
            if got[i * 7 + k] != want[k] {
                return Err(format!("ops {}: operands 0x{x:08x} 0x{y:08x} 0x{z:08x} cpu 0x{:08x} device 0x{:08x}", NAMES[k], want[k], got[i * 7 + k]));
            }
        }
    }
    Ok(PrefillTally { compared: n, words: n * 7, cpu_error_declined: 0 })
}

/// Frames of the route check, including width 1 (the CPU errors when the flag is set).
pub const ROUTE_FRAMES: &[(usize, usize)] = &[(2, 1), (2, 3), (3, 5), (7, 2), (16, 9), (33, 2), (45, 23), (64, 11), (64, 32), (50, 7), (1, 3), (1, 1)];

/// Filter lanes of the route check: sum (1), min on c1 (2), min on both (3, 7), negative, NaN (word INT_MIN), fractional (no
/// prefill), zero, and a value out of the i32 range.
pub const ROUTE_FILTERS: &[f32] = &[1.0, 2.0, 3.0, 7.0, -1.0, f32::NAN, 0.5, -0.75, 0.0, 3.0e9];

/// [`slot33_run`] against `warp_log_gradient::slot33_warp` (the CPU route of `warp_k` and `warp_rel_k`): flag byte, and c1, c2 bit
/// for bit when the prefill runs.
pub fn route_selfcheck(s: &Arc<Session>) -> Result<PrefillTally, String> {
    let mut r = Rng::new(0x5eed_0036);
    let mut t = PrefillTally::default();
    for &(w, h) in ROUTE_FRAMES {
        let n = w * h;
        for (fi, &filter) in ROUTE_FILTERS.iter().enumerate() {
            for &aspect in &[1.0f32, 0.75] {
                if aspect != 1.0 && fi % 3 != 0 {
                    continue;
                }
                let kind_u = (fi + w) % 6;
                let kind_v = (fi + h + 3) % 6;
                let u = plane_kind(&mut r, kind_u, w, h);
                let v = plane_kind(&mut r, kind_v, w, h);
                let name = format!("route {w}x{h} filter {filter} aspect {aspect} kinds {kind_u} {kind_v}");
                // the CPU route as warp_k runs it
                let cpu = if aspect != 1.0 {
                    Err("host aspect".to_string())
                } else {
                    let prim = Prim33 { n: cvt_i32(filter), cc: 0 };
                    let rec = slot33_record(w, h);
                    let mut c1 = vec![warp_log_gradient::SENT; n];
                    let mut c2 = vec![warp_log_gradient::SENT; n];
                    let mut pool = warp_log_gradient::Pool::default();
                    let th = warp_log_gradient::Threads { nthr: 1, nthr2: 1, tid: 0 };
                    let mut ev = Vec::new();
                    warp_log_gradient::slot33_warp(warp_log_gradient::FW_AVX2, &prim, Some(&rec), None, (&u, &v), &mut c1, &mut c2, &mut pool, &th, &mut ev)
                        .map(|o| (o.flag_byte, c1, c2))
                        .map_err(|e| format!("{e:?}"))
                };
                let uf: Vec<f32> = u.iter().map(|b| f32::from_bits(*b)).collect();
                let vf: Vec<f32> = v.iter().map(|b| f32::from_bits(*b)).collect();
                let coords = DevImage::upload(s, &[uf.as_slice(), vf.as_slice()]).map_err(|e| format!("{name}: upload: {e}"))?;
                let dev = slot33_run(s, w, h, filter, aspect, coords.plane(0), coords.plane(1));
                match (cpu, dev) {
                    (Ok((fb, c1, c2)), Ok(d)) => {
                        if fb != d.plan.flag_byte {
                            return Err(format!("{name}: flag byte cpu {fb} device {}", d.plan.flag_byte));
                        }
                        match d.c {
                            None => {
                                if c1.iter().chain(&c2).any(|b| *b != warp_log_gradient::SENT) {
                                    return Err(format!("{name}: the device skipped a prefill the CPU ran"));
                                }
                            }
                            Some(img) => {
                                let got = img.download().map_err(|e| format!("{name}: {e}"))?;
                                for (p, want) in [c1, c2].iter().enumerate() {
                                    let g: Vec<u32> = got[p].iter().map(|f| f.to_bits()).collect();
                                    if let Some(i) = first_diff(want, &g) {
                                        return Err(format!("{name}: c{} word {i} cpu 0x{:08x} device 0x{:08x}", p + 1, want[i], g.get(i).copied().unwrap_or(0)));
                                    }
                                    t.words += n;
                                }
                            }
                        }
                        t.compared += 1;
                    }
                    (Err(_), Err(DevError::Unsupported(_))) => t.cpu_error_declined += 1,
                    (Ok(_), Err(e)) => return Err(format!("{name}: the device declined a route the CPU runs: {e}")),
                    (Err(e), Ok(_)) => return Err(format!("{name}: the CPU route fails ({e}) but the device produced an output")),
                    (Err(e), Err(d)) => return Err(format!("{name}: CPU error {e}, device error {d}")),
                }
            }
        }
    }
    if t.words == 0 {
        return Err("route: no prefill ran on the device".into());
    }
    Ok(t)
}

/// Single edit mutants of `FOOTPRINT_PREFILL` (name, text replaced once, replacement): each must make one of the three
/// self-checks fail when built in place of the real source.
pub const MUTANTS: &[(&str, &str, &str)] = &[
    ("hardware_nan_fma213", "    if (fp33_isn(dst)) {\n        return dst | 0x400000u;\n    }\n    if (fp33_isn(s2)) {\n        return s2 | 0x400000u;\n    }\n    if (fp33_isn(s3)) {\n        return s3 | 0x400000u;\n    }\n    float c = sub ? -as_float(s3) : as_float(s3);\n    return fp33_fix(fma(as_float(s2), as_float(dst), c));", "    float c = sub ? -as_float(s3) : as_float(s3);\n    return as_uint(fma(as_float(s2), as_float(dst), c));"),
    ("nan_priority_swap_fma213", "    if (fp33_isn(dst)) {\n        return dst | 0x400000u;\n    }\n    if (fp33_isn(s2)) {\n        return s2 | 0x400000u;\n    }\n    if (fp33_isn(s3)) {\n        return s3 | 0x400000u;\n    }\n    float c", "    if (fp33_isn(s2)) {\n        return s2 | 0x400000u;\n    }\n    if (fp33_isn(dst)) {\n        return dst | 0x400000u;\n    }\n    if (fp33_isn(s3)) {\n        return s3 | 0x400000u;\n    }\n    float c"),
    ("variant_swap", "        if (flag != 0) {\n            uint mx", "        if (flag == 0) {\n            uint mx"),
    ("edge_row_scale_one", "uint s = (y == 0 || y == h - 1) ? 0x40000000u : 0x3f800000u;", "uint s = (y == 0) ? 0x40000000u : 0x3f800000u;"),
    ("last_column_unfused", "uint nv = fp33_fma213(sy, dn, hb8, 1) & 0x7fffffffu;", "uint nv = fp33_sub(fp33_mul(sy, dn), hb8) & 0x7fffffffu;"),
    ("finish_single", "uint m32 = fp33_cvtsd2ss(r);", "uint m32 = fp33_fix((float)as_double(xa) + (float)as_double(d) * 0x1p-1f);"),
    ("finish_compare_lt", "if (as_double(xa) > as_double(d)) {", "if (as_double(xa) < as_double(d)) {"),
    ("exponent_bias_127", "int e = (int)((m32 >> 23) & 0xffu) - 128;", "int e = (int)((m32 >> 23) & 0xffu) - 127;"),
    ("operand_swap_dl", "uint dl = fp33_sub(cx, cp);", "uint dl = fp33_sub(cp, cx);"),
    ("sum_order_finish", "r = fp33_fma213_sd(xa, 0x3fe0000000000000ul, d);", "r = fp33_fma213_sd(d, 0x3fe0000000000000ul, xa);"),
    ("select_arm_prev", "__global const uint* prev = (y == 0) ? cur : cur - w;", "__global const uint* prev = (y == 0) ? cur + w : cur - w;"),
    ("constant_two_v", "uint two_v = fp33_mul(0x40000000u, v);", "uint two_v = fp33_mul(0x3f800000u, v);"),
    ("finish_zero_test", "if (!(as_float(m32) > 0x0p+0f)) {", "if (as_float(m32) < 0x0p+0f) {"),
    ("min_equal_arm", "    return (as_float(a) < as_float(b)) ? a : b;", "    return (as_float(a) <= as_float(b)) ? a : b;"),
];

/// Single edit mutants that no self-check can refuse on an x86 OpenCL device, with the reason. They are built and run like
/// [`MUTANTS`] and reported, not required to fail.
/// * `hardware_nan_sub`: the explicit NaN tests of `fp33_sub` reproduce the x86 rule (first NaN operand quieted, default NaN
///   0xffc00000), which is also the NaN rule of `subss` on x86; the tests matter on a device with another NaN rule.
/// * `finish_compare_ge`: at `xa == d` both arms compute the same value (`d * 0.5 + xa` and `0.5 * xa + d` with `xa == d`, and
///   xa and d are never -0.0, being magnitudes or sums of magnitudes).
pub const EQUIVALENT_MUTANTS: &[(&str, &str, &str)] = &[
    ("hardware_nan_sub", "uint fp33_sub(uint a, uint b) {\n    if (fp33_isn(a)) {\n        return a | 0x400000u;\n    }\n    if (fp33_isn(b)) {\n        return b | 0x400000u;\n    }\n    return fp33_fix(as_float(a) - as_float(b));", "uint fp33_sub(uint a, uint b) {\n    return as_uint(as_float(a) - as_float(b));"),
    ("finish_compare_ge", "if (as_double(xa) > as_double(d)) {", "if (as_double(xa) >= as_double(d)) {"),
];

// ---- driver self-check: the footprint branch and the sampler with chosen prefill maps -------------------------------

/// Frames of the driver check (no slot 33 here, so width 1 runs too).
pub const DRIVER_FRAMES: &[(usize, usize)] = &[(1, 1), (1, 3), (2, 1), (2, 3), (3, 5), (7, 2), (16, 9), (33, 2), (45, 23), (64, 11), (64, 32), (50, 7)];

/// The CPU driver for one frame with the given prefill maps: `warp_render::slot53_warp` (`warp_driver::run`) for `warp`,
/// `warp_rel::render` (`warp_rel_sampler::run`) for `warp_rel`, row by row with the record fields of `kernels2::warp_k` and
/// `kernels2::warp_rel_k`, flag byte 1. None when the CPU reports an error.
#[allow(clippy::too_many_arguments)]
fn cpu_driver(rel: bool, mip: &crate::types::MipImage, w: usize, h: usize, kp: &crate::types::KParams, a: (&[f32], &[f32]), c: (&[f32], &[f32]), mx: i32, my: i32) -> Option<Vec<Vec<f32>>> {
    let ch = mip.channels;
    let g = crate::kernels2::geom_of(kp, w, h);
    let (nl, np) = (mip.ripmap.nl(), mip.ripmap.np());
    let mk = || warp_sampler::Params {
        n: ch as i32,
        width_tab: mip.ripmap.widths.iter().map(|&v| v as i32).collect(),
        height_tab: mip.ripmap.heights.iter().map(|&v| v as i32).collect(),
        t: mip.scale.as_ref().map(|s| s.iter().map(|b| f32::from_bits(*b)).collect()).unwrap_or_default(),
        flag_x: 0,
        flag_y: 0,
    };
    let lat = warp_sampler::Lattice {
        bufs: (0..nl).map(|l| (0..np).map(|j| mip.ripmap.planes[mip.ripmap.idx(l, j)].iter().map(|b| f32::from_bits(*b)).collect()).collect()).collect(),
    };
    let mut out: Vec<Vec<f32>> = vec![vec![0.0; w * h]; ch];
    for y in 0..h {
        let mut row: Vec<Vec<f32>> = vec![vec![0.0; w]; ch];
        if rel {
            let prim = warp_rel::Prim { const_coords: false, const_a1: 0.0, const_a2: 0.0, a1_arr: a.0.to_vec(), a2_arr: a.1.to_vec(), c1_arr: c.0.to_vec(), c2_arr: c.1.to_vec(), flag: 1, mx, my };
            let mut img = warp_rel::Image { f1: mk(), w: w as i32, h: h as i32, d_cell_d: nl as i32, d_cell_e: np as i32, host: warp_rel::Host::Mode0 };
            let fr = crate::kernels2::warp_rel_row(&g, w, y, ch);
            warp_rel::render(&prim, &fr, &mut img, &lat, &mut row).ok()?;
        } else {
            let prim = warp_render::WarpPrim { const_coords: false, const_a1: 0.0, const_a2: 0.0, a1_arr: a.0.to_vec(), a2_arr: a.1.to_vec(), c1_arr: c.0.to_vec(), c2_arr: c.1.to_vec(), flag: 1, mx, my };
            let mut img = warp_render::DriverImage { f1: mk(), w: w as i32, h: h as i32, d_cell_d: nl as i32, d_cell_e: np as i32 };
            let rec = crate::kernels2::warp_row_rec(&g, w, y);
            warp_render::slot53_warp(&prim, &rec, None, &mut img, &lat, &mut row).ok()?;
        }
        for k in 0..ch {
            out[k][y * w..(y + 1) * w].copy_from_slice(&row[k]);
        }
    }
    Some(out)
}

/// A prefill map for the driver check. kind 0: fractions in (0, levels); 1: zeros of both signs, negatives and values above the
/// level count; 2: NaN of both signs, infinities, subnormals and huge values among fractions; 3: the same value in both maps
/// (a == b); 4: values exactly on integers and on the clamp bound. The driver check adds kind 5 (coordinates near the top left
/// corner with a > b) and kind 6 (an all -0.0 source with c2 = -0.0).
fn c_plane(r: &mut Rng, kind: usize, n: usize, levels: usize, twin: Option<&[f32]>) -> Vec<f32> {
    let lv = levels as f32;
    let sp = adversarial_f32(30, r.next_u64());
    (0..n)
        .map(|i| match kind {
            0 => unit(r) * lv,
            1 => match r.below(5) {
                0 => 0.0,
                1 => -0.0,
                2 => -unit(r) * 4.0,
                3 => lv + unit(r) * 9.0,
                _ => unit(r) * lv,
            },
            2 => {
                if r.below(4) == 0 {
                    let w = NAN_WORDS.iter().chain(SPECIAL_WORDS).copied().collect::<Vec<u32>>();
                    f32::from_bits(w[r.next_u32() as usize % w.len()])
                } else if r.below(6) == 0 {
                    sp[r.next_u32() as usize % sp.len()]
                } else {
                    unit(r) * lv
                }
            }
            3 => twin.map(|t| t[i]).unwrap_or_else(|| unit(r) * lv),
            _ => match r.below(4) {
                0 => r.below(levels as u64 + 1) as f32,
                1 => (levels as f64 - 1.01) as f32,
                2 => f32::from_bits(((levels as f64 - 1.01) as f32).to_bits() + 1),
                _ => unit(r) * lv,
            },
        })
        .collect()
}

/// The device footprint branch and F1 sampler (`fp_warp_f32` through [`fp_warp_launch`]) against the CPU driver of the class
/// (`warp_driver::run` for warp, `warp_rel_sampler::run` for warp_rel) with chosen prefill maps, so every branch of the driver
/// (c1 or c2 positive, zero, negative, NaN; a > b, b > a, a == b; clamp_dir at its bound) is reached independently of the
/// prefill. Multi level ripmaps of make_mipmap at filter 1 (the host one uploaded and the device resident one, alternately),
/// every wrap mode, 1 to 4 channels. A NaN output word counts as
/// equal to a NaN (the sampler uses the hardware's NaN rule, like the CPU; a NaN output reruns on the CPU in the executor).
pub fn driver_selfcheck(s: &Arc<Session>, rel: bool) -> Result<PrefillTally, String> {
    use crate::types::{Arg, Image, KParams};
    let class = if rel { "warp_rel" } else { "warp" };
    let mut t = PrefillTally::default();
    let mut r = Rng::new(if rel { 0xd41f_0001 } else { 0xd41f_0002 });
    let modes: &[(i32, i32)] = if rel { &[(0, 0), (1, 1), (2, 2), (1, 2), (2, 0), (3, 3)] } else { &[(0, 0), (1, 1), (2, 2), (1, 2), (2, 0)] };
    let mut k = 0usize;
    for &(w, h) in DRIVER_FRAMES {
        let n = w * h;
        for ch in 1..=4usize {
            for &(mx, my) in modes {
                for kind in 0..7usize {
                    k += 1;
                    let mut kp = KParams::standin();
                    if kind == 5 {
                        // unit scale and no offset: the coordinate planes give x and y directly near the top left corner
                        kp.geom = Some(crate::types::Geom { left: -1.0, right: 1.0, aspect: 1.0, assumption_e: 0.0, assumption_h: 0.0, roi_x: 0, roi_y: 0 });
                    } else if k % 3 == 1 {
                        kp.geom = Some(crate::types::Geom { left: -0.5, right: w as f32 - 0.5, aspect: 1.0, assumption_e: 0.25, assumption_h: 0.75, roi_x: 3, roi_y: -2 });
                    }
                    let src = Image { n, planes: (0..ch).map(|_| (0..n).map(|_| if kind == 6 { -0.0 } else { unit(&mut r) * 2.0 - 0.5 }).collect()).collect() };
                    let wrap = [mx.clamp(0, 2) as f32, my.clamp(0, 2) as f32, 0.0, 0.0];
                    let margs = [Arg::Img(&src), Arg::Const { n: 2, lanes: [wrap[0].to_bits(), wrap[1].to_bits(), 0, 0] }, Arg::Const { n: 1, lanes: [1.0f32.to_bits(), 0, 0, 0] }];
                    let mip = match crate::kernels::run_any("make_mipmap", ch as i32, &margs, n, (w, h), &kp) {
                        Ok(crate::kernels2::Out::Mip(m)) => m,
                        _ => continue,
                    };
                    let (nl, np) = (mip.ripmap.nl(), mip.ripmap.np());
                    let spread = if kind == 1 { 6.0 } else { 1.2 };
                    let mut a1: Vec<f32> = (0..n).map(|_| (unit(&mut r) * 2.0 - 1.0) * spread).collect();
                    let mut a2: Vec<f32> = (0..n).map(|_| (unit(&mut r) * 2.0 - 1.0) * spread).collect();
                    let mut c1 = c_plane(&mut r, kind.min(4), n, nl, None);
                    let mut c2 = c_plane(&mut r, kind.min(4), n, np, if kind == 3 { Some(&c1) } else { None });
                    if kind == 5 {
                        // x and y in (-0.9, 0.4): sampler fractions below 1 with every mantissa bit; a > b in most pixels
                        for i in 0..n {
                            let (xi, yi) = ((i % w) as f32, (i / w) as f32);
                            let (tx, ty) = (unit(&mut r) * 1.3 - 0.9, unit(&mut r) * 1.3 - 0.9);
                            if rel {
                                a1[i] = tx - xi;
                                a2[i] = yi - ty;
                            } else {
                                a1[i] = tx;
                                a2[i] = 1.0 - ty;
                            }
                            let lo = (nl - 1).max(1) as f32;
                            c1[i] = (r.below((nl as u64).saturating_sub(1).max(1)) as f32) + 0.5 + unit(&mut r) * 0.49;
                            c2[i] = (r.below((np as u64).saturating_sub(1).max(1)) as f32) + unit(&mut r) * 0.5;
                            if c1[i] >= lo - 0.01 {
                                c1[i] = lo - 0.5;
                            }
                        }
                    }
                    if kind == 6 {
                        // a negative zero c2 with a positive c1: b is -0.0, which only the sign of a zero product shows
                        for v in c2.iter_mut() {
                            *v = -0.0;
                        }
                        for v in c1.iter_mut() {
                            *v = 0.5 + unit(&mut r) * 0.4;
                        }
                    }
                    let want = cpu_driver(rel, &mip, w, h, &kp, (&a1, &a2), (&c1, &c2), mx, my);
                    let name = format!("{class} driver {w}x{h} ch{ch} mode({mx},{my}) kind{kind}");
                    let dev = (|| -> Result<Vec<Vec<f32>>, DevError> {
                        let coords = DevImage::upload(s, &[a1.as_slice(), a2.as_slice()])?;
                        let pre = DevImage::upload(s, &[c1.as_slice(), c2.as_slice()])?;
                        // even cases: the device resident ripmap of make_mipmap (its own buffer and offsets); odd: the host one uploaded
                        let resident = if k % 2 == 0 {
                            let sd = DevImage::upload(s, &src.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>())?;
                            let call = crate::backend::NodeCall { idx: 0, class: "make_mipmap", tag: ch as i32, n, dims: (w, h), kp: &kp };
                            let dargs = [crate::device_ops::DevArg::Img(&sd), crate::device_ops::DevArg::Const { n: 2, lanes: [wrap[0].to_bits(), wrap[1].to_bits(), 0, 0] }, crate::device_ops::DevArg::Const { n: 1, lanes: [1.0f32.to_bits(), 0, 0, 0] }];
                            let out = crate::device_ops_ripmap::make_mipmap_run(s, &call, &dargs)?;
                            Some(crate::device_ops_ripmap::wrap_ripmap(&call, &dargs, out)?)
                        } else {
                            None
                        };
                        let mref = match &resident {
                            Some(d) => crate::device_ops_warp::MipRef::Dev(d),
                            None => crate::device_ops_warp::MipRef::Host(&mip),
                        };
                        let lat = fp_lattice(s, &mref)?;
                        let out = DevImage::alloc(s, ch, n)?;
                        let g = crate::kernels2::geom_of(&kp, w, h);
                        fp_warp_launch(s, rel, w, h, ch, &lat, &coords, &pre, mx, my, &g, &out)?;
                        Ok(out.download()?)
                    })();
                    match (want, dev) {
                        (Some(wp), Ok(gp)) => {
                            for (p, (wv, gv)) in wp.iter().zip(&gp).enumerate() {
                                for (i, (x, y)) in wv.iter().zip(gv).enumerate() {
                                    if x.to_bits() != y.to_bits() && !(x.is_nan() && y.is_nan()) {
                                        return Err(format!("{name} plane {p} sample {i}: cpu {:#010x} device {:#010x} (c1 {:#010x} c2 {:#010x})", x.to_bits(), y.to_bits(), c1[i].to_bits(), c2[i].to_bits()));
                                    }
                                }
                                t.words += wv.len();
                            }
                            t.compared += 1;
                        }
                        (Some(_), Err(e)) => return Err(format!("{name}: the device declined a frame the CPU runs: {e}")),
                        (None, Ok(_)) => return Err(format!("{name}: the CPU driver fails but the device produced an output")),
                        (None, Err(DevError::Unsupported(_))) => t.cpu_error_declined += 1,
                        (None, Err(e)) => return Err(format!("{name}: {e}")),
                    }
                }
            }
        }
    }
    if t.compared == 0 {
        return Err(format!("{class} driver: no frame ran on the device"));
    }
    Ok(t)
}

/// Single edit mutants of `FOOTPRINT_WARP` (name, text replaced once, replacement): each must make the warp or the warp_rel
/// self-check fail when built in place of the real source.
pub const WARP_MUTANTS: &[(&str, &str, &str)] = &[
    ("clamp_dir_one", "double d = (double)dim - as_double(0x3ff028f5c28f5c29ul);", "double d = (double)dim - 0x1p+0;"),
    ("c2pos_no_nan", "int c2pos = (0x0p+0f < cv2) || isnan(cv2);", "int c2pos = (0x0p+0f < cv2);"),
    ("q_strict_compare", "if ((0x0p+0f <= cv2) || isnan(cv2)) {", "if ((0x0p+0f < cv2) || isnan(cv2)) {"),
    ("t_index_j_for_x", "float t6 = fma(sx, as_float(ttab[il]), -0x1p-1f);", "float t6 = fma(sx, as_float(ttab[jl]), -0x1p-1f);"),
    ("clamp_pair_always", "if (fx == 0) {", "if (fx >= 0) {"),
    ("f_branch_plane_ij1", "uint pf = (uint)((il + 1) * (long)np + jl);", "uint pf = (uint)(il * (long)np + jl + 1);"),
    ("nest_v_form", "float p5 = fma(w1, s1, w3 * s3);\n            float p6 = fma(w0, s0, w2 * s2);\n            t[e] = p5 + p6;", "t[e] = fma(w2, s2, fma(w3, s3, fma(w0, s0, w1 * s1)));"),
    ("blend_weight_a", "float w1 = 0x1p+0f - a;", "float w1 = a;"),
    ("operand_swap_fa", "float fa = p - (float)ip;", "float fa = (float)ip - p;"),
    ("lerp_unfused", "fma(t2[k] - t1[k], a, t1[k])", "(t2[k] - t1[k]) * a + t1[k]"),
    ("nest_weight_sum_order", "w0 = ((0x1p+0f - fn) - fs) + tt;", "w0 = (0x1p+0f - fn) - (fs - tt);"),
    ("select_arm_negative_c1", "p = 0x0p+0f;", "p = fpw_clamp_dir(cv1, nl);"),
    ("compare_direction_ab", "if (a > b) {", "if (a < b) {"),
    ("constant_half_x", "float sx = 0x1p-1f + x;", "float sx = 0x1p+0f + x;"),
    ("level_count_q", "q = fpw_clamp_dir(cv2, np);", "q = fpw_clamp_dir(cv2, nl);"),
    ("stride_plus_two", "int st1 = (int)(((uint)wtab[il] + 3u) * (uint)n);", "int st1 = (int)(((uint)wtab[il] + 2u) * (uint)n);"),
];

/// Single edit mutants of `FOOTPRINT_WARP` that no input can separate from the real source, with the reason. They are built and
/// run like [`WARP_MUTANTS`] and reported, not required to fail.
/// * `f_nest_first_is_stride_off`: the weight `((1 - fs) - fn) + t` against `((1 - fn) - fs) + t`. Every sampler fraction is
///   `s - trunc(s)` of `s = 1 + t` in single precision, so it is a multiple of 2^-24, and of 2^-23 when negative; then `1 - fs`
///   and `1 - fn` are exact, and both orders round the same exact value `1 - fs - fn` once. NaN operands give a NaN either way.
pub const WARP_EQUIVALENT_MUTANTS: &[(&str, &str, &str)] = &[("f_nest_first_is_stride_off", "fxb, fya, 1, t3)", "fxb, fya, 0, t3)")];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_footprint_plan_matches_the_cpu_route_scalars() {
        // v1 = (w - 0.5 - -0.5) * 0.25 = w / 4 and v2 = (geom33_a - assumption_h) * 0.25 = w / 4 at aspect 1.0
        for &(w, h) in &[(2usize, 1usize), (64, 32), (50, 7), (1920, 1080)] {
            let p = slot33_plan(w, h, 1.0, 1.0).unwrap();
            assert_eq!(p.flag_byte, 1);
            assert_eq!((p.min1, p.min2), (0, 0));
            assert_eq!(p.v1, (w as f32 * 0.25).to_bits());
            assert_eq!(p.v2, (w as f32 * 0.25).to_bits());
        }
        assert_eq!(slot33_plan(8, 8, 2.0, 1.0).unwrap().min1, 1);
        assert_eq!(slot33_plan(8, 8, 2.0, 1.0).unwrap().min2, 0);
        assert_eq!(slot33_plan(8, 8, 3.0, 1.0).unwrap().min2, 1);
        assert_eq!(slot33_plan(8, 8, 0.5, 1.0).unwrap().flag_byte, 0);
        assert_eq!(slot33_plan(8, 8, f32::NAN, 1.0).unwrap(), Slot33Plan { flag_byte: 1, min1: 0, min2: 0, ..slot33_plan(8, 8, 1.0, 1.0).unwrap() });
        assert!(slot33_plan(8, 8, 1.0, 0.75).is_err());
    }

    #[test]
    fn device_footprint_dim_eps_literal_is_the_cpu_constant() {
        // fpw_clamp_dir subtracts as_double(0x3ff028f5c28f5c29), the bits of warp_driver::DIM_EPS and warp_rel_sampler::DIM_EPS
        let src = fx_device::kernels_footprint::FOOTPRINT_WARP;
        assert!(src.contains("as_double(0x3ff028f5c28f5c29ul)"));
        assert_eq!(1.01f64.to_bits(), 0x3ff028f5c28f5c29);
    }

    #[test]
    fn device_footprint_warp_mutants_apply_once_and_differ() {
        let src = fx_device::kernels_footprint::FOOTPRINT_WARP;
        let mut seen = vec![src.to_string()];
        for (name, from, to) in WARP_MUTANTS.iter().chain(WARP_EQUIVALENT_MUTANTS) {
            assert_eq!(src.matches(from).count(), 1, "mutant {name}: the edited text occurs once");
            let m = src.replacen(from, to, 1);
            assert!(!seen.contains(&m), "mutant {name} equals the base or an earlier mutant");
            seen.push(m);
        }
    }

    #[test]
    fn device_footprint_mutants_apply_once_and_differ() {
        let src = fx_device::kernels_footprint::FOOTPRINT_PREFILL;
        let mut seen = vec![src.to_string()];
        for (name, from, to) in MUTANTS.iter().chain(EQUIVALENT_MUTANTS) {
            assert_eq!(src.matches(from).count(), 1, "mutant {name}: the edited text occurs once");
            let m = src.replacen(from, to, 1);
            assert!(!seen.contains(&m), "mutant {name} equals the base or an earlier mutant");
            seen.push(m);
        }
    }
}
