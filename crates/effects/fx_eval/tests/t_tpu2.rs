//! Step TPU2 (W708 (e) 1, 3): the host point parameter 'center' is a host input; its lanes come from the run's recorded point lines through
//! the ported point route. These tests check the native centre against the original's recorded point-arm results (BMR1 canary re-records,
//! bmr1.pt.res r0 and r1) by bits.
use fx_eval::hostcurve::{fold, HostCurveValues};

fn lines(sx: i32, sy: i32) -> String {
    let mut t = String::from("host effect_ref 1592590336\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\nhost global 1\nhost selector 0\n");
    let f = [
        ("list", "none".to_string()), ("gid", "1".into()), ("selector", "0".into()), ("rtype", "1".into()), ("size_x", sx.to_string()), ("size_y", sy.to_string()),
        ("s11c", "1".into()), ("s120", "1".into()), ("s124", "1".into()), ("s128", "1".into()), ("off_a", "0".into()), ("off_b", "0".into()), ("scaled", "1".into()),
        ("cur_time", "0".into()), ("time_step", "1".into()), ("time_scale", "24".into()), ("px", "3276800".into()), ("py", "3276800".into()),
        ("d4", "3ff0000000000000".into()), ("d5", "3ff0000000000000".into()), ("rec6c", "3f800000".into()), ("x3", "3f800000".into()),
        ("prelude_status", "0".into()), ("prelude_type", "6".into()),
    ];
    for (k, v) in f {
        t.push_str(&format!("point center {} {}\n", k, v));
    }
    t
}

fn centre(sx: i32, sy: i32) -> [u32; 2] {
    let mut h = HostCurveValues::parse(&lines(sx, sy)).unwrap();
    h.time_frames = Some(0.0);
    h.ae32 = true;
    let mut d = Vec::new();
    let r = fold("Transform_and_Blur_Autogen", "center", 0.0, &[0.0, 0.0], 2, &h, &mut d).unwrap_or_else(|| panic!("{:?}", d));
    [r[0].to_bits(), r[1].to_bits()]
}

/// can_tb_64x32r: the default Center XY (50, 50) px gives (0.5625, -1.0625), 0x3f100000 and 0xbf880000, in every point-arm result of the record.
#[test]
fn tpu2_centre_64x32_equals_record() {
    if !effects_rtdata::have_tables() {
        return;
    }
    assert_eq!(centre(64, 32), [0x3f100000, 0xbf880000]);
}

/// can_tb_50x7r: the same default at 50x7 gives 0x3f800000 and 0xbfee147b in every point-arm result of the record.
#[test]
fn tpu2_centre_50x7_equals_record() {
    if !effects_rtdata::have_tables() {
        return;
    }
    assert_eq!(centre(50, 7), [0x3f800000, 0xbfee147b]);
}
