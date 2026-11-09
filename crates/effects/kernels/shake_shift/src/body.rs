//! Render body 0x454be0 (AVX route), ported in the listing's operation order.
//!
//! The body works on a stack frame of 0xb10 bytes below its frame pointer; the frame is modelled as a byte array addressed by the
//! listing's signed offset from the frame pointer, so every spill and every aliasing between slots behaves as in the original.
//! Fused sites use `f32::mul_add` / `f64::mul_add` exactly where the listing has a vfmadd/vfmsub/vfnmadd instruction (18 sites).
//! Conversions use the truncating/rounding semantics of cvtt*/cvt* (out of range and NaN give i32::MIN).
//! Callees (hash1, hash2, sine) are supplied through [`Services`]; the fill routine is ported in `fill` and runs on the frame.

use crate::initlist::{init_store_offset, INIT_STORE_COUNT};

// Constants of the image (cells read by the body).
pub const K_1: f32 = f32::from_bits(0x3f80_0000); // 0x15e850c
pub const K_2: f32 = f32::from_bits(0x4000_0000); // 0x15e8510
pub const K_HALF: f32 = f32::from_bits(0x3f00_0000); // 0x15e8514
pub const K_2M12: f32 = f32::from_bits(0x3980_0000); // 0x15e8518
pub const K_4096: f32 = f32::from_bits(0x4580_0000); // 0x15e853c
pub const KD_HALF: f64 = f64::from_bits(0x3fe0_0000_0000_0000); // 0x15e9378
pub const KD_2M30: f64 = f64::from_bits(0x3e10_0000_0000_0000); // 0x15e9390
pub const KD_ONE: f64 = f64::from_bits(0x3ff0_0000_0000_0000); // 0x15e8490
pub const KD_HASH_SCALE: f64 = f64::from_bits(0x3e79_0000_0000_0000); // 0x15f0698
pub const KD_2M29: f64 = f64::from_bits(0x3e20_0000_0000_0000); // 0x15f06a0
pub const K_2M10: f32 = f32::from_bits(0x3a80_0000); // 0x15f0684
pub const K_2M30: f32 = f32::from_bits(0x3080_0000); // 0x15f0680
pub const K_2P23: f32 = f32::from_bits(0x4b00_0000); // 0x15f0720
pub const INIT_WORD: u32 = 0xffff_8300; // lane value of the vector at 0x15f06e0

pub const FRAME: usize = 0xb10;
pub const FILL_LEN: usize = 0xa28;
pub const TAB_LEN: usize = 1024;

/// The object fields the body reads (offsets from the object pointer), all single precision.
#[derive(Clone, Copy, Debug, Default)]
pub struct Params {
    pub p4c: f32,
    pub p7c: f32,
    pub pac: f32,
    pub pdc: f32,
    pub p10c: f32,
    pub p13c: f32,
    pub p16c: f32,
    pub p19c: f32,
    pub p1cc: f32,
    pub p1fc: f32,
    pub p22c: f32,
    pub p25c: f32,
}

/// The float read at 0x4555e1 from the cell at listing 0x15e9b98: 0x40c90fdb (6.28318548, two pi as a single-precision
/// value), read in an emulator run of the original (cell bytes db 0f c9 40), held as one constant. The sine argument is
/// K_SINE * wave_frequency * time, so a wave has one cycle per 1 / wave_frequency time units.
pub const K_SINE: f32 = f32::from_bits(0x40c9_0fdb);

/// Inputs the body takes besides its parameters.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The float read at 0x4555e1 (default [`K_SINE`]; other values exist for equality tests against a patched cell).
    pub k_sine: f32,
}

impl Default for Config {
    fn default() -> Config {
        Config { k_sine: K_SINE }
    }
}

/// Global cells the body reads and writes: guard (0x1e0a320), the 1,024 entry table (0x1dfdea0, 12 bytes per entry: squared weight,
/// prefix sum, hash), the entry just past the table (read when the search ends at index 1,024; it holds zero in the original and no
/// instruction writes it), the weight table (0x1e05b20) and the scale cell (0x1e0a324).
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

// ---------------------------------------------------------------- arithmetic helpers (silicon NaN rules)
const DEFAULT_NAN32: u32 = 0xffc0_0000;
const DEFAULT_NAN64: u64 = 0xfff8_0000_0000_0000;

