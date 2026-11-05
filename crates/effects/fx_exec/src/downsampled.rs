//! DS1: the `exec-downsampled` command (vtable, execute) as explicit-parameter Rust, std only.
//!
//! What the original does (STATIC, listing; MEASURED on the emulator, see README.md of this folder):
//! 1. factor f = 2^blur_res (the command prints `exec-downsampled f dest (srcs) (body)`), L = log2(f) levels (= log2f).
//! 2. every source plane is halved L times: dims (w+1)>>1, kernel (AVX2 tier of the CPU word dispatcher) = 3x3 binomial
//!    [1 2 1; 2 4 2; 1 2 1] / 16 centred on the even sample (2x, 2y), edge replicate, plus a constant 7/4096 inside the sum (see `down_plane`).
//! 3. the body runs on a cloned context whose frame size is ceil(W/f) x ceil(H/f), whose geometry floats ctx are divided by f
//!    (this module does not run the body; `exec.rs` does that on the small planes).
//! 4. the destination planes are doubled L times: pass 1 kernel (horizontal), pass 2 kernel (vertical), then a row copy.
//!
//! Data cells (tiers): 2.0, 4.0, 1/16, 7/4096 and 0.5 are MEASURED-DYLIB (`__TEXT,__const`
//! dvc.py). The CPU word dispatch picks the AVX2 body (ROUTE AVX2-ONLY, as every other kernel of this project).
//! The down kernel has three operation orders for the same formula: S (scalar head, tail and short rows), V (vector body, 16 or 8
//! wide) and the edge formulas; which interior output takes which order depends on the row length and on the destination row address
//! modulo 32 (ALIGN: the allocator address of the small plane, `dst_phase`; MEASURED 0 in all 24 emulator runs, a host value otherwise).

pub const BIAS: f32 = 0.001708984375;
const C16: f32 = 0.0625;

/// MUTANT switch for the W63 mutation sweep (env DS_MUT=n, 0 = the port as written); read once.
fn mu() -> u32 {
    use std::sync::OnceLock;
    static M: OnceLock<u32> = OnceLock::new();
    *M.get_or_init(|| std::env::var("DS_MUT").ok().and_then(|v| v.parse().ok()).unwrap_or(0))
}
fn bias() -> f32 {
    if mu() == 1 { f32::from_bits(BIAS.to_bits() + 1) } else { BIAS }
}
fn c16() -> f32 {
    if mu() == 14 { f32::from_bits(C16.to_bits() + 1) } else { C16 }
}

#[inline]
fn fma_s(a: f32, b: f32, c: f32) -> f32 {
    a.mul_add(b, c)
}

/// DS2: an FMA site that the mutation sweep can unfuse (mutant `id` = multiply, round, add)
#[inline]
fn fma_m(a: f32, b: f32, c: f32, id: u32) -> f32 {
    if mu() == id { a * b + c } else { a.mul_add(b, c) }
}

/// order S (scalar head, tail, rows with fewer than 16 interior outputs): 
#[inline]
fn ord_s(r0: f32, r1: f32, rm: f32, u0: f32, u1: f32, um: f32, d0: f32, d1: f32, dm: f32) -> f32 {
    let x6 = um + dm;
    let x3 = r1 + u0;
    let x4 = d0 + rm;
    let x7 = x6 + u1;
    let x8 = fma_m(r0, 4.0, d1, 16);
    let x13 = x3 + x4;
    let x13 = fma_m(x13, 2.0, bias(), 17);
    let x12 = x7 + x8;
    c16() * (x12 + x13)
}

/// order V (16 wide body and 8 wide body)
#[inline]
fn ord_v(r0: f32, r1: f32, rm: f32, u0: f32, u1: f32, um: f32, d0: f32, d1: f32, dm: f32) -> f32 {
    let t = d0 + (u0 + r1);
    let s1 = t + rm;
    let x = fma_m(s1, 2.0, bias(), 18);
    let y = if mu() == 15 { fma_s(r0, 4.0, d1) } else { fma_m(r0, 4.0, u1, 19) };
    let z = d1 + um;
    c16() * (((x + y) + z) + dm)
}

