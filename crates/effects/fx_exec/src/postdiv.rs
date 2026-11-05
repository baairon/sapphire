//! INT10 (BCX1 PATCH 2): native port of class postdivide's slot 53 function (vtable), copy_alpha < 0 route.
//!
//! Read from the listing (INT10):
//! - nout = virtual slot 31 (call [rax]) = the combo tag; elnum e = [this]; view = [this]; source plane k = [view+8k];
//!   the alpha plane is the view cell = plane e - 1; copy_alpha = [this]; n = [info] (jle: n <= 0 writes nothing).
//! - copy_alpha >= 0 takes the alpha from another image: `postdiv_into` refuses it (POSTDIV-COPY-ALPHA-SOURCE);
//!   `postdiv_copy_alpha_into` ports the recorded route only (POSTDIV-COPY-ALPHA-PORTED, PDA1): see its doc.
//! - e = 1 or 3 (`and r8d, ~2; cmp 1`) first copies nout planes through and then runs the same arm: refused here
//!   (POSTDIV-ELNUM-FOLDED: the compiler folds postdivide on elnum 1 and 3 to its argument, fold.rs, so the executor never meets it).
//! - per pixel i, with a = alpha[i], eps = = 0x39000000 (2^-13) and one = = 1.0f:
//!   `comiss eps, a; jbe` then `comiss one, a; jbe` (an unordered compare takes jbe: NaN goes to the copy arm)
//!   nout 1: out0 = 0 | p0 / a | p0
//!   nout 2: out0 = 0 | p0 / a | p0;            out1 = a | a | 1.0
//!   nout 3: outk = 0 | pk * (1 / a) | pk (k < 3; the divide first, then the multiply; no alpha store)
//!   nout 4: as nout 3, and out3 = a | a | 1.0
//!   any other nout: nothing is written.
//! The arms for nout 3 and 4 read the view's planes 0, 1, 2 whatever e is; the native requires them to exist.

pub const EPS_BITS: u32 = 0x3900_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdError {
    CopyAlphaSource(i32),
    ElnumFolded(usize),
    Planes(String),
}

impl std::fmt::Display for PdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdError::CopyAlphaSource(c) => write!(f, "POSTDIV-COPY-ALPHA-SOURCE (copy_alpha {} >= 0: the alpha from another image is not ported)", c),
            PdError::ElnumFolded(e) => write!(f, "POSTDIV-ELNUM-FOLDED (elnum {}: the compiler folds postdivide on elnum 1 and 3; the copy route is not ported)", e),
            PdError::Planes(s) => write!(f, "postdivide: {}", s),
        }
    }
}

/// T2 mutants (INT10): each changes one rule of the listing; `None` is the port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdMut {
    None,
    EpsGe,       // eps >= a instead of eps > a
    OneGe,       // one >= a instead of one > a
    DivDirect,   // nout 3 / 4: pk / a instead of pk * (1 / a)
    MulInv2,     // nout 1 / 2: p0 * (1 / a) instead of p0 / a
    ZeroKeepsA0, // the eps arm keeps p0 instead of 0
    AlphaOneEps, // the eps arm stores alpha 1.0
    AlphaKeepCopy, // the copy arm keeps a instead of 1.0
    NoAlpha4,    // nout 4 writes no alpha
    AlphaPlane0, // the alpha is plane 0 instead of plane e - 1
    NanToZero,   // NaN alpha takes the eps arm
    EpsBits,     // eps = 2^-12
    SkipLast,    // the last pixel is not written
    AlphaFromPlane2, // copy-alpha arm: the alpha is colour plane 2 instead of the host alpha plane
    AlphaIgnored,    // copy-alpha arm: the host alpha plane is ignored (alpha 1.0)
    AlphaDenormFlush, // copy-alpha arm: a denormal host alpha is flushed to zero
    AlphaAbs,        // copy-alpha arm: the absolute value of the host alpha
    RouteAnyElnum,   // copy-alpha arm: the route check accepts any copy_alpha >= 0
}

