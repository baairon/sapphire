//! PROVENANCE: copied (not moved) from wave_shake/rebuild/src/native.rs (builder R-series, finished). Shake keeps its own file; this copy is
//! the shared one. Contents: HashTable/hash1/hash2, memset_x, round_q12_x, cvttsd2si32, weight tables.
//!
//! native: hand-written Rust twins of the small non-math callees of the AVX render body and of the
//! one-time table builders that feed it. Every function names the dump offset of the code it replaces (the
//! offsets are positions in `neutral_1.text.bin`; the data addresses quoted are the captured-image VAs, which are
//! the dump offsets plus the code bias). Source of truth: the code dump and the captured data image only.
//!
//! Integer and float widths follow the originals: hashes are 32-bit wrapping integer code, the kernel math is
//! `f32` with the one `f64` detour the code takes, and every float to integer conversion is the x86 `cvttsd2si`
//! (an out-of-range or NaN input gives 0x80000000, NOT Rust's saturating `as i32`).
//!
//! What is here (the whole call set of except the math chain, which is `math.rs`):
//!
//! | original                 | here                                    | role |
//! |--------------------------|-----------------------------------------|------|
//! | | [`HashTable::build`] | builder of the 1,024 byte hash table |
//! | | [`hash_1`] / [`HashTable::hash1`] | 4 round table hash of one u32 |
//! | | [`hash_2`] / [`HashTable::hash2`] | 8 round table hash of two u32 |
//! | | [`memset_x`] | memset (the body zeroes bytes of its frame) |
//! | | [`round_q12_x`] | f32 -> i32 round to 1/4096 units, half away from zero |
//! | | [`build_weights_2`] / [`build_weights_1`] | builders of the two 4,608 entry weight tables |
//! | | [`scale_x`] | scale float from the cutoff integer |

// ---------------------------------------------------------------------------------------------------------------
// Hash table: builder and the two hashes 
// ---------------------------------------------------------------------------------------------------------------

/// Number of dwords of the hash table (1,024 bytes).
pub const HASH_TABLE_LEN: usize = 256;

/// Modulus of the generator inside (the classic ran2 constants: 4096, 150889, 714025).
pub const LCG_MOD: u64 = 714_025;
/// Multiplier of the generator (applied as `x << 12`).
pub const LCG_MUL: u64 = 4096;
/// Increment of the generator (`lea r10,[rcx]`).
pub const LCG_ADD: u64 = 150_889;

/// One generator step of : `x -> (4096 x + 150889) mod 714025`computed the way the code does it
/// `ecx = x << 12` (32-bit), `n = rcx + `signed 64x64 -> 128 `imul` by
/// 0x5dfc995681872319, `q = high_half >> 18`result `rcx + q * -714025 + ` (the low 32 bits are kept).
/// The test `lcg_step_equals_modulo_for_every_state` proves it equals `%` for every state 0..714025.
#[inline]
pub fn lcg_step_x(x: u32) -> u32 {
    const MAGIC: i64 = 0x5dfc_9956_8187_2319_u64 as i64;
    let rcx = (x << 12) as u64; // `shl ecx,0xc`: 32-bit shift, zero extended
    let n = rcx.wrapping_add(LCG_ADD); // `lea r10,[rcx]`
    let prod = (n as i64 as i128) * (MAGIC as i128); // `imul r10` (rdx:rax)
    let q = ((prod >> 64) as u64) >> 18; // `shr rdx,0x12`
    let r = rcx
        .wrapping_add(q.wrapping_mul(0xffff_ffff_fff5_1ad7)) // `imul r11,rdx,-714025`
        .wrapping_add(LCG_ADD); // `lea rdi,[rcx+r11*1]`
    r as u32 // the next `mov ecx,edi` keeps the low 32 bits
}

/// The hash table of plus the extra dword the builder leaves behind.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HashTable {
    /// `T[k] = (y1 >> 2) + (y2 << 8) + (y3 << 18)` (wrapping u32), the three generator values of entry k.
    pub t: [u32; HASH_TABLE_LEN],
    /// Last generator value (ecx), stored by the original to the dword (dump).
    /// Nothing in the render pair reads it (INFERRED: it is the generator state kept for another user).
    pub tail: u32,
}

