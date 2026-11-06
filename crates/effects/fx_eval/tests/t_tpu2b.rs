//! Step (CURVE-PARAM-DIRECT-READ): a host curve parameter's value at the current time, through hostcurve::fold at t = 0,
//! equals the original's parameter record lane by f32 bits for every bound value in the recorded records (FEED_RULES.tsv, 30 of 30 records,
//! params_after_loop): rotate lane = -host Rotate, z_dist lane = host Z Dist. The Blur_Only centre row is present.
use fx_eval::hostcurve::{fold, table_row, HostCurveValues};
static RECORDS: effects_rtdata::Table = effects_rtdata::Table::new("data/TICKMAP_PARAM_RECORDS.tsv");


const HOST: &str = "host effect_ref 1592590336\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\nhost global 1\nhost selector 0\n";

fn at_now(style: &str, p: &str, idx: i32, host: f64) -> u32 {
    let raw = (host * 65536.0).round() as i64;
    let mut h = HostCurveValues::parse(&format!("{}value {} {} 0 1 24 0 {}\n", HOST, p, idx, raw)).unwrap();
    h.time_frames = Some(0.0);
    h.ae32 = true;
    let mut d = Vec::new();
    fold(style, p, 0.0, &[0.0], 1, &h, &mut d).unwrap_or_else(|| panic!("{} {}: {:?}", style, p, d))[0].to_bits()
}

#[test]
fn tpu2b_rotate_lane_is_negated_host_by_bits() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // (host Rotate, record lane) pairs from the records: rot_tb 30, c_TB_R1 17.5, TB_K1/BO_K1 12.5, TB_R2 -23.75, TB_RC2 9.375, TB_RL2 31.25, default 0
    for (hostv, lane) in [(30.0, -30.0f32), (17.5, -17.5), (12.5, -12.5), (-23.75, 23.75), (9.375, -9.375), (31.25, -31.25), (0.0, -0.0)] {
        for s in ["Transform_and_Blur_Autogen", "Blur_Motion_Curves_No_Transform_Autogen"] {
            assert_eq!(at_now(s, "rotate", 23, hostv), lane.to_bits(), "{} rotate {}", s, hostv);
        }
    }
}

#[test]
fn tpu2b_z_dist_lane_is_host_by_bits() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // c_TB_Z1 1.375, TB_Z2/TB_Z5 0.8125, default 1
    for (hostv, lane) in [(1.375, 1.375f32), (0.8125, 0.8125), (1.0, 1.0)] {
        for s in ["Transform_and_Blur_Autogen", "Blur_Motion_Curves_No_Transform_Autogen"] {
            assert_eq!(at_now(s, "z_dist", 22, hostv), lane.to_bits(), "{} z_dist {}", s, hostv);
        }
    }
}

#[test]
fn tpu2b_blur_only_centre_row() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let r = table_row(&RECORDS, "Blur_Motion_Curves_No_Transform_Autogen", "center").expect("row");
    assert_eq!(&r[2..], &["1", "2", "2", "4", "0", "0", "21"]);
}