#[inline]
fn q32(x: f32) -> f32 {
    f32::from_bits(x.to_bits() | 0x0040_0000)
}
#[inline]
fn q64(x: f64) -> f64 {
    f64::from_bits(x.to_bits() | 0x0008_0000_0000_0000)
}
#[inline]
fn bin32(a: f32, b: f32, r: f32) -> f32 {
    note_pair(a.is_nan() as usize + b.is_nan() as usize);
    if a.is_nan() {
        q32(a)
    } else if b.is_nan() {
        q32(b)
    } else if r.is_nan() {
        f32::from_bits(DEFAULT_NAN32)
    } else {
        r
    }
}
#[inline]
fn bin64(a: f64, b: f64, r: f64) -> f64 {
    note_pair(a.is_nan() as usize + b.is_nan() as usize);
    if a.is_nan() {
        q64(a)
    } else if b.is_nan() {
        q64(b)
    } else if r.is_nan() {
        f64::from_bits(DEFAULT_NAN64)
    } else {
        r
    }
}
fn add(a: f32, b: f32) -> f32 {
    bin32(a, b, a + b)
}
fn sub(a: f32, b: f32) -> f32 {
    bin32(a, b, a - b)
}
fn mul(a: f32, b: f32) -> f32 {
    bin32(a, b, a * b)
}
fn div(a: f32, b: f32) -> f32 {
    bin32(a, b, a / b)
}
fn dmul(a: f64, b: f64) -> f64 {
    bin64(a, b, a * b)
}
/// Fused result with the NaN priority of the VEX FMA forms: the first NaN among (destination, second source, third source).
fn fused32(order: [f32; 3], r: f32) -> f32 {
    note_pair(order.iter().filter(|v| v.is_nan()).count());
    for v in order {
        if v.is_nan() {
            return q32(v);
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
            return q64(v);
        }
    }
    if r.is_nan() {
        f64::from_bits(DEFAULT_NAN64)
    } else {
        r
    }
}
/// cvtss2sd
fn dbl(x: f32) -> f64 {
    if x.is_nan() {
        q64(f64::from_bits(((x.to_bits() as u64 & 0x007f_ffff) << 29) | ((x.to_bits() as u64 & 0x8000_0000) << 32) | 0x7ff0_0000_0000_0000))
    } else {
        x as f64
    }
}
/// cvtsd2ss
fn sgl(x: f64) -> f32 {
    if x.is_nan() {
        let b = x.to_bits();
        q32(f32::from_bits((((b >> 32) & 0x8000_0000) as u32) | 0x7f80_0000 | (((b >> 29) & 0x007f_ffff) as u32)))
    } else {
        x as f32
    }
}
/// cvttss2si (32 bit): truncation, NaN and out of range give the integer indefinite.
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
/// cvtss2si (32 bit) in round-to-nearest-even.
fn cvt(x: f32) -> i32 {
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
fn cvtsi(i: i32) -> f32 {
    i as f32
}
/// Rounding of 0x454f56..0x454f90: trunc(x + copysign(0.5, x)) when |x| < 2^23, else x; then cvtss2si.
fn round_key(x: f32) -> i32 {
    let b = x.to_bits();
    let half = f32::from_bits((b & 0x8000_0000) ^ 0x3f00_0000);
    let s = add(half, x);
    let t = if s.is_nan() { q32(s) } else { s.trunc() };
    let abs = f32::from_bits(b & 0x7fff_ffff);
    let v = if abs < K_2P23 { t } else { x };
    cvt(v)
}
// comiss flag tests: (a ? b)
fn jb(a: f32, b: f32) -> bool {
    !(a >= b)
}
fn jae(a: f32, b: f32) -> bool {
    a >= b
}
fn ja(a: f32, b: f32) -> bool {
    a > b
}
fn jbe(a: f32, b: f32) -> bool {
    !(a > b)
}

// ---------------------------------------------------------------- frame
pub struct Frame {
    pub b: Vec<u8>,
    /// Bytes stored by the body (for comparison against a reference frame).
    pub written: Vec<bool>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame { b: vec![0u8; FRAME + 8], written: vec![false; FRAME + 8] }
    }
    fn ix(off: i32) -> usize {
        (FRAME as i32 + off) as usize
    }
    pub fn ld(&self, off: i32) -> u32 {
        let i = Self::ix(off);
        u32::from_le_bytes([self.b[i], self.b[i + 1], self.b[i + 2], self.b[i + 3]])
    }
    pub fn st(&mut self, off: i32, v: u32) {
        let i = Self::ix(off);
        self.b[i..i + 4].copy_from_slice(&v.to_le_bytes());
        for k in 0..4 {
            self.written[i + k] = true;
        }
    }
    pub fn ldf(&self, off: i32) -> f32 {
        f32::from_bits(self.ld(off))
    }
    pub fn stf(&mut self, off: i32, v: f32) {
        self.st(off, v.to_bits())
    }
    /// The fill routine (0x136fbd0) called with value 0 and `count` bytes from `off`. The frame is 32-byte aligned in the
    /// original; the stored bytes do not depend on the alignment (the fill test covers all sixteen phases).
    fn fill_zero(&mut self, off: i32, count: usize) {
        let i = Self::ix(off);
        crate::fill::fill(&mut self.b[i..i + count], 0, 0, count as i64, &crate::fill::FillEnv::MEASURED);
        for k in 0..count {
            self.written[i + k] = true;
        }
    }
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

/// Frame cache initialiser (0x454cb6..0x454ee6 and 0x4556aa..0x4558da): the constant vector stored with stride 0x280, four times.
fn init_cache(fr: &mut Frame) {
    fr.st(-0xaec, 1);
    fr.st(-0xb10, INIT_WORD);
    fr.st(-0xb0c, INIT_WORD);
    let mut base = -0xae8i32;
    for _ in 0..4 {
        for i in 0..INIT_STORE_COUNT {
            fr.st(base + init_store_offset(i), INIT_WORD);
        }
        base += 0x280;
    }
}

/// Table builder shared by both halves (0x455315..0x455474 and 0x455d01..0x455e44); `spill` is the frame accumulator cell -0xad8.
fn build_table(g: &mut Globals, sv: &mut dyn Services, fr: &mut Frame) {
    let mut eax = sv.hash1(0x1b9e);
    for i in 0..TAB_LEN {
        eax = eax.wrapping_add(0x1b9e);
        eax = sv.hash1(eax);
        let edx = eax & 0x3fff_ffff;
        let x0 = cvtsi(edx as i32);
        let x1 = mul(x0, K_2M30);
        let x2 = mul(x1, x1);
        g.tab[i][0] = x2.to_bits();
        g.tab[i][2] = eax;
        let x3 = fr.ldf(-0xad8);
        let x4 = add(x3, x2);
        fr.stf(-0xad8, x4);
    }
    let x0 = mul(K_2M10, fr.ldf(-0xad8));
    let mut x3 = fr.ldf(-0xaf8);
    let mut j = 0;
    while j < TAB_LEN {
        let x1 = f32::from_bits(g.tab[j][0]);
        let x2 = div(x1, x0);
        g.tab[j][0] = x2.to_bits();
        let x4 = add(x3, x2);
        g.tab[j][1] = x3.to_bits();
        let y3 = f32::from_bits(g.tab[j + 1][0]);
        let x5 = div(y3, x0);
        g.tab[j + 1][0] = x5.to_bits();
        x3 = add(x4, x5);
        g.tab[j + 1][1] = x4.to_bits();
        j += 2;
    }
    g.guard = 1;
}

/// Binary search of the prefix table (0x45547e..0x4554da and 0x455e4e..0x455eaa); returns the entry index.
fn search(g: &Globals, f: f32, lo0: i32, hi0: i32) -> i64 {
    let mut lo = lo0;
    let mut hi = hi0;
    loop {
        let mid = lo.wrapping_add(hi) >> 1;
        let e = g.entry(mid as i64);
        let prefix = f32::from_bits(e[1]);
        if !jb(f, prefix) {
            let x0 = add(prefix, f32::from_bits(e[0]));
            if ja(x0, f) {
                return mid as i64;
            }
        }
        // 0x4554c2
        let mut m = mid;
        if jae(prefix, f) {
            hi = m; // cmovae edx, r9d
        }
        m = m.wrapping_add(1);
        if jb(prefix, f) {
            lo = m; // cmovb r15d, r9d
        }
        if !(lo < hi) {
            return lo as i64;
        }
    }
}

/// The value written at 0x45554d.. (sign step on the derived random value).
fn sign_step(p_b: f32, x1: f32) -> (bool, f32) {
    // returns (took the divide path, value)
    let x0 = f32::from_bits(x1.to_bits() & 0x7fff_ffff);
    if jb(p_b, x0) {
        let q = div(x1, x0);
        // vfnmadd: -(q * p_b) + x1
        let r = fused32([p_b, q, x1], (-q).mul_add(p_b, x1));
        (true, r)
    } else {
        (false, 0.0)
    }
}

/// Runs the body. `out` is the float stored through the output pointer (rdx).
pub fn shake_shift_body(p: &Params, cfg: &Config, g: &mut Globals, sv: &mut dyn Services, fr: &mut Frame) -> f32 {
    // ---- prologue 0x454be0..0x454c7b
    let x5 = mul(K_4096, p.p4c);
    let x3_22c = p.p22c;
    let r14 = cvtt(x5);
    fr.stf(-0x48, p.p25c);
    fr.stf(-0xe0, p.p7c);
    fr.stf(-0x50, p.pac);
    fr.stf(-0xe8, p.pdc);
    fr.stf(-0x80, 0.0);
    let path_a = p.p1fc == 0.0; // ucomiss, jne / jp
    let mut x3 = x3_22c;

    if path_a {
        // ---- stage 1, path A (0x454c8b..0x455257)
        fr.fill_zero(-0xb10, FILL_LEN);
        fr.st(-0x88, 0);
        let ptr = -0x88;
        init_cache(fr);
        let x0 = fr.ldf(-0xe0);
        let x1 = mul(x0, K_2);
        let x8 = mul(x1, fr.ldf(-0xe8));
        let x2 = mul(x8, K_4096);
        let r9 = cvtt(x2);
        if r9 as u32 == fr.ld(-0xb10) && fr.ld(-0xb0c) == 0 {
            let x0 = cvtsi(fr.ld(-0xb00) as i32);
            let x1 = mul(x0, K_2M12);
            fr.stf(ptr, x1);
        } else {
            let edx0 = round_key(x8);
            let x9 = cvtsi(edx0);
            fr.st(-0xb4, edx0 as u32);
            let esi0 = edx0.wrapping_sub(1);
            fr.st(-0xb8, esi0 as u32);
            let r8_0 = edx0.wrapping_add(1);
            fr.st(-0xb0, r8_0 as u32);
            let edx = edx0 & 3;
            let esi = esi0 & 3;
            let r8 = r8_0 & 3;
            fr.st(-0xa8, 0);
            let mut x1 = fr.ldf(-0x80);
            let mut x2 = x1;
            fr.st(-0x20, 0);
            let x13 = sub(x8, x9);
            let x10 = x13;
            let x12 = add(x13, x13);
            let x8n = mul(x13, x13);
            let x10 = fused32([x10, x13, x12], x13.mul_add(x10, x12)); // 0x454fdd vfmadd213ss
            let x13 = fused32([x13, x13, x12], x13.mul_add(x13, -x12)); // 0x454fe2 vfmsub213ss
            fr.stf(-0x2c, x8n);
            let x11 = add(x10, K_1);
            let x14 = add(x13, K_1);
            fr.stf(-0x30, x11);
            fr.st(-0x24, K_1.to_bits());
            fr.stf(-0x28, x14);
            fr.st(-0x1c, K_1.to_bits());
            fr.st(-0xac, 0xffff_ffff);
            fr.st(-0xa4, 1);
            fr.st(-0x9c, edx as u32);
            fr.st(-0x90, 0);
            fr.st(-0xa0, esi as u32);
            fr.st(-0x98, r8 as u32);
            fr.st(-0x94, 0xc);
            fr.st(-0x8c, 4);
            fr.st(-0xc8, r9 as u32);
            fr.st(-0x40, r14 as u32);
            for r15 in 0..3i32 {
                for r14i in 0..3i32 {
                    let a = fr.ldf(r15 * 4 - 0x24);
                    let scale = g.scale;
                    let x4 = add(a, fr.ldf(r14i * 4 - 0x30));
                    if jbe(scale, x4) {
                        continue;
                    }
                    let x4 = mul(x4, K_4096);
                    let eax = cvtt(x4) >> 1;
                    let ecx = (fr.ld(r15 * 4 - 0x94) as i32).wrapping_add(fr.ld(r14i * 4 - 0xa0) as i32);
                    let r12 = (ecx as i64) * 5;
                    let wx = g.weights[eax as usize];
                    let r13 = fr.ld(r14i * 4 - 0xb8);
                    let edx_k = fr.ld(r15 * 4 - 0xac);
                    let base = (r12 * 4 - 0xae8) as i32;
                    let x4v;
                    if r13 == fr.ld(base) && edx_k == fr.ld(base + 4) {
                        let t = cvtsi(fr.ld(base + 8) as i32);
                        x4v = mul(K_2M12, t);
                    } else {
                        fr.st(-0xd0, edx_k);
                        fr.stf(-0xe8, wx);
                        fr.stf(-0xe0, x1);
                        fr.stf(-0xd8, x2);
                        let edi = (edx_k << 4) ^ fr.ld(-0x40);
                        let mut eax = sv.hash2(edi, r13);
                        eax &= 0x1fff;
                        let eax = (eax as i32).wrapping_add(-0x1000);
                        let t = cvtsi(eax);
                        x4v = mul(K_2M12, t);
                        fr.st(base, r13);
                        fr.st(base + 4, edx_k);
                        fr.st(base + 8, eax as u32);
                    }
                    let x4m = mul(wx, x4v);
                    x1 = add(x1, wx);
                    x2 = add(x4m, x2);
                }
            }
            // 0x4551b8
            let x0 = K_1;
            let x3d = div(x0, x1);
            let cur = f32::from_bits(fr.ld(ptr));
            let x2 = fused32([x2, x3d, cur], x3d.mul_add(x2, cur)); // 0x4551de vfmadd213ss
            fr.stf(ptr, x2);
            let r9 = fr.ld(-0xc8);
            let x1d = dbl(fr.ldf(ptr));
            let x2d = fused64([KD_HALF, x1d, KD_HALF], x1d.mul_add(KD_HALF, KD_HALF)); // 0x4551fe vfmadd213sd
            fr.stf(ptr, sgl(x2d));
            fr.st(-0xb10, r9);
            let x5 = mul(K_4096, fr.ldf(ptr));
            let esi = cvtt(x5);
            fr.st(-0xb0c, 0);
            fr.st(-0xb00, esi as u32);
        }
        // 0x455235
        let x0 = fr.ldf(-0x88);
        let x2 = fr.ldf(-0x48);
        let x1 = sub(x0, K_HALF);
        let x4 = add(x2, x2);
        let x3m = mul(x1, fr.ldf(-0x50));
        let x5 = mul(x3m, x4);
        fr.stf(-0x58, x5);
    } else {
        // ---- stage 1, path B (0x455261..0x4555dc)
        let x2 = K_1;
        let edi = (r14 as u32).wrapping_add(0x3fd);
        let x0 = fr.ldf(-0x48);
        x3 = sub(x2, x3);
        let x1 = f32::from_bits(x0.to_bits() ^ 0x8000_0000);
        fr.stf(-0xb10, x1);
        fr.stf(-0xb00, x3);
        let eax = sv.hash1(edi) & 0x3fff_ffff;
        let x0 = eax as i32 as f64;
        let x1 = dmul(x0, KD_HASH_SCALE);
        let x1 = sgl(x1);
        let x4 = fr.ldf(-0xe0);
        let me8 = fr.ldf(-0xe8);
        let x4 = fused32([x4, x1, me8], x4.mul_add(me8, x1)); // 0x4552be vfmadd132ss
        let x7 = fr.ldf(-0x80);
        fr.stf(-0xad8, x7);
        fr.stf(-0xaf8, x7);
        let x2 = mul(x4, K_2M10);
        let x3f = if x2.is_nan() { q32(x2) } else { x2.floor() };
        let esi = cvt(x3f);
        let eax = esi.wrapping_shl(10);
        let x5 = cvtsi(eax);
        let x6 = sub(x4, x5);
        fr.stf(-0xb08, x6);
        if g.guard == 0 {
            fr.st(-0xae8, 0x400);
            fr.st(-0xae0, esi as u32);
            fr.st(-0xaf0, 0);
            fr.st(-0x40, r14 as u32);
            build_table(g, sv, fr);
        }
        let f = fr.ldf(-0xb08);
        let idx = search(g, f, 0, 0x400);
        let e = g.entry(idx);
        let esi2 = esi.wrapping_add(e[2] as i32);
        let r14n = sv.hash2(r14 as u32, esi2 as u32);
        let eax = sv.hash1(r14n) & 0x3fff_ffff;
        let x0 = eax as i32 as f64;
        let x1 = fused64([KD_2M29, x0, KD_ONE], x0.mul_add(KD_2M29, -KD_ONE)); // 0x45551c vfmsub213sd
        let x2 = fr.ldf(-0xb00);
        let x1 = sgl(x1);
        let (_took, val) = sign_step(x2, x1);
        fr.stf(-0xb00, val);
        let x2 = fr.ldf(-0x48);
        let x4 = fr.ldf(-0xb10);
        let r14m = (r14n & 0x3fff_ffff) as i32;
        let x5 = r14m as f64;
        let x3 = sub(x2, x4);
        let x4d = dbl(x4);
        let x3d = dbl(x3);
        let x6 = dmul(x3d, KD_2M30);
        let x6 = fused64([x6, x5, x4d], x5.mul_add(x6, x4d)); // 0x45559e vfmadd213sd
        let x1 = fr.ldf(-0xb08);
        let x0 = dbl(fr.ldf(-0x50));
        let x7 = sub(x1, f32::from_bits(e[1]));
        let x6 = sgl(x6);
        let x8 = dmul(x0, KD_HALF);
        let mem = fr.ldf(-0xb00);
        let x7 = fused32([x7, x6, mem], x7.mul_add(mem, x6)); // 0x4555c6 vfmadd132ss
        let x7d = dbl(x7);
        let x9 = dmul(x8, x7d);
        let x9 = sgl(x9);
        fr.stf(-0x58, x9);
    }

    // ---- sine and the second half's inputs (0x4555e1..0x455665)
    let x1 = mul(cfg.k_sine, p.p13c);
    let x2 = p.p7c;
    let x0 = mul(x1, x2);
    fr.stf(-0xe8, x2);
    let s = sv.sine(x0);
    let x6 = mul(K_4096, p.p1cc);
    let x1 = mul(s, p.p10c);
    let r14 = cvtt(x6);
    fr.stf(-0xe0, p.p19c);
    fr.stf(-0x50, x1);
    fr.stf(-0x48, p.p16c);
    fr.stf(-0x40, p.p25c);
    let x3 = p.p22c;
    let x7 = p.p1fc;
    let eq0 = x7 == fr.ldf(-0x80); // ucomiss, jne / jp

    let x9;
    if eq0 {
        // ---- stage 2, path A (0x45567f..0x455c47)
        fr.fill_zero(-0xb10, FILL_LEN);
        fr.st(-0x90, 0);
        let ptr = -0x90;
        init_cache(fr);
        let x0 = fr.ldf(-0xe0);
        let x1 = mul(x0, K_2);
        let x8 = mul(x1, fr.ldf(-0xe8));
        let x2 = mul(x8, K_4096);
        let r9 = cvtt(x2);
        if r9 as u32 == fr.ld(-0xb10) && fr.ld(-0xb0c) == 0 {
            let x0 = cvtsi(fr.ld(-0xb00) as i32);
            let x1 = mul(x0, K_2M12);
            fr.stf(ptr, x1);
        } else {
            let edx0 = round_key(x8);
            let x9r = cvtsi(edx0);
            fr.st(-0xbc, edx0 as u32);
            let esi0 = edx0.wrapping_sub(1);
            fr.st(-0xc0, esi0 as u32);
            let r8_0 = edx0.wrapping_add(1);
            fr.st(-0xb8, r8_0 as u32);
            let edx = edx0 & 3;
            let esi = esi0 & 3;
            let r8 = r8_0 & 3;
            fr.st(-0xb0, 0);
            let mut x1 = fr.ldf(-0x80);
            let mut x2 = x1;
            fr.st(-0x18, 0);
            let x13 = sub(x8, x9r);
            let x10 = x13;
            let x12 = add(x13, x13);
            let x8n = mul(x13, x13);
            let x10 = fused32([x10, x13, x12], x13.mul_add(x10, x12)); // 0x4559d1
            let x13 = fused32([x13, x13, x12], x13.mul_add(x13, -x12)); // 0x4559d6
            fr.stf(-0x24, x8n);
            let x11 = add(x10, K_1);
            let x14 = add(x13, K_1);
            fr.stf(-0x28, x11);
            fr.st(-0x1c, K_1.to_bits());
            fr.stf(-0x20, x14);
            fr.st(-0x14, K_1.to_bits());
            fr.st(-0xb4, 0xffff_ffff);
            fr.st(-0xac, 1);
            fr.st(-0xa4, edx as u32);
            fr.st(-0x98, 0);
            fr.st(-0xa8, esi as u32);
            fr.st(-0xa0, r8 as u32);
            fr.st(-0x9c, 0xc);
            fr.st(-0x94, 4);
            fr.st(-0xd0, r9 as u32);
            fr.st(-0x30, r14 as u32);
            for r13 in 0..3i32 {
                for r15 in 0..3i32 {
                    let a = fr.ldf(r13 * 4 - 0x1c);
                    let scale = g.scale;
                    let x4 = add(a, fr.ldf(r15 * 4 - 0x28));
                    if jbe(scale, x4) {
                        continue;
                    }
                    let x4 = mul(K_4096, x4);
                    let eax = cvtt(x4) >> 1;
                    let ecx = (fr.ld(r13 * 4 - 0x9c) as i32).wrapping_add(fr.ld(r15 * 4 - 0xa8) as i32);
                    let r12 = (ecx as i64) * 5;
                    let wx = g.weights[eax as usize];
                    let r14k = fr.ld(r15 * 4 - 0xc0);
                    let edx_k = fr.ld(r13 * 4 - 0xb4);
                    let base = (r12 * 4 - 0xae8) as i32;
                    let x4v;
                    if r14k == fr.ld(base) && edx_k == fr.ld(base + 4) {
                        let t = cvtsi(fr.ld(base + 8) as i32);
                        x4v = mul(K_2M12, t);
                    } else {
                        fr.st(-0xd8, edx_k);
                        fr.stf(-0x80, x2);
                        fr.stf(-0xe8, wx);
                        fr.stf(-0xe0, x1);
                        let edi = (edx_k << 4) ^ fr.ld(-0x30);
                        let mut eax = sv.hash2(edi, r14k);
                        eax &= 0x1fff;
                        let eax = (eax as i32).wrapping_add(-0x1000);
                        let t = cvtsi(eax);
                        x4v = mul(K_2M12, t);
                        fr.st(base, r14k);
                        fr.st(base + 4, edx_k);
                        fr.st(base + 8, eax as u32);
                    }
                    let x4m = mul(wx, x4v);
                    x1 = add(x1, wx);
                    x2 = add(x4m, x2);
                }
            }
            // 0x455ba2
            let x0 = K_1;
            let x1d = div(x0, x1);
            fr.stf(-0x80, x2);
            let x2 = fr.ldf(-0x80);
            let cur = f32::from_bits(fr.ld(ptr));
            let x2 = fused32([x2, x1d, cur], x1d.mul_add(x2, cur)); // 0x455bc3 vfmadd213ss
            fr.stf(ptr, x2);
            let x3d = dbl(fr.ldf(ptr));
            let x4d = fused64([KD_HALF, x3d, KD_HALF], x3d.mul_add(KD_HALF, KD_HALF)); // 0x455bf2 vfmadd213sd
            fr.stf(ptr, sgl(x4d));
            let r9 = fr.ld(-0xd0);
            fr.st(-0xb10, r9);
            fr.st(-0xb0c, 0);
            let x6 = mul(K_4096, fr.ldf(ptr));
            let esi = cvtt(x6);
            fr.st(-0xb00, esi as u32);
        }
        // 0x455c25
        let x0 = fr.ldf(-0x90);
        let x2 = fr.ldf(-0x40);
        let x1 = sub(x0, K_HALF);
        let x4 = add(x2, x2);
        let x3m = mul(x1, fr.ldf(-0x48));
        x9 = mul(x3m, x4);
    } else {
        // ---- stage 2, path B (0x455c4c..0x455f92)
        let x2 = K_1;
        let edi = (r14 as u32).wrapping_add(0x3fd);
        let x0 = fr.ldf(-0x40);
        let x3 = sub(x2, x3);
        let x1 = f32::from_bits(x0.to_bits() ^ 0x8000_0000);
        fr.stf(-0xb10, x1);
        fr.stf(-0xb08, x3);
        let eax = sv.hash1(edi) & 0x3fff_ffff;
        let x0 = eax as i32 as f64;
        let x1 = dmul(x0, KD_HASH_SCALE);
        let x1 = sgl(x1);
        let x4 = fr.ldf(-0xe0);
        let me8 = fr.ldf(-0xe8);
        let x4 = fused32([x4, x1, me8], x4.mul_add(me8, x1)); // 0x455ca9 vfmadd132ss
        let x7 = fr.ldf(-0x80);
        fr.stf(-0xad8, x7);
        fr.stf(-0xaf8, x7);
        let x2 = mul(x4, K_2M10);
        let x3f = if x2.is_nan() { q32(x2) } else { x2.floor() };
        let r13 = cvt(x3f);
        let eax = r13.wrapping_shl(10);
        let x5 = cvtsi(eax);
        let x6 = sub(x4, x5);
        fr.stf(-0xb00, x6);
        if g.guard == 0 {
            fr.st(-0xae8, 0x400);
            fr.st(-0xae0, r13 as u32);
            fr.st(-0xaf0, 0);
            fr.st(-0x30, r14 as u32);
            build_table(g, sv, fr);
        }
        let f = fr.ldf(-0xb00);
        let idx = search(g, f, 0, 0x400);
        let e = g.entry(idx);
        let r13b = r13.wrapping_add(e[2] as i32);
        let r13n = sv.hash2(r14 as u32, r13b as u32);
        let eax = sv.hash1(r13n) & 0x3fff_ffff;
        let x0 = eax as i32 as f64;
        let x1 = fused64([KD_2M29, x0, KD_ONE], x0.mul_add(KD_2M29, -KD_ONE)); // 0x455eef vfmsub213sd
        let x2 = fr.ldf(-0xb08);
        let x1 = sgl(x1);
        let x0a = f32::from_bits(x1.to_bits() & 0x7fff_ffff);
        if !jae(x2, x0a) {
            let q = div(x1, x0a);
            let m = fr.ldf(-0xb08);
            let r = fused32([q, x1, m], (-q).mul_add(m, x1)); // 0x455f16 vfnmadd132ss
            fr.stf(-0x80, r);
        }
        let x4 = fr.ldf(-0xb10);
        let x2 = fr.ldf(-0x40);
        let r13m = (r13n & 0x3fff_ffff) as i32;
        let x5 = r13m as f64;
        let x3 = sub(x2, x4);
        let x4d = dbl(x4);
        let x3d = dbl(x3);
        let x6 = dmul(x3d, KD_2M30);
        let x6 = fused64([x6, x5, x4d], x5.mul_add(x6, x4d)); // 0x455f5c vfmadd213sd
        let x1 = fr.ldf(-0xb00);
        let x0 = dbl(fr.ldf(-0x48));
        let x7 = sub(x1, f32::from_bits(e[1]));
        let x6 = sgl(x6);
        let x8 = dmul(x0, KD_HALF);
        let mem = fr.ldf(-0x80);
        let x7 = fused32([x7, x6, mem], x7.mul_add(mem, x6)); // 0x455f84 vfmadd132ss
        let x7d = dbl(x7);
        let x9b = dmul(x8, x7d);
        x9 = sgl(x9b);
    }
    // ---- tail 0x455f97
    let x0 = fr.ldf(-0x58);
    let x1 = add(x0, fr.ldf(-0x50));
    add(x1, x9)
}
