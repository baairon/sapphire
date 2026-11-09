//!: the host-value to record-value conversion of normalize_scale parameters, lane 0.
//! "PARAM-UNITS (lane 0, ns 2 and 4: record = host x ((2.0 / W) x ns) in single precision, divided by 100.0 when [rec] & 0x40 is set;
//! W = (float)[ctx] x (float)([[ctx]] / [[ctx]]); host values on the 16.16 grid, 32 of 32 recorded tuples)".
//! Order read by vdivss 2.0 (cell) / W, vmulss by [rec] (ns), vmulss by the host value; the
//! divide by 100.0 (cell) after it.: lane 1 and ns 1 by conv_record, judged on
//! 18,880 emulator cases (test t24); the popup rule by popup_record. The 'ae32' route still converts lane 0 only (normalize_scale_record).

/// W of the conversion: f32((f64) ctx_unit_width x ([[ctx]] / [[ctx]])), the product in double and
/// one rounding to single, as transcription of reads it and its rig measured (18,880 of 18,880 emulator cases). The earlier
/// form (the ratio rounded to f32 first, then an f32 product) differs from the rig on 262 of the 18,880 cases (ratio other than 1.0); at
/// ratio 1.0, the only ratio of every claim run, both forms are equal (MEASURED by test t24).
pub fn units_width(ctx_unit_width: i32, fx_run_cell_b: f64, fx_run_cell_a: f64) -> f32 {
    ((ctx_unit_width as f32) as f64 * (fx_run_cell_b / fx_run_cell_a)) as f32
}

/// The ratio arm of: Plain is the arm that keeps the lane-1 operands at 1.0 ([ctx] != 0 and owner flag 0x10 clear;
/// the rig's "clear" arm); Ratio takes a = f32(d_cell_c / conv_in_a) and b = f32(d_cell_b / conv_in_a) (the rig's "null" and "set" arms).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RatioArm {
    Plain,
    Ratio,
}

/// One input of the conversion function, as rig T2 drives it (bit patterns kept: NaN payloads and signs count).
#[derive(Clone, Copy, Debug)]
pub struct ConvIn {
    pub w: i32,
    /// [ctx] as recorded by the run (never assumed 1.0)
    pub conv_in_b: f32,
    pub arm: RatioArm,
    pub conv_in_a: f64,
    pub d_cell_b: f64,
    pub d_cell_c: f64,
    /// parameter type: 1 float, 2 integer slider (rounded through cvttss2si), 4 and 5 not taken by kind 5 rows
    pub ptype: u32,
    pub ns: f32,
    pub kind: u32,
    pub flags: u32,
    pub lane: u32,
    pub host: f32,
}

/// vcvttss2si: truncation toward zero; NaN and out of range give the integer indefinite 0x80000000.
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x as f64 >= 2147483648.0 || (x as f64) < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// The record value of one lane: lane 0 = host x ((2 / W) x ns); lane 1 = lane 0 x
/// ((conv_in_b x clamp(a)) / (-clamp(b))) with clamp(v) = v when 0.0001 < v (as double) or v is NaN, else 1.0; the divide by 100.0 when
/// flags & 0x40 comes after. Kind (low 4 bits) 0 is the unit-less path (host kept, then the percent divide). None for the paths the rig did
/// not judge (types other than 1 and 2, kinds other than 0 and 5).
pub fn conv_record(c: &ConvIn) -> Option<f32> {
    let width = units_width(c.w, c.conv_in_a, c.d_cell_b);
    let (a, b) = match c.arm {
        RatioArm::Plain => (1.0f32, 1.0f32),
        RatioArm::Ratio => ((c.d_cell_c / c.conv_in_a) as f32, (c.d_cell_b / c.conv_in_a) as f32),
    };
    let mut x = c.host;
    match c.ptype {
        1 => {}
        2 => x = cvtt(x + 0.5f32) as f32,
        _ => return None,
    }
    match c.kind & 0xf {
        0 => {}
        5 => {
            let clamp = |v: f32| if (0.0001f64 < v as f64) || v.is_nan() { v } else { 1.0f32 };
            let k = (2.0f32 / width) * c.ns;
            x *= k;
            if c.lane == 1 {
                let num = c.conv_in_b * clamp(a);
                let den = -clamp(b);
                x *= num / den;
            }
        }
        _ => return None,
    }
    if c.flags & 0x40 != 0 {
        x /= 100.0f32;
    }
    Some(x)
}

