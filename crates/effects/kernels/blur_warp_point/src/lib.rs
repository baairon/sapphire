//! blur_warp_bidirectional P1: the shared end-of-body routine, AVX2 route only
//! (body). Class id `blur_warp_bidirectional`.
//! Source of truth: the code dump listing; data VAs are listing.
//! VEX follows the SDM, not silicon (engine statement).

/// Explicit host state. Field names INFERRED, offsets MEASURED from the listing.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// channel count per pixel, [params]
    pub n: i32,
    /// [[params] field], clamp bound for x
    pub width: i32,
    /// [[params] field], clamp bound for y
    pub height: i32,
    ///  ecx: 0 means clamp x to [-1, width]
    pub flag_x: i32,
    ///  r8d: 0 means clamp y to [-1, height]
    pub flag_y: i32,
}

pub struct Assumption {
    pub id: &'static str,
    pub va: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "A1", va: "", basis: "feature word read at the dispatch; AVX2 route needs (word & 0x9d97ff) == 0x9d97ff; the port assumes the word 0x9d97ff", label: "ASSUMPTION" },
        Assumption { id: "A2", va: "stack", basis: "tmp buffer at rbp is 32 byte aligned (prologue: and rsp,-32; push rbp; push rbp), so the alignment peel count r9 is 0", label: "INFERRED" },
        Assumption { id: "A3", va: "stack", basis: "tmp buffer has 4*n bytes from rbp; n > 12 overwrites the frame in the original, so supported and tested range is 0 <= n <= 12", label: "ASSUMPTION" },
        Assumption { id: "A4", va: "", basis: "constants +1.0 (0000803f) and -1.0 (000080bf), captured (lookup.py cov_asOFF N4)", label: "MEASURED" },
        Assumption { id: "A5", va: "callee ", basis: "ported for the params == 0 branch only: base = n + n*(width+3) floats into the plane, row stride = (width+3)*n floats; the nonzero branch reads other struct fields and is an explicit input (Geometry)", label: "ASSUMPTION" },
        Assumption { id: "A6", va: "x/y NaN or out of range", basis: "cvttss2si gives i32::MIN, the address becomes wild in the original; not supported, not tested", label: "ASSUMPTION" },
    ]
}

/// Host-resolved addressing (results of calls), in f32 elements.
#[derive(Clone, Copy, Debug)]
pub struct Geometry {
    /// element index of the address returned by (params, 0, 0)
    pub base: i64,
    /// value returned by (params, 0)
    pub stride: i32,
}

impl Geometry {
    /// params == 0 branch of 
    pub fn mode0(p: &Params) -> Geometry {
        let rowf = p.width.wrapping_add(3);
        let r9 = p.n.wrapping_mul(rowf);
        Geometry { base: p.n as i64 + r9 as i64, stride: r9 }
    }
}

fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 { i32::MIN } else { x as i32 }
}

/// vminss dst,a,b : returns b when unordered or equal
fn minss(a: f32, b: f32) -> f32 { if a < b { a } else { b } }

///  per axis
fn clamp_axis(flag: i32, v: f32, bound: i32) -> f32 {
    if flag != 0 { return v; }
    let lo = -1.0f32;
    if lo > v { lo } else { minss(bound as f32, v) }
}

/// Interpolation taps for the n channels at (x, y), exact order of the AVX2 body.
pub fn sample(p: &Params, g: &Geometry, src: &[f32], x: f32, y: f32) -> Vec<f32> {
    let n = p.n;
    let xc = clamp_axis(p.flag_x, x, p.width);
    let yc = clamp_axis(p.flag_y, y, p.height);
    let one = 1.0f32;
    let x1 = one + xc;
    let y1 = one + yc;
    let ix = cvtt(x1);
    let iy = cvtt(y1);
    let fx = x1 - ix as f32;
    let fy = y1 - iy as f32;
    let offx = ix.wrapping_sub(1).wrapping_mul(n) as i64;
    let offy = iy.wrapping_sub(1).wrapping_mul(g.stride) as i64;
    let pos = g.base + offx + offy;
    let fxy = fx * fy;
    let w00 = ((one - fx) - fy) + fxy;
    let w10 = fx - fxy;
    let w01 = fy - fxy;
    let w11 = fxy;
    let nn = n.max(0) as usize;
    let mut tmp = vec![0.0f32; nn];
    let at = |i: i64| src[i as usize];
    let st = g.stride as i64;
    let nl = n as i64;
    // vector order: ((w10*B fused w00*A) fused w11*D) fused w01*C
    let vec_el = |q: i64| -> f32 {
        let v = w10 * at(q + nl);
        let v = w00.mul_add(at(q), v);
        let v = w11.mul_add(at(q + nl + st), v);
        w01.mul_add(at(q + st), v)
    };
    // scalar order: fma(w10,B,w11*D) + fma(w00,A,w01*C)
    let sc_el = |q: i64| -> f32 {
        let a = w10.mul_add(at(q + nl), w11 * at(q + nl + st));
        let b = w00.mul_add(at(q), w01 * at(q + st));
        a + b
    };
    if nn >= 16 {
        let r11 = nn & !15;
        for j in 0..r11 { tmp[j] = vec_el(pos + j as i64); }
        let rem = nn - r11;
        let q4 = rem & !3;
        for j in r11..r11 + q4 { tmp[j] = vec_el(pos + j as i64); }
        for j in r11 + q4..nn { tmp[j] = sc_el(pos + j as i64); }
    } else {
        for j in 0..nn { tmp[j] = sc_el(pos + j as i64); }
    }
    tmp
}

/// Full routine: writes channel k of the sample into out[k][idx] (raw 32 bit moves).
pub fn render_point(p: &Params, g: &Geometry, src: &[f32], x: f32, y: f32, idx: usize, out: &mut [&mut [f32]]) {
    let tmp = sample(p, g, src, x, y);
    for (k, v) in tmp.iter().enumerate() { out[k][idx] = *v; }
}
