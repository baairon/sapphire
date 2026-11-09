//! The writers of the placement model as the executor uses them (--xy-base-list route): one entry per default-xy writer, each with its own consumer list
//! and its own base; a preserve writer runs in render 1 only, so render 2 takes its render 1 base and refuses an entry that is not marked reused.
//! Blur_Motion_Autogen has three writers in plan order: two planes (not preserved) and a row buffer inside the preserve group. One test function: the
//! writer statics are process wide.
mod t_ds3_common;
use fx_exec::exec::*;
use fx_exec::types::XyHeadMode;
use t_ds3_common::program;

#[test]
fn writers_keep_their_consumers_and_bases_and_the_preserve_writer_is_reused() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = program("Blur_Motion_Autogen");
    let b = Binding::defaults();
    // render 1: every writer runs at its listed base with the head form of its placement
    *XY_LIST.lock().unwrap() = Some((vec![(5, false), (9, false), (11, false)], 1));
    compile_for(&g, &b).expect("render 1");
    let w1 = XY_WRITERS.lock().unwrap().clone();
    assert_eq!(w1.len(), 3, "one entry per writer, no merge of the consumers");
    assert_eq!(w1[0].0, vec![(87usize, 0usize), (87, 1)]);
    assert_eq!(w1[1].0, vec![(98usize, 0usize)]);
    assert_eq!(w1[2].0, vec![(95usize, 0usize)]);
    assert_eq!(w1.iter().map(|w| w.1).collect::<Vec<_>>(), vec![5, 9, 11]);
    assert_eq!(w1.iter().map(|w| w.2).collect::<Vec<_>>(), vec![XyHeadMode::PerRow, XyHeadMode::PerRow, XyHeadMode::OneHead]);
    assert_eq!(w1.iter().map(|w| w.3).collect::<Vec<_>>(), vec![false, false, true]);
    // render 2: the preserve writer carries its render 1 base, marked reused
    *XY_LIST.lock().unwrap() = Some((vec![(5, false), (9, false), (11, true)], 2));
    compile_for(&g, &b).expect("render 2");
    let w2 = XY_WRITERS.lock().unwrap().clone();
    assert_eq!(w2.len(), 3);
    assert_eq!(w2.iter().map(|w| w.1).collect::<Vec<_>>(), vec![5, 9, 11], "the preserve writer keeps the render 1 base in render 2");
    // render 2 with the preserve entry not marked reused is refused
    *XY_LIST.lock().unwrap() = Some((vec![(5, false), (9, false), (11, false)], 2));
    let e = compile_for(&g, &b).err().expect("refused");
    assert!(format!("{:?}", e).contains("XY-BASE-PRESERVE-REUSE-REQUIRED"), "{:?}", e);
    *XY_LIST.lock().unwrap() = None;
}
