//! INT29 step TW (W559 (a)): FRAME-INDEX-REQUIRED at compile on the claim route, and the get_frame request keyed by (frame, stored)
//! through the executor on Time_Warp_RGB_Autogen at F = 2.5, step 2 (TWS1 F25's requests: frames 3, 2, 1, arguments 7, 5, 3).
use fx_exec::exec::{compile_for, execute_compiled, Binding, HostFrames, Inputs};
use fx_exec::twframe::TwRoute;
use fx_exec::Image;
use std::collections::BTreeMap;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn tw() -> fx_eval::Genotype {
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = fx_eval::Corpus::from_programs_tsv(&tsv);
    fx_eval::build_effect(&c, c.find_effect("Time_Warp_RGB_Autogen", None).unwrap()).unwrap()
}

fn bind() -> Binding {
    let mut b = Binding::defaults();
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    b
}

fn img(n: usize, seed: f32) -> Image {
    Image { n, planes: (0..4).map(|c| (0..n).map(|i| seed + 0.01 * (c * n + i) as f32).collect()).collect() }
}

#[test]
fn frame_index_required_on_claim_route() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = tw();
    assert!(g.nodes.iter().any(|x| x.name == "get_frame_number"));
    let mut b = bind();
    b.frame_index_required = true;
    let e = compile_for(&g, &b).err().expect("refused without F");
    assert!(format!("{}", e).contains("FRAME-INDEX-REQUIRED"), "{}", e);
    b.host.insert("get_frame_number".into(), vec![2.5]);
    assert!(compile_for(&g, &b).is_ok());
    // state A (not the claim route) keeps NV3-A7's zeros
    assert!(compile_for(&g, &bind()).is_ok());
}

fn run(by: &[((i32, f32), f32)]) -> Result<fx_exec::exec::ExecOutput, fx_exec::types::ExecError> {
    let g = tw();
    let mut b = bind();
    b.frame_index_required = true;
    b.host.insert("get_frame_number".into(), vec![2.5]);
    let cp = compile_for(&g, &b).unwrap();
    let (w, h) = (8usize, 4usize);
    let n = w * h;
    let mut images = BTreeMap::new();
    images.insert("Source".to_string(), img(n, 0.5));
    let inp = Inputs { width: w, height: h, images };
    let mut host = HostFrames::default();
    host.tw_route = Some(TwRoute { big_f: 2.5, dur_frames: 240.0, limit_cell: 0, step: 2, kind: 0, f4: 0, r8: 0, r9: 1 });
    for &((fr, st), seed) in by {
        host.by_request.insert((fr, st.to_bits()), img(n, seed));
    }
    execute_compiled(&cp, &b, &inp, &host)
}

/// The request key discriminates: frames answered at (frame, stored) = (3, 3.5), (2, 2.5), (1, 1.5) run; the same frames keyed by the
/// integer frame alone ((3, 3.0), ...) are not the plugin's request and the run refuses MissingHostFrame.
#[test]
fn request_key_frame_stored() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let ok = run(&[((3, 3.5), 0.1), ((2, 2.5), 0.2), ((1, 1.5), 0.3)]);
    assert!(ok.is_ok(), "{:?}", ok.err());
    let bad = run(&[((3, 3.0), 0.1), ((2, 2.0), 0.2), ((1, 1.0), 0.3)]);
    let e = format!("{}", bad.err().expect("integer keys refused"));
    assert!(e.contains("missing host frame") && e.contains("stored 3.5"), "{}", e);
}
