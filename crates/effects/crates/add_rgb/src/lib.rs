//! add_rgb (BLW1): native port of slot 53 of class add_rgb (vtable), (800 B incl. 14 B padding,
//! about 205 instructions), plain legacy SSE (movss/addss), no dispatcher, no feature-word test, no VEX, no FMA, no x87, no
//! rip-relative data read (MEASURED, neutral_1.asm). Listing addresses; data VA = offset (unused here, no data reads).
//!
//! The body is the additive twin of scale_rgb slot 53 (crate `scale_rgb`): same `this` layout, same channel-count mismatch
//! tree, same per-channel plane choice, `addss` where scale_rgb has `mulss`. It was ported separately from the listing, not by
//! copying scale_rgb's semantics: every branch below was re-read from the add_rgb listing.
//!
//! Fields of `this` (rdi): n_in, n_out, byte flag_a, byte flag_b, f32 in_p[4],
//! f32 out_p[4], -> struct whose channel pointer k is at+8k (src), -> same (aux).
//! rsi = info (pixel count at), rdx = outs (array of output plane pointers). Vcall [[this]] = channel count nch.
//! Per channel c < nch: kin = min(c, n_in-1), kout = min(c, n_out-1); for i in 0..n:
//!   flag_a: out[c][i] = in_p[kin] + aux[kout][i]
//!   else flag_b: out[c][i] = out_p[kout] + src[kin][i]
//!   else: out[c][i] = src[kin][i] + aux[kout][i]
//! Pad helper (host plane fill/copy for the extra channel) is recorded, not ported (ADD-A1).
//! std only, no unsafe. Every stand-in is listed in `assumptions()`.

#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// a plane pointer slot or a whole plane struct is null / missing and the loop would dereference it
    NullPlane(&'static str),
    /// a plane is shorter than the element count (the original reads/writes out of its buffer)
    Short(&'static str),
    /// a channel count or index outside the contract of this port (see ADD-A2)
    Contract(&'static str),
}

/// The fields of the effect object (`this`, rdi) read by the body.
#[derive(Clone, Debug)]
pub struct Obj {
    pub n_in: i32,        // [this]
    pub n_out: i32,       // [this]
    pub flag_a: bool,     // byte [this]
    pub flag_b: bool,     // byte [this]
    pub in_p: [f32; 4],   // [this]
    pub out_p: [f32; 4],  // [this]
    /// [this] -> struct, channel k data pointer at+8k
    pub src: Option<Vec<Vec<f32>>>,
    /// [this] -> struct, same layout
    pub aux: Option<Vec<Vec<f32>>>,
}

/// One call of the host plane wrapper (this, mode in esi, idx in edx, plane rcx, roi r8).
#[derive(Clone, Debug, PartialEq)]
pub struct PadCall {
    pub mode: u32,
    pub idx: i32,
}

pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ADD-A1", " (host plane wrapper: fills or copies one extra channel plane) is NOT ported: the port records its argument tuple (mode, idx) and leaves that plane untouched; the oracle hooks it with record_args (no write). The executor runs the finished plane_service crate for it (as for scale_rgb)"),
        ("ADD-A2", "the vcall [vtable] (channel count) is an explicit parameter `nch`; n_in, n_out must be 1..=4 and plane arrays hold 4 planes (shipped contract, W205 c); n >= 1 is modelled only under the test feature wide_counts (BLE1 FM2-lens probes); other values are outside the original (it indexes by min(c, n-1) and reads null pointer slots)"),
        ("ADD-A3", "the plane structs [this], [this] are modelled as Option<Vec<plane>>; a missing struct or slot is a null pointer and faults only when a loop with n > 0 dereferences it, like the original"),
        ("ADD-A4", "NaN payloads are compared as a class (x86 addss operand-order payload rules are not modelled); non-NaN results are bit exact"),
        ("ADD-A5", "class identity: is slot 53 of add_rgb by vtable pair (MEASURED, class_vtable_pairs.tsv); the meaning 'add an offset to the colour channels' is INFERRED from the name and the body"),
        ("ADD-A6", "slot 54 of add_rgb (12 B) is the alternate-path thunk, not ported (outside the CPU route); slot 38 is the shared `+` collapse rule (arg 1 zero -> 0 of fx_eval, already wired)"),
        ("ADD-A7", "data reads: none (no rip-relative operand in). No stand-in cell"),
    ]
}

fn idx_min(c: i32, top: i32) -> usize {
    // cmp c, top ; cmovl : min(c, top)
    (if c < top { c } else { top }) as usize
}

fn pl<'a>(s: &'a Option<Vec<Vec<f32>>>, k: usize, what: &'static str) -> Result<&'a Vec<f32>, Fault> {
    match s {
        None => Err(Fault::NullPlane(what)),
        Some(v) => v.get(k).ok_or(Fault::NullPlane(what)),
    }
}

