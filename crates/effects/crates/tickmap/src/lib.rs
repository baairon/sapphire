//! TICKMAP: port of the mapping part of the original routine (listing offsets, plugin
//! libsapphire-ae 2019.02). The routine converts a shutter time into a host query, asks the host
//! for the parameter value of every lane and turns the answer into the node lanes.
//!
//! Scope (the route of the BlurMoCurves parameters rotate, z_dist and shift, types 1 and 2 with
//! subtype 0, plus the types 4 and 5 of the same generic path): tick computation,
//! the time multiplier with its doubling rule, the
//! checkout call shape, the 16.16 conversion, the
//! checkin, the lane loop, and the no-time path selected by the word [pinfo]
//! . NOT ported, reported as errors: the colour route (type 3), the
//! point route and the unit conversion for the subtype nibbles 1 to 7
//! (for the nibble 0 and 8 to 15 it returns its input unchanged, STATIC `ja `MEASURED in T2).
//!
//! Every constant is one `const` line in `k` so that a mutant differs from the base by one line.
//! The host is an explicit trait: the plugin never reads key lists, it asks the host.

/// Constants and thresholds of the routine, one per line (MUTANT-CONST sites).
pub mod k {
    /// 2^-16 (float, MEASURED-DYLIB in T2 `t2/consts.json`)
    pub const SCALE_16_16: f32 = 1.52587890625e-5;
    /// divisor when [rec] & F_DIV (float : MEASURED-DYLIB 100.0, not the 30.0 of older notes)
    pub const DIV_FLAGGED: f32 = 100.0;
    /// rounding offset of type 2 (float)
    pub const HALF: f32 = 0.5;
    /// sign mask xor-ed into the value when the negate flag is set (low dword of)
    pub const NEG_MASK: u32 = 0x8000_0000;
    /// value of the global that enables the doubling
    pub const DOUBLING_ID: i32 = 4;
    /// factor of the doubling (`lea edx,[rcx+rcx]`, `add ecx,ecx`)
    pub const DOUBLING_FACTOR: i32 = 2;
    /// record type of a colour parameter (: compared with 3)
    pub const T_COLOR: i32 = 3;
    /// record type of a popup (generic path: value - 1 clamped at 0)
    pub const T_POPUP: i32 = 4;
    /// record type of a checkbox
    pub const T_CHECK: i32 = 5;
    /// record type that is rounded (floor(v + 0.5)); types 1 and 2 are the scaled numeric types
    pub const T_ROUND: i32 = 2;
    /// the largest numeric (scaled) type; types 1 to this value are scaled
    pub const T_NUM_MAX: i32 = 2;
    /// popup bias of the generic path (`dec ecx`)
    pub const POPUP_BIAS: i32 = 1;
    /// [rec] flag: divide by DIV_FLAGGED
    pub const F_DIV: u8 = 0x40;
    /// [rec] flag: negate
    pub const F_NEG: u8 = 0x02;
    /// [rec] value that routes to the point records
    pub const K_CELL_A_POINT: u8 = 2;
    /// bit set of the subtype nibbles [rec]&0xf that take the point route {1,2,4,6,7}
    pub const POINT_NIBBLES: u32 = 0xd6;
    /// selector value of the generic (time passing) path
    pub const SEL_GENERIC: i16 = 0;
    /// no-time path: index bias (`dec esi`)
    pub const NT_INDEX_BIAS: i32 = 1;
    /// generic path: index step per lane (`add esi, r12d`)
    pub const LANE_STEP: i32 = 1;
    /// upper bound of the 32 bit conversions (2^31): at or above it, `cvttss2si`/`cvtss2si` give 0x80000000
    pub const CVT_HI: f32 = 2147483648.0;
    /// lower bound of the 32 bit conversions (-2^31): below it the integer indefinite value
    pub const CVT_LO: f32 = -2147483648.0;
    /// the tick uses the truncating conversion (`vcvttss2si`); true would round to nearest
    pub const TICK_ROUND: bool = false;
    /// no-time template (41 bytes copied to the out struct before every call), MEASURED-DYLIB
    pub const NT_TEMPLATE: [u8; 41] = [0u8; 41];
    /// unit conversion a listed site, nibble 5 (jump table index 4, arm a listed site): the double at a listed site, MEASURED-DYLIB
    pub const N5_BOUND: f64 = 1.0000000000000000e-4;
    /// the float at a listed site used when the size term is not above N5_BOUND, MEASURED-DYLIB
    pub const N5_FALLBACK: f32 = 1.0;
    /// the nibble that takes arm a listed site
    pub const N5_NIBBLE: u32 = 5;
    /// the arm constant K, float at a listed site, MEASURED-DYLIB
    pub const N5_K: f32 = 2.0;
    /// the lane that takes the second arm (`cmp r8d, 1`)
    pub const N5_LANE: usize = 1;
    /// the nibble that takes arm a listed site (jump table index 3), MEASURED table
    pub const N4_NIBBLE: u32 = 4;
    /// the float at a listed site (half), MEASURED-DYLIB
    pub const N4_HALF: f32 = 0.5;
    /// the lane that takes the second arm a listed site (`cmp r8d, 1`)
    pub const N4_LANE: usize = 1;
    /// flags bit tested by `test dl, 2` at a listed site
    pub const N4_FLAG_NEG: u32 = 2;
    /// flags the generic path passes at the call a listed site
    pub const UNIT_FLAGS_GENERIC: u32 = 2;
}

