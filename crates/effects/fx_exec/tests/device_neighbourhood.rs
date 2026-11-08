//! Self-check of the neighbourhood device kernels (blur, masked_blur, grad, sharpen, edge_aware_sharpen, static2) against the CPU
//! kernels, bit for bit. Prints NOT RUN without an OpenCL device (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend};
use fx_exec::device_ops_neighbourhood::ALLOW;

#[test]
fn the_neighbourhood_kernels_pass_the_self_check_against_the_cpu() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            return;
        }
    };
    let b = DeviceBackend::with_allowlist(ctx, ALLOW);
    let rep = b.report();
    for (op, why) in &rep.refused {
        println!("REFUSED {op}: {why}");
    }
    assert!(rep.refused.is_empty(), "{:?}", rep.refused);
    let mut want: Vec<&str> = ALLOW.to_vec();
    want.sort();
    assert_eq!(b.admitted(), want);
    println!("ADMITTED {:?}", b.admitted());
    for d in fx_exec::device_ops::DEVICE_OPS.iter().filter(|d| ALLOW.contains(&d.class)) {
        println!("CORPUS {}: {} cases", d.class, (d.corpus)().len());
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// unit level checks of the edge aware pieces against the CPU jobs, bit for bit (the end to end self-check is blind to last bit
// differences of the weight planes, since only window membership reads them)

mod esh_units {
    use super::*;
    use edge_aware_sharpen_jobs as j3;
    use fx_device::cl::Arg as ClArg;
    use fx_device::DevImage;

    fn rng(seed: u64) -> impl FnMut() -> f32 {
        let mut s = seed ^ 0x9e37_79b9_7f4a_7c15;
        move || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (s >> 40) as f32 / (1u64 << 24) as f32
        }
    }

    fn session() -> Option<std::sync::Arc<fx_device::Session>> {
        match fx_device::select(DeviceKind::Gpu) {
            Handle::Device(d) => Some(d.session.clone()),
            other => {
                super::not_run(&format!("no usable OpenCL device ({})", other.describe()));
                None
            }
        }
    }

    fn bits(v: &[f32]) -> Vec<u32> {
        v.iter().map(|x| x.to_bits()).collect()
    }

    #[test]
    fn weights_cumsums_filters_and_the_transform_equal_the_cpu_jobs() {
        let Some(s) = session() else { return };
        let mut total = 0usize;
        for &(w, h) in &[(1usize, 1usize), (2, 1), (1, 4), (3, 5), (9, 7), (33, 12), (70, 9)] {
            for nch in 1..=3usize {
                for mode_b in [true, false] {
                    for (cs, base) in [(1u64, 0.5f32), (2, 1.0), (3, 8.0), (4, 0.0625)] {
                        let n = w * h;
                        let mut r = rng(cs * 1000 + (w * 31 + h * 7 + nch) as u64);
                        let chs: Vec<Vec<f32>> = (0..4).map(|_| (0..n).map(|_| r()).collect()).collect();
                        let cur: Vec<f32> = (0..n).map(|_| r()).collect();
                        let s2 = base / (nch as f32 * 1.5);
                        let k = if cs % 2 == 0 { 1.0f32 } else { 0.5 };
                        let sigma0 = [0.4f32, 1e-12, 2.0, 0.9999999999][(cs as usize) % 4];
                        // CPU
                        let ctx = j3::Ctx { w: w as i32, s: h as i32, nch: nch as i32, flag_cell_b: if mode_b { 0 } else { 1 }, f0: s2, f4: base, f8: s2, fc: base, esh2_job_ctx_a: sigma0, k };
                        let mut a = vec![f32::from_bits(0x7fc0_1234); n];
                        let mut t = vec![f32::from_bits(0x7fc0_1234); n];
                        {
                            let mut b = j3::Bufs { ch: [&chs[0], &chs[1], &chs[2], &chs[3]], cur: &cur, a: &mut a, t: &mut t };
                            j3::job_2(&ctx, 0, h as i32, &mut b).unwrap();
                        }
                        // device (the host computes the constant mode values as esh_run does)
                        let ca = j3::c_a(base);
                        let cb = j3::c_a(base);
                        let kcb = k * cb;
                        let (mut c10, mut c11) = (0.0f32, 0.0f32);
                        if !mode_b {
                            c10 = ca;
                            let bd = sigma0 as f64;
                            if j3::LOW > bd {
                                c11 = k * cb;
                            } else if bd > j3::HIGH {
                                c10 = j3::ONE;
                                c11 = k;
                            } else {
                                let x = j3::ONE / sigma0;
                                c10 = j3::minss(x, c10);
                                c11 = k * j3::minss(x, cb);
                            }
                        }
                        let up = |v: &Vec<f32>| DevImage::upload(&s, &[v.as_slice()]).unwrap();
                        let (d0, d1, d2, dcur) = (up(&chs[0]), up(&chs[1]), up(&chs[2]), up(&cur));
                        let da = DevImage::alloc(&s, 1, n).unwrap();
                        let dt = DevImage::alloc(&s, 1, n).unwrap();
                        s.launch(
                            "esh_weights_f32",
                            n,
                            &[
                                d0.plane(0).arg(), d1.plane(0).arg(), d2.plane(0).arg(), ClArg::I32(nch as i32), dcur.plane(0).arg(), ClArg::I32(mode_b as i32),
                                da.plane(0).arg(), dt.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(s2), ClArg::F32(s2), ClArg::F32(k),
                                ClArg::F32(ca), ClArg::F32(cb), ClArg::F32(kcb), ClArg::F32(c10), ClArg::F32(c11),
                            ],
                        )
                        .unwrap();
                        let ga = da.download().unwrap().remove(0);
                        let gt = dt.download().unwrap().remove(0);
                        assert_eq!(bits(&ga), bits(&a), "weights a {w}x{h} nch {nch} mode_b {mode_b} base {base}");
                        assert_eq!(bits(&gt), bits(&t), "weights t {w}x{h} nch {nch} mode_b {mode_b} base {base}");
                        // running sums
                        let mut p0 = vec![0f32; n];
                        let mut p1 = vec![0f32; n];
                        j3::job_cell_a(w as i32, h as i32, (0, h as i32), (0, w as i32), &a, &t, &mut p0, &mut p1).unwrap();
                        let dp0 = DevImage::alloc(&s, 1, n).unwrap();
                        let dp1 = DevImage::alloc(&s, 1, n).unwrap();
                        s.launch("esh_cumsum_x_f32", h, &[da.plane(0).arg(), dp0.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32)]).unwrap();
                        s.launch("esh_cumsum_y_f32", w, &[dt.plane(0).arg(), dp1.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32)]).unwrap();
                        assert_eq!(bits(&dp0.download().unwrap()[0]), bits(&p0), "cumsum x {w}x{h}");
                        assert_eq!(bits(&dp1.download().unwrap()[0]), bits(&p1), "cumsum y {w}x{h}");
                        // filters: guide = the running sums, radii around 1 to 6
                        for radius in [0.5f32, 1.0, 1.5, 2.0, 2.5, 3.75, 6.0, 40.0] {
                            for src_plane in 0..1 {
                                let srcv = &chs[src_plane];
                                // x job
                                let mut mem: Vec<f32> = Vec::new();
                                mem.extend_from_slice(srcv);
                                mem.extend(std::iter::repeat(f32::from_bits(0x7fc0_4321)).take(n));
                                mem.extend_from_slice(&p0);
                                let job = edge_aware_sharpen_box_filter::Job { w: w as i32, h: h as i32, radius, src: 0, dst: n, aux: 2 * n };
                                let mut rec = edge_aware_sharpen_box_filter::Rec::default();
                                edge_aware_sharpen_box_filter::job_x(&mut mem, &job, 0, h as i32, &mut rec);
                                let want_x = mem[n..2 * n].to_vec();
                                let p = edge_aware_sharpen_box_filter::cb_cell_a(radius);
                                let dsrc = up(srcv);
                                let ddst = DevImage::alloc(&s, 1, n).unwrap();
                                s.launch("esh_filter_x_f32", h, &[dsrc.plane(0).arg(), ddst.plane(0).arg(), dp0.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(p[0]), ClArg::F32(p[1]), ClArg::F32(p[2])]).unwrap();
                                assert_eq!(bits(&ddst.download().unwrap()[0]), bits(&want_x), "filter x {w}x{h} radius {radius}");
                                // y job: aux is the column major running sum of t
                                let mut mem: Vec<f32> = Vec::new();
                                mem.extend_from_slice(srcv);
                                mem.extend(std::iter::repeat(f32::from_bits(0x7fc0_4321)).take(n));
                                mem.extend_from_slice(&p1);
                                let mut rec = edge_aware_sharpen_box_filter::Rec::default();
                                edge_aware_sharpen_box_filter::job_y(&mut mem, &job, 0, w as i32, &mut rec);
                                let want_y = mem[n..2 * n].to_vec();
                                let ddst = DevImage::alloc(&s, 1, n).unwrap();
                                s.launch("esh_filter_y_f32", w, &[dsrc.plane(0).arg(), ddst.plane(0).arg(), dp1.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(p[0]), ClArg::F32(p[1]), ClArg::F32(p[2])]).unwrap();
                                assert_eq!(bits(&ddst.download().unwrap()[0]), bits(&want_y), "filter y {w}x{h} radius {radius}");
                                total += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("ESH UNITS: {total} filter pairs and the matching weight and sum planes equal");
    }

    #[test]
    fn the_transform_equals_the_cpu_job_on_every_class_of_value() {
        let Some(s) = session() else { return };
        let vals = fx_device::selfcheck::adversarial_f32(4000, 77);
        let mut x: Vec<f32> = vals.clone();
        let mut r = rng(5);
        x.extend((0..3000).map(|_| (r() - 0.5) * 20.0));
        x.extend((0..500).map(|_| (r() - 0.5) * 400.0));
        let n = x.len();
        for s0 in [0.0f32, 1.0, -1.0, 0.37, 2.5, 4.25, 100.0, -0.0, f32::NAN, 1e-30] {
            let bb: Vec<f32> = x.iter().rev().cloned().collect();
            let aa = x.clone();
            let mut want = vec![0.25f32; n];
            for i in 0..n {
                let c = bb[i].mul_add(-1.0, 0.0);
                let c = c + aa[i];
                let c = j3::exp_tanh_elem(s0, c);
                want[i] += c;
            }
            let up = |v: &Vec<f32>| DevImage::upload(&s, &[v.as_slice()]).unwrap();
            let (db, da, dout) = (up(&bb), up(&aa), up(&vec![0.25f32; n]));
            s.launch("esh_level_f32", n, &[db.plane(0).arg(), da.plane(0).arg(), dout.plane(0).arg(), ClArg::U32(n as u32), ClArg::F32(s0)]).unwrap();
            let got = dout.download().unwrap().remove(0);
            for i in 0..n {
                let (g, w) = (got[i], want[i]);
                assert!(g.to_bits() == w.to_bits() || (g.is_nan() && w.is_nan()), "s0 {s0} i {i} b {:08x} a {:08x}: got {:08x} want {:08x}", bb[i].to_bits(), aa[i].to_bits(), g.to_bits(), w.to_bits());
            }
        }
    }
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}

// ---------------------------------------------------------------------------------------------------------------------------------
// two-NaN operands in the window sums: the device must return the CPU's NaN bits (first NaN operand, quieted), payloads included

#[test]
fn the_blur_kernels_equal_the_cpu_bit_for_bit_on_two_nan_operand_cases() {
    use fx_device::selfcheck::NanPayloads;
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            return;
        }
    };
    let cases = fx_exec::device_ops_neighbourhood::blur_nan_pair_cases();
    assert!(cases.len() >= 50);
    let mut op = fx_exec::device_ops_neighbourhood::BLUR;
    op.corpus = fx_exec::device_ops_neighbourhood::blur_nan_pair_cases;
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect("device equals the CPU");
    println!("NAN-PAIR cases {} outputs {}", cases.len(), tally.outputs);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "NaN payloads differ: {}", tally);
}
