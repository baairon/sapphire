//! The refusal contract of the glint op: a refusal returns its name, leaves every plane at its sentinel word and makes no host call
//! (the layout query is not counted: it is a pure question), on every route and at every entry.
use glint_dispatch::{refusal_of, wrapper, Error, GlintArgs, Host as DHost, Layer, Refusal, Roi as DRoi, Scratch as DScratch, SourceRecord, ThreeArgs};
use glint_wire::host::ComposedHost;
use glint_wire::op::{render as op_render, source_record, Request};
use glint_wire::refusal::{BAND, BLUR_FLAG, EXTENT, REGION_SELECTOR, SHIFT};
use glint_worker::{Abort, BlockId, Call, CallbackArgs, Host as WHost, Input, PlaneId, Roi, Scratch, SourceRecord as WSource};
use std::collections::HashMap;

const SENTINEL: u32 = 0x7fc0_ffee;

/// Dispatch-level host: every service except the layout query is logged; the planes are sentinel-filled and never touched by it.
struct Spy {
    ks: Vec<i32>,
    n: usize,
    log: Vec<&'static str>,
    planes: HashMap<PlaneId, Vec<f32>>,
}
impl Spy {
    fn new(ks: Vec<i32>, count: usize) -> Spy {
        let planes = (0..32u64).map(|k| (PlaneId(k), vec![f32::from_bits(SENTINEL); count])).collect();
        Spy { ks, n: 0, log: vec![], planes }
    }
    fn all_sentinel(&self) -> bool {
        self.planes.values().all(|p| p.iter().all(|x| x.to_bits() == SENTINEL))
    }
}
impl DHost for Spy {
    fn layout(&mut self) -> i32 {
        let k = self.ks[self.n.min(self.ks.len() - 1)];
        self.n += 1;
        k
    }
    fn region(&mut self, s: &SourceRecord) -> DRoi {
        self.log.push("region");
        DRoi { w: s.width, h: s.height, c: s.width, d: 0, e: 4 }
    }
    fn copy_plane(&mut self, _d: PlaneId, _s: PlaneId, _r: i32, _roi: &DRoi) {
        self.log.push("copy");
    }
    fn worker_three(&mut self, _a: &ThreeArgs) {
        self.log.push("three");
    }
}

fn record(selector: i32, w: i32, h: i32, diag: i32) -> SourceRecord {
    WSource { selector, plane_bytes: w * h * 4, count: w, width: w, height: h, diagonal_mode: diag }
}
fn layer(kind: i32, pass_scale: [f32; 4]) -> Layer {
    Layer {
        planes: (0..16).map(|k| PlaneId(100 + k)).collect(),
        scratch: DScratch { kind, src: [1000, 1001, 1002].map(PlaneId) },
        args: GlintArgs { strength: 1.0, a: [2.0; 3], b: [1.0; 3], pass_scale, pass_gain: [1.0; 4] },
    }
}
const NONE: [f32; 4] = [0.0; 4];
const DIAG: [f32; 4] = [0.0, 0.0, 1.0, 0.0];
const SHIFTING: [f32; 4] = [f32::INFINITY, 0.0, 0.0, 0.0];

#[derive(Clone, Copy)]
enum Route {
    ThreeDirect,
    ThreeCopy,
    One,
}
fn route_ks(r: Route) -> (Vec<i32>, i32) {
    match r {
        Route::ThreeDirect => (vec![3, 3, 3], 0),
        Route::ThreeCopy => (vec![3, 3, 3], 1),
        Route::One => (vec![1, 1, 1], 0),
    }
}

/// One wrapper call that must refuse: the name, the sentinel planes and the empty host log.
fn check_wrapper(route: Route, src_record: SourceRecord, pass_scale: [f32; 4], flag: u8, want: Refusal) {
    let (ks, kind) = route_ks(route);
    let mut spy = Spy::new(ks, (src_record.width.max(1) * src_record.height.max(1)) as usize);
    let got = wrapper(&mut spy, &layer(kind, pass_scale), &src_record, flag);
    assert_eq!(got, Err(Error::Refused(want)));
    assert!(spy.log.is_empty(), "host calls {:?}", spy.log);
    assert!(spy.all_sentinel());
}

