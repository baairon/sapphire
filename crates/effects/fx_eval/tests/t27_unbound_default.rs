//! the unbound-default test. Host default = text_default x 1920 / (2 x ns) on the 16.16 grid (INFERRED,
//! ), then normalize_scale_record. Blur_Color_Autogen blur_amount (text default 0.4, ns 4, NS_PARAMS.tsv) gives 12.0 at W 64
//! and 15.36 at W 50. Tolerance at W 50 fixed before the run: 2e-6 (about 2 ulp at 15.36).
use fx_eval::units::{ns_of, unbound_default_record, units_width};

#[test]
fn blur_amount_unbound_default() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let ns = ns_of("Blur_Color_Autogen", "blur_amount").expect("ns row");
    assert_eq!(ns, 4.0);
    let value_at_width_sixty_four = unbound_default_record(0.4, ns, units_width(64, 1.0, 1.0), false);
    assert_eq!(value_at_width_sixty_four, 12.0f32);
    let value_at_width_fifty = unbound_default_record(0.4, ns, units_width(50, 1.0, 1.0), false);
    assert!((value_at_width_fifty - 15.36f32).abs() <= 2e-6, "{}", value_at_width_fifty);
}
