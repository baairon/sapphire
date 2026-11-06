//! element_of(A, idx): a channel gather. Slot 52 (the class overrides the dispatcher; EG: slot 52), AVX2 body .
//! Operand A (image or constant) has `count` channels (1..4); the index operand is read from its CONSTANT lanes at this (the flag at is never read).
//! For each output channel ch < nout (virtual slot 31, re-read every iteration): k = cvttss2si(idx[ch]) folded into 0..count-1 (count 2: & 1, count 3: signed
//! remainder fixed to 0..2, count 4: & 3), then out[ch] = A.consts[k] filled over n elements when A is a constant, else the plane service copies plane k unless
//! the output buffer already is plane k. count 1: tail jump to the plane service (mode 0, chan 0, outs[0]).
use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMutant {
    None,
    IdxRound,
    NanZero,
    C3And3,
    C2And3,
    C3NoFix,
    ConstIdx0,
    AliasIgnore,
    SvcChan0,
    NoutCap3,
    FillTail,
    ConstAsImage,
    IdxShift,
    FillNMinus1,
}

impl EMutant {
    pub fn from_name(s: &str) -> Option<EMutant> {
        Some(match s {
            "idx_round" => EMutant::IdxRound,
            "nan_zero" => EMutant::NanZero,
            "c3_and3" => EMutant::C3And3,
            "c2_and3" => EMutant::C2And3,
            "c3_nofix" => EMutant::C3NoFix,
            "const_idx0" => EMutant::ConstIdx0,
            "alias_ignore" => EMutant::AliasIgnore,
            "svc_chan0" => EMutant::SvcChan0,
            "nout_cap3" => EMutant::NoutCap3,
            "fill_tail" => EMutant::FillTail,
            "const_as_image" => EMutant::ConstAsImage,
            "idx_shift" => EMutant::IdxShift,
            "fill_n_minus1" => EMutant::FillNMinus1,
            _ => return None,
        })
    }
    pub const NAMES: [&'static str; 13] = [
        "idx_round", "nan_zero", "c3_and3", "c2_and3", "c3_nofix", "const_idx0", "alias_ignore", "svc_chan0", "nout_cap3", "fill_tail", "const_as_image", "idx_shift",
        "fill_n_minus1",
    ];
}

/// `count` = [this], `nout` = virtual slot 31 (explicit, EM1-A6), `n` = [info], `a_const` = byte [this], `a_consts` = [this],
/// `idx` = [this] as floats. No defaults.
#[derive(Clone, Copy, Debug)]
pub struct EParams {
    pub count: i32,
    pub nout: i32,
    pub n: i32,
    pub a_const: bool,
    pub a_consts: [f32; 4],
    pub idx: [f32; 4],
}

fn fold(count: i32, raw: i32, m: EMutant) -> i32 {
    match count {
        2 => {
            if m == EMutant::C2And3 {
                raw & 3
            } else {
                raw & 1
            }
        }
        3 => {
            if m == EMutant::C3And3 {
                return raw & 3;
            }
            let r = rem3_raw(raw);
            if r < 0 && m != EMutant::C3NoFix {
                r + 3
            } else {
                r
            }
        }
        _ => raw & 3,
    }
}

/// `planes`: A's image planes (owned: an output buffer may alias one of them), `outs`: the output buffers, `dst_alias[ch] = k >= 0` means outs[ch] IS A.planes[k]
/// (the original compares the pointers); in that case the writes go to planes[k] and `outs[ch]` is not touched.
pub fn render_m(p: &EParams, planes: &mut Vec<Vec<f32>>, svc: &Svc, outs: &mut [Vec<f32>], dst_alias: [i32; 4], m: EMutant) -> Result<Trace, ElError> {
    let mut tr = Trace::default();
    if !(1..=4).contains(&p.count) {
        return Err(ElError::BadCount(p.count));
    }
    if p.count == 1 {
        let refs: Vec<&[f32]> = planes.iter().map(|v| v.as_slice()).collect();
        call_service(svc, 1, &refs, 0, &mut outs[0], p.n, &mut tr)?;
        return Ok(tr);
    }
    let nout = if m == EMutant::NoutCap3 { p.nout.min(3) } else { p.nout };
    if nout <= 0 {
        return Ok(tr);
    }
    if nout > 4 {
        return Err(ElError::BadNout(nout));
    }
    let a_const = p.a_const && m != EMutant::ConstAsImage;
    for ch in 0..nout as usize {
        let ich = if m == EMutant::IdxShift { (ch + 1) % 4 } else { ch };
        let raw = if m == EMutant::IdxRound {
            let x = p.idx[ich];
            if x.is_nan() { i32::MIN } else { x.round() as i32 }
        } else if m == EMutant::NanZero && p.idx[ich].is_nan() {
            0
        } else {
            cvtt(p.idx[ich])
        };
        let k = fold(p.count, raw, m);
        let alias = dst_alias[ch];
        if a_const {
            let k = if m == EMutant::ConstIdx0 { 0 } else { k as usize };
            let bits = p.a_consts[k].to_bits();
            let dst: &mut Vec<f32> = if alias >= 0 { &mut planes[alias as usize] } else { &mut outs[ch] };
            if p.n > 0 {
                let mut cnt = p.n as usize;
                if m == EMutant::FillNMinus1 {
                    cnt -= 1;
                }
                if m == EMutant::FillTail && cnt > 16 {
                    cnt -= 1;
                }
                if dst.len() < cnt {
                    return Err(ElError::DstShort);
                }
                for d in dst[..cnt].iter_mut() {
                    *d = f32::from_bits(bits);
                }
            }
        } else {
            if p.count as usize > planes.len() {
                return Err(ElError::PlaneMissing);
            }
            if alias == k && m != EMutant::AliasIgnore {
                continue;
            }
            let chan = if m == EMutant::SvcChan0 { 0 } else { k };
            if alias >= 0 {
                let mut taken = std::mem::take(&mut planes[alias as usize]);
                let r = {
                    let refs: Vec<&[f32]> = planes.iter().map(|v| v.as_slice()).collect();
                    call_service(svc, p.count, &refs, chan, &mut taken, p.n, &mut tr)
                };
                planes[alias as usize] = taken;
                r?;
            } else {
                let refs: Vec<&[f32]> = planes.iter().map(|v| v.as_slice()).collect();
                call_service(svc, p.count, &refs, chan, &mut outs[ch], p.n, &mut tr)?;
            }
        }
    }
    Ok(tr)
}

pub fn render(p: &EParams, planes: &mut Vec<Vec<f32>>, svc: &Svc, outs: &mut [Vec<f32>], dst_alias: [i32; 4]) -> Result<Trace, ElError> {
    render_m(p, planes, svc, outs, dst_alias, EMutant::None)
}
