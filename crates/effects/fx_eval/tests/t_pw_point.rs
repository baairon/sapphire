//! Step PW: the recorded point fields reach map_param's PointCtx through the HOST-CURVE-VALUES text (PT records).
use fx_eval::hostcurve::{HostCurveValues, POINT_FIELDS, POINT_UNIT_FIELDS};

fn text(sx: i32, sy: i32, px: i32, py: i32, list: &str, drop: Option<&str>) -> String {
    let mut v: Vec<(String, String)> = vec![
        ("list", list.to_string()), ("gid", "1".into()), ("selector", "0".into()), ("rtype", "1".into()), ("size_x", sx.to_string()), ("size_y", sy.to_string()),
        ("s11c", "1".into()), ("s120", "1".into()), ("s124", "1".into()), ("s128", "1".into()), ("off_a", "0".into()), ("off_b", "0".into()), ("scaled", "1".into()),
        ("cur_time", "0".into()), ("time_step", "1".into()), ("time_scale", "24".into()), ("px", px.to_string()), ("py", py.to_string()),
        ("d4", "3ff0000000000000".into()), ("d5", "3ff0000000000000".into()), ("rec6c", "3f800000".into()), ("x3", "3f800000".into()),
        ("prelude_status", "0".into()), ("prelude_type", "6".into()),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b))
    .collect();
    if let Some(d) = drop {
        v.retain(|(k, _)| k != d);
    }
    let mut t = String::from("host effect_ref 1592262656\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\n");
    for (k, x) in v {
        t.push_str(&format!("point center {} {}\n", k, x));
    }
    t
}

fn run(t: &str) -> (Option<Vec<f32>>, Vec<String>) {
    let mut h = HostCurveValues::parse(t).unwrap();
    h.time_frames = Some(0.0);
    h.ae32 = true;
    let mut d = Vec::new();
    let r = fx_eval::hostcurve::fold("Transform_and_Blur_Autogen", "center", 0.5, &[0.0, 0.0], 2, &h, &mut d);
    (r, d)
}

fn bits(r: Option<Vec<f32>>) -> (u32, u32) {
    let r = r.expect("fold refused");
    (r[0].to_bits(), r[1].to_bits())
}

#[test]
fn recorded_entries_reproduce_through_the_generic_path() {
    if !effects_rtdata::have_tables() {
        return;
    }
    assert_eq!(bits(run(&text(64, 32, 3276800, 3276800, "none", None)).0), (0x3f100000, 0xbf880000));
    assert_eq!(bits(run(&text(50, 7, 3276800, 3276800, "none", None)).0), (0x3f800000, 0xbfee147b));
    assert_eq!(bits(run(&text(64, 32, 819200, -475136, "none", None)).0), (0xbf1c0000, 0x3f3a0000));
    // the same record read through a host list entry of type 6
    assert_eq!(bits(run(&text(64, 32, 3276800, 3276800, "6", None)).0), (0x3f100000, 0xbf880000));
}

#[test]
fn every_missing_point_field_refuses() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let all: Vec<&str> = POINT_FIELDS.iter().chain(POINT_UNIT_FIELDS.iter()).copied().chain(["prelude_status", "prelude_type"]).collect();
    for f in all {
        let (r, d) = run(&text(64, 32, 3276800, 3276800, "none", Some(f)));
        assert!(r.is_none(), "{} dropped but not refused", f);
        assert!(d.iter().any(|x| x.contains("TICKMAP-HOST-FIELD-UNSOURCED") && x.contains(&format!("point field {} ", f))), "{}: {:?}", f, d);
    }
}

#[test]
fn no_point_lines_keeps_the_point_refusal_and_bad_fields_refuse() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let base = "host effect_ref 1\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\n";
    let (r, d) = run(base);
    assert!(r.is_none() && d.iter().any(|x| x.contains("TICKMAP-ROUTE-UNPORTED") || x.contains("HOST-CURVE-VALUE-MISSING") || x.contains("TICKMAP")), "{:?}", d);
    // a gid differing from the run's global, a record type differing from the parameter record, a non-zero prelude status
    let t = text(64, 32, 3276800, 3276800, "none", None).replace("point center gid 1", "point center gid 9");
    assert!(run(&t).0.is_none());
    let t = text(64, 32, 3276800, 3276800, "none", None).replace("point center rtype 1", "point center rtype 3");
    assert!(run(&t).0.is_none());
    let t = text(64, 32, 3276800, 3276800, "none", None).replace("point center prelude_status 0", "point center prelude_status 517");
    let (r, d) = run(&t);
    assert!(r.is_none() && d.iter().any(|x| x.contains("TICKMAP-POINT-PRELUDE")), "{:?}", d);
    let t = text(64, 32, 3276800, 3276800, "none", None).replace("point center selector 0", "point center selector 1");
    assert!(run(&t).0.is_none());
}
