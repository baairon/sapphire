//! INT29 step TW (W559 (a)): TWF1, TWF2 and TWF3's frame port merged as module fx_exec::twframe, the TWF3 crate text verbatim (its
//! mutation switch MUT stays 0 outside tests/t_int29_twf.rs) plus the claim-route request at the end.
//! TWF3 (copy of TWF2's port with mutation switches and the new ports in the second half). Original header: TWF2: Time_Warp_RGB's frame-index writer, the checkout request and limit, and both frame-range paths, ported as read from the
//! neutral_1 listing (addresses are listing addresses). The emulated original is the judge (t2/emu.tsv, tests/t2.rs).
//! Labels: every function names its listing range; "INFERRED" marks a reading that no emulator block covers.
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// TWF3 mutation switch: 0 = the port as TWF2 wrote it. Nonzero ids select one deliberate mutation (list in MUTANTS.tsv); used only by tests/t3_mut.rs.
pub static MUT: AtomicU32 = AtomicU32::new(0);
#[inline] fn mu(id: u32) -> bool { MUT.load(Ordering::Relaxed) == id }


/// vcvttss2si r32: truncation, NaN and out-of-range give the integer indefinite 0x80000000.
pub fn cvtt_ss(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 { i32::MIN } else { x as i32 }
}
/// vcvttsd2si r32.
pub fn cvtt_sd(x: f64) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x <= -2147483649.0 { i32::MIN } else { x as i32 }
}
/// vcvtss2si r32 (round to nearest even; the operand is already integral after vroundss).
pub fn cvt_ss(x: f32) -> i32 {
    let r = x.round_ties_even();
    if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 { i32::MIN } else { r as i32 }
}
/// maxss/vmaxss semantics: dest = (a > b) ? a : b (second operand on NaN or equal).
fn x86max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }

// ---------------------------------------------------------------- render entry (of)

/// Outputs passed to the writer : xmm0 = F (stored at ctx), xmm1 = clip duration in frames (ctx), xmm2 = time scale / step, r8d = step used.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WriterOut { pub f: f32, pub dur_frames: f32, pub ts_over_step: f32, pub step: i32 }

/// . `kind` is the host id; `f4` is in_data; `eax_in` is eax on entry to 
/// (1 on the fall-through path, used only when the step is not positive; INFERRED for the jump source).
pub fn render_entry_f(current_time: i32, duration: i32, step: i32, time_scale: u32, f4: i32, kind: i32, eax_in: i32) -> WriterOut {
    let dbl = kind == 4 && f4 != 0 && !mu(6);
    let ecx = if dbl { step.wrapping_add(step) } else { step };
    let r8 = if ecx <= 0 { if mu(15) { 0 } else { eax_in } } else if dbl { step.wrapping_add(step) } else { step };
    let d = r8 as f32;
    let ts = if mu(16) { (time_scale as i32 as i64) as f32 } else { (time_scale as u64 as i64) as f32 };
    WriterOut { f: (current_time as f32) / d, dur_frames: (duration as f32) / d, ts_over_step: ts / d, step: r8 }
}

// ---------------------------------------------------------------- checkout 

#[derive(Debug, Clone, Copy)]
pub struct CheckoutIn {
    pub f: i32,            // edx: absolute integer frame asked by the executor loop
    pub big_f: f32,        // ctx
    pub dur_frames: f32,   // ctx
    pub lim_global: i32,   //  (a host-side time cell written from [[owner]]; INFERRED meaning)
    pub step: Option<i32>, // [[ctx]], None when ctx is null
    pub kind: i32,         // 
    pub f4: i32,           // [[ctx]]
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
    pub f: i32,
    pub x: f32,          // (float)f - floor(F) + F
    pub trunc_x: i32,    // eax 
    pub lim: i32,        //  / step
    pub frame: i32,      // ecx after the limit
    pub stored: f32,     // (x - floor(x)) + (float)frame, [rbp]
    pub step: i32,       // step used (doubled when kind 4 and f4 != 0)
    pub cache_key: i32,  // trunc(fma(stored, 2.0, 100.0)) (FMA runs through avxeng in the T2)
    pub host_time: i32,  // edx : trunc((float)step * stored)
}
#[derive(Debug, PartialEq)]
pub enum FrameError { DivideError, MissingHostFrame { f: i32, x_bits: u32 } }