/// Fields of the render record `pinfo` read by the routine.
#[derive(Clone, Copy, Debug)]
pub struct Pinfo {
    /// [pinfo], current time in frames (f32)
    pub time_frames: f32,
    /// [pinfo], the selector word (signed 16 bit, `movsx`)
    pub selector: i16,
    /// a host field] and a host field] (doubles, narrowed to f32 for the unit conversion a listed site)
    pub unit_x3: f32,
    pub unit_d4: f64,
    pub unit_d5: f64,
}

/// Fields of the host record `[pinfo]` (r13 in the listing).
#[derive(Clone, Copy, Debug)]
pub struct HostRec {
    /// [r13] effect reference passed to checkout and checkin
    pub effect_ref: u64,
    /// [r13] time multiplier (ticks per frame)
    pub time_mult: i32,
    /// [r13] time scale
    pub time_scale: i32,
    /// [r13] doubling flag
    pub double_flag: i32,
    /// [r13] and [r13] size terms (only forwarded to the unit conversion)
    pub size_x: i32,
    pub size_y: i32,
}

/// Fields of the host object `[[pinfo]]` used by the no-time path.
#[derive(Clone, Copy, Debug)]
pub struct NoTimeObj {
    /// [obj]
    pub a: i32,
    /// [obj]
    pub c: u64,
}

/// Fields of the parameter record `rec` (rsi in the listing).
#[derive(Clone, Copy, Debug)]
pub struct ParamRec {
    /// record type field
    pub ptype: i32,
    /// [rec] lane count (byte)
    pub nlanes: u8,
    /// [rec]
    pub kind_byte: u8,
    /// [rec] (subtype nibble in the low four bits)
    pub subtype_byte: u8,
    /// [rec]
    pub flags_a_byte: u8,
    /// [rec]
    pub flags_b_byte: u8,
    /// [rec] first host index (signed 16 bit)
    pub index: i16,
    /// a host field (f32), read by the unit conversion nibble 5
    pub rec_unit_value: f32,
}

/// The host interface. All three calls are explicit; the plugin never reads key lists.
pub trait Host {
    /// the checkout callback: checkout(effect_ref, index, tick, time_step, time_scale, &pd). Returns
    /// (status, int32 value at pd). A non-zero status leaves the lane unwritten.
    fn checkout(&mut self, effect_ref: u64, index: i32, tick: i32, time_step: i32, time_scale: i32) -> (i32, i32);
    /// the checkin callback: checkin(effect_ref, &pd), called after every checkout, also after a failure.
    fn checkin(&mut self, effect_ref: u64);
    /// `call [[obj]]` (no-time path): fetch(obj.a, index, obj.c, out). `out` is the 41 byte
    /// struct prefilled with `k::NT_TEMPLATE`; the plugin reads the four bytes at offset 4.
    fn fetch_no_time(&mut self, a: i32, index: i32, c: u64, out: &mut [u8; 41]);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// type 3 record (colour route): not ported
    ColourRoute,
    /// point route : not ported
    PointRoute,
    /// subtype nibble 1..7: the unit conversion is not ported
    UnitConversion,
    /// selector non-zero but no host object supplied
    MissingHostObject,
    /// destination shorter than the lane count
    DestTooShort,
    /// point route: record type is not 1 or 2 (other jump table arms of a listed site)
    PointRecType,
    /// point route: selector non-zero (the no-time variant at a listed site is not ported)
    PointNoTime,
    /// point route: no host parameter list, or the host object is not a point (generic route a listed site)
    PointNotRouted,
    /// nibble 4 on the generic mapping path (flags 2): judged at slot entry only, size inputs not recorded
    N4GenericUnrecorded,
    /// point route: the prelude checkout (no host list, listing a listed site) returned a non-zero status (a listed site or other)
    PointPrelude(i32),
}

