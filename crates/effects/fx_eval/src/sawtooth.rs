//! Constant path of the `random_sawtooth` builtin: a hash-driven random value with a dwell dead zone.
//!
//! Label: APPROX-FREE. Exact f32 and f64 operations in the order of the AVX2/FMA body, `mul_add` at the
//! four fused sites, integer hash rounds, std only, no platform libm.
//!
//! Row 1024 is the 12 bytes after the table in the original's memory, which no instruction writes and which hold zero; a search
//! that ends past the last row reads it. Stand-in label: `SAWTOOTH-SEED-CELL` (the seed is argument 0, scaled by 4096 and truncated).

use std::sync::OnceLock;

const ROWS: usize = 1024;

struct Table {
    /// normalized weights (row 1024 is the zero row after the table)
    a: [f32; ROWS + 1],
    /// running sum of the normalized weights before each row
    c: [f32; ROWS + 1],
    /// the hash chain value of each row
    h: [u32; ROWS + 1],
}

fn hash_rounds() -> &'static [u32; 256] {
    static T: OnceLock<[u32; 256]> = OnceLock::new();
    T.get_or_init(|| {
        let step = |x: u64| ((x << 12) + 150889) % 714025;
        let mut t = [0u32; 256];
        let mut st = 0u64;
        for e in t.iter_mut() {
            let y1 = step(st);
            let y2 = step(y1);
            let y3 = step(y2);
            *e = ((y1 >> 2) as u32)
                .wrapping_add((y2 << 8) as u32)
                .wrapping_add((y3 << 18) as u32);
            st = y3;
        }
        t
    })
}

fn rounds4(t: &[u32; 256], mut v: u32) -> u32 {
    for _ in 0..4 {
        v = (v >> 8).wrapping_add(t[(v & 0xff) as usize]);
    }
    v
}

fn hash1(x: u32) -> u32 {
    rounds4(hash_rounds(), x)
}

fn hash2(x: u32, y: u32) -> u32 {
    let t = hash_rounds();
    rounds4(t, rounds4(t, x).wrapping_add(y))
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let mut a = [0f32; ROWS + 1];
        let mut c = [0f32; ROWS + 1];
        let mut h = [0u32; ROWS + 1];
        let mut sum = 0f32;
        let mut e = hash1(0x1b9e);
        for i in 0..ROWS {
            e = hash1(e.wrapping_add(0x1b9e));
            let m = (e & 0x3fff_ffff) as i32;
            let r = (m as f32) * f32::from_bits(0x3080_0000);
            a[i] = r * r;
            h[i] = e;
            sum += a[i];
        }
        let norm = sum * f32::from_bits(0x3a80_0000);
        let mut cum = 0f32;
        for i in 0..ROWS {
            a[i] /= norm;
            c[i] = cum;
            cum += a[i];
        }
        Table { a, c, h }
    })
}

fn quiet(x: f32) -> f32 {
    f32::from_bits(x.to_bits() | 0x0040_0000)
}

/// cvtss2si on an already integral value, cvttss2si on any value: NaN or out of range gives i32::MIN.
pub fn cvt_i32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// x86 fused multiply-add in f32: the first NaN among the operands in priority order is returned quieted,
/// a fresh NaN is 0xffc00000; `prod` and `sum` are the multiplied pair and the added value.
fn fma32(pri: [f32; 3], x: f32, y: f32, z: f32, neg: bool) -> f32 {
    for p in pri {
        if p.is_nan() {
            return quiet(p);
        }
    }
    let r = if neg { (-x).mul_add(y, z) } else { x.mul_add(y, z) };
    if r.is_nan() { f32::from_bits(0xffc0_0000) } else { r }
}

fn fma64_to32(pri: [f32; 3], x: f64, y: f64, z: f64) -> f32 {
    for p in pri {
        if p.is_nan() {
            return quiet(p);
        }
    }
    let r = x.mul_add(y, z);
    if r.is_nan() { f32::from_bits(0xffc0_0000) } else { r as f32 }
}