impl HashTable {
    ///  run once from the zero guard state (guard dword = 0, which is the .bss state; the
    /// original returns immediately when the guard is already 1, that is "a second call does nothing", which in
    /// this functional form is simply not calling `build` again). The generator state starts at ecx = 0 and
    /// carries from entry to entry (the third value of entry k is the seed of entry k + 1).
    pub fn build() -> HashTable {
        let mut t = [0u32; HASH_TABLE_LEN];
        let mut state: u32 = 0; // `xor ecx,ecx`
        for slot in t.iter_mut() {
            let y1 = lcg_step_x(state);
            let y2 = lcg_step_x(y1);
            let y3 = lcg_step_x(y2);
            *slot = (y1 >> 2).wrapping_add(y2 << 8).wrapping_add(y3 << 18);
            state = y3;
        }
        HashTable { t, tail: state }
    }

    /// The table as the 1,024 little-endian bytes .
    pub fn to_bytes(&self) -> [u8; HASH_TABLE_LEN * 4] {
        let mut out = [0u8; HASH_TABLE_LEN * 4];
        for (i, v) in self.t.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        out
    }

    /// Read a table back from its 1,024 bytes.
    pub fn from_bytes(bytes: &[u8; HASH_TABLE_LEN * 4], tail: u32) -> HashTable {
        let mut t = [0u32; HASH_TABLE_LEN];
        for (i, v) in t.iter_mut().enumerate() {
            *v = u32::from_le_bytes([bytes[i * 4], bytes[i * 4 + 1], bytes[i * 4 + 2], bytes[i * 4 + 3]]);
        }
        HashTable { t, tail }
    }

    ///  on this table.
    #[inline]
    pub fn hash1(&self, x: u32) -> u32 {
        hash_1(&self.t, x)
    }

    ///  on this table.
    #[inline]
    pub fn hash2(&self, x: u32, y: u32) -> u32 {
        hash_2(&self.t, x, y)
    }
}

/// One hash round of both functions: `movzx r,dil ; shr edi,8 ; add edi,[T + r*4]`.
#[inline(always)]
fn round(t: &[u32; HASH_TABLE_LEN], v: u32) -> u32 {
    (v >> 8).wrapping_add(t[(v & 0xff) as usize])
}

/// one argument (edi), returns eax. Four rounds of `v = (v >> 8) + T[v & 0xff]`all in u32 wrapping
/// arithmetic. Reads only the table; no flags, no other state.
#[inline]
pub fn hash_1(t: &[u32; HASH_TABLE_LEN], x: u32) -> u32 {
    let mut v = x;
    for _ in 0..4 {
        v = round(t, v);
    }
    v
}

/// two arguments (edi = x, esi = y), returns eax. Four rounds on x, then `add edi,esi` (
/// wrapping), then four more rounds. Reads only the table.
#[inline]
pub fn hash_2(t: &[u32; HASH_TABLE_LEN], x: u32, y: u32) -> u32 {
    let mut v = x;
    for _ in 0..4 {
        v = round(t, v);
    }
    v = v.wrapping_add(y);
    for _ in 0..4 {
        v = round(t, v);
    }
    v
}

// ---------------------------------------------------------------------------------------------------------------
// memset: (a `jmp `)
// ---------------------------------------------------------------------------------------------------------------

/// `memset(rdi = dst, esi = value, rdx = count)`returns rdi. The code fills exactly
/// `count` bytes with the LOW BYTE of esi (`mov dl,sil`, then replicated by `imul` with 0x0101010101010101) and
/// treats `count` as a SIGNED 64-bit value: `count <= 0` returns at once without a store (`jl`), so a
/// negative count is a no-op, not a huge fill. Here `dst` is the destination slice and `count` is the original
/// signed count; a count larger than `dst.len()` panics (the original would write out of bounds: undefined).
///
/// The only call in the AVX body is `memset(rbp, 0)` (2,600 bytes).
/// Return value (rax = rdi) is the destination itself, which a slice API has no use for.
///
/// MEASURED/INFERRED split: the fill contract is read from the dump listing (all stores of every path cover
/// exactly `[rdi, rdi + count)`); the real code dispatches through four jump tables and two CPU dependent data
/// words that are NOT in the captured image (data_gaps.tsv:,
/// ), so the oracle test of this function runs with RECONSTRUCTED tables (INFERRED).
pub fn memset_x(dst: &mut [u8], value: i32, count: i64) {
    if count <= 0 {
        return;
    }
    let n = count as u64 as usize;
    assert!(n <= dst.len(), "memset_x: count {} exceeds the destination ({} bytes)", count, dst.len());
    let b = value as u8;
    for x in dst[..n].iter_mut() {
        *x = b;
    }
}

