//! copy (DSW1): native port of slot 52 of builtin class `copy` (vtable, slot function, 128 bytes).
//!
//! The slot function is a driver loop: `n = vcall[this]` (the class count getter, virtual slot 31); for ch in 0..n it calls the
//! host plane service (finished crate `plane_service`) as service(this, mode 0, channel ch, destination = outs[ch], info), re-reading
//! the count getter after every call (the loop bound is the getter's current value). The function itself touches no pixel; every sample
//! is written by the plane service, which copies argument 0's channel min(ch, count-1) (flat copy) or fills a constant.
//! std only, no unsafe. Mutation: `BUILD_MUTANT` is a compile-time constant, one source line per mutant build.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutant {
    None,
    /// `jle` replaced by `jl`: a count of 0 still runs one iteration
    ZeroRunsOnce,
    /// loop condition `jl` replaced by `jle`: one extra iteration
    OneExtra,
    /// the loop starts at channel 1
    StartAtOne,
    /// the mode argument is 1 (argument block 1)
    ModeOne,
    /// mode and channel swapped
    ModeChanSwapped,
    /// every call writes outs[0]
    DestAlwaysFirst,
    /// the last iteration is skipped
    SkipLast,
    /// the destination is outs[ch + 1]
    DestShifted,
    /// the count is clamped to 4
    ClampFour,
    /// the channels run downward
    Reversed,
    /// the channel argument is ch + 1
    ChanPlusOne,
    /// negative counts are treated as their absolute value
    AbsCount,
}

/// DSW1: the mutant compiled into this build. Base build: None. A mutant build differs by this one line only.
pub const BUILD_MUTANT: Mutant = Mutant::None;

/// One call of the plane service: argument block `mode`, channel `chan`, destination plane `out` (index into the outs array of the caller).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PadCall {
    pub mode: i32,
    pub chan: i32,
    pub out: i32,
}

/// The calls the slot function makes for a class count getter returning `nout`.
#[inline(always)]
pub fn calls(nout: i32, m: Mutant) -> Vec<PadCall> {
    let mut v = Vec::new();
    let mut n = nout;
    if m == Mutant::AbsCount {
        n = n.wrapping_abs();
    }
    if m == Mutant::ClampFour {
        n = n.min(4);
    }
    // test eax, eax; jle exit
    let skip = if m == Mutant::ZeroRunsOnce { n < 0 } else { n <= 0 };
    if skip {
        return v;
    }
    let mut chans: Vec<i32> = Vec::new();
    // do { call; ch++ } while (ch < count)
    let mut ch: i32 = if m == Mutant::StartAtOne { 1 } else { 0 };
    loop {
        chans.push(ch);
        ch += 1;
        let go = if m == Mutant::OneExtra { ch <= n } else { ch < n };
        if !go {
            break;
        }
    }
    if m == Mutant::SkipLast {
        chans.pop();
    }
    if m == Mutant::Reversed {
        chans.reverse();
    }
    for ch in chans {
        let mode = match m {
            Mutant::ModeOne => 1,
            Mutant::ModeChanSwapped => ch,
            _ => 0,
        };
        let chan = match m {
            Mutant::ModeChanSwapped => 0,
            Mutant::ChanPlusOne => ch + 1,
            _ => ch,
        };
        let out = match m {
            Mutant::DestAlwaysFirst => 0,
            Mutant::DestShifted => ch + 1,
            _ => ch,
        };
        v.push(PadCall { mode, chan, out });
    }
    v
}

/// The slot function for the executor: the calls for `nout` with the build's mutant (None in every shipped build).
pub fn slot52_calls(nout: i32) -> Vec<PadCall> {
    calls(nout, BUILD_MUTANT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts() {
        assert!(slot52_calls(0).is_empty());
        assert!(slot52_calls(-1).is_empty());
        assert!(slot52_calls(i32::MIN).is_empty());
        assert_eq!(slot52_calls(1), vec![PadCall { mode: 0, chan: 0, out: 0 }]);
        let c = slot52_calls(3);
        assert_eq!(c.iter().map(|p| (p.mode, p.chan, p.out)).collect::<Vec<_>>(), vec![(0, 0, 0), (0, 1, 1), (0, 2, 2)]);
        assert_eq!(slot52_calls(4).len(), 4);
    }

    #[test]
    fn mutants_change_the_record() {
        for m in [Mutant::ZeroRunsOnce, Mutant::OneExtra, Mutant::StartAtOne, Mutant::ModeOne, Mutant::ModeChanSwapped, Mutant::DestAlwaysFirst, Mutant::SkipLast,
            Mutant::DestShifted, Mutant::ClampFour, Mutant::Reversed, Mutant::ChanPlusOne, Mutant::AbsCount] {
            let differs = (-3..=8).any(|n| calls(n, m) != calls(n, Mutant::None));
            assert!(differs, "{:?}", m);
        }
    }
}
