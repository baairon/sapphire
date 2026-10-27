use neutral_world::{load, World};
use std::path::PathBuf;

#[test]
#[ignore = "debug helper: needs DBG_VEC (and DBG_CASE, DBG_NAME) in the environment, makes no parity claim"]
fn dbg_word() {
    let vec = std::env::var("DBG_VEC").unwrap();
    let vf = load(&PathBuf::from(&vec)).unwrap();
    let c = &vf.cases[0];
    let w = World::new(&vf.header, c, None);
    for a in [0x200ed290u64, 0x200ed298] {
        println!("WORD {:x} seed {:x} word {:x}", a, c.seed, w.lay.word(c.seed, a));
    }
}
