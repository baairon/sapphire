//! Bright-pass arithmetic, read from the listing of body 0x674000 (instruction order kept).
//! No FMA appears in the body, so there is no mul_add here. No rcpss/rsqrtss: one exact vdivss.
use crate::copy::{copy_plane, RoiRecord};
use crate::plane::{check, FxError, Plane, PlaneMut};
use fx_common::nanop::NanOps;
use crate::x86::{maxss, minss};

/// 1.0f, read at VA 0x15ea70c.
const ONE: f32 = 1.0;
/// VA 0x1610fb0, captured: 0x3c000000 = 2^-7. MEASURED.
const FLOOR: f32 = 0.0078125;

/// Parameters of one bright-pass call (arguments of 0x673fa0 / 0x674000). Meanings INFERRED.
#[derive(Clone, Copy, Debug)]
pub struct BrightParams {
    /// float[3] at rsi (r14 in the body): per-channel thresholds th0, th1, th2 (set by the 0x33bca0 and
    /// 0x33bfb0 callers from the glow threshold lanes). The 1 plane call reads only thresholds[0].
    pub thresholds: [f32; 3],
}

/// Stand-in or unread dependency record.
#[derive(Clone, Copy, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub va: &'static str,
    pub value: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "copy-flat", va: "0x672c60", value: "the host copy is one contiguous copy of c*b elements from the start of both planes; for dense planes (stride == width) that is the plain copy of the visible samples",
            basis: "listing 672c60..672d4a and callees 670340, 6704b0, 136fbb0", label: "closed" },
        Assumption { id: "copy-roi", va: "0x5b9b10", value: "both ROI forms are reachable from 0x33bca0 and 0x33bfb0; n = c * b with c = width (form 1) or the row pitch field (form 2)",
            basis: "listing 5b9b10..5b9bce, 33bca0, 33bfb0; the meaning of the row pitch field is inferred", label: "closed, pitch meaning inferred" },
        Assumption { id: "dense-planes", va: "0x674000", value: "parity is claimed for dense planes (stride == width); padded strides are processed per visible sample and are outside the original's contract (it walks n contiguous floats)",
            basis: "the original walks c*b contiguous floats in the copy and in the pass", label: "documented limit" },
        Assumption { id: "roi-constants", va: "0x160f270", value: "ROI constants at 0x160f270 (zeros), 0x160f280 (qword 4), 0x160f290 (zeros), 0x160f2a0 (qword 4); they do not enter pixel values",
            basis: "captured data image", label: "closed" },
        Assumption { id: "default-slots", va: "0x1610b70", value: "the default slots at 0x1610b70 and 0x16112f0 are not used; every parameter is explicit",
            basis: "address audit of the captured data image", label: "not used" },
    ]
}

/// Scale factor k of the pass, or None when the body returns without touching memory.
/// Listing 674033..6740a9 (and 674077..6740a9). n = 1: th0 only. n = 3: th0..th2.
pub fn scale(t: &[f32; 3], nplanes: usize) -> Option<f32> {
    let m;
    if nplanes == 1 {
        if t[0] == 0.0 {
            return None; // 674040..674048 -> 6743df
        }
        m = t[0];
    } else {
        if t[0] == 0.0 && t[1] == 0.0 && t[2] == 0.0 {
            return None; // 674040..674072 -> 6743df
        }
        m = minss(minss(t[0], t[1]), t[2]); // 674089, 67408d
    }
    // 6740a1 vsubss xmm1 = ONE - m; 6740a5 vmaxss xmm0 = maxss(FLOOR, xmm1); 6740a9 vdivss xmm0 = ONE / xmm0
    Some(ONE / maxss(FLOOR, ONE - m))
}

/// One sample of the 1 plane call. Vector loop 674220..674284 and scalar tail 6742a7..6742d9 agree lane for lane.
#[inline]
fn one_plane(x: f32, t: f32, k: f32) -> f32 {
    // zero when th >= x (ordered) or 0 >= x (ordered); NaN passes the tests (vcmpleps false, vcomiss unordered).
    if t >= x || 0.0 >= x {
        return 0.0;
    }
    let d = k.fmul(x.fsub(t)); // vsubps then vmulps, separate roundings
    if x > d { d } else { x }
}

/// One pixel of the 3 plane call (674308..6743cc, same ops as 0x6792d0). None = pixel unchanged.
#[inline]
fn three_plane(r: f32, g: f32, b: f32, t: &[f32; 3], k: f32) -> Option<[f32; 3]> {
    let mut e = if r > t[0] { r.fsub(t[0]) } else { 0.0 };
    if g > t[1] {
        e = e.fadd(g.fsub(t[1]));
    }
    if b > t[2] {
        e = e.fadd(b.fsub(t[2]));
    }
    if !(e > 0.0) {
        return Some([0.0; 3]);
    }
    let mr = maxss(r, 0.0);
    let s1 = if g > 0.0 { g.fadd(mr) } else { mr };
    let s = if b > 0.0 { b.fadd(s1) } else { s1 };
    if !(s > 0.0) {
        return Some([0.0; 3]);
    }
    let tt = k.fmul(e);
    if s > tt {
        let f = tt.fdiv(s);
        return Some([r.fmul(f), f.fmul(g), f.fmul(b)]);
    }
    None
}

