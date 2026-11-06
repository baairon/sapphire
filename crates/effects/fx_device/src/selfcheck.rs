//! The device gate: comparison helpers, the adversarial corpus and the base self-check.
// Provenance: adapted from twixtor plugins/retime-engine/src/device/selfcheck.rs (8087d06): Mismatch, Refusal, NanPayloads, Tally, corpus idea.
//!
//! Before a device renders anything, the base kernels run on it over an adversarial corpus and every output is
//! compared bit for bit with the CPU code; any difference refuses the device and names the first failing input.
//! The executor then runs the same comparison for each allowlisted op kernel against the CPU kernel itself
//! (`fx_exec::device_ops`), using the corpus builders here.
//!
//! The corpus covers f32 values with subnormals, -0.0, NaN (quiet and signalling, with payloads), infinities, the
//! largest finite values, exact ties, and products where a fused multiply add rounds differently from a multiply
//! and an add (found with `f32::mul_add` when the corpus is built); odd sizes including 1; and a dependent loop of
//! 64 steps.
//!
//! NaN payloads: IEEE 754 fixes which operations give NaN but not the payload. The check requires NaN in exactly
//! the places the CPU has one and reports payload differences apart ([`NanPayloads`]); the executor never takes a
//! float output that holds a NaN from the device, so a payload difference is never visible.

use crate::cl::{Arg, Buffer};
use crate::image::Session;
use crate::kernels;
use crate::probe;
use crate::sim::Rng;
use std::fmt;

/// The first output that differs from the CPU.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mismatch {
    pub kernel: String,
    /// The launch within the kernel's corpus (for example the size).
    pub case: String,
    pub index: usize,
    pub input: String,
    pub expected: String,
    pub got: String,
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: output {} for {}: expected {}, got {}", self.kernel, self.case, self.index, self.input, self.expected, self.got)
    }
}

/// Why a device is refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The probe requirements do not hold; nothing ran.
    Requirements(Vec<String>),
    /// The driver failed: no context, a build error, a failed launch or read.
    Driver(String),
    /// A kernel gave other bits than the CPU.
    Mismatch(Mismatch),
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::Requirements(r) => write!(f, "{}", r.join("; ")),
            Refusal::Driver(e) => write!(f, "driver: {e}"),
            Refusal::Mismatch(m) => write!(f, "differs from the CPU: {m}"),
        }
    }
}

/// How the device's f32 NaN results compare with the CPU's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NanPayloads {
    Same,
    /// The first output where both are NaN with other bits.
    Differ(Mismatch),
}

/// What the checks of one source found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tally {
    /// Outputs compared.
    pub outputs: usize,
    pub nan_payloads: NanPayloads,
}

impl Tally {
    pub fn empty() -> Tally {
        Tally { outputs: 0, nan_payloads: NanPayloads::Same }
    }

    pub fn add(&mut self, other: Tally) {
        self.outputs += other.outputs;
        if let (NanPayloads::Same, NanPayloads::Differ(m)) = (&self.nan_payloads, other.nan_payloads) {
            self.nan_payloads = NanPayloads::Differ(m);
        }
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.nan_payloads {
            NanPayloads::Same => write!(f, "ok ({} outputs equal to the CPU)", self.outputs),
            NanPayloads::Differ(m) => write!(f, "ok ({} outputs equal to the CPU; NaN payloads differ, first: {m})", self.outputs),
        }
    }
}

/// Compares two f32 outputs by bits. Equal bits pass; two NaNs with other bits are a payload difference; anything else is a mismatch.
pub fn compare_f32(kernel: &str, case: &str, input: impl Fn(usize) -> String, expected: &[f32], got: &[f32]) -> Result<Tally, Mismatch> {
    let show = |v: f32| format!("0x{:08x} ({v:e})", v.to_bits());
    if expected.len() != got.len() {
        return Err(Mismatch {
            kernel: kernel.into(),
            case: case.into(),
            index: 0,
            input: "length".into(),
            expected: expected.len().to_string(),
            got: got.len().to_string(),
        });
    }
    let mut nan_payloads = NanPayloads::Same;
    for (i, (&e, &g)) in expected.iter().zip(got).enumerate() {
        if e.to_bits() == g.to_bits() {
            continue;
        }
        let m = Mismatch { kernel: kernel.into(), case: case.into(), index: i, input: input(i), expected: show(e), got: show(g) };
        if e.is_nan() && g.is_nan() {
            if nan_payloads == NanPayloads::Same {
                nan_payloads = NanPayloads::Differ(m);
            }
        } else {
            return Err(m);
        }
    }
    Ok(Tally { outputs: expected.len(), nan_payloads })
}

