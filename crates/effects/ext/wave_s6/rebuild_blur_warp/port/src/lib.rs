//! blur_warp (class vtable, PG row "own class, NOT blur_warp_bidirectional"): native port of the CPU render entry,
//! slot 52 = (768 B, EG table: the class overrides the dispatcher slot 52, so slot 52 is the entry for flag 0;
//! slots 53/54 stay the base stubs). The slot is glue: it reads the class object, fills five host statics
//! (the same cells bwb_bodya's `Host` models) and tail-calls one of the four feature dispatchers
//!  selected by the channel count n = [[this]] (1..4). Those dispatchers and the
//! bodies A..D are the SAME functions the class blur_warp_bidirectional calls
//! (MEASURED: identical call targets in). The bodies are reused by READ-ONLY path dependency from
//! wave_s1\rebuild_bwb (no private copies). Source of truth: neutral_1.asm and the captured data image. Names INFERRED, offsets MEASURED.
//! The emulator follows the SDM, not silicon.

use bwb_bodya::p2::{Host, P1Cfg, Plane};
use bwb_bodya::{Inputs, Params};

/// Captured constant loaded into xmm3 (the fourth float argument) (
/// lookup_bw.tsv: 0000803f = 1.0f, CAPTURED).
pub const XMM3_CONST: f32 = 1.0;

/// Fields of the class object `this` (rdi) read by slot 52. Offsets MEASURED in the listing.
#[derive(Clone, Debug)]
pub struct ThisObj {
    /// this (byte): nonzero = the first segment endpoint is one constant pair (this, this), else two arrays at [[this]]
    pub p1_const: u8,
    /// array (or the single inline value at index 0 when `p1_const != 0`): this / [[this]]
    pub p1x: Vec<f32>,
    /// this / [[this]]
    pub p1y: Vec<f32>,
    /// this (byte): same for the second endpoint, inline at this / this, arrays at [[this]]
    pub p2_const: u8,
    pub p2x: Vec<f32>,
    pub p2y: Vec<f32>,
    /// this (xmm0)
    pub f_cc: f32,
    /// this (xmm1 and, copied, xmm2)
    pub assumption_a: f32,
    /// this (xmm4, the output scale a4 of the bodies)
    pub assumption_b: f32,
    /// this (edge mode x, 0 none, 1 wrap, 2 mirror)
    pub mode_x: i32,
    /// this (edge mode y)
    pub mode_y: i32,
    /// this (the body's flag_cell_a stack argument)
    pub flag_cell_a: i32,
}

/// The image object [this] (called "params block" in bwb notes).
#[derive(Clone, Debug)]
pub struct ImageObj {
    ///
    pub width: i32,
    ///
    pub height: i32,
    /// channel count n; selects the body (1 = A, 2 = B, 3 = C, 4 = D)
    pub n: i32,
    /// ripmap level 0 width [[img] field], used by (stride = (w0 + 3) * n) and by P1
    pub w0: i32,
    /// ripmap level 0 height [[img] field]
    pub h0: i32,
    /// plane (0,0) of the ripmap: the buffer [[[img] field] field]; data pointer = buffer + 4 * (n + n * (w0 + 3)) bytes
    pub plane00: std::sync::Arc<Vec<f32>>,
}

/// The record `rsi` (the body's "this"): count at and the S0 fields. All explicit.
#[derive(Clone, Copy, Debug)]
pub struct Rec {
    pub count: i32,
    pub aspect_block_d: f32,
    pub assumption_e: f32,
    pub assumption_c: i32,
    pub assumption_d: i32,
    pub assumption_f: f32,
    pub assumption_g: f32,
    pub assumption_h: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// a body ran (n = 1..4)
    Body { n: i32 },
    /// width * height == 0: slot 52 tail-calls (out object, rec, edx = n, ecx = 0) (the pad fill of every plane): NOT REBUILT, call record only
    EmptyImageTail { n: i32 },
    /// n outside 1..4: the slot returns without calling anything
    NoBody { n: i32 },
}

#[derive(Debug, PartialEq)]
pub enum RenderError {
    /// cl != 0 (render flag): the GPU/other path is NOT REBUILT
    FlagNotZero,
    /// the body reported a state it does not model (Debug text of its Unsupported)
    Body(String),
}

