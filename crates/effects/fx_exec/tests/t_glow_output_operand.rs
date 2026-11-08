//! The destination operand (the special node Output) of alpha_from_rgb_max and coerce_up_a1 in Glow's program: a constant of
//! -999 in each destination channel, run as an operand; and the combination tables hold one row per class.
use fx_eval::{build_effect, Corpus};
use fx_exec::exec::{combo_table, compile_for, execute_compiled, Binding, HostFrames, Inputs};
use fx_exec::selector::Tables;
use fx_exec::types::Image;
use std::collections::{BTreeMap, HashSet};

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
}

fn synth(ch: usize, n: usize) -> Image {
    let mut s = 12345u64;
    let planes = (0..ch)
        .map(|_| {
            (0..n)
                .map(|_| {
                    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    0.05 + ((s >> 33) as u32 as f32 / 2147483648.0)
                })
                .collect()
        })
        .collect();
    Image::from_planes(planes).unwrap()
}

fn glow_run(src: i32, out: i32, show: Option<f32>) -> Result<(Vec<String>, usize), String> {
    glow_go(src, out, show, true)
}

fn glow_go(src: i32, out: i32, show: Option<f32>, exec: bool) -> Result<(Vec<String>, usize), String> {
    let c = Corpus::native();
    let g = build_effect(&c, c.find_effect("Glow", None).unwrap()).unwrap();
    let mut b = Binding::defaults();
    b.v3_bound_count = true;
    b.input_elnum.insert("Source".into(), src);
    b.input_elnum.insert("Output".into(), out);
    b.const_inputs.insert("Background".into(), vec![0.0]);
    b.const_inputs.insert("SparksMatte".into(), vec![0.0]);
    if let Some(v) = show {
        b.overrides.insert("show".into(), vec![v]);
    }
    let (w, h) = (16usize, 8usize);
    let mut images = BTreeMap::new();
    images.insert("Source".to_string(), synth(src as usize, w * h));
    let inp = Inputs { width: w, height: h, images };
    let cp = compile_for(&g, &b).map_err(|e| format!("{}", e))?;
    if !exec {
        return Ok((Vec::new(), 0));
    }
    let r = execute_compiled(&cp, &b, &inp, &HostFrames::default()).map_err(|e| format!("{}", e))?;
    Ok((r.trace.iter().map(|t| format!("{}:{}", t.class, t.operands.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(","))).collect(), r.dest.planes.len()))
}

#[test]
fn four_plane_destination_runs_alpha_from_rgb_max_with_a_four_lane_operand() {
    big(|| {
        let (tr, planes) = glow_run(4, 4, None).expect("run");
        assert!(tr.iter().any(|t| t == "alpha_from_rgb_max:4,1,4"), "{:?}", tr);
        assert_eq!(planes, 4);
    });
}

#[test]
fn three_plane_destination_compiles_without_the_count_refusal() {
    big(|| {
        let r = glow_go(3, 3, None, false);
        assert!(r.is_ok(), "{:?}", r.err());
    });
}

#[test]
fn show_two_runs_the_destination_operand_of_the_four_channel_coercion() {
    big(|| {
        let r = glow_run(4, 4, Some(1.0));
        assert!(r.is_ok(), "{:?}", r.err());
    });
}

#[test]
fn alpha_from_rgb_max_selects_for_three_and_four_channel_operands() {
    let t = combo_table();
    assert_eq!(t.select("alpha_from_rgb_max", &[4, 1, 4]).map(|x| x), Some(4));
    assert_eq!(t.select("alpha_from_rgb_max", &[3, 1, 3]).map(|x| x), Some(3));
    let e = Tables::embedded();
    assert_eq!(e.select_counts("alpha_from_rgb_max", &[4, 1, 4]), 4);
    assert_eq!(e.select_counts("alpha_from_rgb_max", &[3, 1, 3]), 3);
}

#[test]
fn each_class_has_one_combination_row() {
    let mut seen = HashSet::new();
    for c in effects_rtdata::classes::COMBOS {
        assert!(seen.insert(c.class), "duplicate COMBOS row {}", c.class);
    }
    let mut seen = HashSet::new();
    for c in effects_rtdata::classes::COMBOS_EXEC {
        assert!(seen.insert(c.class), "duplicate COMBOS_EXEC row {}", c.class);
    }
}
