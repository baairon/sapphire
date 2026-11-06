//! monochrome: native port of the CPU render entry of builtin class `monochrome`
//! (vtable, slot 53 =, AVX2 body inlined).
//!
//! Source of truth: neutral_1.asm (listing offsets; captured VA = offset).
//! The node turns a 3-or-more channel image into one luma plane:
//!   out = f32( 0.5*g + 0.3125*r + 0.1875*b )   computed in f64 (vcvtss2sd, vmulsd, 2 x vfmadd231sd, vcvtsd2ss)
//! Sections of (S = section):
//!   S0 dispatcher: word = **; (word & 0x9d97ff) == 0x9d97ff -> AVX2 body 
//!      ((word & 0x117ff) == 0x117ff -> AVX tier; bit 0 -> SSE tier; else init)
//!   S1 channel count c = [this]; c < 3 -> S6
//!   S2 planes: [this] object, plane pointers at (null object -> null planes)
//!   S3 n = (i32)[info]; n <= 0 returns; dest = the first output pointer
//!   S4 section split (n < 8 all scalar; n < 0x31d vector for n & ~7 elements, no peel;
//!      otherwise alignment peel from dest address & 0x1f, then vector, then scalar tail)
//!   S5 scalar loop and vector loop : same arithmetic, weights from different cells
//!   S6 tail call (this, 0, 0, outs[0], info) (host plane service, fills the pad plane)
//! FMA sites: scalar peel; vector; scalar tail .
//! Conversion sites: vcvtss2sd; vcvtps2pd 
//! vcvtsd2ss; vcvtpd2ps .

/// Luma weights (f64). Scalar cells are CAPTURED: = 0.3125 (plane 0),
///   = 0.5 (plane 1), = 0.1875 (plane 2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weights {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub const SCALAR_W: Weights = Weights { r: 0.3125, g: 0.5, b: 0.1875 };

/// Vector cells (plane 0), (plane 1),
///  (plane 2): 32 bytes each, NOT in the captured image. ASSUMED-SEMANTIC
/// MN1-A1..A3: each lane equals the scalar weight (the scalar peel/tail computes the same output
/// plane with the same operation order, so a different value would make results depend on the
/// buffer address).
pub const VEC_W: Weights = SCALAR_W;

/// The fields of the class object and the call info the entry reads.
/// No Default: every field is an explicit input.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// this: channel count of the single input image.
    pub channels: i32,
    /// info: pixels in the call (signed 32 bit).
    pub n: i32,
    /// (dest pointer) & 0x1f in bytes; 0 = 32 byte aligned buffer. Only multiples of 4 are
    /// reachable with f32 planes; a value with (phase & 3) != 0 sends the whole row down the scalar loop.
    pub dst_phase: u32,
}

/// A call the entry hands to a callee, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    /// (this, 0, 0, outs[0], info): tail call of the pad-plane host service (not ported here)
    Fill,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonoError {
    /// a plane is shorter than n
    ShortPlane,
    /// [this] is null while n > 0: the original dereferences address 0x20 (outside its contract)
    MissingSource,
}

/// Elements [lo, hi) go through the vector loop (vector weights), all others through the scalar loops.
/// Listing . n must be > 0.
pub fn split(n: i32, dst_phase: u32, m: u32) -> (usize, usize) {
    let t8 = if m == 3 { 9 } else { 8 };
    let t797 = if m == 4 { 800 } else { 0x31d };
    if n < t8 {
        return (0, 0);
    }
    if n < t797 {
        let hi = (n & !7) as usize;
        return (0, hi);
    }
    let r8 = dst_phase & 0x1f;
    let mut peel: u32 = 0;
    if m != 12 && r8 != 0 {
        if r8 & 3 != 0 {
            return (0, 0);
        }
        peel = (32 - r8) >> 2;
        if m == 5 {
            peel += 1;
        }
    }
    if (n as i64) < peel as i64 + 8 {
        return (0, 0);
    }
    let rem = ((n as u32).wrapping_sub(peel)) & if m == 6 { 3 } else { 7 };
    let hi = n as u32 - rem;
    (peel as usize, hi as usize)
}

fn to_f32_trunc(x: f64) -> f32 {
    let r = x as f32;
    if r.is_nan() || r.is_infinite() || (r as f64).abs() <= x.abs() {
        return r;
    }
    f32::from_bits(r.to_bits().wrapping_sub(1))
}

/// One pixel, order: t = wg*g; t = fma(r, wr, t); t = fma(b, wb, t); round to f32.
pub fn luma(w: &Weights, r: f32, g: f32, b: f32, m: u32) -> f32 {
    let (wr, wg, wb) = if m == 1 { (w.b, w.g, w.r) } else if m == 2 { (w.g, w.r, w.b) } else { (w.r, w.g, w.b) };
    if m == 8 {
        let t = (wg as f32) * g;
        let t = r.mul_add(wr as f32, t);
        return b.mul_add(wb as f32, t);
    }
    let (r, g, b) = (r as f64, g as f64, b as f64);
    let t = if m == 9 {
        let t = r * wr;
        let t = g.mul_add(wg, t);
        b.mul_add(wb, t)
    } else if m == 11 {
        let t = wg * g;
        let t = r * wr + t;
        b * wb + t
    } else {
        let t = wg * g;
        let t = r.mul_add(wr, t);
        b.mul_add(wb, t)
    };
    if m == 10 {
        to_f32_trunc(t)
    } else if m == 16 {
        let v = t as f32;
        if v != 0.0 && v.abs() < f32::MIN_POSITIVE { 0.0 } else { v }
    } else {
        t as f32
    }
}

