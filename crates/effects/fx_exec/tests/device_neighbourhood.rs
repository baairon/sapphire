//! Self-check of the neighbourhood device kernels (blur, masked_blur, grad, sharpen, edge_aware_sharpen, static2) against the CPU
//! kernels, bit for bit. Prints NOT RUN without an OpenCL device (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend};
use fx_exec::device_ops_neighbourhood::ALLOW;
use fx_exec::{KParams, Val};

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

    /// Strict two-NaN set: planes and scalars holding NaN of both signs, quiet and signalling, with distinct payloads, plus infinities
    /// and signed zeros. The device must give the CPU's bits (payload included) for the weights, both running sums and both filters.
    #[test]
    fn weights_cumsums_and_filters_equal_the_cpu_on_two_nan_inputs() {
        use fx_common::nanop::NanOps;
        let Some(s) = session() else { return };
        let nans = [0x7f80_0001u32, 0xff80_0002, 0x7fc0_0003, 0xffc0_0004, 0x7fa0_0000, 0xffe0_0005];
        let fixed = [0x7f80_0000u32, 0xff80_0000, 0, 0x8000_0000, 0x0000_0001, 0x3f80_0000, 0xbf80_0000, 0x4000_0000];
        let mut st = 0x2545_f491_4f6c_dd1du64;
        let mut next = move || {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            st
        };
        let mut cases = 0usize;
        for round in 0..120usize {
            let (w, h) = [(1usize, 3usize), (2, 2), (5, 4), (9, 7), (33, 6), (17, 17)][round % 6];
            let n = w * h;
            let nch = 1 + round % 3;
            let mode_b = round % 2 == 0;
            let rate = [3u64, 8, 25][(round / 6) % 3];
            let mut gen = |nx: &mut dyn FnMut() -> u64| -> f32 {
                let v = nx();
                if v % rate == 0 {
                    f32::from_bits(nans[(v >> 8) as usize % 6])
                } else if v % 11 == 0 {
                    f32::from_bits(fixed[(v >> 8) as usize % 8])
                } else {
                    ((v >> 8) % 1000) as f32 / 500.0 - 1.0
                }
            };
            let chs: Vec<Vec<f32>> = (0..4).map(|_| (0..n).map(|_| gen(&mut next)).collect()).collect();
            let cur: Vec<f32> = (0..n).map(|_| { let v = gen(&mut next); if v.is_nan() { v } else { v.abs().fract() } }).collect();
            let mut sc = [0.5f32, 1.5, 0.5, 1.5, 0.4, 0.75];
            for i in 0..6 {
                if next() % 4 == 0 {
                    sc[i] = f32::from_bits(nans[(next() >> 8) as usize % 6]);
                }
            }
            let [s2d, base, s2e, fc, sigma0, k] = sc;
            let ctx = j3::Ctx { w: w as i32, s: h as i32, nch: nch as i32, flag_cell_b: if mode_b { 0 } else { 1 }, f0: s2d, f4: base, f8: s2e, fc, esh2_job_ctx_a: sigma0, k };
            let mut a = vec![0f32; n];
            let mut t = vec![0f32; n];
            {
                let mut b = j3::Bufs { ch: [&chs[0], &chs[1], &chs[2], &chs[3]], cur: &cur, a: &mut a, t: &mut t };
                j3::job_2(&ctx, 0, h as i32, &mut b).unwrap();
            }
            let ca = j3::c_a(base);
            let cb = j3::c_a(fc);
            let kcb = k.fmul(cb);
            let (mut c10, mut c11) = (0.0f32, 0.0f32);
            if !mode_b {
                c10 = ca;
                let bd = sigma0 as f64;
                if j3::LOW > bd {
                    c11 = k.fmul(cb);
                } else if bd > j3::HIGH {
                    c10 = j3::ONE;
                    c11 = k;
                } else {
                    let x = j3::ONE / sigma0;
                    c10 = j3::minss(x, c10);
                    c11 = k.fmul(j3::minss(x, cb));
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
                    da.plane(0).arg(), dt.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(s2d), ClArg::F32(s2e), ClArg::F32(k),
                    ClArg::F32(ca), ClArg::F32(cb), ClArg::F32(kcb), ClArg::F32(c10), ClArg::F32(c11),
                ],
            )
            .unwrap();
            assert_eq!(bits(&da.download().unwrap()[0]), bits(&a), "strict weights a round {round}");
            assert_eq!(bits(&dt.download().unwrap()[0]), bits(&t), "strict weights t round {round}");
            let mut p0 = vec![0f32; n];
            let mut p1 = vec![0f32; n];
            j3::job_cell_a(w as i32, h as i32, (0, h as i32), (0, w as i32), &a, &t, &mut p0, &mut p1).unwrap();
            let dp0 = DevImage::alloc(&s, 1, n).unwrap();
            let dp1 = DevImage::alloc(&s, 1, n).unwrap();
            s.launch("esh_cumsum_x_f32", h, &[da.plane(0).arg(), dp0.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32)]).unwrap();
            s.launch("esh_cumsum_y_f32", w, &[dt.plane(0).arg(), dp1.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32)]).unwrap();
            assert_eq!(bits(&dp0.download().unwrap()[0]), bits(&p0), "strict cumsum x round {round}");
            assert_eq!(bits(&dp1.download().unwrap()[0]), bits(&p1), "strict cumsum y round {round}");
            // filters on the NaN-bearing source with the NaN-bearing sums as guide
            for radius in [1.5f32, 3.75, 6.0] {
                let srcv = &chs[0];
                let job = edge_aware_sharpen_box_filter::Job { w: w as i32, h: h as i32, radius, src: 0, dst: n, aux: 2 * n };
                let p = edge_aware_sharpen_box_filter::cb_cell_a(radius);
                let mut p = p;
                if round % 5 == 0 {
                    p[2] = f32::from_bits(nans[round % 6]);
                }
                let _ = job;
                let dsrc = up(srcv);
                let mut mem: Vec<f32> = Vec::new();
                mem.extend_from_slice(srcv);
                mem.extend(std::iter::repeat(0f32).take(n));
                mem.extend_from_slice(&p0);
                let mut want = vec![0f32; n];
                for row in 0..h {
                    let mut m2: Vec<f32> = Vec::new();
                    m2.extend_from_slice(&mem[row * w..(row + 1) * w]);
                    m2.extend(std::iter::repeat(0f32).take(w));
                    m2.extend_from_slice(&mem[2 * n + row * w..2 * n + (row + 1) * w]);
                    let mut rec = edge_aware_sharpen_box_filter::Rec::default();
                    edge_aware_sharpen_box_filter::filter3(&mut m2, 0, w, 2 * w, w as i64, p, &mut rec);
                    want[row * w..(row + 1) * w].copy_from_slice(&m2[w..2 * w]);
                }
                let ddst = DevImage::alloc(&s, 1, n).unwrap();
                s.launch("esh_filter_x_f32", h, &[dsrc.plane(0).arg(), ddst.plane(0).arg(), dp0.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(p[0]), ClArg::F32(p[1]), ClArg::F32(p[2])]).unwrap();
                assert_eq!(bits(&ddst.download().unwrap()[0]), bits(&want), "strict filter x round {round} radius {radius}");
                cases += 1;
            }
            // plane add kernel
            let mut out0: Vec<f32> = (0..n).map(|_| gen(&mut next)).collect();
            let dout = up(&out0);
            s.launch("esh_add_f32", n, &[da.plane(0).arg(), dout.plane(0).arg(), ClArg::U32(n as u32)]).unwrap();
            for i in 0..n {
                out0[i] = out0[i].fadd(a[i]);
            }
            assert_eq!(bits(&dout.download().unwrap()[0]), bits(&out0), "strict add round {round}");
        }
        println!("ESH STRICT NAN-PAIR: {cases} filter cases and 120 weight, sum and add rounds equal, payload included");
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

