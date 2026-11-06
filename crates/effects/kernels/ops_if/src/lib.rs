//! ops_if: native port of the select operators `if+` and `if>`.
//!
//! Source of truth: the slot 52 AVX2 bodies of the original (if+, if>, listing offsets in
//! neutral_1.asm; captured VA = listing). Both are one template: per output channel `ch`per element `i`
//!   out[ch][i] = cond(i) ? x[ch][i] : y[ch][i]       (bit copies, no arithmetic)
//! with cond = (c > 0) for `if+ c x y` and (a > b) for `if> a b x y`, an ordered compare (NaN gives false, so the y arm).
//! Operands are an image (planes of `n` floats) or a constant (4 floats), exactly as the host fills the object.
//! Std only, no unsafe, no Default on parameters. Assumption prefix -.

pub const PREFIX: &str = "OP3-";
/// Output planes start filled with this NaN pattern in the harness (img_oracle SENTINEL_BITS).
pub const SENT: u32 = 0x7fc0ffee;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// `if+ c x y`: operands [c, x, y]
    Plus,
    /// `if> a b x y`: operands [a, b, x, y]
    Gt,
}

/// One operand of the node as the host fills it: channel count, "is a constant" flag, 4 constants (bits) and the plane record.
#[derive(Clone, Debug)]
pub struct Operand {
    pub count: i32,
    pub is_const: bool,
    pub consts: [u32; 4],
    /// `None` = null record pointer. Planes are bit patterns of f32.
    pub planes: Option<Vec<Vec<u32>>>,
}

#[derive(Clone, Debug)]
pub struct Params {
    /// output channel count (virtual slot 31 =, hooked; OP3-A4)
    pub nout: i32,
    /// elements per plane, info
    pub n: i32,
    /// the captured 1.0f (OP3-A1)
    pub one: u32,
    /// all-ones vector mask (OP3-A2): only the sign bit of each lane matters; every lane is assumed equal
    pub k_mask: u32,
    /// destination plane address & 0x1f in bytes (OP3-A9), same for every channel
    pub dst_phase: usize,
    /// initial content of the destination planes (the harness sentinel)
    pub dst_init: u32,
}

pub struct Assumption {
    pub id: &'static str,
    pub tier: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "OP3-A1", tier: "CAPTURED", text: "ONE = 1.0f, used for the missing alpha of an odd-count x or y operand at channel 3" },
        Assumption { id: "OP3-A2", tier: "SEMANTIC", text: "vector mask (32 bytes, uncovered): all ones. Basis: vxorps K,cmpmask is used only as a vmaskmovps/vblendvps control (sign bit); the scalar head and tail use the complement test (jbe), so the vector else-lanes must be !mask. Explicit parameter k_mask, T2 runs all-ones and zero" },
        Assumption { id: "OP3-A3", tier: "JUMPTABLE-DERIVED", text: "the dispatch jump tables (if+, if>, 16 qwords of offsets) are uncovered; the cell is a data stand-in on both sides, each index mapped to the case entry whose emulated behaviour equals the model for that flag word (bijection checked)" },
        Assumption { id: "OP3-A4", tier: "HOST-HOOKED", text: "virtual slot 31 is the output channel count nout; hooked ret_const, meaning INFERRED from the loop bound" },
        Assumption { id: "OP3-A5", tier: "INFERRED", text: "object layout (operand count at, the flag field, constants, plane records, plane c at the record's per-channel pointer) MEASURED from the loads; that the host fills it so is INFERRED" },
        Assumption { id: "OP3-A6", tier: "LIBC-MODEL", text: "memcpy thunk (whole-plane arm copy when the condition is constant) hooked as memcpy" },
        Assumption { id: "OP3-A7", tier: "ROUTE", text: "AVX2 route only (word 0x9d97ff); AVX and SSE tiers not compared; HOST-FP-ASSUMED (no arithmetic in the bodies, only ordered compares)" },
        Assumption { id: "OP3-A8", tier: "CONTRACT", text: "channel counts 1..4, nout <= 4 (constants are read at index ch), n >= 0 when a whole-plane copy runs, destination planes disjoint from the sources, flag (rcx) = 0" },
        Assumption { id: "OP3-A9", tier: "ALIGN-DEPENDENT", text: "the vector region (peel from the destination phase) is visible only when k_mask is not all ones; dst_phase explicit" },
    ]
}