/// The whole entry with an explicit vector weight triple and a mutation id (0 = the port; the others are
/// test mutants for the T2 kill rate). Planes: `src` = the three plane slices (the [this] object, None =
/// null), `out` = the dest plane starting at the dest pointer.
pub fn render_with(p: &Params, src: Option<[&[f32]; 3]>, out: &mut [f32], vecw: &Weights, m: u32) -> Result<Vec<Call>, MonoError> {
    let lim = if m == 7 { 3 } else { 2 };
    if p.channels <= lim {
        // a single tail call, whatever n is
        return Ok(vec![Call::Fill]);
    }
    let n = if m == 13 && p.n == 0 { 1 } else { p.n };
    if n <= 0 {
        return Ok(Vec::new());
    }
    let src = src.ok_or(MonoError::MissingSource)?;
    let nu = n as usize;
    for pl in src.iter() {
        if pl.len() < nu {
            return Err(MonoError::ShortPlane);
        }
    }
    if out.len() < nu {
        return Err(MonoError::ShortPlane);
    }
    let (lo, hi) = split(n, p.dst_phase, m);
    for i in 0..nu {
        let w = if i >= lo && i < hi { vecw } else { &SCALAR_W };
        out[i] = luma(w, src[0][i], src[1][i], src[2][i], m);
    }
    Ok(Vec::new())
}

/// The port: vector cells at their ASSUMED-SEMANTIC value.
pub fn render(p: &Params, src: Option<[&[f32]; 3]>, out: &mut [f32]) -> Result<Vec<Call>, MonoError> {
    render_with(p, src, out, &VEC_W, 0)
}