/// House form of a refusal (the label text the INT step prints). `prog` and `param` fill the placeholders.
pub fn refusal_text(e: &Error, prog: &str, param: &str) -> String {
    match e {
        Error::PointRecType => format!("TICKMAP-POINT-RECTYPE ({} {}: the point-parameter evaluator reaches its point arm only for record types 1 and 2; TMP3, W582)", prog, param),
        Error::PointNoTime => format!("TICKMAP-POINT-NOTIME ({} {}: the no-time selector is non-zero; that variant is not ported; TMP3, W582)", prog, param),
        Error::PointNotRouted => format!("TICKMAP-POINT-NOTROUTED ({} {}: the host object is not of type 6 and the routing global does not send nibble 1, 2 or 4 to the point arm; the generic route is not ported; TMP3, W582)", prog, param),
        Error::PointPrelude(st) => format!("TICKMAP-POINT-PRELUDE ({} {}: the host's type checkout for a point parameter returned status {}; the original logs it and continues with the answer as left, which is not modelled; TMP3, W596)", prog, param, st),
        Error::N4GenericUnrecorded => format!("TICKMAP-N4-GENERIC-UNRECORDED ({} {}: nibble 4 on the generic mapping path (flags 2) is judged at slot entry only; its size inputs on that path are not recorded; TMP2, W577)", prog, param),
        other => format!("TICKMAP-ROUTE-UNPORTED ({} {}: {:?})", prog, param, other),
    }
}

/// The host inputs of the point route, required (never defaulted) when the caller says a listed site evaluates the parameter.
pub struct PointCtx<'a> {
    pub h: &'a PointHostRec,
    /// [list[index]]; None when the host parameter list is absent
    pub obj_type: Option<i32>,
    pub host: &'a mut dyn PointHost,
}

/// The point route of the parameter evaluator a listed site (record types 1 and 2, host object type 6).
pub mod kp {
    /// jump table a listed site arms: record types 1 and 2 reach a listed site
    pub const REC_TYPE_LO: i32 = 1;
    pub const REC_TYPE_HI: i32 = 2;
    /// `cmp a host field, 6`
    pub const OBJ_POINT: i32 = 6;
    /// global a host field values: 9 routes nibbles 1, 2, 4 to the point route; 3 skips the offsets
    pub const GLOBAL_POINT: i32 = 9;
    pub const GLOBAL_NO_OFFSET: i32 = 3;
    /// nibbles that the global 9 route accepts
    pub const GLOBAL_NIBBLES: u32 = 0x16;
    /// |a host field| and |a host field| limit (a listed site)
    pub const OFFSET_LIMIT: i64 = 50000;
    /// the flag word bit that selects scaled sizes (a host field] & 0x10)
    pub const SCALED_BIT: u32 = 0x10;
    /// double added before truncating a scaled size (a listed site)
    pub const SIZE_ROUND: f64 = 0.5;
    /// flags passed to a listed site (edx)
    pub const UNIT_FLAGS: u32 = 0;
}

/// Fields of the host record read by the point route.
#[derive(Clone, Copy, Debug)]
pub struct PointHostRec {
    /// a host field, a host field, a host field, a host field
    pub effect_ref: u64,
    pub cur_time: i32,
    pub time_step: i32,
    pub time_scale: i32,
    /// a host field, a host field
    pub size_x: i32,
    pub size_y: i32,
    /// a host field, a host field
    pub off_a: i32,
    pub off_b: i32,
    /// bit 0x10 of a host field]
    pub scaled: bool,
    /// a host field, a host field, a host field, a host field
    pub hcv_host_a: i32,
    pub hcv_host_b: i32,
    pub hcv_host_c: i32,
    pub hcv_host_d: i32,
}

