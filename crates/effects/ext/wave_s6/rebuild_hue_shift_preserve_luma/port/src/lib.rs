//! hue_shift_preserve_luma (HP1, patched copy HP2): native port of the CPU render entry of builtin class
//! `hue_shift_preserve_luma` (vtable, slot 52 = dispatcher, AVX2 body).
//!
//! HP2 change: the two vpermd index cells HP1-V14 (used) and
//! HP1-V15 (used) take the ORIGINAL FILE's words 0,2,4,6,0,0,0,0
//! (MEASURED-DYLIB) instead of the stand-in identity 0..7; vpermd is implemented faithfully (index & 7 selects the source
//! dword) and the red luma-term gate of variant A is modelled with real lane vectors (`red_kept`). Every other line of the
//! port is HP1's. The operand of both vpermd instructions is a lane-uniform all-ones or all-zero vector on every path
//! (listing proof in the README), so natural outputs do not depend on the index words; `Force` (test only) writes a
//! lane-varying operand to show that the model and the emulator agree when it is not uniform.
//!
//! Source of truth: neutral_1.asm (listing offsets; captured VA = offset).
//! Per pixel: the HSV hue rotation of hue_shift (V, S from min/max with the negative colour flip, hue by the
//! three max cases, x = h/6 + shift, floor wrap, six sectors) and then a luma restore: the colour's clamped
//! luma (weights 0.3125, 0.5, 0.1875, a component counts only when > 0) is compared with the luma of the
//! rotated colour; when they differ and the new luma is > 0 the result is scaled by old/new.
//! Sections of (S = section):
//!   S0 prologue, plane count (vcall [vtable]); count < 3 -> S9 (pad planes via host service)
//!   S1 flag byte != 0 -> (this, info, outs) (generic path, host hook), then S8
//!   S2 [this] source object (planes at), [this] mask object (plane at)
//!   S3 [this] != 0 and [this] != 0: constant colour, scalar shift :
//!      8 wide ymm loop for n & ~7, 4 wide xmm loop for (rest & ~3), scalar loop for the remainder
//!   S4 constant colour, shift = mask[i] : scalar only
//!   S5 planes, scalar shift : scalar only
//!   S6 planes, mask shift : scalar only
//!   S8 plane count == 4 -> tail jmp (this, 0, 3, outs[3], info)
//!   S9 count < 3: for i in 0..count (this, 0, i, outs[i], info)
//! The vector loops divide with vrcpps plus one Newton step (APPROX-MODEL: 1/x exact), do not fuse
//! h/6 + shift (vmulps, vaddps) and fuse only the sector products; the scalar loops use vdivss and fuse
//! h/6 + shift (vfmadd). FMA sites: scalar
//!       
//! vector (ymm), (xmm).
//! Approx sites: vrcpps (ymm), (xmm).
//! vpermd sites: (ymm9 = red mask, ymm loop setup) and (ymm10 = red mask, xmm section setup).
use std::sync::atomic::{AtomicU32, Ordering};

static MUT: AtomicU32 = AtomicU32::new(0);
/// Test-only: select a deliberate fault (T2 kill-rate runs). 0 = the port.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}
thread_local! {
    /// APPROX-BOUND test only: relative perturbation of every rcp estimate before the Newton step.
    static PERT: std::cell::Cell<f32> = std::cell::Cell::new(0.0);
}
fn mu(id: u32) -> bool {
    MUT.load(Ordering::Relaxed) == id
}

/// Scalar cells (CAPTURED): 0.3125 red, 0.5 green, 
/// 0.1875 blue, 1.0, 2.0, 4.0, 1/6 (0x3e2aaaad), sign mask (16 B).
/// 6.0 is STD-006 (MEASURED-DYLIB: the file holds 6.0f).
pub const W_R: f32 = 0.3125;
pub const W_G: f32 = 0.5;
pub const W_B: f32 = 0.1875;
pub const SIXTH: f32 = f32::from_bits(0x3e2aaaad);
pub const SIX: f32 = 6.0;

/// HP1-V14 / HP1-V15 as read from the original file (MEASURED-DYLIB, 32 B each):
/// dwords 0, 2, 4, 6, 0, 0, 0, 0 (both cells equal).
pub const PERM_FILE: [u32; 8] = [0, 2, 4, 6, 0, 0, 0, 0];
/// The old HP1 stand-in (identity), kept only for the before/after comparison and the mutants.
pub const PERM_OLD: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// vpermd ymm, idx, src: result[i] = src[idx[i] & 7] (SDM; the index register is the FIRST source in VEX.vvvv).
pub fn vpermd(idx: [u32; 8], src: [u32; 8]) -> [u32; 8] {
    let mut o = [0u32; 8];
    for i in 0..8 {
        o[i] = src[(idx[i] & 7) as usize];
    }
    o
}

