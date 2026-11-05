//! bwb_libm: native port of the libm routine at listing address used by blur_warp_bidirectional.
//! It is the double precision fmod (x, y). AVX2 route: the dispatcher sees the low byte 0xff of the CPU feature word
//! 0x9d97ff and tail-jumps to (legacy SSE2 encoding only, no x87, no approximate instruction).
//! Ported in listing order on u64 register images, no fused operations.

const ABS: u64 = 0x7fff_ffff_ffff_ffff;
const SIGN: u64 = 0x8000_0000_0000_0000;
const K_MASK: u64 = 0xffff_ffff_f800_0000; // 
const K_2P64: u64 = 0x43f0_0000_0000_0000; // 
const K_2M64: u64 = 0x3bf0_0000_0000_0000; // 
const K_HALF: u64 = 0x3fe0_0000_0000_0000; // 
const K_DBLMIN: u64 = 0x0010_0000_0000_0000; // 
const K_2P1023: u64 = 0x7fe0_0000_0000_0000; // 

fn f(b: u64) -> f64 {
    f64::from_bits(b)
}
fn bt(x: f64) -> u64 {
    x.to_bits()
}
fn mul(a: u64, b: u64) -> u64 {
    bt(f(a) * f(b))
}
fn sub(a: u64, b: u64) -> u64 {
    bt(f(a) - f(b))
}
fn add(a: u64, b: u64) -> u64 {
    bt(f(a) + f(b))
}
fn div(a: u64, b: u64) -> u64 {
    bt(f(a) / f(b))
}
fn psllq(x: u64, c: u64) -> u64 {
    if c > 63 { 0 } else { x << c }
}
fn psrlq(x: u64, c: u64) -> u64 {
    if c > 63 { 0 } else { x >> c }
}
/// cmpnlesd with a zero destination: all ones when !(0 <= x), NaN included.
fn nle0(x: u64) -> u64 {
    if 0.0 <= f(x) { 0 } else { u64::MAX }
}
fn w3(b: u64) -> u64 {
    (b >> 48) & 0xffff
}

pub fn assumptions() -> Vec<&'static str> {
    vec![
        "A1 feature word 0x9d97ff (low byte 0xff), so the dispatcher takes (INFERRED from the p1 precedent)",
        "A2 the error handler (called only for y == 0, via) is UNREAD; modelled as leaving the NaN result unchanged",
        "A3 the SSE route is not ported (not taken on the AVX2 route)",
        "A4 NaN payloads follow the host FPU and are compared as a class",
        "A5 MXCSR default (round to nearest, no FTZ or DAZ)",
    ]
}

fn handler_nan(yb: u64) -> u64 {
    div(yb, yb)
}

fn inf_nan_x(xb: u64, yb: u64, rdx: u64, r10: u64) -> u64 {
    // 
    if (xb << 12) != 0 {
        return sub(xb, xb);
    }
    let rd = (rdx as u32 as i32) as i64 as u64;
    if ((r10 & rd) as i64) < 0 {
        return handler_nan(yb);
    }
    if (yb << 1) > 0xffe0_0000_0000_0000 {
        return add(xb, yb);
    }
    sub(xb, xb)
}

fn y_pow2(xb: u64, rcx: u64) -> u64 {
    // 
    let rdx = 0x34u64.wrapping_sub(rcx);
    if (rdx as i64) <= 0 {
        // 
        if (w3(xb) & 0x7fff) == 0x7ff0 {
            return inf_nan_x(xb, 0, 0, 0);
        }
        return 0;
    }
    let t = psllq(psrlq(xb, rdx), rdx);
    sub(xb, t) | (xb & SIGN)
}

fn big_exp(xb: u64, yb: u64, r11: u64) -> u64 {
    // 
    let n = 0u64.wrapping_sub(r11);
    let x0 = if n >= 0x3e7 { bt(fmod_x(f(xb), f(mul(yb, K_2P1023)))) } else { xb };
    mul(bt(fmod_x(f(mul(x0, K_2P64)), f(mul(yb, K_2P64)))), K_2M64)
}

fn y_special(xb: u64, yb: u64, rdx: u64, r10: u64, r11: u64) -> u64 {
    // 
    if (w3(xb) & 0x7ff0) == 0x7ff0 {
        return inf_nan_x(xb, yb, rdx, r10);
    }
    if (rdx as u32 as i32) < 0 {
        // 
        if (r10 as i64) < 0 {
            return handler_nan(yb);
        }
        if ((w3(xb) & 0x7ff0) as i32) > 1 {
            return big_exp(xb, yb, r11);
        }
        let x2 = mul(xb, K_2P64);
        let y1 = mul(yb, K_2P64);
        let q = div(x2, y1);
        let e = (((w3(q) & 0x7ff0) >> 4) as i32).wrapping_sub(0x3ff);
        if e < 0 {
            return xb;
        }
        let ecx = 0x34u32.wrapping_sub(e as u32) as u64;
        let qq = psllq(psrlq(q, ecx), ecx);
        return mul(sub(x2, mul(qq, y1)), K_2M64);
    }
    if (r10 as i64) < 0 {
        return xb;
    }
    add(xb, yb)
}

/// Double precision fmod as computed by on the AVX2 route (target).
pub fn fmod_x(x: f64, y: f64) -> f64 {
    f(fmod_bits(bt(x), bt(y)))
}

