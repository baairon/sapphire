//! Native port of the class `one_minus` (vtable), AVX2 body (listing offsets, VA = offset).
//! Scratch only. Source of truth: neutral_1.asm and the captured data image (see SPEC.md). Std only, no unsafe.
//!
//! Body (352 B), entered through slot 53 = (dispatcher: mask 0x9d97ff).
//! Arguments: rdi = this (count i32 at, record pointer at, source plane pointer of channel c at the record (one pointer per channel c)),
//! rsi = info (pixel count n, i32 at), rdx = array of destination plane pointers.
//! For each channel c in 0..count and n > 0: out[c][i] = 1 - src[c][i] (one vsubss/vsubps per element, no FMA, no approximate
//! instruction, no x87, no callee). The scalar head and tail use ONE (captured 1.0f); the 16-wide vector core uses the
//! 32 byte constant (UNCOVERED, registry STD-002, SEMANTIC 1.0f per lane): it is the explicit parameter `vec_one`.
//! The split between head, vector core and tail depends on n and on the destination address phase (address & 0x1f): an explicit input.

/// ONE, scalar head and tail: captured f32 1.0, bytes 0000803f (lookup_om.tsv). ON1-A1.
pub const ONE_SCALAR: f32 = 1.0;
/// The sentinel the harness writes into unwritten output samples.
pub const SENTINEL_BITS: u32 = 0x7fc0_ffee;

#[derive(Clone, Debug)]
pub struct Params {
    /// i32 at this: number of channels (planes) processed; count <= 0 does nothing.
    pub count: i32,
    /// i32 at info: floats per channel; n <= 0 skips the channel.
    pub n: i32,
    /// Value of every lane of the 32 byte vector constant (listing). ON1-A2, STD-002: required, no default, T2 runs 1.0 and 0.75.
    pub vec_one: f32,
    /// Destination pointer address & 0x1f per channel, in bytes (missing entries = 0, a 32 byte aligned buffer). ON1-A5.
    pub dst_phase_bytes: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Fewer source or destination planes than `count`.
    PlaneCount { need: usize, have_src: usize, have_out: usize },
    /// A plane shorter than n (the original would read or write out of bounds: outside the contract).
    ShortPlane { channel: usize, need: usize, have: usize },
}

/// Deliberate bugs for the mutation run. `None` is the real port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutant {
    None,
    /// scalar head/tail compute x - 1 instead of 1 - x
    ScalarSignFlip,
    /// vector core computes x - K instead of K - x
    VectorSignFlip,
    /// peel count one too large
    PeelPlusOne,
    /// the "n < 29" cutoff (cmp rdi,0x1d) becomes n < 28
    Cutoff28,
    /// the "n < 16" cutoff (cmp rdi,0x10) becomes n < 17
    Small17,
    /// vector end computed as n & !15 instead of n - ((n - peel) & 15)
    VecEndMask,
    /// the 4 byte alignment test (test r15d,3) is dropped
    NoQuadTest,
    /// the last tail element is not written
    SkipLast,
    /// the vector core ignores vec_one and uses 1.0
    VecOneIgnored,
    /// one channel too few
    ChannelMinusOne,
    /// channel c reads the source of channel c+1 (a different record offset for c = 0)
    SrcShift,
    /// NaN quieting dropped (a signalling NaN passes through unchanged)
    NoQuiet,
    /// the phase-0 case takes the peel path with a full 8 float peel
    Phase0Peel8,
    /// n <= 0 test becomes n < 0 (n == 0 then enters the loops: equivalent)
    ZeroN,
    /// the peel loop runs peel - 1 elements (vector core starts one element late)
    PeelShort,
}

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "scalar_sign" => ScalarSignFlip,
            "vector_sign" => VectorSignFlip,
            "peel_plus1" => PeelPlusOne,
            "cutoff28" => Cutoff28,
            "small17" => Small17,
            "vec_end_mask" => VecEndMask,
            "no_quad_test" => NoQuadTest,
            "skip_last" => SkipLast,
            "vec_one_ignored" => VecOneIgnored,
            "channel_minus1" => ChannelMinusOne,
            "src_shift" => SrcShift,
            "no_quiet" => NoQuiet,
            "phase0_peel8" => Phase0Peel8,
            "zero_n" => ZeroN,
            "peel_short" => PeelShort,
            _ => return Option::None,
        })
    }
    pub fn all() -> &'static [&'static str] {
        &[
            "scalar_sign", "vector_sign", "peel_plus1", "cutoff28", "small17", "vec_end_mask", "no_quad_test", "skip_last",
            "vec_one_ignored", "channel_minus1", "src_shift", "no_quiet", "phase0_peel8", "zero_n", "peel_short",
        ]
    }
}