fn mask8(b: bool) -> [u32; 8] {
    [if b { 0xffff_ffff } else { 0 }; 8]
}

/// Test-only forced vpermd operands (FORCED-OPERAND): the emulator side writes the same 256 bit value into ymm9 (site
/// ) or ymm10 (site) just before the vpermd. Not reachable from the original's own data.
#[derive(Clone, Copy, Debug, Default)]
pub struct Force {
    pub ymm: Option<[u32; 8]>,
    pub xmm: Option<[u32; 8]>,
}

/// Vector constant cells of one width (ymm cells xx, xmm cells xx / xx). Each field is one cell
/// value (HP1-V01..V11 and the vpermd cell for ymm, HP1-X01..X11 for xmm; HP1-V15 is the vpermd cell read,
/// which belongs to the xmm section, so the xmm VCells carries it). No Default: the caller names every value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VCells {
    /// 1.0: (1 - f)
    pub one: f32,
    /// 1/6: the hue scale (vmulps)
    pub sixth: f32,
    /// 6.0: red wrap (q + 6), t = 6 * frac, clamp compare
    pub six: f32,
    /// 3: vpcmpgtd (3 > i) and vpcmpeqd (i == 3)
    pub three_i: i32,
    /// 1: vpcmpeqd (i == 1)
    pub one_i: i32,
    /// 4: vpcmpeqd (i == 4)
    pub four_i: i32,
    /// red luma weight of the result
    pub w_r: f32,
    /// 0.5: green luma weight of the result and the S < 0 hue offset (one cell, two uses)
    pub half: f32,
    /// blue luma weight of the result
    pub w_b: f32,
    /// 2.0: green-max hue offset
    pub two: f32,
    /// 4.0: blue-max hue offset
    pub four: f32,
    /// vpermd index words: HP1-V14 (ymm site) or HP1-V15 (xmm-section site)
    pub perm: [u32; 8],
}

