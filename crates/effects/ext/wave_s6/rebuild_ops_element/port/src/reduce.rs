//! The channel reduction shared by max_element (vmaxss/vmaxps) and min_element (vminss/vminps): AVX2 bodies (their normalised
//! listings differ only in the mnemonic). out[i] = f(f(f(c0[i], c1[i]), c2[i]), c3[i]) over the `count` channels of ONE operand, per element;
//! count 1 is a tail jump to the plane service (mode 0, chan 0, dst = outs[0]).
use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Max,
    Min,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMutant {
    None,
    SwapAb,
    OtherOp,
    RustMax,
    GeEq,
    C4As3,
    C3As2,
    TailShort,
    HeadSkip,
    NoSvc,
    SvcChan1,
    NCap28,
    ChainRev,
}

impl RMutant {
    pub fn from_name(s: &str) -> Option<RMutant> {
        Some(match s {
            "swap_ab" => RMutant::SwapAb,
            "other_op" => RMutant::OtherOp,
            "rust_max" => RMutant::RustMax,
            "ge_eq" => RMutant::GeEq,
            "c4_as_3" => RMutant::C4As3,
            "c3_as_2" => RMutant::C3As2,
            "tail_short" => RMutant::TailShort,
            "head_skip" => RMutant::HeadSkip,
            "no_svc" => RMutant::NoSvc,
            "svc_chan1" => RMutant::SvcChan1,
            "n_cap28" => RMutant::NCap28,
            "chain_rev" => RMutant::ChainRev,
            _ => return None,
        })
    }
    pub const NAMES: [&'static str; 12] =
        ["swap_ab", "other_op", "rust_max", "ge_eq", "c4_as_3", "c3_as_2", "tail_short", "head_skip", "no_svc", "svc_chan1", "n_cap28", "chain_rev"];
}

/// `count` = [this] (operand channel count), `n` = [info]. No defaults.
#[derive(Clone, Copy, Debug)]
pub struct RParams {
    pub count: i32,
    pub n: i32,
}

fn f(kind: Kind, m: RMutant, a: f32, b: f32) -> f32 {
    let (a, b) = if m == RMutant::SwapAb { (b, a) } else { (a, b) };
    let k = match (kind, m) {
        (Kind::Max, RMutant::OtherOp) => Kind::Min,
        (Kind::Min, RMutant::OtherOp) => Kind::Max,
        (k, _) => k,
    };
    match (k, m) {
        (Kind::Max, RMutant::RustMax) => a.max(b),
        (Kind::Min, RMutant::RustMax) => a.min(b),
        (Kind::Max, RMutant::GeEq) => {
            if a >= b {
                a
            } else {
                b
            }
        }
        (Kind::Min, RMutant::GeEq) => {
            if a <= b {
                a
            } else {
                b
            }
        }
        (Kind::Max, _) => x86_max(a, b),
        (Kind::Min, _) => x86_min(a, b),
    }
}

pub fn render_m(kind: Kind, p: &RParams, planes: &[&[f32]], svc: &Svc, dst: &mut [f32], m: RMutant) -> Result<Trace, ElError> {
    let mut tr = Trace::default();
    let mut count = p.count;
    if !(1..=4).contains(&count) {
        return Err(ElError::BadCount(count));
    }
    if count == 1 {
        if m != RMutant::NoSvc {
            let chan = if m == RMutant::SvcChan1 { 1 } else { 0 };
            call_service(svc, 1, planes, chan, dst, p.n, &mut tr)?;
        }
        return Ok(tr);
    }
    if m == RMutant::C4As3 && count == 4 {
        count = 3;
    }
    if m == RMutant::C3As2 && count == 3 {
        count = 2;
    }
    let mut n = p.n;
    if n <= 0 {
        return Ok(tr);
    }
    if m == RMutant::NCap28 && n > 28 {
        n = 28;
    }
    let nn = n as usize;
    if planes.len() < count as usize {
        return Err(ElError::PlaneMissing);
    }
    if planes[..count as usize].iter().any(|pl| pl.len() < nn) {
        return Err(ElError::PlaneShort);
    }
    if dst.len() < nn {
        return Err(ElError::DstShort);
    }
    let (lo, hi) = (if m == RMutant::HeadSkip { 1 } else { 0 }, if m == RMutant::TailShort { nn - 1 } else { nn });
    for i in lo..hi {
        let mut acc = planes[0][i];
        if m == RMutant::ChainRev {
            let mut r = planes[count as usize - 1][i];
            for c in (0..count as usize - 1).rev() {
                r = f(kind, m, planes[c][i], r);
            }
            dst[i] = r;
            continue;
        }
        for c in 1..count as usize {
            acc = f(kind, m, acc, planes[c][i]);
        }
        dst[i] = acc;
    }
    Ok(tr)
}
