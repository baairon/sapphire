//! convert_pair: native port of the original convert-from-source (reader)
//! and convert-to-dest (writer) for format code 16 (host memory B,G,R,A, f32),
//! four planes, full-frame view (no edge margins). std only, safe Rust, explicit parameters, no Default.
//!
//! Host memory of one pixel (format 16): m[0]=B, m[1]=G, m[2]=R, m[3]=A. Internal planes: 0=R, 1=G, 2=B, 3=A.
//! Reader (MEASURED, per pixel scalar): every sample passes the finite test of 
//! (|bits| < 0x7f800000, otherwise the sample is replaced by +0.0); a = finite(A); plane0 = finite(R) * a, plane1 = finite(G) * a,
//! plane2 = finite(B) * a, plane3 = a. The product is a plain f32 multiply (mulss, no FMA).
//! Writer (MEASURED, for mode 1, for modes 2 and 3 (3 channels written)): dest = (plane2, plane1, plane0, alpha)
//! with colour bits copied unchanged; alpha by mode, see [`AlphaMode`].

use std::sync::atomic::{AtomicUsize, Ordering};

static MUT: AtomicUsize = AtomicUsize::new(0);

/// Mutation switch used by the mutation sweep (feature `mutants`); 0 = original behaviour.
pub fn set_mutant(n: usize) {
    MUT.store(n, Ordering::Relaxed);
}

#[inline]
fn mu(i: usize) -> bool {
    cfg!(feature = "mutants") && MUT.load(Ordering::Relaxed) == i
}

/// Alpha mode byte at ctx as the executor sees it after the context initialiser and the converter's own adjustment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlphaMode {
    /// 0: the writer reports an error and writes nothing: the destination keeps its previous bytes.
    NoWrite,
    /// 1: alpha is clamped: a < 0 -> +0.0, NaN stays (bits unchanged), a > 1 -> 1.0, else a.
    ClampKeep,
    /// 2: colour written (3 channels), alpha written as +0.0 (MEASURED: not the float at ctx, which only fills out-of-extent margins).
    ZeroAlpha,
    /// 3: colour written (3 channels), alpha = 1.0.
    One,
}

impl AlphaMode {
    pub fn from_byte(b: i8) -> Option<AlphaMode> {
        match b {
            0 => Some(AlphaMode::NoWrite),
            1 => Some(AlphaMode::ClampKeep),
            2 => Some(AlphaMode::ZeroAlpha),
            3 => Some(AlphaMode::One),
            _ => None,
        }
    }
}

/// Mode derivation of the context initialiser (stores; the same rule in)
/// followed by the converter's adjustment (: mode 1 with an odd plane count becomes 3). All four inputs are explicit.
/// `owner_override` = dword [owner] (nonzero wins), `program_mode` = dword [program], `ctx_mode_dword` = dword [ctx],
/// `dest_field_cell_a` = dword [[ctx]], `planes` = plane count of the image (odd count: mode 1 -> 3).
pub fn derive_alpha_mode(owner_override: i32, program_mode: i32, ctx_mode_dword: i32, dest_field_cell_a: i32, planes: usize) -> i8 {
    let r8 = if (owner_override != 0) != mu(16) { owner_override } else { program_mode } as i8;
    let mut m = r8;
    if r8 == if mu(17) { 1 } else { 2 } {
        if (ctx_mode_dword == 0) != mu(18) {
            m = 3;
        } else if dest_field_cell_a == if mu(19) { 2 } else { 3 } {
            m = 1;
        }
    }
    if m == 1 && (planes & 1) == if mu(20) { 0 } else { 1 } {
        m = if mu(21) { 2 } else { 3 };
    }
    m
}

/// 1 when the sample is finite (bits & 0x7fffffff < 0x7f800000).
#[inline]
pub fn is_finite_sample(x: f32) -> bool {
    let b = x.to_bits() & 0x7fff_ffff;
    if mu(1) {
        return b <= 0x7f80_0000;
    }
    b < 0x7f80_0000
}

