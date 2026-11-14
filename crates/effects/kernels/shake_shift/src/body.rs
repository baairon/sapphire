//! Shake shift render body (AVX route) as ordinary Rust.
//!
//! The body runs two noise stages and a sine term: stage one uses the first seed, rate and amplitude, stage two the second ones, and the
//! result is `stage_one + sine_term + stage_two`. Each stage is either the smooth route (a four by four neighbourhood of cached hash
//! values weighted by a cubic filter, `route == 0`) or the hashed route (a lookup in a 1,024 entry prefix-sum table built from the first
//! hash). Every float operation keeps the hardware NaN rules of the original (the first NaN operand wins and is quieted; an invalid
//! result gives the default NaN), and fused multiply-adds sit exactly where the original fuses (18 sites, `f32::mul_add` and
//! `f64::mul_add`). Conversions use the truncating and rounding semantics of the hardware instructions (out of range and NaN give
//! `i32::MIN`). Callees (hash1, hash2, sine) are supplied through [`Services`].
//!
//! Smooth route, dropped state: the original kept a small cache of hash values keyed by two words and pre-filled with a marker word.
//! Within one stage every (row, column) pair of the filter reads a distinct cache entry, and the second key (one of 0xffffffff, 0, 1)
//! never equals the marker, so a lookup never hits and the miss path always runs; the cache is therefore not kept. The one-entry
//! result shortcut of the smooth route compared a freshly stored marker with zero and can never be taken either.

// Constants of the image (cells read by the body). Origin: image cells 0x15e850c to 0x15f0720.
pub const K_1: f32 = f32::from_bits(0x3f80_0000);
pub const K_2: f32 = f32::from_bits(0x4000_0000);
pub const K_HALF: f32 = f32::from_bits(0x3f00_0000);
pub const K_2M12: f32 = f32::from_bits(0x3980_0000);
pub const K_4096: f32 = f32::from_bits(0x4580_0000);
pub const KD_HALF: f64 = f64::from_bits(0x3fe0_0000_0000_0000);
pub const KD_2M30: f64 = f64::from_bits(0x3e10_0000_0000_0000);
pub const KD_ONE: f64 = f64::from_bits(0x3ff0_0000_0000_0000);
pub const KD_HASH_SCALE: f64 = f64::from_bits(0x3e79_0000_0000_0000);
pub const KD_2M29: f64 = f64::from_bits(0x3e20_0000_0000_0000);
pub const K_2M10: f32 = f32::from_bits(0x3a80_0000);
pub const K_2M30: f32 = f32::from_bits(0x3080_0000);
pub const K_2P23: f32 = f32::from_bits(0x4b00_0000);

// Origin: the cell at listing 0x15e9b98, read at 0x4555e1.
/// The sine scale: 0x40c90fdb (6.28318548, two pi as a single-precision value), read in an emulator run of the original (cell
/// bytes db 0f c9 40), held as one constant. The sine argument is K_SINE * wave_frequency * time, so a wave has one cycle per
/// 1 / wave_frequency time units.
pub const K_SINE: f32 = f32::from_bits(0x40c9_0fdb);

pub const TAB_LEN: usize = 1024;

/// The parameter words the body reads, all single precision.
#[derive(Clone, Copy, Debug, Default)]
pub struct Params {
    /// Stage one seed, scaled by 4096 and truncated to the hash key.
    pub seed_x: f32,
    /// Time: scales both noise coordinates and the sine argument.
    pub time: f32,
    /// Stage one amplitude.
    pub amp_x: f32,
    /// Stage one noise rate (the coordinate is time * 2 * rate on the smooth route).
    pub rate_x: f32,
    /// Sine term amplitude.
    pub sine_amp: f32,
    /// Sine term frequency.
    pub sine_freq: f32,
    /// Stage two amplitude.
    pub amp_y: f32,
    /// Stage two noise rate.
    pub rate_y: f32,
    /// Stage two seed, scaled by 4096 and truncated to the hash key.
    pub seed_y: f32,
    /// Route selector: exactly zero (either sign) takes the smooth route, anything else (NaN included) the hashed route.
    pub route: f32,
    /// Hashed route: one minus this value is the magnitude beyond which the random offset is folded back.
    pub limit_gap: f32,
    /// Overall gain applied to both stages.
    pub gain: f32,
}