/// Rust-side mutants for the mutation test (T2 `run_r.py mut`). `None` = the port.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mut {
    ConstOneToZero,
    Xmm2FromCc,
    ScaleFromD0,
    ModesSwapped,
    FlagCellAZero,
    HostFCellAromModeY,
    HostFCellANotEq1,
    HsWsSwapped,
    P1ConstInverted,
    P2ConstIgnored,
    BodyBCSwapped,
    EmptyTestWidthOnly,
    P1XYSwapped,
    CountPlusOne,
    P2XYSwapped,
    ModeOneIsB,
}

pub const ALL_MUTS: [Mut; 16] = [
    Mut::ConstOneToZero, Mut::Xmm2FromCc, Mut::ScaleFromD0, Mut::ModesSwapped, Mut::FlagCellAZero, Mut::HostFCellAromModeY, Mut::HostFCellANotEq1,
    Mut::HsWsSwapped, Mut::P1ConstInverted, Mut::P2ConstIgnored, Mut::BodyBCSwapped, Mut::EmptyTestWidthOnly, Mut::P1XYSwapped,
    Mut::CountPlusOne, Mut::P2XYSwapped, Mut::ModeOneIsB,
];

/// The host statics the slot writes before the dispatcher call (29, 2a, 2b, 2c, 30), as the bodies read them.
pub fn host_statics(mode_x: i32, mode_y: i32, width: i32, height: i32) -> Host {
    Host {
        port_blk_a: (mode_x == 2) as u8,
        cassumption_a: (mode_y == 2) as u8,
        cassumption_b: if mode_x == 1 { 1 } else { 0 },
        cassumption_c: if mode_y == 1 { 1 } else { 0 },
        hs: height as f32,
        ws: width as f32,
    }
}

thread_local! {
    static LAST_HOST: std::cell::Cell<Option<Host>> = std::cell::Cell::new(None);
}

/// The host statics the last non-empty `render_m` call wrote (None for flag != 0 and for empty images), for the T2 statics compare.
pub fn last_host() -> Option<Host> {
    LAST_HOST.with(|c| c.take())
}

pub fn render(this: &ThisObj, rec: &Rec, img: &ImageObj, flag: u8, out: &mut Vec<Vec<f32>>) -> Result<Outcome, RenderError> {
    render_m(this, rec, img, flag, out, None)
}