/// Host calls of the point route: a checkout that returns the two int32 at pd and pd, and the checkin.
pub trait PointHost {
    fn checkout_point(&mut self, effect_ref: u64, index: i32, cur: i32, step: i32, scale: i32) -> (i32, i32);
    fn checkin(&mut self, effect_ref: u64);
    /// the prelude checkout when the host list is absent (listing a listed site): same arguments, the pd buffer is the static scratch
    /// object at a listed site; returns (status, int32 at pd, the object type word). No checkin follows.
    fn checkout_type(&mut self, effect_ref: u64, index: i32, cur: i32, step: i32, scale: i32) -> (i32, i32);
}

/// `vcvttsd2si` (32 bit).
#[inline(always)]
pub fn cvtt_sd2si(x: f64) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 { i32::MIN } else { x as i32 }
}

/// One scaled size (listing a listed site..a listed site): ratio = (f32)n / (f32)(u32)d when d != 0, times (f32)size, plus 0.5 in
/// double, truncated, 1 when not positive.
fn scaled_size(n: i32, d: i32, size: i32) -> f32 {
    let mut a = n as f32;
    if d != 0 {
        a /= (d as u32) as f32;
    }
    let p = (size as f32) * a;
    let e = cvtt_sd2si(k_size_round() + (p as f64));
    (if e > 0 { e } else { 1 }) as f32
}
#[inline(always)]
fn k_size_round() -> f64 { kp::SIZE_ROUND }

/// The unit step of the point route for a given lane (nibbles 4 and 5 ported, 0 and 8..15 identity).
fn point_unit(rec: &ParamRec, pinfo: &Pinfo, lane: usize, v: f32, x1: f32, x2: f32) -> Result<f32, Error> {
    let nib = (rec.subtype_byte & 0xf) as u32;
    if nib == 0 || nib >= 8 {
        Ok(v)
    } else if nib == k::N4_NIBBLE {
        Ok(unit_nibble4(v, lane, kp::UNIT_FLAGS, x1, x2, pinfo.unit_x3))
    } else if nib == k::N5_NIBBLE {
        Ok(unit_nibble5(v, lane, &UnitIn { x1, rec_unit_value: rec.rec_unit_value, x3: pinfo.unit_x3, d4: pinfo.unit_d4, d5: pinfo.unit_d5 }))
    } else {
        Err(Error::UnitConversion)
    }
}

/// Port of the point route of a listed site (selector 0). `obj_type` is [list[index]] (None: no host list).
/// Returns None when the routine returns 0 without writing (negative index), else the two lane values stored at
/// a host field and a host field. No divide, rounding or negate step follows (listing a listed site jumps to the return).
pub fn map_point_param(
    pinfo: &Pinfo,
    h: &PointHostRec,
    global_id: i32,
    rec: &ParamRec,
    obj_type: Option<i32>,
    host: &mut dyn PointHost,
) -> Result<Option<[f32; 2]>, Error> {
    if (rec.index as i32) < 0 {
        return Ok(None);
    }
    if rec.ptype < kp::REC_TYPE_LO || rec.ptype > kp::REC_TYPE_HI {
        return Err(Error::PointRecType);
    }
    let ot = match obj_type {
        Some(t) => t,
        None => {
            let (st, t) = host.checkout_type(h.effect_ref, rec.index as i32, h.cur_time, h.time_step, h.time_scale);
            if st != 0 {
                return Err(Error::PointPrelude(st));
            }
            t
        }
    };
    let nib = (rec.subtype_byte & 0xf) as u32;
    let routed = ot == kp::OBJ_POINT || (global_id == kp::GLOBAL_POINT && (kp::GLOBAL_NIBBLES >> nib) & 1 != 0);
    if !routed {
        return Err(Error::PointNotRouted);
    }
    if pinfo.selector != k::SEL_GENERIC {
        return Err(Error::PointNoTime);
    }
    let (sx, sy) = if h.scaled {
        (scaled_size(h.hcv_host_a, h.hcv_host_b, h.size_x), scaled_size(h.hcv_host_c, h.hcv_host_d, h.size_y))
    } else {
        (h.size_x as f32, h.size_y as f32)
    };
    let (px, py) = host.checkout_point(h.effect_ref, rec.index as i32, h.cur_time, h.time_step, h.time_scale);
    host.checkin(h.effect_ref);
    let x1 = cvtt_ss2si(sx) as f32;
    let x2 = cvtt_ss2si(sy) as f32;
    let fx = k::SCALE_16_16 * (px as f32);
    let fy = k::SCALE_16_16 * (py as f32);
    let mut v0 = fx;
    let mut v1 = x2 - fy;
    if global_id != kp::GLOBAL_NO_OFFSET && (h.off_a as i64).abs() <= kp::OFFSET_LIMIT && (h.off_b as i64).abs() <= kp::OFFSET_LIMIT {
        v0 -= h.off_a as f32;
        v1 += h.off_b as f32;
    }
    let r0 = point_unit(rec, pinfo, 0, v0, x1, x2)?;
    let r1 = point_unit(rec, pinfo, 1, v1, x1, x2)?;
    Ok(Some([r0, r1]))
}