fn at(p: &[f32], i: usize, what: &'static str) -> Result<f32, Fault> {
    p.get(i).copied().ok_or(Fault::Short(what))
}

fn put(p: &mut [f32], i: usize, v: f32, what: &'static str) -> Result<(), Fault> {
    match p.get_mut(i) {
        Some(s) => {
            *s = v;
            Ok(())
        }
        None => Err(Fault::Short(what)),
    }
}

fn contract(o: &Obj, out: &[Vec<f32>]) -> Result<(), Fault> {
    // BLE1 (FM2 lens): the listing tests n_in and n_out only with signed compares against 2 and 4 and indexes by min(c, n-1), so every
    // n >= 1 is well defined as long as the channel loop stays below 4; only n < 1 (index -1) and fewer than 4 output planes are refused.
    // W205 (c) / BLE2-H1 (INT5): the shipped contract is 1..=4; the n >= 1 relaxation (BLE1's FM2-lens probes) is the test-only feature `wide_counts`.
    let wide = cfg!(feature = "wide_counts");
    if (wide && (o.n_in < 1 || o.n_out < 1)) || (!wide && (!(1..=4).contains(&o.n_in) || !(1..=4).contains(&o.n_out))) || out.len() < 4 {
        return Err(Fault::Contract("n_in/n_out outside the contract (1..=4; n >= 1 under feature wide_counts) or fewer than 4 output planes"));
    }
    Ok(())
}