/// Slot 52 with flag 0. `out` = the output object's planes (plane k receives body output k).
pub fn render_m(this: &ThisObj, rec: &Rec, img: &ImageObj, flag: u8, out: &mut Vec<Vec<f32>>, m: Option<Mut>) -> Result<Outcome, RenderError> {
    if flag != 0 {
        return Err(RenderError::FlagNotZero);
    }
    let is = |x: Mut| m == Some(x);
    // edi = w * h (imul, 32 bit wrap); zero -> tail 
    let empty = if is(Mut::EmptyTestWidthOnly) { img.width == 0 } else { img.width.wrapping_mul(img.height) == 0 };
    if empty {
        return Ok(Outcome::EmptyImageTail { n: img.n });
    }
    let (mut mx, mut my) = (this.mode_x, this.mode_y);
    if is(Mut::ModesSwapped) {
        std::mem::swap(&mut mx, &mut my);
    }
    let mut host = host_statics(mx, my, img.width, img.height);
    if is(Mut::HostFCellAromModeY) {
        host.port_blk_a = (my == 2) as u8;
    }
    if is(Mut::HostFCellANotEq1) {
        host.cassumption_b = (mx != 0) as u8;
    }
    if is(Mut::HsWsSwapped) {
        std::mem::swap(&mut host.hs, &mut host.ws);
    }
    LAST_HOST.with(|c| c.set(Some(host)));
    let mut body_n = img.n;
    if is(Mut::BodyBCSwapped) {
        body_n = match body_n {
            2 => 3,
            3 => 2,
            x => x,
        };
    }
    if is(Mut::ModeOneIsB) && body_n == 1 {
        body_n = 2;
    }
    if !(1..=4).contains(&img.n) {
        return Ok(Outcome::NoBody { n: img.n });
    }
    // arguments of the dispatcher call (etc.): xmm0 =, xmm1 =, xmm2 = xmm1, xmm3 = captured 1.0f, xmm4 = 
    let xmm3 = if is(Mut::ConstOneToZero) { 0.0 } else { XMM3_CONST };
    let xmm2 = if is(Mut::Xmm2FromCc) { this.f_cc } else { this.assumption_a };
    let xmm4 = if is(Mut::ScaleFromD0) { this.assumption_a } else { this.assumption_b };
    let (mut p1x, mut p1y) = (this.p1x.clone(), this.p1y.clone());
    if is(Mut::P1XYSwapped) {
        std::mem::swap(&mut p1x, &mut p1y);
    }
    let (mut p2x, mut p2y) = (this.p2x.clone(), this.p2y.clone());
    if is(Mut::P2XYSwapped) {
        std::mem::swap(&mut p2x, &mut p2y);
    }
    // r9d / r15d: the byte flags, normalised to 0/1 (cmovne with 1)
    let mut p1c = this.p1_const != 0;
    if is(Mut::P1ConstInverted) {
        p1c = !p1c;
    }
    let p2c = if is(Mut::P2ConstIgnored) { false } else { this.p2_const != 0 };
    let stride = img.n.wrapping_mul(img.w0.wrapping_add(3));
    let inp = Inputs {
        a: [this.f_cc, this.assumption_a, xmm2, xmm3, xmm4],
        p1x,
        p1y,
        p2x,
        p2y,
        p1_const: p1c,
        p2_const: p2c,
        flag_cell_a: if is(Mut::FlagCellAZero) { 0 } else { this.flag_cell_a as u32 },
        mode_x: mx,
        mode_y: my,
        acc_ptr: 0,
        acc_stride: stride,
    };
    let count = if is(Mut::CountPlusOne) { rec.count + 1 } else { rec.count };
    let p = Params { count, aspect_block_d: rec.aspect_block_d, assumption_e: rec.assumption_e, assumption_c: rec.assumption_c, assumption_d: rec.assumption_d, assumption_f: rec.assumption_f, assumption_g: rec.assumption_g, assumption_h: rec.assumption_h, img_w: img.width, img_h: img.height };
    //  (ripmap level 0, plane (0,0)): data pointer = plane + 4 * (n + n * (w0 + 3)) bytes
    let pl = Plane { buf: img.plane00.clone(), origin: img.n as i64 + (img.n as i64) * (img.w0 as i64 + 3) };
    let cfg = P1Cfg { n: img.n, w0: img.w0, h0: img.h0 };
    let r = match body_n {
        1 => bwb_bodya::p2::run_planes_full(&p, &inp, &host, &pl, Some(&cfg), out).map_err(|e| format!("{:?}", e)),
        2 => bwb_bodyb::run_planes_b(&p, &inp, &host, &pl, Some(&cfg), out).map_err(|e| format!("{:?}", e)),
        3 => bwb_bodyc::run_planes_full(&p, &inp, &host, &pl, Some(&cfg), out).map_err(|e| format!("{:?}", e)),
        _ => bwb_bodyd::run_planes_full(&p, &inp, &host, &pl, Some(&cfg), out).map_err(|e| format!("{:?}", e)),
    };
    match r {
        Ok(()) => Ok(Outcome::Body { n: img.n }),
        Err(s) => Err(RenderError::Body(s)),
    }
}