#[inline]
fn finite_or_zero(x: f32) -> f32 {
    if is_finite_sample(x) {
        x
    } else {
        0.0
    }
}

/// Mode 1 alpha clamp (scalar tail and vector agree): vcomiss 0 vs a, jbe -> vminss(1.0, a); else +0.0.
#[inline]
pub fn clamp_alpha(a: f32) -> f32 {
    if !mu(2) && 0.0f32 > a {
        return 0.0;
    }
    if mu(3) && a.is_nan() {
        return 0.0;
    }
    // vminss dest, src1 = 1.0, src2 = a: result = if 1.0 < a { 1.0 } else { a } (NaN -> src2 = a)
    if mu(4) {
        return if 1.0f32 <= a { 1.0 } else { a };
    }
    if 1.0f32 < a {
        1.0
    } else {
        a
    }
}

/// Convert-from-source for one line of `n` pixels. `mem` holds 4*n floats (B,G,R,A per pixel); `planes[k][i]` receive the planes.
pub fn convert_from_source_line(mem: &[f32], planes: &mut [Vec<f32>; 4], n: usize) {
    for p in planes.iter_mut() {
        p.clear();
        p.resize(n, 0.0);
    }
    for i in 0..n {
        let m0 = mem[4 * i];
        let m1 = mem[4 * i + 1];
        let m2 = mem[4 * i + 2];
        let m3 = mem[4 * i + 3];
        let a = if mu(5) { m3 } else { finite_or_zero(m3) };
        let r = if mu(6) { m2 } else { finite_or_zero(m2) };
        let g = finite_or_zero(m1);
        let b = finite_or_zero(m0);
        planes[0][i] = if mu(7) { r } else { r * a };
        planes[1][i] = if mu(8) { g * g } else { g * a };
        planes[2][i] = if mu(9) { b * a.abs() } else { b * a };
        planes[3][i] = if mu(10) { 1.0 } else { a };
    }
}

/// Explicit parameters of the writer. No Default.
#[derive(Clone, Copy, Debug)]
pub struct ToDestParams {
    pub mode: AlphaMode,
}

/// Outcome of the writer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DestOutcome {
    Written,
    /// Mode 0: nothing written (the original logs an error through).
    NotWritten,
}

/// Convert-to-dest for one line of `n` pixels: `dest` has 4*n floats (B,G,R,A).
pub fn convert_to_dest_line(planes: &[Vec<f32>; 4], p: ToDestParams, dest: &mut [f32], n: usize) -> DestOutcome {
    if p.mode == AlphaMode::NoWrite {
        return DestOutcome::NotWritten;
    }
    for i in 0..n {
        dest[4 * i] = if mu(11) { planes[0][i] } else { planes[2][i] };
        dest[4 * i + 1] = planes[1][i];
        dest[4 * i + 2] = if mu(12) { planes[2][i] } else { planes[0][i] };
        dest[4 * i + 3] = match p.mode {
            AlphaMode::ClampKeep => clamp_alpha(planes[3][i]),
            AlphaMode::ZeroAlpha => {
                if mu(13) {
                    planes[3][i]
                } else {
                    0.0
                }
            }
            AlphaMode::One => {
                if mu(14) {
                    clamp_alpha(planes[3][i])
                } else {
                    1.0
                }
            }
            AlphaMode::NoWrite => unreachable!(),
        };
    }
    DestOutcome::Written
}

/// the converter band replication (loop, memcpy): after the line at row `y` is written, rows y+1 ..= min(y+cell-1, last_row)
/// receive a copy of the written bytes. `dest` is the whole destination (rows of `n` pixels, 4 floats each).
pub fn replicate_band(dest: &mut [f32], n: usize, y: usize, cell: usize, last_row: usize) {
    if cell <= 1 {
        return;
    }
    let mut count = cell - 1;
    if mu(15) {
        count = cell;
    }
    if last_row < y + cell - 1 {
        count = last_row - y;
    }
    let row = 4 * n;
    for k in 1..=count {
        let (a, b) = dest.split_at_mut((y + k) * row);
        b[..row].copy_from_slice(&a[y * row..(y + 1) * row]);
    }
}