/// f32 bit patterns that matter: zeros, subnormals, normals at the edges, ties, infinities, NaNs of both signs with payloads.
pub const SPECIAL_F32_BITS: &[u32] = &[
    0x0000_0000, 0x8000_0000, 0x0000_0001, 0x8000_0001, 0x0000_0002, 0x007f_ffff, 0x807f_ffff, 0x0040_0000, 0x0080_0000, 0x8080_0000, 0x0080_0001,
    0x3f80_0000, 0xbf80_0000, 0x3f00_0000, 0xbf00_0000, 0x3f80_0001, 0x3f7f_ffff, 0xbf7f_ffff, 0x3e80_0000, 0x3380_0000, 0x3300_0000, 0x3400_0000,
    0x3300_0001, 0x32ff_ffff, 0x3000_0000, 0x4b00_0000, 0x4b80_0000, 0xcb00_0000, 0x4b00_0001, 0x4700_0000, 0x4effffff, 0x4f00_0000, 0x5f00_0000,
    0xcf00_0000, 0x7f7f_ffff, 0xff7f_ffff, 0x7f00_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_0000, 0xffc0_0000, 0x7fc0_0001, 0xffc0_0001, 0x7fff_ffff,
    0xffff_ffff, 0x7f80_0001, 0xff80_0001, 0x7fa0_0000, 0x7fc0_ffee,
];

/// The adversarial f32 inputs: the specials, then `random` seeded random bit patterns.
pub fn adversarial_f32(random: usize, seed: u64) -> Vec<f32> {
    let mut v: Vec<f32> = SPECIAL_F32_BITS.iter().map(|b| f32::from_bits(*b)).collect();
    let mut rng = Rng::new(seed);
    for _ in 0..random {
        v.push(f32::from_bits(rng.next_u32()));
    }
    v
}

/// Finite pairs `(x, k)` where `v = v * k + x` repeated [`kernels::CHAIN_STEPS`] times gives other bits fused than unfused
/// (found with `mul_add`), so a contracting build shows. Plus pairs that overflow and pairs with subnormals.
pub fn chain_pairs(count: usize, seed: u64) -> Vec<(f32, f32)> {
    let mut rng = Rng::new(seed);
    let mut out: Vec<(f32, f32)> = vec![(0.0, 0.0), (1.0, 1.0), (f32::MIN_POSITIVE / 2.0, 1.0), (f32::from_bits(1), 0.5), (f32::MAX, 2.0), (-0.0, 1.0)];
    let mut tries = 0;
    while out.len() < count + 6 && tries < 2_000_000 {
        tries += 1;
        let x = f32::from_bits(rng.next_u32() & 0xbfff_ffff);
        let k = f32::from_bits(0x3f00_0000 | (rng.next_u32() & 0x007f_ffff));
        if !x.is_finite() || x.abs() > 1.0e6 {
            continue;
        }
        let mut a = 0.0f32;
        for _ in 0..kernels::CHAIN_STEPS {
            a = a * k + x;
        }
        let mut fused = 0.0f32;
        for _ in 0..kernels::CHAIN_STEPS {
            fused = fused.mul_add(k, x);
        }
        if fused.to_bits() != a.to_bits() {
            out.push((x, k));
        }
    }
    out
}

/// The sizes the launch checks use: one, two, odd and near powers of two.
pub const SIZES: &[usize] = &[1, 2, 3, 7, 31, 33, 255, 257, 1001];

/// The probe requirements, the program build and every check of [`check_base`].
pub fn check_base(session: &Session) -> Result<Tally, Refusal> {
    let reasons = probe::requirements(&session.info);
    if !reasons.is_empty() {
        return Err(Refusal::Requirements(reasons));
    }
    let mut tally = Tally::empty();
    for check in [check_copy_u32, check_copy_f32, check_chain] {
        match check(session) {
            Ok(Ok(t)) => tally.add(t),
            Ok(Err(m)) => return Err(Refusal::Mismatch(m)),
            Err(e) => return Err(Refusal::Driver(e.to_string())),
        }
    }
    Ok(tally)
}

type Check = Result<Result<Tally, Mismatch>, crate::cl::ClError>;

fn cycle<T: Copy>(v: &[T], n: usize) -> Vec<T> {
    (0..n).map(|i| v[i % v.len()]).collect()
}

fn check_copy_u32(s: &Session) -> Check {
    let words: Vec<u32> = adversarial_f32(2000, 11).iter().map(|f| f.to_bits()).collect();
    let mut tally = Tally::empty();
    for &n in SIZES {
        let src = cycle(&words, n);
        let (a, b): (Buffer<u32>, Buffer<u32>) = (s.input(&src)?, s.output(n)?);
        s.launch("copy_u32", n, &[a.arg(), b.arg(), Arg::U32(n as u32)])?;
        let got = s.queue.read(&b)?;
        let (e, g): (Vec<f32>, Vec<f32>) = (src.iter().map(|w| f32::from_bits(*w)).collect(), got.iter().map(|w| f32::from_bits(*w)).collect());
        match compare_f32("copy_u32", &format!("n={n}"), |i| format!("0x{:08x}", src[i]), &e, &g) {
            Ok(t) => tally.add(t),
            Err(m) => return Ok(Err(m)),
        }
    }
    Ok(Ok(tally))
}