/// `vcvttss2si` (32 bit): truncate toward zero; NaN and out of range give 0x80000000.
#[inline(always)]
pub fn cvtt_ss2si(x: f32) -> i32 {
    if x.is_nan() || x >= k::CVT_HI || x < k::CVT_LO { i32::MIN } else { x as i32 }
}

/// `vcvtss2si` (32 bit, default rounding mode): round half to even; NaN and out of range give 0x80000000.
#[inline(always)]
pub fn cvt_ss2si(x: f32) -> i32 {
    if x.is_nan() || x >= k::CVT_HI || x < k::CVT_LO { return i32::MIN; }
    let r = x.round_ties_even();
    if r >= k::CVT_HI || r < k::CVT_LO { i32::MIN } else { r as i32 }
}

/// The time multiplier: [r13], doubled when the global equals DOUBLING_ID and [r13] != 0.
#[inline(always)]
pub fn time_multiplier(h: &HostRec, global_id: i32) -> i32 {
    if global_id == k::DOUBLING_ID && h.double_flag != 0 {
        h.time_mult.wrapping_mul(k::DOUBLING_FACTOR)
    } else {
        h.time_mult
    }
}

/// The tick : trunc_toward_zero((t + pinfo.time) * (float)M), f32 add, f32 multiply.
#[inline(always)]
pub fn shutter_tick(t: f32, p: &Pinfo, m: i32) -> i32 {
    let a = t + p.time_frames;
    let prod = a * (m as f32);
    if k::TICK_ROUND { cvt_ss2si(prod) } else { cvtt_ss2si(prod) }
}

/// The size term of a listed site (listing a listed site..a listed site): the f32 of the double d unless the double at
/// a listed site is not below it (or the compare is unordered, which keeps d), then the float at a listed site.
#[inline(always)]
fn n5_term(d: f64) -> f32 {
    let x = d as f32;
    // comisd bound, (double)x ; jb keeps x when bound < x or unordered
    let xd = x as f64;
    if k::N5_BOUND < xd || xd.is_nan() || k::N5_BOUND.is_nan() { x } else { k::N5_FALLBACK }
}

/// Inputs of the unit conversion a listed site that the nibble 5 arm reads.
#[derive(Clone, Copy, Debug)]
pub struct UnitIn {
    /// (float)a host field (xmm1 at the call a listed site)
    pub x1: f32,
    /// a host field (f32)
    pub rec_unit_value: f32,
    /// a host field (f32, xmm3)
    pub x3: f32,
    /// a host field] and (doubles narrowed to f32 at a listed site, then clamped)
    pub d4: f64,
    pub d5: f64,
}

/// Unit conversion a listed site at subtype nibble 5 (jump table index 4, arm a listed site, MEASURED table):
/// r = v * ((K / x1) * rec_unit_value); lane 1 only: r = r * ((t5 * x3) / (-t4)).
pub fn unit_nibble5(v: f32, lane: usize, u: &UnitIn) -> f32 {
    let t4 = n5_term(u.d4);
    let t5 = n5_term(u.d5);
    let scale = (k::N5_K / u.x1) * u.rec_unit_value;
    let r = v * scale;
    if lane != k::N5_LANE {
        r
    } else {
        let den = f32::from_bits(t4.to_bits() ^ k::NEG_MASK);
        r * ((t5 * u.x3) / den)
    }
}