#[test]
fn the_masked_blur_and_sharpen_kernels_equal_the_cpu_bit_for_bit_on_two_nan_operand_cases() {
    use fx_device::selfcheck::NanPayloads;
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            return;
        }
    };
    let mut op = fx_exec::device_ops_neighbourhood::MASKED_BLUR;
    let cases = fx_exec::device_ops_neighbourhood::masked_blur_nan_pair_cases();
    assert!(cases.len() >= 50);
    op.corpus = fx_exec::device_ops_neighbourhood::masked_blur_nan_pair_cases;
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect("masked_blur: device equals the CPU");
    println!("MASKED NAN-PAIR cases {} outputs {}", cases.len(), tally.outputs);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "masked_blur NaN payloads differ: {}", tally);
    let mut op = fx_exec::device_ops_neighbourhood::SHARPEN;
    let cases = fx_exec::device_ops_neighbourhood::sharpen_nan_pair_cases();
    assert!(cases.len() >= 50);
    op.corpus = fx_exec::device_ops_neighbourhood::sharpen_nan_pair_cases;
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect("sharpen: device equals the CPU");
    println!("SHARPEN NAN-PAIR cases {} outputs {}", cases.len(), tally.outputs);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "sharpen NaN payloads differ: {}", tally);
}

// ---------------------------------------------------------------------------------------------------------------------------------
// NaN-exact admission of blur, masked_blur, sharpen and edge_aware_sharpen: their NaN cases (long rows and columns, NaN gains and
// lanes, NaN masks) report equal NaN payloads, and a NaN output of each stays on the device with the CPU's words