/// Inputs the body takes besides its parameters.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The sine scale (default [`K_SINE`]; other values exist for equality tests against a patched cell).
    pub k_sine: f32,
}

impl Default for Config {
    fn default() -> Config {
        Config { k_sine: K_SINE }
    }
}

// Origin: guard 0x1e0a320, table 0x1dfdea0, weight table 0x1e05b20, scale cell 0x1e0a324.
/// Global cells the body reads and writes: the guard, the 1,024 entry table (12 bytes per entry: squared weight, prefix sum, hash),
/// the entry just past the table (read when the search ends at index 1,024; it holds zero in the original and no instruction
/// writes it), the weight table and the scale cell.
#[derive(Clone, Debug)]
pub struct Globals {
    pub guard: u32,
    pub tab: Vec<[u32; 3]>,
    pub tail: [u32; 3],
    pub weights: Vec<f32>,
    pub scale: f32,
}

pub trait Services {
    fn hash1(&mut self, x: u32) -> u32;
    fn hash2(&mut self, x: u32, y: u32) -> u32;
    fn sine(&mut self, x: f32) -> f32;
}

use std::sync::atomic::{AtomicU64, Ordering};
/// Count of operations with two or more NaN operands (the pair rule applies); used by comparisons to classify NaN-pair choices.
pub static NAN_PAIRS: AtomicU64 = AtomicU64::new(0);
#[inline]
fn note_pair(n: usize) {
    if n >= 2 {
        NAN_PAIRS.fetch_add(1, Ordering::Relaxed);
    }
}

// ---------------------------------------------------------------- arithmetic helpers (hardware NaN rules)
const QUIET_BIT_32: u32 = 1 << 22;
const MANTISSA_32: u64 = (1 << 23) - 1;
const DEFAULT_NAN32: u32 = 0xffc0_0000;
const DEFAULT_NAN64: u64 = 0xfff8_0000_0000_0000;