impl PdMut {
    pub fn from_name(s: &str) -> Option<PdMut> {
        Some(match s {
            "EpsGe" => PdMut::EpsGe,
            "OneGe" => PdMut::OneGe,
            "DivDirect" => PdMut::DivDirect,
            "MulInv2" => PdMut::MulInv2,
            "ZeroKeepsA0" => PdMut::ZeroKeepsA0,
            "AlphaOneEps" => PdMut::AlphaOneEps,
            "AlphaKeepCopy" => PdMut::AlphaKeepCopy,
            "NoAlpha4" => PdMut::NoAlpha4,
            "AlphaPlane0" => PdMut::AlphaPlane0,
            "NanToZero" => PdMut::NanToZero,
            "EpsBits" => PdMut::EpsBits,
            "SkipLast" => PdMut::SkipLast,
            "AlphaFromPlane2" => PdMut::AlphaFromPlane2,
            "AlphaIgnored" => PdMut::AlphaIgnored,
            "AlphaDenormFlush" => PdMut::AlphaDenormFlush,
            "AlphaAbs" => PdMut::AlphaAbs,
            "RouteAnyElnum" => PdMut::RouteAnyElnum,
            _ => return None,
        })
    }
}

/// The slot 53 body on bit planes. `src[k]` is the view's plane k (at least max(e, min(nout, 3)) planes of n floats);
/// `out` holds the destination planes (only planes 0..nout are written, and only the first n floats).
pub fn postdiv_into(src: &[&[u32]], elnum: usize, copy_alpha: i32, nout: i32, n: i64, out: &mut [Vec<u32>], m: PdMut) -> Result<(), PdError> {
    if copy_alpha >= 0 {
        return Err(PdError::CopyAlphaSource(copy_alpha));
    }
    if elnum == 1 || elnum == 3 {
        return Err(PdError::ElnumFolded(elnum));
    }
    if !(1..=4).contains(&nout) || n <= 0 {
        return Ok(());
    }
    if elnum == 0 || elnum > 4 {
        return Err(PdError::Planes(format!("elnum {} outside 2 and 4", elnum)));
    }
    let n = n as usize;
    let nc = (nout as usize).min(3);
    let need = elnum.max(nc);
    if src.len() < need || src.iter().take(need).any(|p| p.len() < n) {
        return Err(PdError::Planes(format!("the view holds {} planes; {} of {} floats are read", src.len(), need, n)));
    }
    if out.len() < nout as usize || out.iter().take(nout as usize).any(|p| p.len() < n) {
        return Err(PdError::Planes("destination planes too short".into()));
    }
    let eps = f32::from_bits(if m == PdMut::EpsBits { 0x3980_0000 } else { EPS_BITS });
    let ap = if m == PdMut::AlphaPlane0 { 0 } else { elnum - 1 };
    let last = if m == PdMut::SkipLast { n - 1 } else { n };
    for i in 0..last {
        let ab = src[ap][i];
        let a = f32::from_bits(ab);
        let lt_eps = match m {
            PdMut::EpsGe => eps >= a,
            PdMut::NanToZero => eps > a || a.is_nan(),
            _ => eps > a,
        };
        let lt_one = !lt_eps && (if m == PdMut::OneGe { 1.0f32 >= a } else { 1.0f32 > a });
        if nout <= 2 {
            out[0][i] = if lt_eps {
                if m == PdMut::ZeroKeepsA0 { src[0][i] } else { 0 }
            } else if lt_one {
                if m == PdMut::MulInv2 { (f32::from_bits(src[0][i]) * (1.0f32 / a)).to_bits() } else { (f32::from_bits(src[0][i]) / a).to_bits() }
            } else {
                src[0][i]
            };
            if nout == 2 {
                out[1][i] = alpha_word(lt_eps, lt_one, ab, m);
            }
        } else {
            let inv = 1.0f32 / a;
            for k in 0..3 {
                out[k][i] = if lt_eps {
                    if m == PdMut::ZeroKeepsA0 && k == 0 { src[0][i] } else { 0 }
                } else if lt_one {
                    if m == PdMut::DivDirect { (f32::from_bits(src[k][i]) / a).to_bits() } else { (f32::from_bits(src[k][i]) * inv).to_bits() }
                } else {
                    src[k][i]
                };
            }
            if nout == 4 && m != PdMut::NoAlpha4 {
                out[3][i] = alpha_word(lt_eps, lt_one, ab, m);
            }
        }
    }
    Ok(())
}