/// Unit conversion a listed site at subtype nibble 4 (jump table index 3, arm a listed site; lane 1 arm a listed site):
/// s = K / x1; lane != 1: (v - x1 * 0.5) * s; lane 1, flags bit 1: -(((v + x2 * 0.5) * s) * x3);
/// lane 1, flags bit 1 clear: ((v - x2 * 0.5) * s) * x3.  x1 and x2 are the two sizes as f32, x3 = a host field.
pub fn unit_nibble4(v: f32, lane: usize, flags: u32, x1: f32, x2: f32, x3: f32) -> f32 {
    let s = k::N5_K / x1;
    if lane != k::N4_LANE {
        (v - x1 * k::N4_HALF) * s
    } else if flags & k::N4_FLAG_NEG != 0 {
        let r = ((v + x2 * k::N4_HALF) * s) * x3;
        f32::from_bits(r.to_bits() ^ k::NEG_MASK)
    } else {
        ((v - x2 * k::N4_HALF) * s) * x3
    }
}

/// The shared tail of the numeric conversion: unit step (identity for nibble 0 and 8..15), divide by
/// DIV_FLAGGED, rounding of type 2, negate.
#[inline(always)]
fn finish_numeric(rec: &ParamRec, pinfo: &Pinfo, lane: Option<(usize, f32, f32)>, mut v: f32) -> Result<f32, Error> {
    let nib = (rec.subtype_byte & 0xf) as u32;
    if (1..=7).contains(&nib) {
        match lane {
            Some(_) if nib == k::N4_NIBBLE => return Err(Error::N4GenericUnrecorded),
            Some((l, x1, _)) if nib == k::N5_NIBBLE => v = unit_nibble5(v, l, &UnitIn { x1, rec_unit_value: rec.rec_unit_value, x3: pinfo.unit_x3, d4: pinfo.unit_d4, d5: pinfo.unit_d5 }),
            _ => return Err(Error::UnitConversion),
        }
    }
    if rec.flags_a_byte & k::F_DIV != 0 {
        v /= k::DIV_FLAGGED;
    }
    if rec.ptype == k::T_ROUND {
        let r = (k::HALF + v).floor();
        v = cvt_ss2si(r) as f32;
    }
    if rec.flags_b_byte & k::F_NEG != 0 {
        v = f32::from_bits(v.to_bits() ^ k::NEG_MASK);
    }
    Ok(v)
}

/// Port of the mapping part of .
///
/// * `host_rec` = [pinfo]; `None` returns at once without writing .
/// * `nt_obj` = the host object [[pinfo]], needed only when `pinfo.selector != 0`.
/// * `global_id` = the global word .
/// * `t` = the shutter time (xmm0).
/// * `dest` = the node lanes (rdx); lanes the routine does not write keep their content.
pub fn map_param(
    pinfo: &Pinfo,
    host_rec: Option<&HostRec>,
    nt_obj: Option<&NoTimeObj>,
    global_id: i32,
    rec: &ParamRec,
    t: f32,
    host: &mut dyn Host,
    dest: &mut [f32],
    point: Option<PointCtx<'_>>,
) -> Result<(), Error> {
    let h = match host_rec {
        None => return Ok(()),
        Some(h) => h,
    };
    let m = time_multiplier(h, global_id);
    let tick = shutter_tick(t, pinfo, m);
    let nib = (rec.subtype_byte & 0xf) as u32;
    if let Some(pc) = point {
        let routed = pc.obj_type == Some(kp::OBJ_POINT) || (global_id == kp::GLOBAL_POINT && (kp::GLOBAL_NIBBLES >> nib) & 1 != 0);
        if routed || pc.obj_type.is_none() {
            return match map_point_param(pinfo, pc.h, global_id, rec, pc.obj_type, pc.host)? {
                None => Ok(()),
                Some(v) => {
                    if dest.len() < 2 {
                        return Err(Error::DestTooShort);
                    }
                    dest[0] = v[0];
                    dest[1] = v[1];
                    Ok(())
                }
            };
        }
    }
    if rec.kind_byte == k::K_CELL_A_POINT && (k::POINT_NIBBLES >> nib) & 1 != 0 {
        return Err(Error::PointRoute);
    }
    if rec.ptype == k::T_COLOR {
        return Err(Error::ColourRoute);
    }
    let n = rec.nlanes as usize;
    if dest.len() < n {
        return Err(Error::DestTooShort);
    }
    if pinfo.selector == k::SEL_GENERIC {
        for lane in 0..n {
            let index = (rec.index as i32).wrapping_add((lane as i32).wrapping_mul(k::LANE_STEP));
            let (status, raw) = host.checkout(h.effect_ref, index, tick, m, h.time_scale);
            if status == 0 {
                let ty = rec.ptype;
                if ty > 0 && ty <= k::T_NUM_MAX {
                    let v = (raw as f32) * k::SCALE_16_16;
                    dest[lane] = finish_numeric(rec, pinfo, Some((lane, h.size_x as f32, h.size_y as f32)), v)?;
                } else if ty == k::T_POPUP {
                    let c = raw.wrapping_sub(k::POPUP_BIAS);
                    let c = if c < 0 { 0 } else { c };
                    dest[lane] = c as f32;
                } else if ty == k::T_CHECK {
                    dest[lane] = raw as f32;
                }
            }
            host.checkin(h.effect_ref);
        }
    } else {
        let o = match nt_obj {
            None => return Err(Error::MissingHostObject),
            Some(o) => o,
        };
        for lane in 0..n {
            let mut out = k::NT_TEMPLATE;
            let index = (rec.index as i32).wrapping_sub(k::NT_INDEX_BIAS);
            host.fetch_no_time(o.a, index, o.c, &mut out);
            let word = u32::from_le_bytes([out[4], out[5], out[6], out[7]]);
            let ty = rec.ptype;
            if ty > 0 && ty <= k::T_NUM_MAX {
                dest[lane] = finish_numeric(rec, pinfo, None, f32::from_bits(word))?;
            } else if ty == k::T_POPUP {
                let c = word as i32;
                dest[lane] = (if c <= 0 { 0 } else { c }) as f32;
            } else if ty == k::T_CHECK {
                dest[lane] = (word as i32) as f32;
            }
        }
    }
    Ok(())
}

