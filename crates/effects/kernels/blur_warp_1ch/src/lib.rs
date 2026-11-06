//! blur_warp_bidirectional body A (mode 1), port piece 1:
//! skeleton (outer loop), S0..S5 and the zero-length P1 stub .
//! Not ported: S6..S12 sampling (BW5B, BW5C). The port stops at the end of S5,
//! exactly where the T2 harness has a fault hook. VEX follows the SDM, not silicon (engine statement).
//! Source of truth: the body A listing. Names INFERRED, offsets and arithmetic MEASURED from the listing.

use libm_fmod::fmod_x;
pub mod p2;

/// One call the original makes, in order. Pointer arguments are implied (see assumptions B4).
#[derive(Clone, Debug, PartialEq)]
pub enum Rec {
    /// (rdi = stack scratch, rsi = this): writes 16 constant bytes to [rdi]
    ScratchCellA,
    /// (xmm0 = x as f64 bits, xmm1 = y as f64 bits)
    Fmod { x: u64, y: u64 },
    /// (rdi = image, esi, edx)
    ImgCellA { esi: u32, edx: u32 },
    /// (rdi = image, esi)
    ImgCellB { esi: u32 },
    /// (rdi = out object, esi = sample, rdx = image, ecx = mode x, r8d = mode y, xmm0 = x, xmm1 = y as f32 bits)
    P1 { esi: u32, ecx: u32, r8: u32, x: u32, y: u32 },
}

/// this-object fields (offsets MEASURED) and image fields.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// this, sample count (signed compare)
    pub count: i32,
    /// this
    pub aspect_block_d: f32,
    /// this
    pub assumption_e: f32,
    /// this (int, converted to f32)
    pub assumption_c: i32,
    /// this (int, converted to f32)
    pub assumption_d: i32,
    /// this
    pub assumption_f: f32,
    /// this
    pub assumption_g: f32,
    /// this
    pub assumption_h: f32,
    /// image object
    pub img_w: i32,
    /// image object
    pub img_h: i32,
}

/// Call arguments: five xmm floats, arrays and six stack arguments.
#[derive(Clone, Debug)]
pub struct Inputs {
    /// xmm0..xmm4 at entry (spilled to rbp, -0xe0, -, -0xe8, -)
    pub a: [f32; 5],
    /// rcx/r8 arrays (index = sample)
    pub p1x: Vec<f32>,
    pub p1y: Vec<f32>,
    /// stack arrays
    pub p2x: Vec<f32>,
    pub p2y: Vec<f32>,
    /// r9b: p1 is constant (arrays only read at index 0)
    pub p1_const: bool,
    /// stack byte: p2 is constant
    pub p2_const: bool,
    /// stack (consumed by S6, carried)
    pub flag_cell_a: u32,
    /// stack edge modes (0 none, 1 wrap, 2 mirror)
    pub mode_x: i32,
    pub mode_y: i32,
    /// stand-ins for (data pointer) and (stride), see assumptions B2
    pub acc_ptr: u64,
    pub acc_stride: i32,
}

/// Optional numeric environment for the P1 stub.
pub struct P1Env {
    pub p: blur_warp_point::Params,
    pub g: blur_warp_point::Geometry,
    pub src: Vec<f32>,
    /// output planes; plane 0 also receives out[i] = a4 * out[i]
    pub out: Vec<Vec<f32>>,
}

/// State at the end of S5 (hand-off to S6), for the first non-degenerate sample.
#[derive(Clone, Debug)]
pub struct S5State {
    pub sample: usize,
    pub x: f32,
    pub y: f32,
    pub p1x: f32,
    pub p1y: f32,
    pub p2x: f32,
    pub p2y: f32,
    pub e2x: f32,
    pub e2y: f32,
    pub dx: f32,
    pub dy: f32,
    pub adx: f32,
    pub ady: f32,
    /// vcomiss adx, ady; jb: true goes to the Y-major sections 
    pub y_major: bool,
    pub data_ptr: u64,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub scx: f32,
    pub scy: f32,
    pub k_a1_a0: f32,
    pub k_a2_a1: f32,
    pub k_1_a3: f32,
    pub scale: f32,
    pub flag_cell_a: u32,
    pub mode_x: i32,
    pub mode_y: i32,
}

#[derive(Debug)]
pub struct Output {
    pub recs: Vec<Rec>,
    /// indexes of samples that took the zero-length P1 stub
    pub stub_samples: Vec<usize>,
    /// Some when a non-degenerate sample reached the end of S5 (the port stops there)
    pub s5: Option<S5State>,
}

#[derive(Debug, PartialEq)]
pub enum Unsupported {
    /// idiv by zero or i32::MIN / -1 in the mirror test (the original raises #DE)
    IdivFault,
    /// Y-major sample (S9/S10, BW5C)
    YMajor,
    /// source index outside the modelled plane buffer
    Oob,
    /// zero-length segment needs the P1 stub numerics (plane mode)
    P1Needed,
}