/// The limit's subtrahend, double, read in place: 0.01.
pub const LIMIT_SUB: f64 = 0.01;

pub fn checkout_request(i: CheckoutIn) -> Result<Request, FrameError> {
    let x = if mu(8) { ((i.f as f32) - i.big_f.trunc()) + i.big_f } else { ((i.f as f32) - i.big_f.floor()) + i.big_f };
    let step = match i.step {
        None => 1,
        Some(s) => if i.kind == 4 && i.f4 != 0 && !mu(6) { s.wrapping_add(s) } else { s },
    };
    let lim = if mu(22) {
        if step == 0 { return Err(FrameError::DivideError); } i.lim_global.wrapping_div_euclid(step)
    } else { i.lim_global.checked_div(step).ok_or(FrameError::DivideError)? };
    let eax = if mu(9) { cvtt_ss(x.floor()) } else { cvtt_ss(x) };
    let sub = if mu(7) { 0.0 } else { LIMIT_SUB };
    let c = cvtt_sd((i.dur_frames as f64) - sub);
    let cl = if c >= eax { eax } else { c };
    let frame = if mu(2) { cl } else if mu(12) { if lim >= eax { lim } else { cl } } else if mu(13) { if lim > cl { lim } else { cl } } else if lim > eax { lim } else { cl };
    let fx = if mu(23) { x.trunc() } else { x.floor() };
    let stored = if mu(3) { x } else if mu(14) { (x - fx) + (eax as f32) } else { (x - fx) + (frame as f32) };
    let cache_key = if mu(18) { cvtt_ss(stored * 2.0 + 100.0) } else { cvtt_ss(stored.mul_add(2.0, 100.0)) };
    let host_time = if mu(17) { cvt_ss((step as f32) * stored) } else { cvtt_ss((step as f32) * stored) };
    Ok(Request { f: i.f, x, trunc_x: eax, lim, frame, stored, step, cache_key, host_time })
}

/// Host model: frames keyed by (f, x bits) as the plugin computes them; an unanswered request is an error.
#[derive(Default)]
pub struct HostModel<T> { pub frames: HashMap<(i32, u32), T> }
impl<T> HostModel<T> {
    pub fn answer(&self, r: &Request) -> Result<&T, FrameError> {
        self.frames.get(&(r.f, r.x.to_bits())).ok_or(FrameError::MissingHostFrame { f: r.f, x_bits: r.x.to_bits() })
    }
    /// Keyed by the time the plugin really hands to the host call : (frame after the limit, stored time bits). Equal to `answer`'s key
    /// whenever the limit leaves the frame at trunc(x) and x >= 0; differs where the limit moves the frame (T2 block C: 931 + 362 of 1620 cases).
    pub fn answer_stored(&self, r: &Request) -> Result<&T, FrameError> {
        self.frames.get(&(r.frame, r.stored.to_bits())).ok_or(FrameError::MissingHostFrame { f: r.frame, x_bits: r.stored.to_bits() })
    }
}

// ---------------------------------------------------------------- get_frame executor frame range 

/// Accumulator initial values, read in place: = 0x7f7fffff (FLT_MAX), min; = 0x00800000 (FLT_MIN), max.
pub const MIN_INIT: f32 = f32::MAX;
pub const MAX_INIT: f32 = f32::MIN_POSITIVE;

