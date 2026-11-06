//! make_mipmap: the class slots 33, 32 and 34 end to end (EG table rows 6 to 8: pre-pass, pass runner, post step).
//!
//! slot 33 = free planes (image vtable slot 6), allocator (sizes, tables), plane allocation (image vtable slot 5)
//! slot 32 = base level fill (ONE row per call: row rec - rec of plane (0,0)), ripmap objects only (the object's mode field == 3, the record's mode field == 0)
//! slot 34 = the pyramid builder of lib.rs (`build`)
//! Std only, no unsafe, explicit parameters (no Default).

use crate::{build, Error, Params, RipMap};

/// captured 2^-12 (bytes 00008039): the s16 and s32 conversion scale of 
pub const S16_SCALE_BITS: u32 = 0x3980_0000;
/// captured 1.0f used by the allocator's reciprocal table
pub const ONE_BITS: u32 = 0x3f80_0000;
/// captured double (divisor of the level count: ln 2); verified by the allocator T2 over W = 1 to 4200
pub const LN2: f64 = std::f64::consts::LN_2;

/// Deliberate bugs for the mutation test of the new code. `None` is the port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mut2 {
    None,
    ScaleDouble,
    FmtSwap01,
    RowOffsetIgnored,
    XShiftIgnored,
    ClampPadSwap,
    TilePadSwap,
    ConstPadSkipped,
    LevelsPlus1,
    LevelsFloor,
    WidthsFloorHalf,
    ScaleTableSkipped,
    ScaleTableFlip,
    FmtTwoConvert,
    RowGuardOff,
    PitchNoRound,
    FilterAsCount,
    HeightsFromWidth,
}

#[derive(Debug, Clone)]
pub struct ObjParams {
    /// obj
    pub w: i32,
    /// obj
    pub h: i32,
    /// obj
    pub c: usize,
    /// obj (GPU pitch unit; only feeds the and fields)
    pub d0: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllocOut {
    pub nl: i32,
    pub np: i32,
    pub widths: Vec<i32>,
    pub heights: Vec<i32>,
    /// obj table, set only when the filter is nonzero
    pub scale: Option<Vec<u32>>,
    pub width_sum: i32,
    pub height_sum: i32,
    pub pitch_elems: u32,
    pub pitch_unit_bytes: i64,
    /// sizes handed to the host heap by, in call order
    pub heap_requests: Vec<u64>,
    /// sizes handed to the plane allocator ([obj] vtable slot 0) by slot 5, in call order (l major, j minor)
    pub plane_requests: Vec<u64>,
}

fn cvttss2si(v: f32) -> i32 {
    if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
        i32::MIN
    } else {
        v as i32
    }
}

/// log(x) / ln 2 for x >= 1, from only exact IEEE operations (no platform libm): the binary exponent k plus
/// ln(m) / ln 2 for the mantissa m in [1, 2), where ln(m) = 2 atanh((m - 1) / (m + 1)) is summed as a series.
fn log_over_ln2(x: i32) -> f64 {
    let k = 31 - x.leading_zeros() as i32;
    let m = x as f64 / (1u64 << k) as f64;
    let z = (m - 1.0) / (m + 1.0);
    let z2 = z * z;
    let mut term = z;
    let mut sum = 0.0f64;
    let mut n = 1.0f64;
    for _ in 0..40 {
        sum += term / n;
        term *= z2;
        n += 2.0;
    }
    k as f64 + 2.0 * sum / LN2
}

/// levels = ceil(log(x) / ln 2) + 2, or 2 when that is negative (: log, divsd by ln 2, cvtsd2ss, ceilf (roundss 2 = round up), cvttss2si)
fn level_count(x: i32, m: Mut2) -> i32 {
    // log of a non-positive size is -inf or NaN, which the conversion below turns into the 2-level result
    let l = if x >= 1 { log_over_ln2(x) } else { f64::NEG_INFINITY };
    let mut f = (l as f32).ceil();
    if m == Mut2::LevelsFloor {
        f = (l as f32).floor();
    }
    let q = cvttss2si(f);
    let add = if m == Mut2::LevelsPlus1 { 3 } else { 2 };
    if q < 0 {
        2
    } else {
        q.wrapping_add(add)
    }
}

