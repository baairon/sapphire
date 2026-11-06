//! the render-history rule of the Trails style, as rules, formats and constants.
//!
//! A frame record holds the previous render's time, the restart time, the saved size words and the allocation state.
//! A render KEEPS the history when the program flag bit is set, the previous time is below the current time, the current time
//! (as f32) is below the window limit (step * K + previous time, one fused multiply-add rounded to f32) and the saved size
//! words equal the current frame size; otherwise it RESTARTS. Each render ends with a save that writes the size words.
//! Mutation switches (cargo features) exist for the unit tests only.
//! K = 1.9 is a rule constant (MEASURED-DYLIB); the limit is the AVX tier's fused form
//! (VARIANT-INERT-ON-ROUTE); depth, allocator cell, program flag and host id are required per-run record inputs of fx_run's
//! history driver (no defaults). Code addresses are cited in PATCH.md and MERGE_LOG.tsv, not here.

/// K of the window limit, the double 0x3ffe666666666666 (1.9, not exactly representable). MEASURED-DYLIB by.
pub const WINDOW_K: f64 = 1.9;
/// The record's `last` value of a fresh owner: -1,000,000.
pub const FRESH_LAST: i32 = -1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Keep,
    Restart,
}

/// The frame record. `last_t` is the previous render's time, `restart_t` the time of the last restart, `w4`/`w6` the saved size
/// words, `alloc_w`/`alloc_h` the allocation's size words, `depth` the allocated depth, `flag9` the byte set after each save.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameRecord {
    pub last_t: i32,
    pub restart_t: i32,
    pub w4: i16,
    pub w6: i16,
    pub alloc_w: i16,
    pub alloc_h: i16,
    pub depth: u8,
    pub buffer: bool,
    pub flag9: u8,
}

impl FrameRecord {
    /// A fresh owner: last time -1,000,000, everything else zero.
    pub fn fresh() -> Self {
        FrameRecord { last_t: FRESH_LAST, restart_t: 0, w4: 0, w6: 0, alloc_w: 0, alloc_h: 0, depth: 0, buffer: false, flag9: 0 }
    }

    /// The window limit as f32: fma(step, K, last_t) rounded once to f32 (the fused form of the AVX body; the separate
    /// multiply and add of the SSE twin differs only for steps near 2^24, and is not used on the claim host).
    pub fn window_limit(last_t: i32, step: i32) -> f32 {
        // the unfused-limit mutant (separate multiply and add, the SSE twin's form) exists only to be killed
        // by fused_limit_discriminates; never in a shipped build.
        if cfg!(feature = "hpt1_unfused_mut") {
            return ((step as f64) * WINDOW_K + last_t as f64) as f32;
        }
        (step as f64).mul_add(WINDOW_K, last_t as f64) as f32
    }

    /// The decision for a render at time `t`, step `step`, frame size `size` (width, height) with the program flag bit.
    pub fn decide(&self, flag_bit: bool, t: i32, step: i32, size: (i32, i32)) -> Decision {
        #[cfg(feature = "hpt1_always_keep_mut")]
        {
            let _ = (flag_bit, t, step, size);
            return Decision::Keep;
        }
        #[cfg(feature = "hpt1_always_restart_mut")]
        {
            let _ = (flag_bit, t, step, size);
            return Decision::Restart;
        }
        #[allow(unreachable_code)]
        {
            let lim = Self::window_limit(self.last_t, step);
            let size_equal = size.0 == self.w4 as i32 && size.1 == self.w6 as i32;
            if flag_bit && self.last_t < t && !((t as f32) >= lim) && size_equal {
                Decision::Keep
            } else {
                Decision::Restart
            }
        }
    }

    /// Start of a render: decides, then applies the record update. With the flag bit clear the record is untouched and the
    /// render restarts with no history. A restart clears the flag byte and sets the restart time; both set the last time.
    pub fn begin(&mut self, flag_bit: bool, t: i32, step: i32, size: (i32, i32)) -> Decision {
        if !flag_bit {
            return Decision::Restart;
        }
        let d = self.decide(true, t, step, size);
        if d == Decision::Restart {
            self.flag9 = 0;
            self.restart_t = t;
        }
        self.last_t = t;
        d
    }