/// lo = floor(max(0,min)), hi = (round_up ? ceil : floor)(max(0,max)); vcvtss2si.
pub fn range_tail(min: f32, max: f32, round_up: bool) -> (i32, i32) {
    let round_up = round_up && !mu(4);
    let lo = if mu(10) { cvt_ss(min.floor()) } else { cvt_ss(x86max(0.0, min).floor()) };
    let m = x86max(0.0, max);
    let hi = cvt_ss(if round_up != mu(11) { m.ceil() } else { m.floor() });
    (lo, hi)
}
/// Scalar path (r9d != 0): min = max = t, jump to directly.
pub fn frame_range_scalar(t: f32, round_up: bool) -> (i32, i32) { if mu(21) { let q = lane_trunc(t); range_tail(q, q, round_up) } else { range_tail(t, t, round_up) } }
/// One lane through (and the peel/body loops): vcvttss2si then vcvtsi2ss.
pub fn lane_trunc(x: f32) -> f32 { if mu(5) { x } else { cvtt_ss(x) as f32 } }
/// Lane path (r9d == 0): n = w*h (32-bit imul); n <= 0 goes straight to the tail with the initial values; otherwise min/max over the n truncated lanes
/// folded onto the initial values. The five code routes (scalar, 16..56 vector, aligned and peeled vector with 8-lane and scalar tails) all
/// reduce the same multiset because every lane is an integer-valued float after the conversion (T2 checks 1831 cases across the routes).
pub fn frame_range_lanes(w: i32, h: i32, lanes: &[f32], round_up: bool) -> (i32, i32) {
    let n = if mu(19) { (w as i64 * h as i64).clamp(-1, lanes.len() as i64) as i32 } else { w.wrapping_mul(h) };
    let (mut mn, mut mx) = (if mu(20) { f32::INFINITY } else { MIN_INIT }, if mu(1) { 0.0 } else { MAX_INIT });
    if n > 0 {
        for &v in &lanes[..n as usize] {
            let t = lane_trunc(v);
            mn = if mn > t { t } else { mn };
            mx = if t > mx { t } else { mx };
        }
    }
    range_tail(mn, mx, round_up)
}

// ================================================================ TWF3 additions

// ---------------------------------------------------------------- the limit cell and its record history 