fn gt(l: u32, r: u32, m: Option<&str>) -> bool {
    let (l, r) = (f32::from_bits(l), f32::from_bits(r));
    match m {
        Some("cmp_ge") => l >= r,
        Some("nan_true") => !(l <= r),
        _ => l > r,
    }
}

fn plane<'a>(o: &'a Operand, idx: i32, n: usize) -> Result<&'a [u32], String> {
    let pl = o.planes.as_ref().ok_or_else(|| "plane read through a null record".to_string())?;
    if idx < 0 || idx as usize >= pl.len() {
        return Err(format!("plane index {} outside the record ({} planes)", idx, pl.len()));
    }
    let p = &pl[idx as usize];
    if p.len() < n {
        return Err("plane shorter than n".into());
    }
    Ok(&p[..])
}

/// vector region of the plane-arm kernels (K users): n >= 16, phase from the destination, `[peel, vend)` in blocks of 16
fn region(n: usize, phase: usize, m: Option<&str>) -> Option<(usize, usize)> {
    if n < 16 {
        return None;
    }
    let ph = phase & 0x1f;
    let peel = if m == Some("peel_off") || ph == 0 {
        0
    } else if ph & 3 != 0 {
        return None;
    } else {
        (32 - ph) >> 2
    };
    if n < peel + 16 {
        return None;
    }
    let vend = if m == Some("vend_n") { n } else { n - ((n - peel) & 15) };
    Some((peel, vend))
}

pub fn render(kind: Kind, p: &Params, ops: &[Operand], out: &mut [Vec<u32>]) -> Result<(), String> {
    render_mut(kind, p, ops, out, None)
}