fn chain(w0: i32, n: i32, m: Mut2) -> Vec<i32> {
    let mut v = Vec::new();
    let mut e = w0;
    for _ in 0..n.max(0) {
        v.push(e);
        e = if m == Mut2::WidthsFloorHalf { e >> 1 } else { e.wrapping_add(1) >> 1 };
    }
    v
}

/// The allocator (obj, filter != 0) for the CPU object (obj == 0), with the plane allocation of slot 5.
pub fn alloc(o: &ObjParams, filter: i32, m: Mut2) -> Result<AllocOut, Error> {
    let f = if m == Mut2::FilterAsCount { ((filter & 0xff) != 0) as i32 } else { (filter != 0) as i32 };
    let (nl, np) = if f != 0 {
        (level_count(o.w, m), level_count(if m == Mut2::HeightsFromWidth { o.w } else { o.h }, m))
    } else {
        (1, 1)
    };
    let widths = chain(o.w, nl, m);
    let heights = chain(if m == Mut2::HeightsFromWidth { o.w } else { o.h }, np, m);
    let width_sum = widths.iter().fold(0i32, |a, &w| a.wrapping_add(w).wrapping_add(3));
    let height_sum = heights.iter().fold(0i32, |a, &h| a.wrapping_add(h).wrapping_add(3));
    let d4 = (o.d0 as i64) << 2;
    if d4 == 0 {
        return Err(Error::Contract("obj is zero (divides by it)".into()));
    }
    let rdi = (width_sum as i64 as u64).wrapping_mul(d4 as u64);
    let up = if m == Mut2::PitchNoRound { rdi / 256 } else { rdi.wrapping_add(255) / 256 };
    let pitch_elems = (up.wrapping_mul(256) / (d4 as u64)) as u32;
    let mut heap = vec![(nl.max(0) as u64) * 4, (np.max(0) as u64) * 4, (nl.max(0) as u64) * 8, (nl.max(0) as u64) * 8];
    for _ in 0..nl.max(0) {
        heap.push((np.max(0) as u64) * 8);
        heap.push((np.max(0) as u64) * 8);
    }
    let scale = if f != 0 {
        let mx = nl.max(np);
        heap.push((mx.max(0) as u64) * 4);
        let mut t = Vec::new();
        for k in 0..mx.max(0) {
            let k2 = if m == Mut2::ScaleTableFlip { mx - 1 - k } else { k };
            let den = (1i32.wrapping_shl(k2 as u32)) as f32;
            t.push((f32::from_bits(ONE_BITS) / den).to_bits());
        }
        if m == Mut2::ScaleTableSkipped {
            None
        } else {
            Some(t)
        }
    } else {
        None
    };
    let mut plane_requests = Vec::new();
    for &w in &widths {
        for &h in &heights {
            let n = (h.wrapping_add(3)).wrapping_mul(w.wrapping_add(3)).wrapping_mul(o.c as i32);
            plane_requests.push((n as i64 as u64).wrapping_shl(2));
        }
    }
    Ok(AllocOut { nl, np, widths, heights, scale, width_sum, height_sum, pitch_elems, pitch_unit_bytes: d4, heap_requests: heap, plane_requests })
}

/// Slot 33: builds the (sentinel filled) ripmap object for a CPU ripmap output.
pub fn slot33(o: &ObjParams, filter: i32, fill: u32, m: Mut2) -> Result<(AllocOut, RipMap), Error> {
    let a = alloc(o, filter, m)?;
    if a.widths.iter().chain(a.heights.iter()).any(|&v| v <= 0) {
        return Err(Error::Dims("non positive extent".into()));
    }
    let rm = RipMap::new(o.c, a.widths.iter().map(|&v| v as usize).collect(), a.heights.iter().map(|&v| v as usize).collect(), fill)?;
    Ok((a, rm))
}