fn require_device() -> Option<std::sync::Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            None
        }
    }
}

type CaseFn = fn() -> Vec<fx_exec::device_ops::SelfCase>;

fn nan_case_sets() -> Vec<(fx_exec::device_ops::DeviceOp, &'static str, CaseFn)> {
    use fx_exec::device_ops_neighbourhood as nb;
    vec![
        (nb::BLUR, "blur wide", nb::blur_nan_wide_cases as CaseFn),
        (nb::MASKED_BLUR, "masked_blur wide", nb::masked_blur_nan_wide_cases as CaseFn),
        (nb::SHARPEN, "sharpen wide", nb::sharpen_nan_wide_cases as CaseFn),
        (nb::EDGE_AWARE_SHARPEN, "edge_aware_sharpen", nb::esh_nan_cases as CaseFn),
    ]
}

/// The CPU output words of a case and the operands as executor values (constants at their operand positions).
fn cpu_words_and_vals(class: &'static str, c: &fx_exec::device_ops::SelfCase) -> (Vec<Vec<u32>>, Vec<Val>) {
    let total = c.args.len() + c.consts.len();
    let mut imgs = c.args.iter();
    let mut vals = Vec::with_capacity(total);
    for pos in 0..total {
        match c.consts.iter().find(|k| k.0 == pos) {
            Some(k) => vals.push(Val::Const { n: k.1, lanes: k.2 }),
            None => vals.push(Val::Img(std::rc::Rc::new(imgs.next().expect("an image operand").clone()))),
        }
    }
    let args: Vec<fx_exec::Arg> = vals
        .iter()
        .map(|v| match v {
            Val::Img(i) => fx_exec::Arg::Img(i),
            Val::Const { n, lanes } => fx_exec::Arg::Const { n: *n, lanes: *lanes },
            _ => unreachable!(),
        })
        .collect();
    let kp = c.kp.unwrap_or_else(KParams::standin);
    let n = c.dims.0 * c.dims.1;
    let want = match fx_exec::kernels::run_any(class, c.tag, &args, n, c.dims, &kp).expect("the CPU kernel runs the case") {
        fx_exec::kernels2::Out::Img(i) => i.planes.iter().map(|p| p.iter().map(|v| v.to_bits()).collect()).collect(),
        _ => panic!("not an image"),
    };
    (want, vals)
}

#[test]
fn the_wide_and_edge_aware_nan_cases_report_equal_nan_payloads() {
    use fx_device::selfcheck::NanPayloads;
    let Some(ctx) = require_device() else { return };
    for (mut op, name, cases) in nan_case_sets() {
        let list = cases();
        let mut nan_out = 0usize;
        let mut words = std::collections::BTreeSet::new();
        for c in &list {
            let (want, _) = cpu_words_and_vals(op.class, c);
            for w in want.iter().flatten().filter(|w| (*w & 0x7fff_ffff) > 0x7f80_0000) {
                nan_out += 1;
                words.insert(*w);
            }
        }
        op.corpus = cases;
        let tally = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect("device equals the CPU");
        println!("NAN-CASES {name}: cases {} outputs {} CPU NaN output words {nan_out} distinct {}", list.len(), tally.outputs, words.len());
        assert!(nan_out > 0 && words.len() >= 4, "{name}: the cases must reach NaN outputs with several words");
        assert!(matches!(tally.nan_payloads, NanPayloads::Same), "{name}: NaN payloads differ: {tally}");
    }
}