/// The sign rule alone (rotate is negated): helper for callers that hold the compiler lane.
#[inline(always)]
pub fn lane_from_raw_numeric(raw: i32, negate: bool) -> f32 {
    let v = (raw as f32) * k::SCALE_16_16;
    if negate { f32::from_bits(v.to_bits() ^ k::NEG_MASK) } else { v }
}

#[cfg(test)]
mod pt_tests;

#[cfg(test)]
mod tests {
    use super::*;
    struct H { log: Vec<String> }
    impl Host for H {
        fn checkout(&mut self, r: u64, i: i32, t: i32, s: i32, sc: i32) -> (i32, i32) {
            self.log.push(format!("{r},{i},{t},{s},{sc}"));
            (0, 65536 * (i + 1))
        }
        fn checkin(&mut self, _r: u64) {}
        fn fetch_no_time(&mut self, _a: i32, _i: i32, _c: u64, _o: &mut [u8; 41]) {}
    }
    #[test]
    fn rotate_is_negated_and_scaled() {
        let p = Pinfo { time_frames: 1.0, selector: 0, unit_x3: 1.0, unit_d4: 1.0, unit_d5: 1.0 };
        let hr = HostRec { effect_ref: 7, time_mult: 256, time_scale: 1, double_flag: 0, size_x: 0, size_y: 0 };
        let rec = ParamRec { ptype: 1, nlanes: 1, kind_byte: 1, subtype_byte: 0, flags_a_byte: 0, flags_b_byte: 6, index: 0, rec_unit_value: 1.0 };
        let mut d = [0.0f32; 1];
        let mut h = H { log: vec![] };
        map_param(&p, Some(&hr), None, 0, &rec, 0.5, &mut h, &mut d, None).unwrap();
        assert_eq!(d[0], -1.0);
        assert_eq!(h.log[0], "7,0,384,256,1");
    }
    struct FixedHost(Vec<i32>);
    impl Host for FixedHost {
        fn checkout(&mut self, _r: u64, i: i32, _t: i32, _s: i32, _sc: i32) -> (i32, i32) { (0, self.0[(i - 24) as usize]) }
        fn checkin(&mut self, _r: u64) {}
        fn fetch_no_time(&mut self, _a: i32, _i: i32, _c: u64, _o: &mut [u8; 41]) {}
    }
    fn shift_rec(b2: u8) -> ParamRec { ParamRec { ptype: 1, nlanes: 2, kind_byte: 2, subtype_byte: b2, flags_a_byte: 0, flags_b_byte: 0, index: 24, rec_unit_value: 1.0 } }
    fn shift_run(b2: u8, sel: i16, raws: Vec<i32>) -> Result<[f32; 2], Error> {
        let p = Pinfo { time_frames: 0.0, selector: sel, unit_x3: 1.0, unit_d4: 1.0, unit_d5: 1.0 };
        let hr = HostRec { effect_ref: 1, time_mult: 1, time_scale: 24, double_flag: 0, size_x: 64, size_y: 32 };
        let o = NoTimeObj { a: 0, c: 0 };
        let mut d = [0.0f32; 2];
        map_param(&p, Some(&hr), Some(&o), 1, &shift_rec(b2), 0.5, &mut FixedHost(raws), &mut d, None)?;
        Ok(d)
    }
    #[test]
    fn nibble5_default_lane0_plus_zero_lane1_minus_zero() {
        let d = shift_run(5, 0, vec![0, 0]).unwrap();
        assert_eq!(d[0].to_bits(), 0x0000_0000);
        assert_eq!(d[1].to_bits(), 0x8000_0000);
    }
    #[test]
    fn nibble5_nonzero_lanes() {
        let d = shift_run(5, 0, vec![0x4000, -0x2000]).unwrap();
        assert_eq!(d[0].to_bits(), 0x3c00_0000);
        assert_eq!(d[1].to_bits(), 0x3b80_0000);
    }
    fn gen_rec(b2: u8) -> ParamRec { ParamRec { ptype: 1, nlanes: 2, kind_byte: 1, subtype_byte: b2, flags_a_byte: 0, flags_b_byte: 0, index: 24, rec_unit_value: 1.0 } }
    fn gen_run(b2: u8, raws: Vec<i32>) -> Result<[f32; 2], Error> {
        let p = Pinfo { time_frames: 0.0, selector: 0, unit_x3: 1.0, unit_d4: 1.0, unit_d5: 1.0 };
        let hr = HostRec { effect_ref: 1, time_mult: 1, time_scale: 24, double_flag: 0, size_x: 64, size_y: 32 };
        let mut d = [0.0f32; 2];
        map_param(&p, Some(&hr), None, 1, &gen_rec(b2), 0.5, &mut FixedHost(raws), &mut d, None)?;
        Ok(d)
    }
    #[test]
    fn nibble4_arm_record_values_flags0() {
        // the claim record: lane 0 v 50.0, lane 1 v -18.0, flags 0, sizes 64 x 32, x3 1.0
        assert_eq!(unit_nibble4(50.0, 0, 0, 64.0, 32.0, 1.0).to_bits(), 0x3f10_0000);
        assert_eq!(unit_nibble4(-18.0, 1, 0, 64.0, 32.0, 1.0).to_bits(), 0xbf88_0000);
    }
    #[test]
    fn nibble4_generic_refuses_unrecorded() {
        assert_eq!(gen_run(4, vec![0, 0]), Err(Error::N4GenericUnrecorded));
        assert_eq!(gen_run(4, vec![0x4000, -0x2000]), Err(Error::N4GenericUnrecorded));
    }
    #[test]
    fn nibbles_1_2_3_6_7_still_refuse_and_nibble4_point_refuses() {
        for b2 in [1u8, 2, 3, 6, 7] { assert_eq!(gen_run(b2, vec![0, 0]), Err(Error::UnitConversion)); }
        assert_eq!(gen_run(5, vec![0, 0]).is_ok(), true);
    }
    #[test]
    fn nibble4_and_no_time_nibble5_refuse() {
        assert_eq!(shift_run(4, 0, vec![0, 0]), Err(Error::PointRoute));
        assert_eq!(shift_run(3, 0, vec![0, 0]), Err(Error::UnitConversion));
        assert_eq!(shift_run(5, 1, vec![0, 0]), Err(Error::UnitConversion));
    }
    #[test]
    fn cvt_edges() {
        assert_eq!(cvtt_ss2si(f32::NAN), i32::MIN);
        assert_eq!(cvtt_ss2si(-1.9), -1);
        assert_eq!(cvt_ss2si(2.5), 2);
        assert_eq!(cvt_ss2si(3.5), 4);
        assert_eq!(cvt_ss2si(2147483648.0), i32::MIN);
    }
}