/// (: dec eax; cmovs eax, 0; vcvtsi2ss): the record of a popup is f32(max(0, answer - 1)).
/// "POPUP-ONE-BASED (MEASURED at Filter 0,1,2,3,-3; Combine Masks 1; Opacity 2; clamp at 0, no wrap)". No upper clamp in the
/// block (INFERRED, not tested above the item count).
pub fn popup_record(answer: i32) -> f32 {
    let v = answer.wrapping_sub(1);
    (if v < 0 { 0 } else { v }) as f32
}

/// The 16.16 grid of the host checkout. round half to even, as the x86 conversion under the
/// default MXCSR rounding does (the earlier `round` sent the half-grid points k + 0.5 away from zero). Tests: t29_step7 (k = 0 and k = 2).
pub fn snap_16_16(host: f64) -> f32 {
    ((host * 65536.0).round_ties_even() / 65536.0) as f32
}

/// record = host x ((2 / W) x ns), f32 throughout; / 100 when the record flag 0x40 is set.
pub fn normalize_scale_record(host: f32, width: f32, ns: f32, flag40: bool) -> f32 {
    let t = 2.0f32 / width;
    let k = t * ns;
    let r = host * k;
    if flag40 {
        r / 100.0f32
    } else {
        r
    }
}

/// The number of lanes the range registration of PROGRAM's parameter PARAM gives, None when the parameter has no range row.
pub fn param_lanes(program: &str, param: &str) -> Option<usize> {
    effects_rtdata::range_defs().find(|r| r.program == program && r.parameter == param).map(|r| r.lanes)
}

/// Lane 0 of a normalize_scale parameter bound by `--host-param NAME=HOST` with one host lane:
/// normalize_scale_record(host, W, ns, false) with W the units width of the frame width, for every axis (x_, y_, z_ and tilt
/// parameters alike). `uh` is the units width of the frame height and is read only by the mutant ns1_height_mut.
/// A normalize_scale of 1 is judged for a one-lane parameter only (a two-lane parameter takes both host lanes);
/// 2 and 4 are judged for lane 0; any other normalize_scale is refused with PARAM-UNITS-NOT-JUDGED.
pub fn host_param_lane0(program: &str, param: &str, host: f32, ns: f32, uw: f32, uh: f32) -> Result<f32, String> {
    if ns != 1.0 && ns != 2.0 && ns != 4.0 {
        return Err(format!("PARAM-UNITS-NOT-JUDGED: {} has normalize_scale {} (the label covers 1, 2 and 4, lane 0)", param, ns));
    }
    if ns == 1.0 && param_lanes(program, param) != Some(1) {
        return Err(format!(
            "PARAM-UNITS-NOT-JUDGED: {} has normalize_scale 1 and {:?} registered lanes (one host lane is judged for a one-lane parameter; a two-lane parameter takes --host-param {}=X,Y)",
            param,
            param_lanes(program, param),
            param
        ));
    }
    let width = if cfg!(feature = "ns1_height_mut") && ns == 1.0 { uh } else { uw };
    Ok(normalize_scale_record(host, width, ns, false))
}

/// The registered host value of a normalize_scale slider whose program text default is `text_default`, lane 0:
/// f32(d / f32(f32(2 / 1920) x ns)), the reference width 1920 and every operation a single-precision one.
pub fn registered_ns_host(text_default: f32, ns: f32) -> f32 {
    let t = 2.0f32 / 1920.0f32;
    let k = t * ns;
    text_default / k
}

/// The registered 16.16 word of a normalize_scale slider: the registered host value widened to f64, multiplied by 65536
/// and truncated toward zero (NaN or out of range gives i32::MIN, as the conversion does).
pub fn registered_ns_bits(text_default: f32, ns: f32) -> i64 {
    let v = registered_ns_host(text_default, ns) as f64 * 65536.0;
    if v.is_nan() || v >= 2147483648.0 || v <= -2147483649.0 { i32::MIN as i64 } else { v.trunc() as i64 }
}

/// The record of an unbound normalize_scale parameter: the host value is the registered 16.16 word over 65536,
/// then the record is normalize_scale_record of it.
pub fn unbound_default_record(text_default: f32, ns: f32, width: f32, flag40: bool) -> f32 {
    let host = (registered_ns_bits(text_default, ns) as f64 / 65536.0) as f32;
    normalize_scale_record(host, width, ns, flag40)
}

/// the registered normalize_scale value per (program, parameter), defined in effects_rtdata (styles/<style>.rs, field `ns`).
/// The normalize_scale of a registered parameter, None when the program row has no normalize_scale for it.
pub fn ns_of(program: &str, param: &str) -> Option<f32> {
    effects_rtdata::ns_param(program, param).and_then(|r| r.normalize_scale)
}

