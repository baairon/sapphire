//! Host getter folds against recorded getter results. HOST_GETTER_CASES names a TSV with the columns
//! run, curtime, total_in, phase, getter, value_bits (hex f32), value, fields_read; without it the test prints NOT RUN.
//! The fold of get_percent_time, get_total_frames and get_aspect_ratio takes the explicit host value, so the lane must carry exactly the
//! recorded bits; the recorded percent time must also equal frame / (total - 1) in f32.
use fx_eval::fold::fold_lanes;

#[test]
fn fold_carries_recorded_bits() {
    let Ok(p) = std::env::var("HOST_GETTER_CASES") else {
        println!("NOT RUN (HOST_GETTER_CASES unset)");
        return;
    };
    let mut n = 0;
    for l in std::fs::read_to_string(p).unwrap().lines().skip(1) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 6 || !matches!(f[4], "get_percent_time" | "get_total_frames" | "get_aspect_ratio") {
            continue;
        }
        let bits = u32::from_str_radix(f[5], 16).unwrap();
        let r = fold_lanes(f[4], &[], 1, Some(&[f32::from_bits(bits)])).unwrap();
        assert_eq!(r[0], bits, "{}", l);
        if f[4] == "get_percent_time" && f[1] != "0" {
            let ct: f32 = f[1].parse().unwrap();
            let tot: f32 = f[2].parse().unwrap();
            assert_eq!((ct / (tot - 1.0)).to_bits(), bits, "{}", l);
        }
        if f[4] == "get_total_frames" {
            let tot: f32 = f[2].parse().unwrap();
            assert_eq!(tot.to_bits(), bits, "{}", l);
        }
        n += 1;
    }
    println!("cases {}", n);
    assert!(n > 0);
}
