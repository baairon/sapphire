use optical_flow_entry::wiring::*;
use optical_flow_solve::solve::PlaneStore;
use optical_flow_solve::{Level, Params};
use std::cell::Cell;

/// Counting store: planes of one word vector per id, with read and write counts.
struct Counting {
    planes: Vec<Vec<f32>>,
    reads: Cell<usize>,
    writes: usize,
}
impl Counting {
    fn new(n: usize) -> Self {
        Counting { planes: vec![vec![0.0; 4096]; n], reads: Cell::new(0), writes: 0 }
    }
}
impl PlaneStore for Counting {
    fn read(&self, id: u64) -> Vec<f32> {
        self.reads.set(self.reads.get() + 1);
        self.planes[id as usize].clone()
    }
    fn write(&mut self, id: u64, v: &[f32]) {
        self.writes += 1;
        self.planes[id as usize] = v.to_vec();
    }
    fn align_bytes(&self, _id: u64) -> u64 {
        0
    }
}

fn params() -> Params {
    Params { outer_iters: 0, mid_iters: 0, inner_iters: 0, counted_route: 0, flow_gain: 0, relax_scale: 0, tol: 0.0, smooth_bias: 0, alt_launches: 0 }
}
fn level(w: i32, h: i32) -> Level {
    Level { width: w, height: h, stride: w, ..Default::default() }
}

#[test]
fn aborted_returns_no_frame_and_writes_nothing() {
    let mut store = Counting::new(1);
    let out = run_solve_body(&[level(8, 8)], 0, &params(), &mut store, &mut || true);
    assert_eq!(out, WiringOutcome::NoFrame);
    assert_eq!((store.reads.get(), store.writes), (0, 0));
}

#[test]
fn done_run_reports_done() {
    let mut store = Counting::new(1);
    let out = run_solve_body(&[level(8, 8)], 0, &params(), &mut store, &mut || false);
    assert_eq!(out, WiringOutcome::Done);
}

#[test]
fn small_level_refused_before_any_store_access() {
    let mut store = Counting::new(1);
    let out = run_solve_body(&[level(1, 8)], 0, &params(), &mut store, &mut || false);
    assert_eq!(out, WiringOutcome::Refused(EntryRefusal::SmallLevel));
    assert_eq!((store.reads.get(), store.writes), (0, 0));
}

#[test]
fn empty_table_has_no_levels() {
    let mut store = Counting::new(1);
    assert_eq!(run_solve_body(&[], -1, &params(), &mut store, &mut || false), WiringOutcome::NoLevels);
}

fn row(r: EntryRefusal, text: &str) {
    let rep = fx_run_refusal(r);
    assert_eq!(rep.status_code, 2);
    assert_eq!(rep.stderr, text);
    assert!(rep.output.is_none());
}

#[test]
fn row_small_level() {
    row(EntryRefusal::SmallLevel, "OPTICAL-FLOW-SMALL-LEVEL-REFUSED (optical_flow: a pyramid level with a side below 2 reached the flow solve; the level build stores no such level, so this route is not rebuilt)");
}
#[test]
fn row_level_table_full() {
    row(EntryRefusal::LevelTableFull, "OPTICAL-FLOW-LEVEL-TABLE-FULL-REFUSED (optical_flow: the pyramid would hold more levels than the level table's capacity; the original writes past its table there, so this route is not rebuilt)");
}
#[test]
fn row_null_image() {
    row(EntryRefusal::NullImage, "OPTICAL-FLOW-NULL-IMAGE-REFUSED (optical_flow: the host answered no image for a pyramid buffer; the original reads through the missing buffer there, so this route is not rebuilt)");
}
#[test]
fn row_flag_route_from_both_entries() {
    row(EntryRefusal::FlagRoute, "OPTICAL-FLOW-FLAG-ROUTE-REFUSED (optical flow: the host image handle route is not rendered)");
    assert_eq!(flag_route_refusal(1, 0), Some(EntryRefusal::FlagRoute));
    assert_eq!(flag_route_refusal(0, 1), Some(EntryRefusal::FlagRoute));
    assert_eq!(flag_route_refusal(0, 0), None);
}