/// The text default and the normalize_scale of a registered parameter, None when the row has no row or either is not a number.
pub fn ns_entry(program: &str, param: &str) -> Option<(f32, f32)> {
    let r = effects_rtdata::ns_param(program, param)?;
    Some((r.text_default?, r.normalize_scale?))
}

/// Every (parameter, normalize_scale) row of PROGRAM in table order (rows without a numeric normalize_scale are left out).
pub fn ns_rows_of(program: &str) -> Vec<(String, f32)> {
    effects_rtdata::ns_params_of(program).filter_map(|r| r.normalize_scale.map(|ns| (r.param.to_string(), ns))).collect()
}

/// the host's registration data per claimed style (one named claim-harness record per style), with the
/// registered_value_bits column (the int32 of the first four checkout value bytes of each host type 2 row, absent when not recorded).
/// One row of the registration for a program parameter (by record name).
#[derive(Debug, Clone, PartialEq)]
pub struct StyleParamRow {
    pub display_name: String,
    pub host_index: i32,
    pub host_type: i32,
    pub registered_value_bits: Option<i64>,
}

/// The registration row of PROGRAM's parameter PARAM, None when the program or the parameter has no row.
pub fn style_param_row(program: &str, param: &str) -> Option<StyleParamRow> {
    let r = effects_rtdata::style_param(program, param)?;
    Some(StyleParamRow { display_name: r.display_name.to_string(), host_index: r.host_index, host_type: r.host_type, registered_value_bits: r.registered_value_bits })
}

/// true when PARAM of PROGRAM is a fixed-slider (host type 2) parameter outside the normalize_scale table,
/// the parameters whose unbound loop lane is the registered 16.16 default. The mutant df_ns_mut drops the normalize_scale exclusion.
pub fn registered_fixed_slider(program: &str, param: &str) -> bool {
    let t2 = style_param_row(program, param).map(|r| r.host_type == 2).unwrap_or(false);
    t2 && (cfg!(feature = "df_ns_mut") || ns_of(program, param).is_none())
}

/// The value an unbound popup parameter of a registered program takes: f32(max(v - 1, 0)) for v the registered default
/// checkout integer of the parameter (the one-based popup answer converted to the loop lane). None when the parameter has no registered
/// default, in which case its text default stays. Mutants: pd_text_mut (None, the text default), pd_nominus_mut (v in place of v - 1).
pub fn registered_popup_default(program: &str, param: &str) -> Option<f32> {
    if cfg!(feature = "pd_text_mut") {
        return None;
    }
    let v = effects_rtdata::style_param(program, param)?.default_checkout_integer?;
    if cfg!(feature = "pd_nominus_mut") {
        return Some(v as f32);
    }
    Some((v - 1).max(0) as f32)
}

/// the registered 16.16 default of an unbound fixed-slider parameter, f32(trunc(d x 65536) / 65536) of its
/// text default lane d; a negative off-grid d refuses PARAM-DEFAULT-OFFGRID-NEGATIVE. Mutants: df_round_mut (round half to even),
/// df_f32text_mut (the f32 text default unchanged).
pub fn registered_16_16(program: &str, param: &str, d: f32) -> Result<f32, String> {
    if cfg!(feature = "df_f32text_mut") {
        return Ok(d);
    }
    let v = d as f64 * 65536.0;
    if v < 0.0 && v.fract() != 0.0 {
        return Err(format!("PARAM-DEFAULT-OFFGRID-NEGATIVE ({} {}: the unbound fixed-slider default {} is negative and off the 16.16 grid; truncation and flooring differ there and no record shows which value the host registers)", program, param, d));
    }
    let k = if cfg!(feature = "df_round_mut") { v.round_ties_even() } else { v.trunc() };
    Ok((k / 65536.0) as f32)
}

/// SDS widened: the host value of an UNBOUND normalize_scale host type 2 slider is the first int32 of its registered
/// default bytes (STYLE_PARAM_INDEX registered_value_bits) divided by 65536, taken before any normalize_scale or width scaling.
pub fn unbound_ns_host(program: &str, param: &str) -> Option<f32> {
    let row = style_param_row(program, param)?;
    if row.host_type != 2 {
        return None;
    }
    Some((row.registered_value_bits? as f64 / 65536.0) as f32)
}