/// Assumption list (S3 render shape: explicit).
pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("MN1-A1", "ASSUMED-SEMANTIC (tier SEMANTIC): vector weight cell plane 1, 32 B, NOT CAPTURED (lookup_mono.tsv U32): every lane = 0.5 (f64), basis: scalar loops compute the same plane with captured 0.5; sensitivity: T2 alt run changes the cell and the output changes"),
        ("MN1-A2", "ASSUMED-SEMANTIC: vector weight cell plane 0, 32 B, NOT CAPTURED: every lane = 0.3125 (f64), same basis (scalar cell CAPTURED)"),
        ("MN1-A3", "ASSUMED-SEMANTIC: vector weight cell plane 2, 32 B, NOT CAPTURED: every lane = 0.1875 (f64), same basis (scalar cell CAPTURED)"),
        ("MN1-A4", "ROUTE AVX2-ONLY: feature word cell (pointer to, word zero in the capture) is served as 0x9d97ff in the R-PARITY run; AVX tier, SSE tier and the init retry are not ported"),
        ("MN1-A5", "HOST-HOOKED: tail callee (pad plane host service, channels < 3) is a record-only hook on the emulator side and a Call::Fill token on the Rust side native is not used (no READY at build time): class is PAD-UNVERIFIED"),
        ("MN1-A6", "ALIGN-DEPENDENT: Params.dst_phase is the explicit stand-in for (dest pointer & 0x1f); it only chooses which elements use the vector cells, so it changes outputs only if A1..A3 are wrong"),
        ("MN1-A7", "HOST-FP-ASSUMED: MXCSR round to nearest, no FTZ, no DAZ; null [this] with n > 0 is outside the contract (reported, not compared); dest pointer with (phase & 3) != 0 is not reachable with f32 planes (not compared)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(ch: i32, n: i32, ph: u32, r: &[f32], g: &[f32], b: &[f32]) -> (Vec<f32>, Vec<Call>) {
        let mut out = vec![f32::from_bits(0x7fc0ffee); 1300];
        let p = Params { channels: ch, n, dst_phase: ph };
        let c = render(&p, Some([r, g, b]), &mut out).unwrap();
        (out, c)
    }

    #[test]
    fn luma_hand_values() {
        // listing weights: r 0.3125, g 0.5, b 0.1875 (hand evaluated)
        assert_eq!(luma(&SCALAR_W, 1.0, 0.0, 0.0, 0), 0.3125);
        assert_eq!(luma(&SCALAR_W, 0.0, 1.0, 0.0, 0), 0.5);
        assert_eq!(luma(&SCALAR_W, 0.0, 0.0, 1.0, 0), 0.1875);
        assert_eq!(luma(&SCALAR_W, 1.0, 1.0, 1.0, 0), 1.0); // identity on grey
        assert_eq!(luma(&SCALAR_W, 0.4, 0.4, 0.4, 0), 0.4f32);
        assert_eq!(luma(&SCALAR_W, 2.0, -2.0, 0.0, 0), -0.375); // 2*0.3125 - 1
        assert_eq!(luma(&SCALAR_W, 0.0, 0.0, 0.0, 0), 0.0);
    }

    #[test]
    fn split_hand_values() {
        assert_eq!(split(1, 0, 0), (0, 0));
        assert_eq!(split(7, 0, 0), (0, 0));
        assert_eq!(split(8, 0, 0), (0, 8)); // n & ~7
        assert_eq!(split(15, 0, 0), (0, 8));
        assert_eq!(split(796, 0, 0), (0, 792));
        assert_eq!(split(797, 0, 0), (0, 792)); // phase 0: no peel, 797 - (797 & 7) = 792
        assert_eq!(split(800, 4, 0), (7, 800 - ((800 - 7) & 7) as usize)); // peel (32-4)/4 = 7
        assert_eq!(split(1000, 28, 0), (1, 1000 - ((1000 - 1) & 7) as usize));
        assert_eq!(split(900, 2, 0), (0, 0)); // phase & 3 != 0: all scalar
    }

    #[test]
    fn grey_identity_and_channels() {
        let v: Vec<f32> = (0..40).map(|i| i as f32 * 0.25 - 3.0).collect();
        let (o, c) = run(3, 40, 0, &v, &v, &v);
        assert!(c.is_empty());
        assert_eq!(&o[..40], &v[..]);
        assert!(o[40].is_nan()); // nothing written past n
        // channels 4 and 5 use the same three planes, no call
        assert_eq!(run(4, 40, 0, &v, &v, &v).0[..40], v[..]);
        assert_eq!(run(5, 9, 0, &v, &v, &v).1, vec![]);
    }

    #[test]
    fn small_channel_counts_call_fill() {
        let v = vec![0.0f32; 4];
        for ch in [-3, 0, 1, 2] {
            let (o, c) = run(ch, 4, 0, &v, &v, &v);
            assert_eq!(c, vec![Call::Fill]);
            assert!(o[0].is_nan());
        }
        // n <= 0 with channels >= 3 returns with no call and no write
        for n in [0, -1, -100] {
            let (o, c) = run(3, n, 0, &v, &v, &v);
            assert!(c.is_empty() && o[0].is_nan());
        }
    }

    #[test]
    fn nan_inf_denormal_determinism() {
        let r = [f32::NAN, f32::INFINITY, -f32::INFINITY, 1e-40, 1e30, -0.0, 0.0, 3.0];
        let g = [0.0, f32::NEG_INFINITY, f32::INFINITY, 1e-40, 1e30, -0.0, -0.0, 1.0];
        let b = [0.0, 0.0, 0.0, 1e-40, 1e30, -0.0, 0.0, 2.0];
        let (o, _) = run(3, 8, 0, &r, &g, &b);
        assert!(o[0].is_nan()); // NaN in r
        assert!(o[1].is_nan() && o[2].is_nan()); // inf - inf
        assert!(o[3] > 0.0 && o[3] < f32::MIN_POSITIVE); // denormal survives (no FTZ)
        assert!(o[4] == 1e30f32); // grey 1e30 is the identity (weights sum to 1)
        assert_eq!(o[5].to_bits(), 0x8000_0000); // -0 * 0.5 = -0; fma(-0, w, -0) = -0; stays -0
        assert_eq!(o[6].to_bits(), 0x0000_0000); // fma(+0, w, -0) = +0 (round to nearest)
        let (o2, _) = run(3, 8, 0, &r, &g, &b);
        assert_eq!(o.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), o2.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    }

    #[test]
    fn errors() {
        let mut out = vec![0.0f32; 8];
        let p = Params { channels: 3, n: 8, dst_phase: 0 };
        assert_eq!(render(&p, None, &mut out).unwrap_err(), MonoError::MissingSource);
        let s = vec![0.0f32; 4];
        assert_eq!(render(&p, Some([&s, &s, &s]), &mut out).unwrap_err(), MonoError::ShortPlane);
        // channels < 3 never touches the planes
        let q = Params { channels: 2, n: 8, dst_phase: 0 };
        assert_eq!(render(&q, None, &mut out).unwrap(), vec![Call::Fill]);
    }

    #[test]
    fn vector_weights_matter_only_in_vector_range() {
        let alt = Weights { r: 0.3, g: 0.6, b: 0.1 };
        let r: Vec<f32> = (0..20).map(|i| 0.1 + i as f32 * 0.07).collect();
        let g: Vec<f32> = r.iter().map(|v| v * 0.5 + 0.2).collect();
        let b: Vec<f32> = r.iter().map(|v| 1.0 - v).collect();
        let p = Params { channels: 3, n: 20, dst_phase: 0 };
        let mut a = vec![0.0f32; 20];
        let mut c = vec![0.0f32; 20];
        render_with(&p, Some([&r, &g, &b]), &mut a, &VEC_W, 0).unwrap();
        render_with(&p, Some([&r, &g, &b]), &mut c, &alt, 0).unwrap();
        // n = 20: vector range [0,16), scalar tail 16..20
        assert_ne!(a[..16], c[..16]);
        assert_eq!(a[16..], c[16..]);
    }
}