/// Quiet a single-precision NaN (set the top mantissa bit).
#[inline]
fn quiet32(x: f32) -> f32 {
    f32::from_bits(x.to_bits() | QUIET_BIT_32)
}
#[inline]
fn quiet64(x: f64) -> f64 {
    f64::from_bits(x.to_bits() | 0x0008_0000_0000_0000)
}
#[inline]
fn binary32(a: f32, b: f32, r: f32) -> f32 {
    note_pair(a.is_nan() as usize + b.is_nan() as usize);
    if a.is_nan() {
        quiet32(a)
    } else if b.is_nan() {
        quiet32(b)
    } else if r.is_nan() {
        f32::from_bits(DEFAULT_NAN32)
    } else {
        r
    }
}
#[inline]
fn binary64(a: f64, b: f64, r: f64) -> f64 {
    note_pair(a.is_nan() as usize + b.is_nan() as usize);
    if a.is_nan() {
        quiet64(a)
    } else if b.is_nan() {
        quiet64(b)
    } else if r.is_nan() {
        f64::from_bits(DEFAULT_NAN64)
    } else {
        r
    }
}
fn add(a: f32, b: f32) -> f32 {
    binary32(a, b, a + b)
}
fn sub(a: f32, b: f32) -> f32 {
    binary32(a, b, a - b)
}
fn mul(a: f32, b: f32) -> f32 {
    binary32(a, b, a * b)
}
fn div(a: f32, b: f32) -> f32 {
    binary32(a, b, a / b)
}
fn mul64(a: f64, b: f64) -> f64 {
    binary64(a, b, a * b)
}
/// Fused result with the NaN priority of the hardware fused forms: the first NaN among `order` (the destination, second source and
/// third source of the form used) wins.
fn fused32(order: [f32; 3], r: f32) -> f32 {
    note_pair(order.iter().filter(|v| v.is_nan()).count());
    for v in order {
        if v.is_nan() {
            return quiet32(v);
        }
    }
    if r.is_nan() {
        f32::from_bits(DEFAULT_NAN32)
    } else {
        r
    }
}
fn fused64(order: [f64; 3], r: f64) -> f64 {
    note_pair(order.iter().filter(|v| v.is_nan()).count());
    for v in order {
        if v.is_nan() {
            return quiet64(v);
        }
    }
    if r.is_nan() {
        f64::from_bits(DEFAULT_NAN64)
    } else {
        r
    }
}
/// Single to double precision; a NaN keeps its payload and sign and is quieted.
fn widen(x: f32) -> f64 {
    if x.is_nan() {
        quiet64(f64::from_bits(((x.to_bits() as u64 & MANTISSA_32) << 29) | ((x.to_bits() as u64 & 0x8000_0000) << 32) | 0x7ff0_0000_0000_0000))
    } else {
        x as f64
    }
}
/// Double to single precision; a NaN keeps the top of its payload and its sign and is quieted.
fn narrow(x: f64) -> f32 {
    if x.is_nan() {
        let b = x.to_bits();
        quiet32(f32::from_bits((((b >> 32) & 0x8000_0000) as u32) | 0x7f80_0000 | (((b >> 29) & MANTISSA_32) as u32)))
    } else {
        x as f32
    }
}
/// Truncate to i32: NaN and out of range give `i32::MIN` (the integer indefinite).
fn truncate_to_i32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
/// Round to i32 in round-to-nearest-even: NaN and out of range give `i32::MIN`.
fn round_to_i32(x: f32) -> i32 {
    if x.is_nan() {
        return i32::MIN;
    }
    let r = x.round_ties_even();
    if r >= 2147483648.0 || r < -2147483648.0 {
        i32::MIN
    } else {
        r as i32
    }
}
fn i32_to_f32(i: i32) -> f32 {
    i as f32
}
/// Nearest integer key of `x`: trunc(x + copysign(0.5, x)) when |x| < 2^23, else x; then rounded to i32.
fn round_key(x: f32) -> i32 {
    let b = x.to_bits();
    let half = f32::from_bits((b & 0x8000_0000) ^ 0x3f00_0000);
    let s = add(half, x);
    let t = if s.is_nan() { quiet32(s) } else { s.trunc() };
    let abs = f32::from_bits(b & 0x7fff_ffff);
    let v = if abs < K_2P23 { t } else { x };
    round_to_i32(v)
}
// Ordered-compare tests of a ? b. Each is true or false in the same way as the flag test of the hardware compare, so an unordered
// pair (a NaN operand) answers as written in the name.
/// a < b, or unordered.
fn less_or_unordered(a: f32, b: f32) -> bool {
    !(a >= b)
}
/// a >= b (false when unordered).
fn greater_or_equal(a: f32, b: f32) -> bool {
    a >= b
}
/// a > b (false when unordered).
fn greater(a: f32, b: f32) -> bool {
    a > b
}
/// a <= b, or unordered.
fn less_equal_or_unordered(a: f32, b: f32) -> bool {
    !(a > b)
}

impl Globals {
    fn entry(&self, idx: i64) -> [u32; 3] {
        if (0..TAB_LEN as i64).contains(&idx) {
            self.tab[idx as usize]
        } else if idx == TAB_LEN as i64 {
            self.tail
        } else {
            panic!("table index {} outside the modelled range", idx)
        }
    }
}

// Origin: 0x455315..0x455474 and 0x455d01..0x455e44.
/// Table builder of the hashed route: squared weights from the first hash, normalised by
/// the scaled sum, with the running prefix sum alongside.
fn build_table(g: &mut Globals, sv: &mut dyn Services) {
    let mut hash = sv.hash1(0x1b9e);
    let mut sum = 0.0f32;
    for i in 0..TAB_LEN {
        hash = hash.wrapping_add(0x1b9e);
        hash = sv.hash1(hash);
        let low = hash & 0x3fff_ffff;
        let unit = mul(i32_to_f32(low as i32), K_2M30);
        let square = mul(unit, unit);
        g.tab[i][0] = square.to_bits();
        g.tab[i][2] = hash;
        sum = add(sum, square);
    }
    let norm = mul(K_2M10, sum);
    let mut prefix = 0.0f32;
    let mut j = 0;
    while j < TAB_LEN {
        let weight_even = div(f32::from_bits(g.tab[j][0]), norm);
        g.tab[j][0] = weight_even.to_bits();
        let after_even = add(prefix, weight_even);
        g.tab[j][1] = prefix.to_bits();
        let weight_odd = div(f32::from_bits(g.tab[j + 1][0]), norm);
        g.tab[j + 1][0] = weight_odd.to_bits();
        prefix = add(after_even, weight_odd);
        g.tab[j + 1][1] = after_even.to_bits();
        j += 2;
    }
    g.guard = 1;
}