/// K, the double, read in place: 0x3ffe666666666666 = 1.9 (not exactly representable).
pub const K_WINDOW: f64 = 1.9;
/// One of the two records in the owner (listing: owner and owner, a fixed stride). Fields as the writers use them:
/// the two settings words (compared by), the keep flag byte, `last` (time of the previous render), `anchor` (the cell's value).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rec { pub settings_a: i16, pub settings_b: i16, pub keep_flag: u8, pub last: i32, pub anchor: i32 }
/// Owner history: the two records, bit 0 of the byte owner, and the selected record ([owner]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OwnerHist { pub rec: [Rec; 2], pub bit0: bool, pub cur: usize }
impl OwnerHist {
    /// A fresh owner: constructor zeroes the bytes, then stores 0xfff0bdc0 (-1,000,000) at owner and
    /// (record `last`) and owner into [owner]. `anchor`, the settings words and the keep flag byte stay 0. MEASURED by reading.
    pub fn fresh() -> Self {
        let r = Rec { settings_a: 0, settings_b: 0, keep_flag: 0, last: -1_000_000, anchor: 0 };
        OwnerHist { rec: [r, r], bit0: false, cur: 0 }
    }
    /// The reset function (owner words 0 and 1 and the owner flag byte cleared, `last` = -1,000,000; `anchor` is NOT touched; the settings words are not touched).
    pub fn reset_records(&mut self) { for r in self.rec.iter_mut() { r.last = -1_000_000; } }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Arith { Fma, MulAdd }
/// Inputs of the render-entry writer (function AVX / SSE twin; the twin uses mulsd+addsd where the AVX body uses vfmadd231sd).
#[derive(Clone, Copy, Debug)]
pub struct EntryIn {
    pub host_id: i32,        // 
    pub cur_time: i32,       // in_data
    pub step: i32,           // in_data
    pub entry_in_a: i32, pub entry_in_b: i32, pub entry_in_c: i32, pub entry_in_d: i32,
    pub entry_in_e: i32, pub entry_in_f: i32, pub entry_in_g: i32, pub entry_in_h: i32,
    pub host_arg_value: i32,        // [r8 argument] (host id 2 test)
    pub layer_empty: bool,   // [[ctx]] == 0 or [[ctx]] == 0
    pub flag_cell_a_4: bool,     // [[owner]] bit 2 (value 4)
    pub settings: (i32, i32),// [[ctx]] and +4, compared with the record words by 
    pub arith: Arith,
}
/// 0 when edi == sign-extended first settings word and esi == sign-extended second settings word, else 1.
pub fn settings_differ(rec: &Rec, s: (i32, i32)) -> bool { !(s.0 == rec.settings_a as i32 && s.1 == rec.settings_b as i32) }
/// lim = (float)(step * 1.9 + last) (FMA in the AVX body, mulsd+addsd in the SSE twin); the run continues when last < cur and (float)cur < lim.
pub fn limit_window(cur: i32, last: i32, step: i32, arith: Arith) -> (bool, f32) {
    let lim: f32 = match arith {
        Arith::Fma => (step as f64).mul_add(K_WINDOW, last as f64) as f32,
        Arith::MulAdd => ((step as f64) * K_WINDOW + last as f64) as f32,
    };
    (cur > last && !((cur as f32) >= lim), lim)
}
/// Result of one entry: Some(cell value written to) or None when the early-out path is taken (no cell write, no F write).
pub fn render_entry_limit(h: &mut OwnerHist, i: &EntryIn) -> Option<i32> {
    // 
    if i.entry_in_a == 0 && i.entry_in_c == 0 && i.entry_in_b == 0 && i.entry_in_d == 0 && i.layer_empty { return None; }
    //  (bit 0 of owner)
    h.bit0 = match i.host_id {
        2 => (i.host_arg_value as i64 - 128).abs() < 10,
        9 => i.entry_in_e == 1 && (i.entry_in_f & !8) == 5 && i.entry_in_g == 1 && (i.entry_in_h & !8) == 5,
        _ => false,
    };
    if !i.flag_cell_a_4 { return Some(0); } // 
    h.cur = if h.bit0 { 1 } else { 0 };
    let r = &mut h.rec[h.cur];
    let (in_window, _lim) = limit_window(i.cur_time, r.last, i.step, i.arith);
    let keep = in_window && !settings_differ(r, i.settings);
    if !keep { r.keep_flag = 0; r.anchor = i.cur_time; }
    r.last = i.cur_time;
    Some(r.anchor)
}
/// The two other record writers (same record, they do NOT write the cell): ebx = trunc of a float local.
pub fn render_entry_seq(h: &mut OwnerHist, ebx: i32, settings: (i32, i32), flag_cell_a_4: bool) {
    if !flag_cell_a_4 { return; }
    h.cur = 0;
    let r = &mut h.rec[0];
    let eax = ebx.wrapping_sub(r.last);
    let keep = ebx != 0 && eax == 1 && !settings_differ(r, settings);
    if !keep { r.keep_flag = 0; r.anchor = ebx; }
    r.last = ebx;
}

// ---------------------------------------------------------------- the frame cache (entries + k * the entry size)

/// Program flag text "cache_alt_frames" (read in place) sets bit 0x80 of [[owner]] .
#[derive(Clone, Debug, Default)]
pub struct FrameCache { pub entries: Vec<(bool, i32, i64)>, pub count: usize, pub cur: Option<usize> }
impl FrameCache {
    /// Lookup then miss handling : with flag80 a new zeroed entry is appended at index count and count += 1;
    /// without it count never changes here (the loop bound is count), so with count == 0 there is no hit at all. Returns Some(value) on a hit.
    pub fn checkout(&mut self, key: i32, value: i64, flag80: bool) -> Option<i64> {
        for k in 0..self.count.min(self.entries.len()) {
            if self.entries[k].0 && self.entries[k].1 == key { self.cur = Some(k); return Some(self.entries[k].2); }
        }
        if flag80 {
            if self.entries.len() <= self.count { self.entries.resize(self.count + 1, (false, 0, 0)); }
            self.entries[self.count] = (true, key, value);
            self.cur = Some(self.count);
            self.count += 1;
        }
        None
    }
    /// End of render (and the twin): runs only when [ctx] == 0; count is zeroed on both flag80 paths and the
    /// entries are released (flag80: all valid ones; else the current one). With [ctx] != 0 nothing is reset and the entries persist into the next render.
    pub fn end_of_render(&mut self, ctx_keep_zero: bool, flag80: bool) {
        if !ctx_keep_zero { return; }
        if flag80 { for e in self.entries.iter_mut() { e.0 = false; } } else if let Some(k) = self.cur { if k < self.entries.len() { self.entries[k].0 = false; } }
        self.count = 0;
    }
}

// ---------------------------------------------------------------- INT29 step TW: the claim-route request (W559 (a), (c))

/// The per-run request inputs of the get_frame route, every one from the run's record (no default; Batch 69 rule 1).
/// `big_f` is F (the --host-frame value), `dur_frames` the clip duration in frames, `limit_cell` the host limit cell taken per run from
/// the record (LIMIT-CELL-BY-RECORD), `step` local_time_step, `kind` the host id, `f4` the step struct flag, `r8` and `r9` the two
/// frame-range selectors (r9 != 0 is the scalar path).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwRoute { pub big_f: f32, pub dur_frames: f32, pub limit_cell: i32, pub step: i32, pub kind: i32, pub f4: i32, pub r8: i32, pub r9: i32 }

/// One host request as the plugin makes it: the frame after the limit, the stored time and the host call argument trunc(step x stored).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwRequest { pub frame: i32, pub stored: f32, pub arg: i32 }

/// The request for one get_frame offset t (float32(F + shift) as folded): the scalar frame range, then the checkout request.
/// Refusals are named: the lane path is not on the recorded route (r9 = 0), a range of more than one frame, a zero step.
pub fn route_request(q: &TwRoute, t: f32) -> Result<TwRequest, String> {
    if q.r9 == 0 {
        return Err("FRAME-RANGE-LANE-PATH-NOT-ON-ROUTE (r9d = 0: the lane path of the frame range is ported but the recorded route is the scalar path; TWS1, W559)".into());
    }
    let (lo, hi) = frame_range_scalar(t, q.r8 != 0);
    if lo != hi {
        return Err(format!("FRAME-RANGE-MULTI-UNRECORDED (frames {} to {} for one request; the recorded route asks one frame per offset; TWS1, W559)", lo, hi));
    }
    let r = checkout_request(CheckoutIn { f: lo, big_f: q.big_f, dur_frames: q.dur_frames, lim_global: q.limit_cell, step: Some(q.step), kind: q.kind, f4: q.f4 })
        .map_err(|e| format!("FRAME-REQUEST-REFUSED ({:?}; step {})", e, q.step))?;
    Ok(TwRequest { frame: r.frame, stored: r.stored, arg: r.host_time })
}

#[cfg(test)]
mod int29_tests {
    use super::*;
    fn q(f: f32, step: i32) -> TwRoute { TwRoute { big_f: f, dur_frames: 240.0, limit_cell: 0, step, kind: 0, f4: 0, r8: 0, r9: 1 } }
    /// REQUEST-LIST (TWS1 RECORDS_W574 F25, MEASURED keylog): F 2.5 then 3.5 at step 2 with shifts +1, 0, -1 ask frames 3,2,1 and 4,3,2
    /// with arguments 7,5,3 and 9,7,5.
    #[test]
    fn request_list_f25() {
        for (f, frames, args) in [(2.5f32, [3, 2, 1], [7, 5, 3]), (3.5f32, [4, 3, 2], [9, 7, 5])] {
            for (k, sh) in [1.0f32, 0.0, -1.0].iter().enumerate() {
                let r = route_request(&q(f, 2), f + sh).unwrap();
                assert_eq!((r.frame, r.arg), (frames[k], args[k]));
                assert_eq!(r.stored, 0.5 + frames[k] as f32);
            }
        }
    }
    /// The request key is (frame, stored), not (f, x): with a limit cell above the frame the frame moves and the stored time follows it.
    #[test]
    fn request_key_discriminates_limit() {
        let mut a = q(2.5, 2);
        a.limit_cell = 20; // 20 / 2 = 10 > trunc(x) = 3: the lower bound moves the frame
        let r = route_request(&a, 3.5).unwrap();
        assert_eq!((r.frame, r.stored, r.arg), (10, 10.5, 21));
        let r0 = route_request(&q(2.5, 2), 3.5).unwrap();
        assert_ne!((r.frame, r.stored.to_bits()), (r0.frame, r0.stored.to_bits()));
    }
    #[test]
    fn refusals_named() {
        let mut a = q(1.0, 1);
        a.r9 = 0;
        assert!(route_request(&a, 1.0).unwrap_err().starts_with("FRAME-RANGE-LANE-PATH-NOT-ON-ROUTE"));
        let mut b = q(1.0, 1);
        b.r8 = 1;
        assert!(route_request(&b, 1.5).unwrap_err().starts_with("FRAME-RANGE-MULTI-UNRECORDED"));
        assert!(route_request(&q(1.0, 0), 1.0).unwrap_err().starts_with("FRAME-REQUEST-REFUSED"));
    }
}