/// Whole-frame pair for cell `cell`: bands start at rows 0, cell, 2*cell, ... (the original's exec-linewise worker converts the first row
/// of each band, MEASURED: cell 1 -> one call per row, cell 4096 -> one call). `src`: h rows of w pixels in host memory order.
pub fn convert_pair_frame(src: &[f32], w: usize, h: usize, cell: usize, p: ToDestParams, dest: &mut [f32]) -> usize {
    let mut planes: [Vec<f32>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut calls = 0;
    let mut y = 0;
    while y < h {
        convert_from_source_line(&src[4 * w * y..4 * w * (y + 1)], &mut planes, w);
        let _ = convert_to_dest_line(&planes, p, &mut dest[4 * w * y..4 * w * (y + 1)], w);
        replicate_band(dest, w, y, cell, h - 1);
        calls += 1;
        y += cell.max(1);
    }
    calls
}


// ---------------------------------------------------------------------------------------------------------------------------
// the 3-plane branch (destination image with 3 planes, the Source of every no_alpha_only program).
// Reader: format 16, plane count 3: dispatcher (cmp eax,3) -> (SSE-flag variant: a 16-pixel chunk loop
// plain variant:; both read B,G,R,A of every host pixel, finite-test each sample with,
// multiply the three colours by the finite alpha with mulss, and store ONLY three plane rows: the alpha plane (a fourth pointer) is never read
// or written, the alpha value is a weight that is then dropped). Writer: with edx = 3 -> table, the same
// three-plane body the 4-plane image reaches in modes 2 and 3: it loads three plane pointers, mode 2 -> alpha +0.0 (path), modes 1 and 3
// alpha 1.0 (share), any other mode value logs and writes nothing.
// ---------------------------------------------------------------------------------------------------------------------------

/// Convert-from-source for one line of `n` pixels into a 3-plane image: `planes[0..3]` receive R*a, G*a, B*a; `planes[3]` is left empty (the original
/// has no fourth plane pointer to write). `mem` holds 4*n floats (B,G,R,A per pixel).
pub fn convert_from_source_line_3(mem: &[f32], planes: &mut [Vec<f32>; 4], n: usize) {
    for k in 0..3 {
        planes[k].clear();
        planes[k].resize(n, 0.0);
    }
    planes[3].clear();
    for i in 0..n {
        let m0 = mem[4 * i];
        let m1 = mem[4 * i + 1];
        let m2 = mem[4 * i + 2];
        let m3 = mem[4 * i + 3];
        let a = if mu(24) { m3 } else { finite_or_zero(m3) };
        let r = if mu(25) { m2 } else { finite_or_zero(m2) };
        let g = finite_or_zero(m1);
        let b = finite_or_zero(m0);
        let (p0, p2) = if mu(22) { (b * a, r * a) } else { (r * a, b * a) };
        planes[0][i] = p0;
        planes[1][i] = if mu(23) { g } else { g * a };
        planes[2][i] = p2;
    }
}

/// Convert-to-dest for one line of `n` pixels from a 3-plane result; `dest` has 4*n floats (B,G,R,A). Mode 1 behaves as mode 3 (alpha 1.0), mode 2
/// writes +0.0, mode 0 writes nothing. The source alpha is not available (no fourth plane).
pub fn convert_to_dest_line_3(planes: &[Vec<f32>; 4], p: ToDestParams, dest: &mut [f32], n: usize) -> DestOutcome {
    if p.mode == AlphaMode::NoWrite && !mu(29) {
        return DestOutcome::NotWritten;
    }
    let alpha = match p.mode {
        AlphaMode::ZeroAlpha => {
            if mu(27) {
                1.0
            } else {
                0.0
            }
        }
        AlphaMode::ClampKeep => {
            if mu(26) {
                0.0
            } else {
                1.0
            }
        }
        AlphaMode::One | AlphaMode::NoWrite => 1.0,
    };
    for i in 0..n {
        dest[4 * i] = if mu(28) { planes[0][i] } else { planes[2][i] };
        dest[4 * i + 1] = planes[1][i];
        dest[4 * i + 2] = if mu(28) { planes[2][i] } else { planes[0][i] };
        dest[4 * i + 3] = alpha;
    }
    DestOutcome::Written
}

/// Whole-frame pair for a 3-plane image (see [`convert_pair_frame`]).
pub fn convert_pair_frame_3(src: &[f32], w: usize, h: usize, cell: usize, p: ToDestParams, dest: &mut [f32]) -> usize {
    let mut planes: [Vec<f32>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut calls = 0;
    let mut y = 0;
    while y < h {
        convert_from_source_line_3(&src[4 * w * y..4 * w * (y + 1)], &mut planes, w);
        let _ = convert_to_dest_line_3(&planes, p, &mut dest[4 * w * y..4 * w * (y + 1)], w);
        replicate_band(dest, w, y, cell, h - 1);
        calls += 1;
        y += cell.max(1);
    }
    calls
}


// ---------------------------------------------------------------------------------------------------------------------------
// the 1- and 2-plane destination arms (Blur_Mono ends in a 2-plane image: gray, alpha). Mutants 40..49.
// Selection (MEASURED listing, the converter): : n = min([dest], dword [ctx]) (signed); the writer receives edx = n for mode 1
// (: an odd n turns the mode into 3, byte [ctx] rewritten, and the adjustment is skipped), and edx = n + (n & 1) - 1 for modes 0, 2, 3.
// So a 2-plane image reaches the 2-plane arm only in mode 1; in modes 0, 2, 3 it is written by the 1-plane arm (edx = 1), and a 1-plane image always
// reaches the 1-plane arm (mode 1 is turned into 3). Writer dispatch (cmp edx,1), (cmp edx,2); format 16 (idx 14):
// 1-plane table, 2-plane table; with stack word [rbp] == 0 the 1-plane arm is (AVX2 body
// ) and the 2-plane arm is the inline body (cell [rbp] == 1, mode [rbp] == 1).
// ---------------------------------------------------------------------------------------------------------------------------

/// Writer plane-count argument edx and the (possibly rewritten) mode, as the converter computes them. `image_planes` = dword [dest image] (the plane count of
/// the result image, read into ecx and tested), `to_dest_params_a` = dword [ctx], `mode` = byte [ctx] (signed). n = min(image_planes, to_dest_params_a).
/// Mode 1: edx = n, and when the IMAGE plane count (not n) is odd the mode becomes 3; modes 0, 2, 3: edx = n + (n & 1) - 1. Returns (edx, mode).
pub fn writer_args(image_planes: i32, to_dest_params_a: i32, mode: i8) -> (i32, i8) {
    let n = if (image_planes < to_dest_params_a) != mu(47) { image_planes } else { to_dest_params_a };
    if mode == 1 {
        if (image_planes & 1) == if mu(40) { 0 } else { 1 } {
            return (n, 3);
        }
        return (n, 1);
    }
    let adj = if mu(41) { n - (n & 1) - 1 } else { n + (n & 1) - 1 };
    (adj, mode)
}

/// Outcome of the 1- and 2-plane writers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dest12 {
    Written,
    /// Mode 0 (and any mode value other than 1, 2, 3): the arm logs through and writes nothing (1-plane arm).
    NotWritten,
    /// A (plane count, mode) pair the converter never produces (1-plane arm with mode 1, 2-plane arm with mode other than 1): not ported, refused.
    NotReachable,
}

/// 1-plane arm, one line of `n` pixels, cell 1: dest = (g, g, g, alpha) with g = planes[0][i] copied bit for bit (NaN, inf, -0 unchanged);
/// mode 3: alpha 1.0; mode 2: alpha +0.0; mode 0: nothing written; mode 1 (MEASURED in the T2 runs, reachable only when ctx < the image plane count, since a
/// 1-plane image turns mode 1 into 3): alpha = the mode 1 clamp of the gray value (scalar ff. / body).
pub fn convert_to_dest_line_1(planes: &[Vec<f32>; 4], p: ToDestParams, dest: &mut [f32], n: usize) -> Dest12 {
    let alpha_from_gray = p.mode == AlphaMode::ClampKeep;
    let alpha = match p.mode {
        AlphaMode::NoWrite => return Dest12::NotWritten,
        AlphaMode::ClampKeep => 0.0,
        AlphaMode::ZeroAlpha => {
            if mu(42) {
                1.0
            } else {
                0.0
            }
        }
        AlphaMode::One => {
            if mu(43) {
                0.0
            } else {
                1.0
            }
        }
    };
    for i in 0..n {
        let g = planes[0][i];
        dest[4 * i] = g;
        dest[4 * i + 1] = if mu(44) { 0.0 } else { g };
        dest[4 * i + 2] = g;
        dest[4 * i + 3] = if alpha_from_gray {
            if mu(49) {
                g
            } else {
                clamp_alpha(g)
            }
        } else {
            alpha
        };
    }
    Dest12::Written
}

/// 2-plane arm, one line of `n` pixels, cell 1, mode 1: dest = (g, g, g, clamp(a)) with g = planes[0][i] bit for bit and a = planes[1][i], the mode 1 clamp
/// of the 4-plane arm (a < 0 -> +0.0, NaN unchanged, a > 1 -> 1.0). Scalar head and tail (vcomiss, vminss) and the 8-wide body (vcmpltps, vminps, vandnps) agree.
pub fn convert_to_dest_line_2(planes: &[Vec<f32>; 4], p: ToDestParams, dest: &mut [f32], n: usize) -> Dest12 {
    if p.mode != AlphaMode::ClampKeep {
        return Dest12::NotReachable;
    }
    for i in 0..n {
        let g = planes[0][i];
        dest[4 * i] = g;
        dest[4 * i + 1] = if mu(48) { planes[1][i] } else { g };
        dest[4 * i + 2] = if mu(45) { planes[1][i] } else { g };
        dest[4 * i + 3] = if mu(46) { planes[1][i] } else { clamp_alpha(planes[1][i]) };
    }
    Dest12::Written
}

/// Convert-to-dest for a result image with `image_planes` planes (1 or 2 handled here, 3 and 4 by the existing arms): picks the writer arm like the converter.
/// `to_dest_params_a` = dword [ctx]. Returns the writer's edx and the outcome.
pub fn convert_to_dest_line_n12(image_planes: i32, to_dest_params_a: i32, mode_byte: i8, planes: &[Vec<f32>; 4], dest: &mut [f32], n: usize) -> (i32, Dest12) {
    let (edx, m) = writer_args(image_planes, to_dest_params_a, mode_byte);
    let mode = match AlphaMode::from_byte(m) {
        Some(x) => x,
        None => return (edx, Dest12::NotWritten),
    };
    let p = ToDestParams { mode };
    let o = match edx {
        1 => convert_to_dest_line_1(planes, p, dest, n),
        2 => convert_to_dest_line_2(planes, p, dest, n),
        _ => Dest12::NotReachable,
    };
    (edx, o)
}

/// Whole-frame helper for planes already produced (host stand-in for the executor): `lines[y]` holds the planes of row y; band replication as the converter.
pub fn convert_dest_frame_n12(image_planes: i32, to_dest_params_a: i32, mode_byte: i8, planes_rows: &[[Vec<f32>; 4]], w: usize, h: usize, cell: usize, dest: &mut [f32]) -> usize {
    let mut calls = 0;
    let mut y = 0;
    while y < h {
        let _ = convert_to_dest_line_n12(image_planes, to_dest_params_a, mode_byte, &planes_rows[y], &mut dest[4 * w * y..4 * w * (y + 1)], w);
        replicate_band(dest, w, y, cell, h - 1);
        calls += 1;
        y += cell.max(1);
    }
    calls
}

// ---------------------------------------------------------------------------------------------------------------------------
//: the class-3 pair of the 'ae32' route, format 14 host words A, R, G, B
// per pixel (u32 bits, the first dword = A). Source: variant B head (listing text, both loops): the second and third dwords,
// through the finite test, no read of the first dword, no multiply (CONVERT-VARIANT-B-UNWEIGHTED, MEASURED r1b). Destination: the mode-2
// three-plane body stores the second and third dwords, only; slot 0 is the host source row's slot 0 by the row copier (14, 14)
// (DEST-ALPHA-FROM-SOURCE-ROW, MEASURED r1b and r1c). Variant C is NOT here (Opus 57).
// Mutants are fixed at build time by env CONVERT_PAIR_G3_MUT (one build per mutant, each binary's SHA-256 recorded):
// 1 planes weighted by alpha, 2 slot 0 written 0.0, 3 slot 0 written 1.0, 4 planes 4, 5 finite test removed.
// ---------------------------------------------------------------------------------------------------------------------------

const G3_MUT: &str = match option_env!("CONVERT_PAIR_G3_MUT") {
    Some(s) => s,
    None => "0",
};

#[inline]
fn g3m(k: &str) -> bool {
    G3_MUT == k
}

/// The mutant tag this build carries ("0" = the port).
pub fn g3_mutant() -> &'static str {
    G3_MUT
}

