//! DX2: the v3_to_v4_if4 count is the element count of the bound image argument (argument 2, here Output).
use fx_exec::exec::combo_table;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn ds(output_el: i32) -> Vec<(usize, usize, usize, Vec<u32>, [u32; 4], &'static str, Vec<u32>)> {
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = fx_eval::Corpus::from_programs_tsv(&tsv);
    let g = fx_eval::build_effect(&c, c.find_effect("Drop_Shadow", None).unwrap()).unwrap();
    let mut o = fx_eval::CompileOptions::default();
    o.input_elnum.insert("Foreground".into(), 4);
    o.input_elnum.insert("Output".into(), output_el);
    o.const_inputs.insert("Background".into(), vec![0.0]);
    let _ = fx_eval::fold::v3_take();
    let r = fx_eval::compile(&g, &combo_table(), &o);
    let v = fx_eval::fold::v3_take();
    if v.is_empty() {
        panic!("no v3 fold record: {:?}", r.err().map(|e| e.msg));
    }
    v
}

#[test]
fn drop_shadow_node_folds_to_the_four_lane_vector_at_four_planes() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let v = ds(4);
    let x = &v[0];
    assert_eq!(x.1, 4, "n");
    assert_eq!(x.4.map(f32::from_bits), [0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn drop_shadow_node_at_three_planes_keeps_the_three_lane_form() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let v = ds(3);
    assert_eq!(v[0].1, 3);
    assert_eq!(v[0].4[3], 0);
}