#[test]
fn a_nan_output_of_the_neighbourhood_nan_exact_ops_stays_on_the_device_with_the_cpu_words() {
    let Some(ctx) = require_device() else { return };
    let classes = ["blur", "masked_blur", "sharpen", "edge_aware_sharpen"];
    let mut b = DeviceBackend::with_allowlist(ctx, &classes);
    for c in classes {
        assert!(fx_exec::device_ops::NAN_EXACT.contains(&c), "{c} is on NAN_EXACT");
        assert!(b.nan_exact().contains(&c), "{c}: the self-check must report equal NaN payloads ({:?})", b.nan_exact());
    }
    for (op, name, cases) in nan_case_sets() {
        let list = cases();
        let mut ran = 0;
        for c in list.iter().step_by(7).take(4) {
            let (want, vals) = cpu_words_and_vals(op.class, c);
            if !want.iter().flatten().any(|w| (*w & 0x7fff_ffff) > 0x7f80_0000) {
                continue;
            }
            let kp = c.kp.unwrap_or_else(KParams::standin);
            let call = fx_exec::backend::NodeCall { idx: 0, class: op.class, tag: c.tag, n: c.dims.0 * c.dims.1, dims: c.dims, kp: &kp };
            let got = b.try_run(&call, &vals);
            let Ok(Val::Dev(d)) = got else { panic!("{name} {}: expected a device result, got {:?}", c.name, got.err()) };
            let img = d.materialize().expect("download");
            let words: Vec<Vec<u32>> = img.planes.iter().map(|p| p.iter().map(|v| v.to_bits()).collect()).collect();
            assert_eq!(words, want, "{name} {}: device words differ from the CPU", c.name);
            ran += 1;
        }
        assert!(ran > 0, "{name}: no case with a NaN output");
        println!("NAN-STAYS {name}: {ran} nodes with NaN outputs stayed on the device");
    }
    assert!(b.report().fallback.is_empty(), "no fallback expected: {:?}", b.report().fallback);
}

/// sharpen at gain exactly 1.0 (x * 2 - b fused): where the source is finite and the blurred value is a lone NaN, the result is
/// that NaN quieted with its own sign, never negated; the device gives the CPU's words.
#[test]
fn sharpen_at_gain_one_returns_a_lone_blurred_nan_quieted_with_its_own_sign() {
    let Some(ctx) = require_device() else { return };
    let (w, h) = (21usize, 1usize);
    for word in [0x7fa0_0001u32, 0xff80_0007, 0x7fc0_0003, 0xffc0_0009] {
        let mut x: Vec<f32> = (0..w).map(|i| 0.25 + i as f32 / 64.0).collect();
        x[10] = f32::from_bits(word);
        let mut kp = KParams::standin();
        kp.sharpen_count = Some(w * h);
        let case = fx_exec::device_ops::SelfCase {
            name: format!("gain one lone NaN {word:08x}"),
            dims: (w, h),
            tag: 1,
            args: vec![fx_exec::Image { n: w * h, planes: vec![x.clone()] }],
            consts: vec![(1, 4, [0x3f80_0000; 4]), (2, 1, [0.4f32.to_bits(), 0, 0, 0])],
            kp: Some(kp),
        };
        let (want, vals) = cpu_words_and_vals("sharpen", &case);
        let quiet = word | 0x0040_0000;
        let lone: Vec<usize> = (0..w).filter(|&i| i != 10 && want[0][i] == quiet).collect();
        assert!(!lone.is_empty(), "{word:08x}: the blur must spread the NaN to finite source samples: {:08x?}", want[0]);
        let mut b = DeviceBackend::with_allowlist(ctx.clone(), &["sharpen"]);
        assert!(b.nan_exact().contains(&"sharpen"));
        let call = fx_exec::backend::NodeCall { idx: 0, class: "sharpen", tag: 1, n: w * h, dims: (w, h), kp: &kp };
        let Ok(Val::Dev(d)) = b.try_run(&call, &vals) else { panic!("{word:08x}: expected a device result") };
        let got: Vec<u32> = d.materialize().expect("download").planes[0].iter().map(|v| v.to_bits()).collect();
        assert_eq!(got, want[0], "{word:08x}: device words differ from the CPU");
        println!("GAIN-ONE {word:08x}: {} finite-source samples give {quiet:08x}", lone.len());
    }
}