/// Source set `[this]`: format at the second field (0 s16, 1 s32, 2 f32 bits), channel count, one element array per channel (one pointer per channel c).
#[derive(Debug, Clone)]
pub struct SrcRow {
    pub format: i32,
    pub c: usize,
    /// element values per channel: format 0 uses the low 16 bits, formats 1 and 2 all 32 bits
    pub planes: Vec<Vec<u32>>,
}

/// rec fields read by (explicit)
#[derive(Debug, Clone)]
pub struct RecRow {
    /// [[rec] field]
    pub roi_x: i32,
    pub x_cell_a: i32,
    pub y_cell_a: i32,
    pub y_cell_b: i32,
}

fn convert(fmt: i32, v: u32, m: Mut2) -> Result<u32, Error> {
    let fmt = if m == Mut2::FmtSwap01 && fmt <= 1 { 1 - fmt } else { fmt };
    let sc = if m == Mut2::ScaleDouble { f32::from_bits(S16_SCALE_BITS) * 2.0 } else { f32::from_bits(S16_SCALE_BITS) };
    match fmt {
        0 => Ok((((v & 0xffff) as u16 as i16 as i32) as f32 * sc).to_bits()),
        1 => Ok(((v as i32) as f32 * sc).to_bits()),
        2 => {
            if m == Mut2::FmtTwoConvert {
                Ok(((v as i32) as f32 * sc).to_bits())
            } else {
                Ok(v)
            }
        }
        _ => Err(Error::Contract("source format outside 0..2 (asserts)".into())),
    }
}

/// Slot 32, ripmap object, the record's mode field == 0: (rec, obj, [this], wrap_x). Converts one row of plane (0,0) and pads its side columns.
pub fn base_fill(rm: &mut RipMap, src: &SrcRow, rec: &RecRow, wrap_x: i32, pad: &[u32], m: Mut2) -> Result<(), Error> {
    let c = rm.c;
    if src.c != c {
        return Err(Error::Contract("source channel count differs from the destination (left pad position uses the source count)".into()));
    }
    if src.planes.len() != c || pad.len() != c {
        return Err(Error::Pad);
    }
    let w = rm.widths[0];
    let h = rm.heights[0] as i64;
    let xs = if m == Mut2::XShiftIgnored { 0 } else { rec.x_cell_a.wrapping_sub(rec.roi_x) as i64 };
    let y = if m == Mut2::RowOffsetIgnored { rec.y_cell_a } else { rec.y_cell_a.wrapping_sub(rec.y_cell_b) } as i64;
    let (lo, hi) = if m == Mut2::RowGuardOff { (0, h + 1) } else { (0, h) };
    if y < lo || y >= hi {
        return Ok(());
    }
    let y = y as usize;
    for x in 0..w {
        let idx = xs + x as i64;
        for ch in 0..c {
            if idx < 0 || idx as usize >= src.planes[ch].len() {
                return Err(Error::Contract("source element index outside the supplied row".into()));
            }
            let v = convert(src.format, src.planes[ch][idx as usize], m)?;
            let i = rm.at(0, x as isize, y as isize, ch);
            rm.planes[0][i] = v;
        }
    }
    for ch in 0..c {
        let first = rm.planes[0][rm.at(0, 0, y as isize, ch)];
        let last = rm.planes[0][rm.at(0, w as isize - 1, y as isize, ch)];
        let (l, r1, r2) = match wrap_x {
            2 => {
                if m == Mut2::ClampPadSwap {
                    (last, first, first)
                } else {
                    (first, last, last)
                }
            }
            1 => {
                if m == Mut2::TilePadSwap {
                    (first, last, last)
                } else {
                    (last, first, first)
                }
            }
            _ => {
                if m == Mut2::ConstPadSkipped {
                    continue;
                }
                (pad[ch], pad[ch], pad[ch])
            }
        };
        let (a, b, d) = (rm.at(0, -1, y as isize, ch), rm.at(0, w as isize, y as isize, ch), rm.at(0, w as isize + 1, y as isize, ch));
        rm.planes[0][a] = l;
        rm.planes[0][b] = r1;
        rm.planes[0][d] = r2;
    }
    Ok(())
}