pub struct Assumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "B1", basis: " (19 B) writes 16 constant bytes from (captured) to [rdi]; rdi is the stack local rbp, never read in S0..S5; treated as a no-op with a recorded call", label: "MEASURED listing, no-op INFERRED" },
        Assumption { id: "B2", basis: " return values (data pointer, stride) are explicit inputs; S5 only stores them (rbp, r8d); not used before S6", label: "ASSUMPTION" },
        Assumption { id: "B3", basis: "mirror mode 2 with a zero dimension raises #DE (idiv) in the original; the port returns Unsupported::IdivFault; the zero-dimension stubs are unreachable", label: "INFERRED" },
        Assumption { id: "B4", basis: "pointer call arguments (rdi, rdx) are not compared by value; T2 checks them by region identity (image, out object, this)", label: "ASSUMPTION" },
        Assumption { id: "B5", basis: "T2 fmod hook returns Python math.fmod semantics (exact IEEE, special cases mapped), equal to libm_fmod on finite inputs (libm T2 0 diffs); inf/NaN inputs compared as class", label: "ASSUMPTION" },
        Assumption { id: "B6", basis: "T2 stops at the first non-degenerate sample (fault hook); later samples are not compared. Planes are NOT compared in this piece", label: "ASSUMPTION" },
        Assumption { id: "B8", basis: "plane-mode data pointer: source buffer with origin offset; reads outside the buffer return Unsupported::Oob (the emulator would read padding); T2 keeps indices inside it", label: "ASSUMPTION" },
        Assumption { id: "B9", basis: "host statics ..2b (edge flag bytes) and (float hs) are explicit inputs; in T2 hs=height, bytes random 0/1", label: "ASSUMPTION" },
        Assumption { id: "B10", basis: "X-major path has no P1 call; zero-length samples need the stub numerics (Unsupported::P1Needed in plane mode, T2 cases avoid them); the 16-byte constant is zero (lookup_out.tsv)", label: "MEASURED" },
        Assumption { id: "B11", basis: "FMA sites of S6..S8: (fnmadd231), (fmsub213), (fmadd213), (fmadd213), (fmsub213), (fmadd213); modelled with mul_add; no rcp/rsqrt/x87 in S6..S8", label: "MEASURED listing" },
        Assumption { id: "B7", basis: "FMA sites in S0..S5: (vfmadd132ss), (vfmsub213ss), (vfmadd132ss), (vfnmadd132ss); no rcp/rsqrt, no x87 in S0..S5", label: "MEASURED listing" },
    ]
}

/// vcvttss2si r32
pub fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

fn idiv_q(a: i32, d: i32) -> Result<i32, Unsupported> {
    if d == 0 || (a == i32::MIN && d == -1) {
        return Err(Unsupported::IdivFault);
    }
    Ok(a / d)
}

/// fmod call plus the negative-value add of fabs(dim): 
fn fm(recs: &mut Vec<Rec>, v: f32, dim: i32) -> f32 {
    let wf = dim as f32;
    let neg = 0.0f32 > v; // vcomiss 0,v ; jbe skips when 0<=v or unordered
    recs.push(Rec::Fmod { x: (v as f64).to_bits(), y: (wf as f64).to_bits() });
    let r = fmod_x(v as f64, wf as f64);
    let r = if neg { (wf.abs() as f64) + r } else { r };
    r as f32
}

/// edge wrap of one coordinate (S2 for x, S3 for y)
pub fn wrap(recs: &mut Vec<Rec>, v: f32, dim: i32, mode: i32) -> Result<f32, Unsupported> {
    match mode {
        1 => {
            if dim == 0 {
                return Ok(0.0);
            }
            Ok(fm(recs, v, dim))
        }
        2 => {
            let q = idiv_q(cvtt(v), dim)?;
            let par = q & 1;
            // vcomiss v,0 ; jb: v<0 or unordered
            let negb = !(v >= 0.0);
            let refl = if negb { par == 0 } else { par != 0 };
            let wf = dim as f32;
            if refl {
                let r = fm(recs, v, dim);
                Ok(wf - r)
            } else {
                Ok(fm(recs, v, dim))
            }
        }
        _ => Ok(v),
    }
}

/// S0 scale constants: (scx = rbp, scy = rbp)
pub fn s0_scales(p: &Params) -> (f32, f32) {
    let half = 0.5f32;
    let x12 = half * p.aspect_block_d;
    let x13 = p.assumption_g - p.assumption_f;
    (half * x13, x12 * x13)
}

