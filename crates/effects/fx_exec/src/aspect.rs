//! native port of the plugin function `set_aspect` and of its rounding helper, plus the callers' clamp range.
//! Every constant is read from the approved file (virtual address = listing address), see SETASPECT.md.
//!
//! set_aspect(ctx rdi, a xmm0 as f32), in listing order:
//!   if a == 0 (either sign) { jmp (error logger, edi = 9); no store } 
//!   [ctx] = (a != 1.0f) as u32 (a NaN: the parity jump skips the equal arm, so 1) 
//!   [ctx] = a; [ctx] = 1.0f / a (divss, f32) 
//!   r1 = round_scaled(a) (call)
//!   [ctx] = r1; [ctx] = round_scaled([ctx] * a) (mulss, f32; calls) 
//!   [ctx] = ((double)a > 1.6) as u32 (comisd against the double; NaN: jbe taken, 0)
//! round_scaled(x) : t = (double)(x * 4096.0f); s = copysign(0.5, x) [sign mask];
//!   cvttsd2si32(s + t)   (truncation; NaN or out of int32 range gives 0x80000000).
//! Callers (and the same pattern elsewhere) call set_aspect only when (double)a > 1e-4 and a < 10000.0f
//!  (comisd / comiss with jbe, so NaN falls to the fallback), otherwise with the constant 1.0f .

pub const A_LO: f64 = 1e-4; //  (double)
pub const A_HI: f32 = 10000.0; //  (float)
pub const THRESH_CELL_A: f64 = 1.6; //  (double)
pub const ROUND_SCALE: f32 = 4096.0; //  (float)

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectBlock {
    pub aspect_block_b: u32,
    pub aspect_block_c: u32,
    pub aspect_block_d: u32,
    pub aspect_block_e: u32,
    pub aspect_block_f: i32,
    pub aspect_block_g: i32,
}

fn cvttsd2si32(v: f64) -> i32 {
    if v.is_nan() || v >= 2147483648.0 || v <= -2147483649.0 {
        i32::MIN
    } else {
        v as i32
    }
}

/// 
pub fn round_scaled(x: f32) -> i32 {
    let t = (x * ROUND_SCALE) as f64;
    let s = if x.is_sign_negative() { -0.5f64 } else { 0.5f64 };
    cvttsd2si32(s + t)
}

/// . `aspect_block_a` is the f32 at ctx before the call. None = the a == 0 error path (nothing stored).
pub fn set_aspect(a: f32, aspect_block_a: f32) -> Option<AspectBlock> {
    if a == 0.0 {
        return None;
    }
    Some(AspectBlock {
        aspect_block_b: if a == 1.0 { 0 } else { 1 },
        aspect_block_c: a.to_bits(),
        aspect_block_d: (1.0f32 / a).to_bits(),
        aspect_block_e: if (a as f64) > THRESH_CELL_A { 1 } else { 0 },
        aspect_block_f: round_scaled(aspect_block_a * a),
        aspect_block_g: round_scaled(a),
    })
}

/// The callers' range test: (double)a > 1e-4 and a < 1e4 (NaN fails both).
pub fn in_caller_range(a: f32) -> bool {
    (a as f64) > A_LO && a < A_HI
}

// ---------------------------------------------------------------------------------------------------------------------------------
// the caller block (listing addresses), ported as read. Constants read from the approved file (VA = listing):
//    = double 0.001 (MEASURED bytes fca9f1d24d62503f), = double 1e-4, = float 10000.0, = float 1.0.
// Operation order as read:
//     ov = f32 at the override cell (GOT slot -> bss cell, 0 in the file; writer per);
//                       if (double)ov > 0.001 (comisd, ja: NaN not taken) then set_aspect(ov) with no range test (jump to)
//     w_cell_e == 0 or w_cell_f == 0 -> fallback
//     w_cell_a != 0 and w_cell_b != 0 (cmovne to 1 then and, cmp 1) else fallback
//     w_cell_c != 0 and w_cell_d != 0 else fallback
//     num = (float)(u64)w_cell_f (cvtsi2ss from rax = zero-extended dword), den = (float)(i32)w_cell_e; dd = d_cell_b / d_cell_c (divsd);
//                       host id == 6: q = den / num (arm: divss xmm2,xmm0), else q = num / den (divss xmm0,xmm2);
//                       a = (f32)((f64)q * dd)   (mulsd then cvtsd2ss)
//     (double)a > 1e-4 (comisd, jbe -> fallback, includes NaN); 1e4f > a (comiss xmm1=1e4, xmm0=a; jbe -> fallback, includes NaN)
//      set_aspect(a) or set_aspect(1.0f)
pub const OVERRIDE_THRESH: f64 = 0.001; //  (double)
pub const FALLBACK_A: f32 = 1.0; //  (float)
pub const HOST_ID_INVERT: u32 = 6; // cmp, 6 

/// The host record and cells the block reads. All dwords as stored.
#[derive(Clone, Copy, Debug)]
pub struct CallerInputs {
    pub override_cell: f32, // 
    pub host_id: u32,       // 
    pub w_cell_a: u32,
    pub w_cell_b: u32,
    pub w_cell_c: u32,
    pub w_cell_d: u32,
    pub w_cell_e: u32,
    pub w_cell_f: u32,
    pub d_cell_b: f64, // [[r13]]
    pub d_cell_c: f64, // [[r13]]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Override,
    Computed,
    Fallback,
}

/// The value the block passes to set_aspect, and which arm produced it.
pub fn caller_aspect(i: &CallerInputs) -> (Route, f32) {
    if (i.override_cell as f64) > OVERRIDE_THRESH {
        return (Route::Override, i.override_cell);
    }
    if i.w_cell_e == 0 || i.w_cell_f == 0 {
        return (Route::Fallback, FALLBACK_A);
    }
    if !(i.w_cell_a != 0 && i.w_cell_b != 0) || !(i.w_cell_c != 0 && i.w_cell_d != 0) {
        return (Route::Fallback, FALLBACK_A);
    }
    let num = i.w_cell_f as f32; // cvtsi2ss xmm0, rax (zero-extended dword)
    let den = (i.w_cell_e as i32) as f32; // cvtsi2ss xmm2, ecx (signed)
    let dd = i.d_cell_b / i.d_cell_c;
    let q = if i.host_id == HOST_ID_INVERT { den / num } else { num / den };
    let a = ((q as f64) * dd) as f32;
    if !((a as f64) > A_LO) {
        return (Route::Fallback, FALLBACK_A);
    }
    if !(A_HI > a) {
        return (Route::Fallback, FALLBACK_A);
    }
    (Route::Computed, a)
}
