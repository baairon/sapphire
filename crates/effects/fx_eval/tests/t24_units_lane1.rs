//! t24 (INT16): UNT2 PATCH 2 (the lane-1 term) and PATCH 3 (the popup rule) in fx_eval::units.
//! Vectors: data/UNT2_CONV_T2.tsv, a field-for-field copy of UNT2's rig T2 output (wave_s9/unt2_units_rest/out/conv_t2_out.jsonl,
//! SHA-256 bebafd6798719af6f2b945a9016e2f3a21d253d85d43a19cf1b6535e8f49c652): 18,880 cases of the original's conversion function 0x11ca10
//! run in the emulator (entries disp, vex and sse; lanes 0 and 1; ns 1 and others; arms clear, null and set; types 1 and 2).
use fx_eval::units::{conv_record, normalize_scale_record, popup_record, units_width, ConvIn, RatioArm};

fn v() -> &'static str {
    Box::leak(std::fs::read_to_string(effects_vecdir::path("fx_eval/data/UNT2_CONV_T2.tsv")).unwrap_or_default().into_boxed_str())
}

fn rows() -> Vec<(ConvIn, u32)> {
    v().lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let u = |i: usize| f[i].parse::<u64>().unwrap() as u32;
            let c = ConvIn {
                w: f[0].parse().unwrap(),
                conv_in_b: f32::from_bits(u(1)),
                arm: if f[2] == "clear" { RatioArm::Plain } else { RatioArm::Ratio },
                conv_in_a: f[3].parse().unwrap(),
                d_cell_b: f[4].parse().unwrap(),
                d_cell_c: f[5].parse().unwrap(),
                ptype: u(6),
                ns: f32::from_bits(u(7)),
                kind: u(8),
                flags: u(9),
                lane: u(10),
                host: f32::from_bits(u(11)),
            };
            (c, u(12))
        })
        .collect()
}

#[test]
fn t24_rig_vectors_bit_equal() {
    if !effects_vecdir::have_all(&["fx_eval/data/UNT2_CONV_T2.tsv"]) {
        return;
    }
    let r = rows();
    assert_eq!(r.len(), 18880);
    let mut bad = Vec::new();
    let mut lane1 = 0;
    for (c, want) in &r {
        let got = conv_record(c).expect("every rig case is a judged path").to_bits();
        if got != *want {
            bad.push((c.clone(), got, *want));
        }
        if c.lane == 1 {
            lane1 += 1;
        }
    }
    assert!(lane1 > 9000);
    assert!(bad.is_empty(), "{} of 18880 differ; first {:?}", bad.len(), bad.first());
}

#[test]
fn t24_shift_zero_sign_at_host_zero() {
    // the record of shift_mocha_mask at host 0 (SMR10's compile record): lanes (0, -0) at the plain-free ratio arm, conv_in_b 1.0
    let mut c = ConvIn { w: 64, conv_in_b: 1.0, arm: RatioArm::Ratio, conv_in_a: 1.0, d_cell_b: 1.0, d_cell_c: 1.0, ptype: 1, ns: 1.0, kind: 5, flags: 0, lane: 0, host: 0.0 };
    assert_eq!(conv_record(&c).unwrap().to_bits(), 0x0000_0000);
    c.lane = 1;
    assert_eq!(conv_record(&c).unwrap().to_bits(), 0x8000_0000);
}

#[test]
fn t24_lane0_equals_route_form_at_ratio_one() {
    // the 'ae32' route's lane-0 conversion is the same value as conv_record at ratio 1.0 (every claim run)
    for &(w, ns, host) in &[(64, 2.0f32, 3.0f32), (50, 4.0, 0.5272979736328125), (50, 2.0, 96.0), (128, 2.0, 3.0)] {
        let c = ConvIn { w, conv_in_b: 1.0, arm: RatioArm::Plain, conv_in_a: 1.0, d_cell_b: 1.0, d_cell_c: 1.0, ptype: 1, ns, kind: 5, flags: 0, lane: 0, host };
        assert_eq!(conv_record(&c).unwrap().to_bits(), normalize_scale_record(host, units_width(w, 1.0, 1.0), ns, false).to_bits());
    }
}

#[test]
fn t24_lane1_term_not_vacuous_and_sign_mutant_killed() {
    if !effects_vecdir::have_all(&["fx_eval/data/UNT2_CONV_T2.tsv"]) {
        return;
    }
    // the lane-1 record differs from lane 0 on some rig cases (the term is not vacuous), and the mutant without the vxorps sign flip
    // (the negated lane-1 value) fails the rig on some case
    let r = rows();
    let (mut nonvac, mut killed) = (0, 0);
    for (c, want) in r.iter().filter(|(c, _)| c.lane == 1 && c.kind & 0xf == 5) {
        let mut m = *c;
        m.lane = 0;
        if conv_record(&m).unwrap().to_bits() != *want {
            nonvac += 1;
        }
        if (-conv_record(c).unwrap()).to_bits() != *want {
            killed += 1;
        }
    }
    assert!(nonvac > 0 && killed > 0, "nonvacuous {} killed {}", nonvac, killed);
}

#[test]
fn t24_popup_one_based() {
    // UNT2 section 5 (MEASURED): Filter answer 2 -> 1.0, 3 -> 2.0, 1 -> 0.0, 0 -> 0.0, -3 -> 0.0; Combine Masks 1 -> 0.0; Opacity 2 -> 1.0
    for &(a, r) in &[(2, 1.0f32), (3, 2.0), (1, 0.0), (0, 0.0), (-3, 0.0)] {
        assert_eq!(popup_record(a).to_bits(), r.to_bits(), "answer {}", a);
    }
    assert_eq!(popup_record(i32::MIN), 2147483647i32 as f32); // dec wraps, sign flag clear, kept (cmovs not taken)
}