/// Plane count of a class-3 input (the lane count: three colour lanes, no alpha lane).
pub fn class3_planes() -> usize {
    if g3m("4") {
        4
    } else {
        3
    }
}

#[inline]
fn fin_bits_g3(b: u32) -> u32 {
    if g3m("5") || (b & 0x7fff_ffff) < 0x7f80_0000 {
        b
    } else {
        0
    }
}

/// Variant B source line (head): `planes` receives class3_planes rows of `n` samples (bits).
pub fn convert_from_source_line_3_b(mem: &[u32], planes: &mut Vec<Vec<u32>>, n: usize) {
    let np = class3_planes();
    planes.clear();
    planes.resize(np, vec![0u32; n]);
    for i in 0..n {
        let m = &mem[4 * i..4 * i + 4];
        for k in 0..3 {
            let v = fin_bits_g3(m[1 + k]);
            planes[k][i] = if g3m("1") { (f32::from_bits(v) * f32::from_bits(fin_bits_g3(m[0]))).to_bits() } else { v };
        }
        if np == 4 {
            planes[3][i] = fin_bits_g3(m[0]);
        }
    }
}

/// Mode-2 three-plane destination line (body, then the row copier for (14, 14) from the host source row `src_row`).
pub fn convert_to_dest_line_3_m2(planes: &[Vec<u32>], src_row: &[u32], dest: &mut [u32], n: usize) -> Result<(), String> {
    if planes.len() != 3 {
        return Err(format!("CLASS3-PLANES-MISMATCH: the mode-2 three-plane body takes 3 planes, {} given", planes.len()));
    }
    for i in 0..n {
        dest[4 * i + 1] = planes[0][i];
        dest[4 * i + 2] = planes[1][i];
        dest[4 * i + 3] = planes[2][i];
        dest[4 * i] = if g3m("2") {
            0
        } else if g3m("3") {
            1.0f32.to_bits()
        } else {
            src_row[4 * i]
        };
    }
    Ok(())
}
