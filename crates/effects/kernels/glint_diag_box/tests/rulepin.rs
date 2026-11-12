// RULE-PIN and reference model tests. Independent first-operand-rule reference for every window sum, box pass and shift blend site:
// a two-NaN event keeps the first operand (quieted). These tests pin that rule directly.
use glint_diag_box::*;

fn q(a: f32) -> f32 { f32::from_bits(a.to_bits() | (1u32 << 22)) }
fn pick(a: f32, b: f32) -> f32 { if a.is_nan() { q(a) } else if b.is_nan() { q(b) } else { f32::from_bits(0xffc0_0000) } }
fn add(a: f32, b: f32) -> f32 { let r = a + b; if r.is_nan() { pick(a, b) } else { r } }
fn sub(a: f32, b: f32) -> f32 { let r = a - b; if r.is_nan() { pick(a, b) } else { r } }
fn mul(a: f32, b: f32) -> f32 { let r = a * b; if r.is_nan() { pick(a, b) } else { r } }

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); self.0 >> 33 }
}
fn pool(g: &mut Lcg) -> f32 {
    const P: [u32; 14] = [0x0000_0000, 0x8000_0000, 0x3f80_0000, 0xc020_0000, 0x7fc0_0011, 0x7fc0_0022, 0xffc0_0033, 0x7f80_0000, 0xff80_0000, 0x4e6e_6b28, 0xce6e_6b28, 0x3dcc_cccd, 0x0000_0001, 0x7fa0_0044];
    f32::from_bits(P[(g.next() % 14) as usize])
}
fn fill(g: &mut Lcg, n: usize, nanrate: u64) -> Vec<f32> {
    (0..n).map(|_| if g.next() % 100 < nanrate { f32::from_bits([0x7fc0_0011u32, 0x7fc0_0022, 0xffc0_0033, 0x7fa0_0044][(g.next() % 4) as usize]) } else { pool(g) }).collect()
}
fn same(a: f32, b: f32) -> bool { a.to_bits() == b.to_bits() }

fn ref_scalar(x: &[f32], n: usize) -> f32 {
    let mut t = 0.0f32;
    for v in &x[..n] { t = add(t, *v); }
    add(0.0, t)
}

fn ref_vec(x: &[f32], n: usize, phase: u32) -> f32 {
    let ph = phase & 31;
    let p = if ph == 0 { 0 } else { ((32 - ph) >> 2) as usize };
    let mut t = 0.0f32;
    for i in 0..p { t = add(t, x[i]); }
    let trips = (n - p) / 64;
    let mut acc = vec![0.0f32; 64];
    for tr in 0..trips { for i in 0..64 { acc[i] = add(acc[i], x[p + 64 * tr + i]); } }
    let mut lanes = [0.0f32; 8];
    for l in 0..8 {
        let s = |k: usize| acc[8 * k + l];
        lanes[l] = add(add(add(s(0), s(1)), add(s(2), s(3))), add(add(s(4), s(5)), add(s(6), s(7))));
    }
    let end = p + trips * 64;
    let rem = n - end;
    let mut done = 0;
    if rem >= 8 {
        while done + 8 <= rem { for l in 0..8 { lanes[l] = add(lanes[l], x[end + done + l]); } done += 8; }
    }
    for j in done..rem { t = add(t, x[end + j]); }
    let h: Vec<f32> = (0..4).map(|l| add(lanes[l], lanes[l + 4])).collect();
    let group_a = add(h[0], h[2]);
    let group_b = add(h[1], h[3]);
    add(add(group_a, group_b), t)
}

fn ref_box(x: &[f32], l: usize, r: usize, mut s: f32) -> Vec<f32> {
    let mut out = vec![0.0f32; l];
    for i in 0..r { out[i] = s; s = add(s, x[r + 1 + i]); }
    let m = l as i64 - 2 * r as i64 - 1;
    if m > 0 {
        let m = m as usize;
        let paired = m - m % 2;
        for k in 0..m {
            out[r + k] = s;
            let (xi, xo) = (x[2 * r + 1 + k], x[k]);
            s = if k % 2 == 0 && k < paired { add(sub(xi, xo), s) } else { sub(add(s, xi), xo) };
        }
    }
    let mid_pos = l - 2 * r - 1;
    for j in 0..=r { out[l - r - 1 + j] = s; if j < r { s = sub(s, x[mid_pos + j]); } }
    out
}

