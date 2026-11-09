//! FPS-UNGIVEN: on the verified render path a program that reads get_frames_per_second refuses without a given rate.
use fx_exec::exec::{compile_for, Binding};

#[test]
fn ungiven_rate_is_refused_by_name() {
    let c = fx_eval::Corpus::native();
    let name = fx_eval::programs_native::registered_names().remove(0);
    let mut g = fx_eval::build_effect(&c, c.find_effect(&name, None).unwrap()).unwrap();
    let mut n = g.nodes[0].clone();
    n.name = "get_frames_per_second".into();
    g.nodes.push(n);
    let mut b = Binding::defaults();
    b.frame_index_required = true;
    let e = compile_for(&g, &b).err().expect("refused without a rate");
    assert!(format!("{}", e).contains("FPS-UNGIVEN"), "{}", e);
    b.host.insert("get_frames_per_second".into(), vec![24.0]);
    let e2 = compile_for(&g, &b).err().map(|e| format!("{}", e)).unwrap_or_default();
    assert!(!e2.contains("FPS-UNGIVEN"), "{}", e2);
}
