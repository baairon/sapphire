use neutral_world::{check, load};
use std::path::PathBuf;

#[test]
#[ignore = "debug helper: needs DBG_VEC (and DBG_CASE, DBG_NAME) in the environment, makes no parity claim"]
fn dbg_case() {
    let case: usize = std::env::var("DBG_CASE").unwrap().parse().unwrap();
    let name = std::env::var("DBG_NAME").unwrap();
    let off = u64::from_str_radix(name.trim_start_matches("f_"), 16).unwrap();
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let code = loop {
        let p = d.join("original").join("neutral_3.text.bin");
        if p.exists() { break std::fs::read(p).unwrap(); }
        assert!(d.pop());
    };
    let vec = std::env::var("DBG_VEC").unwrap();
    lift_7::pin::load_sidecar(&PathBuf::from(format!("{}.pins", vec)));
    let mut vf = load(&PathBuf::from(&vec)).unwrap();
    vf.cases = vec![vf.cases[case].clone()];
    let _ = off;
    let rep = check(&vf, Some(&code), &mut |w: &mut neutral_world::World, c: &neutral_world::WCase| {
        let mut pe = lift_7::pin::PinEnv::new(w);
        let mut le = lift_7::pin::LogEnv { inner: &mut pe };
        let r = lift_7::p2v::q001::f_0001b438(&mut le, [c.int(0), c.int(1), c.int(2), 0, c.int(3), c.int(4)], [c.f64_bits(5), c.f64_bits(6), c.f64_bits(7), c.f64_bits(8), c.f64_bits(9), c.f64_bits(10), c.f64_bits(11), c.f64_bits(12)], &[]);
        vec![r.rax as u128]
    });
    println!("REPORT compared={} failed={}", rep.compared, rep.failed);
}