/// vsubss / vsubps a - b with the x86 NaN rule (QNaN of the first NaN operand, signalling NaN quieted, both NaN gives the first
/// operand) and the default NaN 0xffc00000 for inf - inf. Round to nearest (ON1-A4, HOST-FP-ASSUMED).
pub fn x86_sub(a: f32, b: f32) -> f32 {
    x86_sub_q(a, b, true)
}

fn x86_sub_q(a: f32, b: f32, quiet: bool) -> f32 {
    let q = |x: f32| if quiet { f32::from_bits(x.to_bits() | 0x0040_0000) } else { x };
    if a.is_nan() {
        return q(a);
    }
    if b.is_nan() {
        return q(b);
    }
    let r = a - b;
    if r.is_nan() {
        return f32::from_bits(0xffc0_0000);
    }
    r
}

/// Where the elements of one channel go: scalar head `0..peel`, vector core `peel..vec_end` (16 per step), scalar tail `vec_end..n`.
/// `None` = all scalar. Mirrors (listing).
pub fn plan(n: usize, phase_bytes: u32, m: Mutant) -> Option<(usize, usize)> {
    let small = if m == Mutant::Small17 { 17 } else { 16 };
    if n < small {
        return None; //  jl 
    }
    let cut = if m == Mutant::Cutoff28 { 28 } else { 29 };
    if n < cut {
        return Some((0, n & !15)); //  jl : no peel, r15 = n & ~0xf
    }
    let ph = (phase_bytes & 0x1f) as usize;
    let mut peel = if ph == 0 {
        if m == Mutant::Phase0Peel8 { 8 } else { 0 }
    } else if ph & 3 != 0 && m != Mutant::NoQuadTest {
        return None; //  test r15d,3; jne 
    } else {
        (32 - ph) >> 2
    };
    if m == Mutant::PeelPlusOne {
        peel += 1;
    }
    if n < peel + 16 {
        return None; //  jl (unreachable for n >= 29, peel <= 8: arithmetic)
    }
    let e = if m == Mutant::VecEndMask { n & !15 } else { n - ((n - peel) & 15) };
    Some((peel, e))
}