/// The one-plane layout is refused by the layout name before any host call, whatever else the request holds.
#[test]
fn one_plane_layout_refused_with_no_host_call() {
    for (rec, ps, flag) in [(record(0, 8, 8, 0), NONE, 0), (record(0, 8, 8, 0), NONE, 1), (record(7, 8, 8, 0), NONE, 0), (record(0, 1, 5, 1), DIAG, 0), (record(0, 8, 8, 0), SHIFTING, 0)] {
        let (ks, kind) = route_ks(Route::One);
        let mut spy = Spy::new(ks, 64);
        assert_eq!(wrapper(&mut spy, &layer(kind, ps), &rec, flag), Err(Error::LayoutRefused));
        assert!(spy.log.is_empty() && spy.all_sentinel());
    }
    assert!(glint_wire::refusal::LAYOUT.starts_with("GLINT-LAYOUT-REFUSED"));
}
#[test]
fn blur_flag_three_plane_direct() {
    check_wrapper(Route::ThreeDirect, record(0, 8, 8, 0), NONE, 1, Refusal::BlurFlag);
}
#[test]
fn blur_flag_three_plane_copy_detour() {
    check_wrapper(Route::ThreeCopy, record(0, 8, 8, 0), NONE, 1, Refusal::BlurFlag);
}
#[test]
fn region_selector_three_plane_direct() {
    check_wrapper(Route::ThreeDirect, record(1, 8, 8, 0), NONE, 0, Refusal::RegionSelector);
}
#[test]
fn region_selector_three_plane_copy_detour() {
    check_wrapper(Route::ThreeCopy, record(-5, 8, 8, 0), NONE, 0, Refusal::RegionSelector);
}
#[test]
fn diag_extent_three_plane_direct() {
    check_wrapper(Route::ThreeDirect, record(0, 1, 5, 1), DIAG, 0, Refusal::DiagExtent);
}
#[test]
fn diag_extent_three_plane_copy_detour() {
    check_wrapper(Route::ThreeCopy, record(0, 1, 5, 1), DIAG, 0, Refusal::DiagExtent);
}
/// The 2x1 partition has a negative row count: refused with the extent name on both three-plane routes.
#[test]
fn diag_extent_2x1_three_plane_direct() {
    check_wrapper(Route::ThreeDirect, record(0, 2, 1, 1), DIAG, 0, Refusal::DiagExtent);
}
#[test]
fn diag_extent_2x1_three_plane_copy_detour() {
    check_wrapper(Route::ThreeCopy, record(0, 2, 1, 1), DIAG, 0, Refusal::DiagExtent);
}
#[test]
fn shift_three_plane_direct() {
    check_wrapper(Route::ThreeDirect, record(0, 8, 8, 0), SHIFTING, 0, Refusal::Shift);
}
#[test]
fn shift_three_plane_copy_detour() {
    check_wrapper(Route::ThreeCopy, record(0, 8, 8, 0), SHIFTING, 0, Refusal::Shift);
}

/// Worker-level spy: any call at all is a failure.
#[derive(Default)]
struct WSpy {
    calls: usize,
}
impl WHost for WSpy {
    fn call(&mut self, _c: &Call, _a: &CallbackArgs) {
        self.calls += 1;
    }
    fn alloc(&mut self, _r: Scratch, _b: u64, _s: u32) -> Option<BlockId> {
        self.calls += 1;
        None
    }
    fn abort(&mut self, _w: Abort) -> i32 {
        self.calls += 1;
        0
    }
}
fn three_input(src_record: SourceRecord, pass_scale: [f32; 4], flag: i32) -> Input {
    let u = f32::to_bits;
    Input {
        source: src_record, blur_x: 0, blurred: [1, 2, 3].map(PlaneId), sources: [11, 12, 13].map(PlaneId), outputs: [21, 22, 23].map(PlaneId), layer: PlaneId(31), saved: None,
        a: [u(2.0); 3], b: [u(1.0); 3], c: [0; 3], pass_scale: pass_scale.map(u), pass_gain: [u(1.0); 4], roi: Roi { w: src_record.width, h: src_record.height, c: src_record.width, d: 0, e: 4 }, flag,
    }
}

#[test]
fn three_plane_worker_entry_refuses_each_name_with_no_host_call() {
    for (src_record, ps, flag, want) in [
        (record(0, 8, 8, 0), NONE, 1, Refusal::BlurFlag),
        (record(3, 8, 8, 0), NONE, 0, Refusal::RegionSelector),
        (record(0, 1, 5, 1), DIAG, 0, Refusal::DiagExtent),
        (record(0, 8, 8, 0), SHIFTING, 0, Refusal::Shift),
    ] {
        let mut s = WSpy::default();
        assert_eq!(glint_worker::run(&three_input(src_record, ps, flag), &mut s), Err(want));
        assert_eq!(s.calls, 0);
    }
}