/// Slot 34: pyramid (this wrap_x, this wrap_y, this filter)
pub fn slot34(rm: &mut RipMap, wrap_x: i32, wrap_y: i32, filter: i32, pad: &[u32]) -> Result<(), Error> {
    build(&Params { wrap_x, wrap_y, filter, pad_pixel: pad.to_vec() }, rm)
}

pub fn assumptions2() -> Vec<(&'static str, &'static str)> {
    vec![
        ("MM2-A1", "slot 32 converts exactly ONE row per call: row y = rec - rec of plane (0,0) from per-row source pointers [this] (one pointer per channel c) shifted by x_cell_a - [[rec] field] elements; the host driver's call pattern (one call per base row) is NOT measured (HIGH-STAKES)"),
        ("MM2-A2", "CONTRACT-LIMIT 1 <= C <= 4 for slot 32: keeps the shifted source pointers in a stack array of 8 byte slots at rbp that overlaps its saved registers from the fifth channel on; slot 60 has combos for C = 1..4 only"),
        ("MM2-A3", "the source channel count must equal the destination's (the left pad pixel position is computed from the source count at [src], the loops from the destination's)"),
        ("MM2-A4", "CAPTURED constants: s16/s32 scale 2^-12, 1.0f, ln 2 as double; LIBM-MODEL: log and floorf ran as real emulated code in the allocator T2"),
        ("MM2-A5", "HOST-HOOKED: the host heap and the plane allocator object at obj (alloc, free, lock) are service hooks; their request sizes are compared; plane memory is sentinel filled, alignment of returned planes is a parameter (ALIGN-DEPENDENT, output neutral)"),
        ("MM2-A6", "obj is a host field that only feeds the GPU pitch fields obj and obj: PARAM-UNCAPTURED explicit parameter, two or more values in T2; GPU object (obj != 0) paths and obj tail are NOT REBUILT"),
        ("MM2-A7", "the object's mode field != 3 branch of slot 32 (plane copy NOT REBUILT rect copy) and the record's mode field != 0 (GPU) are NOT REBUILT"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The level count of the former libm route, kept here as the reference for the exact route.
    fn level_count_libm(x: i32, m: Mut2) -> i32 {
        let l = (x as f64).ln() / LN2;
        let mut f = (l as f32).ceil();
        if m == Mut2::LevelsFloor {
            f = (l as f32).floor();
        }
        let q = cvttss2si(f);
        let add = if m == Mut2::LevelsPlus1 { 3 } else { 2 };
        if q < 0 {
            2
        } else {
            q.wrapping_add(add)
        }
    }

    #[test]
    fn level_count_matches_the_libm_route_for_every_size() {
        for m in [Mut2::None, Mut2::LevelsFloor, Mut2::LevelsPlus1] {
            for x in -2..=8192 {
                assert_eq!(level_count(x, m), level_count_libm(x, m), "x={x}");
            }
            for x in (8193..=1 << 24).step_by(997) {
                assert_eq!(level_count(x, m), level_count_libm(x, m), "x={x}");
            }
        }
    }

    #[test]
    fn levels_and_extents_from_the_listing() {
        // NL = ceil(log2 W) + 2, widths halve with ceil: W = 5 -> 5 levels 5,3,2,1,1; W = 8 -> 5 levels 8,4,2,1,1; W = 1 -> 2 levels 1,1
        let o = ObjParams { w: 5, h: 8, c: 3, d0: 16 };
        let a = alloc(&o, 1, Mut2::None).unwrap();
        assert_eq!((a.nl, a.np), (5, 5));
        assert_eq!(a.widths, vec![5, 3, 2, 1, 1]);
        assert_eq!(a.heights, vec![8, 4, 2, 1, 1]);
        assert_eq!(a.width_sum, 8 + 6 + 5 + 4 + 4);
        assert_eq!(a.scale.as_ref().unwrap().len(), 5);
        assert_eq!(f32::from_bits(a.scale.as_ref().unwrap()[3]), 0.125);
        let a = alloc(&ObjParams { w: 1, h: 1, c: 1, d0: 4 }, 1, Mut2::None).unwrap();
        assert_eq!((a.nl, a.np), (2, 2));
        let a = alloc(&o, 0, Mut2::None).unwrap();
        assert_eq!((a.nl, a.np, a.scale.is_none()), (1, 1, true));
        assert_eq!(a.plane_requests, vec![(8u64 + 3) * (5 + 3) * 3 * 4]);
    }

    #[test]
    fn pitch_fields_follow_x() {
        // pitch_elems = ceil(width_sum * 4 d0 / 256) * 256 / (4 d0); pitch_unit_bytes = 4 d0
        let a = alloc(&ObjParams { w: 5, h: 8, c: 3, d0: 3 }, 0, Mut2::None).unwrap();
        assert_eq!(a.width_sum, 8);
        assert_eq!(a.pitch_unit_bytes, 12);
        assert_eq!(a.pitch_elems, ((8u64 * 12 + 255) / 256 * 256 / 12) as u32);
        assert!(alloc(&ObjParams { w: 5, h: 8, c: 3, d0: 0 }, 0, Mut2::None).is_err());
    }

    fn rm1(w: usize, h: usize, c: usize) -> RipMap {
        RipMap::new(c, vec![w], vec![h], 0x7fc0ffee).unwrap()
    }

    #[test]
    fn base_fill_formats_and_pads() {
        let mut rm = rm1(3, 2, 2);
        let src = SrcRow { format: 0, c: 2, planes: vec![vec![0xffff, 4096, 0x8000, 7], vec![1, 2, 3, 4]] };
        let rec = RecRow { roi_x: 0, x_cell_a: 1, y_cell_a: 11, y_cell_b: 10 };
        // row 1, x shift 1: pixels 0..2 read elements 1..3
        base_fill(&mut rm, &src, &rec, 1, &[0, 0], Mut2::None).unwrap();
        let g = |rm: &RipMap, x: isize, y: isize, ch: usize| f32::from_bits(rm.planes[0][rm.at(0, x, y, ch)]);
        assert_eq!(g(&rm, 0, 1, 0), 1.0);
        assert_eq!(g(&rm, 1, 1, 0), -8.0);
        assert_eq!(g(&rm, 2, 1, 0), 7.0 / 4096.0);
        assert_eq!(g(&rm, 0, 1, 1), 2.0 / 4096.0);
        // tile: left <- last, right <- first, other rows untouched
        assert_eq!(g(&rm, -1, 1, 0), 7.0 / 4096.0);
        assert_eq!(g(&rm, 3, 1, 0), 1.0);
        assert_eq!(g(&rm, 4, 1, 0), 1.0);
        assert_eq!(rm.planes[0][rm.at(0, 0, 0, 0)], 0x7fc0ffee);
        // clamp and constant
        base_fill(&mut rm, &src, &rec, 2, &[0, 0], Mut2::None).unwrap();
        assert_eq!(g(&rm, -1, 1, 0), 1.0);
        assert_eq!(g(&rm, 3, 1, 0), 7.0 / 4096.0);
        let pad = [f32::to_bits(9.0), f32::to_bits(-1.0)];
        base_fill(&mut rm, &src, &rec, 0, &pad, Mut2::None).unwrap();
        assert_eq!(g(&rm, -1, 1, 1), -1.0);
        assert_eq!(g(&rm, 4, 1, 0), 9.0);
        // row outside the image: nothing written
        let mut rm2 = rm1(3, 2, 2);
        base_fill(&mut rm2, &src, &RecRow { roi_x: 0, x_cell_a: 1, y_cell_a: 12, y_cell_b: 10 }, 1, &[0, 0], Mut2::None).unwrap();
        assert!(rm2.planes[0].iter().all(|&v| v == 0x7fc0ffee));
    }

    #[test]
    fn s32_and_f32_formats() {
        let mut rm = rm1(2, 1, 1);
        let src = SrcRow { format: 1, c: 1, planes: vec![vec![16777217, 0xffff_f000]] };
        let rec = RecRow { roi_x: 5, x_cell_a: 5, y_cell_a: 0, y_cell_b: 0 };
        base_fill(&mut rm, &src, &rec, 2, &[0], Mut2::None).unwrap();
        // 16777217 -> 16777216 (cvtsi2ss rounds to even) times 2^-12; -4096 times 2^-12
        assert_eq!(f32::from_bits(rm.planes[0][rm.at(0, 0, 0, 0)]), 4096.0);
        assert_eq!(f32::from_bits(rm.planes[0][rm.at(0, 1, 0, 0)]), -1.0);
        let src = SrcRow { format: 2, c: 1, planes: vec![vec![0x7fc00123, 0x80000000]] };
        base_fill(&mut rm, &src, &rec, 2, &[0], Mut2::None).unwrap();
        assert_eq!(rm.planes[0][rm.at(0, 0, 0, 0)], 0x7fc00123);
        assert_eq!(rm.planes[0][rm.at(0, 1, 0, 0)], 0x8000_0000);
        let bad = SrcRow { format: 3, c: 1, planes: vec![vec![0, 0]] };
        assert!(base_fill(&mut rm, &bad, &rec, 2, &[0], Mut2::None).is_err());
    }

    #[test]
    fn end_to_end_small_and_errors() {
        let o = ObjParams { w: 3, h: 2, c: 1, d0: 1 };
        let (a, mut rm) = slot33(&o, 1, 0, Mut2::None).unwrap();
        assert_eq!((a.nl, a.np), (4, 3));
        let src = SrcRow { format: 2, c: 1, planes: vec![vec![1.0f32.to_bits(), 3.0f32.to_bits(), 5.0f32.to_bits()]] };
        for y in 0..2 {
            base_fill(&mut rm, &src, &RecRow { roi_x: 0, x_cell_a: 0, y_cell_a: y, y_cell_b: 0 }, 2, &[0], Mut2::None).unwrap();
        }
        slot34(&mut rm, 2, 2, 1, &[0]).unwrap();
        // level 1 x = 0 is (1 + 3) / 2
        assert_eq!(f32::from_bits(rm.planes[rm.idx(1, 0)][rm.at(1, 0, 0, 0)]), 2.0);
        assert!(slot33(&ObjParams { w: 0, h: 2, c: 1, d0: 1 }, 1, 0, Mut2::None).is_err());
        assert!(slot33(&ObjParams { w: 3, h: 2, c: 1, d0: 0 }, 1, 0, Mut2::None).is_err());
        let again = alloc(&o, 1, Mut2::None).unwrap();
        assert_eq!(a, again);
        assert_eq!(assumptions2().len(), 7);
    }

    #[test]
    fn mutants_change_results() {
        let o = ObjParams { w: 7, h: 6, c: 2, d0: 5 };
        let base = alloc(&o, 1, Mut2::None).unwrap();
        for m in [Mut2::LevelsPlus1, Mut2::WidthsFloorHalf, Mut2::ScaleTableFlip, Mut2::PitchNoRound, Mut2::HeightsFromWidth, Mut2::ScaleTableSkipped] {
            assert_ne!(alloc(&o, 1, m).unwrap(), base, "{:?}", m);
        }
    }
}
