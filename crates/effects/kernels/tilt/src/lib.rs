//! Port of the AVX2 and FMA body of class tilt. This module holds the prologue (state loads), both sincosf calls,
//! the flag tests, the tilt2 = 0 loops (rotate, shear and wrap variants) and, in `perspective.rs`, the tilt2 != 0
//! (perspective) loops. The flag path (byte argument non-zero, host offload serializers) is refused by name.
//!
//! Origin (comment only): render body 0x63b5d0, selected by the dispatcher 0x63b580 when
//! (feature word & 0x9d97ff) == 0x9d97ff.
//!
//! Operation order follows the listing; every add, sub, mul and div goes through `fx_common::nanop` in source operand
//! order; a fresh NaN is 0xffc00000.
use fx_common::nanop::{fma231, NanOps};

mod perspective;

/// Scalar f32 cells read by the body (values read from the emulated image, then fixed as constants).
pub const C_ONE: f32 = 1.0; // 0x15e850c
pub const C_DEG_BASE: f32 = 0.017453292; // 0x15e8c20, bits 0x3c8efa35 (pi / 180)
pub const C_ONE_HALF: f32 = 1.5; // 0x15e8b00, bits 0x3fc00000
pub const C_EPS: f32 = 1.0e-4; // 0x15e852c, bits 0x38d1b717
pub const C_DEG: f32 = C_DEG_BASE;
/// f64 clamp bounds (cells 0x160e650 and 0x160e658, bits 0xc04ff33333333333 and 0x404ff33333333333).
pub const CLAMP_LO: f64 = -63.9;
pub const CLAMP_HI: f64 = 63.9;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Path {
    /// byte argument cl != 0 (host offload serializers 0x364310 and 0x363d40): refused by name.
    Flag,
    /// a plane slice is shorter than the element count
    ShortBuffers,
}

/// Parameter block (offsets of the operand block in the instance, stride 0x1c per operand).
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub swivel: f32,        // +0x6c
    pub tilt: f32,          // +0x70
    pub perspective: f32,   // +0x88
    pub shift: [f32; 2],    // +0xa4, +0xa8
    pub rotate: f32,        // +0xc0
    pub scale: [f32; 2],    // +0xdc, +0xe0
    pub shear: [f32; 2],    // +0xf8, +0xfc
    pub wrap: [u32; 2],     // +0x114, +0x118 (bit patterns: zero clamps, anything else passes)
}

impl Default for State {
    fn default() -> Self {
        State { swivel: 0.0, tilt: 0.0, perspective: 1.0, shift: [0.0; 2], rotate: 0.0, scale: [1.0; 2], shear: [0.0; 2], wrap: [0; 2] }
    }
}

/// Values the prologue derives from the operand block.
#[derive(Debug, Clone, Copy)]
pub struct Setup {
    pub persp_flag: bool, // swivel^2 + tilt^2 != 0 (a NaN counts as non-zero)
    pub rot_flag: bool,   // rotate != 0
    pub shear_flag: bool, // shear.x != 0 or shear.y != 0
    pub neg_tilt: f32,
    pub rot_sin: f32,
    pub rot_cos: f32,
    // perspective constants, written only when persp_flag (zero otherwise)
    pub inv_persp: f32,
    pub inv_len: f32,
    pub tilt_sin: f32,
    pub tilt_cos: f32,
    pub axis_u: f32,
    pub axis_v: f32,
    pub sin_over_persp: f32,
    pub cos_over_persp: f32,
    pub sin_times_persp: f32,
    pub cos_times_persp: f32,
}

/// The one native for the sincosf callee (crate libm_sincos): (sin, cos).
pub fn sincos_native(x: f32) -> (f32, f32) {
    libm_sincos::sincosf_x(x)
}

