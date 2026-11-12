//! STYLE-SELECT, the style of a multi-style effect from the host popup "Effect".
//! MEASURED in the emulator: the render path reads the program count
//! [owner] (movsx word; cmp 1; jg); with a count above 1 it checks out "Effect" (registered index 5, popup) and takes
//! style = max(answer - 1, 0) (dec ecx; cmovs ecx, 0); load the program pointer from [owner + 8 x style].
//! With a count of 1 or less the style index is 0 and no checkout is made. MEASURED for S_Blur: Effect = 2 gives the program pointer of
//! index 1 (Blur_Mono) and the Mono render, unbound or 1 gives Blur_Color.

/// The style index of the render path: max(answer - 1, 0) when the program count is above 1, else 0 (no checkout).
pub fn style_index(prog_count: i16, answer: i32) -> usize {
    if (prog_count as i32) > 1 {
        let v = answer.wrapping_sub(1);
        if v < 0 {
            0
        } else {
            v as usize
        }
    } else {
        0
    }
}

/// The program table [owner] per multi-style effect, in index order, as MEASURED (for S_Blur). Effects not listed are not
/// claimed here; a style of one of them is refused by STYLE-SELECT only when the run declares an Effect answer.
pub const STYLE_TABLE: &[(&str, &[&str])] = &[
    ("S_Blur", &["Blur_Color", "Blur_Mono"]),
    // (outcome A): S_BlurMoCurves has two styles. Effect = 2 is the Blur_Only layout object. The plugin's own def registration refuses the
    // def name Blur_Motion_Curves_No_Transform_Autogen (39 characters, bound 32), so the Blur_Only render is the layout object's own plan, which is
    // plan-equal (runs/cmp_rm30.txt) to the native compile of that def. The style name used here is the layout's style name.
    ("S_BlurMoCurves", &["Transform_and_Blur", "Blur_Only"]),
    // S_BlurMotion: two registered programs (the Effect popup, index 5, has 2 choices); Effect = 2 is Blur_Motion_Mono.
    ("S_BlurMotion", &["Blur_Motion", "Blur_Motion_Mono"]),
];

/// the registered def a style of S_BlurMoCurves runs. Blur_Only has no registered def in the plugin; its layout object compiles the same plan as
/// the def Blur_Motion_Curves_No_Transform_Autogen (the native compiles that text).
pub fn style_program(core: &str) -> Option<&'static str> {
    match core {
        "Transform_and_Blur" => Some("Transform_and_Blur_Autogen"),
        "Blur_Only" | "Blur_Motion_Curves_No_Transform" => Some("Blur_Motion_Curves_No_Transform_Autogen"),
        "Blur_Motion" => Some("Blur_Motion_Autogen"),
        "Blur_Motion_Mono" => Some("Blur_Motion_Mono_Autogen"),
        _ => None,
    }
}

/// The core program name of a registered program (the Autogen wrapper selects the same style as its core).
pub fn core_name(prog: &str) -> &str {
    prog.strip_suffix("_Autogen").unwrap_or(prog)
}

/// The effect and the selected program for a declared answer and count; Err names the refusal.
pub fn select(prog: &str, prog_count: i16, answer: i32) -> Result<(&'static str, usize, &'static str), String> {
    let core = core_name(prog);
    let (eff, tab) = STYLE_TABLE
        .iter()
        .find(|(_, t)| t.contains(&core))
        .ok_or_else(|| format!("STYLE-SELECT-NOT-TABLED: {} is in no measured program table", prog))?;
    if prog_count as usize != tab.len() {
        return Err(format!("STYLE-SELECT-COUNT-MISMATCH: {} declares [owner] = {}, the measured table has {}", eff, prog_count, tab.len()));
    }
    let i = style_index(prog_count, answer);
    let sel = tab.get(i).ok_or_else(|| format!("STYLE-INDEX-OUT-OF-TABLE: {} Effect answer {} gives index {}", eff, answer, i))?;
    Ok((eff, i, sel))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn style_index_rule() {
        assert_eq!(style_index(2, 1), 0);
        assert_eq!(style_index(2, 2), 1);
        assert_eq!(style_index(2, 0), 0);
        assert_eq!(style_index(2, -3), 0);
        assert_eq!(style_index(1, 2), 0); // count 1: no checkout
        assert_eq!(style_index(2, i32::MIN), i32::MAX as usize); // dec wraps, sign clear, kept (cmovs not taken)
    }
    #[test]
    fn style_select_s_blur_mo_curves() {
        assert_eq!(select("Blur_Only", 2, 2).unwrap(), ("S_BlurMoCurves", 1, "Blur_Only"));
        assert_eq!(select("Transform_and_Blur_Autogen", 2, 1).unwrap(), ("S_BlurMoCurves", 0, "Transform_and_Blur"));
        assert_eq!(select("Transform_and_Blur", 2, 2).unwrap().2, "Blur_Only"); // Effect 2 wins over the label
        assert_eq!(select("Transform_and_Blur", 2, 0).unwrap().2, "Transform_and_Blur"); // answer 0 clamps to style 0
        assert!(select("Transform_and_Blur", 2, 3).is_err()); // index 2 is out of the table
        assert!(select("Transform_and_Blur", 3, 2).is_err()); // wrong declared count
        assert!(select("Blur_Motion_Curves_No_Transform_Autogen", 2, 2).is_err()); // the 39 character def is in no table (STYLE-REFUSED-BY-PLUGIN)
    }
    #[test]
    fn style_select_s_blur_motion() {
        assert_eq!(select("Blur_Motion_Autogen", 2, 1).unwrap(), ("S_BlurMotion", 0, "Blur_Motion"));
        assert_eq!(select("Blur_Motion_Autogen", 2, 0).unwrap().2, "Blur_Motion"); // unbound answer clamps to style 0
        assert_eq!(select("Blur_Motion_Mono_Autogen", 2, 2).unwrap(), ("S_BlurMotion", 1, "Blur_Motion_Mono"));
        assert_eq!(select("Blur_Motion_Autogen", 2, 2).unwrap().2, "Blur_Motion_Mono"); // Effect 2 wins over the label
        assert!(select("Blur_Motion_Autogen", 2, 3).is_err()); // index 2 is out of the table
        assert!(select("Blur_Motion_Autogen", 3, 2).is_err()); // wrong declared count
        assert_eq!(style_program("Blur_Motion_Mono"), Some("Blur_Motion_Mono_Autogen"));
        assert_eq!(style_program("Blur_Motion"), Some("Blur_Motion_Autogen"));
    }
    #[test]
    fn style_program_rule() {
        assert_eq!(style_program("Blur_Only"), Some("Blur_Motion_Curves_No_Transform_Autogen"));
        assert_eq!(style_program("Transform_and_Blur"), Some("Transform_and_Blur_Autogen"));
        assert_eq!(style_program("Blur_Color"), None);
    }
    #[test]
    fn style_select_s_blur() {
        assert_eq!(select("Blur_Mono", 2, 2).unwrap(), ("S_Blur", 1, "Blur_Mono"));
        assert_eq!(select("Blur_Mono_Autogen", 2, 2).unwrap().2, "Blur_Mono");
        assert_eq!(select("Blur_Color", 2, 1).unwrap().2, "Blur_Color");
        assert_eq!(select("Blur_Mono", 2, 1).unwrap().2, "Blur_Color"); // a Mono label without Effect = 2 runs Color (VOID-NOT-MONO)
        assert!(select("Blur_Mono", 2, 3).is_err());
        assert!(select("Diffuse", 1, 1).is_err());
    }
}
