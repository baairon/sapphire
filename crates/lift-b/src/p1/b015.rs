//! Batch 015: bytes-per-row calculators selected by a format id (jump tables), a 12 word to 4 dword unpack and
//! the RGB to 10-bit YCbCr line converter with two matrices.
use crate::cov::{br, hit};
use crate::*;

/// Row size of 6 pixels per 16 bytes (v210-like, 48 pixel groups of 128 bytes): round esi up to a multiple of
/// 48, times 8, divided by 3. 32-bit arithmetic where the original uses 32-bit registers, 64-bit multiplies by
/// the reciprocal 0xaaaaaaab.
pub(crate) fn rowbytes_48x128(esi: u32) -> u32 {
    let q = ((esi as u64).wrapping_mul(0xaaaa_aaab) >> 37) as u32; // esi / 48
    let ecx = q.wrapping_shl(4);
    let ecx = ecx.wrapping_add(ecx.wrapping_mul(2)); // q * 48
    let rem = esi.wrapping_sub(ecx);
    let mut e = esi;
    if rem != 0 {
        e = e.wrapping_add(0x30).wrapping_sub(rem);
    }
    let e = e.wrapping_shl(3);
    (((e as u64).wrapping_mul(0xaaaa_aaab)) >> 33) as u32
}

/// parity: P1 @0x583a0 size=196
/// Bytes per row for a format id edi (0..=20, jump table at 0x58410) and width esi: ids 0 and 8 use the
/// 48-pixel/128-byte rule, 1 and 5 give 2*w, 2, 3, 4, 6, 7, 15, 20 give 4*w, 9 gives w/2, 12 and 13 give 3*w,
/// 16 gives 6*w; the other ids and anything above 20 give 0.
pub fn f_000583a0(edi: u32, esi: u32) -> u32 {
    match edi {
        0 | 8 => {
            hit();
            rowbytes_48x128(esi)
        }
        1 | 5 => {
            hit();
            esi.wrapping_add(esi)
        }
        2 | 3 | 4 | 6 | 7 | 15 | 20 => {
            hit();
            esi.wrapping_shl(2)
        }
        9 => {
            hit();
            (esi >> 1) & 0x3fff_ffff
        }
        12 | 13 => {
            hit();
            esi.wrapping_add(esi.wrapping_mul(2))
        }
        16 => {
            hit();
            let e = esi.wrapping_add(esi);
            e.wrapping_add(e.wrapping_mul(2))
        }
        _ => {
            hit();
            0
        }
    }
}

/// parity: P1 @0xc5b9a size=274
/// Bytes per row for a format id edi (1..=36, jump table at 0xc5c1c) and width esi, a larger variant of
/// f_000583a0: ids 1 and 9 use the 48-pixel/128-byte rule, 2 and 6 give 2*w, 3, 4, 5, 7, 8, 16, 21 give 4*w,
/// 13 and 14 give 3*w, 17 gives (4w*9)>>3, 24 and 26..=29 give 6*w, 31 and 32 give (2w*5)>>3, 33 and 34 give
/// (4w*5)>>4, 35 and 36 give w; every other id (and anything outside 1..=36) gives 0.
pub fn f_000c5b9a(edi: u32, esi: u32) -> u32 {
    let n = edi.wrapping_sub(1);
    match n {
        0 | 8 => {
            hit();
            rowbytes_48x128(esi)
        }
        1 | 5 => {
            hit();
            esi.wrapping_add(esi)
        }
        2 | 3 | 4 | 6 | 7 | 15 | 20 => {
            hit();
            esi.wrapping_shl(2)
        }
        12 | 13 => {
            hit();
            esi.wrapping_add(esi.wrapping_mul(2))
        }
        16 => {
            hit();
            let e = esi.wrapping_shl(2);
            e.wrapping_add(e.wrapping_mul(8)) >> 3
        }
        23 | 25 | 26 | 27 | 28 => {
            hit();
            let e = esi.wrapping_add(esi);
            e.wrapping_add(e.wrapping_mul(2))
        }
        30 | 31 => {
            hit();
            let e = esi.wrapping_add(esi);
            e.wrapping_add(e.wrapping_mul(4)) >> 3
        }
        32 | 33 => {
            hit();
            let e = esi.wrapping_shl(2);
            e.wrapping_add(e.wrapping_mul(4)) >> 4
        }
        34 | 35 => {
            hit();
            esi
        }
        _ => {
            hit();
            0
        }
    }
}

