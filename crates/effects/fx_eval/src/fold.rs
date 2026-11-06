//! NV-3: the original compiler's per-class collapse (slot 38) and constant folding.
//!
//! Listing offsets (VA = offset). Sources:
//! - dispatches `alias = prim.slot38(ctx)` for every function node (MEASURED, NV-2).
//! - helper "constant argument is zero": every lane i < elnum of the argument satisfies
//!   -Z < x < Z with Z = double (bits 0x3ddb7cdfd9d7bdbb = 1e-10), -Z .
//!   Non-constant argument (flag byte [desc]) -> false. MEASURED (audit_addr lookup, bytes covered).
//! - helper "constant argument is one": lo < x < hi with hi 
//!   (0x3ff000000006df38 = 1 + 1e-10) and lo (0x3feffffffff24190 = 1 - 1e-10). MEASURED.
//! - slot 38 bodies (return value = the argument index the node collapses to, -1 none):
//!   `*+-`gamma, hue_shift and
//!   hue_shift_preserve_luma, sat_scale, postdivide, blur,
//!   monochrome, element_of, max_element/min_element, coerce_to_alpha
//! rotate, expt, coerce family, comp_rgba,
//!   warp_rel, v3_to_v4_if4 (flag bit, INFERRED never set).
//!   Every other class uses (returns -1) or an UNREAD body (listed in README).
//! - constant folding: (sets b160 = 1, argc = 0) when slot 30 is true,
//!   i.e. all arguments constant and the class not flagged. The lanes are produced by the class kernel
//!   through slot 36 (generic). Arithmetic here: f32 per lane (MEASURED by the bit-exact
//!   comparison against the original's folded lanes in the EP-3 plans); result lanes at or past the
//!   node's elnum are zero (MEASURED: `(1.5, 0.75, 3, 0)`).

use crate::compile::ArgView2;

pub const ZERO_EPS: f64 = f64::from_bits(0x3ddb7cdfd9d7bdbb);
pub const ZERO_EPS_NEG: f64 = f64::from_bits(0xbddb7cdfd9d7bdbb);
pub const ONE_HI: f64 = f64::from_bits(0x3ff000000006df38);
pub const ONE_LO: f64 = f64::from_bits(0x3feffffffff24190);

/// 
pub fn is_zero(a: &ArgView2) -> bool {
    a.is_const && (0..a.n.clamp(1, 4)).all(|i| {
        let x = a.lanes[i] as f64;
        ZERO_EPS > x && x > ZERO_EPS_NEG
    })
}

/// 
pub fn is_one(a: &ArgView2) -> bool {
    a.is_const && (0..a.n.clamp(1, 4)).all(|i| {
        let x = a.lanes[i] as f64;
        ONE_HI > x && x > ONE_LO
    })
}

fn arg(a: &[ArgView2], i: usize) -> Option<&ArgView2> {
    a.get(i)
}

fn z(a: &[ArgView2], i: usize) -> bool {
    arg(a, i).map(is_zero).unwrap_or(false)
}

fn one(a: &[ArgView2], i: usize) -> bool {
    arg(a, i).map(is_one).unwrap_or(false)
}

///  (called by interp slot 38 with indices 0 and 1): both arguments constant and equal within
/// 1e-10 on every lane below the first argument's element count (y + eps > x and x > y - eps, in doubles).
pub fn args_equal(a: &[ArgView2], i: usize, j: usize) -> bool {
    match (arg(a, i), arg(a, j)) {
        (Some(x), Some(y)) if x.is_const && y.is_const => (0..x.n.clamp(1, 4)).all(|k| {
            let (xv, yv) = (x.lanes[k] as f64, y.lanes[k] as f64);
            yv + ZERO_EPS > xv && xv > yv - ZERO_EPS
        }),
        _ => false,
    }
}

/// The complex-one test inlined in cmult's slot 38 : constant, lane 0 inside (1 - 1e-10, 1 + 1e-10) and,
/// when the argument has 2 or more lanes, lane 1 inside (-1e-10, 1e-10).
pub fn is_complex_one(a: &[ArgView2], i: usize) -> bool {
    match arg(a, i) {
        Some(x) if x.is_const => {
            let l0 = x.lanes[0] as f64;
            if !(ONE_HI > l0 && l0 > ONE_LO) {
                return false;
            }
            if x.n < 2 {
                return true;
            }
            let l1 = x.lanes[1] as f64;
            ZERO_EPS > l1 && l1 > ZERO_EPS_NEG
        }
        _ => false,
    }
}

/// DCP1: the count the original stores for a v3_to_v4_if4 node is the element count of its bound image argument (argument 2).
/// None when the argument has no known count.
pub fn v3_count(a: &[ArgView2]) -> Option<i32> {
    a.get(2).and_then(|x| x.elnum)
}

/// DCP1: postdivide's copy_alpha argument (index 1) is a constant with lane 0 below zero. A missing or non-constant argument
/// is not negative (the collapse test reads the stored constant).
pub fn copy_alpha_negative(a: &[ArgView2]) -> bool {
    arg(a, 1).map(|x| x.is_const && x.lanes[0] < 0.0).unwrap_or(false)
}

