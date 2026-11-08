//! The second input of a transition program (SwishPan) through the executor: a make_mipmap node that is read only
//! linewise has the plain image format, so a screen_super that takes it as an operand finds its planes; a bound
//! To image and a To of all-zero pixels both render, and the two renders differ where the transition reads To.
use fx_eval::{build_effect, Corpus};
use fx_exec::{compile_for, execute_compiled, Binding, HostFrames, Image, Inputs};
use std::collections::BTreeMap;

fn grad(w: usize, h: usize, ch: usize, bias: f32) -> Image {
    let n = w * h;
    let planes: Vec<Vec<f32>> = (0..ch).map(|c| (0..n).map(|i| bias + (i as f32 / n as f32) * (0.25 + 0.25 * c as f32)).collect()).collect();
    Image::from_planes(planes).unwrap()
}

fn zeros(w: usize, h: usize) -> Image {
    Image::from_planes(vec![vec![0.0f32; w * h]; 4]).unwrap()
}

fn run(amount: f32, to: Image) -> Result<Vec<Vec<u32>>, String> {
    let (w, h) = (16usize, 8usize);
    let corpus = Corpus::native();
    let ei = corpus.find_effect("SwishPan", None).expect("SwishPan is registered");
    let g = build_effect(&corpus, ei).map_err(|e| format!("{e:?}"))?;
    let mut b = Binding::defaults();
    b.hc_zero_legacy = true;
    b.overrides.insert("auto_trans".to_string(), vec![0.0]);
    b.overrides.insert("wipe_amt".to_string(), vec![amount]);
    let cp = compile_for(&g, &b).map_err(|e| e.to_string())?;
    let mut images = BTreeMap::new();
    images.insert("From".to_string(), grad(w, h, 4, 0.1));
    images.insert("To".to_string(), to);
    let inputs = Inputs { width: w, height: h, images };
    let out = execute_compiled(&cp, &b, &inputs, &HostFrames::default()).map_err(|e| e.to_string())?;
    Ok(out.dest.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect())
}

#[test]
fn linewise_mipmap_is_a_plain_image() {
    let (w, h) = (16usize, 8usize);
    let corpus = Corpus::native();
    let ei = corpus.find_effect("SwishPan", None).unwrap();
    let g = build_effect(&corpus, ei).unwrap();
    let mut b = Binding::defaults();
    b.hc_zero_legacy = true;
    b.overrides.insert("auto_trans".to_string(), vec![0.0]);
    let cp = compile_for(&g, &b).unwrap();
    let fmts: Vec<u32> = cp.nodes.iter().filter(|n| n.prim == "make_mipmap" && n.idx != cp.root).map(|n| fx_eval::cpuplan::node_format(&cp, n.idx)).collect();
    assert!(fmts.contains(&2), "the From mipmap is read only by screen_super: {fmts:?}");
    let mut images = BTreeMap::new();
    images.insert("From".to_string(), grad(w, h, 4, 0.1));
    images.insert("To".to_string(), grad(w, h, 4, 0.3));
    let r = execute_compiled(&cp, &b, &Inputs { width: w, height: h, images }, &HostFrames::default());
    assert!(r.is_ok(), "{:?}", r.err().map(|e| e.to_string()));
}

#[test]
fn bound_to_and_zero_to_render_differently_inside_the_range() {
    let bound = run(0.5, grad(16, 8, 4, 0.3)).expect("a bound To renders");
    let none = run(0.5, zeros(16, 8)).expect("an all-zero To renders");
    assert_eq!(bound.len(), 4);
    assert_ne!(bound, none, "the To image reaches the output");
}