/// left edge element x = 0 
#[inline]
fn left_edge(r0: f32, r1: f32, u0: f32, u1: f32, d0: f32, d1: f32) -> f32 {
    let x0 = r0 + r1;
    let x8 = fma_m(r0, 4.0, d0, 20);
    let x2 = u0 + d0;
    let x4 = if mu() == 13 { u1 + d0 } else { u1 + d1 };
    let x13 = x0 + x2;
    let x7 = x4 + u0;
    let x13 = fma_m(x13, 2.0, bias(), 21);
    let x12 = x7 + x8;
    c16() * (x12 + x13)
}

/// right edge element x = w2 - 1; cr = the clamped right column
#[inline]
fn right_edge(rm: f32, r0: f32, r1c: f32, u0: f32, u1c: f32, um: f32, d0: f32, d1c: f32, dm: f32) -> f32 {
    let x8 = fma_m(r0, 4.0, dm, 22);
    let x3 = rm + u0;
    let x4 = d0 + r1c;
    let x6 = u1c + d1c;
    let x13 = x3 + x4;
    let x7 = x6 + um;
    let x13 = fma_m(x13, 2.0, bias(), 23);
    let x12 = x7 + x8;
    c16() * (x12 + x13)
}

/// Which interior outputs (j = 0 .. n-1, output x = j + 1) take order V: `true`. `row_addr_plus4` = (destination row address + 4) in bytes,
/// only its value modulo 32 matters (the alignment peel).
pub fn v_classes(n: usize, row_addr_plus4: u64) -> Vec<bool> {
    let mut cl = vec![false; n];
    if n < (if mu() == 3 { 17 } else { 16 }) {
        return cl;
    }
    let h;
    let lim;
    if n < 29 {
        h = 0usize;
        lim = n & !15usize;
    } else {
        let m = (row_addr_plus4 & 31) as usize;
        if m == 0 {
            h = if mu() == 4 { 8 } else { 0 };
        } else if m & 3 != 0 {
            return cl;
        } else {
            h = (32 - m) >> 2;
        }
        if n < h + 16 {
            return cl;
        }
        lim = n - ((n - h) & 15);
    }
    let mut j = h;
    loop {
        for q in 0..16 {
            cl[j + q] = true;
        }
        j += 16;
        if j >= lim {
            break;
        }
    }
    let rem = n - lim;
    if rem >= (if mu() == 5 { 9 } else { 8 }) {
        for q in 0..8 {
            cl[lim + q] = true;
        }
    }
    cl
}

/// One halving: `big` is w x h (row stride w), the result is w2 x h2 with w2 = (w+1)>>1, h2 = (h+1)>>1 (row stride w2).
/// `dst_phase` = destination plane base address modulo 32 (bytes). Requires w >= 3 and h >= 1 (w = 1, 2 are NOT claimed: the listing reads
/// column -1 for w2 = 1).
pub fn down_plane(big: &[f32], w: usize, h: usize, dst_phase: u32) -> Vec<f32> {
    assert!(w >= 3 && h >= 1 && big.len() >= w * h);
    let (w2, h2) = ((w + 1) >> 1, (h + 1) >> 1);
    let mut out = vec![0.0f32; w2 * h2];
    let n = w2 as isize - 2;
    for y in 0..h2 {
        let rc = &big[2 * y * w..2 * y * w + w];
        let up: &[f32] = if y == 0 && mu() != 9 { rc } else { &big[(2 * y - 1) * w..(2 * y - 1) * w + w] };
        let dn: &[f32] = if (if mu() == 10 { 2 * y > h - 1 } else { 2 * y >= h - 1 }) { rc } else { &big[(2 * y + 1) * w..(2 * y + 1) * w + w] };
        let o = &mut out[y * w2..y * w2 + w2];
        o[0] = left_edge(rc[0], rc[1], up[0], up[1], dn[0], dn[1]);
        if n > 0 {
            let n = n as usize;
            let cl = v_classes(n, dst_phase as u64 + 4 * (y * w2) as u64 + 4);
            for j in 0..n {
                let c = 2 * (j + 1);
                let a = (rc[c], rc[c + 1], rc[c - 1], up[c], up[c + 1], up[c - 1], dn[c], dn[c + 1], dn[c - 1]);
                o[j + 1] = if cl[j] && mu() != 2 { ord_v(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8) } else { ord_s(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8) };
            }
        }
        if w2 >= 2 {
            let c = 2 * w2 - 2;
            let cr = if mu() == 6 { 2 * w2 - 1 } else { (2 * w2 - 1).min(w - 1) };
            o[w2 - 1] = right_edge(rc[c - 1], rc[c], rc[cr], up[c], up[cr], up[c - 1], dn[c], dn[cr], dn[c - 1]);
        }
    }
    out
}