pub struct Assumption {
    pub id: &'static str,
    pub tier: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "BW1-A1", tier: "CAPTURED", basis: "xmm3 constant 1.0f, lookup_bw.tsv N4 0000803f; also STD-002's scalar twin cell" },
        Assumption { id: "BW1-A2", tier: "ROUTE", basis: "feature word 0x9d97ff served through the pointer cell (STD-032): dispatchers etc. jump to the AVX2 body only on a full mask match " },
        Assumption { id: "BW1-A3", tier: "INFERRED", basis: "the bodies A..D are the bwb bodies: same call targets as the bwb render; their parity (K-PARITY-THIN for A..D floor per body in their READMEs) is inherited, not re-proved; the argument mapping and host statics are the new glue and are proved by run_r.py" },
        Assumption { id: "BW1-A4", tier: "INFERRED", basis: "channel count n = [[this]] picks the body (cmp 1/2/3/4) and is the same field P1 reads as n; matches the COMBOS.tsv rows (arg 0 values 1,2,3,4)" },
        Assumption { id: "BW1-A5", tier: "HOST-HOOKED", basis: "fmod returns Python math.fmod (bwb_libm is bit equal on finite inputs per its README), records its call (writes 16 captured bytes to a stack scratch the bodies never read); the image accessors and P1 run as REAL emulated code in the R-PARITY run" },
        Assumption { id: "BW1-A6", tier: "NOT REBUILT", basis: "empty image tail (pad fill of each plane through, callers PL): call record only, label PAD-UNVERIFIED for w*h == 0" },
        Assumption { id: "BW1-A7", tier: "PARAM-EXPLICIT", basis: "ripmap level 0 geometry (w0, h0), image n and the plane buffer are explicit inputs (no default); whether the host's ripmap built by make_mipmap has w0 = image width is not measured (the evaluator must pass the make_mipmap output); the real call writes planes only through the bodies" },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base(n: i32, w: i32, h: i32) -> (ThisObj, Rec, ImageObj) {
        let stride = n * (w + 3);
        let plane: Vec<f32> = (0..(stride * (h + 6)) as usize).map(|i| (i % 7) as f32 * 0.5).collect();
        (
            ThisObj { p1_const: 1, p1x: vec![0.5], p1y: vec![1.5], p2_const: 1, p2x: vec![0.7], p2y: vec![1.2], f_cc: 0.5, assumption_a: 1.0, assumption_b: 1.0, mode_x: 0, mode_y: 0, flag_cell_a: 0 },
            Rec { count: 1, aspect_block_d: h as f32 / w as f32, assumption_e: h as f32, assumption_c: 0, assumption_d: 0, assumption_f: 0.0, assumption_g: w as f32, assumption_h: 0.0 },
            ImageObj { width: w, height: h, n, w0: w, h0: h, plane00: std::sync::Arc::new(plane) },
        )
    }

    #[test]
    fn host_statics_hand() {
        let h = host_statics(2, 1, 8, 5);
        assert_eq!((h.port_blk_a, h.cassumption_a, h.cassumption_b, h.cassumption_c, h.ws, h.hs), (1, 0, 0, 1, 8.0, 5.0));
        let h = host_statics(1, 2, 3, 4);
        assert_eq!((h.port_blk_a, h.cassumption_a, h.cassumption_b, h.cassumption_c), (0, 1, 1, 0));
        let h = host_statics(0, 0, 3, 4);
        assert_eq!((h.port_blk_a, h.cassumption_a, h.cassumption_b, h.cassumption_c), (0, 0, 0, 0));
    }

    #[test]
    fn flag_and_empty_and_nobody() {
        let (t, r, i) = base(1, 8, 8);
        let mut out = vec![vec![0.0; 1]; 4];
        assert_eq!(render(&t, &r, &i, 1, &mut out), Err(RenderError::FlagNotZero));
        let mut i0 = i.clone();
        i0.height = 0;
        assert_eq!(render(&t, &r, &i0, 0, &mut out), Ok(Outcome::EmptyImageTail { n: 1 }));
        let mut i5 = i.clone();
        i5.n = 5;
        assert_eq!(render(&t, &r, &i5, 0, &mut out), Ok(Outcome::NoBody { n: 5 }));
        i5.n = 0;
        assert_eq!(render(&t, &r, &i5, 0, &mut out), Ok(Outcome::NoBody { n: 0 }));
        // wrap-around product: 65536 * 65536 == 0 in 32 bits (imul edi, r12d)
        let mut iw = i.clone();
        iw.width = 65536;
        iw.height = 65536;
        assert_eq!(render(&t, &r, &iw, 0, &mut out), Ok(Outcome::EmptyImageTail { n: 1 }));
    }

    #[test]
    fn deterministic_and_finite_all_n() {
        for n in 1..=4 {
            let (t, r, i) = base(n, 9, 7);
            let mut o1 = vec![vec![0.0f32; 1]; 4];
            let mut o2 = o1.clone();
            let a = render(&t, &r, &i, 0, &mut o1);
            let b = render(&t, &r, &i, 0, &mut o2);
            assert_eq!(a, b);
            assert_eq!(o1, o2);
            if a.is_ok() {
                for k in 0..n as usize {
                    assert!(o1[k][0].is_finite(), "n={} plane {}", n, k);
                }
            }
        }
    }

    #[test]
    fn nan_inf_parameters_do_not_panic_and_are_deterministic() {
        for v in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.0, -0.0, 1e-40, 1e30] {
            for n in 1..=4 {
                let (mut t, r, i) = base(n, 9, 7);
                t.f_cc = v;
                t.assumption_a = v;
                t.assumption_b = v;
                t.p1x = vec![v];
                t.p2y = vec![v];
                let mut o1 = vec![vec![0.0f32; 1]; 4];
                let mut o2 = o1.clone();
                let a = render(&t, &r, &i, 0, &mut o1);
                let b = render(&t, &r, &i, 0, &mut o2);
                assert_eq!(a, b);
                for k in 0..4 {
                    assert_eq!(o1[k][0].to_bits() == o2[k][0].to_bits() || o1[k][0].is_nan(), true);
                }
            }
        }
    }

    #[test]
    fn identity_of_scale_one_flag_paths_host_cells_only_by_mode() {
        // the host statics depend only on modes and size, not on the data fields
        let (mut t, r, i) = base(1, 8, 5);
        t.mode_x = 2;
        t.mode_y = 1;
        let mut o = vec![vec![0.0f32; 1]; 4];
        let _ = render(&t, &r, &i, 0, &mut o);
        let h = last_host().unwrap();
        assert_eq!((h.port_blk_a, h.cassumption_a, h.cassumption_b, h.cassumption_c, h.ws, h.hs), (1, 0, 0, 1, 8.0, 5.0));
        assert!(last_host().is_none());
    }

    #[test]
    fn count_zero_and_negative_write_nothing() {
        let (t, mut r, i) = base(2, 8, 8);
        let mut out = vec![vec![7.0f32; 2]; 4];
        r.count = 0;
        assert_eq!(render(&t, &r, &i, 0, &mut out), Ok(Outcome::Body { n: 2 }));
        r.count = -3;
        assert_eq!(render(&t, &r, &i, 0, &mut out), Ok(Outcome::Body { n: 2 }));
        assert!(out.iter().all(|p| p.iter().all(|&v| v == 7.0)));
    }

    #[test]
    fn mutants_change_something() {
        // every mutant must change the argument mapping: at least one of the probe cases differs from the port
        let mut changed = 0;
        for m in ALL_MUTS {
            let mut diff = false;
            for n in 1..=4 {
                for (mx, my, port_blk_a, hh) in [(1, 2, 1, 6), (2, 1, 0, 0), (0, 0, 1, 5)] {
                    let (mut t, r, mut i) = base(n, 9, 7);
                    t.mode_x = mx;
                    t.mode_y = my;
                    t.flag_cell_a = port_blk_a;
                    t.p1_const = 0;
                    t.p1x = vec![0.2, 0.9];
                    t.p1y = vec![1.1, 1.7];
                    t.p2_const = 0;
                    t.p2x = vec![0.8, 0.1];
                    t.p2y = vec![1.4, 1.2];
                    i.height = if hh == 0 { 0 } else { 7 };
                    let mut r2 = r;
                    r2.count = 2;
                    let mut o1 = vec![vec![0.0f32; 2]; 4];
                    let mut o2 = o1.clone();
                    let a = render_m(&t, &r2, &i, 0, &mut o1, None);
                    let b = render_m(&t, &r2, &i, 0, &mut o2, Some(m));
                    let eq = a == b && o1.iter().zip(&o2).all(|(x, y)| x.iter().zip(y).all(|(p, q)| p.to_bits() == q.to_bits() || (p.is_nan() && q.is_nan())));
                    diff |= !eq;
                }
            }
            if diff {
                changed += 1;
            } else {
                eprintln!("mutant {:?} not visible in probes", m);
            }
        }
        // the remaining mutants need non-degenerate geometry and are judged by the emulator T2 (run_r.py mut)
        assert!(changed >= 10, "only {} of 16 visible", changed);
    }
}
