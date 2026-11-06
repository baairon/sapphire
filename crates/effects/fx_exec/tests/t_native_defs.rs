//! The selector tables defined in `effects_rtdata` equal the stored copy of COMBOS_EXEC (test data under `$VEC_DIR/effects/rtdata`;
//! NOT RUN when the copy is absent): every defined class holds every entry of the copy's class, in the copy's order.
use fx_exec::selector::Tables;

#[test]
fn selector_tables_equal_the_copy() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let full = Tables::from_tsv(effects_rtdata::text("fx_exec/data/COMBOS_EXEC.tsv"));
    let mut entries = 0;
    for c in effects_rtdata::classes::COMBOS_EXEC {
        let want = &full.classes[c.class];
        assert_eq!(want.len(), c.entries.len(), "{}", c.class);
        for (w, d) in want.iter().zip(c.entries) {
            assert_eq!((&w.vals[..], w.tag, w.len), (d.vals, d.tag, d.len), "{}", c.class);
        }
        entries += c.entries.len();
    }
    let d = Tables::embedded();
    assert_eq!(d.classes.len(), effects_rtdata::classes::COMBOS_EXEC.len());
    println!("COMBOS_EXEC: {} classes {} entries equal copy rows", effects_rtdata::classes::COMBOS_EXEC.len(), entries);
}
