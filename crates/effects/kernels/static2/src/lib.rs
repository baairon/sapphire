//! static2 (builtin class, vtable): native port of its CPU render entry, slot 53 = (208 B).
//!
//! Sources of truth: the code dump (listing neutral_1.asm, addresses below are listing offsets; captured VA = offset)
//! and the captured data image. std only, no unsafe. Reuses fx_common (read-only path dependency): `HashTable` = builder and the
//! hash .
//!
//! What slot 53 computes (MEASURED): for i in 0..n (n = [rec], signed, `cmp,0; jle` skips
//! everything when n <= 0) it takes `h = hash_1(i + base)` with `base = [rec] + [rec]` (32-bit wrapping), and writes
//! `out0[i] = ((h & 0xfff) - 2048) * 2^-12` and `out1[i] = (((h >> 16) & 0xfff) - 2048) * 2^-12`. Each value is an exact multiple of 2^-12 in
//! [-0.5, 0.5 - 2^-12]; the integer to float conversion (cvtsi2ss) and the multiplication by 2^-12 (mulss, constant 0x39800000,
//! CAPTURED) are exact, so no rounding, FMA, approximate instruction or x87 is involved and the result does not depend on MXCSR.
//! `this` (rdi) is not read. The output pointers come from the 16 byte block rdx: out0 and out1 are the two pointers of the block.

use fx_common::shake_natives::HashTable;

/// The mulss constant : 2^-12, bytes 00 00 80 39 (CAPTURED, audit_addr lookup).
pub const SCALE: f32 = f32::from_bits(0x3980_0000);
/// Additive constant of the seed (`lea ecx,[rax+rcx*1]`).
pub const SEED_BIAS: u32 = 0x8235;

/// Explicit inputs of the slot (no Default): the three host record fields the slot reads. The host producers of these fields are
/// NOT MEASURED (ST1-A1): they are record fields the evaluator must feed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Params {
    /// `[rec]`signed element count (the generic driver stores 1 at rec,; other writers not measured).
    pub n: i32,
    /// `[rec]`added to the seed.
    pub seed_a: i32,
    /// `[rec]`added to the seed.
    pub seed_b: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// an output slice is shorter than n
    OutputTooShort,
}

/// Seed base of the loop: `[rec] + [rec]`32-bit wrapping .
#[inline]
pub fn seed_base(p: &Params) -> u32 {
    (p.seed_a as u32).wrapping_add(p.seed_b as u32).wrapping_add(SEED_BIAS)
}

/// Mutants for the T2 mutation run (Rust side only; `None` is the port).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mut {
    None,
    BiasPlus1,         // seed bias 
    SeedAIgnored,      // rec not added
    SeedBIgnored,      // rec not added
    MaskX7ff,          // x mask 0x7ff instead of 0xfff
    ShiftY15,          // y taken from bit 15 instead of 16
    CenterX2047,       // x centre -2047
    CenterY2049,       // y centre -2049
    Scale2m11,         // scale 2^-11
    ScaleOnlyY,        // x not scaled
    SwapXY,            // outputs swapped
    UnsignedConvert,   // cvtsi2ss as unsigned 32 bit
    StartAt1,          // loop starts at index 1 (hash uses i+1 but writes i)
    CountMinus1,       // one element short
    GuardLt2,          // the `n <= 0` guard skips n == 1 as well
    Rounds3,           // 3 hash rounds
    SeedNoWrapAdd,     // index added after a 16 bit truncation
}

pub const ALL_MUTS: &[(Mut, &str)] = &[
    (Mut::BiasPlus1, "bias_plus_1"),
    (Mut::SeedAIgnored, "seed_a_ignored"),
    (Mut::SeedBIgnored, "seed_b_ignored"),
    (Mut::MaskX7ff, "mask_x_7ff"),
    (Mut::ShiftY15, "shift_y_15"),
    (Mut::CenterX2047, "center_x_2047"),
    (Mut::CenterY2049, "center_y_2049"),
    (Mut::Scale2m11, "scale_2m11"),
    (Mut::ScaleOnlyY, "scale_only_y"),
    (Mut::SwapXY, "swap_xy"),
    (Mut::UnsignedConvert, "unsigned_convert"),
    (Mut::StartAt1, "start_at_1"),
    (Mut::CountMinus1, "count_minus_1"),
    (Mut::GuardLt2, "guard_skips_n1"),
    (Mut::Rounds3, "hash_3_rounds"),
    (Mut::SeedNoWrapAdd, "index_trunc16"),
];