// ---------------------------------------------------------------------------------------------------------------
// float -> integer in units of 1/4096
// ---------------------------------------------------------------------------------------------------------------

/// x86 `cvttsd2si eax, xmm`: truncate toward zero; NaN and any value outside the i32 range give the "integer
/// indefinite" 0x80000000 (`i32::MIN`). Rust's `as i32` would saturate and map NaN to 0, which is NOT what the
/// instruction does.
#[inline]
pub fn cvttsd2si32(x: f64) -> i32 {
    if x.is_nan() {
        return i32::MIN;
    }
    let t = x.trunc();
    if t >= -2147483648.0 && t <= 2147483647.0 {
        t as i32
    } else {
        i32::MIN
    }
}

/// Constants read from the captured image by (VA, value): 4096.0f32, 0.5f64,
/// the sign mask 0x8000000000000000 .
const Q12_SCALE: f32 = 4096.0;
const Q12_HALF: f64 = 0.5;
const SIGN64: u64 = 0x8000_0000_0000_0000;

/// `eax = cvttsd2si(copysign(0.5, x) + (f64)(x * 4096.0f32))`i.e. round `x * 4096` half away from
/// zero (the multiply is the f32 `mulss`, the sum is an f64 `addsd`). Out of range, inf and NaN give i32::MIN.
/// The sign comes from the original argument converted to f64 (`cvtss2sd`, `andps` with the sign mask, `orps`).
#[inline]
pub fn round_q12_x(x: f32) -> i32 {
    let scaled = (x * Q12_SCALE) as f64; // mulss xmm0,xmm3 ; cvtss2sd
    let signed_half = f64::from_bits(Q12_HALF.to_bits() | ((x as f64).to_bits() & SIGN64)); // andps / andnps / orps
    cvttsd2si32(signed_half + scaled) // addsd ; cvttsd2si
}

// ---------------------------------------------------------------------------------------------------------------
// Weight tables: builders (SSE twin) and (AVX twin), dispatcher 
// ---------------------------------------------------------------------------------------------------------------

/// Entries per weight table (0x1200 iterations of the builders).
pub const WEIGHT_LEN: usize = 0x1200;

/// Constants of the kernel, read from the captured image: 4608.0f32, threshold 0.5f32,
/// 2.0f32, 1.0f32, 2^-12 f32, 2.0f64, 1.0f64 .
const W_DIVISOR: f32 = 4608.0;
const W_SPLIT: f32 = 0.5;
const W_TWO: f32 = 2.0;
const W_ONE: f32 = 1.0;
const W_UNIT: f32 = 1.0 / 4096.0;
const W_TWO64: f64 = 2.0;
const W_ONE64: f64 = 1.0;

/// Kernel value (before rounding) of entry `i`SSE twin (separate mul and sub/mul roundings):
/// `s = sqrtf(i / 4608)`; if `0.5 > s`: `(f32)(1.0 - 2.0 * (f64)(s*s))`, else `(2.0 * (1.0 - s)) * (1.0 - s)`.
/// The `0.5 > s` test is `comiss`/`jbe`: NaN would take the else branch (cannot happen for i >= 0).
#[inline]
pub fn weight_kernel_2(i: u32) -> f32 {
    let s = ((i as i32) as f32 / W_DIVISOR).sqrt(); // cvtsi2ss ; divss ; sqrtss
    if W_SPLIT > s {
        let t = (s * s) as f64; // mulss ; cvtss2sd
        (W_ONE64 - t * W_TWO64) as f32 // mulsd ; subsd (D0 - t*D1) ; cvtsd2ss
    } else {
        let d = W_ONE - s; // subss (1.0 - s)
        (W_TWO * d) * d // mulss ; mulss
    }
}

