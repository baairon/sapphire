//! Chunk cover and partition proofs on the port side: the splitter model covers [0, total) with disjoint chunks for
//! thread counts 1 to 6, and each per-element callback gives the same words for every partition as for one whole-range pass.
use optical_flow::host::*;
use optical_flow::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn f(&mut self) -> f32 {
        const CLASSES: [u32; 12] = [0, 0x8000_0000, 0x3f80_0000, 0xbf80_0000, 0x7fc0_0000, 0xffc0_0000, 0x7f80_0000, 0xff80_0000, 1, 0x7f7f_ffff, 0x3e99_999a, 0x4049_0fdb];
        let k = self.next();
        if k % 5 == 0 {
            f32::from_bits(CLASSES[(k as usize / 5) % CLASSES.len()])
        } else {
            (self.next() as f32 / 1e9 - 1.0) * (1u32 << (self.next() % 12)) as f32
        }
    }
    fn plane(&mut self, n: usize) -> Vec<f32> {
        (0..n).map(|_| self.f()).collect()
    }
}
fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn chunks_cover_and_are_disjoint() {
    let mut cases = 0;
    for t in 1..=6 {
        for total in 0..200 {
            let mut next = 0;
            for tid in 0..t {
                let (s, c) = chunk(total, t, tid);
                if c > 0 {
                    assert_eq!(s, next, "T {} total {} tid {}", t, total, tid);
                    next = s + c;
                }
            }
            assert_eq!(next, total, "T {} total {}", t, total);
            cases += 1;
        }
    }
    println!("CHUNK-COVER cases {}", cases);
}

const WIDTHS: [i32; 10] = [1, 2, 7, 16, 17, 33, 64, 66, 100, 130];

#[test]
fn per_element_callbacks_are_partition_free() {
    let mut rng = Rng(77);
    let (mut cases, mut bad) = (0u32, 0u32);
    for &w in WIDTHS.iter() {
        for rows in [1, 2, 5, 13, 32] {
            for extra in [0, 1, 3] {
                for phase in (0..32).step_by(4) {
                    let stride = w + extra;
                    let n = (stride * rows) as usize;
                    let (a, b, c, d) = (rng.plane(n), rng.plane(n), rng.plane(n), rng.plane(n));
                    let (s, off, num, den) = (rng.f(), rng.f(), rng.f(), rng.f());
                    // whole range
                    let mut aff = a.clone();
                    affine_rows(&mut aff, w, stride, 0, rows, s, off, phase);
                    let mut sub = vec![0.0f32; n];
                    sub_scaled_rows(&a, &b, &mut sub, w, stride, 0, rows, s, phase);
                    let (mut fx, mut fy) = (b.clone(), c.clone());
                    normalize_flow_rows(&a, &mut fx, &mut fy, w, rows, stride, 0, rows, num, den);
                    let mut st = vec![0.0f32; n];
                    flow_stencil_rows(&mut st, &b, &c, &d, w, rows, stride, s, 0, rows, phase);
                    let mut ac = a.clone();
                    add_const(&mut ac, n as i64, s);
                    let mut bl = vec![0.0f32; n];
                    blend3(&a, &b, &c, &mut bl, n as i64, phase);
                    for t in 2..=6 {
                        let mut aff2 = a.clone();
                        let mut sub2 = vec![0.0f32; n];
                        let (mut fx2, mut fy2) = (b.clone(), c.clone());
                        let mut st2 = vec![0.0f32; n];
                        let mut ac2 = a.clone();
                        let mut bl2 = vec![0.0f32; n];
                        for tid in 0..t {
                            let (r0, k) = chunk(rows, t, tid);
                            affine_rows(&mut aff2, w, stride, r0, k, s, off, phase);
                            sub_scaled_rows(&a, &b, &mut sub2, w, stride, r0, k, s, phase);
                            normalize_flow_rows(&a, &mut fx2, &mut fy2, w, rows, stride, r0, k, num, den);
                            flow_stencil_rows(&mut st2, &b, &c, &d, w, rows, stride, s, r0, k, phase);
                            let (e0, ek) = chunk(n as i32, t, tid);
                            let ph = (phase + 4 * e0 as usize) & 31;
                            add_const(&mut ac2[e0 as usize..], ek as i64, s);
                            blend3(&a[e0 as usize..], &b[e0 as usize..], &c[e0 as usize..], &mut bl2[e0 as usize..], ek as i64, ph);
                        }
                        let ok = bits(&aff) == bits(&aff2) && bits(&sub) == bits(&sub2) && bits(&fx) == bits(&fx2) && bits(&fy) == bits(&fy2)
                            && bits(&st) == bits(&st2) && bits(&ac) == bits(&ac2) && bits(&bl) == bits(&bl2);
                        cases += 1;
                        if !ok {
                            bad += 1;
                            println!("DIFF w {} rows {} extra {} phase {} T {}: aff {} sub {} fx {} fy {} st {} ac {} bl {}", w, rows, extra, phase, t,
                                bits(&aff) == bits(&aff2), bits(&sub) == bits(&sub2), bits(&fx) == bits(&fx2), bits(&fy) == bits(&fy2),
                                bits(&st) == bits(&st2), bits(&ac) == bits(&ac2), bits(&bl) == bits(&bl2));
                        }
                    }
                }
            }
        }
    }
    println!("PARTITION-FREE cases {} differing {}", cases, bad);
    assert_eq!(bad, 0);
}

/// sum_rows is ONE-CHUNK: the same word at every host thread count.
#[test]
fn sum_rows_is_one_chunk() {
    let mut rng = Rng(5);
    let (mut cases, mut differ) = (0u32, 0u32);
    for &w in WIDTHS.iter() {
        for rows in [8, 13, 32, 50] {
            let n = (w * rows) as usize;
            let mut plane = rng.plane(n);
            for v in plane.iter_mut() {
                if !v.is_finite() {
                    *v = 1.0;
                }
            }
            let mut h1 = ComposedHost::new(1);
            h1.add_plane(1, plane.clone(), 0);
            let base = slot_reduce(&h1.sum_slots(1, w, rows, w).unwrap());
            for t in 2..=6 {
                let mut h = ComposedHost::new(t);
                h.add_plane(1, plane.clone(), 0);
                let got = slot_reduce(&h.sum_slots(1, w, rows, w).unwrap());
                cases += 1;
                if got.to_bits() != base.to_bits() {
                    differ += 1;
                }
            }
        }
    }
    println!("SUM-ROWS cases {} differing from T=1 {}", cases, differ);
    assert_eq!(differ, 0);
}
