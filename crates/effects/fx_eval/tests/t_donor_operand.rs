//! Type-only operands: a computed node at a donor position is passed as a -999 lane constant and builds nothing; a donor with another
//! consumer is released at that consumer's last use. Programs are inline text; no stored data.
use fx_eval::*;

fn plan(text: &str) -> Vec<String> {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", text).unwrap();
    let ei = c.find_effect("T", None).unwrap();
    let g = build_effect(&c, ei).unwrap();
    let cp = compile(&g, &ComboTable::default(), &CompileOptions::default()).unwrap();
    cpuplan::plan_text_cpu(&cp).lines().map(|s| s.to_string()).collect()
}

fn runs_of(lines: &[String], class: &str) -> usize {
    lines.iter().map(|l| l.matches(&format!(" {} :ref", class)).count()).sum()
}

#[test]
fn computed_donor_of_coerce_up_a1_builds_nothing() {
    let p = plan("(def_effect T ( S ) ( ) ( ) (coerce_up_a1 S (remove_alpha (blur S 3 3 1 0 1 1 0))))");
    let all = p.join("\n");
    assert!(all.contains("coerce_up_a1 "), "{}", all);
    assert!(all.contains("[ -999"), "{}", all);
    assert_eq!(runs_of(&p, "blur"), 0, "{}", all);
    assert_eq!(runs_of(&p, "remove_alpha"), 0, "{}", all);
}

#[test]
fn donor_with_another_consumer_is_kept_for_that_consumer() {
    let p = plan("(def_effect T ( S ) ( ) ( ) (let ( d = (remove_alpha S) ) (v3_to_v4_if4 d (get_alpha S) (* d 2))))");
    let all = p.join("\n");
    assert_eq!(runs_of(&p, "remove_alpha"), 1, "{}", all);
    assert_eq!(runs_of(&p, "*"), 0, "{}", all);
    assert!(all.contains("[ -999"), "{}", all);
    // the shared producer is released once, at the last command that reads it
    let free_at = p.iter().position(|l| l.contains("v3_to_v4_if4")).unwrap();
    assert!(p[free_at..].iter().any(|l| l.contains("free-images")), "{}", all);
}
