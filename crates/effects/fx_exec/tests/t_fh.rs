//! Step FH: a host frame is converted from the harness's ae32 words by the head of its input, like that input's Source frame.
//! The conversion routine is the one the executor uses for the Source input (ae32_to_image); the CLI test checks the raw path refusal.
use fx_exec::hostconv14::{ae32_to_image, Ae32Source, R3_HEAD, VB3_HEAD};

fn words(w: usize, h: usize) -> Vec<u32> {
    let alpha = [0.0f32, 0.3, 0.7, 1.0, 1.5, 2.5, -0.5];
    let mut v = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let a = alpha[((x / 4) + y) % 7];
            for c in [a, 0.03 + 0.94 * (x as f32) / 15.0, 0.03 + 0.94 * (y as f32) / 7.0, 0.5 + 0.01 * x as f32] {
                v.push(c.to_bits());
            }
        }
    }
    v
}

#[test]
fn fh_r3_frame_is_colour_times_alpha() {
    let (w, h) = (16, 8);
    let wd = words(w, h);
    let im = ae32_to_image(&Ae32Source { words: wd.clone(), reader_head: R3_HEAD, planes: 3 }, w, h).unwrap();
    let mut differ = 0;
    for i in 0..w * h {
        let a = f32::from_bits(wd[4 * i]);
        for c in 0..3 {
            let col = f32::from_bits(wd[4 * i + 1 + c]);
            assert_eq!(im.planes[c][i].to_bits(), (col * a).to_bits(), "pixel {} plane {}", i, c);
            if a != 1.0 && col != 0.0 { differ += 1; }
        }
    }
    assert!(differ > 0, "the case must discriminate raw from premultiplied");
}

#[test]
fn fh_variant_b_frame_is_unchanged_on_alx() {
    let (w, h) = (16, 8);
    let wd = words(w, h);
    let im = ae32_to_image(&Ae32Source { words: wd.clone(), reader_head: VB3_HEAD, planes: 3 }, w, h).unwrap();
    for i in 0..w * h {
        for c in 0..3 {
            assert_eq!(im.planes[c][i].to_bits(), wd[4 * i + 1 + c], "pixel {} plane {}", i, c);
        }
    }
}