/// The recorded copy-alpha route. Every field is a REQUIRED input read from the run's record (no Default): `copy_alpha` is the node's
/// stored copy_alpha at entry, `elnum` its element count, `nout` the combo tag. Only the recorded route (copy_alpha 0, elnum 3, nout 3) is ported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyAlphaRoute {
    pub copy_alpha: i32,
    pub elnum: usize,
    pub nout: i32,
}

pub const ROUTE_COPY_ALPHA: i32 = 0;
pub const ROUTE_ELNUM: usize = 3;
pub const ROUTE_NOUT: i32 = 3;

/// POSTDIV-COPY-ALPHA-PORTED: the executor at copy_alpha >= 0 on the recorded route (copy_alpha 0, elnum 3, nout 3).
/// The body first fetches an alpha plane of `n` floats from the host input `copy_alpha` names (the declared host conversion callee fills a
/// buffer that is not part of the view); `alpha` is that plane, supplied by the caller from the run's record. The colour planes are the view's
/// planes 0, 1, 2 (elnum 3 owns no alpha plane of its own). The per pixel arithmetic is then the nout 3 arm of the copy_alpha < 0 route:
/// eps arm 0, divide arm pk * (1 / a), copy arm pk; no alpha store. Any other route is refused (POSTDIV-COPY-ALPHA-SOURCE).
pub fn postdiv_copy_alpha_into(src: &[&[u32]], alpha: &[u32], route: &CopyAlphaRoute, n: i64, out: &mut [Vec<u32>], m: PdMut) -> Result<(), PdError> {
    let ok = route.copy_alpha == ROUTE_COPY_ALPHA && route.elnum == ROUTE_ELNUM && route.nout == ROUTE_NOUT;
    if !ok && !(m == PdMut::RouteAnyElnum && route.copy_alpha >= 0) {
        return Err(PdError::CopyAlphaSource(route.copy_alpha));
    }
    if n <= 0 {
        return Ok(());
    }
    let nn = n as usize;
    if src.len() < 3 || src.iter().take(3).any(|p| p.len() < nn) || alpha.len() < nn {
        return Err(PdError::Planes(format!("copy-alpha arm needs 3 colour planes and an alpha plane of {} floats", nn)));
    }
    let mut a: Vec<u32> = alpha[..nn].to_vec();
    match m {
        PdMut::AlphaFromPlane2 => a = src[2][..nn].to_vec(),
        PdMut::AlphaIgnored => a = vec![0x3f80_0000; nn],
        PdMut::AlphaDenormFlush => a.iter_mut().for_each(|x| if (*x & 0x7f80_0000) == 0 { *x = 0 }),
        PdMut::AlphaAbs => a.iter_mut().for_each(|x| *x &= 0x7fff_ffff),
        _ => {}
    }
    let view: Vec<&[u32]> = vec![src[0], src[1], src[2], &a];
    postdiv_into(&view, 4, -1, ROUTE_NOUT, n, out, m)
}