pub(crate) fn flip(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

/// Prologue: state loads, flags, both sincosf calls.
pub fn setup(st: &State) -> Setup {
    let tilt_flipped = flip(st.tilt); // vxorps with the sign constant 0x160e6a0 (lane 0 = 0x80000000)
    let sq = tilt_flipped.fmul(tilt_flipped);
    let m = fma231(sq, st.swivel, st.swivel); // vfmadd231ss xmm3, xmm8, xmm8
    let persp_flag = !(m == 0.0); // vcmpneqss
    let rot_flag = !(st.rotate == 0.0);
    let shear_flag = !(st.shear[0] == 0.0 && st.shear[1] == 0.0);
    let mut s = Setup {
        persp_flag, rot_flag, shear_flag, neg_tilt: tilt_flipped, rot_sin: 0.0, rot_cos: 0.0, inv_persp: 0.0, inv_len: 0.0,
        tilt_sin: 0.0, tilt_cos: 0.0, axis_u: 0.0, axis_v: 0.0, sin_over_persp: 0.0, cos_over_persp: 0.0, sin_times_persp: 0.0, cos_times_persp: 0.0,
    };
    if persp_flag {
        let len = m.sqrt(); // vsqrtss (exact; NaN passes through, a negative operand cannot occur: m >= 0 or NaN)
        let len = if m < 0.0 { f32::from_bits(0xffc0_0000) } else { len };
        let inv_persp = C_ONE.fdiv(st.perspective);
        let inv_len = C_ONE.fdiv(len);
        let (sn, cs) = sincos_native(len.fmul(C_DEG));
        s.inv_persp = inv_persp;
        s.inv_len = inv_len;
        s.axis_u = inv_len.fmul(tilt_flipped);
        s.axis_v = inv_len.fmul(st.swivel);
        s.sin_over_persp = sn.fmul(inv_persp);
        s.cos_over_persp = inv_persp.fmul(cs);
        s.sin_times_persp = sn.fmul(st.perspective);
        s.cos_times_persp = cs.fmul(st.perspective);
        s.tilt_sin = sn;
        s.tilt_cos = cs;
    }
    if rot_flag {
        let (sn, cs) = sincos_native(st.rotate.fmul(C_DEG));
        s.rot_sin = sn;
        s.rot_cos = cs;
    }
    s
}

/// Clamp one coordinate to [CLAMP_LO, CLAMP_HI] in double precision.
/// vcvtss2sd, vcomisd lo, x (jbe: lo <= x or unordered skips), vminsd hi, x (second operand when unordered), vcvtsd2ss.
pub fn clamp(x: f32) -> f32 {
    if x.is_nan() {
        return f32::from_bits(x.to_bits() | 0x0040_0000);
    }
    let d = x as f64;
    let lo = CLAMP_LO;
    let hi = CLAMP_HI;
    let r = if lo > d { lo } else if hi < d { hi } else { d };
    r as f32
}

/// Render `n` elements of the tilt2 = 0 body. `x` and `y` are the two input planes, `first_out` and `second_out` the two outputs.
pub fn render(st: &State, x: &[f32], y: &[f32], first_out: &mut [f32], second_out: &mut [f32], n: i32, flag: bool) -> Result<(), Path> {
    let s = setup(st);
    if flag {
        return Err(Path::Flag);
    }
    if n <= 0 {
        return Ok(());
    }
    let n = n as usize;
    if x.len() < n || y.len() < n || first_out.len() < n || second_out.len() < n {
        return Err(Path::ShortBuffers);
    }
    if s.persp_flag {
        perspective::render(st, &s, x, y, first_out, second_out, n);
        return Ok(());
    }
    let (sh_x, sh_y) = (st.shift[0], st.shift[1]);
    for i in 0..n {
        let mut u = sh_x.fadd(x[i]); // vaddss xmm0, xmm8, [plane +0x20]
        let mut v = sh_y.fadd(y[i]); // vaddss xmm12, xmm7, [plane +0x28]
        if s.rot_flag {
            let t = u;
            let a = u.fmul(s.rot_cos);
            let b = v.fmul(s.rot_sin);
            let c = v.fmul(s.rot_cos);
            let d = t.fmul(s.rot_sin);
            u = a.fsub(b);
            v = c.fadd(d);
        }
        let mut p = st.scale[0].fmul(u); // vmulss xmm0, xmm6, xmm0
        let mut q = st.scale[1].fmul(v); // vmulss xmm1, xmm5, xmm12
        if s.shear_flag {
            let k = q.fmul(st.shear[0]);
            let l = p.fmul(st.shear[1]);
            let np = k.fadd(p);
            let nq = l.fadd(q);
            p = np;
            q = nq;
        }
        if st.wrap[0] == 0 {
            p = clamp(p);
        }
        if st.wrap[1] == 0 {
            q = clamp(q);
        }
        first_out[i] = p;
        second_out[i] = q;
    }
    Ok(())
}

pub struct Assumption {
    pub id: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    // Origins (comment only): feature word cell 0x1a98da0; SSE twin body 0x63cb50; flag path serializers 0x364310 and
    // 0x363d40; constant cells 0x15e850c, 0x15e8c20, 0x15e8b00, 0x15e852c, 0x160e650, 0x160e658; sign constant 0x160e6a0.
    vec![
        Assumption { id: "TL-A1", text: "The feature word is the harness AVX2 and FMA value, so the AVX2 body is the judged body; the SSE twin is not ported." },
        Assumption { id: "TL-A2", text: "The byte argument that selects the host offload path is refused by name and not rendered." },
        Assumption { id: "TL-A3", text: "The constant cells hold the values read from the emulated image at their readers (MEASURED, equal to the T2 values); T2 proves the code at those values." },
        Assumption { id: "TL-A4", text: "The sign constant is 0x80000000 in lane 0 and zero in lanes 1 to 3; only lane 0 reaches a result, so it is a one-bit sign flip, not a table." },
        Assumption { id: "SC1-A1", text: "Inherited from libm_sincos: all sincosf table entries and all double table entries are computed by libm_sincos and measured equal to the emulator's tables." },
        Assumption { id: "TL-A5", text: "The perspective loops read the prologue's derived constants as typed values; the registers the original loops read before writing hold no prologue value." },
    ]
}