/// `render` with an optional named mutant (T2 mutation testing; a mutant must be killed by some case)
pub fn render_mut(kind: Kind, p: &Params, ops: &[Operand], out: &mut [Vec<u32>], m: Option<&str>) -> Result<(), String> {
    let has = |s: &str| m == Some(s);
    let (ia, ib, ix, iy) = match kind {
        Kind::Plus => {
            if ops.len() != 3 {
                return Err("if+ takes 3 operands".into());
            }
            (0usize, None, 1usize, 2usize)
        }
        Kind::Gt => {
            if ops.len() != 4 {
                return Err("if> takes 4 operands".into());
            }
            (0usize, Some(1usize), 2usize, 3usize)
        }
    };
    if p.nout <= 0 {
        return Ok(());
    }
    if p.nout > 4 {
        return Err("CONTRACT: nout > 4".into());
    }
    if out.len() < p.nout as usize {
        return Err("too few destination planes".into());
    }
    let nout = if has("nout_cap3") { p.nout.min(3) } else { p.nout };
    let sel_swap = has("swap_arms");
    for ch in 0..nout as usize {
        let chi = ch as i32;
        let cidx = if has("const_idx0") { 0 } else { ch };
        let mut xconst = ops[ix].is_const;
        let mut yconst = ops[iy].is_const;
        let mut xc = ops[ix].consts[cidx];
        let mut yc = ops[iy].consts[cidx];
        if ch == 3 && !has("ch3_plane") {
            if (ops[ix].count - 1) & 1 == 0 {
                xc = p.one;
                xconst = true;
            }
            if (ops[iy].count - 1) & 1 == 0 {
                yc = p.one;
                yconst = true;
            }
        }
        // two-channel arms map channel c to max(c-2, 0) when nout > 2 (g,g,g,a); other counts clamp to count-1
        let two_x = p.nout > 2 && ops[ix].count == 2 && !has("no_two_ch");
        let two_y = p.nout > 2 && ops[iy].count == 2 && !has("no_two_ch");
        let pidx = |o: &Operand, two: bool| -> i32 {
            if two {
                (chi - 2).max(0)
            } else if has("clamp_off") {
                chi.min(o.count)
            } else {
                chi.min(o.count - 1)
            }
        };
        let aidx = |o: &Operand| -> i32 { chi.min(o.count - 1) };
        let n = p.n;
        let cond_const = match ib {
            None => ops[ia].is_const,
            Some(b) => ops[ia].is_const && ops[b].is_const,
        };
        let thr = if has("plus_thresh_one") { p.one } else { 0 };
        let dst = &mut out[ch];
        if cond_const {
            let av = ops[ia].consts[cidx];
            let mut cv = match ib {
                None => gt(av, thr, m),
                Some(b) => {
                    if has("swap_ab") {
                        gt(ops[b].consts[cidx], av, m)
                    } else {
                        gt(av, ops[b].consts[cidx], m)
                    }
                }
            };
            if sel_swap || has("const_cond_inv") {
                cv = !cv;
            }
            let (aconst, aval, ao, two) = if cv { (xconst, xc, ix, two_x) } else { (yconst, yc, iy, two_y) };
            if aconst {
                for i in 0..n.max(0) as usize {
                    dst[i] = aval;
                }
            } else {
                if n < 0 {
                    return Err("CONTRACT: n < 0 on a whole-plane copy".into());
                }
                let nn = n as usize;
                let pl = plane(&ops[ao], pidx(&ops[ao], two), nn)?;
                let cnt = if has("memcpy_short") && nn > 0 { nn - 1 } else { nn };
                dst[..cnt].copy_from_slice(&pl[..cnt]);
            }
            continue;
        }
        if n <= 0 {
            continue;
        }
        let nn = n as usize;
        // operand sources
        let a_pl = if ops[ia].is_const { None } else { Some(plane(&ops[ia], aidx(&ops[ia]), nn)?) };
        let b_pl = match ib {
            Some(b) if !ops[b].is_const => Some(plane(&ops[b], aidx(&ops[b]), nn)?),
            _ => None,
        };
        let x_pl = if xconst { None } else { Some(plane(&ops[ix], pidx(&ops[ix], two_x), nn)?) };
        let y_pl = if yconst { None } else { Some(plane(&ops[iy], pidx(&ops[iy], two_y), nn)?) };
        let kuser = !xconst || !yconst;
        let reg = if kuser && !has("no_vec") { region(nn, p.dst_phase, m) } else { None };
        let ksign = if has("k_zero") { false } else { (p.k_mask >> 31) & 1 == 1 };
        let tail_n = if has("tail_drop") { nn - 1 } else { nn };
        for i in 0..tail_n {
            let av = match a_pl {
                Some(pl) => pl[i],
                None => ops[ia].consts[cidx],
            };
            let mut c = match ib {
                None => gt(av, thr, m),
                Some(b) => {
                    let bv = match b_pl {
                        Some(pl) => pl[i],
                        None => ops[b].consts[cidx],
                    };
                    if has("swap_ab") {
                        gt(bv, av, m)
                    } else {
                        gt(av, bv, m)
                    }
                }
            };
            if sel_swap {
                c = !c;
            }
            let xv = match x_pl {
                Some(pl) => pl[i],
                None => xc,
            };
            let yv = match y_pl {
                Some(pl) => pl[i],
                None => yc,
            };
            let in_vec = match reg {
                Some((pe, ve)) => i >= pe && i < ve,
                None => false,
            };
            dst[i] = if in_vec {
                // vector lanes: maskmov/blend x under the mask, then y under (K ^ mask)
                let mut v = dst[i];
                if c {
                    v = xv;
                }
                if ksign != c {
                    v = yv;
                }
                v
            } else if c {
                xv
            } else {
                yv
            };
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: u32 = 0x3f800000;
    fn f(x: f32) -> u32 {
        x.to_bits()
    }
    fn par(nout: i32, n: i32) -> Params {
        Params { nout, n, one: ONE, k_mask: 0xffffffff, dst_phase: 0, dst_init: SENT }
    }
    fn img(count: i32, vals: &[&[f32]]) -> Operand {
        Operand { count, is_const: false, consts: [0; 4], planes: Some(vals.iter().map(|v| v.iter().map(|x| f(*x)).collect()).collect()) }
    }
    fn cst(count: i32, c: [f32; 4]) -> Operand {
        Operand { count, is_const: true, consts: [f(c[0]), f(c[1]), f(c[2]), f(c[3])], planes: None }
    }
    fn outs() -> Vec<Vec<u32>> {
        vec![vec![SENT; 32]; 6]
    }

    #[test]
    fn plus_hand_example() {
        // c = [1, 0, -1, NaN]: x where c > 0, else y (NaN and 0 and negatives take y)
        let c = img(1, &[&[1.0, 0.0, -1.0, f32::NAN]]);
        let x = cst(1, [10.0, 0.0, 0.0, 0.0]);
        let y = cst(1, [20.0, 0.0, 0.0, 0.0]);
        let mut o = outs();
        render(Kind::Plus, &par(1, 4), &[c, x, y], &mut o).unwrap();
        assert_eq!(&o[0][..4], &[f(10.0), f(20.0), f(20.0), f(20.0)]);
        assert_eq!(o[0][4], SENT);
        assert_eq!(o[1][0], SENT);
    }

    #[test]
    fn gt_hand_example_and_nan_class() {
        let a = img(1, &[&[3.0, 1.0, 2.0, f32::NAN]]);
        let b = cst(1, [2.0, 0.0, 0.0, 0.0]);
        let x = img(1, &[&[1.0, 2.0, 3.0, 4.0]]);
        let y = img(1, &[&[5.0, 6.0, 7.0, 8.0]]);
        let mut o = outs();
        render(Kind::Gt, &par(1, 4), &[a, b, x, y], &mut o).unwrap();
        assert_eq!(&o[0][..4], &[f(1.0), f(6.0), f(7.0), f(8.0)]);
    }

    #[test]
    fn const_condition_copies_a_whole_arm() {
        let c = cst(1, [1.0, 1.0, 1.0, 1.0]);
        let x = img(1, &[&[1.0, 2.0, 3.0]]);
        let y = cst(1, [9.0; 4]);
        let mut o = outs();
        render(Kind::Plus, &par(1, 3), &[c, x, y], &mut o).unwrap();
        assert_eq!(&o[0][..3], &[f(1.0), f(2.0), f(3.0)]);
        assert_eq!(o[0][3], SENT);
    }

    #[test]
    fn channel_three_of_a_three_channel_arm_is_one() {
        let c = cst(1, [1.0; 4]);
        let x = img(3, &[&[1.0, 1.0], &[2.0, 2.0], &[3.0, 3.0]]);
        let y = cst(1, [0.0; 4]);
        let mut o = outs();
        render(Kind::Plus, &par(4, 2), &[c, x, y], &mut o).unwrap();
        assert_eq!(&o[0][..2], &[f(1.0), f(1.0)]);
        assert_eq!(&o[2][..2], &[f(3.0), f(3.0)]);
        assert_eq!(&o[3][..2], &[ONE, ONE]);
    }

    #[test]
    fn two_channel_arm_maps_g_g_g_a() {
        let c = cst(1, [1.0; 4]);
        let x = img(2, &[&[5.0], &[7.0]]);
        let y = cst(1, [0.0; 4]);
        let mut o = outs();
        render(Kind::Plus, &par(4, 1), &[c, x, y], &mut o).unwrap();
        assert_eq!([o[0][0], o[1][0], o[2][0], o[3][0]], [f(5.0), f(5.0), f(5.0), f(7.0)]);
        // with nout 2 the plain clamp applies
        let c = cst(1, [1.0; 4]);
        let x = img(2, &[&[5.0], &[7.0]]);
        let y = cst(1, [0.0; 4]);
        let mut o = outs();
        render(Kind::Plus, &par(2, 1), &[c, x, y], &mut o).unwrap();
        assert_eq!([o[0][0], o[1][0]], [f(5.0), f(7.0)]);
    }

    #[test]
    fn k_mask_changes_only_the_vector_region() {
        let n = 40usize;
        let cv: Vec<f32> = (0..n).map(|i| if i % 3 == 0 { 1.0 } else { -1.0 }).collect();
        let xv: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let yv: Vec<f32> = (0..n).map(|i| 100.0 + i as f32).collect();
        let mk = |k: u32| {
            let mut o = outs();
            o.iter_mut().for_each(|v| v.resize(64, SENT));
            let p = Params { nout: 1, n: n as i32, one: ONE, k_mask: k, dst_phase: 0, dst_init: SENT };
            render(Kind::Plus, &p, &[img(1, &[&cv]), img(1, &[&xv]), img(1, &[&yv])], &mut o).unwrap();
            o[0].clone()
        };
        let ones = mk(0xffffffff);
        let zero = mk(0);
        assert_ne!(ones, zero);
        // region [0, 32): lanes with c > 0 take y (K = 0: mask' = mask) ; tail lanes (>= 32) are identical
        assert_eq!(&ones[32..40], &zero[32..40]);
        assert_eq!(ones[0], f(0.0));
        assert_eq!(zero[0], f(100.0));
    }

    #[test]
    fn degenerate_sizes_and_determinism() {
        let c = img(1, &[&[1.0, 2.0]]);
        let x = cst(1, [1.0; 4]);
        let y = cst(1, [2.0; 4]);
        let mut o1 = outs();
        render(Kind::Plus, &par(1, 0), &[c.clone(), x.clone(), y.clone()], &mut o1).unwrap();
        assert!(o1.iter().all(|v| v.iter().all(|b| *b == SENT)));
        render(Kind::Plus, &par(1, -5), &[c.clone(), x.clone(), y.clone()], &mut o1).unwrap();
        assert!(o1.iter().all(|v| v.iter().all(|b| *b == SENT)));
        let mut o2 = outs();
        let mut o3 = outs();
        render(Kind::Plus, &par(1, 2), &[c.clone(), x.clone(), y.clone()], &mut o2).unwrap();
        render(Kind::Plus, &par(1, 2), &[c, x, y], &mut o3).unwrap();
        assert_eq!(o2, o3);
        assert!(render(Kind::Plus, &par(5, 2), &[img(1, &[&[1.0]]), cst(1, [0.0; 4]), cst(1, [0.0; 4])], &mut outs()).is_err());
    }

    #[test]
    fn mutants_change_something() {
        let c = img(1, &[&[0.0, 1.0, f32::NAN, -0.0]]);
        let x = cst(1, [1.0; 4]);
        let y = cst(1, [2.0; 4]);
        let mut base = outs();
        render(Kind::Plus, &par(1, 4), &[c.clone(), x.clone(), y.clone()], &mut base).unwrap();
        for m in ["cmp_ge", "nan_true", "swap_arms"] {
            let mut o = outs();
            render_mut(Kind::Plus, &par(1, 4), &[c.clone(), x.clone(), y.clone()], &mut o, Some(m)).unwrap();
            assert_ne!(o, base, "{}", m);
        }
    }

    #[test]
    fn assumptions_listed() {
        let a = assumptions();
        assert!(a.iter().all(|x| x.id.starts_with(PREFIX)));
        assert_eq!(a.len(), 9);
    }
}
