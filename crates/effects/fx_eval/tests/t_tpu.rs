//! `unit <param> rec6c|x3|d4|d5 <hex>` lines and the host keys size_x and size_y feed the generic-path nibble-5
//! conversion of shift; a unit entry never makes a point context. Tests (i) to (iii) are the rule's cases; (iv) is t_pw_point and t_tpr unedited.
use fx_eval::hostcurve::{fold, HostCurveValues};
use tickmap::{unit_nibble5, UnitIn};

const STYLES: [&str; 2] = ["Transform_and_Blur_Autogen", "Blur_Motion_Curves_No_Transform_Autogen"];
const HOST: &str = "host effect_ref 1592590336\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\nhost global 1\nhost selector 0\n";

fn hex32(v: f32) -> String {
    format!("{:08x}", v.to_bits())
}
fn hex64(v: f64) -> String {
    format!("{:016x}", v.to_bits())
}

/// shift's checkout answers (16.16 raw) for lanes 0 and 1 at the canary keys, one unit entry, the two size keys.
fn text(raw0: i32, raw1: i32, sx: Option<i32>, sy: Option<i32>, unit: Option<(f32, f32, f64, f64)>, point: bool) -> String {
    let mut t = String::from(HOST);
    if let Some(v) = sx {
        t.push_str(&format!("host size_x {}\n", v));
    }
    if let Some(v) = sy {
        t.push_str(&format!("host size_y {}\n", v));
    }
    t.push_str(&format!("value shift 24 0 1 24 0 {}\nvalue shift 25 0 1 24 0 {}\n", raw0, raw1));
    if let Some((r6, x3, d4, d5)) = unit {
        t.push_str(&format!("unit shift rec6c {}\nunit shift x3 {}\nunit shift d4 {}\nunit shift d5 {}\n", hex32(r6), hex32(x3), hex64(d4), hex64(d5)));
    }
    if point {
        t.push_str("point shift list none\n");
    }
    t
}

fn run(style: &str, t: &str) -> (Option<Vec<f32>>, Vec<String>) {
    let mut h = HostCurveValues::parse(t).unwrap();
    h.time_frames = Some(0.0);
    h.ae32 = true;
    let mut d = Vec::new();
    let r = fold(style, "shift", 0.5, &[0.0, 0.0], 2, &h, &mut d);
    (r, d)
}

/// (i) MEASURED returns: 0.25 and -0.125 at size_x 64, every unit field 1.0, give 0x3c000000 and 0x3b800000; lanes 0 give 0 and -0.
#[test]
fn tpu_tmp1_measured_returns() {
    if !effects_rtdata::have_tables() {
        return;
    }
    for s in STYLES {
        let (r, d) = run(s, &text(16384, -8192, Some(64), Some(32), Some((1.0, 1.0, 1.0, 1.0)), false));
        let r = r.unwrap_or_else(|| panic!("{}: {:?}", s, d));
        assert_eq!([r[0].to_bits(), r[1].to_bits()], [0x3c000000, 0x3b800000], "{}", s);
        let (r, d) = run(s, &text(0, 0, Some(64), Some(32), Some((1.0, 1.0, 1.0, 1.0)), false));
        let r = r.unwrap_or_else(|| panic!("{}: {:?}", s, d));
        assert_eq!([r[0].to_bits(), r[1].to_bits()], [0x00000000, 0x80000000], "{}", s);
    }
}

/// (ii) Constructed pins (lens): each field moved off 1.0 alone, then all together, at size_x 50 and size_y 7.
#[test]
fn tpu_constructed_pins() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let cases: [(f32, f32, f64, f64); 5] = [(2.0, 1.0, 1.0, 1.0), (1.0, 0.5, 1.0, 1.0), (1.0, 1.0, 2.0, 1.0), (1.0, 1.0, 1.0, 0.5), (2.0, 0.5, 2.0, 0.5)];
    for (r6, x3, d4, d5) in cases {
        let u = UnitIn { x1: 50.0, rec_unit_value: r6, x3, d4, d5 };
        let want = [unit_nibble5(0.25, 0, &u).to_bits(), unit_nibble5(-0.125, 1, &u).to_bits()];
        let (r, d) = run(STYLES[0], &text(16384, -8192, Some(50), Some(7), Some((r6, x3, d4, d5)), false));
        let r = r.unwrap_or_else(|| panic!("{:?}", d));
        assert_eq!([r[0].to_bits(), r[1].to_bits()], want, "pins {:?}", (r6, x3, d4, d5));
    }
    // size_x and size_y differ, so a size_x taken from size_y is seen
    let u = UnitIn { x1: 50.0, rec_unit_value: 1.0, x3: 1.0, d4: 1.0, d5: 1.0 };
    let (r, _) = run(STYLES[1], &text(16384, -8192, Some(50), Some(7), Some((1.0, 1.0, 1.0, 1.0)), false));
    assert_eq!(r.unwrap()[0].to_bits(), unit_nibble5(0.25, 0, &u).to_bits());
}

/// (iii) The three refusals.
#[test]
fn tpu_refusals() {
    if !effects_rtdata::have_tables() {
        return;
    }
    for s in STYLES {
        let (r, d) = run(s, &text(16384, -8192, Some(64), Some(32), None, false));
        assert!(r.is_none());
        assert_eq!(d[0], format!("TICKMAP-HOST-FIELD-UNSOURCED (unit fields of {}: shift: rec_unit_value, unit_x3, unit_d4, unit_d5 are not recorded)", s));
        let (r, d) = run(s, &text(16384, -8192, None, Some(32), Some((1.0, 1.0, 1.0, 1.0)), false));
        assert!(r.is_none());
        assert_eq!(d[0], "TICKMAP-HOST-FIELD-UNSOURCED (size_x)");
        let (r, d) = run(s, &text(16384, -8192, Some(64), None, Some((1.0, 1.0, 1.0, 1.0)), false));
        assert!(r.is_none());
        assert_eq!(d[0], "TICKMAP-HOST-FIELD-UNSOURCED (size_y)");
        let (r, d) = run(s, &text(16384, -8192, Some(64), Some(32), Some((1.0, 1.0, 1.0, 1.0)), true));
        assert!(r.is_none());
        assert_eq!(d[0], format!("TICKMAP-UNIT-AND-POINT ({}: shift: a generic-path unit entry and a point entry)", s));
    }
}