/// Binary search over the running sums: the row whose span holds `u`, or the row where the search ends (1024 when `u` is past the
/// last row or unordered).
fn search(t: &Table, u: f32) -> usize {
    let (mut lo, mut hi) = (0i32, ROWS as i32);
    loop {
        let mid = (lo + hi) >> 1;
        let cm = t.c[mid as usize];
        if u >= cm && cm + t.a[mid as usize] > u {
            break mid as usize;
        }
        let cm_ge = cm >= u;
        if cm_ge {
            hi = mid;
        }
        if !cm_ge {
            lo = mid + 1;
        }
        if lo >= hi {
            break lo as usize;
        }
    }
}

/// The per-lane body. `seed` is the integer hash seed, `x` the phase lane, `p1`..`p5` the five scalar
/// parameters (low, high, noise low, noise high, dwell).
pub fn random_sawtooth_body(seed: i32, x: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> f32 {
    random_sawtooth_traced(seed, x, p1, p2, p3, p4, p5).0
}

/// The body plus a flag that is set when the sign division of the dwell step divides a NaN by its own
/// absolute value (both operands NaN, where the silicon rule keeps the first operand).
pub fn random_sawtooth_traced(seed: i32, x: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> (f32, bool) {
    let t = table();
    let s = seed as u32;
    let h0 = (hash1(s.wrapping_add(0x3fd)) & 0x3fff_ffff) as i32;
    let off = ((h0 as f64) * f64::from_bits(0x3e79_0000_0000_0000)) as f32;
    let v = off + x;
    let flo = (v * f32::from_bits(0x3a80_0000)).floor();
    let n = cvt_i32(flo);
    let base = ((n.wrapping_shl(10)) as f32);
    let u = v - base;
    let idx = search(t, u);
    let y = (n as u32).wrapping_add(t.h[idx]);
    let r12 = hash2(s, y);
    let e1 = (hash1(r12) & 0x3fff_ffff) as i32;
    let d = (p4 - p3) as f64;
    let r1 = fma64_to32([p4, p3, p3], e1 as f64, f64::from_bits(0x3e10_0000_0000_0000) * d, p3 as f64);
    let ab = r1.abs();
    let mut adj = 0f32;
    let pair = !(p5 >= ab) && r1.is_nan();
    if !(p5 >= ab) {
        let mut sg = r1 / ab;
        // 0/0 and inf/inf with neither operand NaN make a fresh NaN: pin it to the x86 default NaN word.
        if sg.is_nan() && !r1.is_nan() && !ab.is_nan() {
            sg = f32::from_bits(0xffc0_0000);
        }
        adj = fma32([sg, r1, p5], sg, p5, r1, true);
    }
    let e2 = (r12 & 0x3fff_ffff) as i32;
    let d2 = (p2 - p1) as f64;
    let r2 = fma64_to32([p2, p1, p1], e2 as f64, f64::from_bits(0x3e10_0000_0000_0000) * d2, p1 as f64);
    let w = u - t.c[idx];
    (fma32([w, r2, adj], w, adj, r2, false), pair)
}

/// The seed cell of the evaluator wrapper: argument 0 times 4096, truncated.
pub fn seed_of(a0: f32) -> i32 {
    cvt_i32(f32::from_bits(0x4580_0000) * a0)
}

/// Labels of the stand-ins.
pub fn assumptions() -> [&'static str; 1] {
    ["SAWTOOTH-SEED-CELL"]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The last row ends at 0x447ffffa (1023.9996337890625): a reduced position from there on reads row 1024, the 12 bytes after the table.
    #[test]
    fn the_search_ends_at_row_1024_from_the_end_of_the_last_row() {
        let t = table();
        assert_eq!(search(t, f32::from_bits(0x447f_fff9)), 1023);
        for bits in [0x447f_fffau32, 0x447f_ffff, 0x4480_0000] {
            assert_eq!(search(t, f32::from_bits(bits)), 1024, "{:#x}", bits);
        }
    }
}
