//! step s10_eqck: eq_const in the listing's f64 form and the checkout crate aligned with the listing.
//! Expected values are the listing's arithmetic worked by hand: eps = f64 0x3ddb7cdfd9d7bdbb (about 1.0e-10) is below half an ulp of 2^20
//! (2^-33, about 1.16e-10), so b + eps rounds to b and two equal constants 1048576.0 are NOT equal in the listing's test.
use fx_eval::checkout::*;
use fx_eval::compile::{prune_arm, ArgView2};

fn av(c: bool, l: f32, n: usize) -> ArgView2 {
    ArgView2 { is_const: c, lanes: [l; 4], n, elnum: Some(n as i32) }
}

#[test]
fn eq_const_listing_form() {
    let x = || av(false, 0.0, 4);
    assert_eq!(prune_arm("if_eq", &[av(true, 1.0, 1), av(true, 1.0, 1), x(), x()]), Some(2));
    assert_eq!(prune_arm("if_eq", &[av(true, 1.0, 1), av(true, 1.0000001, 1), x(), x()]), Some(3));
    assert_eq!(prune_arm("if_eq", &[av(true, 1048576.0, 1), av(true, 1048576.0, 1), x(), x()]), Some(3));
    assert_eq!(prune_arm("if_eq", &[av(true, 0.0, 1), av(true, -0.0, 1), x(), x()]), Some(2));
    assert_eq!(prune_arm("if_neq_const", &[av(true, 1048576.0, 1), av(true, 1048576.0, 1), x(), x()]), Some(2));
}

#[test]
fn checkout_ba_and_owner_refusals() {
    let b = |ba| CheckoutBytes { ba, bc: 0 };
    assert_eq!(try_refresh(b(32), 0xff, 0, true), Err(CheckoutError::BaOutOfRange(32)));
    assert_eq!(try_refresh(b(0), 0xff, 0, false), Err(CheckoutError::OwnerNull));
    assert_eq!(try_refresh(b(-1), 0, 0, false), Ok(false));
    assert_eq!(try_refresh(b(-1), 0, 0, true), Ok(true));
    assert_eq!(try_refresh(b(1), 0x02, 0, true), Ok(true));
    assert_eq!(try_refresh(b(1), 0x01, 0, true), Ok(false));
    assert_eq!(try_refresh(b(9), 0xff, 0, true), Ok(false));
    assert_eq!(try_refresh(CheckoutBytes { ba: 9, bc: 1 }, 0xff, 0x10, true), Ok(true));
    for ba in -1i16..8 {
        for m in [0u8, 1, 2, 0x55, 0xff] {
            assert_eq!(try_refresh(b(ba), m, 0, true), Ok(refresh(b(ba), m, 0, true)), "ba {} mask {}", ba, m);
        }
    }
}

#[test]
fn checkout_later_token_keeps_its_ba() {
    let t = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let ins = ["Source", "Background", "Mask"];
    assert_eq!(bind_bytes(&t(&["show_with_mask", "bg_only"]), &ins, Arms::READ).ba, 1);
    assert_eq!(bind_bytes(&t(&["bg_only", "show_with_mask"]), &ins, Arms::READ).ba, 2);
    assert_eq!(bind_bytes(&t(&["mask_only"]), &ins, Arms::READ).ba, 2);
    assert_eq!(bind_bytes(&t(&["show_with_mocha"]), &ins, Arms::READ), CheckoutBytes { ba: -1, bc: 1 });
}