fn render_impl(p: &Params, m: Mutant, srcs: &[&[f32]], outs: &mut [&mut [f32]]) -> Result<(), Error> {
    if p.count <= 0 {
        return Ok(()); //  test rcx,rcx; jle 
    }
    let count = p.count as usize;
    let n_le0 = if m == Mutant::ZeroN { p.n < 0 } else { p.n <= 0 };
    if srcs.len() < count || outs.len() < count {
        return Err(Error::PlaneCount { need: count, have_src: srcs.len(), have_out: outs.len() });
    }
    let quiet = m != Mutant::NoQuiet;
    let kv = if m == Mutant::VecOneIgnored { 1.0 } else { p.vec_one };
    let nch = if m == Mutant::ChannelMinusOne { count - 1 } else { count };
    for c in 0..nch {
        if n_le0 {
            continue; //  test rdi,rdi; jle 
        }
        let n = p.n.max(0) as usize;
        let sc = if m == Mutant::SrcShift && c == 0 && srcs.len() > 1 { 1 } else { c };
        let src = srcs[sc];
        if src.len() < n {
            return Err(Error::ShortPlane { channel: c, need: n, have: src.len() });
        }
        if outs[c].len() < n {
            return Err(Error::ShortPlane { channel: c, need: n, have: outs[c].len() });
        }
        let phase = p.dst_phase_bytes.get(c).copied().unwrap_or(0);
        let scalar = |x: f32| {
            if m == Mutant::ScalarSignFlip { x86_sub_q(x, ONE_SCALAR, quiet) } else { x86_sub_q(ONE_SCALAR, x, quiet) }
        };
        let vector = |x: f32| {
            if m == Mutant::VectorSignFlip { x86_sub_q(x, kv, quiet) } else { x86_sub_q(kv, x, quiet) }
        };
        let (peel, vec_end) = match plan(n, phase, m) {
            Some(v) => v,
            None => (0, 0),
        };
        let has_vec = plan(n, phase, m).is_some();
        let head_end = if m == Mutant::PeelShort && peel > 0 { peel - 1 } else { peel };
        let (head, vstart, vend) = if has_vec { (head_end, peel, vec_end) } else { (0, 0, 0) };
        for i in 0..head {
            outs[c][i] = scalar(src[i]);
        }
        let mut i = vstart;
        // do-while over 16 floats (two ymm), at least once when the plan has a vector core
        if has_vec {
            loop {
                for k in 0..16 {
                    outs[c][i + k] = vector(src[i + k]);
                }
                i += 16;
                if i >= vend {
                    break;
                }
            }
        }
        let tail_start = if has_vec { vend } else { 0 };
        let tail_end = if m == Mutant::SkipLast && n > 0 { n - 1 } else { n };
        for j in tail_start..tail_end {
            outs[c][j] = scalar(src[j]);
        }
    }
    Ok(())
}

/// The S3 style render: explicit parameters, source planes in, destination planes out (n floats written per channel).
pub fn render(p: &Params, srcs: &[&[f32]], outs: &mut [&mut [f32]]) -> Result<(), Error> {
    render_impl(p, Mutant::None, srcs, outs)
}

/// Mutation run entry.
pub fn render_mutant(p: &Params, m: Mutant, srcs: &[&[f32]], outs: &mut [&mut [f32]]) -> Result<(), Error> {
    render_impl(p, m, srcs, outs)
}