fn check_copy_f32(s: &Session) -> Check {
    let words: Vec<u32> = adversarial_f32(2000, 12).iter().map(|f| f.to_bits()).collect();
    let mut tally = Tally::empty();
    for &n in SIZES {
        let src = cycle(&words, n);
        let (a, b): (Buffer<u32>, Buffer<u32>) = (s.input(&src)?, s.output(n)?);
        s.launch("copy_f32", n, &[a.arg(), b.arg(), Arg::U32(n as u32)])?;
        let got = s.queue.read(&b)?;
        if got != src {
            let i = (0..n).find(|&i| got[i] != src[i]).unwrap_or(0);
            return Ok(Err(Mismatch {
                kernel: "copy_f32".into(),
                case: format!("n={n}"),
                index: i,
                input: format!("0x{:08x}", src[i]),
                expected: format!("0x{:08x}", src[i]),
                got: format!("0x{:08x}", got[i]),
            }));
        }
        tally.add(Tally { outputs: n, nan_payloads: NanPayloads::Same });
    }
    Ok(Ok(tally))
}

fn check_chain(s: &Session) -> Check {
    let pairs = chain_pairs(200, 13);
    let mut tally = Tally::empty();
    for &n in SIZES {
        let ps = cycle(&pairs, n);
        let (xs, ks): (Vec<f32>, Vec<f32>) = ps.iter().copied().unzip();
        let (xb, kb, ob) = (s.input(&xs)?, s.input(&ks)?, s.output::<f32>(n)?);
        s.launch("chain_f32", n, &[xb.arg(), kb.arg(), ob.arg(), Arg::U32(n as u32)])?;
        let got = s.queue.read(&ob)?;
        let expected: Vec<f32> = ps.iter().map(|&(x, k)| kernels::rs::chain_f32_item(x, k)).collect();
        match compare_f32("chain_f32", &format!("n={n}"), |i| format!("x=0x{:08x} k=0x{:08x}", xs[i].to_bits(), ks[i].to_bits()), &expected, &got) {
            Ok(t) => tally.add(t),
            Err(m) => return Ok(Err(m)),
        }
    }
    Ok(Ok(tally))
}

/// Runs `chain_f32` alone and reports whether it matched the unfused Rust loop. The test that a contracting build is
/// caught passes a session built from a source with `FP_CONTRACT ON`.
pub fn chain_matches(session: &Session) -> Result<bool, crate::cl::ClError> {
    Ok(check_chain(session)?.is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_separates_payloads_from_mismatches() {
        let nan_a = f32::from_bits(0x7fc0_0000);
        let nan_b = f32::from_bits(0xffc0_0000);
        assert!(compare_f32("k", "c", |_| String::new(), &[1.0, nan_a], &[1.0, nan_a]).unwrap().nan_payloads == NanPayloads::Same);
        let t = compare_f32("k", "c", |_| String::new(), &[1.0, nan_a], &[1.0, nan_b]).unwrap();
        assert!(matches!(t.nan_payloads, NanPayloads::Differ(_)));
        assert!(compare_f32("k", "c", |_| String::new(), &[1.0, nan_a], &[1.0, 2.0]).is_err());
        assert!(compare_f32("k", "c", |_| String::new(), &[0.0], &[-0.0]).is_err(), "signed zero is a mismatch");
        assert!(compare_f32("k", "c", |_| String::new(), &[1.0], &[1.0, 2.0]).is_err());
    }

    #[test]
    fn the_chain_corpus_has_pairs_where_fusion_changes_the_bits() {
        let pairs = chain_pairs(50, 13);
        let fusing = pairs
            .iter()
            .filter(|&&(x, k)| {
                let (mut a, mut f) = (0.0f32, 0.0f32);
                for _ in 0..kernels::CHAIN_STEPS {
                    a = a * k + x;
                    f = f.mul_add(k, x);
                }
                a.to_bits() != f.to_bits()
            })
            .count();
        assert!(fusing >= 40, "only {fusing} pairs distinguish fused from unfused");
    }

    #[test]
    fn the_adversarial_corpus_has_the_special_classes() {
        let v = adversarial_f32(0, 1);
        assert!(v.iter().any(|x| x.is_nan() && x.is_sign_negative()));
        assert!(v.iter().any(|x| x.is_nan() && x.is_sign_positive()));
        assert!(v.iter().any(|x| x.is_infinite()));
        assert!(v.iter().any(|x| *x != 0.0 && !x.is_normal() && x.is_finite()), "subnormals");
        assert!(v.iter().any(|x| x.to_bits() == 0x8000_0000));
    }
}
