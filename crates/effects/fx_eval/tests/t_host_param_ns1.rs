//! The single-lane normalize_scale 1 host route: `--host-param NAME=HOST` for a one-lane parameter converts the host value as
//! host x ((2 / W) x 1) with W the frame width on every axis. No stored data is read.
//! Mutant: the `ns1_height_mut` feature reads the frame height instead; the 50x7 tests below fail on it.
use fx_eval::units::{host_param_lane0, normalize_scale_record, units_width};

const PROG: &str = "Shake_Normal_Autogen";

/// the lane the record route binds: the host value times f32(f32(2 / W) x 1), every operation single precision
fn record_route(host: f32, w: u32) -> f32 {
    host * ((2.0f32 / w as f32) * 1.0f32)
}

fn host_route(param: &str, host: f32, w: i32, h: i32) -> Result<f32, String> {
    host_param_lane0(PROG, param, host, 1.0, units_width(w, 1.0, 1.0), units_width(h, 1.0, 1.0))
}

/// deterministic 16.16 hosts: k / 65536 for a pseudo random k in [0, 100 x 65536)
fn hosts(n: usize) -> Vec<f32> {
    let mut s = 0x2545_f491_4f6c_dd1du64;
    (0..n)
        .map(|_| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            ((s % (100 * 65536)) as f64 / 65536.0) as f32
        })
        .collect()
}

fn sweep(w: i32, h: i32) {
    for name in ["x_rand_amp", "y_rand_amp", "y_wave_amp", "z_rand_amp", "z_wave_amp"] {
        for host in hosts(400) {
            let got = host_route(name, host, w, h).unwrap();
            assert_eq!(got.to_bits(), record_route(host, w as u32).to_bits(), "{} host {} at {}x{}", name, host, w, h);
            assert_eq!(got.to_bits(), normalize_scale_record(host, w as f32, 1.0, false).to_bits());
        }
    }
}

#[test]
fn lanes_at_64x32_equal_the_record_route() {
    sweep(64, 32);
}

#[test]
fn lanes_at_50x7_equal_the_record_route_and_not_the_height_variant() {
    sweep(50, 7);
    // the height variant differs, so a route reading the height fails the sweep above
    let host = 0.35f32;
    assert_ne!(record_route(host, 50).to_bits(), record_route(host, 7).to_bits());
}

#[test]
fn lanes_at_49x9_equal_the_record_route() {
    sweep(49, 9);
}

/// y_ and z_ amplitudes bound in recorded Dissolve_Shake renders at 64x32 and 50x7: the host slider word and lane 0 of the
/// recorded parameter list after the loop, printed at 6 decimals (rows written by a script that reads the records)
#[test]
fn y_and_z_amplitudes_equal_the_child_record_lanes() {
    // W 64: ds_nf2_64 y_rand_amp, ds_nf2_64 z_rand_amp, ds_wr_64 y_wave_amp
    const R64: [(u32, &str); 3] = [(0x3eb33400, "0.010938"), (0x3e199800, "0.004687"), (0x3e800000, "0.007812")];
    // W 50: ds_nf2_50 y_rand_amp, ds_nf2_50 z_rand_amp, ds_wr_50 y_wave_amp
    const R50: [(u32, &str); 3] = [(0x3eb33400, "0.014000"), (0x3e199800, "0.006000"), (0x3e800000, "0.010000")];
    for (w, h, rows) in [(64i32, 32i32, &R64[..]), (50, 7, &R50[..])] {
        for (i, (bits, rec)) in rows.iter().enumerate() {
            let lane = host_route(["y_rand_amp", "z_rand_amp", "y_wave_amp"][i], f32::from_bits(*bits), w, h).unwrap();
            assert_eq!(format!("{:.6}", lane), *rec, "{}x{}", w, h);
            assert_eq!(lane.to_bits(), record_route(f32::from_bits(*bits), w as u32).to_bits());
        }
    }
}

#[test]
fn only_one_lane_ns_1_parameters_are_accepted() {
    // a two-lane ns 1 parameter takes both host lanes: one host lane is refused by name
    for (prog, p) in [("Drop_Shadow", "shift"), ("Drop_Shadow", "shift_mocha_mask")] {
        let e = host_param_lane0(prog, p, 1.0, 1.0, 64.0, 32.0).unwrap_err();
        assert!(e.starts_with("PARAM-UNITS-NOT-JUDGED"), "{}", e);
    }
    // every registered ns 1 parameter is accepted exactly when the range row gives one lane
    let mut one = 0;
    for s in effects_rtdata::styles::STYLES.iter() {
        for r in s.ns.iter().filter(|r| r.normalize_scale == Some(1.0)) {
            let lanes = fx_eval::units::param_lanes(r.program, r.param);
            let got = host_param_lane0(r.program, r.param, 0.5, 1.0, 64.0, 32.0);
            assert_eq!(got.is_ok(), lanes == Some(1), "{} {} lanes {:?}", r.program, r.param, lanes);
            one += got.is_ok() as usize;
        }
    }
    assert!(one > 0);
    // other normalize scales stay refused; 2 and 4 keep their lane 0 route
    for ns in [0.5f32, 3.0, 8.0, f32::NAN] {
        assert!(host_param_lane0(PROG, "y_rand_amp", 1.0, ns, 64.0, 32.0).unwrap_err().starts_with("PARAM-UNITS-NOT-JUDGED"));
    }
    for ns in [2.0f32, 4.0] {
        let got = host_param_lane0(PROG, "y_rand_amp", 0.35, ns, 50.0, 7.0).unwrap();
        assert_eq!(got.to_bits(), normalize_scale_record(0.35, 50.0, ns, false).to_bits());
    }
}