/// S2: wrapped x from the raw p1 x
pub fn s2_x(recs: &mut Vec<Rec>, p: &Params, inp: &Inputs, scx: f32, p1x: f32) -> Result<f32, Unsupported> {
    let t = p1x + 1.0f32;
    let base = p.assumption_f - (p.assumption_c as f32);
    let x = t.mul_add(scx, base); // vfmadd132ss 
    wrap(recs, x, p.img_w, inp.mode_x)
}

/// S3: wrapped y from the raw p1 y
pub fn s3_y(recs: &mut Vec<Rec>, p: &Params, inp: &Inputs, scy: f32, p1y: f32) -> Result<f32, Unsupported> {
    let t = p1y - 1.0f32;
    let m = scy.mul_add(t, -p.assumption_h); // vfmsub213ss : scy*t - assumption_h
    let top = p.assumption_e - (p.assumption_d as f32);
    let y = top - m;
    wrap(recs, y, p.img_h, inp.mode_y)
}

pub fn run(p: &Params, inp: &Inputs, mut p1: Option<&mut P1Env>) -> Result<Output, Unsupported> {
    let mut recs = Vec::new();
    let mut stubs = Vec::new();
    let (scx, scy) = s0_scales(p);
    recs.push(Rec::ScratchCellA); // 
    let mut out = Output { recs: Vec::new(), stub_samples: Vec::new(), s5: None };
    if p.count <= 0 {
        out.recs = recs;
        return Ok(out);
    }
    let g = |v: &Vec<f32>, i: usize| v.get(i).copied().unwrap_or(0.0);
    let (mut l_p1x, mut l_p1y, mut l_p2x, mut l_p2y) = (g(&inp.p1x, 0), g(&inp.p1y, 0), g(&inp.p2x, 0), g(&inp.p2y, 0));
    let k_1_a3 = 1.0f32 - inp.a[3];
    let k_a1_a0 = inp.a[1] - inp.a[0];
    let k_a2_a1 = inp.a[2] - inp.a[1];
    let mut i: usize = 0;
    loop {
        // S1
        if !inp.p1_const {
            l_p1x = g(&inp.p1x, i);
            l_p1y = g(&inp.p1y, i);
        }
        if !inp.p2_const {
            l_p2x = g(&inp.p2x, i);
            l_p2y = g(&inp.p2y, i);
        }
        let x = s2_x(&mut recs, p, inp, scx, l_p1x)?;
        let y = s3_y(&mut recs, p, inp, scy, l_p1y)?;
        // S4
        let dxr = l_p2x - l_p1x;
        let dyr = l_p2y - l_p1y;
        let e2x = dxr.mul_add(scx, x); // vfmadd132ss 
        let e2y = (-dyr).mul_add(scy, y); // vfnmadd132ss 
        let degenerate = x == e2x && y == e2y;
        if degenerate {
            // stub 
            recs.push(Rec::P1 { esi: i as u32, ecx: inp.mode_x as u32, r8: inp.mode_y as u32, x: x.to_bits(), y: y.to_bits() });
            if let Some(env) = p1.as_deref_mut() {
                let mut refs: Vec<&mut [f32]> = env.out.iter_mut().map(|v| v.as_mut_slice()).collect();
                blur_warp_point::render_point(&env.p, &env.g, &env.src, x, y, i, &mut refs);
                drop(refs);
                if let Some(o0) = env.out.get_mut(0) {
                    o0[i] = inp.a[4] * o0[i];
                }
            }
            stubs.push(i);
        } else {
            // S5
            recs.push(Rec::ImgCellA { esi: 0, edx: 0 });
            recs.push(Rec::ImgCellB { esi: 0 });
            let dx = e2x - x;
            let dy = e2y - y;
            let adx = dx.abs();
            let ady = dy.abs();
            out.s5 = Some(S5State {
                sample: i, x, y, p1x: l_p1x, p1y: l_p1y, p2x: l_p2x, p2y: l_p2y, e2x, e2y, dx, dy, adx, ady,
                y_major: !(adx >= ady), data_ptr: inp.acc_ptr, width: p.img_w, height: p.img_h, stride: inp.acc_stride,
                scx, scy, k_a1_a0, k_a2_a1, k_1_a3, scale: inp.a[4], flag_cell_a: inp.flag_cell_a, mode_x: inp.mode_x, mode_y: inp.mode_y,
            });
            break;
        }
        // S11 tail: rbx = i+1 < count
        // rbx = i + 1 (incremented at the loop head); cmp rbx, count; jl
        if !((i as i64 + 1) < p.count as i64) {
            break;
        }
        i += 1;
    }
    out.recs = recs;
    out.stub_samples = stubs;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> (Params, Inputs) {
        let p = Params { count: 1, aspect_block_d: 2.0, assumption_e: 7.0, assumption_c: 0, assumption_d: 0, assumption_f: 0.0, assumption_g: 10.0, assumption_h: 0.0, img_w: 8, img_h: 6 };
        let inp = Inputs {
            a: [0.0, 1.0, 3.0, 0.5, 2.0], p1x: vec![0.25], p1y: vec![3.0], p2x: vec![0.25], p2y: vec![3.0],
            p1_const: true, p2_const: true, flag_cell_a: 0, mode_x: 0, mode_y: 0, acc_ptr: 0x1000, acc_stride: 68,
        };
        (p, inp)
    }

    #[test]
    fn s0_hand() {
        let (p, _) = base();
        assert_eq!(s0_scales(&p), (5.0, 10.0));
    }

    #[test]
    fn s2_s3_mode0_hand() {
        let (p, inp) = base();
        let mut r = vec![];
        assert_eq!(s2_x(&mut r, &p, &inp, 5.0, 0.25).unwrap(), 6.25);
        assert_eq!(s3_y(&mut r, &p, &inp, 10.0, 3.0).unwrap(), -13.0);
        assert!(r.is_empty());
    }

    #[test]
    fn wrap_mode1_positive_and_negative() {
        let mut r = vec![];
        assert_eq!(wrap(&mut r, 10.5, 8, 1).unwrap(), 2.5);
        assert_eq!(wrap(&mut r, -1.5, 8, 1).unwrap(), 6.5);
        assert_eq!(r.len(), 2);
        assert_eq!(wrap(&mut r, 3.0, 0, 1).unwrap(), 0.0);
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn wrap_mode2_mirror() {
        let mut r = vec![];
        assert_eq!(wrap(&mut r, 10.5, 8, 2).unwrap(), 5.5);
        assert_eq!(wrap(&mut r, 2.5, 8, 2).unwrap(), 2.5);
        assert_eq!(wrap(&mut r, -1.5, 8, 2).unwrap(), 1.5);
        assert_eq!(wrap(&mut r, 1.0, 0, 2), Err(Unsupported::IdivFault));
    }

    #[test]
    fn zero_length_segment_takes_p1_stub() {
        let (p, inp) = base();
        let o = run(&p, &inp, None).unwrap();
        assert_eq!(o.stub_samples, vec![0]);
        assert!(o.s5.is_none());
        assert!(matches!(o.recs[0], Rec::ScratchCellA));
        assert!(matches!(o.recs[1], Rec::P1 { esi: 0, .. }));
    }

    #[test]
    fn nonzero_segment_reaches_s5() {
        let (p, mut inp) = base();
        inp.p2x = vec![1.25];
        let o = run(&p, &inp, None).unwrap();
        let s = o.s5.unwrap();
        assert_eq!(s.e2x, 11.25);
        assert_eq!(s.dx, 5.0);
        assert!(!s.y_major);
        assert_eq!(o.recs.len(), 3);
    }

    #[test]
    fn determinism_and_finite() {
        let (p, mut inp) = base();
        inp.p2x = vec![1.25];
        inp.mode_x = 1;
        inp.mode_y = 2;
        let a = run(&p, &inp, None).unwrap();
        let b = run(&p, &inp, None).unwrap();
        assert_eq!(a.recs, b.recs);
        let s = a.s5.unwrap();
        assert!(s.x.is_finite() && s.y.is_finite() && s.e2x.is_finite() && s.e2y.is_finite());
    }

    #[test]
    fn nan_and_inf_edges() {
        let (p, mut inp) = base();
        inp.p1x = vec![f32::NAN];
        inp.p2x = vec![f32::NAN];
        let o = run(&p, &inp, None).unwrap();
        assert!(o.s5.is_some());
        inp.p1x = vec![f32::INFINITY];
        inp.p2x = vec![f32::INFINITY];
        inp.mode_x = 1;
        let o = run(&p, &inp, None).unwrap();
        assert!(o.s5.is_some() || !o.stub_samples.is_empty());
        let mut r = vec![];
        assert!(wrap(&mut r, f32::NAN, 8, 1).unwrap().is_nan());
    }

    #[test]
    fn count_nonpositive_exits() {
        let (mut p, inp) = base();
        p.count = 0;
        let o = run(&p, &inp, None).unwrap();
        assert_eq!(o.recs, vec![Rec::ScratchCellA]);
    }

    #[test]
    fn two_samples_both_stub() {
        let (mut p, mut inp) = base();
        p.count = 2;
        inp.p1_const = false;
        inp.p2_const = false;
        inp.p1x = vec![0.25, 0.5];
        inp.p1y = vec![3.0, 2.0];
        inp.p2x = vec![0.25, 0.5];
        inp.p2y = vec![3.0, 2.0];
        let o = run(&p, &inp, None).unwrap();
        assert_eq!(o.stub_samples, vec![0, 1]);
    }
}