fn ref_shift(x: &[f32], l: usize, dd: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; l];
    if l <= dd { return out; }
    for i in 0..l {
        let hi = if i + dd < l { Some(x[i + dd]) } else { None };
        let lo = if i >= dd { Some(x[i - dd]) } else { None };
        out[i] = match (hi, lo) {
            (Some(a), Some(b)) => mul(0.5, add(a, b)),
            (Some(a), None) => mul(0.5, a),
            (None, Some(b)) => mul(0.5, b),
            _ => 0.0,
        };
    }
    out
}

#[test]
fn rulepin_scalar_window_sum() {
    let mut g = Lcg(11);
    for n in [1usize, 2, 3, 5, 8, 17, 40] {
        for _ in 0..400 {
            let x = fill(&mut g, n + 4, 30);
            assert!(same(window_sum_scalar(&x, n), ref_scalar(&x, n)), "n {n}");
        }
    }
    // hand case: the first NaN in index order wins
    let a = f32::from_bits(0x7fc0_0011);
    let b = f32::from_bits(0x7fc0_0022);
    assert!(same(window_sum_scalar(&[1.0, a, 2.0, b], 4), a));
}

#[test]
fn rulepin_vector_window_sum() {
    let mut g = Lcg(23);
    for phase in [0u32, 4, 8, 16, 28] {
        let p = if phase == 0 { 0 } else { ((32 - phase) >> 2) as usize };
        for n in [64 + p, 65 + p, 71 + p, 72 + p, 79 + p, 127 + p, 128 + p, 130 + p, 136 + p, 191 + p, 200 + p] {
            for rate in [3u64, 15, 60] {
                for _ in 0..60 {
                    let x = fill(&mut g, n + 8, rate);
                    assert!(same(window_sum_vector(&x, n, phase), ref_vec(&x, n, phase)), "phase {phase} n {n}");
                }
            }
        }
    }
    // hand case: A is the first operand at every merge site, so A wins
    let a = f32::from_bits(0x7fc0_0011);
    let mut x = vec![0.0f32; 140];
    x[0] = a;
    for (i, pay) in [(8usize, 22u32), (64, 33), (4, 44), (2, 55), (1, 66), (128, 77)] { x[i] = f32::from_bits(0x7fc0_0000 | pay); }
    assert!(same(window_sum_vector(&x, 130, 0), a));
}

#[test]
fn rulepin_box_pass() {
    let mut g = Lcg(37);
    for l in [3usize, 4, 5, 8, 9, 10, 15, 16, 31, 32, 40] {
        for r in 0..=((l - 1) / 2) {
            for rate in [5u64, 30] {
                for _ in 0..25 {
                    let x = fill(&mut g, l + 4, rate);
                    let s = pool(&mut g);
                    let want = ref_box(&x, l, r, s);
                    let mut got = vec![9.0f32; l + 4];
                    box_pass(&x, &mut got, l, r, s);
                    for i in 0..l { assert!(same(got[i], want[i]), "l {l} r {r} i {i}"); }
                }
            }
        }
    }
}

#[test]
fn rulepin_shift_blend() {
    let mut g = Lcg(41);
    for l in [1usize, 2, 3, 6, 7, 16, 33] {
        for dd in [1usize, 2, 3, 5, 7, 8, 16, 17, 33, 40] {
            for rate in [5u64, 40] {
                for _ in 0..20 {
                    let x = fill(&mut g, l + 2, rate);
                    let want = ref_shift(&x, l, dd);
                    let mut got = vec![9.0f32; l + 2];
                    shift_blend(&x, &mut got, l, dd);
                    for i in 0..l { assert!(same(got[i], want[i]), "l {l} dd {dd} i {i}"); }
                }
            }
        }
    }
}

