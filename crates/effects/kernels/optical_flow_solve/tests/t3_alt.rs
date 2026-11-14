//! The alternate-launch block of the level loop: branch both ways, plane selectors, launch order.
use optical_flow_solve::body::{Callback, Level, Params};
use optical_flow_solve::fetch::{deriv_x, deriv_y, polar, StencilConsts};
use optical_flow_solve::relax::{relax_callback, Ctx, NestIn};
use optical_flow_solve::smooth::{smooth_normalise, Geo, Pass};
use optical_flow_solve::solve::{run_body, PlaneStore, KS_BITS, POLAR_CONSTS, SMOOTH_WEIGHT_BITS};
use std::collections::HashMap;

const N: usize = 1024;

#[derive(Clone)]
struct Store {
    planes: HashMap<u64, Vec<f32>>,
    launches: Vec<Callback>,
    /// copy of every plane at the moment a relax or smooth launch is announced
    snaps: Vec<(Callback, HashMap<u64, Vec<f32>>)>,
}

impl PlaneStore for Store {
    fn read(&self, id: u64) -> Vec<f32> {
        self.planes[&id].clone()
    }
    fn write(&mut self, id: u64, v: &[f32]) {
        self.planes.get_mut(&id).unwrap()[..v.len()].copy_from_slice(v);
    }
    fn align_bytes(&self, id: u64) -> u64 {
        id
    }
    fn launched(&mut self, c: Callback) {
        self.launches.push(c);
        if c == Callback::Relax || c == Callback::Smooth {
            self.snaps.push((c, self.planes.clone()));
        }
    }
}

fn levels(count: usize) -> Vec<Level> {
    (0..count)
        .map(|k| {
            let side = 12 - 3 * k as i32;
            let base = 0x1000 * (k as u64 + 1) * 32;
            let p = |i: u64| base + i * 0x1000;
            Level {
                width: side,
                height: side,
                stride: side,
                rect_word3: 0,
                elem_bytes: 4,
                index: k as u32,
                image_a: p(0), image_b: p(1), grad_x: p(2), grad_y: p(3),
                warped_image: p(4), warped_grad_x: p(5), warped_grad_y: p(6), coeff: p(7),
                flow_u: p(8), flow_v: p(9), flow_u_next: p(10), flow_v_next: p(11),
                nest1_t: p(12), nest1_u: p(13), nest2_t: p(14), nest2_u: p(15),
                relax_first: p(16), relax_second: p(17), median_out: p(18),
                copied_plane: p(19),
            }
        })
        .collect()
}

fn store(ls: &[Level]) -> Store {
    let mut planes = HashMap::new();
    for (k, l) in ls.iter().enumerate() {
        let ids = [l.image_a, l.image_b, l.grad_x, l.grad_y, l.warped_image, l.warped_grad_x, l.warped_grad_y, l.coeff, l.flow_u, l.flow_v, l.flow_u_next, l.flow_v_next, l.nest1_t, l.nest1_u, l.nest2_t, l.nest2_u, l.relax_first, l.relax_second, l.median_out, l.copied_plane];
        for (j, id) in ids.iter().enumerate() {
            let v: Vec<f32> = (0..N).map(|i| ((i * 7 + j * 13 + k * 5) % 29) as f32 * 0.37 - 3.1 + (j as f32) * 0.01).collect();
            planes.insert(*id, v);
        }
        planes.insert(l.copied_plane, vec![0.0; N]);
    }
    Store { planes, launches: vec![], snaps: vec![] }
}

fn params(alt: i32, outer: i32) -> Params {
    Params { outer_iters: outer, mid_iters: 1, inner_iters: 1, counted_route: 0, flow_gain: 0x3f00_0000, relax_scale: 0x3f80_0000, tol: 0.0, smooth_bias: 0x3ca3_d70a, alt_launches: alt }
}

const PREFIX: [Callback; 13] = [
    Callback::DerivX, Callback::DerivY, Callback::Polar, Callback::DerivX, Callback::DerivY,
    Callback::Warp, Callback::Warp, Callback::Warp, Callback::Mix, Callback::Flow, Callback::Relax, Callback::Smooth, Callback::Diff,
];

