//! Cross-implementation check: vectors produced by tools/world.py (Unicorn) for five synthetic
//! functions must be reproduced exactly by ports written against the Env trait.

use neutral_world::{check, load, Env};
use std::path::PathBuf;

fn fx(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name)
}

#[test]
fn data_read() {
    let vf = load(&fx("data_read.vec")).unwrap();
    let rep = check(&vf, None, |w, _c| {
        let a = w.data_va(0x20000);
        vec![w.r32(a) as u128]
    });
    assert_eq!(rep.skipped, 0);
    rep.assert_ok(100);
}

#[test]
fn stub_call() {
    let vf = load(&fx("stub_call.vec")).unwrap();
    let rep = check(&vf, None, |w, _c| {
        let t = w.stub_va(6);
        vec![w.call(t, &[5]).rax as u128]
    });
    rep.assert_ok(100);
}

#[test]
fn heap_write_read() {
    let vf = load(&fx("heap_write_read.vec")).unwrap();
    let rep = check(&vf, None, |w, c| {
        let rdi = c.int(0);
        let esi = c.int(1) as u32;
        w.w32(rdi, esi);
        vec![w.r64(rdi + 8) as u128]
    });
    rep.assert_ok(100);
}

#[test]
fn indirect_call() {
    let vf = load(&fx("indirect_call.vec")).unwrap();
    let rep = check(&vf, None, |w, c| {
        let rdi = c.int(0);
        let t = w.r64(rdi);
        vec![w.call(t, &[rdi]).rax as u128]
    });
    // Most worlds fault in the original (the loaded word is not a callable address); the rest must match.
    assert!(rep.compared >= 20, "only {} worlds ran", rep.compared);
    rep.assert_ok(20);
}

#[test]
fn opaque_callee() {
    let vf = load(&fx("opaque_callee.vec")).unwrap();
    let rep = check(&vf, None, |w, _c| {
        let t = w.code_va(0x38);
        vec![(w.call(t, &[0]).rax as u32) as u128]
    });
    rep.assert_ok(100);
}

#[test]
fn a_wrong_port_is_caught() {
    let vf = load(&fx("stub_call.vec")).unwrap();
    // Passing the wrong argument changes the trace and the returned value.
    let rep = check(&vf, None, |w, _c| {
        let t = w.stub_va(6);
        vec![w.call(t, &[6]).rax as u128]
    });
    assert!(rep.failed > 0);
    let rep = check(&vf, None, |w, _c| {
        let t = w.stub_va(12);
        vec![w.call(t, &[5]).rax as u128]
    });
    assert!(rep.failed > 0);
}

#[test]
fn noreturn_call() {
    let vf = load(&fx("noreturn_call.vec")).unwrap();
    assert!(vf.cases.iter().all(|c| c.noreturn));
    let rep = check(&vf, None, |w, _c| {
        let t = w.stub_va(0xc);
        w.call(t, &[7]);
        vec![]
    });
    rep.assert_ok(100);
    // A port that keeps going after the noreturn call does extra work and is caught by the trace.
    let rep = check(&vf, None, |w, _c| {
        let t = w.stub_va(0xc);
        w.call(t, &[7]);
        let t2 = w.stub_va(6);
        w.call(t2, &[1]);
        vec![]
    });
    assert!(rep.failed > 0);
}

#[test]
fn ptr_chase_through_aligned_heap() {
    let vf = load(&fx("ptr_chase.vec")).unwrap();
    assert!(vf.header.aligned_heap);
    let rep = check(&vf, None, |w, c| {
        let p = c.int(0);
        let q = w.r64(p);
        vec![w.r64(q + 0x30) as u128]
    });
    assert!(rep.compared >= 20, "only {} worlds ran", rep.compared);
    rep.assert_ok(20);
}

#[test]
fn data_next_to_a_fetched_target_stays_data() {
    let vf = load(&fx("fetch_then_read.vec")).unwrap();
    assert!(vf.header.fetch_byte);
    let rep = check(&vf, None, |w, c| {
        let p = c.int(0);
        let t = w.r64(p);
        w.call(t, &[p]);
        vec![w.r64(t + 8) as u128]
    });
    assert!(rep.compared >= 20, "only {} worlds ran", rep.compared);
    rep.assert_ok(20);
}

#[test]
fn call_result_override_returns_arg0_plus_8() {
    let vf = load(&fx("ret_arg0.vec")).unwrap();
    assert_eq!(vf.header.ret_overrides.len(), 1);
    let rep = check(&vf, None, |w, c| {
        let t = w.stub_va(0x12);
        vec![w.call(t, &[c.int(0)]).rax as u128]
    });
    rep.assert_ok(100);
}
