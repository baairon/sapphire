//! INT19 S1: WPE3's per-row head of the original `variable 2` xy x writer, scoped by XY-HEAD-MODE (W444 (b), W452 (c)).
//! The first four tests are WPE3's t26_wpe3_xy.rs (79 original x rows read from the emulator at 0xda745, run e_f1) with xy_ramp's PerRow
//! form called by name; the rest test the OneHead form and the mode derivation from the plan.
use fx_eval::{build_effect, Corpus};
use fx_exec::kernels2::xy_head_mode_of;
use fx_exec::types::{row_head, x_ramp, xy_ramp, xy_ramp_mode, XyHeadMode};

fn rows_text() -> &'static str {
    Box::leak(std::fs::read_to_string(effects_vecdir::path("fx_exec/data/wpe3_xy_rows.tsv")).unwrap_or_default().into_boxed_str())
}
fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn hex32(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap()
}

struct Row {
    w: usize,
    row: usize,
    mod32: usize,
    cnt: usize,
    s: u32,
    d: u32,
    x: Vec<u32>,
}

fn rows() -> Vec<Row> {
    rows_text().lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let x: Vec<u32> = (0..f[6].len() / 8).map(|i| hex32(&f[6][8 * i..8 * i + 8])).collect();
            Row { w: f[0].parse().unwrap(), row: f[1].parse().unwrap(), mod32: f[2].parse().unwrap(), cnt: f[3].parse().unwrap(), s: hex32(f[4]), d: hex32(f[5]), x }
        })
        .collect()
}

#[test]
fn x_ramp_with_the_row_pointer_head_equals_the_original_row_for_all_79_rows() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data/wpe3_xy_rows.tsv"]) {
        return;
    }
    let rs = rows();
    assert_eq!(rs.len(), 79);
    for r in &rs {
        let (s, d) = (-1.0f32 + 1.0f32 / r.w as f32, 2.0f32 / r.w as f32);
        assert_eq!(s.to_bits(), r.s, "s at width {}", r.w);
        assert_eq!(d.to_bits(), r.d, "d at width {}", r.w);
        let got: Vec<u32> = x_ramp(r.cnt, row_head(r.mod32), s, d).iter().map(|v| v.to_bits()).collect();
        assert_eq!(got, r.x, "width {} row {} (ptr mod 32 = {})", r.w, r.row, r.mod32);
    }
}

#[test]
fn xy_ramp_per_row_rows_equal_the_original_for_base_phase_zero() {
    if !effects_vecdir::have_all(&["fx_exec/data/wpe3_xy_rows.tsv"]) {
        return;
    }
    let rs = rows();
    let mut seen = std::collections::BTreeMap::new();
    for r in &rs {
        seen.entry(r.w).or_insert_with(Vec::new).push(r.row);
    }
    for (w, rws) in seen {
        let h = rws.len();
        let im = xy_ramp_mode(w, h, 0, XyHeadMode::PerRow);
        for r in rs.iter().filter(|r| r.w == w) {
            let got: Vec<u32> = im.planes[0][r.row * w..(r.row + 1) * w].iter().map(|v| v.to_bits()).collect();
            assert_eq!(got, r.x, "xy_ramp PerRow width {} row {}", w, r.row);
        }
    }
}

#[test]
fn negative_control_one_head_for_every_row_is_wrong_at_33_and_37() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data/wpe3_xy_rows.tsv"]) {
        return;
    }
    let rs = rows();
    for w in [33usize, 37, 31, 41] {
        let (s, d) = (-1.0f32 + 1.0f32 / w as f32, 2.0f32 / w as f32);
        let bad = rs.iter().filter(|r| r.w == w).filter(|r| x_ramp(r.cnt, 0, s, d).iter().map(|v| v.to_bits()).collect::<Vec<_>>() != r.x).count();
        assert!(bad > 0, "width {}: head 0 on every row must differ from the original on some row", w);
    }
}

#[test]
fn row_head_rule() {
    assert_eq!(row_head(0), 0);
    assert_eq!(row_head(4), 7);
    assert_eq!(row_head(16), 4);
    assert_eq!(row_head(28), 1);
    assert_eq!(row_head(32), 0);
    assert_eq!(row_head(132), 7);
}

#[test]
fn one_head_repeats_row_zero_and_is_the_default_xy_ramp() {
    for (w, h, head) in [(50usize, 7usize, 0usize), (49, 9, 4), (63, 33, 0), (64, 32, 0), (33, 5, 7)] {
        let a = xy_ramp(w, h, head);
        let b = xy_ramp_mode(w, h, head, XyHeadMode::OneHead);
        assert_eq!(a.planes, b.planes);
        for y in 1..h {
            assert_eq!(a.planes[0][y * w..(y + 1) * w], a.planes[0][..w], "OneHead row {} at {}x{}", y, w, h);
        }
        // both modes agree on row 0 and on the y plane
        let p = xy_ramp_mode(w, h, head, XyHeadMode::PerRow);
        assert_eq!(p.planes[1], a.planes[1]);
        assert_eq!(p.planes[0][..w], a.planes[0][..w]);
    }
    // W * 4 a multiple of 32: the two modes are the same plane
    assert_eq!(xy_ramp_mode(64, 32, 0, XyHeadMode::PerRow).planes, xy_ramp(64, 32, 0).planes);
    // W * 4 not a multiple of 32: they differ on some row
    assert_ne!(xy_ramp_mode(50, 7, 0, XyHeadMode::PerRow).planes[0], xy_ramp(50, 7, 0).planes[0]);
}

#[test]
fn head_mode_derived_from_the_plan_matches_the_ruled_scope() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_exec/data/wpe3_xy_rows.tsv"]) {
        return;
    }
    let t = std::fs::read_to_string(pt()).unwrap();
    let c = Corpus::from_programs_tsv(&t);
    let mode = |name: &str| {
        let ei = c.find_effect(name, None).unwrap_or_else(|| panic!("effect {} not found", name));
        xy_head_mode_of(&build_effect(&c, ei).unwrap())
    };
    for p in ["Warp_Perspective", "Warp_Perspective_Autogen"] {
        assert_eq!(mode(p), XyHeadMode::PerRow, "{}", p);
    }
    for p in ["Transform_and_Blur", "Transform_and_Blur_Autogen", "Blur_Motion_Curves_No_Transform", "Blur_Motion_Curves_No_Transform_Autogen"] {
        assert_eq!(mode(p), XyHeadMode::OneHead, "{}", p);
    }
}
