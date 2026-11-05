//! Step TPR (W644 (b), W645 (a)): the six claim-style rows of data/TICKMAP_PARAM_RECORDS.tsv carry the claim harness's parameter
//! records (BMR1 PATCH.md 1, MEASURED at every 0x71c720 entry of BMR1's records); core-program and center rows are unchanged.
use fx_eval::hostcurve::{fold, table_row, HostCurveValues, RECORDS};

const STYLES: [&str; 2] = ["Transform_and_Blur_Autogen", "Blur_Motion_Curves_No_Transform_Autogen"];
// (param, ptype, nlanes, kind_byte, subtype_byte, flags_a_byte, flags_b_byte, index)
const ROWS: [(&str, [&str; 7]); 3] = [
    ("rotate", ["1", "1", "1", "0", "0", "6", "23"]),
    ("shift", ["1", "2", "2", "5", "0", "0", "24"]),
    ("z_dist", ["1", "1", "1", "0", "0", "0", "22"]),
];

fn row_check(style: &str, p: usize) {
    let (name, want) = ROWS[p];
    let r = table_row(&RECORDS, style, name).expect("row present");
    assert_eq!(&r[2..], &want[..], "{} {}", style, name);
}

#[test] fn tpr_row_tb_rotate() {
    if !effects_rtdata::have_tables() {
        return;
    } row_check(STYLES[0], 0) }
#[test] fn tpr_row_tb_shift() {
    if !effects_rtdata::have_tables() {
        return;
    } row_check(STYLES[0], 1) }
#[test] fn tpr_row_tb_z_dist() {
    if !effects_rtdata::have_tables() {
        return;
    } row_check(STYLES[0], 2) }
#[test] fn tpr_row_bo_rotate() {
    if !effects_rtdata::have_tables() {
        return;
    } row_check(STYLES[1], 0) }
#[test] fn tpr_row_bo_shift() {
    if !effects_rtdata::have_tables() {
        return;
    } row_check(STYLES[1], 1) }
#[test] fn tpr_row_bo_z_dist() {
    if !effects_rtdata::have_tables() {
        return;
    } row_check(STYLES[1], 2) }

/// W645 (a) 2: rotate 23 and z_dist 22 pass the index check in both styles (they reach the next refusal, time_frames here).
#[test]
fn tpr_index_check_passes() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let h = HostCurveValues::default();
    for s in STYLES {
        for p in ["rotate", "z_dist"] {
            let mut d = Vec::new();
            assert!(fold(s, p, 0.0, &[0.0], 1, &h, &mut d).is_none());
            assert!(!d[0].starts_with("TICKMAP-INDEX-NOT-REGISTERED"), "{} {}: {:?}", s, p, d);
            assert!(d[0].starts_with("TICKMAP-HOST-FIELD-UNSOURCED"), "{} {}: {:?}", s, p, d);
        }
    }
}

/// The host lines as BMR1's canary records show them (in_data +0xb8, +0xec, +0xf0, +0xf4; the global 1; selector 0).
const HOST: &str = "host effect_ref 1592590336\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\nhost global 1\nhost selector 0\n";

/// W645 (a) 4: shift (subtype_byte 5) refuses without point fields, and with a recorded point entry reaches a result on the generic path.
/// The point entry is BMR1's can_tb_64x32 record (bmr1.pt.arm[0], bmr1.pt.xy[0], prelude status 0 type 6); the checkout answers
/// are that record's shift keys (shift, 24 and 25, tick 0, step 1, scale 24, status 0, answer 0).
#[test]
fn tpr_shift_point_fields() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let mut h = HostCurveValues::parse(&format!("{}value shift 24 0 1 24 0 0\nvalue shift 25 0 1 24 0 0\n", HOST)).unwrap();
    h.time_frames = Some(0.0);
    h.ae32 = true;
    for s in STYLES {
        let mut d = Vec::new();
        assert!(fold(s, "shift", 0.5, &[0.0, 0.0], 2, &h, &mut d).is_none(), "{}", s);
        assert!(d[0].starts_with("TICKMAP-HOST-FIELD-UNSOURCED (unit fields of"), "{}: {:?}", s, d);
    }
    let pt = "list none\ngid 1\nselector 0\nrtype 1\nsize_x 64\nsize_y 32\ns11c 1\ns120 1\ns124 1\ns128 1\noff_a 0\noff_b 0\nscaled 1\ncur_time 0\ntime_step 1\ntime_scale 24\npx 3276800\npy 3276800\nd4 3ff0000000000000\nd5 3ff0000000000000\nrec6c 3f800000\nx3 3f800000\nprelude_status 0\nprelude_type 6\n";
    let mut t = String::from(HOST);
    t.push_str("value shift 24 0 1 24 0 0\nvalue shift 25 0 1 24 0 0\n");
    for l in pt.lines() {
        t.push_str(&format!("point shift {}\n", l));
    }
    let mut h = HostCurveValues::parse(&t).unwrap();
    h.time_frames = Some(0.0);
    h.ae32 = true;
    for s in STYLES {
        let mut d = Vec::new();
        let r = fold(s, "shift", 0.5, &[0.0, 0.0], 2, &h, &mut d);
        assert!(r.is_some(), "{}: {:?}", s, d);
    }
}

/// W645 (a) 5: the core-program rows and the center row are byte identical to PRE's table (int32_merge/stack, copied as tpr/TICKMAP_PARAM_RECORDS.pre.tsv).
#[test]
fn tpr_other_rows_unchanged() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let pre = "Blur_Motion_Curves_No_Transform\trotate\t1\t1\t1\t0\t0\t6\t0\nBlur_Motion_Curves_No_Transform\tshift\t1\t2\t2\t0\t0\t0\t0\nBlur_Motion_Curves_No_Transform\tz_dist\t1\t1\t1\t0\t0\t0\t0\nTransform_and_Blur\trotate\t1\t1\t1\t0\t0\t6\t0\nTransform_and_Blur\tshift\t1\t2\t2\t0\t0\t0\t0\nTransform_and_Blur\tz_dist\t1\t1\t1\t0\t0\t0\t0\nTransform_and_Blur_Autogen\tcenter\t1\t2\t2\t4\t0\t0\t21";
    for l in pre.lines() {
        assert!(RECORDS.lines().any(|x| x == l), "row changed or missing: {}", l);
    }
    // step TPU2b (W746 (a)) adds the Blur_Only centre row: 14 lines become 15 (TEST CHANGE, listed in PATCH.md)
    assert_eq!(RECORDS.lines().count(), 15);
}
