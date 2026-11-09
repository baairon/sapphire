//! A per-channel program built from a defun that takes one channel and keeps the alpha (copy_alpha of the element_of image and the source) and the tail of a channel
//! assembly (v3_to_v4_if4_a of the v3, a scalar built from get_alpha of the three channels, and the source). On a 3 element source (no alpha) the
//! compile drops every copy_alpha and the v3_to_v4_if4_a node and the rendered planes are the channels of the source (the identity nodes pass them
//! through); on a 4 element source both classes stay. The programs are inline text; no stored data.
use fx_eval::{build_effect, Corpus};
use fx_exec::exec::{compile_for, execute, Binding, HostFrames, Inputs};
use fx_exec::types::Image;
use std::collections::BTreeMap;

const ONE_CHANNEL_KEEPING_ALPHA: &str = "(defun one_channel_keeping_alpha (img ch) (copy_alpha (element_of img ch) img))";
const PROBE: &str = "(def_effect Probe (Source) (rgba_ok) () (let ( r = (one_channel_keeping_alpha Source 0) g = (one_channel_keeping_alpha Source 1) b = (one_channel_keeping_alpha Source 2) ) \
    (v3_to_v4_if4_a (v3 (element_of r 0) (element_of g 0) (element_of b 0)) (* 0.33 (+ (get_alpha r) (+ (get_alpha g) (get_alpha b)))) Source)))";

fn genotype() -> fx_eval::Genotype {
    let mut c = Corpus::new();
    c.add_text("defun", "one_channel_keeping_alpha", "0x0", ONE_CHANNEL_KEEPING_ALPHA).expect("parse defun");
    c.add_text("def_effect", "Probe", "0x0", PROBE).expect("parse effect");
    let ei = c.find_effect("Probe", None).expect("find");
    build_effect(&c, ei).expect("build")
}

fn source(channels: usize, n: usize) -> Image {
    let planes = (0..channels)
        .map(|k| (0..n).map(|i| 0.05 + ((i * 7 + k * 13) % 29) as f32 / 40.0).collect())
        .collect();
    Image::from_planes(planes).expect("planes")
}

/// Runs the probe on a `channels` element source at 6 x 3; returns the executed classes with their operands and tags, and the destination planes as bits.
fn run(channels: usize) -> (Vec<(String, Vec<i32>, i32)>, Vec<Vec<u32>>) {
    let g = genotype();
    let mut b = Binding::defaults();
    b.input_elnum.insert("Source".into(), channels as i32);
    let (w, h) = (6usize, 3usize);
    let mut images = BTreeMap::new();
    images.insert("Source".to_string(), source(channels, w * h));
    let inputs = Inputs { width: w, height: h, images };
    let out = execute(&g, &b, &inputs, &HostFrames::default()).expect("execute");
    let nodes = out.trace.iter().map(|r| (r.class.clone(), r.operands.clone(), r.tag)).collect();
    let planes = out.dest.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect();
    (nodes, planes)
}

fn count(nodes: &[(String, Vec<i32>, i32)], class: &str) -> usize {
    nodes.iter().filter(|n| n.0 == class).count()
}

#[test]
fn a_three_element_source_compiles_without_either_node_and_renders_as_before() {
    let (nodes, planes) = run(3);
    assert_eq!(count(&nodes, "copy_alpha"), 0, "{:?}", nodes);
    assert_eq!(count(&nodes, "v3_to_v4_if4_a"), 0, "{:?}", nodes);
    assert_eq!(planes.len(), 3);
    // the identity nodes pass the three channels of the source through: the planes are the source's
    let src = source(3, 6 * 3);
    let want: Vec<Vec<u32>> = src.planes.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect();
    assert_eq!(planes, want, "the collapsed plan renders the channels of the source");
}

#[test]
fn a_four_element_source_keeps_both_nodes() {
    let (nodes, planes) = run(4);
    assert_eq!(count(&nodes, "copy_alpha"), 3, "{:?}", nodes);
    assert_eq!(count(&nodes, "v3_to_v4_if4_a"), 1, "{:?}", nodes);
    assert!(nodes.iter().filter(|n| n.0 == "copy_alpha").all(|n| n.1 == vec![1, 4] && n.2 == 2), "{:?}", nodes);
    assert!(nodes.iter().filter(|n| n.0 == "v3_to_v4_if4_a").all(|n| n.1 == vec![3, 1, 4] && n.2 == 4), "{:?}", nodes);
    assert_eq!(planes.len(), 4);
}

#[test]
fn the_collapse_removes_the_nodes_from_the_compiled_graph() {
    let g = genotype();
    let live = |channels: i32| -> usize {
        let mut b = Binding::defaults();
        b.input_elnum.insert("Source".into(), channels);
        let cp = compile_for(&g, &b).expect("compile");
        cp.nodes.iter().filter(|n| (n.prim == "copy_alpha" || n.prim == "v3_to_v4_if4_a") && !n.folded).count()
    };
    assert_eq!(live(3), 0);
    assert_eq!(live(4), 4);
}