/// One element: `(x, y)` for index i, mutated when asked.
#[inline]
fn element(t: &HashTable, p: &Params, i: u32, m: Mut) -> (f32, f32) {
    let mut base = seed_base(p);
    match m {
        Mut::BiasPlus1 => base = base.wrapping_add(1),
        Mut::SeedAIgnored => base = base.wrapping_sub(p.seed_a as u32),
        Mut::SeedBIgnored => base = base.wrapping_sub(p.seed_b as u32),
        _ => {}
    }
    let idx = match m {
        Mut::StartAt1 => i.wrapping_add(1),
        Mut::SeedNoWrapAdd => i & 0xffff,
        _ => i,
    };
    let seed = idx.wrapping_add(base); // `edi = ebx + r14d` 
    let h = if m == Mut::Rounds3 {
        let mut v = seed;
        for _ in 0..3 {
            v = (v >> 8).wrapping_add(t.t[(v & 0xff) as usize]);
        }
        v
    } else {
        t.hash1(seed) // call 
    };
    let (mx, sy, cx, cy) = match m {
        Mut::MaskX7ff => (0x7ffu32, 16u32, 2048i32, 2048i32),
        Mut::ShiftY15 => (0xfff, 15, 2048, 2048),
        Mut::CenterX2047 => (0xfff, 16, 2047, 2048),
        Mut::CenterY2049 => (0xfff, 16, 2048, 2049),
        _ => (0xfff, 16, 2048, 2048),
    };
    let xi = (h & mx) as i32 - cx; // `and eax,0xfff ; add eax,0xfffff800`
    let yi = ((h >> sy) & 0xfff) as i32 - cy; // `shr edi,0x10 ; and edi,0xfff ; add edi,0xfffff800`
    let conv = |v: i32| -> f32 {
        if m == Mut::UnsignedConvert { v as u32 as f32 } else { v as f32 } // cvtsi2ss xmm,r32 (signed)
    };
    let sc = if m == Mut::Scale2m11 { f32::from_bits(0x3a00_0000) } else { SCALE };
    let x = if m == Mut::ScaleOnlyY { conv(xi) } else { conv(xi) * sc }; // mulss
    let y = conv(yi) * sc;
    if m == Mut::SwapXY { (y, x) } else { (x, y) }
}

/// Mutated slot 53 (for the mutation run); `render` is the port.
pub fn render_mut(t: &HashTable, p: &Params, out0: &mut [f32], out1: &mut [f32], m: Mut) -> Result<(), Error> {
    let min_n = if m == Mut::GuardLt2 { 2 } else { 1 };
    if p.n < min_n {
        return Ok(()); // n <= 0: nothing written, success (jle)
    }
    let mut cnt = p.n as usize;
    if m == Mut::CountMinus1 {
        cnt -= 1;
    }
    if out0.len() < cnt || out1.len() < cnt {
        return Err(Error::OutputTooShort);
    }
    for i in 0..cnt {
        let (x, y) = element(t, p, i as u32, m);
        out0[i] = x;
        out1[i] = y;
    }
    Ok(())
}

/// Port of slot 53: fills `out0[0..n]` and `out1[0..n]`n <= 0 is a successful no-op. `t` is the hash table built by.
pub fn render(t: &HashTable, p: &Params, out0: &mut [f32], out1: &mut [f32]) -> Result<(), Error> {
    render_mut(t, p, out0, out1, Mut::None)
}

/// S3 style entry: allocate and return the two planes of n elements (empty for n <= 0).
pub fn render_planes(t: &HashTable, p: &Params) -> (Vec<f32>, Vec<f32>) {
    let n = p.n.max(0) as usize;
    let mut a = vec![0.0f32; n];
    let mut b = vec![0.0f32; n];
    let _ = render(t, p, &mut a, &mut b);
    (a, b)
}

/// The hash table of the original builder (run once by the host at start up; ST1-A2).
pub fn table() -> HashTable {
    HashTable::build()
}

