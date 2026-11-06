//! M3P2: the 'ae32' destination at mode 3 with a 1-plane destination runs the code-14 one-plane zero-arm body
//! (words 1 to 3 = plane 0 bit for bit, word 0 = 1.0, no row copier). The body's T2 is; this checks the wiring and the mutants.
use fx_exec::hostconv14::*;
use fx_exec::Image;

fn route(planes: usize, mode: i32, prior: Vec<u32>) -> Ae32Route {
    Ae32Route { sources: vec![], dest_stack_word: 0, dest: Some(Ae32Dest { planes, mode, prior, copier_source: None }) }
}

#[test]
fn m3p2_one_plane_mode3_words() {
    let g = [0.25f32, -0.0, f32::NAN, f32::INFINITY, 1.5, -7.0, 1e-40, 0.5];
    let (w, h) = (4, 2);
    let im = Image::from_planes(vec![g.to_vec()]).unwrap();
    let prior = vec![0xaaaa_aaaau32; 4 * w * h];
    let out = ae32_dest(&route(1, 3, prior), &im, w, h).unwrap();
    for i in 0..w * h {
        assert_eq!(out[4 * i], 1.0f32.to_bits(), "word 0 pixel {}", i);
        for k in 1..4 {
            assert_eq!(out[4 * i + k], g[i].to_bits(), "word {} pixel {}", k, i);
        }
    }
}

#[test]
fn m3p2_one_plane_mode2_leaves_word0() {
    let (w, h) = (2, 1);
    let im = Image::from_planes(vec![vec![0.5f32, 0.75]]).unwrap();
    let out = ae32_dest(&route(1, 2, vec![0x1234u32; 8]), &im, w, h).unwrap();
    assert_eq!(out[0], 0x1234);
    assert_eq!(out[1], 0.5f32.to_bits());
}

#[test]
fn m3p2_two_planes_mode3_still_refused_by_the_body() {
    let (w, h) = (2, 1);
    let im = Image::from_planes(vec![vec![0.5f32, 0.75], vec![0.1, 0.2]]).unwrap();
    let e = ae32_dest(&route(2, 3, vec![0u32; 8]), &im, w, h).unwrap_err();
    assert!(format!("{:?}", e).contains("DEST-MODE-LOGGED"), "{:?}", e);
}
