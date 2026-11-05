//! CV3: the native convert pair wired into the executor. Host memory is format code 16 (B,G,R,A f32 per pixel, rows of `width` pixels),
//! cell 1 (one converted row per call, HOST-BAND-MINIMAL), full-frame view, four planes, ctx = 4 (HOST-CH4).
//! convert-from-source runs `convert_pair::convert_from_source_line` for every row (non-finite samples become +0.0, colour * alpha);
//! convert-to-dest runs `convert_pair::convert_to_dest_line` for every row with the explicit alpha mode (a parameter of the host,
//! ALPHA-MODE-HOST; `convert_pair::derive_alpha_mode` is the dump-code rule). Mode 0 writes nothing: the destination keeps `dest_prefill`.

use crate::types::{ExecError, Image};
use convert_pair::{convert_from_source_line, convert_to_dest_line, convert_to_dest_line_n12, AlphaMode, Dest12, DestOutcome, ToDestParams};

/// Everything the host side of the pair needs. No Default.
#[derive(Clone, Debug)]
pub struct HostConvert {
    /// host source memory per source index (`convert-from-source i`): height * width * 4 floats, B,G,R,A per pixel
    pub src: Vec<Vec<f32>>,
    pub mode: AlphaMode,
    /// the destination memory before the pair runs (height * width * 4 floats): what mode 0 leaves untouched
    pub dest_prefill: Vec<f32>,
    /// the band cell of the converts ([[ctx]]); only cell 1 is claimed (W211 (e), CELL-GT1-NOT-CLAIMED)
    pub cell: usize,
    /// the edge margin count of the converted view; only 0 is claimed (W211 (e), W216 (a))
    pub margin: usize,
    /// INT9 (Batch 43 rule 1): the 'ae32' route (memory A,R,G,B f32, format 14 in) with the recorded reader head and plane count per source;
    /// None = the format-16 state A route above. Some: convert-from-source reads `ae32.sources`, convert-to-dest is refused until FMT3c.
    pub ae32: Option<crate::hostconv14::Ae32Route>,
}

impl HostConvert {
    /// W211 (e): a cell other than 1 is CELL-GT1-NOT-CLAIMED, a margin count above 0 is an error.
    pub fn check_claim(&self) -> Result<(), ExecError> {
        if self.margin > 0 {
            return Err(ExecError::Shape(format!("convert: margin count {} above 0 is outside the claim (W211 e)", self.margin)));
        }
        if self.cell != 1 {
            return Err(ExecError::CellGt1NotClaimed(self.cell));
        }
        Ok(())
    }
}

pub fn host_to_image(mem: &[f32], w: usize, h: usize) -> Result<Image, ExecError> {
    if mem.len() != 4 * w * h {
        return Err(ExecError::Shape(format!("host memory has {} floats, {}x{}x4 needed", mem.len(), w, h)));
    }
    let mut planes: [Vec<f32>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut out: Vec<Vec<f32>> = vec![Vec::with_capacity(w * h); 4];
    for y in 0..h {
        convert_from_source_line(&mem[4 * w * y..4 * w * (y + 1)], &mut planes, w);
        for k in 0..4 {
            out[k].extend_from_slice(&planes[k]);
        }
    }
    Image::from_planes(out)
}

/// The 3 plane destination rule (CV4, TW2; DSW6 A.1), stated once: turns mode 1 into mode 3 for an odd plane count and the 3 channel writer arm never
/// reads the fourth plane, so a zero plane stands in for it. Other channel counts pass unchanged. `execute_host` and `fx_run` both call this function.
pub fn dest_for_host(dest: &Image, mode: AlphaMode) -> Result<(Image, AlphaMode), ExecError> {
    if dest.channels() == 3 {
        let mut pl = dest.planes.clone();
        pl.push(vec![0.0f32; dest.n]);
        let padded = Image::from_planes(pl).map_err(|e| ExecError::Shape(format!("{:?}", e)))?;
        let m = if mode == AlphaMode::ClampKeep { AlphaMode::One } else { mode };
        Ok((padded, m))
    } else {
        Ok((dest.clone(), mode))
    }
}

/// Returns the host destination memory and whether the writer wrote (false for mode 0).
pub fn image_to_host(img: &Image, w: usize, h: usize, mode: AlphaMode, prefill: &[f32]) -> Result<(Vec<f32>, bool), ExecError> {
    // CV12: a 1 or 2 plane result image (Blur_Mono ends in gray + alpha) goes through the ported 1- and 2-plane writer arms with the plane count and
    // mode arithmetic of (writer_args, ctx = 4, HOST-CH4); this replaces BLE1's stand-in DEST2-EXPAND.
    if (img.channels() == 1 || img.channels() == 2) && img.n == w * h {
        if prefill.len() != 4 * w * h {
            return Err(ExecError::Shape("dest prefill size".into()));
        }
        let mut dest = prefill.to_vec();
        let mut wrote = true;
        let mode_byte: i8 = match mode {
            AlphaMode::NoWrite => 0,
            AlphaMode::ClampKeep => 1,
            AlphaMode::ZeroAlpha => 2,
            AlphaMode::One => 3,
        };
        for y in 0..h {
            let rows: [Vec<f32>; 4] = [
                img.planes[0][y * w..(y + 1) * w].to_vec(),
                if img.channels() == 2 { img.planes[1][y * w..(y + 1) * w].to_vec() } else { Vec::new() },
                Vec::new(),
                Vec::new(),
            ];
            let (_edx, o) = convert_to_dest_line_n12(img.channels() as i32, 4, mode_byte, &rows, &mut dest[4 * w * y..4 * w * (y + 1)], w);
            match o {
                Dest12::Written => {}
                Dest12::NotWritten => wrote = false,
                Dest12::NotReachable => {
                    return Err(ExecError::Shape(format!("convert-to-dest: no 1/2 plane writer arm for {} planes in mode {}", img.channels(), mode_byte)));
                }
            }
        }
        return Ok((dest, wrote));
    }
    if img.channels() != 4 || img.n != w * h {
        return Err(ExecError::Shape(format!("convert-to-dest needs 4 planes of {} floats, found {} x {}", w * h, img.channels(), img.n)));
    }
    if prefill.len() != 4 * w * h {
        return Err(ExecError::Shape("dest prefill size".into()));
    }
    let mut dest = prefill.to_vec();
    let mut wrote = true;
    for y in 0..h {
        let rows: [Vec<f32>; 4] = [
            img.planes[0][y * w..(y + 1) * w].to_vec(),
            img.planes[1][y * w..(y + 1) * w].to_vec(),
            img.planes[2][y * w..(y + 1) * w].to_vec(),
            img.planes[3][y * w..(y + 1) * w].to_vec(),
        ];
        if convert_to_dest_line(&rows, ToDestParams { mode }, &mut dest[4 * w * y..4 * w * (y + 1)], w) == DestOutcome::NotWritten {
            wrote = false;
        }
    }
    Ok((dest, wrote))
}