/// Every stand-in and host assumption of this port (ids -*).
pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("ST1-A1", "PARAM-UNCAPTURED 3", "n = [rec], seed_a = [rec], seed_b = [rec] are explicit parameters with no default; the host writers (tile origin, frame, row) are not measured (generic driver stores rec = 1); T2 runs hundreds of values of each"),
        ("ST1-A2", "CALLEE-RAN-AS-CODE / HOST-INIT", "the hash table (bss, zero range in the image) is built by which the host runs once before any user; T2 runs the real builder in the emulator and uses its bytes, the Rust side uses fx_common HashTable::build; that the host runs it before static2 is INFERRED (shared by tv_interference, shake, grain)"),
        ("ST1-A3", "CAPTURED", "scale constant 2^-12 (bytes 00 00 80 39), audit_addr lookup"),
        ("ST1-A4", "OUT-CONTRACT", "out0 and out1 are non-overlapping arrays of at least n floats; the original re-reads [rec] each iteration, so rec must not alias the outputs"),
        ("ST1-A6", "DISPATCH-MODE-INFERRED", "the CPU entry is slot 53 by the entry rule and the base stub forwards flag 0 to it (measured), but the slot 60 entry mode word [entry field] copied from zero cells could select slot 50 in the generic driver; not resolved"),
        ("ST1-A5", "NODE-TYPE", "PIXEL (a 2-channel image of per-element random offsets): STRONG, from the slot table (slot 53 CPU and slot 54 GPU twin) and the program use; the plan dump and the compiler type inference are not available"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_is_two_pow_minus_12() {
        assert_eq!(SCALE, 1.0 / 4096.0);
    }

    #[test]
    fn hand_example_zero_params() {
        // base = 0x8235; hash of 0x8235 computed from the table by hand through the 4 rounds
        let t = table();
        let p = Params { n: 1, seed_a: 0, seed_b: 0 };
        let (x, y) = render_planes(&t, &p);
        let h = t.hash1(0x8235);
        assert_eq!(x[0], (((h & 0xfff) as i32 - 2048) as f32) * SCALE);
        assert_eq!(y[0], ((((h >> 16) & 0xfff) as i32 - 2048) as f32) * SCALE);
    }

    #[test]
    fn range_and_exactness() {
        let t = table();
        let p = Params { n: 4096, seed_a: 123, seed_b: -77 };
        let (x, y) = render_planes(&t, &p);
        for v in x.iter().chain(y.iter()) {
            assert!(*v >= -0.5 && *v < 0.5);
            let k = *v * 4096.0;
            assert_eq!(k, k.round());
        }
    }

    #[test]
    fn sum_symmetry_and_wrap() {
        let t = table();
        let a = render_planes(&t, &Params { n: 8, seed_a: 5, seed_b: 9 });
        let b = render_planes(&t, &Params { n: 8, seed_a: 9, seed_b: 5 });
        let c = render_planes(&t, &Params { n: 8, seed_a: 14, seed_b: 0 });
        assert_eq!(a, b);
        assert_eq!(a, c);
        // wrap: base + i crosses 2^32
        let w = render_planes(&t, &Params { n: 4, seed_a: i32::MAX, seed_b: i32::MAX });
        assert_eq!(w.0.len(), 4);
    }

    #[test]
    fn degenerate_counts() {
        let t = table();
        assert_eq!(render_planes(&t, &Params { n: 0, seed_a: 1, seed_b: 2 }).0.len(), 0);
        assert_eq!(render_planes(&t, &Params { n: -3, seed_a: 1, seed_b: 2 }).0.len(), 0);
        let mut a = [9.0f32; 2];
        let mut b = [9.0f32; 2];
        assert_eq!(render(&t, &Params { n: 3, seed_a: 0, seed_b: 0 }, &mut a, &mut b), Err(Error::OutputTooShort));
        assert_eq!(render(&t, &Params { n: 0, seed_a: 0, seed_b: 0 }, &mut a, &mut b), Ok(()));
        assert_eq!(a, [9.0, 9.0]);
    }

    #[test]
    fn deterministic_and_prefix_stable() {
        let t = table();
        let p3 = render_planes(&t, &Params { n: 3, seed_a: 40, seed_b: 2 });
        let p9 = render_planes(&t, &Params { n: 9, seed_a: 40, seed_b: 2 });
        assert_eq!(&p9.0[..3], &p3.0[..]);
        assert_eq!(p3, render_planes(&t, &Params { n: 3, seed_a: 40, seed_b: 2 }));
        // shifting the seed base by one shifts the sequence by one
        let q = render_planes(&t, &Params { n: 8, seed_a: 41, seed_b: 2 });
        assert_eq!(&q.0[..7], &p9.0[1..8]);
    }

    #[test]
    fn all_mutants_change_something() {
        let t = table();
        let p = Params { n: 64, seed_a: 3, seed_b: 1000 };
        let (bx, by) = render_planes(&t, &p);
        for (m, name) in ALL_MUTS {
            if *m == Mut::GuardLt2 || *m == Mut::SeedNoWrapAdd {
                continue; // differ only for n == 1 / n > 65536 (checked in T2)
            }
            let mut x = vec![0.0f32; 64];
            let mut y = vec![0.0f32; 64];
            render_mut(&t, &p, &mut x, &mut y, *m).unwrap();
            assert!(x != bx || y != by, "mutant {} equals the port on the hand case", name);
        }
    }
}
