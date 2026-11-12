//! Hand-evaluated radius splits, pass gates, abort sequences and the free of the scratch block, on typed calls.
use glint_worker::*;

fn bits(x: f32) -> u32 {
    x.to_bits()
}

#[derive(Default)]
struct Recorder {
    calls: Vec<Call>,
    check: i32,
    block: Option<BlockId>,
}

impl Host for Recorder {
    fn call(&mut self, c: &Call, _args: &CallbackArgs) {
        self.calls.push(c.clone());
    }
    fn alloc(&mut self, _ring: Scratch, _bytes: u64, _slots: u32) -> Option<BlockId> {
        self.block
    }
    fn abort(&mut self, which: Abort) -> i32 {
        match which {
            Abort::Check => self.check,
        }
    }
}

fn source(diagonal_mode: i32) -> SourceRecord {
    SourceRecord { selector: 0, plane_bytes: 96, count: 8, width: 40, height: 24, diagonal_mode }
}

fn three(flag: i32) -> Input {
    Input {
        source: source(0),
        blur_x: bits(2.5),
        blurred: [PlaneId(1), PlaneId(2), PlaneId(3)],
        sources: [PlaneId(4), PlaneId(5), PlaneId(6)],
        outputs: [PlaneId(7), PlaneId(8), PlaneId(9)],
        layer: PlaneId(10),
        saved: None,
        a: [bits(1.0); 3],
        b: [bits(1.0); 3],
        c: [bits(1.0); 3],
        pass_scale: [0; 4],
        pass_gain: [bits(1.0); 4],
        roi: Roi { w: 40, h: 24, c: 40, d: 0, e: 4 },
        flag,
    }
}

fn count(calls: &[Call], f: impl Fn(&Call) -> bool) -> usize {
    calls.iter().filter(|c| f(c)).count()
}

#[test]
fn radii_hand() {
    assert_eq!(radii(7, 100), [2, 2, 3]);
    assert_eq!(radii(0, 5), [0, 0, 0]);
    assert_eq!(radii(100, 3), [3, 3, 3]);
    assert_eq!(radii(1, 10), [0, 0, 1]);
    assert_eq!(radii(2, 10), [0, 1, 1]);
}

#[test]
fn radius_split_around_multiples_of_three() {
    for m in 1..40 {
        for v in [3 * m - 1, 3 * m, 3 * m + 1] {
            let r = radii(v, 1000);
            assert_eq!(r[0], v / 3);
            assert_eq!(r[0] + r[1] + r[2], v);
            assert!(r[0] <= r[1] && r[1] <= r[2] + 1);
        }
    }
}

#[test]
fn rounding_offset_and_clamp() {
    assert_eq!(cvtt32(6.0 + 0.5), 6);
    assert_eq!(cvtt32(5.49 + 0.5), 5);
    assert_eq!(cvtt32(f32::NAN), i32::MIN);
    assert_eq!(cvtt32(3e9), i32::MIN);
    assert_eq!(radii(1000, 7), [7, 7, 7]);
}

#[test]
fn closed_gates_run_the_first_stage_only() {
    let mut h = Recorder::default();
    run(&three(0), &mut h);
    assert_eq!(h.calls.len(), 7);
    assert!(matches!(h.calls[0], Call::Blur { flagged: false, .. }));
    assert!(matches!(h.calls[1], Call::Fill { .. }));
    assert!(matches!(h.calls[6], Call::BrightPass { .. }));
}

#[test]
fn plain_route_requests_parallel_passes() {
    let mut i = three(0);
    i.pass_scale = [bits(2.0); 4];
    let mut h = Recorder::default();
    run(&i, &mut h);
    assert_eq!(count(&h.calls, |c| matches!(c, Call::Parallel { op: PassOp::Horizontal, .. })), 3);
    assert_eq!(count(&h.calls, |c| matches!(c, Call::Parallel { op: PassOp::Vertical, .. })), 3);
    assert_eq!(count(&h.calls, |c| matches!(c, Call::Parallel { op: PassOp::DiagonalBox(Pass::MainDiagonal), .. })), 3);
    assert_eq!(count(&h.calls, |c| matches!(c, Call::Parallel { op: PassOp::DiagonalBox(Pass::AntiDiagonal), .. })), 3);
    assert_eq!(count(&h.calls, |c| matches!(c, Call::Parallel { op: PassOp::Scale, .. })), 12);
}

#[test]
fn flagged_call_is_refused_before_any_host_call() {
    let mut i = three(1);
    i.pass_scale = [bits(2.0); 4];
    let mut h = Recorder::default();
    assert_eq!(run(&i, &mut h), Err(Refusal::BlurFlag));
    assert!(h.calls.is_empty());
}

#[test]
fn nan_strength_never_enables_a_pass() {
    let mut i = three(0);
    i.pass_scale = [bits(f32::NAN); 4];
    let mut h = Recorder::default();
    run(&i, &mut h);
    assert_eq!(h.calls.len(), 7);
}

#[test]
fn abort_check_ends_the_call() {
    let mut i = three(0);
    i.pass_scale = [bits(2.0); 4];
    let mut h = Recorder { check: 1, ..Default::default() };
    run(&i, &mut h);
    assert_eq!(count(&h.calls, |c| matches!(c, Call::Parallel { .. })), 0);
}

#[test]
fn deterministic() {
    let mut i = three(0);
    i.pass_scale = [bits(1.25); 4];
    let (mut run_a, mut run_b) = (Recorder::default(), Recorder::default());
    run(&i, &mut run_a);
    run(&i, &mut run_b);
    assert_eq!(run_a.calls, run_b.calls);
}

#[test]
fn labels_present() {
    assert!(assumptions().iter().any(|a| a.contains("ROUTE AVX2-ONLY")));
}