/// Every stand-in and assumption of this port (prefix -).
pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ON1-A1", "ONE = 1.0f CAPTURED (lookup_om.tsv, 0000803f); scalar head and tail"),
        ("ON1-A2", "vector constant (32 B) is UNCOVERED (lookup_om.tsv U32): registry STD-002 SEMANTIC 1.0f per lane, basis: scalar head and tail of the same function over the same array subtract captured 1.0f, so the vector core must too. ASSUMED-SEMANTIC; explicit parameter vec_one with no default; T2 runs 1.0 and 0.75 on both sides"),
        ("ON1-A3", "route AVX2-ONLY: the slot function tests the feature word (cell CAPTURED, value served by the harness cell = 0x9d97ff) and jumps to; the AVX tier and the SSE tier are not ported"),
        ("ON1-A4", "HOST-FP-ASSUMED: MXCSR round to nearest, no FTZ, no DAZ"),
        ("ON1-A5", "ALIGN-DEPENDENT when vec_one != ONE: head/core/tail split depends on n and on the destination address & 0x1f, an explicit input defaulting to 0; output-neutral when vec_one == 1.0"),
        ("ON1-A6", "the body has no callee and no host service; count (this) is read as given; combos 1..4 of the descriptor builder are INFERRED to be channel counts"),
        ("ON1-A7", "planes shorter than n, and overlapping source and destination, are outside the contract (the original reads and writes out of bounds); the port returns an error for short planes"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(p: &Params, src: &[Vec<f32>], len: usize) -> Vec<Vec<f32>> {
        let mut outs: Vec<Vec<f32>> = (0..src.len()).map(|_| vec![f32::from_bits(SENTINEL_BITS); len]).collect();
        let srcs: Vec<&[f32]> = src.iter().map(|v| v.as_slice()).collect();
        {
            let mut ms: Vec<&mut [f32]> = outs.iter_mut().map(|v| v.as_mut_slice()).collect();
            render(p, &srcs, &mut ms).unwrap();
        }
        outs
    }
    fn params(count: i32, n: i32, kv: f32, ph: u32) -> Params {
        Params { count, n, vec_one: kv, dst_phase_bytes: vec![ph; count.max(0) as usize] }
    }

    #[test]
    fn hand_values_scalar() {
        // n = 3 < 16: all scalar, 1 - x
        let o = run(&params(1, 3, 1.0, 0), &[vec![0.0, 0.25, 1.0]], 4);
        assert_eq!(o[0][0], 1.0);
        assert_eq!(o[0][1], 0.75);
        assert_eq!(o[0][2], 0.0);
        assert_eq!(o[0][3].to_bits(), SENTINEL_BITS); // beyond n untouched
        // -0 and +0, inf, large
        let o = run(&params(1, 5, 1.0, 0), &[vec![-0.0, 0.0, f32::INFINITY, f32::NEG_INFINITY, 1e30]], 5);
        assert_eq!(o[0][0].to_bits(), 1.0f32.to_bits());
        assert_eq!(o[0][1].to_bits(), 1.0f32.to_bits());
        assert_eq!(o[0][2], f32::NEG_INFINITY);
        assert_eq!(o[0][3], f32::INFINITY);
        assert_eq!(o[0][4], -1e30);
        // 1 - 1 = +0 under round to nearest
        let o = run(&params(1, 1, 1.0, 0), &[vec![1.0]], 1);
        assert_eq!(o[0][0].to_bits(), 0);
    }

    #[test]
    fn split_hand_evaluated() {
        // phase 0, n = 40: peel 0, vector core 0..32 (two steps), tail 32..40
        assert_eq!(plan(40, 0, Mutant::None), Some((0, 32)));
        // phase 4 bytes, n = 40: peel = (32-4)>>2 = 7, E = 40 - ((40-7)&15) = 39
        assert_eq!(plan(40, 4, Mutant::None), Some((7, 39)));
        // phase 28: peel 1; n = 29: E = 29 - (28 & 15) = 17
        assert_eq!(plan(29, 28, Mutant::None), Some((1, 17)));
        // odd phases: all scalar
        assert_eq!(plan(100, 1, Mutant::None), None);
        assert_eq!(plan(100, 2, Mutant::None), None);
        assert_eq!(plan(100, 3, Mutant::None), None);
        // n < 16 all scalar; 16..28 one 16-wide step from 0
        assert_eq!(plan(15, 0, Mutant::None), None);
        assert_eq!(plan(16, 8, Mutant::None), Some((0, 16)));
        assert_eq!(plan(28, 4, Mutant::None), Some((0, 16)));
        // the vector core is the only place vec_one shows: n = 40, phase 0, K = 0.75, x = 0
        let o = run(&params(1, 40, 0.75, 0), &[vec![0.0; 40]], 40);
        assert!(o[0][..32].iter().all(|&v| v == 0.75));
        assert!(o[0][32..].iter().all(|&v| v == 1.0));
        // phase 4: head 0..7 scalar, vector 7..39, tail 39
        let o = run(&params(1, 40, 0.75, 4), &[vec![0.0; 40]], 40);
        assert!(o[0][..7].iter().all(|&v| v == 1.0));
        assert!(o[0][7..39].iter().all(|&v| v == 0.75));
        assert_eq!(o[0][39], 1.0);
        // odd phase with K = 0.75: no vector lane at all
        let o = run(&params(1, 40, 0.75, 2), &[vec![0.0; 40]], 40);
        assert!(o[0].iter().all(|&v| v == 1.0));
    }

    #[test]
    fn identity_and_split_neutral_at_one() {
        // 1 - (1 - x) = x for x = 0.25, 0.5 exactly; split output-neutral with K = 1
        let x: Vec<f32> = (0..100).map(|i| (i as f32) * 0.0625).collect();
        let base = run(&params(1, 100, 1.0, 0), &[x.clone()], 100);
        for ph in [0, 4, 8, 12, 16, 20, 24, 28, 1, 2, 3, 5] {
            let o = run(&params(1, 100, 1.0, ph), &[x.clone()], 100);
            assert_eq!(o, base, "phase {}", ph);
        }
        let back = run(&params(1, 100, 1.0, 0), &[base[0].clone()], 100);
        assert_eq!(back[0], x);
    }

    #[test]
    fn nan_inf_and_degenerate() {
        let snan = f32::from_bits(0x7f80_0001);
        let qnan = f32::from_bits(0x7fc0_1234);
        let neg_nan = f32::from_bits(0xffc0_0005);
        let o = run(&params(1, 4, 1.0, 0), &[vec![snan, qnan, neg_nan, f32::NAN]], 4);
        assert_eq!(o[0][0].to_bits(), 0x7fc0_0001);
        assert_eq!(o[0][1].to_bits(), 0x7fc0_1234);
        assert_eq!(o[0][2].to_bits(), 0xffc0_0005);
        assert!(o[0][3].is_nan());
        // inf - inf only through an infinite vec_one: default NaN
        assert_eq!(x86_sub(f32::INFINITY, f32::INFINITY).to_bits(), 0xffc0_0000);
        // denormal: 1 - denormal rounds to 1
        let o = run(&params(1, 2, 1.0, 0), &[vec![1e-40, -1e-40]], 2);
        assert_eq!(o[0][0], 1.0);
        assert_eq!(o[0][1], 1.0);
        // count 0, negative count, n 0, negative n: nothing written
        for (c, n) in [(0, 8), (-3, 8), (1, 0), (1, -5)] {
            let mut out = vec![f32::from_bits(SENTINEL_BITS); 8];
            let src = vec![0.5f32; 8];
            let p = Params { count: c, n, vec_one: 1.0, dst_phase_bytes: vec![] };
            render(&p, &[&src], &mut [&mut out]).unwrap();
            assert!(out.iter().all(|v| v.to_bits() == SENTINEL_BITS));
        }
    }

    #[test]
    fn finite_in_finite_out_and_determinism() {
        let mut s = 12345u32;
        let mut x = Vec::new();
        for _ in 0..257 {
            s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            let v = f32::from_bits(s);
            x.push(if v.is_finite() { v } else { 0.5 });
        }
        for ph in [0u32, 4, 16, 1] {
            let a = run(&params(1, 257, 1.0, ph), &[x.clone()], 257);
            let b = run(&params(1, 257, 1.0, ph), &[x.clone()], 257);
            assert_eq!(a, b);
            assert!(a[0].iter().all(|v| v.is_finite()));
        }
    }

    #[test]
    fn channels_and_errors() {
        let src: Vec<Vec<f32>> = (0..4).map(|c| vec![c as f32 * 0.5; 20]).collect();
        let o = run(&params(4, 20, 1.0, 0), &src, 20);
        for c in 0..4 {
            assert!(o[c].iter().all(|&v| v == 1.0 - c as f32 * 0.5));
        }
        // count 2 of 4 planes: channels 2 and 3 untouched
        let o = run(&params(2, 20, 1.0, 0), &src, 20);
        assert!(o[2].iter().all(|v| v.to_bits() == SENTINEL_BITS));
        let mut out = vec![0.0f32; 4];
        let s = vec![0.0f32; 4];
        let e = render(&params(1, 8, 1.0, 0), &[&s], &mut [&mut out]).unwrap_err();
        assert!(matches!(e, Error::ShortPlane { .. }));
        let e = render(&params(2, 4, 1.0, 0), &[&s], &mut [&mut out]).unwrap_err();
        assert!(matches!(e, Error::PlaneCount { .. }));
    }

    #[test]
    fn assumptions_have_prefix() {
        assert!(assumptions().iter().all(|(id, _)| id.starts_with("ON1-")));
    }
}
