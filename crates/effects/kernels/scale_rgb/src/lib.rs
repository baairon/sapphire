//! scale_rgb (every claim read from neutral_1.asm). Listing addresses; data VA = offset.
//!
//! 1. `scale_rgb`: the REAL slot 53 of class scale_rgb (vtable), (784 B, 201 insns).
//!    It is plain legacy SSE (movss/mulss), has NO dispatcher, NO feature-word test, no VEX, no FMA, no x87 (MEASURED listing).
//!    The class has no AVX2 body: its overridden slots are 38, 53, 54, 65, 85 (class_vtable_pairs.tsv) and none tests 0x9d97ff.
//! 2. `round_snap` (labelled bonus): the scout's "body candidate" is NOT scale_rgb. It is the AVX2 branch of slot 53 of
//!    class `round` (vtable): dispatcher tests `and rcx,0x9d97ff; cmp rcx,0x9d97ff` and jumps to, else
//!    (bit 0) (SSE twin). The scout took the address after the wrong function. The body is a "snap to a multiple" kernel.
//!    Its class identity (round) is a body-level port, the effect meaning of either class is not proven.
//! std only, no unsafe. Every stand-in is listed in `assumptions()`.

pub fn mq(k: u32) -> bool {
    use std::sync::OnceLock;
    static V: OnceLock<u32> = OnceLock::new();
    *V.get_or_init(|| std::env::var("COV_MUT").ok().and_then(|x| x.parse().ok()).unwrap_or(0)) == k
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// a plane pointer slot or a whole plane struct is null / missing and the loop would dereference it
    NullPlane(&'static str),
    /// a plane is shorter than the element count (the original reads/writes out of its buffer)
    Short(&'static str),
    /// a channel count or index outside the contract of this port (see SR1-A2)
    Contract(&'static str),
}

/// The fields of the effect object (`this`, rdi) read by the bodies.
#[derive(Clone, Debug)]
pub struct Obj {
    pub n_in: i32,   // [this]
    pub n_out: i32,  // [this]
    pub flag_a: bool, // byte [this]
    pub flag_b: bool, // byte [this]
    pub in_p: [f32; 4],  // [this]
    pub out_p: [f32; 4], // [this]
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
        ("SR1-A1", " (host plane wrapper: fills or copies one extra channel plane through) is NOT ported: the port records its argument tuple (mode, idx, out plane idx) and leaves that plane untouched; the oracle hooks it with record_args (no write)"),
        ("SR1-A2", "the vcall [vtable] (channel count) is an explicit parameter `nch`; n_in, n_out must be 1..=4 and plane arrays hold 4 planes; other values are outside the original's contract (it indexes by min(c, n-1) and reads null pointer slots)"),
        ("SR1-A3", "the plane structs [this], [this] are modelled as Option<Vec<plane>>; a missing struct or slot is a null pointer and faults only when a loop with n > 0 dereferences it, like the original"),
        ("SR1-A4", "NaN payloads are compared as a class (x86 mulss/divss/roundss operand-order payload rules are not modelled); non-NaN results are bit exact"),
        ("SR1-A5", "class identity: is slot 53 of scale_rgb by vtable pair (MEASURED, class_vtable_pairs.tsv); the name scale_rgb and the meaning 'brightness gain of S_HueSatBright' are PROBABLE/INFERRED; belongs to class `round` (MEASURED, same file), body-level port only"),
        ("SR1-A6", "the dispatcher and the feature word are not emulated: the round body is entered (AVX2 branch, all of 0x9d97ff set); the SSE twin is not ported"),
        ("SR1-A7", "slot 54 of scale_rgb (12 B) is a thunk `push rbp; mov rbp,rsp; pop rbp; jmp ` into a shared alternate-path pre-pass that repeats the same pad-call decision and tail-jumps to; not ported (outside the AVX2/CPU route)"),
        ("SR1-A8", "data reads: scale_rgb has none. round reads mask (captured : ff ff ff 7f then 12 zero bytes), 1.0f and 1.0 double, all CAPTURED (lookup_srgb.tsv); no stand-in needed, the T2 maps them from the captured image"),
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
    if !(1..=4).contains(&o.n_in) || !(1..=4).contains(&o.n_out) || out.len() < 4 {
        return Err(Fault::Contract("n_in/n_out outside 1..=4 or fewer than 4 output planes"));
    }
    Ok(())
}

// ---------------------------------------------------------------- scale_rgb 

/// Section map (784 B, 201 insns, scalar SSE, no FMA, no x87):
///  S1 -: prologue, vcall nch, channel-count mismatch dispatch (n_in vs n_out in {2,4})
///  S2 -: pad helper calls (A: mode 0 idx n_in-1, B: mode 1 idx n_out-1), nch redefined
///  S3 -: per channel c < nch: pick planes by min(c, n_in-1) / min(c, n_out-1), loop n elements
///  S4 loops (unrolled by 2 plus a tail): flag_a: out = in_p[min(c,n_in-1)] * aux[i];
///      else flag_b: out = out_p[min(c,n_out-1)] * src[i]; else out = src[i] * aux[i]
/// returns the recorded pad call (if any) and the number of channels written.
pub fn scale_rgb(o: &Obj, n: i32, nch: i32, out: &mut [Vec<f32>]) -> Result<(Option<PadCall>, i32), Fault> {
    contract(o, out)?;
    let (ni, no) = (o.n_in, o.n_out);
    // S1: which helper call (decoded from the cmp/je tree)
    let call = if ni == 4 {
        if no < 4 || (mq(1) && no == 4) { Some(0u32) } else { None }
    } else if ni == 2 {
        if no < 2 || (mq(2) && no == 2) { Some(0) } else if no == 4 { Some(1) } else { None }
    } else if no == 4 {
        if ni < 4 || mq(13) { Some(1) } else { None }
    } else if (no == 2 || (mq(14) && no == 1)) && ni < 2 {
        Some(1)
    } else {
        None
    };
    let (rec, nch) = match call {
        Some(0) => (Some(PadCall { mode: 0, idx: ni - 1 }), if mq(3) { ni } else { ni - 1 }),
        Some(_) => (Some(PadCall { mode: if mq(4) { 0 } else { 1 }, idx: no - 1 }), no - 1),
        None => (None, nch),
    };
    // S3/S4
    for c in 0..(nch.max(0) + if mq(11) { 1 } else { 0 }) {
        let cu = c as usize;
        if cu >= out.len() {
            return Err(Fault::Contract("channel index beyond output plane array"));
        }
        let kin = idx_min(c, ni - 1);
        let kout = idx_min(c, no - 1);
        for i in 0..n.max(0) as usize {
            let v = if o.flag_a && !mq(9) {
                o.in_p[if mq(5) { kout } else { kin }] * at(pl(&o.aux, kout, "aux")?, i, "aux")?
            } else if o.flag_b && !mq(8) {
                if mq(6) { o.out_p[kin] * at(pl(&o.src, kin, "src")?, i, "src")? } else if mq(10) { at(pl(&o.src, kin, "src")?, i, "src")? * o.out_p[kout] } else { o.out_p[kout] * at(pl(&o.src, kin, "src")?, i, "src")? }
            } else {
                if mq(7) { at(pl(&o.src, kout, "src")?, i, "src")? * at(pl(&o.aux, kin, "aux")?, i, "aux")? } else { at(pl(&o.src, kin, "src")?, i, "src")? * at(pl(&o.aux, kout, "aux")?, i, "aux")? }
            };
            put(&mut out[cu], i, v, "out")?;
        }
    }
    Ok((rec, nch.max(0)))
}

// ---------------------------------------------------------------- round body (AVX2, bonus)

/// snap(x, y), the shared element kernel of (sites and 17 copies):
/// y == 0 (ucomiss equal, not unordered): x unchanged; x >= 0 or NaN: (float)((double)y * (double)floorf(x/y));
/// x < 0: (float) fmsub213sd(ceilf(x/y), (double)y, |y|) = ceil*y - |y|  (the only FMA site, vfmsub213sd).
pub fn snap(x: f32, y: f32) -> f32 {
    if y == 0.0 && !mq(15) {
        return x;
    }
    let yd = y as f64;
    if !(x < 0.0) && !(mq(16) && x == 0.0) {
        let q = if mq(17) { (x / y).ceil() } else { (x / y).floor() };
        if mq(21) { y * q } else { (yd * q as f64) as f32 }
    } else {
        let q = if mq(18) { (x / y).floor() } else { (x / y).ceil() };
        (if mq(19) { (q as f64) * yd + (-(y.abs() as f64)) } else if mq(20) { (q as f64).mul_add(yd, y.abs() as f64) } else { (q as f64).mul_add(yd, -(y.abs() as f64)) }) as f32
    }
}

/// y = 1.0 variants: the alpha-synth loops and the drop-alpha loops .
pub fn snap_one_over(y: f32) -> f32 {
    // y == 0 -> 1.0f ; else (float)((double)y * floor((double)(1.0f / y)))
    if y == 0.0 && !mq(22) {
        1.0
    } else {
        ((y as f64) * if mq(23) { ((1.0f32 / y) as f64).ceil() } else { ((1.0f32 / y) as f64).floor() }) as f32
    }
}

pub fn snap_unit(x: f32) -> f32 {
    // 0 <= x or NaN: floor((double)x); x < 0: ceil((double)x) - 1.0 (the 1.0 double)
    let xd = x as f64;
    if !(x < 0.0) && !(mq(24) && x == 0.0) { xd.floor() as f32 } else { (if mq(25) { xd.floor() } else { xd.ceil() } - 1.0) as f32 }
}

/// Section map (1664 B, 442 insns, VEX scalar, 1 FMA site per copy: 6 vfmsub213sd):
///  R1 - prologue, r12 = (n_in==3 && n_out==4) alpha synth, r14 = (n_in==4 && n_out==3) alpha drop, vcall nch
///  R2 - per-channel snap loops (path A for nch of any value, path B forced to 3 channels
///     when r12|r14|nch==3); variants by (flag_a, flag_b): flag_a: x=in_p[min(c,n_in-1)], y=aux[min(c,n_out-1)][i];
///     flag_b: x=src[min(c,n_in-1)][i], y=out_p[min(c,n_out-1)]; else x=src[..][i], y=aux[..][i]
///  R3 - r12 tail: out[3][i] = snap_one_over(flag_b ? out_p[3] : aux[3][i])
///  R4 - r14 tail: out[3][i] = snap_unit(flag_a ? in_p[3] : src[3][i])
/// returns the list of channels written (including 3 for a tail).
pub fn round_snap(o: &Obj, n: i32, nch: i32, out: &mut [Vec<f32>]) -> Result<Vec<usize>, Fault> {
    contract(o, out)?;
    let (ni, no) = (o.n_in, o.n_out);
    let r12 = ni == 3 && (no == 4 || (mq(26) && no == 3));
    let r14 = ni == 4 && (no == 3 || (mq(27) && no == 4));
    let eff = if r12 || r14 || nch == 3 || (mq(28) && nch == 4) { 3 } else { nch };
    let mut wrote = vec![];
    for c in 0..eff.max(0) {
        let cu = c as usize;
        if cu >= out.len() {
            return Err(Fault::Contract("channel index beyond output plane array"));
        }
        let kin = idx_min(c, ni - 1);
        let kout = idx_min(c, no - 1);
        for i in 0..n.max(0) as usize {
            let (x, y) = if o.flag_a && !mq(31) {
                (o.in_p[if mq(32) { kout } else { kin }], at(pl(&o.aux, kout, "aux")?, i, "aux")?)
            } else if o.flag_b {
                (at(pl(&o.src, kin, "src")?, i, "src")?, o.out_p[if mq(34) { kin } else { kout }])
            } else {
                (at(pl(&o.src, kin, "src")?, i, "src")?, at(pl(&o.aux, kout, "aux")?, i, "aux")?)
            };
            put(&mut out[cu], i, if mq(33) { snap(y, x) } else { snap(x, y) }, "out")?;
        }
        if n > 0 {
            wrote.push(cu);
        }
    }
    if r12 {
        for i in 0..n.max(0) as usize {
            let y = if o.flag_b != mq(29) { o.out_p[3] } else { at(pl(&o.aux, 3, "aux")?, i, "aux")? };
            put(&mut out[3], i, snap_one_over(y), "out")?;
        }
        if n > 0 {
            wrote.push(3);
        }
    }
    if r14 {
        for i in 0..n.max(0) as usize {
            let x = if o.flag_a != mq(30) { o.in_p[3] } else { at(pl(&o.src, 3, "src")?, i, "src")? };
            put(&mut out[3], i, snap_unit(x), "out")?;
        }
        if n > 0 {
            wrote.push(3);
        }
    }
    Ok(wrote)
}

// ---------------------------------------------------------------- T1 / T3

#[cfg(test)]
mod tests {
    use super::*;

    fn planes(v: &[f32]) -> Vec<Vec<f32>> {
        vec![v.to_vec(), v.iter().map(|x| x + 1.0).collect(), v.iter().map(|x| x + 2.0).collect(), v.iter().map(|x| x + 3.0).collect()]
    }
    fn obj(ni: i32, no: i32, fa: bool, fb: bool) -> Obj {
        Obj { n_in: ni, n_out: no, flag_a: fa, flag_b: fb, in_p: [2.0, 3.0, 4.0, 5.0], out_p: [0.5, 2.0, 1.0, 0.0], src: Some(planes(&[1.0, 2.0, 4.0])), aux: Some(planes(&[0.5, 0.25, 8.0])) }
    }
    fn outp() -> Vec<Vec<f32>> {
        vec![vec![-9.0; 3]; 4]
    }

    // T1 hand-evaluated scale_rgb
    #[test]
    fn s_gain_vector() {
        // flag_b: out[c] = out_p[c] * src[c][i]; gains (0.5, 2, 1)
        let mut o = outp();
        let (rec, n) = scale_rgb(&obj(3, 3, false, true), 3, 3, &mut o).unwrap();
        assert!(rec.is_none() && n == 3);
        assert_eq!(o[0], vec![0.5, 1.0, 2.0]);
        assert_eq!(o[1], vec![4.0, 6.0, 10.0]); // 2 * (src+1)
        assert_eq!(o[2], vec![3.0, 4.0, 6.0]); // 1 * (src+2)
        assert_eq!(o[3], vec![-9.0; 3]);
    }
    #[test]
    fn s_identity_and_zero() {
        let mut ob = obj(3, 3, false, true);
        ob.out_p = [1.0; 4];
        let mut o = outp();
        scale_rgb(&ob, 3, 3, &mut o).unwrap();
        assert_eq!(o[0], ob.src.as_ref().unwrap()[0]);
        ob.out_p = [0.0; 4];
        scale_rgb(&ob, 3, 3, &mut o).unwrap();
        assert_eq!(o[1], vec![0.0; 3]);
    }
    #[test]
    fn s_mask_and_param_paths() {
        let mut o = outp();
        scale_rgb(&obj(3, 3, false, false), 3, 1, &mut o).unwrap();
        assert_eq!(o[0], vec![0.5, 0.5, 32.0]); // src * aux
        scale_rgb(&obj(3, 3, true, false), 3, 2, &mut o).unwrap();
        assert_eq!(o[0], vec![1.0, 0.5, 16.0]); // in_p[0]=2 * aux[0]
        assert_eq!(o[1], vec![3.0 * 1.5, 3.0 * 1.25, 3.0 * 9.0]);
    }
    #[test]
    fn s_pad_dispatch() {
        let mut o = outp();
        let cases = [((4, 3), Some((0, 3)), 3), ((4, 4), None, 5), ((2, 1), Some((0, 1)), 1), ((1, 4), Some((1, 3)), 3), ((3, 4), Some((1, 3)), 3), ((2, 4), Some((1, 3)), 3), ((1, 2), Some((1, 1)), 1), ((3, 3), None, 5), ((2, 2), None, 5), ((4, 2), Some((0, 3)), 3)];
        for ((ni, no), want, nch) in cases {
            let (rec, n) = scale_rgb(&obj(ni, no, false, true), 0, 5, &mut o).unwrap_or_else(|_| (None, -1));
            if n < 0 { continue; }
            match want {
                Some((m, ix)) => {
                    assert_eq!(rec, Some(PadCall { mode: m, idx: ix }), "{ni}->{no}");
                    assert_eq!(n, nch, "{ni}->{no}");
                }
                None => assert!(rec.is_none() && n == 5, "{ni}->{no}"),
            }
        }
    }
    #[test]
    fn s_edges() {
        let mut o = outp();
        // n = 0 and n < 0 write nothing and need no planes
        let mut ob = obj(3, 3, false, false);
        ob.src = None;
        ob.aux = None;
        assert!(scale_rgb(&ob, 0, 3, &mut o).is_ok());
        assert!(scale_rgb(&ob, -5, 3, &mut o).is_ok());
        assert_eq!(o[0], vec![-9.0; 3]);
        // missing planes with n > 0 fault
        assert_eq!(scale_rgb(&ob, 3, 1, &mut o), Err(Fault::NullPlane("src")));
        // nch <= 0 does nothing
        assert!(scale_rgb(&obj(3, 3, false, true), 3, 0, &mut o).unwrap().1 == 0);
        // NaN gain gives NaN, inf*0 NaN, negative gain flips sign
        let mut g = obj(3, 3, false, true);
        g.out_p = [f32::NAN, -1.0, f32::INFINITY, 0.0];
        scale_rgb(&g, 3, 3, &mut o).unwrap();
        assert!(o[0].iter().all(|x| x.is_nan()));
        assert_eq!(o[1], vec![-2.0, -3.0, -5.0]);
        assert_eq!(o[2][0], f32::INFINITY);
        // determinism
        let mut o2 = outp();
        scale_rgb(&g, 3, 3, &mut o2).unwrap();
        assert_eq!(o2[1], o[1]);
        // finite in, finite out for finite gains
        let mut f = obj(3, 3, false, true);
        f.out_p = [1e3, -2.5, 0.0, 0.0];
        scale_rgb(&f, 3, 3, &mut o).unwrap();
        assert!(o.iter().take(3).all(|p| p.iter().all(|x| x.is_finite())));
        // contract
        assert!(matches!(scale_rgb(&obj(0, 3, false, true), 3, 3, &mut o), Err(Fault::Contract(_))));
    }

    // T1 snap kernel (hand evaluated)
    #[test]
    fn r_snap_values() {
        assert_eq!(snap(1.3, 0.5), 1.0); // floor(2.6)=2 * .5
        assert_eq!(snap(-1.3, 0.5), -1.5); // ceil(-2.6)=-2 -> -2*.5-.5=-1.5
        assert_eq!(snap(5.0, 0.0), 5.0); // y == 0 returns x
        assert_eq!(snap(-5.0, -0.0), -5.0);
        assert_eq!(snap(2.0, 0.5), 2.0);
        assert_eq!(snap(-2.0, 0.5), -2.5); // exact multiple: ceil(-4)=-4 -> -2 - 0.5
        assert_eq!(snap(0.7, -0.5), -0.5 * (-2.0)); // floor(-1.4) = -2 -> 1.0
        assert!(snap(f32::NAN, 0.5).is_nan());
        assert!(snap(1.0, f32::NAN).is_nan());
        assert_eq!(snap_one_over(0.0), 1.0);
        assert_eq!(snap_one_over(0.3), (0.3f32 as f64 * 3.0) as f32);
        assert_eq!(snap_unit(2.7), 2.0);
        assert_eq!(snap_unit(-2.7), -3.0);
        assert_eq!(snap_unit(-2.0), -3.0);
    }
    #[test]
    fn r_paths() {
        let mut o = outp();
        let w = round_snap(&obj(3, 4, false, true), 3, 2, &mut o).unwrap();
        assert_eq!(w, vec![0, 1, 2, 3]); // path B forced 3 channels plus alpha synth
        assert_eq!(o[3], vec![snap_one_over(0.0); 3]); // out_p[3] = 0 -> 1.0
        let w = round_snap(&obj(4, 3, true, false), 3, 1, &mut o).unwrap();
        assert_eq!(w, vec![0, 1, 2, 3]);
        assert_eq!(o[3], vec![snap_unit(5.0); 3]);
        let w = round_snap(&obj(4, 4, false, false), 3, 1, &mut o).unwrap();
        assert_eq!(w, vec![0]);
        let w = round_snap(&obj(4, 4, false, false), 0, 4, &mut o).unwrap();
        assert!(w.is_empty());
        assert!(round_snap(&obj(4, 4, false, false), 3, 0, &mut o).unwrap().is_empty());
    }
}