/// Section map :
///  S0 -: prologue, r12d = n_in-1, ebx = n_out-1, vcall nch (edx), eax = n_in, ecx = n_out
///  S1 --: channel-count mismatch tree
///     n_in == 4: call A iff n_out < 4; n_in == 2: call A iff n_out < 2, call B iff n_out == 4;
///     n_in in {1, 3}: call B iff n_out == 4, or n_out == 2 and n_in < 2; A = (esi 0, edx n_in-1, rcx outs[n_in-1]),
///     B = (esi 1, edx n_out-1, rcx outs[n_out-1]); after A nch = n_in-1 (edx = eax-1), after B nch = n_out-1 
///  S3 -: nch <= 0 -> return; n = info, pairs = n/2 (signed); plane pointers by min(c, n-1); n <= 0 -> next channel
///  S4 (flag_a == 0) pair loop and odd tail : flag_b ? out_p[kout] + src : src + aux
///  S5 (flag_a != 0) pair loop and odd tail : in_p[kin] + aux
///  S6 next channel (unsigned compare against nch), epilogue 
/// returns the recorded pad call (if any) and the number of channels written.
pub fn add_rgb(o: &Obj, n: i32, nch: i32, out: &mut [Vec<f32>]) -> Result<(Option<PadCall>, i32), Fault> {
    contract(o, out)?;
    let (ni, no) = (o.n_in, o.n_out);
    // S1 decoded from the cmp/je tree 
    let call = if ni == 4 {
        if no < 4 { Some(0u32) } else { None }
    } else if ni == 2 {
        if no < 2 { Some(0) } else if no == 4 { Some(1) } else { None }
    } else if no == 4 {
        // cmp n_in, 4; jl -> pad call B (signed).
        if ni < 4 { Some(1) } else { None }
    } else if no == 2 && ni < 2 {
        Some(1)
    } else {
        None
    };
    let (rec, nch) = match call {
        Some(0) => (Some(PadCall { mode: 0, idx: ni - 1 }), ni - 1),
        Some(_) => (Some(PadCall { mode: 1, idx: no - 1 }), no - 1),
        None => (None, nch),
    };
    for c in 0..nch.max(0) {
        let cu = c as usize;
        if cu >= out.len() {
            return Err(Fault::Contract("channel index beyond output plane array"));
        }
        let kin = idx_min(c, ni - 1);
        let kout = idx_min(c, no - 1);
        for i in 0..n.max(0) as usize {
            let v = if o.flag_a {
                o.in_p[kin] + at(pl(&o.aux, kout, "aux")?, i, "aux")?
            } else if o.flag_b {
                o.out_p[kout] + at(pl(&o.src, kin, "src")?, i, "src")?
            } else {
                at(pl(&o.src, kin, "src")?, i, "src")? + at(pl(&o.aux, kout, "aux")?, i, "aux")?
            };
            put(&mut out[cu], i, v, "out")?;
        }
    }
    Ok((rec, nch.max(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(ni: i32, no: i32, fa: bool, fb: bool) -> Obj {
        Obj {
            n_in: ni,
            n_out: no,
            flag_a: fa,
            flag_b: fb,
            in_p: [0.5, 0.25, 0.125, 7.0],
            out_p: [10.0, 20.0, 30.0, 40.0],
            src: Some((0..4).map(|k| (0..5).map(|i| (k * 10 + i) as f32).collect()).collect()),
            aux: Some((0..4).map(|k| (0..5).map(|i| 100.0 + (k * 10 + i) as f32).collect()).collect()),
        }
    }
    fn outp() -> Vec<Vec<f32>> {
        vec![vec![f32::from_bits(0x7fc0ffee); 5]; 4]
    }

    #[test]
    fn src_plus_aux_hand_values() {
        let mut out = outp();
        let (rec, k) = add_rgb(&obj(3, 3, false, false), 5, 3, &mut out).unwrap();
        assert_eq!(rec, None);
        assert_eq!(k, 3);
        for c in 0..3 {
            for i in 0..5 {
                assert_eq!(out[c][i], (c * 10 + i) as f32 + 100.0 + (c * 10 + i) as f32);
            }
        }
        assert_eq!(out[3][0].to_bits(), 0x7fc0ffee);
    }

    #[test]
    fn constant_offset_variants() {
        let mut out = outp();
        add_rgb(&obj(3, 3, true, false), 5, 3, &mut out).unwrap();
        assert_eq!(out[1][2], 0.25 + 112.0);
        let mut out = outp();
        add_rgb(&obj(3, 3, false, true), 5, 3, &mut out).unwrap();
        assert_eq!(out[2][4], 30.0 + 24.0);
    }

    #[test]
    fn pad_call_tree() {
        let cases = [(4, 3, Some((0, 3))), (4, 4, None), (2, 1, Some((0, 1))), (2, 4, Some((1, 3))), (2, 2, None), (2, 3, None), (3, 4, Some((1, 3))), (1, 4, Some((1, 3))), (1, 2, Some((1, 1))), (3, 2, None), (1, 1, None), (1, 3, None), (3, 3, None), (4, 1, Some((0, 3))), (4, 2, Some((0, 3)))];
        for (ni, no, want) in cases {
            let mut out = outp();
            let (rec, _) = add_rgb(&obj(ni, no, false, false), 5, 3, &mut out).unwrap();
            assert_eq!(rec.map(|r| (r.mode, r.idx)), want, "ni {} no {}", ni, no);
        }
    }

    #[test]
    fn odd_even_and_degenerate_counts() {
        for n in [-3, 0, 1, 2, 3, 4, 5] {
            let mut out = outp();
            add_rgb(&obj(3, 3, false, false), n, 3, &mut out).unwrap();
            for i in 0..5usize {
                let written = (i as i32) < n;
                assert_eq!(out[0][i].to_bits() != 0x7fc0ffee, written, "n {} i {}", n, i);
            }
        }
    }

    #[test]
    fn nan_and_inf_follow_ieee() {
        let mut o = obj(1, 1, false, true);
        o.out_p = [f32::INFINITY; 4];
        o.src.as_mut().unwrap()[0][0] = f32::NEG_INFINITY;
        let mut out = outp();
        add_rgb(&o, 5, 1, &mut out).unwrap();
        assert!(out[0][0].is_nan());
        assert_eq!(out[0][1], f32::INFINITY);
    }

    #[test]
    fn null_planes_fault_only_when_read() {
        let mut o = obj(3, 3, false, false);
        o.aux = None;
        let mut out = outp();
        assert!(add_rgb(&o, 0, 3, &mut out).is_ok());
        assert_eq!(add_rgb(&o, 5, 3, &mut outp()), Err(Fault::NullPlane("aux")));
        let mut o2 = obj(3, 3, true, false);
        o2.src = None;
        assert!(add_rgb(&o2, 5, 3, &mut outp()).is_ok());
    }

    #[test]
    fn contract_error() {
        assert!(matches!(add_rgb(&obj(0, 3, false, false), 5, 3, &mut outp()), Err(Fault::Contract(_))));
        assert!(matches!(add_rgb(&obj(3, -2, false, false), 5, 3, &mut outp()), Err(Fault::Contract(_))));
    }

    // INT14 (Batch 50 step 10): the shipped-contract refusal holds only without the test feature wide_counts (which models n >= 1)
    #[cfg(not(feature = "wide_counts"))]
    #[test]
    fn counts_five_and_six_are_outside_the_shipped_contract() {
        assert!(matches!(add_rgb(&obj(5, 3, false, false), 5, 3, &mut outp()), Err(Fault::Contract(_))));
    }

    #[cfg(feature = "wide_counts")]
    #[test]
    fn counts_above_four_follow_the_listing_tree() {
        // n_in 5, n_out 4: cmp n_in,4 jl is not taken, so no pad call (M13 would add call B)
        let (rec, _) = add_rgb(&obj(5, 4, false, false), 5, 3, &mut outp()).unwrap();
        assert_eq!(rec, None);
        let (rec, _) = add_rgb(&obj(1, 4, false, false), 5, 3, &mut outp()).unwrap();
        assert_eq!(rec.map(|r| (r.mode, r.idx)), Some((1, 3)));
        let (rec, _) = add_rgb(&obj(6, 2, false, false), 5, 3, &mut outp()).unwrap();
        assert_eq!(rec, None);
    }
}
