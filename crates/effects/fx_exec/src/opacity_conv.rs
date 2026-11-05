//! INT26 step OP (W558 (a), OPC1 PATCH): the host Opacity checkout integer to the loop value of the program parameter `opacity`.
//! Copy of wave_s9/opc1_opacity_convert/port/src/lib.rs (rules and constants only), with the kind-4 arm selector added by INT26:
//! the integer arm ([prog] == 0) is ported; the other arm (a host call) is refused by name (OPACITY-HOSTCALL-ARM-UNPORTED, W558 (c)).
//! The loop value is f32(max(v - 1, 0)) for v the checkout integer; the parameter record's optional upper and lower clamps then apply.

/// The label of the merged conversion (W558 (a) text, pasted exactly).
pub const OPACITY_CONVERT_PORTED: &str = "OPACITY-CONVERT-PORTED (opacity = f32(max(v - 1, 0)) for v the host's Opacity checkout integer, then the parameter record's clamps; kind case and v MEASURED at Trails answers 2 and 3 and Blur answers 1 and 3; the lane written at the conversion store is the lane the owner loop reads (address MEASURED) and holds 0.0, 1.0, 0.0, 2.0 (MEASURED); the register value at the store instant INFERRED equal to the lane (the memory watch does not see AVX-engine stores; the following clamp stores write the same value); unit tests at both styles' six points and the clamps; identity and v-instead-of-v-1 mutants killed at Trails answers 2 and 3; OPC1, W548, W558)";

/// `rec_max` (upper clamp, a minimum with the value) and `rec_min` (lower clamp, a maximum with the value) are applied when set.
pub fn popup_to_loop_opacity(v: i32, rec_max: Option<f32>, rec_min: Option<f32>) -> f32 {
    let mut x = v.saturating_sub(1).max(0) as f32;
    if let Some(m) = rec_max {
        x = if x < m { x } else { m };
    }
    if let Some(m) = rec_min {
        x = if m > x { m } else { x };
    }
    x
}

/// The kind-4 case with its arm selector: `prog_arm_cell` is the program cell [prog]. Zero takes the integer arm (ported); a nonzero
/// value takes the host-call arm, which is not ported and is refused by name.
pub fn checkout_to_loop_opacity(prog_arm_cell: u32, v: i32, rec_max: Option<f32>, rec_min: Option<f32>) -> Result<f32, String> {
    if prog_arm_cell != 0 {
        return Err(format!(
            "OPACITY-HOSTCALL-ARM-UNPORTED: [prog] = {}: the kind-4 arm that asks the host through [r9] is not ported (OPC1, W558 (c))",
            prog_arm_cell
        ));
    }
    Ok(popup_to_loop_opacity(v, rec_max, rec_min))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mutant 1: the identity conversion the old feed used.
    fn mutant_identity(v: i32) -> f32 {
        v as f32
    }

    /// Mutant 2: v instead of v - 1 (clamped at 0).
    fn mutant_v_not_v_minus_1(v: i32) -> f32 {
        v.max(0) as f32
    }

    // Measured points (OPC1): (style, host checkout integer v, observed loop value). Trails bound values 0, 1, 2; Blur bound values 1, 2, 3.
    const POINTS: [(&str, i32, f32); 6] = [
        ("Trails answer 1", 0, 0.0),
        ("Trails answer 2", 1, 0.0),
        ("Trails answer 3", 2, 1.0),
        ("Blur answer 1", 1, 0.0),
        ("Blur answer 2", 2, 1.0),
        ("Blur answer 3", 3, 2.0),
    ];

    #[test]
    fn measured_points_both_styles() {
        for (n, v, want) in POINTS {
            assert_eq!(popup_to_loop_opacity(v, None, None).to_bits(), want.to_bits(), "{}", n);
        }
    }

    #[test]
    fn clamps() {
        assert_eq!(popup_to_loop_opacity(3, Some(1.0), None), 1.0);
        assert_eq!(popup_to_loop_opacity(1, None, Some(0.5)), 0.5);
        assert_eq!(popup_to_loop_opacity(-5, None, None), 0.0);
        assert_eq!(popup_to_loop_opacity(i32::MIN, None, None), 0.0);
    }

    #[test]
    fn identity_mutant_killed_at_trails_answer_2() {
        let (_, v, want) = POINTS[1];
        assert_ne!(mutant_identity(v).to_bits(), want.to_bits());
    }

    #[test]
    fn v_not_v_minus_1_mutant_killed_at_trails_answer_3() {
        let (_, v, want) = POINTS[2];
        assert_ne!(mutant_v_not_v_minus_1(v).to_bits(), want.to_bits());
    }

    #[test]
    fn hostcall_arm_refused_by_name() {
        let e = checkout_to_loop_opacity(1, 2, None, None).unwrap_err();
        assert!(e.starts_with("OPACITY-HOSTCALL-ARM-UNPORTED"), "{}", e);
        assert_eq!(checkout_to_loop_opacity(0, 2, None, None).unwrap().to_bits(), 1.0f32.to_bits());
    }

    /// W539 (b): Trails default cells (owner arm at opacity 1.0, row_gate_cell 0, 4 planes) derive .
    #[test]
    fn trails_default_cells_derive_r4() {
        use crate::hostconv14::{derived_head, OwnerArm, SourceGate, R4_HEAD};
        let a = OwnerArm { owner_arm_nonzero: false, owner_arm_bit2: false, mode_byte_exit: false, opacity_bits: Some(popup_to_loop_opacity(2, None, None).to_bits()), output_opacity_bits: None };
        let g = SourceGate::from_arm(&a, 0);
        assert_eq!(derived_head(4, &g).unwrap(), R4_HEAD);
        assert_eq!(crate::judge_keys::head_name(R4_HEAD), "R4");
    }

    /// W539 (b): Trails answer-1 cells (opacity 0.0, row_gate_cell 0, 3 planes) derive .
    #[test]
    fn trails_answer1_cells_derive_vb3() {
        use crate::hostconv14::{derived_head, OwnerArm, SourceGate, VB3_HEAD};
        let a = OwnerArm { owner_arm_nonzero: false, owner_arm_bit2: false, mode_byte_exit: false, opacity_bits: Some(popup_to_loop_opacity(0, None, None).to_bits()), output_opacity_bits: None };
        let g = SourceGate::from_arm(&a, 0);
        assert_eq!(derived_head(3, &g).unwrap(), VB3_HEAD);
        assert_eq!(crate::judge_keys::head_name(VB3_HEAD), "VB3");
    }
}
