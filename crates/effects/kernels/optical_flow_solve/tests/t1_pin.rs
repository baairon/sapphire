// RULE-PIN tests: operand order at the fold and add sites when both operands are NaN with distinct payloads. The silicon rule for
// the three-operand vector and scalar adds is: the first source operand, quieted. Expected words are derived from the listing's
// operand order, not from the emulator. Plus one rounding pin for the four-wide remainder block.
#[path = "support/small_side.rs"]
mod small_side;
use small_side::*;

const A: u32 = 0x7fc0_0001;
const B: u32 = 0x7fc0_0002;

fn nan(b: u32) -> f32 {
    f32::from_bits(b)
}

#[test]
fn pin_total_operand_order() {
    // total = fold.fadd(x0): the fold's NaN wins over x0's
    let mut acc = Acc::new();
    acc.v[0][0] = nan(B);
    acc.vx_0 = nan(A);
    let (t, _, _) = finish(&acc, 4);
    assert_eq!(t.to_bits(), B);
}

#[test]
fn pin_fold_operand_order() {
    // x2 = x7 + x5 (accumulators v0+v1 first, v2+v3 second)
    let mut acc = Acc::new();
    acc.v[0][0] = nan(A);
    acc.v[2][0] = nan(B);
    let (t, _, _) = finish(&acc, 4);
    assert_eq!(t.to_bits(), A);
}

#[test]
fn pin_block_add_operand_order() {
    // block add: acc = acc + element; two rows of 32 elements, NaN A in row 0 and NaN B in row 1 at the same lane
    let mut a = vec![0f32; 64];
    a[0] = nan(A);
    a[32] = nan(B);
    let acc = sum_plane(&a, 0, 0, 32, 32, 2);
    let (t, _, _) = finish(&acc, 64);
    assert_eq!(t.to_bits(), A);
}

#[test]
fn pin_four_wide_remainder() {
    // 36 elements, the last four 1e8, 3, 3, 3: the four-wide block and four scalar adds round differently
    let mut a = vec![0f32; 36];
    a[32] = 1.0e8;
    a[33] = 3.0;
    a[34] = 3.0;
    a[35] = 3.0;
    let acc = sum_plane(&a, 0, 0, 36, 36, 1);
    let (t, _, _) = finish(&acc, 36);
    assert_eq!(t, 100000008.0);
}

#[test]
fn pin_scalar_row_operand_order() {
    // scalar row (width 4): x0 = x0 + element, so the first NaN stays
    let a = [nan(A), nan(B), 0.0, 0.0];
    let acc = sum_plane(&a, 0, 0, 4, 4, 1);
    let (t, _, _) = finish(&acc, 4);
    assert_eq!(t.to_bits(), A);
}