#[test]
fn branch_off_has_no_alt_launch_and_equals_the_branch_on_run_without_the_block() {
    let ls = levels(3);
    let (mut off, mut on) = (store(&ls), store(&ls));
    run_body(&ls, 2, &params(0, 1), &mut off, &mut || false).unwrap();
    run_body(&ls, 2, &params(1, 1), &mut on, &mut || false).unwrap();
    assert!(!off.launches.contains(&Callback::Polar));
    assert_eq!(on.launches.iter().filter(|c| **c == Callback::Polar).count(), 3);
    let stripped: Vec<Callback> = {
        let mut v = vec![];
        let mut it = on.launches.iter().peekable();
        while let Some(c) = it.next() {
            if *c == Callback::DerivX && it.clone().next() == Some(&Callback::DerivY) && it.clone().nth(1) == Some(&Callback::Polar) {
                it.next();
                it.next();
                continue;
            }
            v.push(*c);
        }
        v
    };
    assert_eq!(stripped, off.launches);
    // At a nonzero word the smooth launch reads the polar output plane as its scale plane, so the planes it writes differ from the run without the block;
    // the planes upstream of it are equal.
    let bits = |s: &Store, id: u64| s.planes[&id].iter().map(|f| f.to_bits()).collect::<Vec<_>>();
    for l in &ls {
        for id in [l.image_a, l.image_b, l.grad_x, l.grad_y] {
            assert_eq!(bits(&on, id), bits(&off, id), "plane {id:x}");
        }
    }
    assert!(ls.iter().any(|l| [l.nest1_t, l.nest1_u, l.nest2_t, l.nest2_u].iter().any(|id| bits(&on, *id) != bits(&off, *id))));
}

#[test]
fn every_level_starts_with_the_block_then_the_gather_and_the_loop_prefix() {
    let ls = levels(3);
    let mut s = store(&ls);
    run_body(&ls, 2, &params(1, 1), &mut s, &mut || false).unwrap();
    let mut starts = vec![];
    for (i, c) in s.launches.iter().enumerate() {
        if *c == Callback::Polar {
            starts.push(i - 2);
        }
    }
    assert_eq!(starts.len(), 3);
    for st in starts {
        assert_eq!(s.launches[st..st + 13], PREFIX);
    }
}

#[test]
fn block_planes_are_image_a_to_the_gradients_and_the_gradients_to_the_descriptor_plane() {
    let ls = levels(1);
    let l = ls[0];
    let mut s = store(&ls);
    // no loops: the block and the gather only; the descriptor plane keeps the block's output
    run_body(&ls, 0, &params(1, 0), &mut s, &mut || false).unwrap();
    let sc = StencilConsts { k_s: f32::from_bits(KS_BITS), k_v: f32::from_bits(KS_BITS), c_s: 8.0, c_v: 8.0 };
    let fresh = store(&ls);
    let (w, h, st) = (l.width, l.height, l.stride);
    let a = fresh.planes[&l.image_a].clone();
    let (mut gx, mut gy) = (fresh.planes[&l.grad_x].clone(), fresh.planes[&l.grad_y].clone());
    deriv_x(&a, &mut gx, w, st, 0, h, (l.grad_x & 31) as u32, &sc, None);
    deriv_y(&a, &mut gy, h, st, 0, w, &sc, None);
    let mut out = vec![0.0f32; N];
    polar(&gy, &gx, &mut out, w, st, 0, h, (l.grad_y & 31) as u32, &POLAR_CONSTS);
    let got: Vec<u32> = s.planes[&l.copied_plane].iter().map(|f| f.to_bits()).collect();
    let want: Vec<u32> = out.iter().map(|f| f.to_bits()).collect();
    assert_eq!(got, want);
    assert!(want.iter().any(|b| *b != 0));
    // nothing else of the descriptor group is touched by the block
    assert_eq!(s.launches, vec![Callback::DerivX, Callback::DerivY, Callback::Polar, Callback::DerivX, Callback::DerivY]);
    assert_eq!(s.planes[&l.median_out], fresh.planes[&l.median_out]);
}

fn snap(s: &Store, c: Callback) -> &HashMap<u64, Vec<f32>> {
    &s.snaps.iter().rev().find(|(k, _)| *k == c).unwrap().1
}

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|f| f.to_bits()).collect()
}