#[test]
fn body_b_keeps_unwritten_destination_words() {
    for body in [false, true] {
        let (w, h) = (5usize, 4usize);
        let src: Vec<f32> = (0..w * h).map(|i| i as f32).collect();
        let mut dst: Vec<f32> = (0..w * h + 3).map(|i| f32::from_bits(0x3f00_0000 + i as u32)).collect();
        let before = dst.clone();
        let p = Params { radii: [1, 0, 0], d: 0 };
        run_body(body, &p, &RowPhases([0, 0, 0]), Band { start: 1, count: 0 }, &src, &mut dst, w, h).unwrap();
        assert_eq!(dst.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), before.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    }
}

fn nanp(g: &mut Lcg) -> f32 { f32::from_bits([0x7fc0_0011u32, 0x7fc0_0022, 0xffc0_0033, 0x7fa0_0044, 0xffa0_0055][(g.next() % 5) as usize]) }
fn fin(g: &mut Lcg) -> f32 { loop { let v = pool(g); if !v.is_nan() { return v; } } }

#[test]
fn rulepin_vector_peel_and_tail_nans() {
    let mut g = Lcg(77);
    let mut checked = 0;
    for phase in [4u32, 8, 12, 16, 20, 24, 28] {
        let p = ((32 - phase) >> 2) as usize;
        for n in [p + 64, p + 67, p + 73, p + 130, p + 137, p + 127] {
            for _ in 0..40 {
                let mut x: Vec<f32> = (0..n + 8).map(|_| fin(&mut g)).collect();
                let end = p + ((n - p) / 64) * 64;
                for i in 0..p { if g.next() % 2 == 0 { x[i] = nanp(&mut g); } }
                for i in end..n { if g.next() % 2 == 0 { x[i] = nanp(&mut g); } }
                assert!(same(window_sum_vector(&x, n, phase), ref_vec(&x, n, phase)), "phase {phase} n {n}");
                checked += 1;
            }
        }
    }
    assert!(checked > 1000);
}

#[test]
fn normaliser_double_rounding_values() {
    // 1/(2r+1)^3 rounded once from double: the f32 reciprocal of the f32-rounded product differs in the last bit for these radii
    assert_eq!(normaliser([174, 174, 174]).to_bits(), 0x32ca1351);
    assert_eq!(normaliser([359, 359, 359]).to_bits(), 0x3138e1a2);
    assert_eq!(normaliser([433, 433, 433]).to_bits(), 0x30d2e36a);
}

fn ref_run(radii: [i32; 3], d: i32, start: i32, count: i32, src: &[f32], w: usize, h: usize, dst: &mut [f32]) {
    let (wi, hi) = (w as i64, h as i64);
    let mut s = src.to_vec();
    s.extend(std::iter::repeat(0.0).take(4 * w + 16));
    let prod = ((2 * radii[0] + 1) as f64) * ((2 * radii[1] + 1) as f64) * ((2 * radii[2] + 1) as f64);
    let f = (1.0f64 / prod) as f32;
    let dd = d.unsigned_abs() as usize;
    for counter in 0..count as i64 {
        let t = counter + 1 + start as i64 - hi + 1;
        let a = t.max(0);
        let b = (-t).max(0);
        let l = (wi - 2 * a).min(2 * hi - 2 * b);
        if l <= 0 { continue; }
        let off = (wi * b + 2 * a) as usize;
        let l = l as usize;
        let mut av = vec![0.0f32; l];
        let mut bv = vec![0.0f32; l];
        av[0] = s[off];
        bv[0] = if b == 0 { 0.0 } else { mul(0.5, add(s[off], s[off - w])) };
        let (phases, mut last_zeroed) = (b == 0, false);
        for j in 1..l {
            let pos = off + j + w * (j / 2);
            let x = s[pos];
            if j % 2 == 1 {
                if j == l - 1 && !(b + (l as i64 >> 1) < hi - 1) { av[j] = 0.0; last_zeroed = true; } else { av[j] = mul(0.5, add(x, s[pos + w])); }
                bv[j] = x;
            } else {
                av[j] = x;
                bv[j] = mul(0.5, add(x, s[pos - w]));
            }
        }
        let clamp = ((l as i64 - 1) >> 1) as i32;
        for (v, z) in [(&mut av, 0usize), (&mut bv, 1usize)] {
            let zero = |v: &mut Vec<f32>| { if z == 0 && last_zeroed { v[l - 1] = 0.0; } if z == 1 && phases { v[0] = 0.0; } };
            for k in 0..3 {
                if radii[k] <= 0 { continue; }
                let r = radii[k].min(clamp) as usize;
                let sum = ref_scalar(v, r + 1);
                *v = ref_box(v, l, r, sum);
                zero(v);
            }
            if dd != 0 { *v = ref_shift(v, l, dd); zero(v); }
        }
        let mut pos = off;
        for q in 0..l / 2 {
            dst[pos] = mul(f, av[2 * q]);
            dst[pos + 1] = mul(f, bv[2 * q + 1]);
            pos += w + 2;
        }
        if l % 2 == 1 { dst[pos] = mul(f, av[l - 1]); }
    }
    if start == 0 {
        dst[(h - 1) * w] = 0.0;
        dst[(h - 1) * w + 1] = 0.0;
        dst[w - 1] = 0.0;
        if w % 2 == 0 { dst[w - 2] = 0.0; }
    }
}