/// Some(k) when a host type 2 slider's --host-param value is not on the 16.16 grid (f32 x 65536 not an integer).
pub fn host_value_off_grid(program: &str, param: &str, v: f32) -> Option<f64> {
    let row = style_param_row(program, param)?;
    if row.host_type != 2 {
        return None;
    }
    let k = v as f64 * 65536.0;
    if k == k.trunc() { None } else { Some(k) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn units_ns_table_rows() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert_eq!(ns_of("Diffuse_Autogen", "diffuse_amount"), Some(2.0));
        assert_eq!(ns_of("Diffuse_Autogen", "blur_mask"), Some(4.0));
        assert_eq!(ns_of("Diffuse_Autogen", "rel_amount"), None);
    }
    #[test]
    fn registered_ns_bits_rows() {
        assert_eq!(registered_ns_host(2.0, 4.0).to_bits(), 0x43efffff);
        assert_eq!(registered_ns_bits(2.0, 4.0), 31457278);
        assert_eq!(registered_ns_bits(0.4, 4.0), 6291456);
        assert_eq!(registered_ns_bits(0.2, 2.0), 6291456);
        assert_eq!(registered_ns_bits(0.01, 4.0), 157286);
        assert_eq!(registered_ns_bits(0.112, 4.0), 1761607);
        assert_eq!(registered_ns_bits(0.0, 4.0), 0);
        assert!((unbound_default_record(2.0, 4.0, 1920.0, false) - 2.0).abs() < 1e-4);
    }
    // (param, ns, host, W, H, compiled as recorded): TUPLES.tsv and T2_TUPLES.tsv (EQUAL rows, distinct values)
    const T: &[(&str, f32, f64, i32, i32, &str)] = &[
        ("blur_amount", 4.0, 0.5272979736328125, 50, 7, "0.08436767756938934"),
        ("blur_amount", 4.0, 0.5272979736328125, 64, 32, "0.06591224670410156"),
        ("blur_amount", 4.0, 96.0, 50, 7, "15.359999656677246"),
        ("blur_amount", 4.0, 96.0, 64, 32, "12.0"),
        ("diffuse_amount", 2.0, 3.0, 64, 32, "0.1875"),
        ("diffuse_amount", 2.0, 96.0, 64, 32, "6.0"),
        ("diffuse_amount", 2.0, 0.0, 50, 7, "0.0"),
        ("diffuse_amount", 2.0, 3.0, 100, 24, "0.11999999731779099"),
        ("diffuse_amount", 2.0, 3.0, 128, 32, "0.09375"),
        ("diffuse_amount", 2.0, 3.0, 50, 7, "0.23999999463558197"),
        ("diffuse_amount", 2.0, 3.0, 64, 8, "0.1875"),
        ("diffuse_amount", 2.0, 96.0, 50, 7, "7.679999828338623"),
    ];
    #[test]
    fn units_unt1_tuples_bit_equal() {
        for &(p, ns, host, w, _h, c) in T {
            let want: f32 = c.parse().unwrap();
            let got = normalize_scale_record(snap_16_16(host), units_width(w, 1.0, 1.0), ns, false);
            assert_eq!(got.to_bits(), want.to_bits(), "{} ns {} host {} W {}", p, ns, host, w);
        }
    }
    #[test]
    fn units_flag40_and_width_ratio() {
        assert_eq!(normalize_scale_record(96.0, 64.0, 2.0, true).to_bits(), (6.0f32 / 100.0).to_bits());
        assert_eq!(units_width(64, 2.0, 1.0), 128.0);
    }

    #[test]
    fn sds_default_is_first_int32_over_65536() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let u = |p: &str, n: &str| unbound_ns_host(p, n);
        assert_eq!(u("Sharpen_Best_Quality_Autogen", "small_detail_size"), Some(2.399993896484375f32));
        assert_ne!(u("Sharpen_Best_Quality_Autogen", "small_detail_size"), Some(2.4f32));
        assert_eq!(u("Sharpen_Fast_Autogen", "sharpen_width"), Some(26.879989624023438f32));
        assert_eq!(u("Sharpen_Best_Quality_Autogen", "no_such_param"), None);
    }

    #[test]
    fn sds_unbound_lanes_equal_the_records() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let h = unbound_ns_host("Sharpen_Best_Quality_Autogen", "small_detail_size").unwrap();
        assert_eq!(normalize_scale_record(h, 50.0, 4.0, false).to_bits(), 0x3ec49b85);
        assert_eq!(normalize_scale_record(h, 64.0, 4.0, false).to_bits(), 0x3e999980);
        assert_eq!(normalize_scale_record(2.4f32, 50.0, 4.0, false).to_bits(), 0x3ec49ba6);
        let w = unbound_ns_host("Sharpen_Fast_Autogen", "sharpen_width").unwrap();
        assert_eq!(normalize_scale_record(w, 64.0, 4.0, false).to_bits(), 0x40570a38);
        assert_eq!(normalize_scale_record(w, 50.0, 4.0, false).to_bits(), 0x4089a024);
    }

    #[test]
    fn sds_off_grid_refusal_and_on_grid_unchanged() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert!(host_value_off_grid("Sharpen_Best_Quality_Autogen", "small_detail_size", 2.4).is_some());
        assert!(host_value_off_grid("Sharpen_Fast_Autogen", "sharpen_width", 26.88).is_some());
        assert!(host_value_off_grid("Sharpen_Fast_Autogen", "sharpen_width", 60.0).is_none());
        assert!(host_value_off_grid("Sharpen_Fast_Autogen", "sharpen_width", 26.879989624023438).is_none());
        assert!(host_value_off_grid("Sharpen_Fast_Autogen", "no_such_param", 2.4).is_none());
    }

    /// Drop_Shadow's shift is a registered normalize_scale 1 parameter; the lanes from host values are
    /// f32(q16(host)) x f32(f32(2 / W) x 1), lane 1 negated, bit for bit on 200,000 pseudo-random hosts at W 64, 50 and 49 (width fixed at 64 differs).
    #[test]
    fn nss1_drop_shadow_shift_row_and_lanes() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert_eq!(ns_of("Drop_Shadow", "shift"), Some(1.0));
        assert_eq!(ns_of("Drop_Shadow", "shift_mocha_mask"), Some(1.0));
        let mut st: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut nxt = || {
            st = st.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (st >> 33) as u32
        };
        let mut differ64 = 0usize;
        for w in [64.0f32, 50.0, 49.0] {
            for _ in 0..200_000 / 3 + 1 {
                let h = snap_16_16(nxt() as f64 / (1u64 << 31) as f64 * 3.0);
                let want = ((h as f32) * ((2.0f32 / w) * 1.0f32)).to_bits();
                assert_eq!(normalize_scale_record(h, w, 1.0, false).to_bits(), want);
                assert_eq!((-normalize_scale_record(h, w, 1.0, false)).to_bits(), want ^ 0x8000_0000);
                if normalize_scale_record(h, 64.0, 1.0, false).to_bits() != want {
                    differ64 += 1;
                }
            }
        }
        assert!(differ64 > 100_000, "the width-64 mutant must differ at W 50 and 49: {}", differ64);
    }

    #[test]
    fn sds_table_bits_equal_text_default_rule() {
    if !effects_rtdata::have_tables() {
        return;
    }
        // bits = registered_ns_bits(text default, ns) for every NS host type 2 row with bits; disagreements listed
        let mut bad = Vec::new();
        let mut checked = 0;
        for l in effects_rtdata::text("fx_eval/data/NS_PARAMS.tsv").lines().skip(1) {
            let f: Vec<&str> = l.split('\t').collect();
            if f.len() < 4 { continue; }
            let (Ok(td), Ok(ns)) = (f[2].trim().parse::<f32>(), f[3].trim().parse::<f32>()) else { continue };
            if let Some(row) = style_param_row(f[0], f[1]) {
                if row.host_type == 2 {
                    if let Some(bits) = row.registered_value_bits {
                        checked += 1;
                        let want = registered_ns_bits(td, ns);
                        if want != bits { bad.push(format!("{} {} bits {} want {}", f[0], f[1], bits, want)); }
                    }
                }
            }
        }
        assert!(checked > 0);
        assert!(bad.is_empty(), "{:?}", bad);
    }
}

/// the host angle of Warp_Perspective's swing and roll (the program text flags both `angle`). MEASURED in the S_WarpPerspective
/// records (params_after_loop of SR_64x32 and SR_50x7: Swing 25 gives lane -25.0, Roll -15 gives +15.0; the unbound lane is -0.0 = -(+0)): the
/// record lane is the NEGATED host value, no degree to radian scaling. Scope: the two claimed programs and the two names; one host value per angle
/// was seen, so the rule for other values is INFERRED (the fresh rows and test it on originals).
pub fn angle_host_lane(program: &str, param: &str, host: f32) -> Option<f32> {
    if (program == "Warp_Perspective" || program == "Warp_Perspective_Autogen") && (param == "swing" || param == "roll") {
        Some(-host)
    } else {
        None
    }
}