/// Reference for the smooth launch: its four written planes equal the op called directly on the snapshot taken when the launch was announced,
/// with the level's polar output plane as the scale plane at a nonzero word and no scale plane at word zero.
#[test]
fn smooth_launch_equals_the_op_on_its_snapshot_with_the_polar_output_as_scale_plane() {
    let mut results = vec![];
    for alt in [1, 0] {
        let ls = levels(1);
        let l = ls[0];
        let mut s = store(&ls);
        let mut p = params(alt, 1);
        // two inner iterations: the second relax launch reads the nonzero nests the first smooth launch wrote
        p.inner_iters = 2;
        run_body(&ls, 0, &p, &mut s, &mut || false).unwrap();
        let sn = snap(&s, Callback::Smooth);
        let (pa, pd) = (sn[&l.relax_first].clone(), sn[&l.relax_second].clone());
        let scale_plane = if alt != 0 { Some(sn[&l.copied_plane].clone()) } else { None };
        if alt != 0 {
            assert!(sn[&l.copied_plane].iter().any(|f| f.to_bits() != 0));
        }
        let (mut pb, mut pc, mut pe, mut pf) = (sn[&l.nest1_t].clone(), sn[&l.nest1_u].clone(), sn[&l.nest2_t].clone(), sn[&l.nest2_u].clone());
        let geo = Geo { width: l.width, height: l.height, stride: l.stride, relax_scale: f32::from_bits(p.relax_scale), smooth_weight: f32::from_bits(SMOOTH_WEIGHT_BITS), c: f32::from_bits(p.smooth_bias) };
        {
            let mut first = Pass { src: &pa, p: &mut pb, q: &mut pc };
            let mut second = Pass { src: &pd, p: &mut pe, q: &mut pf };
            smooth_normalise(&geo, scale_plane.as_deref(), &mut first, &mut second, 0, l.height as u32);
        }
        assert_eq!(bits(&s.planes[&l.nest1_t]), bits(&pb), "alt {alt}");
        assert_eq!(bits(&s.planes[&l.nest1_u]), bits(&pc), "alt {alt}");
        assert_eq!(bits(&s.planes[&l.nest2_t]), bits(&pe), "alt {alt}");
        assert_eq!(bits(&s.planes[&l.nest2_u]), bits(&pf), "alt {alt}");
        results.push(bits(&pb));
    }
    // the scale plane changes the result, so the reference separates the two bindings
    assert_ne!(results[0], results[1]);
}

/// Reference for the relax launch, the same form: the polar output plane is the scale plane at a nonzero word and absent at word zero.
#[test]
fn relax_launch_equals_the_op_on_its_snapshot_with_the_polar_output_as_scale_plane() {
    let mut results = vec![];
    for alt in [1, 0] {
        let ls = levels(1);
        let l = ls[0];
        let mut s = store(&ls);
        let mut p = params(alt, 1);
        // two inner iterations: the second relax launch reads the nonzero nests the first smooth launch wrote
        p.inner_iters = 2;
        run_body(&ls, 0, &p, &mut s, &mut || false).unwrap();
        let sn = snap(&s, Callback::Relax);
        let scale_plane = if alt != 0 { Some(sn[&l.copied_plane].clone()) } else { None };
        let ctx = Ctx {
            width: l.width,
            height: l.height,
            stride: l.stride,
            scale: f32::from_bits(p.relax_scale),
            v: scale_plane.as_deref(),
            nests: [NestIn { t: &sn[&l.nest1_t], u: &sn[&l.nest1_u], acc: &sn[&l.flow_u_next] }, NestIn { t: &sn[&l.nest2_t], u: &sn[&l.nest2_u], acc: &sn[&l.flow_v_next] }],
            phase_bytes: (l.nest1_t & 63) as u32,
        };
        let (mut relax_a, mut relax_b) = (sn[&l.relax_first].clone(), sn[&l.relax_second].clone());
        relax_callback(&ctx, [&mut relax_a, &mut relax_b], 0, l.height);
        assert_eq!(bits(&s.planes[&l.relax_first]), bits(&relax_a), "alt {alt}");
        assert_eq!(bits(&s.planes[&l.relax_second]), bits(&relax_b), "alt {alt}");
        results.push(bits(&relax_a));
    }
    assert_ne!(results[0], results[1]);
}

/// The stretch leaves the output of its latest horizontal pass in the finer level's median_out plane (compact rows of the finer width).
#[test]
fn stretch_leaves_its_horizontal_pass_in_median_out() {
    let ls = levels(2);
    let (fine, coarse) = (ls[0], ls[1]);
    let mut s = store(&ls);
    let before = s.planes[&fine.median_out].clone();
    run_body(&ls, 1, &params(1, 1), &mut s, &mut || false).unwrap();
    // the last pass of the stretch is the one of the nest2_u pair, run on the coarse plane after its border was zeroed (written back to the store)
    let (sw, sh, dw) = (coarse.width, coarse.height, fine.width);
    let src = s.planes[&coarse.nest2_u].clone();
    let mut want = vec![0f32; (dw * sh) as usize];
    optical_flow_resample::resample_h(&src[..(sw * sh) as usize], sw, sh, dw, &mut want, (coarse.nest2_u & 15) as usize, 0, sh);
    let n = want.len();
    assert_eq!(bits(&s.planes[&fine.median_out][..n]), bits(&want));
    assert_ne!(bits(&before[..n]), bits(&want));
    assert_eq!(bits(&s.planes[&fine.median_out][n..]), bits(&before[n..]));
    // at word zero the plane is left alone (the leave-behind is measured at the nonzero word only)
    let mut z = store(&ls);
    run_body(&ls, 1, &params(0, 1), &mut z, &mut || false).unwrap();
    assert_eq!(bits(&z.planes[&fine.median_out]), bits(&before));
}