/// The identity collapses of the non-conditional classes (slot 38). `out_el` is the node's own element
/// count ([this], the combo tag).
pub fn collapse_arm(class: &str, a: &[ArgView2], out_el: Option<i32>) -> Option<usize> {
    // [this] is read as the first argument's element count (NV3-A2): monochrome 4ch->1ch must not collapse
    let el = a.first().and_then(|x| x.elnum).unwrap_or(-99);
    match class {
        // NV-3f, READ: big_mult slot 38 : arg 0 zero -> 0, arg 1 zero -> 1 (the zero constant absorbs the image), arg 0 one -> 1, arg 1 one -> 0: the rule of `*`
        "*" | "big_mult" => {
            if z(a, 0) {
                Some(0)
            } else if z(a, 1) {
                Some(1)
            } else if one(a, 0) {
                Some(1)
            } else if one(a, 1) {
                Some(0)
            } else {
                None
            }
        }
        "+" => {
            if z(a, 1) {
                Some(0)
            } else if z(a, 0) {
                Some(1)
            } else {
                None
            }
        }
        "-" => {
            if z(a, 1) {
                Some(0)
            } else {
                None
            }
        }
        "/" => {
            if z(a, 0) {
                Some(0)
            } else if one(a, 1) {
                Some(0)
            } else {
                None
            }
        }
        // AG1, READ from the listing: dissolve slot 38 . Argument 2 (t) constant with every lane <= 0.0
        // (helper, a float compare, not the 1e-10 test) -> 0; every lane >= 1.0f (helper, constant at
        //  = 1.0f) -> 1; arguments 0 and 1 equal -> 0; else none.
        "dissolve" => {
            if arg(a, 2).map(|t| t.is_const && (0..t.n.clamp(1, 4)).all(|i| t.lanes[i] <= 0.0)).unwrap_or(false) {
                Some(0)
            } else if arg(a, 2).map(|t| t.is_const && (0..t.n.clamp(1, 4)).all(|i| t.lanes[i] >= 1.0)).unwrap_or(false) {
                Some(1)
            } else if args_equal(a, 0, 1) {
                Some(0)
            } else {
                None
            }
        }
        // DSW3, READ: comp_premult slot 38 (vtable slot 38 of class comp_premult): arguments 1 (B) and 2 (T) both constant zero
        // (helper) -> 0 (the node forwards argument 0, A); else -1
        "comp_premult" => (!off("comp_premult") && z(a, 1) && z(a, 2)).then_some(0),
        "gamma" => one(a, 1).then_some(0),
        "hue_shift" | "hue_shift_preserve_luma" => z(a, 1).then_some(0),
        "sat_scale" => {
            if one(a, 1) || (el != -99 && el < 3) {
                Some(0)
            } else {
                None
            }
        }
        // DCP1: the collapse test in full. Collapse only when the copy_alpha argument is a constant below zero
        // and the tag is 1 or 3; a copy_alpha of zero or above keeps the node (the class-3 tail, MEASURED by DCP1).
        "postdivide" => (copy_alpha_negative(a) && (el == 1 || el == 3)).then_some(0),
        // DCP1 v2: the flag word is the low bit of the node's element count (3 in the class-3 compile, 4 and 2 in
        // the others, MEASURED by DCP1's write watch); set: the node forwards argument 0. A node whose arguments 0 and 1 are both
        // constants is left to the constant fold (the original folds Hue_Sat_Bright's two class-3 nodes to the same constants).
        "v3_to_v4_if4" => (!(a.first().map(|x| x.is_const).unwrap_or(false) && a.get(1).map(|x| x.is_const).unwrap_or(false))
            && out_el.map(|e| e & 1 == 1).unwrap_or(false))
        .then_some(0),
        "blur" => (z(a, 1) && one(a, 3)).then_some(0),
        "monochrome" | "element_of" | "max_element" | "min_element" | "coerce_to_alpha" => (el == 1).then_some(0),
        "rotate" => z(a, 1).then_some(0),
        "expt" => {
            if z(a, 0) || one(a, 1) {
                Some(0)
            } else {
                None
            }
        }
        "coerce_up" | "coerce_to" | "coerce_to_v3" | "coerce_to_v4" | "coerce_mono_to_color" | "coerce_up_a1" | "coerce_up_to_v3" | "remove_alpha" => {
            match (arg(a, 0).and_then(|x| x.elnum), out_el) {
                (Some(x), Some(o)) if x == o => Some(0),
                _ => None,
            }
        }
        // the third branch of (tag 3 / tag 1 with [this] == 3) is not decoded
        "comp_rgba" => {
            if z(a, 1) {
                Some(0)
            } else if z(a, 0) {
                Some(1)
            } else {
                None
            }
        }
        // needs [ctx] == 0 (assumed, NV3-A4)
        "warp_rel" => (z(a, 1) && z(a, 2)).then_some(0),
        // NV3b (EP-3b M3 to M8). Each rule is MEASURED BY VALUE DEPENDENCE on the EP-3b dumps (the originals
        // collapse at these values and keep the node at others), the slot-38 body itself is not read: INFERRED.
        // interp a b t: t zero -> a, t one -> b
        // NV-3c, READ from the listing: interp slot 38 : t (arg 2) zero -> 0; t one -> 1; args 0 and 1 equal 
        // -> 0; arg 0 zero and arg 1 one -> 2 (the result is t).
        "interp" => {
            if z(a, 2) {
                Some(0)
            } else if one(a, 2) {
                Some(1)
            } else if args_equal(a, 0, 1) {
                Some(0)
            } else if z(a, 0) && one(a, 1) {
                Some(2)
            } else {
                None
            }
        }
        // NV-3c, READ: add_rgb shares the `+` slot 38 : arg 1 zero -> 0, arg 0 zero -> 1
        "add_rgb" => {
            if z(a, 1) {
                Some(0)
            } else if z(a, 0) {
                Some(1)
            } else {
                None
            }
        }
        // NV-3c, READ: scale_rgb slot 38 . cnt0 = [this] (arg 0 channels), cnt1 = [this] (arg 1 channels).
        // If cnt0 is odd or cnt0 == cnt1: arg 0 zero -> 0, arg 1 zero -> 1. Then: arg 1 one -> 0, arg 0 one -> 1.
        "scale_rgb" => {
            let c0 = a.first().and_then(|x| x.elnum).unwrap_or(-99);
            let c1 = a.get(1).and_then(|x| x.elnum).unwrap_or(-98);
            if (c0 & 1) != 0 || c0 == c1 {
                if z(a, 0) {
                    return Some(0);
                }
                if z(a, 1) {
                    return Some(1);
                }
            }
            if one(a, 1) {
                Some(0)
            } else if one(a, 0) {
                Some(1)
            } else {
                None
            }
        }
        // NV-3c, READ: screen slot 38 : arg 1 zero -> 0, arg 0 zero -> 1, arg 1 one -> 1, arg 0 one -> 0
        "screen" => {
            if z(a, 1) {
                Some(0)
            } else if z(a, 0) {
                Some(1)
            } else if one(a, 1) {
                Some(1)
            } else if one(a, 0) {
                Some(0)
            } else {
                None
            }
        }
        // NV-3c, READ: edge_aware_blur slot 38 : arg 4, 5 or 6 zero -> 0 (NV-3b used only arg 4)
        "edge_aware_blur" => (z(a, 4) || z(a, 5) || z(a, 6)).then_some(0),
        // SHE4, READ: edge_aware_sharpen slot 38 : helper (constant zero within 1e-10) on argument 5 then argument 6;
        // either true -> 0 (the node forwards argument 0), else -1. In Sharpen_Best_Quality arg 5 = tiny_detail_size, arg 6 = edge_threshold.
        "edge_aware_sharpen" => (!off("eas") && (z(a, 5) || z(a, 6))).then_some(0),
        // screen_super_if_needed a b: b zero -> a (the class is its own: it collapses, it is not `screen_super`)
        "screen_super_if_needed" => z(a, 1).then_some(0),
        "copy" => Some(0),
        // alpha_from_rgb_max rgb amount output: forwards rgb when its output element count equals rgb's (Z_Glow: 3 -> 3
        // collapses; Glow: 4 -> 3 stays; both have amount 1, so the earlier amount-zero reading was wrong). INFERRED.
        "alpha_from_rgb_max" => (a.first().and_then(|x| x.elnum).is_some() && a.first().and_then(|x| x.elnum) == out_el).then_some(0),
        // NV3b M12 (Warp_Transform, INFERRED from 2 dumps): host context has shutter 0, so motion_blur is the identity;
        // force_no_cache is a compile hint; tilt is the identity when shift, shear, rotate, tilt2 and the extra
        // vector are zero and the scale is one (args 1,3,4,6,7 zero, 5 one; 2 = perspective_amount is ignored)
        "motion_blur" | "force_no_cache" => Some(0),
        // NV-3c, READ: tilt slot 38 tests args 1, 3, 4 zero, 5 one, 6 zero; arg 7 is NOT tested (NV-3b also required it)
        "tilt" => (z(a, 1) && z(a, 3) && z(a, 4) && one(a, 5) && z(a, 6)).then_some(0),
        // NV-3c, READ: cmult slot 38, c0 = [this], c1 = [this]: arg 1 complex-one and c0 == 2 -> 0;
        // arg 0 complex-one and c1 == 2 -> 1; arg 0 zero and c0 == 2 -> 0; arg 1 zero and c0 == 2 -> 1.
        "cmult" => {
            let c0 = a.first().and_then(|x| x.elnum).unwrap_or(-99);
            let c1 = a.get(1).and_then(|x| x.elnum).unwrap_or(-98);
            if is_complex_one(a, 1) && c0 == 2 {
                Some(0)
            } else if is_complex_one(a, 0) && c1 == 2 {
                Some(1)
            } else if z(a, 0) && c0 == 2 {
                Some(0)
            } else if z(a, 1) && c0 == 2 {
                Some(1)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// How a foldable class is evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldKind {
    Binary,
    Unary,
    Concat,
    Broadcast,
    V3ToV4If4,
    V3ToV4,
    Host,
    Complex,
    Special,
}

/// NV3b (M1): the fold rule is GENERIC. A node folds when its class is pure (no host image, no state) and
/// every argument is constant (slot 30), with the argument exemptions of `can_fold`. This table is the set
/// of classes for which this crate has a constant path (the arithmetic of slot 36); a class that is not in
/// it stays unfolded even with constant arguments (UNKNOWN, listed in the README), which is the only
/// remaining class list. The bool is MEASURED (a folded node of that class is in the EP-3 or EP-3b dumps
/// with bit-exact lanes) versus INFERRED.
pub fn fold_kind(class: &str) -> Option<(FoldKind, bool)> {
    Some(match class {
        "*" | "-" | "/" | "max" | "min" => (FoldKind::Binary, true),
        "+" | "mod" => (FoldKind::Binary, true),
        "screen" => (FoldKind::Binary, false),
        // IN2-F1: diff = |x - y| (INFERRED: the slot 53 body computes |a - b|; the constant path of slot 36 is not measured)
        "diff" => (FoldKind::Binary, false),
        // IN7-F1: round of two constants folds like the other binary arithmetic classes (generic fold rule); the lane arithmetic is INFERRED
        "round" => (FoldKind::Binary, false),
        "abs" | "one_minus" | "negate" | "cos" | "atan" | "log" => (FoldKind::Unary, true),
        "sin" | "tan" | "exp" | "sqrt" => (FoldKind::Unary, false),
        "coerce_up" | "coerce_to" | "coerce_to_v3" => (FoldKind::Broadcast, true),
        // FX-1: the rest of the coerce family folds by the same member passes (rules read from the pass table, not probed); FX1_COERCE_FOLD_OLD=1 leaves them unfoldable
        "coerce_up_a1" | "coerce_to_v4" | "coerce_to_alpha" | "coerce_up_to_v3" if !coerce_old() && !coerce_extra_off() => (FoldKind::Broadcast, false),
        "v2" | "v3" | "v4" => (FoldKind::Concat, true),
        // NV-3g rule F (RULE-INFERRED, DigitalDamage nf1 and nf2): append folds by the same concatenation as v2 v3 v4 (lanes [1 1 1 0] of the original for damage_freq3 = append [1 1] 1)
        "append" => (FoldKind::Concat, false),
        "v3_to_v4" => (FoldKind::V3ToV4, true),
        "v3_to_v4_if4" => (FoldKind::V3ToV4If4, true),
        "get_curve_val" | "get_frame_number" | "get_time" => (FoldKind::Host, true),
        // NV-3f: 30 in all 24 folded nodes of the dumps (HOST-CONTEXT-MINIMAL frame rate)
        "get_frames_per_second" => (FoldKind::Host, true),
        "cmult" | "cexp" => (FoldKind::Complex, true),
        "cdiv" | "clog" => (FoldKind::Complex, false),
        "interp" | "min_element" | "max_element" | "mocha_render" | "get_alpha" | "log_for_mipmap_rotate" => (FoldKind::Special, true),
        // DSW3, READ: blur slot 30 (constant when the opacity, argument 3, is constant zero and the combo tag is odd or equals [this])
        // and slot 36 (writes zero to every output lane); see can_fold_el
        "blur" => (FoldKind::Special, false),
        "clamp" | "mag" | "dist" => (FoldKind::Special, false),
        // NV-3f (EP-3c/EP-3d fix list B): constant paths of the vector helpers on the Rays/EdgeRays trees and the shake tree. MEASURED values (folded lanes of the original dumps); the fold test is the default slot 30 (all arguments constant)
        "channels_equal" | "element_of" => (FoldKind::Special, true),
        "random_sawtooth" => (FoldKind::Special, false),
        "shake_shift" => (FoldKind::Special, false),
        _ => return None,
    })
}

/// Slot 30 with the measured exemptions: the donor of coerce_up/coerce_to (index 1) and of v3_to_v4_if4
/// (index 2) may be an image; `mocha_render` folds whenever its first argument (mocha_project) is zero;
/// `get_alpha` of a value without alpha (1 or 3 elements) folds to 1 whatever the value is (INFERRED from
/// Glow, where get_alpha of alpha_from_rgb_max is folded).
pub fn can_fold(class: &str, a: &[ArgView2]) -> bool {
    can_fold_el(class, a, None)
}

/// Slot 30 with the node's own combo tag (`out_el`[this]) for the classes whose slot 30 reads it.
/// DSW3, READ: blur slot 30 : argument 3 (opacity) constant zero and (tag odd, or tag equal to [this]).
/// READ RULE (EVT1): [this] is argument 0's bound count (setter; OPERAND MEASURED for counts 2 and 4); the Drop_Shadow blurs have tag 1 (odd), so the field is not exercised there.
pub fn can_fold_el(class: &str, a: &[ArgView2], out_el: Option<i32>) -> bool {
    if class == "blur" {
        let tag = match out_el {
            Some(t) => t,
            None => return false,
        };
        // READ RULE: [this] = argument 0's bound count (setter); SHE4_OFF=blur_ac ablates it
        let ac = if off("blur_ac") { -99 } else { a.first().and_then(|x| x.elnum).unwrap_or(-99) };
        return !off("blur_fold") && a.len() >= 4 && is_zero(&a[3]) && ((tag & 1) != 0 || tag == ac);
    }
    let Some((k, _)) = fold_kind(class) else { return false };
    match class {
        "coerce_up" | "coerce_to" | "coerce_up_a1" => a.first().map(|x| x.is_const).unwrap_or(false),
        "v3_to_v4_if4" => a.len() >= 2 && a[0].is_const && a[1].is_const,
        // DSW3, READ: mocha_render slot 30 returns 1 when [this] is null, or [[this] field] is null, or the scale lane [this] or [this] is
        // zero, else 0. INFERRED: [this] is the mocha data of the project, which the minimal host context never supplies (HOST-MOCHA-ABSENT), so the node folds for any
        // mocha_project value; the earlier rule (project zero) is the special case. Probed by the DSW3 batch (p1, p2: mocha_project non-zero).
        "mocha_render" => !a.is_empty() && !off("mocha_fold"),
        "get_alpha" => a.first().map(|x| matches!(x.elnum, Some(1) | Some(3)) || x.is_const).unwrap_or(false),
        _ => k == FoldKind::Host || (!a.is_empty() && a.iter().all(|x| x.is_const)),
    }
}

/// kept for NV-3 API compatibility
pub fn fold_arg_required(class: &str, i: usize) -> bool {
    !((class == "v3_to_v4_if4" && i >= 2) || ((class == "coerce_up" || class == "coerce_to") && i >= 1))
}

fn lane(a: &ArgView2, i: usize) -> f32 {
    if a.n <= 1 {
        a.lanes[0]
    } else {
        a.lanes[i.min(3)]
    }
}

/// complex view of an argument: two lanes are (re, im), a scalar is (x, 0) (MEASURED: `cmult alpha I` gives (0, alpha))
fn cx(a: &ArgView2) -> (f32, f32) {
    (a.lanes[0], if a.n >= 2 { a.lanes[1] } else { 0.0 })
}

fn math1(class: &str, x: f32) -> f32 {
    match class {
        "abs" => x.abs(),
        "one_minus" => 1.0 - x,
        "negate" => -x,
        "cos" => crate::svml_trig::cos_rev(x),
        // INT3: NV-3g rules E (atan in revolutions) and E2 (log(2.0) lane) are NOT merged; atan and log keep the plain f32 forms (correctly rounded by detmath) until the one-argument origin is traced (CMP2e M9)
        "atan" => crate::detmath::atan(x),
        "log" => crate::detmath::ln(x),
        "sin" => crate::svml_trig::sin_rev(x),
        "tan" => crate::detmath::tan(x),
        "exp" => crate::detmath::exp(x),
        _ => {
            // CMP2e/CMP3: MEASURED (17 of 18 grid atoms with the CMP2e form, 18 of 18 with this form): a NaN or negative argument folds to +0, +-0 keeps its sign (sqrt(-0) = -0)
            if x >= 0.0 { x.sqrt() } else { 0.0 }
        }
    }
}

/// Folded lanes (f32 bits) for `class` over constant arguments; None when not foldable.
/// INT22 step B: the form of a folded v3_to_v4_if4 node. M4: n = 4 (the 4 channel arm); M3Z: n = 3 with a scalar
/// argument 0 and every argument 1 lane bit equal to +0.0 (HSR2's h2F, h2F2 class-3 originals); M2S: n = 2 with scalar arguments (BCJ3's Blur_Mono
/// originals; a vector argument 1 with equal lanes counts as scalar); INT23, widened by INT24: M3V: n = 3, argument 0
/// a 3-lane vector (any lanes), argument 1 a scalar (or a vector with bit-equal lanes) bit equal to 1.0 (0x3f800000); result the three lanes of
/// argument 0, argument 1 not placed (HSR1 hsBR and hsR0 re-scores at equal lanes; FLD2 bindings A and B at distinct lanes); every other form is UNMEASURED-FORM and is refused on the verified render path
/// (V3TOV4IF4-NARROW-UNMEASURED) by fx_run.
pub fn v3_form(a: &[ArgView2], out_el: i32) -> &'static str {
    let n = out_el.clamp(1, 4) as usize;
    if a.len() < 2 {
        return "UNMEASURED-FORM";
    }
    let k1 = a[1].n.clamp(1, 4);
    let a1_zero = (0..k1).all(|j| a[1].lanes[j].to_bits() == 0);
    let a1_equal = (0..k1).all(|j| a[1].lanes[j].to_bits() == a[1].lanes[0].to_bits());
    match n {
        4 => "M4",
        3 if a[0].n <= 1 && a1_zero => "M3Z",
        // INT24 widening: any 3-lane argument 0 (FLD2 bindings A and B, distinct lanes); argument 1 other than 1.0 stays refused
        3 if a[0].n == 3 && a1_equal && a[1].lanes[0].to_bits() == 0x3f80_0000 => "M3V",
        // FLD3-FORM: n = 3, a one-lane argument 0 and a one-lane argument 1. BCJ5's FLD3 records (Blur_Color_Autogen node 67; brightness 1.0,
        // 1.21 and 1.21 with scale_opacity 1.0, 1.0 and 0.6875): the folded constant is argument 0 broadcast to lanes 0 to 2 and argument 1 in lane 3.
        3 if a[0].n <= 1 && a[1].n <= 1 && !cfg!(feature = "fld3_form_kept_mut") => "M3B",
        // FLD3M (BCJ5's node 68 records gA to gD): n = 1 and one-lane arguments: the folded constant is the pair [a0 a1] (measured [1 1], [1 0.6875] and
        // [1.21 0.6875]; lane 0 is argument 0 (brightness), lane 1 argument 1 (scale_opacity)). Any other shape stays refused.
        1 if a[0].n <= 1 && a[1].n <= 1 && !cfg!(feature = "fld3m_kept_mut") => "M1P",
        2 if a[0].n <= 1 && (a[1].n <= 1 || a1_equal) => "M2S",
        _ => "UNMEASURED-FORM",
    }
}

thread_local! {
    /// INT22 step B: every v3_to_v4_if4 fold of the last compile on this thread: (node, n, argument 0 lanes count, argument 1 lane bits, result bits, form,
    /// argument 0 lane bits (INT23)).
    static V3_FOLDS: std::cell::RefCell<Vec<(usize, usize, usize, Vec<u32>, [u32; 4], &'static str, Vec<u32>)>> = std::cell::RefCell::new(Vec::new());
}

/// Note one folded v3_to_v4_if4 node (called by compile at the fold site).
pub fn v3_note(id: usize, a: &[ArgView2], out_el: i32, res: &[u32; 4]) {
    let n = out_el.clamp(1, 4) as usize;
    let a0n = a.first().map(|x| x.n).unwrap_or(0);
    let a1: Vec<u32> = a.get(1).map(|x| (0..x.n.clamp(1, 4)).map(|j| x.lanes[j].to_bits()).collect()).unwrap_or_default();
    let f = v3_form(a, out_el);
    let a0: Vec<u32> = a.first().map(|x| (0..x.n.clamp(1, 4)).map(|j| x.lanes[j].to_bits()).collect()).unwrap_or_default();
    V3_FOLDS.with(|v| v.borrow_mut().push((id, n, a0n, a1, *res, f, a0)));
}

/// The folds noted since the last take, reset.
pub fn v3_take() -> Vec<(usize, usize, usize, Vec<u32>, [u32; 4], &'static str, Vec<u32>)> {
    V3_FOLDS.with(|v| std::mem::take(&mut *v.borrow_mut()))
}

pub fn fold_lanes(class: &str, a: &[ArgView2], out_el: i32, host: Option<&[f32]>) -> Option<[u32; 4]> {
    let (k, _) = fold_kind(class)?;
    let n = out_el.clamp(1, 4) as usize;
    let mut out = [0f32; 4];
    match k {
        FoldKind::Binary => {
            if a.len() < 2 {
                return None;
            }
            for i in 0..n {
                let (x, y) = (lane(&a[0], i), lane(&a[1], i));
                out[i] = match class {
                    "*" => x * y,
                    "+" => x + y,
                    // 1 - (1 - x) (1 - y), INFERRED (Glow: screen of two zeros folds)
                    // CMP2e: MEASURED (81 of 81 exact-atom cases): x + y (1 - x), not 1 - (1 - x)(1 - y)
                    "screen" => x + y * (1.0 - x),
                    "-" => x - y,
                    "diff" => diff_const(x, y),
                    // IN7-A4: the (3,4) and (4,3) operand pairs write lane 3 by the alpha synthesis and drop loops of the class body
                    "round" => {
                        let (na, nb) = (a[0].n, a[1].n);
                        if i == 3 && na == 3 && nb == 4 {
                            snap_one_over_const(y)
                        } else if i == 3 && na == 4 && nb == 3 {
                            snap_unit_const(x)
                        } else {
                            snap_const(x, y)
                        }
                    }
                    // NV-3f: a zero divisor is replaced by 1e-6 (Rays rays_length 0: 1 / [0 0 0] folds to 1e6 = 0x49742400 per lane, MEASURED for numerator 1; the clamp threshold for other small divisors is UNKNOWN)
                    // CMP2e: MEASURED through the original compile (work/cmp): a zero divisor of either sign is replaced by +1e-6 (1 / -0 folds to +1e6)
                    "/" => x / if y == 0.0 { 1e-6f32 } else { y },
                    // GLSL style mod (INFERRED, one sample: 1 mod 2 = 1)
                    "mod" => x - y * (x / y).floor(),
                    "max" => {
                        if x > y {
                            x
                        } else {
                            y
                        }
                    }
                    _ => {
                        if x < y {
                            x
                        } else {
                            y
                        }
                    }
                };
            }
        }
        FoldKind::Unary => {
            let x0 = a.first()?;
            for i in 0..n {
                out[i] = math1(class, lane(x0, i));
            }
        }
        FoldKind::Broadcast => {
            let x0 = a.first()?;
            if coerce_old() {
                for i in 0..n {
                    out[i] = lane(x0, i);
                }
            } else {
                out = coerce_lanes(class, x0, n);
            }
        }
        FoldKind::Concat => {
            let mut k = 0;
            for x in a {
                for j in 0..x.n.clamp(1, 4) {
                    if k < n {
                        out[k] = x.lanes[j];
                        k += 1;
                    }
                }
            }
        }
        FoldKind::V3ToV4If4 => {
            if a.len() < 2 {
                return None;
            }
            // FLD3 wrong-form mutants (build-time features): the candidate forms BCJ5's records exclude (lane 3 of each is the candidate's own prediction, 1.0)
            if cfg!(feature = "fld3_bcast_mut") && n == 3 && a[0].n <= 1 {
                out[..3].fill(lane(&a[0], 0));
                out[3] = 1.0;
            } else if cfg!(feature = "fld3_ownzeros_mut") && n == 3 && a[0].n <= 1 {
                out[0] = lane(&a[0], 0);
                out[3] = 1.0;
            } else if cfg!(feature = "fld3_concat_mut") && n == 3 && a[0].n <= 1 {
                out[0] = lane(&a[0], 0);
                out[1] = lane(&a[1], 0);
                out[3] = 1.0;
            } else if cfg!(feature = "fld3_product_mut") && n == 3 && a[0].n <= 1 {
                for k in 0..3 {
                    out[k] = lane(&a[0], k) * lane(&a[1], k);
                }
                out[3] = 1.0;
            } else if n == 3 && a[0].n <= 1 && !cfg!(feature = "brd2_lane0_mut") || cfg!(feature = "brd2_wide_mut") && n < 4 && a[0].n <= 1 {
                // BRD2 (narrowed from BRD1): with exactly 3 output channels (the 3 plane class-3 layout) a scalar first argument is broadcast over the 3 lanes, as the 4 channel arm below does with lane() (HSR2 h2F and h2F2 end-to-end originals: original minus native = the fed Offset Darks on planes 2 and 3).
                // With 2 channels the original keeps the concatenation [a0 a1]: BCJ3's 8 Blur_Mono originals (BM, COCI, CO, CI at 64x32 and 50x7) are equal to the concat form and differ from the broadcast form (v3_to_v4_if4 0.7417 0.7319 gives [0.7417 0.7319]).
                for k in 0..n {
                    out[k] = lane(&a[0], k);
                }
                // FLD3-FORM: a one-lane argument 1 is placed in lane 3 (zero for the earlier measured M3Z form, so unchanged there)
                if n == 3 && a[1].n <= 1 {
                    out[3] = a[1].lanes[0];
                }
            } else if n < 4 {
                // NV-3f (class F): when the node is not 4-channel (donor without 4 channels) the result is the concatenation of the first two arguments (Blur_Mono: v3_to_v4_if4 1 1 bw is [1 1] with 2 channels in the original, MEASURED node 14; the donor channel rule itself is UNKNOWN)
                let mut k = 0;
                for x in a.iter().take(2) {
                    for j in 0..x.n.clamp(1, 4) {
                        if k < n {
                            out[k] = x.lanes[j];
                            k += 1;
                        }
                    }
                }
                // FLD3M: the measured n = 1 form places argument 1 in lane 1 (candidate-form mutants: one lane only, broadcast of argument 0, product in lane 0)
                if n == 1 && a[0].n <= 1 && a[1].n <= 1 {
                    if cfg!(feature = "fld3m_onelane_mut") {
                    } else if cfg!(feature = "fld3m_bcast4_mut") {
                        out[1] = a[0].lanes[0];
                    } else if cfg!(feature = "fld3m_swapped_mut") {
                        out[0] = a[1].lanes[0];
                        out[1] = a[0].lanes[0];
                    } else if cfg!(feature = "fld3m_product_mut") {
                        out[0] = a[0].lanes[0] * a[1].lanes[0];
                    } else {
                        out[1] = a[1].lanes[0];
                    }
                }
                // INT23: mutant only: argument 1 into the last lane at n = 3 for a vector argument 0; must be killed by HSR1's hsBR and hsR0.
                if cfg!(feature = "int23_a1last_mut") && n == 3 && a[0].n > 1 {
                    out[2] = a[1].lanes[0];
                }
                // INT24: mutant only: lane 0 of a 3-lane argument 0 broadcast at n = 3; must be killed by FLD2's two native compiles.
                if cfg!(feature = "int24_lane0v_mut") && n == 3 && a[0].n == 3 {
                    out[1] = a[0].lanes[0];
                    out[2] = a[0].lanes[0];
                }
            } else {
                out[0] = lane(&a[0], 0);
                out[1] = lane(&a[0], 1);
                out[2] = lane(&a[0], 2);
                out[3] = a[1].lanes[0];
            }
        }
        FoldKind::V3ToV4 => {
            if a.len() < 2 {
                return None;
            }
            for i in 0..3 {
                out[i] = lane(&a[0], i);
            }
            out[3] = a[1].lanes[0];
        }
        FoldKind::Host => {
            if let Some(h) = host {
                for i in 0..n.min(h.len()) {
                    out[i] = h[i];
                }
            } else if class == "get_frames_per_second" {
                out[0] = 30.0;
            }
        }
        FoldKind::Complex => match class {
            "cmult" => {
                let ((ar, ai), (br, bi)) = (cx(a.first()?), cx(a.get(1)?));
                out[0] = ar * br - ai * bi;
                out[1] = ar * bi + ai * br;
            }
            "cdiv" => {
                let ((ar, ai), (br, bi)) = (cx(a.first()?), cx(a.get(1)?));
                let d = br * br + bi * bi;
                out[0] = (ar * br + ai * bi) / d;
                out[1] = (ai * br - ar * bi) / d;
            }
            "cexp" => {
                let (re, im) = cx(a.first()?);
                let e = crate::detmath::exp(re);
                let (s, c) = crate::detmath::sin_cos(im);
                out[0] = e * c;
                out[1] = e * s;
            }
            _ => {
                let (re, im) = cx(a.first()?);
                out[0] = crate::detmath::ln((re * re + im * im).sqrt());
                out[1] = crate::detmath::atan2(im, re);
            }
        },
        FoldKind::Special => match class {
            // a*(1-t) + b*t (INFERRED form; zero and one t collapse before this)
            "interp" => {
                let (x, y, t) = (a.first()?, a.get(1)?, a.get(2)?);
                for i in 0..n {
                    let tt = lane(t, i);
                    // CMP2e: candidate a + (b - a) t (the a (1 - t) + b t form gives NaN for a = 0, t = inf where the original gives inf); kept only if the rerun shows 0 differing
                    out[i] = lane(x, i) + (lane(y, i) - lane(x, i)) * tt;
                }
            }
            "min_element" | "max_element" => {
                let x = a.first()?;
                let m = x.n.clamp(1, 4);
                let mut r = x.lanes[0];
                for i in 1..m {
                    // CMP2e: MEASURED (995 of 995 cases with NaN and signed zero): minss(r, x) = r < x ? r : x and maxss(r, x) = r > x ? r : x, not Rust f32::min / max
                    let xi = x.lanes[i];
                    r = if class == "min_element" { if r < xi { r } else { xi } } else if r > xi { r } else { xi };
                }
                out[0] = r;
            }
            // MEASURED once: Glow, mocha_project 0 gives the constant 1
            // DSW3, READ: mocha_render slot 36 (AVX2 body, SSE bodies): every output lane is the float 
            // (1.0f), replaced by +0.0 when [this] == 0 or [this] == 0 (vucomiss against zero; NaN is not equal; -0 is equal). INFERRED: those two cells
            // are the two lanes of argument 1 (the scale); MEASURED by n6 (scale [0 4] gives 0) against d0 (scale [1 1] gives 1)
            "mocha_render" => {
                let s = a.get(1);
                let zero_x = s.map(|v| lane(v, 0) == 0.0).unwrap_or(false);
                let zero_y = s.map(|v| lane(v, 1) == 0.0).unwrap_or(false);
                out[0] = if zero_x || zero_y { 0.0 } else { 1.0 };
            }
            "get_alpha" => {
                let x = a.first()?;
                out[0] = match x.elnum {
                    Some(e @ (2 | 4)) if x.is_const => x.lanes[(e - 1) as usize],
                    _ => 1.0,
                };
            }
            // FITTED on one sample pair (Warp_Chroma): (rotate 0, z_dist 1.5) gives 0.25 on both lanes and
            // (rotate 0, z_dist 1) gives 0. Only the rotate == 0 line is modelled: (z - 1) / 2. UNKNOWN beyond.
            "log_for_mipmap_rotate" => {
                let (r, z) = (a.first()?, a.get(1)?);
                if !is_zero(r) {
                    return None;
                }
                let v = (z.lanes[0] - 1.0) * 0.5;
                out[0] = v;
                out[1] = v;
            }
            "clamp" => {
                let (x, lo, hi) = (a.first()?, a.get(1)?, a.get(2)?);
                for i in 0..n {
                    out[i] = clamp_const(lane(x, i), lane(lo, i), lane(hi, i));
                }
            }
            // NV-3f: 1 when every channel of the argument is equal, else 0 (Rays a: channels_equal [0 0 0] folds to 1; MEASURED value of the folded lane, 1-element result)
            "channels_equal" => {
                let x = a.first()?;
                let m = x.n.clamp(1, 4);
                // CMP2e: MEASURED (512 cases; two differ without the finite test): equal infinities do not count as equal (the original tests a difference, inf - inf is NaN); the epsilon of that test is NOT measured (INFERRED: exact equality of finite lanes)
                out[0] = if (1..m).all(|i| x.lanes[i] == x.lanes[0] && x.lanes[0].is_finite()) { 1.0 } else { 0.0 };
            }
            // NV-3f: lane `idx` of the vector (element_of [0.25 0.25 0.25] 1 = 0.25, MEASURED in EdgeRays); the index is truncated and clamped to the lanes (clamp rule INFERRED)
            "element_of" => {
                let (x, ix) = (a.first()?, a.get(1)?);
                let m = x.n.clamp(1, 4);
                // CMP2e: MEASURED (10 of 10): the index is cvttss2si (NaN or out of range gives i32::MIN) taken modulo the lane count with a non-negative result
                let cv = { let t = ix.lanes[0]; if t.is_nan() || t >= 2147483648.0 || t < -2147483648.0 { i32::MIN } else { t as i32 } };
                let k = (cv as i64).rem_euclid(m as i64) as usize;
                out[0] = x.lanes[k];
            }
            // NV-3f: random_sawtooth seed phase: 0 at phase 0 (the minimal host context has time 0; MEASURED in the Shake_Normal dumps, 24 folded lanes); other phases stay unfolded (UNKNOWN)
            "random_sawtooth" => {
                let ph = a.get(1)?;
                // NV-3g rule S (RULE-INFERRED, DigitalDamage nf1 and nf2): the sawtooth is 0 at every INTEGER phase, not only at phase 0 (blink_brights uses phase (0 - 1.5) * 10 = -15 and the original folds it to 0, as it does the 9 others; non-integer phases stay unfolded, UNKNOWN)
                if !(0..ph.n.clamp(1, 4)).all(|i| ph.lanes[i].is_finite() && ph.lanes[i] == ph.lanes[i].floor()) {
                    return None;
                }
            }
            // NV-3f: every shake_shift lane of the original is NaN 0xffc00000 at the minimal host context (146 folded nodes in Shake_Normal a and b, MEASURED); WHY (missing time/fps/duration vs the body) is UNKNOWN, so this is FITTED to the minimal context
            "shake_shift" => {
                for i in 0..n {
                    out[i] = f32::from_bits(0xffc0_0000);
                }
            }
            // DSW3, READ: blur slot 36 writes 0 to every lane (loops)
            "blur" => {}
            "mag" => {
                let x = a.first()?;
                let mut s = 0f32;
                for i in 0..x.n.clamp(1, 4) {
                    s += x.lanes[i] * x.lanes[i];
                }
                out[0] = s.sqrt();
            }
            _ => {
                let (x, y) = (a.first()?, a.get(1)?);
                let mut s = 0f32;
                for i in 0..x.n.clamp(1, 4) {
                    let d = x.lanes[i] - lane(y, i);
                    s += d * d;
                }
                out[0] = s.sqrt();
            }
        },
    }
    // FLD3-FORM: the measured n = 3 form carries argument 1 in lane 3 of the original's folded constant; no other form keeps a lane at or beyond n
    let keep3 = class == "v3_to_v4_if4" && n == 3 && a.len() >= 2 && a[0].n <= 1 && a[1].n <= 1;
    let keep1 = class == "v3_to_v4_if4" && n == 1 && a.len() >= 2 && a[0].n <= 1 && a[1].n <= 1;
    for i in n..4 {
        if !((keep3 && i == 3) || (keep1 && i == 1)) {
            out[i] = 0.0;
        }
    }
    Some([out[0].to_bits(), out[1].to_bits(), out[2].to_bits(), out[3].to_bits()])
}

/// FX-1: `FX1_COERCE_FOLD_OLD=1` restores the pre-FX-1 behaviour (every coerce fold replicates the last lane, the extra coerce classes stay unfolded).
pub fn coerce_old() -> bool {
    std::env::var("FX1_COERCE_FOLD_OLD").map(|v| v == "1").unwrap_or(false)
}

/// FX-1: the lanes of a folded coerce family node, from the member passes of wave_s6/rebuild_ops_coerce (CO1, R-PARITY at the pass entry,
/// ) selected by the (arg0 channels, output channels) combo rows of EV-T COMBOS.tsv.
/// C_CELL_A copy channels c < out; D_CELL_A splat channel 0; DE_CELL_A (ch0, ch1, 0.0); E_CELL_A channel 0 for c < out-1, last = 1.0; EF_CELL_A (ch0, ch1, ch2, 1.0);
/// F_CELL_A (ch0, ch0, ch0, ch1); P_CELL_A the LAST channel. MEASURED by probe (CO1 PLAN_ORIG_TrailsCU_V3/S1/V2): coerce_up 3 to 4 gives alpha 1.0,
/// scalar splat, 2 to 4 gives (g, g, g, a). The other rows are read from the pass table, not probed.
pub fn coerce_lanes(class: &str, x: &ArgView2, out_n: usize) -> [f32; 4] {
    let n = x.n.clamp(1, 4);
    let s = |i: usize| x.lanes[i.min(n - 1)];
    let mut o = [0f32; 4];
    if class == "coerce_to_alpha" {
        // combos 1 and 3 use C_CELL_A with tag 1 (channel 0), 2 and 4 use P_CELL_A (the last channel)
        o[0] = if n % 2 == 1 { s(0) } else { s(n - 1) };
        return o;
    }
    let a1 = class == "coerce_up_a1" || class == "coerce_to_v4";
    let m = out_n.clamp(1, 4);
    if n >= m {
        for i in 0..m {
            o[i] = s(i);
        }
    } else if n == 1 {
        // a1 and v4: E_CELL_A (rows (1,2) and (1,4)); row (1,3) is D_CELL_A in every class
        let e_cell_a = a1 && (m == 2 || m == 4);
        for i in 0..m {
            o[i] = if e_cell_a && i == m - 1 { 1.0 } else { s(0) };
        }
    } else if n == 2 && m == 3 {
        o[0] = s(0);
        o[1] = s(1);
        o[2] = 0.0;
    } else if n == 2 && m == 4 {
        o = [s(0), s(0), s(0), s(1)];
    } else if n == 3 && m == 4 {
        o = [s(0), s(1), s(2), 1.0];
    } else {
        for i in 0..m {
            o[i] = s(i);
        }
    }
    o
}

/// FX-1: `FX1_COERCE_EXTRA_OFF=1` keeps the lane fix for coerce_up / coerce_to / coerce_to_v3 but leaves coerce_up_a1, coerce_to_v4,
/// coerce_to_alpha and coerce_up_to_v3 unfolded as before (their fold rules are read from the pass table, not probed).
pub fn coerce_extra_off() -> bool {
    std::env::var("FX1_COERCE_EXTRA_OFF").map(|v| v == "1").unwrap_or(false)
}

/// IN4A: the constant path of `diff` is the class's own element function (crate `diff`, K-PARITY and R-PARITY, IN2): an f32 `vsubss`
/// followed by `vandps` with the abs mask OP2-A3 (0x7fffffff, MEASURED-DYLIB), i.e. |x - y| computed in f32 with the sign bit cleared
/// (a NaN keeps its payload apart from the sign bit). Replaces IN2's INFERRED fold flag: the executor's wrapper and this fold agree
/// bit for bit (checked in fx_exec `tests/t13.rs` against the crate for finite, special and NaN pairs).
pub fn diff_const(x: f32, y: f32) -> f32 {
    f32::from_bits((x - y).to_bits() & 0x7fff_ffff)
}

/// IN7: the constant path of `round(x, y)` (snap x down to a multiple of y) is taken to be the element kernel of the class body 
/// (crate `scale_rgb::snap`): y == 0 returns x; x >= 0 or NaN gives `(float)((double)y * (double)floorf(x / y))`; x < 0 gives
/// `(float) fma(ceilf(x / y), (double)y, -|y|)`. INFERRED (IN7-A4): that the constant path of slot 36 runs the same arithmetic as the pixel
/// body is not measured for this class (it is MEASURED for `+ - * / min max`). `fx_exec` `tests/t14.rs` compares these three functions with
/// the crate over special values (NaN as a class).
pub fn snap_const(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        return x;
    }
    let yd = y as f64;
    if !(x < 0.0) {
        (yd * (x / y).floor() as f64) as f32
    } else {
        ((x / y).ceil() as f64).mul_add(yd, -(y.abs() as f64)) as f32
    }
}

/// alpha synthesis lane of `round` with a 3 lane first and a 4 lane second operand: `y * floor(1 / y)` in double, 1.0 when y == 0.
pub fn snap_one_over_const(y: f32) -> f32 {
    if y == 0.0 {
        1.0
    } else {
        ((y as f64) * ((1.0f32 / y) as f64).floor()) as f32
    }
}

/// alpha drop lane of `round` with a 4 lane first and a 3 lane second operand: `floor(x)` (x >= 0 or NaN), `ceil(x) - 1` (x < 0), in double.
pub fn snap_unit_const(x: f32) -> f32 {
    let xd = x as f64;
    if !(x < 0.0) { xd.floor() as f32 } else { (xd.ceil() - 1.0) as f32 }
}

/// IN4A: the constant path of `clamp(x, lo, hi)` is the class's own median ladder, not `max(x, lo).min(hi)` (IN2 finding: the max/min
/// form disagrees with the native body when lo > hi and for NaN). The ladder is the all-plane shape (flags 000,
/// crate `clamp` `Shape::S000` scalar path, 1 element): with b = lo, c = hi, a = x:
/// `if c > b { if b > a { b } else { minss(c, a) } } else if c > a { c } else { minss(b, a) }` where `minss(p, q) = if p < q { p } else { q }`
/// (vminss: the second operand on unordered). The shape the original host uses for a node whose three arguments are constants is
/// INFERRED (the constant path of slot 36 is not measured); on finite values every shape returns the median of the three operands, so
/// the choice only decides NaN and tie cases. fx_exec `tests/t13.rs` compares this function with `clamp::render` for shape 000.
pub fn clamp_const(x: f32, lo: f32, hi: f32) -> f32 {
    let minss = |p: f32, q: f32| if p < q { p } else { q };
    let (a, b, c) = (x, lo, hi);
    if c > b {
        if b > a {
            b
        } else {
            minss(c, a)
        }
    } else if c > a {
        c
    } else {
        minss(b, a)
    }
}

#[cfg(test)]
mod in4a_tests {
    use super::*;

    #[test]
    fn clamp_const_is_the_median_for_inverted_bounds() {
        // IN2 finding: x = 3, lo = 5, hi = 1: the old fold gave max(3,5).min(1) = 1, the native body returns the median 3
        assert_eq!(clamp_const(3.0, 5.0, 1.0), 3.0);
        assert_eq!(clamp_const(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp_const(-2.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp_const(7.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp_const(0.25, 1.0, 0.0), 0.25);
    }

    #[test]
    fn clamp_const_matches_median_on_a_finite_grid() {
        let v = [-2.0f32, -1.0, 0.0, 0.5, 1.0, 3.0];
        for &a in &v {
            for &b in &v {
                for &c in &v {
                    let mut s = [a, b, c];
                    s.sort_by(|p, q| p.partial_cmp(q).unwrap());
                    assert_eq!(clamp_const(a, b, c), s[1], "{} {} {}", a, b, c);
                }
            }
        }
    }

    #[test]
    fn clamp_const_nan_follows_the_ladder_not_rust_max_min() {
        // x NaN, 0 <= 1: c > b, b > a false, minss(c, a) = c < NaN false -> a = NaN (the old fold gave 0.0 for NaN.max(0).min(1))
        assert!(clamp_const(f32::NAN, 0.0, 1.0).is_nan());
        assert!(f32::NAN.max(0.0).min(1.0) == 0.0);
    }

    #[test]
    fn diff_const_is_abs_difference() {
        assert_eq!(diff_const(1.0, 3.5), 2.5);
        assert_eq!(diff_const(3.5, 1.0), 2.5);
        assert_eq!(diff_const(0.0, -0.0).to_bits(), 0);
        assert!(diff_const(f32::NAN, 1.0).is_nan());
        assert_eq!(diff_const(f32::INFINITY, 1.0), f32::INFINITY);
    }
}

/// SHE4 ablation switches (kill checks only): env SHE4_OFF is a comma list of rule names (eas, clone, mocha_fold, blur_fold, comp_premult).
pub fn off(name: &str) -> bool {
    std::env::var("SHE4_OFF").map(|v| v.split(',').any(|x| x == name)).unwrap_or(false)
}