// Origin: 0x45547e..0x4554da and 0x455e4e..0x455eaa.
/// Binary search of the prefix table; returns the entry index.
fn search(g: &Globals, f: f32, lo0: i32, hi0: i32) -> i64 {
    let mut lo = lo0;
    let mut hi = hi0;
    loop {
        let mid = lo.wrapping_add(hi) >> 1;
        let e = g.entry(mid as i64);
        let prefix = f32::from_bits(e[1]);
        if !less_or_unordered(f, prefix) {
            let upper = add(prefix, f32::from_bits(e[0]));
            if greater(upper, f) {
                return mid as i64;
            }
        }
        if greater_or_equal(prefix, f) {
            hi = mid;
        }
        if less_or_unordered(prefix, f) {
            lo = mid.wrapping_add(1);
        }
        if !(lo < hi) {
            return lo as i64;
        }
    }
}

/// Which operand the hardware fused form tests first for NaN in the fold-back step of the hashed route; the two stages use
/// different forms of the same operation.
#[derive(Clone, Copy)]
enum FoldOrder {
    /// Order: limit, quotient, offset.
    LimitFirst,
    /// Order: quotient, offset, limit.
    QuotientFirst,
}

/// Fold-back of the hashed route: when |offset| exceeds `limit` the result is offset - sign(offset) * limit, else zero.
fn fold_back(limit: f32, offset: f32, order: FoldOrder) -> f32 {
    let magnitude = f32::from_bits(offset.to_bits() & 0x7fff_ffff);
    if less_or_unordered(limit, magnitude) {
        let q = div(offset, magnitude);
        let fused = (-q).mul_add(limit, offset);
        match order {
            FoldOrder::LimitFirst => fused32([limit, q, offset], fused),
            FoldOrder::QuotientFirst => fused32([q, offset, limit], fused),
        }
    } else {
        0.0
    }
}

/// Smooth route of one stage: a cubic filter over a 3 by 3 neighbourhood of hash values. `coord` is the noise coordinate, `key` the
/// stage's hash key.
fn smooth_noise(left: f32, right: f32, key: i32, amp: f32, gain: f32, g: &Globals, sv: &mut dyn Services) -> f32 {
    let coord = mul(mul(left, K_2), right);
    let cell = round_key(coord);
    let cell_f = i32_to_f32(cell);
    // keys of the three neighbouring cells
    let keys = [cell.wrapping_sub(1), cell, cell.wrapping_add(1)];
    let frac = sub(coord, cell_f);
    let twice = add(frac, frac);
    let square = mul(frac, frac);
    let w_lo = fused32([frac, frac, twice], frac.mul_add(frac, twice));
    let w_hi = fused32([frac, frac, twice], frac.mul_add(frac, -twice));
    // column weights and row weights of the 3 by 3 neighbourhood
    let col_w = [add(w_lo, K_1), square, add(w_hi, K_1)];
    let row_w = [K_1, 0.0f32, K_1];
    let row_key = [0xffff_ffffu32, 0, 1];
    let mut sum_w = 0.0f32;
    let mut acc = 0.0f32;
    for r in 0..3 {
        for c in 0..3 {
            let s = add(row_w[r], col_w[c]);
            if less_equal_or_unordered(g.scale, s) {
                continue;
            }
            let idx = truncate_to_i32(mul(s, K_4096)) >> 1;
            let weight = g.weights[idx as usize];
            let col_key = keys[c] as u32;
            let hashed = sv.hash2((row_key[r] << 4) ^ (key as u32), col_key);
            let value = ((hashed & 0x1fff) as i32).wrapping_add(-0x1000);
            let sample = mul(K_2M12, i32_to_f32(value));
            let product = mul(weight, sample);
            sum_w = add(sum_w, weight);
            acc = add(product, acc);
        }
    }
    let recip = div(K_1, sum_w);
    let base = 0.0f32;
    let blended = fused32([acc, recip, base], recip.mul_add(acc, base));
    let wide = widen(blended);
    let shifted = fused64([KD_HALF, wide, KD_HALF], wide.mul_add(KD_HALF, KD_HALF));
    let v = narrow(shifted);
    let centred = sub(v, K_HALF);
    let two_gain = add(gain, gain);
    mul(mul(centred, amp), two_gain)
}

