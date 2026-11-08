//! Top-level free order: an image is freed after its last consumer command, in the order of its last use inside that command.
use fx_eval::*;

fn plan(text: &str) -> String {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", text).unwrap();
    let g = build_effect(&c, c.find_effect("T", None).unwrap()).unwrap();
    let cp = compile(&g, &ComboTable::default(), &CompileOptions::default()).unwrap();
    cpuplan::plan_text_cpu(&cp)
}

/// Images read by one block: a (first and last item), b (second item), c (between). They are freed in the order of their last use.
#[test]
fn top_level_free_follows_last_use() {
    let p = plan("(def_effect T ( S M ) ( ) ( ) ( + ( + ( blur S 2 ) ( blur M 3 ) ) ( * ( + ( * S 2 ) M ) S ) ))");
    let frees: Vec<&str> = p.lines().filter(|l| l.starts_with("CMD") && l.contains("\tfree-images\t")).map(|l| l.split('\t').nth(3).unwrap()).collect();
    assert_eq!(
        frees,
        vec!["(free-images :ref5.2)", "(free-images :ref3.2)", "(free-images :ref0.2)", "(free-images :ref2.2)", "(free-images :ref10.2)"],
        "{}",
        p
    );
}
