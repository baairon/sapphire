// debugging aid (hand written): replays one case of one set of f_00020bd5 and prints the differing write runs
use neutral_world::{load, World, Env, CallRet};
use lift_6::p2v::p006 as p;
use std::path::PathBuf;

fn code() -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let p = d.join("original").join("neutral_3.text.bin");
        if p.exists() { return std::fs::read(p).unwrap(); }
        assert!(d.pop());
    }
}

pub struct LogEnv<'a, 'w> { w: &'a mut World<'w>, watch: Vec<(u64, u64)>, pub log: Vec<String> }
impl<'a, 'w> Env for LogEnv<'a, 'w> {
    fn r8(&mut self, a: u64) -> u8 { let v = self.w.r8(a); if self.watch.iter().any(|(lo, hi)| a >= *lo && a < *hi) { self.log.push(format!("r8 {:x} {:02x}", a, v)); } v }
    fn w8(&mut self, a: u64, v: u8) {
        if self.watch.iter().any(|(lo, hi)| a >= *lo && a < *hi) { self.log.push(format!("w8 {:x} {:02x}", a, v)); }
        self.w.w8(a, v)
    }
    fn call_f(&mut self, target: u64, iargs: &[u64], fargs: &[u64]) -> CallRet {
        self.log.push(format!("call {:x} args {:x?}", target, &iargs[..iargs.len().min(3)]));
        self.w.call_f(target, iargs, fargs)
    }
    fn code_va(&self, off: u64) -> u64 { self.w.code_va(off) }
    fn data_va(&self, rel: i64) -> u64 { self.w.data_va(rel) }
}

#[test]
#[ignore = "needs the stored vectors from sapphire_archive.zip"]
fn dbg_20bd5() {
    let set = std::env::var("DBG_SET").unwrap_or("3".into());
    let case: usize = std::env::var("DBG_CASE").unwrap_or("31".into()).parse().unwrap();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("vectors");
    let name = "f_00020bd5";
    let path = dir.join(format!("{}.world{}.vec", name, set));
    let c = code();
    lift_6::pin::set_pins(lift_6::pin::load_pins(&PathBuf::from(format!("{}.pins", path.display()))));
    lift_6::models::install_from(&PathBuf::from(format!("{}.models", path.display())));
    let vf = load(&path).unwrap();
    lift_6::models::set_stub_zone(vf.header.stub_zone);
    let cs = &vf.cases[case];
    let mut w = World::new(&vf.header, cs, Some(&c));
    lift_6::x86rt::begin_case(cs.seed);
    let mut le = LogEnv { w: &mut w, watch: vec![(0x20216d80, 0x20216d88), (0x40800e80, 0x40800e88)], log: Vec::new() };
    let r = p::f_00020bd5(&mut le, [cs.int(0), cs.int(1), cs.int(2), cs.int(3), cs.int(4), cs.int(5)], [cs.f64_bits(6), cs.f64_bits(7), cs.f64_bits(8), cs.f64_bits(9), cs.f64_bits(10), cs.f64_bits(11), cs.f64_bits(12), cs.f64_bits(13)], &[]);
    for l in &le.log { println!("{}", l); }
    let (runs, _trace) = w.finish();
    println!("ret got {:x} want {:x}", r.rax, cs.rets[0].2);
    println!("runs got {} want {}", runs.len(), cs.writes.len());
    for (i, (a, b)) in runs.iter().zip(&cs.writes).enumerate() {
        if a != b {
            println!("run {} differs: got addr {:x} {:02x?}", i, a.0, a.1);
            println!("             want addr {:x} {:02x?}", b.0, b.1);
        }
    }
}
