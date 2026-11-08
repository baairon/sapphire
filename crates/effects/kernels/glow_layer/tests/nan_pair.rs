// Two NaN operands in one operation: the result is the first source operand of the instruction, quieted. The blur window sums
// must not depend on the operand order the compiler emits for a commutative add.
use glow_layer::*;

const IN: [u32; 35] = [
    0x3e89ff43, 0x3ec74032, 0xbdfad4e5, 0x3e5f7acf, 0x3f000000, 0xff7fffff, 0x00000400, 0x3f63c586, 0x3f4f4160, 0xbef7d354, 0xbf800000, 0xff800000,
    0x80000000, 0xbe19ab61, 0xbe2f5d8a, 0x0da24260, 0xbe2408c6, 0x3eed8bed, 0x00000000, 0xf149f2ca, 0x00000000, 0x7149f2ca, 0x00000000, 0x3fba25a5,
    0x7f7fffff, 0xbed66fd6, 0x00000400, 0x3f4011f3, 0xbeadb8a9, 0x3eb97525, 0xffc00000, 0xbda1321b, 0x7f800001, 0x7149f2ca, 0x3f00d530,
];

fn run(sx: f32, sy: f32) -> Vec<u32> {
    let inp: Vec<f32> = IN.iter().map(|b| f32::from_bits(*b)).collect();
    let mut out = vec![f32::from_bits(0x7fc0ffee); 35];
    let pl = [Plane { data: &inp, width: 5, height: 7, stride: 5 }];
    {
        let mut dm = [PlaneMut { data: &mut out, width: 5, height: 7, stride: 5 }];
        let a = PassArgs { thresholds: [0.9, 0.0, 0.0], sigma_x: [sx, 0.0, 0.0], sigma_y: [sy, 0.0, 0.0], brightness: [1.3, 0.0, 0.0], subpixel: 0, edge_mode: 1, show_threshold: 0, aspect: Some(0.75) };
        glow_pass(&pl, &mut dm, &a).unwrap();
    }
    out.iter().map(|v| v.to_bits()).collect()
}

#[test]
fn nan_row_keeps_first_operand_payloads() {
    // row 6 holds a negative quiet NaN, a finite value and a signalling NaN: the window sum of the later samples starts from the
    // signalling NaN's difference (quieted, positive) and keeps it through the following additions.
    for (sx, sy) in [(0.3f32, 0.0f32), (0.3, 0.3)] {
        let o = run(sx, sy);
        assert_eq!(&o[30..35], &[0xffc00000, 0xffc00000, 0xffc00000, 0x7fc00001, 0x7fc00001], "sx {sx} sy {sy}");
    }
}
