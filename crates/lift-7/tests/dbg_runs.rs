use neutral_world::{load, World, WCase};
use std::path::PathBuf;

#[test]
#[ignore = "debug helper: needs DBG_VEC (and DBG_CASE, DBG_NAME) in the environment, makes no parity claim"]
fn dbg_runs() {
    let vec = std::env::var("DBG_VEC").unwrap();
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let code = loop {
        let p = d.join("original").join("neutral_3.text.bin");
        if p.exists() { break std::fs::read(p).unwrap(); }
        assert!(d.pop());
    };
    lift_7::pin::load_sidecar(&PathBuf::from(format!("{}.pins", vec)));
    let vf = load(&PathBuf::from(&vec)).unwrap();
    let c: &WCase = &vf.cases[0];
    let mut w = World::new(&vf.header, c, Some(&code));
    {
        let mut pe = lift_7::pin::PinEnv::new(&mut w);
        let _ = lift_7::p2v::q001::f_0001b438(&mut pe, [c.int(0), c.int(1), c.int(2), 0, c.int(3), c.int(4)], [c.f64_bits(5), c.f64_bits(6), c.f64_bits(7), c.f64_bits(8), c.f64_bits(9), c.f64_bits(10), c.f64_bits(11), c.f64_bits(12)], &[]);
    }
    let (runs, _t) = w.finish();
    for (i, r) in runs.iter().enumerate().take(6) {
        println!("RUN {} {:x} len {} bytes {:?}", i, r.0, r.1.len(), r.1);
    }
    for (i, r) in c.writes.iter().enumerate().take(6) {
        println!("WANT {} {:x} len {} bytes {:?}", i, r.0, r.1.len(), r.1);
    }
}