/// Bright pass on destination planes in place (1 or 3 planes, equal geometry).
pub fn bright_pass_in_place(p: &BrightParams, planes: &mut [PlaneMut]) -> Result<(), FxError> {
    let n = planes.len();
    if n != 1 && n != 3 {
        return Err(FxError::PlaneCount); // the original calls the assert helper 0x66a800
    }
    let (w, h) = (planes[0].width, planes[0].height);
    for pl in planes.iter() {
        if pl.width != w || pl.height != h {
            return Err(FxError::Geometry);
        }
        check(pl.data.len(), pl.width, pl.height, pl.stride)?;
    }
    let k = match scale(&p.thresholds, n) {
        Some(k) => k,
        None => return Ok(()),
    };
    if n == 1 {
        let pl = &mut planes[0];
        for y in 0..h {
            for v in &mut pl.data[y * pl.stride..y * pl.stride + w] {
                *v = one_plane(*v, p.thresholds[0], k);
            }
        }
    } else {
        for y in 0..h {
            for x in 0..w {
                let i = [y * planes[0].stride + x, y * planes[1].stride + x, y * planes[2].stride + x];
                let (r, g, b) = (planes[0].data[i[0]], planes[1].data[i[1]], planes[2].data[i[2]]);
                if let Some(o) = three_plane(r, g, b, &p.thresholds, k) {
                    planes[0].data[i[0]] = o[0];
                    planes[1].data[i[1]] = o[1];
                    planes[2].data[i[2]] = o[2];
                }
            }
        }
    }
    Ok(())
}

/// Bright-pass layer: copy the source, then the pass in place. `src` and `dst` hold 1 or 3 planes.
pub fn bright_pass(p: &BrightParams, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    if src.len() != dst.len() || (src.len() != 1 && src.len() != 3) {
        return Err(FxError::PlaneCount);
    }
    for (s, d) in src.iter().zip(dst.iter_mut()) {
        if s.width != d.width || s.height != d.height {
            return Err(FxError::Geometry);
        }
        check(s.data.len(), s.width, s.height, s.stride)?;
        check(d.data.len(), d.width, d.height, d.stride)?;
        for y in 0..s.height {
            let (a, b) = (y * s.stride, y * d.stride);
            d.data[b..b + s.width].copy_from_slice(&s.data[a..a + s.width]);
        }
    }
    bright_pass_in_place(p, dst)
}

/// Bright pass on the first `n` contiguous floats of each plane (1 or 3 planes), the shape the original works in (n = c * b of the ROI record).
pub fn bright_pass_flat(p: &BrightParams, planes: &mut [&mut [f32]], n: usize) -> Result<(), FxError> {
    let np = planes.len();
    if np != 1 && np != 3 {
        return Err(FxError::PlaneCount);
    }
    if planes.iter().any(|pl| pl.len() < n) {
        return Err(FxError::Stride);
    }
    let k = match scale(&p.thresholds, np) {
        Some(k) => k,
        None => return Ok(()),
    };
    if np == 1 {
        for v in &mut planes[0][..n] {
            *v = one_plane(*v, p.thresholds[0], k);
        }
    } else {
        for i in 0..n {
            let (r, g, b) = (planes[0][i], planes[1][i], planes[2][i]);
            if let Some(o) = three_plane(r, g, b, &p.thresholds, k) {
                planes[0][i] = o[0];
                planes[1][i] = o[1];
                planes[2][i] = o[2];
            }
        }
    }
    Ok(())
}

/// What the glow callers 0x33bca0 / 0x33bfb0 do with render flag 0, for either ROI form: copy `roi.count()` floats of each source
/// plane to the destination plane (0x672c60), then run the bright pass on the same count (0x674000).
pub fn bright_pass_roi(p: &BrightParams, src: &[&[f32]], dst: &mut [&mut [f32]], roi: &RoiRecord) -> Result<(), FxError> {
    if src.len() != dst.len() || (src.len() != 1 && src.len() != 3) {
        return Err(FxError::PlaneCount);
    }
    for (s, d) in src.iter().zip(dst.iter_mut()) {
        copy_plane(d, s, roi)?;
    }
    let n = roi.count();
    if n < 0 {
        return Err(FxError::Geometry);
    }
    bright_pass_flat(p, dst, n as usize)
}