fn alpha_word(lt_eps: bool, lt_one: bool, ab: u32, m: PdMut) -> u32 {
    if lt_eps {
        if m == PdMut::AlphaOneEps { 0x3f80_0000 } else { ab }
    } else if lt_one {
        ab
    } else if m == PdMut::AlphaKeepCopy {
        ab
    } else {
        0x3f80_0000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn b(x: f32) -> u32 {
        x.to_bits()
    }
    #[test]
    fn t_branches_4() {
        let al = [b(1e-5), b(0.5), b(1.0), b(2.0), 0x7fc0_0000, b(-0.5), 0x8000_0000, b(0.0)];
        let c = vec![b(0.25); 8];
        let src: Vec<&[u32]> = vec![&c, &c, &c, &al];
        let mut out = vec![vec![0xdead_beef; 8]; 4];
        postdiv_into(&src, 4, -1, 4, 8, &mut out, PdMut::None).unwrap();
        assert_eq!(out[0][0], 0); // below eps
        assert_eq!(out[3][0], b(1e-5));
        assert_eq!(out[0][1], b(0.5)); // 0.25 * (1 / 0.5)
        assert_eq!(out[3][1], b(0.5));
        assert_eq!(out[0][2], b(0.25)); // a = 1: copy, alpha 1
        assert_eq!(out[3][2], b(1.0));
        assert_eq!(out[3][3], b(1.0)); // a > 1
        assert_eq!(out[0][4], b(0.25)); // NaN: copy arm, alpha 1
        assert_eq!(out[3][4], b(1.0));
        assert_eq!(out[0][5], 0); // negative: eps arm, alpha kept
        assert_eq!(out[3][5], b(-0.5));
        assert_eq!(out[3][6], 0x8000_0000); // -0: eps arm
        assert_eq!(out[0][7], 0);
    }
    #[test]
    fn t_copy_alpha_route() {
        let c = vec![b(0.25); 4];
        let al = [b(0.5), b(1e-5), b(2.0), 0x7fc0_0000];
        let src: Vec<&[u32]> = vec![&c, &c, &c];
        let r = CopyAlphaRoute { copy_alpha: 0, elnum: 3, nout: 3 };
        let mut out = vec![vec![7u32; 4]; 3];
        postdiv_copy_alpha_into(&src, &al, &r, 4, &mut out, PdMut::None).unwrap();
        assert_eq!(out[0][0], b(0.5));
        assert_eq!(out[0][1], 0);
        assert_eq!(out[1][2], b(0.25));
        assert_eq!(out[2][3], b(0.25));
        for bad in [CopyAlphaRoute { copy_alpha: 1, elnum: 3, nout: 3 }, CopyAlphaRoute { copy_alpha: 0, elnum: 4, nout: 3 }, CopyAlphaRoute { copy_alpha: 0, elnum: 3, nout: 4 }] {
            assert_eq!(postdiv_copy_alpha_into(&src, &al, &bad, 4, &mut out, PdMut::None), Err(PdError::CopyAlphaSource(bad.copy_alpha)));
        }
    }
    #[test]
    fn t_refusals_and_empty() {
        let c = vec![b(0.25); 4];
        let src: Vec<&[u32]> = vec![&c, &c, &c];
        let mut out = vec![vec![7u32; 4]; 3];
        assert_eq!(postdiv_into(&src, 3, -1, 3, 4, &mut out, PdMut::None), Err(PdError::ElnumFolded(3)));
        assert_eq!(postdiv_into(&src, 4, 0, 3, 4, &mut out, PdMut::None), Err(PdError::CopyAlphaSource(0)));
        let src2: Vec<&[u32]> = vec![&c, &c];
        postdiv_into(&src2, 2, -1, 2, 0, &mut out, PdMut::None).unwrap();
        assert!(out.iter().all(|p| p.iter().all(|&x| x == 7)));
        postdiv_into(&src2, 2, -1, 2, 4, &mut out, PdMut::None).unwrap();
        assert_eq!(out[0][0], b(1.0)); // 0.25 / 0.25
        assert_eq!(out[1][0], b(0.25));
    }
}