/// One doubling: `small` is w2 x h2 (row stride w2), the result is w x h (row stride w) with w2 = (w+1)>>1, h2 = (h+1)>>1.
/// Pass 1 : row 2i: even columns copy, odd columns 0.5 * (a + b), the tail from column 2*w2-2 repeats the last sample.
/// Pass 2 : odd rows 2i+1 (i < (h-1)/2) = 0.5 * (row 2i + row 2i+2). Then (caller) rows 2*h2-1 h-1 copy the previous row.
pub fn up_plane(small: &[f32], w2: usize, h2: usize, w: usize, h: usize) -> Vec<f32> {
    assert!(w2 >= 1 && h2 >= 1 && small.len() >= w2 * h2);
    assert!(h2 == (h + 1) >> 1 && 2 * w2 <= w + 2); // DS2: the width contract w2 == (w + 1) >> 1 is relaxed to 2*w2 <= w + 2 only for the out-of-contract tail probes (the executor always passes w2 == (w + 1) >> 1)
    let mut big = vec![0.0f32; w * h];
    for i in 0..h2 {
        let r = 2 * i;
        if r >= h {
            break;
        }
        let s = &small[i * w2..i * w2 + w2];
        let row = &mut big[r * w..r * w + w];
        for x in 0..w2.saturating_sub(1) {
            row[2 * x] = s[x];
            row[2 * x + 1] = if mu() == 24 { s[x] * 0.5 + s[x + 1] * 0.5 } else { (s[x] + s[x + 1]) * (if mu() == 7 { 0.50000006 } else { 0.5 }) };
        }
        let t = s[w2 - 1];
        if mu() != 8 {
            for k in (if mu() == 26 { 2 * w2 - 1 } else { 2 * w2 - 2 })..w {
                row[k] = t;
            }
        }
    }
    let cnt = if mu() == 12 { h / 2 } else { (h - 1) / 2 };
    for i in 0..cnt {
        for k in 0..w {
            let v = if mu() == 25 { big[2 * i * w + k] * 0.5 + big[(2 * i + 2) * w + k] * 0.5 } else { (big[2 * i * w + k] + big[(2 * i + 2) * w + k]) * 0.5 };
            big[(2 * i + 1) * w + k] = v;
        }
    }
    let first = if mu() == 11 { h } else { 2 * h2 - 1 };
    for r in first..h {
        if r == 0 {
            break;
        }
        let (a, b) = big.split_at_mut(r * w);
        b[..w].copy_from_slice(&a[(r - 1) * w..r * w]);
    }
    big
}

/// L halvings of one plane: returns (plane, w_L, h_L). L = 0 returns the input unchanged.
pub fn downsample_plane(plane: &[f32], w: usize, h: usize, levels: u32, dst_phase: u32) -> (Vec<f32>, usize, usize) {
    let mut cur = plane.to_vec();
    let (mut cw, mut ch) = (w, h);
    for _ in 0..levels {
        cur = down_plane(&cur, cw, ch, dst_phase);
        cw = (cw + 1) >> 1;
        ch = (ch + 1) >> 1;
    }
    (cur, cw, ch)
}

/// L doublings of one plane back to w x h (level l target dims = ceil(w / 2^l)).
pub fn upsample_plane(small: &[f32], w: usize, h: usize, levels: u32) -> Vec<f32> {
    let dims = |l: u32| ((w + (1usize << l) - 1) >> l, (h + (1usize << l) - 1) >> l);
    let mut cur = small.to_vec();
    for l in (0..levels).rev() {
        let (tw, th) = dims(l);
        let (sw, sh) = dims(l + 1);
        cur = up_plane(&cur, sw, sh, tw, th);
    }
    cur
}

/// frame size after `levels` halvings
pub fn small_dims(w: usize, h: usize, levels: u32) -> (usize, usize) {
    let (mut cw, mut ch) = (w, h);
    for _ in 0..levels {
        cw = (cw + 1) >> 1;
        ch = (ch + 1) >> 1;
    }
    (cw, ch)
}

/// blur_res -> levels (the factor is 2^levels); None when the program has no blur_res popup node
pub fn levels_of_factor(f: u32) -> u32 {
    f.trailing_zeros()
}