/// parity: P1 @0xc5d09 size=214
/// Packs groups of 12 words into 4 dwords of three words each: dword = w0 + (w1 << 10) + (w2 << 20) (additions,
/// not ors). The loop runs while the word base + 12 is below 2*edx (32 bit); at least one round runs when
/// 2*edx != 0, and nothing happens when 2*edx == 0 (mod 2^32).
pub fn f_000c5d09(src: &[u8], dst: &mut [u8], edx: u32) {
    let end = edx.wrapping_add(edx);
    if br(end == 0) {
        return;
    }
    let mut r8 = 0u32;
    let mut r11 = 5u32;
    loop {
        let base = r11.wrapping_sub(5);
        let w = |i: u32| rd16(src, 2 * base.wrapping_add(i) as usize) as u32;
        for k in 0..4u32 {
            let v = w(3 * k).wrapping_add(w(3 * k + 1) << 10).wrapping_add(w(3 * k + 2) << 20);
            wr32(dst, 4 * r8.wrapping_add(k) as usize, v);
        }
        r8 = r8.wrapping_add(4);
        let next = r11.wrapping_add(7);
        r11 = r11.wrapping_add(12);
        if !br(next < end) {
            break;
        }
    }
}

/// parity: P1 @0x65aef size=281
/// RGB(A) bytes (B at +0, G at +1, R at +2, stride 4) to 10-bit YCbCr words for edx pixels (does nothing for
/// edx <= 0 signed). The matrix depends on the low byte of r8d: non-zero uses the BT.601-style coefficients of
/// f_000c8b9d, zero uses 0x2e8a/0x9c9f/0x0fd2 for Y, -0x18f4/-0x545b/0x6da9 for Cb and 0x6d71/-0x6305/-0x0a06
/// for Cr (looks like BT.709; inferred). Y = (num >> 14) + 0x40, Cb and Cr = ((num >> 14) + 0x200) & 0x3ff.
/// The output starts at byte 2 * sext((2 * ecx) & !3); even pixels write (Cb, Y, Cr) words, odd pixels only Y.
pub fn f_00065aef(src: &[u8], dst: &mut [u8], edx: u32, ecx: u32, r8d: u32) {
    if br((edx as i32) <= 0) {
        return;
    }
    let c2 = ecx.wrapping_add(ecx) & 0xffff_fffc;
    let mut p = (c2 as i32 as i64).wrapping_mul(2);
    let mut j = 0usize;
    loop {
        let r = src[4 * j + 2] as u32;
        let g = src[4 * j + 1] as u32;
        let b = src[4 * j] as u32;
        let (ynum, cbnum, crnum) = if br(r8d as u8 != 0) {
            (
                b.wrapping_mul(0x1910).wrapping_add(g.wrapping_mul(0x810f)).wrapping_add(r.wrapping_mul(0x41bc)),
                b.wrapping_mul(0x7070).wrapping_add(g.wrapping_mul(0xffff_b582)).wrapping_add(r.wrapping_mul(0xffff_da0f)),
                b.wrapping_mul(0xffff_edb7).wrapping_add(g.wrapping_mul(0xffff_a1d9)).wrapping_add(r.wrapping_mul(0x7070)),
            )
        } else {
            (
                b.wrapping_mul(0x0fd2).wrapping_add(g.wrapping_mul(0x9c9f)).wrapping_add(r.wrapping_mul(0x2e8a)),
                b.wrapping_mul(0x6da9).wrapping_add(g.wrapping_mul(0xffff_aba5)).wrapping_add(r.wrapping_mul(0xffff_e70c)),
                b.wrapping_mul(0xffff_f5fa).wrapping_add(g.wrapping_mul(0xffff_9cfb)).wrapping_add(r.wrapping_mul(0x6d71)),
            )
        };
        let y = (ynum >> 14).wrapping_add(0x40);
        if br(j & 1 == 0) {
            let cb = ((cbnum >> 14).wrapping_add(0x200)) & 0x3ff;
            let cr = ((crnum >> 14).wrapping_add(0x200)) & 0x3ff;
            wr16(dst, p as usize, cb as u16);
            wr16(dst, (p + 2) as usize, y as u16);
            wr16(dst, (p + 4) as usize, cr as u16);
            p += 6;
        } else {
            wr16(dst, p as usize, y as u16);
            p += 2;
        }
        j += 1;
        if !br(j as u32 != edx) {
            break;
        }
    }
}