/// Hashed route of one stage: a random offset within one cell, picked through the prefix table, folded back past the limit.
fn hashed_noise(
    left: f32,
    right: f32,
    key: i32,
    limit_gap: f32,
    amp: f32,
    gain: f32,
    order: FoldOrder,
    g: &mut Globals,
    sv: &mut dyn Services,
) -> f32 {
    let limit = sub(K_1, limit_gap);
    let key_in = (key as u32).wrapping_add(0x3fd);
    let neg_gain = f32::from_bits(gain.to_bits() ^ 0x8000_0000);
    let h = sv.hash1(key_in) & 0x3fff_ffff;
    let jitter = narrow(mul64(h as i32 as f64, KD_HASH_SCALE));
    let pos = fused32([left, jitter, right], left.mul_add(right, jitter));
    let scaled = mul(pos, K_2M10);
    let floor = if scaled.is_nan() { quiet32(scaled) } else { scaled.floor() };
    let block = round_to_i32(floor);
    let frac = sub(pos, i32_to_f32(block.wrapping_shl(10)));
    if g.guard == 0 {
        build_table(g, sv);
    }
    let idx = search(g, frac, 0, 0x400);
    let e = g.entry(idx);
    let hash_b = sv.hash2(key as u32, block.wrapping_add(e[2] as i32) as u32);
    let hash_c = sv.hash1(hash_b) & 0x3fff_ffff;
    let w = hash_c as i32 as f64;
    let offset = narrow(fused64([KD_2M29, w, KD_ONE], w.mul_add(KD_2M29, -KD_ONE)));
    let folded = fold_back(limit, offset, order);
    let unit_b = (hash_b & 0x3fff_ffff) as i32 as f64;
    let span = sub(gain, neg_gain);
    let lo = widen(neg_gain);
    let hi = widen(span);
    let scaled_span = mul64(hi, KD_2M30);
    let mixed = narrow(fused64([scaled_span, unit_b, lo], unit_b.mul_add(scaled_span, lo)));
    let rel = sub(frac, f32::from_bits(e[1]));
    let half_amp = mul64(widen(amp), KD_HALF);
    let inner = fused32([rel, mixed, folded], rel.mul_add(folded, mixed));
    narrow(mul64(half_amp, widen(inner)))
}

/// One noise stage: the smooth route when `route == 0`, else the hashed route.
fn noise_stage(
    left: f32,
    right: f32,
    key: i32,
    route: f32,
    limit_gap: f32,
    amp: f32,
    gain: f32,
    order: FoldOrder,
    g: &mut Globals,
    sv: &mut dyn Services,
) -> f32 {
    if route == 0.0 {
        smooth_noise(left, right, key, amp, gain, g, sv)
    } else {
        hashed_noise(left, right, key, limit_gap, amp, gain, order, g, sv)
    }
}

/// Runs the body and returns the output float.
pub fn shake_shift_body(p: &Params, cfg: &Config, g: &mut Globals, sv: &mut dyn Services) -> f32 {
    // stage one
    let key_x = truncate_to_i32(mul(K_4096, p.seed_x));
    let stage_x = noise_stage(p.time, p.rate_x, key_x, p.route, p.limit_gap, p.amp_x, p.gain, FoldOrder::LimitFirst, g, sv);
    // sine term
    let arg = mul(mul(cfg.k_sine, p.sine_freq), p.time);
    let s = sv.sine(arg);
    let key_y = truncate_to_i32(mul(K_4096, p.seed_y));
    let sine_term = mul(s, p.sine_amp);
    // stage two
    let stage_y = noise_stage(p.rate_y, p.time, key_y, p.route, p.limit_gap, p.amp_y, p.gain, FoldOrder::QuotientFirst, g, sv);
    add(add(stage_x, sine_term), stage_y)
}
