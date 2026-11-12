//! Unit tests of the wrapper: both workers, the copy detour, error paths, NaN, determinism.
use glint_dispatch::*;

fn id(v: u64) -> PlaneId {
    PlaneId(v)
}
fn ids<const N: usize>(v: [u64; N]) -> [PlaneId; N] {
    v.map(PlaneId)
}

fn src(count: i32) -> SourceRecord {
    SourceRecord { selector: 0, plane_bytes: 0, count, width: 7, height: 5, diagonal_mode: 0 }
}

#[derive(Default)]
struct H {
    ks: Vec<i32>,
    n: usize,
    ev: Vec<String>,
    three: Vec<ThreeArgs>,
}
impl Host for H {
    fn layout(&mut self) -> i32 {
        let k = self.ks[self.n.min(self.ks.len() - 1)];
        self.n += 1;
        k
    }
    fn region(&mut self, _s: &SourceRecord) -> Roi {
        Roi { w: 7, h: 5, c: 7, d: 0, e: 4 }
    }
    fn copy_plane(&mut self, dst: PlaneId, src: PlaneId, row: i32, roi: &Roi) {
        self.ev.push(format!("cp {:x} {:x} {row} {}", dst.0, src.0, roi.e));
    }
    fn worker_three(&mut self, a: &ThreeArgs) {
        self.three.push(a.clone());
    }
}

fn layer(kind: i32) -> Layer {
    Layer {
        planes: (0..16).map(|k| id(100 + k)).collect(),
        scratch: Scratch { kind, src: ids([1000, 1001, 1002]) },
        args: GlintArgs { strength: 2.0, a: [0.5, 1.0, 2.0], b: [3.0, 5.0, 7.0], pass_scale: [0.0; 4], pass_gain: [1.0; 4] },
    }
}

#[test]
fn three_plane_values() {
    let mut h = H { ks: vec![1, 0, 0], ..Default::default() };
    wrapper(&mut h, &layer(0), &src(8), 0).unwrap();
    let a = &h.three[0];
    assert_eq!(a.scale, 0.03125 * 2.0 * 8.0);
    assert_eq!(a.a, [0.125 * 8.0 * 0.5, 0.125 * 8.0 * 1.0, 0.125 * 8.0 * 2.0]);
    assert_eq!(a.b, [12.0, 20.0, 28.0]);
    assert_eq!(a.blurred, ids([101, 102, 103]));
    assert_eq!(a.layer, id(100));
    assert_eq!(a.saved_index, None);
}

#[test]
fn one_plane_layout_is_refused_before_any_service() {
    let mut h = H { ks: vec![1, 2, 1], ..Default::default() };
    assert_eq!(wrapper(&mut h, &layer(1), &src(8), 0), Err(Error::LayoutRefused));
    assert!(h.three.is_empty() && h.ev.is_empty());
    // the layout refusal precedes the flag refusal
    let mut f = H { ks: vec![1, 2, 1], ..Default::default() };
    assert_eq!(wrapper(&mut f, &layer(1), &src(8), 1), Err(Error::LayoutRefused));
    assert!(f.three.is_empty() && f.ev.is_empty());
    // a flagged call on the three-plane layout is refused by name
    let mut g = H { ks: vec![1, 0, 0], ..Default::default() };
    assert_eq!(wrapper(&mut g, &layer(0), &src(8), 1), Err(Error::Refused(Refusal::BlurFlag)));
    assert!(g.three.is_empty() && g.ev.is_empty());
}

#[test]
fn copy_detour_and_null_read() {
    let mut h = H { ks: vec![3, 0, 0], ..Default::default() };
    wrapper(&mut h, &layer(1), &src(8), 0).unwrap();
    assert_eq!(h.ev, vec!["cp 65 3e8 0 4".to_string(), "cp 66 3e8 0 4".to_string()]);
    assert_eq!(h.three[0].sources, ids([1000, 101, 102]));
    assert_eq!(h.three[0].blurred, ids([103, 104, 105]));
    let mut run_b = H { ks: vec![2, 0, 0], ..Default::default() };
    assert_eq!(wrapper(&mut run_b, &layer(1), &src(8), 0), Err(Error::NullRead));
}

#[test]
fn nan_and_determinism() {
    let mut l = layer(0);
    l.args.strength = f32::from_bits(0x7fc0_0001);
    l.args.a[0] = f32::INFINITY;
    let run = || {
        let mut h = H { ks: vec![1, 0, 0], ..Default::default() };
        wrapper(&mut h, &l, &src(0), 0).unwrap();
        h.three[0].clone()
    };
    let a = run();
    let bits = |t: &ThreeArgs| (t.scale.to_bits(), t.a.map(f32::to_bits), t.b.map(f32::to_bits));
    assert_eq!(bits(&a), bits(&run()));
    assert_eq!(a.scale.to_bits(), 0x7fc0_0001, "the quiet payload is kept");
    assert_eq!(a.a[0].to_bits(), 0xffc0_0000, "inf times zero is the default NaN");
}

#[test]
fn width_one_and_range_errors() {
    let mut l = layer(0);
    l.planes.truncate(2);
    let mut h = H { ks: vec![1, 0, 0], ..Default::default() };
    assert!(matches!(wrapper(&mut h, &l, &src(1), 0), Err(Error::PlaneRange(_))));
}
