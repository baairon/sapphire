//! The `sin` fold evaluates the AVX2 sine (`svml_sin_avx2`), not the earlier `svml_trig` form. Needs no stored data: the inputs are
//! ones where the two functions differ in the last bit, and the expected words are fixed here.
use fx_eval::fold::*;
use fx_eval::svml_sin_avx2;
use fx_eval::svml_trig;
use fx_eval::*;

fn cv(x: f32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: [x; 4], n: 1, elnum: Some(1) }
}

/// (input bits, bits of `svml_sin_avx2::sin_rev`); `svml_trig::sin_rev` differs from the second word by one unit in the last place.
const PINS: [(u32, u32); 6] = [
    (0x3c3b_8415, 0x3d93_25d5),
    (0xc03c_b9a1, 0x3ea1_cab9),
    (0xc032_62c8, 0x3f79_0269),
    (0xc005_0160, 0xbef1_97d9),
    (0xbfe5_b4d5, 0x3f76_05b7),
    (0x3c3b_8415 | 0x8000_0000, 0xbd93_25d5),
];

#[test]
fn the_sin_fold_is_the_avx2_function_where_the_two_forms_differ() {
    for (x, want) in PINS {
        let xf = f32::from_bits(x);
        assert_eq!(svml_sin_avx2::sin_rev(xf).to_bits(), want, "avx2 form at {:08x}", x);
        assert_ne!(svml_trig::sin_rev(xf).to_bits(), want, "the earlier form must differ at {:08x}, or the pin cannot tell them apart", x);
        let r = fold_lanes("sin", &[cv(xf)], 1, None).expect("sin folds over a constant");
        assert_eq!(r[0], want, "sin fold at {:08x}", x);
    }
}