    /// End of a render: the save. `alloc_present` is the allocator cell being nonzero (zero is the free path: the allocation
    /// words and buffer are cleared and the size words are not written). Otherwise the size words take the low 16 bits of the
    /// frame size, and the allocation is redone when the area grows (signed word product), the depth max(3, n) changes, or
    /// no buffer exists, which also clears the flag byte; the wrapper then sets the flag byte to 1.
    pub fn save(&mut self, alloc_present: bool, size: (i32, i32), n: i32) {
        if !alloc_present {
            self.buffer = false;
            self.alloc_w = 0;
            self.alloc_h = 0;
        } else {
            self.w4 = size.0 as i16;
            self.w6 = size.1 as i16;
            let depth = n.max(3) as u8;
            let old_area = (self.alloc_w as i32).wrapping_mul(self.alloc_h as i32);
            let new_area = size.0.wrapping_mul(size.1);
            let realloc = old_area < new_area || self.depth != depth || !self.buffer;
            if realloc {
                self.alloc_w = size.0 as i16;
                self.alloc_h = size.1 as i16;
                self.depth = depth;
                self.buffer = true;
                self.flag9 = 0;
            }
        }
        self.flag9 = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FLAG: bool = true;

    fn after_first(size: (i32, i32), t: i32, step: i32) -> FrameRecord {
        let mut r = FrameRecord::fresh();
        let d = r.begin(FLAG, t, step, size);
        assert_eq!(d, Decision::Restart);
        r.save(true, size, 3);
        r
    }

    #[test]
    fn first_call_restarts_and_stores_size() {
        let r = after_first((64, 32), 1, 1);
        assert_eq!((r.last_t, r.restart_t, r.w4, r.w6, r.flag9), (1, 1, 64, 32, 1));
    }

    #[test]
    fn keep_at_advance_inside_window() {
        let mut r = after_first((64, 32), 1, 1);
        assert_eq!(r.begin(FLAG, 2, 1, (64, 32)), Decision::Keep);
        assert_eq!((r.last_t, r.restart_t), (2, 1));
        r.save(true, (64, 32), 3);
        assert_eq!(r.begin(FLAG, 3, 1, (64, 32)), Decision::Keep);
    }

    #[test]
    fn equal_time_restarts() {
        let mut r = after_first((64, 32), 1, 1);
        assert_eq!(r.begin(FLAG, 1, 1, (64, 32)), Decision::Restart);
        assert_eq!(r.restart_t, 1);
    }

    #[test]
    fn time_back_restarts() {
        let mut r = after_first((64, 32), 5, 1);
        assert_eq!(r.begin(FLAG, 4, 1, (64, 32)), Decision::Restart);
        assert_eq!((r.last_t, r.restart_t), (4, 4));
    }

    #[test]
    fn beyond_window_restarts() {
        // step 10: limit = 19 + 1 = 20 (fma); t = 19 keeps, t = 20 restarts
        let r = after_first((64, 32), 1, 10);
        assert_eq!(FrameRecord::window_limit(1, 10), 20.0);
        let mut k = r;
        assert_eq!(k.begin(FLAG, 19, 10, (64, 32)), Decision::Keep);
        let mut b = r;
        assert_eq!(b.begin(FLAG, 20, 10, (64, 32)), Decision::Restart);
        let mut b2 = r;
        assert_eq!(b2.begin(FLAG, 1000, 10, (64, 32)), Decision::Restart);
    }

    #[test]
    fn size_change_restarts_then_stores_new_size() {
        let mut r = after_first((64, 32), 1, 1);
        assert_eq!(r.begin(FLAG, 2, 1, (50, 7)), Decision::Restart);
        r.save(true, (50, 7), 3);
        assert_eq!((r.w4, r.w6), (50, 7));
        // shrink: no reallocation, allocation words keep the larger size, flag byte set again
        assert_eq!((r.alloc_w, r.alloc_h), (64, 32));
        assert_eq!(r.begin(FLAG, 3, 1, (50, 7)), Decision::Keep);
    }

    #[test]
    fn grow_and_depth_change_reallocate() {
        let mut r = after_first((50, 7), 1, 1);
        r.save(true, (64, 32), 3);
        assert_eq!((r.alloc_w, r.alloc_h), (64, 32));
        r.save(true, (64, 32), 5);
        assert_eq!(r.depth, 5);
        r.save(true, (64, 32), 1);
        assert_eq!(r.depth, 3);
    }

    #[test]
    fn free_path_when_allocator_cell_is_zero() {
        let mut r = after_first((64, 32), 1, 1);
        r.save(false, (10, 10), 3);
        assert_eq!((r.alloc_w, r.alloc_h, r.buffer), (0, 0, false));
        assert_eq!((r.w4, r.w6), (64, 32), "size words are not written on the free path");
    }

    #[test]
    fn flag_clear_never_keeps_and_leaves_record() {
        let mut r = after_first((64, 32), 1, 1);
        let before = r;
        assert_eq!(r.begin(false, 2, 1, (64, 32)), Decision::Restart);
        assert_eq!(r, before);
        assert_eq!(r.decide(false, 2, 1, (64, 32)), Decision::Restart);
    }

    /// a case where the fused and the unfused limit differ. At step 16777215, last_t
    /// -15099493 the fused limit is 16777215.0, so t = 16777215 restarts; the separate multiply and add gives a limit above t and
    /// would keep. The unfused-limit mutant (feature hpt1_unfused_mut) fails this test.
    #[test]
    fn fused_limit_discriminates() {
        let (step, last, t) = (16_777_215i32, -15_099_493i32, 16_777_215i32);
        let fused = (step as f64).mul_add(WINDOW_K, last as f64) as f32;
        let unfused = ((step as f64) * WINDOW_K + last as f64) as f32;
        assert_eq!(fused, 16_777_215.0);
        assert_ne!(fused, unfused);
        assert_eq!(FrameRecord::window_limit(last, step), 16_777_215.0);
        let mut r = FrameRecord::fresh();
        r.last_t = last;
        r.w4 = 64;
        r.w6 = 32;
        assert_eq!(r.decide(true, t, step, (64, 32)), Decision::Restart);
    }

    /// the flag byte is 1 after every save, the free path included.
    #[test]
    fn flag_byte_after_every_save() {
        let mut r = after_first((64, 32), 1, 1);
        r.flag9 = 0;
        r.save(false, (64, 32), 3);
        assert_eq!(r.flag9, 1);
        r.flag9 = 0;
        r.save(true, (64, 32), 4);
        assert_eq!(r.flag9, 1);
    }

    #[test]
    fn limit_is_fused_and_k_is_double_19() {
        // step 1, last 1: 1.9 + 1 = 2.9 in f32
        assert_eq!(FrameRecord::window_limit(1, 1), 2.9f64 as f32);
        assert!(FrameRecord::window_limit(-1_000_000, 1) < 0.0);
    }
}