fn request(w: usize, h: usize, flag: u8, pass_scale: [f32; 4], planes: &[Vec<f32>; 3]) -> Request<'_> {
    Request {
        width: w, height: h, count: w as i32, sources: [&planes[0], &planes[1], &planes[2]], thresholds: [0.5; 3], strength: 1.0, a: [2.0; 3], b: [1.0; 3],
        pass_scale, pass_gain: [1.0; 4], align: Default::default(), flag,
    }
}
#[test]
fn op_entry_refuses_each_name_before_any_plane_is_built() {
    let (w, h) = (1usize, 5usize);
    let planes = [0, 1, 2].map(|_| vec![f32::from_bits(SENTINEL); w * h]);
    assert_eq!(op_render(&request(w, h, 1, NONE, &planes)).unwrap_err(), BLUR_FLAG);
    assert_eq!(op_render(&request(8, 8, 0, SHIFTING, &[0, 1, 2].map(|_| vec![f32::from_bits(SENTINEL); 64]))).unwrap_err(), SHIFT);
    // the extent refusal needs the diagonal word, which the op never sets: the op renders the width-1 frame instead of refusing
    assert!(op_render(&request(w, h, 0, DIAG, &planes)).is_ok());
    // a source of the wrong size is reported only after the predicate, so a flagged call with a short plane still returns the flag name
    assert_eq!(op_render(&request(8, 8, 1, NONE, &planes)).unwrap_err(), BLUR_FLAG);
    for p in &planes {
        assert!(p.iter().all(|x| x.to_bits() == SENTINEL));
    }
}
#[test]
fn op_entry_names_match_the_worker_names() {
    assert_eq!(Refusal::BlurFlag.name(), BLUR_FLAG);
    assert_eq!(Refusal::RegionSelector.name(), REGION_SELECTOR);
    assert_eq!(Refusal::DiagExtent.name(), EXTENT);
    assert_eq!(Refusal::Shift.name(), SHIFT);
}
/// op.rs, `source_record` (the `selector: 0` field) is the only place the op builds the record.
#[test]
fn op_entry_builds_selector_zero_for_every_served_size() {
    for w in [1usize, 2, 3, 5, 8, 64, 1920, 4096] {
        for h in [1usize, 2, 5, 8, 1080, 2160] {
            let r = source_record(w, h, w as i32).unwrap();
            assert_eq!(r.selector, 0);
            assert!(refusal_of(&r, &layer(0, NONE).args, 0).is_none());
        }
    }
}

fn host_for(w: usize, h: usize) -> ComposedHost {
    let mut ho = ComposedHost::new([1, 2, 3].map(PlaneId), [0.5; 3], None);
    for k in [1u64, 2, 3, 11, 12, 13, 21, 22, 23, 31] {
        ho.planes.insert(PlaneId(k), (0..w * h).map(|i| (i % 7) as f32 * 0.25).collect());
    }
    ho.last_alloc = (((w.max(h)) * 4) as u64, 3);
    ho
}
/// No request that passes the predicate reaches the band guard, over width and height 1 to 8 and both diagonal modes.
#[test]
fn no_request_passing_the_predicate_reaches_the_band_guard() {
    let specials = [1.0f32, 0.5, 3.0, f32::INFINITY, f32::NAN, 0.0];
    let mut ran = 0usize;
    let mut hits: Vec<String> = vec![];
    for w in 1..=8i32 {
        for h in 1..=8i32 {
            for diag in [0, 1] {
                for &t in &specials {
                    for pos in 0..4usize {
                        let mut ps = [1.0f32, 1.0, 1.0, 1.0];
                        ps[pos] = t;
                        let inp = three_input(record(0, w, h, diag), ps, 0);
                        if glint_worker::refusal_three(&inp).is_some() {
                            continue;
                        }
                        let mut ho = host_for(w as usize, h as usize);
                        let _ = glint_wire::render(&inp, &mut ho);
                        if ho.err.as_deref() == Some(BAND) {
                            hits.push(format!("{w}x{h} diag {diag} T0[{pos}]={t}"));
                        }
                        ran += 1;
                    }
                }
            }
        }
    }
    assert!(ran > 500, "{ran}");
    assert!(hits.is_empty(), "BAND reached by {} passing requests, first {:?}", hits.len(), &hits[..hits.len().min(4)]);
}
