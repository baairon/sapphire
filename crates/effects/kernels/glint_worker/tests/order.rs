use glint_worker::{refusal, Refusal, SourceRecord, Words};

fn source(diagonal_mode: i32) -> SourceRecord {
    SourceRecord { selector: 0, plane_bytes: 4, count: 1, width: 1, height: 5, diagonal_mode }
}

/// In a diagonal pass the shift check comes before the extent check, as the host's order does.
#[test]
fn shift_is_named_before_the_extent_in_a_diagonal_pass() {
    let one = 1.0f32.to_bits();
    let w = Words { a: [one; 3], b: [one; 3], c: [one; 3], pass_scale: [one, one, f32::INFINITY.to_bits(), one], pass_gain: [one; 4] };
    assert_eq!(refusal(&source(1), 0, 1, 5, &w), Some(Refusal::Shift));
    let ok = Words { a: [one; 3], b: [one; 3], c: [one; 3], pass_scale: [one; 4], pass_gain: [one; 4] };
    assert_eq!(refusal(&source(1), 0, 1, 5, &ok), Some(Refusal::DiagExtent));
}
