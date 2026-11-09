use fx_eval::sawtooth::random_sawtooth_body as body;

const NAN_X86: u32 = 0xffc0_0000;

fn run(p3: f32, p4: f32) -> u32 {
    body(0x0114_64ae, f32::from_bits(0x4495_b800), f32::from_bits(0x4f00_0000), f32::from_bits(0x4120_0000), p3, p4, f32::from_bits(0xbf93_5688)).to_bits()
}

#[test]
fn a_zero_dwell_base_makes_the_x86_default_nan() {
    assert_eq!(run(0.0, 0.0), NAN_X86);
}

#[test]
fn an_infinite_dwell_base_makes_the_x86_default_nan() {
    assert_eq!(run(0.0, f32::INFINITY), NAN_X86);
    assert_eq!(run(f32::NEG_INFINITY, 0.0), NAN_X86);
}