fn fmod_bits(xb: u64, yb: u64) -> u64 {
    let mut rax = xb;
    let mut rcx = ABS & rax;
    let mut r8 = 0x7fef_ffff_ffff_ffffu64;
    let mut r10 = yb;
    let mut rdx = ABS;
    let mut r11 = 0x19u64;
    r8 = r8.wrapping_sub(rcx);
    let mut xmm2 = xb;
    let mut xmm3 = K_MASK;
    rdx &= r10;
    r10 <<= 12;
    rcx = rcx.wrapping_sub(rdx);
    rcx = ((rcx as i64) >> 52) as u64;
    let mut xmm5 = yb;
    r8 |= rcx;
    r11 = r11.wrapping_sub(rcx);
    r8 |= r11;
    r10 >>= 1;
    r10 = r10.wrapping_sub(1);
    let mut xmm4 = r11;
    xmm5 &= xmm3;
    r8 |= r10;
    rdx >>= 52;
    let mut r9 = 0x7fdu64;
    rdx = rdx.wrapping_sub(1);
    r8 |= rdx;
    r9 = r9.wrapping_sub(rdx);
    r8 |= r9;
    if (r8 as i64) >= 0 {
        // 
        xmm2 = div(xmm2, yb);
        rax &= SIGN;
        let mut xmm7 = yb;
        xmm3 = psllq(xmm3, xmm4);
        xmm4 = rax;
        let mut xmm1 = sub(yb, xmm5);
        let xmm6 = ABS;
        xmm5 ^= xmm4;
        xmm2 &= xmm3;
        xmm1 ^= xmm4;
        xmm5 = mul(xmm5, xmm2);
        xmm1 = mul(xmm1, xmm2);
        let mut xmm0 = xb & xmm6;
        xmm0 = sub(xmm0, xmm5);
        xmm7 &= xmm6;
        xmm0 = sub(xmm0, xmm1);
        let m = nle0(xmm0) & xmm7;
        xmm0 = add(xmm0, m);
        return xmm0 | xmm4;
    }
    // 
    r9 |= rdx;
    if (r9 as i64) < 0 {
        return y_special(xb, yb, rdx, r10, r11);
    }
    if (rcx as i64) < 0 {
        return xb;
    }
    if (w3(xb) & 0x7ff0) == 0x7ff0 {
        return inf_nan_x(xb, yb, rdx, r10);
    }
    if (r10 as i64) < 0 {
        return y_pow2(xb, rcx);
    }
    // 
    rdx = ((rdx as u32).wrapping_add(0x1a) as u64) << 4;
    let edx = rdx as u32;
    let mut xmm1 = (yb << 1) >> 1;
    xmm5 = (xmm5 << 1) >> 1;
    xmm4 = sub(xmm1, xmm5);
    xmm2 = (xmm2 << 1) >> 1;
    rax &= SIGN;
    let mut xmm0 = xmm2;
    r11 = 0u64.wrapping_sub(r11);
    if r11 >= 0x3e7 {
        // 
        xmm1 = mul(xmm1, K_2P1023);
        let ecx = edx.wrapping_add(0x3e60);
        xmm5 = mul(xmm5, K_2P1023);
        xmm4 = mul(xmm4, K_2P1023);
        loop {
            xmm2 = div(xmm2, xmm1);
            let mut xmm7 = xmm5;
            xmm2 &= xmm3;
            xmm7 = mul(xmm7, xmm2);
            let mut t = w3(xmm2);
            xmm2 = mul(xmm2, xmm4);
            xmm0 = sub(xmm0, xmm7);
            xmm0 = sub(xmm0, xmm2);
            t &= 0x7ff0;
            t = t.wrapping_sub(0x190);
            let m = nle0(xmm0);
            let xmm6 = mul((t & 0xffff) << 48, xmm1);
            xmm0 = add(xmm0, m & xmm6);
            let r9d = (w3(xmm0) & 0x7ff0) as u32;
            xmm2 = xmm0;
            if r9d <= ecx {
                break;
            }
        }
        xmm1 = mul(mul(xmm1, K_DBLMIN), K_HALF);
        xmm5 = mul(mul(xmm5, K_DBLMIN), K_HALF);
        xmm4 = mul(mul(xmm4, K_DBLMIN), K_HALF);
    }
    // 
    loop {
        xmm2 = div(xmm2, xmm1);
        let mut xmm7 = xmm5;
        xmm2 &= xmm3;
        xmm7 = mul(xmm7, xmm2);
        let mut t = w3(xmm2);
        xmm2 = mul(xmm2, xmm4);
        xmm0 = sub(xmm0, xmm7);
        xmm0 = sub(xmm0, xmm2);
        t &= 0xfff0;
        t = t.wrapping_sub(0x190);
        let m = nle0(xmm0);
        let xmm6 = mul((t & 0xffff) << 48, xmm1);
        xmm0 = add(xmm0, m & xmm6);
        let ecx = (w3(xmm0) & 0x7ff0) as u32;
        xmm2 = xmm0;
        if ecx <= edx {
            break;
        }
    }
    // 
    xmm2 = div(xmm2, xmm1);
    let rcx2 = w3(xmm2) & 0x7ff0;
    if rcx2 < 0x3ff0 {
        return xmm0 | rax;
    }
    let r8b = 0x418u64.wrapping_sub(rcx2 >> 4);
    xmm3 = psllq(xmm3, r8b);
    xmm2 &= xmm3;
    xmm5 = mul(xmm5, xmm2);
    xmm1 = (xmm1 << 1) >> 1;
    xmm2 = mul(xmm2, xmm4);
    xmm0 = sub(xmm0, xmm5);
    xmm0 = sub(xmm0, xmm2);
    let m = nle0(xmm0) & xmm1;
    xmm0 = add(xmm0, m);
    xmm0 | rax
}