/// Kernel value of entry `i`AVX twin (fused operations): first branch `vfnmadd213sd`
/// (`1.0 - 2.0*t` in one rounding), else branch `a = fma(s, 2.0, -2.0)` (`vfmsub213ss`) then `fma(s, a, -a)`
/// (`vfmsub231ss`). `f32::mul_add` / `f64::mul_add` are always fused, so this is deterministic on any host.
#[inline]
pub fn weight_kernel_1(i: u32) -> f32 {
    let s = ((i as i32) as f32 / W_DIVISOR).sqrt(); // vcvtsi2ss ; vdivss ; vsqrtss
    if W_SPLIT > s {
        let t = (s * s) as f64; // vmulss ; vcvtss2sd
        (-W_TWO64).mul_add(t, W_ONE64) as f32 // vfnmadd213sd xmm0,xmm1,[1.0] ; vcvtsd2ss
    } else {
        let a = s.mul_add(W_TWO, -W_TWO); // vfmsub213ss xmm0,xmm2,xmm2  (s*2 - 2)
        s.mul_add(a, -a) // vfmsub231ss xmm0,xmm1,xmm0  (s*a - a)
    }
}

/// Both weight tables and the cutoff the builders store.
#[derive(Clone, PartialEq, Debug)]
pub struct WeightTables {
    /// Int table : `round_q12_x(kernel(i))`4,608 entries (units of 1/4096).
    pub int: Vec<i32>,
    /// Float table : `(f32)int[i] * 2^-12`4,608 entries.
    pub float: Vec<f32>,
    /// `2 * i0` where `i0` is the FIRST index whose int entry is 0 (stored by the builders to the dword at
    /// dump); `None` if no entry is zero (the original then leaves the dword untouched).
    pub cutoff: Option<i32>,
}

impl WeightTables {
    /// Int table as the 18,432 little-endian bytes .
    pub fn int_bytes(&self) -> Vec<u8> {
        self.int.iter().flat_map(|v| v.to_le_bytes()).collect()
    }
    /// Float table as the 18,432 little-endian bytes .
    pub fn float_bytes(&self) -> Vec<u8> {
        self.float.iter().flat_map(|v| v.to_le_bytes()).collect()
    }
}

fn build_weights(kernel: fn(u32) -> f32) -> WeightTables {
    let mut int = Vec::with_capacity(WEIGHT_LEN);
    let mut float = Vec::with_capacity(WEIGHT_LEN);
    let mut cutoff = None;
    for i in 0..WEIGHT_LEN as u32 {
        let e = round_q12_x(kernel(i)); // call 
        int.push(e); // mov [r14+r12*4],eax
        float.push((e as f32) * W_UNIT); // cvtsi2ss ; mulss [2^-12] ; movss [rbx+r12*4]
        if cutoff.is_none() && e == 0 {
            cutoff = Some((i as i32).wrapping_add(i as i32)); // `add r15d,r15d`, once (flag r13b)
        }
    }
    WeightTables { int, float, cutoff }
}

/// the SSE twin of the builder (called through the dispatcher when the AVX feature word is
/// not complete). Whole run, 0x1200 iterations.
pub fn build_weights_2() -> WeightTables {
    build_weights(weight_kernel_2)
}

/// the AVX twin of the builder (the one the rebuild's AVX body uses). Whole run, 0x1200 iterations.
pub fn build_weights_1() -> WeightTables {
    build_weights(weight_kernel_1)
}

///  (static initialiser): `scale = (f32)cutoff_int * 2^-12` (`cvtsi2ss` of the dword,
/// `mulss` by 2^-12, stored). Scale decision of the owner: use the BUILT cutoff 9074
/// (`build_weights_*().cutoff == Some(9074)`, scale 2.2153320); the captured integer is 9215 (scale 2.2497559).
#[inline]
pub fn scale_x(cutoff: i32) -> f32 {
    (cutoff as f32) * W_UNIT
}

/// The built cutoff integer (decision: 9074, not the captured 9215) and its scale value.
pub const CUTOFF_BUILT: i32 = 9074;
/// The integer present in the captured image .
pub const CUTOFF_CAPTURED: i32 = 9215;