impl VCells {
    /// The old HP1 stand-ins (identity vpermd index); history and mutants only.
    pub const STANDIN: VCells = VCells { one: 1.0, sixth: SIXTH, six: 6.0, three_i: 3, one_i: 1, four_i: 4, w_r: W_R, half: W_G, w_b: W_B, two: 2.0, four: 4.0, perm: PERM_OLD };
    /// The cells as read from the file (MEASURED-DYLIB): equal to STANDIN except the vpermd index.
    pub const FILE: VCells = VCells { perm: PERM_FILE, ..VCells::STANDIN };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cells {
    pub ymm: VCells,
    pub xmm: VCells,
}

impl Cells {
    /// Old stand-ins (before/after comparison only).
    pub const STANDIN: Cells = Cells { ymm: VCells::STANDIN, xmm: VCells::STANDIN };
    /// MEASURED-DYLIB cells (what `render` uses).
    pub const FILE: Cells = Cells { ymm: VCells::FILE, xmm: VCells::FILE };
}

/// The fields of the class object and the call info the entry reads. No Default: every field is explicit.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// return of the plane count getter [vtable] (a host service, fixed integer)
    pub plane_count: i32,
    /// the flag byte (cl): nonzero goes to the generic path 
    pub flag: bool,
    /// this: the colour is the constant 
    pub const_colour: bool,
    pub colour: [f32; 3],
    /// this: the shift is the scalar (otherwise the plane at [this])
    pub scalar_shift: bool,
    pub shift: f32,
    /// info: pixels in the call (signed 32 bit)
    pub n: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Call {
    /// (this, 0, plane, outs[plane], info): host plane service
    Fill(i32),
    /// (this, info, outs): generic flag path (host hook)
    Generic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HpError {
    /// an output or input plane is shorter than n
    ShortPlane,
    /// the source object [this] is null but the planes are needed (the original faults)
    MissingSource,
    /// the mask object [this] is null but the shift plane is needed (the original faults)
    MissingMask,
}

fn fmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
fn fmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
/// cvttss2si / cvttps2dq: NaN and out of range give 0x80000000
fn cvtt(t: f32) -> i32 {
    if t.is_nan() || t >= 2147483648.0 || t < -2147483648.0 { i32::MIN } else { t as i32 }
}
fn negf(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

/// vrcpps modelled as exact 1/x (APPROX-MODEL) plus the code's Newton step
/// r1 = (r + r) - ((x * r) * r) (separate roundings: vaddps, vmulps, vmulps, vsubps).
fn newton(x: f32) -> f32 {
    let mut r = 1.0f32 / x;
    let e = PERT.with(|p| p.get());
    if e != 0.0 {
        r *= 1.0 + e;
    }
    if mu(5) {
        return r;
    }
    (r + r) - ((x * r) * r)
}

/// clamped luma of a colour with the given weights: (rt + gt) + bt, a term only when the component is > 0
fn luma(c: [f32; 3], wr: f32, wg: f32, wb: f32) -> f32 {
    luma_gated(c, wr, wg, wb, c[0] > 0.0)
}

/// Same, with the red term gate given (the vector loops of variant A gate the red term with the vptest of the
/// vpermd-permuted mask, `red_kept`; green and blue use their own masks directly).
fn luma_gated(c: [f32; 3], wr: f32, wg: f32, wb: f32, red: bool) -> f32 {
    let (wr, wb) = if mu(1) { (wb, wr) } else { (wr, wb) };
    let pos = |x: f32| x > 0.0 || mu(21);
    let rt = if red || mu(21) { wr * c[0] } else { 0.0 };
    let gt = if pos(c[1]) { wg * c[1] } else { 0.0 };
    let bt = if pos(c[2]) { wb * c[2] } else { 0.0 };
    (rt + gt) + bt
}

/// The red-mask path of variant A (MEASURED listing): mask = (r > 0) ? all ones : 0 in a full ymm register
/// `vpermd ymm, ymm15(index cell), mask`ymm site: `vinserti128`
/// duplicates the low 128 bits and the complement (xor with all ones) is stored to [rbp]; the loop
/// head does `vptest complement, all ones` and zeroes the red term (vxorps ymm4) when ANY complement bit is set;
/// xmm site: the low 128 bits are spilled, complemented and tested the same way (vptest xmm).
/// So the red term is kept iff the four (ymm: duplicated) selected dwords are all ones.
fn red_kept(k: &VCells, wide: bool, operand: [u32; 8]) -> bool {
    let mut idx = k.perm;
    if (wide && mu(22)) || (!wide && mu(23)) {
        idx = PERM_OLD; // mutant: the old identity stand-in
    }
    if (wide && mu(29)) || (!wide && mu(30)) {
        idx = [1, 3, 5, 7, 1, 1, 1, 1]; // mutant: odd dwords instead of even
    }
    if mu(31) {
        idx = [idx[3], idx[2], idx[1], idx[0], idx[4], idx[5], idx[6], idx[7]]; // probe: order inside the set
    }
    let p = if mu(27) { vpermd(operand, idx) } else { vpermd(idx, operand) }; // mutant 27: operands swapped
    let lanes: [u32; 8] = if wide {
        if mu(28) {
            p // probe: no duplication of the low half (full 8 lanes tested)
        } else if mu(24) {
            [p[4], p[5], p[6], p[7], p[4], p[5], p[6], p[7]] // mutant: high half duplicated
        } else {
            [p[0], p[1], p[2], p[3], p[0], p[1], p[2], p[3]]
        }
    } else if mu(25) {
        [p[4], p[5], p[6], p[7], 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff] // mutant: high half used by the xmm site
    } else {
        [p[0], p[1], p[2], p[3], 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff]
    };
    if mu(26) {
        lanes.iter().any(|&w| w == 0xffff_ffff) // mutant: vptest sense (any lane all ones)
    } else {
        lanes.iter().all(|&w| w == 0xffff_ffff)
    }
}

/// sector select shared by both flavours: returns (R, G, B) from V, C, f, 1 - f computed by the caller
fn sector(i: i32, three: i32, one_i: i32, four: i32, v: f32, cc: f32, f: f32, one: f32) -> [f32; 3] {
    let om = one - f;
    let a1 = if mu(6) { v - om * cc } else { (-om).mul_add(cc, v) };
    let a2 = v - cc;
    let a3 = (-f).mul_add(cc, v);
    if three > i {
        if i == 0 {
            [v, a1, a2]
        } else if i == one_i {
            if mu(18) { [a3, v, a1] } else { [a3, v, a2] }
        } else {
            [a2, v, a1]
        }
    } else if i == three {
        [a2, a3, v]
    } else if i == four {
        [a1, a2, v]
    } else {
        [v, a2, a3]
    }
}

/// scalar pixel (loops): vdivss and fused h/6 + shift
fn scalar_pixel(c: [f32; 3], shift: f32) -> [f32; 3] {
    let [r, g, b] = c;
    let lold = luma(c, W_R, W_G, W_B);
    let mn = fmin(fmin(r, g), b);
    let mx = fmax(fmax(r, g), b);
    let flip = if mu(7) { negf(mn) >= mx } else { negf(mn) > mx };
    let (v, hi, lo, rr, gg, bb) = if flip { (mn, negf(mn), negf(mx), negf(r), negf(g), negf(b)) } else { (mx, mx, mn, r, g, b) };
    let delta = hi - lo;
    let mut out = [v, v, v];
    let grey = (hi == 0.0 && !mu(15)) || {
        let s = delta / hi;
        s == 0.0
    };
    if !grey {
        let s0 = delta / hi;
        let h = if rr == hi {
            let q = (gg - bb) / delta;
            if 0.0 > q && !mu(2) || (mu(2) && 0.0 >= q) { q + SIX } else { q }
        } else if gg == hi {
            (if mu(16) { 4.0 } else { 2.0 }) + (bb - rr) / delta
        } else {
            4.0 + (rr - gg) / delta
        };
        let x = if mu(3) { h * SIXTH + shift } else { h.mul_add(SIXTH, shift) };
        let sneg = 0.0 > s0 && !mu(12);
        let s = if sneg { negf(s0) } else { s0 };
        let frac = if sneg {
            let y = x + 0.5;
            y - y.floor()
        } else {
            x - x.floor()
        };
        let t6 = SIX * frac;
        // vcomiss t6, 6.0 / jb: below or unordered (NaN) keeps t6
        let t = if (!(t6 >= SIX) && !mu(11)) || (mu(11) && !(t6 > SIX)) { t6 } else { 0.0 };
        let i = cvtt(t);
        let f = t - (i as f32);
        let cc = v * s;
        out = sector(i, 3, 1, 4, v, cc, f, 1.0);
    }
    let lnew = luma(out, W_R, W_G, W_B);
    if lold != lnew && ((lnew > 0.0 && !mu(8)) || (mu(8) && lnew >= 0.0)) {
        let sc = lold / lnew;
        out = [out[0] * sc, out[1] * sc, out[2] * sc];
    }
    out
}

/// vector pixel (ymm loop, xmm loop): every lane holds the same value in the
/// constant-colour scalar-shift loop, so one lane is computed. vrcpps+Newton instead of vdivss, h/6 and
/// shift not fused, luma weights of the result from the vector cells. `wide` = ymm site (cells.ymm), else the xmm
/// section; `force` replaces the vpermd operand (test only).
fn vec_pixel(k: &VCells, wide: bool, c: [f32; 3], shift: f32, force: Option<[u32; 8]>) -> [f32; 3] {
    let [r, g, b] = c;
    let operand = force.unwrap_or_else(|| mask8(c[0] > 0.0));
    let lold = luma_gated(c, W_R, W_G, W_B, red_kept(k, wide, operand));
    let mn = fmin(fmin(r, g), b);
    let mx = fmax(fmax(r, g), b);
    let flip = if mu(7) { negf(mn) >= mx } else { negf(mn) > mx };
    let (v, hi, lo, rr, gg, bb) = if flip { (mn, negf(mn), negf(mx), negf(r), negf(g), negf(b)) } else { (mx, mx, mn, r, g, b) };
    let delta = hi - lo;
    let s0 = if hi != 0.0 { delta * newton(hi) } else { 0.0 };
    let active = !(s0 == 0.0);
    let mut out = [v, v, v];
    if active {
        let nd = newton(delta);
        let h = if rr == hi {
            let q = (gg - bb) * nd;
            if (q < 0.0 && !mu(19)) || (mu(19) && q <= 0.0) { q + k.six } else { q }
        } else if gg == hi {
            (bb - rr) * nd + k.two
        } else {
            (rr - gg) * nd + (if mu(17) { k.two } else { k.four })
        };
        let x = if mu(4) { h.mul_add(k.sixth, shift) } else { h * k.sixth + shift };
        let sneg = s0 < 0.0 && !mu(12);
        let s = if sneg { negf(s0) } else { s0 };
        let frac = if sneg {
            let y = k.half + x;
            y - y.floor()
        } else {
            x - x.floor()
        };
        let t6 = k.six * frac;
        let t = if t6 >= k.six { 0.0 } else { t6 };
        let i = cvtt(t);
        let f = t - (i as f32);
        let cc = v * s;
        out = sector(i, k.three_i, k.one_i, k.four_i, v, cc, f, k.one);
    }
    let lnew = luma(out, k.w_r, k.half, k.w_b);
    if lold != lnew && lnew > 0.0 {
        let sc = if mu(13) { lold / lnew } else { lold * newton(lnew) };
        out = [out[0] * sc, out[1] * sc, out[2] * sc];
    }
    out
}

/// Render the planes of one call. `src` = the three source planes (the [this] object), `mask` = the shift
/// plane ([this] object). `outs` = the three destination planes (each at least n long). `force` is the
/// test-only vpermd operand override (None = the original's own data).
pub fn render_forced(p: &Params, src: Option<[&[f32]; 3]>, mask: Option<&[f32]>, outs: &mut [Vec<f32>; 3], cells: &Cells, force: Option<&Force>) -> Result<Vec<Call>, HpError> {
    let mut calls = Vec::new();
    if p.plane_count < 3 {
        for i in 0..p.plane_count.max(0) {
            calls.push(Call::Fill(i));
        }
        return Ok(calls);
    }
    if p.flag {
        calls.push(Call::Generic);
    } else if p.n > 0 {
        let n = p.n as usize;
        if outs.iter().any(|o| o.len() < n) {
            return Err(HpError::ShortPlane);
        }
        let put = |outs: &mut [Vec<f32>; 3], i: usize, v: [f32; 3]| {
            outs[0][i] = v[0];
            outs[1][i] = v[1];
            outs[2][i] = v[2];
        };
        if p.const_colour && p.scalar_shift {
            let (t8, t4) = (if mu(9) { 9 } else { 8 }, if mu(10) { 5 } else { 4 });
            let m8 = if p.n >= t8 { n & !7 } else { 0 };
            if m8 > 0 {
                let v = vec_pixel(&cells.ymm, true, p.colour, p.shift, force.and_then(|f| f.ymm));
                for i in 0..m8 {
                    put(outs, i, v);
                }
            }
            let mut idx = m8;
            if n > m8 {
                let rest = (n - m8) as i32;
                if rest >= t4 {
                    let m4 = (rest as usize) & !3;
                    let v = vec_pixel(&cells.xmm, false, p.colour, p.shift, force.and_then(|f| f.xmm));
                    for i in idx..idx + m4 {
                        put(outs, i, v);
                    }
                    idx += m4;
                }
                if idx < n {
                    let v = scalar_pixel(p.colour, p.shift);
                    for i in idx..n {
                        put(outs, i, v);
                    }
                }
            }
        } else {
            let sp: Option<[&[f32]; 3]> = if p.const_colour { None } else { Some(src.ok_or(HpError::MissingSource)?) };
            let mk: Option<&[f32]> = if p.scalar_shift { None } else { Some(mask.ok_or(HpError::MissingMask)?) };
            if let Some(pl) = &sp {
                if pl.iter().any(|q| q.len() < n) {
                    return Err(HpError::ShortPlane);
                }
            }
            if let Some(m) = mk {
                if m.len() < n {
                    return Err(HpError::ShortPlane);
                }
            }
            for i in 0..n {
                let c = match &sp {
                    Some(pl) => [pl[0][i], pl[1][i], pl[2][i]],
                    None => p.colour,
                };
                let sh = match mk {
                    Some(m) => m[i],
                    None => p.shift,
                };
                put(outs, i, scalar_pixel(c, sh));
            }
        }
    }
    if p.plane_count == 4 && !mu(20) {
        calls.push(Call::Fill(3));
    }
    Ok(calls)
}

/// HP1's entry with explicit cells (no operand override).
pub fn render_with(p: &Params, src: Option<[&[f32]; 3]>, mask: Option<&[f32]>, outs: &mut [Vec<f32>; 3], cells: &Cells) -> Result<Vec<Call>, HpError> {
    render_forced(p, src, mask, outs, cells, None)
}

/// The port with the cells read from the original file (MEASURED-DYLIB).
pub fn render(p: &Params, src: Option<[&[f32]; 3]>, mask: Option<&[f32]>, outs: &mut [Vec<f32>; 3]) -> Result<Vec<Call>, HpError> {
    render_with(p, src, mask, outs, &Cells::FILE)
}

/// Every assumption, with its tier: id, what, tier and basis.
pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HP1-S1", "f32 6.0 : registry STD-006, MEASURED-DYLIB (DV1 CONFIRMED; re-read by HP2: 0x40c00000)."),
        ("HP1-V01..V11", "ymm numeric cells 1.0 (registry STD-002), 1/6, 6.0, int 3, int 1, int 4, 0.3125, 0.5, 0.1875, 2.0, 4.0 : MEASURED-DYLIB (VA = listing; equal to the stand-in values; re-read by HP2). Old tier SEMANTIC kept as history."),
        ("HP1-V12,V13", "ymm structural cells: all-ones (STD-001), sign mask (STD-005): MEASURED-DYLIB (equal)."),
        ("HP1-V14", "vpermd index cell, =, used : MEASURED-DYLIB words 0,2,4,6,0,0,0,0 (the old stand-in identity 0..7 was REFUTED by DV1). The port takes the file words (PERM_FILE). Operand is a lane-uniform all-ones/all-zero ymm9 on every path (listing proof), so natural outputs are index independent; the forced lane-varying T2 (HP2) shows model and emulator agree otherwise."),
        ("HP1-V15", "vpermd index cell, =, used (xmm section): MEASURED-DYLIB words 0,2,4,6,0,0,0,0 (same words as V14). Operand ymm10 is a lane-uniform full ymm mask on every path (listing proof)."),
        ("HP1-X01..X11", "xmm numeric cells 1.0, 2.0, 1/6, 6.0, int 3, int 1, int 4, 0.3125, 0.5, 0.1875, 4.0 : MEASURED-DYLIB (equal)."),
        ("HP1-X12..X13", "xmm structural cells: all-ones, sign mask : MEASURED-DYLIB (equal)."),
        ("HP1-A1", "feature word served as 0x9d97ff (ROUTE AVX2-ONLY; AVX tier and SSE tier not rebuilt)."),
        ("HP1-A2", "plane count getter [vtable] is a host service (fixed integer per call)."),
        ("HP1-A3", " (plane service) and (generic flag path) are record-only hooks (HOST-HOOKED); PAD-UNVERIFIED."),
        ("HP1-A4", "vrcpps modelled as exact 1/x (W49, APPROX-MODEL); the Newton step is the code's own."),
        ("HP1-A5", "null source or mask object with n > 0 is outside the contract (the original faults)."),
        ("HP1-A6", "buffers are plain f32 slices: no alignment effect exists in this body (vmaskmovps, no peel): ALIGN-INDEPENDENT."),
        ("HP2-F1", "Force (test only): a lane-varying vpermd operand written into ymm9/ymm10 on the emulator side and passed to red_kept on the Rust side; FORCED-OPERAND, not reachable from the original's data."),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(p: &Params, src: Option<[&[f32]; 3]>, mask: Option<&[f32]>) -> ([Vec<f32>; 3], Vec<Call>) {
        let mut o = [vec![f32::NAN; 64], vec![f32::NAN; 64], vec![f32::NAN; 64]];
        let c = render(p, src, mask, &mut o).unwrap();
        (o, c)
    }
    fn pc(col: [f32; 3], shift: f32, n: i32) -> Params {
        Params { plane_count: 3, flag: false, const_colour: true, colour: col, scalar_shift: true, shift, n }
    }

    #[test]
    fn grey_is_identity() {
        let (o, _) = run(&pc([0.5, 0.5, 0.5], 0.3, 3), None, None);
        assert_eq!([o[0][0], o[1][0], o[2][0]], [0.5, 0.5, 0.5]);
        assert!(o[0][3].is_nan());
    }
    #[test]
    fn red_half_turn_keeps_luma_scalar() {
        // colour (1,0,0): luma 0.3125; hue +0.5 -> cyan-ish (0,1,1) with luma 0.6875, scaled by 0.3125/0.6875
        let p = pc([1.0, 0.0, 0.0], 0.5, 1);
        let (o, _) = run(&p, None, None);
        let sc = 0.3125f32 / 0.6875f32;
        assert_eq!(o[0][0], 0.0);
        assert_eq!(o[1][0], sc);
        assert_eq!(o[2][0], sc);
    }
    #[test]
    fn vector_and_scalar_sections_split() {
        // n = 15: 8 elements ymm, rest 7 -> 4 elements xmm, 3 scalar
        let p = pc([0.9, 0.2, 0.4], 0.3, 15);
        let (o, _) = run(&p, None, None);
        let (a, b, c) = (vec_pixel(&VCells::FILE, true, p.colour, 0.3, None), vec_pixel(&VCells::FILE, false, p.colour, 0.3, None), scalar_pixel(p.colour, 0.3));
        assert_eq!(a, b);
        for i in 0..8 {
            assert_eq!(o[0][i], a[0]);
        }
        for i in 8..12 {
            assert_eq!(o[1][i], b[1]);
        }
        for i in 12..15 {
            assert_eq!(o[2][i], c[2]);
        }
        assert!(o[0][15].is_nan());
        // vector and scalar agree to a few ulp
        for k in 0..3 {
            assert!((a[k] - c[k]).abs() < 1e-5);
        }
    }
    #[test]
    fn planes_and_mask_and_calls() {
        let r = [0.9f32, 0.1, 0.3];
        let g = [0.2f32, 0.8, 0.3];
        let b = [0.4f32, 0.5, 0.3];
        let m = [0.0f32, 0.25, 0.5];
        let p = Params { plane_count: 4, flag: false, const_colour: false, colour: [0.0; 3], scalar_shift: false, shift: 0.0, n: 3 };
        let mut o = [vec![0.0; 3], vec![0.0; 3], vec![0.0; 3]];
        let calls = render(&p, Some([&r, &g, &b]), Some(&m), &mut o).unwrap();
        assert_eq!(calls, vec![Call::Fill(3)]);
        // shift 0 keeps the colour (luma already equal)
        assert!((o[0][0] - 0.9).abs() < 1e-6 && (o[1][0] - 0.2).abs() < 1e-6 && (o[2][0] - 0.4).abs() < 1e-6);
        // grey stays grey
        assert_eq!([o[0][2], o[1][2], o[2][2]], [0.3, 0.3, 0.3]);
        let q = Params { plane_count: 2, ..p };
        assert_eq!(render(&q, None, None, &mut o).unwrap(), vec![Call::Fill(0), Call::Fill(1)]);
        let q = Params { flag: true, ..p };
        assert_eq!(render(&q, None, None, &mut o).unwrap(), vec![Call::Generic, Call::Fill(3)]);
    }
    #[test]
    fn nan_inf_degenerate_and_determinism() {
        for col in [[f32::NAN, 0.2, 0.3], [f32::INFINITY, 0.0, 0.0], [-1.0, -0.5, -0.25], [0.0, 0.0, 0.0], [1e-40, 0.0, 0.0], [f32::NEG_INFINITY, 0.0, 1.0]] {
            for sh in [0.0, -0.0, 0.25, f32::NAN, 1e30, -3.5] {
                let p = pc(col, sh, 13);
                let (a, _) = run(&p, None, None);
                let (b, _) = run(&p, None, None);
                for k in 0..3 {
                    for i in 0..13 {
                        assert_eq!(a[k][i].to_bits(), b[k][i].to_bits());
                    }
                }
            }
        }
        let p = pc([0.3, 0.3, 0.3], 0.1, 0);
        let (o, c) = run(&p, None, None);
        assert!(o[0][0].is_nan() && c.is_empty());
        let p = pc([0.3, 0.3, 0.3], 0.1, -5);
        let (o, _) = run(&p, None, None);
        assert!(o[0][0].is_nan());
    }
    /// APPROX-BOUND: perturb every vrcpps estimate by up to 1.5 x 2^-12 relative (both signs) before the
    /// code's own Newton step and report the maximum ulp change of the vector-loop outputs.
    #[test]
    fn approx_bound_rcp_perturbation() {
        let ulp = |a: f32, b: f32| -> u32 {
            let k = |x: f32| {
                let i = x.to_bits();
                if i < 0x8000_0000 { i } else { 0x8000_0000u32.wrapping_sub(i) }
            };
            k(a).abs_diff(k(b))
        };
        let mut s: u64 = 0x1234_5678_9abc_def0;
        let mut rnd = || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((s >> 40) as f32) / 16777216.0
        };
        let (mut maxu, mut n, mut sumu, mut big2, mut big64) = (0u32, 0u32, 0u64, 0u32, 0u32);
        for _ in 0..4000 {
            let col = [rnd() * 2.0 - 0.5, rnd() * 2.0 - 0.5, rnd() * 2.0 - 0.5];
            let sh = rnd() * 2.0 - 1.0;
            PERT.with(|p| p.set(0.0));
            let base = vec_pixel(&VCells::FILE, true, col, sh, None);
            for e in [1.5 / 4096.0, -1.5 / 4096.0] {
                PERT.with(|p| p.set(e));
                let v = vec_pixel(&VCells::FILE, true, col, sh, None);
                for k in 0..3 {
                    if base[k].is_finite() && v[k].is_finite() {
                        let u = ulp(base[k], v[k]);
                        maxu = maxu.max(u);
                        sumu += u as u64;
                        big2 += (u > 2) as u32;
                        big64 += (u > 64) as u32;
                        n += 1;
                    }
                }
            }
        }
        PERT.with(|p| p.set(0.0));
        println!("APPROX-BOUND max ulp change {} mean {:.3} over {} outputs; {} above 2 ulp, {} above 64 ulp", maxu, sumu as f64 / n as f64, n, big2, big64);
        // a measurement, not a pass/fail gate: floor and sector boundaries make the change discontinuous
    }
    #[test]
    fn errors() {
        let p = Params { plane_count: 3, flag: false, const_colour: false, colour: [0.0; 3], scalar_shift: true, shift: 0.0, n: 2 };
        let mut o = [vec![0.0; 2], vec![0.0; 2], vec![0.0; 2]];
        assert_eq!(render(&p, None, None, &mut o), Err(HpError::MissingSource));
        let p = Params { const_colour: true, scalar_shift: false, ..p };
        assert_eq!(render(&p, None, None, &mut o), Err(HpError::MissingMask));
        let p = Params { n: 5, ..p };
        assert_eq!(render(&p, None, Some(&[0.0; 5]), &mut o), Err(HpError::ShortPlane));
    }

    // ---- HP2: vpermd and the lane-varying operand
    #[test]
    fn vpermd_semantics() {
        let src = [10, 11, 12, 13, 14, 15, 16, 17];
        assert_eq!(vpermd(PERM_FILE, src), [10, 12, 14, 16, 10, 10, 10, 10]);
        assert_eq!(vpermd(PERM_OLD, src), src);
        // only the low 3 bits of an index select the dword
        assert_eq!(vpermd([8 + 3, 0xffff_fff9, 7, 0x10, 0, 0, 0, 0], src), [13, 11, 17, 10, 10, 10, 10, 10]);
    }
    #[test]
    fn file_cells_are_pack_of_even_dwords() {
        assert_eq!(VCells::FILE.perm, [0, 2, 4, 6, 0, 0, 0, 0]);
        assert_eq!(VCells::FILE, VCells { perm: PERM_FILE, ..VCells::STANDIN });
        assert_eq!(Cells::FILE.ymm.perm, Cells::FILE.xmm.perm);
    }
    #[test]
    fn uniform_operand_makes_every_index_equivalent() {
        // the listing proof as a test: for the two uniform operands the red gate is the same for ANY index words
        let mut s: u64 = 99;
        let mut rnd = || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (s >> 33) as u32
        };
        for wide in [true, false] {
            for op in [mask8(true), mask8(false)] {
                for _ in 0..500 {
                    let mut k = VCells::FILE;
                    k.perm = [rnd(), rnd(), rnd(), rnd(), rnd(), rnd(), rnd(), rnd()];
                    assert_eq!(red_kept(&k, wide, op), op[0] == 0xffff_ffff);
                }
            }
        }
    }
    #[test]
    fn lane_varying_operand_depends_on_the_file_index() {
        // lanes 0,2,4,6 ones, odd lanes zero: the file index keeps the red term, the identity does not
        let even = [!0, 0, !0, 0, !0, 0, !0, 0];
        for wide in [true, false] {
            assert!(red_kept(&VCells::FILE, wide, even));
            assert!(!red_kept(&VCells::STANDIN, wide, even));
        }
        // lane 1 zero only: file index skips lane 1 (kept), identity reads it (not kept)
        let l1 = [!0, 0, !0, !0, !0, !0, !0, !0];
        assert!(red_kept(&VCells::FILE, true, l1) && !red_kept(&VCells::STANDIN, true, l1));
        // lanes 0..3 ones, 4..7 zero: identity keeps, file does not (idx 4 and 6 read zeros)
        let low = [!0, !0, !0, !0, 0, 0, 0, 0];
        assert!(!red_kept(&VCells::FILE, false, low) && red_kept(&VCells::STANDIN, false, low));
        // a forced operand changes the pixel: colour with r > 0, operand even-ones: kept (file) vs dropped (identity)
        let col = [0.8, 0.3, 0.2];
        let a = vec_pixel(&VCells::FILE, true, col, 0.2, Some(even));
        let b = vec_pixel(&VCells::STANDIN, true, col, 0.2, Some(even));
        let nat = vec_pixel(&VCells::FILE, true, col, 0.2, None);
        assert_eq!(a, nat);
        assert_ne!(a, b);
    }
    #[test]
    fn natural_outputs_are_identical_for_file_and_old_indices() {
        let mut s: u64 = 4242;
        let mut rnd = || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((s >> 40) as f32) / 16777216.0
        };
        for _ in 0..2000 {
            let col = [rnd() * 2.4 - 0.8, rnd() * 2.4 - 0.8, rnd() * 2.4 - 0.8];
            let sh = rnd() * 2.0 - 1.0;
            for wide in [true, false] {
                let a = vec_pixel(&VCells::FILE, wide, col, sh, None);
                let b = vec_pixel(&VCells::STANDIN, wide, col, sh, None);
                for k in 0..3 {
                    assert_eq!(a[k].to_bits(), b[k].to_bits());
                }
            }
        }
    }
}
