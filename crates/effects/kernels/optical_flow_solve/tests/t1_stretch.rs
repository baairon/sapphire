// Unit tests of the stretch glue in its shipped form (Direct host) and the flag, degenerate-side and tile paths.
use optical_flow_solve::stretch::*;
#[path = "support/small_side.rs"]
mod small_side;
use small_side::SmallSide;

struct Mem {
    d: Vec<f32>,
    l: Level,
    lp: Level,
}
impl Planes for Mem {
    fn src(&self, j: PairPlane) -> (&[f32], u32) {
        (&self.d[(j.of(&self.lp) / 4) as usize..], (j.of(&self.lp) & 15) as u32)
    }
    fn src_mut(&mut self, j: PairPlane) -> &mut [f32] {
        let o = (j.of(&self.lp) / 4) as usize;
        &mut self.d[o..]
    }
    fn dst(&mut self, j: PairPlane) -> &mut [f32] {
        let o = (j.of(&self.l) / 4) as usize;
        &mut self.d[o..]
    }
}
struct Count(Vec<u32>);
impl Host for Count {
    fn helper(&mut self, c: HelperCall) {
        self.0.push(if c.kind == HelperKind::Pair { 0x490ae0 } else { 0x490a50 });
    }
    fn tile_pair<P: Planes>(&mut self, _m: &mut P, _t: &Tile) {
        self.0.push(1);
    }
    fn scale<P: Planes>(&mut self, _m: &mut P, _j: PairPlane, _l: &Level, _r: f32) {
        self.0.push(2);
    }
    fn border<P: Planes>(&mut self, _m: &mut P, _j: PairPlane, _l: &Level) {
        self.0.push(3);
    }
}

fn levels(lw: i32, lh: i32, pw: i32, ph: i32) -> (Level, Level) {
    let plane = |k: usize, base: usize| (4 * 4096 * (2 * k + base)) as u64;
    let mk = |w: i32, h: i32, base: usize| Level {
        width: w,
        height: h,
        stride: w.max(0),
        flow_u: plane(0, base),
        flow_v: plane(1, base),
        nest1_t: plane(2, base),
        nest1_u: plane(3, base),
        nest2_t: plane(4, base),
        nest2_u: plane(5, base),
        ..Level::default()
    };
    (mk(lw, lh, 1), mk(pw, ph, 2))
}

#[test]
fn flag_path_makes_five_helper_calls_and_no_plane_change() {
    let (l, lp) = levels(8, 8, 4, 4);
    let mut m = Mem { d: vec![1.5; 4096 * 14], l, lp };
    let mut h = Count(vec![]);
    run_stretch(&l, &lp, 7, 1, &mut m, &mut h);
    assert_eq!(h.0, vec![0x490ae0, 0x490a50, 0x490a50, 0x490a50, 0x490a50]);
    assert!(m.d.iter().all(|x| *x == 1.5));
}

#[test]
fn degenerate_sides_run_no_row_and_do_not_panic() {
    for (lw, lh, pw, ph) in [(0, 5, 3, 3), (-3, -3, 4, 4), (5, 5, 0, 0), (5, -1, -1, 5), (1, 1, 1, 1)] {
        let (l, lp) = levels(lw, lh, pw, ph);
        let mut m = Mem { d: vec![2.0; 4096 * 14], l, lp };
        run_stretch(&l, &lp, 0, 0, &mut m, &mut SmallSide(Direct));
    }
}

#[test]
fn mean_path_fills_the_finer_plane_with_the_mean() {
    let (l, lp) = levels(3, 2, 4, 1);
    let mut m = Mem { d: vec![0.0; 4096 * 14], l, lp };
    for j in [PairPlane::FlowU, PairPlane::FlowV, PairPlane::Nest1T, PairPlane::Nest1U, PairPlane::Nest2T, PairPlane::Nest2U] {
        let o = (j.of(&lp) / 4) as usize;
        for k in 0..4 {
            m.d[o + k] = (k + 1) as f32;
        }
    }
    let mut h = SmallSide(Count(vec![]));
    run_stretch(&l, &lp, 0, 0, &mut m, &mut h);
    assert_eq!(h.0 .0, vec![2, 2, 3, 3, 3, 3]);
    let o = (l.flow_u / 4) as usize;
    assert_eq!(&m.d[o..o + 3], &[2.5, 2.5, 2.5]);
}

#[test]
fn tile_path_direct_runs_both_passes() {
    let (l, lp) = levels(8, 6, 4, 3);
    let mut m = Mem { d: vec![0.0; 4096 * 14], l, lp };
    for j in [PairPlane::FlowU, PairPlane::FlowV, PairPlane::Nest1T, PairPlane::Nest1U, PairPlane::Nest2T, PairPlane::Nest2U] {
        let o = (j.of(&lp) / 4) as usize;
        for k in 0..12 {
            m.d[o + k] = 3.0;
        }
    }
    run_stretch(&l, &lp, 0, 0, &mut m, &mut Direct);
    let o = (l.flow_u / 4) as usize;
    // a flat source resamples to a flat plane (the scale pass then multiplies by 8/4 = 2 for word 12)
    assert!(m.d[o..o + 8].iter().all(|x| (*x - 6.0).abs() < 1e-3));
}
