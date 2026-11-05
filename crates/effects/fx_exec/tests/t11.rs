//! NV-10 tests: the original `variable 2` xy ramp (x_ramp / xy_ramp) against values read from the emulator hook (audit_ep10 E1 run, 50x7).
use fx_exec::types::{x_ramp, xy_pixel_centre, xy_ramp};

fn hexf(s: &str) -> Vec<f32> {
    let b: Vec<u8> = (0..s.trim().len() / 2).map(|i| u8::from_str_radix(&s.trim()[2 * i..2 * i + 2], 16).unwrap()).collect();
    b.chunks(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

/// The values read from the original run (stored effect data, VEC_DIR); empty when absent.
fn stored(rel: &str) -> &'static str {
    Box::leak(std::fs::read_to_string(effects_vecdir::path(rel)).unwrap_or_default().into_boxed_str())
}
fn t_head4() -> &'static str { stored("data/xv_T01_50x7_head4.hex") }
fn b_head0() -> &'static str { stored("data/xv_B01_50x7_head0.hex") }
fn yv() -> &'static str { stored("data/yv_T01_50x7_first.txt") }

fn sd(w: usize) -> (f32, f32) {
    (-1.0f32 + 1.0f32 / w as f32, 2.0f32 / w as f32)
}

#[test]
fn x_ramp_matches_original_head4_and_head0() {
    if !effects_vecdir::have_all(&["data/xv_T01_50x7_head4.hex", "data/xv_B01_50x7_head0.hex"]) {
        return;
    }
    let (s, d) = sd(50);
    assert_eq!(bits(&x_ramp(50, 4, s, d)), bits(&hexf(t_head4())), "Transform_and_Blur T01 x plane (dest 16 byte offset, head 4)");
    assert_eq!(bits(&x_ramp(50, 0, s, d)), bits(&hexf(b_head0())), "Blur_Motion_Curves_No_Transform B01 x plane (aligned, head 0)");
}

#[test]
fn x_ramp_negative_control_wrong_head_differs() {
    if !effects_vecdir::have_all(&["data/xv_T01_50x7_head4.hex", "data/xv_B01_50x7_head0.hex"]) {
        return;
    }
    let (s, d) = sd(50);
    assert_ne!(bits(&x_ramp(50, 0, s, d)), bits(&hexf(t_head4())));
    assert_ne!(bits(&x_ramp(50, 4, s, d)), bits(&hexf(b_head0())));
}

#[test]
fn y_rows_match_original() {
    if !effects_vecdir::have_all(&["data/yv_T01_50x7_first.txt"]) {
        return;
    }
    let im = xy_ramp(50, 7, 4);
    for l in yv().lines() {
        let mut it = l.split_whitespace();
        let r: usize = it.next().unwrap().parse().unwrap();
        let want = hexf(it.next().unwrap())[0];
        assert_eq!(im.planes[1][r * 50].to_bits(), want.to_bits(), "row {}", r);
        assert_eq!(im.planes[1][r * 50 + 49].to_bits(), want.to_bits());
    }
}

#[test]
fn power_of_two_width_is_the_closed_form() {
    for h in [32usize, 8] {
        let a = xy_ramp(64, h, 4);
        let b = xy_pixel_centre(64, h);
        assert_eq!(bits(&a.planes[0]), bits(&b.planes[0]));
        assert_eq!(bits(&a.planes[1]), bits(&b.planes[1]));
        assert_eq!(bits(&xy_ramp(64, h, 0).planes[0]), bits(&b.planes[0]));
    }
}