#[test]
fn rulepin_whole_call_model() {
    let mut g = Lcg(91);
    let mut compared = 0;
    for (w, h) in [(5usize, 3usize), (6, 4), (7, 4), (8, 5), (9, 6), (12, 3), (3, 7), (10, 10), (13, 5), (4, 2), (2, 5), (11, 1)] {
        let full = ((w as i32 - 1) / 2) + h as i32 - 1;
        for radii in [[1, 0, 0], [2, 1, 1], [3, 0, 2], [5, 5, 5], [0, 0, 0], [1, 2, 0], [4, 0, 0]] {
            for d in [0, 1, -2, 3, 7] {
                for (st, cnt) in [(0, full), (1, full - 1), (2, 3)] {
                    for rate in [8u64, 35] {
                        let src = fill(&mut g, w * h, rate);
                        let mut got = vec![-1.0f32; w * h + 8];
                        let p = Params { radii, d };
                        if run(&p, &RowPhases([0, 0, 0]), Band { start: st, count: cnt }, &src, &mut got, w, h).is_err() { continue; }
                        let mut want = vec![-1.0f32; w * h + 8];
                        ref_run(radii, d, st, cnt, &src, w, h, &mut want);
                        for i in 0..w * h { assert!(same(got[i], want[i]), "w {w} h {h} radii {radii:?} d {d} band {st},{cnt} word {i}: {:08x} vs {:08x}", got[i].to_bits(), want[i].to_bits()); }
                        compared += 1;
                    }
                }
            }
        }
    }
    assert!(compared > 800, "compared {compared}");
}

#[test]
fn rulepin_vector_peel_and_tail_without_infinities() {
    // finite values only apart from the planted NaNs, so no fresh NaN from inf - inf hides the peel and tail accumulator in the final add
    let mut g = Lcg(79);
    let fin2 = |g: &mut Lcg| -> f32 { loop { let v = pool(g); if v.is_finite() { return v; } } };
    let mut checked = 0;
    for phase in [4u32, 8, 12, 16, 20, 24, 28] {
        let p = ((32 - phase) >> 2) as usize;
        for n in [p + 64, p + 67, p + 73, p + 130, p + 137, p + 127] {
            for _ in 0..40 {
                let mut x: Vec<f32> = (0..n + 8).map(|_| fin2(&mut g)).collect();
                let end = p + ((n - p) / 64) * 64;
                let rem = n - end;
                let tail_from = if rem >= 8 { end + (rem & !7) } else { end };
                for i in 0..p { if g.next() % 2 == 0 { x[i] = nanp(&mut g); } }
                for i in tail_from..n { if g.next() % 2 == 0 { x[i] = nanp(&mut g); } }
                assert!(same(window_sum_vector(&x, n, phase), ref_vec(&x, n, phase)), "phase {phase} n {n}");
                checked += 1;
            }
        }
    }
    assert!(checked > 1000);
}
